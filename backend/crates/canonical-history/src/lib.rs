use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use encoding::canonical::validate_id;
use encoding::hash::hash_with_domain;
use encoding::payload::{canonical_json_payload_bytes, canonical_json_payload_hash_hex, to_hex};
use event_log::validation::validate_legacy_import_event;
use event_log::Event;
use replay::ReplayDriver;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use snapshot::build_stage0_snapshot;
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use verification::signatures::{
    authored_candidate_hash_v0, decode_signature64, signed_candidate_bytes_v0,
    AuthoredEventCandidate, PAYLOAD_BINDING_EMBEDDED, SIGNATURE_PROFILE_ED25519_V0,
};

pub const PACKAGE_SCHEMA_VERSION: &str = "canonical-history-package-v1";
pub const EVENT_RECORD_SCHEMA_VERSION: &str = "canonical-event-record-v1";
pub const HASH_ALGORITHM: &str = "blake3-256";
pub const MANIFEST_FILE: &str = "manifest.json";
pub const BLOCKS_FILE: &str = "blocks.ndjson";
pub const EVENTS_FILE: &str = "events.ndjson";
pub const EXPECTED_LATEST_MIGRATION: i64 = 25;
const PACKAGE_DOMAIN: &str = "seed.canonical_history.package.v1";
const COMPONENT_DOMAIN: &str = "seed.canonical_history.component.v1";
const EVENT_RECORD_DOMAIN: &str = "seed.canonical_history.event_record.v1";
const MIGRATION_DOMAIN: &str = "seed.canonical_history.open_core_migrations.v1";

#[derive(Debug, Clone, Copy)]
pub struct ResourceLimits {
    pub max_package_bytes: u64,
    pub max_component_bytes: u64,
    pub max_record_bytes: usize,
    pub max_payload_bytes: usize,
    pub max_candidate_bytes: usize,
    pub max_events: usize,
    pub max_blocks: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_package_bytes: 512 * 1024 * 1024,
            max_component_bytes: 500 * 1024 * 1024,
            max_record_bytes: 8 * 1024 * 1024,
            max_payload_bytes: 4 * 1024 * 1024,
            max_candidate_bytes: 1024 * 1024,
            max_events: 10_000_000,
            max_blocks: 10_000_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ComponentDescriptor {
    pub name: String,
    pub byte_length: u64,
    pub blake3_256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceSnapshot {
    pub block_height: i64,
    pub snapshot_hash: String,
    pub state_root_hash: String,
    pub title_sentence_payload_root: String,
    pub shared_map_commitment: String,
    pub active_rulebook_set_hash: String,
    pub last_event_id: Uuid,
    pub event_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceSummary {
    pub height: i64,
    pub event_count: u64,
    pub last_event_id: Uuid,
    pub open_core_migration_count: u64,
    pub open_core_latest_migration: i64,
    pub open_core_migration_set_hash: String,
    pub snapshot: Option<SourceSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PackageManifest {
    pub schema_version: String,
    pub event_record_schema_version: String,
    pub hash_algorithm: String,
    pub source: SourceSummary,
    pub components: Vec<ComponentDescriptor>,
    pub whole_package_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BlockRecord {
    pub block_height: i64,
    pub block_hash: String,
    pub prev_block_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EventRecord {
    pub record_schema_version: String,
    pub block_height: i64,
    pub event_index: i32,
    pub event_id: Uuid,
    pub event_type: String,
    pub speaker_identity_id: Option<Uuid>,
    pub canonical_payload_bytes_hex: String,
    pub signature: Option<String>,
    pub signature_profile: Option<String>,
    pub author_identity_id: Option<Uuid>,
    pub public_key_ref: Option<String>,
    pub payload_hash: Option<String>,
    pub payload_binding_mode: Option<String>,
    pub payload_ref_hex: Option<String>,
    pub author_observed_at: Option<String>,
    pub signed_candidate_bytes_v0_hex: Option<String>,
    pub authored_candidate_hash_v0: Option<String>,
    pub publication_profile: Option<String>,
    pub finalized_prefix_certificate_ref: Option<String>,
    pub record_hash: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedPackage {
    pub root: PathBuf,
    pub manifest: PackageManifest,
    pub blocks: Vec<BlockRecord>,
    pub events: Vec<EventRecord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportOutcome {
    Imported,
    AlreadyPresent,
}

#[derive(Debug, Clone)]
pub struct ImportReport {
    pub outcome: ImportOutcome,
    pub height: i64,
    pub event_count: i64,
    pub state_root_hash: String,
    pub title_sentence_payload_root: String,
    pub shared_map_commitment: String,
    pub snapshot_hash: String,
    pub idea_count: i64,
    pub connection_count: i64,
    pub representation_count: i64,
    pub ordering_count: i64,
}

#[derive(Debug, FromRow)]
struct DbBlockRow {
    block_height: i64,
    block_hash: String,
    prev_block_hash: Option<String>,
}

#[derive(Debug, FromRow)]
struct DbEventRow {
    block_height: i64,
    event_index: i32,
    event_id: Uuid,
    event_type: String,
    speaker_identity_id: Option<Uuid>,
    payload_json: Value,
    signature: Option<String>,
    signature_profile: Option<String>,
    author_identity_id: Option<Uuid>,
    public_key_ref: Option<String>,
    payload_hash: Option<String>,
    payload_binding_mode: Option<String>,
    payload_ref: Option<Vec<u8>>,
    author_observed_at: Option<String>,
    signed_candidate_bytes_v0: Option<Vec<u8>>,
    authored_candidate_hash_v0: Option<String>,
    publication_profile: Option<String>,
    finalized_prefix_certificate_ref: Option<String>,
}

#[derive(Debug, FromRow)]
struct MigrationRow {
    version: i64,
    checksum: Vec<u8>,
}

#[derive(Debug, FromRow)]
struct SnapshotRow {
    block_height: i64,
    snapshot_hash: String,
    state_root_hash: Option<String>,
    title_sentence_payload_root: Option<String>,
    shared_map_commitment: Option<String>,
    active_rulebook_set_hash: Option<String>,
    last_event_id: Option<Uuid>,
    event_count: Option<i64>,
}

pub async fn export_database(
    pool: &PgPool,
    output: &Path,
    limits: ResourceLimits,
) -> Result<PackageManifest> {
    let events = load_event_records(pool).await?;
    if events.is_empty() {
        bail!("source canonical event log is empty");
    }
    if events.len() > limits.max_events {
        bail!("source event count exceeds configured limit");
    }
    let height = events.last().expect("non-empty events").block_height;
    let last_event_id = events.last().expect("non-empty events").event_id;
    let blocks = load_block_records(pool, height).await?;
    let migrations = load_open_core_migrations(pool).await?;
    require_expected_migrations(&migrations)?;
    let snapshot = load_source_snapshot(pool, height).await?;
    let source = SourceSummary {
        height,
        event_count: events.len() as u64,
        last_event_id,
        open_core_migration_count: migrations.len() as u64,
        open_core_latest_migration: migrations.last().map(|row| row.version).unwrap_or(0),
        open_core_migration_set_hash: migration_set_hash(&migrations),
        snapshot,
    };
    write_package(output, source, &blocks, &events, limits)
}

pub fn validate_package(root: &Path, limits: ResourceLimits) -> Result<ValidatedPackage> {
    let manifest_path = root.join(MANIFEST_FILE);
    let manifest_bytes = bounded_read(&manifest_path, 1024 * 1024)?;
    let manifest: PackageManifest =
        serde_json::from_slice(&manifest_bytes).context("parse canonical history manifest")?;

    if manifest.schema_version != PACKAGE_SCHEMA_VERSION {
        bail!(
            "unsupported package schema_version={}",
            manifest.schema_version
        );
    }
    if manifest.event_record_schema_version != EVENT_RECORD_SCHEMA_VERSION {
        bail!(
            "unsupported event_record_schema_version={}",
            manifest.event_record_schema_version
        );
    }
    if manifest.hash_algorithm != HASH_ALGORITHM {
        bail!("unsupported hash_algorithm={}", manifest.hash_algorithm);
    }
    if manifest.source.open_core_latest_migration != EXPECTED_LATEST_MIGRATION {
        bail!("unsupported Open Core migration level");
    }

    let expected_names = BTreeSet::from([BLOCKS_FILE.to_string(), EVENTS_FILE.to_string()]);
    let actual_names = manifest
        .components
        .iter()
        .map(|item| item.name.clone())
        .collect::<BTreeSet<_>>();
    if actual_names != expected_names || manifest.components.len() != 2 {
        bail!("manifest component set must be exactly blocks.ndjson and events.ndjson");
    }

    let mut total_bytes = manifest_bytes.len() as u64;
    let mut component_bytes = BTreeMap::new();
    for descriptor in &manifest.components {
        validate_component_name(&descriptor.name)?;
        if descriptor.byte_length > limits.max_component_bytes {
            bail!("component {} exceeds configured limit", descriptor.name);
        }
        let bytes = bounded_read(
            &root.join(&descriptor.name),
            descriptor.byte_length as usize,
        )?;
        total_bytes = total_bytes
            .checked_add(bytes.len() as u64)
            .ok_or_else(|| anyhow!("package size overflow"))?;
        if bytes.len() as u64 != descriptor.byte_length {
            bail!("component byte length mismatch: {}", descriptor.name);
        }
        if component_hash(&bytes) != descriptor.blake3_256 {
            bail!("component hash mismatch: {}", descriptor.name);
        }
        component_bytes.insert(descriptor.name.clone(), bytes);
    }
    if total_bytes > limits.max_package_bytes {
        bail!("package exceeds configured total byte limit");
    }
    if package_hash(&manifest)? != manifest.whole_package_hash {
        bail!("whole package hash mismatch");
    }

    let blocks = parse_ndjson::<BlockRecord>(
        component_bytes
            .get(BLOCKS_FILE)
            .expect("validated component"),
        limits.max_record_bytes,
        limits.max_blocks,
        BLOCKS_FILE,
    )?;
    let events = parse_ndjson::<EventRecord>(
        component_bytes
            .get(EVENTS_FILE)
            .expect("validated component"),
        limits.max_record_bytes,
        limits.max_events,
        EVENTS_FILE,
    )?;
    validate_records(&manifest, &blocks, &events, limits)?;

    let disk_names = fs::read_dir(root)
        .with_context(|| format!("read package directory {}", root.display()))?
        .map(|entry| {
            entry
                .map(|item| item.file_name().to_string_lossy().to_string())
                .map_err(anyhow::Error::from)
        })
        .collect::<Result<BTreeSet<_>>>()?;
    let allowed_names = BTreeSet::from([
        MANIFEST_FILE.to_string(),
        BLOCKS_FILE.to_string(),
        EVENTS_FILE.to_string(),
    ]);
    if disk_names != allowed_names {
        bail!("package directory contains omitted or unexpected files");
    }

    Ok(ValidatedPackage {
        root: root.to_path_buf(),
        manifest,
        blocks,
        events,
    })
}

pub async fn import_database(pool: &PgPool, package: &ValidatedPackage) -> Result<ImportReport> {
    let migrations = load_open_core_migrations(pool).await?;
    require_expected_migrations(&migrations)?;
    if migration_set_hash(&migrations) != package.manifest.source.open_core_migration_set_hash {
        bail!("target Open Core migration fingerprint differs from package");
    }

    let existing_events: i64 = sqlx::query_scalar("SELECT COUNT(*)::bigint FROM events")
        .fetch_one(pool)
        .await?;
    if existing_events > 0 {
        if load_event_records(pool).await? == package.events {
            return build_import_report(pool, ImportOutcome::AlreadyPresent, package).await;
        }
        bail!("target database is not fresh and does not exactly match this package");
    }

    let non_event_rows: i64 = sqlx::query_scalar(
        r#"
        SELECT
          (SELECT COUNT(*) FROM snapshots)
        + (SELECT COUNT(*) FROM ideas)
        + (SELECT COUNT(*) FROM connections)
        + (SELECT COUNT(*) FROM representations)
        + (SELECT COUNT(*) FROM orderings)
        + (SELECT CASE
             WHEN COUNT(*) = 1
              AND COUNT(*) FILTER (
                WHERE identity_id = 'ffffffff-ffff-7fff-bfff-ffffffffffff'::uuid
                  AND title = 'system_boundary_emitter'
                  AND created_event_id = 'ffffffff-ffff-7fff-bfff-ffffffffffff'::uuid
              ) = 1 THEN 0 ELSE 1 END
           FROM identities_s0)
        "#,
    )
    .fetch_one(pool)
    .await?;
    if non_event_rows != 0 {
        bail!("target database is not fresh: canonical or derived rows already exist");
    }

    let mut tx = pool.begin().await?;
    sqlx::query("SET LOCAL seed.allow_canonical_mutation = 'on'")
        .execute(&mut *tx)
        .await?;
    for block in &package.blocks {
        sqlx::query(
            "INSERT INTO blocks (block_height, block_hash, prev_block_hash) VALUES ($1, $2, $3)",
        )
        .bind(block.block_height)
        .bind(&block.block_hash)
        .bind(&block.prev_block_hash)
        .execute(&mut *tx)
        .await?;
    }
    for record in &package.events {
        let payload = decode_payload(record)?;
        insert_event_record(&mut tx, record, &payload).await?;
        project_event(&mut tx, record, &payload).await?;
    }
    tx.commit().await?;
    build_import_report(pool, ImportOutcome::Imported, package).await
}

pub fn database_name_from_url(database_url: &str) -> Result<String> {
    let trimmed = database_url.trim();
    let without_suffix = trimmed.split(['?', '#']).next().unwrap_or(trimmed);
    let (_, rest) = without_suffix
        .split_once("://")
        .ok_or_else(|| anyhow!("database URL must include a scheme"))?;
    let (_, path) = rest
        .split_once('/')
        .ok_or_else(|| anyhow!("database URL is missing database name"))?;
    let name = path.trim_matches('/');
    if name.is_empty() || name.contains('/') {
        bail!("database URL must contain one database name");
    }
    let normalized = name.to_ascii_lowercase();
    if matches!(
        normalized.as_str(),
        "seed_dev" | "seed_open_core" | "postgres" | "template0" | "template1"
    ) {
        bail!("database target {} is protected", normalized);
    }
    Ok(name.to_string())
}

fn write_package(
    output: &Path,
    source: SourceSummary,
    blocks: &[BlockRecord],
    events: &[EventRecord],
    limits: ResourceLimits,
) -> Result<PackageManifest> {
    if output.exists() {
        if fs::read_dir(output)
            .with_context(|| format!("read output directory {}", output.display()))?
            .next()
            .is_some()
        {
            bail!("output directory must not exist or must be empty");
        }
    } else {
        fs::create_dir_all(output)
            .with_context(|| format!("create output directory {}", output.display()))?;
    }

    let blocks_bytes = encode_ndjson(blocks)?;
    let events_bytes = encode_ndjson(events)?;
    if blocks.len() > limits.max_blocks || events.len() > limits.max_events {
        bail!("export record count exceeds configured limit");
    }
    if blocks_bytes.len() as u64 > limits.max_component_bytes
        || events_bytes.len() as u64 > limits.max_component_bytes
    {
        bail!("export component exceeds configured limit");
    }

    let components = vec![
        ComponentDescriptor {
            name: BLOCKS_FILE.to_string(),
            byte_length: blocks_bytes.len() as u64,
            blake3_256: component_hash(&blocks_bytes),
        },
        ComponentDescriptor {
            name: EVENTS_FILE.to_string(),
            byte_length: events_bytes.len() as u64,
            blake3_256: component_hash(&events_bytes),
        },
    ];
    let mut manifest = PackageManifest {
        schema_version: PACKAGE_SCHEMA_VERSION.to_string(),
        event_record_schema_version: EVENT_RECORD_SCHEMA_VERSION.to_string(),
        hash_algorithm: HASH_ALGORITHM.to_string(),
        source,
        components,
        whole_package_hash: String::new(),
    };
    manifest.whole_package_hash = package_hash(&manifest)?;
    validate_records(&manifest, blocks, events, limits)?;
    let manifest_byte_length = serde_json::to_vec_pretty(&manifest)?.len() as u64 + 1;
    let package_byte_length = manifest_byte_length
        .checked_add(blocks_bytes.len() as u64)
        .and_then(|value| value.checked_add(events_bytes.len() as u64))
        .ok_or_else(|| anyhow!("export package size overflow"))?;
    if package_byte_length > limits.max_package_bytes {
        bail!("export package exceeds configured total byte limit");
    }

    fs::write(output.join(BLOCKS_FILE), blocks_bytes)?;
    fs::write(output.join(EVENTS_FILE), events_bytes)?;
    let mut manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
    manifest_bytes.push(b'\n');
    fs::write(output.join(MANIFEST_FILE), manifest_bytes)?;
    Ok(manifest)
}

async fn load_block_records(pool: &PgPool, height: i64) -> Result<Vec<BlockRecord>> {
    let rows = sqlx::query_as::<_, DbBlockRow>(
        "SELECT block_height, block_hash, prev_block_hash FROM blocks WHERE block_height <= $1 ORDER BY block_height ASC",
    )
    .bind(height)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| BlockRecord {
            block_height: row.block_height,
            block_hash: row.block_hash,
            prev_block_hash: row.prev_block_hash,
        })
        .collect())
}

async fn load_event_records(pool: &PgPool) -> Result<Vec<EventRecord>> {
    let rows = sqlx::query_as::<_, DbEventRow>(
        r#"
        SELECT
          block_height, event_index, event_id, event_type, speaker_identity_id,
          payload_json, signature, signature_profile, author_identity_id,
          public_key_ref, payload_hash, payload_binding_mode, payload_ref,
          author_observed_at, signed_candidate_bytes_v0, authored_candidate_hash_v0,
          publication_profile, finalized_prefix_certificate_ref
        FROM events
        ORDER BY block_height ASC, event_index ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(event_record_from_row).collect()
}

fn event_record_from_row(row: DbEventRow) -> Result<EventRecord> {
    let payload_bytes =
        canonical_json_payload_bytes(&row.payload_json).map_err(|err| anyhow!(err))?;
    let mut record = EventRecord {
        record_schema_version: EVENT_RECORD_SCHEMA_VERSION.to_string(),
        block_height: row.block_height,
        event_index: row.event_index,
        event_id: row.event_id,
        event_type: row.event_type,
        speaker_identity_id: row.speaker_identity_id,
        canonical_payload_bytes_hex: to_hex(&payload_bytes),
        signature: row.signature,
        signature_profile: row.signature_profile,
        author_identity_id: row.author_identity_id,
        public_key_ref: row.public_key_ref,
        payload_hash: row.payload_hash,
        payload_binding_mode: row.payload_binding_mode,
        payload_ref_hex: row.payload_ref.as_deref().map(to_hex),
        author_observed_at: row.author_observed_at,
        signed_candidate_bytes_v0_hex: row.signed_candidate_bytes_v0.as_deref().map(to_hex),
        authored_candidate_hash_v0: row.authored_candidate_hash_v0,
        publication_profile: row.publication_profile,
        finalized_prefix_certificate_ref: row.finalized_prefix_certificate_ref,
        record_hash: String::new(),
    };
    record.record_hash = event_record_hash(&record)?;
    Ok(record)
}

async fn load_open_core_migrations(pool: &PgPool) -> Result<Vec<MigrationRow>> {
    sqlx::query_as::<_, MigrationRow>(
        r#"
        SELECT
          substring(filename from 1 for 4)::bigint AS version,
          convert_to(filename, 'UTF8') AS checksum
        FROM schema_migrations
        WHERE substring(filename from 1 for 4)::bigint <= $1
        ORDER BY filename ASC
        "#,
    )
    .bind(EXPECTED_LATEST_MIGRATION)
    .fetch_all(pool)
    .await
    .context("read Open Core migration ledger")
}

fn require_expected_migrations(rows: &[MigrationRow]) -> Result<()> {
    if rows.len() != EXPECTED_LATEST_MIGRATION as usize {
        bail!(
            "Open Core migration ledger must contain exactly versions 1 through {}",
            EXPECTED_LATEST_MIGRATION
        );
    }
    for (index, row) in rows.iter().enumerate() {
        if row.version != index as i64 + 1 {
            bail!("Open Core migration ledger is not contiguous");
        }
    }
    Ok(())
}

fn migration_set_hash(rows: &[MigrationRow]) -> String {
    let mut bytes = Vec::new();
    for row in rows {
        bytes.extend_from_slice(&row.version.to_be_bytes());
        bytes.extend_from_slice(&(row.checksum.len() as u64).to_be_bytes());
        bytes.extend_from_slice(&row.checksum);
    }
    to_hex(&hash_with_domain(MIGRATION_DOMAIN, &bytes))
}

async fn load_source_snapshot(pool: &PgPool, height: i64) -> Result<Option<SourceSnapshot>> {
    let row = sqlx::query_as::<_, SnapshotRow>(
        r#"
        SELECT block_height, snapshot_hash, state_root_hash,
          title_sentence_payload_root, shared_map_commitment,
          active_rulebook_set_hash, last_event_id, event_count
        FROM snapshots
        WHERE block_height <= $1
        ORDER BY block_height DESC
        LIMIT 1
        "#,
    )
    .bind(height)
    .fetch_optional(pool)
    .await?;
    row.map(|row| {
        Ok(SourceSnapshot {
            block_height: row.block_height,
            snapshot_hash: row.snapshot_hash,
            state_root_hash: row
                .state_root_hash
                .ok_or_else(|| anyhow!("source snapshot missing state_root_hash"))?,
            title_sentence_payload_root: row
                .title_sentence_payload_root
                .ok_or_else(|| anyhow!("source snapshot missing title_sentence_payload_root"))?,
            shared_map_commitment: row
                .shared_map_commitment
                .ok_or_else(|| anyhow!("source snapshot missing shared_map_commitment"))?,
            active_rulebook_set_hash: row
                .active_rulebook_set_hash
                .ok_or_else(|| anyhow!("source snapshot missing active_rulebook_set_hash"))?,
            last_event_id: row
                .last_event_id
                .ok_or_else(|| anyhow!("source snapshot missing last_event_id"))?,
            event_count: row
                .event_count
                .ok_or_else(|| anyhow!("source snapshot missing event_count"))?,
        })
    })
    .transpose()
}

fn validate_records(
    manifest: &PackageManifest,
    blocks: &[BlockRecord],
    events: &[EventRecord],
    limits: ResourceLimits,
) -> Result<()> {
    if blocks.is_empty() || events.is_empty() {
        bail!("package must contain blocks and events");
    }
    if events.len() as u64 != manifest.source.event_count {
        bail!("manifest event_count mismatch");
    }
    if events.len() > limits.max_events || blocks.len() > limits.max_blocks {
        bail!("package record count exceeds configured limit");
    }

    let mut block_by_height = BTreeMap::new();
    let mut previous_block: Option<&BlockRecord> = None;
    for block in blocks {
        if block.block_height < 0 || !is_lower_hex(&block.block_hash) {
            bail!("invalid block record");
        }
        if block
            .prev_block_hash
            .as_deref()
            .is_some_and(|value| !is_lower_hex(value))
        {
            bail!("prev_block_hash must be lowercase hexadecimal");
        }
        match previous_block {
            None if block.block_height != 0 || block.prev_block_hash.is_some() => {
                bail!("canonical block sequence must begin at height 0 without a predecessor")
            }
            Some(previous)
                if block.block_height != previous.block_height + 1
                    || block.prev_block_hash.as_deref() != Some(previous.block_hash.as_str()) =>
            {
                bail!("canonical block sequence is gapped, reordered, or hash-disconnected")
            }
            _ => {}
        }
        previous_block = Some(block);
        block_by_height.insert(block.block_height, block);
    }

    let mut event_ids = BTreeSet::new();
    let mut previous_position = None;
    let mut expected_index_by_block = BTreeMap::<i64, i32>::new();
    for record in events {
        if record.record_schema_version != EVENT_RECORD_SCHEMA_VERSION {
            bail!("unsupported event record version");
        }
        if record.block_height < 0 || record.event_index < 0 {
            bail!("negative canonical event position");
        }
        if !block_by_height.contains_key(&record.block_height) {
            bail!("event references omitted block");
        }
        let position = (record.block_height, record.event_index);
        if previous_position.is_some_and(|previous| position <= previous) {
            bail!("event records are reordered or duplicated");
        }
        previous_position = Some(position);
        let expected_index = expected_index_by_block
            .entry(record.block_height)
            .or_insert(0);
        if record.event_index != *expected_index {
            bail!(
                "event_index omission or gap at block {}",
                record.block_height
            );
        }
        *expected_index += 1;

        validate_id(&record.event_id.to_string()).map_err(|err| anyhow!(err))?;
        if !event_ids.insert(record.event_id) {
            bail!("duplicate event_id {}", record.event_id);
        }
        if event_record_hash(record)? != record.record_hash {
            bail!("event record hash mismatch event_id={}", record.event_id);
        }
        let payload = decode_payload_with_limit(record, limits.max_payload_bytes)?;
        validate_event_semantics(record, &payload, limits.max_candidate_bytes)?;
    }

    let last = events.last().expect("non-empty");
    if last.block_height != manifest.source.height || last.event_id != manifest.source.last_event_id
    {
        bail!("manifest source height or last_event_id mismatch");
    }
    Ok(())
}

fn validate_event_semantics(
    record: &EventRecord,
    payload: &Value,
    max_candidate_bytes: usize,
) -> Result<()> {
    if !matches!(
        record.event_type.as_str(),
        "genesis"
            | "noop"
            | "identity_create"
            | "idea_create"
            | "connection_create"
            | "representation_create"
            | "ordering_create"
            | "ordering_fork"
            | "cycle_close"
            | "snapshot_commit"
    ) {
        bail!(
            "unsupported event family for deterministic transfer replay: {}",
            record.event_type
        );
    }

    let event = Event {
        id: record.event_id,
        kind: record.event_type.clone(),
        payload: payload.clone(),
        speaker_identity_id: record.speaker_identity_id,
    };
    validate_legacy_import_event(&event).map_err(|err| {
        anyhow!(
            "event validation failed event_id={} {}",
            record.event_id,
            err
        )
    })?;

    let authored_any = [
        record.signature_profile.is_some(),
        record.author_identity_id.is_some(),
        record.public_key_ref.is_some(),
        record.payload_hash.is_some(),
        record.payload_binding_mode.is_some(),
        record.payload_ref_hex.is_some(),
        record.author_observed_at.is_some(),
        record.signed_candidate_bytes_v0_hex.is_some(),
        record.authored_candidate_hash_v0.is_some(),
    ]
    .into_iter()
    .any(|value| value);
    if !authored_any {
        return Ok(());
    }
    let authored_required = [
        record.signature.is_some(),
        record.signature_profile.is_some(),
        record.author_identity_id.is_some(),
        record.public_key_ref.is_some(),
        record.payload_hash.is_some(),
        record.payload_binding_mode.is_some(),
        record.signed_candidate_bytes_v0_hex.is_some(),
        record.authored_candidate_hash_v0.is_some(),
    ];
    if !authored_required.into_iter().all(|value| value) {
        bail!("incomplete authored-candidate audit fields");
    }
    if record.signature_profile.as_deref() != Some(SIGNATURE_PROFILE_ED25519_V0) {
        bail!("unsupported signature profile");
    }
    if !matches!(
        record.event_type.as_str(),
        "idea_create" | "connection_create"
    ) {
        bail!("nonportable signed event family requires unavailable replay authority");
    }
    if record.payload_binding_mode.as_deref() != Some(PAYLOAD_BINDING_EMBEDDED)
        || record.payload_ref_hex.is_some()
    {
        bail!("only embedded Profile-v0 payload binding is transferable");
    }
    let canonical_payload_hash =
        canonical_json_payload_hash_hex(payload).map_err(|err| anyhow!(err))?;
    if record.payload_hash.as_deref() != Some(canonical_payload_hash.as_str()) {
        bail!("authored payload_hash mismatch");
    }

    let candidate = AuthoredEventCandidate {
        signature_profile: record.signature_profile.clone().expect("presence checked"),
        event_id: record.event_id,
        event_type: record.event_type.clone(),
        author_identity_id: record.author_identity_id.expect("presence checked"),
        speaker_identity_id: record.speaker_identity_id,
        public_key_ref: record.public_key_ref.clone().expect("presence checked"),
        payload_hash: record.payload_hash.clone().expect("presence checked"),
        payload_binding_mode: record
            .payload_binding_mode
            .clone()
            .expect("presence checked"),
        payload_ref: None,
        author_observed_at: record.author_observed_at.clone(),
        signature: record.signature.clone().expect("presence checked"),
    };
    let signed_bytes = signed_candidate_bytes_v0(&candidate).map_err(|err| anyhow!(err))?;
    if signed_bytes.len() > max_candidate_bytes {
        bail!("signed candidate exceeds configured limit");
    }
    if decode_hex(
        record
            .signed_candidate_bytes_v0_hex
            .as_deref()
            .expect("presence checked"),
    )? != signed_bytes
    {
        bail!("signed candidate bytes mismatch");
    }
    let signature = decode_signature64(candidate.signature.as_str()).map_err(|err| anyhow!(err))?;
    let candidate_hash =
        authored_candidate_hash_v0(&signed_bytes, &signature).map_err(|err| anyhow!(err))?;
    if record.authored_candidate_hash_v0.as_deref() != Some(candidate_hash.as_str()) {
        bail!("authored candidate hash mismatch");
    }
    Ok(())
}

async fn insert_event_record(
    tx: &mut Transaction<'_, Postgres>,
    record: &EventRecord,
    payload: &Value,
) -> Result<()> {
    let payload_ref = record
        .payload_ref_hex
        .as_deref()
        .map(decode_hex)
        .transpose()?;
    let candidate_bytes = record
        .signed_candidate_bytes_v0_hex
        .as_deref()
        .map(decode_hex)
        .transpose()?;
    sqlx::query(
        r#"
        INSERT INTO events (
          block_height, event_index, event_id, event_type, speaker_identity_id,
          payload_json, signature, signature_profile, author_identity_id,
          public_key_ref, payload_hash, payload_binding_mode, payload_ref,
          author_observed_at, signed_candidate_bytes_v0, authored_candidate_hash_v0,
          publication_profile, finalized_prefix_certificate_ref
        ) VALUES (
          $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18
        )
        "#,
    )
    .bind(record.block_height)
    .bind(record.event_index)
    .bind(record.event_id)
    .bind(&record.event_type)
    .bind(record.speaker_identity_id)
    .bind(payload)
    .bind(&record.signature)
    .bind(&record.signature_profile)
    .bind(record.author_identity_id)
    .bind(&record.public_key_ref)
    .bind(&record.payload_hash)
    .bind(&record.payload_binding_mode)
    .bind(payload_ref)
    .bind(&record.author_observed_at)
    .bind(candidate_bytes)
    .bind(&record.authored_candidate_hash_v0)
    .bind(&record.publication_profile)
    .bind(&record.finalized_prefix_certificate_ref)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn project_event(
    tx: &mut Transaction<'_, Postgres>,
    record: &EventRecord,
    payload: &Value,
) -> Result<()> {
    let object = payload_object(payload)?;
    match record.event_type.as_str() {
        "genesis" | "noop" => {}
        "identity_create" => {
            let identity_id = uuid_field(object, "identity_id")?;
            let title = optional_string(object, "title").unwrap_or("canonical identity");
            sqlx::query(
                "INSERT INTO identities_s0 (identity_id, title, created_event_id) VALUES ($1, $2, $3)",
            )
            .bind(identity_id)
            .bind(title)
            .bind(record.event_id)
            .execute(&mut **tx)
            .await?;
        }
        "idea_create" => {
            let idea_id = uuid_field(object, "idea_id")?;
            let idea_type = string_field(object, "idea_type")?;
            let speaker = record
                .speaker_identity_id
                .ok_or_else(|| anyhow!("idea_create missing speaker_identity_id"))?;
            sqlx::query(
                r#"
                INSERT INTO ideas (
                  idea_id, idea_type, speaker_identity_id, is_identity_idea,
                  underlying_identity_id, is_personal_space_organizer,
                  title_representation_id, sentence_representation_id,
                  created_block_height, created_event_index, created_event_id
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
                "#,
            )
            .bind(idea_id)
            .bind(idea_type)
            .bind(speaker)
            .bind(idea_type == "identity")
            .bind(optional_uuid(object, "underlying_identity_id")?)
            .bind(bool_field_default(
                object,
                "is_personal_space_organizer",
                false,
            )?)
            .bind(optional_uuid(object, "title_representation_id")?)
            .bind(optional_uuid(object, "sentence_representation_id")?)
            .bind(record.block_height)
            .bind(record.event_index)
            .bind(record.event_id)
            .execute(&mut **tx)
            .await?;
        }
        "connection_create" => {
            let connection_id = uuid_field(object, "connection_id")?;
            let from_idea_id = uuid_field(object, "from_idea_id")?;
            let to_idea_id = uuid_field(object, "to_idea_id")?;
            let connection_type = string_field(object, "connection_type")?;
            let usage = optional_string(object, "usage");
            sqlx::query(
                r#"
                INSERT INTO connections (
                  connection_id, from_idea_id, to_idea_id, connection_type,
                  usage, axis, timeframe, scope,
                  created_block_height, created_event_index, created_by_event_id
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
                "#,
            )
            .bind(connection_id)
            .bind(from_idea_id)
            .bind(to_idea_id)
            .bind(connection_type)
            .bind(usage)
            .bind(optional_string(object, "axis"))
            .bind(optional_string(object, "timeframe"))
            .bind(optional_string(object, "scope"))
            .bind(record.block_height)
            .bind(record.event_index)
            .bind(record.event_id)
            .execute(&mut **tx)
            .await?;

            if connection_type == "membership" && usage == Some("has_space") {
                sqlx::query(
                    r#"
                    UPDATE ideas AS target
                    SET is_personal_space_organizer = true
                    WHERE target.idea_id = $1
                      AND EXISTS (
                        SELECT 1
                        FROM ideas AS source
                        WHERE source.idea_id = $2
                          AND source.is_identity_idea = true
                      )
                    "#,
                )
                .bind(to_idea_id)
                .bind(from_idea_id)
                .execute(&mut **tx)
                .await?;
            }
        }
        "representation_create" => {
            let kind = string_field(object, "representation_kind")?;
            let tier_enum = match kind {
                "title" => 0_i16,
                "description" => match string_field(object, "tier_length")? {
                    "sentence" => 1,
                    "paragraph" => 2,
                    "full" => 3,
                    other => bail!("unsupported tier_length={other}"),
                },
                other => bail!("unsupported representation_kind={other}"),
            };
            let tier_complexity = optional_string(object, "tier_complexity")
                .map(parse_tier_complexity)
                .transpose()?;
            let target_kind = match string_field(object, "target_kind")? {
                "idea" => 0_i16,
                "ordering" => 1_i16,
                other => bail!("unsupported target_kind={other}"),
            };
            sqlx::query(
                r#"
                INSERT INTO representations (
                  representation_id, target_kind, target_id, tier_enum, tier_complexity,
                  vocabulary_version_id, payload_hash, payload_text, author_identity_id,
                  language_locale, provenance, created_block_height, created_event_index,
                  created_event_id
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)
                "#,
            )
            .bind(uuid_field(object, "representation_id")?)
            .bind(target_kind)
            .bind(uuid_field(object, "target_object_id")?)
            .bind(tier_enum)
            .bind(tier_complexity)
            .bind(optional_uuid(object, "vocabulary_version_id")?)
            .bind(string_field(object, "payload_hash")?)
            .bind(optional_string(object, "payload_text"))
            .bind(uuid_field(object, "author_identity_id")?)
            .bind(optional_string(object, "language_locale"))
            .bind(optional_string(object, "provenance"))
            .bind(record.block_height)
            .bind(record.event_index)
            .bind(record.event_id)
            .execute(&mut **tx)
            .await?;
        }

        "ordering_create" | "ordering_fork" => {
            let profile = parse_ordering_profile(string_field(object, "ordering_profile")?)?;
            let vine_type = optional_string(object, "vine_type")
                .map(parse_vine_type)
                .transpose()?;
            let ordering_id = uuid_field(object, "ordering_id")?;
            let item_ids = uuid_array(object, "item_idea_ids")?;
            let item_roles = optional_string_array(object, "item_roles")?;
            if profile == 0 && item_roles.is_some() {
                bail!("Vine must not contain item_roles");
            }
            if profile != 0 && item_roles.as_ref().map(Vec::len) != Some(item_ids.len()) {
                bail!("standardized Ordering requires aligned item_roles");
            }
            let speaker = record
                .speaker_identity_id
                .ok_or_else(|| anyhow!("Ordering missing speaker_identity_id"))?;
            sqlx::query(
                r#"
                INSERT INTO orderings (
                  ordering_id, ordering_profile, vine_type, subject_idea_id,
                  speaker_identity_id, created_block_height, created_event_index,
                  created_event_id, base_ordering_id, title_representation_id,
                  sentence_representation_id
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
                "#,
            )
            .bind(ordering_id)
            .bind(profile)
            .bind(vine_type)
            .bind(optional_uuid(object, "subject_idea_id")?)
            .bind(speaker)
            .bind(record.block_height)
            .bind(record.event_index)
            .bind(record.event_id)
            .bind(optional_uuid(object, "base_ordering_id")?)
            .bind(optional_uuid(object, "title_representation_id")?)
            .bind(optional_uuid(object, "sentence_representation_id")?)
            .execute(&mut **tx)
            .await?;
            for (index, idea_id) in item_ids.into_iter().enumerate() {
                let role = item_roles
                    .as_ref()
                    .map(|items| parse_item_role(&items[index]))
                    .transpose()?;
                sqlx::query(
                    "INSERT INTO ordering_items (ordering_id, idx, idea_id, item_role, via_connection_id) VALUES ($1,$2,$3,$4,$5)",
                )
                .bind(ordering_id)
                .bind(index as i32)
                .bind(idea_id)
                .bind(role)
                .bind::<Option<Uuid>>(None)
                .execute(&mut **tx)
                .await?;
            }
        }
        "cycle_close" => {
            let closure_kind = match string_field(object, "closure_kind")? {
                "deliberative" => 0_i16,
                "forced" => 1_i16,
                other => bail!("unsupported closure_kind={other}"),
            };
            let closure_ref = object
                .get("closure_boundary_ref")
                .and_then(Value::as_object)
                .ok_or_else(|| anyhow!("cycle_close missing closure_boundary_ref"))?;
            sqlx::query(
                r#"
                INSERT INTO cycle_boundaries (
                  cycle_index, closure_kind, forced_seal, closure_block_height,
                  source_block_height, source_event_index, source_event_id
                ) VALUES ($1,$2,$3,$4,$5,$6,$7)
                "#,
            )
            .bind(i64_field(object, "cycle_index")?)
            .bind(closure_kind)
            .bind(bool_field_default(object, "forced_seal", false)?)
            .bind(i64_field(closure_ref, "block_height")?)
            .bind(record.block_height)
            .bind(record.event_index)
            .bind(record.event_id)
            .execute(&mut **tx)
            .await?;
        }
        "snapshot_commit" => {
            sqlx::query(
                r#"
                INSERT INTO snapshot_commits (
                  block_height, snapshot_hash, state_root_hash,
                  title_sentence_payload_root, shared_map_commitment,
                  last_event_id, event_count, active_rulebook_set_hash,
                  created_event_id
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
                "#,
            )
            .bind(i64_field(object, "block_height")?)
            .bind(string_field(object, "snapshot_hash")?)
            .bind(string_field(object, "state_root_hash")?)
            .bind(string_field(object, "title_sentence_payload_root")?)
            .bind(string_field(object, "shared_map_commitment")?)
            .bind(uuid_field(object, "last_event_id")?)
            .bind(i64_field(object, "event_count")?)
            .bind(string_field(object, "active_rulebook_set_hash")?)
            .bind(record.event_id)
            .execute(&mut **tx)
            .await?;
        }
        other => bail!("unsupported event family for projection replay: {other}"),
    }

    if [
        "cycle_age_ge_dmin",
        "cycle_age_ge_dmax",
        "constrained_mode",
        "record_only_mode",
    ]
    .iter()
    .all(|field| object.contains_key(*field))
    {
        sqlx::query(
            r#"
            INSERT INTO tempo_predicates (
              block_height, event_index, cycle_age_ge_dmin, cycle_age_ge_dmax,
              constrained_mode, record_only_mode
            ) VALUES ($1,$2,$3,$4,$5,$6)
            "#,
        )
        .bind(record.block_height)
        .bind(record.event_index)
        .bind(bool_field_default(object, "cycle_age_ge_dmin", false)?)
        .bind(bool_field_default(object, "cycle_age_ge_dmax", false)?)
        .bind(bool_field_default(object, "constrained_mode", false)?)
        .bind(bool_field_default(object, "record_only_mode", false)?)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn build_import_report(
    pool: &PgPool,
    outcome: ImportOutcome,
    package: &ValidatedPackage,
) -> Result<ImportReport> {
    let replay = ReplayDriver::run(pool, Some(package.manifest.source.height))
        .await
        .map_err(|err| anyhow!("post-import replay failed: {err}"))?;
    if replay.event_count != package.manifest.source.event_count as i64
        || replay.last_event_id != package.manifest.source.last_event_id
    {
        bail!("post-import event count or last_event_id differs from package");
    }
    let built = build_stage0_snapshot(&replay)
        .map_err(|err| anyhow!("post-import snapshot rebuild failed: {err}"))?;

    if let Some(expected) = &package.manifest.source.snapshot {
        let mut checkpoint_replay = if expected.block_height == package.manifest.source.height {
            replay.clone()
        } else {
            ReplayDriver::run(pool, Some(expected.block_height))
                .await
                .map_err(|err| anyhow!("checkpoint replay failed: {err}"))?
        };
        checkpoint_replay.event_count = expected.event_count;
        checkpoint_replay.last_event_id = expected.last_event_id;
        checkpoint_replay.approximate_timestamp =
            approximate_timestamp_from_event_id(expected.last_event_id)?;
        let checkpoint = build_stage0_snapshot(&checkpoint_replay)
            .map_err(|err| anyhow!("checkpoint snapshot rebuild failed: {err}"))?;
        compare_snapshot(expected, &checkpoint)?;
    }

    let idea_count = sqlx::query_scalar("SELECT COUNT(*)::bigint FROM ideas")
        .fetch_one(pool)
        .await?;
    let connection_count = sqlx::query_scalar("SELECT COUNT(*)::bigint FROM connections")
        .fetch_one(pool)
        .await?;
    let representation_count = sqlx::query_scalar("SELECT COUNT(*)::bigint FROM representations")
        .fetch_one(pool)
        .await?;
    let ordering_count = sqlx::query_scalar("SELECT COUNT(*)::bigint FROM orderings")
        .fetch_one(pool)
        .await?;

    Ok(ImportReport {
        outcome,
        height: replay.height,
        event_count: replay.event_count,
        state_root_hash: to_hex(&built.commitments.state_root_hash),
        title_sentence_payload_root: to_hex(&built.commitments.title_sentence_payload_root),
        shared_map_commitment: to_hex(&built.commitments.shared_map_commitment),
        snapshot_hash: to_hex(&built.snapshot_hash),
        idea_count,
        connection_count,
        representation_count,
        ordering_count,
    })
}

fn compare_snapshot(expected: &SourceSnapshot, actual: &snapshot::Stage0Snapshot) -> Result<()> {
    if expected.snapshot_hash != to_hex(&actual.snapshot_hash)
        || expected.state_root_hash != to_hex(&actual.commitments.state_root_hash)
        || expected.title_sentence_payload_root
            != to_hex(&actual.commitments.title_sentence_payload_root)
        || expected.shared_map_commitment != to_hex(&actual.commitments.shared_map_commitment)
        || expected.active_rulebook_set_hash != to_hex(&actual.commitments.active_rulebook_set_hash)
        || expected.last_event_id != actual.last_event_id
        || expected.event_count != actual.event_count
    {
        bail!("post-import replay/snapshot commitment comparison failed");
    }
    Ok(())
}

fn approximate_timestamp_from_event_id(event_id: Uuid) -> Result<DateTime<Utc>> {
    let timestamp = event_id
        .get_timestamp()
        .ok_or_else(|| anyhow!("snapshot last_event_id is not UUIDv7"))?;
    let (seconds, nanos) = timestamp.to_unix();
    let seconds = i64::try_from(seconds).context("snapshot timestamp exceeds i64")?;
    DateTime::<Utc>::from_timestamp(seconds, nanos)
        .ok_or_else(|| anyhow!("snapshot last_event_id has invalid timestamp components"))
}

fn decode_payload(record: &EventRecord) -> Result<Value> {
    decode_payload_with_limit(record, ResourceLimits::default().max_payload_bytes)
}

fn decode_payload_with_limit(record: &EventRecord, limit: usize) -> Result<Value> {
    let bytes = decode_hex(&record.canonical_payload_bytes_hex)?;
    if bytes.len() > limit {
        bail!("canonical payload exceeds configured limit");
    }
    let payload: Value =
        serde_json::from_slice(&bytes).context("canonical payload bytes are not JSON")?;
    let rebuilt = canonical_json_payload_bytes(&payload).map_err(|err| anyhow!(err))?;
    if rebuilt != bytes {
        bail!("payload bytes are not the exact canonical JSON encoding");
    }
    Ok(payload)
}

fn event_record_hash(record: &EventRecord) -> Result<String> {
    let mut core = record.clone();
    core.record_hash.clear();
    Ok(to_hex(&hash_with_domain(
        EVENT_RECORD_DOMAIN,
        &serde_json::to_vec(&core)?,
    )))
}

fn package_hash(manifest: &PackageManifest) -> Result<String> {
    let mut core = manifest.clone();
    core.whole_package_hash.clear();
    Ok(to_hex(&hash_with_domain(
        PACKAGE_DOMAIN,
        &serde_json::to_vec(&core)?,
    )))
}

fn component_hash(bytes: &[u8]) -> String {
    to_hex(&hash_with_domain(COMPONENT_DOMAIN, bytes))
}

fn encode_ndjson<T: Serialize>(items: &[T]) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    for item in items {
        serde_json::to_writer(&mut bytes, item)?;
        bytes.push(b'\n');
    }
    Ok(bytes)
}

fn parse_ndjson<T>(
    bytes: &[u8],
    max_record_bytes: usize,
    max_records: usize,
    label: &str,
) -> Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let lines = bytes.split(|byte| *byte == b'\n').collect::<Vec<_>>();
    let mut records = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            if index + 1 == lines.len() {
                continue;
            }
            bail!("{label} contains an empty record");
        }
        if line.len() > max_record_bytes {
            bail!("{label} record exceeds configured limit");
        }
        if records.len() >= max_records {
            bail!("{label} record count exceeds configured limit");
        }
        records.push(
            serde_json::from_slice(line)
                .with_context(|| format!("parse {label} record {}", index + 1))?,
        );
    }
    Ok(records)
}

fn bounded_read(path: &Path, limit: usize) -> Result<Vec<u8>> {
    let metadata = fs::metadata(path).with_context(|| format!("stat {}", path.display()))?;
    if metadata.len() > limit as u64 {
        bail!("{} exceeds configured byte limit", path.display());
    }
    fs::read(path).with_context(|| format!("read {}", path.display()))
}

fn validate_component_name(name: &str) -> Result<()> {
    if !matches!(name, BLOCKS_FILE | EVENTS_FILE) || Path::new(name).components().count() != 1 {
        bail!("invalid package component path");
    }
    Ok(())
}

fn decode_hex(value: &str) -> Result<Vec<u8>> {
    if value.len() % 2 != 0 {
        bail!("hex value has odd length");
    }
    let mut bytes = Vec::with_capacity(value.len() / 2);
    for pair in value.as_bytes().chunks_exact(2) {
        bytes.push((hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?);
    }
    Ok(bytes)
}

fn hex_nibble(value: u8) -> Result<u8> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => bail!("hex values must use lowercase hexadecimal"),
    }
}

fn is_lower_hex(value: &str) -> bool {
    !value.is_empty()
        && value
            .as_bytes()
            .iter()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn payload_object(value: &Value) -> Result<&Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| anyhow!("event payload must be an object"))
}

fn string_field<'a>(object: &'a Map<String, Value>, field: &str) -> Result<&'a str> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("payload.{field} must be a string"))
}

fn optional_string<'a>(object: &'a Map<String, Value>, field: &str) -> Option<&'a str> {
    object.get(field).and_then(Value::as_str)
}

fn uuid_field(object: &Map<String, Value>, field: &str) -> Result<Uuid> {
    Uuid::parse_str(string_field(object, field)?)
        .with_context(|| format!("payload.{field} must be a UUID"))
}

fn optional_uuid(object: &Map<String, Value>, field: &str) -> Result<Option<Uuid>> {
    match object.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => {
            Ok(Some(Uuid::parse_str(value).with_context(|| {
                format!("payload.{field} must be a UUID")
            })?))
        }
        _ => bail!("payload.{field} must be a UUID string or null"),
    }
}

fn i64_field(object: &Map<String, Value>, field: &str) -> Result<i64> {
    object
        .get(field)
        .and_then(Value::as_i64)
        .ok_or_else(|| anyhow!("payload.{field} must be an integer"))
}

fn bool_field_default(object: &Map<String, Value>, field: &str, default: bool) -> Result<bool> {
    match object.get(field) {
        None => Ok(default),
        Some(Value::Bool(value)) => Ok(*value),
        _ => bail!("payload.{field} must be a boolean"),
    }
}

fn uuid_array(object: &Map<String, Value>, field: &str) -> Result<Vec<Uuid>> {
    object
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("payload.{field} must be an array"))?
        .iter()
        .map(|item| {
            item.as_str()
                .ok_or_else(|| anyhow!("payload.{field} item must be a UUID string"))
                .and_then(|value| Uuid::parse_str(value).map_err(anyhow::Error::from))
        })
        .collect()
}

fn optional_string_array(object: &Map<String, Value>, field: &str) -> Result<Option<Vec<String>>> {
    match object.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Array(values)) => values
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_string)
                    .ok_or_else(|| anyhow!("payload.{field} items must be strings"))
            })
            .collect::<Result<Vec<_>>>()
            .map(Some),
        _ => bail!("payload.{field} must be an array"),
    }
}

fn parse_tier_complexity(value: &str) -> Result<i16> {
    match value {
        "fundamental" => Ok(0),
        "standard" => Ok(1),
        "advanced" => Ok(2),
        "canonical" => Ok(3),
        _ => bail!("unsupported tier_complexity={value}"),
    }
}

fn parse_ordering_profile(value: &str) -> Result<i16> {
    match value {
        "vine" => Ok(0),
        "evidence_rail" => Ok(1),
        "action_rail" => Ok(2),
        _ => bail!("unsupported ordering_profile={value}"),
    }
}

fn parse_vine_type(value: &str) -> Result<i16> {
    match value {
        "pathway_vine" => Ok(0),
        "narrative_vine" => Ok(1),
        _ => bail!("unsupported vine_type={value}"),
    }
}

fn parse_item_role(value: &str) -> Result<i16> {
    match value {
        "evidence_for" => Ok(0),
        "evidence_against" => Ok(1),
        "action_step" => Ok(2),
        "action_checkpoint" => Ok(3),
        _ => bail!("unsupported item_role={value}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> Uuid {
        Uuid::parse_str(value).unwrap()
    }

    fn sample_records() -> (SourceSummary, Vec<BlockRecord>, Vec<EventRecord>) {
        let blocks = vec![
            BlockRecord {
                block_height: 0,
                block_hash: "00".to_string(),
                prev_block_hash: None,
            },
            BlockRecord {
                block_height: 1,
                block_hash: "01".to_string(),
                prev_block_hash: Some("00".to_string()),
            },
        ];
        let payloads = vec![
            (
                "00000000-0000-7000-8000-000000000001",
                "identity_create",
                serde_json::json!({
                    "identity_id": "00000000-0000-7000-8000-000000000101",
                    "title": "test identity",
                    "initial_public_key_ref": "test",
                    "verification_reference": "test"
                }),
            ),
            (
                "00000000-0000-7000-8000-000000000002",
                "idea_create",
                serde_json::json!({
                    "idea_id": "00000000-0000-7000-8000-000000000201",
                    "idea_type": "conceptual_idea",
                    "title": "alpha",
                    "sentence": "alpha",
                    "paragraph": null,
                    "full": null,
                    "payload_hash": encoding::payload::payload_hash_hex(
                        "alpha", "alpha", None, None
                    ).unwrap()
                }),
            ),
        ];
        let speaker = id("00000000-0000-7000-8000-000000000101");
        let mut events = Vec::new();
        for (index, (event_id, event_type, payload)) in payloads.into_iter().enumerate() {
            let mut record = EventRecord {
                record_schema_version: EVENT_RECORD_SCHEMA_VERSION.to_string(),
                block_height: 1,
                event_index: index as i32,
                event_id: id(event_id),
                event_type: event_type.to_string(),
                speaker_identity_id: Some(speaker),
                canonical_payload_bytes_hex: to_hex(
                    &canonical_json_payload_bytes(&payload).unwrap(),
                ),
                signature: None,
                signature_profile: None,
                author_identity_id: None,
                public_key_ref: None,
                payload_hash: None,
                payload_binding_mode: None,
                payload_ref_hex: None,
                author_observed_at: None,
                signed_candidate_bytes_v0_hex: None,
                authored_candidate_hash_v0: None,
                publication_profile: None,
                finalized_prefix_certificate_ref: None,
                record_hash: String::new(),
            };
            record.record_hash = event_record_hash(&record).unwrap();
            events.push(record);
        }
        let source = SourceSummary {
            height: 1,
            event_count: events.len() as u64,
            last_event_id: events.last().unwrap().event_id,
            open_core_migration_count: EXPECTED_LATEST_MIGRATION as u64,
            open_core_latest_migration: EXPECTED_LATEST_MIGRATION,
            open_core_migration_set_hash: "11".repeat(32),
            snapshot: None,
        };
        (source, blocks, events)
    }

    fn temp_root(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!("canonical-history-{label}-{}", Uuid::new_v4()))
    }

    #[test]
    fn deterministic_repeat_export_is_byte_identical() {
        let (source, blocks, events) = sample_records();
        let left = temp_root("left");
        let right = temp_root("right");
        write_package(
            &left,
            source.clone(),
            &blocks,
            &events,
            ResourceLimits::default(),
        )
        .unwrap();
        write_package(&right, source, &blocks, &events, ResourceLimits::default()).unwrap();

        for name in [MANIFEST_FILE, BLOCKS_FILE, EVENTS_FILE] {
            assert_eq!(
                fs::read(left.join(name)).unwrap(),
                fs::read(right.join(name)).unwrap()
            );
        }
        validate_package(&left, ResourceLimits::default()).unwrap();
        validate_package(&right, ResourceLimits::default()).unwrap();
        fs::remove_dir_all(left).unwrap();
        fs::remove_dir_all(right).unwrap();
    }

    #[test]
    fn rejects_tamper_omission_reorder_duplicate_version_and_oversize() {
        let (mut source, blocks, events) = sample_records();
        let root = temp_root("negative");
        write_package(
            &root,
            source.clone(),
            &blocks,
            &events,
            ResourceLimits::default(),
        )
        .unwrap();
        let mut tampered = fs::read(root.join(EVENTS_FILE)).unwrap();
        tampered[10] ^= 1;
        fs::write(root.join(EVENTS_FILE), tampered).unwrap();
        assert!(validate_package(&root, ResourceLimits::default()).is_err());
        fs::remove_dir_all(&root).unwrap();

        let mut manifest = PackageManifest {
            schema_version: PACKAGE_SCHEMA_VERSION.to_string(),
            event_record_schema_version: EVENT_RECORD_SCHEMA_VERSION.to_string(),
            hash_algorithm: HASH_ALGORITHM.to_string(),
            source: source.clone(),
            components: Vec::new(),
            whole_package_hash: String::new(),
        };
        assert!(validate_records(
            &manifest,
            &blocks,
            &events[..events.len() - 1],
            ResourceLimits::default()
        )
        .is_err());

        let mut reordered = events.clone();
        reordered.swap(0, 1);
        assert!(
            validate_records(&manifest, &blocks, &reordered, ResourceLimits::default()).is_err()
        );

        let mut duplicate = events.clone();
        duplicate.push(events.last().unwrap().clone());
        source.event_count = duplicate.len() as u64;
        manifest.source = source;
        assert!(
            validate_records(&manifest, &blocks, &duplicate, ResourceLimits::default()).is_err()
        );

        let mut unsupported = events.clone();
        unsupported[0].record_schema_version = "canonical-event-record-v2".to_string();
        unsupported[0].record_hash = event_record_hash(&unsupported[0]).unwrap();
        assert!(
            validate_records(&manifest, &blocks, &unsupported, ResourceLimits::default()).is_err()
        );

        let mut limits = ResourceLimits::default();
        limits.max_payload_bytes = 1;
        manifest.source.event_count = events.len() as u64;
        assert!(validate_records(&manifest, &blocks, &events, limits).is_err());
    }

    #[test]
    fn machine_negative_fixture_catalog_matches_implemented_cases() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../docs/conformance/canonical-history-transfer-negative.v1.json");
        let catalog: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        let ids = catalog["cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|case| case["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            ids,
            BTreeSet::from([
                "duplicate_event",
                "omitted_event",
                "oversize_payload",
                "reordered_events",
                "tampered_component",
                "unsupported_version",
            ])
        );
    }

    #[test]
    fn protected_database_names_are_rejected() {
        assert!(database_name_from_url("postgresql://user@localhost/seed_dev").is_err());
        assert!(database_name_from_url("postgresql://user@localhost/postgres").is_err());
        assert_eq!(
            database_name_from_url(
                "postgresql://user@localhost/seed_opencore_canonical_history_transfer_001_a"
            )
            .unwrap(),
            "seed_opencore_canonical_history_transfer_001_a"
        );
    }
}
