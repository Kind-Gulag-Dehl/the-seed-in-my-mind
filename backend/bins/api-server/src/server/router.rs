#[cfg(feature = "full")]
use axum::middleware::from_fn_with_state;
use axum::{extract::DefaultBodyLimit, middleware::map_response, routing::get, Router};
use common::security_limits::API_GLOBAL_BODY_LIMIT_BYTES;
#[cfg(feature = "full")]
use common::security_limits::{
    API_AUTH_BODY_LIMIT_BYTES, API_CANONICAL_BODY_LIMIT_BYTES, API_PRIVATE_AI_BODY_LIMIT_BYTES,
    API_PRIVATE_BODY_LIMIT_BYTES,
};

#[cfg(feature = "full")]
use crate::server::auth::{auth_login, auth_logout, auth_me, auth_middleware, auth_register};
use crate::server::errors::normalize_error_response;
#[cfg(feature = "full")]
use crate::server::handlers::canonical::{
    canonical_attach_importance_argument, canonical_blocked_submission,
    canonical_cast_challenge_vote, canonical_create_connection, canonical_create_idea,
    canonical_create_identity, canonical_create_importance_challenge, canonical_pull_vote_session,
    canonical_verifier_grant_writer, canonical_verifier_revoke_writer,
};
use crate::server::handlers::canonical::{
    canonical_challenge_detail, canonical_coordinates, canonical_cycles_current,
    canonical_event_log, canonical_tempo_status, canonical_verification_status,
};
#[cfg(feature = "full")]
use crate::server::handlers::private::{
    private_ai_draft, private_create_idea, private_create_vine, private_delete_idea,
    private_delete_vine, private_get_idea, private_get_vine, private_list_ideas,
    private_list_vines, private_update_idea, private_update_vine,
};
use crate::server::handlers::public::{
    health_check, idea_detail_handler, idea_neighborhood, idea_rails_handler, ideas_top,
    identity_detail_handler, latest_snapshot, rail_detail_handler, relative_importance_connections,
    search_ideas, snapshot_by_height, snapshot_commit_by_height, snapshot_commit_list,
};
use crate::server::types::AppState;

pub(crate) fn build_app(state: AppState) -> Router {
    let canonical_public_routes = Router::new()
        .route("/cycles/current", get(canonical_cycles_current))
        .route("/event-log", get(canonical_event_log))
        .route("/tempo/status", get(canonical_tempo_status))
        .route(
            "/verification/:identity_id",
            get(canonical_verification_status),
        )
        .route("/challenges/:challenge_id", get(canonical_challenge_detail));
    let app = Router::new()
        .route("/api/v0/health", get(health_check))
        .route("/api/v0/snapshot/latest", get(latest_snapshot))
        .route("/api/v0/snapshot/:height", get(snapshot_by_height))
        .route("/api/v0/snapshots/commits", get(snapshot_commit_list))
        .route(
            "/api/v0/snapshots/commits/:height",
            get(snapshot_commit_by_height),
        )
        .route("/api/v0/ideas/top", get(ideas_top))
        .route("/api/v0/coordinates", get(canonical_coordinates))
        .route("/api/v0/idea/:idea_id", get(idea_detail_handler))
        .route("/api/v0/rail/:rail_id", get(rail_detail_handler))
        .route("/api/v0/idea/:idea_id/rails", get(idea_rails_handler))
        .route("/api/v0/idea/:idea_id/neighborhood", get(idea_neighborhood))
        .route(
            "/api/v0/connections/relative-importance",
            get(relative_importance_connections),
        )
        .route(
            "/api/v0/identity/:identity_id",
            get(identity_detail_handler),
        )
        .route("/api/v0/search/ideas", get(search_ideas))
        .nest("/api/v1/canonical", canonical_public_routes);

    #[cfg(feature = "full")]
    let app = {
        let private_idea_routes = Router::new()
            .route("/ideas", axum::routing::post(private_create_idea))
            .route("/ideas", get(private_list_ideas))
            .route("/ideas/:id", get(private_get_idea))
            .route("/ideas/:id", axum::routing::put(private_update_idea))
            .route("/ideas/:id", axum::routing::patch(private_update_idea))
            .route("/ideas/:id", axum::routing::delete(private_delete_idea))
            .layer(DefaultBodyLimit::max(API_PRIVATE_BODY_LIMIT_BYTES));
        let private_ai_routes = Router::new()
            .route("/draft", axum::routing::post(private_ai_draft))
            .route("/parse", axum::routing::post(private_ai_draft))
            .route("/complete", axum::routing::post(private_ai_draft))
            .layer(DefaultBodyLimit::max(API_PRIVATE_AI_BODY_LIMIT_BYTES));
        let private_routes = Router::new()
            .merge(private_idea_routes)
            .nest("/ai", private_ai_routes)
            .layer(from_fn_with_state(state.clone(), auth_middleware));
        let me_routes = Router::new()
            .route("/vines", get(private_list_vines))
            .route("/vines", axum::routing::post(private_create_vine))
            .route("/vines/:private_vine_id", get(private_get_vine))
            .route(
                "/vines/:private_vine_id",
                axum::routing::put(private_update_vine),
            )
            .route(
                "/vines/:private_vine_id",
                axum::routing::delete(private_delete_vine),
            )
            .layer(DefaultBodyLimit::max(API_PRIVATE_BODY_LIMIT_BYTES))
            .layer(from_fn_with_state(state.clone(), auth_middleware));
        let canonical_write_routes = Router::new()
            .route(
                "/verifier/grants",
                axum::routing::post(canonical_verifier_grant_writer),
            )
            .route(
                "/verifier/revokes",
                axum::routing::post(canonical_verifier_revoke_writer),
            )
            .route(
                "/identity_create",
                axum::routing::post(canonical_create_identity),
            )
            .route("/ideas", axum::routing::post(canonical_create_idea))
            .route(
                "/connections",
                axum::routing::post(canonical_create_connection),
            )
            .route(
                "/challenges/importance",
                axum::routing::post(canonical_create_importance_challenge),
            )
            .route(
                "/challenges/:challenge_id/arguments",
                axum::routing::post(canonical_attach_importance_argument),
            )
            .route(
                "/vote-sessions/pull",
                axum::routing::post(canonical_pull_vote_session),
            )
            .route(
                "/challenges/:challenge_id/votes",
                axum::routing::post(canonical_cast_challenge_vote),
            )
            .route(
                "/blocked_submission",
                axum::routing::post(canonical_blocked_submission),
            )
            .layer(DefaultBodyLimit::max(API_CANONICAL_BODY_LIMIT_BYTES))
            .layer(from_fn_with_state(state.clone(), auth_middleware));
        let auth_routes = Router::new()
            .route("/register", axum::routing::post(auth_register))
            .route("/login", axum::routing::post(auth_login))
            .route("/logout", axum::routing::post(auth_logout))
            .route("/me", get(auth_me))
            .layer(DefaultBodyLimit::max(API_AUTH_BODY_LIMIT_BYTES));

        app.nest("/api/v0/auth", auth_routes)
            .nest("/api/v0/private", private_routes)
            .nest("/api/v0/me", me_routes)
            .nest("/api/v1/canonical", canonical_write_routes)
    };

    app.layer(DefaultBodyLimit::max(API_GLOBAL_BODY_LIMIT_BYTES))
        .layer(map_response(normalize_error_response))
        .with_state(state)
}
