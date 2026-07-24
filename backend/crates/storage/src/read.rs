use super::*;

impl Storage {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new().connect(database_url).await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn list_ideas_top(
        &self,
        snapshot_height: i64,
        offset: i64,
        limit: i64,
        descending: bool,
    ) -> Result<Vec<IdeaSummaryRow>> {
        let query = if descending {
            queries::LIST_IDEAS_TOP_DESC
        } else {
            queries::LIST_IDEAS_TOP
        };
        let rows = sqlx::query_as::<_, IdeaSummaryRow>(query)
            .bind(snapshot_height)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    pub async fn count_ideas(&self, snapshot_height: i64) -> Result<i64> {
        let row = sqlx::query_as::<_, CountRow>(queries::COUNT_IDEAS)
            .bind(snapshot_height)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.total)
    }

    pub async fn search_ideas(
        &self,
        snapshot_height: i64,
        query: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<IdeaSummaryRow>> {
        let pattern = format!("%{}%", query);
        let rows = sqlx::query_as::<_, IdeaSummaryRow>(queries::SEARCH_IDEAS)
            .bind(snapshot_height)
            .bind(pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    pub async fn count_search_ideas(&self, snapshot_height: i64, query: &str) -> Result<i64> {
        let pattern = format!("%{}%", query);
        let row = sqlx::query_as::<_, CountRow>(queries::COUNT_SEARCH_IDEAS)
            .bind(snapshot_height)
            .bind(pattern)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.total)
    }

    pub async fn list_ideas_by_ids(
        &self,
        snapshot_height: i64,
        idea_ids: &[Uuid],
    ) -> Result<Vec<IdeaSummaryRow>> {
        let rows = sqlx::query_as::<_, IdeaSummaryRow>(queries::LIST_IDEAS_BY_IDS)
            .bind(idea_ids)
            .bind(snapshot_height)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    pub async fn list_ideas_by_author(
        &self,
        snapshot_height: i64,
        speaker_identity_id: Uuid,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<IdeaSummaryRow>> {
        let rows = sqlx::query_as::<_, IdeaSummaryRow>(queries::LIST_IDEAS_BY_SPEAKER)
            .bind(snapshot_height)
            .bind(speaker_identity_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    pub async fn get_idea_detail(
        &self,
        snapshot_height: i64,
        idea_id: Uuid,
    ) -> Result<Option<IdeaDetailRow>> {
        let row = sqlx::query_as::<_, IdeaDetailRow>(queries::GET_IDEA)
            .bind(idea_id)
            .bind(snapshot_height)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }

    pub async fn list_connections_for_idea(
        &self,
        snapshot_height: i64,
        idea_id: Uuid,
    ) -> Result<Vec<ConnectionRow>> {
        let rows = sqlx::query_as::<_, ConnectionRow>(queries::LIST_CONNECTIONS_FOR_IDEA)
            .bind(snapshot_height)
            .bind(idea_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    pub async fn list_connections_for_ideas(
        &self,
        snapshot_height: i64,
        idea_ids: &[Uuid],
    ) -> Result<Vec<ConnectionRow>> {
        let rows = sqlx::query_as::<_, ConnectionRow>(queries::LIST_CONNECTIONS_FOR_IDEAS)
            .bind(snapshot_height)
            .bind(idea_ids)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    pub async fn get_canonical_ordering(
        &self,
        snapshot_height: i64,
        ordering_id: Uuid,
    ) -> Result<Option<CanonicalOrderingRow>> {
        let row = sqlx::query_as::<_, CanonicalOrderingRow>(queries::GET_CANONICAL_ORDERING)
            .bind(ordering_id)
            .bind(snapshot_height)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    pub async fn list_canonical_ordering_items(
        &self,
        snapshot_height: i64,
        ordering_id: Uuid,
    ) -> Result<Vec<CanonicalOrderingItemRow>> {
        let rows =
            sqlx::query_as::<_, CanonicalOrderingItemRow>(queries::LIST_CANONICAL_ORDERING_ITEMS)
            .bind(ordering_id)
            .bind(snapshot_height)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }

    pub async fn list_canonical_orderings_for_idea(
        &self,
        snapshot_height: i64,
        idea_id: Uuid,
    ) -> Result<Vec<CanonicalOrderingSummaryRow>> {
        let rows =
            sqlx::query_as::<_, CanonicalOrderingSummaryRow>(queries::LIST_CANONICAL_ORDERINGS_FOR_IDEA)
                .bind(idea_id)
                .bind(snapshot_height)
                .fetch_all(&self.pool)
                .await?;
        Ok(rows)
    }

    pub async fn get_latest_snapshot(&self) -> Result<Option<SnapshotRow>> {
        let row = sqlx::query_as::<_, SnapshotRow>(
            r#"
            SELECT
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
              approximate_timestamp,
              cycle_index,
              cycle_close_height,
              created_at
            FROM snapshots
            ORDER BY block_height DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_identity(&self, identity_id: Uuid) -> Result<Option<IdentityRow>> {
        let row = sqlx::query_as::<_, IdentityRow>(
            r#"
            SELECT identity_id, title, created_event_id, created_at
            FROM identities_s0
            WHERE identity_id = $1
            "#,
        )
        .bind(identity_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn get_snapshot_by_height(&self, block_height: i64) -> Result<Option<SnapshotRow>> {
        let row = sqlx::query_as::<_, SnapshotRow>(
            r#"
            SELECT
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
              approximate_timestamp,
              cycle_index,
              cycle_close_height,
              created_at
            FROM snapshots
            WHERE block_height = $1
            "#,
        )
        .bind(block_height)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn list_snapshot_commits(&self, limit: i64) -> Result<Vec<SnapshotCommitRow>> {
        let rows = sqlx::query_as::<_, SnapshotCommitRow>(
            r#"
            SELECT
              block_height,
              snapshot_hash,
              state_root_hash,
              title_sentence_payload_root,
              shared_map_commitment,
              last_event_id,
              event_count,
              active_rulebook_set_hash,
              created_event_id,
              created_at
            FROM snapshot_commits
            ORDER BY block_height DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_snapshot_commit_by_height(
        &self,
        block_height: i64,
    ) -> Result<Option<SnapshotCommitRow>> {
        let row = sqlx::query_as::<_, SnapshotCommitRow>(
            r#"
            SELECT
              block_height,
              snapshot_hash,
              state_root_hash,
              title_sentence_payload_root,
              shared_map_commitment,
              last_event_id,
              event_count,
              active_rulebook_set_hash,
              created_event_id,
              created_at
            FROM snapshot_commits
            WHERE block_height = $1
            "#,
        )
        .bind(block_height)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }
}
