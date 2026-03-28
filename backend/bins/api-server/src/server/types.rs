#![cfg_attr(not(feature = "full"), allow(dead_code))]

use serde::Deserialize;
use storage::Storage;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) storage: Storage,
}

#[derive(Deserialize)]
pub(crate) struct SnapshotLatestQuery {
    pub(crate) include_preview: Option<bool>,
}

#[derive(Deserialize)]
pub(crate) struct SnapshotByHeightPath {
    pub(crate) height: String,
}

#[derive(Deserialize)]
pub(crate) struct SnapshotCommitListQuery {
    pub(crate) limit: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct SnapshotCommitByHeightPath {
    pub(crate) height: String,
}

#[derive(Deserialize)]
pub(crate) struct IdeasTopQuery {
    pub(crate) limit: Option<String>,
    pub(crate) offset: Option<String>,
    pub(crate) order: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct SearchIdeasQuery {
    pub(crate) q: Option<String>,
    pub(crate) limit: Option<String>,
    pub(crate) offset: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct NeighborhoodQuery {
    pub(crate) depth: Option<String>,
    pub(crate) limit_per_hop: Option<String>,
    pub(crate) ri_dir: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct RelativeImportanceConnectionsQuery {
    pub(crate) idea_ids: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AuthPayload {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PrivateIdeaPayload {
    pub(crate) title: String,
    pub(crate) sentence: String,
    pub(crate) paragraph: Option<String>,
    pub(crate) full: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum VineTypeInput {
    String(String),
    Number(i64),
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PrivateVineItemPayload {
    pub(crate) idea_id: String,
    pub(crate) via_connection_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PrivateVineCreatePayload {
    pub(crate) vine_type: VineTypeInput,
    pub(crate) title: Option<String>,
    pub(crate) sentence: Option<String>,
    pub(crate) paragraph: Option<String>,
    pub(crate) full: Option<String>,
    pub(crate) items: Vec<PrivateVineItemPayload>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PrivateVineUpdatePayload {
    pub(crate) vine_type: Option<VineTypeInput>,
    pub(crate) title: Option<Option<String>>,
    pub(crate) sentence: Option<Option<String>>,
    pub(crate) paragraph: Option<Option<String>>,
    pub(crate) full: Option<Option<String>>,
    pub(crate) items: Option<Vec<PrivateVineItemPayload>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PrivateAiDraftPayload {
    pub(crate) raw_text: String,
    pub(crate) context: Option<PrivateAiDraftContext>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CanonicalIdeaCreatePayload {
    pub(crate) idea_id: Option<String>,
    pub(crate) event_id: Option<String>,
    pub(crate) idea_type: String,
    pub(crate) title: String,
    pub(crate) sentence: String,
    pub(crate) paragraph: Option<String>,
    pub(crate) full: Option<String>,
    pub(crate) author_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CanonicalConnectionCreatePayload {
    pub(crate) connection_id: Option<String>,
    pub(crate) event_id: Option<String>,
    pub(crate) from_idea_id: String,
    pub(crate) to_idea_id: String,
    pub(crate) connection_type: String,
    pub(crate) usage: Option<String>,
    pub(crate) axis: Option<String>,
    pub(crate) timeframe: Option<String>,
    pub(crate) scope: Option<String>,
    pub(crate) author_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CanonicalImportanceChallengeCreatePayload {
    pub(crate) challenge_id: Option<String>,
    pub(crate) event_id: Option<String>,
    pub(crate) framing_representation_ref: String,
    pub(crate) context_key: String,
    pub(crate) axis: String,
    pub(crate) timeframe: String,
    pub(crate) scope: String,
    pub(crate) target_left_idea_id: String,
    pub(crate) target_right_idea_id: String,
    pub(crate) reference_idea_id: Option<String>,
    pub(crate) author_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CanonicalImportanceArgumentAttachPayload {
    pub(crate) connection_id: Option<String>,
    pub(crate) event_id: Option<String>,
    pub(crate) argument_idea_id: String,
    pub(crate) subject_idea_id: String,
    pub(crate) author_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CanonicalVoteSessionPullPayload {
    pub(crate) vote_session_id: Option<String>,
    pub(crate) event_id: Option<String>,
    pub(crate) author_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CanonicalChallengeVoteCastPayload {
    pub(crate) vote_session_id: String,
    pub(crate) vote_choice: String,
    pub(crate) event_id: Option<String>,
    pub(crate) author_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CanonicalVerifierGrantPayload {
    pub(crate) identity_id: String,
    pub(crate) canonical_writer_level: Option<String>,
    pub(crate) email_verified: Option<bool>,
    pub(crate) event_id: Option<String>,
    pub(crate) author_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CanonicalVerifierRevokePayload {
    pub(crate) identity_id: String,
    pub(crate) event_id: Option<String>,
    pub(crate) author_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CanonicalBlockedSubmissionPayload {
    pub(crate) submission_hash: String,
    pub(crate) blocked_reason_code: String,
    pub(crate) blocked_by_identity: String,
    pub(crate) reference_event_id: Option<String>,
    pub(crate) event_id: Option<String>,
    pub(crate) author_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CanonicalIdentityCreatePayload {
    pub(crate) identity_id: Option<String>,
    pub(crate) event_id: Option<String>,
    pub(crate) identity_name: String,
    pub(crate) public_key: String,
    pub(crate) metadata: Option<String>,
    pub(crate) author_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PrivateAiDraftContext {
    pub(crate) title: Option<String>,
    pub(crate) sentence: Option<String>,
    pub(crate) paragraph: Option<String>,
    pub(crate) full: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct AuthenticatedAccount {
    pub(crate) account_id: Uuid,
    pub(crate) username: String,
}

#[derive(Clone, Copy)]
pub(crate) enum IdeasTopOrder {
    Asc,
    Desc,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RelativeImportanceDirection {
    Incoming,
    Outgoing,
    Both,
}
