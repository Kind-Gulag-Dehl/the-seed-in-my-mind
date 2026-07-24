use super::state::*;
use super::*;

pub(super) fn payload_object(
    value: &Value,
) -> Result<&serde_json::Map<String, Value>, ReplayError> {
    value
        .as_object()
        .ok_or_else(|| ReplayError::new("invalid_payload", "payload must be object"))
}

pub(super) fn parse_ordering_profile(value: i16) -> Result<OrderingProfile, ReplayError> {
    match value {
        0 => Ok(OrderingProfile::Vine),
        1 => Ok(OrderingProfile::EvidenceRail),
        2 => Ok(OrderingProfile::ActionRail),
        _ => Err(ReplayError::new(
            "invalid_field",
            format!("invalid ordering_profile {}", value),
        )),
    }
}

pub(super) fn parse_ordering_profile_payload(
    payload: &serde_json::Map<String, Value>,
) -> Result<OrderingProfile, ReplayError> {
    let value = payload
        .get("ordering_profile")
        .ok_or_else(|| ReplayError::new("missing_field", "ordering_profile required"))?;
    match value {
        Value::String(value) if value == "vine" => Ok(OrderingProfile::Vine),
        Value::String(value) if value == "evidence_rail" => Ok(OrderingProfile::EvidenceRail),
        Value::String(value) if value == "action_rail" => Ok(OrderingProfile::ActionRail),
        _ => Err(ReplayError::new(
            "invalid_field",
            "invalid ordering_profile",
        )),
    }
}

pub(super) fn parse_vine_type(value: Option<i16>) -> Result<Option<String>, ReplayError> {
    match value {
        None => Ok(None),
        Some(0) => Ok(Some("pathway_vine".to_string())),
        Some(1) => Ok(Some("narrative_vine".to_string())),
        Some(other) => Err(ReplayError::new(
            "invalid_field",
            format!("invalid vine_type {}", other),
        )),
    }
}

pub(super) fn parse_vine_type_payload(
    payload: &serde_json::Map<String, Value>,
    required: bool,
) -> Result<Option<String>, ReplayError> {
    let Some(value) = payload.get("vine_type") else {
        if required {
            return Err(ReplayError::new("missing_field", "vine_type required"));
        }
        return Ok(None);
    };
    if value.is_null() {
        if required {
            return Err(ReplayError::new("missing_field", "vine_type required"));
        }
        return Ok(None);
    }
    match value {
        Value::String(value) if value == "pathway_vine" || value == "narrative_vine" => {
            Ok(Some(value.clone()))
        }
        _ => Err(ReplayError::new("invalid_field", "invalid vine_type")),
    }
}

pub(super) fn parse_target_kind(value: i16) -> Result<TargetKind, ReplayError> {
    match value {
        0 => Ok(TargetKind::Idea),
        1 => Ok(TargetKind::Ordering),
        _ => Err(ReplayError::new(
            "invalid_field",
            format!("invalid target_kind {}", value),
        )),
    }
}

pub(super) fn parse_tier_enum(value: i16) -> Result<TierEnum, ReplayError> {
    match value {
        0 => Ok(TierEnum::Title),
        1 => Ok(TierEnum::Sentence),
        2 => Ok(TierEnum::Paragraph),
        3 => Ok(TierEnum::Full),
        _ => Err(ReplayError::new(
            "invalid_field",
            format!("invalid tier_enum {}", value),
        )),
    }
}

pub(super) fn parse_initial_representation_refs(
    payload: &serde_json::Map<String, Value>,
) -> Result<PointerSlots, ReplayError> {
    let Some(value) = payload.get("initial_representation_refs") else {
        return Ok(PointerSlots::default());
    };
    let refs = value.as_object().ok_or_else(|| {
        ReplayError::new(
            "invalid_field",
            "initial_representation_refs must be an object",
        )
    })?;
    let title_representation_id = parse_optional_uuid(refs, "title_representation_id")?;
    let sentence_representation_id = parse_optional_uuid(refs, "sentence_representation_id")?;
    Ok(PointerSlots {
        title_representation_id,
        sentence_representation_id,
    })
}

pub(super) fn parse_optional_uuid(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Option<Uuid>, ReplayError> {
    match payload.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => {
            let value = value
                .as_str()
                .ok_or_else(|| ReplayError::new("invalid_field", format!("invalid {}", field)))?;
            let parsed = Uuid::parse_str(value)
                .map_err(|_| ReplayError::new("invalid_id", format!("invalid {}", field)))?;
            Ok(Some(parsed))
        }
    }
}

pub(super) fn parse_representation_pointer_updates(
    payload: &serde_json::Map<String, Value>,
) -> Result<Vec<RepresentationPointerUpdate>, ReplayError> {
    if let Some(value) = payload.get("representation_pointer_updates") {
        return parse_representation_pointer_update_array(value);
    }
    if let Some(value) = payload.get("representation_selections") {
        return parse_representation_pointer_update_array(value);
    }
    if let Some(value) = payload.get("representation_pointer_update") {
        return Ok(vec![parse_representation_pointer_update_value(value)?]);
    }
    if (payload.contains_key("representation_id")
        || payload.contains_key("selected_representation_id"))
        && (payload.contains_key("target_kind") || payload.contains_key("object_kind"))
        && (payload.contains_key("target_object_id") || payload.contains_key("object_id"))
    {
        return Ok(vec![parse_representation_pointer_update_object(payload)?]);
    }
    Ok(Vec::new())
}

pub(super) fn parse_representation_pointer_update_array(
    value: &Value,
) -> Result<Vec<RepresentationPointerUpdate>, ReplayError> {
    let updates = value.as_array().ok_or_else(|| {
        ReplayError::new("invalid_field", "invalid representation pointer updates")
    })?;
    updates
        .iter()
        .map(parse_representation_pointer_update_value)
        .collect()
}

pub(super) fn parse_representation_pointer_update_value(
    value: &Value,
) -> Result<RepresentationPointerUpdate, ReplayError> {
    let payload = value.as_object().ok_or_else(|| {
        ReplayError::new("invalid_field", "invalid representation pointer update")
    })?;
    parse_representation_pointer_update_object(payload)
}

pub(super) fn parse_representation_pointer_update_object(
    payload: &serde_json::Map<String, Value>,
) -> Result<RepresentationPointerUpdate, ReplayError> {
    let target_kind_value = payload
        .get("target_kind")
        .or_else(|| payload.get("object_kind"))
        .ok_or_else(|| ReplayError::new("missing_field", "target_kind required"))?;
    let target_kind = parse_target_kind_value(target_kind_value)?;

    let target_object_value = payload
        .get("target_object_id")
        .or_else(|| payload.get("object_id"))
        .ok_or_else(|| ReplayError::new("missing_field", "target_object_id required"))?;
    let target_object_id = parse_uuid_value(target_object_value, "target_object_id")?;

    let tier_value = payload
        .get("tier_length")
        .or_else(|| payload.get("tier_enum"))
        .ok_or_else(|| ReplayError::new("missing_field", "tier_length required"))?;
    let tier_enum = parse_tier_enum_value(tier_value)?;

    let representation_value = payload
        .get("representation_id")
        .or_else(|| payload.get("selected_representation_id"))
        .ok_or_else(|| ReplayError::new("missing_field", "representation_id required"))?;
    let representation_id = parse_uuid_value(representation_value, "representation_id")?;

    Ok(RepresentationPointerUpdate {
        target_kind,
        target_object_id,
        tier_enum,
        representation_id,
    })
}

pub(super) fn parse_target_kind_value(value: &Value) -> Result<TargetKind, ReplayError> {
    match value {
        Value::String(value) => match value.as_str() {
            "idea" => Ok(TargetKind::Idea),
            "ordering" => Ok(TargetKind::Ordering),
            _ => Err(ReplayError::new("invalid_field", "invalid target_kind")),
        },
        Value::Number(value) => match value.as_u64() {
            Some(0) => Ok(TargetKind::Idea),
            Some(1) => Ok(TargetKind::Ordering),
            _ => Err(ReplayError::new("invalid_field", "invalid target_kind")),
        },
        _ => Err(ReplayError::new("invalid_field", "invalid target_kind")),
    }
}

pub(super) fn parse_tier_enum_value(value: &Value) -> Result<TierEnum, ReplayError> {
    match value {
        Value::String(value) => match value.as_str() {
            "title" => Ok(TierEnum::Title),
            "sentence" => Ok(TierEnum::Sentence),
            "paragraph" => Ok(TierEnum::Paragraph),
            "full" => Ok(TierEnum::Full),
            _ => Err(ReplayError::new("invalid_field", "invalid tier_length")),
        },
        Value::Number(value) => match value.as_u64() {
            Some(0) => Ok(TierEnum::Title),
            Some(1) => Ok(TierEnum::Sentence),
            Some(2) => Ok(TierEnum::Paragraph),
            Some(3) => Ok(TierEnum::Full),
            _ => Err(ReplayError::new("invalid_field", "invalid tier_length")),
        },
        _ => Err(ReplayError::new("invalid_field", "invalid tier_length")),
    }
}

pub(super) fn parse_uuid_value(value: &Value, field: &str) -> Result<Uuid, ReplayError> {
    let value = value
        .as_str()
        .ok_or_else(|| ReplayError::new("invalid_field", format!("invalid {}", field)))?;
    Uuid::parse_str(value).map_err(|_| ReplayError::new("invalid_id", format!("invalid {}", field)))
}

pub(super) fn parse_uuid_payload(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Uuid, ReplayError> {
    let value = payload
        .get(field)
        .ok_or_else(|| ReplayError::new("missing_field", format!("{field} required")))?;
    parse_uuid_value(value, field)
}

pub(super) fn parse_required_string_payload<'a>(
    payload: &'a serde_json::Map<String, Value>,
    field: &str,
) -> Result<&'a str, ReplayError> {
    payload
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| ReplayError::new("missing_field", format!("{field} required")))
}

pub(super) fn parse_required_subject_idea_pair(
    payload: &serde_json::Map<String, Value>,
) -> Result<(Uuid, Uuid), ReplayError> {
    let values = payload
        .get("subject_idea_ids")
        .and_then(Value::as_array)
        .ok_or_else(|| ReplayError::new("missing_field", "subject_idea_ids required"))?;
    if values.len() != 2 {
        return Err(ReplayError::new(
            "invalid_field",
            "importance challenge requires exactly 2 subject_idea_ids",
        ));
    }
    let left = parse_uuid_value(&values[0], "subject_idea_ids[0]")?;
    let right = parse_uuid_value(&values[1], "subject_idea_ids[1]")?;
    if left == right {
        return Err(ReplayError::new(
            "invalid_field",
            "subject_idea_ids must reference distinct ideas",
        ));
    }
    Ok((left, right))
}

pub(super) fn parse_non_negative_i64_payload(
    payload: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<i64, ReplayError> {
    let value = payload
        .get(field)
        .ok_or_else(|| ReplayError::new("missing_field", format!("{field} required")))?;
    parse_non_negative_i64_value(value, field)
}

pub(super) fn parse_non_negative_i64_value(value: &Value, field: &str) -> Result<i64, ReplayError> {
    let parsed = match value {
        Value::Number(number) => number
            .as_i64()
            .ok_or_else(|| ReplayError::new("invalid_field", format!("{field} invalid")))?,
        Value::String(string) => string
            .parse::<i64>()
            .map_err(|_| ReplayError::new("invalid_field", format!("{field} invalid")))?,
        _ => {
            return Err(ReplayError::new(
                "invalid_field",
                format!("{field} invalid"),
            ))
        }
    };
    if parsed < 0 {
        return Err(ReplayError::new(
            "invalid_field",
            format!("{field} must be non-negative"),
        ));
    }
    Ok(parsed)
}
