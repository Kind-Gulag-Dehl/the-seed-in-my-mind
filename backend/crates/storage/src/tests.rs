use super::*;
use anyhow::anyhow;
use common::test_db_guard::require_disposable_database_url;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

fn guarded_database_url() -> Option<String> {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) if !url.trim().is_empty() => url,
        _ => return None,
    };
    match require_disposable_database_url(&database_url) {
        Ok(database_name) => {
            eprintln!("TEST_DB: {database_name} differs_from_seed_dev=true");
            Some(database_url)
        }
        Err(err) => {
            eprintln!("SKIP: DATABASE_URL rejected by test DB guard: {err}");
            None
        }
    }
}

fn load_seed_identity_id() -> Option<Uuid> {
    for key in ["SEED_OWNER_IDENTITY_ID", "seed_owner_identity_id"] {
        if let Ok(value) = std::env::var(key) {
            if let Ok(parsed) = Uuid::parse_str(value.trim()) {
                return Some(parsed);
            }
        }
    }
    Uuid::parse_str("380b7817-db3b-7b76-8cf3-87df879ddddb").ok()
}

#[tokio::test]
async fn seed_identity_detail_and_neighborhood_queries_do_not_error() -> Result<()> {
    let Some(database_url) = guarded_database_url() else {
        return Ok(());
    };
    let identity_id = match load_seed_identity_id() {
        Some(value) => value,
        None => return Ok(()),
    };

    let storage = Storage::new(&database_url).await?;
    let snapshot = match storage.get_latest_snapshot().await? {
        Some(snapshot) => snapshot,
        None => return Ok(()),
    };

    let detail = match storage
        .get_idea_detail(snapshot.block_height, identity_id)
        .await?
    {
        Some(detail) => detail,
        None => return Ok(()),
    };
    assert_eq!(detail.idea_id, identity_id);

    let _ = storage
        .list_connections_for_idea(snapshot.block_height, identity_id)
        .await?;
    let _ = storage
        .list_connections_for_ideas(snapshot.block_height, &[identity_id])
        .await?;

    Ok(())
}

#[tokio::test]
async fn organizer_ideas_are_rankless_and_normal_ideas_are_ranked() -> Result<()> {
    let Some(database_url) = guarded_database_url() else {
        return Ok(());
    };

    let storage = Storage::new(&database_url).await?;
    let snapshot = match storage.get_latest_snapshot().await? {
        Some(snapshot) => snapshot,
        None => return Ok(()),
    };

    let normal_idea_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT idea_id
        FROM ideas
        WHERE created_block_height <= $1
          AND is_personal_space_organizer = false
          AND is_identity_idea = false
        ORDER BY created_block_height ASC, created_event_index ASC
        LIMIT 1
        "#,
    )
    .bind(snapshot.block_height)
    .fetch_optional(&storage.pool)
    .await?;

    let organizer_rows = sqlx::query_as::<_, (Uuid, String)>(
        r#"
        SELECT
          i.idea_id,
          COALESCE(e.payload_json->>'title', '') AS title
        FROM ideas i
        LEFT JOIN events e ON e.event_id = i.created_event_id
        WHERE i.created_block_height <= $1
          AND i.is_personal_space_organizer = true
        ORDER BY i.created_block_height ASC, i.created_event_index ASC
        "#,
    )
    .bind(snapshot.block_height)
    .fetch_all(&storage.pool)
    .await?;

    let normal_idea_id = match normal_idea_id {
        Some(normal_idea_id) if !organizer_rows.is_empty() => normal_idea_id,
        _ => return Ok(()),
    };
    let organizer_ids: Vec<Uuid> = organizer_rows.iter().map(|(idea_id, _)| *idea_id).collect();
    let organizer_set: HashSet<Uuid> = organizer_ids.iter().cloned().collect();

    let normal_detail = storage
        .get_idea_detail(snapshot.block_height, normal_idea_id)
        .await?
        .ok_or_else(|| anyhow!("normal idea missing"))?;
    assert!(
        normal_detail.derived_universal_rank.is_some(),
        "normal idea should have derived_universal_rank"
    );

    for organizer_id in &organizer_ids {
        let organizer_detail = storage
            .get_idea_detail(snapshot.block_height, *organizer_id)
            .await?
            .ok_or_else(|| anyhow!("organizer idea missing"))?;
        assert_eq!(
            organizer_detail.derived_universal_rank, None,
            "organizer ideas must not receive derived_universal_rank in detail path"
        );
    }

    let organizer_summaries = storage
        .list_ideas_by_ids(snapshot.block_height, &organizer_ids)
        .await?
        .into_iter()
        .filter(|idea| organizer_set.contains(&idea.idea_id))
        .collect::<Vec<_>>();
    for organizer in organizer_summaries {
        assert_eq!(
            organizer.derived_universal_rank, None,
            "organizer ideas must not receive derived_universal_rank in list_by_ids path"
        );
    }

    let total = storage.count_ideas(snapshot.block_height).await?;
    let top = storage
        .list_ideas_top(snapshot.block_height, 0, total.saturating_add(10), false)
        .await?;
    assert!(
        top.iter()
            .all(|idea| !organizer_set.contains(&idea.idea_id)),
        "organizer ideas must not appear in /ideas/top rank path"
    );

    for (_, organizer_title) in organizer_rows {
        if organizer_title.trim().is_empty() {
            continue;
        }
        let search_results = storage
            .search_ideas(snapshot.block_height, &organizer_title, 0, 100)
            .await?;
        for organizer in search_results
            .into_iter()
            .filter(|idea| organizer_set.contains(&idea.idea_id))
        {
            assert_eq!(
                organizer.derived_universal_rank, None,
                "organizer ideas must not receive derived_universal_rank in search path"
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn private_orderings_crud_and_owner_isolation() -> Result<()> {
    let Some(database_url) = guarded_database_url() else {
        return Ok(());
    };

    let storage = Storage::new(&database_url).await?;
    let has_private_orderings: Option<String> =
        sqlx::query_scalar("SELECT to_regclass('public.private_orderings')::text")
            .fetch_one(&storage.pool)
            .await?;
    if has_private_orderings.is_none() {
        return Ok(());
    }

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| anyhow!("clock error: {}", err))?
        .as_nanos();
    let owner_a = storage
        .create_account_private_only(&format!("vine_owner_a_{nonce}"), "hash")
        .await?;
    let owner_b = storage
        .create_account_private_only(&format!("vine_owner_b_{nonce}"), "hash")
        .await?;

    let created = storage
        .create_private_ordering(
            owner_a.account_id,
            0,
            Some(0),
            Some("owner-a vine"),
            Some("owner-a sentence"),
            None,
            None,
            &[PrivateOrderingItemInput {
                idx: 0,
                idea_id: Uuid::new_v4(),
                via_connection_id: None,
            }],
        )
        .await?;

    let listed = storage.list_private_orderings(owner_a.account_id).await?;
    assert!(listed
        .iter()
        .any(|row| row.private_ordering_id == created.private_ordering_id));

    let detail = storage
        .get_private_ordering(owner_a.account_id, created.private_ordering_id)
        .await?
        .ok_or_else(|| anyhow!("expected vine detail for owner"))?;
    assert_eq!(detail.private_ordering_id, created.private_ordering_id);

    let items = storage
        .list_private_ordering_items(owner_a.account_id, created.private_ordering_id)
        .await?;
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].idx, 0);

    let not_owner_detail = storage
        .get_private_ordering(owner_b.account_id, created.private_ordering_id)
        .await?;
    assert!(not_owner_detail.is_none());

    let not_owner_update = storage
        .update_private_ordering(
            owner_b.account_id,
            created.private_ordering_id,
            0,
            Some(1),
            Some("should fail"),
            Some("should fail"),
            None,
            None,
            None,
        )
        .await?;
    assert!(not_owner_update.is_none());

    let updated = storage
        .update_private_ordering(
            owner_a.account_id,
            created.private_ordering_id,
            0,
            Some(1),
            Some("updated"),
            Some("updated sentence"),
            Some("updated paragraph"),
            None,
            Some(&[
                PrivateOrderingItemInput {
                    idx: 0,
                    idea_id: Uuid::new_v4(),
                    via_connection_id: None,
                },
                PrivateOrderingItemInput {
                    idx: 1,
                    idea_id: Uuid::new_v4(),
                    via_connection_id: Some(Uuid::new_v4()),
                },
            ]),
        )
        .await?
        .ok_or_else(|| anyhow!("expected owner update to succeed"))?;
    assert_eq!(updated.ordering_profile, 0);
    assert_eq!(updated.vine_type, Some(1));
    assert_eq!(updated.title.as_deref(), Some("updated"));

    let updated_items = storage
        .list_private_ordering_items(owner_a.account_id, created.private_ordering_id)
        .await?;
    assert_eq!(updated_items.len(), 2);
    assert_eq!(updated_items[0].idx, 0);
    assert_eq!(updated_items[1].idx, 1);

    let deleted_by_other = storage
        .delete_private_ordering(owner_b.account_id, created.private_ordering_id)
        .await?;
    assert_eq!(deleted_by_other, 0);

    let deleted_by_owner = storage
        .delete_private_ordering(owner_a.account_id, created.private_ordering_id)
        .await?;
    assert_eq!(deleted_by_owner, 1);

    let _ = sqlx::query("DELETE FROM accounts WHERE account_id = $1")
        .bind(owner_a.account_id)
        .execute(&storage.pool)
        .await;
    let _ = sqlx::query("DELETE FROM accounts WHERE account_id = $1")
        .bind(owner_b.account_id)
        .execute(&storage.pool)
        .await;

    Ok(())
}
