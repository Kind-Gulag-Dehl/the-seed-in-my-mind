use super::*;
use common::security_limits::{
    CANONICAL_AXIS_MAX_CHARS, CANONICAL_CONNECTION_TYPE_MAX_CHARS, CANONICAL_CONTEXT_KEY_MAX_CHARS,
    CANONICAL_IDEA_TYPE_MAX_CHARS, CANONICAL_SCOPE_MAX_CHARS, CANONICAL_TIMEFRAME_MAX_CHARS,
    CANONICAL_USAGE_MAX_CHARS, CANONICAL_VOTE_CHOICE_MAX_CHARS, IDEA_FULL_MAX_CHARS,
    IDEA_PARAGRAPH_MAX_CHARS, IDEA_SENTENCE_MAX_CHARS, IDEA_TITLE_MAX_CHARS,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::fs;
use std::sync::OnceLock;

const SECRET_DETECTED_MESSAGE: &str = "canonical payload rejected: secret-like content detected";
const CYCLE_CLOSE_SIGNATURE_DOMAIN: &[u8] = b"seed.cycle_close.v1";
const SYSTEM_BOUNDARY_SIGNING_KEY_ENV_KEYS: [&str; 2] = [
    "SYSTEM_BOUNDARY_EMITTER_HMAC_KEY",
    "SNAPSHOT_COMMIT_HMAC_KEY",
];
const SYSTEM_BOUNDARY_SIGNING_KEY_FILE_ENV_KEYS: [&str; 2] = [
    "SYSTEM_BOUNDARY_EMITTER_HMAC_KEY_FILE",
    "SNAPSHOT_COMMIT_HMAC_KEY_FILE",
];
const SYSTEM_BOUNDARY_MODE_ENV_KEYS: [&str; 3] = ["SEED_ENV", "APP_ENV", "RUST_ENV"];
const DEV_CYCLE_CLOSE_SIGNING_KEY: &[u8] = b"seed.cycle_close.dev_key.v1";
static SYSTEM_BOUNDARY_SIGNING_KEY: OnceLock<Vec<u8>> = OnceLock::new();

type HmacSha256 = Hmac<Sha256>;

fn reject_secret_like_text(value: &str) -> std::result::Result<(), CanonicalWriteError> {
    if screen_text_for_secrets(value).is_some() {
        return Err(CanonicalWriteError::new(
            "secret_detected",
            SECRET_DETECTED_MESSAGE,
        ));
    }
    Ok(())
}

fn reject_secret_like_optional_text(
    value: Option<&str>,
) -> std::result::Result<(), CanonicalWriteError> {
    if let Some(value) = value {
        reject_secret_like_text(value)?;
    }
    Ok(())
}

fn reject_text_too_long(
    field: &str,
    value: &str,
    max_chars: usize,
) -> std::result::Result<(), CanonicalWriteError> {
    if value.chars().count() > max_chars {
        return Err(CanonicalWriteError::new(
            "invalid_field",
            format!("{field} exceeds maximum length of {max_chars} characters"),
        ));
    }
    Ok(())
}

fn reject_optional_text_too_long(
    field: &str,
    value: Option<&str>,
    max_chars: usize,
) -> std::result::Result<(), CanonicalWriteError> {
    if let Some(value) = value {
        reject_text_too_long(field, value, max_chars)?;
    }
    Ok(())
}

async fn maybe_emit_cycle_close_before_append(
    tx: &mut Transaction<'_, Postgres>,
) -> std::result::Result<Option<Uuid>, CanonicalWriteError> {
    let Some(decision) = maybe_load_cycle_close_decision(tx).await? else {
        return Ok(None);
    };

    let (closure_kind, forced_seal, closure_kind_code) = match decision.disposition {
        CycleCloseDisposition::Deliberative => ("deliberative", false, 0_i16),
        CycleCloseDisposition::Forced => ("forced", true, 1_i16),
    };
    let event_id = Uuid::now_v7();
    let payload = json!({
        "cycle_index": decision.cycle_index,
        "closure_kind": closure_kind,
        "forced_seal": forced_seal,
        "closure_boundary_ref": {
            "block_height": decision.position.block_height
        }
    });
    let event = Event {
        id: event_id,
        kind: "cycle_close".to_string(),
        payload: payload.clone(),
        speaker_identity_id: Some(system_boundary_emitter_id()),
    };
    // Stage 0 still materializes cycle boundaries with the legacy cycle_close payload
    // (`closure_kind`/`forced_seal`). Public Appendix A validation rejects that payload.
    validate_stage0_internal_event(&event).map_err(|err| {
        CanonicalWriteError::new(
            err.code,
            format!("event validation failed: {}", err.message),
        )
    })?;
    let signature = Some(sign_cycle_close(&event)?);

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
    .bind(decision.position.block_height)
    .bind(decision.position.event_index)
    .bind(event_id)
    .bind("cycle_close")
    .bind(Some(system_boundary_emitter_id()))
    .bind(payload)
    .bind(signature)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;
    persist_tempo_predicates(tx, decision.position, decision.tempo_state).await?;

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
    .bind(decision.cycle_index)
    .bind(closure_kind_code)
    .bind(forced_seal)
    .bind(decision.position.block_height)
    .bind(decision.position.block_height)
    .bind(decision.position.event_index)
    .bind(event_id)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;

    ensure_block_row(tx, decision.position.block_height + 1).await?;
    Ok(Some(event_id))
}

fn sign_cycle_close(event: &Event) -> std::result::Result<String, CanonicalWriteError> {
    let key = resolve_system_boundary_signing_key()?;
    let payload = event.payload.as_object().ok_or_else(|| {
        CanonicalWriteError::new("invalid_request", "cycle_close payload must be object")
    })?;
    let mut message = Vec::new();
    message.extend_from_slice(CYCLE_CLOSE_SIGNATURE_DOMAIN);
    message.push(0);
    message.extend_from_slice(event.kind.as_bytes());
    message.push(0);
    message.extend_from_slice(event.id.to_string().as_bytes());
    message.push(0);
    message.extend_from_slice(system_boundary_emitter_id().to_string().as_bytes());
    for field in [
        "cycle_index",
        "closure_kind",
        "forced_seal",
        "closure_boundary_ref",
    ] {
        let value = payload.get(field).ok_or_else(|| {
            CanonicalWriteError::new(
                "invalid_request",
                format!("cycle_close payload missing {}", field),
            )
        })?;
        message.push(0);
        message.extend_from_slice(field.as_bytes());
        message.push(0);
        message.extend_from_slice(value.to_string().as_bytes());
    }

    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|err| CanonicalWriteError::new("storage_error", err.to_string()))?;
    mac.update(&message);
    let digest = mac.finalize().into_bytes();
    Ok(format!("hmac-sha256:{}", bytes_to_hex(&digest)))
}

fn resolve_system_boundary_signing_key() -> std::result::Result<&'static [u8], CanonicalWriteError>
{
    if let Some(key) = SYSTEM_BOUNDARY_SIGNING_KEY.get() {
        return Ok(key.as_slice());
    }

    let computed = compute_system_boundary_signing_key()?;
    let _ = SYSTEM_BOUNDARY_SIGNING_KEY.set(computed);
    let key = SYSTEM_BOUNDARY_SIGNING_KEY.get().ok_or_else(|| {
        CanonicalWriteError::new(
            "storage_error",
            "system boundary signing key initialization failed",
        )
    })?;
    Ok(key.as_slice())
}

fn compute_system_boundary_signing_key() -> std::result::Result<Vec<u8>, CanonicalWriteError> {
    for key_name in SYSTEM_BOUNDARY_SIGNING_KEY_ENV_KEYS {
        if let Ok(value) = std::env::var(key_name) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Ok(trimmed.as_bytes().to_vec());
            }
        }
    }

    for key_name in SYSTEM_BOUNDARY_SIGNING_KEY_FILE_ENV_KEYS {
        if let Ok(path) = std::env::var(key_name) {
            let trimmed = path.trim();
            if trimmed.is_empty() {
                continue;
            }
            let value = fs::read_to_string(trimmed)
                .map_err(|err| CanonicalWriteError::new("storage_error", err.to_string()))?;
            let trimmed_value = value.trim();
            if !trimmed_value.is_empty() {
                return Ok(trimmed_value.as_bytes().to_vec());
            }
        }
    }

    if is_production_mode() {
        return Err(CanonicalWriteError::new(
            "storage_error",
            "SYSTEM_BOUNDARY_EMITTER_HMAC_KEY is required in production mode",
        ));
    }

    eprintln!(
        "[security] SYSTEM_BOUNDARY_EMITTER_HMAC_KEY missing; using deterministic dev-only cycle_close signing key"
    );
    Ok(DEV_CYCLE_CLOSE_SIGNING_KEY.to_vec())
}

fn is_production_mode() -> bool {
    SYSTEM_BOUNDARY_MODE_ENV_KEYS.iter().any(|key_name| {
        if let Ok(value) = std::env::var(key_name) {
            return matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "prod" | "production"
            );
        }
        false
    })
}

const PUBLICATION_PROFILE_BOOTSTRAP_SINGLE_PUBLISHER: &str = "profile_0_bootstrap_single_publisher";

#[derive(Debug, Clone, sqlx::FromRow)]
struct ExistingSignedEventRow {
    block_height: i64,
    event_index: i32,
    event_type: String,
    authored_candidate_hash_v0: Option<String>,
    signature: Option<String>,
    payload_json: serde_json::Value,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct IdentityKeyStateRow {
    public_key_ref: String,
    public_key_bytes: Vec<u8>,
    is_active: bool,
}

#[derive(Debug, Clone, Copy)]
struct SignedWriteObject {
    object_id: Uuid,
    object_type: &'static str,
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

fn storage_candidate_to_verification(
    input: &SignedCanonicalCandidateInput,
) -> AuthoredEventCandidate {
    AuthoredEventCandidate {
        signature_profile: input.signature_profile.clone(),
        event_id: input.event_id,
        event_type: input.event_type.clone(),
        author_identity_id: input.author_identity_id,
        speaker_identity_id: input.speaker_identity_id,
        public_key_ref: input.public_key_ref.clone(),
        payload_hash: input.payload_hash.clone(),
        payload_binding_mode: input.payload_binding_mode.clone(),
        payload_ref: input.payload_ref.clone(),
        author_observed_at: input.author_observed_at.clone(),
        signature: input.signature.clone(),
    }
}

fn signature_error(err: verification::signatures::SignatureValidationError) -> CanonicalWriteError {
    CanonicalWriteError::new(err.code, err.message)
}

fn validate_signed_public_payload(
    input: &SignedCanonicalCandidateInput,
    effective_speaker_identity_id: Uuid,
) -> std::result::Result<SignedWriteObject, CanonicalWriteError> {
    let payload = input
        .payload
        .as_object()
        .ok_or_else(|| CanonicalWriteError::new("invalid_payload", "payload must be object"))?;
    match input.event_type.as_str() {
        "idea_create" => {
            let idea_id = parse_payload_uuid(payload, "idea_id")?;
            let idea_type = parse_payload_string(payload, "idea_type")?;
            if idea_type == "identity" {
                return Err(CanonicalWriteError::new(
                    "unsupported_event_type",
                    "identity idea creation is outside this signed public write slice",
                ));
            }
            ensure_payload_speaker(payload, effective_speaker_identity_id)?;
            Ok(SignedWriteObject {
                object_id: idea_id,
                object_type: "idea",
            })
        }
        "connection_create" => {
            let connection_id = parse_payload_uuid(payload, "connection_id")?;
            ensure_payload_speaker(payload, effective_speaker_identity_id)?;
            Ok(SignedWriteObject {
                object_id: connection_id,
                object_type: "connection",
            })
        }
        _ => Err(CanonicalWriteError::new(
            "unsupported_event_type",
            "unsupported event type",
        )),
    }
}

fn parse_payload_uuid(
    payload: &serde_json::Map<String, serde_json::Value>,
    field: &'static str,
) -> std::result::Result<Uuid, CanonicalWriteError> {
    let value = payload
        .get(field)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| CanonicalWriteError::new("missing_field", format!("{field} required")))?;
    Uuid::parse_str(value)
        .map_err(|_| CanonicalWriteError::new("invalid_id", format!("invalid {field}")))
}

fn parse_payload_string<'a>(
    payload: &'a serde_json::Map<String, serde_json::Value>,
    field: &'static str,
) -> std::result::Result<&'a str, CanonicalWriteError> {
    payload
        .get(field)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| CanonicalWriteError::new("missing_field", format!("{field} required")))
}

fn ensure_payload_speaker(
    payload: &serde_json::Map<String, serde_json::Value>,
    effective_speaker_identity_id: Uuid,
) -> std::result::Result<(), CanonicalWriteError> {
    let payload_speaker = parse_payload_uuid(payload, "speaker_identity_id")?;
    if payload_speaker != effective_speaker_identity_id {
        return Err(CanonicalWriteError::new(
            "invalid_field",
            "payload speaker_identity_id must match the signed candidate speaker/author",
        ));
    }
    Ok(())
}

async fn load_existing_event_by_id(
    tx: &mut Transaction<'_, Postgres>,
    event_id: Uuid,
) -> std::result::Result<Option<ExistingSignedEventRow>, CanonicalWriteError> {
    sqlx::query_as::<_, ExistingSignedEventRow>(
        r#"
        SELECT
          block_height,
          event_index,
          event_type,
          authored_candidate_hash_v0,
          signature,
          payload_json
        FROM events
        WHERE event_id = $1
        FOR UPDATE
        "#,
    )
    .bind(event_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(canonical_storage_error)
}

async fn lock_identity_for_signed_write(
    tx: &mut Transaction<'_, Postgres>,
    identity_id: Uuid,
) -> std::result::Result<(), CanonicalWriteError> {
    let identity_lock_key = advisory_lock_key_for_uuid(identity_id);
    sqlx::query("SELECT pg_advisory_xact_lock($1::bigint)")
        .bind(identity_lock_key)
        .execute(&mut **tx)
        .await
        .map_err(canonical_storage_error)?;
    Ok(())
}

async fn ensure_signed_author_identity(
    tx: &mut Transaction<'_, Postgres>,
    identity_id: Uuid,
) -> std::result::Result<(), CanonicalWriteError> {
    if identity_id == system_boundary_emitter_id() {
        return Err(CanonicalWriteError::new(
            "forbidden",
            "system_boundary_emitter cannot author ordinary human signed writes",
        ));
    }
    ensure_identity_exists(tx, identity_id).await
}

async fn ensure_signed_writer_eligibility(
    tx: &mut Transaction<'_, Postgres>,
    identity_id: Uuid,
    position: EventInsertPosition,
) -> std::result::Result<(), CanonicalWriteError> {
    let row = sqlx::query_as::<_, IdentityWriterVerificationStateRow>(
        r#"
        SELECT
          identity_id,
          email_verified,
          canonical_writer_level,
          granted_by_identity_id,
          source_event_id,
          source_block_height,
          source_event_index
        FROM canonical_writer_verification_states
        WHERE identity_id = $1
          AND (
            source_block_height < $2
            OR (source_block_height = $2 AND source_event_index < $3)
          )
        ORDER BY source_block_height DESC, source_event_index DESC
        LIMIT 1
        FOR UPDATE
        "#,
    )
    .bind(identity_id)
    .bind(position.block_height)
    .bind(position.event_index)
    .fetch_optional(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;

    let Some(row) = row else {
        return Err(CanonicalWriteError::new(
            "forbidden",
            "canonical write requires verified-human canonical writer eligibility",
        ));
    };
    if !row.email_verified || row.canonical_writer_level < MIN_CANONICAL_WRITER_LEVEL {
        return Err(CanonicalWriteError::new(
            "forbidden",
            "canonical writer eligibility is inactive or revoked",
        ));
    }
    Ok(())
}

async fn load_active_identity_key_before_position(
    tx: &mut Transaction<'_, Postgres>,
    input: &SignedCanonicalCandidateInput,
    position: EventInsertPosition,
) -> std::result::Result<IdentityKeyStateRow, CanonicalWriteError> {
    let row = sqlx::query_as::<_, IdentityKeyStateRow>(
        r#"
        SELECT public_key_ref, public_key_bytes, is_active
        FROM canonical_identity_key_states
        WHERE public_key_ref = $1
          AND identity_id = $2
          AND signature_profile = $3
          AND signature_algorithm = 'ed25519'
          AND (
            source_block_height IS NULL
            OR source_block_height < $4
            OR (source_block_height = $4 AND source_event_index < $5)
          )
        ORDER BY source_block_height DESC NULLS LAST,
                 source_event_index DESC NULLS LAST,
                 created_at DESC
        LIMIT 1
        FOR UPDATE
        "#,
    )
    .bind(input.public_key_ref.as_str())
    .bind(input.author_identity_id)
    .bind(input.signature_profile.as_str())
    .bind(position.block_height)
    .bind(position.event_index)
    .fetch_optional(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;

    let Some(row) = row else {
        return Err(CanonicalWriteError::new(
            "unknown_key",
            "public_key_ref is not registered for the author identity",
        ));
    };
    if !row.is_active {
        return Err(CanonicalWriteError::new(
            "revoked_key",
            "public_key_ref is not active at the candidate publication point",
        ));
    }
    Ok(row)
}

async fn validate_signed_context(
    tx: &mut Transaction<'_, Postgres>,
    input: &SignedCanonicalCandidateInput,
    object: SignedWriteObject,
    position: EventInsertPosition,
) -> std::result::Result<(), CanonicalWriteError> {
    let payload = input
        .payload
        .as_object()
        .ok_or_else(|| CanonicalWriteError::new("invalid_payload", "payload must be object"))?;
    match input.event_type.as_str() {
        "idea_create" => {
            let exists: bool =
                sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM ideas WHERE idea_id = $1)")
                    .bind(object.object_id)
                    .fetch_one(&mut **tx)
                    .await
                    .map_err(canonical_storage_error)?;
            if exists {
                return Err(CanonicalWriteError::new(
                    "conflict",
                    "canonical idea_id already exists",
                ));
            }
        }
        "connection_create" => {
            let from_idea_id = parse_payload_uuid(payload, "from_idea_id")?;
            let to_idea_id = parse_payload_uuid(payload, "to_idea_id")?;
            ensure_idea_exists_before_position(tx, from_idea_id, position).await?;
            ensure_idea_exists_before_position(tx, to_idea_id, position).await?;
            let exists: bool = sqlx::query_scalar(
                "SELECT EXISTS (SELECT 1 FROM connections WHERE connection_id = $1)",
            )
            .bind(object.object_id)
            .fetch_one(&mut **tx)
            .await
            .map_err(canonical_storage_error)?;
            if exists {
                return Err(CanonicalWriteError::new(
                    "conflict",
                    "canonical connection_id already exists",
                ));
            }
        }
        _ => {
            return Err(CanonicalWriteError::new(
                "unsupported_event_type",
                "unsupported event type",
            ))
        }
    }
    Ok(())
}

async fn ensure_idea_exists_before_position(
    tx: &mut Transaction<'_, Postgres>,
    idea_id: Uuid,
    position: EventInsertPosition,
) -> std::result::Result<(), CanonicalWriteError> {
    let exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
          SELECT 1
          FROM ideas
          WHERE idea_id = $1
            AND (
              created_block_height < $2
              OR (created_block_height = $2 AND created_event_index < $3)
            )
        )
        "#,
    )
    .bind(idea_id)
    .bind(position.block_height)
    .bind(position.event_index)
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    if !exists {
        return Err(CanonicalWriteError::new(
            "invalid_request",
            "referenced idea does not exist at the validation position",
        ));
    }
    Ok(())
}

async fn insert_signed_event(
    tx: &mut Transaction<'_, Postgres>,
    input: &SignedCanonicalCandidateInput,
    effective_speaker_identity_id: Uuid,
    position: EventInsertPosition,
    signed_candidate_bytes_v0: &[u8],
    authored_candidate_hash_v0: &str,
) -> std::result::Result<(), CanonicalWriteError> {
    sqlx::query(
        r#"
        INSERT INTO events (
          block_height,
          event_index,
          event_id,
          event_type,
          speaker_identity_id,
          payload_json,
          signature,
          signature_profile,
          author_identity_id,
          public_key_ref,
          payload_hash,
          payload_binding_mode,
          payload_ref,
          author_observed_at,
          signed_candidate_bytes_v0,
          authored_candidate_hash_v0,
          publication_profile
        ) VALUES (
          $1, $2, $3, $4, $5, $6, $7,
          $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
        )
        "#,
    )
    .bind(position.block_height)
    .bind(position.event_index)
    .bind(input.event_id)
    .bind(input.event_type.as_str())
    .bind(Some(effective_speaker_identity_id))
    .bind(input.payload.clone())
    .bind(Some(input.signature.as_str()))
    .bind(Some(input.signature_profile.as_str()))
    .bind(Some(input.author_identity_id))
    .bind(Some(input.public_key_ref.as_str()))
    .bind(Some(input.payload_hash.as_str()))
    .bind(Some(input.payload_binding_mode.as_str()))
    .bind(input.payload_ref.as_deref())
    .bind(input.author_observed_at.as_deref())
    .bind(signed_candidate_bytes_v0)
    .bind(Some(authored_candidate_hash_v0))
    .bind(Some(PUBLICATION_PROFILE_BOOTSTRAP_SINGLE_PUBLISHER))
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;
    Ok(())
}

async fn materialize_signed_object(
    tx: &mut Transaction<'_, Postgres>,
    input: &SignedCanonicalCandidateInput,
    object: SignedWriteObject,
    position: EventInsertPosition,
) -> std::result::Result<(), CanonicalWriteError> {
    let payload = input
        .payload
        .as_object()
        .ok_or_else(|| CanonicalWriteError::new("invalid_payload", "payload must be object"))?;
    match input.event_type.as_str() {
        "idea_create" => {
            let idea_type = parse_payload_string(payload, "idea_type")?;
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
                ) VALUES (
                  $1, $2, $3, false, NULL, false, NULL, NULL, $4, $5, $6
                )
                "#,
            )
            .bind(object.object_id)
            .bind(idea_type)
            .bind(
                input
                    .speaker_identity_id
                    .unwrap_or(input.author_identity_id),
            )
            .bind(position.block_height)
            .bind(position.event_index)
            .bind(input.event_id)
            .execute(&mut **tx)
            .await
            .map_err(map_canonical_write_sqlx_error)?;
        }
        "connection_create" => {
            let from_idea_id = parse_payload_uuid(payload, "from_idea_id")?;
            let to_idea_id = parse_payload_uuid(payload, "to_idea_id")?;
            let connection_type = parse_payload_string(payload, "connection_type")?;
            let usage = payload
                .get("usage")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string);
            let axis = payload
                .get("axis")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string);
            let timeframe = payload
                .get("timeframe")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string);
            let scope = payload
                .get("scope")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string);

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
                ) VALUES (
                  $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
                )
                "#,
            )
            .bind(object.object_id)
            .bind(from_idea_id)
            .bind(to_idea_id)
            .bind(connection_type)
            .bind(usage)
            .bind(axis)
            .bind(timeframe)
            .bind(scope)
            .bind(position.block_height)
            .bind(position.event_index)
            .bind(input.event_id)
            .execute(&mut **tx)
            .await
            .map_err(map_canonical_write_sqlx_error)?;
        }
        _ => {
            return Err(CanonicalWriteError::new(
                "unsupported_event_type",
                "unsupported event type",
            ))
        }
    }
    Ok(())
}

impl Storage {
    pub async fn submit_signed_canonical_candidate(
        &self,
        input: SignedCanonicalCandidateInput,
    ) -> std::result::Result<SignedCanonicalWriteResult, CanonicalWriteError> {
        if input.payload_binding_mode != PAYLOAD_BINDING_EMBEDDED {
            return Err(CanonicalWriteError::new(
                "unsupported_payload_binding",
                "this runtime supports only embedded_payload candidates",
            ));
        }
        if input.payload_ref.is_some() {
            return Err(CanonicalWriteError::new(
                "unsupported_payload_binding",
                "payload_ref transport is not implemented by this runtime profile",
            ));
        }
        if !matches!(
            input.event_type.as_str(),
            "idea_create" | "connection_create"
        ) {
            return Err(CanonicalWriteError::new(
                "unsupported_event_type",
                format!("unsupported public signed event type: {}", input.event_type),
            ));
        }

        let computed_payload_hash = canonical_json_payload_hash_hex(&input.payload)
            .map_err(|err| CanonicalWriteError::new("canonical_encoding_failed", err))?;
        if computed_payload_hash != input.payload_hash {
            return Err(CanonicalWriteError::new(
                "invalid_payload_hash",
                "candidate payload_hash does not match canonical embedded payload bytes",
            ));
        }

        let effective_speaker_identity_id = input
            .speaker_identity_id
            .unwrap_or(input.author_identity_id);
        let object = validate_signed_public_payload(&input, effective_speaker_identity_id)?;
        let event = Event {
            id: input.event_id,
            kind: input.event_type.clone(),
            payload: input.payload.clone(),
            speaker_identity_id: Some(effective_speaker_identity_id),
        };
        validate_event(&event).map_err(|err| {
            CanonicalWriteError::new(
                err.code,
                format!("event validation failed: {}", err.message),
            )
        })?;

        let candidate = storage_candidate_to_verification(&input);
        let signed_bytes = signed_candidate_bytes_v0(&candidate).map_err(signature_error)?;
        let signature_bytes = decode_signature64(&candidate.signature).map_err(signature_error)?;
        let candidate_hash =
            authored_candidate_hash_v0(&signed_bytes, &signature_bytes).map_err(signature_error)?;

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        if let Some(existing) = load_existing_event_by_id(&mut tx, input.event_id).await? {
            if existing.authored_candidate_hash_v0.as_deref() == Some(candidate_hash.as_str())
                && existing.signature.as_deref() == Some(input.signature.as_str())
                && existing.payload_json == input.payload
                && existing.event_type == input.event_type
            {
                tx.commit()
                    .await
                    .map_err(|err| canonical_storage_error(err))?;
                return Ok(SignedCanonicalWriteResult {
                    event_id: input.event_id,
                    event_type: input.event_type,
                    block_height: existing.block_height,
                    event_index: existing.event_index,
                    authored_candidate_hash_v0: candidate_hash,
                    object_type: object.object_type.to_string(),
                    object_id: object.object_id,
                    idempotent: true,
                    publication_profile: PUBLICATION_PROFILE_BOOTSTRAP_SINGLE_PUBLISHER.to_string(),
                });
            }
            return Err(CanonicalWriteError::new(
                "conflict",
                "event_id already exists with different candidate bytes",
            ));
        }

        let position = allocate_canonical_event_position(&mut tx).await?;
        lock_identity_for_signed_write(&mut tx, input.author_identity_id).await?;
        ensure_signed_author_identity(&mut tx, input.author_identity_id).await?;
        ensure_signed_writer_eligibility(&mut tx, input.author_identity_id, position).await?;
        let key_state = load_active_identity_key_before_position(&mut tx, &input, position).await?;
        let expected_ref = public_key_ref_v0(&key_state.public_key_bytes, input.author_identity_id)
            .map_err(signature_error)?;
        if key_state.public_key_ref != expected_ref {
            return Err(CanonicalWriteError::new(
                "unknown_key",
                "stored key descriptor does not match public_key_ref",
            ));
        }
        let verified =
            verify_ed25519_v0(&candidate, &key_state.public_key_bytes).map_err(signature_error)?;
        if verified.authored_candidate_hash_v0 != candidate_hash
            || verified.signed_candidate_bytes_v0 != signed_bytes
        {
            return Err(CanonicalWriteError::new(
                "invalid_signature",
                "candidate verification did not reproduce candidate commitments",
            ));
        }
        validate_signed_context(&mut tx, &input, object, position).await?;

        insert_signed_event(
            &mut tx,
            &input,
            effective_speaker_identity_id,
            position,
            &signed_bytes,
            &candidate_hash,
        )
        .await?;
        materialize_signed_object(&mut tx, &input, object, position).await?;

        tx.commit()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        Ok(SignedCanonicalWriteResult {
            event_id: input.event_id,
            event_type: input.event_type,
            block_height: position.block_height,
            event_index: position.event_index,
            authored_candidate_hash_v0: candidate_hash,
            object_type: object.object_type.to_string(),
            object_id: object.object_id,
            idempotent: false,
            publication_profile: PUBLICATION_PROFILE_BOOTSTRAP_SINGLE_PUBLISHER.to_string(),
        })
    }

    pub async fn grant_canonical_writer_level(
        &self,
        account_id: Uuid,
        input: CanonicalVerifierGrantInput,
    ) -> std::result::Result<CanonicalVerifierGrantResult, CanonicalWriteError> {
        if input.canonical_writer_level < MIN_CANONICAL_WRITER_LEVEL {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                format!(
                    "canonical_writer_level must be >= {}",
                    MIN_CANONICAL_WRITER_LEVEL
                ),
            ));
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| canonical_storage_error(err))?;
        let verifier_identity_id = load_canonical_verifier_identity(&mut tx, account_id).await?;
        ensure_identity_exists(&mut tx, input.identity_id).await?;
        let _ = maybe_emit_cycle_close_before_append(&mut tx).await?;

        let cycle_window = load_cycle_window(&mut tx).await?;
        let payload = json!({
            "identity_id": input.identity_id,
            "canonical_writer_level": input.canonical_writer_level,
            "email_verified": input.email_verified
        });
        let author_signature = input.author_signature.clone();
        let event_id = input.event_id.unwrap_or_else(Uuid::now_v7);
        let event = Event {
            id: event_id,
            kind: "canonical_writer_grant".to_string(),
            payload: payload.clone(),
            speaker_identity_id: Some(verifier_identity_id),
        };
        // Stage 0 writer grants are implementation/bootstrap events, not public Protocol v5 events.
        validate_stage0_internal_event(&event).map_err(|err| {
            CanonicalWriteError::new(
                err.code,
                format!("event validation failed: {}", err.message),
            )
        })?;

        let position = allocate_canonical_event_position(&mut tx).await?;
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
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .bind("canonical_writer_grant")
        .bind(Some(verifier_identity_id))
        .bind(payload)
        .bind(author_signature)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
        let _ = refresh_tempo_predicates_for_position(&mut tx, position).await?;

        sqlx::query(
            r#"
            INSERT INTO canonical_writer_verification_states (
              identity_id,
              email_verified,
              canonical_writer_level,
              granted_by_identity_id,
              source_event_id,
              source_block_height,
              source_event_index
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(input.identity_id)
        .bind(input.email_verified)
        .bind(input.canonical_writer_level)
        .bind(verifier_identity_id)
        .bind(event_id)
        .bind(position.block_height)
        .bind(position.event_index)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        tx.commit()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        Ok(CanonicalVerifierGrantResult {
            identity_id: input.identity_id,
            event_id,
            canonical_writer_level: input.canonical_writer_level,
            email_verified: input.email_verified,
            cycle_index: cycle_window.cycle_index,
        })
    }

    pub async fn revoke_canonical_writer_level(
        &self,
        account_id: Uuid,
        input: CanonicalVerifierRevokeInput,
    ) -> std::result::Result<CanonicalVerifierRevokeResult, CanonicalWriteError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| canonical_storage_error(err))?;
        let verifier_identity_id = load_canonical_verifier_identity(&mut tx, account_id).await?;
        ensure_identity_exists(&mut tx, input.identity_id).await?;
        let _ = maybe_emit_cycle_close_before_append(&mut tx).await?;

        let cycle_window = load_cycle_window(&mut tx).await?;
        let payload = json!({
            "identity_id": input.identity_id
        });
        let author_signature = input.author_signature.clone();
        let event_id = input.event_id.unwrap_or_else(Uuid::now_v7);
        let event = Event {
            id: event_id,
            kind: "canonical_writer_revoke".to_string(),
            payload: payload.clone(),
            speaker_identity_id: Some(verifier_identity_id),
        };
        // Stage 0 writer revokes are implementation/bootstrap events, not public Protocol v5 events.
        validate_stage0_internal_event(&event).map_err(|err| {
            CanonicalWriteError::new(
                err.code,
                format!("event validation failed: {}", err.message),
            )
        })?;

        let position = allocate_canonical_event_position(&mut tx).await?;
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
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .bind("canonical_writer_revoke")
        .bind(Some(verifier_identity_id))
        .bind(payload)
        .bind(author_signature)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
        let _ = refresh_tempo_predicates_for_position(&mut tx, position).await?;

        sqlx::query(
            r#"
            INSERT INTO canonical_writer_verification_states (
              identity_id,
              email_verified,
              canonical_writer_level,
              granted_by_identity_id,
              source_event_id,
              source_block_height,
              source_event_index
            ) VALUES ($1, false, 0, $2, $3, $4, $5)
            "#,
        )
        .bind(input.identity_id)
        .bind(verifier_identity_id)
        .bind(event_id)
        .bind(position.block_height)
        .bind(position.event_index)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        tx.commit()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        Ok(CanonicalVerifierRevokeResult {
            identity_id: input.identity_id,
            event_id,
            cycle_index: cycle_window.cycle_index,
        })
    }

    pub async fn get_canonical_verification_status(
        &self,
        identity_id: Uuid,
    ) -> Result<Option<CanonicalVerificationStatus>> {
        let mut tx = self.pool.begin().await?;
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM identities_s0 WHERE identity_id = $1)",
        )
        .bind(identity_id)
        .fetch_one(&mut *tx)
        .await?;
        if !exists {
            tx.rollback().await?;
            return Ok(None);
        }

        let writer_state = load_latest_writer_state(&mut tx, identity_id, false).await?;
        let active_verifier = load_active_verifier_state(&mut tx, identity_id, false).await?;
        tx.commit().await?;

        Ok(Some(CanonicalVerificationStatus {
            identity_id,
            email_verified: writer_state
                .as_ref()
                .map(|row| row.email_verified)
                .unwrap_or(false),
            canonical_writer_level: writer_state
                .as_ref()
                .map(|row| row.canonical_writer_level)
                .unwrap_or(0),
            active_verifier,
            last_updated_event_id: writer_state.as_ref().map(|row| row.source_event_id),
            last_updated_block_height: writer_state.as_ref().map(|row| row.source_block_height),
            last_updated_event_index: writer_state.as_ref().map(|row| row.source_event_index),
        }))
    }

    pub async fn create_canonical_identity(
        &self,
        account_id: Uuid,
        input: CanonicalIdentityCreateInput,
    ) -> std::result::Result<CanonicalIdentityCreateResult, CanonicalWriteError> {
        let identity_name = input.identity_name.trim();
        if identity_name.is_empty() {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "identity_name is required",
            ));
        }
        reject_text_too_long("identity_name", identity_name, IDEA_TITLE_MAX_CHARS)?;
        reject_secret_like_text(identity_name)?;

        let public_key = input.public_key.trim();
        if public_key.is_empty() {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "public_key is required",
            ));
        }
        reject_secret_like_text(public_key)?;

        let verification_reference = input
            .metadata
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                CanonicalWriteError::new(
                    "invalid_request",
                    "metadata is required for identity_create",
                )
            })?;
        reject_secret_like_text(verification_reference)?;

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        let existing_identity_id: Option<Uuid> = sqlx::query_scalar(
            "SELECT canonical_identity_id FROM accounts WHERE account_id = $1 FOR UPDATE",
        )
        .bind(account_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(canonical_storage_error)?
        .flatten();

        if existing_identity_id.is_some() {
            return Err(CanonicalWriteError::new(
                "conflict",
                "account already has a canonical identity",
            ));
        }
        let _ = maybe_emit_cycle_close_before_append(&mut tx).await?;

        let event_id = input.event_id.unwrap_or_else(Uuid::now_v7);
        let payload = json!({
            "identity_id": input.identity_id,
            "title": identity_name,
            "initial_public_key_ref": public_key,
            "verification_reference": verification_reference,
            "speaker_identity_id": input.identity_id
        });
        let event = Event {
            id: event_id,
            kind: "identity_create".to_string(),
            payload: payload.clone(),
            speaker_identity_id: Some(input.identity_id),
        };
        validate_event(&event).map_err(|err| {
            CanonicalWriteError::new(
                err.code,
                format!("event validation failed: {}", err.message),
            )
        })?;

        let position = allocate_canonical_event_position(&mut tx).await?;
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
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .bind("identity_create")
        .bind(Some(input.identity_id))
        .bind(payload)
        .bind(input.author_signature.clone())
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
        let _ = refresh_tempo_predicates_for_position(&mut tx, position).await?;

        sqlx::query(
            r#"
            INSERT INTO identities_s0 (
              identity_id,
              title,
              created_event_id
            ) VALUES ($1, $2, $3)
            "#,
        )
        .bind(input.identity_id)
        .bind(identity_name)
        .bind(event_id)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        sqlx::query(
            r#"
            UPDATE accounts
            SET canonical_identity_id = $2
            WHERE account_id = $1
            "#,
        )
        .bind(account_id)
        .bind(input.identity_id)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        tx.commit()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        Ok(CanonicalIdentityCreateResult {
            identity_id: input.identity_id,
            event_id,
        })
    }

    pub async fn create_blocked_submission(
        &self,
        account_id: Uuid,
        input: CanonicalBlockedSubmissionInput,
    ) -> std::result::Result<CanonicalBlockedSubmissionResult, CanonicalWriteError> {
        let blocked_reason_code = input.blocked_reason_code.trim();
        if blocked_reason_code.is_empty() {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "blocked_reason_code is required",
            ));
        }
        reject_secret_like_text(blocked_reason_code)?;
        let safe_summary_ref = input.safe_summary_ref.trim();
        if safe_summary_ref.is_empty() {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "safe_summary_ref is required",
            ));
        }
        reject_secret_like_text(safe_summary_ref)?;
        let classifier_profile_ref = input.classifier_profile_ref.trim();
        if classifier_profile_ref.is_empty() {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "classifier_profile_ref is required",
            ));
        }
        reject_secret_like_text(classifier_profile_ref)?;
        let rulebook_ref = input.rulebook_ref.trim();
        if rulebook_ref.is_empty() {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "rulebook_ref is required",
            ));
        }
        reject_secret_like_text(rulebook_ref)?;

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| canonical_storage_error(err))?;
        let verifier_identity_id = load_canonical_verifier_identity(&mut tx, account_id).await?;
        let _ = maybe_emit_cycle_close_before_append(&mut tx).await?;
        if input.blocked_by_identity != verifier_identity_id {
            return Err(CanonicalWriteError::new(
                "forbidden",
                "blocked_submission must be authored by the authenticated verifier identity",
            ));
        }

        if let Some(reference_event_id) = input.reference_event_id {
            let exists: bool =
                sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM events WHERE event_id = $1)")
                    .bind(reference_event_id)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(canonical_storage_error)?;
            if !exists {
                return Err(CanonicalWriteError::new(
                    "invalid_request",
                    "reference_event_id not found",
                ));
            }
        }

        let cycle_window = load_cycle_window(&mut tx).await?;
        let payload = json!({
            "submission_hash": input.submission_hash,
            "blocked_reason_code": blocked_reason_code,
            "blocked_by_identity": verifier_identity_id,
            "safe_summary_ref": safe_summary_ref,
            "classifier_profile_ref": classifier_profile_ref,
            "rulebook_ref": rulebook_ref,
            "wrongful_block_challenge_ref": input.wrongful_block_challenge_ref,
            "reference_event_id": input.reference_event_id
        });
        let event_id = input.event_id.unwrap_or_else(Uuid::now_v7);
        let event = Event {
            id: event_id,
            kind: "blocked_submission".to_string(),
            payload: payload.clone(),
            speaker_identity_id: Some(verifier_identity_id),
        };
        validate_event(&event).map_err(|err| {
            CanonicalWriteError::new(
                err.code,
                format!("event validation failed: {}", err.message),
            )
        })?;

        let position = allocate_canonical_event_position(&mut tx).await?;
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
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .bind("blocked_submission")
        .bind(Some(verifier_identity_id))
        .bind(payload)
        .bind(input.author_signature.clone())
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
        let _ = refresh_tempo_predicates_for_position(&mut tx, position).await?;

        tx.commit()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        Ok(CanonicalBlockedSubmissionResult {
            event_id,
            cycle_index: cycle_window.cycle_index,
        })
    }

    pub async fn create_canonical_idea(
        &self,
        account_id: Uuid,
        input: CanonicalIdeaCreateInput,
    ) -> std::result::Result<CanonicalIdeaCreateResult, CanonicalWriteError> {
        reject_text_too_long(
            "idea_type",
            input.idea_type.as_str(),
            CANONICAL_IDEA_TYPE_MAX_CHARS,
        )?;
        reject_text_too_long("title", input.title.as_str(), IDEA_TITLE_MAX_CHARS)?;
        reject_text_too_long("sentence", input.sentence.as_str(), IDEA_SENTENCE_MAX_CHARS)?;
        reject_optional_text_too_long(
            "paragraph",
            input.paragraph.as_deref(),
            IDEA_PARAGRAPH_MAX_CHARS,
        )?;
        reject_optional_text_too_long("full", input.full.as_deref(), IDEA_FULL_MAX_CHARS)?;

        reject_secret_like_text(input.idea_type.as_str())?;
        reject_secret_like_text(input.title.as_str())?;
        reject_secret_like_text(input.sentence.as_str())?;
        reject_secret_like_optional_text(input.paragraph.as_deref())?;
        reject_secret_like_optional_text(input.full.as_deref())?;

        let title = input.title.trim();
        let sentence = input.sentence.trim();
        if title.is_empty() || sentence.is_empty() {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "title and sentence are required",
            ));
        }
        if !is_valid_idea_type(&input.idea_type) || input.idea_type == "identity" {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "idea_type must be one of truth_claim, conceptual_idea, actionable_idea, action",
            ));
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| canonical_storage_error(err))?;
        let writer_identity_id = load_canonical_writer_identity(&mut tx, account_id).await?;
        let _ = maybe_emit_cycle_close_before_append(&mut tx).await?;
        let cycle_window = load_cycle_window(&mut tx).await?;
        let spent_mana =
            load_cycle_build_spend(&mut tx, writer_identity_id, cycle_window.h_start).await?;
        let remaining_mana = BUILD_MANA_CYCLE_CAP.saturating_sub(spent_mana);
        if remaining_mana < IDEA_CREATE_MANA_COST {
            return Err(CanonicalWriteError::new(
                "insufficient_mana",
                format!(
                    "insufficient build mana: required={} remaining={}",
                    IDEA_CREATE_MANA_COST, remaining_mana
                ),
            ));
        }

        let payload_hash = payload_hash_hex(
            title,
            sentence,
            input.paragraph.as_deref(),
            input.full.as_deref(),
        )
        .map_err(|err| CanonicalWriteError::new("invalid_request", err))?;
        let payload = json!({
            "idea_id": input.idea_id,
            "idea_type": input.idea_type,
            "speaker_identity_id": writer_identity_id,
            "title": title,
            "sentence": sentence,
            "paragraph": input.paragraph,
            "full": input.full,
            "payload_hash": payload_hash
        });
        let author_signature = input.author_signature.clone();
        let event_id = input.event_id.unwrap_or_else(Uuid::now_v7);
        let event = Event {
            id: event_id,
            kind: "idea_create".to_string(),
            payload: payload.clone(),
            speaker_identity_id: Some(writer_identity_id),
        };
        validate_event(&event).map_err(|err| {
            CanonicalWriteError::new(
                err.code,
                format!("event validation failed: {}", err.message),
            )
        })?;

        let position = allocate_canonical_event_position(&mut tx).await?;
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
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .bind("idea_create")
        .bind(Some(writer_identity_id))
        .bind(payload)
        .bind(author_signature)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
        let _ = refresh_tempo_predicates_for_position(&mut tx, position).await?;

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
            ) VALUES (
              $1, $2, $3, false, NULL, false, NULL, NULL, $4, $5, $6
            )
            "#,
        )
        .bind(input.idea_id)
        .bind(input.idea_type)
        .bind(writer_identity_id)
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        tx.commit()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        Ok(CanonicalIdeaCreateResult {
            idea_id: input.idea_id,
            event_id,
            cycle_index: cycle_window.cycle_index,
            remaining_build_mana: remaining_mana - IDEA_CREATE_MANA_COST,
        })
    }

    pub async fn create_canonical_connection(
        &self,
        account_id: Uuid,
        input: CanonicalConnectionCreateInput,
    ) -> std::result::Result<CanonicalConnectionCreateResult, CanonicalWriteError> {
        reject_text_too_long(
            "connection_type",
            input.connection_type.as_str(),
            CANONICAL_CONNECTION_TYPE_MAX_CHARS,
        )?;
        reject_optional_text_too_long("usage", input.usage.as_deref(), CANONICAL_USAGE_MAX_CHARS)?;
        reject_optional_text_too_long("axis", input.axis.as_deref(), CANONICAL_AXIS_MAX_CHARS)?;
        reject_optional_text_too_long(
            "timeframe",
            input.timeframe.as_deref(),
            CANONICAL_TIMEFRAME_MAX_CHARS,
        )?;
        reject_optional_text_too_long("scope", input.scope.as_deref(), CANONICAL_SCOPE_MAX_CHARS)?;

        reject_secret_like_text(input.connection_type.as_str())?;
        reject_secret_like_optional_text(input.usage.as_deref())?;
        reject_secret_like_optional_text(input.axis.as_deref())?;
        reject_secret_like_optional_text(input.timeframe.as_deref())?;
        reject_secret_like_optional_text(input.scope.as_deref())?;

        if input.connection_type.trim().is_empty() {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "connection_type is required",
            ));
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| canonical_storage_error(err))?;
        let writer_identity_id = load_canonical_writer_identity(&mut tx, account_id).await?;
        let _ = maybe_emit_cycle_close_before_append(&mut tx).await?;
        let cycle_window = load_cycle_window(&mut tx).await?;
        let spent_mana =
            load_cycle_build_spend(&mut tx, writer_identity_id, cycle_window.h_start).await?;
        let remaining_mana = BUILD_MANA_CYCLE_CAP.saturating_sub(spent_mana);
        if remaining_mana < CONNECTION_CREATE_MANA_COST {
            return Err(CanonicalWriteError::new(
                "insufficient_mana",
                format!(
                    "insufficient build mana: required={} remaining={}",
                    CONNECTION_CREATE_MANA_COST, remaining_mana
                ),
            ));
        }

        let payload = json!({
            "connection_id": input.connection_id,
            "from_idea_id": input.from_idea_id,
            "to_idea_id": input.to_idea_id,
            "connection_type": input.connection_type,
            "usage": input.usage,
            "axis": input.axis,
            "timeframe": input.timeframe,
            "scope": input.scope,
            "speaker_identity_id": writer_identity_id
        });
        let author_signature = input.author_signature.clone();
        let event_id = input.event_id.unwrap_or_else(Uuid::now_v7);
        let event = Event {
            id: event_id,
            kind: "connection_create".to_string(),
            payload: payload.clone(),
            speaker_identity_id: Some(writer_identity_id),
        };
        validate_event(&event).map_err(|err| {
            CanonicalWriteError::new(
                err.code,
                format!("event validation failed: {}", err.message),
            )
        })?;

        let position = allocate_canonical_event_position(&mut tx).await?;
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
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .bind("connection_create")
        .bind(Some(writer_identity_id))
        .bind(payload)
        .bind(author_signature)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
        let _ = refresh_tempo_predicates_for_position(&mut tx, position).await?;

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
            ) VALUES (
              $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            )
            "#,
        )
        .bind(input.connection_id)
        .bind(input.from_idea_id)
        .bind(input.to_idea_id)
        .bind(input.connection_type)
        .bind(input.usage)
        .bind(input.axis)
        .bind(input.timeframe)
        .bind(input.scope)
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        tx.commit()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        Ok(CanonicalConnectionCreateResult {
            connection_id: input.connection_id,
            event_id,
            cycle_index: cycle_window.cycle_index,
            remaining_build_mana: remaining_mana - CONNECTION_CREATE_MANA_COST,
        })
    }

    pub async fn create_canonical_importance_challenge(
        &self,
        account_id: Uuid,
        input: CanonicalImportanceChallengeCreateInput,
    ) -> std::result::Result<CanonicalImportanceChallengeCreateResult, CanonicalWriteError> {
        reject_text_too_long(
            "context_key",
            input.context_key.as_str(),
            CANONICAL_CONTEXT_KEY_MAX_CHARS,
        )?;
        reject_text_too_long("axis", input.axis.as_str(), CANONICAL_AXIS_MAX_CHARS)?;
        reject_text_too_long(
            "timeframe",
            input.timeframe.as_str(),
            CANONICAL_TIMEFRAME_MAX_CHARS,
        )?;
        reject_text_too_long("scope", input.scope.as_str(), CANONICAL_SCOPE_MAX_CHARS)?;

        reject_secret_like_text(input.context_key.as_str())?;
        reject_secret_like_text(input.axis.as_str())?;
        reject_secret_like_text(input.timeframe.as_str())?;
        reject_secret_like_text(input.scope.as_str())?;

        let context_key = input.context_key.trim();
        let axis = input.axis.trim();
        let timeframe = input.timeframe.trim();
        let scope = input.scope.trim();
        if context_key.is_empty() || axis.is_empty() || timeframe.is_empty() || scope.is_empty() {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "context_key, axis, timeframe, and scope are required",
            ));
        }
        if input.target_left_idea_id == input.target_right_idea_id {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "challenge targets must be distinct ideas",
            ));
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| canonical_storage_error(err))?;
        let writer_identity_id = load_canonical_writer_identity(&mut tx, account_id).await?;
        let _ = maybe_emit_cycle_close_before_append(&mut tx).await?;
        let cycle_window = load_cycle_window(&mut tx).await?;
        let spent_mana =
            load_cycle_build_spend(&mut tx, writer_identity_id, cycle_window.h_start).await?;
        let remaining_mana = BUILD_MANA_CYCLE_CAP.saturating_sub(spent_mana);
        if remaining_mana < CHALLENGE_CREATE_MANA_COST {
            return Err(CanonicalWriteError::new(
                "insufficient_mana",
                format!(
                    "insufficient build mana: required={} remaining={}",
                    CHALLENGE_CREATE_MANA_COST, remaining_mana
                ),
            ));
        }

        let payload = json!({
            "challenge_id": input.challenge_id,
            "challenge_domain": "importance_challenge",
            "framing_representation_ref": input.framing_representation_ref,
            "speaker_identity_id": writer_identity_id,
            "context_key": context_key,
            "axis": axis,
            "timeframe": timeframe,
            "scope": scope,
            "subject_idea_ids": [
                input.target_left_idea_id,
                input.target_right_idea_id
            ],
            "reference_idea_id": input.reference_idea_id
        });
        let author_signature = input.author_signature.clone();
        let event_id = input.event_id.unwrap_or_else(Uuid::now_v7);
        let event = Event {
            id: event_id,
            kind: "challenge_create".to_string(),
            payload: payload.clone(),
            speaker_identity_id: Some(writer_identity_id),
        };
        validate_event(&event).map_err(|err| {
            CanonicalWriteError::new(
                err.code,
                format!("event validation failed: {}", err.message),
            )
        })?;

        let position = allocate_canonical_event_position(&mut tx).await?;
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
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .bind("challenge_create")
        .bind(Some(writer_identity_id))
        .bind(payload)
        .bind(author_signature)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
        let _ = refresh_tempo_predicates_for_position(&mut tx, position).await?;

        sqlx::query(
            r#"
            INSERT INTO challenges (
              challenge_id,
              challenge_domain,
              context_key,
              target_left_idea_id,
              target_right_idea_id,
              reference_idea_id,
              framing_representation_ref,
              created_by_identity_id,
              created_block_height,
              created_event_index,
              created_event_id,
              created_cycle_index,
              lifecycle_state,
              terminal_event_id
            ) VALUES (
              $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, 0, NULL
            )
            "#,
        )
        .bind(input.challenge_id)
        .bind("importance_challenge")
        .bind(context_key)
        .bind(input.target_left_idea_id)
        .bind(input.target_right_idea_id)
        .bind(input.reference_idea_id)
        .bind(input.framing_representation_ref)
        .bind(writer_identity_id)
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .bind(cycle_window.cycle_index)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        sqlx::query(
            r#"
            INSERT INTO challenge_context (
              challenge_id,
              axis,
              timeframe,
              scope
            ) VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(input.challenge_id)
        .bind(axis)
        .bind(timeframe)
        .bind(scope)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        sqlx::query(
            r#"
            INSERT INTO challenge_targets (
              challenge_id,
              target_position,
              idea_id
            ) VALUES ($1, $2, $3), ($1, $4, $5)
            "#,
        )
        .bind(input.challenge_id)
        .bind(0_i16)
        .bind(input.target_left_idea_id)
        .bind(1_i16)
        .bind(input.target_right_idea_id)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        tx.commit()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        Ok(CanonicalImportanceChallengeCreateResult {
            challenge_id: input.challenge_id,
            event_id,
            cycle_index: cycle_window.cycle_index,
            remaining_build_mana: remaining_mana - CHALLENGE_CREATE_MANA_COST,
        })
    }

    pub async fn create_canonical_importance_argument_attach(
        &self,
        account_id: Uuid,
        input: CanonicalImportanceArgumentAttachInput,
    ) -> std::result::Result<CanonicalImportanceArgumentAttachResult, CanonicalWriteError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| canonical_storage_error(err))?;
        let writer_identity_id = load_canonical_writer_identity(&mut tx, account_id).await?;
        let _ = maybe_emit_cycle_close_before_append(&mut tx).await?;
        let cycle_window = load_cycle_window(&mut tx).await?;
        let spent_mana =
            load_cycle_build_spend(&mut tx, writer_identity_id, cycle_window.h_start).await?;
        let remaining_mana = BUILD_MANA_CYCLE_CAP.saturating_sub(spent_mana);
        if remaining_mana < CHALLENGE_ARGUMENT_ATTACH_MANA_COST {
            return Err(CanonicalWriteError::new(
                "insufficient_mana",
                format!(
                    "insufficient build mana: required={} remaining={}",
                    CHALLENGE_ARGUMENT_ATTACH_MANA_COST, remaining_mana
                ),
            ));
        }

        let challenge = sqlx::query_as::<_, CanonicalChallengeRow>(
            r#"
            SELECT
              c.challenge_id,
              c.challenge_domain,
              c.context_key,
              ctx.axis,
              ctx.timeframe,
              ctx.scope,
              c.target_left_idea_id,
              c.target_right_idea_id,
              c.reference_idea_id,
              c.framing_representation_ref,
              c.created_by_identity_id,
              c.created_block_height,
              c.created_event_index,
              c.created_event_id,
              c.created_cycle_index,
              c.lifecycle_state
            FROM challenges c
            JOIN challenge_context ctx ON ctx.challenge_id = c.challenge_id
            WHERE c.challenge_id = $1
            FOR UPDATE
            "#,
        )
        .bind(input.challenge_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(canonical_storage_error)?
        .ok_or_else(|| CanonicalWriteError::new("invalid_request", "challenge not found"))?;

        let verdict_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM challenge_verdicts WHERE challenge_id = $1)",
        )
        .bind(input.challenge_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(canonical_storage_error)?;

        let phase = derive_challenge_phase(
            challenge.lifecycle_state,
            challenge.created_cycle_index,
            cycle_window.cycle_index,
            verdict_exists,
        );
        if phase != ChallengePhase::OpenArguments {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "challenge is not open for argument attachments",
            ));
        }
        if input.subject_idea_id != challenge.target_left_idea_id
            && input.subject_idea_id != challenge.target_right_idea_id
        {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "subject_idea_id must match one of the challenge targets",
            ));
        }
        let challenge_axis = challenge.axis.clone();
        let challenge_timeframe = challenge.timeframe.clone();
        let challenge_scope = challenge.scope.clone();
        reject_secret_like_text(challenge_axis.as_str())?;
        reject_secret_like_text(challenge_timeframe.as_str())?;
        reject_secret_like_text(challenge_scope.as_str())?;

        let payload = json!({
            "connection_id": input.connection_id,
            "from_idea_id": input.argument_idea_id,
            "to_idea_id": input.subject_idea_id,
            "connection_type": "relative_importance",
            "usage": "importance_argument",
            "axis": challenge_axis,
            "timeframe": challenge_timeframe,
            "scope": challenge_scope,
            "context_challenge_id": input.challenge_id,
            "speaker_identity_id": writer_identity_id
        });
        let author_signature = input.author_signature.clone();
        let event_id = input.event_id.unwrap_or_else(Uuid::now_v7);
        let event = Event {
            id: event_id,
            kind: "connection_create".to_string(),
            payload: payload.clone(),
            speaker_identity_id: Some(writer_identity_id),
        };
        validate_event(&event).map_err(|err| {
            CanonicalWriteError::new(
                err.code,
                format!("event validation failed: {}", err.message),
            )
        })?;

        let position = allocate_canonical_event_position(&mut tx).await?;
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
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .bind("connection_create")
        .bind(Some(writer_identity_id))
        .bind(payload)
        .bind(author_signature)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
        let _ = refresh_tempo_predicates_for_position(&mut tx, position).await?;

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
            ) VALUES (
              $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            )
            "#,
        )
        .bind(input.connection_id)
        .bind(input.argument_idea_id)
        .bind(input.subject_idea_id)
        .bind("relative_importance")
        .bind("importance_argument")
        .bind(challenge.axis)
        .bind(challenge.timeframe)
        .bind(challenge.scope)
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        sqlx::query(
            r#"
            INSERT INTO challenge_arguments (
              challenge_id,
              connection_id,
              argument_idea_id,
              subject_idea_id,
              created_block_height,
              created_event_index,
              created_event_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(input.challenge_id)
        .bind(input.connection_id)
        .bind(input.argument_idea_id)
        .bind(input.subject_idea_id)
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        tx.commit()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        Ok(CanonicalImportanceArgumentAttachResult {
            challenge_id: input.challenge_id,
            connection_id: input.connection_id,
            event_id,
            cycle_index: cycle_window.cycle_index,
            remaining_build_mana: remaining_mana - CHALLENGE_ARGUMENT_ATTACH_MANA_COST,
        })
    }

    pub async fn get_canonical_challenge_detail(
        &self,
        challenge_id: Uuid,
    ) -> Result<Option<CanonicalChallengeDetail>> {
        let mut tx = self.pool.begin().await?;
        let challenge = sqlx::query_as::<_, CanonicalChallengeRow>(
            r#"
            SELECT
              c.challenge_id,
              c.challenge_domain,
              c.context_key,
              ctx.axis,
              ctx.timeframe,
              ctx.scope,
              c.target_left_idea_id,
              c.target_right_idea_id,
              c.reference_idea_id,
              c.framing_representation_ref,
              c.created_by_identity_id,
              c.created_block_height,
              c.created_event_index,
              c.created_event_id,
              c.created_cycle_index,
              c.lifecycle_state
            FROM challenges c
            JOIN challenge_context ctx ON ctx.challenge_id = c.challenge_id
            WHERE c.challenge_id = $1
            "#,
        )
        .bind(challenge_id)
        .fetch_optional(&mut *tx)
        .await?;
        let Some(challenge) = challenge else {
            tx.rollback().await?;
            return Ok(None);
        };

        let cycle_window = load_cycle_window(&mut tx)
            .await
            .map_err(anyhow::Error::from)?;
        let verdict = sqlx::query_as::<_, CanonicalChallengeVerdictRow>(
            r#"
            SELECT
              verdict_id,
              challenge_id,
              verdict_event_id,
              winning_choice,
              winning_target_idea_id,
              left_votes,
              right_votes,
              total_votes,
              resolved_block_height,
              resolved_event_index
            FROM challenge_verdicts
            WHERE challenge_id = $1
            "#,
        )
        .bind(challenge_id)
        .fetch_optional(&mut *tx)
        .await?;
        let has_verdict = verdict.is_some();
        let phase = derive_challenge_phase(
            challenge.lifecycle_state,
            challenge.created_cycle_index,
            cycle_window.cycle_index,
            has_verdict,
        )
        .label()
        .to_string();
        let arguments = sqlx::query_as::<_, CanonicalChallengeArgumentRow>(
            r#"
            SELECT
              challenge_id,
              connection_id,
              argument_idea_id,
              subject_idea_id,
              created_block_height,
              created_event_index,
              created_event_id
            FROM challenge_arguments
            WHERE challenge_id = $1
            ORDER BY created_block_height ASC, created_event_index ASC
            "#,
        )
        .bind(challenge_id)
        .fetch_all(&mut *tx)
        .await?;
        let votes = sqlx::query_as::<_, CanonicalChallengeVoteRow>(
            r#"
            SELECT
              challenge_id,
              voter_identity_id,
              vote_session_id,
              vote_choice,
              cast_block_height,
              cast_event_index,
              cast_event_id
            FROM challenge_votes
            WHERE challenge_id = $1
            ORDER BY cast_block_height ASC, cast_event_index ASC
            "#,
        )
        .bind(challenge_id)
        .fetch_all(&mut *tx)
        .await?;
        tx.commit().await?;

        Ok(Some(CanonicalChallengeDetail {
            challenge,
            arguments,
            votes,
            verdict,
            current_cycle_index: cycle_window.cycle_index,
            phase,
        }))
    }

    pub async fn pull_canonical_vote_session(
        &self,
        account_id: Uuid,
        input: CanonicalVoteSessionPullInput,
    ) -> std::result::Result<CanonicalVoteSessionPullResult, CanonicalWriteError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| canonical_storage_error(err))?;
        let voter_identity_id = load_canonical_writer_identity(&mut tx, account_id).await?;
        let _ = maybe_emit_cycle_close_before_append(&mut tx).await?;
        let cycle_window = load_cycle_window(&mut tx).await?;
        let spent_voting =
            load_cycle_voting_spend(&mut tx, voter_identity_id, cycle_window.h_start).await?;
        let remaining_voting_mana = VOTING_MANA_CYCLE_CAP.saturating_sub(spent_voting);
        if remaining_voting_mana < VOTE_SESSION_OPEN_MANA_COST {
            return Err(CanonicalWriteError::new(
                "insufficient_mana",
                format!(
                    "insufficient voting mana: required={} remaining={}",
                    VOTE_SESSION_OPEN_MANA_COST, remaining_voting_mana
                ),
            ));
        }

        let session_index = load_next_vote_session_index(&mut tx, voter_identity_id).await?;
        let candidates =
            load_vote_session_candidates(&mut tx, voter_identity_id, cycle_window.cycle_index)
                .await?;
        if candidates.is_empty() {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "no eligible voting-open importance challenge is currently available",
            ));
        }

        let selection_boundary_event_id = load_latest_cycle_boundary_event_id(&mut tx)
            .await?
            .ok_or_else(|| {
                CanonicalWriteError::new(
                    "invalid_request",
                    "vote-session pull requires at least one canonical cycle boundary",
                )
            })?;

        let selected_index = deterministic_vote_session_index(
            voter_identity_id,
            selection_boundary_event_id,
            cycle_window.cycle_index,
            candidates.len(),
        );
        let selected_challenge = candidates[selected_index];

        let payload = json!({
            "vote_session_id": input.vote_session_id,
            "challenge_id": selected_challenge.challenge_id,
            "session_index": session_index,
            "selection_cycle_index": cycle_window.cycle_index,
            "selection_boundary_event_id": selection_boundary_event_id,
            "speaker_identity_id": voter_identity_id
        });
        let author_signature = input.author_signature.clone();
        let event_id = input.event_id.unwrap_or_else(Uuid::now_v7);
        let event = Event {
            id: event_id,
            kind: "vote_session_open".to_string(),
            payload: payload.clone(),
            speaker_identity_id: Some(voter_identity_id),
        };
        // vote_session_open materializes Stage 0 challenge-session assignment. Protocol v5
        // derives this lifecycle state instead of accepting it as a public canonical event.
        validate_stage0_internal_event(&event).map_err(|err| {
            CanonicalWriteError::new(
                err.code,
                format!("event validation failed: {}", err.message),
            )
        })?;

        let position = allocate_canonical_event_position(&mut tx).await?;
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
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .bind("vote_session_open")
        .bind(Some(voter_identity_id))
        .bind(payload)
        .bind(author_signature)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
        let _ = refresh_tempo_predicates_for_position(&mut tx, position).await?;

        sqlx::query(
            r#"
            INSERT INTO challenge_vote_sessions (
              vote_session_id,
              challenge_id,
              voter_identity_id,
              session_index,
              selection_cycle_index,
              selection_boundary_event_id,
              created_block_height,
              created_event_index,
              created_event_id
            ) VALUES (
              $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            "#,
        )
        .bind(input.vote_session_id)
        .bind(selected_challenge.challenge_id)
        .bind(voter_identity_id)
        .bind(session_index)
        .bind(cycle_window.cycle_index)
        .bind(selection_boundary_event_id)
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(event_id)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        tx.commit()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        Ok(CanonicalVoteSessionPullResult {
            vote_session_id: input.vote_session_id,
            challenge_id: selected_challenge.challenge_id,
            event_id,
            session_index,
            cycle_index: cycle_window.cycle_index,
            remaining_voting_mana: remaining_voting_mana - VOTE_SESSION_OPEN_MANA_COST,
        })
    }

    pub async fn cast_canonical_importance_vote(
        &self,
        account_id: Uuid,
        input: CanonicalVoteCastInput,
    ) -> std::result::Result<CanonicalVoteCastResult, CanonicalWriteError> {
        reject_text_too_long(
            "vote_choice",
            input.vote_choice.as_str(),
            CANONICAL_VOTE_CHOICE_MAX_CHARS,
        )?;
        reject_secret_like_text(input.vote_choice.as_str())?;

        let vote_choice = input.vote_choice.trim().to_lowercase();
        if !matches!(vote_choice.as_str(), "left" | "right" | "abstain") {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "vote_choice must be left, right, or abstain",
            ));
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| canonical_storage_error(err))?;
        let voter_identity_id = load_canonical_writer_identity(&mut tx, account_id).await?;
        let _ = maybe_emit_cycle_close_before_append(&mut tx).await?;
        let cycle_window = load_cycle_window(&mut tx).await?;

        let challenge = sqlx::query_as::<_, ChallengeVoteContextRow>(
            r#"
            SELECT
              challenge_id,
              created_by_identity_id,
              target_left_idea_id,
              target_right_idea_id,
              created_cycle_index
            FROM challenges
            WHERE challenge_id = $1
            FOR UPDATE
            "#,
        )
        .bind(input.challenge_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(canonical_storage_error)?
        .ok_or_else(|| CanonicalWriteError::new("invalid_request", "challenge not found"))?;

        if challenge.created_by_identity_id == voter_identity_id {
            return Err(CanonicalWriteError::new(
                "forbidden",
                "challenge creator is not eligible to vote on this challenge",
            ));
        }
        if cycle_window.cycle_index
            < challenge.created_cycle_index + CHALLENGE_ARGUMENT_PHASE_CYCLES
        {
            return Err(CanonicalWriteError::new(
                "invalid_request",
                "challenge is not open for voting",
            ));
        }

        let verdict_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM challenge_verdicts WHERE challenge_id = $1)",
        )
        .bind(input.challenge_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(canonical_storage_error)?;
        if verdict_exists {
            return Err(CanonicalWriteError::new(
                "conflict",
                "challenge already has a finalized verdict",
            ));
        }

        let session = sqlx::query_as::<_, VoteSessionRow>(
            r#"
            SELECT
              vote_session_id,
              challenge_id,
              voter_identity_id,
              session_index
            FROM challenge_vote_sessions
            WHERE vote_session_id = $1
            FOR UPDATE
            "#,
        )
        .bind(input.vote_session_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(canonical_storage_error)?
        .ok_or_else(|| CanonicalWriteError::new("invalid_request", "vote session not found"))?;
        if session.challenge_id != input.challenge_id
            || session.voter_identity_id != voter_identity_id
        {
            return Err(CanonicalWriteError::new(
                "forbidden",
                "vote session does not match the authenticated voter/challenge",
            ));
        }
        let _session_index = session.session_index;

        let existing_vote: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS (
              SELECT 1
              FROM challenge_votes
              WHERE challenge_id = $1
                AND voter_identity_id = $2
            )
            "#,
        )
        .bind(input.challenge_id)
        .bind(voter_identity_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(canonical_storage_error)?;
        if existing_vote {
            return Err(CanonicalWriteError::new(
                "conflict",
                "voter has already cast a vote for this challenge",
            ));
        }

        let payload = json!({
            "challenge_id": input.challenge_id,
            "vote_session_id": input.vote_session_id,
            "vote_choice": vote_choice,
            "speaker_identity_id": voter_identity_id
        });
        let author_signature = input.author_signature.clone();
        let vote_event_id = input.event_id.unwrap_or_else(Uuid::now_v7);
        let event = Event {
            id: vote_event_id,
            kind: "vote_cast".to_string(),
            payload: payload.clone(),
            speaker_identity_id: Some(voter_identity_id),
        };
        validate_event(&event).map_err(|err| {
            CanonicalWriteError::new(
                err.code,
                format!("event validation failed: {}", err.message),
            )
        })?;

        let position = allocate_canonical_event_position(&mut tx).await?;
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
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(vote_event_id)
        .bind("vote_cast")
        .bind(Some(voter_identity_id))
        .bind(payload)
        .bind(author_signature)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
        let _ = refresh_tempo_predicates_for_position(&mut tx, position).await?;

        sqlx::query(
            r#"
            INSERT INTO challenge_votes (
              cast_event_id,
              challenge_id,
              voter_identity_id,
              vote_session_id,
              vote_choice,
              cast_block_height,
              cast_event_index
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(vote_event_id)
        .bind(input.challenge_id)
        .bind(voter_identity_id)
        .bind(input.vote_session_id)
        .bind(vote_choice.as_str())
        .bind(position.block_height)
        .bind(position.event_index)
        .execute(&mut *tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        let votes = sqlx::query_as::<_, ChallengeVoteChoiceRow>(
            r#"
            SELECT
              voter_identity_id,
              vote_choice,
              cast_event_id
            FROM challenge_votes
            WHERE challenge_id = $1
            ORDER BY cast_block_height ASC, cast_event_index ASC
            "#,
        )
        .bind(input.challenge_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(canonical_storage_error)?;

        let spent_voting =
            load_cycle_voting_spend(&mut tx, voter_identity_id, cycle_window.h_start).await?;
        let remaining_voting_mana = VOTING_MANA_CYCLE_CAP.saturating_sub(spent_voting);

        let mut verdict_event_id = None;
        let mut verdict_outcome = None;
        if i64::try_from(votes.len()).unwrap_or(0) == TARGET_JUROR_COUNT {
            let (winning_choice, winning_target_idea_id, left_votes, right_votes, total_votes) =
                aggregate_importance_verdict(
                    &votes,
                    challenge.target_left_idea_id,
                    challenge.target_right_idea_id,
                );

            let vote_event_ids: Vec<Uuid> = votes.iter().map(|row| row.cast_event_id).collect();
            let deterministic_verdict_event_id = deterministic_uuid_v7(
                "challenge_finalize_verdict",
                input.challenge_id,
                &vote_event_ids,
            );
            let deterministic_verdict_id =
                deterministic_uuid_v7("importance_verdict_id", input.challenge_id, &vote_event_ids);
            let verdict_author = votes[0].voter_identity_id;

            let verdict_payload = json!({
                "challenge_id": input.challenge_id,
                "verdict_id": deterministic_verdict_id,
                "winning_choice": winning_choice,
                "winning_target_idea_id": winning_target_idea_id,
                "left_votes": left_votes,
                "right_votes": right_votes,
                "total_votes": total_votes,
                "vote_event_ids": vote_event_ids
            });
            let verdict_event = Event {
                id: deterministic_verdict_event_id,
                kind: "challenge_finalize_verdict".to_string(),
                payload: verdict_payload.clone(),
                speaker_identity_id: Some(verdict_author),
            };
            validate_event(&verdict_event).map_err(|err| {
                CanonicalWriteError::new(
                    err.code,
                    format!("event validation failed: {}", err.message),
                )
            })?;

            let _ = maybe_emit_cycle_close_before_append(&mut tx).await?;
            let verdict_position = allocate_canonical_event_position(&mut tx).await?;
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
            .bind(verdict_position.block_height)
            .bind(verdict_position.event_index)
            .bind(deterministic_verdict_event_id)
            .bind("challenge_finalize_verdict")
            .bind(Some(verdict_author))
            .bind(verdict_payload)
            .bind::<Option<String>>(None)
            .execute(&mut *tx)
            .await
            .map_err(map_canonical_write_sqlx_error)?;
            let _ = refresh_tempo_predicates_for_position(&mut tx, verdict_position).await?;

            sqlx::query(
                r#"
                INSERT INTO challenge_verdicts (
                  verdict_id,
                  challenge_id,
                  verdict_event_id,
                  winning_choice,
                  winning_target_idea_id,
                  left_votes,
                  right_votes,
                  total_votes,
                  resolved_block_height,
                  resolved_event_index
                ) VALUES (
                  $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
                )
                "#,
            )
            .bind(deterministic_verdict_id)
            .bind(input.challenge_id)
            .bind(deterministic_verdict_event_id)
            .bind(winning_choice)
            .bind(winning_target_idea_id)
            .bind(i16::try_from(left_votes).unwrap_or(i16::MAX))
            .bind(i16::try_from(right_votes).unwrap_or(i16::MAX))
            .bind(i16::try_from(total_votes).unwrap_or(i16::MAX))
            .bind(verdict_position.block_height)
            .bind(verdict_position.event_index)
            .execute(&mut *tx)
            .await
            .map_err(map_canonical_write_sqlx_error)?;

            verdict_event_id = Some(deterministic_verdict_event_id);
            verdict_outcome = Some(winning_choice.to_string());
        }

        tx.commit()
            .await
            .map_err(|err| canonical_storage_error(err))?;

        Ok(CanonicalVoteCastResult {
            challenge_id: input.challenge_id,
            vote_event_id,
            cycle_index: cycle_window.cycle_index,
            remaining_voting_mana,
            verdict_event_id,
            verdict_outcome,
        })
    }
}
