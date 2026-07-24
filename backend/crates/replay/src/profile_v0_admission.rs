use encoding::hash::hash_with_domain;
use event_log::profile_v0_admission::{
    validate_profile_v0_identity_create_candidate,
    validate_profile_v0_identity_key_revoke_candidate,
    validate_profile_v0_identity_key_rotate_candidate, DirectKeyHistoryEntryV0, DirectKeyStateV0,
    ProfileV0AdmissionPureState, ProfileV0AdmissionValidationError, ProfileV0DirectKeyPureState,
    ProfileV0IdentityCreatePayload,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;
use verification::admission::{
    public_key_ref_from_descriptor_v0, IdentityStructuralRootPlanV0, IdentityStructuralRootRoleV0,
};
use verification::signatures::{decode_hash32, AuthoredEventCandidate, KeyDescriptorV0};

const PROFILE_V0_ADMISSION_PROFILE: &str = "sponsored_public_admission_v0";
const CAPACITY_DEBIT_UNITS: i64 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProfileV0ReplayPosition {
    pub block_height: i64,
    pub event_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileV0IdentityKind {
    Human,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileV0ProvenanceClass {
    GenesisAdmitted,
    LegacyOperatorProvisioned,
    EventDerived,
    FutureProfileDerived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileV0DirectKeyState {
    Active,
    Superseded,
    Revoked,
    Invalid,
    CompatibilityOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileV0EligibilityLane {
    Granted,
    NotGranted,
    NotYetDerived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileV0DerivationStatus {
    NotYetDerived,
    Derived(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0ReplayDirectKey {
    pub public_key_ref: [u8; 32],
    pub descriptor: KeyDescriptorV0,
    pub state: ProfileV0DirectKeyState,
    pub registration_event_id: Option<Uuid>,
    pub supersession_event_id: Option<Uuid>,
    pub revocation_event_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0StructuralRoots {
    pub roots: [(IdentityStructuralRootRoleV0, Uuid); 4],
    pub membership_connection_ids: [Uuid; 3],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0ReplayEligibilityLanes {
    pub restricted_verification: ProfileV0EligibilityLane,
    pub ordinary_writer: ProfileV0EligibilityLane,
    pub ordinary_challenge: ProfileV0EligibilityLane,
    pub voter: ProfileV0EligibilityLane,
    pub governance: ProfileV0EligibilityLane,
    pub tempo: ProfileV0EligibilityLane,
    pub inviter: ProfileV0EligibilityLane,
}

impl ProfileV0ReplayEligibilityLanes {
    fn admitted_initial() -> Self {
        Self {
            restricted_verification: ProfileV0EligibilityLane::Granted,
            ordinary_writer: ProfileV0EligibilityLane::NotGranted,
            ordinary_challenge: ProfileV0EligibilityLane::NotGranted,
            voter: ProfileV0EligibilityLane::NotGranted,
            governance: ProfileV0EligibilityLane::NotGranted,
            tempo: ProfileV0EligibilityLane::NotGranted,
            inviter: ProfileV0EligibilityLane::NotGranted,
        }
    }

    fn compatibility_only() -> Self {
        Self {
            restricted_verification: ProfileV0EligibilityLane::NotYetDerived,
            ordinary_writer: ProfileV0EligibilityLane::NotGranted,
            ordinary_challenge: ProfileV0EligibilityLane::NotGranted,
            voter: ProfileV0EligibilityLane::NotGranted,
            governance: ProfileV0EligibilityLane::NotGranted,
            tempo: ProfileV0EligibilityLane::NotGranted,
            inviter: ProfileV0EligibilityLane::NotGranted,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0ReplayIdentity {
    pub identity_id: Uuid,
    pub identity_kind: ProfileV0IdentityKind,
    pub provenance_class: ProfileV0ProvenanceClass,
    pub admission_event_id: Option<Uuid>,
    pub admission_profile_version: Option<String>,
    pub sponsor_identity_id: Option<Uuid>,
    pub structural_roots: Option<ProfileV0StructuralRoots>,
    pub direct_keys: BTreeMap<[u8; 32], ProfileV0ReplayDirectKey>,
    pub active_direct_key_ref: Option<[u8; 32]>,
    pub lanes: ProfileV0ReplayEligibilityLanes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0InvitationCapacityDebit {
    pub admission_event_id: Uuid,
    pub sponsor_identity_id: Uuid,
    pub admitted_identity_id: Uuid,
    pub capacity_period_id: Uuid,
    pub rulebook_id: Uuid,
    pub rulebook_version: String,
    pub rulebook_hash: [u8; 32],
    pub debit_units: i64,
    pub position: ProfileV0ReplayPosition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0CompatibilityVerificationRecord {
    pub event_id: Uuid,
    pub identity_id: Uuid,
    pub manifest_version: String,
    pub provenance_class: ProfileV0ProvenanceClass,
    pub position: ProfileV0ReplayPosition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0ReplaySeedIdentity {
    pub identity_id: Uuid,
    pub provenance_class: ProfileV0ProvenanceClass,
    pub direct_key: Option<ProfileV0ReplayDirectKey>,
    pub structural_roots: Option<ProfileV0StructuralRoots>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0CompatibilityManifest {
    pub manifest_version: String,
    pub provenance_class: ProfileV0ProvenanceClass,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProfileV0ReplayEvent {
    Candidate {
        position: ProfileV0ReplayPosition,
        candidate: AuthoredEventCandidate,
        payload: Value,
    },
    CompatibilityIdentityVerificationUpdate {
        position: ProfileV0ReplayPosition,
        event_id: Uuid,
        identity_id: Uuid,
        manifest: ProfileV0CompatibilityManifest,
    },
}

impl ProfileV0ReplayEvent {
    pub fn event_id(&self) -> Uuid {
        match self {
            Self::Candidate { candidate, .. } => candidate.event_id,
            Self::CompatibilityIdentityVerificationUpdate { event_id, .. } => *event_id,
        }
    }

    pub fn position(&self) -> ProfileV0ReplayPosition {
        match self {
            Self::Candidate { position, .. }
            | Self::CompatibilityIdentityVerificationUpdate { position, .. } => *position,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0AdmissionReplayState {
    pub identities: BTreeMap<Uuid, ProfileV0ReplayIdentity>,
    pub invitation_capacity_debits: Vec<ProfileV0InvitationCapacityDebit>,
    pub compatibility_verification_records: Vec<ProfileV0CompatibilityVerificationRecord>,
    pub invitation_capacity_balance: ProfileV0DerivationStatus,
    pub invitation_suspension: ProfileV0DerivationStatus,
    pub maturation: ProfileV0DerivationStatus,
    pub admission_liveness_blocked: ProfileV0DerivationStatus,
}

impl Default for ProfileV0AdmissionReplayState {
    fn default() -> Self {
        Self {
            identities: BTreeMap::new(),
            invitation_capacity_debits: Vec::new(),
            compatibility_verification_records: Vec::new(),
            invitation_capacity_balance: ProfileV0DerivationStatus::NotYetDerived,
            invitation_suspension: ProfileV0DerivationStatus::NotYetDerived,
            maturation: ProfileV0DerivationStatus::NotYetDerived,
            admission_liveness_blocked: ProfileV0DerivationStatus::NotYetDerived,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0ReplayError {
    pub code: &'static str,
    pub message: String,
}

impl ProfileV0ReplayError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    fn from_validation(error: ProfileV0AdmissionValidationError) -> Self {
        Self::new(error.code, error.message)
    }
}

impl std::fmt::Display for ProfileV0ReplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ProfileV0ReplayError {}

#[derive(Debug, Clone)]
pub struct ProfileV0AdmissionReplay {
    state: ProfileV0AdmissionReplayState,
    applied_events: BTreeMap<Uuid, ProfileV0ReplayEvent>,
    event_positions: BTreeMap<ProfileV0ReplayPosition, Uuid>,
}

impl ProfileV0AdmissionReplay {
    pub fn new(seeds: &[ProfileV0ReplaySeedIdentity]) -> Result<Self, ProfileV0ReplayError> {
        let mut state = ProfileV0AdmissionReplayState::default();
        let mut occupied_root_ids = BTreeSet::new();
        let mut occupied_connection_ids = BTreeSet::new();
        let mut seen_keys = BTreeSet::new();
        let mut seen_raw_keys = BTreeSet::new();

        for seed in seeds {
            if state.identities.contains_key(&seed.identity_id) {
                return Err(ProfileV0ReplayError::new(
                    "identity_already_exists",
                    "compatibility seed contains a duplicate identity_id",
                ));
            }
            if let Some(roots) = &seed.structural_roots {
                validate_structural_roots(roots)?;
                for (_, root_id) in roots.roots {
                    if !occupied_root_ids.insert(root_id) {
                        return Err(ProfileV0ReplayError::new(
                            "structural_root_collision",
                            "compatibility seed reuses a structural-root ID",
                        ));
                    }
                }
                for connection_id in roots.membership_connection_ids {
                    if !occupied_connection_ids.insert(connection_id) {
                        return Err(ProfileV0ReplayError::new(
                            "structural_root_collision",
                            "compatibility seed reuses a root membership ID",
                        ));
                    }
                }
            }

            let mut direct_keys = BTreeMap::new();
            let active_direct_key_ref = if let Some(key) = &seed.direct_key {
                if key.descriptor.owning_identity_id != seed.identity_id {
                    return Err(ProfileV0ReplayError::new(
                        "malformed_initial_key_descriptor",
                        "compatibility seed key owner must match identity_id",
                    ));
                }
                let expected =
                    public_key_ref_from_descriptor_v0(&key.descriptor).map_err(|err| {
                        ProfileV0ReplayError::new("malformed_initial_key_descriptor", err.message)
                    })?;
                if expected != key.public_key_ref
                    || !seen_keys.insert(key.public_key_ref)
                    || !seen_raw_keys.insert(key.descriptor.raw_public_key_bytes.clone())
                {
                    return Err(ProfileV0ReplayError::new(
                        "public_key_already_registered",
                        "compatibility seed reuses a direct key",
                    ));
                }
                let active = matches!(key.state, ProfileV0DirectKeyState::Active);
                direct_keys.insert(key.public_key_ref, key.clone());
                active.then_some(key.public_key_ref)
            } else {
                None
            };

            state.identities.insert(
                seed.identity_id,
                ProfileV0ReplayIdentity {
                    identity_id: seed.identity_id,
                    identity_kind: ProfileV0IdentityKind::Human,
                    provenance_class: seed.provenance_class,
                    admission_event_id: None,
                    admission_profile_version: None,
                    sponsor_identity_id: None,
                    structural_roots: seed.structural_roots.clone(),
                    direct_keys,
                    active_direct_key_ref,
                    lanes: ProfileV0ReplayEligibilityLanes::compatibility_only(),
                },
            );
        }

        Ok(Self {
            state,
            applied_events: BTreeMap::new(),
            event_positions: BTreeMap::new(),
        })
    }

    pub fn state(&self) -> &ProfileV0AdmissionReplayState {
        &self.state
    }

    pub fn apply(&mut self, event: ProfileV0ReplayEvent) -> Result<(), ProfileV0ReplayError> {
        let event_id = event.event_id();
        if let Some(existing) = self.applied_events.get(&event_id) {
            if existing == &event {
                return Ok(());
            }
            return Err(ProfileV0ReplayError::new(
                "conflicting_duplicate_event",
                "canonical event_id is already applied with different bytes or classification",
            ));
        }
        if let Some(existing) = self.event_positions.get(&event.position()) {
            return Err(ProfileV0ReplayError::new(
                "conflicting_event_position",
                format!("canonical position is already occupied by event_id={existing}"),
            ));
        }

        let next = self.apply_unrecorded(&event)?;
        self.state = next;
        self.event_positions.insert(event.position(), event_id);
        self.applied_events.insert(event_id, event);
        Ok(())
    }

    pub fn replay(
        seeds: &[ProfileV0ReplaySeedIdentity],
        events: &[ProfileV0ReplayEvent],
    ) -> Result<ProfileV0AdmissionReplayState, ProfileV0ReplayError> {
        let mut ordered = events.to_vec();
        ordered.sort_by_key(|event| (event.position(), event.event_id()));
        let mut replay = Self::new(seeds)?;
        for event in ordered {
            replay.apply(event)?;
        }
        Ok(replay.state)
    }

    fn apply_unrecorded(
        &self,
        event: &ProfileV0ReplayEvent,
    ) -> Result<ProfileV0AdmissionReplayState, ProfileV0ReplayError> {
        let mut next = self.state.clone();
        match event {
            ProfileV0ReplayEvent::Candidate {
                position,
                candidate,
                payload,
            } => match candidate.event_type.as_str() {
                "identity_create" => {
                    apply_identity_create(&mut next, *position, candidate, payload)?;
                }
                "identity_key_rotate" => {
                    apply_identity_key_rotate(&mut next, *position, candidate, payload)?;
                }
                "identity_key_revoke" => {
                    apply_identity_key_revoke(&mut next, *position, candidate, payload)?;
                }
                "identity_verification_update" => {
                    return Err(ProfileV0ReplayError::new(
                        "compatibility_event_not_authorized",
                        "ordinary identity_verification_update is not replay authority",
                    ));
                }
                _ => {
                    return Err(ProfileV0ReplayError::new(
                        "unsupported_admission_profile",
                        "event is not in the Profile-v0 identity-admission event family",
                    ));
                }
            },
            ProfileV0ReplayEvent::CompatibilityIdentityVerificationUpdate {
                position,
                event_id,
                identity_id,
                manifest,
            } => {
                if manifest.manifest_version.trim().is_empty()
                    || matches!(
                        manifest.provenance_class,
                        ProfileV0ProvenanceClass::EventDerived
                    )
                {
                    return Err(ProfileV0ReplayError::new(
                        "compatibility_event_not_authorized",
                        "identity_verification_update requires a non-event-derived versioned compatibility manifest",
                    ));
                }
                if !next.identities.contains_key(identity_id) {
                    return Err(ProfileV0ReplayError::new(
                        "identity_already_exists",
                        "compatibility verification record references an unknown identity",
                    ));
                }
                next.compatibility_verification_records.push(
                    ProfileV0CompatibilityVerificationRecord {
                        event_id: *event_id,
                        identity_id: *identity_id,
                        manifest_version: manifest.manifest_version.clone(),
                        provenance_class: manifest.provenance_class,
                        position: *position,
                    },
                );
            }
        }
        Ok(next)
    }
}

fn apply_identity_create(
    state: &mut ProfileV0AdmissionReplayState,
    position: ProfileV0ReplayPosition,
    candidate: &AuthoredEventCandidate,
    payload_value: &Value,
) -> Result<(), ProfileV0ReplayError> {
    let sponsor = state
        .identities
        .get(&candidate.author_identity_id)
        .ok_or_else(|| {
            ProfileV0ReplayError::new(
                "sponsor_not_human",
                "identity_create sponsor is not a known canonical human identity",
            )
        })?;
    let active_key = active_key(sponsor)?;
    let pure_state = admission_pure_state(state);
    let payload = validate_profile_v0_identity_create_candidate(
        candidate,
        payload_value,
        &active_key.descriptor.raw_public_key_bytes,
        &pure_state,
    )
    .map_err(ProfileV0ReplayError::from_validation)?;

    let roots = roots_from_payload(&payload)?;
    let direct_key = ProfileV0ReplayDirectKey {
        public_key_ref: payload.initial_public_key_ref,
        descriptor: payload.initial_key_descriptor.clone(),
        state: ProfileV0DirectKeyState::Active,
        registration_event_id: Some(candidate.event_id),
        supersession_event_id: None,
        revocation_event_id: None,
    };
    let mut direct_keys = BTreeMap::new();
    direct_keys.insert(payload.initial_public_key_ref, direct_key);
    state.identities.insert(
        payload.identity_id,
        ProfileV0ReplayIdentity {
            identity_id: payload.identity_id,
            identity_kind: ProfileV0IdentityKind::Human,
            provenance_class: ProfileV0ProvenanceClass::EventDerived,
            admission_event_id: Some(candidate.event_id),
            admission_profile_version: Some(PROFILE_V0_ADMISSION_PROFILE.to_string()),
            sponsor_identity_id: Some(candidate.author_identity_id),
            structural_roots: Some(roots),
            direct_keys,
            active_direct_key_ref: Some(payload.initial_public_key_ref),
            lanes: ProfileV0ReplayEligibilityLanes::admitted_initial(),
        },
    );
    state
        .invitation_capacity_debits
        .push(ProfileV0InvitationCapacityDebit {
            admission_event_id: candidate.event_id,
            sponsor_identity_id: candidate.author_identity_id,
            admitted_identity_id: payload.identity_id,
            capacity_period_id: payload.capacity_period_id,
            rulebook_id: payload.rulebook_reference.rulebook_id,
            rulebook_version: payload.rulebook_reference.rulebook_version,
            rulebook_hash: payload.rulebook_reference.rulebook_hash,
            debit_units: CAPACITY_DEBIT_UNITS,
            position,
        });
    Ok(())
}

fn apply_identity_key_rotate(
    state: &mut ProfileV0AdmissionReplayState,
    _position: ProfileV0ReplayPosition,
    candidate: &AuthoredEventCandidate,
    payload_value: &Value,
) -> Result<(), ProfileV0ReplayError> {
    let identity = state
        .identities
        .get(&candidate.author_identity_id)
        .ok_or_else(|| {
            ProfileV0ReplayError::new(
                "key_rotation_authorization_invalid",
                "rotation author is not a known canonical identity",
            )
        })?;
    let active = active_key(identity)?;
    let key_state = direct_key_pure_state(state);
    let payload = validate_profile_v0_identity_key_rotate_candidate(
        candidate,
        payload_value,
        &active.descriptor.raw_public_key_bytes,
        &key_state,
    )
    .map_err(ProfileV0ReplayError::from_validation)?;

    let identity = state
        .identities
        .get_mut(&payload.identity_id)
        .ok_or_else(|| {
            ProfileV0ReplayError::new(
                "key_rotation_authorization_invalid",
                "rotation target identity is not present in replay state",
            )
        })?;
    let previous_key_ref = identity.active_direct_key_ref.ok_or_else(|| {
        ProfileV0ReplayError::new(
            "key_rotation_authorization_invalid",
            "rotation target has no active direct key",
        )
    })?;
    let previous = identity
        .direct_keys
        .get_mut(&previous_key_ref)
        .ok_or_else(|| {
            ProfileV0ReplayError::new(
                "key_rotation_authorization_invalid",
                "active direct key is missing from direct-key history",
            )
        })?;
    previous.state = ProfileV0DirectKeyState::Superseded;
    previous.supersession_event_id = Some(candidate.event_id);
    identity.direct_keys.insert(
        payload.replacement_public_key_ref,
        ProfileV0ReplayDirectKey {
            public_key_ref: payload.replacement_public_key_ref,
            descriptor: payload.replacement_key_descriptor,
            state: ProfileV0DirectKeyState::Active,
            registration_event_id: Some(candidate.event_id),
            supersession_event_id: None,
            revocation_event_id: None,
        },
    );
    identity.active_direct_key_ref = Some(payload.replacement_public_key_ref);
    Ok(())
}

fn apply_identity_key_revoke(
    state: &mut ProfileV0AdmissionReplayState,
    _position: ProfileV0ReplayPosition,
    candidate: &AuthoredEventCandidate,
    payload_value: &Value,
) -> Result<(), ProfileV0ReplayError> {
    let identity = state
        .identities
        .get(&candidate.author_identity_id)
        .ok_or_else(|| {
            ProfileV0ReplayError::new(
                "key_rotation_authorization_invalid",
                "revocation author is not a known canonical identity",
            )
        })?;
    let active = active_key(identity)?;
    let key_state = direct_key_pure_state(state);
    let payload = validate_profile_v0_identity_key_revoke_candidate(
        candidate,
        payload_value,
        &active.descriptor.raw_public_key_bytes,
        &key_state,
    )
    .map_err(ProfileV0ReplayError::from_validation)?;
    let identity = state
        .identities
        .get_mut(&payload.identity_id)
        .ok_or_else(|| {
            ProfileV0ReplayError::new(
                "key_rotation_authorization_invalid",
                "revocation target identity is not present in replay state",
            )
        })?;
    let revoked = identity
        .direct_keys
        .get_mut(&payload.revoked_public_key_ref)
        .ok_or_else(|| {
            ProfileV0ReplayError::new(
                "key_rotation_authorization_invalid",
                "revocation target key is missing from direct-key history",
            )
        })?;
    revoked.state = ProfileV0DirectKeyState::Revoked;
    revoked.revocation_event_id = Some(candidate.event_id);
    Ok(())
}

fn active_key(
    identity: &ProfileV0ReplayIdentity,
) -> Result<&ProfileV0ReplayDirectKey, ProfileV0ReplayError> {
    let key_ref = identity.active_direct_key_ref.ok_or_else(|| {
        ProfileV0ReplayError::new(
            "author_key_inactive",
            "identity has no active direct signing key at this canonical position",
        )
    })?;
    let key = identity.direct_keys.get(&key_ref).ok_or_else(|| {
        ProfileV0ReplayError::new(
            "author_key_inactive",
            "active direct signing key is missing from replay key history",
        )
    })?;
    if key.state != ProfileV0DirectKeyState::Active {
        return Err(ProfileV0ReplayError::new(
            "author_key_inactive",
            "replay active key reference is not active",
        ));
    }
    Ok(key)
}

fn admission_pure_state(state: &ProfileV0AdmissionReplayState) -> ProfileV0AdmissionPureState {
    let mut pure = ProfileV0AdmissionPureState::default();
    for identity in state.identities.values() {
        pure.known_identity_ids.insert(identity.identity_id);
        if let Some(roots) = &identity.structural_roots {
            for (_, root_id) in roots.roots {
                pure.occupied_root_idea_ids.insert(root_id);
            }
            for connection_id in roots.membership_connection_ids {
                pure.occupied_root_connection_ids.insert(connection_id);
            }
        }
        for key in identity.direct_keys.values() {
            pure.historically_registered_key_refs
                .insert(key.public_key_ref);
            if let Ok(raw) = key.descriptor.raw_public_key_bytes.as_slice().try_into() {
                pure.historically_registered_public_keys.insert(raw);
            }
        }
    }
    pure
}

fn direct_key_pure_state(state: &ProfileV0AdmissionReplayState) -> ProfileV0DirectKeyPureState {
    let mut pure = ProfileV0DirectKeyPureState::default();
    for identity in state.identities.values() {
        for key in identity.direct_keys.values() {
            let state = match key.state {
                ProfileV0DirectKeyState::Active => DirectKeyStateV0::Active,
                ProfileV0DirectKeyState::Superseded => DirectKeyStateV0::Superseded,
                ProfileV0DirectKeyState::Revoked => DirectKeyStateV0::Revoked,
                ProfileV0DirectKeyState::Invalid | ProfileV0DirectKeyState::CompatibilityOnly => {
                    DirectKeyStateV0::Inactive
                }
            };
            pure.keys.insert(
                key.public_key_ref,
                DirectKeyHistoryEntryV0 {
                    owner_identity_id: identity.identity_id,
                    state,
                },
            );
            if let Ok(raw) = key.descriptor.raw_public_key_bytes.as_slice().try_into() {
                pure.historically_registered_public_keys.insert(raw);
            }
        }
    }
    pure
}

fn roots_from_payload(
    payload: &ProfileV0IdentityCreatePayload,
) -> Result<ProfileV0StructuralRoots, ProfileV0ReplayError> {
    let plan = &payload.identity_structural_root_plan;
    validate_root_plan(plan)?;
    Ok(ProfileV0StructuralRoots {
        roots: [
            (plan.roots[0].role, plan.roots[0].idea_id),
            (plan.roots[1].role, plan.roots[1].idea_id),
            (plan.roots[2].role, plan.roots[2].idea_id),
            (plan.roots[3].role, plan.roots[3].idea_id),
        ],
        membership_connection_ids: plan.membership_connection_ids,
    })
}

fn validate_structural_roots(roots: &ProfileV0StructuralRoots) -> Result<(), ProfileV0ReplayError> {
    let plan = IdentityStructuralRootPlanV0 {
        roots: roots.roots.map(|(role, idea_id)| {
            verification::admission::IdentityStructuralRootV0 { role, idea_id }
        }),
        membership_connection_ids: roots.membership_connection_ids,
    };
    validate_root_plan(&plan)
}

fn validate_root_plan(plan: &IdentityStructuralRootPlanV0) -> Result<(), ProfileV0ReplayError> {
    verification::admission::validate_identity_structural_root_plan_v0(plan).map_err(|error| {
        ProfileV0ReplayError::new("incomplete_identity_structural_roots", error.message)
    })
}

pub fn profile_v0_public_key_ref_from_hex(value: &str) -> Result<[u8; 32], ProfileV0ReplayError> {
    decode_hash32(value, "public_key_ref")
        .map_err(|error| ProfileV0ReplayError::new(error.code, error.message))
}

pub fn profile_v0_admission_projection_commitment(
    state: &ProfileV0AdmissionReplayState,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    for (identity_id, identity) in &state.identities {
        bytes.extend_from_slice(identity_id.as_bytes());
        bytes.push(identity.provenance_class as u8);
        bytes.extend_from_slice(
            identity
                .admission_event_id
                .unwrap_or(Uuid::nil())
                .as_bytes(),
        );
        bytes.extend_from_slice(
            identity
                .sponsor_identity_id
                .unwrap_or(Uuid::nil())
                .as_bytes(),
        );
        for key in identity.direct_keys.values() {
            bytes.extend_from_slice(&key.public_key_ref);
            bytes.push(key.state as u8);
        }
        if let Some(roots) = &identity.structural_roots {
            for (role, root_id) in roots.roots.iter().copied() {
                bytes.push(role as u8);
                bytes.extend_from_slice(root_id.as_bytes());
            }
            for membership_id in roots.membership_connection_ids {
                bytes.extend_from_slice(membership_id.as_bytes());
            }
        }
    }
    for debit in &state.invitation_capacity_debits {
        bytes.extend_from_slice(debit.admission_event_id.as_bytes());
        bytes.extend_from_slice(debit.sponsor_identity_id.as_bytes());
        bytes.extend_from_slice(debit.admitted_identity_id.as_bytes());
        bytes.extend_from_slice(&debit.debit_units.to_be_bytes());
    }
    bytes.push(derivation_status_tag(state.admission_liveness_blocked));
    hash_with_domain("profile_v0_admission_replay_projection", &bytes)
}

fn derivation_status_tag(value: ProfileV0DerivationStatus) -> u8 {
    match value {
        ProfileV0DerivationStatus::NotYetDerived => 0,
        ProfileV0DerivationStatus::Derived(false) => 1,
        ProfileV0DerivationStatus::Derived(true) => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use encoding::hash::hash_bytes;
    use encoding::payload::to_hex;
    use event_log::profile_v0_admission::{
        canonical_identity_create_payload_bytes_v0, canonical_identity_key_revoke_payload_bytes_v0,
        parse_profile_v0_identity_create_payload, parse_profile_v0_identity_key_revoke_payload,
    };
    use std::sync::OnceLock;
    use verification::signatures::{signed_candidate_bytes_v0, PAYLOAD_BINDING_EMBEDDED};

    fn fixtures() -> &'static Value {
        static FIXTURES: OnceLock<Value> = OnceLock::new();
        FIXTURES.get_or_init(|| {
            serde_json::from_str(include_str!(
                "../../../../docs/conformance/profile-v0-identity-admission.vectors.json"
            ))
            .expect("static Profile-v0 fixture JSON parses")
        })
    }

    fn object(value: &Value) -> &serde_json::Map<String, Value> {
        value.as_object().expect("fixture object")
    }

    fn string<'a>(object: &'a serde_json::Map<String, Value>, key: &str) -> &'a str {
        object
            .get(key)
            .and_then(Value::as_str)
            .expect("fixture string")
    }

    fn candidate_from_fixture(value: &Value) -> AuthoredEventCandidate {
        let value = object(value);
        AuthoredEventCandidate {
            signature_profile: string(value, "signature_profile").to_string(),
            event_id: Uuid::parse_str(string(value, "event_id")).expect("event UUID"),
            event_type: string(value, "event_type").to_string(),
            author_identity_id: Uuid::parse_str(string(value, "author_identity_id"))
                .expect("author UUID"),
            speaker_identity_id: value
                .get("speaker_identity_id")
                .and_then(Value::as_str)
                .map(|id| Uuid::parse_str(id).expect("speaker UUID")),
            public_key_ref: string(value, "public_key_ref").to_string(),
            payload_hash: string(value, "payload_hash").to_string(),
            payload_binding_mode: string(value, "payload_binding_mode").to_string(),
            payload_ref: None,
            author_observed_at: None,
            signature: string(value, "signature").to_string(),
        }
    }

    fn hex32(value: &str) -> [u8; 32] {
        profile_v0_public_key_ref_from_hex(value).expect("hash32")
    }

    fn raw_key(value: &str) -> Vec<u8> {
        (0..value.len())
            .step_by(2)
            .map(|index| u8::from_str_radix(&value[index..index + 2], 16).expect("hex"))
            .collect()
    }

    fn identity_fixture() -> (AuthoredEventCandidate, Value, Vec<u8>) {
        let fixture = fixtures()
            .pointer("/pure_crypto_fixtures/identity_create_primary")
            .expect("identity fixture");
        (
            candidate_from_fixture(fixture.get("candidate").expect("candidate")),
            fixture.get("payload").expect("payload").clone(),
            raw_key(
                fixture
                    .get("sponsor_raw_public_key")
                    .and_then(Value::as_str)
                    .expect("sponsor key"),
            ),
        )
    }

    fn sponsor_seed(
        candidate: &AuthoredEventCandidate,
        raw: Vec<u8>,
    ) -> ProfileV0ReplaySeedIdentity {
        let descriptor = KeyDescriptorV0 {
            key_profile_version: "ed25519_v0".to_string(),
            signature_algorithm: "ed25519".to_string(),
            raw_public_key_bytes: raw,
            owning_identity_id: candidate.author_identity_id,
        };
        ProfileV0ReplaySeedIdentity {
            identity_id: candidate.author_identity_id,
            provenance_class: ProfileV0ProvenanceClass::GenesisAdmitted,
            direct_key: Some(ProfileV0ReplayDirectKey {
                public_key_ref: hex32(&candidate.public_key_ref),
                descriptor,
                state: ProfileV0DirectKeyState::Active,
                registration_event_id: None,
                supersession_event_id: None,
                revocation_event_id: None,
            }),
            structural_roots: None,
        }
    }

    fn identity_event(position: ProfileV0ReplayPosition) -> ProfileV0ReplayEvent {
        let (candidate, payload, _) = identity_fixture();
        ProfileV0ReplayEvent::Candidate {
            position,
            candidate,
            payload,
        }
    }

    fn replay_with_sponsor() -> ProfileV0AdmissionReplay {
        let (candidate, _, raw) = identity_fixture();
        ProfileV0AdmissionReplay::new(&[sponsor_seed(&candidate, raw)]).expect("seed replay")
    }

    fn resign_identity_candidate(
        candidate: &mut AuthoredEventCandidate,
        payload: &Value,
        signer: &SigningKey,
    ) {
        let payload = parse_profile_v0_identity_create_payload(payload).expect("payload parses");
        candidate.payload_hash = to_hex(&hash_bytes(
            &canonical_identity_create_payload_bytes_v0(&payload).expect("payload bytes"),
        ));
        candidate.signature = to_hex(
            &signer
                .sign(&signed_candidate_bytes_v0(candidate).expect("candidate bytes"))
                .to_bytes(),
        );
    }

    #[test]
    fn valid_identity_create_projects_atomic_admission_state_and_is_idempotent() {
        let mut replay = replay_with_sponsor();
        let event = identity_event(ProfileV0ReplayPosition {
            block_height: 1,
            event_index: 0,
        });
        replay
            .apply(event.clone())
            .expect("accepted identity_create");
        replay.apply(event).expect("exact retry is idempotent");

        let state = replay.state();
        let (_, payload, _) = identity_fixture();
        let identity_id = Uuid::parse_str(payload["identity_id"].as_str().unwrap()).unwrap();
        let identity = state
            .identities
            .get(&identity_id)
            .expect("admitted identity");
        assert_eq!(
            identity.provenance_class,
            ProfileV0ProvenanceClass::EventDerived
        );
        assert_eq!(identity.structural_roots.as_ref().unwrap().roots.len(), 4);
        assert_eq!(
            identity.lanes.restricted_verification,
            ProfileV0EligibilityLane::Granted
        );
        assert_eq!(
            identity.lanes.ordinary_writer,
            ProfileV0EligibilityLane::NotGranted
        );
        assert_eq!(state.invitation_capacity_debits.len(), 1);
        assert_eq!(state.invitation_capacity_debits[0].debit_units, 1);
        assert_eq!(
            state.invitation_capacity_balance,
            ProfileV0DerivationStatus::NotYetDerived
        );
        assert_eq!(
            state.admission_liveness_blocked,
            ProfileV0DerivationStatus::NotYetDerived
        );
    }

    #[test]
    fn rejected_identity_candidates_leave_replay_state_unchanged() {
        let cases = [
            "speaker",
            "nonhuman",
            "invalid_proof",
            "duplicate_key",
            "root_collision",
        ];
        for case in cases {
            let mut replay = replay_with_sponsor();
            let (mut candidate, mut payload, _) = identity_fixture();
            let target = Uuid::parse_str(payload["identity_id"].as_str().unwrap()).unwrap();
            match case {
                "speaker" => candidate.speaker_identity_id = Some(candidate.author_identity_id),
                "nonhuman" => {
                    payload["identity_kind"] = Value::String("system".to_string());
                }
                "invalid_proof" => {
                    payload["initial_key_possession_proof"] = Value::String("00".repeat(64));
                    resign_identity_candidate(
                        &mut candidate,
                        &payload,
                        &SigningKey::from_bytes(&[0x11; 32]),
                    );
                }
                "duplicate_key" => {
                    let parsed = parse_profile_v0_identity_create_payload(&payload).unwrap();
                    let identity_id =
                        Uuid::parse_str("00000000-0000-7000-8000-00000000a099").unwrap();
                    let mut descriptor = parsed.initial_key_descriptor.clone();
                    descriptor.owning_identity_id = identity_id;
                    let seed = ProfileV0ReplaySeedIdentity {
                        identity_id,
                        provenance_class: ProfileV0ProvenanceClass::LegacyOperatorProvisioned,
                        direct_key: Some(ProfileV0ReplayDirectKey {
                            public_key_ref: public_key_ref_from_descriptor_v0(&descriptor).unwrap(),
                            descriptor,
                            state: ProfileV0DirectKeyState::CompatibilityOnly,
                            registration_event_id: None,
                            supersession_event_id: None,
                            revocation_event_id: None,
                        }),
                        structural_roots: None,
                    };
                    let (sponsor, _, raw) = identity_fixture();
                    replay = ProfileV0AdmissionReplay::new(&[sponsor_seed(&sponsor, raw), seed])
                        .unwrap();
                }
                "root_collision" => {
                    let parsed = parse_profile_v0_identity_create_payload(&payload).unwrap();
                    let roots = roots_from_payload(&parsed).unwrap();
                    let seed = ProfileV0ReplaySeedIdentity {
                        identity_id: Uuid::parse_str("00000000-0000-7000-8000-00000000a098")
                            .unwrap(),
                        provenance_class: ProfileV0ProvenanceClass::LegacyOperatorProvisioned,
                        direct_key: None,
                        structural_roots: Some(roots),
                    };
                    let (sponsor, _, raw) = identity_fixture();
                    replay = ProfileV0AdmissionReplay::new(&[sponsor_seed(&sponsor, raw), seed])
                        .unwrap();
                }
                _ => unreachable!(),
            }
            let before = replay.state().clone();
            let error = replay
                .apply(ProfileV0ReplayEvent::Candidate {
                    position: ProfileV0ReplayPosition {
                        block_height: 1,
                        event_index: 0,
                    },
                    candidate,
                    payload,
                })
                .expect_err(case);
            assert!(!error.code.is_empty(), "{case}");
            assert_eq!(replay.state(), &before, "{case} target={target}");
        }
    }

    #[test]
    fn key_rotation_and_last_active_key_revocation_follow_direct_key_rules() {
        let fixture = fixtures()
            .pointer("/pure_crypto_fixtures/identity_key_rotate_primary")
            .expect("rotation fixture");
        let candidate = candidate_from_fixture(fixture.get("candidate").unwrap());
        let payload = fixture.get("payload").unwrap().clone();
        let raw = raw_key(
            fixture
                .get("author_raw_public_key")
                .and_then(Value::as_str)
                .unwrap(),
        );
        let descriptor = KeyDescriptorV0 {
            key_profile_version: "ed25519_v0".to_string(),
            signature_algorithm: "ed25519".to_string(),
            raw_public_key_bytes: raw,
            owning_identity_id: candidate.author_identity_id,
        };
        let seed = ProfileV0ReplaySeedIdentity {
            identity_id: candidate.author_identity_id,
            provenance_class: ProfileV0ProvenanceClass::LegacyOperatorProvisioned,
            direct_key: Some(ProfileV0ReplayDirectKey {
                public_key_ref: hex32(&candidate.public_key_ref),
                descriptor,
                state: ProfileV0DirectKeyState::Active,
                registration_event_id: None,
                supersession_event_id: None,
                revocation_event_id: None,
            }),
            structural_roots: None,
        };
        let mut replay = ProfileV0AdmissionReplay::new(&[seed]).unwrap();
        let before_invalid_rotation = replay.state().clone();
        let mut invalid_rotation_candidate = candidate.clone();
        invalid_rotation_candidate.event_id =
            Uuid::parse_str("00000000-0000-7000-8000-000000000105").unwrap();
        invalid_rotation_candidate.signature = "00".repeat(64);
        replay
            .apply(ProfileV0ReplayEvent::Candidate {
                position: ProfileV0ReplayPosition {
                    block_height: 1,
                    event_index: 0,
                },
                candidate: invalid_rotation_candidate,
                payload: payload.clone(),
            })
            .expect_err("invalid rotation cannot mutate replay state");
        assert_eq!(replay.state(), &before_invalid_rotation);
        replay
            .apply(ProfileV0ReplayEvent::Candidate {
                position: ProfileV0ReplayPosition {
                    block_height: 2,
                    event_index: 0,
                },
                candidate: candidate.clone(),
                payload: payload.clone(),
            })
            .expect("rotation accepted");
        let identity = replay
            .state()
            .identities
            .get(&candidate.author_identity_id)
            .unwrap();
        assert_eq!(identity.direct_keys.len(), 2);
        assert_eq!(
            identity.direct_keys[&hex32(&candidate.public_key_ref)].state,
            ProfileV0DirectKeyState::Superseded
        );

        let initial_event = identity_event(ProfileV0ReplayPosition {
            block_height: 3,
            event_index: 0,
        });
        let (_, create_payload, _) = identity_fixture();
        let mut second = replay_with_sponsor();
        second.apply(initial_event).unwrap();
        let parsed = parse_profile_v0_identity_create_payload(&create_payload).unwrap();
        let mut revoke_candidate = AuthoredEventCandidate {
            signature_profile: "ed25519_v0".to_string(),
            event_id: Uuid::parse_str("00000000-0000-7000-8000-000000000104").unwrap(),
            event_type: "identity_key_revoke".to_string(),
            author_identity_id: parsed.identity_id,
            speaker_identity_id: None,
            public_key_ref: create_payload["initial_public_key_ref"]
                .as_str()
                .unwrap()
                .to_string(),
            payload_hash: String::new(),
            payload_binding_mode: PAYLOAD_BINDING_EMBEDDED.to_string(),
            payload_ref: None,
            author_observed_at: None,
            signature: String::new(),
        };
        let revoke_payload = serde_json::json!({
            "identity_id": parsed.identity_id.to_string(),
            "revoked_public_key_ref": create_payload["initial_public_key_ref"].as_str().unwrap(),
        });
        let parsed_revoke = parse_profile_v0_identity_key_revoke_payload(&revoke_payload).unwrap();
        revoke_candidate.payload_hash = to_hex(&hash_bytes(
            &canonical_identity_key_revoke_payload_bytes_v0(&parsed_revoke).unwrap(),
        ));
        revoke_candidate.signature = to_hex(
            &SigningKey::from_bytes(&[0x22; 32])
                .sign(&signed_candidate_bytes_v0(&revoke_candidate).unwrap())
                .to_bytes(),
        );
        let before = second.state().clone();
        let error = second
            .apply(ProfileV0ReplayEvent::Candidate {
                position: ProfileV0ReplayPosition {
                    block_height: 4,
                    event_index: 0,
                },
                candidate: revoke_candidate,
                payload: revoke_payload,
            })
            .expect_err("sole active key cannot be revoked");
        assert_eq!(error.code, "last_active_key_revocation_forbidden");
        assert_eq!(second.state(), &before);
    }

    #[test]
    fn compatibility_verification_is_manifest_only_and_does_not_grant_authority() {
        let (candidate, _, raw) = identity_fixture();
        let mut replay = ProfileV0AdmissionReplay::new(&[sponsor_seed(&candidate, raw)]).unwrap();
        let ordinary = ProfileV0ReplayEvent::Candidate {
            position: ProfileV0ReplayPosition {
                block_height: 1,
                event_index: 0,
            },
            candidate: AuthoredEventCandidate {
                event_type: "identity_verification_update".to_string(),
                ..candidate.clone()
            },
            payload: serde_json::json!({}),
        };
        assert_eq!(
            replay
                .apply(ordinary)
                .expect_err("ordinary update rejected")
                .code,
            "compatibility_event_not_authorized"
        );
        replay
            .apply(
                ProfileV0ReplayEvent::CompatibilityIdentityVerificationUpdate {
                    position: ProfileV0ReplayPosition {
                        block_height: 1,
                        event_index: 1,
                    },
                    event_id: Uuid::parse_str("00000000-0000-7000-8000-000000000105").unwrap(),
                    identity_id: candidate.author_identity_id,
                    manifest: ProfileV0CompatibilityManifest {
                        manifest_version: "legacy-import-v1".to_string(),
                        provenance_class: ProfileV0ProvenanceClass::LegacyOperatorProvisioned,
                    },
                },
            )
            .expect("manifest compatibility record");
        let identity = replay
            .state()
            .identities
            .get(&candidate.author_identity_id)
            .unwrap();
        assert_eq!(
            identity.lanes.ordinary_writer,
            ProfileV0EligibilityLane::NotGranted
        );
        assert_eq!(replay.state().compatibility_verification_records.len(), 1);
    }

    #[test]
    fn replay_is_input_order_independent_and_never_generates_capacity_from_boundary_inputs() {
        let (candidate, _, raw) = identity_fixture();
        let seeds = vec![sponsor_seed(&candidate, raw)];
        let first = identity_event(ProfileV0ReplayPosition {
            block_height: 2,
            event_index: 0,
        });
        let compatibility = ProfileV0ReplayEvent::CompatibilityIdentityVerificationUpdate {
            position: ProfileV0ReplayPosition {
                block_height: 3,
                event_index: 0,
            },
            event_id: Uuid::parse_str("00000000-0000-7000-8000-000000000106").unwrap(),
            identity_id: candidate.author_identity_id,
            manifest: ProfileV0CompatibilityManifest {
                manifest_version: "genesis-import-v1".to_string(),
                provenance_class: ProfileV0ProvenanceClass::GenesisAdmitted,
            },
        };
        let forward =
            ProfileV0AdmissionReplay::replay(&seeds, &[first.clone(), compatibility.clone()])
                .unwrap();
        let reversed = ProfileV0AdmissionReplay::replay(&seeds, &[compatibility, first]).unwrap();
        assert_eq!(forward, reversed);
        assert!(forward
            .invitation_capacity_debits
            .iter()
            .all(|debit| debit.debit_units == 1));
        assert_eq!(
            forward.invitation_capacity_balance,
            ProfileV0DerivationStatus::NotYetDerived
        );
        assert_eq!(
            forward.admission_liveness_blocked,
            ProfileV0DerivationStatus::NotYetDerived
        );
    }
}
