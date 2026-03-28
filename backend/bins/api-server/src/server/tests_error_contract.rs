use axum::http::{Method, StatusCode};
use common::security_limits::IDEA_TITLE_MAX_CHARS;

use super::tests::{assert_error_code, oneshot_json, register_and_get_token, test_app};

#[tokio::test]
async fn invalid_field_length_contract_is_stable_and_no_echo() {
    let Some(token) = register_and_get_token().await else {
        return;
    };
    let Some(app) = test_app().await else {
        return;
    };

    let long_title = "x".repeat(IDEA_TITLE_MAX_CHARS + 1);
    let leaked_fragment = "x".repeat(64);
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
        !snapshot.body_preview.contains(&leaked_fragment),
        "response preview should not echo overlong title fragment"
    );
}

#[tokio::test]
async fn canonical_secret_detected_contract_is_stable_and_no_echo() {
    let Some(token) = register_and_get_token().await else {
        return;
    };
    let Some(app) = test_app().await else {
        return;
    };

    let marker = "-----BEGIN PRIVATE KEY-----";
    let snapshot = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/ideas",
        serde_json::json!({
            "idea_type": "conceptual_idea",
            "title": format!("synthetic-marker {marker}"),
            "sentence": "safe sentence",
            "paragraph": null,
            "full": null
        }),
        Some(&format!("Bearer {token}")),
    )
    .await;

    assert_eq!(snapshot.status, StatusCode::BAD_REQUEST);
    assert_error_code(&snapshot, "secret_detected");
    assert!(
        !snapshot.body_preview.contains(marker),
        "response preview should not echo secret marker"
    );
}
