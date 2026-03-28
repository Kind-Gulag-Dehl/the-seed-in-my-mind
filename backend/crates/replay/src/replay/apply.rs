use super::parsing::*;
use super::state::*;
use super::tempo::*;
use super::voting::*;
use super::*;

#[cfg(test)]
pub(super) fn apply_events(
    events: &[EventRow],
    idea_by_event: &HashMap<Uuid, IdeaRow>,
    rail_by_event: &HashMap<Uuid, RailRow>,
    rail_items_by_rail: &HashMap<Uuid, Vec<RailItemRow>>,
    connection_by_event: &HashMap<Uuid, ConnectionRow>,
    representation_by_event: &HashMap<Uuid, RepresentationRow>,
    payload_by_idea: &HashMap<Uuid, IdeaPayloadRow>,
    tempo_rows: &[TempoPredicateRow],
    cycle_boundary_by_event: &HashMap<Uuid, CycleBoundaryRow>,
) -> Result<ApplyResult, ReplayError> {
    let writer_verification_by_event: HashMap<Uuid, WriterVerificationMaterializedRow> =
        HashMap::new();
    let verifier_role_rows: Vec<VerifierRoleRow> = Vec::new();
    apply_events_with_verification(
        events,
        idea_by_event,
        rail_by_event,
        rail_items_by_rail,
        connection_by_event,
        representation_by_event,
        payload_by_idea,
        tempo_rows,
        cycle_boundary_by_event,
        &writer_verification_by_event,
        &verifier_role_rows,
    )
}

pub(super) fn apply_events_with_verification(
    events: &[EventRow],
    idea_by_event: &HashMap<Uuid, IdeaRow>,
    rail_by_event: &HashMap<Uuid, RailRow>,
    rail_items_by_rail: &HashMap<Uuid, Vec<RailItemRow>>,
    connection_by_event: &HashMap<Uuid, ConnectionRow>,
    representation_by_event: &HashMap<Uuid, RepresentationRow>,
    payload_by_idea: &HashMap<Uuid, IdeaPayloadRow>,
    tempo_rows: &[TempoPredicateRow],
    cycle_boundary_by_event: &HashMap<Uuid, CycleBoundaryRow>,
    writer_verification_by_event: &HashMap<Uuid, WriterVerificationMaterializedRow>,
    verifier_role_rows: &[VerifierRoleRow],
) -> Result<ApplyResult, ReplayError> {
    let mut ideas = Vec::new();
    let mut rails = Vec::new();
    let mut connections = Vec::new();
    let mut payloads = Vec::new();
    let mut seen_ideas = BTreeSet::new();
    let mut seen_rails = BTreeSet::new();
    let mut seen_connections = BTreeSet::new();
    let mut seen_representations = BTreeSet::new();
    let mut idea_pointers: HashMap<Uuid, PointerSlots> = HashMap::new();
    let mut rail_pointers: HashMap<Uuid, PointerSlots> = HashMap::new();
    let mut representations: HashMap<Uuid, ReplayRepresentation> = HashMap::new();
    let mut rail_vine_type: HashMap<Uuid, Option<String>> = HashMap::new();
    let mut tempo_idx = 0usize;
    let mut tempo_state = TempoState::default();
    let mut cycle_index: i64 = 0;
    let mut h_start: i64 = 0;
    let mut w_ema: i64 = W_EMA_INITIAL;
    let mut w_target: i64 = clamp_i64(
        round_div_i64(W_TARGET_SCALE_NUM * w_ema, W_TARGET_SCALE_DEN),
        W_TARGET_MIN,
        W_TARGET_MAX,
    );
    let mut cycle_verdict_count: i64 = 0;
    let mut cycle_distinct_voters: BTreeSet<Uuid> = BTreeSet::new();
    let mut cycle_ready_at: Option<ReplayPosition> = None;
    let mut last_cycle_close_height: Option<i64> = None;
    let mut challenges_by_id: HashMap<Uuid, ImportanceChallengeState> = HashMap::new();
    let mut active_challenges_by_key: HashMap<ImportanceChallengeKey, Uuid> = HashMap::new();
    let mut vote_sessions_by_id: HashMap<Uuid, VoteSessionState> = HashMap::new();
    let mut next_session_index_by_voter: HashMap<Uuid, i64> = HashMap::new();
    let mut challenge_votes: HashMap<Uuid, Vec<ChallengeVoteState>> = HashMap::new();
    let mut vote_by_challenge_and_voter: HashMap<(Uuid, Uuid), Uuid> = HashMap::new();
    let mut session_by_challenge_and_voter: HashMap<(Uuid, Uuid), Uuid> = HashMap::new();
    let mut cycle_vote_session_count_by_voter: HashMap<Uuid, i64> = HashMap::new();
    let mut cycle_boundary_by_index: HashMap<i64, CycleBoundaryRow> = HashMap::new();
    let mut writer_state_by_identity: HashMap<Uuid, WriterVerificationState> = HashMap::new();
    let mut verifier_state_by_identity: HashMap<Uuid, bool> = HashMap::new();
    let mut seen_event_ids: BTreeSet<Uuid> = BTreeSet::new();
    for row in cycle_boundary_by_event.values() {
        cycle_boundary_by_index.insert(row.cycle_index, row.clone());
    }
    let mut ordered_verifier_roles = verifier_role_rows.to_vec();
    ordered_verifier_roles.sort_by_key(|row| {
        (
            row.source_block_height.unwrap_or(-1),
            row.source_event_index.unwrap_or(-1),
            row.source_event_id.unwrap_or(Uuid::nil()),
        )
    });
    for row in ordered_verifier_roles {
        verifier_state_by_identity.insert(row.verifier_identity_id, row.is_active);
    }
    verifier_state_by_identity
        .entry(seed_bootstrap_verifier_identity_id())
        .or_insert(true);

    for row in events {
        let event = Event {
            id: row.event_id,
            kind: row.event_type.clone(),
            payload: row.payload_json.clone(),
            speaker_identity_id: row.speaker_identity_id,
        };

        validate_event(&event).map_err(|err| {
            ReplayError::new(
                "event_validation_failed",
                format!("event_id={} {}", row.event_id, err),
            )
        })?;
        let position = ReplayPosition::from_event(row);
        while tempo_idx < tempo_rows.len() {
            let tempo_row = &tempo_rows[tempo_idx];
            let tempo_position = ReplayPosition {
                block_height: tempo_row.block_height,
                event_index: tempo_row.event_index,
            };
            if tempo_position > position {
                break;
            }
            tempo_state = TempoState {
                cycle_age_ge_dmin: tempo_row.cycle_age_ge_dmin,
                cycle_age_ge_dmax: tempo_row.cycle_age_ge_dmax,
                constrained_mode: tempo_row.constrained_mode,
                record_only_mode: tempo_row.record_only_mode,
            };
            tempo_idx += 1;
        }

        // Protocol v5/cycle-spec closure rule: Dmin && (W >= W_target || Dmax), earliest-valid.
        let observed_work =
            cycle_verdict_count + i64::try_from(cycle_distinct_voters.len()).unwrap_or(0);
        let closure_predicate_satisfied = tempo_state.cycle_age_ge_dmin
            && (observed_work >= w_target || tempo_state.cycle_age_ge_dmax);
        if closure_predicate_satisfied && cycle_ready_at.is_none() {
            cycle_ready_at = Some(position);
        }

        if event.kind == "cycle_close" {
            let ready_position = cycle_ready_at.ok_or_else(|| {
                ReplayError::new(
                    "cycle_close_predicate_not_satisfied",
                    format!(
                        "cycle_close event_id={} occurred before closure predicate was satisfied",
                        row.event_id
                    ),
                )
            })?;
            if ready_position != position {
                return Err(ReplayError::new(
                    "cycle_close_not_earliest_valid",
                    format!(
                        "cycle_close event_id={} is not earliest-valid position; expected {}:{} got {}:{}",
                        row.event_id,
                        ready_position.block_height,
                        ready_position.event_index,
                        position.block_height,
                        position.event_index,
                    ),
                ));
            }

            let payload = payload_object(&row.payload_json)?;
            let payload_cycle_index = parse_non_negative_i64_payload(payload, "cycle_index")?;
            if payload_cycle_index != cycle_index {
                return Err(ReplayError::new(
                    "cycle_close_cycle_index_mismatch",
                    format!(
                        "cycle_close event_id={} cycle_index mismatch expected={} actual={}",
                        row.event_id, cycle_index, payload_cycle_index
                    ),
                ));
            }

            let closure_kind = parse_cycle_closure_kind(payload)?;
            let forced_seal = payload
                .get("forced_seal")
                .and_then(Value::as_bool)
                .ok_or_else(|| ReplayError::new("invalid_field", "forced_seal required"))?;
            let expected_forced = matches!(closure_kind, CycleClosureKind::Forced);
            if forced_seal != expected_forced {
                return Err(ReplayError::new(
                    "invalid_field",
                    format!(
                        "cycle_close event_id={} forced_seal mismatch expected={} actual={}",
                        row.event_id, expected_forced, forced_seal
                    ),
                ));
            }

            let closure_boundary_ref = payload.get("closure_boundary_ref").ok_or_else(|| {
                ReplayError::new("missing_field", "closure_boundary_ref required")
            })?;
            let closure_block_height = parse_closure_boundary_height(closure_boundary_ref)?;
            if closure_block_height != row.block_height {
                return Err(ReplayError::new(
                    "cycle_close_boundary_mismatch",
                    format!(
                        "cycle_close event_id={} closure_boundary_ref mismatch expected={} actual={}",
                        row.event_id, row.block_height, closure_block_height
                    ),
                ));
            }

            let boundary_row = cycle_boundary_by_event.get(&row.event_id).ok_or_else(|| {
                ReplayError::new(
                    "missing_materialized_cycle_boundary",
                    format!("no cycle_boundaries row for event_id={}", row.event_id),
                )
            })?;
            if boundary_row.cycle_index != payload_cycle_index
                || boundary_row.closure_block_height != closure_block_height
                || boundary_row.source_block_height != row.block_height
                || boundary_row.source_event_index != row.event_index
                || boundary_row.source_event_id != row.event_id
            {
                return Err(ReplayError::new(
                    "cycle_boundary_event_mismatch",
                    format!("cycle_boundaries mismatch for event_id={}", row.event_id),
                ));
            }
            if boundary_row.forced_seal != forced_seal {
                return Err(ReplayError::new(
                    "cycle_boundary_event_mismatch",
                    format!(
                        "cycle_boundaries forced_seal mismatch for event_id={}",
                        row.event_id
                    ),
                ));
            }
            let expected_closure_kind = match closure_kind {
                CycleClosureKind::Deliberative => 0_i16,
                CycleClosureKind::Forced => 1_i16,
            };
            if boundary_row.closure_kind != expected_closure_kind {
                return Err(ReplayError::new(
                    "cycle_boundary_event_mismatch",
                    format!(
                        "cycle_boundaries closure_kind mismatch for event_id={}",
                        row.event_id
                    ),
                ));
            }

            let is_deliberative_path = observed_work >= w_target && tempo_state.cycle_age_ge_dmin;
            let is_forced_path = observed_work < w_target
                && tempo_state.cycle_age_ge_dmin
                && tempo_state.cycle_age_ge_dmax;
            match closure_kind {
                CycleClosureKind::Deliberative if !is_deliberative_path => {
                    return Err(ReplayError::new(
                        "cycle_close_invalid_path",
                        format!(
                            "cycle_close event_id={} marked deliberative without deliberative predicate",
                            row.event_id
                        ),
                    ));
                }
                CycleClosureKind::Forced if !is_forced_path => {
                    return Err(ReplayError::new(
                        "cycle_close_invalid_path",
                        format!(
                            "cycle_close event_id={} marked forced without forced predicate",
                            row.event_id
                        ),
                    ));
                }
                _ => {}
            }

            let w_obs = observed_work;
            w_ema = round_div_i64(
                W_TARGET_ALPHA_NUM * w_obs + (W_TARGET_ALPHA_DEN - W_TARGET_ALPHA_NUM) * w_ema,
                W_TARGET_ALPHA_DEN,
            );
            cycle_index += 1;
            h_start = closure_block_height + 1;
            w_target = clamp_i64(
                round_div_i64(W_TARGET_SCALE_NUM * w_ema, W_TARGET_SCALE_DEN),
                W_TARGET_MIN,
                W_TARGET_MAX,
            );
            cycle_verdict_count = 0;
            cycle_distinct_voters.clear();
            cycle_vote_session_count_by_voter.clear();
            cycle_ready_at = None;
            // Cycle-age predicates are scoped to the active cycle; reset on boundary rollover.
            tempo_state.cycle_age_ge_dmin = false;
            tempo_state.cycle_age_ge_dmax = false;
            last_cycle_close_height = Some(closure_block_height);
            seen_event_ids.insert(row.event_id);
            continue;
        }

        if cycle_ready_at == Some(position) {
            return Err(ReplayError::new(
                "missing_cycle_close_at_earliest_valid_position",
                format!(
                    "closure predicate first became true at {}:{} but event {} is {}",
                    position.block_height, position.event_index, row.event_id, event.kind
                ),
            ));
        }

        match event.kind.as_str() {
            "genesis" | "noop" | "identity_create" | "snapshot_commit" => {}
            "blocked_submission" => {
                let payload = payload_object(&row.payload_json)?;
                let speaker_identity_id = row.speaker_identity_id.ok_or_else(|| {
                    ReplayError::new(
                        "missing_field",
                        format!(
                            "blocked_submission event_id={} missing speaker_identity_id",
                            row.event_id
                        ),
                    )
                })?;
                let blocked_by_identity = parse_uuid_payload(payload, "blocked_by_identity")?;
                if blocked_by_identity != speaker_identity_id {
                    return Err(ReplayError::new(
                        "invalid_field",
                        format!(
                            "blocked_submission event_id={} blocked_by_identity mismatch",
                            row.event_id
                        ),
                    ));
                }
                if !verifier_state_by_identity
                    .get(&speaker_identity_id)
                    .copied()
                    .unwrap_or(false)
                {
                    return Err(ReplayError::new(
                        "forbidden_author",
                        format!(
                            "blocked_submission event_id={} speaker {} is not an active verifier",
                            row.event_id, speaker_identity_id
                        ),
                    ));
                }
                if let Some(reference_event_id) = parse_optional_uuid(payload, "reference_event_id")? {
                    if !seen_event_ids.contains(&reference_event_id) {
                        return Err(ReplayError::new(
                            "missing_reference_event",
                            format!(
                                "blocked_submission event_id={} references unknown prior event_id {}",
                                row.event_id, reference_event_id
                            ),
                        ));
                    }
                }
            }
            "challenge_create" => {
                let payload = payload_object(&row.payload_json)?;
                let challenge_id = parse_uuid_payload(payload, "challenge_id")?;
                if challenges_by_id.contains_key(&challenge_id) {
                    return Err(ReplayError::new(
                        "duplicate_challenge_id",
                        format!("duplicate challenge_id {}", challenge_id),
                    ));
                }
                let challenge_domain = parse_required_string_payload(payload, "challenge_domain")?;
                if challenge_domain != "importance_challenge" {
                    return Err(ReplayError::new(
                        "unsupported_challenge_domain",
                        format!("unsupported challenge_domain {}", challenge_domain),
                    ));
                }
                let context_key = parse_required_string_payload(payload, "context_key")?;
                let axis = parse_required_string_payload(payload, "axis")?;
                let timeframe = parse_required_string_payload(payload, "timeframe")?;
                let scope = parse_required_string_payload(payload, "scope")?;
                if context_key.trim().is_empty()
                    || axis.trim().is_empty()
                    || timeframe.trim().is_empty()
                    || scope.trim().is_empty()
                {
                    return Err(ReplayError::new(
                        "invalid_field",
                        "challenge_create requires non-empty context_key/axis/timeframe/scope",
                    ));
                }

                let (target_left_idea_id, target_right_idea_id) =
                    parse_required_subject_idea_pair(payload)?;
                if !seen_ideas.contains(&target_left_idea_id)
                    || !seen_ideas.contains(&target_right_idea_id)
                {
                    return Err(ReplayError::new(
                        "missing_target",
                        format!(
                            "challenge_create references unknown subject ideas {} {}",
                            target_left_idea_id, target_right_idea_id
                        ),
                    ));
                }
                let reference_idea_id = parse_optional_uuid(payload, "reference_idea_id")?;
                if let Some(reference_idea_id) = reference_idea_id {
                    if !seen_ideas.contains(&reference_idea_id) {
                        return Err(ReplayError::new(
                            "missing_target",
                            format!(
                                "challenge_create references unknown reference_idea_id {}",
                                reference_idea_id
                            ),
                        ));
                    }
                }
                let key = ImportanceChallengeKey {
                    context_key: context_key.to_string(),
                    target_left_idea_id,
                    target_right_idea_id,
                    reference_idea_id,
                };
                if let Some(existing_challenge_id) = active_challenges_by_key.get(&key) {
                    return Err(ReplayError::new(
                        "duplicate_challenge_instance",
                        format!(
                            "duplicate active importance challenge key for challenge_id={} existing={}",
                            challenge_id, existing_challenge_id
                        ),
                    ));
                }

                let state = ImportanceChallengeState {
                    key: key.clone(),
                    created_by_identity_id: row.speaker_identity_id.ok_or_else(|| {
                        ReplayError::new(
                            "missing_field",
                            format!(
                                "challenge_create event_id={} missing speaker_identity_id",
                                row.event_id
                            ),
                        )
                    })?,
                    axis: axis.to_string(),
                    timeframe: timeframe.to_string(),
                    scope: scope.to_string(),
                    created_cycle_index: cycle_index,
                    terminal: false,
                    finalized: false,
                };
                active_challenges_by_key.insert(key, challenge_id);
                challenges_by_id.insert(challenge_id, state);
            }
            "challenge_open_arguments"
            | "challenge_close_arguments"
            | "challenge_open_voting"
            | "challenge_close_voting" => {
                let payload = payload_object(&row.payload_json)?;
                let challenge_id = parse_uuid_payload(payload, "challenge_id")?;
                let challenge = challenges_by_id.get(&challenge_id).ok_or_else(|| {
                    ReplayError::new(
                        "missing_challenge",
                        format!(
                            "challenge lifecycle event references unknown challenge_id {}",
                            challenge_id
                        ),
                    )
                })?;
                if challenge.terminal {
                    return Err(ReplayError::new(
                        "challenge_lifecycle_invalid",
                        format!(
                            "challenge lifecycle event for terminal challenge_id {}",
                            challenge_id
                        ),
                    ));
                }
                if matches!(
                    event.kind.as_str(),
                    "challenge_close_arguments" | "challenge_open_voting"
                ) && cycle_index
                    < challenge.created_cycle_index + CHALLENGE_ARGUMENT_PHASE_CYCLES
                {
                    return Err(ReplayError::new(
                        "challenge_lifecycle_invalid",
                        format!(
                            "challenge lifecycle event {} occurred before voting-open boundary for challenge_id {}",
                            event.kind, challenge_id
                        ),
                    ));
                }
            }
            "challenge_cancel" | "challenge_supersede" => {
                let payload = payload_object(&row.payload_json)?;
                let challenge_id = parse_uuid_payload(payload, "challenge_id")?;
                let challenge = challenges_by_id.get_mut(&challenge_id).ok_or_else(|| {
                    ReplayError::new(
                        "missing_challenge",
                        format!(
                            "challenge terminal event references unknown challenge_id {}",
                            challenge_id
                        ),
                    )
                })?;
                if !challenge.terminal {
                    challenge.terminal = true;
                    active_challenges_by_key.remove(&challenge.key);
                }
            }
            "vote_session_open" => {
                let payload = payload_object(&row.payload_json)?;
                let vote_session_id = parse_uuid_payload(payload, "vote_session_id")?;
                if vote_sessions_by_id.contains_key(&vote_session_id) {
                    return Err(ReplayError::new(
                        "duplicate_vote_session",
                        format!("duplicate vote_session_id {}", vote_session_id),
                    ));
                }
                let challenge_id = parse_uuid_payload(payload, "challenge_id")?;
                let session_index = parse_non_negative_i64_payload(payload, "session_index")?;
                let selection_cycle_index =
                    parse_non_negative_i64_payload(payload, "selection_cycle_index")?;
                if selection_cycle_index != cycle_index {
                    return Err(ReplayError::new(
                        "vote_session_cycle_mismatch",
                        format!(
                            "vote_session_open event_id={} selection_cycle_index={} expected={}",
                            row.event_id, selection_cycle_index, cycle_index
                        ),
                    ));
                }
                let selection_boundary_event_id =
                    parse_uuid_payload(payload, "selection_boundary_event_id")?;
                let expected_boundary_event_id = cycle_boundary_by_index
                    .get(&(cycle_index - 1))
                    .map(|row| row.source_event_id)
                    .ok_or_else(|| {
                        ReplayError::new(
                            "missing_cycle_boundary",
                            format!(
                                "vote_session_open event_id={} missing cycle boundary for cycle_index={}",
                                row.event_id,
                                cycle_index - 1
                            ),
                        )
                    })?;
                if selection_boundary_event_id != expected_boundary_event_id {
                    return Err(ReplayError::new(
                        "vote_session_boundary_mismatch",
                        format!(
                            "vote_session_open event_id={} boundary mismatch expected={} actual={}",
                            row.event_id, expected_boundary_event_id, selection_boundary_event_id
                        ),
                    ));
                }

                let voter = row.speaker_identity_id.ok_or_else(|| {
                    ReplayError::new(
                        "missing_field",
                        format!(
                            "vote_session_open event_id={} missing speaker identity",
                            row.event_id
                        ),
                    )
                })?;
                if voter.to_string() == SYSTEM_BOUNDARY_EMITTER_ID_STR {
                    return Err(ReplayError::new(
                        "forbidden_author",
                        format!(
                            "vote_session_open event_id={} cannot be authored by system emitter",
                            row.event_id
                        ),
                    ));
                }

                let expected_session_index = next_session_index_by_voter
                    .get(&voter)
                    .copied()
                    .unwrap_or(0);
                if session_index != expected_session_index {
                    return Err(ReplayError::new(
                        "vote_session_index_mismatch",
                        format!(
                            "vote_session_open event_id={} session_index={} expected={}",
                            row.event_id, session_index, expected_session_index
                        ),
                    ));
                }

                let challenge = challenges_by_id.get(&challenge_id).ok_or_else(|| {
                    ReplayError::new(
                        "missing_challenge",
                        format!(
                            "vote_session_open references unknown challenge_id {}",
                            challenge_id
                        ),
                    )
                })?;
                if challenge.terminal || challenge.finalized {
                    return Err(ReplayError::new(
                        "challenge_lifecycle_invalid",
                        format!(
                            "vote_session_open references terminal challenge_id {}",
                            challenge_id
                        ),
                    ));
                }
                if cycle_index < challenge.created_cycle_index + CHALLENGE_ARGUMENT_PHASE_CYCLES {
                    return Err(ReplayError::new(
                        "challenge_lifecycle_invalid",
                        format!(
                            "vote_session_open before voting-open boundary for challenge_id {}",
                            challenge_id
                        ),
                    ));
                }
                if challenge.created_by_identity_id == voter {
                    return Err(ReplayError::new(
                        "ineligible_voter",
                        format!(
                            "vote_session_open voter {} is challenge creator {}",
                            voter, challenge_id
                        ),
                    ));
                }
                if vote_by_challenge_and_voter.contains_key(&(challenge_id, voter))
                    || session_by_challenge_and_voter.contains_key(&(challenge_id, voter))
                {
                    return Err(ReplayError::new(
                        "duplicate_vote_session",
                        format!(
                            "vote_session_open duplicate voter/challenge pair voter={} challenge={}",
                            voter, challenge_id
                        ),
                    ));
                }

                let candidates = deterministic_vote_session_candidates(
                    voter,
                    cycle_index,
                    &challenges_by_id,
                    &challenge_votes,
                    &vote_by_challenge_and_voter,
                    &session_by_challenge_and_voter,
                );
                if candidates.is_empty() {
                    return Err(ReplayError::new(
                        "vote_session_no_candidates",
                        format!(
                            "vote_session_open event_id={} has no eligible candidates",
                            row.event_id
                        ),
                    ));
                }
                let selected_index = deterministic_vote_session_index(
                    voter,
                    selection_boundary_event_id,
                    cycle_index,
                    candidates.len(),
                );
                let expected_challenge_id = candidates[selected_index];
                if challenge_id != expected_challenge_id {
                    return Err(ReplayError::new(
                        "vote_session_assignment_mismatch",
                        format!(
                            "vote_session_open event_id={} assigned challenge {} expected {}",
                            row.event_id, challenge_id, expected_challenge_id
                        ),
                    ));
                }

                let cycle_session_count =
                    cycle_vote_session_count_by_voter.entry(voter).or_insert(0);
                *cycle_session_count = cycle_session_count.saturating_add(1);
                if *cycle_session_count > TARGET_JUROR_COUNT as i64 {
                    return Err(ReplayError::new(
                        "insufficient_mana",
                        format!(
                            "vote_session_open event_id={} exceeds per-cycle voting capacity for voter {}",
                            row.event_id, voter
                        ),
                    ));
                }

                vote_sessions_by_id.insert(
                    vote_session_id,
                    VoteSessionState {
                        challenge_id,
                        voter_identity_id: voter,
                    },
                );
                session_by_challenge_and_voter.insert((challenge_id, voter), vote_session_id);
                next_session_index_by_voter.insert(voter, expected_session_index + 1);
            }
            "vote_cast" => {
                let payload = payload_object(&row.payload_json)?;
                let challenge_id = parse_uuid_payload(payload, "challenge_id")?;
                let vote_session_id = parse_uuid_payload(payload, "vote_session_id")?;
                let vote_choice = parse_required_string_payload(payload, "vote_choice")?;
                let voter = row.speaker_identity_id.ok_or_else(|| {
                    ReplayError::new(
                        "missing_field",
                        format!(
                            "vote_cast event_id={} missing speaker identity",
                            row.event_id
                        ),
                    )
                })?;
                if voter.to_string() == SYSTEM_BOUNDARY_EMITTER_ID_STR {
                    return Err(ReplayError::new(
                        "forbidden_author",
                        format!(
                            "vote_cast event_id={} authored by system emitter",
                            row.event_id
                        ),
                    ));
                }
                if !matches!(vote_choice, "left" | "right" | "abstain") {
                    return Err(ReplayError::new(
                        "invalid_field",
                        format!("vote_cast event_id={} invalid vote_choice", row.event_id),
                    ));
                }

                let challenge = challenges_by_id.get(&challenge_id).ok_or_else(|| {
                    ReplayError::new(
                        "missing_challenge",
                        format!("vote_cast references unknown challenge_id {}", challenge_id),
                    )
                })?;
                if challenge.terminal || challenge.finalized {
                    return Err(ReplayError::new(
                        "challenge_lifecycle_invalid",
                        format!("vote_cast for terminal challenge_id {}", challenge_id),
                    ));
                }
                if cycle_index < challenge.created_cycle_index + CHALLENGE_ARGUMENT_PHASE_CYCLES {
                    return Err(ReplayError::new(
                        "challenge_lifecycle_invalid",
                        format!(
                            "vote_cast before voting-open boundary for challenge_id {}",
                            challenge_id
                        ),
                    ));
                }
                if challenge.created_by_identity_id == voter {
                    return Err(ReplayError::new(
                        "ineligible_voter",
                        format!(
                            "vote_cast voter {} is challenge creator {}",
                            voter, challenge_id
                        ),
                    ));
                }

                let session = vote_sessions_by_id.get(&vote_session_id).ok_or_else(|| {
                    ReplayError::new(
                        "missing_vote_session",
                        format!(
                            "vote_cast references unknown vote_session_id {}",
                            vote_session_id
                        ),
                    )
                })?;
                if session.challenge_id != challenge_id || session.voter_identity_id != voter {
                    return Err(ReplayError::new(
                        "vote_session_mismatch",
                        format!(
                            "vote_cast session mismatch vote_session_id={} challenge_id={} voter={}",
                            vote_session_id, challenge_id, voter
                        ),
                    ));
                }

                if vote_by_challenge_and_voter.contains_key(&(challenge_id, voter)) {
                    return Err(ReplayError::new(
                        "duplicate_vote",
                        format!(
                            "duplicate vote for challenge_id={} voter={}",
                            challenge_id, voter
                        ),
                    ));
                }

                let votes = challenge_votes.entry(challenge_id).or_default();
                if votes.len() >= TARGET_JUROR_COUNT {
                    return Err(ReplayError::new(
                        "challenge_vote_capacity_reached",
                        format!(
                            "vote_cast event_id={} exceeds target juror count for challenge_id={}",
                            row.event_id, challenge_id
                        ),
                    ));
                }
                votes.push(ChallengeVoteState {
                    vote_choice: vote_choice.to_string(),
                    vote_event_id: row.event_id,
                });
                vote_by_challenge_and_voter.insert((challenge_id, voter), row.event_id);
                cycle_distinct_voters.insert(voter);
            }
            "canonical_writer_grant" => {
                let payload = payload_object(&row.payload_json)?;
                let identity_id = parse_uuid_payload(payload, "identity_id")?;
                let _was_eligible = is_writer_eligible(writer_state_by_identity.get(&identity_id));
                let level = parse_non_negative_i64_payload(payload, "canonical_writer_level")?;
                if level < MIN_CANONICAL_WRITER_LEVEL as i64 || level > i16::MAX as i64 {
                    return Err(ReplayError::new(
                        "invalid_field",
                        format!(
                            "canonical_writer_grant invalid canonical_writer_level={} event_id={}",
                            level, row.event_id
                        ),
                    ));
                }
                let email_verified = payload
                    .get("email_verified")
                    .and_then(Value::as_bool)
                    .ok_or_else(|| ReplayError::new("missing_field", "email_verified required"))?;
                let verifier_identity_id = row.speaker_identity_id.ok_or_else(|| {
                    ReplayError::new(
                        "missing_field",
                        format!(
                            "canonical_writer_grant event_id={} missing speaker_identity_id",
                            row.event_id
                        ),
                    )
                })?;
                if !is_identity_active_verifier(&verifier_state_by_identity, verifier_identity_id) {
                    return Err(ReplayError::new(
                        "forbidden_author",
                        format!(
                            "canonical_writer_grant event_id={} speaker {} is not an active verifier",
                            row.event_id, verifier_identity_id
                        ),
                    ));
                }

                let materialized =
                    writer_verification_by_event
                        .get(&row.event_id)
                        .ok_or_else(|| {
                            ReplayError::new(
                                "missing_materialized_writer_verification",
                                format!(
                                "canonical_writer_grant event_id={} missing materialized state row",
                                row.event_id
                            ),
                            )
                        })?;
                if materialized.identity_id != identity_id
                    || materialized.email_verified != email_verified
                    || materialized.canonical_writer_level != level as i16
                    || materialized.granted_by_identity_id != verifier_identity_id
                    || materialized.source_block_height != row.block_height
                    || materialized.source_event_index != row.event_index
                    || materialized.source_event_id != row.event_id
                {
                    return Err(ReplayError::new(
                        "writer_verification_mismatch",
                        format!(
                            "canonical_writer_grant event_id={} materialized writer state mismatch",
                            row.event_id
                        ),
                    ));
                }

                writer_state_by_identity.insert(
                    identity_id,
                    WriterVerificationState {
                        email_verified,
                        canonical_writer_level: level as i16,
                    },
                );
            }
            "canonical_writer_revoke" => {
                let payload = payload_object(&row.payload_json)?;
                let identity_id = parse_uuid_payload(payload, "identity_id")?;
                let _was_eligible = is_writer_eligible(writer_state_by_identity.get(&identity_id));
                let verifier_identity_id = row.speaker_identity_id.ok_or_else(|| {
                    ReplayError::new(
                        "missing_field",
                        format!(
                            "canonical_writer_revoke event_id={} missing speaker_identity_id",
                            row.event_id
                        ),
                    )
                })?;
                if !is_identity_active_verifier(&verifier_state_by_identity, verifier_identity_id) {
                    return Err(ReplayError::new(
                        "forbidden_author",
                        format!(
                            "canonical_writer_revoke event_id={} speaker {} is not an active verifier",
                            row.event_id, verifier_identity_id
                        ),
                    ));
                }

                let materialized =
                    writer_verification_by_event
                        .get(&row.event_id)
                        .ok_or_else(|| {
                            ReplayError::new(
                                "missing_materialized_writer_verification",
                                format!(
                                    "canonical_writer_revoke event_id={} missing materialized state row",
                                    row.event_id
                                ),
                            )
                        })?;
                if materialized.identity_id != identity_id
                    || materialized.email_verified
                    || materialized.canonical_writer_level != 0
                    || materialized.granted_by_identity_id != verifier_identity_id
                    || materialized.source_block_height != row.block_height
                    || materialized.source_event_index != row.event_index
                    || materialized.source_event_id != row.event_id
                {
                    return Err(ReplayError::new(
                        "writer_verification_mismatch",
                        format!(
                            "canonical_writer_revoke event_id={} materialized writer state mismatch",
                            row.event_id
                        ),
                    ));
                }

                writer_state_by_identity.insert(
                    identity_id,
                    WriterVerificationState {
                        email_verified: false,
                        canonical_writer_level: 0,
                    },
                );
            }
            "idea_create" => {
                let idea = idea_by_event.get(&row.event_id).ok_or_else(|| {
                    ReplayError::new(
                        "missing_materialized_idea",
                        format!("no idea row for event_id={}", row.event_id),
                    )
                })?;

                if idea.created_block_height != row.block_height
                    || idea.created_event_index != row.event_index
                {
                    return Err(ReplayError::new(
                        "idea_event_mismatch",
                        format!("idea row ordering mismatch for event_id={}", row.event_id),
                    ));
                }

                if Some(idea.speaker_identity_id) != row.speaker_identity_id {
                    return Err(ReplayError::new(
                        "idea_event_mismatch",
                        format!("idea speaker mismatch for event_id={}", row.event_id),
                    ));
                }

                if !is_valid_idea_type(&idea.idea_type) {
                    return Err(ReplayError::new(
                        "invalid_field",
                        format!("invalid idea_type {}", idea.idea_type),
                    ));
                }

                if !seen_ideas.insert(idea.idea_id) {
                    return Err(ReplayError::new(
                        "duplicate_idea",
                        format!("duplicate idea_id {}", idea.idea_id),
                    ));
                }

                ideas.push(ReplayIdeaRow {
                    idea_id: idea.idea_id,
                    idea_type: idea.idea_type.clone(),
                    speaker_identity_id: idea.speaker_identity_id,
                    created_event_id: idea.created_event_id,
                    created_block_height: idea.created_block_height,
                    created_event_index: idea.created_event_index,
                });
            }
            "rail_create" | "rail_fork" => {
                let rail = rail_by_event.get(&row.event_id).ok_or_else(|| {
                    ReplayError::new(
                        "missing_materialized_rail",
                        format!("no rail row for event_id={}", row.event_id),
                    )
                })?;

                if rail.created_block_height != row.block_height
                    || rail.created_event_index != row.event_index
                {
                    return Err(ReplayError::new(
                        "rail_event_mismatch",
                        format!("rail row ordering mismatch for event_id={}", row.event_id),
                    ));
                }

                if Some(rail.speaker_identity_id) != row.speaker_identity_id {
                    return Err(ReplayError::new(
                        "rail_event_mismatch",
                        format!("rail speaker mismatch for event_id={}", row.event_id),
                    ));
                }

                if !seen_rails.insert(rail.rail_id) {
                    return Err(ReplayError::new(
                        "duplicate_rail",
                        format!("duplicate rail_id {}", rail.rail_id),
                    ));
                }

                let rail_kind = parse_rail_kind(rail.rail_kind)?;
                let row_vine_type = parse_vine_type(rail.vine_type)?;
                let payload = payload_object(&row.payload_json)?;
                let mut vine_type = parse_vine_type_payload(payload, false)?;
                if event.kind == "rail_fork" && vine_type.is_none() {
                    let base_rail_id = rail.base_rail_id.ok_or_else(|| {
                        ReplayError::new(
                            "invalid_field",
                            format!(
                                "rail_fork missing base_rail_id for rail_id={}",
                                rail.rail_id
                            ),
                        )
                    })?;
                    vine_type = rail_vine_type.get(&base_rail_id).cloned().flatten();
                }
                if vine_type.is_none() {
                    vine_type = row_vine_type.clone();
                }
                if row_vine_type.is_some() && row_vine_type != vine_type {
                    return Err(ReplayError::new(
                        "rail_event_mismatch",
                        format!("vine_type mismatch for rail_id={}", rail.rail_id),
                    ));
                }

                let item_rows = rail_items_by_rail
                    .get(&rail.rail_id)
                    .cloned()
                    .unwrap_or_default();
                for (expected_idx, item) in item_rows.iter().enumerate() {
                    if item.idx != expected_idx as i32 {
                        return Err(ReplayError::new(
                            "invalid_field",
                            format!(
                                "rail_items idx mismatch rail_id={} expected={} actual={}",
                                rail.rail_id, expected_idx, item.idx
                            ),
                        ));
                    }
                }
                let items = item_rows
                    .into_iter()
                    .map(|item| ReplayRailItemRow {
                        idx: item.idx,
                        idea_id: item.idea_id,
                        via_connection_id: item.via_connection_id,
                    })
                    .collect::<Vec<_>>();

                let initial_refs = parse_initial_representation_refs(payload)?;
                if initial_refs.title_representation_id.is_some()
                    || initial_refs.sentence_representation_id.is_some()
                {
                    rail_pointers.insert(rail.rail_id, initial_refs.clone());
                }
                rail_vine_type.insert(rail.rail_id, vine_type.clone());

                rails.push(ReplayRailRow {
                    rail_id: rail.rail_id,
                    rail_kind: rail_kind_to_string(rail_kind).to_string(),
                    vine_type,
                    speaker_identity_id: rail.speaker_identity_id,
                    created_event_id: rail.created_event_id,
                    created_block_height: rail.created_block_height,
                    created_event_index: rail.created_event_index,
                    base_rail_id: rail.base_rail_id,
                    title_representation_id: initial_refs.title_representation_id,
                    sentence_representation_id: initial_refs.sentence_representation_id,
                    items,
                });
            }
            "connection_create" => {
                let connection = connection_by_event.get(&row.event_id).ok_or_else(|| {
                    ReplayError::new(
                        "missing_materialized_connection",
                        format!("no connection row for event_id={}", row.event_id),
                    )
                })?;

                if connection.created_block_height != row.block_height
                    || connection.created_event_index != row.event_index
                {
                    return Err(ReplayError::new(
                        "connection_event_mismatch",
                        format!("connection ordering mismatch for event_id={}", row.event_id),
                    ));
                }

                if !is_valid_connection_type(&connection.connection_type) {
                    return Err(ReplayError::new(
                        "invalid_field",
                        format!("invalid connection_type {}", connection.connection_type),
                    ));
                }

                if !seen_connections.insert(connection.connection_id) {
                    return Err(ReplayError::new(
                        "duplicate_connection",
                        format!("duplicate connection_id {}", connection.connection_id),
                    ));
                }

                connections.push(ReplayConnectionRow {
                    connection_id: connection.connection_id,
                    from_idea_id: connection.from_idea_id,
                    to_idea_id: connection.to_idea_id,
                    connection_type: connection.connection_type.clone(),
                    usage: connection.usage.clone(),
                    axis: connection.axis.clone(),
                    timeframe: connection.timeframe.clone(),
                    scope: connection.scope.clone(),
                    created_by_event_id: connection.created_by_event_id,
                    created_block_height: connection.created_block_height,
                    created_event_index: connection.created_event_index,
                });
                if connection.connection_type == "relative_importance"
                    && connection.usage.as_deref() == Some("importance_argument")
                {
                    let payload = payload_object(&row.payload_json)?;
                    if let Some(context_challenge_id) =
                        parse_optional_uuid(payload, "context_challenge_id")?
                    {
                        let challenge = challenges_by_id.get(&context_challenge_id).ok_or_else(|| {
                            ReplayError::new(
                                "missing_challenge",
                                format!(
                                    "importance_argument references unknown context_challenge_id {}",
                                    context_challenge_id
                                ),
                            )
                        })?;
                        if challenge.terminal {
                            return Err(ReplayError::new(
                                "challenge_lifecycle_invalid",
                                format!(
                                    "importance_argument references terminal challenge_id {}",
                                    context_challenge_id
                                ),
                            ));
                        }
                        if cycle_index
                            >= challenge.created_cycle_index + CHALLENGE_ARGUMENT_PHASE_CYCLES
                        {
                            return Err(ReplayError::new(
                                "challenge_lifecycle_invalid",
                                format!(
                                    "importance_argument outside open-arguments phase for challenge_id {}",
                                    context_challenge_id
                                ),
                            ));
                        }
                        if connection.to_idea_id != challenge.key.target_left_idea_id
                            && connection.to_idea_id != challenge.key.target_right_idea_id
                        {
                            return Err(ReplayError::new(
                                "challenge_argument_target_mismatch",
                                format!(
                                    "importance_argument target mismatch challenge_id={} target={}",
                                    context_challenge_id, connection.to_idea_id
                                ),
                            ));
                        }
                        if connection.axis.as_deref() != Some(challenge.axis.as_str())
                            || connection.timeframe.as_deref() != Some(challenge.timeframe.as_str())
                            || connection.scope.as_deref() != Some(challenge.scope.as_str())
                        {
                            return Err(ReplayError::new(
                                "challenge_argument_context_mismatch",
                                format!(
                                    "importance_argument context mismatch for challenge_id={}",
                                    context_challenge_id
                                ),
                            ));
                        }
                    }
                }
            }
            "representation_create" | "rail_update_representation" => {
                let representation =
                    representation_by_event.get(&row.event_id).ok_or_else(|| {
                        ReplayError::new(
                            "missing_materialized_representation",
                            format!("no representation row for event_id={}", row.event_id),
                        )
                    })?;

                if representation.created_block_height != row.block_height
                    || representation.created_event_index != row.event_index
                {
                    return Err(ReplayError::new(
                        "representation_event_mismatch",
                        format!(
                            "representation ordering mismatch for event_id={}",
                            row.event_id
                        ),
                    ));
                }

                if !seen_representations.insert(representation.representation_id) {
                    return Err(ReplayError::new(
                        "duplicate_representation",
                        format!(
                            "duplicate representation_id {}",
                            representation.representation_id
                        ),
                    ));
                }

                let target_kind = parse_target_kind(representation.target_kind)?;
                if event.kind == "rail_update_representation" && target_kind != TargetKind::Rail {
                    return Err(ReplayError::new(
                        "invalid_field",
                        format!(
                            "rail_update_representation must target rail: {}",
                            representation.representation_id
                        ),
                    ));
                }
                let tier_enum = parse_tier_enum(representation.tier_enum)?;
                representations.insert(
                    representation.representation_id,
                    ReplayRepresentation {
                        representation_id: representation.representation_id,
                        target_kind,
                        target_id: representation.target_id,
                        tier_enum,
                        payload_hash: representation.payload_hash.clone(),
                        payload_text: representation.payload_text.clone(),
                    },
                );
            }
            "challenge_finalize_verdict" => {
                let payload = payload_object(&row.payload_json)?;
                let challenge_id = parse_uuid_payload(payload, "challenge_id")?;
                let challenge = challenges_by_id.get_mut(&challenge_id).ok_or_else(|| {
                    ReplayError::new(
                        "missing_challenge",
                        format!(
                            "challenge_finalize_verdict references unknown challenge_id {}",
                            challenge_id
                        ),
                    )
                })?;
                if challenge.finalized {
                    return Err(ReplayError::new(
                        "duplicate_verdict",
                        format!(
                            "challenge_finalize_verdict duplicate finalization for challenge_id {}",
                            challenge_id
                        ),
                    ));
                }

                let has_vote_fields = payload.contains_key("winning_choice")
                    || payload.contains_key("winning_target_idea_id")
                    || payload.contains_key("left_votes")
                    || payload.contains_key("right_votes")
                    || payload.contains_key("total_votes");
                let has_recorded_votes = challenge_votes
                    .get(&challenge_id)
                    .map(|rows| !rows.is_empty())
                    .unwrap_or(false);
                if has_vote_fields || has_recorded_votes {
                    let accepted_votes = challenge_votes.get(&challenge_id).ok_or_else(|| {
                        ReplayError::new(
                            "insufficient_votes",
                            format!(
                                "challenge_finalize_verdict missing accepted votes for challenge_id {}",
                                challenge_id
                            ),
                        )
                    })?;
                    if accepted_votes.len() != TARGET_JUROR_COUNT {
                        return Err(ReplayError::new(
                            "insufficient_votes",
                            format!(
                                "challenge_finalize_verdict expected {} accepted votes for challenge_id {} but saw {}",
                                TARGET_JUROR_COUNT,
                                challenge_id,
                                accepted_votes.len()
                            ),
                        ));
                    }

                    let (
                        expected_choice,
                        expected_target,
                        expected_left,
                        expected_right,
                        expected_total,
                    ) = aggregate_importance_verdict_for_replay(accepted_votes, challenge);
                    let actual_choice = parse_required_string_payload(payload, "winning_choice")?;
                    if actual_choice != expected_choice {
                        return Err(ReplayError::new(
                            "verdict_mismatch",
                            format!(
                                "challenge_finalize_verdict choice mismatch challenge_id={} expected={} actual={}",
                                challenge_id, expected_choice, actual_choice
                            ),
                        ));
                    }
                    let actual_target = parse_optional_uuid(payload, "winning_target_idea_id")?;
                    if actual_target != expected_target {
                        return Err(ReplayError::new(
                            "verdict_mismatch",
                            format!(
                                "challenge_finalize_verdict target mismatch challenge_id={} expected={:?} actual={:?}",
                                challenge_id, expected_target, actual_target
                            ),
                        ));
                    }
                    let actual_left = parse_non_negative_i64_payload(payload, "left_votes")?;
                    let actual_right = parse_non_negative_i64_payload(payload, "right_votes")?;
                    let actual_total = parse_non_negative_i64_payload(payload, "total_votes")?;
                    if actual_left != expected_left
                        || actual_right != expected_right
                        || actual_total != expected_total
                    {
                        return Err(ReplayError::new(
                            "verdict_mismatch",
                            format!(
                                "challenge_finalize_verdict tally mismatch challenge_id={} expected=({},{},{}) actual=({},{},{})",
                                challenge_id,
                                expected_left,
                                expected_right,
                                expected_total,
                                actual_left,
                                actual_right,
                                actual_total
                            ),
                        ));
                    }

                    if let Some(vote_event_ids) = payload.get("vote_event_ids") {
                        let ids = vote_event_ids.as_array().ok_or_else(|| {
                            ReplayError::new("invalid_field", "vote_event_ids must be array")
                        })?;
                        if ids.len() != TARGET_JUROR_COUNT {
                            return Err(ReplayError::new(
                                "invalid_field",
                                format!(
                                    "vote_event_ids must contain {} entries",
                                    TARGET_JUROR_COUNT
                                ),
                            ));
                        }
                        for (idx, value) in ids.iter().enumerate() {
                            let parsed = parse_uuid_value(value, "vote_event_ids")?;
                            if parsed != accepted_votes[idx].vote_event_id {
                                return Err(ReplayError::new(
                                    "verdict_mismatch",
                                    format!(
                                        "vote_event_ids mismatch at index={} expected={} actual={}",
                                        idx, accepted_votes[idx].vote_event_id, parsed
                                    ),
                                ));
                            }
                        }
                    }
                }
                challenge.finalized = true;
                challenge.terminal = true;
                active_challenges_by_key.remove(&challenge.key);
                let updates = parse_representation_pointer_updates(payload)?;
                for update in updates {
                    let representation =
                        representations.get(&update.representation_id).ok_or_else(|| {
                            ReplayError::new(
                                "missing_representation",
                                format!(
                                    "challenge_finalize_verdict references unknown representation_id {}",
                                    update.representation_id
                                ),
                            )
                        })?;
                    if representation.target_kind != update.target_kind
                        || representation.target_id != update.target_object_id
                        || representation.tier_enum != update.tier_enum
                    {
                        return Err(ReplayError::new(
                            "invalid_field",
                            format!(
                                "challenge_finalize_verdict selection mismatch representation_id={}",
                                update.representation_id
                            ),
                        ));
                    }

                    match update.target_kind {
                        TargetKind::Idea => {
                            if !seen_ideas.contains(&update.target_object_id) {
                                return Err(ReplayError::new(
                                    "missing_target",
                                    format!(
                                        "challenge_finalize_verdict target idea missing: {}",
                                        update.target_object_id
                                    ),
                                ));
                            }
                            let pointers =
                                idea_pointers.entry(update.target_object_id).or_default();
                            apply_pointer_update(pointers, &update);
                        }
                        TargetKind::Rail => {
                            if !seen_rails.contains(&update.target_object_id) {
                                return Err(ReplayError::new(
                                    "missing_target",
                                    format!(
                                        "challenge_finalize_verdict target rail missing: {}",
                                        update.target_object_id
                                    ),
                                ));
                            }
                            let pointers =
                                rail_pointers.entry(update.target_object_id).or_default();
                            apply_pointer_update(pointers, &update);
                        }
                    }
                }
            }
            _ => {
                return Err(ReplayError::new(
                    "unsupported_event_type",
                    format!("event type not supported: {}", event.kind),
                ));
            }
        }

        if event.kind == "challenge_finalize_verdict" {
            cycle_verdict_count = cycle_verdict_count.saturating_add(1);
        }
        seen_event_ids.insert(row.event_id);
    }

    for idea in &ideas {
        let base_payload = payload_by_idea.get(&idea.idea_id).ok_or_else(|| {
            ReplayError::new(
                "missing_payload",
                format!("missing payload for idea_id={}", idea.idea_id),
            )
        })?;
        let pointers = idea_pointers
            .get(&idea.idea_id)
            .cloned()
            .unwrap_or_default();
        let mut title = base_payload.title.clone();
        let mut sentence = base_payload.sentence.clone();
        let mut title_payload_hash = None;
        let mut sentence_payload_hash = None;

        if let Some(representation_id) = pointers.title_representation_id {
            let resolved = resolve_representation_payload(
                &representations,
                representation_id,
                TargetKind::Idea,
                idea.idea_id,
                TierEnum::Title,
            )?;
            title = Some(resolved.0);
            title_payload_hash = Some(resolved.1);
        }
        if let Some(representation_id) = pointers.sentence_representation_id {
            let resolved = resolve_representation_payload(
                &representations,
                representation_id,
                TargetKind::Idea,
                idea.idea_id,
                TierEnum::Sentence,
            )?;
            sentence = Some(resolved.0);
            sentence_payload_hash = Some(resolved.1);
        }

        payloads.push(ReplayPayloadRow {
            object_kind: ReplayObjectKind::Idea,
            object_id: idea.idea_id,
            title,
            sentence,
            paragraph: base_payload.paragraph.clone(),
            full: base_payload.full.clone(),
            payload_hash: base_payload.payload_hash.clone(),
            title_payload_hash,
            sentence_payload_hash,
        });
    }

    for rail in &mut rails {
        let pointers = rail_pointers
            .get(&rail.rail_id)
            .cloned()
            .unwrap_or_default();
        let title_representation_id = pointers
            .title_representation_id
            .or(rail.title_representation_id)
            .ok_or_else(|| {
                ReplayError::new(
                    "missing_payload",
                    format!("missing title representation for rail_id={}", rail.rail_id),
                )
            })?;
        let sentence_representation_id = pointers
            .sentence_representation_id
            .or(rail.sentence_representation_id)
            .ok_or_else(|| {
                ReplayError::new(
                    "missing_payload",
                    format!(
                        "missing sentence representation for rail_id={}",
                        rail.rail_id
                    ),
                )
            })?;

        let title = resolve_representation_payload(
            &representations,
            title_representation_id,
            TargetKind::Rail,
            rail.rail_id,
            TierEnum::Title,
        )?;
        let sentence = resolve_representation_payload(
            &representations,
            sentence_representation_id,
            TargetKind::Rail,
            rail.rail_id,
            TierEnum::Sentence,
        )?;

        rail.title_representation_id = Some(title_representation_id);
        rail.sentence_representation_id = Some(sentence_representation_id);

        payloads.push(ReplayPayloadRow {
            object_kind: ReplayObjectKind::Rail,
            object_id: rail.rail_id,
            title: Some(title.0),
            sentence: Some(sentence.0),
            paragraph: None,
            full: None,
            payload_hash: None,
            title_payload_hash: Some(title.1),
            sentence_payload_hash: Some(sentence.1),
        });
    }

    let observed_work =
        cycle_verdict_count + i64::try_from(cycle_distinct_voters.len()).unwrap_or(0);
    let closure_predicate_satisfied = tempo_state.cycle_age_ge_dmin
        && (observed_work >= w_target || tempo_state.cycle_age_ge_dmax);
    let current_height = events.last().map(|row| row.block_height).unwrap_or(0);
    let cycle_status = ReplayCycleStatus {
        cycle_index,
        h_start,
        current_height,
        w_target,
        observed_work,
        cycle_age_ge_dmin: tempo_state.cycle_age_ge_dmin,
        cycle_age_ge_dmax: tempo_state.cycle_age_ge_dmax,
        closure_predicate_satisfied,
        last_cycle_close_height,
    };
    let tempo_status = ReplayTempoStatus {
        cycle_age_ge_dmin: tempo_state.cycle_age_ge_dmin,
        cycle_age_ge_dmax: tempo_state.cycle_age_ge_dmax,
        constrained_mode: tempo_state.constrained_mode,
        record_only_mode: tempo_state.record_only_mode,
    };

    Ok(ApplyResult {
        ideas,
        rails,
        connections,
        payloads,
        cycle_status,
        tempo_status,
    })
}

pub(super) fn resolve_representation_payload(
    representations: &HashMap<Uuid, ReplayRepresentation>,
    representation_id: Uuid,
    expected_kind: TargetKind,
    expected_target_id: Uuid,
    expected_tier: TierEnum,
) -> Result<(String, String), ReplayError> {
    let representation = representations.get(&representation_id).ok_or_else(|| {
        ReplayError::new(
            "missing_representation",
            format!("representation not found {}", representation_id),
        )
    })?;
    if representation.representation_id != representation_id {
        return Err(ReplayError::new(
            "invalid_field",
            format!(
                "representation identity mismatch for representation_id={}",
                representation_id
            ),
        ));
    }
    if representation.target_kind != expected_kind
        || representation.target_id != expected_target_id
        || representation.tier_enum != expected_tier
    {
        return Err(ReplayError::new(
            "invalid_field",
            format!(
                "representation target mismatch representation_id={}",
                representation_id
            ),
        ));
    }
    let text = representation.payload_text.clone().ok_or_else(|| {
        ReplayError::new(
            "missing_payload",
            format!(
                "representation payload_text missing representation_id={}",
                representation_id
            ),
        )
    })?;
    Ok((text, representation.payload_hash.clone()))
}

pub(super) fn apply_pointer_update(
    pointers: &mut PointerSlots,
    update: &RepresentationPointerUpdate,
) {
    match update.tier_enum {
        TierEnum::Title => {
            pointers.title_representation_id = Some(update.representation_id);
        }
        TierEnum::Sentence => {
            pointers.sentence_representation_id = Some(update.representation_id);
        }
        TierEnum::Paragraph | TierEnum::Full => {}
    }
}
