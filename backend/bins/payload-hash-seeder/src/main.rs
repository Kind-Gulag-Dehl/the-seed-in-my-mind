use encoding::payload::payload_hash_hex;
use sqlx::{postgres::PgPoolOptions, FromRow};
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct IdeaPayloadRow {
    idea_id: Uuid,
    created_event_id: Uuid,
    title: Option<String>,
    sentence: Option<String>,
    paragraph: Option<String>,
    full: Option<String>,
    payload_hash: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let validate_only = std::env::args().any(|arg| arg == "--validate");
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    let rows: Vec<IdeaPayloadRow> = sqlx::query_as(
        r#"
        SELECT
          i.idea_id,
          e.event_id AS created_event_id,
          e.payload_json->>'title' AS title,
          e.payload_json->>'sentence' AS sentence,
          e.payload_json->>'paragraph' AS paragraph,
          e.payload_json->>'full' AS full,
          e.payload_json->>'payload_hash' AS payload_hash
        FROM ideas i
        JOIN events e ON e.event_id = i.created_event_id
        ORDER BY i.created_block_height ASC, i.created_event_index ASC
        "#,
    )
    .fetch_all(&pool)
    .await?;

    let mut updated = 0usize;
    let mut missing = Vec::new();
    let mut mismatched = Vec::new();

    for row in &rows {
        let idea_id = row.idea_id.to_string();
        let title = row
            .title
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("missing title for idea_id={idea_id}"))?;
        let sentence = row
            .sentence
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("missing sentence for idea_id={idea_id}"))?;

        let expected = payload_hash_hex(
            title,
            sentence,
            row.paragraph.as_deref(),
            row.full.as_deref(),
        )
        .map_err(|err| anyhow::anyhow!(err))?;

        let needs_update = match row.payload_hash.as_deref() {
            Some(value) if !value.trim().is_empty() => value != expected,
            _ => true,
        };

        if needs_update {
            if validate_only {
                match row.payload_hash.as_deref() {
                    Some(value) if !value.trim().is_empty() => mismatched.push(row.idea_id),
                    _ => missing.push(row.idea_id),
                }
            } else {
                sqlx::query(
                    r#"
                    UPDATE events
                    SET payload_json = jsonb_set(payload_json, '{payload_hash}', to_jsonb($2::text), true)
                    WHERE event_id = $1
                    "#,
                )
                .bind(row.created_event_id)
                .bind(&expected)
                .execute(&pool)
                .await?;
                updated += 1;
            }
        }
    }

    if validate_only && (!missing.is_empty() || !mismatched.is_empty()) {
        let missing_sample: Vec<String> =
            missing.iter().take(10).map(|id| id.to_string()).collect();
        let mismatch_sample: Vec<String> = mismatched
            .iter()
            .take(10)
            .map(|id| id.to_string())
            .collect();
        return Err(anyhow::anyhow!(
            "payload_hash validation failed: missing_count={} missing_sample=[{}]; mismatch_count={} mismatch_sample=[{}]",
            missing.len(),
            missing_sample.join(", "),
            mismatched.len(),
            mismatch_sample.join(", ")
        ));
    }

    if !validate_only {
        let missing_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)::bigint
            FROM ideas i
            JOIN events e ON e.event_id = i.created_event_id
            WHERE (e.payload_json->>'payload_hash' IS NULL OR e.payload_json->>'payload_hash' = '')
            "#,
        )
        .fetch_one(&pool)
        .await?;

        if missing_count > 0 {
            let missing_ids: Vec<Uuid> = sqlx::query_scalar(
                r#"
                SELECT i.idea_id
                FROM ideas i
                JOIN events e ON e.event_id = i.created_event_id
                WHERE (e.payload_json->>'payload_hash' IS NULL OR e.payload_json->>'payload_hash' = '')
                ORDER BY i.created_block_height ASC, i.created_event_index ASC, i.idea_id ASC
                LIMIT 10
                "#,
            )
            .fetch_all(&pool)
            .await?;

            let sample: Vec<String> = missing_ids.iter().map(|id| id.to_string()).collect();
            return Err(anyhow::anyhow!(
                "missing payload_hash for {} ideas; sample idea_ids: {}",
                missing_count,
                sample.join(", ")
            ));
        }
    }

    if validate_only {
        println!("payload-hash-seeder: validated={}", rows.len());
    } else {
        println!(
            "payload-hash-seeder: updated={} validated={}",
            updated,
            rows.len()
        );
    }
    Ok(())
}
