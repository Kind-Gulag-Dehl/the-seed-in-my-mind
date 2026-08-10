use crate::secret_screen::screen_json_for_secrets;
use crate::{schema::Event, snapshot_interval_blocks, system_boundary_emitter_id};
use encoding::canonical::validate_id;
use encoding::payload::payload_hash_hex;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub code: &'static str,
    pub message: String,
}

impl ValidationError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventValidationMode {
    PublicCanonical,
    Stage0Internal,
    LegacyImport,
}

pub fn validate_event(event: &Event) -> Result<(), ValidationError> {
    validate_event_with_mode(event, EventValidationMode::PublicCanonical)
}

pub fn validate_stage0_internal_event(event: &Event) -> Result<(), ValidationError> {
    validate_event_with_mode(event, EventValidationMode::Stage0Internal)
}

pub fn validate_legacy_import_event(event: &Event) -> Result<(), ValidationError> {
    validate_event_with_mode(event, EventValidationMode::LegacyImport)
}

pub fn validate_event_with_mode(
    event: &Event,
    mode: EventValidationMode,
) -> Result<(), ValidationError> {
    let event_id = event.id.to_string();
    validate_id(&event_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    if event.kind.trim().is_empty() {
        return Err(ValidationError::new(
            "invalid_event_type",
            "event type is required",
        ));
    }

    let payload = event
        .payload
        .as_object()
        .ok_or_else(|| ValidationError::new("invalid_payload", "payload must be a JSON object"))?;
    if screen_json_for_secrets(&event.payload).is_some() {
        return Err(ValidationError::new(
            "secret_detected",
            "event payload contains secret-like content",
        ));
    }
    let system_emitter = system_boundary_emitter_id();
    if event.speaker_identity_id == Some(system_emitter)
        && !is_system_boundary_event_kind(event.kind.as_str())
    {
        return Err(ValidationError::new(
            "forbidden_author",
            "system_boundary_emitter may only author boundary events",
        ));
    }

    validate_event_kind(event, payload, mode)
}

fn validate_event_kind(
    event: &Event,
    payload: &serde_json::Map<String, Value>,
    mode: EventValidationMode,
) -> Result<(), ValidationError> {
    match event.kind.as_str() {
        "genesis" | "noop" if mode != EventValidationMode::PublicCanonical => Ok(()),
        "identity_create" => validate_identity_create(payload, event.speaker_identity_id),
        "idea_create" => validate_idea_create(payload, event.speaker_identity_id),
        "connection_create" => validate_connection_create(payload, event.speaker_identity_id),
        "ordering_create" => validate_ordering_create(payload, event.speaker_identity_id),
        "ordering_fork" => validate_ordering_fork(payload, event.speaker_identity_id),
        "representation_create" => {
            validate_representation_create(payload, event.speaker_identity_id, None)
        }
        "challenge_create" => validate_challenge_create(payload, event.speaker_identity_id),
        "challenge_open_arguments"
        | "challenge_close_arguments"
        | "challenge_open_voting"
        | "challenge_close_voting" => {
            validate_challenge_id_only(payload, event.speaker_identity_id)
        }
        "vote_session_open" if mode != EventValidationMode::PublicCanonical => {
            validate_vote_session_open(payload, event.speaker_identity_id)
        }
        "vote_cast" => validate_vote_cast(payload, event.speaker_identity_id),
        "blocked_submission" => validate_blocked_submission(payload, event.speaker_identity_id),
        "canonical_writer_grant" if mode != EventValidationMode::PublicCanonical => {
            validate_canonical_writer_grant(payload, event.speaker_identity_id)
        }
        "canonical_writer_revoke" if mode != EventValidationMode::PublicCanonical => {
            validate_canonical_writer_revoke(payload, event.speaker_identity_id)
        }
        "challenge_cancel" | "challenge_supersede" => {
            validate_challenge_cancel_like(payload, event.speaker_identity_id)
        }
        "challenge_finalize_verdict" => {
            validate_challenge_finalize_verdict(payload, event.speaker_identity_id)
        }
        "cycle_close" if mode == EventValidationMode::PublicCanonical => {
            validate_cycle_close(payload, event.speaker_identity_id)
        }
        "cycle_close" => validate_legacy_cycle_close(payload, event.speaker_identity_id),
        "snapshot_commit" => validate_snapshot_commit(payload, event.speaker_identity_id),
        _ => Err(ValidationError::new(
            "unsupported_event_type",
            format!("event type not supported: {}", event.kind),
        )),
    }
}

fn is_system_boundary_event_kind(kind: &str) -> bool {
    matches!(kind, "cycle_close" | "snapshot_commit")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetKind {
    Idea,
    Ordering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RepresentationKind {
    Title,
    Description,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderingProfile {
    Vine,
    EvidenceRail,
    ActionRail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TierComplexity {
    Fundamental,
    Standard,
    Advanced,
    Canonical,
}

fn validate_idea_create(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    let speaker_identity_id = speaker_identity_id
        .ok_or_else(|| ValidationError::new("missing_field", "speaker_identity_id required"))?;

    let idea_id = require_string(payload, "idea_id")?;
    validate_id(idea_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    if let Some(idea_type) = optional_string(payload, "idea_type")? {
        let is_valid_idea_type = matches!(
            idea_type.as_str(),
            "truth_claim" | "conceptual_idea" | "actionable_idea" | "action" | "identity"
        );
        if !is_valid_idea_type {
            return Err(ValidationError::new(
                "invalid_field",
                "unsupported idea_type",
            ));
        }
    }

    validate_payload_speaker_identity(payload, speaker_identity_id)?;

    let title = require_string(payload, "title")?;
    let sentence = require_string(payload, "sentence")?;
    let paragraph = optional_string(payload, "paragraph")?;
    let full = optional_string(payload, "full")?;
    let payload_hash = require_string(payload, "payload_hash")?;
    validate_hex_64(payload_hash, "payload_hash")?;

    let expected = payload_hash_hex(title, sentence, paragraph.as_deref(), full.as_deref())
        .map_err(|err| ValidationError::new("invalid_payload_hash", err))?;

    if payload_hash != expected {
        return Err(ValidationError::new(
            "invalid_payload_hash",
            "payload_hash does not match canonical hash",
        ));
    }

    Ok(())
}

fn validate_identity_create(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    let speaker_identity_id = speaker_identity_id
        .ok_or_else(|| ValidationError::new("missing_field", "speaker_identity_id required"))?;

    let identity_id = require_string(payload, "identity_id")?;
    validate_id(identity_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let title = require_string(payload, "title")?;
    if title.trim().is_empty() {
        return Err(ValidationError::new(
            "invalid_field",
            "title must be non-empty",
        ));
    }

    if speaker_identity_id.to_string() != identity_id {
        return Err(ValidationError::new(
            "invalid_field",
            "speaker_identity_id must match identity_id",
        ));
    }

    let _description = optional_string(payload, "description")?;
    let _public_key_ref = optional_string(payload, "initial_public_key_ref")?;
    let _verification_reference = optional_string(payload, "verification_reference")?;

    Ok(())
}

fn validate_connection_create(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    let speaker_identity_id = speaker_identity_id
        .ok_or_else(|| ValidationError::new("missing_field", "speaker_identity_id required"))?;

    let connection_id = require_string(payload, "connection_id")?;
    validate_id(connection_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    validate_payload_speaker_identity(payload, speaker_identity_id)?;

    let from_id = require_string(payload, "from_idea_id")?;
    let to_id = require_string(payload, "to_idea_id")?;
    validate_id(from_id).map_err(|err| ValidationError::new("invalid_id", err))?;
    validate_id(to_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let connection_type = require_string(payload, "connection_type")?;
    let is_valid = matches!(
        connection_type,
        "same_as" | "relative_importance" | "membership"
    );
    if !is_valid {
        return Err(ValidationError::new(
            "invalid_field",
            "unsupported connection_type",
        ));
    }

    if connection_type == "relative_importance" {
        let usage = require_string(payload, "usage")?;
        let axis = require_string(payload, "axis")?;
        let timeframe = require_string(payload, "timeframe")?;
        let scope = require_string(payload, "scope")?;
        if usage.trim().is_empty()
            || axis.trim().is_empty()
            || timeframe.trim().is_empty()
            || scope.trim().is_empty()
        {
            return Err(ValidationError::new(
                "invalid_field",
                "usage, axis, timeframe, and scope must be non-empty for relative_importance",
            ));
        }
    } else {
        optional_string(payload, "usage")?;
        optional_string(payload, "axis")?;
        optional_string(payload, "timeframe")?;
        optional_string(payload, "scope")?;
    }
    if let Some(context_challenge_id) = optional_string(payload, "context_challenge_id")? {
        validate_id(&context_challenge_id)
            .map_err(|err| ValidationError::new("invalid_id", err))?;
    }

    Ok(())
}

fn validate_payload_speaker_identity(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: uuid::Uuid,
) -> Result<(), ValidationError> {
    let Some(payload_speaker) = payload.get("speaker_identity_id") else {
        return Ok(());
    };
    let payload_speaker = payload_speaker.as_str().ok_or_else(|| {
        ValidationError::new("invalid_field", "speaker_identity_id must be string")
    })?;
    validate_id(payload_speaker).map_err(|err| ValidationError::new("invalid_id", err))?;
    if payload_speaker != speaker_identity_id.to_string() {
        return Err(ValidationError::new(
            "invalid_field",
            "payload speaker_identity_id must match event speaker_identity_id",
        ));
    }
    Ok(())
}

fn validate_ordering_create(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    ensure_speaker_identity_present(speaker_identity_id)?;

    let ordering_id = require_string(payload, "ordering_id")?;
    validate_id(ordering_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let ordering_profile = parse_ordering_profile(payload, "ordering_profile")?;
    validate_profile_vine_type(payload, ordering_profile, true)?;
    let item_count = validate_item_idea_ids(payload)?;
    validate_ordering_profile_bindings(payload, ordering_profile, item_count)?;
    validate_step_meta(payload, item_count)?;
    validate_initial_representation_refs(payload)?;

    validate_payload_speaker_identity(
        payload,
        speaker_identity_id.expect("speaker checked above"),
    )?;

    Ok(())
}

fn validate_ordering_fork(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    ensure_speaker_identity_present(speaker_identity_id)?;

    let base_ordering_id = require_string(payload, "base_ordering_id")?;
    validate_id(base_ordering_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let ordering_id = require_string(payload, "ordering_id")?;
    validate_id(ordering_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let ordering_profile = parse_ordering_profile(payload, "ordering_profile")?;
    let item_count = validate_item_idea_ids(payload)?;
    validate_ordering_profile_bindings(payload, ordering_profile, item_count)?;
    validate_step_meta(payload, item_count)?;
    validate_profile_vine_type(payload, ordering_profile, false)?;
    validate_initial_representation_refs(payload)?;

    validate_payload_speaker_identity(
        payload,
        speaker_identity_id.expect("speaker checked above"),
    )?;

    Ok(())
}

fn validate_representation_create(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
    force_target_kind: Option<TargetKind>,
) -> Result<(), ValidationError> {
    ensure_speaker_identity_present(speaker_identity_id)?;

    let representation_id = require_string(payload, "representation_id")?;
    validate_id(representation_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let target_kind = parse_target_kind(payload, "target_kind")?;
    if let Some(expected) = force_target_kind {
        if target_kind != expected {
            return Err(ValidationError::new(
                "invalid_field",
                "target_kind mismatch for event alias",
            ));
        }
    }

    let target_object_id = require_string(payload, "target_object_id")?;
    validate_id(target_object_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let representation_kind = parse_representation_kind(payload)?;

    let payload_hash = require_string(payload, "payload_hash")?;
    validate_hex_64(payload_hash, "payload_hash")?;

    let author_identity_id = require_string(payload, "author_identity_id")?;
    validate_id(author_identity_id).map_err(|err| ValidationError::new("invalid_id", err))?;
    let speaker_identity_id = speaker_identity_id.expect("speaker checked above");
    if author_identity_id != speaker_identity_id.to_string() {
        return Err(ValidationError::new(
            "invalid_field",
            "author_identity_id must match event speaker_identity_id",
        ));
    }

    match representation_kind {
        RepresentationKind::Title => {
            for field in ["tier_length", "tier_complexity", "vocabulary_version_id"] {
                if payload.contains_key(field) {
                    return Err(ValidationError::new(
                        "invalid_field",
                        format!("{field} is forbidden for a title representation"),
                    ));
                }
            }
        }
        RepresentationKind::Description => {
            parse_description_tier_length(payload, "tier_length")?;
            match parse_tier_complexity(payload, "tier_complexity")? {
                TierComplexity::Canonical => {
                    let vocabulary_version_id = require_string(payload, "vocabulary_version_id")?;
                    validate_id(vocabulary_version_id)
                        .map_err(|err| ValidationError::new("invalid_id", err))?;
                }
                TierComplexity::Fundamental
                | TierComplexity::Standard
                | TierComplexity::Advanced => {
                    if payload.contains_key("vocabulary_version_id") {
                        return Err(ValidationError::new(
                            "invalid_field",
                            "vocabulary_version_id is only valid for canonical complexity",
                        ));
                    }
                }
            }
        }
    }

    let _language_locale = optional_string(payload, "language_locale")?;
    let _provenance = optional_string(payload, "provenance")?;

    validate_optional_payload_text(payload, "payload_text")?;
    validate_optional_payload_text(payload, "text")?;
    validate_optional_payload_text(payload, "payload")?;

    Ok(())
}

fn validate_challenge_create(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    let speaker_identity_id = speaker_identity_id
        .ok_or_else(|| ValidationError::new("missing_field", "speaker_identity_id required"))?;
    validate_payload_speaker_identity(payload, speaker_identity_id)?;

    let challenge_id = require_string(payload, "challenge_id")?;
    validate_id(challenge_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let domain = require_string(payload, "challenge_domain")?;
    if !matches!(
        domain,
        "truth_challenge"
            | "importance_challenge"
            | "action_challenge"
            | "representation_challenge"
    ) {
        return Err(ValidationError::new(
            "invalid_field",
            "unsupported challenge_domain",
        ));
    }

    let framing_ref = require_string(payload, "framing_representation_ref")?;
    validate_id(framing_ref).map_err(|err| ValidationError::new("invalid_id", err))?;

    if domain == "importance_challenge" {
        let context_key = require_string(payload, "context_key")?;
        let axis = require_string(payload, "axis")?;
        let timeframe = require_string(payload, "timeframe")?;
        let scope = require_string(payload, "scope")?;
        if context_key.trim().is_empty()
            || axis.trim().is_empty()
            || timeframe.trim().is_empty()
            || scope.trim().is_empty()
        {
            return Err(ValidationError::new(
                "invalid_field",
                "context_key, axis, timeframe, and scope must be non-empty",
            ));
        }
        let subject_idea_ids = payload
            .get("subject_idea_ids")
            .ok_or_else(|| ValidationError::new("missing_field", "subject_idea_ids required"))?;
        let values = subject_idea_ids.as_array().ok_or_else(|| {
            ValidationError::new("invalid_field", "subject_idea_ids must be array")
        })?;
        if values.len() != 2 {
            return Err(ValidationError::new(
                "invalid_field",
                "importance_challenge requires exactly 2 subject_idea_ids",
            ));
        }
        let first = values[0].as_str().ok_or_else(|| {
            ValidationError::new("invalid_field", "subject_idea_ids entries must be string")
        })?;
        let second = values[1].as_str().ok_or_else(|| {
            ValidationError::new("invalid_field", "subject_idea_ids entries must be string")
        })?;
        validate_id(first).map_err(|err| ValidationError::new("invalid_id", err))?;
        validate_id(second).map_err(|err| ValidationError::new("invalid_id", err))?;
        if first == second {
            return Err(ValidationError::new(
                "invalid_field",
                "subject_idea_ids must reference distinct ideas",
            ));
        }
        if let Some(reference_idea_id) = optional_string(payload, "reference_idea_id")? {
            validate_id(&reference_idea_id)
                .map_err(|err| ValidationError::new("invalid_id", err))?;
        }
    }

    if let Some(subject_idea_ids) = payload.get("subject_idea_ids") {
        validate_id_array(subject_idea_ids, "subject_idea_ids")?;
    }

    if let Some(subject_ordering_ids) = payload.get("subject_ordering_ids") {
        validate_id_array(subject_ordering_ids, "subject_ordering_ids")?;
    }

    Ok(())
}

fn validate_challenge_id_only(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    ensure_speaker_identity_present(speaker_identity_id)?;
    let challenge_id = require_string(payload, "challenge_id")?;
    validate_id(challenge_id).map_err(|err| ValidationError::new("invalid_id", err))?;
    Ok(())
}

fn validate_vote_cast(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    let speaker_identity_id = speaker_identity_id
        .ok_or_else(|| ValidationError::new("missing_field", "speaker_identity_id required"))?;
    validate_payload_speaker_identity(payload, speaker_identity_id)?;
    let challenge_id = require_string(payload, "challenge_id")?;
    validate_id(challenge_id).map_err(|err| ValidationError::new("invalid_id", err))?;
    let vote_session_id = require_string(payload, "vote_session_id")?;
    validate_id(vote_session_id).map_err(|err| ValidationError::new("invalid_id", err))?;
    let vote_choice = require_string(payload, "vote_choice")?;
    if !matches!(vote_choice, "left" | "right" | "abstain") {
        return Err(ValidationError::new(
            "invalid_field",
            "vote_choice must be left, right, or abstain",
        ));
    }
    Ok(())
}

fn validate_vote_session_open(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    let speaker_identity_id = speaker_identity_id
        .ok_or_else(|| ValidationError::new("missing_field", "speaker_identity_id required"))?;
    validate_payload_speaker_identity(payload, speaker_identity_id)?;

    let vote_session_id = require_string(payload, "vote_session_id")?;
    validate_id(vote_session_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let challenge_id = require_string(payload, "challenge_id")?;
    validate_id(challenge_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let session_index = payload
        .get("session_index")
        .ok_or_else(|| ValidationError::new("missing_field", "session_index required"))?;
    parse_non_negative_i64_value(session_index, "session_index")?;

    let selection_cycle_index = payload
        .get("selection_cycle_index")
        .ok_or_else(|| ValidationError::new("missing_field", "selection_cycle_index required"))?;
    parse_non_negative_i64_value(selection_cycle_index, "selection_cycle_index")?;

    let selection_boundary_event_id = require_string(payload, "selection_boundary_event_id")?;
    validate_id(selection_boundary_event_id)
        .map_err(|err| ValidationError::new("invalid_id", err))?;

    Ok(())
}

fn validate_challenge_cancel_like(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    validate_challenge_id_only(payload, speaker_identity_id)?;
    let reason_ref = require_string(payload, "reason_representation_ref")?;
    validate_id(reason_ref).map_err(|err| ValidationError::new("invalid_id", err))?;
    Ok(())
}

fn validate_challenge_finalize_verdict(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    ensure_speaker_identity_present(speaker_identity_id)?;

    let challenge_id = require_string(payload, "challenge_id")?;
    validate_id(challenge_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let verdict_id = require_string(payload, "verdict_id")?;
    validate_id(verdict_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    if let Some(winning_choice) = optional_string(payload, "winning_choice")? {
        if !matches!(winning_choice.as_str(), "left" | "right" | "no_change") {
            return Err(ValidationError::new(
                "invalid_field",
                "winning_choice must be left, right, or no_change",
            ));
        }
        let left_votes = payload
            .get("left_votes")
            .ok_or_else(|| ValidationError::new("missing_field", "left_votes required"))?;
        let right_votes = payload
            .get("right_votes")
            .ok_or_else(|| ValidationError::new("missing_field", "right_votes required"))?;
        let total_votes = payload
            .get("total_votes")
            .ok_or_else(|| ValidationError::new("missing_field", "total_votes required"))?;
        let left_votes = parse_non_negative_i64_value(left_votes, "left_votes")?;
        let right_votes = parse_non_negative_i64_value(right_votes, "right_votes")?;
        let total_votes = parse_non_negative_i64_value(total_votes, "total_votes")?;
        if left_votes + right_votes != total_votes {
            return Err(ValidationError::new(
                "invalid_field",
                "left_votes + right_votes must equal total_votes",
            ));
        }
        match (
            winning_choice.as_str(),
            optional_string(payload, "winning_target_idea_id")?,
        ) {
            ("left" | "right", Some(target_id)) => {
                validate_id(&target_id).map_err(|err| ValidationError::new("invalid_id", err))?;
            }
            ("no_change", None) => {}
            ("no_change", Some(_)) => {
                return Err(ValidationError::new(
                    "invalid_field",
                    "winning_target_idea_id must be absent for no_change verdict",
                ));
            }
            _ => {
                return Err(ValidationError::new(
                    "missing_field",
                    "winning_target_idea_id required for decisive verdict",
                ));
            }
        }
    }

    if let Some(update) = payload.get("representation_pointer_update") {
        validate_representation_pointer_update(update)?;
    }
    if let Some(updates) = payload.get("representation_pointer_updates") {
        validate_representation_pointer_updates(updates)?;
    }
    if let Some(updates) = payload.get("representation_selections") {
        validate_representation_pointer_updates(updates)?;
    }
    if payload.contains_key("representation_id")
        || payload.contains_key("target_kind")
        || payload.contains_key("target_object_id")
    {
        validate_representation_pointer_update(&Value::Object(payload.clone()))?;
    }

    Ok(())
}

fn validate_cycle_close(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    let speaker_identity_id = speaker_identity_id
        .ok_or_else(|| ValidationError::new("missing_field", "speaker_identity_id required"))?;
    if speaker_identity_id != system_boundary_emitter_id() {
        return Err(ValidationError::new(
            "invalid_field",
            "cycle_close must be authored by system_boundary_emitter",
        ));
    }

    if payload.contains_key("forced_seal")
        || payload.contains_key("closure_kind")
        || payload.contains_key("cycle_index")
    {
        return Err(ValidationError::new(
            "invalid_field",
            "legacy cycle_close fields are not valid in public canonical validation",
        ));
    }

    let cycle_index_closed = payload
        .get("cycle_index_closed")
        .ok_or_else(|| ValidationError::new("missing_field", "cycle_index_closed required"))?;
    let cycle_index_closed =
        parse_non_negative_i64_value(cycle_index_closed, "cycle_index_closed")?;
    let next_cycle_index = payload
        .get("next_cycle_index")
        .ok_or_else(|| ValidationError::new("missing_field", "next_cycle_index required"))?;
    let next_cycle_index = parse_non_negative_i64_value(next_cycle_index, "next_cycle_index")?;
    if next_cycle_index != cycle_index_closed + 1 {
        return Err(ValidationError::new(
            "invalid_field",
            "next_cycle_index must equal cycle_index_closed + 1",
        ));
    }

    let boundary_type = require_string(payload, "boundary_type")?;
    if !matches!(boundary_type, "deliberative" | "forced") {
        return Err(ValidationError::new(
            "invalid_field",
            "boundary_type must be deliberative or forced",
        ));
    }

    let trigger = require_string(payload, "trigger")?;
    let trigger_allowed = match boundary_type {
        "deliberative" => trigger == "dmin_plus_work_target",
        "forced" => matches!(trigger, "dmax_forced" | "dmax_structural_liveness_forced"),
        _ => false,
    };
    if !trigger_allowed {
        return Err(ValidationError::new(
            "invalid_field",
            "trigger must match boundary_type",
        ));
    }

    parse_non_negative_i64_value(
        payload
            .get("W_score")
            .ok_or_else(|| ValidationError::new("missing_field", "W_score required"))?,
        "W_score",
    )?;
    parse_non_negative_i64_value(
        payload
            .get("W_target")
            .ok_or_else(|| ValidationError::new("missing_field", "W_target required"))?,
        "W_target",
    )?;
    require_non_empty_string(payload, "dmin_target_key")?;
    require_non_empty_string(payload, "dmax_target_key")?;
    require_non_empty_string(payload, "tempo_profile_hash")?;
    require_non_empty_string(payload, "derived_state_commitment")?;

    let authorization_frontier_before =
        payload
            .get("authorization_frontier_before")
            .ok_or_else(|| {
                ValidationError::new("missing_field", "authorization_frontier_before required")
            })?;
    parse_i64_value(
        authorization_frontier_before,
        "authorization_frontier_before",
    )?;

    let closure_boundary_ref = payload
        .get("closure_boundary_ref")
        .ok_or_else(|| ValidationError::new("missing_field", "closure_boundary_ref required"))?;
    parse_closure_boundary_height(closure_boundary_ref)?;

    Ok(())
}

fn validate_legacy_cycle_close(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    let speaker_identity_id = speaker_identity_id
        .ok_or_else(|| ValidationError::new("missing_field", "speaker_identity_id required"))?;
    if speaker_identity_id != system_boundary_emitter_id() {
        return Err(ValidationError::new(
            "invalid_field",
            "cycle_close must be authored by system_boundary_emitter",
        ));
    }

    let cycle_index = payload
        .get("cycle_index")
        .ok_or_else(|| ValidationError::new("missing_field", "cycle_index required"))?;
    parse_non_negative_i64_value(cycle_index, "cycle_index")?;

    let closure_kind = require_string(payload, "closure_kind")?;
    if !matches!(closure_kind, "deliberative" | "forced") {
        return Err(ValidationError::new(
            "invalid_field",
            "closure_kind must be deliberative or forced",
        ));
    }

    let forced_seal = require_bool(payload, "forced_seal")?;
    if forced_seal != (closure_kind == "forced") {
        return Err(ValidationError::new(
            "invalid_field",
            "forced_seal must match closure_kind",
        ));
    }

    let closure_boundary_ref = payload
        .get("closure_boundary_ref")
        .ok_or_else(|| ValidationError::new("missing_field", "closure_boundary_ref required"))?;
    parse_closure_boundary_height(closure_boundary_ref)?;

    Ok(())
}

fn validate_snapshot_commit(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    let speaker_identity_id = speaker_identity_id
        .ok_or_else(|| ValidationError::new("missing_field", "speaker_identity_id required"))?;
    if speaker_identity_id != system_boundary_emitter_id() {
        return Err(ValidationError::new(
            "invalid_field",
            "snapshot_commit must be authored by system_boundary_emitter",
        ));
    }

    let block_height = payload
        .get("block_height")
        .ok_or_else(|| ValidationError::new("missing_field", "block_height required"))?;
    let block_height = parse_non_negative_i64_value(block_height, "block_height")?;
    let interval = snapshot_interval_blocks();
    if block_height % interval != 0 {
        return Err(ValidationError::new(
            "invalid_field",
            format!(
                "block_height must align with snapshot interval {}",
                interval
            ),
        ));
    }

    let snapshot_hash = require_string(payload, "snapshot_hash")?;
    validate_hex_64(snapshot_hash, "snapshot_hash")?;
    let state_root_hash = require_string(payload, "state_root_hash")?;
    validate_hex_64(state_root_hash, "state_root_hash")?;
    let title_sentence_payload_root = require_string(payload, "title_sentence_payload_root")?;
    validate_hex_64(title_sentence_payload_root, "title_sentence_payload_root")?;
    let shared_map_commitment = require_string(payload, "shared_map_commitment")?;
    validate_hex_64(shared_map_commitment, "shared_map_commitment")?;
    let active_rulebook_set_hash = require_string(payload, "active_rulebook_set_hash")?;
    validate_hex_64(active_rulebook_set_hash, "active_rulebook_set_hash")?;

    let last_event_id = require_string(payload, "last_event_id")?;
    validate_id(last_event_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let event_count = payload
        .get("event_count")
        .ok_or_else(|| ValidationError::new("missing_field", "event_count required"))?;
    parse_non_negative_i64_value(event_count, "event_count")?;

    if let Some(value) = payload.get("snapshot_id") {
        let snapshot_id = value
            .as_str()
            .ok_or_else(|| ValidationError::new("invalid_field", "snapshot_id must be string"))?;
        validate_hex_64(snapshot_id, "snapshot_id")?;
        if snapshot_id != snapshot_hash {
            return Err(ValidationError::new(
                "invalid_field",
                "snapshot_id must equal snapshot_hash when provided",
            ));
        }
    }

    Ok(())
}

fn validate_blocked_submission(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    let speaker_identity_id = speaker_identity_id
        .ok_or_else(|| ValidationError::new("missing_field", "speaker_identity_id required"))?;

    let submission_hash = require_string(payload, "submission_hash")?;
    validate_hex_64(submission_hash, "submission_hash")?;

    let blocked_reason_code = require_string(payload, "blocked_reason_code")?;
    if blocked_reason_code.trim().is_empty() {
        return Err(ValidationError::new(
            "invalid_field",
            "blocked_reason_code must be non-empty",
        ));
    }

    let blocked_by_identity = require_string(payload, "blocked_by_identity")?;
    validate_id(blocked_by_identity).map_err(|err| ValidationError::new("invalid_id", err))?;
    if blocked_by_identity != speaker_identity_id.to_string() {
        return Err(ValidationError::new(
            "invalid_field",
            "blocked_by_identity must match speaker_identity_id",
        ));
    }

    require_non_empty_string(payload, "safe_summary_ref")?;
    require_non_empty_string(payload, "classifier_profile_ref")?;
    require_non_empty_string(payload, "rulebook_ref")?;

    if let Some(reference_event_id) = optional_string(payload, "reference_event_id")? {
        validate_id(&reference_event_id).map_err(|err| ValidationError::new("invalid_id", err))?;
    }
    if let Some(wrongful_block_challenge_ref) =
        optional_string(payload, "wrongful_block_challenge_ref")?
    {
        validate_id(&wrongful_block_challenge_ref)
            .map_err(|err| ValidationError::new("invalid_id", err))?;
    }

    Ok(())
}

fn validate_canonical_writer_grant(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    ensure_speaker_identity_present(speaker_identity_id)?;
    let identity_id = require_string(payload, "identity_id")?;
    validate_id(identity_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    let level_value = payload
        .get("canonical_writer_level")
        .ok_or_else(|| ValidationError::new("missing_field", "canonical_writer_level required"))?;
    let level = parse_non_negative_i64_value(level_value, "canonical_writer_level")?;
    if level < 1 {
        return Err(ValidationError::new(
            "invalid_field",
            "canonical_writer_level must be >= 1",
        ));
    }

    require_bool(payload, "email_verified")?;
    Ok(())
}

fn validate_canonical_writer_revoke(
    payload: &serde_json::Map<String, Value>,
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    ensure_speaker_identity_present(speaker_identity_id)?;
    let identity_id = require_string(payload, "identity_id")?;
    validate_id(identity_id).map_err(|err| ValidationError::new("invalid_id", err))?;
    Ok(())
}

fn ensure_speaker_identity_present(
    speaker_identity_id: Option<uuid::Uuid>,
) -> Result<(), ValidationError> {
    if speaker_identity_id.is_none() {
        return Err(ValidationError::new(
            "missing_field",
            "speaker_identity_id required",
        ));
    }
    Ok(())
}

fn validate_optional_payload_text(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<(), ValidationError> {
    match payload.get(field) {
        None | Some(Value::Null) => Ok(()),
        Some(value) => {
            if value.as_str().is_some() {
                Ok(())
            } else {
                Err(ValidationError::new(
                    "invalid_field",
                    format!("{field} must be string"),
                ))
            }
        }
    }
}

fn parse_target_kind(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<TargetKind, ValidationError> {
    let value = payload
        .get(field)
        .ok_or_else(|| ValidationError::new("missing_field", format!("{field} required")))?;
    parse_target_kind_value(value, field)
}

fn parse_target_kind_value(value: &Value, field: &str) -> Result<TargetKind, ValidationError> {
    match value {
        Value::String(v) => match v.as_str() {
            "idea" => Ok(TargetKind::Idea),
            "ordering" => Ok(TargetKind::Ordering),
            _ => Err(ValidationError::new(
                "invalid_field",
                format!("{field} has unsupported value"),
            )),
        },
        Value::Number(v) => match v.as_u64() {
            Some(0) => Ok(TargetKind::Idea),
            Some(1) => Ok(TargetKind::Ordering),
            _ => Err(ValidationError::new(
                "invalid_field",
                format!("{field} has unsupported value"),
            )),
        },
        _ => Err(ValidationError::new(
            "invalid_field",
            format!("{field} must be string or integer"),
        )),
    }
}

fn parse_ordering_profile(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<OrderingProfile, ValidationError> {
    let value = payload
        .get(field)
        .ok_or_else(|| ValidationError::new("missing_field", format!("{field} required")))?;
    match value {
        Value::String(v) if v == "vine" => Ok(OrderingProfile::Vine),
        Value::String(v) if v == "evidence_rail" => Ok(OrderingProfile::EvidenceRail),
        Value::String(v) if v == "action_rail" => Ok(OrderingProfile::ActionRail),
        _ => Err(ValidationError::new(
            "invalid_field",
            format!("{field} must be vine, evidence_rail, or action_rail"),
        )),
    }
}

fn validate_profile_vine_type(
    payload: &serde_json::Map<String, Value>,
    ordering_profile: OrderingProfile,
    vine_type_required: bool,
) -> Result<(), ValidationError> {
    match ordering_profile {
        OrderingProfile::Vine => parse_vine_type(payload, "vine_type", vine_type_required),
        OrderingProfile::EvidenceRail | OrderingProfile::ActionRail => {
            if payload
                .get("vine_type")
                .is_some_and(|value| !value.is_null())
            {
                return Err(ValidationError::new(
                    "invalid_field",
                    "vine_type is only valid for the vine ordering_profile",
                ));
            }
            Ok(())
        }
    }
}

fn parse_vine_type(
    payload: &serde_json::Map<String, Value>,
    field: &str,
    required: bool,
) -> Result<(), ValidationError> {
    let value = payload.get(field);
    if !required && value.is_none() {
        return Ok(());
    }
    let value =
        value.ok_or_else(|| ValidationError::new("missing_field", format!("{field} required")))?;
    match value {
        Value::Null if !required => Ok(()),
        Value::String(v) if v == "pathway_vine" || v == "narrative_vine" => Ok(()),
        _ => Err(ValidationError::new(
            "invalid_field",
            format!("{field} has unsupported value"),
        )),
    }
}

fn parse_representation_kind(
    payload: &serde_json::Map<String, Value>,
) -> Result<RepresentationKind, ValidationError> {
    match payload.get("representation_kind") {
        None => Err(ValidationError::new(
            "missing_field",
            "representation_kind required",
        )),
        Some(Value::String(value)) if value == "title" => Ok(RepresentationKind::Title),
        Some(Value::String(value)) if value == "description" => Ok(RepresentationKind::Description),
        Some(_) => Err(ValidationError::new(
            "invalid_field",
            "representation_kind must be title or description",
        )),
    }
}

fn parse_description_tier_length(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<(), ValidationError> {
    match payload.get(field) {
        Some(Value::String(value))
            if matches!(value.as_str(), "sentence" | "paragraph" | "full") =>
        {
            Ok(())
        }
        Some(_) => Err(ValidationError::new(
            "invalid_field",
            format!("{field} must be sentence, paragraph, or full"),
        )),
        None => Err(ValidationError::new(
            "missing_field",
            format!("{field} required"),
        )),
    }
}

fn parse_tier_complexity(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<TierComplexity, ValidationError> {
    let value = payload
        .get(field)
        .ok_or_else(|| ValidationError::new("missing_field", format!("{field} required")))?;
    match value {
        Value::String(v) if v == "fundamental" => Ok(TierComplexity::Fundamental),
        Value::String(v) if v == "standard" => Ok(TierComplexity::Standard),
        Value::String(v) if v == "advanced" => Ok(TierComplexity::Advanced),
        Value::String(v) if v == "canonical" => Ok(TierComplexity::Canonical),
        _ => Err(ValidationError::new(
            "invalid_field",
            format!("{field} has unsupported value"),
        )),
    }
}

fn validate_ordering_profile_bindings(
    payload: &serde_json::Map<String, Value>,
    ordering_profile: OrderingProfile,
    item_count: usize,
) -> Result<(), ValidationError> {
    match ordering_profile {
        OrderingProfile::Vine => {
            if payload.contains_key("subject_idea_id") || payload.contains_key("item_roles") {
                return Err(ValidationError::new(
                    "invalid_field",
                    "Vines must not carry subject_idea_id or item_roles",
                ));
            }
            Ok(())
        }
        OrderingProfile::EvidenceRail | OrderingProfile::ActionRail => {
            if item_count == 0 {
                return Err(ValidationError::new(
                    "invalid_field",
                    "standardized Orderings require at least one item",
                ));
            }

            let subject_idea_id = require_string(payload, "subject_idea_id")?;
            validate_id(subject_idea_id).map_err(|err| ValidationError::new("invalid_id", err))?;

            let item_idea_ids = payload["item_idea_ids"]
                .as_array()
                .expect("item_idea_ids checked above");
            let mut unique_ids = std::collections::HashSet::with_capacity(item_count);
            for idea_id in item_idea_ids {
                let idea_id = idea_id
                    .as_str()
                    .expect("item_idea_ids entries checked above");
                if !unique_ids.insert(idea_id) {
                    return Err(ValidationError::new(
                        "invalid_field",
                        "standardized Orderings must not contain duplicate item IDs",
                    ));
                }
            }

            let roles = payload
                .get("item_roles")
                .ok_or_else(|| ValidationError::new("missing_field", "item_roles required"))?
                .as_array()
                .ok_or_else(|| ValidationError::new("invalid_field", "item_roles must be array"))?;
            if roles.len() != item_count {
                return Err(ValidationError::new(
                    "invalid_field",
                    "item_roles must align one-for-one with item_idea_ids",
                ));
            }

            let role_strings = roles
                .iter()
                .map(|role| {
                    role.as_str().ok_or_else(|| {
                        ValidationError::new("invalid_field", "item_roles entries must be strings")
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            match ordering_profile {
                OrderingProfile::EvidenceRail => {
                    if role_strings
                        .iter()
                        .any(|role| !matches!(*role, "potential_evidence" | "actual_evidence"))
                    {
                        return Err(ValidationError::new(
                            "invalid_field",
                            "Evidence Rail roles must be potential_evidence or actual_evidence",
                        ));
                    }
                }
                OrderingProfile::ActionRail => {
                    let first = role_strings[0];
                    if !matches!(first, "potential_action" | "proposed_action")
                        || role_strings.iter().any(|role| *role != first)
                    {
                        return Err(ValidationError::new(
                            "invalid_field",
                            "Action Rail roles must form one homogeneous potential or proposed lane",
                        ));
                    }
                }
                OrderingProfile::Vine => unreachable!(),
            }
            Ok(())
        }
    }
}

fn validate_item_idea_ids(
    payload: &serde_json::Map<String, Value>,
) -> Result<usize, ValidationError> {
    let value = payload
        .get("item_idea_ids")
        .ok_or_else(|| ValidationError::new("missing_field", "item_idea_ids required"))?;
    let ids = value
        .as_array()
        .ok_or_else(|| ValidationError::new("invalid_field", "item_idea_ids must be array"))?;
    for idea_id in ids {
        let idea_id = idea_id.as_str().ok_or_else(|| {
            ValidationError::new("invalid_field", "item_idea_ids entries must be string")
        })?;
        validate_id(idea_id).map_err(|err| ValidationError::new("invalid_id", err))?;
    }
    Ok(ids.len())
}

fn validate_step_meta(
    payload: &serde_json::Map<String, Value>,
    item_count: usize,
) -> Result<(), ValidationError> {
    let Some(value) = payload.get("step_meta") else {
        return Ok(());
    };
    let entries = value
        .as_array()
        .ok_or_else(|| ValidationError::new("invalid_field", "step_meta must be array"))?;
    let expected = item_count.saturating_sub(1);
    if entries.len() != expected {
        return Err(ValidationError::new(
            "invalid_field",
            format!("step_meta length must equal {}", expected),
        ));
    }
    for entry in entries {
        let obj = entry.as_object().ok_or_else(|| {
            ValidationError::new("invalid_field", "step_meta entries must be objects")
        })?;
        match obj.get("via_connection_id") {
            None | Some(Value::Null) => {}
            Some(value) => {
                let via_connection_id = value.as_str().ok_or_else(|| {
                    ValidationError::new("invalid_field", "via_connection_id must be string")
                })?;
                validate_id(via_connection_id)
                    .map_err(|err| ValidationError::new("invalid_id", err))?;
            }
        }
    }
    Ok(())
}

fn validate_initial_representation_refs(
    payload: &serde_json::Map<String, Value>,
) -> Result<(), ValidationError> {
    let Some(value) = payload.get("initial_representation_refs") else {
        return Ok(());
    };
    let refs = value.as_object().ok_or_else(|| {
        ValidationError::new(
            "invalid_field",
            "initial_representation_refs must be object",
        )
    })?;
    for field in ["title_representation_id", "sentence_representation_id"] {
        if let Some(value) = refs.get(field) {
            let representation_id = value.as_str().ok_or_else(|| {
                ValidationError::new("invalid_field", format!("{field} must be string"))
            })?;
            validate_id(representation_id)
                .map_err(|err| ValidationError::new("invalid_id", err))?;
        }
    }
    Ok(())
}

fn validate_representation_pointer_updates(value: &Value) -> Result<(), ValidationError> {
    let updates = value.as_array().ok_or_else(|| {
        ValidationError::new("invalid_field", "representation updates must be array")
    })?;
    for update in updates {
        validate_representation_pointer_update(update)?;
    }
    Ok(())
}

fn validate_representation_pointer_update(value: &Value) -> Result<(), ValidationError> {
    let object = value.as_object().ok_or_else(|| {
        ValidationError::new(
            "invalid_field",
            "representation pointer update must be object",
        )
    })?;

    let target_kind = object
        .get("target_kind")
        .or_else(|| object.get("object_kind"))
        .ok_or_else(|| ValidationError::new("missing_field", "target_kind required"))?;
    parse_target_kind_value(target_kind, "target_kind")?;

    let target_object_id = object
        .get("target_object_id")
        .or_else(|| object.get("object_id"))
        .ok_or_else(|| ValidationError::new("missing_field", "target_object_id required"))?;
    let target_object_id = target_object_id
        .as_str()
        .ok_or_else(|| ValidationError::new("invalid_field", "target_object_id must be string"))?;
    validate_id(target_object_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    match parse_representation_kind(object)? {
        RepresentationKind::Title => {
            for field in ["tier_length", "tier_complexity", "vocabulary_version_id"] {
                if object.contains_key(field) {
                    return Err(ValidationError::new(
                        "invalid_field",
                        format!("{field} is forbidden for a title representation pointer"),
                    ));
                }
            }
        }
        RepresentationKind::Description => {
            parse_description_tier_length(object, "tier_length")?;
            parse_tier_complexity(object, "tier_complexity")?;
            if object.contains_key("vocabulary_version_id") {
                return Err(ValidationError::new(
                    "invalid_field",
                    "representation pointer updates must not repeat vocabulary_version_id",
                ));
            }
        }
    }

    let representation_id = object
        .get("representation_id")
        .or_else(|| object.get("selected_representation_id"))
        .ok_or_else(|| ValidationError::new("missing_field", "representation_id required"))?;
    let representation_id = representation_id
        .as_str()
        .ok_or_else(|| ValidationError::new("invalid_field", "representation_id must be string"))?;
    validate_id(representation_id).map_err(|err| ValidationError::new("invalid_id", err))?;

    Ok(())
}

fn validate_id_array(value: &Value, field: &str) -> Result<(), ValidationError> {
    let values = value
        .as_array()
        .ok_or_else(|| ValidationError::new("invalid_field", format!("{field} must be array")))?;
    for value in values {
        let id = value.as_str().ok_or_else(|| {
            ValidationError::new("invalid_field", format!("{field} entries must be string"))
        })?;
        validate_id(id).map_err(|err| ValidationError::new("invalid_id", err))?;
    }
    Ok(())
}

fn require_string<'a>(
    payload: &'a serde_json::Map<String, Value>,
    field: &str,
) -> Result<&'a str, ValidationError> {
    payload
        .get(field)
        .and_then(|value| value.as_str())
        .ok_or_else(|| ValidationError::new("missing_field", format!("{field} required")))
}

fn require_non_empty_string<'a>(
    payload: &'a serde_json::Map<String, Value>,
    field: &str,
) -> Result<&'a str, ValidationError> {
    let value = require_string(payload, field)?;
    if value.trim().is_empty() {
        return Err(ValidationError::new(
            "invalid_field",
            format!("{field} must be non-empty"),
        ));
    }
    Ok(value)
}

fn require_bool(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<bool, ValidationError> {
    payload
        .get(field)
        .and_then(|value| value.as_bool())
        .ok_or_else(|| ValidationError::new("missing_field", format!("{field} required")))
}

fn parse_i64_value(value: &Value, field: &str) -> Result<i64, ValidationError> {
    match value {
        Value::Number(number) => number
            .as_i64()
            .ok_or_else(|| ValidationError::new("invalid_field", format!("{field} invalid"))),
        Value::String(string) => string
            .parse::<i64>()
            .map_err(|_| ValidationError::new("invalid_field", format!("{field} invalid"))),
        _ => Err(ValidationError::new(
            "invalid_field",
            format!("{field} invalid"),
        )),
    }
}

fn parse_non_negative_i64_value(value: &Value, field: &str) -> Result<i64, ValidationError> {
    let parsed = match value {
        Value::Number(number) => number
            .as_i64()
            .ok_or_else(|| ValidationError::new("invalid_field", format!("{field} invalid")))?,
        Value::String(string) => string
            .parse::<i64>()
            .map_err(|_| ValidationError::new("invalid_field", format!("{field} invalid")))?,
        _ => {
            return Err(ValidationError::new(
                "invalid_field",
                format!("{field} invalid"),
            ))
        }
    };
    if parsed < 0 {
        return Err(ValidationError::new(
            "invalid_field",
            format!("{field} must be non-negative"),
        ));
    }
    Ok(parsed)
}

fn parse_closure_boundary_height(value: &Value) -> Result<i64, ValidationError> {
    match value {
        Value::Object(object) => {
            let block_height = object.get("block_height").ok_or_else(|| {
                ValidationError::new(
                    "missing_field",
                    "closure_boundary_ref.block_height required",
                )
            })?;
            parse_non_negative_i64_value(block_height, "closure_boundary_ref.block_height")
        }
        Value::Number(_) | Value::String(_) => {
            parse_non_negative_i64_value(value, "closure_boundary_ref")
        }
        _ => Err(ValidationError::new(
            "invalid_field",
            "closure_boundary_ref must be object, string, or number",
        )),
    }
}

fn optional_string(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Option<String>, ValidationError> {
    match payload.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_str()
            .map(|value| Some(value.to_string()))
            .ok_or_else(|| {
                ValidationError::new("invalid_field", format!("{field} must be string"))
            }),
    }
}

fn validate_hex_64(value: &str, field: &str) -> Result<(), ValidationError> {
    if value.len() != 64 {
        return Err(ValidationError::new(
            "invalid_field",
            format!("{field} must be 64 hex chars"),
        ));
    }
    if !value
        .as_bytes()
        .iter()
        .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
    {
        return Err(ValidationError::new(
            "invalid_field",
            format!("{field} must be lowercase hex"),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use encoding::payload::{canonical_json_payload_bytes, canonical_json_payload_hash_hex};
    use serde_json::json;
    use std::collections::HashMap;
    use uuid::Uuid;

    fn v7(id: &str) -> Uuid {
        Uuid::parse_str(id).expect("uuid parse")
    }

    #[test]
    fn validates_idea_create() {
        let payload_hash = payload_hash_hex("title", "sentence", None, None).expect("hash");
        let speaker = v7("00000000-0000-7000-8000-00000000a001");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000101"),
            kind: "idea_create".to_string(),
            payload: json!({
                "idea_id": "00000000-0000-7000-8000-00000000b001",
                "idea_type": "truth_claim",
                "speaker_identity_id": speaker,
                "title": "title",
                "sentence": "sentence",
                "payload_hash": payload_hash
            }),
            speaker_identity_id: Some(speaker),
        };
        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn rejects_missing_sentence() {
        let payload_hash = payload_hash_hex("title", "sentence", None, None).expect("hash");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000101"),
            kind: "idea_create".to_string(),
            payload: json!({
                "idea_id": "00000000-0000-7000-8000-00000000b001",
                "idea_type": "truth_claim",
                "title": "title",
                "payload_hash": payload_hash
            }),
            speaker_identity_id: Some(v7("00000000-0000-7000-8000-00000000a001")),
        };
        let err = validate_event(&event).expect_err("should error");
        assert_eq!(err.code, "missing_field");
    }

    #[test]
    fn rejects_invalid_event_type() {
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000101"),
            kind: "unknown_event".to_string(),
            payload: json!({}),
            speaker_identity_id: None,
        };
        let err = validate_event(&event).expect_err("should error");
        assert_eq!(err.code, "unsupported_event_type");
    }

    #[test]
    fn rejects_stale_aliases_and_noncanonical_helper_names_publicly() {
        for kind in [
            "idea_created",
            "connection_created",
            "challenge_opened",
            "verdict_reached",
            "identity_created",
            "identity_verified",
            "completion_truth_claim",
            "snapshot_created",
            "vote_session_open",
            "canonical_writer_grant",
            "canonical_writer_revoke",
            "genesis",
            "noop",
            "censorship_alert",
        ] {
            let event = Event {
                id: v7("00000000-0000-7000-8000-00000000010a"),
                kind: kind.to_string(),
                payload: json!({}),
                speaker_identity_id: None,
            };
            let err = validate_event(&event).expect_err("public validation should reject");
            assert_eq!(err.code, "unsupported_event_type", "kind={kind}");
        }
    }

    #[test]
    fn allows_genesis_and_noop_only_in_stage0_internal_validation() {
        for kind in ["genesis", "noop"] {
            let event = Event {
                id: v7("00000000-0000-7000-8000-00000000010b"),
                kind: kind.to_string(),
                payload: json!({}),
                speaker_identity_id: None,
            };
            assert!(
                validate_stage0_internal_event(&event).is_ok(),
                "kind={kind}"
            );
        }
    }

    #[test]
    fn validates_cycle_close_from_system_boundary_emitter() {
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000201"),
            kind: "cycle_close".to_string(),
            payload: json!({
                "cycle_index_closed": 0,
                "next_cycle_index": 1,
                "boundary_type": "forced",
                "trigger": "dmax_forced",
                "W_score": 0,
                "W_target": 1,
                "dmin_target_key": "tempo_target(0, dmin)",
                "dmax_target_key": "tempo_target(0, dmax)",
                "tempo_profile_hash": "hash_tempo_profile_test",
                "authorization_frontier_before": -1,
                "derived_state_commitment": "hash_cycle_state",
                "closure_boundary_ref": {
                    "block_height": 1
                }
            }),
            speaker_identity_id: Some(system_boundary_emitter_id()),
        };
        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn validates_deliberative_cycle_close_payload() {
        let event = Event {
            id: v7("00000000-0000-7000-8000-00000000020f"),
            kind: "cycle_close".to_string(),
            payload: json!({
                "cycle_index_closed": 2,
                "next_cycle_index": 3,
                "boundary_type": "deliberative",
                "trigger": "dmin_plus_work_target",
                "W_score": 4,
                "W_target": 3,
                "dmin_target_key": "tempo_target(2, dmin)",
                "dmax_target_key": "tempo_target(2, dmax)",
                "tempo_profile_hash": "hash_tempo_profile_test",
                "authorization_frontier_before": 0,
                "derived_state_commitment": "hash_cycle_state",
                "closure_boundary_ref": {
                    "block_height": 7
                }
            }),
            speaker_identity_id: Some(system_boundary_emitter_id()),
        };
        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn rejects_cycle_close_from_human_identity() {
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000202"),
            kind: "cycle_close".to_string(),
            payload: json!({
                "cycle_index_closed": 0,
                "next_cycle_index": 1,
                "boundary_type": "forced",
                "trigger": "dmax_forced",
                "W_score": 0,
                "W_target": 1,
                "dmin_target_key": "tempo_target(0, dmin)",
                "dmax_target_key": "tempo_target(0, dmax)",
                "tempo_profile_hash": "hash_tempo_profile_test",
                "authorization_frontier_before": -1,
                "derived_state_commitment": "hash_cycle_state",
                "closure_boundary_ref": 1
            }),
            speaker_identity_id: Some(v7("00000000-0000-7000-8000-00000000a001")),
        };
        let err = validate_event(&event).expect_err("should error");
        assert_eq!(err.code, "invalid_field");
    }

    #[test]
    fn rejects_legacy_cycle_close_payload_in_public_validation() {
        let event = Event {
            id: v7("00000000-0000-7000-8000-00000000020e"),
            kind: "cycle_close".to_string(),
            payload: json!({
                "cycle_index": 0,
                "closure_kind": "forced",
                "forced_seal": true,
                "closure_boundary_ref": {
                    "block_height": 1
                }
            }),
            speaker_identity_id: Some(system_boundary_emitter_id()),
        };
        let err = validate_event(&event).expect_err("public validation should reject legacy field");
        assert_eq!(err.code, "invalid_field");
        assert!(validate_stage0_internal_event(&event).is_ok());
    }

    #[test]
    fn validates_snapshot_commit_from_system_boundary_emitter() {
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000206"),
            kind: "snapshot_commit".to_string(),
            payload: json!({
                "block_height": 100,
                "snapshot_id": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "snapshot_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "state_root_hash": "1123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "title_sentence_payload_root": "2123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "shared_map_commitment": "3123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "last_event_id": "00000000-0000-7000-8000-000000000205",
                "event_count": 250,
                "active_rulebook_set_hash": "4123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            }),
            speaker_identity_id: Some(system_boundary_emitter_id()),
        };
        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn validates_identity_create() {
        let speaker = v7("00000000-0000-7000-8000-00000000a000");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000100"),
            kind: "identity_create".to_string(),
            payload: json!({
                "identity_id": speaker,
                "title": "New Identity",
                "initial_public_key_ref": "ed25519:test-key",
                "verification_reference": "self_attested",
                "speaker_identity_id": speaker
            }),
            speaker_identity_id: Some(speaker),
        };
        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn rejects_snapshot_commit_with_non_boundary_height() {
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000207"),
            kind: "snapshot_commit".to_string(),
            payload: json!({
                "block_height": 101,
                "snapshot_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "state_root_hash": "1123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "title_sentence_payload_root": "2123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "shared_map_commitment": "3123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "last_event_id": "00000000-0000-7000-8000-000000000205",
                "event_count": 250,
                "active_rulebook_set_hash": "4123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            }),
            speaker_identity_id: Some(system_boundary_emitter_id()),
        };
        let err = validate_event(&event).expect_err("should error");
        assert_eq!(err.code, "invalid_field");
    }

    #[test]
    fn rejects_system_boundary_emitter_non_boundary_event() {
        let payload_hash = payload_hash_hex("title", "sentence", None, None).expect("hash");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000203"),
            kind: "idea_create".to_string(),
            payload: json!({
                "idea_id": "00000000-0000-7000-8000-00000000b001",
                "idea_type": "truth_claim",
                "title": "title",
                "sentence": "sentence",
                "payload_hash": payload_hash
            }),
            speaker_identity_id: Some(system_boundary_emitter_id()),
        };
        let err = validate_event(&event).expect_err("should error");
        assert_eq!(err.code, "forbidden_author");
    }

    #[test]
    fn validates_importance_challenge_create_with_required_fields() {
        let speaker = v7("00000000-0000-7000-8000-00000000a002");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000204"),
            kind: "challenge_create".to_string(),
            payload: json!({
                "challenge_id": "00000000-0000-7000-8000-00000000c101",
                "challenge_domain": "importance_challenge",
                "framing_representation_ref": "00000000-0000-7000-8000-00000000d101",
                "speaker_identity_id": speaker,
                "context_key": "universal:default",
                "axis": "important_to_humanity",
                "timeframe": "medium_term",
                "scope": "universal",
                "subject_idea_ids": [
                    "00000000-0000-7000-8000-00000000b101",
                    "00000000-0000-7000-8000-00000000b102"
                ]
            }),
            speaker_identity_id: Some(speaker),
        };
        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn rejects_importance_challenge_create_without_context_key() {
        let speaker = v7("00000000-0000-7000-8000-00000000a003");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000205"),
            kind: "challenge_create".to_string(),
            payload: json!({
                "challenge_id": "00000000-0000-7000-8000-00000000c102",
                "challenge_domain": "importance_challenge",
                "framing_representation_ref": "00000000-0000-7000-8000-00000000d102",
                "speaker_identity_id": speaker,
                "axis": "important_to_humanity",
                "timeframe": "medium_term",
                "scope": "universal",
                "subject_idea_ids": [
                    "00000000-0000-7000-8000-00000000b103",
                    "00000000-0000-7000-8000-00000000b104"
                ]
            }),
            speaker_identity_id: Some(speaker),
        };
        let err = validate_event(&event).expect_err("should error");
        assert_eq!(err.code, "missing_field");
    }

    #[test]
    fn gates_vote_session_open_to_stage0_internal_validation() {
        let speaker = v7("00000000-0000-7000-8000-00000000a010");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000210"),
            kind: "vote_session_open".to_string(),
            payload: json!({
                "vote_session_id": "00000000-0000-7000-8000-00000000e201",
                "challenge_id": "00000000-0000-7000-8000-00000000c201",
                "session_index": 0,
                "selection_cycle_index": 1,
                "selection_boundary_event_id": "00000000-0000-7000-8000-000000000211",
                "speaker_identity_id": speaker
            }),
            speaker_identity_id: Some(speaker),
        };
        let err = validate_event(&event).expect_err("public validation should reject");
        assert_eq!(err.code, "unsupported_event_type");
        assert!(validate_stage0_internal_event(&event).is_ok());
    }

    #[test]
    fn rejects_vote_cast_without_vote_session_id() {
        let speaker = v7("00000000-0000-7000-8000-00000000a011");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000212"),
            kind: "vote_cast".to_string(),
            payload: json!({
                "challenge_id": "00000000-0000-7000-8000-00000000c202",
                "vote_choice": "left",
                "speaker_identity_id": speaker
            }),
            speaker_identity_id: Some(speaker),
        };
        let err = validate_event(&event).expect_err("should error");
        assert_eq!(err.code, "missing_field");
    }

    #[test]
    fn validates_blocked_submission() {
        let speaker = v7("00000000-0000-7000-8000-00000000a012");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000213"),
            kind: "blocked_submission".to_string(),
            payload: json!({
                "submission_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "blocked_reason_code": "unsafe_payload",
                "blocked_by_identity": speaker,
                "safe_summary_ref": "00000000-0000-7000-8000-00000000d213",
                "classifier_profile_ref": "safety-profile:test",
                "rulebook_ref": "safety-rulebook:test",
                "reference_event_id": "00000000-0000-7000-8000-000000000101"
            }),
            speaker_identity_id: Some(speaker),
        };
        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn rejects_blocked_submission_with_mismatched_blocked_by_identity() {
        let speaker = v7("00000000-0000-7000-8000-00000000a013");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000214"),
            kind: "blocked_submission".to_string(),
            payload: json!({
                "submission_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "blocked_reason_code": "unsafe_payload",
                "blocked_by_identity": "00000000-0000-7000-8000-00000000b013",
                "safe_summary_ref": "00000000-0000-7000-8000-00000000d214",
                "classifier_profile_ref": "safety-profile:test",
                "rulebook_ref": "safety-rulebook:test"
            }),
            speaker_identity_id: Some(speaker),
        };
        let err = validate_event(&event).expect_err("should error");
        assert_eq!(err.code, "invalid_field");
    }

    #[test]
    fn rejects_blocked_submission_without_safe_metadata_refs() {
        let speaker = v7("00000000-0000-7000-8000-00000000a014");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000215"),
            kind: "blocked_submission".to_string(),
            payload: json!({
                "submission_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "blocked_reason_code": "unsafe_payload",
                "blocked_by_identity": speaker
            }),
            speaker_identity_id: Some(speaker),
        };
        let err = validate_event(&event).expect_err("should error");
        assert_eq!(err.code, "missing_field");
    }

    #[test]
    fn gates_canonical_writer_grant_and_revoke_to_stage0_internal_validation() {
        let speaker = v7("00000000-0000-7000-8000-00000000a050");
        let grant = Event {
            id: v7("00000000-0000-7000-8000-000000000250"),
            kind: "canonical_writer_grant".to_string(),
            payload: json!({
                "identity_id": "00000000-0000-7000-8000-00000000b250",
                "canonical_writer_level": 1,
                "email_verified": true
            }),
            speaker_identity_id: Some(speaker),
        };
        let err = validate_event(&grant).expect_err("public validation should reject grant");
        assert_eq!(err.code, "unsupported_event_type");
        assert!(validate_stage0_internal_event(&grant).is_ok());

        let revoke = Event {
            id: v7("00000000-0000-7000-8000-000000000251"),
            kind: "canonical_writer_revoke".to_string(),
            payload: json!({
                "identity_id": "00000000-0000-7000-8000-00000000b250"
            }),
            speaker_identity_id: Some(speaker),
        };
        let err = validate_event(&revoke).expect_err("public validation should reject revoke");
        assert_eq!(err.code, "unsupported_event_type");
        assert!(validate_stage0_internal_event(&revoke).is_ok());
    }

    #[test]
    fn rejects_canonical_writer_grant_with_non_positive_level() {
        let speaker = v7("00000000-0000-7000-8000-00000000a051");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000252"),
            kind: "canonical_writer_grant".to_string(),
            payload: json!({
                "identity_id": "00000000-0000-7000-8000-00000000b251",
                "canonical_writer_level": 0,
                "email_verified": true
            }),
            speaker_identity_id: Some(speaker),
        };
        let err = validate_stage0_internal_event(&event).expect_err("should error");
        assert_eq!(err.code, "invalid_field");
    }

    #[test]
    fn rejects_secret_like_payload_before_kind_validation() {
        let payload_hash =
            payload_hash_hex("title", "password=do-not-store", None, None).expect("hash");
        let speaker = v7("00000000-0000-7000-8000-00000000a060");
        let event = Event {
            id: v7("00000000-0000-7000-8000-000000000260"),
            kind: "idea_create".to_string(),
            payload: json!({
                "idea_id": "00000000-0000-7000-8000-00000000b260",
                "idea_type": "truth_claim",
                "speaker_identity_id": speaker,
                "title": "title",
                "sentence": "password=do-not-store",
                "payload_hash": payload_hash
            }),
            speaker_identity_id: Some(speaker),
        };

        let err = validate_event(&event).expect_err("should reject secret-like payload");
        assert_eq!(err.code, "secret_detected");
    }

    #[test]
    fn representation_kind_separates_title_from_description_cells() {
        let speaker = v7("00000000-0000-7000-8000-00000000a070");
        let base = |id: &str, payload: Value| Event {
            id: v7(id),
            kind: "representation_create".to_string(),
            payload,
            speaker_identity_id: Some(speaker),
        };

        let title = base(
            "00000000-0000-7000-8000-000000000270",
            json!({
                "representation_id": "00000000-0000-7000-8000-00000000b270",
                "target_kind": "idea",
                "target_object_id": "00000000-0000-7000-8000-00000000c270",
                "representation_kind": "title",
                "payload_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "author_identity_id": speaker
            }),
        );
        assert!(validate_event(&title).is_ok());

        let title_with_tier = base(
            "00000000-0000-7000-8000-000000000271",
            json!({
                "representation_id": "00000000-0000-7000-8000-00000000b271",
                "target_kind": "idea",
                "target_object_id": "00000000-0000-7000-8000-00000000c270",
                "representation_kind": "title",
                "tier_length": "title",
                "tier_complexity": "standard",
                "payload_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "author_identity_id": speaker
            }),
        );
        assert_eq!(
            validate_event(&title_with_tier)
                .expect_err("title tier fields must be rejected")
                .code,
            "invalid_field"
        );

        let canonical = base(
            "00000000-0000-7000-8000-000000000272",
            json!({
                "representation_id": "00000000-0000-7000-8000-00000000b272",
                "target_kind": "idea",
                "target_object_id": "00000000-0000-7000-8000-00000000c270",
                "representation_kind": "description",
                "tier_length": "sentence",
                "tier_complexity": "canonical",
                "vocabulary_version_id": "00000000-0000-7000-8000-00000000d270",
                "payload_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "author_identity_id": speaker
            }),
        );
        assert!(validate_event(&canonical).is_ok());

        let canonical_without_vocabulary = base(
            "00000000-0000-7000-8000-000000000273",
            json!({
                "representation_id": "00000000-0000-7000-8000-00000000b273",
                "target_kind": "idea",
                "target_object_id": "00000000-0000-7000-8000-00000000c270",
                "representation_kind": "description",
                "tier_length": "sentence",
                "tier_complexity": "canonical",
                "payload_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "author_identity_id": speaker
            }),
        );
        assert_eq!(
            validate_event(&canonical_without_vocabulary)
                .expect_err("canonical vocabulary is required")
                .code,
            "missing_field"
        );

        let noncanonical_with_vocabulary = base(
            "00000000-0000-7000-8000-000000000274",
            json!({
                "representation_id": "00000000-0000-7000-8000-00000000b274",
                "target_kind": "idea",
                "target_object_id": "00000000-0000-7000-8000-00000000c270",
                "representation_kind": "description",
                "tier_length": "paragraph",
                "tier_complexity": "standard",
                "vocabulary_version_id": "00000000-0000-7000-8000-00000000d270",
                "payload_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "author_identity_id": speaker
            }),
        );
        assert_eq!(
            validate_event(&noncanonical_with_vocabulary)
                .expect_err("noncanonical vocabulary is forbidden")
                .code,
            "invalid_field"
        );
    }

    #[test]
    fn native_ordering_conformance_vectors_match_validation_and_hashing() {
        #[derive(Clone)]
        struct FixtureOrdering {
            profile: String,
            subject_idea_id: Option<Uuid>,
            item_roles: HashMap<Uuid, String>,
            action_lane: Option<String>,
        }

        let fixture: Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../docs/conformance/native-ordering.vectors.json"
        )))
        .expect("native Ordering fixture must parse");
        let idea_types: HashMap<Uuid, String> = fixture["idea_types"]
            .as_object()
            .expect("idea_types object")
            .iter()
            .map(|(idea_id, idea_type)| {
                (
                    v7(idea_id),
                    idea_type.as_str().expect("idea type string").to_string(),
                )
            })
            .collect();

        for vector in fixture["vectors"]
            .as_array()
            .expect("vectors must be an array")
        {
            let mut orderings: HashMap<Uuid, FixtureOrdering> = HashMap::new();
            let mut actual_code: Option<String> = None;
            for fixture_event in vector["events"]
                .as_array()
                .expect("events must be an array")
            {
                let event = Event {
                    id: v7(fixture_event["id"].as_str().expect("event id")),
                    kind: fixture_event["kind"]
                        .as_str()
                        .expect("event kind")
                        .to_string(),
                    payload: fixture_event["payload"].clone(),
                    speaker_identity_id: Some(v7(fixture_event["speaker_identity_id"]
                        .as_str()
                        .expect("speaker_identity_id"))),
                };

                if let Err(error) = validate_event(&event) {
                    actual_code = Some(error.code.to_string());
                    break;
                }

                if matches!(event.kind.as_str(), "ordering_create" | "ordering_fork") {
                    let payload = event.payload.as_object().expect("Ordering payload object");
                    let ordering_id =
                        v7(payload["ordering_id"].as_str().expect("ordering_id string"));
                    let profile = match &payload["ordering_profile"] {
                        Value::String(value) => value.clone(),
                        _ => panic!("validator accepted invalid ordering_profile"),
                    };
                    let subject_idea_id = payload
                        .get("subject_idea_id")
                        .and_then(Value::as_str)
                        .map(v7);
                    if let Some(subject_idea_id) = subject_idea_id {
                        let expected_type = match profile.as_str() {
                            "evidence_rail" => Some("truth_claim"),
                            "action_rail" => Some("actionable_idea"),
                            _ => None,
                        };
                        if expected_type.is_some_and(|expected| {
                            idea_types.get(&subject_idea_id).map(String::as_str) != Some(expected)
                        }) {
                            actual_code = Some("subject_type_mismatch".to_string());
                            break;
                        }
                    }
                    let item_ids = payload["item_idea_ids"]
                        .as_array()
                        .expect("item_idea_ids")
                        .iter()
                        .map(|value| v7(value.as_str().expect("item idea ID")))
                        .collect::<Vec<_>>();
                    let roles = payload
                        .get("item_roles")
                        .and_then(Value::as_array)
                        .map(|values| {
                            values
                                .iter()
                                .map(|value| value.as_str().expect("item role").to_string())
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    let item_roles = item_ids
                        .iter()
                        .copied()
                        .zip(roles.iter().cloned())
                        .collect::<HashMap<_, _>>();
                    let action_lane = if profile == "action_rail" {
                        roles.first().cloned()
                    } else {
                        None
                    };
                    if event.kind == "ordering_fork" {
                        let base_ordering_id = v7(payload["base_ordering_id"]
                            .as_str()
                            .expect("base_ordering_id string"));
                        match orderings.get(&base_ordering_id) {
                            None => {
                                actual_code = Some("base_ordering_not_found".to_string());
                                break;
                            }
                            Some(base) if base.profile != profile => {
                                actual_code = Some("ordering_profile_mismatch".to_string());
                                break;
                            }
                            Some(base) => {
                                if base.subject_idea_id != subject_idea_id {
                                    actual_code = Some("ordering_subject_mismatch".to_string());
                                    break;
                                }
                                if base.item_roles.iter().any(|(idea_id, base_role)| {
                                    item_roles
                                        .get(idea_id)
                                        .is_some_and(|fork_role| fork_role != base_role)
                                }) {
                                    actual_code = Some("ordering_item_role_mismatch".to_string());
                                    break;
                                }
                                if profile == "action_rail" && base.action_lane != action_lane {
                                    actual_code = Some("action_lane_mismatch".to_string());
                                    break;
                                }
                            }
                        }
                    }
                    orderings.insert(
                        ordering_id,
                        FixtureOrdering {
                            profile,
                            subject_idea_id,
                            item_roles,
                            action_lane,
                        },
                    );
                }
            }

            let expected_accept = vector["expected"]["accept"]
                .as_bool()
                .expect("expected.accept");
            let expected_code = vector["expected"]["code"].as_str().map(str::to_string);
            assert_eq!(
                actual_code.is_none(),
                expected_accept,
                "{} acceptance mismatch",
                vector["id"].as_str().unwrap_or("unknown")
            );
            assert_eq!(
                actual_code,
                expected_code,
                "{} error-code mismatch",
                vector["id"].as_str().unwrap_or("unknown")
            );
        }

        for vector in fixture["hash_vectors"]
            .as_array()
            .expect("hash_vectors must be an array")
        {
            let payload = &vector["payload"];
            let canonical_bytes =
                canonical_json_payload_bytes(payload).expect("canonical JSON bytes");
            assert_eq!(
                String::from_utf8(canonical_bytes).expect("canonical JSON is UTF-8"),
                vector["canonical_json_utf8"]
                    .as_str()
                    .expect("canonical_json_utf8")
            );
            let actual_hash =
                canonical_json_payload_hash_hex(payload).expect("canonical JSON hash");
            assert_eq!(
                actual_hash,
                vector["blake3"].as_str().expect("blake3"),
                "{} BLAKE3 mismatch",
                vector["id"].as_str().unwrap_or("unknown")
            );
        }
    }

    #[test]
    fn seed_conformance_binding_vectors_match_validation_and_hashing() {
        fn position(value: &Value) -> (i64, i64) {
            (
                value["block_height"].as_i64().expect("block_height"),
                value["event_index"].as_i64().expect("event_index"),
            )
        }

        let fixture: Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../docs/conformance/seed-conformance-bindings.vectors.json"
        )))
        .expect("Seed conformance-binding fixture must parse");
        let identity_positions: HashMap<Uuid, (i64, i64)> = fixture["identity_positions"]
            .as_object()
            .expect("identity_positions object")
            .iter()
            .map(|(identity_id, value)| (v7(identity_id), position(value)))
            .collect();
        let idea_positions: HashMap<Uuid, (i64, i64)> = fixture["idea_positions"]
            .as_object()
            .expect("idea_positions object")
            .iter()
            .map(|(idea_id, value)| (v7(idea_id), position(value)))
            .collect();

        for vector in fixture["vectors"]
            .as_array()
            .expect("vectors must be an array")
        {
            let mut actual_code: Option<String> = None;
            for fixture_event in vector["events"].as_array().expect("events array") {
                let event = Event {
                    id: v7(fixture_event["id"].as_str().expect("event id")),
                    kind: fixture_event["kind"]
                        .as_str()
                        .expect("event kind")
                        .to_string(),
                    payload: fixture_event["payload"].clone(),
                    speaker_identity_id: Some(v7(fixture_event["speaker_identity_id"]
                        .as_str()
                        .expect("speaker identity ID"))),
                };
                if let Err(error) = validate_event(&event) {
                    actual_code = Some(error.code.to_string());
                    break;
                }

                let event_position = (
                    fixture_event["block_height"]
                        .as_i64()
                        .expect("block_height"),
                    fixture_event["event_index"].as_i64().expect("event_index"),
                );
                let payload = event.payload.as_object().expect("payload object");
                let author_identity_id =
                    v7(payload["author_identity_id"].as_str().expect("author ID"));
                match identity_positions.get(&author_identity_id) {
                    None => {
                        actual_code = Some("unknown_author".to_string());
                        break;
                    }
                    Some(author_position) if *author_position >= event_position => {
                        actual_code = Some("author_not_preexisting".to_string());
                        break;
                    }
                    Some(_) => {}
                }
                if let Some(vocabulary_id) =
                    payload.get("vocabulary_version_id").and_then(Value::as_str)
                {
                    match idea_positions.get(&v7(vocabulary_id)) {
                        None => {
                            actual_code = Some("unknown_vocabulary".to_string());
                            break;
                        }
                        Some(vocabulary_position) if *vocabulary_position >= event_position => {
                            actual_code = Some("vocabulary_not_preexisting".to_string());
                            break;
                        }
                        Some(_) => {}
                    }
                }
            }

            let expected_accept = vector["expected"]["accept"]
                .as_bool()
                .expect("expected.accept");
            let expected_code = vector["expected"]["code"].as_str().map(str::to_string);
            assert_eq!(
                actual_code.is_none(),
                expected_accept,
                "{} acceptance mismatch",
                vector["id"].as_str().unwrap_or("unknown")
            );
            assert_eq!(
                actual_code,
                expected_code,
                "{} error-code mismatch",
                vector["id"].as_str().unwrap_or("unknown")
            );
        }

        let mut hash_mismatches = Vec::new();
        for vector in fixture["hash_vectors"]
            .as_array()
            .expect("hash_vectors must be an array")
        {
            let payload = &vector["payload"];
            let canonical_bytes =
                canonical_json_payload_bytes(payload).expect("canonical JSON bytes");
            assert_eq!(
                String::from_utf8(canonical_bytes).expect("canonical JSON is UTF-8"),
                vector["canonical_json_utf8"]
                    .as_str()
                    .expect("canonical_json_utf8")
            );
            let actual_hash =
                canonical_json_payload_hash_hex(payload).expect("canonical JSON hash");
            let expected_hash = vector["blake3"].as_str().expect("blake3");
            if actual_hash != expected_hash {
                hash_mismatches.push(format!(
                    "{}={}",
                    vector["id"].as_str().unwrap_or("unknown"),
                    actual_hash
                ));
            }
        }
        assert!(
            hash_mismatches.is_empty(),
            "BLAKE3 mismatches: {}",
            hash_mismatches.join(", ")
        );
    }
}
