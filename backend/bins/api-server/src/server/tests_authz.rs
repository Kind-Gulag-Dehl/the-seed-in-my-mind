use axum::http::{Method, StatusCode};

use super::tests::{
    assert_error_code, oneshot_json, status_is_unauthorized_or_forbidden, test_app,
};

#[tokio::test]
async fn auth_me_requires_authorization_header() {
    let Some(app) = test_app().await else {
        return;
    };

    let snapshot = oneshot_json(
        app,
        Method::GET,
        "/api/v0/auth/me",
        serde_json::json!({}),
        None,
    )
    .await;

    assert!(
        status_is_unauthorized_or_forbidden(snapshot.status),
        "unexpected status {}",
        snapshot.status
    );
    assert_error_code(&snapshot, "unauthorized");
}

#[tokio::test]
async fn private_ideas_requires_authorization_header() {
    let Some(app) = test_app().await else {
        return;
    };

    let snapshot = oneshot_json(
        app,
        Method::POST,
        "/api/v0/private/ideas",
        serde_json::json!({
            "title": "x",
            "sentence": "y",
            "paragraph": null,
            "full": null
        }),
        None,
    )
    .await;

    assert!(
        status_is_unauthorized_or_forbidden(snapshot.status),
        "unexpected status {}",
        snapshot.status
    );
    assert_error_code(&snapshot, "unauthorized");
}

#[tokio::test]
async fn canonical_ideas_requires_authorization_header() {
    let Some(app) = test_app().await else {
        return;
    };

    let snapshot = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/ideas",
        serde_json::json!({
            "idea_type": "conceptual_idea",
            "title": "x",
            "sentence": "y",
            "paragraph": null,
            "full": null
        }),
        None,
    )
    .await;

    assert!(
        status_is_unauthorized_or_forbidden(snapshot.status),
        "unexpected status {}",
        snapshot.status
    );
    assert_error_code(&snapshot, "unauthorized");
}

#[tokio::test]
async fn malformed_bearer_without_token_is_rejected() {
    let Some(app) = test_app().await else {
        return;
    };

    let snapshot = oneshot_json(
        app,
        Method::GET,
        "/api/v0/auth/me",
        serde_json::json!({}),
        Some("Bearer"),
    )
    .await;

    assert!(
        status_is_unauthorized_or_forbidden(snapshot.status),
        "unexpected status {}",
        snapshot.status
    );
    assert_error_code(&snapshot, "unauthorized");
}

#[tokio::test]
async fn malformed_bearer_invalid_token_is_rejected_without_echo() {
    let Some(app) = test_app().await else {
        return;
    };

    let malformed_token = "nothex";
    let snapshot = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/ideas",
        serde_json::json!({
            "idea_type": "conceptual_idea",
            "title": "x",
            "sentence": "y",
            "paragraph": null,
            "full": null
        }),
        Some("Bearer nothex"),
    )
    .await;

    assert!(
        status_is_unauthorized_or_forbidden(snapshot.status),
        "unexpected status {}",
        snapshot.status
    );
    assert_error_code(&snapshot, "unauthorized");
    assert!(
        !snapshot.body_preview.contains(malformed_token),
        "response preview should not echo bearer token fragment"
    );
    assert_ne!(snapshot.status, StatusCode::INTERNAL_SERVER_ERROR);
}
