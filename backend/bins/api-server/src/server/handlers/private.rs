use api_types_private::{
    PrivateAiDraftResponse, PrivateAiSuggestion, PrivateIdeaResponse, PrivateIdeasResponse,
    PrivateVineResponse, PrivateVinesResponse,
};
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use common::security_limits::{
    AI_RAW_TEXT_MAX_CHARS, IDEA_FULL_MAX_CHARS, IDEA_PARAGRAPH_MAX_CHARS, IDEA_SENTENCE_MAX_CHARS,
    IDEA_TITLE_MAX_CHARS,
};

use crate::server::errors::json_error;
use crate::server::helpers::{
    ensure_pathway_items, normalize_optional_text, parse_private_vine_items, parse_uuid_any,
    parse_vine_type_input, validate_max_len, validate_optional_max_len,
};
use crate::server::mapping::{
    private_idea_detail, private_idea_summary, private_vine_detail, private_vine_summary,
};
use crate::server::types::{
    AppState, AuthenticatedAccount, PrivateAiDraftPayload, PrivateIdeaPayload,
    PrivateVineCreatePayload, PrivateVineUpdatePayload,
};

pub(crate) async fn private_create_idea(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<PrivateIdeaPayload>,
) -> Response {
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
    if payload.title.trim().is_empty() || payload.sentence.trim().is_empty() {
        return json_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "title and sentence required",
        );
    }
    let row = match state
        .storage
        .create_private_idea(
            auth.account_id,
            payload.title.trim(),
            payload.sentence.trim(),
            payload.paragraph.as_deref(),
            payload.full.as_deref(),
        )
        .await
    {
        Ok(row) => row,
        Err(err) => {
            tracing::error!(?err, "failed to create private idea");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let body = PrivateIdeaResponse {
        idea: private_idea_detail(&row),
    };
    (StatusCode::OK, Json(body)).into_response()
}

pub(crate) async fn private_list_ideas(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
) -> Response {
    let rows = match state.storage.list_private_ideas(auth.account_id).await {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to list private ideas");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let ideas = rows.iter().map(private_idea_summary).collect();
    let body = PrivateIdeasResponse { ideas };
    (StatusCode::OK, Json(body)).into_response()
}

pub(crate) async fn private_get_idea(
    Path(idea_id): Path<String>,
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
) -> Response {
    let idea_id = match parse_uuid_any(&idea_id) {
        Ok(idea_id) => idea_id,
        Err(response) => return response,
    };
    let row = match state
        .storage
        .get_private_idea(auth.account_id, idea_id)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load private idea");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let body = PrivateIdeaResponse {
        idea: private_idea_detail(&row),
    };
    (StatusCode::OK, Json(body)).into_response()
}

pub(crate) async fn private_update_idea(
    Path(idea_id): Path<String>,
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<PrivateIdeaPayload>,
) -> Response {
    let idea_id = match parse_uuid_any(&idea_id) {
        Ok(idea_id) => idea_id,
        Err(response) => return response,
    };
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
    if payload.title.trim().is_empty() || payload.sentence.trim().is_empty() {
        return json_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "title and sentence required",
        );
    }
    let row = match state
        .storage
        .update_private_idea(
            auth.account_id,
            idea_id,
            payload.title.trim(),
            payload.sentence.trim(),
            payload.paragraph.as_deref(),
            payload.full.as_deref(),
        )
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to update private idea");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let body = PrivateIdeaResponse {
        idea: private_idea_detail(&row),
    };
    (StatusCode::OK, Json(body)).into_response()
}

pub(crate) async fn private_delete_idea(
    Path(idea_id): Path<String>,
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
) -> Response {
    let idea_id = match parse_uuid_any(&idea_id) {
        Ok(idea_id) => idea_id,
        Err(response) => return response,
    };
    let deleted = match state
        .storage
        .delete_private_idea(auth.account_id, idea_id)
        .await
    {
        Ok(count) => count,
        Err(err) => {
            tracing::error!(?err, "failed to delete private idea");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    if deleted == 0 {
        return json_error(StatusCode::NOT_FOUND, "not_found", "not found");
    }
    (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response()
}

pub(crate) async fn private_list_vines(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
) -> Response {
    let rows = match state.storage.list_private_vines(auth.account_id).await {
        Ok(rows) => rows,
        Err(err) => {
            tracing::error!(?err, "failed to list private vines");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let body = PrivateVinesResponse {
        vines: rows.iter().map(private_vine_summary).collect(),
    };
    (StatusCode::OK, Json(body)).into_response()
}

pub(crate) async fn private_get_vine(
    Path(private_vine_id): Path<String>,
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
) -> Response {
    let private_vine_id = match parse_uuid_any(&private_vine_id) {
        Ok(private_vine_id) => private_vine_id,
        Err(response) => return response,
    };

    let row = match state
        .storage
        .get_private_vine(auth.account_id, private_vine_id)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load private vine");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let items = match state
        .storage
        .list_private_vine_items(auth.account_id, private_vine_id)
        .await
    {
        Ok(items) => items,
        Err(err) => {
            tracing::error!(?err, "failed to load private vine items");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let body = PrivateVineResponse {
        vine: private_vine_detail(&row, &items),
    };
    (StatusCode::OK, Json(body)).into_response()
}

pub(crate) async fn private_create_vine(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<PrivateVineCreatePayload>,
) -> Response {
    let vine_type = match parse_vine_type_input(&payload.vine_type) {
        Ok(vine_type) => vine_type,
        Err(response) => return response,
    };
    let items = match parse_private_vine_items(&payload.items) {
        Ok(items) => items,
        Err(response) => return response,
    };
    if let Err(response) = ensure_pathway_items(vine_type, items.len()) {
        return response;
    }
    if let Err(response) =
        validate_optional_max_len("title", payload.title.as_deref(), IDEA_TITLE_MAX_CHARS)
    {
        return response;
    }
    if let Err(response) = validate_optional_max_len(
        "sentence",
        payload.sentence.as_deref(),
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
    let title = normalize_optional_text(payload.title);
    let sentence = normalize_optional_text(payload.sentence);
    let paragraph = normalize_optional_text(payload.paragraph);
    let full = normalize_optional_text(payload.full);

    let row = match state
        .storage
        .create_private_vine(
            auth.account_id,
            vine_type,
            title.as_deref(),
            sentence.as_deref(),
            paragraph.as_deref(),
            full.as_deref(),
            &items,
        )
        .await
    {
        Ok(row) => row,
        Err(err) => {
            tracing::error!(?err, "failed to create private vine");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let detail_items = match state
        .storage
        .list_private_vine_items(auth.account_id, row.private_vine_id)
        .await
    {
        Ok(items) => items,
        Err(err) => {
            tracing::error!(?err, "failed to load created private vine items");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let body = PrivateVineResponse {
        vine: private_vine_detail(&row, &detail_items),
    };
    (StatusCode::OK, Json(body)).into_response()
}

pub(crate) async fn private_update_vine(
    Path(private_vine_id): Path<String>,
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<PrivateVineUpdatePayload>,
) -> Response {
    let private_vine_id = match parse_uuid_any(&private_vine_id) {
        Ok(private_vine_id) => private_vine_id,
        Err(response) => return response,
    };

    let existing = match state
        .storage
        .get_private_vine(auth.account_id, private_vine_id)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to load private vine before update");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let vine_type = match payload.vine_type.as_ref() {
        Some(value) => match parse_vine_type_input(value) {
            Ok(value) => value,
            Err(response) => return response,
        },
        None => existing.vine_type,
    };

    let title = match payload.title {
        Some(value) => normalize_optional_text(value),
        None => existing.title.clone(),
    };
    let sentence = match payload.sentence {
        Some(value) => normalize_optional_text(value),
        None => existing.sentence.clone(),
    };
    let paragraph = match payload.paragraph {
        Some(value) => normalize_optional_text(value),
        None => existing.paragraph.clone(),
    };
    let full = match payload.full {
        Some(value) => normalize_optional_text(value),
        None => existing.full.clone(),
    };
    if let Err(response) =
        validate_optional_max_len("title", title.as_deref(), IDEA_TITLE_MAX_CHARS)
    {
        return response;
    }
    if let Err(response) =
        validate_optional_max_len("sentence", sentence.as_deref(), IDEA_SENTENCE_MAX_CHARS)
    {
        return response;
    }
    if let Err(response) =
        validate_optional_max_len("paragraph", paragraph.as_deref(), IDEA_PARAGRAPH_MAX_CHARS)
    {
        return response;
    }
    if let Err(response) = validate_optional_max_len("full", full.as_deref(), IDEA_FULL_MAX_CHARS) {
        return response;
    }

    let replacement_items = match payload.items.as_ref() {
        Some(items) => match parse_private_vine_items(items) {
            Ok(items) => Some(items),
            Err(response) => return response,
        },
        None => None,
    };
    let effective_item_count = if let Some(items) = replacement_items.as_ref() {
        items.len()
    } else {
        match state
            .storage
            .list_private_vine_items(auth.account_id, private_vine_id)
            .await
        {
            Ok(items) => items.len(),
            Err(err) => {
                tracing::error!(?err, "failed to load existing private vine items");
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                );
            }
        }
    };
    if let Err(response) = ensure_pathway_items(vine_type, effective_item_count) {
        return response;
    }

    let row = match state
        .storage
        .update_private_vine(
            auth.account_id,
            private_vine_id,
            vine_type,
            title.as_deref(),
            sentence.as_deref(),
            paragraph.as_deref(),
            full.as_deref(),
            replacement_items.as_deref(),
        )
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        Err(err) => {
            tracing::error!(?err, "failed to update private vine");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    let detail_items = match state
        .storage
        .list_private_vine_items(auth.account_id, private_vine_id)
        .await
    {
        Ok(items) => items,
        Err(err) => {
            tracing::error!(?err, "failed to load updated private vine items");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let body = PrivateVineResponse {
        vine: private_vine_detail(&row, &detail_items),
    };
    (StatusCode::OK, Json(body)).into_response()
}

pub(crate) async fn private_delete_vine(
    Path(private_vine_id): Path<String>,
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
) -> Response {
    let private_vine_id = match parse_uuid_any(&private_vine_id) {
        Ok(private_vine_id) => private_vine_id,
        Err(response) => return response,
    };
    let deleted = match state
        .storage
        .delete_private_vine(auth.account_id, private_vine_id)
        .await
    {
        Ok(deleted) => deleted,
        Err(err) => {
            tracing::error!(?err, "failed to delete private vine");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };
    if deleted == 0 {
        return json_error(StatusCode::NOT_FOUND, "not_found", "not found");
    }
    (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response()
}

pub(crate) async fn private_ai_draft(
    State(_state): State<AppState>,
    Extension(auth): Extension<AuthenticatedAccount>,
    Json(payload): Json<PrivateAiDraftPayload>,
) -> Response {
    if let Err(response) =
        validate_max_len("raw_text", payload.raw_text.as_str(), AI_RAW_TEXT_MAX_CHARS)
    {
        return response;
    }
    if let Some(context) = payload.context.as_ref() {
        if let Err(response) = validate_optional_max_len(
            "context.title",
            context.title.as_deref(),
            IDEA_TITLE_MAX_CHARS,
        ) {
            return response;
        }
        if let Err(response) = validate_optional_max_len(
            "context.sentence",
            context.sentence.as_deref(),
            IDEA_SENTENCE_MAX_CHARS,
        ) {
            return response;
        }
        if let Err(response) = validate_optional_max_len(
            "context.paragraph",
            context.paragraph.as_deref(),
            IDEA_PARAGRAPH_MAX_CHARS,
        ) {
            return response;
        }
        if let Err(response) =
            validate_optional_max_len("context.full", context.full.as_deref(), IDEA_FULL_MAX_CHARS)
        {
            return response;
        }
    }
    let base = payload.raw_text.trim();
    let title = payload
        .context
        .as_ref()
        .and_then(|ctx| ctx.title.clone())
        .unwrap_or_else(|| format!("Draft: {}", base.chars().take(48).collect::<String>()));
    let sentence = payload
        .context
        .as_ref()
        .and_then(|ctx| ctx.sentence.clone())
        .unwrap_or_else(|| {
            format!(
                "Stub summary for {}",
                base.chars().take(80).collect::<String>()
            )
        });

    let suggestion = PrivateAiSuggestion {
        title,
        sentence,
        paragraph: payload
            .context
            .as_ref()
            .and_then(|ctx| ctx.paragraph.clone()),
        full: payload.context.as_ref().and_then(|ctx| ctx.full.clone()),
        confidence: Some(0.1),
        source: format!("stub:{}", auth.username),
    };

    let body = PrivateAiDraftResponse {
        suggestions: vec![suggestion],
    };
    (StatusCode::OK, Json(body)).into_response()
}
