#![cfg_attr(not(feature = "full"), allow(dead_code, unused_imports))]

use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::Response,
};
use storage::{ConnectionRow, PrivateVineItemInput};
use uuid::Uuid;

use crate::server::errors::json_error;
use crate::server::types::{
    IdeasTopOrder, PrivateVineItemPayload, RelativeImportanceDirection, VineTypeInput,
};

const INVALID_FIELD_LENGTH_CODE: &str = "invalid_field_length";

pub(crate) fn parse_uuid_v7(value: &str) -> Result<Uuid, Response> {
    let uuid = Uuid::parse_str(value)
        .map_err(|_| json_error(StatusCode::BAD_REQUEST, "invalid_id", "id must be uuidv7"))?;
    if uuid.get_version_num() != 7 {
        return Err(json_error(
            StatusCode::BAD_REQUEST,
            "invalid_id",
            "id must be uuidv7",
        ));
    }
    Ok(uuid)
}

pub(crate) fn parse_uuid_any(value: &str) -> Result<Uuid, Response> {
    let uuid = Uuid::parse_str(value)
        .map_err(|_| json_error(StatusCode::BAD_REQUEST, "invalid_id", "id must be uuid"))?;
    Ok(uuid)
}

pub(crate) fn parse_uuid_v7_field(value: &str, field: &str) -> Result<Uuid, Response> {
    let uuid = Uuid::parse_str(value).map_err(|_| {
        json_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            &format!("{field} must be uuidv7"),
        )
    })?;
    if uuid.get_version_num() != 7 {
        return Err(json_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            &format!("{field} must be uuidv7"),
        ));
    }
    Ok(uuid)
}

pub(crate) fn parse_vine_type_input(input: &VineTypeInput) -> Result<i16, Response> {
    match input {
        VineTypeInput::String(value) => match value.trim() {
            "pathway_vine" => Ok(0),
            "narrative_vine" => Ok(1),
            _ => Err(json_error(
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "vine_type must be pathway_vine or narrative_vine",
            )),
        },
        VineTypeInput::Number(value) => match *value {
            0 => Ok(0),
            1 => Ok(1),
            _ => Err(json_error(
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "vine_type must be 0 or 1",
            )),
        },
    }
}

pub(crate) fn vine_type_label(vine_type: Option<i16>) -> Option<String> {
    match vine_type {
        Some(0) => Some("pathway_vine".to_string()),
        Some(1) => Some("narrative_vine".to_string()),
        Some(other) => Some(format!("unknown_{other}")),
        None => None,
    }
}

pub(crate) fn private_vine_type_label(vine_type: i16) -> String {
    vine_type_label(Some(vine_type)).unwrap_or_else(|| "unknown".to_string())
}

pub(crate) fn rail_kind_label(rail_kind: i16) -> String {
    match rail_kind {
        0 => "vine".to_string(),
        other => format!("unknown_{other}"),
    }
}

pub(crate) fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

pub(crate) fn parse_private_vine_items(
    items: &[PrivateVineItemPayload],
) -> Result<Vec<PrivateVineItemInput>, Response> {
    let mut out = Vec::with_capacity(items.len());
    for (idx, item) in items.iter().enumerate() {
        let idea_id = parse_uuid_v7_field(item.idea_id.trim(), "items[].idea_id")?;
        let via_connection_id = match item.via_connection_id.as_deref() {
            Some(value) if !value.trim().is_empty() => Some(parse_uuid_v7_field(
                value.trim(),
                "items[].via_connection_id",
            )?),
            _ => None,
        };
        out.push(PrivateVineItemInput {
            idx: idx as i32,
            idea_id,
            via_connection_id,
        });
    }
    Ok(out)
}

pub(crate) fn ensure_pathway_items(vine_type: i16, items_len: usize) -> Result<(), Response> {
    if vine_type == 0 && items_len == 0 {
        return Err(json_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "pathway_vine requires at least one item",
        ));
    }
    Ok(())
}

pub(crate) fn parse_uuid_v7_env(value: &str) -> Option<Uuid> {
    let uuid = Uuid::parse_str(value).ok()?;
    if uuid.get_version_num() != 7 {
        return None;
    }
    Some(uuid)
}

pub(crate) fn parse_non_negative_i64(value: &str) -> Option<i64> {
    let parsed: i64 = value.parse().ok()?;
    if parsed < 0 {
        None
    } else {
        Some(parsed)
    }
}

pub(crate) fn clamp_limit(limit: i64) -> i64 {
    limit.clamp(1, 200)
}

pub(crate) fn parse_ideas_top_order(value: Option<&str>) -> Result<IdeasTopOrder, Response> {
    match value.map(|v| v.trim().to_lowercase()) {
        None => Ok(IdeasTopOrder::Asc),
        Some(v) if v == "asc" => Ok(IdeasTopOrder::Asc),
        Some(v) if v == "desc" => Ok(IdeasTopOrder::Desc),
        _ => Err(json_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "order must be asc or desc",
        )),
    }
}

pub(crate) fn parse_relative_importance_direction(
    value: Option<&str>,
) -> Result<RelativeImportanceDirection, Response> {
    match value.map(|v| v.trim().to_lowercase()) {
        None => Ok(RelativeImportanceDirection::Incoming),
        Some(v) if v.is_empty() => Ok(RelativeImportanceDirection::Incoming),
        Some(v) if v == "incoming" => Ok(RelativeImportanceDirection::Incoming),
        Some(v) if v == "outgoing" => Ok(RelativeImportanceDirection::Outgoing),
        Some(v) if v == "both" => Ok(RelativeImportanceDirection::Both),
        _ => Err(json_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "ri_dir must be incoming, outgoing, or both",
        )),
    }
}

pub(crate) fn header_value(value: &str) -> Result<HeaderValue, Response> {
    HeaderValue::from_str(value).map_err(|_| {
        json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            "invalid header value",
        )
    })
}

pub(crate) fn bearer_token(headers: &HeaderMap) -> Option<String> {
    let header = headers.get("Authorization")?.to_str().ok()?;
    let header = header.trim();
    if let Some(token) = header.strip_prefix("Bearer ") {
        let token = token.trim();
        if !token.is_empty() {
            return Some(token.to_string());
        }
    }
    None
}

pub(crate) fn validate_username(username: &str) -> Result<String, Response> {
    let username = username.trim();
    if username.is_empty() {
        return Err(json_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "username required",
        ));
    }
    if username.len() < 3 || username.len() > 32 {
        return Err(json_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "username length must be 3-32",
        ));
    }
    Ok(username.to_string())
}

pub(crate) fn validate_password(password: &str) -> Result<(), Response> {
    if password.trim().is_empty() {
        return Err(json_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "password required",
        ));
    }
    if password.len() < 8 || password.len() > 128 {
        return Err(json_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "password length must be 8-128",
        ));
    }
    Ok(())
}

pub(crate) fn validate_max_len(field: &str, value: &str, max_chars: usize) -> Result<(), Response> {
    if value.chars().count() > max_chars {
        return Err(json_error(
            StatusCode::BAD_REQUEST,
            INVALID_FIELD_LENGTH_CODE,
            &format!("{field} exceeds maximum length of {max_chars} characters"),
        ));
    }
    Ok(())
}

pub(crate) fn validate_optional_max_len(
    field: &str,
    value: Option<&str>,
    max_chars: usize,
) -> Result<(), Response> {
    if let Some(value) = value {
        validate_max_len(field, value, max_chars)?;
    }
    Ok(())
}

pub(crate) fn is_reference_scoped_connection(
    reference_idea_id: Uuid,
    row: &ConnectionRow,
    relative_importance_direction: RelativeImportanceDirection,
) -> bool {
    if row.connection_type == "relative_importance" {
        match relative_importance_direction {
            RelativeImportanceDirection::Incoming => row.to_idea_id == reference_idea_id,
            RelativeImportanceDirection::Outgoing => row.from_idea_id == reference_idea_id,
            RelativeImportanceDirection::Both => {
                row.from_idea_id == reference_idea_id || row.to_idea_id == reference_idea_id
            }
        }
    } else {
        row.from_idea_id == reference_idea_id || row.to_idea_id == reference_idea_id
    }
}

pub(crate) fn scoped_neighbor_from_reference(
    reference_idea_id: Uuid,
    row: &ConnectionRow,
    relative_importance_direction: RelativeImportanceDirection,
) -> Option<Uuid> {
    if !is_reference_scoped_connection(reference_idea_id, row, relative_importance_direction) {
        return None;
    }
    if row.connection_type == "relative_importance" {
        return match relative_importance_direction {
            RelativeImportanceDirection::Incoming => {
                if row.to_idea_id == reference_idea_id && row.from_idea_id != reference_idea_id {
                    Some(row.from_idea_id)
                } else {
                    None
                }
            }
            RelativeImportanceDirection::Outgoing => {
                if row.from_idea_id == reference_idea_id && row.to_idea_id != reference_idea_id {
                    Some(row.to_idea_id)
                } else {
                    None
                }
            }
            RelativeImportanceDirection::Both => {
                if row.from_idea_id == reference_idea_id && row.to_idea_id != reference_idea_id {
                    Some(row.to_idea_id)
                } else if row.to_idea_id == reference_idea_id
                    && row.from_idea_id != reference_idea_id
                {
                    Some(row.from_idea_id)
                } else {
                    None
                }
            }
        };
    }
    if row.from_idea_id == reference_idea_id {
        return Some(row.to_idea_id);
    }
    if row.to_idea_id == reference_idea_id {
        return Some(row.from_idea_id);
    }
    None
}
