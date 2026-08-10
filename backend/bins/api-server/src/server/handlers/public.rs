use api_types_canonical::{
    ApiCapabilitiesResponse, CanonicalOrderingResponse, CanonicalOrderingsResponse,
    CanonicalRepresentationResponse, CanonicalRepresentationsResponse, ExactMatchIdeasResponse,
    IdeaBatchResolutionResponse, IdeaDetailResponse, IdeasTopResponse, IdentityInfo,
    IdentityResponse, NeighborhoodResponse, RelativeImportanceConnectionsResponse,
    SearchIdeasResponse, SnapshotCommitListResponse, SnapshotCommitResponse,
    SnapshotLatestResponse, SnapshotResponse, API_CONTRACT_VERSION, EXPECTED_MIGRATION_HEAD,
    PUBLIC_CONTRACT_ARTIFACT, SNAPSHOT_FORMAT_VERSION,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use common::security_limits::{IDEA_SENTENCE_MAX_CHARS, IDEA_TITLE_MAX_CHARS};
use replay::ReplayDriver;
use std::collections::{BTreeMap, BTreeSet};
use storage::IdeaSummaryRow;
use uuid::Uuid;

use crate::server::errors::json_error;
use crate::server::helpers::{
    clamp_limit, is_reference_scoped_connection, parse_bounded_uuid_v7_csv, parse_ideas_top_order,
    parse_non_negative_i64, parse_relative_importance_direction, parse_uuid_v7, resolve_snapshot,
    scoped_neighbor_from_reference,
};
use crate::server::mapping::{
    canonical_ordering_detail_from_replay, canonical_ordering_summary,
    canonical_representation_detail, connection_summary, idea_detail, idea_summary, snapshot_basis,
    snapshot_commit_metadata, snapshot_headers, snapshot_metadata, with_headers,
};
use crate::server::types::{
    AppState, BatchIdeaResolutionQuery, BoundedListQuery, ExactMatchIdeasQuery, IdeaDetailQuery,
    IdeasTopOrder, IdeasTopQuery, NeighborhoodQuery, OrderingDetailQuery,
    RelativeImportanceConnectionsQuery, SearchIdeasQuery, SnapshotByHeightPath,
    SnapshotCommitByHeightPath, SnapshotCommitListQuery, SnapshotLatestQuery, SnapshotPinQuery,
};

pub(crate) async fn health_check() -> Response {
    (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response()
}

pub(crate) async fn capabilities(State(state): State<AppState>) -> Response {
    let migration = match state.storage.get_applied_migration_head().await {
        Ok(Some(migration)) => migration,
        Ok(None) => {
            return json_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "migration_head_unavailable",
                "migration head unavailable",
            )
        }
        Err(err) => {
            tracing::error!(?err, "failed to load applied migration head");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let migration_head = format!(
        "{:04}_{}",
        migration.version,
        migration.description.replace([' ', '-'], "_")
    );
    if migration_head != EXPECTED_MIGRATION_HEAD {
        tracing::warn!(
            actual = migration_head,
            expected = EXPECTED_MIGRATION_HEAD,
            "database migration head differs from this build"
        );
    }
    #[cfg(feature = "full")]
    let active_feature_profile = "full";
    #[cfg(not(feature = "full"))]
    let active_feature_profile = "open_core";

    (
        StatusCode::OK,
        Json(ApiCapabilitiesResponse {
            api_contract_version: API_CONTRACT_VERSION.to_string(),
            build_revision: option_env!("SEED_BUILD_REVISION")
                .unwrap_or(env!("CARGO_PKG_VERSION"))
                .to_string(),
            migration_head,
            snapshot_format_version: SNAPSHOT_FORMAT_VERSION.to_string(),
            active_feature_profile: active_feature_profile.to_string(),
            supported_canonical_signed_write_kinds: vec![
                "idea_create".to_string(),
                "connection_create".to_string(),
            ],
            public_contract_artifact: PUBLIC_CONTRACT_ARTIFACT.to_string(),
        }),
    )
        .into_response()
}

pub(crate) async fn latest_snapshot(
    State(state): State<AppState>,
    Query(query): Query<SnapshotLatestQuery>,
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
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };

    let metadata = match snapshot_metadata(&snapshot) {
        Ok(metadata) => metadata,
        Err(response) => return response,
    };

    let preview_ideas = if query.include_preview.unwrap_or(false) {
        match state
            .storage
            .list_ideas_top(snapshot.block_height, 0, 10, false)
            .await
        {
            Ok(rows) => {
                let mut ideas = Vec::with_capacity(rows.len());
                for row in rows {
                    match idea_summary(&row) {
                        Ok(summary) => ideas.push(summary),
                        Err(response) => return response,
                    }
                }
                Some(ideas)
            }
            Err(err) => {
                tracing::error!(?err, "failed to load preview ideas");
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                );
            }
        }
    } else {
        None
    };

    let body = SnapshotLatestResponse {
        basis,
        snapshot: metadata,
        preview_ideas,
    };

    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

pub(crate) async fn snapshot_by_height(
    State(state): State<AppState>,
    Path(path): Path<SnapshotByHeightPath>,
) -> Response {
    let height = match parse_non_negative_i64(&path.height) {
        Some(height) => height,
        None => return json_error(StatusCode::BAD_REQUEST, "invalid_request", "invalid height"),
    };

    let snapshot = match state.storage.get_snapshot_by_height(height).await {
        Ok(Some(snapshot)) => snapshot,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load snapshot by height");
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
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };

    let metadata = match snapshot_metadata(&snapshot) {
        Ok(metadata) => metadata,
        Err(response) => return response,
    };

    let body = SnapshotResponse {
        basis,
        snapshot: metadata,
    };
    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

pub(crate) async fn snapshot_commit_list(
    State(state): State<AppState>,
    Query(query): Query<SnapshotCommitListQuery>,
) -> Response {
    let limit = query
        .limit
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(50)
        .clamp(1, 200);

    let rows = match state.storage.list_snapshot_commits(limit).await {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to load snapshot commits");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    (
        StatusCode::OK,
        Json(SnapshotCommitListResponse {
            commits: rows.iter().map(snapshot_commit_metadata).collect(),
        }),
    )
        .into_response()
}

pub(crate) async fn snapshot_commit_by_height(
    State(state): State<AppState>,
    Path(path): Path<SnapshotCommitByHeightPath>,
) -> Response {
    let height = match parse_non_negative_i64(&path.height) {
        Some(height) => height,
        None => return json_error(StatusCode::BAD_REQUEST, "invalid_request", "invalid height"),
    };

    let row = match state.storage.get_snapshot_commit_by_height(height).await {
        Ok(Some(row)) => row,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load snapshot commit by height");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    (
        StatusCode::OK,
        Json(SnapshotCommitResponse {
            commit: snapshot_commit_metadata(&row),
        }),
    )
        .into_response()
}

pub(crate) async fn ideas_top(
    State(state): State<AppState>,
    Query(query): Query<IdeasTopQuery>,
) -> Response {
    let limit = query
        .limit
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(50);
    let offset = query
        .offset
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(0);
    let order = match parse_ideas_top_order(query.order.as_deref()) {
        Ok(order) => order,
        Err(response) => return response,
    };

    let limit = clamp_limit(limit);

    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };

    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };

    let total = match state.storage.count_ideas(snapshot.block_height).await {
        Ok(total) => total,
        Err(err) => {
            tracing::error!(?err, "failed to count ideas");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let rows = match state
        .storage
        .list_ideas_top(
            snapshot.block_height,
            offset,
            limit,
            matches!(order, IdeasTopOrder::Desc),
        )
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to list ideas");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let mut ideas = Vec::with_capacity(rows.len());
    for row in rows {
        match idea_summary(&row) {
            Ok(summary) => ideas.push(summary),
            Err(response) => return response,
        }
    }

    let body = IdeasTopResponse {
        basis,
        ideas,
        total: total.to_string(),
        offset: offset.to_string(),
        limit: limit.to_string(),
    };

    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

pub(crate) async fn idea_detail_handler(
    Path(idea_id): Path<String>,
    State(state): State<AppState>,
    Query(query): Query<IdeaDetailQuery>,
) -> Response {
    let idea_id = match parse_uuid_v7(&idea_id) {
        Ok(idea_id) => idea_id,
        Err(response) => return response,
    };
    let connection_limit = query
        .connection_limit
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(200)
        .clamp(1, 200);
    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };

    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };

    let idea = match state
        .storage
        .get_idea_detail(snapshot.block_height, idea_id)
        .await
    {
        Ok(Some(idea)) => idea,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load idea detail");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let connections = match state
        .storage
        .list_connections_for_idea_bounded(snapshot.block_height, idea_id, connection_limit)
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to load idea connections");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let mut incoming = Vec::new();
    let mut outgoing = Vec::new();
    for row in connections {
        if row.to_idea_id == idea_id {
            incoming.push(connection_summary(&row));
        }
        if row.from_idea_id == idea_id {
            outgoing.push(connection_summary(&row));
        }
    }

    let detail = match idea_detail(&idea, incoming, outgoing) {
        Ok(detail) => detail,
        Err(response) => return response,
    };

    let response = (
        StatusCode::OK,
        Json(IdeaDetailResponse {
            basis,
            idea: detail,
        }),
    )
        .into_response();
    with_headers(response, headers)
}

pub(crate) async fn identity_detail_handler(
    Path(identity_id): Path<String>,
    State(state): State<AppState>,
    Query(query): Query<SnapshotPinQuery>,
) -> Response {
    let identity_id = match parse_uuid_v7(&identity_id) {
        Ok(identity_id) => identity_id,
        Err(response) => return response,
    };

    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };
    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };

    let identity = match state
        .storage
        .get_identity_at_snapshot(snapshot.block_height, identity_id)
        .await
    {
        Ok(Some(identity)) => identity,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load identity");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let body = IdentityResponse {
        basis,
        identity: IdentityInfo {
            identity_id: identity.identity_id.to_string(),
            title: identity.title,
        },
    };

    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

pub(crate) async fn idea_neighborhood(
    Path(idea_id): Path<String>,
    State(state): State<AppState>,
    Query(query): Query<NeighborhoodQuery>,
) -> Response {
    let idea_id = match parse_uuid_v7(&idea_id) {
        Ok(idea_id) => idea_id,
        Err(response) => return response,
    };
    let depth = query
        .depth
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(1)
        .clamp(1, 2);
    let relative_importance_direction =
        match parse_relative_importance_direction(query.ri_dir.as_deref()) {
            Ok(value) => value,
            Err(response) => return response,
        };
    let limit_per_hop = query
        .limit_per_hop
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(50)
        .clamp(1, 200) as usize;

    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };

    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };

    let central_row = match state
        .storage
        .get_idea_detail(snapshot.block_height, idea_id)
        .await
    {
        Ok(Some(idea)) => idea,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load central idea");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let mut central_connections = match state
        .storage
        .list_connections_for_idea_bounded(snapshot.block_height, idea_id, limit_per_hop as i64)
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to load central connections");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    central_connections
        .retain(|row| is_reference_scoped_connection(idea_id, row, relative_importance_direction));

    central_connections.sort_by_key(|row| (row.created_block_height, row.created_event_index));

    let mut incoming = Vec::new();
    let mut outgoing = Vec::new();
    for row in &central_connections {
        if row.to_idea_id == idea_id {
            incoming.push(connection_summary(row));
        }
        if row.from_idea_id == idea_id {
            outgoing.push(connection_summary(row));
        }
    }

    let mut hop1_seen = BTreeSet::new();
    let mut hop1_ids = Vec::new();
    for row in &central_connections {
        let Some(neighbor) =
            scoped_neighbor_from_reference(idea_id, row, relative_importance_direction)
        else {
            continue;
        };
        if neighbor == idea_id || !hop1_seen.insert(neighbor) {
            continue;
        }
        hop1_ids.push(neighbor);
    }
    if hop1_ids.len() > limit_per_hop {
        hop1_ids.truncate(limit_per_hop);
    }

    let hop1_rows = if hop1_ids.is_empty() {
        Vec::new()
    } else {
        match state
            .storage
            .list_ideas_by_ids(snapshot.block_height, &hop1_ids)
            .await
        {
            Ok(rows) => rows,
            Err(err) => {
                tracing::error!(?err, "failed to load adjacent ideas");
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                );
            }
        }
    };

    let mut adjacent = Vec::new();
    let mut hop1_map: BTreeMap<Uuid, IdeaSummaryRow> = hop1_rows
        .into_iter()
        .map(|row| (row.idea_id, row))
        .collect();
    for id in &hop1_ids {
        if let Some(row) = hop1_map.remove(id) {
            match idea_summary(&row) {
                Ok(summary) => adjacent.push(summary),
                Err(response) => return response,
            }
        }
    }

    let mut depth_reached = 1;
    let mut all_connections = central_connections;
    let all_adjacent_ids: Vec<Uuid> = adjacent
        .iter()
        .filter_map(|idea| Uuid::parse_str(&idea.idea_id).ok())
        .collect();

    if depth == 2 && !all_adjacent_ids.is_empty() {
        let neighbor_connections = match state
            .storage
            .list_connections_for_ideas_bounded(
                snapshot.block_height,
                &all_adjacent_ids,
                (limit_per_hop * 2) as i64,
            )
            .await
        {
            Ok(rows) => rows,
            Err(err) => {
                tracing::error!(?err, "failed to load neighborhood connections");
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                );
            }
        };
        all_connections.extend(neighbor_connections);
    }

    all_connections.sort_by_key(|row| (row.created_block_height, row.created_event_index));

    if depth == 2 && !all_adjacent_ids.is_empty() {
        let hop1_set: BTreeSet<Uuid> = hop1_ids.iter().cloned().collect();
        let mut hop2_seen: BTreeSet<Uuid> = BTreeSet::new();
        let mut hop2_ids: Vec<Uuid> = Vec::new();
        for row in &all_connections {
            let candidates = [row.from_idea_id, row.to_idea_id];
            for candidate in candidates {
                if candidate == idea_id
                    || hop1_set.contains(&candidate)
                    || !hop2_seen.insert(candidate)
                {
                    continue;
                }
                hop2_ids.push(candidate);
                if hop2_ids.len() >= limit_per_hop {
                    break;
                }
            }
            if hop2_ids.len() >= limit_per_hop {
                break;
            }
        }

        if !hop2_ids.is_empty() {
            let hop2_rows = match state
                .storage
                .list_ideas_by_ids(snapshot.block_height, &hop2_ids)
                .await
            {
                Ok(rows) => rows,
                Err(err) => {
                    tracing::error!(?err, "failed to load depth-2 ideas");
                    return json_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "internal_error",
                        "internal server error",
                    );
                }
            };

            let mut hop2_map: BTreeMap<Uuid, IdeaSummaryRow> = hop2_rows
                .into_iter()
                .map(|row| (row.idea_id, row))
                .collect();
            for id in &hop2_ids {
                if let Some(row) = hop2_map.remove(id) {
                    match idea_summary(&row) {
                        Ok(summary) => adjacent.push(summary),
                        Err(response) => return response,
                    }
                }
            }

            depth_reached = 2;
        }
    }

    let connections = all_connections
        .iter()
        .map(connection_summary)
        .collect::<Vec<_>>();

    let central_detail = match idea_detail(&central_row, incoming, outgoing) {
        Ok(detail) => detail,
        Err(response) => return response,
    };

    let body = NeighborhoodResponse {
        basis,
        central_idea: central_detail,
        adjacent_ideas: adjacent,
        connections,
        depth_reached: depth_reached.to_string(),
    };

    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

pub(crate) async fn ordering_detail_handler(
    Path(ordering_id): Path<String>,
    State(state): State<AppState>,
    Query(query): Query<OrderingDetailQuery>,
) -> Response {
    let ordering_id = match parse_uuid_v7(&ordering_id) {
        Ok(ordering_id) => ordering_id,
        Err(response) => return response,
    };
    let item_limit = query
        .item_limit
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(200)
        .clamp(1, 200);

    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };
    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };

    let replay = match ReplayDriver::run(state.storage.pool(), Some(snapshot.block_height)).await {
        Ok(replay) => replay,
        Err(err) => {
            tracing::error!(?err, "failed to replay ordering detail at snapshot");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let Some(row) = replay
        .orderings
        .iter()
        .find(|ordering| ordering.ordering_id == ordering_id)
    else {
        return json_error(StatusCode::NOT_FOUND, "not_found", "not found");
    };

    let body = CanonicalOrderingResponse {
        basis,
        ordering: canonical_ordering_detail_from_replay(
            row,
            &replay.representations,
            item_limit as usize,
        ),
    };
    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

pub(crate) async fn idea_orderings_handler(
    Path(idea_id): Path<String>,
    State(state): State<AppState>,
    Query(query): Query<BoundedListQuery>,
) -> Response {
    let idea_id = match parse_uuid_v7(&idea_id) {
        Ok(idea_id) => idea_id,
        Err(response) => return response,
    };
    let limit = query
        .limit
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(200)
        .clamp(1, 200);

    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };
    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };

    let rows = match state
        .storage
        .list_canonical_orderings_for_idea_bounded(snapshot.block_height, idea_id, limit)
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to load orderings for idea");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let body = CanonicalOrderingsResponse {
        basis,
        orderings: rows.iter().map(canonical_ordering_summary).collect(),
    };
    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

pub(crate) async fn representation_detail_handler(
    Path(representation_id): Path<String>,
    State(state): State<AppState>,
    Query(query): Query<SnapshotPinQuery>,
) -> Response {
    let representation_id = match parse_uuid_v7(&representation_id) {
        Ok(representation_id) => representation_id,
        Err(response) => return response,
    };

    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };
    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };

    let row = match state
        .storage
        .get_canonical_representation(snapshot.block_height, representation_id)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load canonical representation");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let body = CanonicalRepresentationResponse {
        basis,
        representation: canonical_representation_detail(&row),
    };
    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

pub(crate) async fn idea_representations_handler(
    Path(idea_id): Path<String>,
    State(state): State<AppState>,
    Query(query): Query<BoundedListQuery>,
) -> Response {
    let idea_id = match parse_uuid_v7(&idea_id) {
        Ok(idea_id) => idea_id,
        Err(response) => return response,
    };
    let limit = query
        .limit
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(50)
        .clamp(1, 200);
    let offset = query
        .offset
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(0);
    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };
    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };
    match state
        .storage
        .get_idea_detail(snapshot.block_height, idea_id)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to verify representation target idea");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    }
    let total = match state
        .storage
        .count_canonical_representations_for_idea(snapshot.block_height, idea_id)
        .await
    {
        Ok(total) => total,
        Err(err) => {
            tracing::error!(?err, "failed to count idea representations");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let rows = match state
        .storage
        .list_canonical_representations_for_idea(snapshot.block_height, idea_id, offset, limit)
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to list idea representations");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let body = CanonicalRepresentationsResponse {
        basis,
        representations: rows.iter().map(canonical_representation_detail).collect(),
        total: total.to_string(),
        offset: offset.to_string(),
        limit: limit.to_string(),
    };
    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

pub(crate) async fn resolve_ideas_batch(
    State(state): State<AppState>,
    Query(query): Query<BatchIdeaResolutionQuery>,
) -> Response {
    let idea_ids = match parse_bounded_uuid_v7_csv(query.idea_ids.as_deref(), "idea_ids", 200) {
        Ok(ids) => ids,
        Err(response) => return response,
    };
    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };
    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };
    let rows = match state
        .storage
        .list_ideas_by_ids(snapshot.block_height, &idea_ids)
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to resolve idea batch");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let mut rows_by_id: BTreeMap<Uuid, IdeaSummaryRow> =
        rows.into_iter().map(|row| (row.idea_id, row)).collect();
    let mut ideas = Vec::new();
    let mut missing_idea_ids = Vec::new();
    for idea_id in idea_ids {
        match rows_by_id.remove(&idea_id) {
            Some(row) => match idea_summary(&row) {
                Ok(summary) => ideas.push(summary),
                Err(response) => return response,
            },
            None => missing_idea_ids.push(idea_id.to_string()),
        }
    }
    let body = IdeaBatchResolutionResponse {
        basis,
        ideas,
        missing_idea_ids,
    };
    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

pub(crate) async fn exact_match_ideas(
    State(state): State<AppState>,
    Query(query): Query<ExactMatchIdeasQuery>,
) -> Response {
    let field = match query.field.as_deref().map(str::trim) {
        Some("title") => "title",
        Some("sentence") => "sentence",
        _ => {
            return json_error(
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "field must be title or sentence",
            )
        }
    };
    let value = match query.value.as_deref() {
        Some(value) if !value.is_empty() => value,
        _ => return json_error(StatusCode::BAD_REQUEST, "invalid_request", "missing value"),
    };
    let max_chars = if field == "title" {
        IDEA_TITLE_MAX_CHARS
    } else {
        IDEA_SENTENCE_MAX_CHARS
    };
    if value.chars().count() > max_chars {
        return json_error(
            StatusCode::BAD_REQUEST,
            "invalid_field_length",
            "exact-match value exceeds the canonical field limit",
        );
    }
    let limit = query
        .limit
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(50)
        .clamp(1, 200);
    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };
    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };
    let mut ids = match state
        .storage
        .list_exact_match_idea_ids(snapshot.block_height, field, value, limit + 1)
        .await
    {
        Ok(ids) => ids,
        Err(err) => {
            tracing::error!(?err, "failed exact-match idea lookup");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let truncated = ids.len() > limit as usize;
    ids.truncate(limit as usize);
    let rows = match state
        .storage
        .list_ideas_by_ids(snapshot.block_height, &ids)
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to load exact-match ideas");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let mut matches = Vec::with_capacity(rows.len());
    for row in rows {
        match idea_summary(&row) {
            Ok(summary) => matches.push(summary),
            Err(response) => return response,
        }
    }
    let body = ExactMatchIdeasResponse {
        basis,
        field: field.to_string(),
        value: value.to_string(),
        matches,
        truncated,
        limit: limit.to_string(),
    };
    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}
pub(crate) async fn search_ideas(
    State(state): State<AppState>,
    Query(query): Query<SearchIdeasQuery>,
) -> Response {
    let q = match query.q.as_deref() {
        Some(q) if !q.trim().is_empty() => q.trim(),
        _ => return json_error(StatusCode::BAD_REQUEST, "invalid_request", "missing q"),
    };
    let limit = query
        .limit
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(50);
    let offset = query
        .offset
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(0);

    let limit = clamp_limit(limit);

    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };

    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };

    let total = match state
        .storage
        .count_search_ideas(snapshot.block_height, q)
        .await
    {
        Ok(total) => total,
        Err(err) => {
            tracing::error!(?err, "failed to count search ideas");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let rows = match state
        .storage
        .search_ideas(snapshot.block_height, q, offset, limit)
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to search ideas");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        match idea_summary(&row) {
            Ok(summary) => results.push(summary),
            Err(response) => return response,
        }
    }

    let body = SearchIdeasResponse {
        basis,
        results,
        total: total.to_string(),
    };

    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}

pub(crate) async fn relative_importance_connections(
    State(state): State<AppState>,
    Query(query): Query<RelativeImportanceConnectionsQuery>,
) -> Response {
    let idea_ids = match parse_bounded_uuid_v7_csv(query.idea_ids.as_deref(), "idea_ids", 200) {
        Ok(ids) => ids,
        Err(response) => return response,
    };
    let limit = query
        .limit
        .as_deref()
        .and_then(parse_non_negative_i64)
        .unwrap_or(200)
        .clamp(1, 200);
    let snapshot = match resolve_snapshot(&state.storage, query.snapshot_height.as_deref()).await {
        Ok(snapshot) => snapshot,
        Err(response) => return response,
    };
    let headers = match snapshot_headers(&snapshot) {
        Ok(headers) => headers,
        Err(response) => return response,
    };
    let basis = match snapshot_basis(&snapshot) {
        Ok(basis) => basis,
        Err(response) => return response,
    };

    let idea_id_set: BTreeSet<Uuid> = idea_ids.iter().copied().collect();
    let mut rows = match state
        .storage
        .list_connections_for_ideas_bounded(snapshot.block_height, &idea_ids, limit + 1)
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to load relative importance connections");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    rows.retain(|row| {
        row.connection_type == "relative_importance"
            && idea_id_set.contains(&row.from_idea_id)
            && idea_id_set.contains(&row.to_idea_id)
    });
    rows.sort_by_key(|row| (row.created_block_height, row.created_event_index));
    let truncated = rows.len() > limit as usize;
    rows.truncate(limit as usize);

    let body = RelativeImportanceConnectionsResponse {
        basis,
        connections: rows.iter().map(connection_summary).collect(),
        truncated,
        limit: limit.to_string(),
    };
    let response = (StatusCode::OK, Json(body)).into_response();
    with_headers(response, headers)
}
