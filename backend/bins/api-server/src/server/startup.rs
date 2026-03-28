use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use rand_core::OsRng;
use storage::{ensure_session_hmac_key_ready, Storage};
use uuid::Uuid;

use crate::server::helpers::parse_uuid_v7_env;
use crate::server::router::build_app;
use crate::server::types::AppState;

pub(crate) async fn run() {
    tracing_subscriber::fmt::init();

    ensure_session_hmac_key_ready().expect(
        "session security initialization failed: set AUTH_SESSION_HMAC_KEY (required in production)",
    );

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let storage = Storage::new(&database_url)
        .await
        .expect("storage init failed");

    if let Err(err) = maybe_bootstrap_owner(&storage).await {
        tracing::warn!(?err, "owner bootstrap skipped");
    }

    let state = AppState { storage };
    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind listener");

    axum::serve(listener, app).await.expect("server failed");
}

async fn maybe_bootstrap_owner(storage: &Storage) -> Result<(), String> {
    let enabled = std::env::var("SEED_OWNER_BOOTSTRAP").unwrap_or_default();
    if enabled.trim() != "1" {
        tracing::info!("owner bootstrap disabled");
        return Ok(());
    }

    let username = std::env::var("SEED_OWNER_USERNAME").unwrap_or_default();
    let password = std::env::var("SEED_OWNER_PASSWORD").unwrap_or_default();
    if username.trim().is_empty() || password.trim().is_empty() {
        return Err("owner bootstrap missing SEED_OWNER_USERNAME/SEED_OWNER_PASSWORD".to_string());
    }

    if password.trim().len() < 12
        || password.trim() == username.trim()
        || password.trim() == "kind_gulag_dehl"
        || password.trim() == "password"
    {
        return Err("owner bootstrap refused insecure password".to_string());
    }

    let identity_id = match std::env::var("SEED_OWNER_IDENTITY_ID") {
        Ok(value) if !value.trim().is_empty() => parse_uuid_v7_env(value.trim())
            .ok_or_else(|| "SEED_OWNER_IDENTITY_ID must be uuidv7".to_string())?,
        _ => Uuid::parse_str("380b7817-db3b-7b76-8cf3-87df879ddddb")
            .map_err(|_| "default seed identity id invalid".to_string())?,
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.trim().as_bytes(), &salt)
        .map_err(|_| "failed to hash owner password".to_string())?
        .to_string();

    if let Some(account) = storage
        .get_account_by_username(username.trim())
        .await
        .map_err(|err| format!("owner lookup failed: {}", err))?
    {
        storage
            .update_account_credentials(account.account_id, &password_hash, identity_id)
            .await
            .map_err(|err| format!("owner update failed: {}", err))?;
    } else {
        let account = storage
            .create_account_private_only(username.trim(), &password_hash)
            .await
            .map_err(|err| format!("owner create failed: {}", err))?;
        storage
            .update_account_credentials(account.account_id, &password_hash, identity_id)
            .await
            .map_err(|err| format!("owner update failed: {}", err))?;
    }

    tracing::info!(username = username.trim(), "owner bootstrap ready");
    Ok(())
}
