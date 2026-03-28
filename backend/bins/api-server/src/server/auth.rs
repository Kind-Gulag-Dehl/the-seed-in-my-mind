use api_types_private::{AuthMeResponse, AuthTokenResponse};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use rand_core::{OsRng, RngCore};

use crate::server::errors::json_error;
use crate::server::helpers::{bearer_token, validate_password, validate_username};
use crate::server::types::{AppState, AuthPayload, AuthenticatedAccount};

fn generate_session_token() -> String {
    let mut bytes = [0_u8; 32];
    OsRng.fill_bytes(&mut bytes);
    let mut token = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        let _ = write!(&mut token, "{byte:02x}");
    }
    token
}

pub(crate) async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let token = match bearer_token(request.headers()) {
        Some(token) => token,
        None => {
            return json_error(
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "missing bearer token",
            )
        }
    };

    let account = match state.storage.get_account_by_token(&token).await {
        Ok(Some(account)) => account,
        Ok(None) => {
            let _ = state.storage.delete_expired_session(&token).await;
            return json_error(StatusCode::UNAUTHORIZED, "unauthorized", "invalid token");
        }
        Err(err) => {
            tracing::error!(?err, "failed to load auth session");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    if let Err(err) = state.storage.touch_session(&token).await {
        tracing::error!(?err, "failed to touch session");
    }

    request.extensions_mut().insert(AuthenticatedAccount {
        account_id: account.account_id,
        username: account.username,
    });

    next.run(request).await
}

pub(crate) async fn auth_register(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Response {
    let username = match validate_username(&payload.username) {
        Ok(username) => username,
        Err(response) => return response,
    };
    if let Err(response) = validate_password(&payload.password) {
        return response;
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "failed to hash password",
            )
        }
    };

    let account = match state
        .storage
        .create_account_private_only(&username, &password_hash)
        .await
    {
        Ok(account) => account,
        Err(err) => {
            tracing::error!(?err, "failed to create account");
            return json_error(
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "username unavailable",
            );
        }
    };

    let token = generate_session_token();
    let _session = match state
        .storage
        .create_session(account.account_id, &token)
        .await
    {
        Ok(session) => session,
        Err(err) => {
            tracing::error!(?err, "failed to create session");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let identity_title = match account.canonical_identity_id {
        Some(identity_id) => match state.storage.get_identity(identity_id).await {
            Ok(Some(identity)) => Some(identity.title),
            Ok(None) => None,
            Err(err) => {
                tracing::error!(?err, "failed to lookup identity title");
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                );
            }
        },
        None => None,
    };

    let body = AuthTokenResponse {
        account_id: account.account_id.to_string(),
        username: account.username,
        token,
        identity_id: account.canonical_identity_id.map(|value| value.to_string()),
        identity_title,
    };
    (StatusCode::OK, Json(body)).into_response()
}

pub(crate) async fn auth_login(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Response {
    let username = match validate_username(&payload.username) {
        Ok(username) => username,
        Err(response) => return response,
    };
    if let Err(response) = validate_password(&payload.password) {
        return response;
    }

    let account = match state.storage.get_account_by_username(&username).await {
        Ok(Some(account)) => account,
        Ok(None) => {
            return json_error(
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "invalid credentials",
            )
        }
        Err(err) => {
            tracing::error!(?err, "failed to lookup account");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let parsed_hash = match PasswordHash::new(&account.password_hash) {
        Ok(hash) => hash,
        Err(_) => {
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "invalid stored password hash",
            )
        }
    };

    let argon2 = Argon2::default();
    if argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return json_error(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "invalid credentials",
        );
    }

    let token = generate_session_token();
    let _session = match state
        .storage
        .create_session(account.account_id, &token)
        .await
    {
        Ok(session) => session,
        Err(err) => {
            tracing::error!(?err, "failed to create session");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let identity_title = match account.canonical_identity_id {
        Some(identity_id) => match state.storage.get_identity(identity_id).await {
            Ok(Some(identity)) => Some(identity.title),
            Ok(None) => None,
            Err(err) => {
                tracing::error!(?err, "failed to lookup identity title");
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                );
            }
        },
        None => None,
    };

    let body = AuthTokenResponse {
        account_id: account.account_id.to_string(),
        username: account.username,
        token,
        identity_id: account.canonical_identity_id.map(|value| value.to_string()),
        identity_title,
    };
    (StatusCode::OK, Json(body)).into_response()
}

#[cfg(test)]
mod tests {
    use super::generate_session_token;

    #[test]
    fn session_token_generation_is_hex_and_32_bytes() {
        let token = generate_session_token();
        assert_eq!(token.len(), 64);
        assert!(token
            .as_bytes()
            .iter()
            .all(|ch| matches!(ch, b'0'..=b'9' | b'a'..=b'f')));
    }
}

pub(crate) async fn auth_logout(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let token = match bearer_token(&headers) {
        Some(token) => token,
        None => {
            return json_error(
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "missing bearer token",
            )
        }
    };
    let _ = state.storage.delete_session(&token).await;
    (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response()
}

pub(crate) async fn auth_me(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let token = match bearer_token(&headers) {
        Some(token) => token,
        None => {
            return json_error(
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "missing bearer token",
            )
        }
    };
    let account = match state.storage.get_account_by_token(&token).await {
        Ok(Some(account)) => account,
        Ok(None) => return json_error(StatusCode::UNAUTHORIZED, "unauthorized", "invalid token"),
        Err(err) => {
            tracing::error!(?err, "failed to load account");
            return json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "internal server error",
            );
        }
    };

    let identity_title = match account.canonical_identity_id {
        Some(identity_id) => match state.storage.get_identity(identity_id).await {
            Ok(Some(identity)) => Some(identity.title),
            Ok(None) => None,
            Err(err) => {
                tracing::error!(?err, "failed to lookup identity title");
                return json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                );
            }
        },
        None => None,
    };

    let body = AuthMeResponse {
        account_id: account.account_id.to_string(),
        username: account.username,
        identity_id: account.canonical_identity_id.map(|value| value.to_string()),
        identity_title,
    };
    (StatusCode::OK, Json(body)).into_response()
}
