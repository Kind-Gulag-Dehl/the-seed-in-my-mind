use axum::http::{Method, StatusCode};

use super::tests::{
    assert_error_code, create_canonical_user_via_db_and_login, login_seed_owner_session,
    oneshot_json, test_app,
};

#[tokio::test]
async fn canonical_writer_role_transition_denied_then_allowed_then_denied() {
    let Some(writer_session) = create_canonical_user_via_db_and_login().await else {
        return;
    };
    let Some(writer_identity_id) = writer_session.identity_id.clone() else {
        eprintln!("SKIP: writer session is missing identity_id");
        return;
    };

    let idea_title_nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .map(|value| value.as_nanos())
        .unwrap_or(0);

    let Some(app) = test_app().await else {
        return;
    };
    let denied_before_grant = oneshot_json(
        app.clone(),
        Method::POST,
        "/api/v1/canonical/ideas",
        serde_json::json!({
            "idea_type": "conceptual_idea",
            "title": format!("Role transition idea {idea_title_nonce}"),
            "sentence": "role transition pre-grant",
            "paragraph": null,
            "full": null
        }),
        Some(&format!("Bearer {}", writer_session.token)),
    )
    .await;

    assert_eq!(
        denied_before_grant.status,
        StatusCode::FORBIDDEN,
        "expected canonical write denial before grant"
    );
    assert_error_code(&denied_before_grant, "forbidden");

    let Some(owner_session) = login_seed_owner_session().await else {
        return;
    };

    let grant = oneshot_json(
        app.clone(),
        Method::POST,
        "/api/v1/canonical/verifier/grants",
        serde_json::json!({
            "identity_id": writer_identity_id,
            "canonical_writer_level": "1",
            "email_verified": true
        }),
        Some(&format!("Bearer {}", owner_session.token)),
    )
    .await;
    if grant.status != StatusCode::OK {
        eprintln!(
            "SKIP: seed owner token cannot grant writer in this environment (status={})",
            grant.status
        );
        return;
    }

    let allowed_after_grant = oneshot_json(
        app.clone(),
        Method::POST,
        "/api/v1/canonical/ideas",
        serde_json::json!({
            "idea_type": "conceptual_idea",
            "title": format!("Role transition idea granted {idea_title_nonce}"),
            "sentence": "role transition granted",
            "paragraph": null,
            "full": null
        }),
        Some(&format!("Bearer {}", writer_session.token)),
    )
    .await;

    assert!(
        allowed_after_grant.status == StatusCode::OK
            || allowed_after_grant.status == StatusCode::CREATED,
        "expected canonical write success after grant; got {}",
        allowed_after_grant.status
    );

    let revoke = oneshot_json(
        app.clone(),
        Method::POST,
        "/api/v1/canonical/verifier/revokes",
        serde_json::json!({
            "identity_id": writer_identity_id
        }),
        Some(&format!("Bearer {}", owner_session.token)),
    )
    .await;
    if revoke.status != StatusCode::OK {
        eprintln!(
            "SKIP: seed owner token cannot revoke writer in this environment (status={})",
            revoke.status
        );
        return;
    }

    let denied_after_revoke = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/ideas",
        serde_json::json!({
            "idea_type": "conceptual_idea",
            "title": format!("Role transition idea revoked {idea_title_nonce}"),
            "sentence": "role transition revoked",
            "paragraph": null,
            "full": null
        }),
        Some(&format!("Bearer {}", writer_session.token)),
    )
    .await;

    assert_eq!(
        denied_after_revoke.status,
        StatusCode::FORBIDDEN,
        "expected canonical write denial after revoke"
    );
    assert_error_code(&denied_after_revoke, "forbidden");
}
