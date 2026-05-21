use super::*;
use hmac::{Hmac, Mac};
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use std::sync::OnceLock;

const SESSION_TOKEN_HMAC_DOMAIN: &[u8] = b"seed.auth.session.token.v1";
const SESSION_HMAC_KEY_BYTES: usize = 32;
const SESSION_HMAC_KEY_ENV_KEYS: [&str; 2] = ["AUTH_SESSION_HMAC_KEY", "SESSION_TOKEN_HMAC_KEY"];
const SESSION_HMAC_MODE_ENV_KEYS: [&str; 3] = ["SEED_ENV", "APP_ENV", "RUST_ENV"];
static SESSION_TOKEN_HMAC_KEY: OnceLock<Vec<u8>> = OnceLock::new();
type HmacSha256 = Hmac<Sha256>;

fn is_production_mode() -> bool {
    SESSION_HMAC_MODE_ENV_KEYS.iter().any(|key_name| {
        if let Ok(value) = std::env::var(key_name) {
            return matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "prod" | "production"
            );
        }
        false
    })
}

fn compute_session_token_hash_key() -> Result<Vec<u8>> {
    for key_name in SESSION_HMAC_KEY_ENV_KEYS {
        if let Ok(value) = std::env::var(key_name) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Ok(trimmed.as_bytes().to_vec());
            }
        }
    }

    if is_production_mode() {
        return Err(anyhow!(
            "AUTH_SESSION_HMAC_KEY is required in production mode"
        ));
    }

    let mut key = vec![0_u8; SESSION_HMAC_KEY_BYTES];
    OsRng.fill_bytes(&mut key);
    eprintln!(
        "[security] AUTH_SESSION_HMAC_KEY missing; using ephemeral dev-only session HMAC key"
    );
    Ok(key)
}

fn resolve_session_token_hash_key() -> Result<&'static [u8]> {
    if let Some(key) = SESSION_TOKEN_HMAC_KEY.get() {
        return Ok(key.as_slice());
    }

    let computed = compute_session_token_hash_key()?;
    let _ = SESSION_TOKEN_HMAC_KEY.set(computed);
    let key = SESSION_TOKEN_HMAC_KEY
        .get()
        .ok_or_else(|| anyhow!("session token HMAC key initialization failed"))?;
    Ok(key.as_slice())
}

pub fn ensure_session_hmac_key_ready() -> Result<()> {
    let _ = resolve_session_token_hash_key()?;
    Ok(())
}

fn bytes_to_hex(value: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(value.len() * 2);
    for byte in value {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn hmac_sha256_hex(key: &[u8], message: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC-SHA256 accepts arbitrary key size");
    mac.update(message);
    let digest = mac.finalize().into_bytes();
    bytes_to_hex(&digest)
}

fn hash_session_token_with_key(token: &str, key: &[u8]) -> Result<String> {
    let token = token.trim();
    if token.is_empty() {
        return Err(anyhow!("session token required"));
    }

    let mut message = Vec::with_capacity(SESSION_TOKEN_HMAC_DOMAIN.len() + 1 + token.len());
    message.extend_from_slice(SESSION_TOKEN_HMAC_DOMAIN);
    message.push(0);
    message.extend_from_slice(token.as_bytes());
    Ok(hmac_sha256_hex(key, &message))
}

fn hash_session_token(token: &str) -> Result<String> {
    let key = resolve_session_token_hash_key()?;
    hash_session_token_with_key(token, key)
}

impl Storage {
    pub async fn create_account_private_only(
        &self,
        username: &str,
        password_hash: &str,
    ) -> Result<AccountRow> {
        let mut tx = self.pool.begin().await?;
        let account_id = Uuid::new_v4();
        let row = sqlx::query_as::<_, AccountRow>(
            r#"
            INSERT INTO accounts (account_id, username, password_hash)
            VALUES ($1, $2, $3)
            RETURNING account_id, username, password_hash, created_at, canonical_identity_id
            "#,
        )
        .bind(account_id)
        .bind(username)
        .bind(password_hash)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(row)
    }

    pub async fn create_account(&self, username: &str, password_hash: &str) -> Result<AccountRow> {
        self.create_account_private_only(username, password_hash)
            .await
    }

    pub async fn create_account_with_canonical_cluster(
        &self,
        username: &str,
        password_hash: &str,
        display_name: &str,
    ) -> Result<AccountRow> {
        let normalized_display_name = display_name.trim();
        if normalized_display_name.is_empty() {
            return Err(anyhow!("display_name required"));
        }

        let mut tx = self.pool.begin().await?;

        let target_block_height: i64 = sqlx::query_scalar(
            r#"
            SELECT block_height
            FROM snapshots
            ORDER BY block_height DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&mut *tx)
        .await?
        .unwrap_or(1);

        let block_hash = format!("{:x}", target_block_height.max(0));
        let prev_block_hash = if target_block_height > 0 {
            Some(format!("{:x}", target_block_height - 1))
        } else {
            None
        };

        sqlx::query(
            r#"
            INSERT INTO blocks (block_height, block_hash, prev_block_hash)
            VALUES ($1, $2, $3)
            ON CONFLICT (block_height) DO NOTHING
            "#,
        )
        .bind(target_block_height)
        .bind(block_hash)
        .bind(prev_block_hash)
        .execute(&mut *tx)
        .await?;

        let _locked_block_height: i64 = sqlx::query_scalar(
            "SELECT block_height FROM blocks WHERE block_height = $1 FOR UPDATE",
        )
        .bind(target_block_height)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| anyhow!("missing canonical block_height {}", target_block_height))?;

        let current_max_event_index: i32 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(event_index), -1)::int FROM events WHERE block_height = $1",
        )
        .bind(target_block_height)
        .fetch_one(&mut *tx)
        .await?;
        let mut next_event_index = current_max_event_index + 1;

        let account_id = Uuid::new_v4();
        let identity_id = Uuid::now_v7();

        let account = sqlx::query_as::<_, AccountRow>(
            r#"
            INSERT INTO accounts (account_id, username, password_hash, canonical_identity_id)
            VALUES ($1, $2, $3, $4)
            RETURNING account_id, username, password_hash, created_at, canonical_identity_id
            "#,
        )
        .bind(account_id)
        .bind(username)
        .bind(password_hash)
        .bind(identity_id)
        .fetch_one(&mut *tx)
        .await?;

        let identity_event_id = Uuid::now_v7();
        let identity_payload_hash =
            payload_hash_hex(normalized_display_name, normalized_display_name, None, None)
                .map_err(|err| anyhow!(err))?;
        let identity_payload = json!({
            "idea_id": identity_id,
            "idea_type": "identity",
            "speaker_identity_id": identity_id,
            "title": normalized_display_name,
            "sentence": normalized_display_name,
            "paragraph": null,
            "full": null,
            "payload_hash": identity_payload_hash
        });

        let identity_event_index = next_event_index;
        next_event_index += 1;

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
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(target_block_height)
        .bind(identity_event_index)
        .bind(identity_event_id)
        .bind("idea_create")
        .bind(Some(identity_id))
        .bind(identity_payload)
        .bind::<Option<String>>(None)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO ideas (
              idea_id,
              idea_type,
              speaker_identity_id,
              is_identity_idea,
              underlying_identity_id,
              created_block_height,
              created_event_index,
              created_event_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(identity_id)
        .bind("identity")
        .bind(identity_id)
        .bind(true)
        .bind(Some(identity_id))
        .bind(target_block_height)
        .bind(identity_event_index)
        .bind(identity_event_id)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO identities_s0 (
              identity_id,
              title,
              created_event_id
            ) VALUES ($1, $2, $3)
            ON CONFLICT (identity_id) DO UPDATE SET
              title = EXCLUDED.title,
              created_event_id = EXCLUDED.created_event_id
            "#,
        )
        .bind(identity_id)
        .bind(normalized_display_name)
        .bind(identity_event_id)
        .execute(&mut *tx)
        .await?;

        let organizer_titles = [
            format!("{}'s Mind Garden", normalized_display_name),
            format!("{}'s Backyard of Relationships", normalized_display_name),
            format!("{}'s Self Tree", normalized_display_name),
            format!("{}'s Anthill", normalized_display_name),
            format!("{}'s Saved Ideas", normalized_display_name),
        ];

        let mut organizer_ids = Vec::with_capacity(organizer_titles.len());

        for organizer_title in organizer_titles {
            let organizer_idea_id = Uuid::now_v7();
            let organizer_event_id = Uuid::now_v7();
            let organizer_payload_hash =
                payload_hash_hex(&organizer_title, &organizer_title, None, None)
                    .map_err(|err| anyhow!(err))?;
            let organizer_payload = json!({
                "idea_id": organizer_idea_id,
                "idea_type": "conceptual_idea",
                "speaker_identity_id": identity_id,
                "title": organizer_title,
                "sentence": organizer_title,
                "paragraph": null,
                "full": null,
                "payload_hash": organizer_payload_hash
            });

            let organizer_event_index = next_event_index;
            next_event_index += 1;

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
                ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(target_block_height)
            .bind(organizer_event_index)
            .bind(organizer_event_id)
            .bind("idea_create")
            .bind(Some(identity_id))
            .bind(organizer_payload)
            .bind::<Option<String>>(None)
            .execute(&mut *tx)
            .await?;

            sqlx::query(
                r#"
                INSERT INTO ideas (
                  idea_id,
                  idea_type,
                  speaker_identity_id,
                  is_identity_idea,
                  underlying_identity_id,
                  is_personal_space_organizer,
                  created_block_height,
                  created_event_index,
                  created_event_id
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                "#,
            )
            .bind(organizer_idea_id)
            .bind("conceptual_idea")
            .bind(identity_id)
            .bind(false)
            .bind::<Option<Uuid>>(None)
            .bind(true)
            .bind(target_block_height)
            .bind(organizer_event_index)
            .bind(organizer_event_id)
            .execute(&mut *tx)
            .await?;

            organizer_ids.push(organizer_idea_id);
        }

        for organizer_idea_id in organizer_ids {
            for (from_idea_id, to_idea_id, usage) in [
                (identity_id, organizer_idea_id, Some("has_space")),
                (organizer_idea_id, identity_id, Some("space_of")),
            ] {
                let connection_id = Uuid::now_v7();
                let connection_event_id = Uuid::now_v7();
                let connection_payload = json!({
                    "connection_id": connection_id,
                    "from_idea_id": from_idea_id,
                    "to_idea_id": to_idea_id,
                    "connection_type": "membership",
                    "usage": usage
                });
                let connection_event_index = next_event_index;
                next_event_index += 1;

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
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#,
                )
                .bind(target_block_height)
                .bind(connection_event_index)
                .bind(connection_event_id)
                .bind("connection_create")
                .bind(Some(identity_id))
                .bind(connection_payload)
                .bind::<Option<String>>(None)
                .execute(&mut *tx)
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
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                    "#,
                )
                .bind(connection_id)
                .bind(from_idea_id)
                .bind(to_idea_id)
                .bind("membership")
                .bind(usage)
                .bind::<Option<&str>>(None)
                .bind::<Option<&str>>(None)
                .bind::<Option<&str>>(None)
                .bind(target_block_height)
                .bind(connection_event_index)
                .bind(connection_event_id)
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        Ok(account)
    }

    pub async fn create_account_with_identity_cluster(
        &self,
        username: &str,
        password_hash: &str,
        display_name: &str,
    ) -> Result<AccountRow> {
        self.create_account_with_canonical_cluster(username, password_hash, display_name)
            .await
    }

    pub async fn get_account_by_username(&self, username: &str) -> Result<Option<AccountRow>> {
        let row = sqlx::query_as::<_, AccountRow>(
            r#"
            SELECT account_id, username, password_hash, created_at, canonical_identity_id
            FROM accounts
            WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn create_session(&self, account_id: Uuid, token: &str) -> Result<SessionRow> {
        let token_hash = hash_session_token(token)?;
        let session_id = Uuid::new_v4();
        let row = sqlx::query_as::<_, SessionRow>(
            r#"
            INSERT INTO auth_sessions (session_id, account_id, token_hash, expires_at)
            VALUES ($1, $2, $3, NOW() + interval '30 days')
            RETURNING session_id, account_id, token_hash AS token, created_at, last_seen_at, expires_at
            "#,
        )
        .bind(session_id)
        .bind(account_id)
        .bind(token_hash)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn delete_session(&self, token: &str) -> Result<u64> {
        let token_hash = hash_session_token(token)?;
        let result = sqlx::query("DELETE FROM auth_sessions WHERE token_hash = $1")
            .bind(token_hash)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn get_account_by_token(&self, token: &str) -> Result<Option<AccountRow>> {
        let token_hash = hash_session_token(token)?;
        let row = sqlx::query_as::<_, AccountRow>(
            r#"
            SELECT a.account_id, a.username, a.password_hash, a.created_at, a.canonical_identity_id
            FROM accounts a
            JOIN auth_sessions s ON s.account_id = a.account_id
            WHERE s.token_hash = $1 AND s.expires_at > NOW()
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn update_account_credentials(
        &self,
        account_id: Uuid,
        password_hash: &str,
        canonical_identity_id: Uuid,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE accounts
            SET password_hash = $2,
                canonical_identity_id = $3
            WHERE account_id = $1
            "#,
        )
        .bind(account_id)
        .bind(password_hash)
        .bind(canonical_identity_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn touch_session(&self, token: &str) -> Result<()> {
        let token_hash = hash_session_token(token)?;
        sqlx::query("UPDATE auth_sessions SET last_seen_at = NOW() WHERE token_hash = $1")
            .bind(token_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_expired_session(&self, token: &str) -> Result<u64> {
        let token_hash = hash_session_token(token)?;
        let result =
            sqlx::query("DELETE FROM auth_sessions WHERE token_hash = $1 AND expires_at <= NOW()")
                .bind(token_hash)
                .execute(&self.pool)
                .await?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn session_token_hash_is_stable_and_not_plaintext() {
        let key = b"unit-test-session-hmac-key";
        let token = "session_token_example";
        let first = hash_session_token_with_key(token, key).expect("first hash");
        let second = hash_session_token_with_key(token, key).expect("second hash");

        assert_eq!(first, second);
        assert_ne!(first, token);
        assert_eq!(first.len(), 64);
        assert!(first
            .as_bytes()
            .iter()
            .all(|ch| matches!(ch, b'0'..=b'9' | b'a'..=b'f')));
    }

    #[test]
    fn session_token_hash_changes_with_token_or_key() {
        let key_a = b"unit-test-session-hmac-key-a";
        let key_b = b"unit-test-session-hmac-key-b";
        let token_a = "session_token_a";
        let token_b = "session_token_b";

        let a = hash_session_token_with_key(token_a, key_a).expect("hash a");
        let b = hash_session_token_with_key(token_b, key_a).expect("hash b");
        let c = hash_session_token_with_key(token_a, key_b).expect("hash c");

        assert_ne!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn session_token_hash_rejects_empty_token() {
        let key = b"unit-test-session-hmac-key";
        assert!(hash_session_token_with_key("   ", key).is_err());
    }

    #[tokio::test]
    async fn session_lookup_uses_hashed_storage_only() -> Result<()> {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) if !url.trim().is_empty() => url,
            _ => return Ok(()),
        };

        let storage = Storage::new(&database_url).await?;
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|err| anyhow!("clock error: {}", err))?
            .as_nanos();
        let username = format!("session_hash_test_{nonce}");
        let account = storage
            .create_account_private_only(&username, "hash")
            .await?;
        let raw_token = format!("session-token-{nonce}-abcdef0123456789");

        storage
            .create_session(account.account_id, &raw_token)
            .await?;
        let stored_hash: String = sqlx::query_scalar(
            "SELECT token_hash FROM auth_sessions WHERE account_id = $1 ORDER BY created_at DESC LIMIT 1",
        )
        .bind(account.account_id)
        .fetch_one(storage.pool())
        .await?;

        assert_ne!(stored_hash, raw_token);
        assert_eq!(stored_hash.len(), 64);

        let plaintext_rows: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM auth_sessions WHERE token_hash = $1")
                .bind(raw_token.as_str())
                .fetch_one(storage.pool())
                .await?;
        assert_eq!(plaintext_rows, 0);

        let found = storage.get_account_by_token(&raw_token).await?;
        assert!(found.is_some());

        let _ = sqlx::query("DELETE FROM accounts WHERE account_id = $1")
            .bind(account.account_id)
            .execute(storage.pool())
            .await;
        Ok(())
    }
}
