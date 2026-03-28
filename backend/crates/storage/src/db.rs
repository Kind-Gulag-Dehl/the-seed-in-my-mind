use super::*;
use sqlx::FromRow;

#[derive(Debug, Clone, Copy)]
pub(crate) struct CycleWindow {
    pub(crate) cycle_index: i64,
    pub(crate) h_start: i64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EventInsertPosition {
    pub(crate) block_height: i64,
    pub(crate) event_index: i32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TempoPredicateState {
    pub(crate) cycle_age_ge_dmin: bool,
    pub(crate) cycle_age_ge_dmax: bool,
    pub(crate) constrained_mode: bool,
    pub(crate) record_only_mode: bool,
}

impl Default for TempoPredicateState {
    fn default() -> Self {
        Self {
            cycle_age_ge_dmin: false,
            cycle_age_ge_dmax: false,
            constrained_mode: false,
            record_only_mode: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum CycleCloseDisposition {
    Deliberative,
    Forced,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CycleCloseDecision {
    pub(crate) cycle_index: i64,
    pub(crate) position: EventInsertPosition,
    pub(crate) disposition: CycleCloseDisposition,
    pub(crate) tempo_state: TempoPredicateState,
}

#[derive(Debug, Clone, Copy, FromRow)]
struct TempoPredicateStateRow {
    cycle_age_ge_dmin: bool,
    cycle_age_ge_dmax: bool,
    constrained_mode: bool,
    record_only_mode: bool,
}

#[derive(Debug, Clone, Copy, FromRow)]
struct CycleBoundaryHistoryRow {
    cycle_index: i64,
    closure_block_height: i64,
    source_block_height: i64,
    source_event_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChallengePhase {
    OpenArguments,
    OpenVoting,
    Finalized,
    Cancelled,
    Superseded,
}

impl ChallengePhase {
    pub(crate) fn label(self) -> &'static str {
        match self {
            ChallengePhase::OpenArguments => "open_arguments",
            ChallengePhase::OpenVoting => "open_voting",
            ChallengePhase::Finalized => "finalized",
            ChallengePhase::Cancelled => "cancelled",
            ChallengePhase::Superseded => "superseded",
        }
    }
}

pub(crate) fn derive_challenge_phase(
    lifecycle_state: i16,
    created_cycle_index: i64,
    current_cycle_index: i64,
    has_verdict: bool,
) -> ChallengePhase {
    if has_verdict {
        return ChallengePhase::Finalized;
    }
    match lifecycle_state {
        2 => ChallengePhase::Finalized,
        3 => ChallengePhase::Cancelled,
        4 => ChallengePhase::Superseded,
        _ => {
            if current_cycle_index >= created_cycle_index + CHALLENGE_ARGUMENT_PHASE_CYCLES {
                ChallengePhase::OpenVoting
            } else {
                ChallengePhase::OpenArguments
            }
        }
    }
}

pub(crate) async fn load_canonical_writer_identity(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
) -> std::result::Result<Uuid, CanonicalWriteError> {
    let canonical_identity_id = load_account_canonical_identity(tx, account_id).await?;
    let writer_state = load_latest_writer_state(tx, canonical_identity_id, true).await?;
    let Some(writer_state) = writer_state else {
        return Err(CanonicalWriteError::new(
            "forbidden",
            "canonical write requires verified account and canonical writer level",
        ));
    };
    if !writer_state.email_verified
        || writer_state.canonical_writer_level < MIN_CANONICAL_WRITER_LEVEL
    {
        return Err(CanonicalWriteError::new(
            "forbidden",
            "canonical write requires verified account and canonical writer level",
        ));
    }

    Ok(canonical_identity_id)
}

pub(crate) async fn load_canonical_verifier_identity(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
) -> std::result::Result<Uuid, CanonicalWriteError> {
    let canonical_identity_id = load_account_canonical_identity(tx, account_id).await?;
    let is_verifier = load_active_verifier_state(tx, canonical_identity_id, true).await?;
    if !is_verifier {
        return Err(CanonicalWriteError::new(
            "forbidden",
            "canonical verifier role required",
        ));
    }
    Ok(canonical_identity_id)
}

pub(crate) async fn load_account_canonical_identity(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
) -> std::result::Result<Uuid, CanonicalWriteError> {
    let canonical_identity_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT canonical_identity_id FROM accounts WHERE account_id = $1 FOR UPDATE",
    )
    .bind(account_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(canonical_storage_error)?
    .flatten();
    let canonical_identity_id = canonical_identity_id.ok_or_else(|| {
        CanonicalWriteError::new(
            "forbidden",
            "canonical writer identity is not bound to this account",
        )
    })?;
    if canonical_identity_id == system_boundary_emitter_id() {
        return Err(CanonicalWriteError::new(
            "forbidden",
            "system boundary emitter is not allowed to author human canonical write events",
        ));
    }

    let identity_exists: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM identities_s0 WHERE identity_id = $1)")
            .bind(canonical_identity_id)
            .fetch_one(&mut **tx)
            .await
            .map_err(canonical_storage_error)?;
    if !identity_exists {
        return Err(CanonicalWriteError::new(
            "forbidden",
            "canonical writer identity is missing from canonical identities",
        ));
    }

    let identity_lock_key = advisory_lock_key_for_uuid(canonical_identity_id);
    sqlx::query("SELECT pg_advisory_xact_lock($1::bigint)")
        .bind(identity_lock_key)
        .execute(&mut **tx)
        .await
        .map_err(canonical_storage_error)?;

    Ok(canonical_identity_id)
}

pub(crate) async fn ensure_identity_exists(
    tx: &mut Transaction<'_, Postgres>,
    identity_id: Uuid,
) -> std::result::Result<(), CanonicalWriteError> {
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM identities_s0 WHERE identity_id = $1)")
            .bind(identity_id)
            .fetch_one(&mut **tx)
            .await
            .map_err(canonical_storage_error)?;
    if !exists {
        return Err(CanonicalWriteError::new(
            "invalid_request",
            format!("identity {} not found", identity_id),
        ));
    }
    Ok(())
}

pub(crate) async fn load_latest_writer_state(
    tx: &mut Transaction<'_, Postgres>,
    identity_id: Uuid,
    for_update: bool,
) -> std::result::Result<Option<IdentityWriterVerificationStateRow>, CanonicalWriteError> {
    let sql = if for_update {
        r#"
        SELECT
          identity_id,
          email_verified,
          canonical_writer_level,
          granted_by_identity_id,
          source_event_id,
          source_block_height,
          source_event_index
        FROM canonical_writer_verification_states
        WHERE identity_id = $1
        ORDER BY source_block_height DESC, source_event_index DESC
        LIMIT 1
        FOR UPDATE
        "#
    } else {
        r#"
        SELECT
          identity_id,
          email_verified,
          canonical_writer_level,
          granted_by_identity_id,
          source_event_id,
          source_block_height,
          source_event_index
        FROM canonical_writer_verification_states
        WHERE identity_id = $1
        ORDER BY source_block_height DESC, source_event_index DESC
        LIMIT 1
        "#
    };

    let row = sqlx::query_as::<_, IdentityWriterVerificationStateRow>(sql)
        .bind(identity_id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(canonical_storage_error)?;
    Ok(row)
}

pub(crate) fn seed_bootstrap_verifier_identity_id() -> Uuid {
    Uuid::parse_str(SEED_BOOTSTRAP_VERIFIER_ID_STR)
        .expect("SEED_BOOTSTRAP_VERIFIER_ID_STR must be a valid UUID")
}

pub(crate) async fn load_active_verifier_state(
    tx: &mut Transaction<'_, Postgres>,
    identity_id: Uuid,
    for_update: bool,
) -> std::result::Result<bool, CanonicalWriteError> {
    let sql = if for_update {
        r#"
        SELECT is_active
        FROM verifier_role_assignments
        WHERE verifier_identity_id = $1
        ORDER BY source_block_height DESC NULLS LAST, source_event_index DESC NULLS LAST, created_at DESC
        LIMIT 1
        FOR UPDATE
        "#
    } else {
        r#"
        SELECT is_active
        FROM verifier_role_assignments
        WHERE verifier_identity_id = $1
        ORDER BY source_block_height DESC NULLS LAST, source_event_index DESC NULLS LAST, created_at DESC
        LIMIT 1
        "#
    };

    let row = sqlx::query_as::<_, VerifierRoleAssignmentRow>(sql)
        .bind(identity_id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(canonical_storage_error)?;
    if let Some(row) = row {
        return Ok(row.is_active);
    }
    Ok(identity_id == seed_bootstrap_verifier_identity_id())
}

pub(crate) async fn load_cycle_window(
    tx: &mut Transaction<'_, Postgres>,
) -> std::result::Result<CycleWindow, CanonicalWriteError> {
    let row = sqlx::query_as::<_, CycleWindowRow>(
        r#"
        SELECT
          MAX(cycle_index) AS max_cycle_index,
          MAX(closure_block_height) AS max_closure_height
        FROM cycle_boundaries
        "#,
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;

    Ok(CycleWindow {
        cycle_index: row.max_cycle_index.unwrap_or(-1) + 1,
        h_start: row.max_closure_height.unwrap_or(-1) + 1,
    })
}

pub(crate) async fn load_cycle_build_spend(
    tx: &mut Transaction<'_, Postgres>,
    identity_id: Uuid,
    cycle_start_height: i64,
) -> std::result::Result<i64, CanonicalWriteError> {
    let spent: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(
          SUM(
            CASE
              WHEN event_type = 'idea_create' THEN $2
              WHEN event_type = 'connection_create' THEN $3
              WHEN event_type = 'challenge_create' THEN $4
              ELSE 0
            END
          ),
          0
        )::bigint
        FROM events
        WHERE speaker_identity_id = $1
          AND block_height >= $5
          AND event_type IN ('idea_create', 'connection_create', 'challenge_create')
        "#,
    )
    .bind(identity_id)
    .bind(IDEA_CREATE_MANA_COST)
    .bind(CONNECTION_CREATE_MANA_COST)
    .bind(CHALLENGE_CREATE_MANA_COST)
    .bind(cycle_start_height)
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    Ok(spent)
}

pub(crate) async fn load_cycle_voting_spend(
    tx: &mut Transaction<'_, Postgres>,
    identity_id: Uuid,
    cycle_start_height: i64,
) -> std::result::Result<i64, CanonicalWriteError> {
    let spent: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(COUNT(*), 0)::bigint
        FROM events
        WHERE speaker_identity_id = $1
          AND block_height >= $2
          AND event_type = 'vote_session_open'
        "#,
    )
    .bind(identity_id)
    .bind(cycle_start_height)
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    Ok(spent * VOTE_SESSION_OPEN_MANA_COST)
}

pub(crate) async fn load_next_vote_session_index(
    tx: &mut Transaction<'_, Postgres>,
    voter_identity_id: Uuid,
) -> std::result::Result<i64, CanonicalWriteError> {
    let next: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(MAX(session_index), -1)::bigint + 1
        FROM challenge_vote_sessions
        WHERE voter_identity_id = $1
        "#,
    )
    .bind(voter_identity_id)
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    Ok(next)
}

pub(crate) async fn load_latest_cycle_boundary_event_id(
    tx: &mut Transaction<'_, Postgres>,
) -> std::result::Result<Option<Uuid>, CanonicalWriteError> {
    let value: Option<Uuid> = sqlx::query_scalar(
        r#"
        SELECT source_event_id
        FROM cycle_boundaries
        ORDER BY cycle_index DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(canonical_storage_error)?
    .flatten();
    Ok(value)
}

pub(crate) async fn load_vote_session_candidates(
    tx: &mut Transaction<'_, Postgres>,
    voter_identity_id: Uuid,
    current_cycle_index: i64,
) -> std::result::Result<Vec<ChallengeVoteSessionCandidateRow>, CanonicalWriteError> {
    let rows = sqlx::query_as::<_, ChallengeVoteSessionCandidateRow>(
        r#"
        SELECT
          c.challenge_id,
          c.created_by_identity_id,
          c.created_cycle_index
        FROM challenges c
        LEFT JOIN challenge_verdicts cv ON cv.challenge_id = c.challenge_id
        WHERE cv.challenge_id IS NULL
          AND c.created_by_identity_id <> $1
          AND $2 >= c.created_cycle_index + $3
          AND (
            SELECT COUNT(*)
            FROM challenge_votes v
            WHERE v.challenge_id = c.challenge_id
          ) < $4
          AND NOT EXISTS (
            SELECT 1
            FROM challenge_votes v
            WHERE v.challenge_id = c.challenge_id
              AND v.voter_identity_id = $1
          )
          AND NOT EXISTS (
            SELECT 1
            FROM challenge_vote_sessions s
            WHERE s.challenge_id = c.challenge_id
              AND s.voter_identity_id = $1
          )
        ORDER BY c.challenge_id ASC
        "#,
    )
    .bind(voter_identity_id)
    .bind(current_cycle_index)
    .bind(CHALLENGE_ARGUMENT_PHASE_CYCLES)
    .bind(TARGET_JUROR_COUNT)
    .fetch_all(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    Ok(rows)
}

pub(crate) fn deterministic_vote_session_index(
    voter_identity_id: Uuid,
    selection_boundary_event_id: Uuid,
    cycle_index: i64,
    candidate_count: usize,
) -> usize {
    if candidate_count <= 1 {
        return 0;
    }

    let mut seed_input = Vec::with_capacity(16 + 16 + 8);
    seed_input.extend_from_slice(&voter_identity_id.as_u128().to_be_bytes());
    seed_input.extend_from_slice(&selection_boundary_event_id.as_u128().to_be_bytes());
    seed_input.extend_from_slice(&cycle_index.to_be_bytes());
    let digest = hash_bytes(&seed_input);
    let mut top = [0_u8; 8];
    top.copy_from_slice(&digest[..8]);
    let value = u64::from_be_bytes(top);
    (value % candidate_count as u64) as usize
}

pub(crate) fn aggregate_importance_verdict(
    votes: &[ChallengeVoteChoiceRow],
    left_target_idea_id: Uuid,
    right_target_idea_id: Uuid,
) -> (&'static str, Option<Uuid>, i64, i64, i64) {
    let mut left_votes = 0_i64;
    let mut right_votes = 0_i64;
    for vote in votes {
        match vote.vote_choice.as_str() {
            "left" => left_votes = left_votes.saturating_add(1),
            "right" => right_votes = right_votes.saturating_add(1),
            _ => {}
        }
    }
    let total_votes = i64::try_from(votes.len()).unwrap_or(0);
    if left_votes >= 2 {
        (
            "left",
            Some(left_target_idea_id),
            left_votes,
            right_votes,
            total_votes,
        )
    } else if right_votes >= 2 {
        (
            "right",
            Some(right_target_idea_id),
            left_votes,
            right_votes,
            total_votes,
        )
    } else {
        ("no_change", None, left_votes, right_votes, total_votes)
    }
}

pub(crate) fn deterministic_uuid_v7(
    label: &str,
    challenge_id: Uuid,
    vote_event_ids: &[Uuid],
) -> Uuid {
    let mut input = Vec::with_capacity(label.len() + 16 + vote_event_ids.len() * 16);
    input.extend_from_slice(label.as_bytes());
    input.extend_from_slice(&challenge_id.as_u128().to_be_bytes());
    for vote_event_id in vote_event_ids {
        input.extend_from_slice(&vote_event_id.as_u128().to_be_bytes());
    }
    let digest = hash_bytes(&input);
    let mut bytes = [0_u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x70;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}

pub(crate) async fn allocate_canonical_event_position(
    tx: &mut Transaction<'_, Postgres>,
) -> std::result::Result<EventInsertPosition, CanonicalWriteError> {
    let target_block_height: i64 =
        sqlx::query_scalar("SELECT COALESCE(MAX(block_height), 0)::bigint FROM blocks")
            .fetch_one(&mut **tx)
            .await
            .map_err(canonical_storage_error)?;

    ensure_block_row(tx, target_block_height).await?;
    let event_index: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(event_index), -1)::int + 1 FROM events WHERE block_height = $1",
    )
    .bind(target_block_height)
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;

    Ok(EventInsertPosition {
        block_height: target_block_height,
        event_index,
    })
}

pub(crate) async fn ensure_block_row(
    tx: &mut Transaction<'_, Postgres>,
    block_height: i64,
) -> std::result::Result<(), CanonicalWriteError> {
    let block_hash = format!("{:x}", block_height.max(0));
    let prev_block_hash = if block_height > 0 {
        Some(format!("{:x}", block_height - 1))
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
    .bind(block_height)
    .bind(block_hash)
    .bind(prev_block_hash)
    .execute(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    let _locked_block_height: i64 =
        sqlx::query_scalar("SELECT block_height FROM blocks WHERE block_height = $1 FOR UPDATE")
            .bind(block_height)
            .fetch_optional(&mut **tx)
            .await
            .map_err(canonical_storage_error)?
            .ok_or_else(|| {
                CanonicalWriteError::new(
                    "storage_error",
                    format!("missing canonical block_height {}", block_height),
                )
            })?;
    Ok(())
}

pub(crate) async fn maybe_load_cycle_close_decision(
    tx: &mut Transaction<'_, Postgres>,
) -> std::result::Result<Option<CycleCloseDecision>, CanonicalWriteError> {
    let cycle_window = load_cycle_window(tx).await?;
    let observed_work = load_cycle_observed_work_since_height(tx, cycle_window.h_start).await?;
    let w_target = load_current_cycle_w_target(tx).await?;
    let tempo_state = load_current_cycle_tempo_state(tx).await?;
    let closure_predicate_satisfied = tempo_state.cycle_age_ge_dmin
        && (observed_work >= w_target || tempo_state.cycle_age_ge_dmax);
    if !closure_predicate_satisfied {
        return Ok(None);
    }

    let position = allocate_canonical_event_position(tx).await?;
    let disposition = if observed_work >= w_target {
        CycleCloseDisposition::Deliberative
    } else {
        CycleCloseDisposition::Forced
    };
    Ok(Some(CycleCloseDecision {
        cycle_index: cycle_window.cycle_index,
        position,
        disposition,
        tempo_state,
    }))
}

pub(crate) async fn load_current_cycle_tempo_state(
    tx: &mut Transaction<'_, Postgres>,
) -> std::result::Result<TempoPredicateState, CanonicalWriteError> {
    let latest_cycle_boundary: Option<CycleBoundaryHistoryRow> = sqlx::query_as(
        r#"
        SELECT
          cycle_index,
          closure_block_height,
          source_block_height,
          source_event_index
        FROM cycle_boundaries
        ORDER BY cycle_index DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;

    let row = if let Some(boundary) = latest_cycle_boundary {
        sqlx::query_as::<_, TempoPredicateStateRow>(
            r#"
            SELECT
              cycle_age_ge_dmin,
              cycle_age_ge_dmax,
              constrained_mode,
              record_only_mode
            FROM tempo_predicates
            WHERE block_height > $1
               OR (block_height = $1 AND event_index > $2)
            ORDER BY block_height DESC, event_index DESC
            LIMIT 1
            "#,
        )
        .bind(boundary.source_block_height)
        .bind(boundary.source_event_index)
        .fetch_optional(&mut **tx)
        .await
        .map_err(canonical_storage_error)?
    } else {
        sqlx::query_as::<_, TempoPredicateStateRow>(
            r#"
            SELECT
              cycle_age_ge_dmin,
              cycle_age_ge_dmax,
              constrained_mode,
              record_only_mode
            FROM tempo_predicates
            ORDER BY block_height DESC, event_index DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&mut **tx)
        .await
        .map_err(canonical_storage_error)?
    };

    Ok(row
        .map(|value| TempoPredicateState {
            cycle_age_ge_dmin: value.cycle_age_ge_dmin,
            cycle_age_ge_dmax: value.cycle_age_ge_dmax,
            constrained_mode: value.constrained_mode,
            record_only_mode: value.record_only_mode,
        })
        .unwrap_or_default())
}

pub(crate) async fn persist_tempo_predicates(
    tx: &mut Transaction<'_, Postgres>,
    position: EventInsertPosition,
    tempo_state: TempoPredicateState,
) -> std::result::Result<(), CanonicalWriteError> {
    sqlx::query(
        r#"
        INSERT INTO tempo_predicates (
          block_height,
          event_index,
          cycle_age_ge_dmin,
          cycle_age_ge_dmax,
          constrained_mode,
          record_only_mode
        ) VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (block_height, event_index) DO UPDATE SET
          cycle_age_ge_dmin = EXCLUDED.cycle_age_ge_dmin,
          cycle_age_ge_dmax = EXCLUDED.cycle_age_ge_dmax,
          constrained_mode = EXCLUDED.constrained_mode,
          record_only_mode = EXCLUDED.record_only_mode
        "#,
    )
    .bind(position.block_height)
    .bind(position.event_index)
    .bind(tempo_state.cycle_age_ge_dmin)
    .bind(tempo_state.cycle_age_ge_dmax)
    .bind(tempo_state.constrained_mode)
    .bind(tempo_state.record_only_mode)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;
    Ok(())
}

pub(crate) async fn refresh_tempo_predicates_for_position(
    tx: &mut Transaction<'_, Postgres>,
    position: EventInsertPosition,
) -> std::result::Result<TempoPredicateState, CanonicalWriteError> {
    let tempo_state = load_current_cycle_tempo_state(tx).await?;
    persist_tempo_predicates(tx, position, tempo_state).await?;
    Ok(tempo_state)
}

async fn load_current_cycle_w_target(
    tx: &mut Transaction<'_, Postgres>,
) -> std::result::Result<i64, CanonicalWriteError> {
    const W_TARGET_ALPHA_NUM: i64 = 1;
    const W_TARGET_ALPHA_DEN: i64 = 2;
    const W_TARGET_SCALE_NUM: i64 = 1;
    const W_TARGET_SCALE_DEN: i64 = 1;
    const W_TARGET_MIN: i64 = 1;
    const W_TARGET_MAX: i64 = 10_000;
    const W_EMA_INITIAL: i64 = W_TARGET_MIN;

    let boundaries: Vec<CycleBoundaryHistoryRow> = sqlx::query_as(
        r#"
        SELECT
          cycle_index,
          closure_block_height,
          source_block_height,
          source_event_index
        FROM cycle_boundaries
        ORDER BY cycle_index ASC
        "#,
    )
    .fetch_all(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;

    let mut w_ema = W_EMA_INITIAL;
    let mut w_target = clamp_i64(
        round_div_i64(W_TARGET_SCALE_NUM * w_ema, W_TARGET_SCALE_DEN),
        W_TARGET_MIN,
        W_TARGET_MAX,
    );
    let mut cycle_start_height = 0_i64;
    for boundary in boundaries {
        let _ = boundary.cycle_index;
        let observed_work = load_cycle_observed_work_before_position(
            tx,
            cycle_start_height,
            boundary.source_block_height,
            boundary.source_event_index,
        )
        .await?;
        w_ema = round_div_i64(
            W_TARGET_ALPHA_NUM * observed_work + (W_TARGET_ALPHA_DEN - W_TARGET_ALPHA_NUM) * w_ema,
            W_TARGET_ALPHA_DEN,
        );
        w_target = clamp_i64(
            round_div_i64(W_TARGET_SCALE_NUM * w_ema, W_TARGET_SCALE_DEN),
            W_TARGET_MIN,
            W_TARGET_MAX,
        );
        cycle_start_height = boundary.closure_block_height + 1;
    }
    Ok(w_target)
}

async fn load_cycle_observed_work_since_height(
    tx: &mut Transaction<'_, Postgres>,
    cycle_start_height: i64,
) -> std::result::Result<i64, CanonicalWriteError> {
    let verdict_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(COUNT(*), 0)::bigint
        FROM challenge_verdicts
        WHERE resolved_block_height >= $1
        "#,
    )
    .bind(cycle_start_height)
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    let distinct_voters: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(COUNT(DISTINCT voter_identity_id), 0)::bigint
        FROM challenge_votes
        WHERE cast_block_height >= $1
        "#,
    )
    .bind(cycle_start_height)
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    Ok(verdict_count.saturating_add(distinct_voters))
}

async fn load_cycle_observed_work_before_position(
    tx: &mut Transaction<'_, Postgres>,
    cycle_start_height: i64,
    end_block_height: i64,
    end_event_index: i32,
) -> std::result::Result<i64, CanonicalWriteError> {
    let verdict_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(COUNT(*), 0)::bigint
        FROM challenge_verdicts
        WHERE resolved_block_height >= $1
          AND (
            resolved_block_height < $2
            OR (resolved_block_height = $2 AND resolved_event_index < $3)
          )
        "#,
    )
    .bind(cycle_start_height)
    .bind(end_block_height)
    .bind(end_event_index)
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    let distinct_voters: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(COUNT(DISTINCT voter_identity_id), 0)::bigint
        FROM challenge_votes
        WHERE cast_block_height >= $1
          AND (
            cast_block_height < $2
            OR (cast_block_height = $2 AND cast_event_index < $3)
          )
        "#,
    )
    .bind(cycle_start_height)
    .bind(end_block_height)
    .bind(end_event_index)
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    Ok(verdict_count.saturating_add(distinct_voters))
}

fn round_div_i64(numerator: i64, denominator: i64) -> i64 {
    if denominator <= 0 {
        return numerator;
    }
    if numerator >= 0 {
        (numerator + denominator / 2) / denominator
    } else {
        (numerator - denominator / 2) / denominator
    }
}

fn clamp_i64(value: i64, min: i64, max: i64) -> i64 {
    value.clamp(min, max)
}

pub(crate) fn map_canonical_write_sqlx_error(err: sqlx::Error) -> CanonicalWriteError {
    if let sqlx::Error::Database(db) = &err {
        if let Some(code) = db.code() {
            if code.as_ref() == "23505" {
                if let Some(constraint) = db.constraint() {
                    if constraint == "challenges_active_importance_key_idx" {
                        return CanonicalWriteError::new(
                            "conflict",
                            "duplicate active importance challenge for the same context and targets",
                        );
                    }
                    if constraint == "challenge_votes_challenge_voter_idx" {
                        return CanonicalWriteError::new(
                            "conflict",
                            "voter has already cast a vote for this challenge",
                        );
                    }
                    if constraint == "challenge_vote_sessions_voter_challenge_idx" {
                        return CanonicalWriteError::new(
                            "conflict",
                            "an active vote session already exists for this voter/challenge pair",
                        );
                    }
                    if constraint == "challenge_verdicts_challenge_id_key" {
                        return CanonicalWriteError::new(
                            "conflict",
                            "challenge already has a finalized verdict",
                        );
                    }
                }
                return CanonicalWriteError::new("conflict", "canonical object id already exists");
            }
            if code.as_ref() == "23503" {
                return CanonicalWriteError::new(
                    "invalid_request",
                    "referenced canonical object does not exist",
                );
            }
            if code.as_ref() == "42501" {
                return CanonicalWriteError::new(
                    "forbidden",
                    "canonical mutation blocked by append-only policy",
                );
            }
        }
    }
    canonical_storage_error(err)
}

pub(crate) fn canonical_storage_error(err: impl ToString) -> CanonicalWriteError {
    CanonicalWriteError::new("storage_error", err.to_string())
}

pub(crate) fn advisory_lock_key_for_uuid(value: Uuid) -> i64 {
    let bytes = value.as_u128().to_be_bytes();
    let mut upper = [0_u8; 8];
    upper.copy_from_slice(&bytes[..8]);
    let mut lower = [0_u8; 8];
    lower.copy_from_slice(&bytes[8..]);
    i64::from_be_bytes(upper) ^ i64::from_be_bytes(lower)
}

pub(crate) fn is_valid_idea_type(value: &str) -> bool {
    matches!(
        value,
        "truth_claim" | "conceptual_idea" | "actionable_idea" | "action" | "identity"
    )
}

pub(crate) async fn replace_private_vine_items(
    tx: &mut Transaction<'_, Postgres>,
    private_vine_id: Uuid,
    items: &[PrivateVineItemInput],
) -> Result<()> {
    sqlx::query(
        r#"
        DELETE FROM private_vine_items
        WHERE private_vine_id = $1
        "#,
    )
    .bind(private_vine_id)
    .execute(&mut **tx)
    .await?;

    for item in items {
        sqlx::query(
            r#"
            INSERT INTO private_vine_items (
              private_vine_id,
              idx,
              idea_id,
              via_connection_id
            ) VALUES (
              $1, $2, $3, $4
            )
            "#,
        )
        .bind(private_vine_id)
        .bind(item.idx)
        .bind(item.idea_id)
        .bind(item.via_connection_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}
