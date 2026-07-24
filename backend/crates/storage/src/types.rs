use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Storage {
    pub(crate) pool: PgPool,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct SnapshotRow {
    pub snapshot_id: Uuid,
    pub block_height: i64,
    pub format_version: String,
    pub snapshot_hash: String,
    pub prev_snapshot_hash: Option<String>,
    pub state_root_hash: Option<String>,
    pub title_sentence_payload_root: Option<String>,
    pub shared_map_commitment: Option<String>,
    pub active_rulebook_set_hash: Option<String>,
    pub last_event_id: Option<Uuid>,
    pub event_count: Option<i64>,
    pub approximate_timestamp: Option<DateTime<Utc>>,
    pub cycle_index: Option<i64>,
    pub cycle_close_height: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct SnapshotCommitRow {
    pub block_height: i64,
    pub snapshot_hash: String,
    pub state_root_hash: String,
    pub title_sentence_payload_root: String,
    pub shared_map_commitment: String,
    pub last_event_id: Uuid,
    pub event_count: i64,
    pub active_rulebook_set_hash: String,
    pub created_event_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct IdeaSummaryRow {
    pub idea_id: Uuid,
    pub idea_type: String,
    pub is_personal_space_organizer: bool,
    pub speaker_identity_id: Uuid,
    pub speaker_identity_title: Option<String>,
    pub created_event_id: Uuid,
    pub created_block_height: i64,
    pub created_event_index: i32,
    pub title: Option<String>,
    pub sentence: Option<String>,
    pub derived_universal_rank: Option<i64>,
    pub ri_in_count: i64,
    pub ri_out_count: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct IdeaDetailRow {
    pub idea_id: Uuid,
    pub idea_type: String,
    pub is_personal_space_organizer: bool,
    pub speaker_identity_id: Uuid,
    pub speaker_identity_title: Option<String>,
    pub created_event_id: Uuid,
    pub created_block_height: i64,
    pub created_event_index: i32,
    pub is_identity_idea: bool,
    pub underlying_identity_id: Option<Uuid>,
    pub title: Option<String>,
    pub sentence: Option<String>,
    pub payload_hash: Option<String>,
    pub derived_universal_rank: Option<i64>,
    pub ri_in_count: i64,
    pub ri_out_count: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct ConnectionRow {
    pub connection_id: Uuid,
    pub from_idea_id: Uuid,
    pub to_idea_id: Uuid,
    pub connection_type: String,
    pub usage: Option<String>,
    pub axis: Option<String>,
    pub timeframe: Option<String>,
    pub scope: Option<String>,
    pub created_by_event_id: Uuid,
    pub created_block_height: i64,
    pub created_event_index: i32,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AccountRow {
    pub account_id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub canonical_identity_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct SessionRow {
    pub session_id: Uuid,
    pub account_id: Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct IdentityRow {
    pub identity_id: Uuid,
    pub title: String,
    pub created_event_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PrivateIdeaRow {
    pub idea_id: Uuid,
    pub owner_account_id: Uuid,
    pub title: String,
    pub sentence: String,
    pub paragraph: Option<String>,
    pub full: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CanonicalOrderingRow {
    pub ordering_id: Uuid,
    pub ordering_profile: i16,
    pub vine_type: Option<i16>,
    pub author_identity_id: Uuid,
    pub title_representation_id: Option<Uuid>,
    pub sentence_representation_id: Option<Uuid>,
    pub title_payload_hash: Option<String>,
    pub sentence_payload_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CanonicalOrderingItemRow {
    pub idx: i32,
    pub idea_id: Uuid,
    pub via_connection_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CanonicalOrderingSummaryRow {
    pub ordering_id: Uuid,
    pub ordering_profile: i16,
    pub vine_type: Option<i16>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PrivateOrderingRow {
    pub private_ordering_id: Uuid,
    pub owner_account_id: Uuid,
    pub ordering_profile: i16,
    pub vine_type: Option<i16>,
    pub title: Option<String>,
    pub sentence: Option<String>,
    pub paragraph: Option<String>,
    pub full: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PrivateOrderingListRow {
    pub private_ordering_id: Uuid,
    pub ordering_profile: i16,
    pub vine_type: Option<i16>,
    pub title: Option<String>,
    pub sentence: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub item_count: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PrivateOrderingItemRow {
    pub private_ordering_id: Uuid,
    pub idx: i32,
    pub idea_id: Uuid,
    pub via_connection_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub struct PrivateOrderingItemInput {
    pub idx: i32,
    pub idea_id: Uuid,
    pub via_connection_id: Option<Uuid>,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct CountRow {
    pub(crate) total: i64,
}

pub(crate) const BUILD_MANA_CYCLE_CAP: i64 = 20;
pub(crate) const IDEA_CREATE_MANA_COST: i64 = 1;
pub(crate) const CONNECTION_CREATE_MANA_COST: i64 = 1;
pub(crate) const CHALLENGE_CREATE_MANA_COST: i64 = 1;
pub(crate) const CHALLENGE_ARGUMENT_ATTACH_MANA_COST: i64 = 1;
pub(crate) const VOTE_SESSION_OPEN_MANA_COST: i64 = 1;
pub(crate) const CHALLENGE_ARGUMENT_PHASE_CYCLES: i64 = 1;
pub(crate) const VOTING_MANA_CYCLE_CAP: i64 = 3;
pub(crate) const TARGET_JUROR_COUNT: i64 = 3;
pub(crate) const MIN_CANONICAL_WRITER_LEVEL: i16 = 1;
pub(crate) const SEED_BOOTSTRAP_VERIFIER_ID_STR: &str = "380b7817-db3b-7b76-8cf3-87df879ddddb";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalWriteError {
    pub code: &'static str,
    pub message: String,
}

impl CanonicalWriteError {
    pub(crate) fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for CanonicalWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for CanonicalWriteError {}

#[derive(Debug, Clone)]
pub struct CanonicalIdeaCreateInput {
    pub idea_id: Uuid,
    pub event_id: Option<Uuid>,
    pub idea_type: String,
    pub title: String,
    pub sentence: String,
    pub paragraph: Option<String>,
    pub full: Option<String>,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CanonicalConnectionCreateInput {
    pub connection_id: Uuid,
    pub event_id: Option<Uuid>,
    pub from_idea_id: Uuid,
    pub to_idea_id: Uuid,
    pub connection_type: String,
    pub usage: Option<String>,
    pub axis: Option<String>,
    pub timeframe: Option<String>,
    pub scope: Option<String>,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CanonicalIdeaCreateResult {
    pub idea_id: Uuid,
    pub event_id: Uuid,
    pub cycle_index: i64,
    pub remaining_build_mana: i64,
}

#[derive(Debug, Clone)]
pub struct CanonicalConnectionCreateResult {
    pub connection_id: Uuid,
    pub event_id: Uuid,
    pub cycle_index: i64,
    pub remaining_build_mana: i64,
}

#[derive(Debug, Clone)]
pub struct SignedCanonicalCandidateInput {
    pub signature_profile: String,
    pub event_id: Uuid,
    pub event_type: String,
    pub author_identity_id: Uuid,
    pub speaker_identity_id: Option<Uuid>,
    pub public_key_ref: String,
    pub payload_hash: String,
    pub payload_binding_mode: String,
    pub payload_ref: Option<Vec<u8>>,
    pub author_observed_at: Option<String>,
    pub signature: String,
    pub payload: Value,
}

#[derive(Debug, Clone)]
pub struct SignedCanonicalWriteResult {
    pub event_id: Uuid,
    pub event_type: String,
    pub block_height: i64,
    pub event_index: i32,
    pub authored_candidate_hash_v0: String,
    pub object_type: String,
    pub object_id: Uuid,
    pub idempotent: bool,
    pub publication_profile: String,
}

#[derive(Debug, Clone)]
pub struct ProfileV0IdentityAdmissionResult {
    pub identity_id: Uuid,
    pub event_id: Uuid,
    pub block_height: i64,
    pub event_index: i32,
    pub invitation_capacity_debit_units: i64,
    pub idempotent: bool,
}

#[derive(Debug, Clone)]
pub struct CanonicalImportanceChallengeCreateInput {
    pub challenge_id: Uuid,
    pub event_id: Option<Uuid>,
    pub framing_representation_ref: Uuid,
    pub context_key: String,
    pub axis: String,
    pub timeframe: String,
    pub scope: String,
    pub target_left_idea_id: Uuid,
    pub target_right_idea_id: Uuid,
    pub reference_idea_id: Option<Uuid>,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CanonicalImportanceChallengeCreateResult {
    pub challenge_id: Uuid,
    pub event_id: Uuid,
    pub cycle_index: i64,
    pub remaining_build_mana: i64,
}

#[derive(Debug, Clone)]
pub struct CanonicalImportanceArgumentAttachInput {
    pub challenge_id: Uuid,
    pub connection_id: Uuid,
    pub event_id: Option<Uuid>,
    pub argument_idea_id: Uuid,
    pub subject_idea_id: Uuid,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CanonicalImportanceArgumentAttachResult {
    pub challenge_id: Uuid,
    pub connection_id: Uuid,
    pub event_id: Uuid,
    pub cycle_index: i64,
    pub remaining_build_mana: i64,
}

#[derive(Debug, Clone)]
pub struct CanonicalVoteSessionPullInput {
    pub vote_session_id: Uuid,
    pub event_id: Option<Uuid>,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CanonicalVoteSessionPullResult {
    pub vote_session_id: Uuid,
    pub challenge_id: Uuid,
    pub event_id: Uuid,
    pub session_index: i64,
    pub cycle_index: i64,
    pub remaining_voting_mana: i64,
}

#[derive(Debug, Clone)]
pub struct CanonicalVoteCastInput {
    pub challenge_id: Uuid,
    pub vote_session_id: Uuid,
    pub vote_choice: String,
    pub event_id: Option<Uuid>,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CanonicalVoteCastResult {
    pub challenge_id: Uuid,
    pub vote_event_id: Uuid,
    pub cycle_index: i64,
    pub remaining_voting_mana: i64,
    pub verdict_event_id: Option<Uuid>,
    pub verdict_outcome: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CanonicalVerifierGrantInput {
    pub identity_id: Uuid,
    pub canonical_writer_level: i16,
    pub email_verified: bool,
    pub event_id: Option<Uuid>,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CanonicalVerifierGrantResult {
    pub identity_id: Uuid,
    pub event_id: Uuid,
    pub canonical_writer_level: i16,
    pub email_verified: bool,
    pub cycle_index: i64,
}

#[derive(Debug, Clone)]
pub struct CanonicalVerifierRevokeInput {
    pub identity_id: Uuid,
    pub event_id: Option<Uuid>,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CanonicalVerifierRevokeResult {
    pub identity_id: Uuid,
    pub event_id: Uuid,
    pub cycle_index: i64,
}

#[derive(Debug, Clone)]
pub struct CanonicalBlockedSubmissionInput {
    pub event_id: Option<Uuid>,
    pub submission_hash: String,
    pub blocked_reason_code: String,
    pub blocked_by_identity: Uuid,
    pub safe_summary_ref: String,
    pub classifier_profile_ref: String,
    pub rulebook_ref: String,
    pub reference_event_id: Option<Uuid>,
    pub wrongful_block_challenge_ref: Option<Uuid>,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CanonicalBlockedSubmissionResult {
    pub event_id: Uuid,
    pub cycle_index: i64,
}

#[derive(Debug, Clone)]
pub struct CanonicalIdentityCreateInput {
    pub identity_id: Uuid,
    pub event_id: Option<Uuid>,
    pub identity_name: String,
    pub public_key: String,
    pub metadata: Option<String>,
    pub author_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CanonicalIdentityCreateResult {
    pub identity_id: Uuid,
    pub event_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct CanonicalVerificationStatus {
    pub identity_id: Uuid,
    pub email_verified: bool,
    pub canonical_writer_level: i16,
    pub active_verifier: bool,
    pub last_updated_event_id: Option<Uuid>,
    pub last_updated_block_height: Option<i64>,
    pub last_updated_event_index: Option<i32>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CanonicalChallengeRow {
    pub challenge_id: Uuid,
    pub challenge_domain: String,
    pub context_key: String,
    pub axis: String,
    pub timeframe: String,
    pub scope: String,
    pub target_left_idea_id: Uuid,
    pub target_right_idea_id: Uuid,
    pub reference_idea_id: Option<Uuid>,
    pub framing_representation_ref: Uuid,
    pub created_by_identity_id: Uuid,
    pub created_block_height: i64,
    pub created_event_index: i32,
    pub created_event_id: Uuid,
    pub created_cycle_index: i64,
    pub lifecycle_state: i16,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CanonicalChallengeArgumentRow {
    pub challenge_id: Uuid,
    pub connection_id: Uuid,
    pub argument_idea_id: Uuid,
    pub subject_idea_id: Uuid,
    pub created_block_height: i64,
    pub created_event_index: i32,
    pub created_event_id: Uuid,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CanonicalChallengeVoteRow {
    pub challenge_id: Uuid,
    pub voter_identity_id: Uuid,
    pub vote_session_id: Uuid,
    pub vote_choice: String,
    pub cast_block_height: i64,
    pub cast_event_index: i32,
    pub cast_event_id: Uuid,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CanonicalChallengeVerdictRow {
    pub verdict_id: Uuid,
    pub challenge_id: Uuid,
    pub verdict_event_id: Uuid,
    pub winning_choice: String,
    pub winning_target_idea_id: Option<Uuid>,
    pub left_votes: i16,
    pub right_votes: i16,
    pub total_votes: i16,
    pub resolved_block_height: i64,
    pub resolved_event_index: i32,
}

#[derive(Debug, Clone)]
pub struct CanonicalChallengeDetail {
    pub challenge: CanonicalChallengeRow,
    pub arguments: Vec<CanonicalChallengeArgumentRow>,
    pub votes: Vec<CanonicalChallengeVoteRow>,
    pub verdict: Option<CanonicalChallengeVerdictRow>,
    pub current_cycle_index: i64,
    pub phase: String,
}

#[derive(Debug, Clone, Copy, Serialize, FromRow)]
pub struct IdentityWriterVerificationStateRow {
    pub identity_id: Uuid,
    pub email_verified: bool,
    pub canonical_writer_level: i16,
    pub granted_by_identity_id: Uuid,
    pub source_event_id: Uuid,
    pub source_block_height: i64,
    pub source_event_index: i32,
}

#[derive(Debug, Clone, Copy, FromRow)]
pub(crate) struct VerifierRoleAssignmentRow {
    pub(crate) is_active: bool,
}

#[derive(Debug, Clone, Copy, FromRow)]
pub(crate) struct CycleWindowRow {
    pub(crate) max_cycle_index: Option<i64>,
    pub(crate) max_closure_height: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, FromRow)]
pub(crate) struct ChallengeVoteSessionCandidateRow {
    pub(crate) challenge_id: Uuid,
    pub(crate) created_by_identity_id: Uuid,
    pub(crate) created_cycle_index: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, FromRow)]
pub(crate) struct VoteSessionRow {
    pub(crate) vote_session_id: Uuid,
    pub(crate) challenge_id: Uuid,
    pub(crate) voter_identity_id: Uuid,
    pub(crate) session_index: i64,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct ChallengeVoteChoiceRow {
    pub(crate) voter_identity_id: Uuid,
    pub(crate) vote_choice: String,
    pub(crate) cast_event_id: Uuid,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, FromRow)]
pub(crate) struct ChallengeVoteContextRow {
    pub(crate) challenge_id: Uuid,
    pub(crate) created_by_identity_id: Uuid,
    pub(crate) target_left_idea_id: Uuid,
    pub(crate) target_right_idea_id: Uuid,
    pub(crate) created_cycle_index: i64,
}
