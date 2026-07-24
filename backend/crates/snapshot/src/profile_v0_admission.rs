//! Deterministic, database-free Profile-v0 identity-admission snapshot packing.
//!
//! This module intentionally commits only to the pure replay projection. The
//! Stage-0 database-backed snapshot pack remains unchanged until its migration
//! and publication workflow can consume the Profile-v0 projection.

use crate::format::SnapshotSection;
use encoding::canonical::{
    encode_id, encode_string, encode_u16, encode_u32, encode_u64, encode_u8,
};
use encoding::hash::hash_with_domain;
use replay::{
    ProfileV0AdmissionReplayState, ProfileV0DerivationStatus, ProfileV0DirectKeyState,
    ProfileV0EligibilityLane, ProfileV0IdentityKind, ProfileV0ProvenanceClass,
};
use uuid::Uuid;

pub const PROFILE_V0_IDENTITIES_SECTION_ID: u16 = 0x0001;
pub const PROFILE_V0_DIRECT_KEYS_SECTION_ID: u16 = 0x0011;
pub const PROFILE_V0_LINEAGE_SECTION_ID: u16 = 0x0012;
pub const PROFILE_V0_DERIVATION_SECTION_ID: u16 = 0x80D1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0AdmissionSnapshot {
    pub bytes: Vec<u8>,
    pub snapshot_hash: Vec<u8>,
    pub state_root_hash: Vec<u8>,
    pub sections: Vec<SnapshotSection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0AdmissionSnapshotError {
    pub code: &'static str,
    pub message: String,
}

impl ProfileV0AdmissionSnapshotError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ProfileV0AdmissionSnapshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ProfileV0AdmissionSnapshotError {}

pub fn build_profile_v0_admission_snapshot(
    state: &ProfileV0AdmissionReplayState,
) -> Result<ProfileV0AdmissionSnapshot, ProfileV0AdmissionSnapshotError> {
    let sections = build_sections(state)?;
    let state_root_hash = state_root_hash(&sections);
    let bytes = encode_snapshot(&sections, &state_root_hash)?;
    let snapshot_hash = hash_with_domain("snapshot_profile_v0_admission", &bytes);

    Ok(ProfileV0AdmissionSnapshot {
        bytes,
        snapshot_hash,
        state_root_hash,
        sections,
    })
}

pub fn verify_profile_v0_admission_snapshot(
    snapshot: &ProfileV0AdmissionSnapshot,
    state: &ProfileV0AdmissionReplayState,
) -> Result<(), ProfileV0AdmissionSnapshotError> {
    let expected = build_profile_v0_admission_snapshot(state)?;
    if snapshot.state_root_hash != expected.state_root_hash
        || snapshot.snapshot_hash != expected.snapshot_hash
        || snapshot.bytes != expected.bytes
        || snapshot.sections != expected.sections
    {
        return Err(ProfileV0AdmissionSnapshotError::new(
            "snapshot_projection_mismatch",
            "snapshot does not commit to the supplied Profile-v0 admission replay state",
        ));
    }
    Ok(())
}

fn build_sections(
    state: &ProfileV0AdmissionReplayState,
) -> Result<Vec<SnapshotSection>, ProfileV0AdmissionSnapshotError> {
    let identity_bytes = encode_identities(state)?;
    let key_bytes = encode_direct_keys(state)?;
    let lineage_bytes = encode_lineage(state)?;
    let derivation_bytes = encode_derivation(state)?;

    Ok(vec![
        make_section(
            PROFILE_V0_IDENTITIES_SECTION_ID,
            u32::try_from(state.identities.len()).map_err(|_| {
                ProfileV0AdmissionSnapshotError::new("invalid_item_count", "too many identities")
            })?,
            identity_bytes,
        ),
        make_section(
            PROFILE_V0_DIRECT_KEYS_SECTION_ID,
            u32::try_from(
                state
                    .identities
                    .values()
                    .map(|identity| identity.direct_keys.len())
                    .sum::<usize>(),
            )
            .map_err(|_| {
                ProfileV0AdmissionSnapshotError::new("invalid_item_count", "too many keys")
            })?,
            key_bytes,
        ),
        make_section(
            PROFILE_V0_LINEAGE_SECTION_ID,
            u32::try_from(state.identities.len()).map_err(|_| {
                ProfileV0AdmissionSnapshotError::new("invalid_item_count", "too many identities")
            })?,
            lineage_bytes,
        ),
        make_section(
            PROFILE_V0_DERIVATION_SECTION_ID,
            u32::try_from(state.identities.len()).map_err(|_| {
                ProfileV0AdmissionSnapshotError::new("invalid_item_count", "too many identities")
            })?,
            derivation_bytes,
        ),
    ])
}

fn make_section(id: u16, item_count: u32, bytes: Vec<u8>) -> SnapshotSection {
    let mut payload = Vec::new();
    payload.extend_from_slice(&encode_u16(id));
    payload.extend_from_slice(&encode_u32(item_count));
    payload.extend_from_slice(&bytes);
    SnapshotSection {
        id,
        item_count,
        hash: hash_with_domain("snapshot_section", &payload),
        bytes,
    }
}

fn encode_identities(
    state: &ProfileV0AdmissionReplayState,
) -> Result<Vec<u8>, ProfileV0AdmissionSnapshotError> {
    let mut out = Vec::new();
    for identity in state.identities.values() {
        if identity.provenance_class == ProfileV0ProvenanceClass::EventDerived
            && identity.structural_roots.is_none()
        {
            return Err(ProfileV0AdmissionSnapshotError::new(
                "incomplete_identity_structural_roots",
                "event-derived Profile-v0 identity lacks a complete structural-root set",
            ));
        }
        out.extend_from_slice(&encode_uuid(identity.identity_id)?);
        out.extend_from_slice(&encode_u8(identity_kind_tag(identity.identity_kind)));
        out.extend_from_slice(&encode_u8(provenance_tag(identity.provenance_class)));
        encode_optional_uuid(&mut out, identity.admission_event_id)?;
        encode_optional_string(&mut out, identity.admission_profile_version.as_deref());
        encode_optional_uuid(&mut out, identity.sponsor_identity_id)?;
        match &identity.structural_roots {
            Some(roots) => {
                out.extend_from_slice(&encode_u8(1));
                let mut ordered_roots = roots.roots.to_vec();
                ordered_roots.sort_by_key(|(role, _)| *role as u8);
                out.extend_from_slice(&encode_u8(
                    u8::try_from(ordered_roots.len()).expect("four root roles fit in u8"),
                ));
                for (role, root_id) in ordered_roots {
                    out.extend_from_slice(&encode_u8(role as u8));
                    out.extend_from_slice(&encode_uuid(root_id)?);
                }
                for membership_id in roots.membership_connection_ids {
                    out.extend_from_slice(&encode_uuid(membership_id)?);
                }
            }
            None => out.extend_from_slice(&encode_u8(0)),
        }
    }
    Ok(out)
}

fn encode_direct_keys(
    state: &ProfileV0AdmissionReplayState,
) -> Result<Vec<u8>, ProfileV0AdmissionSnapshotError> {
    let mut out = Vec::new();
    for identity in state.identities.values() {
        for key in identity.direct_keys.values() {
            out.extend_from_slice(&encode_uuid(identity.identity_id)?);
            out.extend_from_slice(&key.public_key_ref);
            out.extend_from_slice(&encode_string(&key.descriptor.key_profile_version));
            out.extend_from_slice(&encode_string(&key.descriptor.signature_algorithm));
            out.extend_from_slice(&encode_u32(
                u32::try_from(key.descriptor.raw_public_key_bytes.len()).map_err(|_| {
                    ProfileV0AdmissionSnapshotError::new(
                        "malformed_initial_key_descriptor",
                        "direct key descriptor is too large for snapshot encoding",
                    )
                })?,
            ));
            out.extend_from_slice(&key.descriptor.raw_public_key_bytes);
            out.extend_from_slice(&encode_u8(direct_key_state_tag(key.state)));
            encode_optional_uuid(&mut out, key.registration_event_id)?;
            encode_optional_uuid(&mut out, key.supersession_event_id)?;
            encode_optional_uuid(&mut out, key.revocation_event_id)?;
        }
    }
    Ok(out)
}

fn encode_lineage(
    state: &ProfileV0AdmissionReplayState,
) -> Result<Vec<u8>, ProfileV0AdmissionSnapshotError> {
    let mut out = Vec::new();
    for identity in state.identities.values() {
        out.extend_from_slice(&encode_uuid(identity.identity_id)?);
        encode_optional_uuid(&mut out, identity.sponsor_identity_id)?;
        encode_optional_uuid(&mut out, identity.admission_event_id)?;
        out.extend_from_slice(&encode_u8(provenance_tag(identity.provenance_class)));
    }
    Ok(out)
}

fn encode_derivation(
    state: &ProfileV0AdmissionReplayState,
) -> Result<Vec<u8>, ProfileV0AdmissionSnapshotError> {
    let mut out = Vec::new();
    for identity in state.identities.values() {
        out.extend_from_slice(&encode_uuid(identity.identity_id)?);
        for lane in [
            identity.lanes.restricted_verification,
            identity.lanes.ordinary_writer,
            identity.lanes.ordinary_challenge,
            identity.lanes.voter,
            identity.lanes.governance,
            identity.lanes.tempo,
            identity.lanes.inviter,
        ] {
            out.extend_from_slice(&encode_u8(eligibility_lane_tag(lane)));
        }
    }

    let mut debits = state.invitation_capacity_debits.clone();
    debits.sort_by_key(|debit| {
        (
            debit.position.block_height,
            debit.position.event_index,
            debit.admission_event_id,
        )
    });
    out.extend_from_slice(&encode_u32(u32::try_from(debits.len()).map_err(|_| {
        ProfileV0AdmissionSnapshotError::new("invalid_item_count", "too many capacity debits")
    })?));
    for debit in debits {
        out.extend_from_slice(&encode_uuid(debit.admission_event_id)?);
        out.extend_from_slice(&encode_uuid(debit.sponsor_identity_id)?);
        out.extend_from_slice(&encode_uuid(debit.admitted_identity_id)?);
        out.extend_from_slice(&encode_uuid(debit.capacity_period_id)?);
        out.extend_from_slice(&encode_uuid(debit.rulebook_id)?);
        out.extend_from_slice(&encode_string(&debit.rulebook_version));
        out.extend_from_slice(&debit.rulebook_hash);
        out.extend_from_slice(&debit.debit_units.to_be_bytes());
        out.extend_from_slice(&debit.position.block_height.to_be_bytes());
        out.extend_from_slice(&debit.position.event_index.to_be_bytes());
    }

    out.extend_from_slice(&encode_u32(
        u32::try_from(state.compatibility_verification_records.len()).map_err(|_| {
            ProfileV0AdmissionSnapshotError::new(
                "invalid_item_count",
                "too many compatibility verification records",
            )
        })?,
    ));
    let mut compatibility_records = state.compatibility_verification_records.clone();
    compatibility_records.sort_by_key(|record| {
        (
            record.position.block_height,
            record.position.event_index,
            record.event_id,
        )
    });
    for record in compatibility_records {
        out.extend_from_slice(&encode_uuid(record.event_id)?);
        out.extend_from_slice(&encode_uuid(record.identity_id)?);
        out.extend_from_slice(&encode_string(&record.manifest_version));
        out.extend_from_slice(&encode_u8(provenance_tag(record.provenance_class)));
        out.extend_from_slice(&record.position.block_height.to_be_bytes());
        out.extend_from_slice(&record.position.event_index.to_be_bytes());
    }

    for status in [
        state.invitation_capacity_balance,
        state.invitation_suspension,
        state.maturation,
        state.admission_liveness_blocked,
    ] {
        out.extend_from_slice(&encode_u8(derivation_status_tag(status)));
    }
    Ok(out)
}

fn state_root_hash(sections: &[SnapshotSection]) -> Vec<u8> {
    let mut payload = Vec::new();
    for section in sections {
        payload.extend_from_slice(&encode_u16(section.id));
        payload.extend_from_slice(&section.hash);
    }
    hash_with_domain("snapshot_profile_v0_admission_state_root", &payload)
}

fn encode_snapshot(
    sections: &[SnapshotSection],
    state_root_hash: &[u8],
) -> Result<Vec<u8>, ProfileV0AdmissionSnapshotError> {
    let mut header = Vec::new();
    header.extend_from_slice(state_root_hash);
    header.extend_from_slice(&encode_u16(u16::try_from(sections.len()).map_err(
        |_| {
            ProfileV0AdmissionSnapshotError::new(
                "invalid_section_count",
                "too many snapshot sections",
            )
        },
    )?));
    for section in sections {
        header.extend_from_slice(&encode_u16(section.id));
        header.extend_from_slice(&encode_u32(section.item_count));
        header.extend_from_slice(&encode_u64(u64::try_from(section.bytes.len()).map_err(
            |_| ProfileV0AdmissionSnapshotError::new("invalid_section_length", "section too large"),
        )?));
        header.extend_from_slice(&section.hash);
    }

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"MCCADMP0");
    bytes.extend_from_slice(&encode_u16(0));
    bytes.extend_from_slice(&encode_u16(0));
    bytes.extend_from_slice(&encode_u32(u32::try_from(header.len()).map_err(|_| {
        ProfileV0AdmissionSnapshotError::new("invalid_header", "snapshot header too large")
    })?));
    bytes.extend_from_slice(&header);
    for section in sections {
        bytes.extend_from_slice(&section.bytes);
    }
    Ok(bytes)
}

fn encode_uuid(value: Uuid) -> Result<Vec<u8>, ProfileV0AdmissionSnapshotError> {
    encode_id(&value.to_string())
        .map_err(|error| ProfileV0AdmissionSnapshotError::new("invalid_id", error))
}

fn encode_optional_uuid(
    out: &mut Vec<u8>,
    value: Option<Uuid>,
) -> Result<(), ProfileV0AdmissionSnapshotError> {
    match value {
        Some(value) => {
            out.extend_from_slice(&encode_u8(1));
            out.extend_from_slice(&encode_uuid(value)?);
        }
        None => out.extend_from_slice(&encode_u8(0)),
    }
    Ok(())
}

fn encode_optional_string(out: &mut Vec<u8>, value: Option<&str>) {
    match value {
        Some(value) => {
            out.extend_from_slice(&encode_u8(1));
            out.extend_from_slice(&encode_string(value));
        }
        None => out.extend_from_slice(&encode_u8(0)),
    }
}

fn identity_kind_tag(kind: ProfileV0IdentityKind) -> u8 {
    match kind {
        ProfileV0IdentityKind::Human => 0x01,
    }
}

fn provenance_tag(provenance: ProfileV0ProvenanceClass) -> u8 {
    match provenance {
        ProfileV0ProvenanceClass::GenesisAdmitted => 0x01,
        ProfileV0ProvenanceClass::LegacyOperatorProvisioned => 0x02,
        ProfileV0ProvenanceClass::EventDerived => 0x03,
        ProfileV0ProvenanceClass::FutureProfileDerived => 0x04,
    }
}

fn direct_key_state_tag(state: ProfileV0DirectKeyState) -> u8 {
    match state {
        ProfileV0DirectKeyState::Active => 0x01,
        ProfileV0DirectKeyState::Superseded => 0x02,
        ProfileV0DirectKeyState::Revoked => 0x03,
        ProfileV0DirectKeyState::Invalid => 0x04,
        ProfileV0DirectKeyState::CompatibilityOnly => 0x05,
    }
}

fn eligibility_lane_tag(lane: ProfileV0EligibilityLane) -> u8 {
    match lane {
        ProfileV0EligibilityLane::Granted => 0x01,
        ProfileV0EligibilityLane::NotGranted => 0x02,
        ProfileV0EligibilityLane::NotYetDerived => 0x03,
    }
}

fn derivation_status_tag(status: ProfileV0DerivationStatus) -> u8 {
    match status {
        ProfileV0DerivationStatus::NotYetDerived => 0x00,
        ProfileV0DerivationStatus::Derived(false) => 0x01,
        ProfileV0DerivationStatus::Derived(true) => 0x02,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use replay::{
        ProfileV0InvitationCapacityDebit, ProfileV0ReplayDirectKey,
        ProfileV0ReplayEligibilityLanes, ProfileV0ReplayIdentity, ProfileV0ReplayPosition,
        ProfileV0StructuralRoots,
    };
    use std::collections::BTreeMap;
    use verification::admission::IdentityStructuralRootRoleV0;
    use verification::signatures::KeyDescriptorV0;

    fn id(value: &str) -> Uuid {
        Uuid::parse_str(value).unwrap()
    }

    fn sample_state() -> ProfileV0AdmissionReplayState {
        let identity_id = id("00000000-0000-7000-8000-000000000201");
        let sponsor_id = id("00000000-0000-7000-8000-000000000202");
        let admission_event_id = id("00000000-0000-7000-8000-000000000203");
        let key_ref = [0x11; 32];
        let key = ProfileV0ReplayDirectKey {
            public_key_ref: key_ref,
            descriptor: KeyDescriptorV0 {
                key_profile_version: "ed25519_v0".to_string(),
                signature_algorithm: "ed25519".to_string(),
                raw_public_key_bytes: vec![0x22; 32],
                owning_identity_id: identity_id,
            },
            state: ProfileV0DirectKeyState::Active,
            registration_event_id: Some(admission_event_id),
            supersession_event_id: None,
            revocation_event_id: None,
        };
        let roots = ProfileV0StructuralRoots {
            roots: [
                (
                    IdentityStructuralRootRoleV0::Mindgarden,
                    id("00000000-0000-7000-8000-000000000204"),
                ),
                (
                    IdentityStructuralRootRoleV0::BackyardOfRelationships,
                    id("00000000-0000-7000-8000-000000000205"),
                ),
                (
                    IdentityStructuralRootRoleV0::SelfTree,
                    id("00000000-0000-7000-8000-000000000206"),
                ),
                (
                    IdentityStructuralRootRoleV0::Anthill,
                    id("00000000-0000-7000-8000-000000000207"),
                ),
            ],
            membership_connection_ids: [
                id("00000000-0000-7000-8000-000000000208"),
                id("00000000-0000-7000-8000-000000000209"),
                id("00000000-0000-7000-8000-000000000210"),
            ],
        };
        let identity = ProfileV0ReplayIdentity {
            identity_id,
            identity_kind: ProfileV0IdentityKind::Human,
            provenance_class: ProfileV0ProvenanceClass::EventDerived,
            admission_event_id: Some(admission_event_id),
            admission_profile_version: Some("sponsored_public_admission_v0".to_string()),
            sponsor_identity_id: Some(sponsor_id),
            structural_roots: Some(roots),
            direct_keys: BTreeMap::from([(key_ref, key)]),
            active_direct_key_ref: Some(key_ref),
            lanes: ProfileV0ReplayEligibilityLanes {
                restricted_verification: ProfileV0EligibilityLane::Granted,
                ordinary_writer: ProfileV0EligibilityLane::NotGranted,
                ordinary_challenge: ProfileV0EligibilityLane::NotGranted,
                voter: ProfileV0EligibilityLane::NotGranted,
                governance: ProfileV0EligibilityLane::NotGranted,
                tempo: ProfileV0EligibilityLane::NotGranted,
                inviter: ProfileV0EligibilityLane::NotGranted,
            },
        };
        ProfileV0AdmissionReplayState {
            identities: BTreeMap::from([(identity_id, identity)]),
            invitation_capacity_debits: vec![ProfileV0InvitationCapacityDebit {
                admission_event_id,
                sponsor_identity_id: sponsor_id,
                admitted_identity_id: identity_id,
                capacity_period_id: id("00000000-0000-7000-8000-000000000211"),
                rulebook_id: id("00000000-0000-7000-8000-000000000212"),
                rulebook_version: "v0".to_string(),
                rulebook_hash: [0x33; 32],
                debit_units: 1,
                position: ProfileV0ReplayPosition {
                    block_height: 7,
                    event_index: 1,
                },
            }],
            compatibility_verification_records: Vec::new(),
            invitation_capacity_balance: ProfileV0DerivationStatus::NotYetDerived,
            invitation_suspension: ProfileV0DerivationStatus::NotYetDerived,
            maturation: ProfileV0DerivationStatus::NotYetDerived,
            admission_liveness_blocked: ProfileV0DerivationStatus::NotYetDerived,
        }
    }

    #[test]
    fn profile_v0_admission_snapshot_is_deterministic_and_verifiable() {
        let state = sample_state();
        let first = build_profile_v0_admission_snapshot(&state).unwrap();
        let second = build_profile_v0_admission_snapshot(&state).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.sections.len(), 4);
        verify_profile_v0_admission_snapshot(&first, &state).unwrap();
    }

    #[test]
    fn changed_admission_projection_does_not_verify() {
        let state = sample_state();
        let snapshot = build_profile_v0_admission_snapshot(&state).unwrap();

        let mut changed_capacity = state.clone();
        changed_capacity.invitation_capacity_debits[0].debit_units = 2;

        let mut changed_lineage = state.clone();
        changed_lineage
            .identities
            .values_mut()
            .next()
            .unwrap()
            .sponsor_identity_id = Some(id("00000000-0000-7000-8000-000000000213"));

        let mut changed_root = state.clone();
        changed_root
            .identities
            .values_mut()
            .next()
            .unwrap()
            .structural_roots
            .as_mut()
            .unwrap()
            .roots[0]
            .1 = id("00000000-0000-7000-8000-000000000214");

        let mut changed_key = state.clone();
        changed_key
            .identities
            .values_mut()
            .next()
            .unwrap()
            .direct_keys
            .values_mut()
            .next()
            .unwrap()
            .state = ProfileV0DirectKeyState::Superseded;

        let mut changed_liveness = state.clone();
        changed_liveness.admission_liveness_blocked = ProfileV0DerivationStatus::Derived(true);

        for changed in [
            changed_capacity,
            changed_lineage,
            changed_root,
            changed_key,
            changed_liveness,
        ] {
            let error = verify_profile_v0_admission_snapshot(&snapshot, &changed).unwrap_err();
            assert_eq!(error.code, "snapshot_projection_mismatch");
        }
    }

    #[test]
    fn event_derived_identity_requires_complete_roots() {
        let mut state = sample_state();
        state
            .identities
            .values_mut()
            .next()
            .unwrap()
            .structural_roots = None;
        let error = build_profile_v0_admission_snapshot(&state).unwrap_err();
        assert_eq!(error.code, "incomplete_identity_structural_roots");
    }
}
