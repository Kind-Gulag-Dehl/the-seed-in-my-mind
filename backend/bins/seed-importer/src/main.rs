use anyhow::{anyhow, Context, Result};
use encoding::payload::payload_hash_hex;
use event_log::validation::validate_event;
use event_log::Event;
use event_log::{SYSTEM_BOUNDARY_EMITTER_ID_STR, SYSTEM_BOUNDARY_EMITTER_TITLE};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgPoolOptions, Postgres, Transaction};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct SeedFile {
    version: String,
    events: Vec<SeedEvent>,
}

#[derive(Debug, Deserialize, Clone)]
struct SeedEvent {
    id: Uuid,
    kind: String,
    payload: Value,
    speaker_identity_id: Option<Uuid>,
}

struct Options {
    file: PathBuf,
    force: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TargetKind {
    Idea,
    Rail,
}

impl TargetKind {
    fn as_i16(self) -> i16 {
        match self {
            TargetKind::Idea => 0,
            TargetKind::Rail => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TierEnum {
    Title,
    Sentence,
    Paragraph,
    Full,
}

impl TierEnum {
    fn as_i16(self) -> i16 {
        match self {
            TierEnum::Title => 0,
            TierEnum::Sentence => 1,
            TierEnum::Paragraph => 2,
            TierEnum::Full => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RailKind {
    Vine,
}

impl RailKind {
    fn as_i16(self) -> i16 {
        match self {
            RailKind::Vine => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum VineType {
    PathwayVine,
    NarrativeVine,
}

impl VineType {
    fn as_i16(self) -> i16 {
        match self {
            VineType::PathwayVine => 0,
            VineType::NarrativeVine => 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct PointerState {
    title_representation_id: Option<Uuid>,
    sentence_representation_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
struct RepresentationKey {
    target_kind: TargetKind,
    target_object_id: Uuid,
    tier_enum: TierEnum,
}

#[derive(Debug, Clone)]
struct RepresentationPointerUpdate {
    target_kind: TargetKind,
    target_object_id: Uuid,
    tier_enum: TierEnum,
    representation_id: Uuid,
}

#[derive(Debug, Clone)]
struct TempoPredicateInput {
    cycle_age_ge_dmin: bool,
    cycle_age_ge_dmax: bool,
    constrained_mode: bool,
    record_only_mode: bool,
}

#[derive(Debug, Clone)]
struct CycleBoundaryIngestRow {
    cycle_index: i64,
    closure_kind: i16,
    forced_seal: bool,
    closure_block_height: i64,
    source_block_height: i64,
    source_event_index: i32,
    source_event_id: Uuid,
}

#[derive(Debug, Clone)]
struct SnapshotCommitIngestRow {
    block_height: i64,
    snapshot_hash: String,
    state_root_hash: String,
    title_sentence_payload_root: String,
    shared_map_commitment: String,
    last_event_id: Uuid,
    event_count: i64,
    active_rulebook_set_hash: String,
    created_event_id: Uuid,
}

fn system_boundary_emitter_id() -> Uuid {
    Uuid::parse_str(SYSTEM_BOUNDARY_EMITTER_ID_STR).expect("valid system boundary emitter uuid")
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = parse_args()?;

    let seed_path = if options.file.is_relative() {
        env::current_dir()?.join(&options.file)
    } else {
        options.file.clone()
    };

    let contents = fs::read_to_string(&seed_path)
        .with_context(|| format!("reading seed file {}", seed_path.display()))?;
    let seed: SeedFile = serde_json::from_str(&contents)
        .with_context(|| format!("parsing seed JSON from {}", seed_path.display()))?;

    if seed.version.trim() != "seed-data-v0" {
        return Err(anyhow!("unsupported seed version: {}", seed.version));
    }

    if seed.events.is_empty() {
        return Err(anyhow!("seed file contains no events"));
    }

    let (seed_identity_id, identity_event_id) = extract_seed_identity(&seed.events)?;
    let seed_identity_title = extract_seed_identity_title(&seed.events, seed_identity_id)?;
    let mut canonical_events = seed.events.clone();
    ensure_seed_identity_cluster_events(
        &mut canonical_events,
        seed_identity_id,
        &seed_identity_title,
    )?;
    enforce_seed_identity(&canonical_events, seed_identity_id)?;

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    let existing_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events")
        .fetch_one(&pool)
        .await?;
    if existing_count.0 > 0 && !options.force {
        println!("seed-importer: skip (events already present)");
        return Ok(());
    }

    let mut tx = pool.begin().await?;
    sqlx::query("SET LOCAL seed.allow_canonical_mutation = 'on'")
        .execute(&mut *tx)
        .await?;

    if options.force {
        truncate_canonical_tables(&mut tx).await?;
    }

    insert_blocks(&mut tx).await?;

    let mut seen_event_ids = HashSet::new();
    let mut seen_rail_ids = HashSet::new();
    let mut seen_representation_ids = HashSet::new();
    let mut idea_rows = Vec::new();
    let mut connection_rows = Vec::new();
    let mut rail_rows = Vec::new();
    let mut rail_item_rows = Vec::new();
    let mut representation_rows = Vec::new();
    let mut cycle_boundary_rows = Vec::new();
    let mut snapshot_commit_rows = Vec::new();
    let mut tempo_rows: Vec<(i64, i32, TempoPredicateInput)> = Vec::new();
    let mut representation_keys: HashMap<Uuid, RepresentationKey> = HashMap::new();
    let mut idea_pointer_updates: HashMap<Uuid, PointerState> = HashMap::new();
    let mut rail_pointer_updates: HashMap<Uuid, PointerState> = HashMap::new();

    for (idx, event) in canonical_events.iter().enumerate() {
        if !seen_event_ids.insert(event.id) {
            return Err(anyhow!("duplicate event_id in seed file: {}", event.id));
        }

        let event_index = idx as i32;
        let stage0_event = Event {
            id: event.id,
            kind: event.kind.clone(),
            payload: event.payload.clone(),
            speaker_identity_id: event.speaker_identity_id,
        };

        validate_event(&stage0_event)
            .map_err(|err| anyhow!("event validation failed event_id={} {}", event.id, err))?;

        insert_event(&mut tx, 1, event_index, &stage0_event).await?;
        if let Some(tempo) = extract_tempo_predicates(&stage0_event.payload)? {
            tempo_rows.push((1, event_index, tempo));
        }

        match stage0_event.kind.as_str() {
            "idea_create" => {
                let payload = payload_object(&stage0_event.payload)?;
                let idea_id = parse_uuid_field(payload, "idea_id")?;
                let speaker = stage0_event
                    .speaker_identity_id
                    .ok_or_else(|| anyhow!("missing speaker_identity_id for idea_create"))?;

                let is_identity_idea = idea_id == seed_identity_id;
                let idea_type = if is_identity_idea {
                    "identity".to_string()
                } else {
                    "conceptual_idea".to_string()
                };

                idea_rows.push(IdeaRow {
                    idea_id,
                    idea_type,
                    speaker_identity_id: speaker,
                    is_identity_idea,
                    underlying_identity_id: if is_identity_idea {
                        Some(seed_identity_id)
                    } else {
                        None
                    },
                    is_personal_space_organizer: false,
                    title_representation_id: None,
                    sentence_representation_id: None,
                    created_block_height: 1,
                    created_event_index: event_index,
                    created_event_id: stage0_event.id,
                });
            }
            "connection_create" => {
                let payload = payload_object(&stage0_event.payload)?;
                let connection_id = parse_uuid_field(payload, "connection_id")?;
                let from_idea_id = parse_uuid_field(payload, "from_idea_id")?;
                let to_idea_id = parse_uuid_field(payload, "to_idea_id")?;
                let connection_type = parse_string_field(payload, "connection_type")?.to_string();
                let usage = optional_string_field(payload, "usage")?;
                let axis = optional_string_field(payload, "axis")?;
                let timeframe = optional_string_field(payload, "timeframe")?;
                let scope = optional_string_field(payload, "scope")?;

                connection_rows.push(ConnectionRow {
                    connection_id,
                    from_idea_id,
                    to_idea_id,
                    connection_type,
                    usage,
                    axis,
                    timeframe,
                    scope,
                    created_block_height: 1,
                    created_event_index: event_index,
                    created_by_event_id: stage0_event.id,
                });
            }
            "rail_create" => {
                let payload = payload_object(&stage0_event.payload)?;
                let rail_id = parse_uuid_field(payload, "rail_id")?;
                if !seen_rail_ids.insert(rail_id) {
                    return Err(anyhow!("duplicate rail_id in seed file: {}", rail_id));
                }

                let rail_kind = parse_rail_kind_field(payload, "rail_kind")?;
                let vine_type = parse_vine_type_field(
                    payload,
                    "vine_type",
                    matches!(rail_kind, RailKind::Vine),
                )?;
                let speaker_identity_id = stage0_event
                    .speaker_identity_id
                    .ok_or_else(|| anyhow!("missing speaker_identity_id for rail_create"))?;
                let item_idea_ids = parse_uuid_array_field(payload, "item_idea_ids")?;
                let step_meta = parse_step_meta(payload, item_idea_ids.len())?;
                let initial_refs = parse_initial_representation_refs(payload)?;

                rail_rows.push(RailRow {
                    rail_id,
                    rail_kind: rail_kind.as_i16(),
                    vine_type: vine_type.map(VineType::as_i16),
                    speaker_identity_id,
                    created_block_height: 1,
                    created_event_index: event_index,
                    created_event_id: stage0_event.id,
                    base_rail_id: None,
                    title_representation_id: initial_refs.title_representation_id,
                    sentence_representation_id: initial_refs.sentence_representation_id,
                });
                if initial_refs.title_representation_id.is_some()
                    || initial_refs.sentence_representation_id.is_some()
                {
                    rail_pointer_updates.insert(rail_id, initial_refs);
                }

                for (idx, idea_id) in item_idea_ids.into_iter().enumerate() {
                    let via_connection_id = if idx == 0 {
                        None
                    } else {
                        step_meta.get(idx - 1).copied().flatten()
                    };
                    rail_item_rows.push(RailItemRow {
                        rail_id,
                        idx: idx as i32,
                        idea_id,
                        via_connection_id,
                    });
                }
            }
            "rail_fork" => {
                let payload = payload_object(&stage0_event.payload)?;
                let rail_id = parse_uuid_field(payload, "rail_id")?;
                if !seen_rail_ids.insert(rail_id) {
                    return Err(anyhow!("duplicate rail_id in seed file: {}", rail_id));
                }

                let base_rail_id = parse_uuid_field(payload, "base_rail_id")?;
                let base = rail_rows
                    .iter()
                    .find(|row| row.rail_id == base_rail_id)
                    .ok_or_else(|| anyhow!("rail_fork base_rail_id not found: {}", base_rail_id))?;
                let vine_type = parse_vine_type_field(payload, "vine_type", false)?
                    .map(VineType::as_i16)
                    .or(base.vine_type);
                let speaker_identity_id = stage0_event
                    .speaker_identity_id
                    .ok_or_else(|| anyhow!("missing speaker_identity_id for rail_fork"))?;
                let item_idea_ids = parse_uuid_array_field(payload, "item_idea_ids")?;
                let step_meta = parse_step_meta(payload, item_idea_ids.len())?;
                let initial_refs = parse_initial_representation_refs(payload)?;

                rail_rows.push(RailRow {
                    rail_id,
                    rail_kind: RailKind::Vine.as_i16(),
                    vine_type,
                    speaker_identity_id,
                    created_block_height: 1,
                    created_event_index: event_index,
                    created_event_id: stage0_event.id,
                    base_rail_id: Some(base_rail_id),
                    title_representation_id: initial_refs.title_representation_id,
                    sentence_representation_id: initial_refs.sentence_representation_id,
                });
                if initial_refs.title_representation_id.is_some()
                    || initial_refs.sentence_representation_id.is_some()
                {
                    rail_pointer_updates.insert(rail_id, initial_refs);
                }

                for (idx, idea_id) in item_idea_ids.into_iter().enumerate() {
                    let via_connection_id = if idx == 0 {
                        None
                    } else {
                        step_meta.get(idx - 1).copied().flatten()
                    };
                    rail_item_rows.push(RailItemRow {
                        rail_id,
                        idx: idx as i32,
                        idea_id,
                        via_connection_id,
                    });
                }
            }
            "representation_create" | "rail_update_representation" => {
                let payload = payload_object(&stage0_event.payload)?;
                let representation_id = parse_uuid_field(payload, "representation_id")?;
                if !seen_representation_ids.insert(representation_id) {
                    return Err(anyhow!(
                        "duplicate representation_id in seed file: {}",
                        representation_id
                    ));
                }
                let target_kind = parse_target_kind_field(payload, "target_kind")?;
                let target_object_id = parse_uuid_field(payload, "target_object_id")?;
                let tier_enum = parse_tier_enum_field(payload, "tier_length")?;
                let tier_complexity = parse_tier_complexity_field(payload, "tier_complexity")?;
                let payload_hash = parse_string_field(payload, "payload_hash")?.to_string();
                let author_identity_id = parse_uuid_field(payload, "author_identity_id")?;
                let language_locale = optional_string_field(payload, "language_locale")?;
                let provenance = optional_string_field(payload, "provenance")?;
                let payload_text = optional_representation_payload_text(payload)?;

                representation_rows.push(RepresentationRow {
                    representation_id,
                    target_kind: target_kind.as_i16(),
                    target_id: target_object_id,
                    tier_enum: tier_enum.as_i16(),
                    tier_complexity,
                    payload_hash: payload_hash.clone(),
                    payload_text,
                    author_identity_id,
                    language_locale,
                    provenance,
                    created_block_height: 1,
                    created_event_index: event_index,
                    created_event_id: stage0_event.id,
                });
                representation_keys.insert(
                    representation_id,
                    RepresentationKey {
                        target_kind,
                        target_object_id,
                        tier_enum,
                    },
                );
            }
            "challenge_finalize_verdict" => {
                let payload = payload_object(&stage0_event.payload)?;
                for update in parse_representation_pointer_updates(payload)? {
                    let key = representation_keys
                        .get(&update.representation_id)
                        .ok_or_else(|| {
                            anyhow!(
                            "challenge_finalize_verdict references unknown representation_id {}",
                            update.representation_id
                        )
                        })?;
                    if key.target_kind != update.target_kind
                        || key.target_object_id != update.target_object_id
                        || key.tier_enum != update.tier_enum
                    {
                        return Err(anyhow!(
                            "challenge_finalize_verdict selection mismatch representation_id={}",
                            update.representation_id
                        ));
                    }

                    match update.target_kind {
                        TargetKind::Idea => {
                            if !idea_rows
                                .iter()
                                .any(|row| row.idea_id == update.target_object_id)
                            {
                                return Err(anyhow!(
                                    "challenge_finalize_verdict target idea missing: {}",
                                    update.target_object_id
                                ));
                            }
                            let pointers = idea_pointer_updates
                                .entry(update.target_object_id)
                                .or_default();
                            apply_pointer_update(pointers, &update);
                        }
                        TargetKind::Rail => {
                            if !rail_rows
                                .iter()
                                .any(|row| row.rail_id == update.target_object_id)
                            {
                                return Err(anyhow!(
                                    "challenge_finalize_verdict target rail missing: {}",
                                    update.target_object_id
                                ));
                            }
                            let pointers = rail_pointer_updates
                                .entry(update.target_object_id)
                                .or_default();
                            apply_pointer_update(pointers, &update);
                        }
                    }
                }
            }
            "cycle_close" => {
                let payload = payload_object(&stage0_event.payload)?;
                let cycle_index = parse_non_negative_i64_field(payload, "cycle_index")?;
                let closure_kind_label = parse_string_field(payload, "closure_kind")?;
                let closure_kind = match closure_kind_label {
                    "deliberative" => 0_i16,
                    "forced" => 1_i16,
                    _ => {
                        return Err(anyhow!(
                            "cycle_close has invalid closure_kind={}",
                            closure_kind_label
                        ))
                    }
                };
                let forced_seal = parse_bool_field(payload, "forced_seal")?;
                if forced_seal != (closure_kind == 1) {
                    return Err(anyhow!(
                        "cycle_close forced_seal mismatch cycle_index={}",
                        cycle_index
                    ));
                }
                let closure_boundary_ref = payload
                    .get("closure_boundary_ref")
                    .ok_or_else(|| anyhow!("cycle_close missing closure_boundary_ref"))?;
                let closure_block_height = parse_closure_boundary_height(closure_boundary_ref)?;

                cycle_boundary_rows.push(CycleBoundaryIngestRow {
                    cycle_index,
                    closure_kind,
                    forced_seal,
                    closure_block_height,
                    source_block_height: 1,
                    source_event_index: event_index,
                    source_event_id: stage0_event.id,
                });
            }
            "snapshot_commit" => {
                let payload = payload_object(&stage0_event.payload)?;
                snapshot_commit_rows.push(SnapshotCommitIngestRow {
                    block_height: parse_non_negative_i64_field(payload, "block_height")?,
                    snapshot_hash: parse_string_field(payload, "snapshot_hash")?.to_string(),
                    state_root_hash: parse_string_field(payload, "state_root_hash")?.to_string(),
                    title_sentence_payload_root: parse_string_field(
                        payload,
                        "title_sentence_payload_root",
                    )?
                    .to_string(),
                    shared_map_commitment: parse_string_field(payload, "shared_map_commitment")?
                        .to_string(),
                    last_event_id: parse_uuid_field(payload, "last_event_id")?,
                    event_count: parse_non_negative_i64_field(payload, "event_count")?,
                    active_rulebook_set_hash: parse_string_field(
                        payload,
                        "active_rulebook_set_hash",
                    )?
                    .to_string(),
                    created_event_id: stage0_event.id,
                });
            }
            _ => {}
        }
    }

    let identity_idea_ids: HashSet<Uuid> = idea_rows
        .iter()
        .filter(|row| row.is_identity_idea)
        .map(|row| row.idea_id)
        .collect();
    let organizer_idea_ids: HashSet<Uuid> = connection_rows
        .iter()
        .filter(|row| row.connection_type == "membership")
        .filter(|row| row.usage.as_deref() == Some("has_space"))
        .filter(|row| identity_idea_ids.contains(&row.from_idea_id))
        .map(|row| row.to_idea_id)
        .collect();
    for row in &mut idea_rows {
        if organizer_idea_ids.contains(&row.idea_id) {
            row.is_personal_space_organizer = true;
        }
        if let Some(pointers) = idea_pointer_updates.get(&row.idea_id) {
            row.title_representation_id = pointers.title_representation_id;
            row.sentence_representation_id = pointers.sentence_representation_id;
        }
    }

    for row in &mut rail_rows {
        if let Some(pointers) = rail_pointer_updates.get(&row.rail_id) {
            if pointers.title_representation_id.is_some() {
                row.title_representation_id = pointers.title_representation_id;
            }
            if pointers.sentence_representation_id.is_some() {
                row.sentence_representation_id = pointers.sentence_representation_id;
            }
        }
    }

    let created_event_id = identity_event_id.unwrap_or(canonical_events[0].id);
    upsert_identity(
        &mut tx,
        seed_identity_id,
        &seed_identity_title,
        created_event_id,
    )
    .await?;
    upsert_identity(
        &mut tx,
        system_boundary_emitter_id(),
        SYSTEM_BOUNDARY_EMITTER_TITLE,
        system_boundary_emitter_id(),
    )
    .await?;

    insert_cycle_boundaries(&mut tx, &cycle_boundary_rows).await?;
    insert_snapshot_commits(&mut tx, &snapshot_commit_rows).await?;
    insert_tempo_predicates(&mut tx, &tempo_rows).await?;
    insert_representations(&mut tx, &representation_rows).await?;
    insert_ideas(&mut tx, &idea_rows).await?;
    insert_connections(&mut tx, &connection_rows).await?;
    insert_rails(&mut tx, &rail_rows).await?;
    insert_rail_items(&mut tx, &rail_item_rows).await?;

    tx.commit().await?;

    println!(
        "seed-importer: imported events={} ideas={} connections={} rails={} representations={}",
        canonical_events.len(),
        idea_rows.len(),
        connection_rows.len(),
        rail_rows.len(),
        representation_rows.len()
    );

    Ok(())
}

fn parse_args() -> Result<Options> {
    let mut file: Option<PathBuf> = None;
    let mut force = false;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--file" => {
                let value = args
                    .next()
                    .ok_or_else(|| anyhow!("missing value for --file"))?;
                file = Some(PathBuf::from(value));
            }
            "--force" => {
                force = true;
            }
            _ => {
                return Err(anyhow!(
                    "unexpected argument '{}' (usage: seed-importer [--file <path>] [--force])",
                    arg
                ));
            }
        }
    }

    let file = match file {
        Some(path) => path,
        None => default_seed_path()?,
    };

    Ok(Options { file, force })
}

fn default_seed_path() -> Result<PathBuf> {
    let repo_root = repo_root()?;
    Ok(repo_root.join("seed").join("seed-data-v0.json"))
}

fn repo_root() -> Result<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let backend_root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .ok_or_else(|| anyhow!("unable to resolve backend root"))?;
    let repo_root = backend_root
        .parent()
        .ok_or_else(|| anyhow!("unable to resolve repo root"))?;
    Ok(repo_root.to_path_buf())
}

async fn truncate_canonical_tables(tx: &mut Transaction<'_, Postgres>) -> Result<()> {
    sqlx::query(
        "TRUNCATE TABLE identities_s0, tempo_predicates, cycle_boundaries, snapshot_commits, challenge_arguments, challenge_targets, challenge_context, challenges, rail_items, rails, representations, connections, ideas, events, snapshots, blocks RESTART IDENTITY CASCADE",
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn insert_blocks(tx: &mut Transaction<'_, Postgres>) -> Result<()> {
    sqlx::query(
        "INSERT INTO blocks (block_height, block_hash, prev_block_hash) VALUES ($1, $2, $3) ON CONFLICT (block_height) DO NOTHING",
    )
    .bind(0_i64)
    .bind("00")
    .bind::<Option<&str>>(None)
    .execute(&mut **tx)
    .await?;

    sqlx::query(
        "INSERT INTO blocks (block_height, block_hash, prev_block_hash) VALUES ($1, $2, $3) ON CONFLICT (block_height) DO NOTHING",
    )
    .bind(1_i64)
    .bind("01")
    .bind(Some("00"))
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_event(
    tx: &mut Transaction<'_, Postgres>,
    block_height: i64,
    event_index: i32,
    event: &Event,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO events (
          block_height,
          event_index,
          event_id,
          event_type,
          speaker_identity_id,
          payload_json,
          signature
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(block_height)
    .bind(event_index)
    .bind(event.id)
    .bind(&event.kind)
    .bind(event.speaker_identity_id)
    .bind(&event.payload)
    .bind::<Option<String>>(None)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn upsert_identity(
    tx: &mut Transaction<'_, Postgres>,
    identity_id: Uuid,
    title: &str,
    created_event_id: Uuid,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO identities_s0 (
          identity_id,
          title,
          created_event_id
        ) VALUES ($1, $2, $3)
        ON CONFLICT (identity_id) DO UPDATE SET
          title = EXCLUDED.title,
          created_event_id = EXCLUDED.created_event_id
        "#,
    )
    .bind(identity_id)
    .bind(title)
    .bind(created_event_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

#[derive(Debug)]
struct IdeaRow {
    idea_id: Uuid,
    idea_type: String,
    speaker_identity_id: Uuid,
    is_identity_idea: bool,
    underlying_identity_id: Option<Uuid>,
    is_personal_space_organizer: bool,
    title_representation_id: Option<Uuid>,
    sentence_representation_id: Option<Uuid>,
    created_block_height: i64,
    created_event_index: i32,
    created_event_id: Uuid,
}

async fn insert_ideas(tx: &mut Transaction<'_, Postgres>, ideas: &[IdeaRow]) -> Result<()> {
    for idea in ideas {
        sqlx::query(
            r#"
            INSERT INTO ideas (
              idea_id,
              idea_type,
              speaker_identity_id,
              is_identity_idea,
              underlying_identity_id,
              is_personal_space_organizer,
              title_representation_id,
              sentence_representation_id,
              created_block_height,
              created_event_index,
              created_event_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(idea.idea_id)
        .bind(&idea.idea_type)
        .bind(idea.speaker_identity_id)
        .bind(idea.is_identity_idea)
        .bind(idea.underlying_identity_id)
        .bind(idea.is_personal_space_organizer)
        .bind(idea.title_representation_id)
        .bind(idea.sentence_representation_id)
        .bind(idea.created_block_height)
        .bind(idea.created_event_index)
        .bind(idea.created_event_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

#[derive(Debug)]
struct ConnectionRow {
    connection_id: Uuid,
    from_idea_id: Uuid,
    to_idea_id: Uuid,
    connection_type: String,
    usage: Option<String>,
    axis: Option<String>,
    timeframe: Option<String>,
    scope: Option<String>,
    created_block_height: i64,
    created_event_index: i32,
    created_by_event_id: Uuid,
}

#[derive(Debug)]
struct RepresentationRow {
    representation_id: Uuid,
    target_kind: i16,
    target_id: Uuid,
    tier_enum: i16,
    tier_complexity: i16,
    payload_hash: String,
    payload_text: Option<String>,
    author_identity_id: Uuid,
    language_locale: Option<String>,
    provenance: Option<String>,
    created_block_height: i64,
    created_event_index: i32,
    created_event_id: Uuid,
}

#[derive(Debug)]
struct RailRow {
    rail_id: Uuid,
    rail_kind: i16,
    vine_type: Option<i16>,
    speaker_identity_id: Uuid,
    created_block_height: i64,
    created_event_index: i32,
    created_event_id: Uuid,
    base_rail_id: Option<Uuid>,
    title_representation_id: Option<Uuid>,
    sentence_representation_id: Option<Uuid>,
}

#[derive(Debug)]
struct RailItemRow {
    rail_id: Uuid,
    idx: i32,
    idea_id: Uuid,
    via_connection_id: Option<Uuid>,
}

async fn insert_connections(
    tx: &mut Transaction<'_, Postgres>,
    connections: &[ConnectionRow],
) -> Result<()> {
    for connection in connections {
        sqlx::query(
            r#"
            INSERT INTO connections (
              connection_id,
              from_idea_id,
              to_idea_id,
              connection_type,
              usage,
              axis,
              timeframe,
              scope,
              created_block_height,
              created_event_index,
              created_by_event_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(connection.connection_id)
        .bind(connection.from_idea_id)
        .bind(connection.to_idea_id)
        .bind(&connection.connection_type)
        .bind(&connection.usage)
        .bind(&connection.axis)
        .bind(&connection.timeframe)
        .bind(&connection.scope)
        .bind(connection.created_block_height)
        .bind(connection.created_event_index)
        .bind(connection.created_by_event_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn insert_cycle_boundaries(
    tx: &mut Transaction<'_, Postgres>,
    rows: &[CycleBoundaryIngestRow],
) -> Result<()> {
    for row in rows {
        sqlx::query(
            r#"
            INSERT INTO cycle_boundaries (
              cycle_index,
              closure_kind,
              forced_seal,
              closure_block_height,
              source_block_height,
              source_event_index,
              source_event_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(row.cycle_index)
        .bind(row.closure_kind)
        .bind(row.forced_seal)
        .bind(row.closure_block_height)
        .bind(row.source_block_height)
        .bind(row.source_event_index)
        .bind(row.source_event_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn insert_snapshot_commits(
    tx: &mut Transaction<'_, Postgres>,
    rows: &[SnapshotCommitIngestRow],
) -> Result<()> {
    for row in rows {
        sqlx::query(
            r#"
            INSERT INTO snapshot_commits (
              block_height,
              snapshot_hash,
              state_root_hash,
              title_sentence_payload_root,
              shared_map_commitment,
              last_event_id,
              event_count,
              active_rulebook_set_hash,
              created_event_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(row.block_height)
        .bind(&row.snapshot_hash)
        .bind(&row.state_root_hash)
        .bind(&row.title_sentence_payload_root)
        .bind(&row.shared_map_commitment)
        .bind(row.last_event_id)
        .bind(row.event_count)
        .bind(&row.active_rulebook_set_hash)
        .bind(row.created_event_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn insert_tempo_predicates(
    tx: &mut Transaction<'_, Postgres>,
    rows: &[(i64, i32, TempoPredicateInput)],
) -> Result<()> {
    for (block_height, event_index, row) in rows {
        sqlx::query(
            r#"
            INSERT INTO tempo_predicates (
              block_height,
              event_index,
              cycle_age_ge_dmin,
              cycle_age_ge_dmax,
              constrained_mode,
              record_only_mode
            ) VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (block_height, event_index) DO UPDATE SET
              cycle_age_ge_dmin = EXCLUDED.cycle_age_ge_dmin,
              cycle_age_ge_dmax = EXCLUDED.cycle_age_ge_dmax,
              constrained_mode = EXCLUDED.constrained_mode,
              record_only_mode = EXCLUDED.record_only_mode
            "#,
        )
        .bind(*block_height)
        .bind(*event_index)
        .bind(row.cycle_age_ge_dmin)
        .bind(row.cycle_age_ge_dmax)
        .bind(row.constrained_mode)
        .bind(row.record_only_mode)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn insert_representations(
    tx: &mut Transaction<'_, Postgres>,
    rows: &[RepresentationRow],
) -> Result<()> {
    for row in rows {
        sqlx::query(
            r#"
            INSERT INTO representations (
              representation_id,
              target_kind,
              target_id,
              tier_enum,
              tier_complexity,
              payload_hash,
              payload_text,
              author_identity_id,
              language_locale,
              provenance,
              created_block_height,
              created_event_index,
              created_event_id
            ) VALUES (
              $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
            )
            "#,
        )
        .bind(row.representation_id)
        .bind(row.target_kind)
        .bind(row.target_id)
        .bind(row.tier_enum)
        .bind(row.tier_complexity)
        .bind(&row.payload_hash)
        .bind(&row.payload_text)
        .bind(row.author_identity_id)
        .bind(&row.language_locale)
        .bind(&row.provenance)
        .bind(row.created_block_height)
        .bind(row.created_event_index)
        .bind(row.created_event_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn insert_rails(tx: &mut Transaction<'_, Postgres>, rows: &[RailRow]) -> Result<()> {
    for row in rows {
        sqlx::query(
            r#"
            INSERT INTO rails (
              rail_id,
              rail_kind,
              vine_type,
              speaker_identity_id,
              created_block_height,
              created_event_index,
              created_event_id,
              base_rail_id,
              title_representation_id,
              sentence_representation_id
            ) VALUES (
              $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
            )
            "#,
        )
        .bind(row.rail_id)
        .bind(row.rail_kind)
        .bind(row.vine_type)
        .bind(row.speaker_identity_id)
        .bind(row.created_block_height)
        .bind(row.created_event_index)
        .bind(row.created_event_id)
        .bind(row.base_rail_id)
        .bind(row.title_representation_id)
        .bind(row.sentence_representation_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn insert_rail_items(tx: &mut Transaction<'_, Postgres>, rows: &[RailItemRow]) -> Result<()> {
    for row in rows {
        sqlx::query(
            r#"
            INSERT INTO rail_items (
              rail_id,
              idx,
              idea_id,
              via_connection_id
            ) VALUES (
              $1, $2, $3, $4
            )
            "#,
        )
        .bind(row.rail_id)
        .bind(row.idx)
        .bind(row.idea_id)
        .bind(row.via_connection_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

fn payload_object(value: &Value) -> Result<&serde_json::Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| anyhow!("payload must be a JSON object"))
}

fn parse_uuid_field(payload: &serde_json::Map<String, Value>, field: &str) -> Result<Uuid> {
    let value = parse_string_field(payload, field)?;
    Ok(Uuid::parse_str(value).map_err(|_| anyhow!("invalid uuid in field {}", field))?)
}

fn parse_string_field<'a>(
    payload: &'a serde_json::Map<String, Value>,
    field: &str,
) -> Result<&'a str> {
    payload
        .get(field)
        .and_then(|value| value.as_str())
        .ok_or_else(|| anyhow!("missing or invalid {}", field))
}

fn optional_string_field(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Option<String>> {
    match payload.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_str()
            .map(|value| Some(value.to_string()))
            .ok_or_else(|| anyhow!("invalid {}", field)),
    }
}

fn parse_bool_field(payload: &serde_json::Map<String, Value>, field: &str) -> Result<bool> {
    payload
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| anyhow!("missing or invalid {}", field))
}

fn parse_non_negative_i64_field(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<i64> {
    let value = payload
        .get(field)
        .ok_or_else(|| anyhow!("missing {}", field))?;
    parse_non_negative_i64_value(value, field)
}

fn parse_non_negative_i64_value(value: &Value, field: &str) -> Result<i64> {
    let parsed = match value {
        Value::Number(number) => number
            .as_i64()
            .ok_or_else(|| anyhow!("invalid {}", field))?,
        Value::String(text) => text
            .parse::<i64>()
            .map_err(|_| anyhow!("invalid {}", field))?,
        _ => return Err(anyhow!("invalid {}", field)),
    };
    if parsed < 0 {
        return Err(anyhow!("{} must be non-negative", field));
    }
    Ok(parsed)
}

fn parse_closure_boundary_height(value: &Value) -> Result<i64> {
    match value {
        Value::Object(object) => {
            let block_height = object
                .get("block_height")
                .ok_or_else(|| anyhow!("missing closure_boundary_ref.block_height"))?;
            parse_non_negative_i64_value(block_height, "closure_boundary_ref.block_height")
        }
        Value::Number(_) | Value::String(_) => {
            parse_non_negative_i64_value(value, "closure_boundary_ref")
        }
        _ => Err(anyhow!(
            "closure_boundary_ref must be object, string, or number"
        )),
    }
}

fn extract_tempo_predicates(payload: &Value) -> Result<Option<TempoPredicateInput>> {
    let payload = payload_object(payload)?;
    let source = if let Some(value) = payload.get("tempo_predicates") {
        value
            .as_object()
            .ok_or_else(|| anyhow!("invalid tempo_predicates object"))?
    } else if payload.contains_key("cycle_age_ge_dmin") || payload.contains_key("cycle_age_ge_dmax")
    {
        payload
    } else {
        return Ok(None);
    };

    let cycle_age_ge_dmin = source
        .get("cycle_age_ge_dmin")
        .and_then(Value::as_bool)
        .ok_or_else(|| anyhow!("tempo_predicates.cycle_age_ge_dmin required"))?;
    let cycle_age_ge_dmax = source
        .get("cycle_age_ge_dmax")
        .and_then(Value::as_bool)
        .ok_or_else(|| anyhow!("tempo_predicates.cycle_age_ge_dmax required"))?;
    let constrained_mode = source
        .get("constrained_mode")
        .or_else(|| source.get("tempo_constrained_mode"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let record_only_mode = source
        .get("record_only_mode")
        .or_else(|| source.get("tempo_record_only_mode"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    Ok(Some(TempoPredicateInput {
        cycle_age_ge_dmin,
        cycle_age_ge_dmax,
        constrained_mode,
        record_only_mode,
    }))
}

fn optional_representation_payload_text(
    payload: &serde_json::Map<String, Value>,
) -> Result<Option<String>> {
    for field in ["payload_text", "text", "payload"] {
        if let Some(value) = payload.get(field) {
            if value.is_null() {
                continue;
            }
            let text = value.as_str().ok_or_else(|| anyhow!("invalid {}", field))?;
            return Ok(Some(text.to_string()));
        }
    }
    Ok(None)
}

fn parse_target_kind_field(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<TargetKind> {
    let value = payload
        .get(field)
        .ok_or_else(|| anyhow!("missing {}", field))?;
    match value {
        Value::String(value) => match value.as_str() {
            "idea" => Ok(TargetKind::Idea),
            "rail" => Ok(TargetKind::Rail),
            _ => Err(anyhow!("invalid {}", field)),
        },
        Value::Number(value) => match value.as_u64() {
            Some(0) => Ok(TargetKind::Idea),
            Some(1) => Ok(TargetKind::Rail),
            _ => Err(anyhow!("invalid {}", field)),
        },
        _ => Err(anyhow!("invalid {}", field)),
    }
}

fn parse_rail_kind_field(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<RailKind> {
    let value = payload
        .get(field)
        .ok_or_else(|| anyhow!("missing {}", field))?;
    match value {
        Value::String(value) if value == "vine" => Ok(RailKind::Vine),
        Value::Number(value) if value.as_u64() == Some(0) => Ok(RailKind::Vine),
        _ => Err(anyhow!("invalid {}", field)),
    }
}

fn parse_vine_type_field(
    payload: &serde_json::Map<String, Value>,
    field: &str,
    required: bool,
) -> Result<Option<VineType>> {
    let Some(value) = payload.get(field) else {
        if required {
            return Err(anyhow!("missing {}", field));
        }
        return Ok(None);
    };
    if value.is_null() {
        if required {
            return Err(anyhow!("missing {}", field));
        }
        return Ok(None);
    }
    match value {
        Value::String(value) => match value.as_str() {
            "pathway_vine" => Ok(Some(VineType::PathwayVine)),
            "narrative_vine" => Ok(Some(VineType::NarrativeVine)),
            _ => Err(anyhow!("invalid {}", field)),
        },
        Value::Number(value) => match value.as_u64() {
            Some(0) => Ok(Some(VineType::PathwayVine)),
            Some(1) => Ok(Some(VineType::NarrativeVine)),
            _ => Err(anyhow!("invalid {}", field)),
        },
        _ => Err(anyhow!("invalid {}", field)),
    }
}

fn parse_tier_enum_field(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<TierEnum> {
    let value = payload
        .get(field)
        .ok_or_else(|| anyhow!("missing {}", field))?;
    parse_tier_enum_value(value, field)
}

fn parse_tier_enum_value(value: &Value, field: &str) -> Result<TierEnum> {
    match value {
        Value::String(value) => match value.as_str() {
            "title" => Ok(TierEnum::Title),
            "sentence" => Ok(TierEnum::Sentence),
            "paragraph" => Ok(TierEnum::Paragraph),
            "full" => Ok(TierEnum::Full),
            _ => Err(anyhow!("invalid {}", field)),
        },
        Value::Number(value) => match value.as_u64() {
            Some(0) => Ok(TierEnum::Title),
            Some(1) => Ok(TierEnum::Sentence),
            Some(2) => Ok(TierEnum::Paragraph),
            Some(3) => Ok(TierEnum::Full),
            _ => Err(anyhow!("invalid {}", field)),
        },
        _ => Err(anyhow!("invalid {}", field)),
    }
}

fn parse_tier_complexity_field(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<i16> {
    let value = payload
        .get(field)
        .ok_or_else(|| anyhow!("missing {}", field))?;
    match value {
        Value::String(value) => match value.as_str() {
            "fundamental" => Ok(0),
            "standard" => Ok(1),
            "advanced" => Ok(2),
            "canonical" => Ok(3),
            _ => Err(anyhow!("invalid {}", field)),
        },
        Value::Number(value) => match value.as_u64() {
            Some(raw) if raw <= 3 => Ok(raw as i16),
            _ => Err(anyhow!("invalid {}", field)),
        },
        _ => Err(anyhow!("invalid {}", field)),
    }
}

fn parse_uuid_array_field(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Vec<Uuid>> {
    let value = payload
        .get(field)
        .ok_or_else(|| anyhow!("missing {}", field))?;
    let values = value
        .as_array()
        .ok_or_else(|| anyhow!("invalid {}", field))?;
    values
        .iter()
        .map(|value| {
            let value = value.as_str().ok_or_else(|| anyhow!("invalid {}", field))?;
            Uuid::parse_str(value).map_err(|_| anyhow!("invalid uuid in {}", field))
        })
        .collect()
}

fn parse_step_meta(
    payload: &serde_json::Map<String, Value>,
    item_count: usize,
) -> Result<Vec<Option<Uuid>>> {
    let expected = item_count.saturating_sub(1);
    let Some(value) = payload.get("step_meta") else {
        return Ok(vec![None; expected]);
    };
    let entries = value
        .as_array()
        .ok_or_else(|| anyhow!("invalid step_meta"))?;
    if entries.len() != expected {
        return Err(anyhow!(
            "invalid step_meta length: expected {}, got {}",
            expected,
            entries.len()
        ));
    }
    let mut out = Vec::with_capacity(entries.len());
    for entry in entries {
        let object = entry
            .as_object()
            .ok_or_else(|| anyhow!("invalid step_meta entry"))?;
        let via_connection_id = match object.get("via_connection_id") {
            None | Some(Value::Null) => None,
            Some(value) => {
                let value = value
                    .as_str()
                    .ok_or_else(|| anyhow!("invalid step_meta.via_connection_id"))?;
                Some(
                    Uuid::parse_str(value)
                        .map_err(|_| anyhow!("invalid uuid in step_meta.via_connection_id"))?,
                )
            }
        };
        out.push(via_connection_id);
    }
    Ok(out)
}

fn parse_initial_representation_refs(
    payload: &serde_json::Map<String, Value>,
) -> Result<PointerState> {
    let Some(value) = payload.get("initial_representation_refs") else {
        return Ok(PointerState::default());
    };
    let refs = value
        .as_object()
        .ok_or_else(|| anyhow!("invalid initial_representation_refs"))?;
    let title_representation_id = parse_optional_uuid_field(refs, "title_representation_id")?;
    let sentence_representation_id = parse_optional_uuid_field(refs, "sentence_representation_id")?;
    Ok(PointerState {
        title_representation_id,
        sentence_representation_id,
    })
}

fn parse_optional_uuid_field(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Option<Uuid>> {
    match payload.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => {
            let value = value.as_str().ok_or_else(|| anyhow!("invalid {}", field))?;
            let parsed =
                Uuid::parse_str(value).map_err(|_| anyhow!("invalid uuid in {}", field))?;
            Ok(Some(parsed))
        }
    }
}

fn parse_representation_pointer_updates(
    payload: &serde_json::Map<String, Value>,
) -> Result<Vec<RepresentationPointerUpdate>> {
    if let Some(value) = payload.get("representation_pointer_updates") {
        return parse_representation_pointer_updates_array(value);
    }
    if let Some(value) = payload.get("representation_selections") {
        return parse_representation_pointer_updates_array(value);
    }
    if let Some(value) = payload.get("representation_pointer_update") {
        return Ok(vec![parse_representation_pointer_update_value(value)?]);
    }
    if (payload.contains_key("representation_id")
        || payload.contains_key("selected_representation_id"))
        && (payload.contains_key("target_kind") || payload.contains_key("object_kind"))
        && (payload.contains_key("target_object_id") || payload.contains_key("object_id"))
    {
        return Ok(vec![parse_representation_pointer_update_from_object(
            payload,
        )?]);
    }
    Ok(Vec::new())
}

fn parse_representation_pointer_updates_array(
    value: &Value,
) -> Result<Vec<RepresentationPointerUpdate>> {
    let values = value
        .as_array()
        .ok_or_else(|| anyhow!("invalid representation pointer updates"))?;
    values
        .iter()
        .map(parse_representation_pointer_update_value)
        .collect()
}

fn parse_representation_pointer_update_value(value: &Value) -> Result<RepresentationPointerUpdate> {
    let payload = value
        .as_object()
        .ok_or_else(|| anyhow!("invalid representation pointer update"))?;
    parse_representation_pointer_update_from_object(payload)
}

fn parse_representation_pointer_update_from_object(
    payload: &serde_json::Map<String, Value>,
) -> Result<RepresentationPointerUpdate> {
    let target_kind = if payload.contains_key("target_kind") {
        parse_target_kind_field(payload, "target_kind")?
    } else {
        parse_target_kind_field(payload, "object_kind")?
    };
    let target_object_id = if payload.contains_key("target_object_id") {
        parse_uuid_field(payload, "target_object_id")?
    } else {
        parse_uuid_field(payload, "object_id")?
    };
    let tier_enum = if let Some(value) = payload.get("tier_enum") {
        parse_tier_enum_value(value, "tier_enum")?
    } else {
        parse_tier_enum_field(payload, "tier_length")?
    };
    let representation_id = if let Some(value) = payload.get("selected_representation_id") {
        let value = value
            .as_str()
            .ok_or_else(|| anyhow!("invalid selected_representation_id"))?;
        Uuid::parse_str(value).map_err(|_| anyhow!("invalid selected_representation_id"))?
    } else {
        parse_uuid_field(payload, "representation_id")?
    };
    Ok(RepresentationPointerUpdate {
        target_kind,
        target_object_id,
        tier_enum,
        representation_id,
    })
}

fn apply_pointer_update(pointers: &mut PointerState, update: &RepresentationPointerUpdate) {
    match update.tier_enum {
        TierEnum::Title => {
            pointers.title_representation_id = Some(update.representation_id);
        }
        TierEnum::Sentence => {
            pointers.sentence_representation_id = Some(update.representation_id);
        }
        TierEnum::Paragraph | TierEnum::Full => {}
    }
}

fn extract_seed_identity(events: &[SeedEvent]) -> Result<(Uuid, Option<Uuid>)> {
    for event in events {
        if event.kind == "identity_create" {
            let payload = payload_object(&event.payload)?;
            let identity_id = parse_uuid_field(payload, "identity_id")?;
            let speaker = event
                .speaker_identity_id
                .ok_or_else(|| anyhow!("identity_create missing speaker_identity_id"))?;
            if speaker != identity_id {
                return Err(anyhow!(
                    "identity_create speaker_identity_id mismatch: {} != {}",
                    speaker,
                    identity_id
                ));
            }
            return Ok((identity_id, Some(event.id)));
        }
    }

    for event in events {
        if let Some(speaker) = event.speaker_identity_id {
            return Ok((speaker, Some(event.id)));
        }
    }

    Err(anyhow!(
        "unable to determine seed_identity_id from seed events"
    ))
}

fn extract_seed_identity_title(events: &[SeedEvent], seed_identity_id: Uuid) -> Result<String> {
    for event in events {
        if event.kind != "identity_create" {
            continue;
        }
        let payload = payload_object(&event.payload)?;
        let identity_id = parse_uuid_field(payload, "identity_id")?;
        if identity_id != seed_identity_id {
            continue;
        }
        let title = parse_string_field(payload, "title")?.trim();
        if title.is_empty() {
            return Err(anyhow!("identity_create title is empty"));
        }
        return Ok(title.to_string());
    }

    Ok("seed bootstrap identity".to_string())
}

fn ensure_seed_identity_cluster_events(
    events: &mut Vec<SeedEvent>,
    seed_identity_id: Uuid,
    seed_identity_title: &str,
) -> Result<()> {
    let organizer_specs: [(&str, String); 5] = [
        (
            "mind_garden",
            format!("{}'s Mind Garden", seed_identity_title),
        ),
        (
            "backyard_of_ideas",
            format!("{}'s Backyard of Ideas", seed_identity_title),
        ),
        ("self_tree", format!("{}'s Self Tree", seed_identity_title)),
        ("anthill", format!("{}'s Anthill", seed_identity_title)),
        (
            "saved_ideas",
            format!("{}'s Saved Ideas", seed_identity_title),
        ),
    ];

    let mut occupied_event_ids: HashSet<Uuid> = events.iter().map(|event| event.id).collect();
    let mut occupied_idea_ids: HashSet<Uuid> = HashSet::new();
    let mut occupied_connection_ids: HashSet<Uuid> = HashSet::new();
    let mut idea_id_by_normalized_title = std::collections::HashMap::new();
    let mut has_identity_idea = false;
    let mut membership_edges: HashSet<(Uuid, Uuid, String)> = HashSet::new();

    for event in events.iter() {
        match event.kind.as_str() {
            "idea_create" => {
                let payload = payload_object(&event.payload)?;
                let idea_id = parse_uuid_field(payload, "idea_id")?;
                let title = parse_string_field(payload, "title")?;
                occupied_idea_ids.insert(idea_id);
                if idea_id == seed_identity_id {
                    has_identity_idea = true;
                }
                idea_id_by_normalized_title
                    .entry(normalize_title(title))
                    .or_insert(idea_id);
            }
            "connection_create" => {
                let payload = payload_object(&event.payload)?;
                let connection_id = parse_uuid_field(payload, "connection_id")?;
                occupied_connection_ids.insert(connection_id);
                let connection_type = parse_string_field(payload, "connection_type")?;
                if connection_type == "membership" {
                    let from_idea_id = parse_uuid_field(payload, "from_idea_id")?;
                    let to_idea_id = parse_uuid_field(payload, "to_idea_id")?;
                    let usage = optional_string_field(payload, "usage")?
                        .unwrap_or_default()
                        .to_lowercase();
                    membership_edges.insert((from_idea_id, to_idea_id, usage));
                }
            }
            _ => {}
        }
    }

    if !has_identity_idea {
        let identity_event_id = next_deterministic_uuid_v7(
            &format!("seed-owner-identity-idea-event:{}", seed_identity_id),
            &mut occupied_event_ids,
        );
        let identity_payload_hash =
            payload_hash_hex(seed_identity_title, seed_identity_title, None, None)
                .map_err(|err| anyhow!(err))?;

        events.push(SeedEvent {
            id: identity_event_id,
            kind: "idea_create".to_string(),
            speaker_identity_id: Some(seed_identity_id),
            payload: serde_json::json!({
                "idea_id": seed_identity_id,
                "idea_type": "identity",
                "speaker_identity_id": seed_identity_id,
                "title": seed_identity_title,
                "sentence": seed_identity_title,
                "paragraph": Value::Null,
                "full": Value::Null,
                "payload_hash": identity_payload_hash
            }),
        });

        occupied_idea_ids.insert(seed_identity_id);
        idea_id_by_normalized_title.insert(normalize_title(seed_identity_title), seed_identity_id);
    }

    let mut organizer_ids = Vec::with_capacity(organizer_specs.len());
    for (slug, organizer_title) in organizer_specs {
        let normalized_title = normalize_title(&organizer_title);
        let organizer_id =
            if let Some(existing) = idea_id_by_normalized_title.get(&normalized_title) {
                *existing
            } else {
                let organizer_idea_id = next_deterministic_uuid_v7(
                    &format!("seed-owner-organizer-idea-id:{}:{}", seed_identity_id, slug),
                    &mut occupied_idea_ids,
                );
                let organizer_event_id = next_deterministic_uuid_v7(
                    &format!(
                        "seed-owner-organizer-idea-event:{}:{}",
                        seed_identity_id, slug
                    ),
                    &mut occupied_event_ids,
                );
                let organizer_payload_hash =
                    payload_hash_hex(&organizer_title, &organizer_title, None, None)
                        .map_err(|err| anyhow!(err))?;

                events.push(SeedEvent {
                    id: organizer_event_id,
                    kind: "idea_create".to_string(),
                    speaker_identity_id: Some(seed_identity_id),
                    payload: serde_json::json!({
                        "idea_id": organizer_idea_id,
                        "idea_type": "conceptual_idea",
                        "speaker_identity_id": seed_identity_id,
                        "title": organizer_title,
                        "sentence": organizer_title,
                        "paragraph": Value::Null,
                        "full": Value::Null,
                        "payload_hash": organizer_payload_hash
                    }),
                });

                idea_id_by_normalized_title.insert(normalized_title, organizer_idea_id);
                organizer_idea_id
            };

        organizer_ids.push((slug.to_string(), organizer_id));
    }

    for (slug, organizer_id) in organizer_ids {
        let edges = [
            (seed_identity_id, organizer_id, "has_space"),
            (organizer_id, seed_identity_id, "space_of"),
        ];
        for (from_idea_id, to_idea_id, usage) in edges {
            let usage_key = usage.to_string();
            if membership_edges.contains(&(from_idea_id, to_idea_id, usage_key.clone())) {
                continue;
            }

            let connection_id = next_deterministic_uuid_v7(
                &format!(
                    "seed-owner-organizer-connection-id:{}:{}:{}:{}",
                    seed_identity_id, slug, from_idea_id, to_idea_id
                ),
                &mut occupied_connection_ids,
            );
            let connection_event_id = next_deterministic_uuid_v7(
                &format!(
                    "seed-owner-organizer-connection-event:{}:{}:{}:{}",
                    seed_identity_id, slug, from_idea_id, to_idea_id
                ),
                &mut occupied_event_ids,
            );

            events.push(SeedEvent {
                id: connection_event_id,
                kind: "connection_create".to_string(),
                speaker_identity_id: Some(seed_identity_id),
                payload: serde_json::json!({
                    "connection_id": connection_id,
                    "from_idea_id": from_idea_id,
                    "to_idea_id": to_idea_id,
                    "connection_type": "membership",
                    "usage": usage
                }),
            });

            membership_edges.insert((from_idea_id, to_idea_id, usage_key));
        }
    }

    Ok(())
}

fn normalize_title(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn next_deterministic_uuid_v7(base: &str, occupied: &mut HashSet<Uuid>) -> Uuid {
    let mut index: u32 = 0;
    loop {
        let candidate_key = if index == 0 {
            base.to_string()
        } else {
            format!("{}:{}", base, index)
        };
        let candidate = deterministic_uuid_v7(&candidate_key);
        if occupied.insert(candidate) {
            return candidate;
        }
        index += 1;
    }
}

fn deterministic_uuid_v7(seed: &str) -> Uuid {
    let digest = Sha256::digest(seed.as_bytes());
    let mut bytes = [0_u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x70;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}

fn enforce_seed_identity(events: &[SeedEvent], seed_identity_id: Uuid) -> Result<()> {
    for event in events {
        match event.kind.as_str() {
            "idea_create"
            | "connection_create"
            | "rail_create"
            | "rail_fork"
            | "representation_create"
            | "rail_update_representation"
            | "challenge_create"
            | "challenge_open_arguments"
            | "challenge_close_arguments"
            | "challenge_open_voting"
            | "challenge_close_voting"
            | "challenge_cancel"
            | "challenge_supersede"
            | "challenge_finalize_verdict"
            | "vote_cast" => {
                let speaker = event
                    .speaker_identity_id
                    .ok_or_else(|| anyhow!("event missing speaker_identity_id: {}", event.id))?;
                if speaker != seed_identity_id {
                    return Err(anyhow!(
                        "speaker_identity_id mismatch for event {}",
                        event.id
                    ));
                }
            }
            "cycle_close" | "snapshot_commit" => {
                let speaker = event
                    .speaker_identity_id
                    .ok_or_else(|| anyhow!("event missing speaker_identity_id: {}", event.id))?;
                if speaker != system_boundary_emitter_id() {
                    return Err(anyhow!(
                        "boundary event {} must be authored by system_boundary_emitter",
                        event.id
                    ));
                }
            }
            "identity_create" => {
                let payload = payload_object(&event.payload)?;
                let identity_id = parse_uuid_field(payload, "identity_id")?;
                let speaker = event
                    .speaker_identity_id
                    .ok_or_else(|| anyhow!("identity_create missing speaker_identity_id"))?;
                if identity_id != seed_identity_id || speaker != seed_identity_id {
                    return Err(anyhow!("identity_create mismatch for event {}", event.id));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn v7(id: &str) -> Uuid {
        Uuid::parse_str(id).expect("uuid parse")
    }

    #[test]
    fn ensures_seed_identity_cluster_events_when_missing() {
        let seed_identity_id = v7("380b7817-db3b-7b76-8cf3-87df879ddddb");
        let mut events = vec![SeedEvent {
            id: v7("27e74d2a-4f97-73ed-83d8-d3499f67f307"),
            kind: "identity_create".to_string(),
            speaker_identity_id: Some(seed_identity_id),
            payload: json!({
                "identity_id": seed_identity_id,
                "title": "kind gulag dehl"
            }),
        }];

        ensure_seed_identity_cluster_events(&mut events, seed_identity_id, "kind gulag dehl")
            .expect("cluster build");
        let len_after_first = events.len();
        ensure_seed_identity_cluster_events(&mut events, seed_identity_id, "kind gulag dehl")
            .expect("idempotent cluster build");
        assert_eq!(events.len(), len_after_first);

        let mut idea_titles = std::collections::HashSet::new();
        let mut membership_edges = std::collections::HashSet::new();
        for event in &events {
            match event.kind.as_str() {
                "idea_create" => {
                    let payload = payload_object(&event.payload).expect("idea payload");
                    idea_titles.insert(
                        parse_string_field(payload, "title")
                            .expect("idea title")
                            .to_string(),
                    );
                }
                "connection_create" => {
                    let payload = payload_object(&event.payload).expect("connection payload");
                    let from = parse_uuid_field(payload, "from_idea_id").expect("from idea");
                    let to = parse_uuid_field(payload, "to_idea_id").expect("to idea");
                    let usage = optional_string_field(payload, "usage")
                        .expect("usage")
                        .unwrap_or_default();
                    membership_edges.insert((from, to, usage));
                }
                _ => {}
            }
        }

        for expected_title in [
            "kind gulag dehl",
            "kind gulag dehl's Mind Garden",
            "kind gulag dehl's Backyard of Ideas",
            "kind gulag dehl's Self Tree",
            "kind gulag dehl's Anthill",
            "kind gulag dehl's Saved Ideas",
        ] {
            assert!(idea_titles.contains(expected_title));
        }

        let mut organizer_ids = Vec::new();
        for event in &events {
            if event.kind != "idea_create" {
                continue;
            }
            let payload = payload_object(&event.payload).expect("idea payload");
            let title = parse_string_field(payload, "title").expect("title");
            if title == "kind gulag dehl" {
                continue;
            }
            organizer_ids.push(parse_uuid_field(payload, "idea_id").expect("idea id"));
        }

        assert_eq!(organizer_ids.len(), 5);
        for organizer_id in organizer_ids {
            assert!(membership_edges.contains(&(
                seed_identity_id,
                organizer_id,
                "has_space".to_string()
            )));
            assert!(membership_edges.contains(&(
                organizer_id,
                seed_identity_id,
                "space_of".to_string()
            )));
        }
    }
}
