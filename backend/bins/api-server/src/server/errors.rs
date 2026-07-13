use api_types_canonical::ApiError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use storage::CanonicalWriteError;

pub(crate) fn json_error(status: StatusCode, error_code: &str, message: &str) -> Response {
    (
        status,
        Json(ApiError {
            error_code: error_code.to_string(),
            message: message.to_string(),
        }),
    )
        .into_response()
}

pub(crate) fn snapshot_unavailable() -> Response {
    json_error(StatusCode::NOT_FOUND, "not_found", "not found")
}

pub(crate) fn canonical_write_error_response(err: CanonicalWriteError) -> Response {
    match err.code {
        "invalid_request"
        | "invalid_field"
        | "invalid_payload"
        | "invalid_payload_hash"
        | "invalid_id"
        | "invalid_hash"
        | "canonical_encoding_failed"
        | "malformed_signature"
        | "malformed_public_key"
        | "malformed_public_key_ref"
        | "unsupported_signature_profile"
        | "unsupported_payload_binding"
        | "invalid_payload_binding_mode"
        | "unsupported_event_type"
        | "secret_detected" => json_error(StatusCode::BAD_REQUEST, err.code, &err.message),
        "forbidden" => json_error(StatusCode::FORBIDDEN, err.code, &err.message),
        "unknown_key" | "revoked_key" | "wrong_key_owner" | "invalid_signature" => {
            json_error(StatusCode::FORBIDDEN, err.code, &err.message)
        }
        "insufficient_mana" | "conflict" | "duplicate_event" => {
            json_error(StatusCode::CONFLICT, err.code, &err.message)
        }
        _ => {
            tracing::error!(?err, "canonical write failed");
            json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            )
        }
    }
}

pub(crate) async fn normalize_error_response(response: Response) -> Response {
    match response.status() {
        StatusCode::NOT_FOUND => json_error(StatusCode::NOT_FOUND, "not_found", "not found"),
        StatusCode::PAYLOAD_TOO_LARGE => json_error(
            StatusCode::PAYLOAD_TOO_LARGE,
            "payload_too_large",
            "payload too large",
        ),
        StatusCode::METHOD_NOT_ALLOWED => json_error(
            StatusCode::METHOD_NOT_ALLOWED,
            "method_not_allowed",
            "method not allowed",
        ),
        _ => response,
    }
}
