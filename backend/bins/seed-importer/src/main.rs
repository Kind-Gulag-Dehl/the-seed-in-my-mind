use anyhow::{anyhow, Context, Result};
use encoding::payload::payload_hash_hex;
use event_log::validation::validate_legacy_import_event;
use event_log::Event;
use event_log::{SYSTEM_BOUNDARY_EMITTER_ID_STR, SYSTEM_BOUNDARY_EMITTER_TITLE};
use serde::Deserialize;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgPoolOptions, Postgres, Transaction};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use uuid::Uuid;

const V4_PILOT_SCHEMA_VERSION: &str = "seed-v4-pilot-manifest-v1";
const V4_PILOT_ARTIFACT_TYPE: &str = "unsigned_noncanonical_isolated_local_pilot";
const V4_PILOT_OPEN_CORE_BASELINE: &str = "6068072160fc032eb1ec3b7641cb917c38f08776";
const V4_PILOT_PRIVATE_BASELINE: &str = "ce8b0c6ac4dc97f155dda83fb9ed19bd735819fd";
const V4_PILOT_UUID_DOMAIN: &str = "seed.v4.pilot.uuidv7.v1";
const V4_PILOT_SPEAKER_ID: &str = "380b7817-db3b-7b76-8cf3-87df879ddddb";
const V4_PILOT_IDEA_COUNT: usize = 50;
const V4_PILOT_REPRESENTATION_COUNT: usize = 600;
const V4_PILOT_COMPONENT_FILES: [&str; 15] = [
    "seed-v4-pilot-selection.v1.json",
    "seed-v4-pilot-generator-provenance.v1.json",
    "seed-v4-pilot-source-provenance.v1.json",
    "seed-v4-pilot-ideas.v1.json",
    "seed-v4-pilot-representations.v1.json",
    "seed-v4-pilot-connections.v1.json",
    "seed-v4-pilot-orderings.v1.json",
    "seed-v4-pilot-importance-contexts-and-arguments.v1.json",
    "seed-v4-pilot-derived-importance.v1.json",
    "seed-v4-pilot-document-reconstruction.v1.json",
    "seed-v4-pilot-authored-event-templates.v1.json",
    "seed-v4-pilot-import-projection.v1.json",
    "seed-v4-pilot-mechanical-validation.v1.json",
    "seed-v4-pilot-semantic-evaluation.v1.json",
    "seed-v4-pilot-review-and-readable-projection.v1.json",
];
const UNIVERSAL_ORIENTATIONS: [&str; 4] = [
    "important_to_current_individual",
    "important_for_current_individual",
    "important_to_collective",
    "important_for_collective",
];
const IMPORTANCE_HORIZONS: [&str; 5] = [
    "near_term",
    "mid_term",
    "long_term",
    "very_long_term",
    "trans_generational",
];
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
    validate_only: bool,
}

#[derive(Debug, Deserialize)]
struct PilotManifest {
    schema_version: String,
    artifact_type: String,
    frozen_at: String,
    package_domain: String,
    status: String,
    finalized: bool,
    signed: bool,
    canonical: bool,
    import_authorized: bool,
    owner_accepted: bool,
    owner_review: PilotOwnerReview,
    baseline_authority: PilotBaseline,
    component_count: usize,
    non_manifest_component_digest_sha256: String,
    components: Vec<PilotComponent>,
    manifest_payload_sha256: String,
}

#[derive(Debug, Deserialize)]
struct PilotBaseline {
    private_product: String,
    open_core: String,
    v3_pair_status: String,
}

#[derive(Debug, Deserialize)]
struct PilotOwnerReview {
    completed: usize,
    required: usize,
}

#[derive(Debug, Deserialize)]
struct PilotComponent {
    file: String,
    byte_length: u64,
    sha256: String,
}

#[derive(Debug, PartialEq, Eq)]
struct PilotValidationSummary {
    idea_count: usize,
    description_count: usize,
    connection_count: usize,
    ordering_count: usize,
    universal_profile_count: usize,
    relative_lens_count: usize,
    component_hash_set_digest_sha256: String,
}

#[derive(Debug)]
struct UniversalProfileProjection {
    idea_id: Uuid,
    aggregate_score: i64,
    horizon_subtotals: BTreeMap<String, i64>,
    selection_index: i64,
    cumulative_rank: i64,
}

#[derive(Debug)]
struct PilotImportanceProfile {
    aggregate_score: Option<i64>,
    horizon_subtotals: BTreeMap<String, Option<i64>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TargetKind {
    Idea,
    Ordering,
}

impl TargetKind {
    fn as_i16(self) -> i16 {
        match self {
            TargetKind::Idea => 0,
            TargetKind::Ordering => 1,
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
enum OrderingProfile {
    Vine,
    EvidenceRail,
    ActionRail,
}

impl OrderingProfile {
    fn as_i16(self) -> i16 {
        match self {
            OrderingProfile::Vine => 0,
            OrderingProfile::EvidenceRail => 1,
            OrderingProfile::ActionRail => 2,
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
    tier_complexity: Option<i16>,
}

#[derive(Debug, Clone)]
struct RepresentationPointerUpdate {
    target_kind: TargetKind,
    target_object_id: Uuid,
    tier_enum: TierEnum,
    tier_complexity: Option<i16>,
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
    let document: Value = serde_json::from_str(&contents)
        .with_context(|| format!("parsing seed JSON from {}", seed_path.display()))?;

    if document.get("schema_version").and_then(Value::as_str) == Some(V4_PILOT_SCHEMA_VERSION) {
        require_v4_pilot_validate_only(&options)?;
        let summary = validate_v4_pilot_package(&seed_path, &contents, document)?;
        println!(
            "seed-importer: validate-only pass canonical_writes=0 status=unsigned_noncanonical ideas={} representations={} connections={} orderings={} universal_profiles={} relative_contexts={} non_manifest_component_digest_sha256={}",
            summary.idea_count,
            summary.description_count,
            summary.connection_count,
            summary.ordering_count,
            summary.universal_profile_count,
            summary.relative_lens_count,
            summary.component_hash_set_digest_sha256
        );
        return Ok(());
    }

    if options.validate_only {
        return Err(anyhow!(
            "--validate-only accepts only schema_version={}",
            V4_PILOT_SCHEMA_VERSION
        ));
    }

    let seed: SeedFile = serde_json::from_value(document)
        .with_context(|| format!("parsing legacy seed JSON from {}", seed_path.display()))?;

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
    let mut seen_ordering_ids = HashSet::new();
    let mut seen_representation_ids = HashSet::new();
    let mut seen_identity_ids = HashSet::new();
    let mut seen_idea_ids = HashSet::new();
    let mut idea_rows = Vec::new();
    let mut connection_rows = Vec::new();
    let mut ordering_rows = Vec::new();
    let mut ordering_item_rows = Vec::new();
    let mut representation_rows = Vec::new();
    let mut cycle_boundary_rows = Vec::new();
    let mut snapshot_commit_rows = Vec::new();
    let mut tempo_rows: Vec<(i64, i32, TempoPredicateInput)> = Vec::new();
    let mut representation_keys: HashMap<Uuid, RepresentationKey> = HashMap::new();
    let mut idea_pointer_updates: HashMap<Uuid, PointerState> = HashMap::new();
    let mut ordering_pointer_updates: HashMap<Uuid, PointerState> = HashMap::new();

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

        validate_legacy_import_event(&stage0_event)
            .map_err(|err| anyhow!("event validation failed event_id={} {}", event.id, err))?;

        insert_event(&mut tx, 1, event_index, &stage0_event).await?;
        if let Some(tempo) = extract_tempo_predicates(&stage0_event.payload)? {
            tempo_rows.push((1, event_index, tempo));
        }

        match stage0_event.kind.as_str() {
            "identity_create" => {
                let payload = payload_object(&stage0_event.payload)?;
                let identity_id = parse_uuid_field(payload, "identity_id")?;
                if !seen_identity_ids.insert(identity_id) {
                    return Err(anyhow!(
                        "duplicate identity_id in seed file: {}",
                        identity_id
                    ));
                }
            }
            "idea_create" => {
                let payload = payload_object(&stage0_event.payload)?;
                let idea_id = parse_uuid_field(payload, "idea_id")?;
                if !seen_idea_ids.insert(idea_id) {
                    return Err(anyhow!("duplicate idea_id in seed file: {}", idea_id));
                }
                let speaker = stage0_event
                    .speaker_identity_id
                    .ok_or_else(|| anyhow!("missing speaker_identity_id for idea_create"))?;

                let is_identity_idea = idea_id == seed_identity_id;
                let payload_idea_type = parse_string_field(payload, "idea_type")?;
                let idea_type = if is_identity_idea {
                    "identity".to_string()
                } else {
                    payload_idea_type.to_string()
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
            "ordering_create" => {
                let payload = payload_object(&stage0_event.payload)?;
                let ordering_id = parse_uuid_field(payload, "ordering_id")?;
                if !seen_ordering_ids.insert(ordering_id) {
                    return Err(anyhow!(
                        "duplicate ordering_id in seed file: {}",
                        ordering_id
                    ));
                }

                let ordering_profile = parse_ordering_profile_field(payload, "ordering_profile")?;
                let vine_type = parse_vine_type_field(
                    payload,
                    "vine_type",
                    matches!(ordering_profile, OrderingProfile::Vine),
                )?;
                if ordering_profile != OrderingProfile::Vine && vine_type.is_some() {
                    return Err(anyhow!("vine_type is only valid for vine ordering_profile"));
                }
                let speaker_identity_id = stage0_event
                    .speaker_identity_id
                    .ok_or_else(|| anyhow!("missing speaker_identity_id for ordering_create"))?;
                let item_idea_ids = parse_uuid_array_field(payload, "item_idea_ids")?;
                let subject_idea_id =
                    parse_ordering_subject(payload, ordering_profile, &idea_rows)?;
                let item_roles =
                    parse_ordering_item_roles(payload, ordering_profile, item_idea_ids.len())?;
                let step_meta = parse_step_meta(payload, item_idea_ids.len())?;
                let initial_refs = parse_initial_representation_refs(payload)?;

                ordering_rows.push(OrderingRow {
                    ordering_id,
                    ordering_profile: ordering_profile.as_i16(),
                    vine_type: vine_type.map(VineType::as_i16),
                    subject_idea_id,
                    speaker_identity_id,
                    created_block_height: 1,
                    created_event_index: event_index,
                    created_event_id: stage0_event.id,
                    base_ordering_id: None,
                    title_representation_id: initial_refs.title_representation_id,
                    sentence_representation_id: initial_refs.sentence_representation_id,
                });
                if initial_refs.title_representation_id.is_some()
                    || initial_refs.sentence_representation_id.is_some()
                {
                    ordering_pointer_updates.insert(ordering_id, initial_refs);
                }

                for (idx, idea_id) in item_idea_ids.into_iter().enumerate() {
                    let via_connection_id = if idx == 0 {
                        None
                    } else {
                        step_meta.get(idx - 1).copied().flatten()
                    };
                    ordering_item_rows.push(OrderingItemRow {
                        ordering_id,
                        idx: idx as i32,
                        idea_id,
                        item_role: item_roles[idx],
                        via_connection_id,
                    });
                }
            }
            "ordering_fork" => {
                let payload = payload_object(&stage0_event.payload)?;
                let ordering_id = parse_uuid_field(payload, "ordering_id")?;
                if !seen_ordering_ids.insert(ordering_id) {
                    return Err(anyhow!(
                        "duplicate ordering_id in seed file: {}",
                        ordering_id
                    ));
                }

                let base_ordering_id = parse_uuid_field(payload, "base_ordering_id")?;
                let base = ordering_rows
                    .iter()
                    .find(|row| row.ordering_id == base_ordering_id)
                    .ok_or_else(|| {
                        anyhow!(
                            "ordering_fork base_ordering_id not found: {}",
                            base_ordering_id
                        )
                    })?;
                let ordering_profile = parse_ordering_profile_field(payload, "ordering_profile")?;
                if ordering_profile.as_i16() != base.ordering_profile {
                    return Err(anyhow!(
                        "ordering_fork ordering_profile differs from base ordering"
                    ));
                }
                let supplied_vine_type = parse_vine_type_field(payload, "vine_type", false)?;
                if ordering_profile != OrderingProfile::Vine && supplied_vine_type.is_some() {
                    return Err(anyhow!("vine_type is only valid for vine ordering_profile"));
                }
                let vine_type = supplied_vine_type.map(VineType::as_i16).or(base.vine_type);
                let base_subject_idea_id = base.subject_idea_id;
                let speaker_identity_id = stage0_event
                    .speaker_identity_id
                    .ok_or_else(|| anyhow!("missing speaker_identity_id for ordering_fork"))?;
                let item_idea_ids = parse_uuid_array_field(payload, "item_idea_ids")?;
                let subject_idea_id =
                    parse_ordering_subject(payload, ordering_profile, &idea_rows)?;
                if subject_idea_id != base_subject_idea_id {
                    return Err(anyhow!(
                        "ordering_fork subject_idea_id differs from base ordering"
                    ));
                }
                let item_roles =
                    parse_ordering_item_roles(payload, ordering_profile, item_idea_ids.len())?;
                let base_roles = ordering_item_rows
                    .iter()
                    .filter(|row| row.ordering_id == base_ordering_id)
                    .map(|row| (row.idea_id, row.item_role))
                    .collect::<HashMap<_, _>>();
                for (idea_id, item_role) in item_idea_ids.iter().zip(item_roles.iter()) {
                    if let Some(base_role) = base_roles.get(idea_id) {
                        if base_role != item_role {
                            return Err(anyhow!(
                                "ordering_fork retained item changed role idea_id={}",
                                idea_id
                            ));
                        }
                    }
                }
                if ordering_profile == OrderingProfile::ActionRail {
                    let base_lane = base_roles.values().next().copied().flatten();
                    let fork_lane = item_roles.first().copied().flatten();
                    if base_lane.is_none() || base_lane != fork_lane {
                        return Err(anyhow!(
                            "ordering_fork Action Rail lane differs from base ordering"
                        ));
                    }
                }
                let step_meta = parse_step_meta(payload, item_idea_ids.len())?;
                let initial_refs = parse_initial_representation_refs(payload)?;

                ordering_rows.push(OrderingRow {
                    ordering_id,
                    ordering_profile: ordering_profile.as_i16(),
                    vine_type,
                    subject_idea_id,
                    speaker_identity_id,
                    created_block_height: 1,
                    created_event_index: event_index,
                    created_event_id: stage0_event.id,
                    base_ordering_id: Some(base_ordering_id),
                    title_representation_id: initial_refs.title_representation_id,
                    sentence_representation_id: initial_refs.sentence_representation_id,
                });
                if initial_refs.title_representation_id.is_some()
                    || initial_refs.sentence_representation_id.is_some()
                {
                    ordering_pointer_updates.insert(ordering_id, initial_refs);
                }

                for (idx, idea_id) in item_idea_ids.into_iter().enumerate() {
                    let via_connection_id = if idx == 0 {
                        None
                    } else {
                        step_meta.get(idx - 1).copied().flatten()
                    };
                    ordering_item_rows.push(OrderingItemRow {
                        ordering_id,
                        idx: idx as i32,
                        idea_id,
                        item_role: item_roles[idx],
                        via_connection_id,
                    });
                }
            }
            "representation_create" => {
                let (representation_row, representation_key) = project_representation_row(
                    &stage0_event,
                    event_index,
                    &mut seen_representation_ids,
                    &seen_identity_ids,
                    &seen_idea_ids,
                )?;
                let representation_id = representation_row.representation_id;
                representation_rows.push(representation_row);
                representation_keys.insert(representation_id, representation_key);
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
                        || key.tier_complexity != update.tier_complexity
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
                        TargetKind::Ordering => {
                            if !ordering_rows
                                .iter()
                                .any(|row| row.ordering_id == update.target_object_id)
                            {
                                return Err(anyhow!(
                                    "challenge_finalize_verdict target ordering missing: {}",
                                    update.target_object_id
                                ));
                            }
                            let pointers = ordering_pointer_updates
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

    for row in &mut ordering_rows {
        if let Some(pointers) = ordering_pointer_updates.get(&row.ordering_id) {
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
    insert_orderings(&mut tx, &ordering_rows).await?;
    insert_ordering_items(&mut tx, &ordering_item_rows).await?;

    tx.commit().await?;

    println!(
        "seed-importer: imported events={} ideas={} connections={} orderings={} representations={}",
        canonical_events.len(),
        idea_rows.len(),
        connection_rows.len(),
        ordering_rows.len(),
        representation_rows.len()
    );

    Ok(())
}

fn require_v4_pilot_validate_only(options: &Options) -> Result<()> {
    if !options.validate_only {
        return Err(anyhow!(
            "unsigned noncanonical V4 pilot packages are validate-only and are ineligible for canonical import, signing, or genesis"
        ));
    }
    if options.force {
        return Err(anyhow!("--force cannot be combined with --validate-only"));
    }
    Ok(())
}

fn validate_v4_pilot_package(
    manifest_path: &Path,
    manifest_text: &str,
    document: Value,
) -> Result<PilotValidationSummary> {
    let manifest: PilotManifest =
        serde_json::from_value(document).context("parsing V4 pilot validation manifest")?;

    if manifest.schema_version != V4_PILOT_SCHEMA_VERSION {
        return Err(anyhow!(
            "unsupported V4 pilot schema_version={}",
            manifest.schema_version
        ));
    }
    if manifest.artifact_type != V4_PILOT_ARTIFACT_TYPE {
        return Err(anyhow!(
            "invalid V4 pilot artifact_type={}",
            manifest.artifact_type
        ));
    }
    validate_pilot_package_status(&manifest)?;
    validate_manifest_payload_hash(manifest_text, &manifest.manifest_payload_sha256)?;

    let package_dir = manifest_path
        .parent()
        .ok_or_else(|| anyhow!("V4 pilot manifest must have a parent directory"))?;
    let expected_files: BTreeSet<&str> = V4_PILOT_COMPONENT_FILES.iter().copied().collect();
    let mut observed_files = BTreeSet::new();
    let mut component_values = BTreeMap::new();

    for component in &manifest.components {
        if !expected_files.contains(component.file.as_str()) {
            return Err(anyhow!(
                "unexpected V4 pilot component file={}",
                component.file
            ));
        }
        if !observed_files.insert(component.file.as_str()) {
            return Err(anyhow!(
                "duplicate V4 pilot component file={}",
                component.file
            ));
        }
        validate_sha256_hex(
            &component.sha256,
            &format!("component {} sha256", component.file),
        )?;
        let component_path = resolve_pilot_component_path(package_dir, &component.file)?;
        let bytes = fs::read(&component_path)
            .with_context(|| format!("reading V4 pilot component {}", component_path.display()))?;
        if bytes.len() as u64 != component.byte_length {
            return Err(anyhow!(
                "component {} byte_length mismatch: expected {}, got {}",
                component.file,
                component.byte_length,
                bytes.len()
            ));
        }
        let actual_hash = sha256_hex(&bytes);
        if actual_hash != component.sha256 {
            return Err(anyhow!(
                "component {} sha256 mismatch: expected {}, got {}",
                component.file,
                component.sha256,
                actual_hash
            ));
        }
        let value: Value = serde_json::from_slice(&bytes)
            .with_context(|| format!("parsing V4 pilot component {}", component.file))?;
        component_values.insert(component.file.clone(), value);
    }

    if observed_files != expected_files {
        let missing = expected_files
            .difference(&observed_files)
            .copied()
            .collect::<Vec<_>>();
        return Err(anyhow!(
            "missing required V4 pilot components: {}",
            missing.join(", ")
        ));
    }
    if manifest.component_count != manifest.components.len() + 1 {
        return Err(anyhow!(
            "component_count must include the manifest: expected {}, got {}",
            manifest.components.len() + 1,
            manifest.component_count
        ));
    }

    validate_sha256_hex(
        &manifest.non_manifest_component_digest_sha256,
        "non_manifest_component_digest_sha256",
    )?;
    let actual_component_digest = component_hash_set_digest(&manifest.components);
    if actual_component_digest != manifest.non_manifest_component_digest_sha256 {
        return Err(anyhow!(
            "non_manifest_component_digest_sha256 mismatch: expected {}, got {}",
            manifest.non_manifest_component_digest_sha256,
            actual_component_digest
        ));
    }

    let generator = require_component(
        &component_values,
        "seed-v4-pilot-generator-provenance.v1.json",
        "seed-v4-pilot-generator-provenance-v1",
    )?;
    validate_generator_provenance(generator, &manifest)?;

    let idea_records = pilot_component_records(
        require_component(
            &component_values,
            "seed-v4-pilot-ideas.v1.json",
            "seed-v4-pilot-ideas-v1",
        )?,
        "seed-v4-pilot-ideas.v1.json",
        "records",
        "count",
    )?;
    let idea_ids =
        validate_pilot_ideas(idea_records, &manifest.package_domain, &manifest.frozen_at)?;

    let ordering_records = pilot_component_records(
        require_component(
            &component_values,
            "seed-v4-pilot-orderings.v1.json",
            "seed-v4-pilot-orderings-v1",
        )?,
        "seed-v4-pilot-orderings.v1.json",
        "records",
        "count",
    )?;
    validate_pilot_orderings(
        ordering_records,
        &idea_ids,
        &manifest.package_domain,
        &manifest.frozen_at,
    )?;

    let representation_records = pilot_component_records(
        require_component(
            &component_values,
            "seed-v4-pilot-representations.v1.json",
            "seed-v4-pilot-representations-v1",
        )?,
        "seed-v4-pilot-representations.v1.json",
        "records",
        "count",
    )?;
    validate_pilot_representations(
        representation_records,
        &idea_ids,
        &manifest.package_domain,
        &manifest.frozen_at,
    )?;

    let connection_records = pilot_component_records(
        require_component(
            &component_values,
            "seed-v4-pilot-connections.v1.json",
            "seed-v4-pilot-connections-v1",
        )?,
        "seed-v4-pilot-connections.v1.json",
        "records",
        "count",
    )?;
    let connection_ids = validate_pilot_connections(
        connection_records,
        &idea_ids,
        &manifest.package_domain,
        &manifest.frozen_at,
    )?;

    let importance = require_component(
        &component_values,
        "seed-v4-pilot-importance-contexts-and-arguments.v1.json",
        "seed-v4-pilot-importance-contexts-and-arguments-v1",
    )?;
    let universal_records = component_array(importance, "universal_profiles", "importance")?;
    validate_declared_count(
        importance,
        "universal_profile_count",
        universal_records.len(),
        "importance",
    )?;
    let profile_values = validate_pilot_universal_profiles(universal_records, &idea_ids)?;
    let relative_records = component_array(importance, "relative_contexts", "importance")?;
    validate_declared_count(
        importance,
        "relative_context_count",
        relative_records.len(),
        "importance",
    )?;
    validate_pilot_relative_contexts(
        relative_records,
        &idea_ids,
        &connection_ids,
        &manifest.package_domain,
        &manifest.frozen_at,
    )?;

    let derived_records = pilot_component_records(
        require_component(
            &component_values,
            "seed-v4-pilot-derived-importance.v1.json",
            "seed-v4-pilot-derived-importance-v1",
        )?,
        "seed-v4-pilot-derived-importance.v1.json",
        "records",
        "count",
    )?;
    validate_pilot_derived_importance(derived_records, &profile_values)?;

    let event_templates = pilot_component_records(
        require_component(
            &component_values,
            "seed-v4-pilot-authored-event-templates.v1.json",
            "seed-v4-pilot-authored-event-templates-v1",
        )?,
        "seed-v4-pilot-authored-event-templates.v1.json",
        "records",
        "count",
    )?;
    validate_pilot_event_templates(
        event_templates,
        &idea_ids,
        &manifest.package_domain,
        &manifest.frozen_at,
    )?;

    let import_projection = require_component(
        &component_values,
        "seed-v4-pilot-import-projection.v1.json",
        "seed-v4-pilot-import-projection-v1",
    )?;
    validate_import_projection(import_projection)?;

    Ok(PilotValidationSummary {
        idea_count: idea_records.len(),
        description_count: representation_records.len(),
        connection_count: connection_records.len(),
        ordering_count: ordering_records.len(),
        universal_profile_count: universal_records.len(),
        relative_lens_count: relative_records.len(),
        component_hash_set_digest_sha256: actual_component_digest,
    })
}

fn validate_pilot_package_status(manifest: &PilotManifest) -> Result<()> {
    if manifest.finalized
        || manifest.signed
        || manifest.canonical
        || manifest.import_authorized
        || manifest.owner_accepted
        || manifest.owner_review.completed != 0
        || manifest.owner_review.required != V4_PILOT_IDEA_COUNT
        || manifest.status != "dependency_pending_50_of_50_owner_review_required"
    {
        return Err(anyhow!(
            "V4 pilot package must be unfinished, unsigned, noncanonical, import-ineligible, unaccepted, and pending 50/50 owner review"
        ));
    }
    if manifest.baseline_authority.open_core != V4_PILOT_OPEN_CORE_BASELINE
        || manifest.baseline_authority.private_product != V4_PILOT_PRIVATE_BASELINE
        || manifest.baseline_authority.v3_pair_status != "historical_only"
    {
        return Err(anyhow!("V4 pilot baseline authority mismatch"));
    }
    let expected_domain = format!(
        "the-seed-in-my-mind/seed-v4-pilot/{}/{}/{}",
        V4_PILOT_PRIVATE_BASELINE, V4_PILOT_OPEN_CORE_BASELINE, manifest.frozen_at
    );
    if manifest.package_domain != expected_domain {
        return Err(anyhow!("V4 pilot package_domain mismatch"));
    }
    Ok(())
}

fn validate_manifest_payload_hash(manifest_text: &str, declared: &str) -> Result<()> {
    validate_sha256_hex(declared, "manifest_payload_sha256")?;
    let compact = compact_json(manifest_text)?;
    let bound = format!("\"manifest_payload_sha256\":\"{}\"", declared);
    if compact.matches(&bound).count() != 1 {
        return Err(anyhow!(
            "manifest_payload_sha256 must occur exactly once as its top-level bound value"
        ));
    }
    let payload = compact.replacen(&bound, "\"manifest_payload_sha256\":null", 1);
    let actual = sha256_hex(payload.as_bytes());
    if actual != declared {
        return Err(anyhow!(
            "manifest_payload_sha256 mismatch: expected {}, got {}",
            declared,
            actual
        ));
    }
    Ok(())
}

fn compact_json(input: &str) -> Result<String> {
    let mut output = String::with_capacity(input.len());
    let mut in_string = false;
    let mut escaped = false;
    for character in input.chars() {
        if in_string {
            output.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
        } else if character == '"' {
            in_string = true;
            output.push(character);
        } else if !character.is_whitespace() {
            output.push(character);
        }
    }
    if in_string || escaped {
        return Err(anyhow!("manifest JSON string is unterminated"));
    }
    Ok(output)
}

fn resolve_pilot_component_path(package_dir: &Path, relative: &str) -> Result<PathBuf> {
    if relative.trim().is_empty() || relative.contains('\\') {
        return Err(anyhow!(
            "component path must be a non-empty portable relative path"
        ));
    }
    let path = Path::new(relative);
    if path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(anyhow!(
            "component path must stay within the V4 pilot package"
        ));
    }
    Ok(package_dir.join(path))
}

fn component_hash_set_digest(components: &[PilotComponent]) -> String {
    let mut ordered = components.iter().collect::<Vec<_>>();
    ordered.sort_by(|left, right| left.file.cmp(&right.file));
    let mut bytes = Vec::new();
    for component in ordered {
        bytes.extend_from_slice(format!("{}\t{}\n", component.file, component.sha256).as_bytes());
    }
    sha256_hex(&bytes)
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn validate_sha256_hex(value: &str, field: &str) -> Result<()> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(anyhow!(
            "{} must be 64 lowercase hexadecimal characters",
            field
        ));
    }
    Ok(())
}

fn require_component<'a>(
    components: &'a BTreeMap<String, Value>,
    file: &str,
    schema: &str,
) -> Result<&'a Map<String, Value>> {
    let component = components
        .get(file)
        .ok_or_else(|| anyhow!("required component {} is absent", file))?;
    let object = component
        .as_object()
        .ok_or_else(|| anyhow!("component {} must be a JSON object", file))?;
    if require_string(object, "schema_version", file)? != schema {
        return Err(anyhow!(
            "component {} schema_version must be {}",
            file,
            schema
        ));
    }
    Ok(object)
}

fn pilot_component_records<'a>(
    component: &'a Map<String, Value>,
    file: &str,
    records_field: &str,
    count_field: &str,
) -> Result<&'a [Value]> {
    let records = component
        .get(records_field)
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("component {} must contain {}", file, records_field))?;
    validate_declared_count(component, count_field, records.len(), file)?;
    Ok(records)
}

fn validate_declared_count(
    component: &Map<String, Value>,
    field: &str,
    actual: usize,
    context: &str,
) -> Result<()> {
    let count = require_i64(component, field, context)?;
    if count < 0 || count as usize != actual {
        return Err(anyhow!(
            "{}.{} count mismatch: declared {}, actual {}",
            context,
            field,
            count,
            actual
        ));
    }
    Ok(())
}

fn component_array<'a>(
    component: &'a Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<&'a [Value]> {
    component
        .get(field)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| anyhow!("{}.{} must be an array", context, field))
}

fn validate_generator_provenance(
    generator: &Map<String, Value>,
    manifest: &PilotManifest,
) -> Result<()> {
    if require_string(generator, "frozen_at", "generator provenance")? != manifest.frozen_at
        || require_string(generator, "package_domain", "generator provenance")?
            != manifest.package_domain
    {
        return Err(anyhow!("generator provenance package binding mismatch"));
    }
    let declaration =
        require_object_field(generator, "deterministic_uuid_v7", "generator provenance")?;
    if require_string(declaration, "domain", "deterministic_uuid_v7")? != V4_PILOT_UUID_DOMAIN
        || require_string(declaration, "timestamp_source", "deterministic_uuid_v7")?
            != manifest.frozen_at
        || require_string(declaration, "package_domain", "deterministic_uuid_v7")?
            != manifest.package_domain
    {
        return Err(anyhow!("deterministic UUIDv7 declaration mismatch"));
    }
    Ok(())
}

fn validate_pilot_ideas(
    records: &[Value],
    package_domain: &str,
    frozen_at: &str,
) -> Result<HashSet<Uuid>> {
    if records.len() != V4_PILOT_IDEA_COUNT {
        return Err(anyhow!(
            "ideas component must contain exactly {} records, got {}",
            V4_PILOT_IDEA_COUNT,
            records.len()
        ));
    }
    let mut ids = HashSet::new();
    let mut slugs = HashSet::new();
    let mut identity_seen = false;
    for (index, value) in records.iter().enumerate() {
        let context = format!("ideas.records[{}]", index);
        let record = require_object(value, &context)?;
        let idea_id = validate_uuid_v7(
            require_string(record, "runtime_idea_id", &context)?,
            &format!("{}.runtime_idea_id", context),
        )?;
        if !ids.insert(idea_id) {
            return Err(anyhow!("{} duplicate runtime_idea_id={}", context, idea_id));
        }
        match record.get("inherited_stable_slug") {
            Some(Value::String(stable_slug)) => {
                validate_slug(stable_slug, &format!("{}.inherited_stable_slug", context))?;
                if !slugs.insert(stable_slug.clone()) {
                    return Err(anyhow!(
                        "{} duplicate inherited_stable_slug={}",
                        context,
                        stable_slug
                    ));
                }
                validate_deterministic_uuid(
                    idea_id,
                    package_domain,
                    frozen_at,
                    "idea",
                    stable_slug,
                    &context,
                )?;
            }
            Some(Value::Null) if idea_id.to_string() == V4_PILOT_SPEAKER_ID => {
                identity_seen = true;
            }
            _ => {
                return Err(anyhow!(
                    "{} inherited_stable_slug must be a slug, except for the DEC-044 identity",
                    context
                ))
            }
        }
        let idea_type = require_string(record, "idea_type", &context)?;
        if !matches!(
            idea_type,
            "truth_claim" | "conceptual_idea" | "actionable_idea" | "action" | "identity"
        ) {
            return Err(anyhow!("{} invalid idea_type={}", context, idea_type));
        }
        let authorship_status = require_string(record, "authorship_status", &context)?;
        let valid_authorship_status = authorship_status == "speaker_attributed_not_human_authored"
            || (idea_id.to_string() == V4_PILOT_SPEAKER_ID
                && authorship_status
                    == "owner_decision_materialized_not_ordinary_per_record_human_authorship");
        if require_string(record, "speaker_identity_id", &context)? != V4_PILOT_SPEAKER_ID
            || !valid_authorship_status
        {
            return Err(anyhow!(
                "{} must retain Kind Gulag Dehl speaker attribution without a human-authorship claim",
                context
            ));
        }
        validate_provenance_and_review(record, &context)?;
        let review = require_object_field(record, "review_state", &context)?;
        if require_bool(
            review,
            "human_authorship_claimed",
            &format!("{}.review_state", context),
        )? || require_string(
            review,
            "owner_acceptance",
            &format!("{}.review_state", context),
        )? != "not_requested"
        {
            return Err(anyhow!("{} has invalid review/authorship status", context));
        }
        reject_live_rail_keys(value, &context)?;
    }
    if slugs.len() != V4_PILOT_IDEA_COUNT - 1 || !identity_seen {
        return Err(anyhow!(
            "ideas must contain 49 inherited slugs plus the fixed DEC-044 identity"
        ));
    }
    Ok(ids)
}

fn validate_pilot_representations(
    records: &[Value],
    idea_ids: &HashSet<Uuid>,
    package_domain: &str,
    frozen_at: &str,
) -> Result<()> {
    if records.len() != V4_PILOT_REPRESENTATION_COUNT {
        return Err(anyhow!(
            "representations component must contain exactly {} records",
            V4_PILOT_REPRESENTATION_COUNT
        ));
    }
    let mut ids = HashSet::new();
    let mut cells = HashMap::<Uuid, HashSet<(String, String)>>::new();
    for (index, value) in records.iter().enumerate() {
        let context = format!("representations.records[{}]", index);
        let record = require_object(value, &context)?;
        let representation_id = validate_uuid_v7(
            require_string(record, "representation_id", &context)?,
            &format!("{}.representation_id", context),
        )?;
        if !ids.insert(representation_id) {
            return Err(anyhow!(
                "{} duplicate representation_id={}",
                context,
                representation_id
            ));
        }
        let target_id = validate_uuid_v7(
            require_string(record, "target_idea_id", &context)?,
            &format!("{}.target_idea_id", context),
        )?;
        if !idea_ids.contains(&target_id) {
            return Err(anyhow!("{}.target_idea_id does not resolve", context));
        }
        let length = require_string(record, "length", &context)?;
        let complexity = require_string(record, "complexity", &context)?;
        if !matches!(length, "sentence" | "paragraph" | "full") {
            return Err(anyhow!("{} invalid length", context));
        }
        if !matches!(
            complexity,
            "fundamental" | "standard" | "advanced" | "canonical"
        ) {
            return Err(anyhow!("{} invalid complexity", context));
        }
        if !cells
            .entry(target_id)
            .or_default()
            .insert((complexity.to_string(), length.to_string()))
        {
            return Err(anyhow!("{} duplicate representation cell", context));
        }
        let stable_key = match record.get("inherited_stable_slug") {
            Some(Value::String(slug)) => slug.as_str(),
            Some(Value::Null) if target_id.to_string() == V4_PILOT_SPEAKER_ID => "kind-gulag-dehl",
            _ => return Err(anyhow!("{} has invalid inherited_stable_slug", context)),
        };
        validate_deterministic_uuid(
            representation_id,
            package_domain,
            frozen_at,
            "representation",
            &format!("{}/{}/{}", stable_key, complexity, length),
            &context,
        )?;
        let text = require_string(record, "text", &context)?;
        let limit = match length {
            "sentence" => 250,
            "paragraph" => 1250,
            "full" => 6250,
            _ => unreachable!("validated length"),
        };
        if text.trim().is_empty() || text.chars().count() > limit || text.contains('\r') {
            return Err(anyhow!(
                "{} text violates the {} runtime limit",
                context,
                length
            ));
        }
        let text_hash = require_string(record, "text_sha256", &context)?;
        if sha256_hex(text.as_bytes()) != text_hash {
            return Err(anyhow!("{} text_sha256 mismatch", context));
        }
        if !matches!(
            record.get("canonical_payload_hash_blake3"),
            Some(Value::Null)
        ) || require_string(record, "payload_hash_status", &context)?
            != "dependency_pending_profile_v0_reference_implementation"
        {
            return Err(anyhow!(
                "{} must retain an explicit unresolved canonical BLAKE3 payload hash",
                context
            ));
        }
        validate_provenance_and_review(record, &context)?;
        let review = require_object_field(record, "review_state", &context)?;
        if require_bool(
            review,
            "human_reviewed",
            &format!("{}.review_state", context),
        )? || require_bool(
            review,
            "human_authorship_claimed",
            &format!("{}.review_state", context),
        )? {
            return Err(anyhow!(
                "{} falsely claims human review/authorship",
                context
            ));
        }
        reject_live_rail_keys(value, &context)?;
    }
    if cells.len() != V4_PILOT_IDEA_COUNT || cells.values().any(|idea_cells| idea_cells.len() != 12)
    {
        return Err(anyhow!(
            "representations must provide all 12 complexity/length cells for each of 50 ideas"
        ));
    }
    Ok(())
}

fn validate_pilot_connections(
    records: &[Value],
    idea_ids: &HashSet<Uuid>,
    package_domain: &str,
    frozen_at: &str,
) -> Result<HashSet<Uuid>> {
    let mut ids = HashSet::new();
    for (index, value) in records.iter().enumerate() {
        let context = format!("connections.records[{}]", index);
        let record = require_object(value, &context)?;
        let connection_id = validate_uuid_v7(
            require_string(record, "connection_id", &context)?,
            &format!("{}.connection_id", context),
        )?;
        if !ids.insert(connection_id) {
            return Err(anyhow!(
                "{} duplicate connection_id={}",
                context,
                connection_id
            ));
        }
        validate_deterministic_uuid(
            connection_id,
            package_domain,
            frozen_at,
            "connection",
            require_string(record, "inherited_connection_id", &context)?,
            &context,
        )?;
        for field in ["source_idea_id", "target_idea_id"] {
            let idea_id = validate_uuid_v7(
                require_string(record, field, &context)?,
                &format!("{}.{}", context, field),
            )?;
            if !idea_ids.contains(&idea_id) {
                return Err(anyhow!("{}.{} does not resolve", context, field));
            }
        }
        if !matches!(
            require_string(record, "connection_type", &context)?,
            "same_as" | "membership" | "relative_importance"
        ) {
            return Err(anyhow!("{} invalid connection_type", context));
        }
        if require_string(record, "speaker_identity_id", &context)? != V4_PILOT_SPEAKER_ID {
            return Err(anyhow!("{} speaker identity mismatch", context));
        }
        validate_provenance_and_review(record, &context)?;
        reject_live_rail_keys(value, &context)?;
    }
    Ok(ids)
}

fn validate_pilot_orderings(
    records: &[Value],
    idea_ids: &HashSet<Uuid>,
    package_domain: &str,
    frozen_at: &str,
) -> Result<()> {
    if records.len() != 3 {
        return Err(anyhow!(
            "the V4 pilot must contain exactly three native Orderings"
        ));
    }
    let mut ids = HashSet::new();
    let mut pending_bases = Vec::new();
    for (index, value) in records.iter().enumerate() {
        let context = format!("orderings.records[{}]", index);
        let record = require_object(value, &context)?;
        let ordering_id = validate_uuid_v7(
            require_string(record, "ordering_id", &context)?,
            &format!("{}.ordering_id", context),
        )?;
        if !ids.insert(ordering_id) {
            return Err(anyhow!("{} duplicate ordering_id={}", context, ordering_id));
        }
        let event_type = require_string(record, "event_type", &context)?;
        let profile = require_string(record, "ordering_profile", &context)?;
        if profile != "vine" {
            return Err(anyhow!("{} invalid ordering_profile={}", context, profile));
        }
        let vine_type = require_string(record, "vine_type", &context)?;
        let stable_key = match (event_type, vine_type) {
            ("ordering_create", "narrative_vine") => "pilot-narrative-vine",
            ("ordering_create", "pathway_vine") => "pilot-pathway-vine",
            ("ordering_fork", "pathway_vine") => "pilot-pathway-vine-risk-fork",
            _ => return Err(anyhow!("{} invalid native Ordering/Vine shape", context)),
        };
        validate_deterministic_uuid(
            ordering_id,
            package_domain,
            frozen_at,
            "ordering",
            stable_key,
            &context,
        )?;
        let items = record
            .get("item_idea_ids")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("{} item_idea_ids must be an array", context))?;
        if items.is_empty() {
            return Err(anyhow!("{} item_idea_ids must not be empty", context));
        }
        for (item_index, item) in items.iter().enumerate() {
            let idea_id = validate_uuid_v7(
                item.as_str().ok_or_else(|| {
                    anyhow!("{}.item_idea_ids[{}] must be a UUID", context, item_index)
                })?,
                &format!("{}.item_idea_ids[{}]", context, item_index),
            )?;
            if !idea_ids.contains(&idea_id) {
                return Err(anyhow!(
                    "{}.item_idea_ids[{}] does not resolve",
                    context,
                    item_index
                ));
            }
        }
        if require_string(record, "speaker_identity_id", &context)? != V4_PILOT_SPEAKER_ID
            || require_string(record, "review_state", &context)? != "owner_review_required"
        {
            return Err(anyhow!("{} speaker/review boundary mismatch", context));
        }
        match (event_type, record.get("base_ordering_id")) {
            ("ordering_create", Some(Value::Null)) => {}
            ("ordering_fork", Some(Value::String(base))) => {
                pending_bases.push((ordering_id, validate_uuid_v7(base, "base_ordering_id")?));
            }
            _ => return Err(anyhow!("{} invalid base_ordering_id semantics", context)),
        }
        reject_live_rail_keys(value, &context)?;
    }
    if pending_bases.len() != 1
        || pending_bases
            .iter()
            .any(|(ordering, base)| ordering == base || !ids.contains(base))
    {
        return Err(anyhow!(
            "ordering_fork must resolve one distinct package base Ordering"
        ));
    }
    Ok(())
}

fn validate_pilot_universal_profiles(
    records: &[Value],
    idea_ids: &HashSet<Uuid>,
) -> Result<HashMap<Uuid, PilotImportanceProfile>> {
    if records.len() != V4_PILOT_IDEA_COUNT {
        return Err(anyhow!(
            "universal_importance_profiles must contain exactly {} profiles",
            V4_PILOT_IDEA_COUNT
        ));
    }
    let expected_slots = UNIVERSAL_ORIENTATIONS
        .iter()
        .flat_map(|orientation| {
            IMPORTANCE_HORIZONS
                .iter()
                .map(move |horizon| format!("{}__{}", orientation, horizon))
        })
        .collect::<BTreeSet<_>>();
    let mut seen_ideas = HashSet::new();
    let mut ranks_by_slot: BTreeMap<String, Vec<i64>> = BTreeMap::new();
    let mut profiles = HashMap::new();
    let mut unresolved_count = 0;

    for (index, value) in records.iter().enumerate() {
        let context = format!("universal_importance_profiles.records[{}]", index);
        let record = require_object(value, &context)?;
        let idea_id = validate_uuid_v7(
            require_string(record, "idea_id", &context)?,
            &format!("{}.idea_id", context),
        )?;
        if !idea_ids.contains(&idea_id) || !seen_ideas.insert(idea_id) {
            return Err(anyhow!("{} idea_id is missing or duplicated", context));
        }
        validate_provenance_and_review(record, &context)?;

        let slots = record
            .get("slots")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("{} slots must be an array", context))?;
        if slots.len() != expected_slots.len() {
            return Err(anyhow!("{} must contain exactly 20 slots", context));
        }
        let mut observed_slots = BTreeSet::new();
        let mut aggregate_score = 0_i64;
        let mut has_null_rank = false;
        let mut computed_horizons: BTreeMap<String, i64> = IMPORTANCE_HORIZONS
            .iter()
            .map(|value| (value.to_string(), 0_i64))
            .collect();
        for (slot_index, slot) in slots.iter().enumerate() {
            let slot_context = format!("{}.slots[{}]", context, slot_index);
            let slot = require_object(slot, &slot_context)?;
            let orientation = require_string(slot, "orientation", &slot_context)?;
            let timeframe = require_string(slot, "timeframe", &slot_context)?;
            let slot_key = format!("{}__{}", orientation, timeframe);
            if !expected_slots.contains(&slot_key) || !observed_slots.insert(slot_key.clone()) {
                return Err(anyhow!(
                    "{} invalid or duplicate slot={}",
                    slot_context,
                    slot_key
                ));
            }
            let rank = slot.get("pilot_subset_rank_value");
            if matches!(rank, Some(Value::Null)) {
                has_null_rank = true;
                continue;
            }
            let rank_value = rank.and_then(Value::as_i64).ok_or_else(|| {
                anyhow!(
                    "{}.pilot_subset_rank_value must be integer or null",
                    slot_context
                )
            })?;
            if !(1..V4_PILOT_IDEA_COUNT as i64).contains(&rank_value) {
                return Err(anyhow!(
                    "{} pilot_subset_rank_value out of range",
                    slot_context
                ));
            }
            aggregate_score += rank_value;
            *computed_horizons
                .get_mut(timeframe)
                .expect("validated horizon") += rank_value;
            ranks_by_slot.entry(slot_key).or_default().push(rank_value);
        }
        if observed_slots != expected_slots {
            return Err(anyhow!("{} universal slot coverage mismatch", context));
        }

        if has_null_rank {
            if idea_id.to_string() != V4_PILOT_SPEAKER_ID
                || slots.iter().any(|slot| {
                    !matches!(
                        slot.as_object()
                            .and_then(|object| object.get("pilot_subset_rank_value")),
                        Some(Value::Null)
                    )
                })
            {
                return Err(anyhow!(
                    "{} only the DEC-044 identity may have an entirely unresolved profile",
                    context
                ));
            }
            unresolved_count += 1;
            profiles.insert(
                idea_id,
                PilotImportanceProfile {
                    aggregate_score: None,
                    horizon_subtotals: IMPORTANCE_HORIZONS
                        .iter()
                        .map(|horizon| (horizon.to_string(), None))
                        .collect(),
                },
            );
        } else {
            profiles.insert(
                idea_id,
                PilotImportanceProfile {
                    aggregate_score: Some(aggregate_score),
                    horizon_subtotals: computed_horizons
                        .into_iter()
                        .map(|(horizon, value)| (horizon, Some(value)))
                        .collect(),
                },
            );
        }
        reject_live_rail_keys(value, &context)?;
    }

    if seen_ideas != *idea_ids {
        return Err(anyhow!(
            "universal_importance_profiles must cover every pilot idea exactly once"
        ));
    }
    if unresolved_count != 1 {
        return Err(anyhow!(
            "exactly one universal profile must remain unresolved"
        ));
    }
    let expected_ranks = (1..V4_PILOT_IDEA_COUNT as i64).collect::<Vec<_>>();
    for (slot, ranks) in &mut ranks_by_slot {
        ranks.sort_unstable();
        if *ranks != expected_ranks {
            return Err(anyhow!(
                "universal rank slot {} must be a complete 1..={} permutation",
                slot,
                V4_PILOT_IDEA_COUNT - 1
            ));
        }
    }
    Ok(profiles)
}

fn validate_pilot_relative_contexts(
    records: &[Value],
    idea_ids: &HashSet<Uuid>,
    connection_ids: &HashSet<Uuid>,
    package_domain: &str,
    frozen_at: &str,
) -> Result<()> {
    let mut lens_ids = HashSet::new();
    for (index, value) in records.iter().enumerate() {
        let context = format!("relative_contexts[{}]", index);
        let record = require_object(value, &context)?;
        let lens_id = validate_uuid_v7(
            require_string(record, "lens_id", &context)?,
            &format!("{}.lens_id", context),
        )?;
        if !lens_ids.insert(lens_id) {
            return Err(anyhow!("{} duplicate lens_id", context));
        }
        validate_deterministic_uuid(
            lens_id,
            package_domain,
            frozen_at,
            "relative_importance_lens",
            require_string(record, "inherited_lens_id", &context)?,
            &context,
        )?;
        let connection_id = validate_uuid_v7(
            require_string(record, "connection_id", &context)?,
            &format!("{}.connection_id", context),
        )?;
        if !connection_ids.contains(&connection_id) {
            return Err(anyhow!("{}.connection_id does not resolve", context));
        }
        for field in ["reference_idea_id", "source_idea_id", "target_idea_id"] {
            let idea_id = validate_uuid_v7(
                require_string(record, field, &context)?,
                &format!("{}.{}", context, field),
            )?;
            if !idea_ids.contains(&idea_id) {
                return Err(anyhow!("{}.{} does not resolve", context, field));
            }
        }
        if !matches!(
            require_string(record, "axis", &context)?,
            "important_to_reference" | "important_for_reference"
        ) || !IMPORTANCE_HORIZONS.contains(&require_string(record, "timeframe", &context)?)
            || require_i64(record, "rank_value", &context)? <= 0
        {
            return Err(anyhow!(
                "{} invalid relative-importance coordinate",
                context
            ));
        }
        validate_provenance_and_review(record, &context)?;
        reject_live_rail_keys(value, &context)?;
    }
    Ok(())
}

fn validate_pilot_derived_importance(
    records: &[Value],
    profiles: &HashMap<Uuid, PilotImportanceProfile>,
) -> Result<()> {
    if records.len() != V4_PILOT_IDEA_COUNT {
        return Err(anyhow!(
            "derived importance must contain exactly 50 records"
        ));
    }
    let mut seen = HashSet::new();
    let mut projections = Vec::new();
    let mut unresolved = 0;
    for (index, value) in records.iter().enumerate() {
        let context = format!("derived_importance.records[{}]", index);
        let record = require_object(value, &context)?;
        let idea_id = validate_uuid_v7(
            require_string(record, "idea_id", &context)?,
            &format!("{}.idea_id", context),
        )?;
        let profile = profiles
            .get(&idea_id)
            .ok_or_else(|| anyhow!("{}.idea_id has no universal profile", context))?;
        if !seen.insert(idea_id) {
            return Err(anyhow!("{} duplicate idea_id", context));
        }
        let selection_index = require_i64(
            require_object_field(record, "creation_order", &context)?,
            "selection_index",
            &format!("{}.creation_order", context),
        )?;
        if !(1..=V4_PILOT_IDEA_COUNT as i64).contains(&selection_index) {
            return Err(anyhow!("{} selection_index out of range", context));
        }
        match profile.aggregate_score {
            None => {
                if idea_id.to_string() != V4_PILOT_SPEAKER_ID
                    || !matches!(record.get("aggregate_score"), Some(Value::Null))
                    || !matches!(record.get("cumulative_rank"), Some(Value::Null))
                {
                    return Err(anyhow!("{} invalid unresolved DEC-037 projection", context));
                }
                unresolved += 1;
            }
            Some(expected_score) => {
                let actual_score = require_i64(record, "aggregate_score", &context)?;
                if actual_score != expected_score {
                    return Err(anyhow!("{} aggregate_score mismatch", context));
                }
                let declared_horizons =
                    require_object_field(record, "horizon_subtotals", &context)?;
                let mut horizons = BTreeMap::new();
                for horizon in IMPORTANCE_HORIZONS {
                    let actual = require_i64(
                        declared_horizons,
                        horizon,
                        &format!("{}.horizon_subtotals", context),
                    )?;
                    if Some(actual) != profile.horizon_subtotals[horizon] {
                        return Err(anyhow!("{} {} subtotal mismatch", context, horizon));
                    }
                    horizons.insert(horizon.to_string(), actual);
                }
                projections.push(UniversalProfileProjection {
                    idea_id,
                    aggregate_score: actual_score,
                    horizon_subtotals: horizons,
                    selection_index,
                    cumulative_rank: require_i64(record, "cumulative_rank", &context)?,
                });
            }
        }
    }
    if seen.len() != profiles.len() || unresolved != 1 {
        return Err(anyhow!("derived importance coverage mismatch"));
    }
    projections.sort_by_key(|projection| {
        (
            projection.aggregate_score,
            projection.horizon_subtotals["trans_generational"],
            projection.horizon_subtotals["very_long_term"],
            projection.horizon_subtotals["long_term"],
            projection.horizon_subtotals["mid_term"],
            projection.horizon_subtotals["near_term"],
            projection.selection_index,
        )
    });
    for (index, projection) in projections.iter().enumerate() {
        if projection.cumulative_rank != index as i64 + 1 {
            return Err(anyhow!(
                "derived importance idea_id={} cumulative_rank mismatch",
                projection.idea_id
            ));
        }
    }
    Ok(())
}

fn validate_pilot_event_templates(
    records: &[Value],
    idea_ids: &HashSet<Uuid>,
    package_domain: &str,
    frozen_at: &str,
) -> Result<()> {
    if records.len() != V4_PILOT_IDEA_COUNT {
        return Err(anyhow!(
            "authored event templates must contain exactly 50 records"
        ));
    }
    let mut seen_ideas = HashSet::new();
    for (index, value) in records.iter().enumerate() {
        let context = format!("authored_event_templates.records[{}]", index);
        let record = require_object(value, &context)?;
        let payload = require_object_field(record, "payload", &context)?;
        let idea_id = validate_uuid_v7(
            require_string(payload, "idea_id", &format!("{}.payload", context))?,
            &format!("{}.payload.idea_id", context),
        )?;
        if !idea_ids.contains(&idea_id) || !seen_ideas.insert(idea_id) {
            return Err(anyhow!("{} payload idea is missing or duplicated", context));
        }
        for (field, record_type) in [
            ("template_id", "authored_event_template"),
            ("event_id", "event"),
        ] {
            let id = validate_uuid_v7(
                require_string(record, field, &context)?,
                &format!("{}.{}", context, field),
            )?;
            validate_deterministic_uuid(
                id,
                package_domain,
                frozen_at,
                record_type,
                &format!("idea_create/{}", idea_id),
                &context,
            )?;
        }
        if require_string(record, "stage", &context)? != "local_draft"
            || require_string(record, "signature_profile", &context)? != "ed25519_v0"
            || require_string(record, "event_type", &context)? != "idea_create"
            || require_string(record, "speaker_identity_id", &context)? != V4_PILOT_SPEAKER_ID
            || require_string(record, "payload_binding_mode", &context)? != "embedded_payload"
            || !require_string(record, "import_eligibility", &context)?.starts_with("blocked_")
        {
            return Err(anyhow!(
                "{} violates the unsigned Profile-v0 template boundary",
                context
            ));
        }
        for field in [
            "author_identity_id",
            "public_key_ref",
            "payload_hash",
            "payload_ref",
            "author_observed_at",
            "signature",
        ] {
            if !matches!(record.get(field), Some(Value::Null)) {
                return Err(anyhow!("{}.{} must remain null", context, field));
            }
        }
        let provenance = require_object_field(record, "provenance", &context)?;
        if require_bool(
            provenance,
            "human_authorship_claimed",
            &format!("{}.provenance", context),
        )? {
            return Err(anyhow!("{} falsely claims human authorship", context));
        }
    }
    Ok(())
}

fn validate_import_projection(projection: &Map<String, Value>) -> Result<()> {
    if !require_bool(projection, "validate_only_required", "import projection")?
        || !require_bool(
            projection,
            "validate_only_zero_canonical_writes_required",
            "import projection",
        )?
        || require_bool(projection, "importer_result_claimed", "import projection")?
        || !matches!(projection.get("importer_result"), Some(Value::Null))
    {
        return Err(anyhow!(
            "import projection does not preserve validate-only isolation"
        ));
    }
    let state = require_object_field(projection, "package_state", "import projection")?;
    if require_bool(state, "signed", "import projection.package_state")?
        || require_bool(state, "canonical", "import projection.package_state")?
        || require_bool(state, "genesis", "import projection.package_state")?
        || !require_bool(
            state,
            "isolated_local_only",
            "import projection.package_state",
        )?
        || require_bool(state, "owner_accepted", "import projection.package_state")?
    {
        return Err(anyhow!(
            "import projection package_state is not unsigned/noncanonical"
        ));
    }
    Ok(())
}

fn validate_slug(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(anyhow!("{} must be lowercase kebab-case", field));
    }
    Ok(())
}

fn validate_deterministic_uuid(
    actual: Uuid,
    package_domain: &str,
    frozen_at: &str,
    record_type: &str,
    stable_key: &str,
    context: &str,
) -> Result<()> {
    let expected = deterministic_pilot_uuid_v7(package_domain, frozen_at, record_type, stable_key)?;
    if actual != expected {
        return Err(anyhow!(
            "{} UUIDv7 does not match deterministic derivation: expected {}, got {}",
            context,
            expected,
            actual
        ));
    }
    Ok(())
}

fn deterministic_pilot_uuid_v7(
    package_domain: &str,
    frozen_at: &str,
    record_type: &str,
    stable_key: &str,
) -> Result<Uuid> {
    let timestamp_ms = parse_exact_utc_millis(frozen_at)?;
    let digest = Sha256::new()
        .chain_update(format!("{}\0", V4_PILOT_UUID_DOMAIN))
        .chain_update(package_domain.as_bytes())
        .chain_update(b"\0")
        .chain_update(record_type.as_bytes())
        .chain_update(b"\0")
        .chain_update(stable_key.as_bytes())
        .finalize();
    let mut bytes = [0_u8; 16];
    let timestamp_bytes = timestamp_ms.to_be_bytes();
    bytes[..6].copy_from_slice(&timestamp_bytes[2..]);
    bytes[6..].copy_from_slice(&digest[..10]);
    bytes[6] = (bytes[6] & 0x0f) | 0x70;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Ok(Uuid::from_bytes(bytes))
}

fn parse_exact_utc_millis(value: &str) -> Result<u64> {
    if value.len() != 24
        || &value[4..5] != "-"
        || &value[7..8] != "-"
        || &value[10..11] != "T"
        || &value[13..14] != ":"
        || &value[16..17] != ":"
        || &value[19..20] != "."
        || &value[23..24] != "Z"
    {
        return Err(anyhow!("frozen_at must be exact YYYY-MM-DDTHH:MM:SS.mmmZ"));
    }
    let parse = |range: std::ops::Range<usize>| -> Result<i64> {
        value[range]
            .parse::<i64>()
            .map_err(|_| anyhow!("frozen_at contains a non-numeric field"))
    };
    let year = parse(0..4)?;
    let month = parse(5..7)?;
    let day = parse(8..10)?;
    let hour = parse(11..13)?;
    let minute = parse(14..16)?;
    let second = parse(17..19)?;
    let millis = parse(20..23)?;
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || !(0..=23).contains(&hour)
        || !(0..=59).contains(&minute)
        || !(0..=59).contains(&second)
    {
        return Err(anyhow!("frozen_at contains an out-of-range UTC field"));
    }
    let adjusted_year = year - i64::from(month <= 2);
    let era = if adjusted_year >= 0 {
        adjusted_year
    } else {
        adjusted_year - 399
    } / 400;
    let year_of_era = adjusted_year - era * 400;
    let shifted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * shifted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days_since_epoch = era * 146_097 + day_of_era - 719_468;
    let total = ((((days_since_epoch * 24) + hour) * 60 + minute) * 60 + second) * 1000 + millis;
    u64::try_from(total).map_err(|_| anyhow!("frozen_at precedes the Unix epoch"))
}

fn validate_uuid_v7(value: &str, field: &str) -> Result<Uuid> {
    let parsed = Uuid::parse_str(value).map_err(|_| anyhow!("{} must be a UUID", field))?;
    if parsed.get_version_num() != 7 || parsed.to_string() != value {
        return Err(anyhow!("{} must be canonical lowercase UUIDv7", field));
    }
    Ok(parsed)
}

fn validate_provenance_and_review(record: &Map<String, Value>, context: &str) -> Result<()> {
    let provenance = require_object_field(record, "provenance", context)?;
    if provenance.is_empty() {
        return Err(anyhow!("{}.provenance must not be empty", context));
    }
    match (record.get("review_state"), record.get("review_status")) {
        (Some(Value::Object(review_state)), _) if !review_state.is_empty() => {}
        (_, Some(Value::String(review_status))) if !review_status.trim().is_empty() => {}
        _ => {
            return Err(anyhow!(
                "{} requires a non-empty review_state object or review_status string",
                context
            ))
        }
    }
    Ok(())
}

fn reject_live_rail_keys(value: &Value, context: &str) -> Result<()> {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                if key.starts_with("rail_") {
                    return Err(anyhow!(
                        "{} contains forbidden live Rail substrate field={}",
                        context,
                        key
                    ));
                }
                reject_live_rail_keys(child, context)?;
            }
        }
        Value::Array(values) => {
            for child in values {
                reject_live_rail_keys(child, context)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn require_object<'a>(value: &'a Value, context: &str) -> Result<&'a Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| anyhow!("{} must be a JSON object", context))
}

fn require_object_field<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<&'a Map<String, Value>> {
    object
        .get(field)
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("{}.{} must be a JSON object", context, field))
}

fn require_string<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<&'a str> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("{}.{} must be a string", context, field))
}

fn require_i64(object: &Map<String, Value>, field: &str, context: &str) -> Result<i64> {
    object
        .get(field)
        .and_then(Value::as_i64)
        .ok_or_else(|| anyhow!("{}.{} must be an integer", context, field))
}

fn require_bool(object: &Map<String, Value>, field: &str, context: &str) -> Result<bool> {
    object
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| anyhow!("{}.{} must be a boolean", context, field))
}

fn parse_args() -> Result<Options> {
    let mut file: Option<PathBuf> = None;
    let mut force = false;
    let mut validate_only = false;

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
            "--validate-only" => {
                validate_only = true;
            }
            _ => {
                return Err(anyhow!(
                    "unexpected argument '{}' (usage: seed-importer [--file <path>] [--force] [--validate-only])",
                    arg
                ));
            }
        }
    }

    let file = match file {
        Some(path) => path,
        None => default_seed_path()?,
    };

    Ok(Options {
        file,
        force,
        validate_only,
    })
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
        "TRUNCATE TABLE identities_s0, tempo_predicates, cycle_boundaries, snapshot_commits, challenge_arguments, challenge_targets, challenge_context, challenges, ordering_items, orderings, representations, connections, ideas, events, snapshots, blocks RESTART IDENTITY CASCADE",
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
    tier_complexity: Option<i16>,
    vocabulary_version_id: Option<Uuid>,
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
struct OrderingRow {
    ordering_id: Uuid,
    ordering_profile: i16,
    vine_type: Option<i16>,
    subject_idea_id: Option<Uuid>,
    speaker_identity_id: Uuid,
    created_block_height: i64,
    created_event_index: i32,
    created_event_id: Uuid,
    base_ordering_id: Option<Uuid>,
    title_representation_id: Option<Uuid>,
    sentence_representation_id: Option<Uuid>,
}

#[derive(Debug)]
struct OrderingItemRow {
    ordering_id: Uuid,
    idx: i32,
    idea_id: Uuid,
    item_role: Option<i16>,
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
              vocabulary_version_id,
              payload_hash,
              payload_text,
              author_identity_id,
              language_locale,
              provenance,
              created_block_height,
              created_event_index,
              created_event_id
            ) VALUES (
              $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
            )
            "#,
        )
        .bind(row.representation_id)
        .bind(row.target_kind)
        .bind(row.target_id)
        .bind(row.tier_enum)
        .bind(row.tier_complexity)
        .bind(row.vocabulary_version_id)
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

async fn insert_orderings(tx: &mut Transaction<'_, Postgres>, rows: &[OrderingRow]) -> Result<()> {
    for row in rows {
        sqlx::query(
            r#"
            INSERT INTO orderings (
              ordering_id,
              ordering_profile,
              vine_type,
              subject_idea_id,
              speaker_identity_id,
              created_block_height,
              created_event_index,
              created_event_id,
              base_ordering_id,
              title_representation_id,
              sentence_representation_id
            ) VALUES (
              $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            )
            "#,
        )
        .bind(row.ordering_id)
        .bind(row.ordering_profile)
        .bind(row.vine_type)
        .bind(row.subject_idea_id)
        .bind(row.speaker_identity_id)
        .bind(row.created_block_height)
        .bind(row.created_event_index)
        .bind(row.created_event_id)
        .bind(row.base_ordering_id)
        .bind(row.title_representation_id)
        .bind(row.sentence_representation_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn insert_ordering_items(
    tx: &mut Transaction<'_, Postgres>,
    rows: &[OrderingItemRow],
) -> Result<()> {
    for row in rows {
        sqlx::query(
            r#"
            INSERT INTO ordering_items (
              ordering_id,
              idx,
              idea_id,
              item_role,
              via_connection_id
            ) VALUES (
              $1, $2, $3, $4, $5
            )
            "#,
        )
        .bind(row.ordering_id)
        .bind(row.idx)
        .bind(row.idea_id)
        .bind(row.item_role)
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
            "ordering" => Ok(TargetKind::Ordering),
            _ => Err(anyhow!("invalid {}", field)),
        },
        _ => Err(anyhow!("invalid {}", field)),
    }
}

fn parse_ordering_profile_field(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<OrderingProfile> {
    let value = payload
        .get(field)
        .ok_or_else(|| anyhow!("missing {}", field))?;
    match value {
        Value::String(value) if value == "vine" => Ok(OrderingProfile::Vine),
        Value::String(value) if value == "evidence_rail" => Ok(OrderingProfile::EvidenceRail),
        Value::String(value) if value == "action_rail" => Ok(OrderingProfile::ActionRail),
        _ => Err(anyhow!("invalid {}", field)),
    }
}

fn parse_ordering_subject(
    payload: &serde_json::Map<String, Value>,
    profile: OrderingProfile,
    ideas: &[IdeaRow],
) -> Result<Option<Uuid>> {
    if profile == OrderingProfile::Vine {
        if payload.contains_key("subject_idea_id") {
            return Err(anyhow!("Vine must not carry subject_idea_id"));
        }
        return Ok(None);
    }
    let subject_id = parse_uuid_field(payload, "subject_idea_id")?;
    let subject = ideas
        .iter()
        .find(|idea| idea.idea_id == subject_id)
        .ok_or_else(|| anyhow!("ordering subject must exist before use: {}", subject_id))?;
    let expected_type = match profile {
        OrderingProfile::EvidenceRail => "truth_claim",
        OrderingProfile::ActionRail => "actionable_idea",
        OrderingProfile::Vine => unreachable!(),
    };
    if subject.idea_type != expected_type {
        return Err(anyhow!(
            "ordering subject {} must have idea_type {}",
            subject_id,
            expected_type
        ));
    }
    Ok(Some(subject_id))
}

fn parse_ordering_item_roles(
    payload: &serde_json::Map<String, Value>,
    profile: OrderingProfile,
    item_count: usize,
) -> Result<Vec<Option<i16>>> {
    if profile == OrderingProfile::Vine {
        if payload.contains_key("item_roles") {
            return Err(anyhow!("Vine must not carry item_roles"));
        }
        return Ok(vec![None; item_count]);
    }
    if item_count == 0 {
        return Err(anyhow!("standardized Ordering requires at least one item"));
    }
    let values = payload
        .get("item_roles")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("item_roles array required"))?;
    if values.len() != item_count {
        return Err(anyhow!(
            "item_roles must align one-for-one with item_idea_ids"
        ));
    }
    let mut roles = Vec::with_capacity(values.len());
    for value in values {
        let role = match value.as_str() {
            Some("potential_evidence") => 0,
            Some("actual_evidence") => 1,
            Some("potential_action") => 2,
            Some("proposed_action") => 3,
            _ => return Err(anyhow!("invalid item_role")),
        };
        if (profile == OrderingProfile::EvidenceRail && !matches!(role, 0 | 1))
            || (profile == OrderingProfile::ActionRail && !matches!(role, 2 | 3))
        {
            return Err(anyhow!("item_role is invalid for ordering_profile"));
        }
        roles.push(Some(role));
    }
    if profile == OrderingProfile::ActionRail && roles.iter().any(|role| *role != roles[0]) {
        return Err(anyhow!("Action Rail must use one homogeneous lane"));
    }
    Ok(roles)
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
        _ => Err(anyhow!("invalid {}", field)),
    }
}

fn parse_representation_slot(
    payload: &serde_json::Map<String, Value>,
) -> Result<(TierEnum, Option<i16>)> {
    match parse_string_field(payload, "representation_kind")? {
        "title" => {
            for field in ["tier_length", "tier_complexity", "vocabulary_version_id"] {
                if payload.contains_key(field) {
                    return Err(anyhow!("{} is forbidden for a title representation", field));
                }
            }
            Ok((TierEnum::Title, None))
        }
        "description" => {
            let tier = parse_tier_enum_field(payload, "tier_length")?;
            if tier == TierEnum::Title {
                return Err(anyhow!("title is not a description tier_length"));
            }
            let complexity = parse_tier_complexity_field(payload, "tier_complexity")?;
            Ok((tier, Some(complexity)))
        }
        _ => Err(anyhow!("invalid representation_kind")),
    }
}

fn project_representation_row(
    event: &Event,
    event_index: i32,
    seen_representation_ids: &mut HashSet<Uuid>,
    seen_identity_ids: &HashSet<Uuid>,
    seen_idea_ids: &HashSet<Uuid>,
) -> Result<(RepresentationRow, RepresentationKey)> {
    let payload = payload_object(&event.payload)?;
    let representation_id = parse_uuid_field(payload, "representation_id")?;
    if !seen_representation_ids.insert(representation_id) {
        return Err(anyhow!(
            "duplicate representation_id in seed file: {}",
            representation_id
        ));
    }
    let target_kind = parse_target_kind_field(payload, "target_kind")?;
    let target_object_id = parse_uuid_field(payload, "target_object_id")?;
    let (tier_enum, tier_complexity) = parse_representation_slot(payload)?;
    let payload_hash = parse_string_field(payload, "payload_hash")?.to_string();
    let author_identity_id = parse_uuid_field(payload, "author_identity_id")?;
    if !seen_identity_ids.contains(&author_identity_id) {
        return Err(anyhow!(
            "representation author identity must exist before use: {}",
            author_identity_id
        ));
    }
    let vocabulary_version_id = parse_optional_uuid_field(payload, "vocabulary_version_id")?;
    match tier_complexity {
        Some(3) => {
            let vocabulary_id = vocabulary_version_id
                .ok_or_else(|| anyhow!("canonical description missing vocabulary_version_id"))?;
            if !seen_idea_ids.contains(&vocabulary_id) {
                return Err(anyhow!(
                    "vocabulary idea must exist before use: {}",
                    vocabulary_id
                ));
            }
        }
        _ if vocabulary_version_id.is_some() => {
            return Err(anyhow!(
                "vocabulary_version_id is forbidden outside canonical descriptions"
            ))
        }
        _ => {}
    }
    let language_locale = optional_string_field(payload, "language_locale")?;
    let provenance = optional_string_field(payload, "provenance")?;
    let payload_text = optional_representation_payload_text(payload)?;

    Ok((
        RepresentationRow {
            representation_id,
            target_kind: target_kind.as_i16(),
            target_id: target_object_id,
            tier_enum: tier_enum.as_i16(),
            tier_complexity,
            vocabulary_version_id,
            payload_hash,
            payload_text,
            author_identity_id,
            language_locale,
            provenance,
            created_block_height: 1,
            created_event_index: event_index,
            created_event_id: event.id,
        },
        RepresentationKey {
            target_kind,
            target_object_id,
            tier_enum,
            tier_complexity,
        },
    ))
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
    let (tier_enum, tier_complexity) = parse_representation_slot(payload)?;
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
        tier_complexity,
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
            "backyard_of_relationships",
            format!("{}'s Backyard of Relationships", seed_identity_title),
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
            | "ordering_create"
            | "ordering_fork"
            | "representation_create"
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
            "kind gulag dehl's Backyard of Relationships",
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

    #[test]
    fn native_ordering_profile_fields_are_named_and_deterministic() {
        let payload = json!({
            "ordering_profile": "evidence_rail",
            "vine_type": null
        });
        let fields = payload.as_object().expect("payload object");
        assert_eq!(
            parse_ordering_profile_field(fields, "ordering_profile").expect("profile"),
            OrderingProfile::EvidenceRail
        );
        assert_eq!(
            parse_vine_type_field(fields, "vine_type", false).expect("vine type"),
            None
        );

        let numeric = json!({"ordering_profile": 1, "vine_type": 0});
        let numeric_fields = numeric.as_object().expect("numeric payload object");
        assert!(parse_ordering_profile_field(numeric_fields, "ordering_profile").is_err());
        assert!(parse_vine_type_field(numeric_fields, "vine_type", false).is_err());
    }

    #[test]
    fn unsigned_v4_pilot_is_rejected_for_import_before_database_access() {
        let options = Options {
            file: PathBuf::from("unused.json"),
            force: false,
            validate_only: false,
        };
        let error = require_v4_pilot_validate_only(&options).expect_err("import must fail");
        assert!(error
            .to_string()
            .contains("ineligible for canonical import, signing, or genesis"));

        let force_validate = Options {
            file: PathBuf::from("unused.json"),
            force: true,
            validate_only: true,
        };
        assert!(require_v4_pilot_validate_only(&force_validate).is_err());
    }

    #[test]
    fn manifest_payload_hash_reconstruction_matches_generator_bytes() {
        let payload = r#"{"schema_version":"test","manifest_payload_sha256":null}"#;
        let digest = sha256_hex(payload.as_bytes());
        let manifest = format!(
            "{{\n  \"schema_version\": \"test\",\n  \"manifest_payload_sha256\": \"{}\"\n}}\n",
            digest
        );
        validate_manifest_payload_hash(&manifest, &digest).expect("manifest hash");
    }

    #[test]
    fn deterministic_uuidv7_matches_exact_v4_pilot_vector() {
        let frozen_at = "2026-07-26T22:05:23.394Z";
        let package_domain = format!(
            "the-seed-in-my-mind/seed-v4-pilot/{}/{}/{}",
            V4_PILOT_PRIVATE_BASELINE, V4_PILOT_OPEN_CORE_BASELINE, frozen_at
        );
        let actual =
            deterministic_pilot_uuid_v7(&package_domain, frozen_at, "idea", "the-seed-in-my-mind")
                .expect("deterministic UUID");
        assert_eq!(actual.to_string(), "019fa076-1e42-7bc7-96be-d88890397179");
    }

    #[test]
    fn native_profile_names_do_not_allow_live_rail_substrate_fields() {
        reject_live_rail_keys(&json!({"ordering_profile": "evidence_rail"}), "profile")
            .expect("ecosystem profile name remains valid");
        let error = reject_live_rail_keys(&json!({"rail_id": "legacy"}), "profile")
            .expect_err("live rail field must fail");
        assert!(error
            .to_string()
            .contains("forbidden live Rail substrate field"));
    }

    #[test]
    fn title_representation_projects_as_a_separate_slot() {
        let author_identity_id = v7("00000000-0000-7000-8000-00000000a001");
        let target_object_id = v7("00000000-0000-7000-8000-00000000c001");
        let representation_id = v7("00000000-0000-7000-8000-00000000d001");
        let event_id = v7("00000000-0000-7000-8000-00000000e001");
        let event = Event {
            id: event_id,
            kind: "representation_create".to_string(),
            speaker_identity_id: Some(author_identity_id),
            payload: json!({
                "representation_id": representation_id,
                "target_kind": "idea",
                "target_object_id": target_object_id,
                "representation_kind": "title",
                "payload_hash": "1111111111111111111111111111111111111111111111111111111111111111",
                "payload_text": "The Seed in My Mind",
                "author_identity_id": author_identity_id
            }),
        };
        validate_legacy_import_event(&event).expect("valid title event");

        let mut seen_representation_ids = HashSet::new();
        let seen_identity_ids = HashSet::from([author_identity_id]);
        let (row, key) = project_representation_row(
            &event,
            2,
            &mut seen_representation_ids,
            &seen_identity_ids,
            &HashSet::new(),
        )
        .expect("title projection");

        assert_eq!(row.representation_id, representation_id);
        assert_eq!(row.target_kind, TargetKind::Idea.as_i16());
        assert_eq!(row.target_id, target_object_id);
        assert_eq!(row.tier_enum, TierEnum::Title.as_i16());
        assert_eq!(row.tier_complexity, None);
        assert_eq!(row.vocabulary_version_id, None);
        assert_eq!(row.payload_text.as_deref(), Some("The Seed in My Mind"));
        assert_eq!(row.author_identity_id, author_identity_id);
        assert_eq!(row.created_event_id, event_id);
        assert_eq!(row.created_event_index, 2);
        assert_eq!(key.target_kind, TargetKind::Idea);
        assert_eq!(key.target_object_id, target_object_id);
        assert_eq!(key.tier_enum, TierEnum::Title);
        assert_eq!(key.tier_complexity, None);
    }

    #[test]
    fn canonical_description_rejects_a_forward_vocabulary_reference() {
        let author_identity_id = v7("00000000-0000-7000-8000-00000000a001");
        let vocabulary_version_id = v7("00000000-0000-7000-8000-00000000b003");
        let event = Event {
            id: v7("00000000-0000-7000-8000-00000000e045"),
            kind: "representation_create".to_string(),
            speaker_identity_id: Some(author_identity_id),
            payload: json!({
                "representation_id": "00000000-0000-7000-8000-00000000d045",
                "target_kind": "idea",
                "target_object_id": "00000000-0000-7000-8000-00000000c001",
                "representation_kind": "description",
                "tier_length": "full",
                "tier_complexity": "canonical",
                "vocabulary_version_id": vocabulary_version_id,
                "payload_hash": "1111111111111111111111111111111111111111111111111111111111111111",
                "author_identity_id": author_identity_id
            }),
        };
        validate_legacy_import_event(&event).expect("valid event shape");
        let seen_identity_ids = HashSet::from([author_identity_id]);

        let error = project_representation_row(
            &event,
            1,
            &mut HashSet::new(),
            &seen_identity_ids,
            &HashSet::new(),
        )
        .expect_err("a UUID alone cannot satisfy event-order existence");
        assert!(error
            .to_string()
            .contains("vocabulary idea must exist before use"));

        let seen_idea_ids = HashSet::from([vocabulary_version_id]);
        let (row, _) = project_representation_row(
            &event,
            2,
            &mut HashSet::new(),
            &seen_identity_ids,
            &seen_idea_ids,
        )
        .expect("pre-existing ordinary idea is sufficient");
        assert_eq!(row.vocabulary_version_id, Some(vocabulary_version_id));
    }
}
