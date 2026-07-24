use super::*;

impl Storage {
    pub async fn create_private_idea(
        &self,
        owner_account_id: Uuid,
        title: &str,
        sentence: &str,
        paragraph: Option<&str>,
        full: Option<&str>,
    ) -> Result<PrivateIdeaRow> {
        let idea_id = Uuid::new_v4();
        let row = sqlx::query_as::<_, PrivateIdeaRow>(
            r#"
            INSERT INTO private_ideas (
              idea_id, owner_account_id, title, sentence, paragraph, "full", created_at, updated_at
            ) VALUES (
              $1, $2, $3, $4, $5, $6, NOW(), NOW()
            )
            RETURNING idea_id, owner_account_id, title, sentence, paragraph, "full", created_at, updated_at
            "#,
        )
        .bind(idea_id)
        .bind(owner_account_id)
        .bind(title)
        .bind(sentence)
        .bind(paragraph)
        .bind(full)
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn list_private_ideas(&self, owner_account_id: Uuid) -> Result<Vec<PrivateIdeaRow>> {
        let rows = sqlx::query_as::<_, PrivateIdeaRow>(
            r#"
            SELECT idea_id, owner_account_id, title, sentence, paragraph, "full", created_at, updated_at
            FROM private_ideas
            WHERE owner_account_id = $1
            ORDER BY updated_at DESC, idea_id ASC
            "#,
        )
        .bind(owner_account_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_private_idea(
        &self,
        owner_account_id: Uuid,
        idea_id: Uuid,
    ) -> Result<Option<PrivateIdeaRow>> {
        let row = sqlx::query_as::<_, PrivateIdeaRow>(
            r#"
            SELECT idea_id, owner_account_id, title, sentence, paragraph, "full", created_at, updated_at
            FROM private_ideas
            WHERE owner_account_id = $1 AND idea_id = $2
            "#,
        )
        .bind(owner_account_id)
        .bind(idea_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn update_private_idea(
        &self,
        owner_account_id: Uuid,
        idea_id: Uuid,
        title: &str,
        sentence: &str,
        paragraph: Option<&str>,
        full: Option<&str>,
    ) -> Result<Option<PrivateIdeaRow>> {
        let row = sqlx::query_as::<_, PrivateIdeaRow>(
            r#"
            UPDATE private_ideas
            SET title = $3,
                sentence = $4,
                paragraph = $5,
                "full" = $6,
                updated_at = NOW()
            WHERE owner_account_id = $1 AND idea_id = $2
            RETURNING idea_id, owner_account_id, title, sentence, paragraph, "full", created_at, updated_at
            "#,
        )
        .bind(owner_account_id)
        .bind(idea_id)
        .bind(title)
        .bind(sentence)
        .bind(paragraph)
        .bind(full)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn delete_private_idea(&self, owner_account_id: Uuid, idea_id: Uuid) -> Result<u64> {
        let result =
            sqlx::query("DELETE FROM private_ideas WHERE owner_account_id = $1 AND idea_id = $2")
                .bind(owner_account_id)
                .bind(idea_id)
                .execute(&self.pool)
                .await?;
        Ok(result.rows_affected())
    }

    pub async fn create_private_ordering(
        &self,
        owner_account_id: Uuid,
        ordering_profile: i16,
        vine_type: Option<i16>,
        title: Option<&str>,
        sentence: Option<&str>,
        paragraph: Option<&str>,
        full: Option<&str>,
        items: &[PrivateOrderingItemInput],
    ) -> Result<PrivateOrderingRow> {
        for (expected_idx, item) in items.iter().enumerate() {
            if item.idx != expected_idx as i32 {
                return Err(anyhow!(
                    "private ordering item idx mismatch expected={} actual={}",
                    expected_idx,
                    item.idx
                ));
            }
        }

        let mut tx = self.pool.begin().await?;
        let private_ordering_id = Uuid::new_v4();
        let row = sqlx::query_as::<_, PrivateOrderingRow>(
            r#"
            INSERT INTO private_orderings (
              private_ordering_id,
              owner_account_id,
              ordering_profile,
              vine_type,
              title,
              sentence,
              paragraph,
              "full",
              created_at,
              updated_at
            ) VALUES (
              $1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW()
            )
            RETURNING
              private_ordering_id,
              owner_account_id,
              ordering_profile,
              vine_type,
              title,
              sentence,
              paragraph,
              "full",
              created_at,
              updated_at
            "#,
        )
        .bind(private_ordering_id)
        .bind(owner_account_id)
        .bind(ordering_profile)
        .bind(vine_type)
        .bind(title)
        .bind(sentence)
        .bind(paragraph)
        .bind(full)
        .fetch_one(&mut *tx)
        .await?;

        replace_private_ordering_items(&mut tx, private_ordering_id, items).await?;
        tx.commit().await?;
        Ok(row)
    }

    pub async fn list_private_orderings(
        &self,
        owner_account_id: Uuid,
    ) -> Result<Vec<PrivateOrderingListRow>> {
        let rows = sqlx::query_as::<_, PrivateOrderingListRow>(
            r#"
            SELECT
              v.private_ordering_id,
              v.ordering_profile,
              v.vine_type,
              v.title,
              v.sentence,
              v.updated_at,
              COALESCE(COUNT(i.private_ordering_id), 0)::bigint AS item_count
            FROM private_orderings v
            LEFT JOIN private_ordering_items i ON i.private_ordering_id = v.private_ordering_id
            WHERE v.owner_account_id = $1
            GROUP BY v.private_ordering_id, v.ordering_profile, v.vine_type, v.title, v.sentence, v.updated_at
            ORDER BY v.updated_at DESC, v.private_ordering_id ASC
            "#,
        )
        .bind(owner_account_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_private_ordering(
        &self,
        owner_account_id: Uuid,
        private_ordering_id: Uuid,
    ) -> Result<Option<PrivateOrderingRow>> {
        let row = sqlx::query_as::<_, PrivateOrderingRow>(
            r#"
            SELECT
              private_ordering_id,
              owner_account_id,
              ordering_profile,
              vine_type,
              title,
              sentence,
              paragraph,
              "full",
              created_at,
              updated_at
            FROM private_orderings
            WHERE owner_account_id = $1
              AND private_ordering_id = $2
            "#,
        )
        .bind(owner_account_id)
        .bind(private_ordering_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn list_private_ordering_items(
        &self,
        owner_account_id: Uuid,
        private_ordering_id: Uuid,
    ) -> Result<Vec<PrivateOrderingItemRow>> {
        let rows = sqlx::query_as::<_, PrivateOrderingItemRow>(
            r#"
            SELECT
              i.private_ordering_id,
              i.idx,
              i.idea_id,
              i.via_connection_id
            FROM private_ordering_items i
            JOIN private_orderings v ON v.private_ordering_id = i.private_ordering_id
            WHERE v.owner_account_id = $1
              AND i.private_ordering_id = $2
            ORDER BY i.idx ASC
            "#,
        )
        .bind(owner_account_id)
        .bind(private_ordering_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn update_private_ordering(
        &self,
        owner_account_id: Uuid,
        private_ordering_id: Uuid,
        ordering_profile: i16,
        vine_type: Option<i16>,
        title: Option<&str>,
        sentence: Option<&str>,
        paragraph: Option<&str>,
        full: Option<&str>,
        items: Option<&[PrivateOrderingItemInput]>,
    ) -> Result<Option<PrivateOrderingRow>> {
        if let Some(items) = items {
            for (expected_idx, item) in items.iter().enumerate() {
                if item.idx != expected_idx as i32 {
                    return Err(anyhow!(
                        "private ordering item idx mismatch expected={} actual={}",
                        expected_idx,
                        item.idx
                    ));
                }
            }
        }

        let mut tx = self.pool.begin().await?;
        let row = sqlx::query_as::<_, PrivateOrderingRow>(
            r#"
            UPDATE private_orderings
            SET ordering_profile = $3,
                vine_type = $4,
                title = $5,
                sentence = $6,
                paragraph = $7,
                "full" = $8,
                updated_at = NOW()
            WHERE owner_account_id = $1
              AND private_ordering_id = $2
            RETURNING
              private_ordering_id,
              owner_account_id,
              ordering_profile,
              vine_type,
              title,
              sentence,
              paragraph,
              "full",
              created_at,
              updated_at
            "#,
        )
        .bind(owner_account_id)
        .bind(private_ordering_id)
        .bind(ordering_profile)
        .bind(vine_type)
        .bind(title)
        .bind(sentence)
        .bind(paragraph)
        .bind(full)
        .fetch_optional(&mut *tx)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        if let Some(items) = items {
            replace_private_ordering_items(&mut tx, private_ordering_id, items).await?;
        }

        tx.commit().await?;
        Ok(Some(row))
    }

    pub async fn delete_private_ordering(
        &self,
        owner_account_id: Uuid,
        private_ordering_id: Uuid,
    ) -> Result<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM private_orderings
            WHERE owner_account_id = $1
              AND private_ordering_id = $2
            "#,
        )
        .bind(owner_account_id)
        .bind(private_ordering_id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }
}
