use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub error_code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RailItem {
    pub idx: String,
    pub idea_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_connection_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalRailRepresentations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_representation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_payload_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentence_representation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentence_payload_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalRailSummary {
    pub rail_id: String,
    pub rail_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vine_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalRailDetail {
    pub rail_id: String,
    pub rail_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vine_type: Option<String>,
    pub author_identity_id: String,
    pub canonical_representations: CanonicalRailRepresentations,
    pub items: Vec<RailItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalRailResponse {
    pub rail: CanonicalRailDetail,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalRailsResponse {
    pub rails: Vec<CanonicalRailSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthorInfo {
    pub author_identity_id: Option<String>,
    pub author_identity_title: Option<String>,
    pub verification_level: Option<String>,
    pub persona_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IdeaSummary {
    pub idea_id: String,
    pub idea_type: String,
    pub is_personal_space_organizer: bool,
    pub speaker_identity_id: String,
    pub speaker_identity_title: Option<String>,
    pub created_event_id: String,
    pub title: String,
    pub sentence: Option<String>,
    pub derived_universal_rank: Option<String>,
    pub ri_in_count: String,
    pub ri_out_count: String,
    pub author: AuthorInfo,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectionSummary {
    pub connection_id: String,
    pub from_idea_id: String,
    pub to_idea_id: String,
    pub connection_type: String,
    pub created_by_event_id: String,
    pub usage: Option<String>,
    pub axis: Option<String>,
    pub timeframe: Option<String>,
    pub scope: Option<String>,
    pub value_representation: Option<String>,
    pub certainty_band: Option<String>,
    pub weight: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IdeaDetail {
    pub idea_id: String,
    pub idea_type: String,
    pub is_personal_space_organizer: bool,
    pub speaker_identity_id: String,
    pub speaker_identity_title: Option<String>,
    pub created_event_id: String,
    pub title: String,
    pub sentence: Option<String>,
    pub derived_universal_rank: Option<String>,
    pub ri_in_count: String,
    pub ri_out_count: String,
    pub derived_universal_axis_ranks: Option<BTreeMap<String, String>>,
    pub author: AuthorInfo,
    pub payload_hash: String,
    pub incoming_connections: Vec<ConnectionSummary>,
    pub outgoing_connections: Vec<ConnectionSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NeighborhoodResponse {
    pub central_idea: IdeaDetail,
    pub adjacent_ideas: Vec<IdeaSummary>,
    pub connections: Vec<ConnectionSummary>,
    pub depth_reached: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotMetadata {
    pub snapshot_id: String,
    pub height: String,
    pub snapshot_hash: String,
    pub state_root_hash: String,
    pub title_sentence_payload_root: String,
    pub shared_map_commitment: String,
    pub prev_snapshot_hash: Option<String>,
    pub event_count: String,
    pub approximate_timestamp: String,
    pub cycle_index: Option<String>,
    pub cycle_close_height: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotLatestResponse {
    pub snapshot: SnapshotMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_ideas: Option<Vec<IdeaSummary>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotResponse {
    pub snapshot: SnapshotMetadata,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotCommitMetadata {
    pub block_height: String,
    pub snapshot_id: String,
    pub snapshot_hash: String,
    pub state_root_hash: String,
    pub title_sentence_payload_root: String,
    pub shared_map_commitment: String,
    pub last_event_id: String,
    pub event_count: String,
    pub active_rulebook_set_hash: String,
    pub created_event_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotCommitResponse {
    pub commit: SnapshotCommitMetadata,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotCommitListResponse {
    pub commits: Vec<SnapshotCommitMetadata>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IdeasTopResponse {
    pub ideas: Vec<IdeaSummary>,
    pub total: String,
    pub offset: String,
    pub limit: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchIdeasResponse {
    pub results: Vec<IdeaSummary>,
    pub total: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RelativeImportanceConnectionsResponse {
    pub connections: Vec<ConnectionSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IdentityInfo {
    pub identity_id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IdentityResponse {
    pub identity: IdentityInfo,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalCycleStatus {
    pub cycle_index: String,
    pub h_start: String,
    pub current_height: String,
    pub w_target: String,
    pub observed_work: String,
    pub cycle_age_ge_dmin: bool,
    pub cycle_age_ge_dmax: bool,
    pub closure_predicate_satisfied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_cycle_close_height: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalCycleStatusResponse {
    pub cycle: CanonicalCycleStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalTempoStatus {
    pub cycle_age_ge_dmin: bool,
    pub cycle_age_ge_dmax: bool,
    pub constrained_mode: bool,
    pub record_only_mode: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalTempoStatusResponse {
    pub tempo: CanonicalTempoStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalEventLogEvent {
    pub event_id: String,
    pub global_index: String,
    pub block_height: String,
    pub block_event_index: String,
    pub event_type: String,
    pub authorship_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_identity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_identity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_binding_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authored_candidate_hash_v0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_profile: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalEventLogBlockBand {
    pub id: String,
    pub block_height: String,
    pub start_global_index: String,
    pub end_global_index: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalEventLogCycleBand {
    pub id: String,
    pub cycle_index: String,
    pub start_global_index: String,
    pub end_global_index: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closure_event_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalEventLogResponse {
    pub events: Vec<CanonicalEventLogEvent>,
    pub blocks: Vec<CanonicalEventLogBlockBand>,
    pub cycles: Vec<CanonicalEventLogCycleBand>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalIdeaCreateResponse {
    pub idea_id: String,
    pub event_id: String,
    pub cycle_index: String,
    pub remaining_build_mana: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalConnectionCreateResponse {
    pub connection_id: String,
    pub event_id: String,
    pub cycle_index: String,
    pub remaining_build_mana: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignedCanonicalCandidate {
    pub signature_profile: String,
    pub event_id: String,
    pub event_type: String,
    pub author_identity_id: String,
    pub speaker_identity_id: Option<String>,
    pub public_key_ref: String,
    pub payload_hash: String,
    pub payload_binding_mode: String,
    pub payload_ref: Option<String>,
    pub author_observed_at: Option<String>,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignedCanonicalEventSubmitRequest {
    pub candidate: SignedCanonicalCandidate,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct SignedCanonicalEventSubmitEvent {
    pub event_id: String,
    pub event_type: String,
    pub block_height: String,
    pub event_index: String,
    pub authored_candidate_hash_v0: String,
    pub publication_profile: String,
    pub idempotent: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SignedCanonicalEventSubmitObject {
    pub object_type: String,
    pub object_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SignedCanonicalEventSubmitResponse {
    pub event: SignedCanonicalEventSubmitEvent,
    pub object: SignedCanonicalEventSubmitObject,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalChallengeCreateResponse {
    pub challenge_id: String,
    pub event_id: String,
    pub cycle_index: String,
    pub remaining_build_mana: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalChallengeArgumentAttachResponse {
    pub challenge_id: String,
    pub connection_id: String,
    pub event_id: String,
    pub cycle_index: String,
    pub remaining_build_mana: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalChallengeArgumentSummary {
    pub connection_id: String,
    pub argument_idea_id: String,
    pub subject_idea_id: String,
    pub created_event_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalChallengeDetail {
    pub challenge_id: String,
    pub challenge_domain: String,
    pub context_key: String,
    pub axis: String,
    pub timeframe: String,
    pub scope: String,
    pub target_left_idea_id: String,
    pub target_right_idea_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_idea_id: Option<String>,
    pub framing_representation_ref: String,
    pub created_by_identity_id: String,
    pub created_event_id: String,
    pub created_cycle_index: String,
    pub current_cycle_index: String,
    pub phase: String,
    pub arguments: Vec<CanonicalChallengeArgumentSummary>,
    pub votes: Vec<CanonicalChallengeVoteSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict: Option<CanonicalChallengeVerdictSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalChallengeDetailResponse {
    pub challenge: CanonicalChallengeDetail,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalVoteSessionPullResponse {
    pub vote_session_id: String,
    pub challenge_id: String,
    pub event_id: String,
    pub session_index: String,
    pub cycle_index: String,
    pub remaining_voting_mana: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalVoteCastResponse {
    pub challenge_id: String,
    pub vote_event_id: String,
    pub cycle_index: String,
    pub remaining_voting_mana: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict_event_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict_outcome: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalBlockedSubmissionResponse {
    pub event_id: String,
    pub cycle_index: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalIdentityCreateResponse {
    pub identity_id: String,
    pub event_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalVerifierGrantResponse {
    pub identity_id: String,
    pub event_id: String,
    pub canonical_writer_level: String,
    pub email_verified: bool,
    pub cycle_index: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalVerifierRevokeResponse {
    pub identity_id: String,
    pub event_id: String,
    pub canonical_writer_level: String,
    pub email_verified: bool,
    pub cycle_index: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalVerificationStatus {
    pub identity_id: String,
    pub email_verified: bool,
    pub canonical_writer_level: String,
    pub active_verifier: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_event_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_block_height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_event_index: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalVerificationStatusResponse {
    pub verification: CanonicalVerificationStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalChallengeVoteSummary {
    pub vote_event_id: String,
    pub vote_session_id: String,
    pub voter_identity_id: String,
    pub vote_choice: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CanonicalChallengeVerdictSummary {
    pub verdict_id: String,
    pub verdict_event_id: String,
    pub winning_choice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winning_target_idea_id: Option<String>,
    pub left_votes: String,
    pub right_votes: String,
    pub total_votes: String,
}
