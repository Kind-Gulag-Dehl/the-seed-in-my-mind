use chrono::{DateTime, Utc};
use encoding::canonical::{
    canonicalize_string, encode_id, encode_string, encode_u16, encode_u32, encode_u64, encode_u8,
    encode_varint_u64,
};
use encoding::hash::{hash_bytes, hash_with_domain};
use encoding::merkle::{compute_root_with_tags, empty_payload_root};
use encoding::payload::payload_hash_hex;
use replay::{
    ReplayConnectionRow, ReplayIdeaRow, ReplayObjectKind, ReplayOrderingRow, ReplayOutput,
    ReplayPayloadRow, ReplayRepresentationRow,
};
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub const IDEAS_SECTION_ID: u16 = 0x8001;
pub const CONNECTIONS_SECTION_ID: u16 = 0x8002;
pub const REPRESENTATIONS_SECTION_ID: u16 = 0x8003;
pub const ORDERINGS_SECTION_ID: u16 = 0x000F;
pub const ORDERING_REPRESENTATION_INDEX_SECTION_ID: u16 = 0x0010;

#[derive(Debug, Clone)]
pub struct SnapshotFormat {
    pub height: u64,
    pub state_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotSection {
    pub id: u16,
    pub item_count: u32,
    pub bytes: Vec<u8>,
    pub hash: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Stage0Commitments {
    pub state_root_hash: Vec<u8>,
    pub title_sentence_payload_root: Vec<u8>,
    pub shared_map_commitment: Vec<u8>,
    pub active_rulebook_set_hash: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Stage0Snapshot {
    pub bytes: Vec<u8>,
    pub snapshot_hash: Vec<u8>,
    pub commitments: Stage0Commitments,
    pub sections: Vec<SnapshotSection>,
    pub height: i64,
    pub last_event_id: Uuid,
    pub event_count: i64,
    pub approximate_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotError {
    pub code: &'static str,
    pub message: String,
}

impl SnapshotError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for SnapshotError {}

pub fn build_stage0_snapshot(replay: &ReplayOutput) -> Result<Stage0Snapshot, SnapshotError> {
    let sections = build_stage0_sections(
        &replay.ideas,
        &replay.connections,
        &replay.orderings,
        &replay.representations,
    )?;
    let commitments = compute_stage0_commitments(&sections, &replay.payloads)?;

    let bytes = encode_snapshot_v0(replay, &commitments, &sections, "", 0)?;
    let snapshot_hash = hash_with_domain("snapshot", &bytes);

    Ok(Stage0Snapshot {
        bytes,
        snapshot_hash,
        commitments,
        sections,
        height: replay.height,
        last_event_id: replay.last_event_id,
        event_count: replay.event_count,
        approximate_timestamp: replay.approximate_timestamp,
    })
}

pub fn encode_snapshot_v0(
    replay: &ReplayOutput,
    commitments: &Stage0Commitments,
    sections: &[SnapshotSection],
    snapshot_tier_id: &str,
    snapshot_kind: u8,
) -> Result<Vec<u8>, SnapshotError> {
    let height = replay.height;
    if height < 0 {
        return Err(SnapshotError::new(
            "invalid_height",
            "snapshot height must be non-negative",
        ));
    }
    if replay.event_count < 0 {
        return Err(SnapshotError::new(
            "invalid_event_count",
            "event_count must be non-negative",
        ));
    }

    let mut header_body = Vec::new();
    header_body.extend_from_slice(&encode_u64(height as u64));
    header_body.extend_from_slice(&encode_u8(snapshot_kind));
    header_body.extend_from_slice(&encode_string(snapshot_tier_id));
    header_body.extend_from_slice(
        &encode_id(&replay.last_event_id.to_string())
            .map_err(|err| SnapshotError::new("invalid_id", err))?,
    );
    header_body.extend_from_slice(&encode_u64(replay.event_count as u64));
    header_body.extend_from_slice(&commitments.active_rulebook_set_hash);
    header_body.extend_from_slice(&commitments.state_root_hash);
    header_body.extend_from_slice(&commitments.title_sentence_payload_root);
    header_body
        .extend_from_slice(&encode_u16(u16::try_from(sections.len()).map_err(
            |_| SnapshotError::new("invalid_section_count", "too many sections"),
        )?));

    for section in sections {
        header_body.extend_from_slice(&encode_u16(section.id));
        header_body.extend_from_slice(&encode_u32(section.item_count));
        header_body.extend_from_slice(&encode_u64(section.bytes.len() as u64));
        header_body.extend_from_slice(&section.hash);
    }

    let header_byte_len = u32::try_from(header_body.len())
        .map_err(|_| SnapshotError::new("invalid_header", "header too large"))?;

    let mut header_bytes = Vec::new();
    header_bytes.extend_from_slice(b"MCCSNAP0");
    header_bytes.extend_from_slice(&encode_u16(0));
    header_bytes.extend_from_slice(&encode_u16(0));
    header_bytes.extend_from_slice(&encode_u32(header_byte_len));
    header_bytes.extend_from_slice(&header_body);

    let mut body_bytes = Vec::new();
    for section in sections {
        body_bytes.extend_from_slice(&section.bytes);
    }

    let mut snapshot_bytes = Vec::new();
    snapshot_bytes.extend_from_slice(&header_bytes);
    snapshot_bytes.extend_from_slice(&body_bytes);

    Ok(snapshot_bytes)
}

pub fn compute_title_sentence_payload_root(
    payloads: &[ReplayPayloadRow],
) -> Result<Vec<u8>, SnapshotError> {
    if payloads.is_empty() {
        return Ok(empty_payload_root());
    }

    let mut leaves = Vec::with_capacity(payloads.len() * 2);
    for row in payloads {
        let object_id = row.object_id.to_string();
        let title = row.title.as_deref().ok_or_else(|| {
            SnapshotError::new("missing_title", format!("missing title for {object_id}"))
        })?;
        let sentence = row.sentence.as_deref().ok_or_else(|| {
            SnapshotError::new(
                "missing_sentence",
                format!("missing sentence for {object_id}"),
            )
        })?;

        let title_bytes = canonicalize_string_anyhow(title)?;
        let sentence_bytes = canonicalize_string_anyhow(sentence)?;
        let title_hash = hash_bytes(&title_bytes);
        let sentence_hash = hash_bytes(&sentence_bytes);

        let mut title_leaf = Vec::new();
        title_leaf.extend_from_slice(&encode_u8(row.object_kind.as_u8()));
        title_leaf.extend_from_slice(&encode_id_anyhow(&object_id)?);
        title_leaf.extend_from_slice(&encode_u8(0));
        title_leaf.extend_from_slice(&title_hash);
        leaves.push(title_leaf);

        let mut sentence_leaf = Vec::new();
        sentence_leaf.extend_from_slice(&encode_u8(row.object_kind.as_u8()));
        sentence_leaf.extend_from_slice(&encode_id_anyhow(&object_id)?);
        sentence_leaf.extend_from_slice(&encode_u8(1));
        sentence_leaf.extend_from_slice(&sentence_hash);
        leaves.push(sentence_leaf);
    }

    let root = compute_root_with_tags(
        &leaves,
        "seed-merkle-leaf",
        "seed-merkle-node",
        true,
        Some(empty_payload_root()),
    );
    Ok(root.0)
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    to_hex(&digest)
}

pub fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

fn compute_stage0_commitments(
    sections: &[SnapshotSection],
    payloads: &[ReplayPayloadRow],
) -> Result<Stage0Commitments, SnapshotError> {
    let ideas = sections
        .iter()
        .find(|section| section.id == IDEAS_SECTION_ID)
        .ok_or_else(|| SnapshotError::new("missing_section", "ideas section missing"))?;
    let connections = sections
        .iter()
        .find(|section| section.id == CONNECTIONS_SECTION_ID)
        .ok_or_else(|| SnapshotError::new("missing_section", "connections section missing"))?;
    let orderings = sections
        .iter()
        .find(|section| section.id == ORDERINGS_SECTION_ID)
        .ok_or_else(|| SnapshotError::new("missing_section", "orderings section missing"))?;
    let representations = sections
        .iter()
        .find(|section| section.id == REPRESENTATIONS_SECTION_ID)
        .ok_or_else(|| SnapshotError::new("missing_section", "representations section missing"))?;

    let mut state_root_payload = Vec::new();
    state_root_payload.extend_from_slice(&ideas.hash);
    state_root_payload.extend_from_slice(&connections.hash);
    state_root_payload.extend_from_slice(&orderings.hash);
    state_root_payload.extend_from_slice(&representations.hash);
    let state_root_hash = hash_with_domain("snapshot_state_root", &state_root_payload);

    validate_payload_hashes(payloads)?;
    let title_sentence_payload_root = compute_title_sentence_payload_root(payloads)?;

    let shared_map_commitment = hash_with_domain(
        "shared_map_commitment_v0",
        &[
            state_root_hash.as_slice(),
            title_sentence_payload_root.as_slice(),
        ]
        .concat(),
    );

    let active_rulebook_set_hash = hash_with_domain("snapshot_rulebook_set", &[]);

    Ok(Stage0Commitments {
        state_root_hash,
        title_sentence_payload_root,
        shared_map_commitment,
        active_rulebook_set_hash,
    })
}

fn build_stage0_sections(
    ideas: &[ReplayIdeaRow],
    connections: &[ReplayConnectionRow],
    orderings: &[ReplayOrderingRow],
    representations: &[ReplayRepresentationRow],
) -> Result<Vec<SnapshotSection>, SnapshotError> {
    let (ideas_bytes, ideas_count) = build_ideas_section(ideas)?;
    let (connections_bytes, connections_count) = build_connections_section(connections)?;
    let (orderings_bytes, orderings_count) = build_orderings_section(orderings, ideas)?;
    let (representations_bytes, representations_count) =
        build_representations_section(representations)?;
    let (ordering_representation_index_bytes, ordering_representation_index_count) =
        build_ordering_representation_index_section(orderings)?;

    let ideas_hash = section_hash(IDEAS_SECTION_ID, &ideas_bytes);
    let connections_hash = section_hash(CONNECTIONS_SECTION_ID, &connections_bytes);
    let orderings_hash = section_hash(ORDERINGS_SECTION_ID, &orderings_bytes);
    let representations_hash = section_hash(REPRESENTATIONS_SECTION_ID, &representations_bytes);
    let ordering_representation_index_hash = section_hash(
        ORDERING_REPRESENTATION_INDEX_SECTION_ID,
        &ordering_representation_index_bytes,
    );

    Ok(vec![
        SnapshotSection {
            id: IDEAS_SECTION_ID,
            item_count: ideas_count,
            bytes: ideas_bytes,
            hash: ideas_hash,
        },
        SnapshotSection {
            id: CONNECTIONS_SECTION_ID,
            item_count: connections_count,
            bytes: connections_bytes,
            hash: connections_hash,
        },
        SnapshotSection {
            id: ORDERINGS_SECTION_ID,
            item_count: orderings_count,
            bytes: orderings_bytes,
            hash: orderings_hash,
        },
        SnapshotSection {
            id: REPRESENTATIONS_SECTION_ID,
            item_count: representations_count,
            bytes: representations_bytes,
            hash: representations_hash,
        },
        SnapshotSection {
            id: ORDERING_REPRESENTATION_INDEX_SECTION_ID,
            item_count: ordering_representation_index_count,
            bytes: ordering_representation_index_bytes,
            hash: ordering_representation_index_hash,
        },
    ])
}

fn build_ideas_section(ideas: &[ReplayIdeaRow]) -> Result<(Vec<u8>, u32), SnapshotError> {
    let mut sorted = ideas.to_vec();
    sorted.sort_by_key(|row| (row.created_block_height, row.created_event_index));

    let mut out = Vec::new();
    for row in &sorted {
        out.extend_from_slice(&encode_id_anyhow(&row.idea_id.to_string())?);
        out.extend_from_slice(&encode_string_canon(&row.idea_type)?);
        out.extend_from_slice(&encode_id_anyhow(&row.speaker_identity_id.to_string())?);
        out.extend_from_slice(&encode_id_anyhow(&row.created_event_id.to_string())?);
        out.extend_from_slice(&encode_u64(row.created_block_height as u64));
        out.extend_from_slice(&encode_u32(row.created_event_index as u32));
    }
    let count = u32::try_from(sorted.len())
        .map_err(|_| SnapshotError::new("invalid_count", "ideas count overflow"))?;
    Ok((out, count))
}

fn build_connections_section(
    connections: &[ReplayConnectionRow],
) -> Result<(Vec<u8>, u32), SnapshotError> {
    let mut sorted = connections.to_vec();
    sorted.sort_by_key(|row| (row.created_block_height, row.created_event_index));

    let mut out = Vec::new();
    for row in &sorted {
        out.extend_from_slice(&encode_id_anyhow(&row.connection_id.to_string())?);
        out.extend_from_slice(&encode_id_anyhow(&row.from_idea_id.to_string())?);
        out.extend_from_slice(&encode_id_anyhow(&row.to_idea_id.to_string())?);
        out.extend_from_slice(&encode_string_canon(&row.connection_type)?);
        push_optional_string(&mut out, row.usage.as_deref())?;
        push_optional_string(&mut out, row.axis.as_deref())?;
        push_optional_string(&mut out, row.timeframe.as_deref())?;
        push_optional_string(&mut out, row.scope.as_deref())?;
        out.extend_from_slice(&encode_id_anyhow(&row.created_by_event_id.to_string())?);
        out.extend_from_slice(&encode_u64(row.created_block_height as u64));
        out.extend_from_slice(&encode_u32(row.created_event_index as u32));
    }
    let count = u32::try_from(sorted.len())
        .map_err(|_| SnapshotError::new("invalid_count", "connections count overflow"))?;
    Ok((out, count))
}

fn build_representations_section(
    representations: &[ReplayRepresentationRow],
) -> Result<(Vec<u8>, u32), SnapshotError> {
    let mut sorted = representations.to_vec();
    sorted.sort_by_key(|row| (row.created_block_height, row.created_event_index));

    let mut seen = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for row in &sorted {
        if !seen.insert(row.representation_id) {
            return Err(SnapshotError::new(
                "duplicate_representation",
                format!("duplicate representation_id={}", row.representation_id),
            ));
        }
        out.extend_from_slice(&encode_id_anyhow(&row.representation_id.to_string())?);
        out.extend_from_slice(&encode_u8(match row.target_kind.as_str() {
            "idea" => 0,
            "ordering" => 1,
            other => {
                return Err(SnapshotError::new(
                    "invalid_target_kind",
                    format!("unsupported representation target_kind={other}"),
                ))
            }
        }));
        out.extend_from_slice(&encode_id_anyhow(&row.target_object_id.to_string())?);

        match row.representation_kind.as_str() {
            "title" => {
                if row.tier_length.is_some()
                    || row.tier_complexity.is_some()
                    || row.vocabulary_version_id.is_some()
                {
                    return Err(SnapshotError::new(
                        "invalid_representation_slot",
                        format!(
                            "title representation carries description fields: {}",
                            row.representation_id
                        ),
                    ));
                }
                out.extend_from_slice(&encode_u8(0));
                out.extend_from_slice(&encode_u8(0));
            }
            "description" => {
                out.extend_from_slice(&encode_u8(1));
                let length = match row.tier_length.as_deref() {
                    Some("sentence") => 0,
                    Some("paragraph") => 1,
                    Some("full") => 2,
                    _ => {
                        return Err(SnapshotError::new(
                            "invalid_tier_length",
                            format!(
                                "description has invalid tier_length: {}",
                                row.representation_id
                            ),
                        ))
                    }
                };
                let complexity = match row.tier_complexity.as_deref() {
                    Some("fundamental") => 0,
                    Some("standard") => 1,
                    Some("advanced") => 2,
                    Some("canonical") => 3,
                    _ => {
                        return Err(SnapshotError::new(
                            "invalid_tier_complexity",
                            format!(
                                "description has invalid tier_complexity: {}",
                                row.representation_id
                            ),
                        ))
                    }
                };
                out.extend_from_slice(&encode_u8(length));
                out.extend_from_slice(&encode_u8(complexity));
                match (complexity, row.vocabulary_version_id) {
                    (3, Some(vocabulary_id)) => {
                        out.extend_from_slice(&encode_u8(1));
                        out.extend_from_slice(&encode_id_anyhow(&vocabulary_id.to_string())?);
                    }
                    (3, None) => {
                        return Err(SnapshotError::new(
                            "missing_vocabulary",
                            format!(
                                "canonical description missing vocabulary: {}",
                                row.representation_id
                            ),
                        ))
                    }
                    (_, Some(_)) => {
                        return Err(SnapshotError::new(
                            "invalid_vocabulary",
                            format!(
                                "noncanonical description carries vocabulary: {}",
                                row.representation_id
                            ),
                        ))
                    }
                    (_, None) => out.extend_from_slice(&encode_u8(0)),
                }
            }
            other => {
                return Err(SnapshotError::new(
                    "invalid_representation_kind",
                    format!("unsupported representation_kind={other}"),
                ))
            }
        }

        out.extend_from_slice(&encode_id_anyhow(&row.author_identity_id.to_string())?);
        let payload_hash = decode_hex_32(&row.payload_hash)?;
        if let Some(text) = row.payload_text.as_deref() {
            let actual = hash_bytes(&canonicalize_string_anyhow(text)?);
            if actual.as_slice() != payload_hash.as_slice() {
                return Err(SnapshotError::new(
                    "payload_hash_mismatch",
                    format!(
                        "representation payload hash mismatch: {}",
                        row.representation_id
                    ),
                ));
            }
        }
        out.extend_from_slice(&payload_hash);
        match row.payload_text.as_deref() {
            Some(text) => {
                out.extend_from_slice(&encode_u8(1));
                out.extend_from_slice(&encode_string_canon(text)?);
            }
            None => out.extend_from_slice(&encode_u8(0)),
        }
        push_optional_string(&mut out, row.language_locale.as_deref())?;
        push_optional_string(&mut out, row.provenance.as_deref())?;
        out.extend_from_slice(&encode_id_anyhow(&row.created_event_id.to_string())?);
        out.extend_from_slice(&encode_u64(row.created_block_height as u64));
        out.extend_from_slice(&encode_u32(row.created_event_index as u32));
    }

    let count = u32::try_from(sorted.len())
        .map_err(|_| SnapshotError::new("invalid_count", "representations count overflow"))?;
    Ok((out, count))
}

fn build_orderings_section(
    orderings: &[ReplayOrderingRow],
    ideas: &[ReplayIdeaRow],
) -> Result<(Vec<u8>, u32), SnapshotError> {
    let mut sorted = orderings.to_vec();
    sorted.sort_by_key(|row| (row.created_block_height, row.created_event_index));
    let profiles = sorted
        .iter()
        .map(|row| (row.ordering_id, row.ordering_profile.as_str()))
        .collect::<std::collections::HashMap<_, _>>();
    let ordering_by_id = sorted
        .iter()
        .map(|row| (row.ordering_id, row))
        .collect::<std::collections::HashMap<_, _>>();
    let idea_types = ideas
        .iter()
        .map(|row| (row.idea_id, row.idea_type.as_str()))
        .collect::<std::collections::HashMap<_, _>>();

    let mut out = Vec::new();
    for row in &sorted {
        let profile_code = ordering_profile_code(&row.ordering_profile)?;
        match (profile_code, row.vine_type.as_deref()) {
            (0, Some(vine_type)) => {
                vine_type_code(vine_type)?;
            }
            (0, None) => {
                return Err(SnapshotError::new(
                    "missing_vine_type",
                    format!("missing vine_type for ordering_id={}", row.ordering_id),
                ));
            }
            (_, Some(_)) => {
                return Err(SnapshotError::new(
                    "invalid_vine_type",
                    format!(
                        "vine_type is forbidden for ordering_profile={} ordering_id={}",
                        row.ordering_profile, row.ordering_id
                    ),
                ));
            }
            (_, None) => {}
        }
        match (profile_code, row.subject_idea_id) {
            (0, None) => {}
            (0, Some(_)) => {
                return Err(SnapshotError::new(
                    "invalid_subject",
                    format!("Vine must not carry a subject: {}", row.ordering_id),
                ))
            }
            (1, Some(subject_id)) => {
                if idea_types.get(&subject_id) != Some(&"truth_claim") {
                    return Err(SnapshotError::new(
                        "invalid_subject",
                        format!(
                            "Evidence Rail requires truth_claim subject: {}",
                            row.ordering_id
                        ),
                    ));
                }
            }
            (2, Some(subject_id)) => {
                if idea_types.get(&subject_id) != Some(&"actionable_idea") {
                    return Err(SnapshotError::new(
                        "invalid_subject",
                        format!(
                            "Action Rail requires actionable_idea subject: {}",
                            row.ordering_id
                        ),
                    ));
                }
            }
            (_, None) => {
                return Err(SnapshotError::new(
                    "missing_subject",
                    format!("standardized Ordering missing subject: {}", row.ordering_id),
                ))
            }
            _ => unreachable!(),
        }
        if profile_code != 0 {
            if row.items.is_empty() {
                return Err(SnapshotError::new(
                    "missing_ordering_item",
                    format!("standardized Ordering has no items: {}", row.ordering_id),
                ));
            }
            let mut ids = std::collections::BTreeSet::new();
            for item in &row.items {
                if !ids.insert(item.idea_id) {
                    return Err(SnapshotError::new(
                        "duplicate_ordering_item",
                        format!(
                            "standardized Ordering contains duplicate item {}",
                            item.idea_id
                        ),
                    ));
                }
                let role = item_role_code(item.item_role.as_deref())?.ok_or_else(|| {
                    SnapshotError::new(
                        "missing_item_role",
                        format!(
                            "standardized Ordering item missing role: {}",
                            row.ordering_id
                        ),
                    )
                })?;
                if (profile_code == 1 && !matches!(role, 0 | 1))
                    || (profile_code == 2 && !matches!(role, 2 | 3))
                {
                    return Err(SnapshotError::new(
                        "invalid_item_role",
                        format!("invalid item role for ordering_id={}", row.ordering_id),
                    ));
                }
            }
            if profile_code == 2 {
                let lane = item_role_code(row.items[0].item_role.as_deref())?
                    .expect("standardized role checked above");
                if row.items.iter().any(|item| {
                    item_role_code(item.item_role.as_deref()).ok().flatten() != Some(lane)
                }) {
                    return Err(SnapshotError::new(
                        "invalid_action_lane",
                        format!("Action Rail lane is not homogeneous: {}", row.ordering_id),
                    ));
                }
            }
        } else if row.items.iter().any(|item| item.item_role.is_some()) {
            return Err(SnapshotError::new(
                "invalid_item_role",
                format!("Vine must not carry item roles: {}", row.ordering_id),
            ));
        }
        if let Some(base_ordering_id) = row.base_ordering_id {
            let base_profile = profiles.get(&base_ordering_id).ok_or_else(|| {
                SnapshotError::new(
                    "missing_base_ordering",
                    format!("missing base_ordering_id={base_ordering_id}"),
                )
            })?;
            if *base_profile != row.ordering_profile.as_str() {
                return Err(SnapshotError::new(
                    "ordering_profile_mismatch",
                    format!(
                        "fork profile differs from base for ordering_id={}",
                        row.ordering_id
                    ),
                ));
            }
            let base = ordering_by_id[&base_ordering_id];
            if base.subject_idea_id != row.subject_idea_id {
                return Err(SnapshotError::new(
                    "ordering_subject_mismatch",
                    format!("fork subject differs from base: {}", row.ordering_id),
                ));
            }
            let base_roles = base
                .items
                .iter()
                .map(|item| (item.idea_id, item.item_role.as_deref()))
                .collect::<std::collections::HashMap<_, _>>();
            for item in &row.items {
                if let Some(base_role) = base_roles.get(&item.idea_id) {
                    if *base_role != item.item_role.as_deref() {
                        return Err(SnapshotError::new(
                            "ordering_role_mismatch",
                            format!("fork changed retained-item role: {}", item.idea_id),
                        ));
                    }
                }
            }
            if profile_code == 2 {
                let base_lane = base
                    .items
                    .first()
                    .and_then(|item| item_role_code(item.item_role.as_deref()).ok().flatten());
                let fork_lane = row
                    .items
                    .first()
                    .and_then(|item| item_role_code(item.item_role.as_deref()).ok().flatten());
                if base_lane.is_none() || base_lane != fork_lane {
                    return Err(SnapshotError::new(
                        "ordering_lane_mismatch",
                        format!("Action Rail fork changed base lane: {}", row.ordering_id),
                    ));
                }
            }
        }

        out.extend_from_slice(&encode_id_anyhow(&row.ordering_id.to_string())?);
        out.extend_from_slice(&encode_u8(profile_code));
        match row.vine_type.as_deref() {
            Some(vine_type) => {
                out.extend_from_slice(&encode_u8(1));
                out.extend_from_slice(&encode_u8(vine_type_code(vine_type)?));
            }
            None => out.extend_from_slice(&encode_u8(0)),
        }
        match row.subject_idea_id {
            Some(subject_id) => {
                out.extend_from_slice(&encode_u8(1));
                out.extend_from_slice(&encode_id_anyhow(&subject_id.to_string())?);
            }
            None => out.extend_from_slice(&encode_u8(0)),
        }
        out.extend_from_slice(&encode_id_anyhow(&row.speaker_identity_id.to_string())?);
        out.extend_from_slice(&encode_id_anyhow(&row.created_event_id.to_string())?);
        match row.base_ordering_id {
            Some(base_ordering_id) => {
                out.extend_from_slice(&encode_u8(1));
                out.extend_from_slice(&encode_id_anyhow(&base_ordering_id.to_string())?);
            }
            None => {
                out.extend_from_slice(&encode_u8(0));
            }
        }

        out.extend_from_slice(&encode_u32(u32::try_from(row.items.len()).map_err(
            |_| SnapshotError::new("invalid_count", "ordering items count overflow"),
        )?));
        for item in &row.items {
            out.extend_from_slice(&encode_id_anyhow(&item.idea_id.to_string())?);
            match item_role_code(item.item_role.as_deref())? {
                Some(role) => {
                    out.extend_from_slice(&encode_u8(1));
                    out.extend_from_slice(&encode_u8(role));
                }
                None => out.extend_from_slice(&encode_u8(0)),
            }
        }
        out.extend_from_slice(&encode_u32(u32::try_from(row.items.len()).map_err(
            |_| SnapshotError::new("invalid_count", "ordering step metadata count overflow"),
        )?));
        for item in &row.items {
            match item.via_connection_id {
                Some(via_connection_id) => {
                    out.extend_from_slice(&encode_u8(1));
                    out.extend_from_slice(&encode_id_anyhow(&via_connection_id.to_string())?);
                }
                None => {
                    out.extend_from_slice(&encode_u8(0));
                }
            }
        }
    }

    let count = u32::try_from(sorted.len())
        .map_err(|_| SnapshotError::new("invalid_count", "orderings count overflow"))?;
    Ok((out, count))
}

fn item_role_code(value: Option<&str>) -> Result<Option<u8>, SnapshotError> {
    match value {
        None => Ok(None),
        Some("potential_evidence") => Ok(Some(0)),
        Some("actual_evidence") => Ok(Some(1)),
        Some("potential_action") => Ok(Some(2)),
        Some("proposed_action") => Ok(Some(3)),
        Some(other) => Err(SnapshotError::new(
            "invalid_item_role",
            format!("unsupported item_role={other}"),
        )),
    }
}

fn build_ordering_representation_index_section(
    orderings: &[ReplayOrderingRow],
) -> Result<(Vec<u8>, u32), SnapshotError> {
    let mut sorted = orderings.to_vec();
    sorted.sort_by_key(|row| (row.created_block_height, row.created_event_index));

    let mut out = Vec::new();
    for row in &sorted {
        let title_representation_id = row.title_representation_id.ok_or_else(|| {
            SnapshotError::new(
                "missing_representation_pointer",
                format!(
                    "missing title representation pointer for ordering_id={}",
                    row.ordering_id
                ),
            )
        })?;
        let sentence_representation_id = row.sentence_representation_id.ok_or_else(|| {
            SnapshotError::new(
                "missing_representation_pointer",
                format!(
                    "missing sentence representation pointer for ordering_id={}",
                    row.ordering_id
                ),
            )
        })?;
        out.extend_from_slice(&encode_id_anyhow(&row.ordering_id.to_string())?);
        out.extend_from_slice(&encode_id_anyhow(&title_representation_id.to_string())?);
        out.extend_from_slice(&encode_id_anyhow(&sentence_representation_id.to_string())?);
        out.extend_from_slice(&encode_u32(0));
    }

    let count = u32::try_from(sorted.len()).map_err(|_| {
        SnapshotError::new(
            "invalid_count",
            "ordering representation index count overflow",
        )
    })?;
    Ok((out, count))
}

fn ordering_profile_code(value: &str) -> Result<u8, SnapshotError> {
    match value {
        "vine" => Ok(0),
        "evidence_rail" => Ok(1),
        "action_rail" => Ok(2),
        _ => Err(SnapshotError::new(
            "invalid_ordering_profile",
            format!("unsupported ordering_profile={value}"),
        )),
    }
}

fn vine_type_code(value: &str) -> Result<u8, SnapshotError> {
    match value {
        "pathway_vine" => Ok(0),
        "narrative_vine" => Ok(1),
        _ => Err(SnapshotError::new(
            "invalid_vine_type",
            format!("unsupported vine_type={value}"),
        )),
    }
}

fn section_hash(section_id: u16, section_bytes: &[u8]) -> Vec<u8> {
    let mut payload = Vec::with_capacity(2 + section_bytes.len());
    payload.extend_from_slice(&encode_u16(section_id));
    payload.extend_from_slice(section_bytes);
    hash_with_domain("snapshot_section", &payload)
}

fn validate_payload_hashes(payloads: &[ReplayPayloadRow]) -> Result<(), SnapshotError> {
    let mut missing = Vec::new();
    let mut mismatched = Vec::new();

    for row in payloads {
        let object_id = row.object_id.to_string();
        let object_kind = match row.object_kind {
            ReplayObjectKind::Idea => "idea",
            ReplayObjectKind::Ordering => "ordering",
        };
        let title = row.title.as_deref().ok_or_else(|| {
            SnapshotError::new(
                "missing_title",
                format!("missing title for {object_kind}:{object_id}"),
            )
        })?;
        let sentence = row.sentence.as_deref().ok_or_else(|| {
            SnapshotError::new(
                "missing_sentence",
                format!("missing sentence for {object_kind}:{object_id}"),
            )
        })?;

        if let Some(expected_title_hash) = row.title_payload_hash.as_deref() {
            let title_hash = to_hex(&hash_bytes(&canonicalize_string_anyhow(title)?));
            if title_hash != expected_title_hash {
                mismatched.push(format!("{}:{}:title", object_kind, row.object_id));
            }
        }
        if let Some(expected_sentence_hash) = row.sentence_payload_hash.as_deref() {
            let sentence_hash = to_hex(&hash_bytes(&canonicalize_string_anyhow(sentence)?));
            if sentence_hash != expected_sentence_hash {
                mismatched.push(format!("{}:{}:sentence", object_kind, row.object_id));
            }
        }

        if row.object_kind == ReplayObjectKind::Idea
            && row.title_payload_hash.is_none()
            && row.sentence_payload_hash.is_none()
        {
            let expected = payload_hash_hex(
                title,
                sentence,
                row.paragraph.as_deref(),
                row.full.as_deref(),
            )
            .map_err(|err| SnapshotError::new("invalid_payload_hash", err))?;

            match row.payload_hash.as_deref() {
                Some(value) if !value.trim().is_empty() => {
                    if value != expected {
                        mismatched.push(format!("idea:{}:combined", row.object_id));
                    }
                }
                _ => {
                    missing.push(format!("idea:{}:combined", row.object_id));
                }
            }
        } else if row.object_kind == ReplayObjectKind::Ordering
            && (row.title_payload_hash.is_none() || row.sentence_payload_hash.is_none())
        {
            if row.title_payload_hash.is_none() {
                missing.push(format!("ordering:{}:title", row.object_id));
            }
            if row.sentence_payload_hash.is_none() {
                missing.push(format!("ordering:{}:sentence", row.object_id));
            }
        }
    }

    if missing.is_empty() && mismatched.is_empty() {
        return Ok(());
    }

    let missing_sample: Vec<String> = missing.iter().take(10).cloned().collect();
    let mismatch_sample: Vec<String> = mismatched.iter().take(10).cloned().collect();

    Err(SnapshotError::new(
        "payload_hash_mismatch",
        format!(
            "payload_hash validation failed: missing_count={} missing_sample=[{}]; mismatch_count={} mismatch_sample=[{}]",
            missing.len(),
            missing_sample.join(", "),
            mismatched.len(),
            mismatch_sample.join(", ")
        ),
    ))
}

fn push_optional_string(out: &mut Vec<u8>, value: Option<&str>) -> Result<(), SnapshotError> {
    match value {
        Some(value) => {
            out.extend_from_slice(&encode_u8(1));
            out.extend_from_slice(&encode_string_canon(value)?);
        }
        None => {
            out.extend_from_slice(&encode_u8(0));
        }
    }
    Ok(())
}

fn encode_string_canon(value: &str) -> Result<Vec<u8>, SnapshotError> {
    let bytes = canonicalize_string_anyhow(value)?;
    let mut out = encode_varint_u64(bytes.len() as u64);
    out.extend_from_slice(&bytes);
    Ok(out)
}

fn canonicalize_string_anyhow(value: &str) -> Result<Vec<u8>, SnapshotError> {
    canonicalize_string(value).map_err(|err| SnapshotError::new("invalid_string", err))
}

fn encode_id_anyhow(value: &str) -> Result<Vec<u8>, SnapshotError> {
    encode_id(value).map_err(|err| SnapshotError::new("invalid_id", err))
}

fn decode_hex_32(value: &str) -> Result<Vec<u8>, SnapshotError> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(SnapshotError::new(
            "invalid_payload_hash",
            "payload_hash must be 64 hexadecimal characters",
        ));
    }
    (0..64)
        .step_by(2)
        .map(|index| {
            u8::from_str_radix(&value[index..index + 2], 16).map_err(|_| {
                SnapshotError::new("invalid_payload_hash", "payload_hash contains invalid hex")
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use replay::{ReplayCycleStatus, ReplayOrderingItemRow, ReplayOrderingRow, ReplayTempoStatus};

    fn v7(id: &str) -> Uuid {
        Uuid::parse_str(id).expect("uuid parse")
    }

    fn fixture() -> ReplayOutput {
        let payload_hash = payload_hash_hex("title", "sentence", None, None).expect("hash");
        let ordering_title = "Evidence Rail title";
        let ordering_sentence = "Evidence Rail sentence";
        let ordering_title_hash = to_hex(&hash_bytes(
            &canonicalize_string_anyhow(ordering_title).expect("title"),
        ));
        let ordering_sentence_hash = to_hex(&hash_bytes(
            &canonicalize_string_anyhow(ordering_sentence).expect("sentence"),
        ));
        ReplayOutput {
            height: 1,
            event_count: 2,
            last_event_id: v7("00000000-0000-7000-8000-000000000102"),
            approximate_timestamp: Utc.timestamp_opt(0, 0).single().unwrap(),
            ideas: vec![ReplayIdeaRow {
                idea_id: v7("00000000-0000-7000-8000-00000000b001"),
                idea_type: "truth_claim".to_string(),
                speaker_identity_id: v7("00000000-0000-7000-8000-00000000a001"),
                created_event_id: v7("00000000-0000-7000-8000-000000000101"),
                created_block_height: 1,
                created_event_index: 0,
            }],
            orderings: vec![ReplayOrderingRow {
                ordering_id: v7("00000000-0000-7000-8000-00000000b010"),
                ordering_profile: "evidence_rail".to_string(),
                vine_type: None,
                subject_idea_id: Some(v7("00000000-0000-7000-8000-00000000b001")),
                speaker_identity_id: v7("00000000-0000-7000-8000-00000000a001"),
                created_event_id: v7("00000000-0000-7000-8000-000000000102"),
                created_block_height: 1,
                created_event_index: 1,
                base_ordering_id: None,
                title_representation_id: Some(v7("00000000-0000-7000-8000-00000000d010")),
                sentence_representation_id: Some(v7("00000000-0000-7000-8000-00000000d011")),
                items: vec![ReplayOrderingItemRow {
                    idx: 0,
                    idea_id: v7("00000000-0000-7000-8000-00000000b001"),
                    item_role: Some("potential_evidence".to_string()),
                    via_connection_id: None,
                }],
            }],
            representations: vec![
                ReplayRepresentationRow {
                    representation_id: v7("00000000-0000-7000-8000-00000000d010"),
                    target_kind: "ordering".to_string(),
                    target_object_id: v7("00000000-0000-7000-8000-00000000b010"),
                    representation_kind: "title".to_string(),
                    tier_length: None,
                    tier_complexity: None,
                    vocabulary_version_id: None,
                    payload_hash: ordering_title_hash.clone(),
                    payload_text: Some(ordering_title.to_string()),
                    author_identity_id: v7("00000000-0000-7000-8000-00000000a001"),
                    language_locale: None,
                    provenance: None,
                    created_event_id: v7("00000000-0000-7000-8000-000000000103"),
                    created_block_height: 1,
                    created_event_index: 2,
                },
                ReplayRepresentationRow {
                    representation_id: v7("00000000-0000-7000-8000-00000000d011"),
                    target_kind: "ordering".to_string(),
                    target_object_id: v7("00000000-0000-7000-8000-00000000b010"),
                    representation_kind: "description".to_string(),
                    tier_length: Some("sentence".to_string()),
                    tier_complexity: Some("standard".to_string()),
                    vocabulary_version_id: None,
                    payload_hash: ordering_sentence_hash.clone(),
                    payload_text: Some(ordering_sentence.to_string()),
                    author_identity_id: v7("00000000-0000-7000-8000-00000000a001"),
                    language_locale: None,
                    provenance: None,
                    created_event_id: v7("00000000-0000-7000-8000-000000000104"),
                    created_block_height: 1,
                    created_event_index: 3,
                },
            ],
            connections: vec![],
            payloads: vec![
                ReplayPayloadRow {
                    object_kind: ReplayObjectKind::Idea,
                    object_id: v7("00000000-0000-7000-8000-00000000b001"),
                    title: Some("title".to_string()),
                    sentence: Some("sentence".to_string()),
                    paragraph: None,
                    full: None,
                    payload_hash: Some(payload_hash),
                    title_payload_hash: None,
                    sentence_payload_hash: None,
                },
                ReplayPayloadRow {
                    object_kind: ReplayObjectKind::Ordering,
                    object_id: v7("00000000-0000-7000-8000-00000000b010"),
                    title: Some(ordering_title.to_string()),
                    sentence: Some(ordering_sentence.to_string()),
                    paragraph: None,
                    full: None,
                    payload_hash: None,
                    title_payload_hash: Some(ordering_title_hash),
                    sentence_payload_hash: Some(ordering_sentence_hash),
                },
            ],
            cycle_status: ReplayCycleStatus {
                cycle_index: 0,
                h_start: 0,
                current_height: 1,
                w_target: 1,
                observed_work: 0,
                cycle_age_ge_dmin: false,
                cycle_age_ge_dmax: false,
                closure_predicate_satisfied: false,
                last_cycle_close_height: None,
            },
            tempo_status: ReplayTempoStatus {
                cycle_age_ge_dmin: false,
                cycle_age_ge_dmax: false,
                constrained_mode: false,
                record_only_mode: false,
            },
        }
    }

    #[test]
    fn encode_is_deterministic() {
        let replay = fixture();
        let first = build_stage0_snapshot(&replay).expect("snapshot");
        let second = build_stage0_snapshot(&replay).expect("snapshot");
        assert_eq!(first.bytes, second.bytes);
        assert_eq!(first.snapshot_hash, second.snapshot_hash);
    }

    #[test]
    fn snapshot_hash_matches_bytes() {
        let replay = fixture();
        let snapshot = build_stage0_snapshot(&replay).expect("snapshot");
        let recomputed = hash_with_domain("snapshot", &snapshot.bytes);
        assert_eq!(snapshot.snapshot_hash, recomputed);
        let sha1 = sha256_hex(&snapshot.bytes);
        let sha2 = sha256_hex(&snapshot.bytes);
        assert_eq!(sha1, sha2);
    }

    #[test]
    fn ordering_representation_index_is_excluded_from_state_root() {
        let first = build_stage0_snapshot(&fixture()).expect("first snapshot");
        let mut changed = fixture();
        changed.orderings[0].title_representation_id =
            Some(v7("00000000-0000-7000-8000-00000000d012"));
        let second = build_stage0_snapshot(&changed).expect("second snapshot");
        assert_eq!(
            first.commitments.state_root_hash,
            second.commitments.state_root_hash
        );
        assert_ne!(first.snapshot_hash, second.snapshot_hash);
    }

    #[test]
    fn representation_author_is_committed_to_state_root() {
        let first = build_stage0_snapshot(&fixture()).expect("first snapshot");
        let mut changed = fixture();
        changed.representations[0].author_identity_id = v7("00000000-0000-7000-8000-00000000a002");
        let second = build_stage0_snapshot(&changed).expect("second snapshot");
        assert_ne!(
            first.commitments.state_root_hash,
            second.commitments.state_root_hash
        );
    }

    #[test]
    fn title_representation_is_committed_as_a_distinct_snapshot_record() {
        let replay = fixture();
        let title = &replay.representations[0];
        assert_eq!(title.representation_kind, "title");
        assert_eq!(title.tier_length, None);
        assert_eq!(title.tier_complexity, None);
        assert_eq!(title.vocabulary_version_id, None);

        let with_title = build_stage0_snapshot(&replay).expect("snapshot with title");
        let with_title_section = with_title
            .sections
            .iter()
            .find(|section| section.id == REPRESENTATIONS_SECTION_ID)
            .expect("representations section");
        assert_eq!(with_title_section.item_count, 2);

        let mut changed_title = replay;
        let changed_title_text = "Changed Evidence Rail title";
        let changed_title_hash = to_hex(&hash_bytes(
            &canonicalize_string_anyhow(changed_title_text).expect("changed title"),
        ));
        changed_title.representations[0].payload_text = Some(changed_title_text.to_string());
        changed_title.representations[0].payload_hash = changed_title_hash.clone();
        changed_title.payloads[1].title = Some(changed_title_text.to_string());
        changed_title.payloads[1].title_payload_hash = Some(changed_title_hash);
        let changed_title =
            build_stage0_snapshot(&changed_title).expect("snapshot with changed title record");
        let changed_title_section = changed_title
            .sections
            .iter()
            .find(|section| section.id == REPRESENTATIONS_SECTION_ID)
            .expect("representations section");
        assert_eq!(changed_title_section.item_count, 2);
        assert_ne!(with_title_section.hash, changed_title_section.hash);
        assert_ne!(
            with_title.commitments.state_root_hash,
            changed_title.commitments.state_root_hash
        );
    }

    #[test]
    fn snapshot_rejects_description_metadata_on_title() {
        let mut replay = fixture();
        replay.representations[0].tier_length = Some("sentence".to_string());
        let error = build_stage0_snapshot(&replay).expect_err("invalid title slot");
        assert_eq!(error.code, "invalid_representation_slot");
    }

    #[test]
    fn snapshot_rejects_vine_metadata_on_standardized_profile() {
        let mut replay = fixture();
        replay.orderings[0].vine_type = Some("narrative_vine".to_string());
        let error = build_stage0_snapshot(&replay).expect_err("invalid profile metadata");
        assert_eq!(error.code, "invalid_vine_type");
    }
}
