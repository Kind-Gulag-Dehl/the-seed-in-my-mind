use axum::http::{Method, StatusCode};
use common::security_limits::{
    API_AUTH_BODY_LIMIT_BYTES, API_CANONICAL_BODY_LIMIT_BYTES, API_PRIVATE_AI_BODY_LIMIT_BYTES,
    IDEA_TITLE_MAX_CHARS,
};

use super::tests::{assert_error_code, oneshot_json, register_and_get_token, test_app};

#[tokio::test]
async fn auth_login_rejects_oversized_body_with_413() {
    let Some(app) = test_app().await else {
        return;
    };

    let oversized_password = "x".repeat(API_AUTH_BODY_LIMIT_BYTES + 128);
    let snapshot = oneshot_json(
        app,
        Method::POST,
        "/api/v0/auth/login",
        serde_json::json!({
            "username": "user123",
            "password": oversized_password
        }),
        None,
    )
    .await;

    assert_eq!(snapshot.status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_error_code(&snapshot, "payload_too_large");
}

#[tokio::test]
async fn private_ai_draft_rejects_oversized_body_with_413() {
    let Some(token) = register_and_get_token().await else {
        return;
    };
    let Some(app) = test_app().await else {
        return;
    };

    let oversized_raw_text = "x".repeat(API_PRIVATE_AI_BODY_LIMIT_BYTES + 128);
    let snapshot = oneshot_json(
        app,
        Method::POST,
        "/api/v0/private/ai/draft",
        serde_json::json!({
            "raw_text": oversized_raw_text,
            "context": null
        }),
        Some(&format!("Bearer {token}")),
    )
    .await;

    assert_eq!(snapshot.status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_error_code(&snapshot, "payload_too_large");
}

#[tokio::test]
async fn canonical_ideas_rejects_oversized_body_with_413() {
    let Some(token) = register_and_get_token().await else {
        return;
    };
    let Some(app) = test_app().await else {
        return;
    };

    let oversized_title = "x".repeat(API_CANONICAL_BODY_LIMIT_BYTES + 128);
    let snapshot = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/ideas",
        serde_json::json!({
            "idea_type": "conceptual_idea",
            "title": oversized_title,
            "sentence": "short",
            "paragraph": null,
            "full": null
        }),
        Some(&format!("Bearer {token}")),
    )
    .await;

    assert_eq!(snapshot.status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_error_code(&snapshot, "payload_too_large");
}

#[tokio::test]
async fn private_ideas_overlong_title_returns_invalid_field_length_without_echo() {
    let Some(token) = register_and_get_token().await else {
        return;
    };
    let Some(app) = test_app().await else {
        return;
    };

    let long_title = "t".repeat(IDEA_TITLE_MAX_CHARS + 1);
    let title_fragment = "t".repeat(64);
    let snapshot = oneshot_json(
        app,
        Method::POST,
        "/api/v0/private/ideas",
        serde_json::json!({
            "title": long_title,
            "sentence": "short sentence",
            "paragraph": null,
            "full": null
        }),
        Some(&format!("Bearer {token}")),
    )
    .await;

    assert_eq!(snapshot.status, StatusCode::BAD_REQUEST);
    assert_error_code(&snapshot, "invalid_field_length");
    assert!(
        !snapshot.body_preview.contains(&title_fragment),
        "response preview should not echo long title fragment"
    );
}
