use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use axum::{
    body::{to_bytes, Body},
    extract::DefaultBodyLimit,
    http::{Method, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use common::security_limits::{API_AUTH_BODY_LIMIT_BYTES, IDEA_TITLE_MAX_CHARS};
use common::test_db_guard::require_disposable_database_url;
use rand_core::OsRng;
use std::collections::BTreeSet;
use storage::{ensure_session_hmac_key_ready, ConnectionRow, Storage};
use tower::ServiceExt;
use uuid::Uuid;

use crate::server::helpers::{
    ensure_pathway_items, is_reference_scoped_connection, parse_private_vine_items,
    parse_relative_importance_direction, scoped_neighbor_from_reference, validate_max_len,
};
use crate::server::router::build_app;
use crate::server::types::{
    AppState, PrivateIdeaPayload, PrivateVineCreatePayload, PrivateVineItemPayload,
    PrivateVineUpdatePayload, RelativeImportanceDirection, VineTypeInput,
};
use serde_json::Value;

const MAX_DEBUG_BODY_CHARS: usize = 512;

#[derive(Debug)]
pub(super) struct ResponseSnapshot {
    pub status: StatusCode,
    pub json: Value,
    pub body_preview: String,
}

#[derive(Debug, Clone)]
pub(super) struct AuthSession {
    pub token: String,
    pub identity_id: Option<String>,
}

pub(super) async fn try_test_db() -> Option<Storage> {
    if let Err(err) = ensure_session_hmac_key_ready() {
        eprintln!("SKIP: failed to initialize session HMAC key: {err}");
        return None;
    }

    let database_url = match std::env::var("DATABASE_URL") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            eprintln!("SKIP: DATABASE_URL is missing");
            return None;
        }
    };
    match require_disposable_database_url(&database_url) {
        Ok(database_name) => {
            eprintln!("TEST_DB: {database_name} differs_from_seed_dev=true");
        }
        Err(err) => {
            eprintln!("SKIP: DATABASE_URL rejected by test DB guard: {err}");
            return None;
        }
    }

    match Storage::new(&database_url).await {
        Ok(storage) => Some(storage),
        Err(err) => {
            eprintln!("SKIP: DATABASE_URL unreachable: {err}");
            None
        }
    }
}

pub(super) async fn test_app_with_storage() -> Option<(Router, Storage)> {
    let storage = try_test_db().await?;
    let app = build_app(AppState {
        storage: storage.clone(),
    });
    Some((app, storage))
}

pub(super) async fn test_app() -> Option<Router> {
    let (app, _storage) = test_app_with_storage().await?;
    Some(app)
}

pub(super) async fn oneshot_json(
    app: Router,
    method: Method,
    uri: &str,
    body: Value,
    auth_header: Option<&str>,
) -> ResponseSnapshot {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json");
    if let Some(value) = auth_header {
        builder = builder.header("authorization", value);
    }
    let request = builder
        .body(Body::from(body.to_string()))
        .expect("request build");
    let response = app.oneshot(request).await.expect("oneshot response");
    snapshot_response(response).await
}

#[cfg(feature = "full")]
pub(super) fn status_is_unauthorized_or_forbidden(status: StatusCode) -> bool {
    status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN
}

pub(super) fn assert_error_code(snapshot: &ResponseSnapshot, expected: &str) {
    assert_eq!(
        snapshot
            .json
            .get("error_code")
            .and_then(|value| value.as_str()),
        Some(expected),
        "unexpected error_code; status={} preview={}",
        snapshot.status,
        snapshot.body_preview
    );
}

pub(super) async fn register_and_get_token() -> Option<String> {
    register_and_get_session()
        .await
        .map(|session| session.token)
}

pub(super) async fn register_and_get_session() -> Option<AuthSession> {
    let app = test_app().await?;
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_nanos();
    let username = format!("api_router_test_{nonce}");
    let password = "router-test-password-1234";
    let snapshot = oneshot_json(
        app,
        Method::POST,
        "/api/v0/auth/register",
        serde_json::json!({
            "username": username,
            "password": password
        }),
        None,
    )
    .await;

    if snapshot.status != StatusCode::OK {
        eprintln!(
            "[skip] api-server router tests: unable to register test user (status={})",
            snapshot.status
        );
        return None;
    }

    auth_session_from_snapshot(&snapshot).or_else(|| {
        eprintln!("[skip] api-server router tests: register response missing session fields");
        None
    })
}

pub(super) async fn create_canonical_user_via_db_and_login() -> Option<AuthSession> {
    let (app, storage) = test_app_with_storage().await?;

    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_nanos();
    let username = format!("canonical_writer_test_{nonce}");
    let password = format!("canonical-pass-{nonce}-Aa1!");
    let display_name = format!("Canonical Writer {nonce}");

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .ok()?
        .to_string();

    if let Err(err) = storage
        .create_account_with_canonical_cluster(&username, &password_hash, &display_name)
        .await
    {
        eprintln!("SKIP: unable to create canonical test account: {err}");
        return None;
    }

    let snapshot = oneshot_json(
        app,
        Method::POST,
        "/api/v0/auth/login",
        serde_json::json!({
            "username": username,
            "password": password
        }),
        None,
    )
    .await;

    if snapshot.status != StatusCode::OK {
        eprintln!(
            "SKIP: canonical test account login failed with status={}",
            snapshot.status
        );
        return None;
    }

    let session = auth_session_from_snapshot(&snapshot)?;
    if session.identity_id.is_none() {
        eprintln!("SKIP: canonical test account login missing identity_id");
        return None;
    }
    Some(session)
}

pub(super) async fn login_seed_owner_session() -> Option<AuthSession> {
    let username = match std::env::var("SEED_OWNER_USERNAME") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            eprintln!("SKIP: SEED_OWNER_USERNAME is missing");
            return None;
        }
    };
    let password = match std::env::var("SEED_OWNER_PASSWORD") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            eprintln!("SKIP: SEED_OWNER_PASSWORD is missing");
            return None;
        }
    };
    let app = test_app().await?;
    let snapshot = oneshot_json(
        app,
        Method::POST,
        "/api/v0/auth/login",
        serde_json::json!({
            "username": username,
            "password": password
        }),
        None,
    )
    .await;

    if snapshot.status != StatusCode::OK {
        eprintln!(
            "SKIP: seed owner login failed with status={} (owner bootstrap may be unavailable)",
            snapshot.status
        );
        return None;
    }

    auth_session_from_snapshot(&snapshot).or_else(|| {
        eprintln!("SKIP: seed owner login response missing session fields");
        None
    })
}

async fn snapshot_response(response: Response) -> ResponseSnapshot {
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("read response body");
    let body = String::from_utf8(bytes.to_vec()).expect("response body utf8");
    let body_preview = truncate_for_debug(&body);
    let json = serde_json::from_str::<Value>(&body).unwrap_or(Value::Null);

    ResponseSnapshot {
        status,
        json,
        body_preview,
    }
}

fn truncate_for_debug(value: &str) -> String {
    value.chars().take(MAX_DEBUG_BODY_CHARS).collect()
}

fn auth_session_from_snapshot(snapshot: &ResponseSnapshot) -> Option<AuthSession> {
    let token = snapshot.json.get("token")?.as_str()?.to_string();
    let identity_id = snapshot
        .json
        .get("identity_id")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string());
    Some(AuthSession { token, identity_id })
}

fn sample_connection_row(
    from_idea_id: Uuid,
    to_idea_id: Uuid,
    connection_type: &str,
) -> ConnectionRow {
    ConnectionRow {
        connection_id: Uuid::parse_str("00000000-0000-7000-8000-0000000000a1").unwrap(),
        from_idea_id,
        to_idea_id,
        connection_type: connection_type.to_string(),
        usage: None,
        axis: None,
        timeframe: None,
        scope: None,
        created_by_event_id: Uuid::parse_str("00000000-0000-7000-8000-0000000000b1").unwrap(),
        created_block_height: 1,
        created_event_index: 0,
    }
}

#[test]
fn private_vine_create_payload_deserializes() {
    let payload: PrivateVineCreatePayload = serde_json::from_str(
        r#"{
          "vine_type":"pathway_vine",
          "title":"Test",
          "items":[
            {"idea_id":"00000000-0000-7000-8000-000000000001"},
            {"idea_id":"00000000-0000-7000-8000-000000000002","via_connection_id":"00000000-0000-7000-8000-000000000003"}
          ]
        }"#,
    )
    .expect("create payload should deserialize");
    assert_eq!(payload.items.len(), 2);
    match payload.vine_type {
        VineTypeInput::String(value) => assert_eq!(value, "pathway_vine"),
        _ => panic!("expected string vine_type"),
    }
}

#[test]
fn private_vine_update_payload_deserializes_partial() {
    let payload: PrivateVineUpdatePayload = serde_json::from_str(
        r#"{
          "sentence":"Updated",
          "items":[{"idea_id":"00000000-0000-7000-8000-000000000010"}]
        }"#,
    )
    .expect("update payload should deserialize");
    assert!(payload.vine_type.is_none());
    assert!(payload.sentence.is_some());
    assert!(payload.items.is_some());
}

#[test]
fn private_vine_item_ordering_is_deterministic() {
    let items = vec![
        PrivateVineItemPayload {
            idea_id: "00000000-0000-7000-8000-000000000011".to_string(),
            via_connection_id: None,
        },
        PrivateVineItemPayload {
            idea_id: "00000000-0000-7000-8000-000000000012".to_string(),
            via_connection_id: Some("00000000-0000-7000-8000-000000000013".to_string()),
        },
    ];
    let parsed = parse_private_vine_items(&items).expect("items should parse");
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].idx, 0);
    assert_eq!(parsed[1].idx, 1);
    assert!(parsed[1].via_connection_id.is_some());
}

#[test]
fn pathway_vine_requires_non_empty_items() {
    assert!(ensure_pathway_items(0, 0).is_err());
    assert!(ensure_pathway_items(0, 1).is_ok());
    assert!(ensure_pathway_items(1, 0).is_ok());
}

#[test]
fn relative_importance_reference_membership_is_incoming_by_default() {
    let reference = Uuid::parse_str("00000000-0000-7000-8000-000000000101").unwrap();
    let neighbor = Uuid::parse_str("00000000-0000-7000-8000-000000000102").unwrap();

    let outgoing = sample_connection_row(reference, neighbor, "relative_importance");
    let incoming = sample_connection_row(neighbor, reference, "relative_importance");

    assert!(!is_reference_scoped_connection(
        reference,
        &outgoing,
        RelativeImportanceDirection::Incoming
    ));
    assert!(is_reference_scoped_connection(
        reference,
        &incoming,
        RelativeImportanceDirection::Incoming
    ));
    assert_eq!(
        scoped_neighbor_from_reference(reference, &outgoing, RelativeImportanceDirection::Incoming),
        None
    );
    assert_eq!(
        scoped_neighbor_from_reference(reference, &incoming, RelativeImportanceDirection::Incoming),
        Some(neighbor)
    );
}

#[test]
fn relative_importance_reference_membership_supports_outgoing_and_both() {
    let reference = Uuid::parse_str("00000000-0000-7000-8000-000000000111").unwrap();
    let neighbor = Uuid::parse_str("00000000-0000-7000-8000-000000000112").unwrap();

    let outgoing = sample_connection_row(reference, neighbor, "relative_importance");
    let incoming = sample_connection_row(neighbor, reference, "relative_importance");

    assert!(is_reference_scoped_connection(
        reference,
        &outgoing,
        RelativeImportanceDirection::Outgoing
    ));
    assert!(!is_reference_scoped_connection(
        reference,
        &incoming,
        RelativeImportanceDirection::Outgoing
    ));
    assert_eq!(
        scoped_neighbor_from_reference(reference, &outgoing, RelativeImportanceDirection::Outgoing),
        Some(neighbor)
    );
    assert_eq!(
        scoped_neighbor_from_reference(reference, &incoming, RelativeImportanceDirection::Outgoing),
        None
    );

    assert!(is_reference_scoped_connection(
        reference,
        &outgoing,
        RelativeImportanceDirection::Both
    ));
    assert!(is_reference_scoped_connection(
        reference,
        &incoming,
        RelativeImportanceDirection::Both
    ));
    assert_eq!(
        scoped_neighbor_from_reference(reference, &outgoing, RelativeImportanceDirection::Both),
        Some(neighbor)
    );
    assert_eq!(
        scoped_neighbor_from_reference(reference, &incoming, RelativeImportanceDirection::Both),
        Some(neighbor)
    );
}

#[test]
fn non_relative_connections_remain_bidirectional_for_reference_membership() {
    let reference = Uuid::parse_str("00000000-0000-7000-8000-000000000201").unwrap();
    let neighbor = Uuid::parse_str("00000000-0000-7000-8000-000000000202").unwrap();

    let outgoing = sample_connection_row(reference, neighbor, "membership");
    let incoming = sample_connection_row(neighbor, reference, "same_as");

    assert!(is_reference_scoped_connection(
        reference,
        &outgoing,
        RelativeImportanceDirection::Incoming
    ));
    assert!(is_reference_scoped_connection(
        reference,
        &incoming,
        RelativeImportanceDirection::Incoming
    ));
    assert_eq!(
        scoped_neighbor_from_reference(reference, &outgoing, RelativeImportanceDirection::Incoming),
        Some(neighbor)
    );
    assert_eq!(
        scoped_neighbor_from_reference(reference, &incoming, RelativeImportanceDirection::Incoming),
        Some(neighbor)
    );
}

#[test]
fn parse_relative_importance_direction_defaults_to_incoming() {
    assert_eq!(
        parse_relative_importance_direction(None).unwrap(),
        RelativeImportanceDirection::Incoming
    );
    assert_eq!(
        parse_relative_importance_direction(Some("")).unwrap(),
        RelativeImportanceDirection::Incoming
    );
    assert_eq!(
        parse_relative_importance_direction(Some("incoming")).unwrap(),
        RelativeImportanceDirection::Incoming
    );
    assert_eq!(
        parse_relative_importance_direction(Some("outgoing")).unwrap(),
        RelativeImportanceDirection::Outgoing
    );
    assert_eq!(
        parse_relative_importance_direction(Some("both")).unwrap(),
        RelativeImportanceDirection::Both
    );
    assert!(parse_relative_importance_direction(Some("sideways")).is_err());
}

async fn test_json_ok(Json(_payload): Json<serde_json::Value>) -> Response {
    (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response()
}

async fn test_private_title_validator(Json(payload): Json<PrivateIdeaPayload>) -> Response {
    if let Err(response) = validate_max_len("title", payload.title.as_str(), IDEA_TITLE_MAX_CHARS) {
        return response;
    }
    (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response()
}

#[tokio::test]
async fn oversized_body_is_rejected_with_413() {
    let app = Router::new()
        .route("/auth/register", post(test_json_ok))
        .layer(DefaultBodyLimit::max(API_AUTH_BODY_LIMIT_BYTES));

    let oversized_password = "x".repeat(API_AUTH_BODY_LIMIT_BYTES + 128);
    let body = serde_json::json!({
        "username": "user123",
        "password": oversized_password,
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn too_long_title_returns_stable_400_without_body_echo() {
    let app = Router::new().route("/private/ideas", post(test_private_title_validator));
    let long_title = "t".repeat(IDEA_TITLE_MAX_CHARS + 1);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/private/ideas")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "title": long_title,
                        "sentence": "short sentence",
                        "paragraph": null,
                        "full": null
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    let body = String::from_utf8(body.to_vec()).expect("utf8");
    assert!(body.contains("\"error_code\":\"invalid_field_length\""));
    assert!(!body.contains(&"t".repeat(32)));
}

#[tokio::test]
async fn coordinates_endpoint_is_deterministic_for_global_and_reference_modes() {
    let Some((app, storage)) = test_app_with_storage().await else {
        return;
    };

    let Some(snapshot) = storage.get_latest_snapshot().await.ok().flatten() else {
        eprintln!("SKIP: snapshot unavailable for coordinates endpoint test");
        return;
    };

    let idea_rows = match storage
        .list_ideas_top(snapshot.block_height, 0, 50, false)
        .await
    {
        Ok(rows) if !rows.is_empty() => rows,
        Ok(_) => {
            eprintln!("SKIP: no ideas available for coordinates endpoint test");
            return;
        }
        Err(err) => {
            eprintln!("SKIP: unable to query ideas for coordinates endpoint test: {err}");
            return;
        }
    };
    let mut reference_id = None;
    let mut expected_reference_ids = BTreeSet::new();
    for idea_row in &idea_rows {
        let mut connections = match storage
            .list_connections_for_idea(snapshot.block_height, idea_row.idea_id)
            .await
        {
            Ok(rows) => rows,
            Err(err) => {
                eprintln!("SKIP: unable to query coordinate reference connections: {err}");
                return;
            }
        };
        connections.retain(|row| {
            is_reference_scoped_connection(idea_row.idea_id, row, RelativeImportanceDirection::Both)
        });

        let mut expected_ids = BTreeSet::from([idea_row.idea_id.to_string()]);
        for row in &connections {
            if let Some(neighbor_id) = scoped_neighbor_from_reference(
                idea_row.idea_id,
                row,
                RelativeImportanceDirection::Both,
            ) {
                expected_ids.insert(neighbor_id.to_string());
            }
        }

        if expected_ids.len() > 1 {
            reference_id = Some(idea_row.idea_id.to_string());
            expected_reference_ids = expected_ids;
            break;
        }
    }

    let Some(reference_id) = reference_id else {
        eprintln!("SKIP: no connected idea available for coordinates endpoint test");
        return;
    };

    let global_uri = "/api/v0/coordinates";
    let reference_uri = format!("/api/v0/coordinates?reference_id={reference_id}");

    let global_first = oneshot_json(
        app.clone(),
        Method::GET,
        global_uri,
        serde_json::json!({}),
        None,
    )
    .await;
    let global_second = oneshot_json(
        app.clone(),
        Method::GET,
        global_uri,
        serde_json::json!({}),
        None,
    )
    .await;

    assert_eq!(global_first.status, StatusCode::OK);
    assert_eq!(global_first.json, global_second.json);
    assert_eq!(
        global_first
            .json
            .get("mode")
            .and_then(|value| value.as_str()),
        Some("global")
    );
    assert!(
        global_first
            .json
            .get("coords")
            .and_then(|value| value.as_array())
            .map(|coords| !coords.is_empty())
            .unwrap_or(false),
        "global coordinates response should contain at least one node"
    );

    let reference_first = oneshot_json(
        app.clone(),
        Method::GET,
        &reference_uri,
        serde_json::json!({}),
        None,
    )
    .await;
    let reference_second = oneshot_json(
        app,
        Method::GET,
        &reference_uri,
        serde_json::json!({}),
        None,
    )
    .await;

    assert_eq!(reference_first.status, StatusCode::OK);
    assert_eq!(reference_first.json, reference_second.json);
    assert_eq!(
        reference_first
            .json
            .get("mode")
            .and_then(|value| value.as_str()),
        Some("reference")
    );
    assert_eq!(
        reference_first
            .json
            .get("reference_id")
            .and_then(|value| value.as_str()),
        Some(reference_id.as_str())
    );
    assert_eq!(
        reference_first
            .json
            .pointer("/coords/0/id")
            .and_then(|value| value.as_str()),
        Some(reference_id.as_str())
    );
    assert_eq!(
        reference_first
            .json
            .pointer("/coords/0/x")
            .and_then(|value| value.as_f64()),
        Some(0.0)
    );
    assert_eq!(
        reference_first
            .json
            .pointer("/coords/0/y")
            .and_then(|value| value.as_f64()),
        Some(0.0)
    );
    let response_ids = reference_first
        .json
        .get("coords")
        .and_then(|value| value.as_array())
        .map(|coords| {
            coords
                .iter()
                .filter_map(|entry| entry.get("id").and_then(|value| value.as_str()))
                .map(str::to_string)
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    assert_eq!(response_ids, expected_reference_ids);
    assert!(
        reference_first
            .json
            .pointer("/meta/relaxed")
            .and_then(|value| value.as_bool())
            .is_some(),
        "reference coordinates response should expose meta.relaxed"
    );
}
