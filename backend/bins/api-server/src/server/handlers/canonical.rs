#![cfg_attr(not(feature = "full"), allow(dead_code, unused_imports))]

use api_types_canonical::{
    CanonicalBlockedSubmissionResponse, CanonicalChallengeArgumentAttachResponse,
    CanonicalChallengeArgumentSummary, CanonicalChallengeCreateResponse, CanonicalChallengeDetail,
    CanonicalChallengeDetailResponse, CanonicalChallengeVerdictSummary,
    CanonicalChallengeVoteSummary, CanonicalConnectionCreateResponse, CanonicalCycleStatus,
    CanonicalCycleStatusResponse, CanonicalEventLogBlockBand, CanonicalEventLogCycleBand,
    CanonicalEventLogEvent, CanonicalEventLogResponse, CanonicalIdeaCreateResponse,
    CanonicalIdentityCreateResponse, CanonicalTempoStatus, CanonicalTempoStatusResponse,
    CanonicalVerificationStatus, CanonicalVerificationStatusResponse,
    CanonicalVerifierGrantResponse, CanonicalVerifierRevokeResponse, CanonicalVoteCastResponse,
    CanonicalVoteSessionPullResponse,
};
#[cfg(feature = "full")]
use axum::Extension;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use common::security_limits::{
    CANONICAL_AXIS_MAX_CHARS, CANONICAL_CONNECTION_TYPE_MAX_CHARS, CANONICAL_CONTEXT_KEY_MAX_CHARS,
    CANONICAL_IDEA_TYPE_MAX_CHARS, CANONICAL_SCOPE_MAX_CHARS, CANONICAL_TIMEFRAME_MAX_CHARS,
    CANONICAL_USAGE_MAX_CHARS, CANONICAL_VOTE_CHOICE_MAX_CHARS, CANONICAL_WRITER_LEVEL_MAX_CHARS,
    IDEA_FULL_MAX_CHARS, IDEA_PARAGRAPH_MAX_CHARS, IDEA_SENTENCE_MAX_CHARS, IDEA_TITLE_MAX_CHARS,
};
use replay::ReplayDriver;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
#[cfg(feature = "full")]
use storage::{
    screen_text_for_secrets, CanonicalBlockedSubmissionInput, CanonicalConnectionCreateInput,
    CanonicalIdeaCreateInput, CanonicalIdentityCreateInput, CanonicalImportanceArgumentAttachInput,
    CanonicalImportanceChallengeCreateInput, CanonicalVerifierGrantInput,
    CanonicalVerifierRevokeInput, CanonicalVoteCastInput, CanonicalVoteSessionPullInput,
};
use uuid::Uuid;

use crate::server::coordinates::{
    project_slots, CoordinatePoint, CoordinateScatterConfig, CoordinateSlotInput, DEFAULT_SPACING,
};
#[cfg(feature = "full")]
use crate::server::errors::canonical_write_error_response;
use crate::server::errors::json_error;
#[cfg(feature = "full")]
use crate::server::helpers::parse_non_negative_i64;
use crate::server::helpers::{is_reference_scoped_connection, scoped_neighbor_from_reference};
#[cfg(feature = "full")]
use crate::server::helpers::{
    normalize_optional_text, validate_max_len, validate_optional_max_len,
};
use crate::server::helpers::{parse_uuid_v7, parse_uuid_v7_field};
use crate::server::mapping::{snapshot_headers, with_headers};
use crate::server::types::AppState;
use crate::server::types::RelativeImportanceDirection;
#[cfg(feature = "full")]
use crate::server::types::{
    AuthenticatedAccount, CanonicalBlockedSubmissionPayload, CanonicalChallengeVoteCastPayload,
    CanonicalConnectionCreatePayload, CanonicalIdeaCreatePayload, CanonicalIdentityCreatePayload,
    CanonicalImportanceArgumentAttachPayload, CanonicalImportanceChallengeCreatePayload,
    CanonicalVerifierGrantPayload, CanonicalVerifierRevokePayload, CanonicalVoteSessionPullPayload,
};

#[cfg(feature = "full")]
const SECRET_DETECTED_CODE: &str = "secret_detected";
#[cfg(feature = "full")]
const SECRET_DETECTED_MESSAGE: &str = "canonical payload rejected: secret-like content detected";

#[derive(Serialize)]
struct CoordinateNodeResponse {
    id: String,
    x: f64,
    y: f64,
    title: String,
    sentence: Option<String>,
    idea_type: String,
    derived_universal_rank: Option<String>,
    ri_in_count: String,
    ri_out_count: String,
}

#[derive(Serialize)]
struct CoordinateMetaResponse {
    spacing: f64,
    algo: String,
    relaxed: bool,
}

#[derive(Serialize)]
struct CoordinateViewResponse {
    mode: String,
    reference_id: Option<String>,
    coords: Vec<CoordinateNodeResponse>,
    meta: CoordinateMetaResponse,
}

#[derive(sqlx::FromRow)]
struct EventLogEventRow {
    block_height: i64,
    event_index: i32,
    event_id: Uuid,
    event_type: String,
}

#[derive(sqlx::FromRow)]
struct EventLogCycleBoundaryRow {
    cycle_index: i64,
    source_event_id: Uuid,
}

#[cfg(feature = "full")]
fn reject_secret_text(value: &str) -> Result<(), Response> {
    if screen_text_for_secrets(value).is_some() {
        return Err(json_error(
            StatusCode::BAD_REQUEST,
            SECRET_DETECTED_CODE,
            SECRET_DETECTED_MESSAGE,
        ));
    }
    Ok(())
}

#[cfg(feature = "full")]
fn reject_secret_optional_text(value: Option<&str>) -> Result<(), Response> {
    if let Some(value) = value {
        reject_secret_text(value)?;
    }
    Ok(())
}

pub(crate) async fn canonical_cycles_current(State(state): State<AppState>) -> Response {
    let replay = match ReplayDriver::run(state.storage.pool(), None).await {
        Ok(replay) => replay,
        Err(err) if err.code == "replay_empty" => {
            return json_error(StatusCode::NOT_FOUND, "not_found", "not found");
        }
        Err(err) => {
            tracing::error!(?err, "failed to derive current cycle status");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let body = CanonicalCycleStatusResponse {
        cycle: CanonicalCycleStatus {
            cycle_index: replay.cycle_status.cycle_index.to_string(),
            h_start: replay.cycle_status.h_start.to_string(),
            current_height: replay.cycle_status.current_height.to_string(),
            w_target: replay.cycle_status.w_target.to_string(),
            observed_work: replay.cycle_status.observed_work.to_string(),
            cycle_age_ge_dmin: replay.cycle_status.cycle_age_ge_dmin,
            cycle_age_ge_dmax: replay.cycle_status.cycle_age_ge_dmax,
            closure_predicate_satisfied: replay.cycle_status.closure_predicate_satisfied,
            last_cycle_close_height: replay
                .cycle_status
                .last_cycle_close_height
                .map(|value| value.to_string()),
        },
    };

    (StatusCode::OK, Json(body)).into_response()
}

pub(crate) async fn canonical_tempo_status(State(state): State<AppState>) -> Response {
    let replay = match ReplayDriver::run(state.storage.pool(), None).await {
        Ok(replay) => replay,
        Err(err) if err.code == "replay_empty" => {
            return json_error(StatusCode::NOT_FOUND, "not_found", "not found");
        }
        Err(err) => {
            tracing::error!(?err, "failed to derive current tempo status");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let body = CanonicalTempoStatusResponse {
        tempo: CanonicalTempoStatus {
            cycle_age_ge_dmin: replay.tempo_status.cycle_age_ge_dmin,
            cycle_age_ge_dmax: replay.tempo_status.cycle_age_ge_dmax,
            constrained_mode: replay.tempo_status.constrained_mode,
            record_only_mode: replay.tempo_status.record_only_mode,
        },
    };

    (StatusCode::OK, Json(body)).into_response()
}

pub(crate) async fn canonical_event_log(State(state): State<AppState>) -> Response {
    let events = match sqlx::query_as::<_, EventLogEventRow>(
        r#"
        SELECT
          block_height,
          event_index,
          event_id,
          event_type
        FROM events
        ORDER BY block_height ASC, event_index ASC
        "#,
    )
    .fetch_all(state.storage.pool())
    .await
    {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to load canonical event log");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    if events.is_empty() {
        return (
            StatusCode::OK,
            Json(CanonicalEventLogResponse {
                events: Vec::new(),
                blocks: Vec::new(),
                cycles: Vec::new(),
            }),
        )
            .into_response();
    }

    let cycle_boundaries = match sqlx::query_as::<_, EventLogCycleBoundaryRow>(
        r#"
        SELECT
          cycle_index,
          source_event_id
        FROM cycle_boundaries
        ORDER BY source_block_height ASC, source_event_index ASC
        "#,
    )
    .fetch_all(state.storage.pool())
    .await
    {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(
                ?err,
                "failed to load canonical cycle boundaries for event log"
            );
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let replay = match ReplayDriver::run(state.storage.pool(), None).await {
        Ok(replay) => replay,
        Err(err) => {
            tracing::error!(
                ?err,
                "failed to derive replay status for canonical event log"
            );
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let mut blocks = Vec::<CanonicalEventLogBlockBand>::new();
    let mut events_response = Vec::<CanonicalEventLogEvent>::with_capacity(events.len());
    let mut event_global_index_by_id = HashMap::<Uuid, i64>::with_capacity(events.len());
    let mut block_start_index = 0_i64;
    let mut current_block_height = events[0].block_height;

    for (global_index, row) in events.iter().enumerate() {
        let global_index = global_index as i64;
        event_global_index_by_id.insert(row.event_id, global_index);
        events_response.push(CanonicalEventLogEvent {
            event_id: row.event_id.to_string(),
            global_index: global_index.to_string(),
            block_height: row.block_height.to_string(),
            block_event_index: row.event_index.to_string(),
            event_type: row.event_type.clone(),
        });

        let next_block_height = events
            .get(global_index as usize + 1)
            .map(|next| next.block_height);
        if row.block_height != current_block_height {
            current_block_height = row.block_height;
            block_start_index = global_index;
        }
        if next_block_height != Some(row.block_height) {
            blocks.push(CanonicalEventLogBlockBand {
                id: format!("block:{}", row.block_height),
                block_height: row.block_height.to_string(),
                start_global_index: block_start_index.to_string(),
                end_global_index: global_index.to_string(),
                label: format!("Block {}", row.block_height),
            });
        }
    }

    let mut cycles = Vec::<CanonicalEventLogCycleBand>::new();
    let mut cycle_start_index = 0_i64;

    for boundary in cycle_boundaries {
        let Some(end_global_index) = event_global_index_by_id
            .get(&boundary.source_event_id)
            .copied()
        else {
            continue;
        };
        cycles.push(CanonicalEventLogCycleBand {
            id: format!("cycle:{}", boundary.cycle_index),
            cycle_index: boundary.cycle_index.to_string(),
            start_global_index: cycle_start_index.to_string(),
            end_global_index: end_global_index.to_string(),
            label: format!("Cycle {}", boundary.cycle_index),
            closure_event_id: Some(boundary.source_event_id.to_string()),
        });
        cycle_start_index = end_global_index.saturating_add(1);
    }

    let last_global_index = i64::try_from(events.len().saturating_sub(1)).unwrap_or(0);
    if cycle_start_index <= last_global_index {
        cycles.push(CanonicalEventLogCycleBand {
            id: format!("cycle:{}", replay.cycle_status.cycle_index),
            cycle_index: replay.cycle_status.cycle_index.to_string(),
            start_global_index: cycle_start_index.to_string(),
            end_global_index: last_global_index.to_string(),
            label: format!("Cycle {}", replay.cycle_status.cycle_index),
            closure_event_id: None,
        });
    }

    (
        StatusCode::OK,
        Json(CanonicalEventLogResponse {
            events: events_response,
            blocks,
            cycles,
        }),
    )
        .into_response()
}

fn coordinate_node_response(
    row: &storage::IdeaSummaryRow,
    point: &CoordinatePoint,
) -> Result<CoordinateNodeResponse, Response> {
    let title = row.title.clone().ok_or_else(|| {
        json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            "idea title unavailable",
        )
    })?;
    Ok(CoordinateNodeResponse {
        id: row.idea_id.to_string(),
        x: point.x,
        y: point.y,
        title,
        sentence: row.sentence.clone(),
        idea_type: row.idea_type.clone(),
        derived_universal_rank: row.derived_universal_rank.map(|value| value.to_string()),
        ri_in_count: row.ri_in_count.to_string(),
        ri_out_count: row.ri_out_count.to_string(),
    })
}

pub(crate) async fn canonical_coordinates(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Response {
    let snapshot = match state.storage.get_latest_snapshot().await {
        Ok(Some(snapshot)) => snapshot,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load latest snapshot");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };

    let reference_id = match query.get("reference_id").map(|value| value.trim()) {
        Some(value) if !value.is_empty() => match parse_uuid_v7(value) {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        _ => None,
    };

    let config = CoordinateScatterConfig::default();
    let mode = if reference_id.is_some() {
        "reference".to_string()
    } else {
        "global".to_string()
    };

    let (coords, relaxed) = if let Some(reference_id) = reference_id {
        let mut connections = match state
            .storage
            .list_connections_for_idea(snapshot.block_height, reference_id)
            .await
        {
            Ok(rows) => rows,
            Err(err) => {
                tracing::error!(?err, "failed to load reference connections");
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                );
            }
        };
        connections.retain(|row| {
            is_reference_scoped_connection(reference_id, row, RelativeImportanceDirection::Both)
        });
        connections.sort_by(|left, right| {
            (
                left.created_block_height,
                left.created_event_index,
                left.connection_id.to_string(),
            )
                .cmp(&(
                    right.created_block_height,
                    right.created_event_index,
                    right.connection_id.to_string(),
                ))
        });

        let mut allowed_ids = HashSet::from([reference_id]);
        let mut earliest_relative_edge_by_neighbor = HashMap::<Uuid, (String, i64, i32)>::new();
        let mut neighbor_ids = Vec::<Uuid>::new();
        let mut seen_neighbor_ids = HashMap::<Uuid, ()>::new();
        for row in &connections {
            let Some(neighbor_id) = scoped_neighbor_from_reference(
                reference_id,
                row,
                RelativeImportanceDirection::Both,
            ) else {
                continue;
            };
            allowed_ids.insert(neighbor_id);

            if !seen_neighbor_ids.contains_key(&neighbor_id) {
                seen_neighbor_ids.insert(neighbor_id, ());
                neighbor_ids.push(neighbor_id);
            }

            if row.connection_type == "relative_importance" {
                earliest_relative_edge_by_neighbor
                    .entry(neighbor_id)
                    .or_insert_with(|| {
                        (
                            row.connection_id.to_string(),
                            row.created_block_height,
                            row.created_event_index,
                        )
                    });
            }
        }

        let mut idea_ids = neighbor_ids.clone();
        idea_ids.push(reference_id);
        let idea_rows = match state
            .storage
            .list_ideas_by_ids(snapshot.block_height, &idea_ids)
            .await
        {
            Ok(rows) => rows,
            Err(err) => {
                tracing::error!(?err, "failed to load coordinate ideas");
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                );
            }
        };

        let idea_rows_by_id = idea_rows
            .into_iter()
            .map(|row| (row.idea_id, row))
            .collect::<HashMap<_, _>>();
        let Some(reference_row) = idea_rows_by_id.get(&reference_id) else {
            return json_error(StatusCode::NOT_FOUND, "not_found", "not found");
        };

        let mut neighbor_rows = neighbor_ids
            .into_iter()
            .filter_map(|idea_id| idea_rows_by_id.get(&idea_id).cloned())
            .collect::<Vec<_>>();
        neighbor_rows.sort_by(|left, right| {
            let left_key = earliest_relative_edge_by_neighbor
                .get(&left.idea_id)
                .map(|(_, block_height, event_index)| (*block_height, *event_index))
                .unwrap_or((left.created_block_height, left.created_event_index));
            let right_key = earliest_relative_edge_by_neighbor
                .get(&right.idea_id)
                .map(|(_, block_height, event_index)| (*block_height, *event_index))
                .unwrap_or((right.created_block_height, right.created_event_index));

            left_key
                .cmp(&right_key)
                .then_with(|| left.idea_id.to_string().cmp(&right.idea_id.to_string()))
        });

        let scatter_projection = project_slots(
            &neighbor_rows
                .iter()
                .enumerate()
                .map(|(slot_idx, row)| CoordinateSlotInput {
                    id: row.idea_id.to_string(),
                    slot_idx,
                    hash_key: earliest_relative_edge_by_neighbor
                        .get(&row.idea_id)
                        .map(|(edge_id, block_height, event_index)| {
                            format!(
                                "edge:{edge_id}:idx:{block_height}:{event_index}:idea:{}",
                                row.idea_id
                            )
                        })
                        .unwrap_or_else(|| {
                            format!(
                                "idea:{}:birth:{}:{}",
                                row.idea_id, row.created_block_height, row.created_event_index
                            )
                        }),
                })
                .collect::<Vec<_>>(),
            config,
        );

        let mut response = vec![CoordinateNodeResponse {
            id: reference_row.idea_id.to_string(),
            x: 0.0,
            y: 0.0,
            title: reference_row
                .title
                .clone()
                .unwrap_or_else(|| reference_row.idea_id.to_string()),
            sentence: reference_row.sentence.clone(),
            idea_type: reference_row.idea_type.clone(),
            derived_universal_rank: reference_row
                .derived_universal_rank
                .map(|value| value.to_string()),
            ri_in_count: reference_row.ri_in_count.to_string(),
            ri_out_count: reference_row.ri_out_count.to_string(),
        }];

        for (row, point) in neighbor_rows.iter().zip(scatter_projection.points.iter()) {
            match coordinate_node_response(row, point) {
                Ok(entry) => response.push(entry),
                Err(response) => return response,
            }
        }

        response.retain(|entry| {
            Uuid::parse_str(&entry.id)
                .ok()
                .map(|id| allowed_ids.contains(&id))
                .unwrap_or(false)
        });
        debug_assert!(response.iter().all(|entry| {
            Uuid::parse_str(&entry.id)
                .ok()
                .map(|id| allowed_ids.contains(&id))
                .unwrap_or(false)
        }));

        (response, scatter_projection.relaxed)
    } else {
        let total = match state.storage.count_ideas(snapshot.block_height).await {
            Ok(total) => total,
            Err(err) => {
                tracing::error!(?err, "failed to count ideas for coordinates");
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                );
            }
        };
        let mut rows = match state
            .storage
            .list_ideas_top(snapshot.block_height, 0, total, false)
            .await
        {
            Ok(rows) => rows,
            Err(err) => {
                tracing::error!(?err, "failed to list ideas for coordinates");
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                );
            }
        };
        rows.sort_by(|left, right| {
            (
                left.created_block_height,
                left.created_event_index,
                left.idea_id.to_string(),
            )
                .cmp(&(
                    right.created_block_height,
                    right.created_event_index,
                    right.idea_id.to_string(),
                ))
        });

        let scatter_projection = project_slots(
            &rows
                .iter()
                .enumerate()
                .map(|(slot_idx, row)| CoordinateSlotInput {
                    id: row.idea_id.to_string(),
                    slot_idx,
                    hash_key: format!(
                        "idea:{}:birth:{}:{}",
                        row.idea_id, row.created_block_height, row.created_event_index
                    ),
                })
                .collect::<Vec<_>>(),
            config,
        );

        let mut response = Vec::with_capacity(rows.len());
        for (row, point) in rows.iter().zip(scatter_projection.points.iter()) {
            match coordinate_node_response(row, point) {
                Ok(entry) => response.push(entry),
                Err(response) => return response,
            }
        }
        (response, scatter_projection.relaxed)
    };

    let body = CoordinateViewResponse {
        mode,
        reference_id: reference_id.map(|value| value.to_string()),
        coords,
        meta: CoordinateMetaResponse {
            spacing: DEFAULT_SPACING,
            algo: "phyllotaxis_fnv1a64_jitter_relax_v2".to_string(),
            relaxed,
        },
    };

    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

#[cfg(feature = "full")]
pub(crate) async fn canonical_create_identity(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<CanonicalIdentityCreatePayload>,
) -> Response {
    if let Err(response) = validate_max_len(
        "identity_name",
        payload.identity_name.as_str(),
        IDEA_TITLE_MAX_CHARS,
    ) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.identity_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.event_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.identity_name.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.public_key.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.metadata.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.author_signature.as_deref()) {
        return response;
    }

    let identity_id = match payload.identity_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "identity_id") {
            Ok(value) => value,
            Err(response) => return response,
        },
        None => Uuid::now_v7(),
    };
    let event_id = match payload.event_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "event_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };

    let input = CanonicalIdentityCreateInput {
        identity_id,
        event_id,
        identity_name: payload.identity_name,
        public_key: payload.public_key,
        metadata: normalize_optional_text(payload.metadata),
        author_signature: normalize_optional_text(payload.author_signature),
    };

    let result = match state
        .storage
        .create_canonical_identity(auth.account_id, input)
        .await
    {
        Ok(result) => result,
        Err(err) => return canonical_write_error_response(err),
    };

    (
        StatusCode::OK,
        Json(CanonicalIdentityCreateResponse {
            identity_id: result.identity_id.to_string(),
            event_id: result.event_id.to_string(),
        }),
    )
        .into_response()
}

#[cfg(feature = "full")]
pub(crate) async fn canonical_blocked_submission(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<CanonicalBlockedSubmissionPayload>,
) -> Response {
    if let Err(response) = reject_secret_text(payload.blocked_reason_code.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.blocked_by_identity.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.reference_event_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.event_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.author_signature.as_deref()) {
        return response;
    }

    let blocked_by_identity =
        match parse_uuid_v7_field(payload.blocked_by_identity.trim(), "blocked_by_identity") {
            Ok(value) => value,
            Err(response) => return response,
        };
    let reference_event_id = match payload.reference_event_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "reference_event_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };
    let event_id = match payload.event_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "event_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };

    let input = CanonicalBlockedSubmissionInput {
        event_id,
        submission_hash: payload.submission_hash.trim().to_string(),
        blocked_reason_code: payload.blocked_reason_code.trim().to_string(),
        blocked_by_identity,
        reference_event_id,
        author_signature: normalize_optional_text(payload.author_signature),
    };
    let result = match state
        .storage
        .create_blocked_submission(auth.account_id, input)
        .await
    {
        Ok(result) => result,
        Err(err) => return canonical_write_error_response(err),
    };

    (
        StatusCode::OK,
        Json(CanonicalBlockedSubmissionResponse {
            event_id: result.event_id.to_string(),
            cycle_index: result.cycle_index.to_string(),
        }),
    )
        .into_response()
}

#[cfg(feature = "full")]
pub(crate) async fn canonical_create_idea(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<CanonicalIdeaCreatePayload>,
) -> Response {
    if let Err(response) = validate_max_len(
        "idea_type",
        payload.idea_type.as_str(),
        CANONICAL_IDEA_TYPE_MAX_CHARS,
    ) {
        return response;
    }
    if let Err(response) = validate_max_len("title", payload.title.as_str(), IDEA_TITLE_MAX_CHARS) {
        return response;
    }
    if let Err(response) = validate_max_len(
        "sentence",
        payload.sentence.as_str(),
        IDEA_SENTENCE_MAX_CHARS,
    ) {
        return response;
    }
    if let Err(response) = validate_optional_max_len(
        "paragraph",
        payload.paragraph.as_deref(),
        IDEA_PARAGRAPH_MAX_CHARS,
    ) {
        return response;
    }
    if let Err(response) =
        validate_optional_max_len("full", payload.full.as_deref(), IDEA_FULL_MAX_CHARS)
    {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.idea_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.event_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.author_signature.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.idea_type.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.title.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.sentence.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.paragraph.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.full.as_deref()) {
        return response;
    }

    let idea_id = match payload.idea_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "idea_id") {
            Ok(value) => value,
            Err(response) => return response,
        },
        None => Uuid::now_v7(),
    };
    let event_id = match payload.event_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "event_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };

    let input = CanonicalIdeaCreateInput {
        idea_id,
        event_id,
        idea_type: payload.idea_type.trim().to_string(),
        title: payload.title,
        sentence: payload.sentence,
        paragraph: normalize_optional_text(payload.paragraph),
        full: normalize_optional_text(payload.full),
        author_signature: normalize_optional_text(payload.author_signature),
    };

    let result = match state
        .storage
        .create_canonical_idea(auth.account_id, input)
        .await
    {
        Ok(result) => result,
        Err(err) => return canonical_write_error_response(err),
    };

    (
        StatusCode::OK,
        Json(CanonicalIdeaCreateResponse {
            idea_id: result.idea_id.to_string(),
            event_id: result.event_id.to_string(),
            cycle_index: result.cycle_index.to_string(),
            remaining_build_mana: result.remaining_build_mana.to_string(),
        }),
    )
        .into_response()
}

#[cfg(feature = "full")]
pub(crate) async fn canonical_create_connection(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<CanonicalConnectionCreatePayload>,
) -> Response {
    if let Err(response) = validate_max_len(
        "connection_type",
        payload.connection_type.as_str(),
        CANONICAL_CONNECTION_TYPE_MAX_CHARS,
    ) {
        return response;
    }
    if let Err(response) =
        validate_optional_max_len("usage", payload.usage.as_deref(), CANONICAL_USAGE_MAX_CHARS)
    {
        return response;
    }
    if let Err(response) =
        validate_optional_max_len("axis", payload.axis.as_deref(), CANONICAL_AXIS_MAX_CHARS)
    {
        return response;
    }
    if let Err(response) = validate_optional_max_len(
        "timeframe",
        payload.timeframe.as_deref(),
        CANONICAL_TIMEFRAME_MAX_CHARS,
    ) {
        return response;
    }
    if let Err(response) =
        validate_optional_max_len("scope", payload.scope.as_deref(), CANONICAL_SCOPE_MAX_CHARS)
    {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.connection_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.event_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.from_idea_id.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.to_idea_id.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.connection_type.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.usage.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.axis.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.timeframe.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.scope.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.author_signature.as_deref()) {
        return response;
    }

    let connection_id = match payload.connection_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "connection_id") {
            Ok(value) => value,
            Err(response) => return response,
        },
        None => Uuid::now_v7(),
    };
    let event_id = match payload.event_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "event_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };
    let from_idea_id = match parse_uuid_v7_field(payload.from_idea_id.trim(), "from_idea_id") {
        Ok(value) => value,
        Err(response) => return response,
    };
    let to_idea_id = match parse_uuid_v7_field(payload.to_idea_id.trim(), "to_idea_id") {
        Ok(value) => value,
        Err(response) => return response,
    };

    let input = CanonicalConnectionCreateInput {
        connection_id,
        event_id,
        from_idea_id,
        to_idea_id,
        connection_type: payload.connection_type.trim().to_string(),
        usage: normalize_optional_text(payload.usage),
        axis: normalize_optional_text(payload.axis),
        timeframe: normalize_optional_text(payload.timeframe),
        scope: normalize_optional_text(payload.scope),
        author_signature: normalize_optional_text(payload.author_signature),
    };

    let result = match state
        .storage
        .create_canonical_connection(auth.account_id, input)
        .await
    {
        Ok(result) => result,
        Err(err) => return canonical_write_error_response(err),
    };

    (
        StatusCode::OK,
        Json(CanonicalConnectionCreateResponse {
            connection_id: result.connection_id.to_string(),
            event_id: result.event_id.to_string(),
            cycle_index: result.cycle_index.to_string(),
            remaining_build_mana: result.remaining_build_mana.to_string(),
        }),
    )
        .into_response()
}

#[cfg(feature = "full")]
pub(crate) async fn canonical_create_importance_challenge(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<CanonicalImportanceChallengeCreatePayload>,
) -> Response {
    if let Err(response) = validate_max_len(
        "context_key",
        payload.context_key.as_str(),
        CANONICAL_CONTEXT_KEY_MAX_CHARS,
    ) {
        return response;
    }
    if let Err(response) = validate_max_len("axis", payload.axis.as_str(), CANONICAL_AXIS_MAX_CHARS)
    {
        return response;
    }
    if let Err(response) = validate_max_len(
        "timeframe",
        payload.timeframe.as_str(),
        CANONICAL_TIMEFRAME_MAX_CHARS,
    ) {
        return response;
    }
    if let Err(response) =
        validate_max_len("scope", payload.scope.as_str(), CANONICAL_SCOPE_MAX_CHARS)
    {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.challenge_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.event_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.framing_representation_ref.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.context_key.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.axis.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.timeframe.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.scope.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.target_left_idea_id.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.target_right_idea_id.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.reference_idea_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.author_signature.as_deref()) {
        return response;
    }

    let challenge_id = match payload.challenge_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "challenge_id") {
            Ok(value) => value,
            Err(response) => return response,
        },
        None => Uuid::now_v7(),
    };
    let event_id = match payload.event_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "event_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };
    let framing_representation_ref = match parse_uuid_v7_field(
        payload.framing_representation_ref.trim(),
        "framing_representation_ref",
    ) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let target_left_idea_id =
        match parse_uuid_v7_field(payload.target_left_idea_id.trim(), "target_left_idea_id") {
            Ok(value) => value,
            Err(response) => return response,
        };
    let target_right_idea_id =
        match parse_uuid_v7_field(payload.target_right_idea_id.trim(), "target_right_idea_id") {
            Ok(value) => value,
            Err(response) => return response,
        };
    let reference_idea_id = match payload.reference_idea_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "reference_idea_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };

    let input = CanonicalImportanceChallengeCreateInput {
        challenge_id,
        event_id,
        framing_representation_ref,
        context_key: payload.context_key.trim().to_string(),
        axis: payload.axis.trim().to_string(),
        timeframe: payload.timeframe.trim().to_string(),
        scope: payload.scope.trim().to_string(),
        target_left_idea_id,
        target_right_idea_id,
        reference_idea_id,
        author_signature: normalize_optional_text(payload.author_signature),
    };

    let result = match state
        .storage
        .create_canonical_importance_challenge(auth.account_id, input)
        .await
    {
        Ok(result) => result,
        Err(err) => return canonical_write_error_response(err),
    };

    (
        StatusCode::OK,
        Json(CanonicalChallengeCreateResponse {
            challenge_id: result.challenge_id.to_string(),
            event_id: result.event_id.to_string(),
            cycle_index: result.cycle_index.to_string(),
            remaining_build_mana: result.remaining_build_mana.to_string(),
        }),
    )
        .into_response()
}

#[cfg(feature = "full")]
pub(crate) async fn canonical_attach_importance_argument(
    Path(challenge_id): Path<String>,
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<CanonicalImportanceArgumentAttachPayload>,
) -> Response {
    if let Err(response) = reject_secret_text(challenge_id.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.connection_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.event_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.argument_idea_id.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.subject_idea_id.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.author_signature.as_deref()) {
        return response;
    }

    let challenge_id = match parse_uuid_v7_field(challenge_id.trim(), "challenge_id") {
        Ok(value) => value,
        Err(response) => return response,
    };
    let connection_id = match payload.connection_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "connection_id") {
            Ok(value) => value,
            Err(response) => return response,
        },
        None => Uuid::now_v7(),
    };
    let event_id = match payload.event_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "event_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };
    let argument_idea_id =
        match parse_uuid_v7_field(payload.argument_idea_id.trim(), "argument_idea_id") {
            Ok(value) => value,
            Err(response) => return response,
        };
    let subject_idea_id =
        match parse_uuid_v7_field(payload.subject_idea_id.trim(), "subject_idea_id") {
            Ok(value) => value,
            Err(response) => return response,
        };

    let input = CanonicalImportanceArgumentAttachInput {
        challenge_id,
        connection_id,
        event_id,
        argument_idea_id,
        subject_idea_id,
        author_signature: normalize_optional_text(payload.author_signature),
    };

    let result = match state
        .storage
        .create_canonical_importance_argument_attach(auth.account_id, input)
        .await
    {
        Ok(result) => result,
        Err(err) => return canonical_write_error_response(err),
    };

    (
        StatusCode::OK,
        Json(CanonicalChallengeArgumentAttachResponse {
            challenge_id: result.challenge_id.to_string(),
            connection_id: result.connection_id.to_string(),
            event_id: result.event_id.to_string(),
            cycle_index: result.cycle_index.to_string(),
            remaining_build_mana: result.remaining_build_mana.to_string(),
        }),
    )
        .into_response()
}

#[cfg(feature = "full")]
pub(crate) async fn canonical_pull_vote_session(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<CanonicalVoteSessionPullPayload>,
) -> Response {
    if let Err(response) = reject_secret_optional_text(payload.vote_session_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.event_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.author_signature.as_deref()) {
        return response;
    }

    let vote_session_id = match payload.vote_session_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "vote_session_id") {
            Ok(value) => value,
            Err(response) => return response,
        },
        None => Uuid::now_v7(),
    };
    let event_id = match payload.event_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "event_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };

    let input = CanonicalVoteSessionPullInput {
        vote_session_id,
        event_id,
        author_signature: normalize_optional_text(payload.author_signature),
    };
    let result = match state
        .storage
        .pull_canonical_vote_session(auth.account_id, input)
        .await
    {
        Ok(result) => result,
        Err(err) => return canonical_write_error_response(err),
    };

    (
        StatusCode::OK,
        Json(CanonicalVoteSessionPullResponse {
            vote_session_id: result.vote_session_id.to_string(),
            challenge_id: result.challenge_id.to_string(),
            event_id: result.event_id.to_string(),
            session_index: result.session_index.to_string(),
            cycle_index: result.cycle_index.to_string(),
            remaining_voting_mana: result.remaining_voting_mana.to_string(),
        }),
    )
        .into_response()
}

#[cfg(feature = "full")]
pub(crate) async fn canonical_cast_challenge_vote(
    Path(challenge_id): Path<String>,
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<CanonicalChallengeVoteCastPayload>,
) -> Response {
    if let Err(response) = validate_max_len(
        "vote_choice",
        payload.vote_choice.as_str(),
        CANONICAL_VOTE_CHOICE_MAX_CHARS,
    ) {
        return response;
    }
    if let Err(response) = reject_secret_text(challenge_id.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.vote_session_id.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.vote_choice.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.event_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.author_signature.as_deref()) {
        return response;
    }

    let challenge_id = match parse_uuid_v7_field(challenge_id.trim(), "challenge_id") {
        Ok(value) => value,
        Err(response) => return response,
    };
    let vote_session_id =
        match parse_uuid_v7_field(payload.vote_session_id.trim(), "vote_session_id") {
            Ok(value) => value,
            Err(response) => return response,
        };
    let event_id = match payload.event_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "event_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };

    let input = CanonicalVoteCastInput {
        challenge_id,
        vote_session_id,
        vote_choice: payload.vote_choice,
        event_id,
        author_signature: normalize_optional_text(payload.author_signature),
    };
    let result = match state
        .storage
        .cast_canonical_importance_vote(auth.account_id, input)
        .await
    {
        Ok(result) => result,
        Err(err) => return canonical_write_error_response(err),
    };

    (
        StatusCode::OK,
        Json(CanonicalVoteCastResponse {
            challenge_id: result.challenge_id.to_string(),
            vote_event_id: result.vote_event_id.to_string(),
            cycle_index: result.cycle_index.to_string(),
            remaining_voting_mana: result.remaining_voting_mana.to_string(),
            verdict_event_id: result.verdict_event_id.map(|value| value.to_string()),
            verdict_outcome: result.verdict_outcome,
        }),
    )
        .into_response()
}

#[cfg(feature = "full")]
pub(crate) async fn canonical_verifier_grant_writer(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<CanonicalVerifierGrantPayload>,
) -> Response {
    if let Err(response) = validate_optional_max_len(
        "canonical_writer_level",
        payload.canonical_writer_level.as_deref(),
        CANONICAL_WRITER_LEVEL_MAX_CHARS,
    ) {
        return response;
    }
    if let Err(response) = reject_secret_text(payload.identity_id.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.canonical_writer_level.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.event_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.author_signature.as_deref()) {
        return response;
    }

    let identity_id = match parse_uuid_v7_field(payload.identity_id.trim(), "identity_id") {
        Ok(value) => value,
        Err(response) => return response,
    };
    let canonical_writer_level = match payload.canonical_writer_level.as_deref() {
        Some(value) => {
            let parsed = match parse_non_negative_i64(value.trim()) {
                Some(parsed) => parsed,
                None => {
                    return json_error(
                        StatusCode::BAD_REQUEST,
                        "invalid_request",
                        "canonical_writer_level must be a non-negative integer",
                    )
                }
            };
            if parsed > i16::MAX as i64 {
                return json_error(
                    StatusCode::BAD_REQUEST,
                    "invalid_request",
                    "canonical_writer_level is out of range",
                );
            }
            parsed as i16
        }
        None => 1,
    };
    let email_verified = payload.email_verified.unwrap_or(true);
    let event_id = match payload.event_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "event_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };

    let input = CanonicalVerifierGrantInput {
        identity_id,
        canonical_writer_level,
        email_verified,
        event_id,
        author_signature: normalize_optional_text(payload.author_signature),
    };
    let result = match state
        .storage
        .grant_canonical_writer_level(auth.account_id, input)
        .await
    {
        Ok(result) => result,
        Err(err) => return canonical_write_error_response(err),
    };

    (
        StatusCode::OK,
        Json(CanonicalVerifierGrantResponse {
            identity_id: result.identity_id.to_string(),
            event_id: result.event_id.to_string(),
            canonical_writer_level: result.canonical_writer_level.to_string(),
            email_verified: result.email_verified,
            cycle_index: result.cycle_index.to_string(),
        }),
    )
        .into_response()
}

#[cfg(feature = "full")]
pub(crate) async fn canonical_verifier_revoke_writer(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<CanonicalVerifierRevokePayload>,
) -> Response {
    if let Err(response) = reject_secret_text(payload.identity_id.as_str()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.event_id.as_deref()) {
        return response;
    }
    if let Err(response) = reject_secret_optional_text(payload.author_signature.as_deref()) {
        return response;
    }

    let identity_id = match parse_uuid_v7_field(payload.identity_id.trim(), "identity_id") {
        Ok(value) => value,
        Err(response) => return response,
    };
    let event_id = match payload.event_id.as_deref() {
        Some(value) => match parse_uuid_v7_field(value.trim(), "event_id") {
            Ok(value) => Some(value),
            Err(response) => return response,
        },
        None => None,
    };
    let input = CanonicalVerifierRevokeInput {
        identity_id,
        event_id,
        author_signature: normalize_optional_text(payload.author_signature),
    };
    let result = match state
        .storage
        .revoke_canonical_writer_level(auth.account_id, input)
        .await
    {
        Ok(result) => result,
        Err(err) => return canonical_write_error_response(err),
    };

    (
        StatusCode::OK,
        Json(CanonicalVerifierRevokeResponse {
            identity_id: result.identity_id.to_string(),
            event_id: result.event_id.to_string(),
            canonical_writer_level: "0".to_string(),
            email_verified: false,
            cycle_index: result.cycle_index.to_string(),
        }),
    )
        .into_response()
}

pub(crate) async fn canonical_verification_status(
    Path(identity_id): Path<String>,
    State(state): State<AppState>,
) -> Response {
    let identity_id = match parse_uuid_v7_field(identity_id.trim(), "identity_id") {
        Ok(value) => value,
        Err(response) => return response,
    };
    let verification = match state
        .storage
        .get_canonical_verification_status(identity_id)
        .await
    {
        Ok(Some(status)) => status,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load canonical verification status");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    (
        StatusCode::OK,
        Json(CanonicalVerificationStatusResponse {
            verification: CanonicalVerificationStatus {
                identity_id: verification.identity_id.to_string(),
                email_verified: verification.email_verified,
                canonical_writer_level: verification.canonical_writer_level.to_string(),
                active_verifier: verification.active_verifier,
                last_updated_event_id: verification
                    .last_updated_event_id
                    .map(|value| value.to_string()),
                last_updated_block_height: verification
                    .last_updated_block_height
                    .map(|value| value.to_string()),
                last_updated_event_index: verification
                    .last_updated_event_index
                    .map(|value| value.to_string()),
            },
        }),
    )
        .into_response()
}

pub(crate) async fn canonical_challenge_detail(
    Path(challenge_id): Path<String>,
    State(state): State<AppState>,
) -> Response {
    let challenge_id = match parse_uuid_v7_field(challenge_id.trim(), "challenge_id") {
        Ok(value) => value,
        Err(response) => return response,
    };

    let detail = match state
        .storage
        .get_canonical_challenge_detail(challenge_id)
        .await
    {
        Ok(Some(detail)) => detail,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load canonical challenge detail");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let body = CanonicalChallengeDetailResponse {
        challenge: CanonicalChallengeDetail {
            challenge_id: detail.challenge.challenge_id.to_string(),
            challenge_domain: detail.challenge.challenge_domain,
            context_key: detail.challenge.context_key,
            axis: detail.challenge.axis,
            timeframe: detail.challenge.timeframe,
            scope: detail.challenge.scope,
            target_left_idea_id: detail.challenge.target_left_idea_id.to_string(),
            target_right_idea_id: detail.challenge.target_right_idea_id.to_string(),
            reference_idea_id: detail
                .challenge
                .reference_idea_id
                .map(|value| value.to_string()),
            framing_representation_ref: detail.challenge.framing_representation_ref.to_string(),
            created_by_identity_id: detail.challenge.created_by_identity_id.to_string(),
            created_event_id: detail.challenge.created_event_id.to_string(),
            created_cycle_index: detail.challenge.created_cycle_index.to_string(),
            current_cycle_index: detail.current_cycle_index.to_string(),
            phase: detail.phase,
            arguments: detail
                .arguments
                .iter()
                .map(|row| CanonicalChallengeArgumentSummary {
                    connection_id: row.connection_id.to_string(),
                    argument_idea_id: row.argument_idea_id.to_string(),
                    subject_idea_id: row.subject_idea_id.to_string(),
                    created_event_id: row.created_event_id.to_string(),
                })
                .collect(),
            votes: detail
                .votes
                .iter()
                .map(|row| CanonicalChallengeVoteSummary {
                    vote_event_id: row.cast_event_id.to_string(),
                    vote_session_id: row.vote_session_id.to_string(),
                    voter_identity_id: row.voter_identity_id.to_string(),
                    vote_choice: row.vote_choice.clone(),
                })
                .collect(),
            verdict: detail
                .verdict
                .as_ref()
                .map(|verdict| CanonicalChallengeVerdictSummary {
                    verdict_id: verdict.verdict_id.to_string(),
                    verdict_event_id: verdict.verdict_event_id.to_string(),
                    winning_choice: verdict.winning_choice.clone(),
                    winning_target_idea_id: verdict
                        .winning_target_idea_id
                        .map(|value| value.to_string()),
                    left_votes: verdict.left_votes.to_string(),
                    right_votes: verdict.right_votes.to_string(),
                    total_votes: verdict.total_votes.to_string(),
                }),
        },
    };

    (StatusCode::OK, Json(body)).into_response()
}
