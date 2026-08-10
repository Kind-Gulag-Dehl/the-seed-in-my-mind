use api_types_canonical::{
    AuthorInfo, CanonicalOrderingDetail, CanonicalOrderingRepresentations,
    CanonicalOrderingSummary, CanonicalRepresentationDetail, ConnectionSummary, IdeaDetail,
    IdeaSummary, OrderingItem, SnapshotBasis, SnapshotCommitMetadata, SnapshotMetadata,
};
#[cfg(feature = "full")]
use api_types_private::{
    PrivateIdeaDetail, PrivateIdeaSummary, PrivateOrderingDetail, PrivateOrderingSummary,
};
use axum::{
    http::{HeaderMap, StatusCode},
    response::Response,
};
use chrono::DateTime;
use replay::{ReplayOrderingRow, ReplayRepresentationRow};
use storage::{
    CanonicalOrderingSummaryRow, CanonicalRepresentationRow, ConnectionRow, IdeaDetailRow,
    IdeaSummaryRow, SnapshotCommitRow, SnapshotRow,
};
#[cfg(feature = "full")]
use storage::{PrivateIdeaRow, PrivateOrderingItemRow, PrivateOrderingListRow, PrivateOrderingRow};
use uuid::Uuid;

use crate::server::errors::{json_error, snapshot_unavailable};
use crate::server::helpers::{header_value, ordering_profile_label, vine_type_label};

pub(crate) fn snapshot_headers(snapshot: &SnapshotRow) -> Result<HeaderMap, Response> {
    let mut headers = HeaderMap::new();
    let state_root_hash = snapshot
        .state_root_hash
        .as_deref()
        .ok_or_else(snapshot_unavailable)?;
    let title_sentence_payload_root = snapshot
        .title_sentence_payload_root
        .as_deref()
        .ok_or_else(|| snapshot_unavailable())?;
    let shared_map_commitment = snapshot
        .shared_map_commitment
        .as_deref()
        .ok_or_else(snapshot_unavailable)?;

    let active_rulebook_set_hash = snapshot
        .active_rulebook_set_hash
        .as_deref()
        .ok_or_else(snapshot_unavailable)?;
    let last_event_id = snapshot.last_event_id.ok_or_else(snapshot_unavailable)?;
    let event_count = snapshot.event_count.ok_or_else(snapshot_unavailable)?;

    headers.insert("ETag", header_value(&snapshot.snapshot_hash)?);
    headers.insert("X-Snapshot-Id", header_value(&snapshot.snapshot_hash)?);
    headers.insert("X-Snapshot-Hash", header_value(&snapshot.snapshot_hash)?);
    headers.insert(
        "X-Snapshot-Height",
        header_value(&snapshot.block_height.to_string())?,
    );
    headers.insert(
        "X-Active-Rulebook-Set-Hash",
        header_value(active_rulebook_set_hash)?,
    );
    headers.insert("X-Last-Event-Id", header_value(&last_event_id.to_string())?);
    headers.insert("X-Event-Count", header_value(&event_count.to_string())?);
    headers.insert("X-State-Root-Hash", header_value(state_root_hash)?);
    headers.insert(
        "X-Title-Sentence-Payload-Root",
        header_value(title_sentence_payload_root)?,
    );
    headers.insert(
        "X-Shared-Map-Commitment",
        header_value(shared_map_commitment)?,
    );
    headers.insert("Cache-Control", header_value("max-age=300")?);

    Ok(headers)
}

pub(crate) fn with_headers(mut response: Response, headers: HeaderMap) -> Response {
    response.headers_mut().extend(headers);
    response
}

pub(crate) fn snapshot_metadata(snapshot: &SnapshotRow) -> Result<SnapshotMetadata, Response> {
    let state_root_hash = snapshot
        .state_root_hash
        .clone()
        .ok_or_else(snapshot_unavailable)?;
    let title_sentence_payload_root = snapshot
        .title_sentence_payload_root
        .clone()
        .ok_or_else(|| snapshot_unavailable())?;
    let shared_map_commitment = snapshot
        .shared_map_commitment
        .clone()
        .ok_or_else(|| snapshot_unavailable())?;
    let event_count = snapshot.event_count.ok_or_else(snapshot_unavailable)?;
    let approximate_timestamp: i64 = snapshot
        .approximate_timestamp
        .as_ref()
        .map(DateTime::timestamp)
        .ok_or_else(snapshot_unavailable)?;

    Ok(SnapshotMetadata {
        snapshot_id: snapshot.snapshot_hash.clone(),
        height: snapshot.block_height.to_string(),
        snapshot_hash: snapshot.snapshot_hash.clone(),
        state_root_hash,
        title_sentence_payload_root,
        shared_map_commitment,
        prev_snapshot_hash: snapshot.prev_snapshot_hash.clone(),
        event_count: event_count.to_string(),
        approximate_timestamp: approximate_timestamp.to_string(),
        cycle_index: snapshot.cycle_index.map(|value| value.to_string()),
        cycle_close_height: snapshot.cycle_close_height.map(|value| value.to_string()),
    })
}

pub(crate) fn snapshot_basis(snapshot: &SnapshotRow) -> Result<SnapshotBasis, Response> {
    Ok(SnapshotBasis {
        snapshot_id: snapshot.snapshot_hash.clone(),
        snapshot_height: snapshot.block_height.to_string(),
        snapshot_hash: snapshot.snapshot_hash.clone(),
        state_root_hash: snapshot
            .state_root_hash
            .clone()
            .ok_or_else(snapshot_unavailable)?,
        title_sentence_payload_root: snapshot
            .title_sentence_payload_root
            .clone()
            .ok_or_else(snapshot_unavailable)?,
        shared_map_commitment: snapshot
            .shared_map_commitment
            .clone()
            .ok_or_else(snapshot_unavailable)?,
        active_rulebook_set_hash: snapshot
            .active_rulebook_set_hash
            .clone()
            .ok_or_else(snapshot_unavailable)?,
        last_event_id: snapshot
            .last_event_id
            .ok_or_else(snapshot_unavailable)?
            .to_string(),
        event_count: snapshot
            .event_count
            .ok_or_else(snapshot_unavailable)?
            .to_string(),
    })
}

pub(crate) fn snapshot_commit_metadata(row: &SnapshotCommitRow) -> SnapshotCommitMetadata {
    SnapshotCommitMetadata {
        block_height: row.block_height.to_string(),
        snapshot_id: row.snapshot_hash.clone(),
        snapshot_hash: row.snapshot_hash.clone(),
        state_root_hash: row.state_root_hash.clone(),
        title_sentence_payload_root: row.title_sentence_payload_root.clone(),
        shared_map_commitment: row.shared_map_commitment.clone(),
        last_event_id: row.last_event_id.to_string(),
        event_count: row.event_count.to_string(),
        active_rulebook_set_hash: row.active_rulebook_set_hash.clone(),
        created_event_id: row.created_event_id.to_string(),
    }
}

pub(crate) fn author_info(
    speaker_identity_id: &Uuid,
    speaker_identity_title: Option<&str>,
) -> AuthorInfo {
    AuthorInfo {
        author_identity_id: Some(speaker_identity_id.to_string()),
        author_identity_title: speaker_identity_title.map(|value| value.to_string()),
        verification_level: None,
        persona_id: None,
    }
}

pub(crate) fn idea_summary(row: &IdeaSummaryRow) -> Result<IdeaSummary, Response> {
    let title = row.title.clone().ok_or_else(|| {
        json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            "missing idea title",
        )
    })?;

    Ok(IdeaSummary {
        idea_id: row.idea_id.to_string(),
        idea_type: row.idea_type.clone(),
        is_personal_space_organizer: row.is_personal_space_organizer,
        speaker_identity_id: row.speaker_identity_id.to_string(),
        speaker_identity_title: row.speaker_identity_title.clone(),
        created_event_id: row.created_event_id.to_string(),
        title,
        sentence: row.sentence.clone(),
        derived_universal_rank: row.derived_universal_rank.map(|value| value.to_string()),
        ri_in_count: row.ri_in_count.to_string(),
        ri_out_count: row.ri_out_count.to_string(),
        author: author_info(
            &row.speaker_identity_id,
            row.speaker_identity_title.as_deref(),
        ),
    })
}

pub(crate) fn idea_detail(
    row: &IdeaDetailRow,
    incoming: Vec<ConnectionSummary>,
    outgoing: Vec<ConnectionSummary>,
) -> Result<IdeaDetail, Response> {
    let title = row.title.clone().ok_or_else(|| {
        json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            "missing idea title",
        )
    })?;
    let payload_hash = row.payload_hash.clone().ok_or_else(|| {
        json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            "missing payload_hash",
        )
    })?;

    Ok(IdeaDetail {
        idea_id: row.idea_id.to_string(),
        idea_type: row.idea_type.clone(),
        is_personal_space_organizer: row.is_personal_space_organizer,
        speaker_identity_id: row.speaker_identity_id.to_string(),
        speaker_identity_title: row.speaker_identity_title.clone(),
        created_event_id: row.created_event_id.to_string(),
        title,
        sentence: row.sentence.clone(),
        derived_universal_rank: row.derived_universal_rank.map(|value| value.to_string()),
        ri_in_count: row.ri_in_count.to_string(),
        ri_out_count: row.ri_out_count.to_string(),
        derived_universal_axis_ranks: None,
        author: author_info(
            &row.speaker_identity_id,
            row.speaker_identity_title.as_deref(),
        ),
        payload_hash,
        incoming_connections: incoming,
        outgoing_connections: outgoing,
    })
}

pub(crate) fn connection_summary(row: &ConnectionRow) -> ConnectionSummary {
    ConnectionSummary {
        connection_id: row.connection_id.to_string(),
        from_idea_id: row.from_idea_id.to_string(),
        to_idea_id: row.to_idea_id.to_string(),
        connection_type: row.connection_type.clone(),
        created_by_event_id: row.created_by_event_id.to_string(),
        usage: row.usage.clone(),
        axis: row.axis.clone(),
        timeframe: row.timeframe.clone(),
        scope: row.scope.clone(),
        value_representation: None,
        certainty_band: None,
        weight: None,
    }
}

#[cfg(feature = "full")]
pub(crate) fn private_idea_summary(row: &PrivateIdeaRow) -> PrivateIdeaSummary {
    PrivateIdeaSummary {
        idea_id: row.idea_id.to_string(),
        idea_type: "private_draft".to_string(),
        scope: "private".to_string(),
        title: row.title.clone(),
        sentence: row.sentence.clone(),
        updated_at: row.updated_at.to_rfc3339(),
    }
}

#[cfg(feature = "full")]
pub(crate) fn private_idea_detail(row: &PrivateIdeaRow) -> PrivateIdeaDetail {
    PrivateIdeaDetail {
        idea_id: row.idea_id.to_string(),
        idea_type: "private_draft".to_string(),
        scope: "private".to_string(),
        title: row.title.clone(),
        sentence: row.sentence.clone(),
        paragraph: row.paragraph.clone(),
        full: row.full.clone(),
        created_at: row.created_at.to_rfc3339(),
        updated_at: row.updated_at.to_rfc3339(),
    }
}

pub(crate) fn canonical_ordering_summary(
    row: &CanonicalOrderingSummaryRow,
) -> CanonicalOrderingSummary {
    CanonicalOrderingSummary {
        ordering_id: row.ordering_id.to_string(),
        ordering_profile: ordering_profile_label(row.ordering_profile),
        vine_type: vine_type_label(row.vine_type),
        subject_idea_id: row.subject_idea_id.map(|value| value.to_string()),
    }
}

#[cfg(feature = "full")]
pub(crate) fn private_ordering_item(row: &PrivateOrderingItemRow) -> OrderingItem {
    OrderingItem {
        idx: row.idx.to_string(),
        idea_id: row.idea_id.to_string(),
        item_role: None,
        via_connection_id: row.via_connection_id.map(|value| value.to_string()),
    }
}

pub(crate) fn canonical_ordering_detail_from_replay(
    row: &ReplayOrderingRow,
    representations: &[ReplayRepresentationRow],
    item_limit: usize,
) -> CanonicalOrderingDetail {
    let payload_hash = |representation_id: Option<Uuid>| {
        representation_id.and_then(|representation_id| {
            representations
                .iter()
                .find(|representation| representation.representation_id == representation_id)
                .map(|representation| representation.payload_hash.clone())
        })
    };
    CanonicalOrderingDetail {
        ordering_id: row.ordering_id.to_string(),
        ordering_profile: row.ordering_profile.clone(),
        vine_type: row.vine_type.clone(),
        subject_idea_id: row.subject_idea_id.map(|value| value.to_string()),
        author_identity_id: row.speaker_identity_id.to_string(),
        canonical_representations: CanonicalOrderingRepresentations {
            title_representation_id: row.title_representation_id.map(|value| value.to_string()),
            title_payload_hash: payload_hash(row.title_representation_id),
            sentence_representation_id: row
                .sentence_representation_id
                .map(|value| value.to_string()),
            sentence_payload_hash: payload_hash(row.sentence_representation_id),
        },
        items: row
            .items
            .iter()
            .take(item_limit)
            .map(|item| OrderingItem {
                idx: item.idx.to_string(),
                idea_id: item.idea_id.to_string(),
                item_role: item.item_role.clone(),
                via_connection_id: item.via_connection_id.map(|value| value.to_string()),
            })
            .collect(),
    }
}

pub(crate) fn canonical_representation_detail(
    row: &CanonicalRepresentationRow,
) -> CanonicalRepresentationDetail {
    CanonicalRepresentationDetail {
        representation_id: row.representation_id.to_string(),
        target_kind: representation_target_kind_label(row.target_kind),
        target_object_id: row.target_id.to_string(),
        representation_kind: representation_kind_label(row.tier_enum),
        tier_length: representation_tier_length_label(row.tier_enum),
        tier_complexity: row
            .tier_complexity
            .map(representation_tier_complexity_label),
        vocabulary_version_id: row.vocabulary_version_id.map(|value| value.to_string()),
        payload_hash: row.payload_hash.clone(),
        payload_text: row.payload_text.clone(),
        author_identity_id: row.author_identity_id.to_string(),
        language_locale: row.language_locale.clone(),
        provenance: row.provenance.clone(),
        created_event_id: row.created_event_id.to_string(),
        created_block_height: row.created_block_height.to_string(),
        created_event_index: row.created_event_index.to_string(),
    }
}

fn representation_target_kind_label(target_kind: i16) -> String {
    match target_kind {
        0 => "idea".to_string(),
        1 => "ordering".to_string(),
        other => format!("unknown_{other}"),
    }
}

fn representation_kind_label(tier_length: i16) -> String {
    if tier_length == 0 {
        "title".to_string()
    } else {
        "description".to_string()
    }
}

fn representation_tier_length_label(tier_length: i16) -> Option<String> {
    match tier_length {
        0 => None,
        1 => Some("sentence".to_string()),
        2 => Some("paragraph".to_string()),
        3 => Some("full".to_string()),
        other => Some(format!("unknown_{other}")),
    }
}

fn representation_tier_complexity_label(tier_complexity: i16) -> String {
    match tier_complexity {
        0 => "fundamental".to_string(),
        1 => "standard".to_string(),
        2 => "advanced".to_string(),
        3 => "canonical".to_string(),
        other => format!("unknown_{other}"),
    }
}

#[cfg(feature = "full")]
pub(crate) fn private_ordering_summary(row: &PrivateOrderingListRow) -> PrivateOrderingSummary {
    let fallback_title = row
        .title
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .or_else(|| {
            row.sentence
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| value.to_string())
        })
        .unwrap_or_else(|| "Untitled vine".to_string());
    PrivateOrderingSummary {
        private_ordering_id: row.private_ordering_id.to_string(),
        ordering_profile: ordering_profile_label(row.ordering_profile),
        vine_type: vine_type_label(row.vine_type),
        title: fallback_title,
        updated_at: row.updated_at.to_rfc3339(),
        item_count: row.item_count.to_string(),
    }
}

#[cfg(feature = "full")]
pub(crate) fn private_ordering_detail(
    row: &PrivateOrderingRow,
    items: &[PrivateOrderingItemRow],
) -> PrivateOrderingDetail {
    PrivateOrderingDetail {
        private_ordering_id: row.private_ordering_id.to_string(),
        ordering_profile: ordering_profile_label(row.ordering_profile),
        vine_type: vine_type_label(row.vine_type),
        title: row.title.clone(),
        sentence: row.sentence.clone(),
        paragraph: row.paragraph.clone(),
        full: row.full.clone(),
        created_at: row.created_at.to_rfc3339(),
        updated_at: row.updated_at.to_rfc3339(),
        items: items.iter().map(private_ordering_item).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v7(id: &str) -> Uuid {
        Uuid::parse_str(id).expect("uuid parse")
    }

    #[test]
    fn title_representation_maps_to_the_public_dto_without_description_fields() {
        let representation_id = v7("00000000-0000-7000-8000-00000000d001");
        let target_object_id = v7("00000000-0000-7000-8000-00000000c001");
        let author_identity_id = v7("00000000-0000-7000-8000-00000000a001");
        let created_event_id = v7("00000000-0000-7000-8000-00000000e001");
        let payload_hash = "1111111111111111111111111111111111111111111111111111111111111111";
        let row = CanonicalRepresentationRow {
            representation_id,
            target_kind: 0,
            target_id: target_object_id,
            tier_enum: 0,
            tier_complexity: None,
            vocabulary_version_id: None,
            payload_hash: payload_hash.to_string(),
            payload_text: Some("Example title".to_string()),
            author_identity_id,
            language_locale: None,
            provenance: None,
            created_event_id,
            created_block_height: 2,
            created_event_index: 0,
        };

        let detail = canonical_representation_detail(&row);
        assert_eq!(detail.representation_id, representation_id.to_string());
        assert_eq!(detail.target_kind, "idea");
        assert_eq!(detail.target_object_id, target_object_id.to_string());
        assert_eq!(detail.representation_kind, "title");
        assert_eq!(detail.tier_length, None);
        assert_eq!(detail.tier_complexity, None);
        assert_eq!(detail.vocabulary_version_id, None);
        assert_eq!(detail.payload_hash, payload_hash);
        assert_eq!(detail.author_identity_id, author_identity_id.to_string());
        assert_eq!(detail.created_event_id, created_event_id.to_string());

        let body = serde_json::to_value(&detail).expect("serialize representation detail");
        let representation = body.as_object().expect("representation object");
        assert_eq!(
            representation
                .get("representation_kind")
                .and_then(serde_json::Value::as_str),
            Some("title")
        );
        assert!(!representation.contains_key("tier_length"));
        assert!(!representation.contains_key("tier_complexity"));
        assert!(!representation.contains_key("vocabulary_version_id"));
    }
}
