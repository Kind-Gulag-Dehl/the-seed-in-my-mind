use chrono::{DateTime, Utc};
use event_log::validation::validate_event;
use event_log::{snapshot_interval_blocks, system_boundary_emitter_id, Event};
use hmac::{Hmac, Mac};
use replay::ReplayDriver;
use serde_json::json;
use sha2::Sha256;
use snapshot::{build_stage0_snapshot, sha256_hex, to_hex};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool, Postgres, Transaction};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;
const SNAPSHOT_COMMIT_SIGNATURE_DOMAIN: &[u8] = b"seed.snapshot_commit.v1";
const SNAPSHOT_COMMIT_KEY_ENV_KEYS: [&str; 2] = [
    "SYSTEM_BOUNDARY_EMITTER_HMAC_KEY",
    "SNAPSHOT_COMMIT_HMAC_KEY",
];
const SNAPSHOT_COMMIT_KEY_FILE_ENV_KEYS: [&str; 2] = [
    "SYSTEM_BOUNDARY_EMITTER_HMAC_KEY_FILE",
    "SNAPSHOT_COMMIT_HMAC_KEY_FILE",
];
const SNAPSHOT_COMMIT_MODE_ENV_KEYS: [&str; 3] = ["SEED_ENV", "APP_ENV", "RUST_ENV"];
const DEV_SNAPSHOT_COMMIT_SIGNING_KEY: &[u8] = b"seed.snapshot_commit.dev_key.v1";
static SNAPSHOT_COMMIT_SIGNING_KEY: OnceLock<Vec<u8>> = OnceLock::new();

#[derive(Debug, FromRow)]
struct PrevSnapshotRow {
    snapshot_hash: String,
}

#[derive(Debug, FromRow)]
struct SnapshotBasisCountRow {
    event_count: i64,
}

#[derive(Debug, FromRow)]
struct SnapshotBasisLastEventRow {
    event_id: Uuid,
}

#[derive(Debug, Clone, FromRow)]
struct SnapshotCommitRow {
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

#[derive(Debug, Clone)]
struct SnapshotBasis {
    event_count: i64,
    last_event_id: Uuid,
    approximate_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct SnapshotCommitMaterialization {
    block_height: i64,
    snapshot_hash: String,
    state_root_hash: String,
    title_sentence_payload_root: String,
    shared_map_commitment: String,
    last_event_id: Uuid,
    event_count: i64,
    active_rulebook_set_hash: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnapshotCommitOutcome {
    SkippedNonBoundary,
    AlreadyExists(Uuid),
    Emitted(Uuid),
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Stage 0 snapshot builder (snapshot-format-v0):
    // - Replay canonical events deterministically.
    // - Build snapshot bytes + commitments from replay output.
    // - Persist snapshot artifact bytes and metadata.
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&database_url).await?;
    let interval_blocks = snapshot_interval_blocks();
    let require_snapshot_commit = env_flag("REQUIRE_SNAPSHOT_COMMIT");

    let replay = ReplayDriver::run(&pool, None)
        .await
        .map_err(|err| anyhow::anyhow!(err))?;
    let snapshot_basis = load_snapshot_basis(&pool, replay.height).await?;
    let mut replay_for_snapshot = replay.clone();
    replay_for_snapshot.event_count = snapshot_basis.event_count;
    replay_for_snapshot.last_event_id = snapshot_basis.last_event_id;
    replay_for_snapshot.approximate_timestamp = snapshot_basis.approximate_timestamp;

    let stage0 = build_stage0_snapshot(&replay_for_snapshot).map_err(|err| anyhow::anyhow!(err))?;
    let snapshot_hash_hex = to_hex(&stage0.snapshot_hash);
    let artifact_sha256 = sha256_hex(&stage0.bytes);
    let artifact_path = write_snapshot_artifact(&stage0.bytes, stage0.height, &snapshot_hash_hex)?;

    let prev_snapshot_hash = sqlx::query_as::<_, PrevSnapshotRow>(
        r#"
        SELECT snapshot_hash
        FROM snapshots
        WHERE block_height < $1
        ORDER BY block_height DESC
        LIMIT 1
        "#,
    )
    .bind(stage0.height)
    .fetch_optional(&pool)
    .await?
    .map(|row| row.snapshot_hash);

    if let Some(ref prev) = prev_snapshot_hash {
        if !is_hex_64(prev) {
            return Err(anyhow::anyhow!(
                "invalid prev_snapshot_hash length or format"
            ));
        }
    }

    upsert_snapshot(
        &pool,
        stage0.height,
        &snapshot_hash_hex,
        prev_snapshot_hash.as_deref(),
        &to_hex(&stage0.commitments.state_root_hash),
        &to_hex(&stage0.commitments.title_sentence_payload_root),
        &to_hex(&stage0.commitments.shared_map_commitment),
        &to_hex(&stage0.commitments.active_rulebook_set_hash),
        stage0.last_event_id,
        stage0.event_count,
        stage0.approximate_timestamp,
        replay_for_snapshot.cycle_status.cycle_index,
        replay_for_snapshot.cycle_status.last_cycle_close_height,
        &artifact_path,
        &artifact_sha256,
    )
    .await?;

    let snapshot_commit = SnapshotCommitMaterialization {
        block_height: stage0.height,
        snapshot_hash: snapshot_hash_hex.clone(),
        state_root_hash: to_hex(&stage0.commitments.state_root_hash),
        title_sentence_payload_root: to_hex(&stage0.commitments.title_sentence_payload_root),
        shared_map_commitment: to_hex(&stage0.commitments.shared_map_commitment),
        last_event_id: stage0.last_event_id,
        event_count: stage0.event_count,
        active_rulebook_set_hash: to_hex(&stage0.commitments.active_rulebook_set_hash),
    };
    let snapshot_commit_outcome =
        match ensure_snapshot_commit(&pool, &snapshot_commit, interval_blocks).await {
            Ok(outcome) => outcome,
            Err(err) => {
                eprintln!(
                    "snapshot-builder: snapshot_commit emission failed at height={} interval={} error={}",
                    stage0.height, interval_blocks, err
                );
                if require_snapshot_commit && should_emit_snapshot_commit(stage0.height, interval_blocks)
                {
                    return Err(err);
                }
                SnapshotCommitOutcome::SkippedNonBoundary
            }
        };

    println!(
        "snapshot-builder: height={} snapshot_hash={} state_root_hash={} artifact_path={}",
        stage0.height,
        snapshot_hash_hex,
        to_hex(&stage0.commitments.state_root_hash),
        artifact_path
    );
    match snapshot_commit_outcome {
        SnapshotCommitOutcome::SkippedNonBoundary => {
            println!(
                "snapshot-builder: snapshot_commit skipped height={} interval={}",
                stage0.height, interval_blocks
            );
        }
        SnapshotCommitOutcome::AlreadyExists(event_id) => {
            println!(
                "snapshot-builder: snapshot_commit already exists height={} event_id={}",
                stage0.height, event_id
            );
        }
        SnapshotCommitOutcome::Emitted(event_id) => {
            println!(
                "snapshot-builder: snapshot_commit emitted height={} event_id={} author={}",
                stage0.height,
                event_id,
                system_boundary_emitter_id()
            );
        }
    }

    Ok(())
}

fn write_snapshot_artifact(
    bytes: &[u8],
    height: i64,
    snapshot_id: &str,
) -> Result<String, anyhow::Error> {
    let backend_root = backend_root()?;
    let repo_root = backend_root
        .parent()
        .ok_or_else(|| anyhow::anyhow!("unable to resolve repo root"))?;

    let relative = format!(
        "backend/var/snapshots/v0/{}/{}.snapshot",
        height, snapshot_id
    );
    let abs_path = repo_root.join(Path::new(&relative));
    let parent = abs_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("invalid artifact path"))?;
    fs::create_dir_all(parent)?;
    fs::write(&abs_path, bytes)?;

    Ok(relative)
}

fn backend_root() -> Result<PathBuf, anyhow::Error> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let backend_root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .ok_or_else(|| anyhow::anyhow!("unable to resolve backend root"))?;
    Ok(backend_root.to_path_buf())
}

async fn load_snapshot_basis(pool: &PgPool, height: i64) -> Result<SnapshotBasis, anyhow::Error> {
    let count = sqlx::query_as::<_, SnapshotBasisCountRow>(
        r#"
        SELECT COUNT(*)::bigint AS event_count
        FROM events
        WHERE block_height < $1
           OR (block_height = $1 AND event_type <> 'snapshot_commit')
        "#,
    )
    .bind(height)
    .fetch_one(pool)
    .await?;
    let last_event = sqlx::query_as::<_, SnapshotBasisLastEventRow>(
        r#"
        SELECT event_id
        FROM events
        WHERE block_height < $1
           OR (block_height = $1 AND event_type <> 'snapshot_commit')
        ORDER BY block_height DESC, event_index DESC
        LIMIT 1
        "#,
    )
    .bind(height)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow::anyhow!("no snapshot basis event found for height {}", height))?;

    Ok(SnapshotBasis {
        event_count: count.event_count,
        approximate_timestamp: approximate_timestamp_from_event_id(last_event.event_id)?,
        last_event_id: last_event.event_id,
    })
}

async fn ensure_snapshot_commit(
    pool: &PgPool,
    commit: &SnapshotCommitMaterialization,
    interval_blocks: i64,
) -> Result<SnapshotCommitOutcome, anyhow::Error> {
    if !should_emit_snapshot_commit(commit.block_height, interval_blocks) {
        return Ok(SnapshotCommitOutcome::SkippedNonBoundary);
    }

    if let Some(existing) = load_snapshot_commit(pool, commit.block_height).await? {
        ensure_snapshot_commit_matches(&existing, commit)?;
        return Ok(SnapshotCommitOutcome::AlreadyExists(existing.created_event_id));
    }

    let event_id = Uuid::now_v7();
    let payload = json!({
        "block_height": commit.block_height,
        "snapshot_id": commit.snapshot_hash,
        "snapshot_hash": commit.snapshot_hash,
        "state_root_hash": commit.state_root_hash,
        "title_sentence_payload_root": commit.title_sentence_payload_root,
        "shared_map_commitment": commit.shared_map_commitment,
        "last_event_id": commit.last_event_id,
        "event_count": commit.event_count,
        "active_rulebook_set_hash": commit.active_rulebook_set_hash
    });
    let event = Event {
        id: event_id,
        kind: "snapshot_commit".to_string(),
        payload: payload.clone(),
        speaker_identity_id: Some(system_boundary_emitter_id()),
    };
    validate_event(&event)
        .map_err(|err| anyhow::anyhow!("snapshot_commit validation failed: {}", err))?;
    let signature = Some(sign_snapshot_commit(&event)?);

    let mut tx = pool.begin().await?;
    sqlx::query("SET LOCAL seed.allow_canonical_mutation = 'on'")
        .execute(&mut *tx)
        .await?;
    let _locked_block_height: i64 =
        sqlx::query_scalar("SELECT block_height FROM blocks WHERE block_height = $1 FOR UPDATE")
            .bind(commit.block_height)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!("missing canonical block_height {}", commit.block_height)
            })?;

    if let Some(existing) = load_snapshot_commit_tx(&mut tx, commit.block_height).await? {
        ensure_snapshot_commit_matches(&existing, commit)?;
        tx.rollback().await?;
        return Ok(SnapshotCommitOutcome::AlreadyExists(existing.created_event_id));
    }

    let event_index: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(event_index), -1)::int + 1 FROM events WHERE block_height = $1",
    )
    .bind(commit.block_height)
    .fetch_one(&mut *tx)
    .await?;

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
    .bind(commit.block_height)
    .bind(event_index)
    .bind(event_id)
    .bind("snapshot_commit")
    .bind(Some(system_boundary_emitter_id()))
    .bind(payload)
    .bind(signature)
    .execute(&mut *tx)
    .await?;

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
    .bind(commit.block_height)
    .bind(&commit.snapshot_hash)
    .bind(&commit.state_root_hash)
    .bind(&commit.title_sentence_payload_root)
    .bind(&commit.shared_map_commitment)
    .bind(commit.last_event_id)
    .bind(commit.event_count)
    .bind(&commit.active_rulebook_set_hash)
    .bind(event_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(SnapshotCommitOutcome::Emitted(event_id))
}

async fn load_snapshot_commit(
    pool: &PgPool,
    block_height: i64,
) -> Result<Option<SnapshotCommitRow>, anyhow::Error> {
    sqlx::query_as::<_, SnapshotCommitRow>(
        r#"
        SELECT
          block_height,
          snapshot_hash,
          state_root_hash,
          title_sentence_payload_root,
          shared_map_commitment,
          last_event_id,
          event_count,
          active_rulebook_set_hash,
          created_event_id
        FROM snapshot_commits
        WHERE block_height = $1
        "#,
    )
    .bind(block_height)
    .fetch_optional(pool)
    .await
    .map_err(anyhow::Error::from)
}

async fn load_snapshot_commit_tx(
    tx: &mut Transaction<'_, Postgres>,
    block_height: i64,
) -> Result<Option<SnapshotCommitRow>, anyhow::Error> {
    sqlx::query_as::<_, SnapshotCommitRow>(
        r#"
        SELECT
          block_height,
          snapshot_hash,
          state_root_hash,
          title_sentence_payload_root,
          shared_map_commitment,
          last_event_id,
          event_count,
          active_rulebook_set_hash,
          created_event_id
        FROM snapshot_commits
        WHERE block_height = $1
        "#,
    )
    .bind(block_height)
    .fetch_optional(&mut **tx)
    .await
    .map_err(anyhow::Error::from)
}

fn ensure_snapshot_commit_matches(
    existing: &SnapshotCommitRow,
    commit: &SnapshotCommitMaterialization,
) -> Result<(), anyhow::Error> {
    if existing.block_height != commit.block_height
        || existing.snapshot_hash != commit.snapshot_hash
        || existing.state_root_hash != commit.state_root_hash
        || existing.title_sentence_payload_root != commit.title_sentence_payload_root
        || existing.shared_map_commitment != commit.shared_map_commitment
        || existing.last_event_id != commit.last_event_id
        || existing.event_count != commit.event_count
        || existing.active_rulebook_set_hash != commit.active_rulebook_set_hash
    {
        return Err(anyhow::anyhow!(
            "snapshot_commit mismatch at height {}",
            commit.block_height
        ));
    }
    Ok(())
}

fn should_emit_snapshot_commit(block_height: i64, interval_blocks: i64) -> bool {
    interval_blocks > 0 && block_height >= 0 && block_height % interval_blocks == 0
}

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|value| matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn sign_snapshot_commit(event: &Event) -> Result<String, anyhow::Error> {
    let key = resolve_snapshot_commit_signing_key()?;
    let payload = event
        .payload
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("snapshot_commit payload must be object"))?;
    let mut message = Vec::new();
    message.extend_from_slice(SNAPSHOT_COMMIT_SIGNATURE_DOMAIN);
    message.push(0);
    message.extend_from_slice(event.kind.as_bytes());
    message.push(0);
    message.extend_from_slice(event.id.to_string().as_bytes());
    message.push(0);
    message.extend_from_slice(system_boundary_emitter_id().to_string().as_bytes());
    for field in [
        "block_height",
        "snapshot_hash",
        "state_root_hash",
        "title_sentence_payload_root",
        "shared_map_commitment",
        "last_event_id",
        "event_count",
        "active_rulebook_set_hash",
    ] {
        let value = payload
            .get(field)
            .ok_or_else(|| anyhow::anyhow!("snapshot_commit payload missing {}", field))?;
        message.push(0);
        message.extend_from_slice(field.as_bytes());
        message.push(0);
        message.extend_from_slice(value.to_string().as_bytes());
    }

    let mut mac = HmacSha256::new_from_slice(key)?;
    mac.update(&message);
    let digest = mac.finalize().into_bytes();
    Ok(format!("hmac-sha256:{}", bytes_to_hex(&digest)))
}

fn resolve_snapshot_commit_signing_key() -> Result<&'static [u8], anyhow::Error> {
    if let Some(key) = SNAPSHOT_COMMIT_SIGNING_KEY.get() {
        return Ok(key.as_slice());
    }

    let computed = compute_snapshot_commit_signing_key()?;
    let _ = SNAPSHOT_COMMIT_SIGNING_KEY.set(computed);
    let key = SNAPSHOT_COMMIT_SIGNING_KEY
        .get()
        .ok_or_else(|| anyhow::anyhow!("snapshot_commit signing key initialization failed"))?;
    Ok(key.as_slice())
}

fn compute_snapshot_commit_signing_key() -> Result<Vec<u8>, anyhow::Error> {
    for key_name in SNAPSHOT_COMMIT_KEY_ENV_KEYS {
        if let Ok(value) = std::env::var(key_name) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Ok(trimmed.as_bytes().to_vec());
            }
        }
    }

    for key_name in SNAPSHOT_COMMIT_KEY_FILE_ENV_KEYS {
        if let Ok(path) = std::env::var(key_name) {
            let trimmed = path.trim();
            if trimmed.is_empty() {
                continue;
            }
            let value = fs::read_to_string(trimmed)?;
            let trimmed_value = value.trim();
            if !trimmed_value.is_empty() {
                return Ok(trimmed_value.as_bytes().to_vec());
            }
        }
    }

    if is_production_mode() {
        return Err(anyhow::anyhow!(
            "SYSTEM_BOUNDARY_EMITTER_HMAC_KEY is required in production mode"
        ));
    }

    eprintln!(
        "[security] SYSTEM_BOUNDARY_EMITTER_HMAC_KEY missing; using deterministic dev-only snapshot_commit signing key"
    );
    Ok(DEV_SNAPSHOT_COMMIT_SIGNING_KEY.to_vec())
}

fn is_production_mode() -> bool {
    SNAPSHOT_COMMIT_MODE_ENV_KEYS.iter().any(|key_name| {
        if let Ok(value) = std::env::var(key_name) {
            return matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "prod" | "production"
            );
        }
        false
    })
}

fn bytes_to_hex(value: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(value.len() * 2);
    for byte in value {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn approximate_timestamp_from_event_id(event_id: Uuid) -> Result<DateTime<Utc>, anyhow::Error> {
    let timestamp = event_id
        .get_timestamp()
        .ok_or_else(|| anyhow::anyhow!("event_id={} missing UUIDv7 timestamp", event_id))?;
    let (seconds, nanos) = timestamp.to_unix();
    let seconds_i64 = i64::try_from(seconds)
        .map_err(|_| anyhow::anyhow!("event_id={} timestamp out of range", event_id))?;
    DateTime::<Utc>::from_timestamp(seconds_i64, nanos).ok_or_else(|| {
        anyhow::anyhow!("event_id={} invalid timestamp components", event_id)
    })
}

async fn upsert_snapshot(
    pool: &PgPool,
    height: i64,
    snapshot_hash: &str,
    prev_snapshot_hash: Option<&str>,
    state_root_hash: &str,
    title_sentence_payload_root: &str,
    shared_map_commitment: &str,
    active_rulebook_set_hash: &str,
    last_event_id: Uuid,
    event_count: i64,
    approximate_timestamp: DateTime<Utc>,
    cycle_index: i64,
    cycle_close_height: Option<i64>,
    artifact_path: &str,
    artifact_sha256: &str,
) -> Result<(), anyhow::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET LOCAL seed.allow_canonical_mutation = 'on'")
        .execute(&mut *tx)
        .await?;

    let snapshot_id = Uuid::now_v7();
    sqlx::query(
        r#"
        INSERT INTO snapshots (
          snapshot_id,
          block_height,
          format_version,
          snapshot_hash,
          prev_snapshot_hash,
          state_root_hash,
          title_sentence_payload_root,
          shared_map_commitment,
          active_rulebook_set_hash,
          last_event_id,
          event_count,
          approximate_timestamp,
          cycle_index,
          cycle_close_height,
          artifact_path,
          artifact_sha256,
          created_at
        ) VALUES (
          $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, NOW()
        )
        ON CONFLICT (block_height) DO UPDATE SET
          format_version = EXCLUDED.format_version,
          snapshot_hash = EXCLUDED.snapshot_hash,
          prev_snapshot_hash = EXCLUDED.prev_snapshot_hash,
          state_root_hash = EXCLUDED.state_root_hash,
          title_sentence_payload_root = EXCLUDED.title_sentence_payload_root,
          shared_map_commitment = EXCLUDED.shared_map_commitment,
          active_rulebook_set_hash = EXCLUDED.active_rulebook_set_hash,
          last_event_id = EXCLUDED.last_event_id,
          event_count = EXCLUDED.event_count,
          approximate_timestamp = EXCLUDED.approximate_timestamp,
          cycle_index = EXCLUDED.cycle_index,
          cycle_close_height = EXCLUDED.cycle_close_height,
          artifact_path = EXCLUDED.artifact_path,
          artifact_sha256 = EXCLUDED.artifact_sha256,
          created_at = EXCLUDED.created_at
        "#,
    )
    .bind(snapshot_id)
    .bind(height)
    .bind("0")
    .bind(snapshot_hash)
    .bind(prev_snapshot_hash)
    .bind(state_root_hash)
    .bind(title_sentence_payload_root)
    .bind(shared_map_commitment)
    .bind(active_rulebook_set_hash)
    .bind(last_event_id)
    .bind(event_count)
    .bind(approximate_timestamp)
    .bind(cycle_index)
    .bind(cycle_close_height)
    .bind(artifact_path)
    .bind(artifact_sha256)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

fn is_hex_64(value: &str) -> bool {
    if value.len() != 64 {
        return false;
    }
    value
        .as_bytes()
        .iter()
        .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_snapshot_commit_only_on_interval_boundary() {
        assert!(should_emit_snapshot_commit(100, 100));
        assert!(should_emit_snapshot_commit(200, 100));
        assert!(!should_emit_snapshot_commit(99, 100));
        assert!(!should_emit_snapshot_commit(101, 100));
    }

    #[test]
    fn accepts_matching_existing_snapshot_commit() {
        let event_id = Uuid::parse_str("00000000-0000-7000-8000-000000000201").expect("uuid");
        let existing = SnapshotCommitRow {
            block_height: 100,
            snapshot_hash: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .to_string(),
            state_root_hash: "1123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .to_string(),
            title_sentence_payload_root:
                "2123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .to_string(),
            shared_map_commitment:
                "3123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .to_string(),
            last_event_id: Uuid::parse_str("00000000-0000-7000-8000-000000000111")
                .expect("uuid"),
            event_count: 250,
            active_rulebook_set_hash:
                "4123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .to_string(),
            created_event_id: event_id,
        };
        let commit = SnapshotCommitMaterialization {
            block_height: 100,
            snapshot_hash: existing.snapshot_hash.clone(),
            state_root_hash: existing.state_root_hash.clone(),
            title_sentence_payload_root: existing.title_sentence_payload_root.clone(),
            shared_map_commitment: existing.shared_map_commitment.clone(),
            last_event_id: existing.last_event_id,
            event_count: existing.event_count,
            active_rulebook_set_hash: existing.active_rulebook_set_hash.clone(),
        };

        assert!(ensure_snapshot_commit_matches(&existing, &commit).is_ok());
    }
}
