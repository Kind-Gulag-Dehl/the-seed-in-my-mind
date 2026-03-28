use super::parsing::parse_non_negative_i64_value;
use super::state::{CycleClosureKind, RailKind, WriterVerificationState};
use super::*;

pub(super) fn parse_cycle_closure_kind(
    payload: &serde_json::Map<String, Value>,
) -> Result<CycleClosureKind, ReplayError> {
    let closure_kind = payload
        .get("closure_kind")
        .and_then(Value::as_str)
        .ok_or_else(|| ReplayError::new("missing_field", "closure_kind required"))?;
    match closure_kind {
        "deliberative" => Ok(CycleClosureKind::Deliberative),
        "forced" => Ok(CycleClosureKind::Forced),
        _ => Err(ReplayError::new(
            "invalid_field",
            "closure_kind must be deliberative or forced",
        )),
    }
}

pub(super) fn parse_closure_boundary_height(value: &Value) -> Result<i64, ReplayError> {
    match value {
        Value::Object(object) => {
            let block_height = object.get("block_height").ok_or_else(|| {
                ReplayError::new(
                    "missing_field",
                    "closure_boundary_ref.block_height required",
                )
            })?;
            parse_non_negative_i64_value(block_height, "closure_boundary_ref.block_height")
        }
        Value::Number(_) | Value::String(_) => {
            parse_non_negative_i64_value(value, "closure_boundary_ref")
        }
        _ => Err(ReplayError::new(
            "invalid_field",
            "closure_boundary_ref must be object, string, or number",
        )),
    }
}

pub(super) fn round_div_i64(numerator: i64, denominator: i64) -> i64 {
    if denominator <= 0 {
        return numerator;
    }
    if numerator >= 0 {
        (numerator + denominator / 2) / denominator
    } else {
        (numerator - denominator / 2) / denominator
    }
}

pub(super) fn clamp_i64(value: i64, min: i64, max: i64) -> i64 {
    value.clamp(min, max)
}

pub(super) fn approximate_timestamp_from_event_id(
    event_id: Uuid,
) -> Result<DateTime<Utc>, ReplayError> {
    let timestamp = event_id.get_timestamp().ok_or_else(|| {
        ReplayError::new(
            "invalid_id",
            format!("event_id={} missing UUIDv7 timestamp", event_id),
        )
    })?;
    let (seconds, nanos) = timestamp.to_unix();
    let seconds_i64 = i64::try_from(seconds).map_err(|_| {
        ReplayError::new(
            "invalid_id",
            format!("event_id={} timestamp out of range", event_id),
        )
    })?;
    DateTime::<Utc>::from_timestamp(seconds_i64, nanos).ok_or_else(|| {
        ReplayError::new(
            "invalid_id",
            format!("event_id={} invalid timestamp components", event_id),
        )
    })
}

pub(super) fn rail_kind_to_string(kind: RailKind) -> &'static str {
    match kind {
        RailKind::Vine => "vine",
    }
}

pub(super) fn is_valid_idea_type(value: &str) -> bool {
    matches!(
        value,
        "truth_claim" | "conceptual_idea" | "actionable_idea" | "action" | "identity"
    )
}

pub(super) fn is_valid_connection_type(value: &str) -> bool {
    matches!(value, "same_as" | "relative_importance" | "membership")
}

pub(super) fn seed_bootstrap_verifier_identity_id() -> Uuid {
    Uuid::parse_str(SEED_BOOTSTRAP_VERIFIER_ID_STR)
        .expect("SEED_BOOTSTRAP_VERIFIER_ID_STR must be a valid UUID")
}

pub(super) fn is_identity_active_verifier(
    verifier_state_by_identity: &HashMap<Uuid, bool>,
    identity_id: Uuid,
) -> bool {
    verifier_state_by_identity
        .get(&identity_id)
        .copied()
        .unwrap_or(identity_id == seed_bootstrap_verifier_identity_id())
}

pub(super) fn is_writer_eligible(writer_state: Option<&WriterVerificationState>) -> bool {
    writer_state
        .map(|state| {
            state.email_verified && state.canonical_writer_level >= MIN_CANONICAL_WRITER_LEVEL
        })
        .unwrap_or(false)
}
