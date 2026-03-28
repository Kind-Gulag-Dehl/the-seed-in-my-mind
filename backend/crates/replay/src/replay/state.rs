use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TargetKind {
    Idea,
    Rail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TierEnum {
    Title,
    Sentence,
    Paragraph,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RailKind {
    Vine,
}

#[derive(Debug, Clone, Default)]
pub(super) struct PointerSlots {
    pub(super) title_representation_id: Option<Uuid>,
    pub(super) sentence_representation_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub(super) struct ReplayRepresentation {
    pub(super) representation_id: Uuid,
    pub(super) target_kind: TargetKind,
    pub(super) target_id: Uuid,
    pub(super) tier_enum: TierEnum,
    pub(super) payload_hash: String,
    pub(super) payload_text: Option<String>,
}

#[derive(Debug, Clone)]
pub(super) struct RepresentationPointerUpdate {
    pub(super) target_kind: TargetKind,
    pub(super) target_object_id: Uuid,
    pub(super) tier_enum: TierEnum,
    pub(super) representation_id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct ReplayPosition {
    pub(super) block_height: i64,
    pub(super) event_index: i32,
}

impl ReplayPosition {
    pub(super) fn from_event(row: &EventRow) -> Self {
        Self {
            block_height: row.block_height,
            event_index: row.event_index,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct TempoState {
    pub(super) cycle_age_ge_dmin: bool,
    pub(super) cycle_age_ge_dmax: bool,
    pub(super) constrained_mode: bool,
    pub(super) record_only_mode: bool,
}

impl Default for TempoState {
    fn default() -> Self {
        Self {
            cycle_age_ge_dmin: false,
            cycle_age_ge_dmax: false,
            constrained_mode: false,
            record_only_mode: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CycleClosureKind {
    Deliberative,
    Forced,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct ImportanceChallengeKey {
    pub(super) context_key: String,
    pub(super) target_left_idea_id: Uuid,
    pub(super) target_right_idea_id: Uuid,
    pub(super) reference_idea_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub(super) struct ImportanceChallengeState {
    pub(super) key: ImportanceChallengeKey,
    pub(super) created_by_identity_id: Uuid,
    pub(super) axis: String,
    pub(super) timeframe: String,
    pub(super) scope: String,
    pub(super) created_cycle_index: i64,
    pub(super) terminal: bool,
    pub(super) finalized: bool,
}

#[derive(Debug, Clone)]
pub(super) struct VoteSessionState {
    pub(super) challenge_id: Uuid,
    pub(super) voter_identity_id: Uuid,
}

#[derive(Debug, Clone)]
pub(super) struct ChallengeVoteState {
    pub(super) vote_choice: String,
    pub(super) vote_event_id: Uuid,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct WriterVerificationState {
    pub(super) email_verified: bool,
    pub(super) canonical_writer_level: i16,
}

#[derive(Debug)]
pub(super) struct ApplyResult {
    pub(super) ideas: Vec<ReplayIdeaRow>,
    pub(super) rails: Vec<ReplayRailRow>,
    pub(super) connections: Vec<ReplayConnectionRow>,
    pub(super) payloads: Vec<ReplayPayloadRow>,
    pub(super) cycle_status: ReplayCycleStatus,
    pub(super) tempo_status: ReplayTempoStatus,
}
