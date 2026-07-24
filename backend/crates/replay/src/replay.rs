use chrono::{DateTime, Utc};
use encoding::hash::hash_bytes;
use event_log::validation::validate_stage0_internal_event;
use event_log::Event;
use event_log::SYSTEM_BOUNDARY_EMITTER_ID_STR;
use serde_json::Value;
use sqlx::{FromRow, PgPool};
use std::collections::{BTreeSet, HashMap};
use uuid::Uuid;

const W_TARGET_ALPHA_NUM: i64 = 1;
const W_TARGET_ALPHA_DEN: i64 = 2;
const W_TARGET_SCALE_NUM: i64 = 1;
const W_TARGET_SCALE_DEN: i64 = 1;
const W_TARGET_MIN: i64 = 1;
const W_TARGET_MAX: i64 = 10_000;
const W_EMA_INITIAL: i64 = W_TARGET_MIN;
const CHALLENGE_ARGUMENT_PHASE_CYCLES: i64 = 1;
const TARGET_JUROR_COUNT: usize = 3;
const MIN_CANONICAL_WRITER_LEVEL: i16 = 1;
const SEED_BOOTSTRAP_VERIFIER_ID_STR: &str = "380b7817-db3b-7b76-8cf3-87df879ddddb";

mod apply;
mod parsing;
mod state;
mod tempo;
mod voting;

#[cfg(test)]
mod tests;

use apply::*;
use tempo::*;

#[derive(Debug, Default)]
pub struct ReplayDriver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayObjectKind {
    Idea,
    Ordering,
}

impl ReplayObjectKind {
    pub fn as_u8(self) -> u8 {
        match self {
            ReplayObjectKind::Idea => 0,
            ReplayObjectKind::Ordering => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayIdeaRow {
    pub idea_id: Uuid,
    pub idea_type: String,
    pub speaker_identity_id: Uuid,
    pub created_event_id: Uuid,
    pub created_block_height: i64,
    pub created_event_index: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayOrderingItemRow {
    pub idx: i32,
    pub idea_id: Uuid,
    pub via_connection_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayOrderingRow {
    pub ordering_id: Uuid,
    pub ordering_profile: String,
    pub vine_type: Option<String>,
    pub speaker_identity_id: Uuid,
    pub created_event_id: Uuid,
    pub created_block_height: i64,
    pub created_event_index: i32,
    pub base_ordering_id: Option<Uuid>,
    pub title_representation_id: Option<Uuid>,
    pub sentence_representation_id: Option<Uuid>,
    pub items: Vec<ReplayOrderingItemRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayConnectionRow {
    pub connection_id: Uuid,
    pub from_idea_id: Uuid,
    pub to_idea_id: Uuid,
    pub connection_type: String,
    pub usage: Option<String>,
    pub axis: Option<String>,
    pub timeframe: Option<String>,
    pub scope: Option<String>,
    pub created_by_event_id: Uuid,
    pub created_block_height: i64,
    pub created_event_index: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayPayloadRow {
    pub object_kind: ReplayObjectKind,
    pub object_id: Uuid,
    pub title: Option<String>,
    pub sentence: Option<String>,
    pub paragraph: Option<String>,
    pub full: Option<String>,
    pub payload_hash: Option<String>,
    pub title_payload_hash: Option<String>,
    pub sentence_payload_hash: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReplayOutput {
    pub height: i64,
    pub event_count: i64,
    pub last_event_id: Uuid,
    pub approximate_timestamp: DateTime<Utc>,
    pub ideas: Vec<ReplayIdeaRow>,
    pub orderings: Vec<ReplayOrderingRow>,
    pub connections: Vec<ReplayConnectionRow>,
    pub payloads: Vec<ReplayPayloadRow>,
    pub cycle_status: ReplayCycleStatus,
    pub tempo_status: ReplayTempoStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCycleStatus {
    pub cycle_index: i64,
    pub h_start: i64,
    pub current_height: i64,
    pub w_target: i64,
    pub observed_work: i64,
    pub cycle_age_ge_dmin: bool,
    pub cycle_age_ge_dmax: bool,
    pub closure_predicate_satisfied: bool,
    pub last_cycle_close_height: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayTempoStatus {
    pub cycle_age_ge_dmin: bool,
    pub cycle_age_ge_dmax: bool,
    pub constrained_mode: bool,
    pub record_only_mode: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayError {
    pub code: &'static str,
    pub message: String,
}

impl ReplayError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ReplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ReplayError {}

#[derive(Debug, FromRow, Clone)]
struct EventRow {
    block_height: i64,
    event_index: i32,
    event_id: Uuid,
    event_type: String,
    speaker_identity_id: Option<Uuid>,
    payload_json: Value,
}

#[derive(Debug, FromRow)]
struct IdeaRow {
    idea_id: Uuid,
    idea_type: String,
    speaker_identity_id: Uuid,
    created_event_id: Uuid,
    created_block_height: i64,
    created_event_index: i32,
}

#[derive(Debug, FromRow)]
struct OrderingRow {
    ordering_id: Uuid,
    ordering_profile: i16,
    vine_type: Option<i16>,
    speaker_identity_id: Uuid,
    created_event_id: Uuid,
    created_block_height: i64,
    created_event_index: i32,
    base_ordering_id: Option<Uuid>,
}

#[derive(Debug, FromRow, Clone)]
struct OrderingItemRow {
    ordering_id: Uuid,
    idx: i32,
    idea_id: Uuid,
    via_connection_id: Option<Uuid>,
}

#[derive(Debug, FromRow)]
struct ConnectionRow {
    connection_id: Uuid,
    from_idea_id: Uuid,
    to_idea_id: Uuid,
    connection_type: String,
    usage: Option<String>,
    axis: Option<String>,
    timeframe: Option<String>,
    scope: Option<String>,
    created_by_event_id: Uuid,
    created_block_height: i64,
    created_event_index: i32,
}

#[derive(Debug, FromRow, Clone)]
struct RepresentationRow {
    representation_id: Uuid,
    target_kind: i16,
    target_id: Uuid,
    tier_enum: i16,
    payload_hash: String,
    payload_text: Option<String>,
    created_event_id: Uuid,
    created_block_height: i64,
    created_event_index: i32,
}

#[derive(Debug, FromRow)]
struct IdeaPayloadRow {
    idea_id: Uuid,
    title: Option<String>,
    sentence: Option<String>,
    paragraph: Option<String>,
    full: Option<String>,
    payload_hash: Option<String>,
}

#[derive(Debug, FromRow)]
struct BlockMax {
    max_height: Option<i64>,
}

#[derive(Debug, FromRow)]
struct CountRow {
    total: i64,
}

#[derive(Debug, FromRow)]
struct LastEventRow {
    event_id: Uuid,
}

#[derive(Debug, FromRow, Clone)]
struct TempoPredicateRow {
    block_height: i64,
    event_index: i32,
    cycle_age_ge_dmin: bool,
    cycle_age_ge_dmax: bool,
    constrained_mode: bool,
    record_only_mode: bool,
}

#[derive(Debug, FromRow, Clone)]
struct CycleBoundaryRow {
    cycle_index: i64,
    closure_kind: i16,
    forced_seal: bool,
    closure_block_height: i64,
    source_event_id: Uuid,
    source_block_height: i64,
    source_event_index: i32,
}

#[derive(Debug, FromRow, Clone)]
struct WriterVerificationMaterializedRow {
    identity_id: Uuid,
    email_verified: bool,
    canonical_writer_level: i16,
    granted_by_identity_id: Uuid,
    source_event_id: Uuid,
    source_block_height: i64,
    source_event_index: i32,
}

#[derive(Debug, FromRow, Clone)]
struct VerifierRoleRow {
    verifier_identity_id: Uuid,
    is_active: bool,
    source_event_id: Option<Uuid>,
    source_block_height: Option<i64>,
    source_event_index: Option<i32>,
}

impl ReplayDriver {
    pub async fn run(pool: &PgPool, height: Option<i64>) -> Result<ReplayOutput, ReplayError> {
        let max_height: BlockMax =
            sqlx::query_as("SELECT MAX(block_height) AS max_height FROM blocks")
                .fetch_one(pool)
                .await
                .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;
        let height = height.unwrap_or_else(|| max_height.max_height.unwrap_or(0));

        let event_count: CountRow =
            sqlx::query_as("SELECT COUNT(*) AS total FROM events WHERE block_height <= $1")
                .bind(height)
                .fetch_one(pool)
                .await
                .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let last_event = sqlx::query_as::<_, LastEventRow>(
            r#"
            SELECT event_id
            FROM events
            WHERE block_height <= $1
            ORDER BY block_height DESC, event_index DESC
            LIMIT 1
            "#,
        )
        .bind(height)
        .fetch_optional(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let last_event_id = match last_event {
            Some(row) => row.event_id,
            None => {
                return Err(ReplayError::new(
                    "replay_empty",
                    format!("no events found up to height {height}"),
                ))
            }
        };
        let approximate_timestamp = approximate_timestamp_from_event_id(last_event_id)?;

        let events: Vec<EventRow> = sqlx::query_as(
            r#"
            SELECT block_height, event_index, event_id, event_type, speaker_identity_id, payload_json
            FROM events
            WHERE block_height <= $1
            ORDER BY block_height ASC, event_index ASC
            "#,
        )
        .bind(height)
        .fetch_all(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let idea_rows: Vec<IdeaRow> = sqlx::query_as(
            r#"
            SELECT
              idea_id,
              idea_type,
              speaker_identity_id,
              created_event_id,
              created_block_height,
              created_event_index
            FROM ideas
            WHERE created_block_height <= $1
            ORDER BY created_block_height ASC, created_event_index ASC
            "#,
        )
        .bind(height)
        .fetch_all(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let ordering_rows: Vec<OrderingRow> = sqlx::query_as(
            r#"
            SELECT
              ordering_id,
              ordering_profile,
              vine_type,
              speaker_identity_id,
              created_event_id,
              created_block_height,
              created_event_index,
              base_ordering_id
            FROM orderings
            WHERE created_block_height <= $1
            ORDER BY created_block_height ASC, created_event_index ASC
            "#,
        )
        .bind(height)
        .fetch_all(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let ordering_item_rows: Vec<OrderingItemRow> = sqlx::query_as(
            r#"
            SELECT
              ri.ordering_id,
              ri.idx,
              ri.idea_id,
              ri.via_connection_id
            FROM ordering_items ri
            JOIN orderings r ON r.ordering_id = ri.ordering_id
            WHERE r.created_block_height <= $1
            ORDER BY r.created_block_height ASC, r.created_event_index ASC, ri.idx ASC
            "#,
        )
        .bind(height)
        .fetch_all(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let connection_rows: Vec<ConnectionRow> = sqlx::query_as(
            r#"
            SELECT
              connection_id,
              from_idea_id,
              to_idea_id,
              connection_type,
              usage,
              axis,
              timeframe,
              scope,
              created_by_event_id,
              created_block_height,
              created_event_index
            FROM connections
            WHERE created_block_height <= $1
            ORDER BY created_block_height ASC, created_event_index ASC
            "#,
        )
        .bind(height)
        .fetch_all(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let representation_rows: Vec<RepresentationRow> = sqlx::query_as(
            r#"
            SELECT
              representation_id,
              target_kind,
              target_id,
              tier_enum,
              payload_hash,
              payload_text,
              created_event_id,
              created_block_height,
              created_event_index
            FROM representations
            WHERE created_block_height <= $1
            ORDER BY created_block_height ASC, created_event_index ASC
            "#,
        )
        .bind(height)
        .fetch_all(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let idea_payload_rows: Vec<IdeaPayloadRow> = sqlx::query_as(
            r#"
            SELECT
              i.idea_id,
              e.payload_json->>'title' AS title,
              e.payload_json->>'sentence' AS sentence,
              e.payload_json->>'paragraph' AS paragraph,
              e.payload_json->>'full' AS full,
              e.payload_json->>'payload_hash' AS payload_hash
            FROM ideas i
            JOIN events e ON e.event_id = i.created_event_id
            WHERE i.created_block_height <= $1
            ORDER BY i.created_block_height ASC, i.created_event_index ASC
            "#,
        )
        .bind(height)
        .fetch_all(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let tempo_rows: Vec<TempoPredicateRow> = sqlx::query_as(
            r#"
            SELECT
              block_height,
              event_index,
              cycle_age_ge_dmin,
              cycle_age_ge_dmax,
              constrained_mode,
              record_only_mode
            FROM tempo_predicates
            WHERE block_height <= $1
            ORDER BY block_height ASC, event_index ASC
            "#,
        )
        .bind(height)
        .fetch_all(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let cycle_boundary_rows: Vec<CycleBoundaryRow> = sqlx::query_as(
            r#"
            SELECT
              cycle_index,
              closure_kind,
              forced_seal,
              closure_block_height,
              source_event_id,
              source_block_height,
              source_event_index
            FROM cycle_boundaries
            WHERE source_block_height <= $1
            ORDER BY source_block_height ASC, source_event_index ASC
            "#,
        )
        .bind(height)
        .fetch_all(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let writer_verification_rows: Vec<WriterVerificationMaterializedRow> = sqlx::query_as(
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
            WHERE source_block_height <= $1
            ORDER BY source_block_height ASC, source_event_index ASC
            "#,
        )
        .bind(height)
        .fetch_all(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let verifier_role_rows: Vec<VerifierRoleRow> = sqlx::query_as(
            r#"
            SELECT
              verifier_identity_id,
              is_active,
              source_event_id,
              source_block_height,
              source_event_index
            FROM verifier_role_assignments
            WHERE source_block_height IS NULL OR source_block_height <= $1
            ORDER BY source_block_height ASC NULLS FIRST, source_event_index ASC NULLS FIRST, created_at ASC
            "#,
        )
        .bind(height)
        .fetch_all(pool)
        .await
        .map_err(|err| ReplayError::new("storage_error", err.to_string()))?;

        let idea_by_event: HashMap<Uuid, IdeaRow> = idea_rows
            .into_iter()
            .map(|row| (row.created_event_id, row))
            .collect();
        let ordering_by_event: HashMap<Uuid, OrderingRow> = ordering_rows
            .into_iter()
            .map(|row| (row.created_event_id, row))
            .collect();
        let mut ordering_items_by_ordering: HashMap<Uuid, Vec<OrderingItemRow>> = HashMap::new();
        for row in ordering_item_rows {
            ordering_items_by_ordering.entry(row.ordering_id).or_default().push(row);
        }
        let connection_by_event: HashMap<Uuid, ConnectionRow> = connection_rows
            .into_iter()
            .map(|row| (row.created_by_event_id, row))
            .collect();
        let representation_by_event: HashMap<Uuid, RepresentationRow> = representation_rows
            .into_iter()
            .map(|row| (row.created_event_id, row))
            .collect();
        let payload_by_idea: HashMap<Uuid, IdeaPayloadRow> = idea_payload_rows
            .into_iter()
            .map(|row| (row.idea_id, row))
            .collect();
        let cycle_boundary_by_event: HashMap<Uuid, CycleBoundaryRow> = cycle_boundary_rows
            .into_iter()
            .map(|row| (row.source_event_id, row))
            .collect();
        let writer_verification_by_event: HashMap<Uuid, WriterVerificationMaterializedRow> =
            writer_verification_rows
                .into_iter()
                .map(|row| (row.source_event_id, row))
                .collect();

        let apply = apply_events_with_verification(
            &events,
            &idea_by_event,
            &ordering_by_event,
            &ordering_items_by_ordering,
            &connection_by_event,
            &representation_by_event,
            &payload_by_idea,
            &tempo_rows,
            &cycle_boundary_by_event,
            &writer_verification_by_event,
            &verifier_role_rows,
        )?;

        Ok(ReplayOutput {
            height,
            event_count: event_count.total,
            last_event_id,
            approximate_timestamp,
            ideas: apply.ideas,
            orderings: apply.orderings,
            connections: apply.connections,
            payloads: apply.payloads,
            cycle_status: apply.cycle_status,
            tempo_status: apply.tempo_status,
        })
    }
}
