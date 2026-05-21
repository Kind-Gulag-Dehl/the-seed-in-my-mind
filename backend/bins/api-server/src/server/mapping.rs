use api_types_canonical::{
    AuthorInfo, CanonicalRailDetail, CanonicalRailRepresentations, CanonicalRailSummary,
    ConnectionSummary, IdeaDetail, IdeaSummary, RailItem, SnapshotCommitMetadata, SnapshotMetadata,
};
#[cfg(feature = "full")]
use api_types_private::{
    PrivateIdeaDetail, PrivateIdeaSummary, PrivateVineDetail, PrivateVineSummary,
};
use axum::{
    http::{HeaderMap, StatusCode},
    response::Response,
};
use chrono::DateTime;
use storage::{
    CanonicalRailItemRow, CanonicalRailRow, CanonicalRailSummaryRow, ConnectionRow, IdeaDetailRow,
    IdeaSummaryRow, SnapshotCommitRow, SnapshotRow,
};
#[cfg(feature = "full")]
use storage::{PrivateIdeaRow, PrivateVineItemRow, PrivateVineListRow, PrivateVineRow};
use uuid::Uuid;

use crate::server::errors::{json_error, snapshot_unavailable};
#[cfg(feature = "full")]
use crate::server::helpers::private_vine_type_label;
use crate::server::helpers::{header_value, rail_kind_label, vine_type_label};

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

    headers.insert("ETag", header_value(&snapshot.snapshot_hash)?);
    headers.insert(
        "X-Snapshot-Height",
        header_value(&snapshot.block_height.to_string())?,
    );
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

pub(crate) fn canonical_rail_summary(row: &CanonicalRailSummaryRow) -> CanonicalRailSummary {
    CanonicalRailSummary {
        rail_id: row.rail_id.to_string(),
        rail_kind: rail_kind_label(row.rail_kind),
        vine_type: vine_type_label(row.vine_type),
    }
}

pub(crate) fn canonical_rail_item(row: &CanonicalRailItemRow) -> RailItem {
    RailItem {
        idx: row.idx.to_string(),
        idea_id: row.idea_id.to_string(),
        via_connection_id: row.via_connection_id.map(|value| value.to_string()),
    }
}

#[cfg(feature = "full")]
pub(crate) fn private_vine_item(row: &PrivateVineItemRow) -> RailItem {
    RailItem {
        idx: row.idx.to_string(),
        idea_id: row.idea_id.to_string(),
        via_connection_id: row.via_connection_id.map(|value| value.to_string()),
    }
}

pub(crate) fn canonical_rail_detail(
    row: &CanonicalRailRow,
    items: &[CanonicalRailItemRow],
) -> CanonicalRailDetail {
    CanonicalRailDetail {
        rail_id: row.rail_id.to_string(),
        rail_kind: rail_kind_label(row.rail_kind),
        vine_type: vine_type_label(row.vine_type),
        author_identity_id: row.author_identity_id.to_string(),
        canonical_representations: CanonicalRailRepresentations {
            title_representation_id: row.title_representation_id.map(|value| value.to_string()),
            title_payload_hash: row.title_payload_hash.clone(),
            sentence_representation_id: row
                .sentence_representation_id
                .map(|value| value.to_string()),
            sentence_payload_hash: row.sentence_payload_hash.clone(),
        },
        items: items.iter().map(canonical_rail_item).collect(),
    }
}

#[cfg(feature = "full")]
pub(crate) fn private_vine_summary(row: &PrivateVineListRow) -> PrivateVineSummary {
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
    PrivateVineSummary {
        private_vine_id: row.private_vine_id.to_string(),
        vine_type: private_vine_type_label(row.vine_type),
        title: fallback_title,
        updated_at: row.updated_at.to_rfc3339(),
        item_count: row.item_count.to_string(),
    }
}

#[cfg(feature = "full")]
pub(crate) fn private_vine_detail(
    row: &PrivateVineRow,
    items: &[PrivateVineItemRow],
) -> PrivateVineDetail {
    PrivateVineDetail {
        private_vine_id: row.private_vine_id.to_string(),
        vine_type: private_vine_type_label(row.vine_type),
        title: row.title.clone(),
        sentence: row.sentence.clone(),
        paragraph: row.paragraph.clone(),
        full: row.full.clone(),
        created_at: row.created_at.to_rfc3339(),
        updated_at: row.updated_at.to_rfc3339(),
        items: items.iter().map(private_vine_item).collect(),
    }
}
