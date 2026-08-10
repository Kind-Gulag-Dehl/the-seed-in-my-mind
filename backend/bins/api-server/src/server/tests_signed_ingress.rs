use axum::{
    body::{to_bytes, Body},
    http::{Method, Request, StatusCode},
};
use ed25519_dalek::{Signer, SigningKey};
use encoding::payload::{canonical_json_payload_hash_hex, payload_hash_hex};
use replay::ReplayDriver;
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::borrow::Cow;
use std::path::Path;
use storage::Storage;
use tower::ServiceExt;
use uuid::Uuid;
use verification::signatures::{
    public_key_ref_v0, signed_candidate_bytes_v0, AuthoredEventCandidate, PAYLOAD_BINDING_EMBEDDED,
    SIGNATURE_PROFILE_ED25519_V0,
};

use crate::server::router::build_app;
use crate::server::tests::{assert_error_code, oneshot_json};
use crate::server::types::AppState;

const TEST_DB_ADMIN_ENV: &str = "SEED_TEST_DATABASE_ADMIN_URL";
const PUBLICATION_PROFILE: &str = "profile_0_bootstrap_single_publisher";
const SIGNED_SUBSTRATE_MIGRATION_VERSION: i64 = 22;
const AUTHOR_ID: &str = "00000000-0000-7000-8000-00000000a001";
const BOOTSTRAP_VERIFIER_ID: &str = "380b7817-db3b-7b76-8cf3-87df879ddddb";
const BOOTSTRAP_IDENTITY_EVENT_ID: &str = "00000000-0000-7000-8000-000000000001";
const BOOTSTRAP_VERIFIER_EVENT_ID: &str = "00000000-0000-7000-8000-000000000003";
const BOOTSTRAP_WRITER_EVENT_ID: &str = "00000000-0000-7000-8000-000000000002";

#[tokio::test]
async fn signed_http_ingress_accepts_retries_replays_and_reads_back() {
    let Some(db) = IsolatedSignedIngressDb::create().await else {
        return;
    };
    let result = run_signed_ingress_acceptance(&db).await;
    db.cleanup().await;
    result.expect("signed ingress acceptance");
}

#[tokio::test]
async fn signed_http_ingress_rejects_invalid_candidates_atomically() {
    let Some(db) = IsolatedSignedIngressDb::create().await else {
        return;
    };
    let result = run_signed_ingress_rejections(&db).await;
    db.cleanup().await;
    result.expect("signed ingress rejections");
}

#[tokio::test]
async fn signed_ingress_migration_preserves_pre_0022_legacy_rows() {
    let Some(db) = IsolatedSignedIngressDb::create_legacy_upgraded().await else {
        return;
    };
    let result = run_legacy_migration_compatibility(&db).await;
    db.cleanup().await;
    result.expect("legacy migration compatibility");
}

#[derive(Clone, Copy)]
enum MigrationMode {
    Latest,
    LegacyThenSigned,
}

struct IsolatedSignedIngressDb {
    database_name: String,
    database_url: String,
    admin_pool: PgPool,
    storage: Storage,
}

impl IsolatedSignedIngressDb {
    async fn create() -> Option<Self> {
        Self::create_with_migration_mode(MigrationMode::Latest).await
    }

    async fn create_legacy_upgraded() -> Option<Self> {
        Self::create_with_migration_mode(MigrationMode::LegacyThenSigned).await
    }

    async fn create_with_migration_mode(migration_mode: MigrationMode) -> Option<Self> {
        let admin_url = match std::env::var(TEST_DB_ADMIN_ENV) {
            Ok(value) if !value.trim().is_empty() => value,
            _ => {
                eprintln!("SKIP: {TEST_DB_ADMIN_ENV} is missing; signed ingress DB tests require an explicit disposable Postgres admin URL");
                return None;
            }
        };
        let database_name = unique_test_database_name();
        let Some(database_url) = database_url_for(&admin_url, &database_name) else {
            eprintln!("SKIP: unable to derive isolated database URL from {TEST_DB_ADMIN_ENV}");
            return None;
        };
        let admin_pool = match PgPoolOptions::new()
            .max_connections(1)
            .connect(&admin_url)
            .await
        {
            Ok(pool) => pool,
            Err(err) => {
                eprintln!("SKIP: unable to connect to {TEST_DB_ADMIN_ENV}: {err}");
                return None;
            }
        };
        let create_sql = format!("CREATE DATABASE {}", quote_ident(&database_name));
        if let Err(err) = sqlx::query(&create_sql).execute(&admin_pool).await {
            eprintln!("SKIP: unable to create isolated test database {database_name}: {err}");
            admin_pool.close().await;
            return None;
        }
        eprintln!("ISOLATED_DB: {database_name} differs_from_seed_dev=true");

        let migration_pool = match PgPoolOptions::new()
            .max_connections(2)
            .connect(&database_url)
            .await
        {
            Ok(pool) => pool,
            Err(err) => {
                eprintln!("SKIP: unable to connect to isolated database {database_name}: {err}");
                drop_database(&admin_pool, &database_name).await;
                admin_pool.close().await;
                return None;
            }
        };
        let full_migrator = match load_signed_ingress_migrator().await {
            Ok(migrator) => migrator,
            Err(err) => {
                eprintln!(
                    "SKIP: unable to load migrations for isolated database {database_name}: {err}"
                );
                migration_pool.close().await;
                drop_database(&admin_pool, &database_name).await;
                admin_pool.close().await;
                return None;
            }
        };
        match migration_mode {
            MigrationMode::Latest => {
                if let Err(err) = full_migrator.run(&migration_pool).await {
                    eprintln!(
                        "SKIP: unable to apply migrations in isolated database {database_name}: {err}"
                    );
                    migration_pool.close().await;
                    drop_database(&admin_pool, &database_name).await;
                    admin_pool.close().await;
                    return None;
                }
            }
            MigrationMode::LegacyThenSigned => {
                let pre_signed_migrator = migrator_before_signed_substrate(&full_migrator);
                if let Err(err) = pre_signed_migrator.run(&migration_pool).await {
                    eprintln!(
                        "SKIP: unable to apply pre-0022 migrations in isolated database {database_name}: {err}"
                    );
                    migration_pool.close().await;
                    drop_database(&admin_pool, &database_name).await;
                    admin_pool.close().await;
                    return None;
                }
                if let Err(err) = insert_pre_0022_legacy_rows(&migration_pool).await {
                    eprintln!(
                        "SKIP: unable to insert pre-0022 legacy rows in isolated database {database_name}: {err}"
                    );
                    migration_pool.close().await;
                    drop_database(&admin_pool, &database_name).await;
                    admin_pool.close().await;
                    return None;
                }
                if let Err(err) = full_migrator.run(&migration_pool).await {
                    eprintln!(
                        "SKIP: unable to apply 0022 migration over legacy rows in isolated database {database_name}: {err}"
                    );
                    migration_pool.close().await;
                    drop_database(&admin_pool, &database_name).await;
                    admin_pool.close().await;
                    return None;
                }
            }
        }
        migration_pool.close().await;

        let storage = match Storage::new(&database_url).await {
            Ok(storage) => storage,
            Err(err) => {
                eprintln!("SKIP: unable to open isolated storage {database_name}: {err}");
                drop_database(&admin_pool, &database_name).await;
                admin_pool.close().await;
                return None;
            }
        };
        if matches!(migration_mode, MigrationMode::Latest) {
            if let Err(err) = bootstrap_signed_ingress_identity(&storage).await {
                eprintln!("SKIP: unable to bootstrap isolated signed-ingress identity: {err}");
                storage.pool().close().await;
                drop_database(&admin_pool, &database_name).await;
                admin_pool.close().await;
                return None;
            }
        }
        Some(Self {
            database_name,
            database_url,
            admin_pool,
            storage,
        })
    }

    fn app(&self) -> axum::Router {
        build_app(AppState {
            storage: self.storage.clone(),
        })
    }

    async fn cleanup(self) {
        self.storage.pool().close().await;
        drop_database(&self.admin_pool, &self.database_name).await;
        self.admin_pool.close().await;
        let _ = self.database_url;
    }
}

async fn load_signed_ingress_migrator(
) -> Result<sqlx::migrate::Migrator, sqlx::migrate::MigrateError> {
    let migration_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations/postgres");
    sqlx::migrate::Migrator::new(migration_dir).await
}

fn migrator_before_signed_substrate(full: &sqlx::migrate::Migrator) -> sqlx::migrate::Migrator {
    sqlx::migrate::Migrator {
        migrations: Cow::Owned(
            full.iter()
                .filter(|migration| migration.version < SIGNED_SUBSTRATE_MIGRATION_VERSION)
                .cloned()
                .collect(),
        ),
        ignore_missing: false,
        locking: true,
        no_tx: false,
    }
}

async fn run_signed_ingress_acceptance(
    db: &IsolatedSignedIngressDb,
) -> Result<(), Box<dyn std::error::Error>> {
    let signing_key = test_signing_key();
    let idea_one_id = uuid("00000000-0000-7000-8000-00000000b101");
    let idea_two_id = uuid("00000000-0000-7000-8000-00000000b102");
    let connection_id = uuid("00000000-0000-7000-8000-00000000c101");
    let idea_one = signed_idea_request(
        &signing_key,
        "00000000-0000-7000-8000-000000000101",
        idea_one_id,
        "Signed route idea one",
    );
    let idea_two = signed_idea_request(
        &signing_key,
        "00000000-0000-7000-8000-000000000102",
        idea_two_id,
        "Signed route idea two",
    );

    let first = oneshot_json(
        db.app(),
        Method::POST,
        "/api/v1/canonical/events",
        idea_one.clone(),
        None,
    )
    .await;
    assert_eq!(first.status, StatusCode::OK, "{}", first.body_preview);
    assert_eq!(
        first
            .json
            .pointer("/event/publication_profile")
            .and_then(Value::as_str),
        Some(PUBLICATION_PROFILE)
    );
    assert_eq!(
        first
            .json
            .pointer("/object/object_type")
            .and_then(Value::as_str),
        Some("idea")
    );

    let retry = oneshot_json(
        db.app(),
        Method::POST,
        "/api/v1/canonical/events",
        idea_one.clone(),
        None,
    )
    .await;
    assert_eq!(retry.status, StatusCode::OK, "{}", retry.body_preview);
    assert_eq!(
        retry
            .json
            .pointer("/event/idempotent")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        count_rows(
            db.storage.pool(),
            "events",
            Some("event_id = '00000000-0000-7000-8000-000000000101'")
        )
        .await?,
        1
    );
    assert_eq!(
        count_rows(
            db.storage.pool(),
            "ideas",
            Some("idea_id = '00000000-0000-7000-8000-00000000b101'")
        )
        .await?,
        1
    );

    let with_private_bearer = oneshot_json(
        db.app(),
        Method::POST,
        "/api/v1/canonical/events",
        idea_two,
        Some("Bearer private-session-does-not-matter"),
    )
    .await;
    assert_eq!(
        with_private_bearer.status,
        StatusCode::OK,
        "{}",
        with_private_bearer.body_preview
    );

    let connection = signed_connection_request(
        &signing_key,
        "00000000-0000-7000-8000-000000000103",
        connection_id,
        idea_one_id,
        idea_two_id,
    );
    let connection_response = oneshot_json(
        db.app(),
        Method::POST,
        "/api/v1/canonical/events",
        connection,
        None,
    )
    .await;
    assert_eq!(
        connection_response.status,
        StatusCode::OK,
        "{}",
        connection_response.body_preview
    );
    assert_eq!(
        connection_response
            .json
            .pointer("/object/object_type")
            .and_then(Value::as_str),
        Some("connection")
    );

    assert_exact_candidate_reconstructs(db.storage.pool(), "00000000-0000-7000-8000-000000000101")
        .await?;
    assert_exact_candidate_reconstructs(db.storage.pool(), "00000000-0000-7000-8000-000000000103")
        .await?;
    assert_rebuilt_signed_projection_matches_materialized(db.storage.pool()).await?;

    let representation_id = uuid("00000000-0000-7000-8000-00000000d101");
    let ordering_id = uuid("00000000-0000-7000-8000-00000000d102");
    insert_test_representation(db.storage.pool(), idea_one_id, representation_id).await?;
    insert_test_ordering(db.storage.pool(), idea_one_id, idea_two_id, ordering_id).await?;

    let replay = ReplayDriver::run(db.storage.pool(), None).await?;
    assert!(
        replay.ideas.iter().any(|idea| idea.idea_id == idea_one_id),
        "replay output should contain first signed idea"
    );
    assert!(
        replay
            .connections
            .iter()
            .any(|connection| connection.connection_id == connection_id),
        "replay output should contain signed connection"
    );
    assert!(
        replay
            .representations
            .iter()
            .any(|representation| representation.representation_id == representation_id),
        "replay output should contain the product-read representation"
    );
    assert!(
        replay
            .orderings
            .iter()
            .any(|ordering| ordering.ordering_id == ordering_id),
        "replay output should contain the product-read ordering"
    );

    insert_test_snapshot(db.storage.pool(), 0).await?;
    let event_log = oneshot_json(
        db.app(),
        Method::GET,
        "/api/v1/canonical/event-log",
        json!({}),
        None,
    )
    .await;
    assert_eq!(
        event_log.status,
        StatusCode::OK,
        "{}",
        event_log.body_preview
    );
    let events = event_log
        .json
        .get("events")
        .and_then(Value::as_array)
        .expect("events");
    let signed_event = events
        .iter()
        .find(|event| {
            event.get("event_id").and_then(Value::as_str)
                == Some("00000000-0000-7000-8000-000000000101")
        })
        .expect("signed event log entry");
    assert_eq!(
        signed_event
            .get("authorship_status")
            .and_then(Value::as_str),
        Some("profile_v0_signed")
    );
    assert!(signed_event
        .get("signature")
        .and_then(Value::as_str)
        .is_some());
    assert!(signed_event
        .get("public_key_ref")
        .and_then(Value::as_str)
        .is_some());
    let legacy_event = events
        .iter()
        .find(|event| {
            event.get("event_id").and_then(Value::as_str) == Some(BOOTSTRAP_IDENTITY_EVENT_ID)
        })
        .expect("legacy bootstrap event log entry");
    assert_eq!(
        legacy_event
            .get("authorship_status")
            .and_then(Value::as_str),
        Some("legacy_or_unsigned")
    );
    assert!(legacy_event.get("signature_profile").is_none());

    let idea_read = oneshot_json(
        db.app(),
        Method::GET,
        &format!("/api/v0/idea/{idea_one_id}"),
        json!({}),
        None,
    )
    .await;
    assert_eq!(
        idea_read.status,
        StatusCode::OK,
        "{}",
        idea_read.body_preview
    );
    assert_eq!(
        idea_read
            .json
            .pointer("/idea/idea_id")
            .and_then(Value::as_str),
        Some(idea_one_id.to_string().as_str())
    );

    let capabilities = oneshot_json(
        db.app(),
        Method::GET,
        "/api/v0/capabilities",
        json!({}),
        None,
    )
    .await;
    assert_eq!(
        capabilities.status,
        StatusCode::OK,
        "{}",
        capabilities.body_preview
    );
    assert_eq!(
        capabilities
            .json
            .get("api_contract_version")
            .and_then(Value::as_str),
        Some("1.0.0")
    );
    assert_eq!(
        capabilities
            .json
            .get("migration_head")
            .and_then(Value::as_str),
        Some("0025_seed_conformance_bindings")
    );
    assert_eq!(
        capabilities
            .json
            .get("supported_canonical_signed_write_kinds")
            .and_then(Value::as_array)
            .map(Vec::len),
        Some(2)
    );

    let pinned_response = db
        .app()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!(
                    "/api/v0/idea/{idea_one_id}?snapshot_height=0&connection_limit=10"
                ))
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(pinned_response.status(), StatusCode::OK);
    assert_eq!(
        pinned_response
            .headers()
            .get("x-snapshot-height")
            .and_then(|value| value.to_str().ok()),
        Some("0")
    );
    assert_eq!(
        pinned_response
            .headers()
            .get("x-state-root-hash")
            .and_then(|value| value.to_str().ok()),
        Some("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
    );
    let pinned_body = to_bytes(pinned_response.into_body(), usize::MAX).await?;
    let pinned_json: Value = serde_json::from_slice(&pinned_body)?;
    assert_eq!(
        pinned_json
            .pointer("/basis/snapshot_height")
            .and_then(Value::as_str),
        Some("0")
    );
    assert_eq!(
        pinned_json
            .pointer("/basis/shared_map_commitment")
            .and_then(Value::as_str),
        Some("dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd")
    );

    let resolved = oneshot_json(
        db.app(),
        Method::GET,
        &format!(
            "/api/v0/ideas/resolve?snapshot_height=0&idea_ids={idea_one_id},00000000-0000-7000-8000-00000000b199"
        ),
        json!({}),
        None,
    )
    .await;
    assert_eq!(resolved.status, StatusCode::OK, "{}", resolved.body_preview);
    assert_eq!(
        resolved
            .json
            .pointer("/ideas/0/idea_id")
            .and_then(Value::as_str),
        Some(idea_one_id.to_string().as_str())
    );
    assert_eq!(
        resolved
            .json
            .pointer("/missing_idea_ids/0")
            .and_then(Value::as_str),
        Some("00000000-0000-7000-8000-00000000b199")
    );

    let exact_title = oneshot_json(
        db.app(),
        Method::GET,
        "/api/v0/ideas/exact-match?snapshot_height=0&field=title&value=Signed%20route%20idea%20one&limit=10",
        json!({}),
        None,
    )
    .await;
    assert_eq!(
        exact_title.status,
        StatusCode::OK,
        "{}",
        exact_title.body_preview
    );
    assert_eq!(
        exact_title
            .json
            .pointer("/matches/0/idea_id")
            .and_then(Value::as_str),
        Some(idea_one_id.to_string().as_str())
    );
    assert_eq!(
        exact_title.json.get("truncated").and_then(Value::as_bool),
        Some(false)
    );

    let exact_sentence = oneshot_json(
        db.app(),
        Method::GET,
        "/api/v0/ideas/exact-match?snapshot_height=0&field=sentence&value=Signed%20ingress%20test%20sentence&limit=10",
        json!({}),
        None,
    )
    .await;
    assert_eq!(
        exact_sentence.status,
        StatusCode::OK,
        "{}",
        exact_sentence.body_preview
    );
    assert_eq!(
        exact_sentence
            .json
            .get("matches")
            .and_then(Value::as_array)
            .map(Vec::len),
        Some(2)
    );

    let representations = oneshot_json(
        db.app(),
        Method::GET,
        &format!("/api/v0/idea/{idea_one_id}/representations?snapshot_height=0&limit=10&offset=0"),
        json!({}),
        None,
    )
    .await;
    assert_eq!(
        representations.status,
        StatusCode::OK,
        "{}",
        representations.body_preview
    );
    assert_eq!(
        representations
            .json
            .pointer("/representations/0/representation_id")
            .and_then(Value::as_str),
        Some(representation_id.to_string().as_str())
    );
    assert_eq!(
        representations
            .json
            .pointer("/representations/0/payload_text")
            .and_then(Value::as_str),
        Some("Product read contract representation")
    );
    assert_eq!(
        representations
            .json
            .pointer("/representations/0/representation_kind")
            .and_then(Value::as_str),
        Some("description")
    );
    assert_eq!(
        representations
            .json
            .pointer("/representations/0/tier_length")
            .and_then(Value::as_str),
        Some("sentence")
    );
    assert_eq!(
        representations
            .json
            .pointer("/representations/0/tier_complexity")
            .and_then(Value::as_str),
        Some("fundamental")
    );
    assert_eq!(
        representations
            .json
            .pointer("/representations/0/provenance")
            .and_then(Value::as_str),
        Some("test://product-read-contract")
    );

    let representation = oneshot_json(
        db.app(),
        Method::GET,
        &format!("/api/v0/representation/{representation_id}?snapshot_height=0"),
        json!({}),
        None,
    )
    .await;
    assert_eq!(
        representation.status,
        StatusCode::OK,
        "{}",
        representation.body_preview
    );
    assert_eq!(
        representation
            .json
            .pointer("/representation/author_identity_id")
            .and_then(Value::as_str),
        Some(AUTHOR_ID)
    );
    assert_eq!(
        representation
            .json
            .pointer("/representation/created_block_height")
            .and_then(Value::as_str),
        Some("0")
    );
    assert_eq!(
        representation
            .json
            .pointer("/basis/snapshot_height")
            .and_then(Value::as_str),
        Some("0")
    );

    let pinned_product_reads = vec![
        "/api/v0/snapshot/0".to_string(),
        "/api/v0/ideas/top?snapshot_height=0&limit=10&offset=0&order=asc".to_string(),
        "/api/v0/search/ideas?snapshot_height=0&q=Signed&limit=10&offset=0".to_string(),
        format!(
            "/api/v0/idea/{idea_one_id}/neighborhood?snapshot_height=0&depth=2&limit_per_hop=10"
        ),
        format!(
            "/api/v0/coordinates?snapshot_height=0&reference_id={idea_one_id}&limit=10"
        ),
        format!(
            "/api/v0/idea/{idea_one_id}/orderings?snapshot_height=0&limit=10"
        ),
        format!(
            "/api/v0/connections/relative-importance?snapshot_height=0&idea_ids={idea_one_id},{idea_two_id}&limit=10"
        ),
        format!("/api/v0/identity/{AUTHOR_ID}?snapshot_height=0"),
    ];
    for uri in pinned_product_reads {
        let response = oneshot_json(db.app(), Method::GET, &uri, json!({}), None).await;
        assert_eq!(response.status, StatusCode::OK, "{}", response.body_preview);
        assert_eq!(
            response
                .json
                .pointer("/basis/snapshot_height")
                .and_then(Value::as_str),
            Some("0"),
            "missing pinned basis for {uri}"
        );
    }

    let ordering = oneshot_json(
        db.app(),
        Method::GET,
        &format!("/api/v0/ordering/{ordering_id}?snapshot_height=0&item_limit=10"),
        json!({}),
        None,
    )
    .await;
    assert_eq!(ordering.status, StatusCode::OK, "{}", ordering.body_preview);
    assert_eq!(
        ordering
            .json
            .pointer("/ordering/ordering_id")
            .and_then(Value::as_str),
        Some(ordering_id.to_string().as_str())
    );
    assert_eq!(
        ordering
            .json
            .pointer("/ordering/items/1/idea_id")
            .and_then(Value::as_str),
        Some(idea_two_id.to_string().as_str())
    );
    assert_eq!(
        ordering
            .json
            .pointer("/basis/snapshot_height")
            .and_then(Value::as_str),
        Some("0")
    );

    Ok(())
}

async fn run_legacy_migration_compatibility(
    db: &IsolatedSignedIngressDb,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(sqlx::FromRow)]
    struct LegacyEventRow {
        signature_profile: Option<String>,
        public_key_ref: Option<String>,
        signed_candidate_bytes_v0: Option<Vec<u8>>,
        authored_candidate_hash_v0: Option<String>,
        publication_profile: Option<String>,
    }

    let legacy_event: LegacyEventRow = sqlx::query_as(
        r#"
        SELECT
          signature_profile,
          public_key_ref,
          signed_candidate_bytes_v0,
          authored_candidate_hash_v0,
          publication_profile
        FROM events
        WHERE event_id = $1
        "#,
    )
    .bind(uuid("00000000-0000-7000-8000-00000000e101"))
    .fetch_one(db.storage.pool())
    .await?;
    assert!(legacy_event.signature_profile.is_none());
    assert!(legacy_event.public_key_ref.is_none());
    assert!(legacy_event.signed_candidate_bytes_v0.is_none());
    assert!(legacy_event.authored_candidate_hash_v0.is_none());
    assert!(legacy_event.publication_profile.is_none());

    assert_eq!(
        count_rows(
            db.storage.pool(),
            "ideas",
            Some("idea_id = '00000000-0000-7000-8000-00000000e201'")
        )
        .await?,
        1
    );
    assert_eq!(
        count_rows(
            db.storage.pool(),
            "connections",
            Some("connection_id = '00000000-0000-7000-8000-00000000e301'")
        )
        .await?,
        1
    );

    let replay = ReplayDriver::run(db.storage.pool(), None).await?;
    assert!(
        replay
            .ideas
            .iter()
            .any(|idea| idea.idea_id == uuid("00000000-0000-7000-8000-00000000e201")),
        "replay should preserve legacy idea"
    );
    assert!(
        replay.connections.iter().any(|connection| {
            connection.connection_id == uuid("00000000-0000-7000-8000-00000000e301")
        }),
        "replay should preserve legacy connection"
    );

    insert_test_snapshot(db.storage.pool(), 0).await?;
    let event_log = oneshot_json(
        db.app(),
        Method::GET,
        "/api/v1/canonical/event-log",
        json!({}),
        None,
    )
    .await;
    assert_eq!(
        event_log.status,
        StatusCode::OK,
        "{}",
        event_log.body_preview
    );
    let events = event_log
        .json
        .get("events")
        .and_then(Value::as_array)
        .expect("events");
    let legacy_event = events
        .iter()
        .find(|event| {
            event.get("event_id").and_then(Value::as_str)
                == Some("00000000-0000-7000-8000-00000000e101")
        })
        .expect("legacy event log row");
    assert_eq!(
        legacy_event
            .get("authorship_status")
            .and_then(Value::as_str),
        Some("legacy_or_unsigned")
    );
    assert!(legacy_event.get("signature_profile").is_none());

    Ok(())
}

async fn run_signed_ingress_rejections(
    db: &IsolatedSignedIngressDb,
) -> Result<(), Box<dyn std::error::Error>> {
    let signing_key = test_signing_key();
    let valid = signed_idea_request(
        &signing_key,
        "00000000-0000-7000-8000-000000000201",
        uuid("00000000-0000-7000-8000-00000000b201"),
        "Rejected candidate baseline",
    );

    assert_rejects_without_effect(db, mutate_signature(&valid, ""), "malformed_signature").await?;
    assert_rejects_without_effect(
        db,
        mutate_signature(&valid, &"00".repeat(63)),
        "malformed_signature",
    )
    .await?;
    assert_rejects_without_effect(
        db,
        mutate_signature(
            &valid,
            &format!("{}00", &candidate_signature(&valid)[..126]),
        ),
        "invalid_signature",
    )
    .await?;
    assert_rejects_without_effect(
        db,
        mutate_candidate_field(&valid, "signature_profile", json!("ed25519ctx_v0")),
        "unsupported_signature_profile",
    )
    .await?;
    assert_rejects_without_effect(
        db,
        mutate_candidate_field(
            &valid,
            "author_identity_id",
            json!("00000000-0000-7000-8000-00000000a099"),
        ),
        "invalid_request",
    )
    .await?;
    assert_rejects_without_effect(
        db,
        mutate_candidate_field(&valid, "public_key_ref", json!("00".repeat(32))),
        "unknown_key",
    )
    .await?;
    append_author_key_state(db.storage.pool(), false).await?;
    assert_rejects_without_effect(
        db,
        signed_idea_request(
            &signing_key,
            "00000000-0000-7000-8000-000000000202",
            uuid("00000000-0000-7000-8000-00000000b202"),
            "Revoked key candidate",
        ),
        "revoked_key",
    )
    .await?;
    append_author_key_state(db.storage.pool(), true).await?;
    append_writer_state(db.storage.pool(), 0).await?;
    assert_rejects_without_effect(
        db,
        signed_idea_request(
            &signing_key,
            "00000000-0000-7000-8000-000000000203",
            uuid("00000000-0000-7000-8000-00000000b203"),
            "Ineligible writer candidate",
        ),
        "forbidden",
    )
    .await?;
    append_writer_state(db.storage.pool(), 1).await?;
    assert_rejects_without_effect(
        db,
        mutate_candidate_field(&valid, "payload_hash", json!("11".repeat(32))),
        "invalid_payload_hash",
    )
    .await?;
    assert_rejects_without_effect(
        db,
        signed_invalid_idea_type_request(
            &signing_key,
            "00000000-0000-7000-8000-000000000204",
            uuid("00000000-0000-7000-8000-00000000b204"),
        ),
        "unsupported_event_type",
    )
    .await?;
    assert_rejects_without_effect(
        db,
        signed_connection_request(
            &signing_key,
            "00000000-0000-7000-8000-000000000205",
            uuid("00000000-0000-7000-8000-00000000c205"),
            uuid("00000000-0000-7000-8000-00000000b299"),
            uuid("00000000-0000-7000-8000-00000000b298"),
        ),
        "invalid_request",
    )
    .await?;
    assert_rejects_without_effect(
        db,
        signed_unsupported_event_request(&signing_key, "00000000-0000-7000-8000-000000000206"),
        "unsupported_event_type",
    )
    .await?;
    assert_rejects_without_effect(
        db,
        mutate_candidate_field(&valid, "payload_binding_mode", json!("payload_ref")),
        "unsupported_payload_binding",
    )
    .await?;
    assert_rejects_without_effect(
        db,
        mutate_candidate_field(&valid, "payload_ref", json!("feed")),
        "unsupported_payload_binding",
    )
    .await?;

    let before_accept_events = total_events(db.storage.pool()).await?;
    let before_accept_ideas = total_ideas(db.storage.pool()).await?;
    let accepted = oneshot_json(
        db.app(),
        Method::POST,
        "/api/v1/canonical/events",
        valid.clone(),
        None,
    )
    .await;
    assert_eq!(accepted.status, StatusCode::OK, "{}", accepted.body_preview);
    let conflict = signed_idea_request(
        &signing_key,
        "00000000-0000-7000-8000-000000000201",
        uuid("00000000-0000-7000-8000-00000000b201"),
        "Conflicting payload",
    );
    let conflict_response = oneshot_json(
        db.app(),
        Method::POST,
        "/api/v1/canonical/events",
        conflict,
        None,
    )
    .await;
    assert_eq!(conflict_response.status, StatusCode::CONFLICT);
    assert_error_code(&conflict_response, "conflict");
    assert_eq!(
        total_events(db.storage.pool()).await?,
        before_accept_events + 1
    );
    assert_eq!(
        total_ideas(db.storage.pool()).await?,
        before_accept_ideas + 1
    );

    let oversized = "x".repeat(common::security_limits::API_CANONICAL_BODY_LIMIT_BYTES + 128);
    let response = db
        .app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/canonical/events")
                .header("content-type", "application/json")
                .body(Body::from(oversized))
                .expect("request"),
        )
        .await
        .expect("oversized response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    let body = String::from_utf8(body.to_vec()).expect("utf8");
    assert!(body.contains("\"error_code\":\"payload_too_large\""));

    Ok(())
}

async fn bootstrap_signed_ingress_identity(
    storage: &Storage,
) -> Result<(), Box<dyn std::error::Error>> {
    let author_id = uuid(AUTHOR_ID);
    let signing_key = test_signing_key();
    let public_key = signing_key.verifying_key();
    let public_key_ref = public_key_ref_v0(public_key.as_bytes(), author_id)?;
    sqlx::query(
        r#"
        INSERT INTO blocks (block_height, block_hash, prev_block_hash)
        VALUES (0, '0', NULL)
        ON CONFLICT (block_height) DO NOTHING
        "#,
    )
    .execute(storage.pool())
    .await?;
    sqlx::query(
        r#"
        INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
        VALUES (0, 0, $1, 'identity_create', $2, $3, NULL)
        "#,
    )
    .bind(uuid(BOOTSTRAP_IDENTITY_EVENT_ID))
    .bind(author_id)
    .bind(json!({
        "identity_id": AUTHOR_ID,
        "title": "Signed Ingress Test Human",
        "initial_public_key_ref": public_key_ref,
        "speaker_identity_id": AUTHOR_ID
    }))
    .execute(storage.pool())
    .await?;
    sqlx::query(
        r#"
        INSERT INTO identities_s0 (identity_id, title, created_event_id)
        VALUES ($1, 'Signed Ingress Test Human', $2)
        "#,
    )
    .bind(author_id)
    .bind(uuid(BOOTSTRAP_IDENTITY_EVENT_ID))
    .execute(storage.pool())
    .await?;
    sqlx::query(
        r#"
        INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
        VALUES (0, 1, $1, 'identity_create', $2, $3, NULL)
        "#,
    )
    .bind(uuid(BOOTSTRAP_VERIFIER_EVENT_ID))
    .bind(uuid(BOOTSTRAP_VERIFIER_ID))
    .bind(json!({
        "identity_id": BOOTSTRAP_VERIFIER_ID,
        "title": "Seed Bootstrap Verifier",
        "speaker_identity_id": BOOTSTRAP_VERIFIER_ID
    }))
    .execute(storage.pool())
    .await?;
    sqlx::query(
        r#"
        INSERT INTO identities_s0 (identity_id, title, created_event_id)
        VALUES ($1, 'Seed Bootstrap Verifier', $2)
        "#,
    )
    .bind(uuid(BOOTSTRAP_VERIFIER_ID))
    .bind(uuid(BOOTSTRAP_VERIFIER_EVENT_ID))
    .execute(storage.pool())
    .await?;
    sqlx::query(
        r#"
        INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
        VALUES (0, 2, $1, 'canonical_writer_grant', $2, $3, NULL)
        "#,
    )
    .bind(uuid(BOOTSTRAP_WRITER_EVENT_ID))
    .bind(uuid(BOOTSTRAP_VERIFIER_ID))
    .bind(json!({
        "identity_id": AUTHOR_ID,
        "canonical_writer_level": 1,
        "email_verified": true
    }))
    .execute(storage.pool())
    .await?;
    sqlx::query(
        r#"
        INSERT INTO canonical_writer_verification_states (
          identity_id,
          email_verified,
          canonical_writer_level,
          granted_by_identity_id,
          source_event_id,
          source_block_height,
          source_event_index
        ) VALUES ($1, true, 1, $2, $3, 0, 2)
        "#,
    )
    .bind(author_id)
    .bind(uuid(BOOTSTRAP_VERIFIER_ID))
    .bind(uuid(BOOTSTRAP_WRITER_EVENT_ID))
    .execute(storage.pool())
    .await?;
    sqlx::query(
        r#"
        INSERT INTO canonical_identity_key_states (
          key_state_id,
          identity_id,
          public_key_ref,
          signature_profile,
          signature_algorithm,
          public_key_bytes,
          is_active,
          source_event_id,
          source_block_height,
          source_event_index,
          source_kind,
          recovery_process_ref
        ) VALUES ($1, $2, $3, 'ed25519_v0', 'ed25519', $4, true, NULL, NULL, NULL, 'test_bootstrap', NULL)
        "#,
    )
    .bind(uuid("00000000-0000-7000-8000-00000000d001"))
    .bind(author_id)
    .bind(public_key_ref)
    .bind(public_key.as_bytes().as_slice())
    .execute(storage.pool())
    .await?;
    Ok(())
}

async fn insert_pre_0022_legacy_rows(pool: &PgPool) -> Result<(), sqlx::Error> {
    let author_id = uuid(AUTHOR_ID);
    let identity_event_id = uuid("00000000-0000-7000-8000-00000000e001");
    let idea_one_event_id = uuid("00000000-0000-7000-8000-00000000e101");
    let idea_two_event_id = uuid("00000000-0000-7000-8000-00000000e102");
    let connection_event_id = uuid("00000000-0000-7000-8000-00000000e103");
    let idea_one_id = uuid("00000000-0000-7000-8000-00000000e201");
    let idea_two_id = uuid("00000000-0000-7000-8000-00000000e202");
    let connection_id = uuid("00000000-0000-7000-8000-00000000e301");
    let title_one = "Legacy idea one";
    let title_two = "Legacy idea two";
    let sentence = "Legacy pre-0022 signed-substrate fixture";
    let payload_hash_one =
        payload_hash_hex(title_one, sentence, None, None).expect("legacy payload hash one");
    let payload_hash_two =
        payload_hash_hex(title_two, sentence, None, None).expect("legacy payload hash two");

    sqlx::query(
        r#"
        INSERT INTO blocks (block_height, block_hash, prev_block_hash)
        VALUES (0, '0', NULL)
        "#,
    )
    .execute(pool)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
        VALUES
          (0, 0, $1, 'identity_create', $2, $3, NULL),
          (0, 1, $4, 'idea_create', $2, $5, NULL),
          (0, 2, $6, 'idea_create', $2, $7, NULL),
          (0, 3, $8, 'connection_create', $2, $9, NULL)
        "#,
    )
    .bind(identity_event_id)
    .bind(author_id)
    .bind(json!({
        "identity_id": AUTHOR_ID,
        "title": "Legacy Pre-0022 Human",
        "speaker_identity_id": AUTHOR_ID
    }))
    .bind(idea_one_event_id)
    .bind(json!({
        "idea_id": idea_one_id.to_string(),
        "idea_type": "conceptual_idea",
        "speaker_identity_id": AUTHOR_ID,
        "title": title_one,
        "sentence": sentence,
        "paragraph": null,
        "full": null,
        "payload_hash": payload_hash_one
    }))
    .bind(idea_two_event_id)
    .bind(json!({
        "idea_id": idea_two_id.to_string(),
        "idea_type": "conceptual_idea",
        "speaker_identity_id": AUTHOR_ID,
        "title": title_two,
        "sentence": sentence,
        "paragraph": null,
        "full": null,
        "payload_hash": payload_hash_two
    }))
    .bind(connection_event_id)
    .bind(json!({
        "connection_id": connection_id.to_string(),
        "from_idea_id": idea_one_id.to_string(),
        "to_idea_id": idea_two_id.to_string(),
        "connection_type": "membership",
        "speaker_identity_id": AUTHOR_ID,
        "usage": null,
        "axis": null,
        "timeframe": null,
        "scope": null
    }))
    .execute(pool)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO identities_s0 (identity_id, title, created_event_id)
        VALUES ($1, 'Legacy Pre-0022 Human', $2)
        "#,
    )
    .bind(author_id)
    .bind(identity_event_id)
    .execute(pool)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO ideas (
          idea_id,
          idea_type,
          speaker_identity_id,
          created_block_height,
          created_event_index,
          created_event_id
        ) VALUES
          ($1, 'conceptual_idea', $2, 0, 1, $3),
          ($4, 'conceptual_idea', $2, 0, 2, $5)
        "#,
    )
    .bind(idea_one_id)
    .bind(author_id)
    .bind(idea_one_event_id)
    .bind(idea_two_id)
    .bind(idea_two_event_id)
    .execute(pool)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO connections (
          connection_id,
          from_idea_id,
          to_idea_id,
          connection_type,
          usage,
          axis,
          timeframe,
          scope,
          created_block_height,
          created_event_index,
          created_by_event_id
        ) VALUES ($1, $2, $3, 'membership', NULL, NULL, NULL, NULL, 0, 3, $4)
        "#,
    )
    .bind(connection_id)
    .bind(idea_one_id)
    .bind(idea_two_id)
    .bind(connection_event_id)
    .execute(pool)
    .await?;

    Ok(())
}

fn signed_idea_request(
    signing_key: &SigningKey,
    event_id: &str,
    idea_id: Uuid,
    title: &str,
) -> Value {
    let author_id = uuid(AUTHOR_ID);
    let sentence = "Signed ingress test sentence";
    let payload_hash = payload_hash_hex(title, sentence, None, None).expect("idea payload hash");
    let payload = json!({
        "idea_id": idea_id.to_string(),
        "idea_type": "conceptual_idea",
        "speaker_identity_id": author_id.to_string(),
        "title": title,
        "sentence": sentence,
        "paragraph": null,
        "full": null,
        "payload_hash": payload_hash
    });
    signed_request(signing_key, event_id, "idea_create", payload)
}

fn signed_invalid_idea_type_request(
    signing_key: &SigningKey,
    event_id: &str,
    idea_id: Uuid,
) -> Value {
    let author_id = uuid(AUTHOR_ID);
    let title = "Invalid identity attempt";
    let sentence = "Signed ingress test sentence";
    let payload_hash = payload_hash_hex(title, sentence, None, None).expect("idea payload hash");
    let payload = json!({
        "idea_id": idea_id.to_string(),
        "idea_type": "identity",
        "speaker_identity_id": author_id.to_string(),
        "title": title,
        "sentence": sentence,
        "paragraph": null,
        "full": null,
        "payload_hash": payload_hash
    });
    signed_request(signing_key, event_id, "idea_create", payload)
}

fn signed_connection_request(
    signing_key: &SigningKey,
    event_id: &str,
    connection_id: Uuid,
    from_idea_id: Uuid,
    to_idea_id: Uuid,
) -> Value {
    let author_id = uuid(AUTHOR_ID);
    let payload = json!({
        "connection_id": connection_id.to_string(),
        "from_idea_id": from_idea_id.to_string(),
        "to_idea_id": to_idea_id.to_string(),
        "connection_type": "membership",
        "speaker_identity_id": author_id.to_string(),
        "usage": null,
        "axis": null,
        "timeframe": null,
        "scope": null
    });
    signed_request(signing_key, event_id, "connection_create", payload)
}

fn signed_unsupported_event_request(signing_key: &SigningKey, event_id: &str) -> Value {
    let author_id = uuid(AUTHOR_ID);
    let payload = json!({
        "challenge_id": "00000000-0000-7000-8000-00000000c999",
        "speaker_identity_id": author_id.to_string()
    });
    signed_request(signing_key, event_id, "challenge_create", payload)
}

fn signed_request(
    signing_key: &SigningKey,
    event_id: &str,
    event_type: &str,
    payload: Value,
) -> Value {
    let author_id = uuid(AUTHOR_ID);
    let public_key = signing_key.verifying_key();
    let public_key_ref = public_key_ref_v0(public_key.as_bytes(), author_id).expect("key ref");
    let payload_hash = canonical_json_payload_hash_hex(&payload).expect("canonical payload hash");
    let unsigned = AuthoredEventCandidate {
        signature_profile: SIGNATURE_PROFILE_ED25519_V0.to_string(),
        event_id: uuid(event_id),
        event_type: event_type.to_string(),
        author_identity_id: author_id,
        speaker_identity_id: Some(author_id),
        public_key_ref,
        payload_hash,
        payload_binding_mode: PAYLOAD_BINDING_EMBEDDED.to_string(),
        payload_ref: None,
        author_observed_at: None,
        signature: String::new(),
    };
    let signed_bytes = signed_candidate_bytes_v0(&unsigned).expect("signed bytes");
    let signature = signing_key.sign(&signed_bytes);
    json!({
        "candidate": {
            "signature_profile": unsigned.signature_profile,
            "event_id": unsigned.event_id.to_string(),
            "event_type": unsigned.event_type,
            "author_identity_id": unsigned.author_identity_id.to_string(),
            "speaker_identity_id": unsigned.speaker_identity_id.map(|value| value.to_string()),
            "public_key_ref": unsigned.public_key_ref,
            "payload_hash": unsigned.payload_hash,
            "payload_binding_mode": unsigned.payload_binding_mode,
            "payload_ref": null,
            "author_observed_at": null,
            "signature": encoding::payload::to_hex(&signature.to_bytes())
        },
        "payload": payload
    })
}

fn mutate_signature(request: &Value, signature: &str) -> Value {
    mutate_candidate_field(request, "signature", json!(signature))
}

fn mutate_candidate_field(request: &Value, field: &str, value: Value) -> Value {
    let mut next = request.clone();
    next.get_mut("candidate")
        .and_then(Value::as_object_mut)
        .expect("candidate object")
        .insert(field.to_string(), value);
    next
}

fn candidate_signature(request: &Value) -> &str {
    request
        .pointer("/candidate/signature")
        .and_then(Value::as_str)
        .expect("signature")
}

async fn assert_rejects_without_effect(
    db: &IsolatedSignedIngressDb,
    request: Value,
    error_code: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let before_events = total_events(db.storage.pool()).await?;
    let before_ideas = total_ideas(db.storage.pool()).await?;
    let before_connections = total_connections(db.storage.pool()).await?;
    let response = oneshot_json(
        db.app(),
        Method::POST,
        "/api/v1/canonical/events",
        request,
        None,
    )
    .await;
    assert!(
        response.status.is_client_error(),
        "expected client error for {error_code}; got {} {}",
        response.status,
        response.body_preview
    );
    assert_error_code(&response, error_code);
    assert_eq!(total_events(db.storage.pool()).await?, before_events);
    assert_eq!(total_ideas(db.storage.pool()).await?, before_ideas);
    assert_eq!(
        total_connections(db.storage.pool()).await?,
        before_connections
    );
    Ok(())
}

async fn assert_exact_candidate_reconstructs(
    pool: &PgPool,
    event_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(sqlx::FromRow)]
    struct Row {
        event_id: Uuid,
        event_type: String,
        speaker_identity_id: Option<Uuid>,
        author_identity_id: Uuid,
        signature: String,
        signature_profile: String,
        public_key_ref: String,
        payload_hash: String,
        payload_binding_mode: String,
        payload_json: Value,
        signed_candidate_bytes_v0: Vec<u8>,
        authored_candidate_hash_v0: String,
        publication_profile: String,
    }
    let row: Row = sqlx::query_as(
        r#"
        SELECT
          event_id,
          event_type,
          speaker_identity_id,
          author_identity_id,
          signature,
          signature_profile,
          public_key_ref,
          payload_hash,
          payload_binding_mode,
          payload_json,
          signed_candidate_bytes_v0,
          authored_candidate_hash_v0,
          publication_profile
        FROM events
        WHERE event_id = $1
        "#,
    )
    .bind(uuid(event_id))
    .fetch_one(pool)
    .await?;
    let candidate = AuthoredEventCandidate {
        signature_profile: row.signature_profile,
        event_id: row.event_id,
        event_type: row.event_type,
        author_identity_id: row.author_identity_id,
        speaker_identity_id: row.speaker_identity_id,
        public_key_ref: row.public_key_ref,
        payload_hash: row.payload_hash.clone(),
        payload_binding_mode: row.payload_binding_mode,
        payload_ref: None,
        author_observed_at: None,
        signature: row.signature.clone(),
    };
    assert_eq!(
        canonical_json_payload_hash_hex(&row.payload_json)?,
        row.payload_hash
    );
    let signed_bytes = signed_candidate_bytes_v0(&candidate)?;
    assert_eq!(signed_bytes, row.signed_candidate_bytes_v0);
    let signature = verification::signatures::decode_signature64(&row.signature)?;
    assert_eq!(
        verification::signatures::authored_candidate_hash_v0(&signed_bytes, &signature)?,
        row.authored_candidate_hash_v0
    );
    assert_eq!(row.publication_profile, PUBLICATION_PROFILE);
    Ok(())
}

async fn assert_rebuilt_signed_projection_matches_materialized(
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug, PartialEq, Eq, sqlx::FromRow)]
    struct Idea {
        idea_id: Uuid,
        idea_type: String,
        speaker_identity_id: Uuid,
        created_event_id: Uuid,
        created_block_height: i64,
        created_event_index: i32,
    }
    #[derive(Debug, PartialEq, Eq, sqlx::FromRow)]
    struct Connection {
        connection_id: Uuid,
        from_idea_id: Uuid,
        to_idea_id: Uuid,
        connection_type: String,
        created_by_event_id: Uuid,
        created_block_height: i64,
        created_event_index: i32,
    }
    #[derive(sqlx::FromRow)]
    struct EventRow {
        block_height: i64,
        event_index: i32,
        event_id: Uuid,
        event_type: String,
        speaker_identity_id: Uuid,
        payload_json: Value,
    }
    let events: Vec<EventRow> = sqlx::query_as(
        r#"
        SELECT block_height, event_index, event_id, event_type, speaker_identity_id, payload_json
        FROM events
        WHERE signature_profile = 'ed25519_v0'
        ORDER BY block_height ASC, event_index ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    let mut rebuilt_ideas = Vec::new();
    let mut rebuilt_connections = Vec::new();
    for event in events {
        let payload = event.payload_json.as_object().expect("payload object");
        match event.event_type.as_str() {
            "idea_create" => rebuilt_ideas.push(Idea {
                idea_id: uuid(
                    payload
                        .get("idea_id")
                        .and_then(Value::as_str)
                        .expect("idea_id"),
                ),
                idea_type: payload
                    .get("idea_type")
                    .and_then(Value::as_str)
                    .expect("idea_type")
                    .to_string(),
                speaker_identity_id: event.speaker_identity_id,
                created_event_id: event.event_id,
                created_block_height: event.block_height,
                created_event_index: event.event_index,
            }),
            "connection_create" => rebuilt_connections.push(Connection {
                connection_id: uuid(
                    payload
                        .get("connection_id")
                        .and_then(Value::as_str)
                        .expect("connection_id"),
                ),
                from_idea_id: uuid(
                    payload
                        .get("from_idea_id")
                        .and_then(Value::as_str)
                        .expect("from_idea_id"),
                ),
                to_idea_id: uuid(
                    payload
                        .get("to_idea_id")
                        .and_then(Value::as_str)
                        .expect("to_idea_id"),
                ),
                connection_type: payload
                    .get("connection_type")
                    .and_then(Value::as_str)
                    .expect("connection_type")
                    .to_string(),
                created_by_event_id: event.event_id,
                created_block_height: event.block_height,
                created_event_index: event.event_index,
            }),
            other => panic!("unexpected signed event type {other}"),
        }
    }
    let materialized_ideas: Vec<Idea> = sqlx::query_as(
        r#"
        SELECT idea_id, idea_type, speaker_identity_id, created_event_id, created_block_height, created_event_index
        FROM ideas
        WHERE created_event_id IN (
          SELECT event_id FROM events WHERE signature_profile = 'ed25519_v0'
        )
        ORDER BY created_block_height ASC, created_event_index ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    let materialized_connections: Vec<Connection> = sqlx::query_as(
        r#"
        SELECT connection_id, from_idea_id, to_idea_id, connection_type, created_by_event_id, created_block_height, created_event_index
        FROM connections
        WHERE created_by_event_id IN (
          SELECT event_id FROM events WHERE signature_profile = 'ed25519_v0'
        )
        ORDER BY created_block_height ASC, created_event_index ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    assert_eq!(rebuilt_ideas, materialized_ideas);
    assert_eq!(rebuilt_connections, materialized_connections);
    Ok(())
}

async fn insert_test_ordering(
    pool: &PgPool,
    idea_one_id: Uuid,
    idea_two_id: Uuid,
    ordering_id: Uuid,
) -> Result<(), sqlx::Error> {
    let event_id = uuid("00000000-0000-7000-8000-000000000105");
    let mut tx = pool.begin().await?;
    let block_height = 0_i64;
    let event_index: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(event_index) + 1, 0)::int FROM events WHERE block_height = $1",
    )
    .bind(block_height)
    .fetch_one(&mut *tx)
    .await?;
    let payload = json!({
        "ordering_id": ordering_id,
        "ordering_profile": "vine",
        "vine_type": "narrative_vine",
        "speaker_identity_id": AUTHOR_ID,
        "item_idea_ids": [idea_one_id, idea_two_id]
    });
    sqlx::query(
        r#"
        INSERT INTO events (
          block_height,
          event_index,
          event_id,
          event_type,
          speaker_identity_id,
          payload_json,
          signature
        ) VALUES ($1, $2, $3, 'ordering_create', $4, $5, NULL)
        "#,
    )
    .bind(block_height)
    .bind(event_index)
    .bind(event_id)
    .bind(uuid(AUTHOR_ID))
    .bind(payload)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO orderings (
          ordering_id,
          ordering_profile,
          vine_type,
          subject_idea_id,
          speaker_identity_id,
          created_block_height,
          created_event_index,
          created_event_id,
          base_ordering_id,
          title_representation_id,
          sentence_representation_id
        ) VALUES ($1, 0, 1, NULL, $2, $3, $4, $5, NULL, NULL, NULL)
        "#,
    )
    .bind(ordering_id)
    .bind(uuid(AUTHOR_ID))
    .bind(block_height)
    .bind(event_index)
    .bind(event_id)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO ordering_items (ordering_id, idx, idea_id, item_role, via_connection_id)
        VALUES ($1, 0, $2, NULL, NULL), ($1, 1, $3, NULL, NULL)
        "#,
    )
    .bind(ordering_id)
    .bind(idea_one_id)
    .bind(idea_two_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}
async fn insert_test_representation(
    pool: &PgPool,
    idea_id: Uuid,
    representation_id: Uuid,
) -> Result<(), sqlx::Error> {
    let event_id = uuid("00000000-0000-7000-8000-000000000104");
    let payload_hash = "f".repeat(64);
    let payload_text = "Product read contract representation";
    let payload = json!({
        "representation_id": representation_id,
        "target_kind": "idea",
        "target_object_id": idea_id,
        "representation_kind": "description",
        "tier_length": "sentence",
        "tier_complexity": "fundamental",
        "payload_hash": payload_hash,
        "payload_text": payload_text,
        "author_identity_id": AUTHOR_ID
    });
    let (block_height, event_index) = append_internal_event(
        pool,
        event_id,
        "representation_create",
        Some(uuid(AUTHOR_ID)),
        payload,
    )
    .await?;
    sqlx::query(
        r#"
        INSERT INTO representations (
          representation_id,
          target_kind,
          target_id,
          tier_enum,
          tier_complexity,
          vocabulary_version_id,
          payload_hash,
          payload_text,
          author_identity_id,
          language_locale,
          provenance,
          created_block_height,
          created_event_index,
          created_event_id
        ) VALUES ($1, 0, $2, 1, 0, NULL, $3, $4, $5, 'en', 'test://product-read-contract', $6, $7, $8)
        "#,
    )
    .bind(representation_id)
    .bind(idea_id)
    .bind(payload_hash)
    .bind(payload_text)
    .bind(uuid(AUTHOR_ID))
    .bind(block_height)
    .bind(event_index)
    .bind(event_id)
    .execute(pool)
    .await?;
    Ok(())
}

async fn insert_test_snapshot(pool: &PgPool, height: i64) -> Result<(), sqlx::Error> {
    let last_event_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT event_id FROM events ORDER BY block_height DESC, event_index DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;
    let event_count: i64 = sqlx::query_scalar("SELECT COUNT(*)::bigint FROM events")
        .fetch_one(pool)
        .await?;
    sqlx::query(
        r#"
        INSERT INTO snapshots (
          snapshot_id,
          block_height,
          format_version,
          snapshot_hash,
          prev_snapshot_hash,
          state_root_hash,
          title_sentence_payload_root,
          shared_map_commitment,
          active_rulebook_set_hash,
          last_event_id,
          event_count,
          approximate_timestamp
        ) VALUES (
          $1, $2, 'test', $3, NULL, $4, $5, $6, $7, $8, $9, now()
        )
        ON CONFLICT (block_height) DO NOTHING
        "#,
    )
    .bind(uuid("00000000-0000-7000-8000-00000000f001"))
    .bind(height)
    .bind("a".repeat(64))
    .bind("b".repeat(64))
    .bind("c".repeat(64))
    .bind("d".repeat(64))
    .bind("e".repeat(64))
    .bind(last_event_id)
    .bind(event_count)
    .execute(pool)
    .await?;
    Ok(())
}

async fn append_author_key_state(pool: &PgPool, active: bool) -> Result<(), sqlx::Error> {
    let event_id = Uuid::now_v7();
    let (block_height, event_index) =
        append_internal_event(pool, event_id, "noop", None, json!({})).await?;
    let author_id = uuid(AUTHOR_ID);
    let signing_key = test_signing_key();
    let public_key = signing_key.verifying_key();
    let public_key_ref =
        public_key_ref_v0(public_key.as_bytes(), author_id).expect("test public key ref");
    sqlx::query(
        r#"
        INSERT INTO canonical_identity_key_states (
          key_state_id,
          identity_id,
          public_key_ref,
          signature_profile,
          signature_algorithm,
          public_key_bytes,
          is_active,
          source_event_id,
          source_block_height,
          source_event_index,
          source_kind,
          recovery_process_ref
        ) VALUES ($1, $2, $3, 'ed25519_v0', 'ed25519', $4, $5, $6, $7, $8, 'test_bootstrap_state', NULL)
        "#,
    )
    .bind(Uuid::now_v7())
    .bind(author_id)
    .bind(public_key_ref)
    .bind(public_key.as_bytes().as_slice())
    .bind(active)
    .bind(event_id)
    .bind(block_height)
    .bind(event_index)
    .execute(pool)
    .await?;
    Ok(())
}

async fn append_writer_state(pool: &PgPool, level: i16) -> Result<(), sqlx::Error> {
    let author_id = uuid(AUTHOR_ID);
    let event_id = Uuid::now_v7();
    let event_type = if level > 0 {
        "canonical_writer_grant"
    } else {
        "canonical_writer_revoke"
    };
    let payload = json!({
        "identity_id": AUTHOR_ID,
        "canonical_writer_level": level,
        "email_verified": level > 0
    });
    let (block_height, event_index) =
        append_internal_event(pool, event_id, event_type, Some(author_id), payload).await?;
    sqlx::query(
        r#"
        INSERT INTO canonical_writer_verification_states (
          identity_id,
          email_verified,
          canonical_writer_level,
          granted_by_identity_id,
          source_event_id,
          source_block_height,
          source_event_index
        ) VALUES ($1, $2, $3, $1, $4, $5, $6)
        "#,
    )
    .bind(author_id)
    .bind(level > 0)
    .bind(level)
    .bind(event_id)
    .bind(block_height)
    .bind(event_index)
    .execute(pool)
    .await?;
    Ok(())
}

async fn append_internal_event(
    pool: &PgPool,
    event_id: Uuid,
    event_type: &str,
    speaker_identity_id: Option<Uuid>,
    payload: Value,
) -> Result<(i64, i32), sqlx::Error> {
    let block_height = 0_i64;
    let event_index: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(event_index) + 1, 0)::int FROM events WHERE block_height = $1",
    )
    .bind(block_height)
    .fetch_one(pool)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO events (
          block_height,
          event_index,
          event_id,
          event_type,
          speaker_identity_id,
          payload_json,
          signature
        ) VALUES ($1, $2, $3, $4, $5, $6, NULL)
        "#,
    )
    .bind(block_height)
    .bind(event_index)
    .bind(event_id)
    .bind(event_type)
    .bind(speaker_identity_id)
    .bind(payload)
    .execute(pool)
    .await?;
    Ok((block_height, event_index))
}

async fn total_events(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*)::bigint FROM events")
        .fetch_one(pool)
        .await
}

async fn total_ideas(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*)::bigint FROM ideas")
        .fetch_one(pool)
        .await
}

async fn total_connections(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*)::bigint FROM connections")
        .fetch_one(pool)
        .await
}

async fn count_rows(
    pool: &PgPool,
    table: &str,
    where_clause: Option<&str>,
) -> Result<i64, sqlx::Error> {
    assert!(matches!(table, "events" | "ideas" | "connections"));
    let sql = match where_clause {
        Some(where_clause) => format!("SELECT COUNT(*)::bigint FROM {table} WHERE {where_clause}"),
        None => format!("SELECT COUNT(*)::bigint FROM {table}"),
    };
    sqlx::query_scalar(&sql).fetch_one(pool).await
}

fn test_signing_key() -> SigningKey {
    SigningKey::from_bytes(&[7_u8; 32])
}

fn unique_test_database_name() -> String {
    format!(
        "seed_signed_ingress_test_{}_{}",
        std::process::id(),
        Uuid::now_v7().simple()
    )
}

fn database_url_for(admin_url: &str, database_name: &str) -> Option<String> {
    if !database_name
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return None;
    }
    let (head, query) = match admin_url.find('?') {
        Some(idx) => (&admin_url[..idx], &admin_url[idx..]),
        None => (admin_url, ""),
    };
    let slash = head.rfind('/')?;
    Some(format!("{}{database_name}{query}", &head[..slash + 1]))
}

fn quote_ident(value: &str) -> String {
    assert!(
        value
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_'),
        "unsafe generated database name"
    );
    format!("\"{}\"", value)
}

async fn drop_database(admin_pool: &PgPool, database_name: &str) {
    let _ = sqlx::query(
        r#"
        SELECT pg_terminate_backend(pid)
        FROM pg_stat_activity
        WHERE datname = $1 AND pid <> pg_backend_pid()
        "#,
    )
    .bind(database_name)
    .execute(admin_pool)
    .await;
    let drop_sql = format!("DROP DATABASE IF EXISTS {}", quote_ident(database_name));
    let _ = sqlx::query(&drop_sql).execute(admin_pool).await;
}

fn uuid(value: &str) -> Uuid {
    Uuid::parse_str(value).expect("valid UUID")
}
