use super::state::*;
use super::*;

pub(super) fn deterministic_vote_session_candidates(
    voter_identity_id: Uuid,
    current_cycle_index: i64,
    challenges_by_id: &HashMap<Uuid, ImportanceChallengeState>,
    challenge_votes: &HashMap<Uuid, Vec<ChallengeVoteState>>,
    vote_by_challenge_and_voter: &HashMap<(Uuid, Uuid), Uuid>,
    session_by_challenge_and_voter: &HashMap<(Uuid, Uuid), Uuid>,
) -> Vec<Uuid> {
    let mut candidates = Vec::new();
    for (challenge_id, challenge) in challenges_by_id {
        if challenge.terminal || challenge.finalized {
            continue;
        }
        if challenge.created_by_identity_id == voter_identity_id {
            continue;
        }
        if current_cycle_index < challenge.created_cycle_index + CHALLENGE_ARGUMENT_PHASE_CYCLES {
            continue;
        }
        let vote_count = challenge_votes
            .get(challenge_id)
            .map(|rows| rows.len())
            .unwrap_or(0);
        if vote_count >= TARGET_JUROR_COUNT {
            continue;
        }
        if vote_by_challenge_and_voter.contains_key(&(*challenge_id, voter_identity_id)) {
            continue;
        }
        if session_by_challenge_and_voter.contains_key(&(*challenge_id, voter_identity_id)) {
            continue;
        }
        candidates.push(*challenge_id);
    }
    candidates.sort();
    candidates
}

pub(super) fn deterministic_vote_session_index(
    voter_identity_id: Uuid,
    selection_boundary_event_id: Uuid,
    cycle_index: i64,
    candidate_count: usize,
) -> usize {
    if candidate_count <= 1 {
        return 0;
    }
    let mut input = Vec::with_capacity(16 + 16 + 8);
    input.extend_from_slice(&voter_identity_id.as_u128().to_be_bytes());
    input.extend_from_slice(&selection_boundary_event_id.as_u128().to_be_bytes());
    input.extend_from_slice(&cycle_index.to_be_bytes());
    let digest = hash_bytes(&input);
    let mut top = [0_u8; 8];
    top.copy_from_slice(&digest[..8]);
    let value = u64::from_be_bytes(top);
    (value % candidate_count as u64) as usize
}

pub(super) fn aggregate_importance_verdict_for_replay(
    votes: &[ChallengeVoteState],
    challenge: &ImportanceChallengeState,
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
            Some(challenge.key.target_left_idea_id),
            left_votes,
            right_votes,
            total_votes,
        )
    } else if right_votes >= 2 {
        (
            "right",
            Some(challenge.key.target_right_idea_id),
            left_votes,
            right_votes,
            total_votes,
        )
    } else {
        ("no_change", None, left_votes, right_votes, total_votes)
    }
}
