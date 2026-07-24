use crate::schema::Event;
use encoding::hash::hash_bytes;
use encoding::payload::to_hex;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;
use verification::admission::{
    admission_authorization_reference_v0, optional_hash32_bytes_v0,
    public_key_ref_from_descriptor_v0, rulebook_reference_bytes_v0,
    validate_identity_structural_root_plan_v0, verify_initial_key_possession_proof_v0,
    verify_replacement_key_possession_proof_v0, AdmissionCryptoError, IdentityStructuralRootPlanV0,
    IdentityStructuralRootRoleV0, IdentityStructuralRootV0, InitialKeyPossessionInputV0,
    ReplacementKeyPossessionInputV0, RulebookReferenceV0, PROFILE_V0_ADMISSION_PROFILE,
};
use verification::signatures::{
    decode_hash32, decode_public_key32, decode_signature64, verify_ed25519_v0,
    AuthoredEventCandidate, KeyDescriptorV0, PAYLOAD_BINDING_EMBEDDED, SIGNATURE_ALGORITHM_ED25519,
    SIGNATURE_PROFILE_ED25519_V0,
};

const IDENTITY_CREATE_FIELDS: &[&str] = &[
    "identity_id",
    "initial_key_descriptor",
    "initial_public_key_ref",
    "admission_profile_version",
    "capacity_period_id",
    "rulebook_reference",
    "admission_authorization_reference",
    "verification_reference",
    "identity_structural_roots",
    "identity_structural_root_membership_connection_ids",
    "initial_key_possession_proof",
];
const IDENTITY_KEY_ROTATE_FIELDS: &[&str] = &[
    "identity_id",
    "replacement_key_descriptor",
    "replacement_public_key_ref",
    "replacement_key_possession_proof",
];
const IDENTITY_KEY_REVOKE_FIELDS: &[&str] = &["identity_id", "revoked_public_key_ref"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0AdmissionValidationError {
    pub code: &'static str,
    pub message: String,
}

impl ProfileV0AdmissionValidationError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ProfileV0AdmissionValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ProfileV0AdmissionValidationError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0IdentityCreatePayload {
    pub identity_id: Uuid,
    pub initial_key_descriptor: KeyDescriptorV0,
    pub initial_public_key_ref: [u8; 32],
    pub admission_profile_version: String,
    pub capacity_period_id: Uuid,
    pub rulebook_reference: RulebookReferenceV0,
    pub admission_authorization_reference: [u8; 32],
    pub verification_reference: Option<[u8; 32]>,
    pub identity_structural_root_plan: IdentityStructuralRootPlanV0,
    pub initial_key_possession_proof: [u8; 64],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0IdentityKeyRotatePayload {
    pub identity_id: Uuid,
    pub replacement_key_descriptor: KeyDescriptorV0,
    pub replacement_public_key_ref: [u8; 32],
    pub replacement_key_possession_proof: [u8; 64],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileV0IdentityKeyRevokePayload {
    pub identity_id: Uuid,
    pub revoked_public_key_ref: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectKeyStateV0 {
    Active,
    Inactive,
    Superseded,
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectKeyHistoryEntryV0 {
    pub owner_identity_id: Uuid,
    pub state: DirectKeyStateV0,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProfileV0AdmissionPureState {
    pub known_identity_ids: BTreeSet<Uuid>,
    pub historically_registered_key_refs: BTreeSet<[u8; 32]>,
    pub historically_registered_public_keys: BTreeSet<[u8; 32]>,
    pub occupied_root_idea_ids: BTreeSet<Uuid>,
    pub occupied_root_connection_ids: BTreeSet<Uuid>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProfileV0DirectKeyPureState {
    pub keys: BTreeMap<[u8; 32], DirectKeyHistoryEntryV0>,
    pub historically_registered_public_keys: BTreeSet<[u8; 32]>,
}

pub fn parse_profile_v0_identity_create_payload(
    value: &Value,
) -> Result<ProfileV0IdentityCreatePayload, ProfileV0AdmissionValidationError> {
    let object = require_object(value, "malformed_identity_create_payload")?;
    if let Some(identity_kind) = object.get("identity_kind") {
        let value = identity_kind.as_str().unwrap_or("non-human");
        if value != "human" {
            return Err(ProfileV0AdmissionValidationError::new(
                "invalid_target_identity_kind",
                "Profile-v0 identity_create has fixed human target kind",
            ));
        }
    }
    require_exact_fields(
        object,
        IDENTITY_CREATE_FIELDS,
        "malformed_identity_create_payload",
    )?;

    let identity_id = parse_uuid(
        require_string(object, "identity_id", "malformed_identity_create_payload")?,
        "identity_id",
        "malformed_identity_create_payload",
    )?;
    let initial_key_descriptor = parse_key_descriptor(
        require_value(
            object,
            "initial_key_descriptor",
            "malformed_identity_create_payload",
        )?,
        "malformed_initial_key_descriptor",
    )?;
    if initial_key_descriptor.owning_identity_id != identity_id {
        return Err(ProfileV0AdmissionValidationError::new(
            "malformed_initial_key_descriptor",
            "initial key descriptor owner must equal identity_id",
        ));
    }
    let initial_public_key_ref = parse_hash32(
        require_string(
            object,
            "initial_public_key_ref",
            "malformed_identity_create_payload",
        )?,
        "initial_public_key_ref",
        "malformed_initial_key_descriptor",
    )?;
    let admission_profile_version = require_string(
        object,
        "admission_profile_version",
        "malformed_identity_create_payload",
    )?
    .to_string();
    if admission_profile_version != PROFILE_V0_ADMISSION_PROFILE {
        return Err(ProfileV0AdmissionValidationError::new(
            "unsupported_admission_profile",
            "Profile-v0 identity_create requires sponsored_public_admission_v0",
        ));
    }
    let capacity_period_id = parse_uuid(
        require_string(
            object,
            "capacity_period_id",
            "malformed_identity_create_payload",
        )?,
        "capacity_period_id",
        "malformed_identity_create_payload",
    )?;
    let rulebook_reference = parse_rulebook_reference(require_value(
        object,
        "rulebook_reference",
        "malformed_identity_create_payload",
    )?)?;
    let admission_authorization_reference = parse_hash32(
        require_string(
            object,
            "admission_authorization_reference",
            "malformed_identity_create_payload",
        )?,
        "admission_authorization_reference",
        "malformed_admission_authorization",
    )?;
    let verification_reference = parse_verification_reference(object)?;
    let identity_structural_root_plan = parse_structural_root_plan(
        require_value(
            object,
            "identity_structural_roots",
            "malformed_identity_create_payload",
        )?,
        require_value(
            object,
            "identity_structural_root_membership_connection_ids",
            "malformed_identity_create_payload",
        )?,
    )?;
    let initial_key_possession_proof = parse_signature64(
        require_string(
            object,
            "initial_key_possession_proof",
            "malformed_identity_create_payload",
        )?,
        "initial_key_possession_proof",
        "invalid_applicant_possession_proof",
    )?;

    Ok(ProfileV0IdentityCreatePayload {
        identity_id,
        initial_key_descriptor,
        initial_public_key_ref,
        admission_profile_version,
        capacity_period_id,
        rulebook_reference,
        admission_authorization_reference,
        verification_reference,
        identity_structural_root_plan,
        initial_key_possession_proof,
    })
}

pub fn parse_profile_v0_identity_key_rotate_payload(
    value: &Value,
) -> Result<ProfileV0IdentityKeyRotatePayload, ProfileV0AdmissionValidationError> {
    let object = require_object(value, "malformed_identity_key_rotate_payload")?;
    require_exact_fields(
        object,
        IDENTITY_KEY_ROTATE_FIELDS,
        "malformed_identity_key_rotate_payload",
    )?;
    let identity_id = parse_uuid(
        require_string(
            object,
            "identity_id",
            "malformed_identity_key_rotate_payload",
        )?,
        "identity_id",
        "malformed_identity_key_rotate_payload",
    )?;
    let replacement_key_descriptor = parse_key_descriptor(
        require_value(
            object,
            "replacement_key_descriptor",
            "malformed_identity_key_rotate_payload",
        )?,
        "malformed_replacement_key_descriptor",
    )?;
    if replacement_key_descriptor.owning_identity_id != identity_id {
        return Err(ProfileV0AdmissionValidationError::new(
            "malformed_replacement_key_descriptor",
            "replacement key descriptor owner must equal identity_id",
        ));
    }
    let replacement_public_key_ref = parse_hash32(
        require_string(
            object,
            "replacement_public_key_ref",
            "malformed_identity_key_rotate_payload",
        )?,
        "replacement_public_key_ref",
        "malformed_replacement_key_descriptor",
    )?;
    let replacement_key_possession_proof = parse_signature64(
        require_string(
            object,
            "replacement_key_possession_proof",
            "malformed_identity_key_rotate_payload",
        )?,
        "replacement_key_possession_proof",
        "replacement_key_proof_invalid",
    )?;

    Ok(ProfileV0IdentityKeyRotatePayload {
        identity_id,
        replacement_key_descriptor,
        replacement_public_key_ref,
        replacement_key_possession_proof,
    })
}

pub fn parse_profile_v0_identity_key_revoke_payload(
    value: &Value,
) -> Result<ProfileV0IdentityKeyRevokePayload, ProfileV0AdmissionValidationError> {
    let object = require_object(value, "malformed_identity_key_revoke_payload")?;
    require_exact_fields(
        object,
        IDENTITY_KEY_REVOKE_FIELDS,
        "malformed_identity_key_revoke_payload",
    )?;
    Ok(ProfileV0IdentityKeyRevokePayload {
        identity_id: parse_uuid(
            require_string(
                object,
                "identity_id",
                "malformed_identity_key_revoke_payload",
            )?,
            "identity_id",
            "malformed_identity_key_revoke_payload",
        )?,
        revoked_public_key_ref: parse_hash32(
            require_string(
                object,
                "revoked_public_key_ref",
                "malformed_identity_key_revoke_payload",
            )?,
            "revoked_public_key_ref",
            "malformed_identity_key_revoke_payload",
        )?,
    })
}

pub fn canonical_identity_create_payload_bytes_v0(
    payload: &ProfileV0IdentityCreatePayload,
) -> Result<Vec<u8>, ProfileV0AdmissionValidationError> {
    let mut out = Vec::new();
    push_id(
        &mut out,
        payload.identity_id,
        "malformed_identity_create_payload",
    )?;
    out.extend_from_slice(&descriptor_bytes(
        &payload.initial_key_descriptor,
        "malformed_initial_key_descriptor",
    )?);
    out.extend_from_slice(&payload.initial_public_key_ref);
    push_ascii(
        &mut out,
        &payload.admission_profile_version,
        "admission_profile_version",
        "malformed_identity_create_payload",
    )?;
    push_id(
        &mut out,
        payload.capacity_period_id,
        "malformed_identity_create_payload",
    )?;
    out.extend_from_slice(
        &rulebook_reference_bytes_v0(&payload.rulebook_reference).map_err(
            map_admission_crypto_error("malformed_identity_create_payload"),
        )?,
    );
    out.extend_from_slice(&payload.admission_authorization_reference);
    out.extend_from_slice(&optional_hash32_bytes_v0(payload.verification_reference));
    out.extend_from_slice(
        &payload
            .identity_structural_root_plan
            .canonical_bytes()
            .map_err(map_admission_crypto_error(
                "incomplete_identity_structural_roots",
            ))?,
    );
    out.extend_from_slice(&payload.initial_key_possession_proof);
    Ok(out)
}

pub fn canonical_identity_key_rotate_payload_bytes_v0(
    payload: &ProfileV0IdentityKeyRotatePayload,
) -> Result<Vec<u8>, ProfileV0AdmissionValidationError> {
    let mut out = Vec::new();
    push_id(
        &mut out,
        payload.identity_id,
        "malformed_identity_key_rotate_payload",
    )?;
    out.extend_from_slice(&descriptor_bytes(
        &payload.replacement_key_descriptor,
        "malformed_replacement_key_descriptor",
    )?);
    out.extend_from_slice(&payload.replacement_public_key_ref);
    out.extend_from_slice(&payload.replacement_key_possession_proof);
    Ok(out)
}

pub fn canonical_identity_key_revoke_payload_bytes_v0(
    payload: &ProfileV0IdentityKeyRevokePayload,
) -> Result<Vec<u8>, ProfileV0AdmissionValidationError> {
    let mut out = Vec::new();
    push_id(
        &mut out,
        payload.identity_id,
        "malformed_identity_key_revoke_payload",
    )?;
    out.extend_from_slice(&payload.revoked_public_key_ref);
    Ok(out)
}

pub fn validate_profile_v0_identity_create_event(
    event: &Event,
) -> Result<ProfileV0IdentityCreatePayload, ProfileV0AdmissionValidationError> {
    if event.speaker_identity_id.is_some() {
        return Err(ProfileV0AdmissionValidationError::new(
            "speaker_not_permitted",
            "Profile-v0 identity_create requires absent speaker_identity_id",
        ));
    }
    parse_profile_v0_identity_create_payload(&event.payload)
}

pub fn validate_profile_v0_identity_key_rotate_event(
    event: &Event,
) -> Result<ProfileV0IdentityKeyRotatePayload, ProfileV0AdmissionValidationError> {
    if event.speaker_identity_id.is_some() {
        return Err(ProfileV0AdmissionValidationError::new(
            "speaker_not_permitted",
            "Profile-v0 identity_key_rotate requires absent speaker_identity_id",
        ));
    }
    parse_profile_v0_identity_key_rotate_payload(&event.payload)
}

pub fn validate_profile_v0_identity_key_revoke_event(
    event: &Event,
) -> Result<ProfileV0IdentityKeyRevokePayload, ProfileV0AdmissionValidationError> {
    if event.speaker_identity_id.is_some() {
        return Err(ProfileV0AdmissionValidationError::new(
            "speaker_not_permitted",
            "Profile-v0 identity_key_revoke requires absent speaker_identity_id",
        ));
    }
    parse_profile_v0_identity_key_revoke_payload(&event.payload)
}

pub fn reject_ordinary_identity_verification_update(
) -> Result<(), ProfileV0AdmissionValidationError> {
    Err(ProfileV0AdmissionValidationError::new(
        "compatibility_event_not_authorized",
        "identity_verification_update requires an explicit versioned compatibility manifest",
    ))
}

pub fn validate_profile_v0_identity_create_candidate(
    candidate: &AuthoredEventCandidate,
    payload_value: &Value,
    sponsor_raw_public_key: &[u8],
    pure_state: &ProfileV0AdmissionPureState,
) -> Result<ProfileV0IdentityCreatePayload, ProfileV0AdmissionValidationError> {
    if candidate.event_type != "identity_create"
        || candidate.signature_profile != SIGNATURE_PROFILE_ED25519_V0
    {
        return Err(ProfileV0AdmissionValidationError::new(
            "unsupported_admission_profile",
            "candidate is not a Profile-v0 identity_create",
        ));
    }
    if candidate.speaker_identity_id.is_some() {
        return Err(ProfileV0AdmissionValidationError::new(
            "speaker_not_permitted",
            "Profile-v0 identity_create requires absent speaker_identity_id",
        ));
    }
    if candidate.payload_binding_mode != PAYLOAD_BINDING_EMBEDDED || candidate.payload_ref.is_some()
    {
        return Err(ProfileV0AdmissionValidationError::new(
            "malformed_identity_create_payload",
            "Profile-v0 identity_create requires embedded payload without payload_ref",
        ));
    }

    let payload = parse_profile_v0_identity_create_payload(payload_value)?;
    if payload.identity_id == candidate.author_identity_id {
        return Err(ProfileV0AdmissionValidationError::new(
            "self_sponsorship_forbidden",
            "identity_create applicant must differ from sponsor author",
        ));
    }
    let expected_payload_hash = to_hex(&hash_bytes(&canonical_identity_create_payload_bytes_v0(
        &payload,
    )?));
    if candidate.payload_hash != expected_payload_hash {
        return Err(ProfileV0AdmissionValidationError::new(
            "invalid_payload_hash",
            "candidate payload_hash does not match canonical identity_create payload bytes",
        ));
    }
    verify_ed25519_v0(candidate, sponsor_raw_public_key).map_err(map_signature_error)?;

    let expected_key_ref = public_key_ref_from_descriptor_v0(&payload.initial_key_descriptor)
        .map_err(map_admission_crypto_error(
            "malformed_initial_key_descriptor",
        ))?;
    if expected_key_ref != payload.initial_public_key_ref {
        return Err(ProfileV0AdmissionValidationError::new(
            "initial_public_key_ref_mismatch",
            "initial_public_key_ref does not match initial_key_descriptor",
        ));
    }
    let proof_input = InitialKeyPossessionInputV0 {
        identity_create_event_id: candidate.event_id,
        target_identity_id: payload.identity_id,
        admission_profile_version: &payload.admission_profile_version,
        initial_key_descriptor: &payload.initial_key_descriptor,
        initial_public_key_ref: payload.initial_public_key_ref,
        sponsor_identity_id: candidate.author_identity_id,
        admission_authorization_reference: payload.admission_authorization_reference,
        verification_reference: payload.verification_reference,
        identity_structural_root_plan: &payload.identity_structural_root_plan,
    };
    verify_initial_key_possession_proof_v0(&proof_input, &payload.initial_key_possession_proof)
        .map_err(map_initial_proof_error)?;

    if pure_state.known_identity_ids.contains(&payload.identity_id) {
        return Err(ProfileV0AdmissionValidationError::new(
            "identity_already_exists",
            "identity_id is already present in supplied canonical state",
        ));
    }
    let initial_public_key = descriptor_public_key_array(
        &payload.initial_key_descriptor,
        "malformed_initial_key_descriptor",
    )?;
    if pure_state
        .historically_registered_key_refs
        .contains(&payload.initial_public_key_ref)
        || pure_state
            .historically_registered_public_keys
            .contains(&initial_public_key)
    {
        return Err(ProfileV0AdmissionValidationError::new(
            "public_key_already_registered",
            "initial key was already registered in supplied canonical history",
        ));
    }

    let expected_authorization = admission_authorization_reference_v0(
        &payload.admission_profile_version,
        candidate.author_identity_id,
        payload.capacity_period_id,
        &payload.rulebook_reference,
    )
    .map_err(map_admission_crypto_error(
        "malformed_admission_authorization",
    ))?;
    if expected_authorization != payload.admission_authorization_reference {
        return Err(ProfileV0AdmissionValidationError::new(
            "invalid_admission_authorization",
            "admission authorization reference does not match the reduced commitment",
        ));
    }

    validate_root_collisions(&payload.identity_structural_root_plan, pure_state)?;
    Ok(payload)
}

pub fn validate_profile_v0_identity_key_rotate_candidate(
    candidate: &AuthoredEventCandidate,
    payload_value: &Value,
    author_raw_public_key: &[u8],
    key_state: &ProfileV0DirectKeyPureState,
) -> Result<ProfileV0IdentityKeyRotatePayload, ProfileV0AdmissionValidationError> {
    if candidate.event_type != "identity_key_rotate"
        || candidate.signature_profile != SIGNATURE_PROFILE_ED25519_V0
    {
        return Err(ProfileV0AdmissionValidationError::new(
            "key_rotation_authorization_invalid",
            "candidate is not a Profile-v0 identity_key_rotate",
        ));
    }
    if candidate.speaker_identity_id.is_some() {
        return Err(ProfileV0AdmissionValidationError::new(
            "speaker_not_permitted",
            "Profile-v0 identity_key_rotate requires absent speaker_identity_id",
        ));
    }
    let payload = parse_profile_v0_identity_key_rotate_payload(payload_value)?;
    if candidate.author_identity_id != payload.identity_id {
        return Err(ProfileV0AdmissionValidationError::new(
            "key_rotation_authorization_invalid",
            "rotation author must equal identity_id",
        ));
    }
    validate_embedded_payload(
        candidate,
        &canonical_identity_key_rotate_payload_bytes_v0(&payload)?,
    )?;
    verify_ed25519_v0(candidate, author_raw_public_key).map_err(map_signature_error)?;
    validate_authorizing_key(candidate, payload.identity_id, key_state)?;

    let expected_key_ref = public_key_ref_from_descriptor_v0(&payload.replacement_key_descriptor)
        .map_err(map_admission_crypto_error(
        "malformed_replacement_key_descriptor",
    ))?;
    if expected_key_ref != payload.replacement_public_key_ref {
        return Err(ProfileV0AdmissionValidationError::new(
            "replacement_public_key_ref_mismatch",
            "replacement_public_key_ref does not match replacement_key_descriptor",
        ));
    }
    let authorizing_public_key_ref = parse_hash32(
        &candidate.public_key_ref,
        "public_key_ref",
        "key_rotation_authorization_invalid",
    )?;
    let proof_input = ReplacementKeyPossessionInputV0 {
        identity_key_rotate_event_id: candidate.event_id,
        identity_id: payload.identity_id,
        authorizing_public_key_ref,
        replacement_key_descriptor: &payload.replacement_key_descriptor,
        replacement_public_key_ref: payload.replacement_public_key_ref,
    };
    verify_replacement_key_possession_proof_v0(
        &proof_input,
        &payload.replacement_key_possession_proof,
    )
    .map_err(map_replacement_proof_error)?;
    let replacement_public_key = descriptor_public_key_array(
        &payload.replacement_key_descriptor,
        "malformed_replacement_key_descriptor",
    )?;
    if key_state
        .keys
        .contains_key(&payload.replacement_public_key_ref)
        || key_state
            .historically_registered_public_keys
            .contains(&replacement_public_key)
    {
        return Err(ProfileV0AdmissionValidationError::new(
            "public_key_already_registered",
            "replacement key was already registered in supplied canonical history",
        ));
    }
    Ok(payload)
}

pub fn validate_profile_v0_identity_key_revoke_candidate(
    candidate: &AuthoredEventCandidate,
    payload_value: &Value,
    author_raw_public_key: &[u8],
    key_state: &ProfileV0DirectKeyPureState,
) -> Result<ProfileV0IdentityKeyRevokePayload, ProfileV0AdmissionValidationError> {
    if candidate.event_type != "identity_key_revoke"
        || candidate.signature_profile != SIGNATURE_PROFILE_ED25519_V0
    {
        return Err(ProfileV0AdmissionValidationError::new(
            "key_rotation_authorization_invalid",
            "candidate is not a Profile-v0 identity_key_revoke",
        ));
    }
    if candidate.speaker_identity_id.is_some() {
        return Err(ProfileV0AdmissionValidationError::new(
            "speaker_not_permitted",
            "Profile-v0 identity_key_revoke requires absent speaker_identity_id",
        ));
    }
    let payload = parse_profile_v0_identity_key_revoke_payload(payload_value)?;
    if candidate.author_identity_id != payload.identity_id {
        return Err(ProfileV0AdmissionValidationError::new(
            "key_rotation_authorization_invalid",
            "revocation author must equal identity_id",
        ));
    }
    validate_embedded_payload(
        candidate,
        &canonical_identity_key_revoke_payload_bytes_v0(&payload)?,
    )?;
    verify_ed25519_v0(candidate, author_raw_public_key).map_err(map_signature_error)?;
    validate_authorizing_key(candidate, payload.identity_id, key_state)?;

    let revoked_key = key_state
        .keys
        .get(&payload.revoked_public_key_ref)
        .ok_or_else(|| {
            ProfileV0AdmissionValidationError::new(
                "key_rotation_authorization_invalid",
                "revocation target is not a known direct key for the identity",
            )
        })?;
    if revoked_key.owner_identity_id != payload.identity_id {
        return Err(ProfileV0AdmissionValidationError::new(
            "key_rotation_authorization_invalid",
            "revocation target belongs to another identity",
        ));
    }
    match revoked_key.state {
        DirectKeyStateV0::Revoked => Err(ProfileV0AdmissionValidationError::new(
            "key_already_revoked",
            "revocation target is already revoked",
        )),
        DirectKeyStateV0::Active => Err(ProfileV0AdmissionValidationError::new(
            "last_active_key_revocation_forbidden",
            "Profile-v0 forbids revocation of the sole active direct key",
        )),
        DirectKeyStateV0::Superseded => Ok(payload),
        DirectKeyStateV0::Inactive => Err(ProfileV0AdmissionValidationError::new(
            "key_rotation_authorization_invalid",
            "revocation target is not a valid historical direct key",
        )),
    }
}

fn validate_embedded_payload(
    candidate: &AuthoredEventCandidate,
    payload_bytes: &[u8],
) -> Result<(), ProfileV0AdmissionValidationError> {
    if candidate.payload_binding_mode != PAYLOAD_BINDING_EMBEDDED || candidate.payload_ref.is_some()
    {
        return Err(ProfileV0AdmissionValidationError::new(
            "malformed_identity_key_payload",
            "Profile-v0 direct-key events require embedded payload without payload_ref",
        ));
    }
    let expected_payload_hash = to_hex(&hash_bytes(payload_bytes));
    if candidate.payload_hash != expected_payload_hash {
        return Err(ProfileV0AdmissionValidationError::new(
            "invalid_payload_hash",
            "candidate payload_hash does not match canonical direct-key payload bytes",
        ));
    }
    Ok(())
}

fn validate_authorizing_key(
    candidate: &AuthoredEventCandidate,
    identity_id: Uuid,
    key_state: &ProfileV0DirectKeyPureState,
) -> Result<(), ProfileV0AdmissionValidationError> {
    let authorizing_reference = parse_hash32(
        &candidate.public_key_ref,
        "public_key_ref",
        "key_rotation_authorization_invalid",
    )?;
    let key = key_state.keys.get(&authorizing_reference).ok_or_else(|| {
        ProfileV0AdmissionValidationError::new(
            "key_rotation_authorization_invalid",
            "authorizing direct key is not present in supplied key state",
        )
    })?;
    if key.owner_identity_id != identity_id {
        return Err(ProfileV0AdmissionValidationError::new(
            "key_rotation_authorization_invalid",
            "authorizing direct key belongs to another identity",
        ));
    }
    match key.state {
        DirectKeyStateV0::Active => {}
        DirectKeyStateV0::Superseded => {
            return Err(ProfileV0AdmissionValidationError::new(
                "key_already_superseded",
                "superseded direct key cannot authorize a later event",
            ))
        }
        DirectKeyStateV0::Revoked => {
            return Err(ProfileV0AdmissionValidationError::new(
                "author_key_revoked",
                "revoked direct key cannot authorize a later event",
            ))
        }
        DirectKeyStateV0::Inactive => {
            return Err(ProfileV0AdmissionValidationError::new(
                "key_rotation_authorization_invalid",
                "inactive direct key cannot authorize a lifecycle event",
            ))
        }
    };
    let active_key_count = key_state
        .keys
        .values()
        .filter(|key| key.owner_identity_id == identity_id && key.state == DirectKeyStateV0::Active)
        .count();
    if active_key_count != 1 {
        return Err(ProfileV0AdmissionValidationError::new(
            "key_rotation_authorization_invalid",
            "Profile-v0 direct-key state must contain exactly one active key for the identity",
        ));
    }
    Ok(())
}

fn validate_root_collisions(
    root_plan: &IdentityStructuralRootPlanV0,
    pure_state: &ProfileV0AdmissionPureState,
) -> Result<(), ProfileV0AdmissionValidationError> {
    validate_identity_structural_root_plan_v0(root_plan).map_err(map_admission_crypto_error(
        "incomplete_identity_structural_roots",
    ))?;
    if root_plan
        .roots
        .iter()
        .any(|root| pure_state.occupied_root_idea_ids.contains(&root.idea_id))
        || root_plan
            .membership_connection_ids
            .iter()
            .any(|id| pure_state.occupied_root_connection_ids.contains(id))
    {
        return Err(ProfileV0AdmissionValidationError::new(
            "structural_root_collision",
            "identity structural root IDs collide with supplied canonical state",
        ));
    }
    Ok(())
}

fn parse_rulebook_reference(
    value: &Value,
) -> Result<RulebookReferenceV0, ProfileV0AdmissionValidationError> {
    let object = require_object(value, "malformed_identity_create_payload")?;
    require_exact_fields(
        object,
        &["rulebook_id", "rulebook_version", "rulebook_hash"],
        "malformed_identity_create_payload",
    )?;
    let rulebook_id = parse_uuid(
        require_string(object, "rulebook_id", "malformed_identity_create_payload")?,
        "rulebook_id",
        "malformed_identity_create_payload",
    )?;
    let rulebook_version = require_string(
        object,
        "rulebook_version",
        "malformed_identity_create_payload",
    )?
    .to_string();
    if !rulebook_version.is_ascii() || rulebook_version.is_empty() {
        return Err(ProfileV0AdmissionValidationError::new(
            "malformed_identity_create_payload",
            "rulebook_version must be non-empty ASCII",
        ));
    }
    let rulebook_hash = parse_hash32(
        require_string(object, "rulebook_hash", "malformed_identity_create_payload")?,
        "rulebook_hash",
        "malformed_identity_create_payload",
    )?;
    Ok(RulebookReferenceV0 {
        rulebook_id,
        rulebook_version,
        rulebook_hash,
    })
}

fn parse_key_descriptor(
    value: &Value,
    error_code: &'static str,
) -> Result<KeyDescriptorV0, ProfileV0AdmissionValidationError> {
    let object = require_object(value, error_code)?;
    require_exact_fields(
        object,
        &[
            "key_profile_version",
            "signature_algorithm",
            "raw_public_key",
            "owning_identity_id",
        ],
        error_code,
    )?;
    let key_profile_version =
        require_string(object, "key_profile_version", error_code)?.to_string();
    let signature_algorithm =
        require_string(object, "signature_algorithm", error_code)?.to_string();
    if key_profile_version != SIGNATURE_PROFILE_ED25519_V0
        || signature_algorithm != SIGNATURE_ALGORITHM_ED25519
    {
        return Err(ProfileV0AdmissionValidationError::new(
            error_code,
            "Profile-v0 key descriptors require ed25519_v0 and ed25519",
        ));
    }
    let raw_public_key = decode_public_key32(require_string(object, "raw_public_key", error_code)?)
        .map_err(|error| ProfileV0AdmissionValidationError::new(error_code, error.message))?;
    let owning_identity_id = parse_uuid(
        require_string(object, "owning_identity_id", error_code)?,
        "owning_identity_id",
        error_code,
    )?;
    Ok(KeyDescriptorV0 {
        key_profile_version,
        signature_algorithm,
        raw_public_key_bytes: raw_public_key.to_vec(),
        owning_identity_id,
    })
}

fn parse_verification_reference(
    object: &Map<String, Value>,
) -> Result<Option<[u8; 32]>, ProfileV0AdmissionValidationError> {
    let Some(value) = object.get("verification_reference") else {
        return Ok(None);
    };
    let value = value.as_str().ok_or_else(|| {
        ProfileV0AdmissionValidationError::new(
            "invalid_verification_reference",
            "verification_reference must be omitted or a lowercase hash32 string",
        )
    })?;
    let hash = parse_hash32(
        value,
        "verification_reference",
        "invalid_verification_reference",
    )?;
    if hash == [0_u8; 32] {
        return Err(ProfileV0AdmissionValidationError::new(
            "invalid_verification_reference",
            "all-zero verification_reference is not a canonical absence encoding",
        ));
    }
    Ok(Some(hash))
}

fn parse_structural_root_plan(
    roots_value: &Value,
    membership_connection_ids_value: &Value,
) -> Result<IdentityStructuralRootPlanV0, ProfileV0AdmissionValidationError> {
    let roots = roots_value.as_array().ok_or_else(|| {
        ProfileV0AdmissionValidationError::new(
            "incomplete_identity_structural_roots",
            "identity_structural_roots must be an array",
        )
    })?;
    if roots.len() != 4 {
        return Err(ProfileV0AdmissionValidationError::new(
            "incomplete_identity_structural_roots",
            "identity_structural_roots must contain exactly four entries",
        ));
    }
    let mut parsed_roots = Vec::with_capacity(4);
    for root in roots {
        let object = require_object(root, "incomplete_identity_structural_roots")?;
        require_exact_fields(
            object,
            &["role", "idea_id"],
            "incomplete_identity_structural_roots",
        )?;
        let role_value = require_value(object, "role", "incomplete_identity_structural_roots")?
            .as_u64()
            .ok_or_else(|| {
                ProfileV0AdmissionValidationError::new(
                    "incomplete_identity_structural_roots",
                    "structural root role must be a closed u8 enum",
                )
            })?;
        let role = u8::try_from(role_value)
            .ok()
            .and_then(|value| IdentityStructuralRootRoleV0::from_u8(value).ok())
            .ok_or_else(|| {
                ProfileV0AdmissionValidationError::new(
                    "incomplete_identity_structural_roots",
                    "unsupported Profile-v0 identity structural-root role",
                )
            })?;
        let idea_id = parse_uuid(
            require_string(object, "idea_id", "incomplete_identity_structural_roots")?,
            "idea_id",
            "incomplete_identity_structural_roots",
        )?;
        parsed_roots.push(IdentityStructuralRootV0 { role, idea_id });
    }
    let membership_ids = membership_connection_ids_value.as_array().ok_or_else(|| {
        ProfileV0AdmissionValidationError::new(
            "incomplete_identity_structural_roots",
            "identity structural-root membership IDs must be an array",
        )
    })?;
    if membership_ids.len() != 3 {
        return Err(ProfileV0AdmissionValidationError::new(
            "incomplete_identity_structural_roots",
            "identity structural-root membership IDs must contain exactly three entries",
        ));
    }
    let mut parsed_membership_ids = Vec::with_capacity(3);
    for value in membership_ids {
        let value = value.as_str().ok_or_else(|| {
            ProfileV0AdmissionValidationError::new(
                "incomplete_identity_structural_roots",
                "identity structural-root membership IDs must be UUIDv7 strings",
            )
        })?;
        parsed_membership_ids.push(parse_uuid(
            value,
            "identity structural-root membership connection ID",
            "incomplete_identity_structural_roots",
        )?);
    }
    let roots: [IdentityStructuralRootV0; 4] = parsed_roots.try_into().map_err(|_| {
        ProfileV0AdmissionValidationError::new(
            "incomplete_identity_structural_roots",
            "identity structural roots must contain exactly four entries",
        )
    })?;
    let membership_connection_ids: [Uuid; 3] = parsed_membership_ids.try_into().map_err(|_| {
        ProfileV0AdmissionValidationError::new(
            "incomplete_identity_structural_roots",
            "identity structural-root membership IDs must contain exactly three entries",
        )
    })?;
    let plan = IdentityStructuralRootPlanV0 {
        roots,
        membership_connection_ids,
    };
    validate_identity_structural_root_plan_v0(&plan).map_err(map_admission_crypto_error(
        "incomplete_identity_structural_roots",
    ))?;
    Ok(plan)
}

fn require_object<'a>(
    value: &'a Value,
    code: &'static str,
) -> Result<&'a Map<String, Value>, ProfileV0AdmissionValidationError> {
    value.as_object().ok_or_else(|| {
        ProfileV0AdmissionValidationError::new(code, "payload value must be a JSON object")
    })
}

fn require_exact_fields(
    object: &Map<String, Value>,
    allowed: &[&str],
    code: &'static str,
) -> Result<(), ProfileV0AdmissionValidationError> {
    for field in allowed {
        if *field != "verification_reference" && !object.contains_key(*field) {
            return Err(ProfileV0AdmissionValidationError::new(
                code,
                format!("missing required field {field}"),
            ));
        }
    }
    if object
        .keys()
        .any(|field| !allowed.contains(&field.as_str()))
    {
        return Err(ProfileV0AdmissionValidationError::new(
            code,
            "payload contains forbidden field",
        ));
    }
    Ok(())
}

fn require_value<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    code: &'static str,
) -> Result<&'a Value, ProfileV0AdmissionValidationError> {
    object.get(field).ok_or_else(|| {
        ProfileV0AdmissionValidationError::new(code, format!("missing required field {field}"))
    })
}

fn require_string<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    code: &'static str,
) -> Result<&'a str, ProfileV0AdmissionValidationError> {
    require_value(object, field, code)?.as_str().ok_or_else(|| {
        ProfileV0AdmissionValidationError::new(code, format!("{field} must be a string"))
    })
}

fn parse_uuid(
    value: &str,
    field: &str,
    code: &'static str,
) -> Result<Uuid, ProfileV0AdmissionValidationError> {
    let uuid = Uuid::parse_str(value).map_err(|_| {
        ProfileV0AdmissionValidationError::new(code, format!("{field} must be a UUIDv7"))
    })?;
    if uuid.get_version_num() != 7 || value != uuid.to_string() {
        return Err(ProfileV0AdmissionValidationError::new(
            code,
            format!("{field} must use canonical UUIDv7 text"),
        ));
    }
    Ok(uuid)
}

fn parse_hash32(
    value: &str,
    field: &'static str,
    code: &'static str,
) -> Result<[u8; 32], ProfileV0AdmissionValidationError> {
    decode_hash32(value, field)
        .map_err(|error| ProfileV0AdmissionValidationError::new(code, error.message))
}

fn parse_signature64(
    value: &str,
    field: &'static str,
    code: &'static str,
) -> Result<[u8; 64], ProfileV0AdmissionValidationError> {
    if field != "signature" && value.len() != 128 {
        return Err(ProfileV0AdmissionValidationError::new(
            code,
            format!("{field} must be exactly 64 raw bytes as lowercase hex"),
        ));
    }
    decode_signature64(value)
        .map_err(|error| ProfileV0AdmissionValidationError::new(code, error.message))
}

fn push_ascii(
    out: &mut Vec<u8>,
    value: &str,
    field: &str,
    code: &'static str,
) -> Result<(), ProfileV0AdmissionValidationError> {
    if !value.is_ascii() {
        return Err(ProfileV0AdmissionValidationError::new(
            code,
            format!("{field} must be ASCII"),
        ));
    }
    out.extend_from_slice(&(value.len() as u32).to_be_bytes());
    out.extend_from_slice(value.as_bytes());
    Ok(())
}

fn push_id(
    out: &mut Vec<u8>,
    value: Uuid,
    code: &'static str,
) -> Result<(), ProfileV0AdmissionValidationError> {
    if value.get_version_num() != 7 {
        return Err(ProfileV0AdmissionValidationError::new(
            code,
            "identifier must be UUIDv7",
        ));
    }
    out.extend_from_slice(&36_u32.to_be_bytes());
    out.extend_from_slice(value.to_string().as_bytes());
    Ok(())
}

fn descriptor_bytes(
    descriptor: &KeyDescriptorV0,
    code: &'static str,
) -> Result<Vec<u8>, ProfileV0AdmissionValidationError> {
    verification::admission::key_descriptor_bytes_from_descriptor_v0(descriptor)
        .map_err(map_admission_crypto_error(code))
}

fn descriptor_public_key_array(
    descriptor: &KeyDescriptorV0,
    code: &'static str,
) -> Result<[u8; 32], ProfileV0AdmissionValidationError> {
    descriptor
        .raw_public_key_bytes
        .as_slice()
        .try_into()
        .map_err(|_| {
            ProfileV0AdmissionValidationError::new(
                code,
                "Profile-v0 key descriptors must contain exactly 32 public-key bytes",
            )
        })
}

fn map_signature_error(
    error: verification::signatures::SignatureValidationError,
) -> ProfileV0AdmissionValidationError {
    ProfileV0AdmissionValidationError::new(error.code, error.message)
}

fn map_admission_crypto_error(
    fallback: &'static str,
) -> impl FnOnce(AdmissionCryptoError) -> ProfileV0AdmissionValidationError {
    move |error| ProfileV0AdmissionValidationError::new(fallback, error.message)
}

fn map_initial_proof_error(error: AdmissionCryptoError) -> ProfileV0AdmissionValidationError {
    let code = match error.code {
        "public_key_ref_mismatch" => "initial_public_key_ref_mismatch",
        "possession_proof_binding_mismatch" => "applicant_proof_binding_mismatch",
        "invalid_possession_proof" => "invalid_applicant_possession_proof",
        "malformed_key_descriptor" => "malformed_initial_key_descriptor",
        _ => "invalid_applicant_possession_proof",
    };
    ProfileV0AdmissionValidationError::new(code, error.message)
}

fn map_replacement_proof_error(error: AdmissionCryptoError) -> ProfileV0AdmissionValidationError {
    let code = match error.code {
        "public_key_ref_mismatch" => "replacement_public_key_ref_mismatch",
        "malformed_key_descriptor" => "malformed_replacement_key_descriptor",
        _ => "replacement_key_proof_invalid",
    };
    ProfileV0AdmissionValidationError::new(code, error.message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use serde_json::json;
    use verification::admission::{
        admission_authorization_reference_v0, initial_key_possession_bytes_v0,
        public_key_ref_from_descriptor_v0, replacement_key_possession_bytes_v0,
    };
    use verification::signatures::{
        public_key_ref_v0, signed_candidate_bytes_v0, AuthoredEventCandidate,
    };

    const EVENT_ID: &str = "00000000-0000-7000-8000-000000000101";
    const SPONSOR_ID: &str = "00000000-0000-7000-8000-00000000a001";
    const TARGET_ID: &str = "00000000-0000-7000-8000-00000000b001";
    const CAPACITY_PERIOD_ID: &str = "00000000-0000-7000-8000-00000000c001";
    const RULEBOOK_ID: &str = "00000000-0000-7000-8000-00000000d001";

    #[test]
    fn ia_002_to_013_identity_create_pure_conformance() {
        let (payload, candidate, sponsor, pure_state) = valid_identity_create_fixture(None);
        let parsed = validate_profile_v0_identity_create_candidate(
            &candidate,
            &payload,
            sponsor.verifying_key().as_bytes(),
            &pure_state,
        )
        .expect("IA-002/004/006/009/010/013 valid candidate");
        assert_eq!(
            parsed.identity_id,
            parse_uuid(TARGET_ID, "target", "test").unwrap()
        );
        assert_eq!(optional_hash32_bytes_v0(None), vec![0x00]);
        assert_eq!(
            optional_hash32_bytes_v0(Some([0x42; 32])),
            [&[0x01][..], &[0x42; 32][..]].concat()
        );

        let event = Event {
            id: candidate.event_id,
            kind: "identity_create".to_string(),
            payload: payload.clone(),
            speaker_identity_id: None,
        };
        assert!(validate_profile_v0_identity_create_event(&event).is_ok());

        let mut speaker_event = event.clone();
        speaker_event.speaker_identity_id = Some(candidate.author_identity_id);
        assert_eq!(
            validate_profile_v0_identity_create_event(&speaker_event)
                .expect_err("IA-003 speaker")
                .code,
            "speaker_not_permitted"
        );

        let mut non_human = payload.clone();
        non_human.as_object_mut().unwrap().insert(
            "identity_kind".to_string(),
            Value::String("organization".to_string()),
        );
        assert_eq!(
            parse_profile_v0_identity_create_payload(&non_human)
                .expect_err("IA-005 non-human")
                .code,
            "invalid_target_identity_kind"
        );

        for invalid_reference in [
            Value::Null,
            Value::String(String::new()),
            Value::String("00".repeat(32)),
        ] {
            let mut invalid = payload.clone();
            invalid
                .as_object_mut()
                .unwrap()
                .insert("verification_reference".to_string(), invalid_reference);
            assert_eq!(
                parse_profile_v0_identity_create_payload(&invalid)
                    .expect_err("IA-008 alternate absence")
                    .code,
                "invalid_verification_reference"
            );
        }

        let mut mutated = payload.clone();
        mutated.as_object_mut().unwrap().insert(
            "capacity_period_id".to_string(),
            Value::String("00000000-0000-7000-8000-00000000c002".to_string()),
        );
        let mutated_candidate = resign_identity_create_candidate(&candidate, &mutated, &sponsor);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &mutated_candidate,
                &mutated,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("IA-011 bound field mutation")
            .code,
            "invalid_admission_authorization"
        );

        let parsed = parse_profile_v0_identity_create_payload(&payload).unwrap();
        let message = initial_key_possession_bytes_v0(&InitialKeyPossessionInputV0 {
            identity_create_event_id: candidate.event_id,
            target_identity_id: parsed.identity_id,
            admission_profile_version: &parsed.admission_profile_version,
            initial_key_descriptor: &parsed.initial_key_descriptor,
            initial_public_key_ref: parsed.initial_public_key_ref,
            sponsor_identity_id: candidate.author_identity_id,
            admission_authorization_reference: parsed.admission_authorization_reference,
            verification_reference: parsed.verification_reference,
            identity_structural_root_plan: &parsed.identity_structural_root_plan,
        })
        .unwrap();
        assert!(!message
            .windows(64)
            .any(|window| window == parsed.initial_key_possession_proof));
    }

    #[test]
    fn exact_admission_commitment_and_root_plan_bytes_follow_the_fixed_layout() {
        let (payload, candidate, _, _) = valid_identity_create_fixture(Some([0x55; 32]));
        let parsed = parse_profile_v0_identity_create_payload(&payload).unwrap();
        let mut expected_commitment_bytes = Vec::new();
        push_ascii(
            &mut expected_commitment_bytes,
            "seed.identity.admission_authorization.v0",
            "domain",
            "test",
        )
        .unwrap();
        push_ascii(
            &mut expected_commitment_bytes,
            PROFILE_V0_ADMISSION_PROFILE,
            "profile",
            "test",
        )
        .unwrap();
        push_id(
            &mut expected_commitment_bytes,
            candidate.author_identity_id,
            "test",
        )
        .unwrap();
        push_id(
            &mut expected_commitment_bytes,
            parsed.capacity_period_id,
            "test",
        )
        .unwrap();
        expected_commitment_bytes
            .extend_from_slice(&rulebook_reference_bytes_v0(&parsed.rulebook_reference).unwrap());
        assert_eq!(
            parsed.admission_authorization_reference,
            hash_bytes(&expected_commitment_bytes).as_slice()
        );

        let mut expected_roots = Vec::new();
        expected_roots.extend_from_slice(&4_u32.to_be_bytes());
        for root in &parsed.identity_structural_root_plan.roots {
            expected_roots.push(root.role as u8);
            expected_roots.extend_from_slice(&36_u32.to_be_bytes());
            expected_roots.extend_from_slice(root.idea_id.to_string().as_bytes());
        }
        expected_roots.extend_from_slice(&3_u32.to_be_bytes());
        for connection_id in &parsed
            .identity_structural_root_plan
            .membership_connection_ids
        {
            expected_roots.extend_from_slice(&36_u32.to_be_bytes());
            expected_roots.extend_from_slice(connection_id.to_string().as_bytes());
        }
        assert_eq!(
            parsed
                .identity_structural_root_plan
                .canonical_bytes()
                .unwrap(),
            expected_roots
        );
        assert_eq!(
            optional_hash32_bytes_v0(parsed.verification_reference),
            [&[0x01][..], &[0x55; 32][..]].concat()
        );
    }

    #[test]
    fn every_applicant_bound_field_rejects_a_resigned_candidate() {
        let (payload, candidate, sponsor, pure_state) = valid_identity_create_fixture(None);
        let mut changed_event_id = candidate.clone();
        changed_event_id.event_id =
            parse_uuid("00000000-0000-7000-8000-000000000102", "event", "test").unwrap();
        changed_event_id = resign_identity_create_candidate(&changed_event_id, &payload, &sponsor);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &changed_event_id,
                &payload,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("event id is applicant bound")
            .code,
            "applicant_proof_binding_mismatch"
        );

        let mut changed_authorization = payload.clone();
        changed_authorization.as_object_mut().unwrap().insert(
            "admission_authorization_reference".to_string(),
            Value::String("66".repeat(32)),
        );
        let changed_authorization_candidate =
            resign_identity_create_candidate(&candidate, &changed_authorization, &sponsor);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &changed_authorization_candidate,
                &changed_authorization,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("authorization is applicant bound")
            .code,
            "applicant_proof_binding_mismatch"
        );

        let mut changed_reference = payload.clone();
        changed_reference.as_object_mut().unwrap().insert(
            "verification_reference".to_string(),
            Value::String("77".repeat(32)),
        );
        let changed_reference_candidate =
            resign_identity_create_candidate(&candidate, &changed_reference, &sponsor);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &changed_reference_candidate,
                &changed_reference,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("verification reference is applicant bound")
            .code,
            "applicant_proof_binding_mismatch"
        );

        let mut changed_roots = payload.clone();
        changed_roots
            .as_object_mut()
            .unwrap()
            .get_mut("identity_structural_roots")
            .unwrap()
            .as_array_mut()
            .unwrap()[0]
            .as_object_mut()
            .unwrap()
            .insert(
                "idea_id".to_string(),
                Value::String("00000000-0000-7000-8000-00000000e101".to_string()),
            );
        let changed_roots_candidate =
            resign_identity_create_candidate(&candidate, &changed_roots, &sponsor);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &changed_roots_candidate,
                &changed_roots,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("root plan is applicant bound")
            .code,
            "applicant_proof_binding_mismatch"
        );

        let mut changed_membership = payload.clone();
        changed_membership
            .as_object_mut()
            .unwrap()
            .get_mut("identity_structural_root_membership_connection_ids")
            .unwrap()
            .as_array_mut()
            .unwrap()[0] = Value::String("00000000-0000-7000-8000-00000000f101".to_string());
        let changed_membership_candidate =
            resign_identity_create_candidate(&candidate, &changed_membership, &sponsor);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &changed_membership_candidate,
                &changed_membership,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("membership plan is applicant bound")
            .code,
            "applicant_proof_binding_mismatch"
        );

        let mut changed_target = payload.clone();
        let target_id = "00000000-0000-7000-8000-00000000b002";
        changed_target.as_object_mut().unwrap().insert(
            "identity_id".to_string(),
            Value::String(target_id.to_string()),
        );
        changed_target
            .as_object_mut()
            .unwrap()
            .get_mut("initial_key_descriptor")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert(
                "owning_identity_id".to_string(),
                Value::String(target_id.to_string()),
            );
        replace_initial_key_reference(&mut changed_target);
        let changed_target_candidate =
            resign_identity_create_candidate(&candidate, &changed_target, &sponsor);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &changed_target_candidate,
                &changed_target,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("target is applicant bound")
            .code,
            "applicant_proof_binding_mismatch"
        );

        let mut changed_descriptor = payload.clone();
        let alternate_applicant = SigningKey::from_bytes(&[0x33; 32]);
        changed_descriptor
            .as_object_mut()
            .unwrap()
            .get_mut("initial_key_descriptor")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert(
                "raw_public_key".to_string(),
                Value::String(to_hex(alternate_applicant.verifying_key().as_bytes())),
            );
        replace_initial_key_reference(&mut changed_descriptor);
        let changed_descriptor_candidate =
            resign_identity_create_candidate(&candidate, &changed_descriptor, &sponsor);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &changed_descriptor_candidate,
                &changed_descriptor,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("descriptor is applicant bound")
            .code,
            "applicant_proof_binding_mismatch"
        );

        let mut changed_key_ref = payload.clone();
        changed_key_ref.as_object_mut().unwrap().insert(
            "initial_public_key_ref".to_string(),
            Value::String("88".repeat(32)),
        );
        let changed_key_ref_candidate =
            resign_identity_create_candidate(&candidate, &changed_key_ref, &sponsor);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &changed_key_ref_candidate,
                &changed_key_ref,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("initial key reference is validated before proof")
            .code,
            "initial_public_key_ref_mismatch"
        );

        let mut changed_profile = payload.clone();
        changed_profile.as_object_mut().unwrap().insert(
            "admission_profile_version".to_string(),
            Value::String("other_profile".to_string()),
        );
        assert_eq!(
            parse_profile_v0_identity_create_payload(&changed_profile)
                .expect_err("Profile-v0 fixed admission profile")
                .code,
            "unsupported_admission_profile"
        );

        let alternate_sponsor = SigningKey::from_bytes(&[0x44; 32]);
        let alternate_sponsor_id =
            parse_uuid("00000000-0000-7000-8000-00000000a002", "sponsor", "test").unwrap();
        let mut changed_sponsor = candidate.clone();
        changed_sponsor.author_identity_id = alternate_sponsor_id;
        changed_sponsor.public_key_ref = public_key_ref_v0(
            alternate_sponsor.verifying_key().as_bytes(),
            alternate_sponsor_id,
        )
        .unwrap();
        let changed_sponsor =
            resign_identity_create_candidate(&changed_sponsor, &payload, &alternate_sponsor);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &changed_sponsor,
                &payload,
                alternate_sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("sponsor is applicant bound")
            .code,
            "applicant_proof_binding_mismatch"
        );

        let mut changed_proof = payload.clone();
        changed_proof.as_object_mut().unwrap().insert(
            "initial_key_possession_proof".to_string(),
            Value::String("00".repeat(64)),
        );
        let changed_proof_candidate =
            resign_identity_create_candidate(&candidate, &changed_proof, &sponsor);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &changed_proof_candidate,
                &changed_proof,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("completed sponsor payload does not make an invalid applicant proof valid")
            .code,
            "applicant_proof_binding_mismatch"
        );
    }

    #[test]
    fn ia_015_and_root_validation_use_supplied_immutable_state() {
        let (payload, candidate, sponsor, mut pure_state) = valid_identity_create_fixture(None);
        let parsed = parse_profile_v0_identity_create_payload(&payload).unwrap();
        pure_state.known_identity_ids.insert(parsed.identity_id);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &candidate,
                &payload,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("IA-014 duplicate identity")
            .code,
            "identity_already_exists"
        );

        let (payload, candidate, sponsor, mut pure_state) = valid_identity_create_fixture(None);
        let parsed = parse_profile_v0_identity_create_payload(&payload).unwrap();
        pure_state
            .historically_registered_key_refs
            .insert(parsed.initial_public_key_ref);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &candidate,
                &payload,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("IA-015 historical key reuse")
            .code,
            "public_key_already_registered"
        );

        let (payload, candidate, sponsor, mut pure_state) = valid_identity_create_fixture(None);
        let parsed = parse_profile_v0_identity_create_payload(&payload).unwrap();
        pure_state
            .historically_registered_public_keys
            .insert(descriptor_public_key_array(&parsed.initial_key_descriptor, "test").unwrap());
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &candidate,
                &payload,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("historical raw public-key reuse")
            .code,
            "public_key_already_registered"
        );

        let (payload, candidate, sponsor, mut pure_state) = valid_identity_create_fixture(None);
        let parsed = parse_profile_v0_identity_create_payload(&payload).unwrap();
        pure_state
            .occupied_root_idea_ids
            .insert(parsed.identity_structural_root_plan.roots[0].idea_id);
        assert_eq!(
            validate_profile_v0_identity_create_candidate(
                &candidate,
                &payload,
                sponsor.verifying_key().as_bytes(),
                &pure_state,
            )
            .expect_err("IA-017 root collision")
            .code,
            "structural_root_collision"
        );
    }

    #[test]
    fn malformed_root_plan_and_proof_are_rejected_without_state() {
        let (payload, _, _, _) = valid_identity_create_fixture(None);
        let mut malformed_roots = payload.clone();
        malformed_roots
            .as_object_mut()
            .unwrap()
            .get_mut("identity_structural_roots")
            .unwrap()
            .as_array_mut()
            .unwrap()
            .pop();
        assert_eq!(
            parse_profile_v0_identity_create_payload(&malformed_roots)
                .expect_err("missing root")
                .code,
            "incomplete_identity_structural_roots"
        );

        let mut malformed_proof = payload;
        malformed_proof.as_object_mut().unwrap().insert(
            "initial_key_possession_proof".to_string(),
            Value::String("00".repeat(63)),
        );
        assert_eq!(
            parse_profile_v0_identity_create_payload(&malformed_proof)
                .expect_err("malformed proof")
                .code,
            "invalid_applicant_possession_proof"
        );

        let (mut malformed_descriptor, _, _, _) = valid_identity_create_fixture(None);
        malformed_descriptor
            .as_object_mut()
            .unwrap()
            .get_mut("initial_key_descriptor")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert(
                "key_profile_version".to_string(),
                Value::String("unrecognized_v0".to_string()),
            );
        assert_eq!(
            parse_profile_v0_identity_create_payload(&malformed_descriptor)
                .expect_err("malformed descriptor")
                .code,
            "malformed_initial_key_descriptor"
        );
    }

    #[test]
    fn ia_025_rotation_proof_and_last_key_revoke_rule() {
        let identity_id = parse_uuid(TARGET_ID, "identity", "test").unwrap();
        let event_id = parse_uuid(EVENT_ID, "event", "test").unwrap();
        let author = SigningKey::from_bytes(&[0x11; 32]);
        let replacement = SigningKey::from_bytes(&[0x22; 32]);
        let author_ref = public_key_ref_v0(author.verifying_key().as_bytes(), identity_id).unwrap();
        let author_ref_bytes = parse_hash32(&author_ref, "author", "test").unwrap();
        let descriptor = KeyDescriptorV0 {
            key_profile_version: SIGNATURE_PROFILE_ED25519_V0.to_string(),
            signature_algorithm: SIGNATURE_ALGORITHM_ED25519.to_string(),
            raw_public_key_bytes: replacement.verifying_key().as_bytes().to_vec(),
            owning_identity_id: identity_id,
        };
        let replacement_ref = public_key_ref_from_descriptor_v0(&descriptor).unwrap();
        let proof_bytes = replacement_key_possession_bytes_v0(&ReplacementKeyPossessionInputV0 {
            identity_key_rotate_event_id: event_id,
            identity_id,
            authorizing_public_key_ref: author_ref_bytes,
            replacement_key_descriptor: &descriptor,
            replacement_public_key_ref: replacement_ref,
        })
        .unwrap();
        let proof = replacement.sign(&proof_bytes).to_bytes();
        let payload = json!({
            "identity_id": identity_id.to_string(),
            "replacement_key_descriptor": descriptor_json(&descriptor),
            "replacement_public_key_ref": to_hex(&replacement_ref),
            "replacement_key_possession_proof": to_hex(&proof),
        });
        let parsed = parse_profile_v0_identity_key_rotate_payload(&payload).unwrap();
        let payload_hash = to_hex(&hash_bytes(
            &canonical_identity_key_rotate_payload_bytes_v0(&parsed).unwrap(),
        ));
        let unsigned = AuthoredEventCandidate {
            signature_profile: SIGNATURE_PROFILE_ED25519_V0.to_string(),
            event_id,
            event_type: "identity_key_rotate".to_string(),
            author_identity_id: identity_id,
            speaker_identity_id: None,
            public_key_ref: author_ref.clone(),
            payload_hash,
            payload_binding_mode: PAYLOAD_BINDING_EMBEDDED.to_string(),
            payload_ref: None,
            author_observed_at: None,
            signature: String::new(),
        };
        let candidate = AuthoredEventCandidate {
            signature: to_hex(
                &author
                    .sign(&signed_candidate_bytes_v0(&unsigned).unwrap())
                    .to_bytes(),
            ),
            ..unsigned
        };
        let mut state = ProfileV0DirectKeyPureState::default();
        state.keys.insert(
            author_ref_bytes,
            DirectKeyHistoryEntryV0 {
                owner_identity_id: identity_id,
                state: DirectKeyStateV0::Active,
            },
        );
        validate_profile_v0_identity_key_rotate_candidate(
            &candidate,
            &payload,
            author.verifying_key().as_bytes(),
            &state,
        )
        .expect("IA-025 valid rotation proof");

        let mut superseded_author_state = state.clone();
        superseded_author_state
            .keys
            .get_mut(&author_ref_bytes)
            .unwrap()
            .state = DirectKeyStateV0::Superseded;
        assert_eq!(
            validate_profile_v0_identity_key_rotate_candidate(
                &candidate,
                &payload,
                author.verifying_key().as_bytes(),
                &superseded_author_state,
            )
            .expect_err("superseded author key")
            .code,
            "key_already_superseded"
        );

        let mut multiple_active_keys = state.clone();
        multiple_active_keys.keys.insert(
            [0x77; 32],
            DirectKeyHistoryEntryV0 {
                owner_identity_id: identity_id,
                state: DirectKeyStateV0::Active,
            },
        );
        assert_eq!(
            validate_profile_v0_identity_key_rotate_candidate(
                &candidate,
                &payload,
                author.verifying_key().as_bytes(),
                &multiple_active_keys,
            )
            .expect_err("one-active-key invariant")
            .code,
            "key_rotation_authorization_invalid"
        );

        let mut bad_replacement_proof = payload.clone();
        bad_replacement_proof.as_object_mut().unwrap().insert(
            "replacement_key_possession_proof".to_string(),
            Value::String("00".repeat(64)),
        );
        let bad_parsed =
            parse_profile_v0_identity_key_rotate_payload(&bad_replacement_proof).unwrap();
        let bad_candidate = resign_key_candidate(
            &candidate,
            "identity_key_rotate",
            &canonical_identity_key_rotate_payload_bytes_v0(&bad_parsed).unwrap(),
            &author,
        );
        assert_eq!(
            validate_profile_v0_identity_key_rotate_candidate(
                &bad_candidate,
                &bad_replacement_proof,
                author.verifying_key().as_bytes(),
                &state,
            )
            .expect_err("IA-026 invalid replacement proof")
            .code,
            "replacement_key_proof_invalid"
        );

        let mut reused_state = state.clone();
        reused_state.keys.insert(
            replacement_ref,
            DirectKeyHistoryEntryV0 {
                owner_identity_id: identity_id,
                state: DirectKeyStateV0::Superseded,
            },
        );
        assert_eq!(
            validate_profile_v0_identity_key_rotate_candidate(
                &candidate,
                &payload,
                author.verifying_key().as_bytes(),
                &reused_state,
            )
            .expect_err("IA-027 historical replacement reuse")
            .code,
            "public_key_already_registered"
        );

        let mut raw_reused_state = state.clone();
        raw_reused_state
            .historically_registered_public_keys
            .insert(descriptor_public_key_array(&descriptor, "test").unwrap());
        assert_eq!(
            validate_profile_v0_identity_key_rotate_candidate(
                &candidate,
                &payload,
                author.verifying_key().as_bytes(),
                &raw_reused_state,
            )
            .expect_err("historical raw replacement-key reuse")
            .code,
            "public_key_already_registered"
        );

        let revoke_payload = json!({
            "identity_id": identity_id.to_string(),
            "revoked_public_key_ref": author_ref,
        });
        let revoke = parse_profile_v0_identity_key_revoke_payload(&revoke_payload).unwrap();
        let revoke_hash = to_hex(&hash_bytes(
            &canonical_identity_key_revoke_payload_bytes_v0(&revoke).unwrap(),
        ));
        let unsigned_revoke = AuthoredEventCandidate {
            event_type: "identity_key_revoke".to_string(),
            payload_hash: revoke_hash,
            signature: String::new(),
            ..candidate.clone()
        };
        let revoke_candidate = AuthoredEventCandidate {
            signature: to_hex(
                &author
                    .sign(&signed_candidate_bytes_v0(&unsigned_revoke).unwrap())
                    .to_bytes(),
            ),
            ..unsigned_revoke
        };
        assert_eq!(
            validate_profile_v0_identity_key_revoke_candidate(
                &revoke_candidate,
                &revoke_payload,
                author.verifying_key().as_bytes(),
                &state,
            )
            .expect_err("IA-028 last active key")
            .code,
            "last_active_key_revocation_forbidden"
        );

        let revoked_target_ref = [0x99; 32];
        let mut revoked_target_state = state.clone();
        revoked_target_state.keys.insert(
            revoked_target_ref,
            DirectKeyHistoryEntryV0 {
                owner_identity_id: identity_id,
                state: DirectKeyStateV0::Revoked,
            },
        );
        let mut revoked_target_payload = revoke_payload.clone();
        revoked_target_payload.as_object_mut().unwrap().insert(
            "revoked_public_key_ref".to_string(),
            Value::String(to_hex(&revoked_target_ref)),
        );
        let parsed_revoked_target =
            parse_profile_v0_identity_key_revoke_payload(&revoked_target_payload).unwrap();
        let revoked_target_candidate = resign_key_candidate(
            &revoke_candidate,
            "identity_key_revoke",
            &canonical_identity_key_revoke_payload_bytes_v0(&parsed_revoked_target).unwrap(),
            &author,
        );
        assert_eq!(
            validate_profile_v0_identity_key_revoke_candidate(
                &revoked_target_candidate,
                &revoked_target_payload,
                author.verifying_key().as_bytes(),
                &revoked_target_state,
            )
            .expect_err("already revoked target")
            .code,
            "key_already_revoked"
        );
    }

    #[test]
    fn ia_029_ordinary_verification_update_is_rejected() {
        assert_eq!(
            reject_ordinary_identity_verification_update()
                .expect_err("ordinary compatibility event")
                .code,
            "compatibility_event_not_authorized"
        );
    }

    #[test]
    fn static_profile_v0_admission_crypto_fixtures_match_runtime() {
        let fixture: Value = serde_json::from_str(include_str!(
            "../../../../docs/conformance/profile-v0-identity-admission.vectors.json"
        ))
        .expect("identity-admission vector JSON");
        let fixtures = fixture
            .get("pure_crypto_fixtures")
            .and_then(Value::as_object)
            .expect("pure crypto fixtures");

        let identity_fixture = fixtures
            .get("identity_create_primary")
            .and_then(Value::as_object)
            .expect("identity_create fixture");
        let identity_payload = identity_fixture.get("payload").expect("identity payload");
        let identity_candidate = candidate_from_fixture(
            identity_fixture
                .get("candidate")
                .expect("identity candidate"),
        );
        let identity_expected = identity_fixture
            .get("expected")
            .and_then(Value::as_object)
            .expect("identity expectations");
        let sponsor_raw_public_key = decode_public_key32(required_fixture_str(
            identity_fixture,
            "sponsor_raw_public_key",
        ))
        .expect("sponsor public key");
        let parsed_identity = validate_profile_v0_identity_create_candidate(
            &identity_candidate,
            identity_payload,
            &sponsor_raw_public_key,
            &ProfileV0AdmissionPureState::default(),
        )
        .expect("identity fixture validates");
        assert_eq!(
            to_hex(&canonical_identity_create_payload_bytes_v0(&parsed_identity).unwrap()),
            required_fixture_str(identity_expected, "canonical_payload_bytes_hex")
        );
        assert_eq!(
            to_hex(&parsed_identity.admission_authorization_reference),
            required_fixture_str(identity_expected, "admission_authorization_reference")
        );
        assert_eq!(
            to_hex(&optional_hash32_bytes_v0(None)),
            required_fixture_str(
                identity_expected,
                "verification_reference_absent_encoding_hex"
            )
        );
        let present_reference = parse_hash32(
            required_fixture_str(identity_expected, "verification_reference_present_hash"),
            "verification_reference_present_hash",
            "test",
        )
        .unwrap();
        assert_eq!(
            to_hex(&optional_hash32_bytes_v0(Some(present_reference))),
            required_fixture_str(
                identity_expected,
                "verification_reference_present_encoding_hex"
            )
        );
        let initial_message = initial_key_possession_bytes_v0(&InitialKeyPossessionInputV0 {
            identity_create_event_id: identity_candidate.event_id,
            target_identity_id: parsed_identity.identity_id,
            admission_profile_version: &parsed_identity.admission_profile_version,
            initial_key_descriptor: &parsed_identity.initial_key_descriptor,
            initial_public_key_ref: parsed_identity.initial_public_key_ref,
            sponsor_identity_id: identity_candidate.author_identity_id,
            admission_authorization_reference: parsed_identity.admission_authorization_reference,
            verification_reference: parsed_identity.verification_reference,
            identity_structural_root_plan: &parsed_identity.identity_structural_root_plan,
        })
        .unwrap();
        assert_eq!(
            to_hex(&initial_message),
            required_fixture_str(identity_expected, "applicant_possession_bytes_hex")
        );
        assert_eq!(
            to_hex(&parsed_identity.initial_key_possession_proof),
            required_fixture_str(identity_expected, "applicant_possession_proof")
        );

        let rotation_fixture = fixtures
            .get("identity_key_rotate_primary")
            .and_then(Value::as_object)
            .expect("identity_key_rotate fixture");
        let rotation_payload = rotation_fixture.get("payload").expect("rotation payload");
        let rotation_candidate = candidate_from_fixture(
            rotation_fixture
                .get("candidate")
                .expect("rotation candidate"),
        );
        let rotation_expected = rotation_fixture
            .get("expected")
            .and_then(Value::as_object)
            .expect("rotation expectations");
        let author_raw_public_key = decode_public_key32(required_fixture_str(
            rotation_fixture,
            "author_raw_public_key",
        ))
        .expect("rotation author public key");
        let authorizing_key_ref =
            parse_hash32(&rotation_candidate.public_key_ref, "public_key_ref", "test").unwrap();
        let mut direct_key_state = ProfileV0DirectKeyPureState::default();
        direct_key_state.keys.insert(
            authorizing_key_ref,
            DirectKeyHistoryEntryV0 {
                owner_identity_id: rotation_candidate.author_identity_id,
                state: DirectKeyStateV0::Active,
            },
        );
        let parsed_rotation = validate_profile_v0_identity_key_rotate_candidate(
            &rotation_candidate,
            rotation_payload,
            &author_raw_public_key,
            &direct_key_state,
        )
        .expect("rotation fixture validates");
        assert_eq!(
            to_hex(&canonical_identity_key_rotate_payload_bytes_v0(&parsed_rotation).unwrap()),
            required_fixture_str(rotation_expected, "canonical_payload_bytes_hex")
        );
        let replacement_message =
            replacement_key_possession_bytes_v0(&ReplacementKeyPossessionInputV0 {
                identity_key_rotate_event_id: rotation_candidate.event_id,
                identity_id: parsed_rotation.identity_id,
                authorizing_public_key_ref: authorizing_key_ref,
                replacement_key_descriptor: &parsed_rotation.replacement_key_descriptor,
                replacement_public_key_ref: parsed_rotation.replacement_public_key_ref,
            })
            .unwrap();
        assert_eq!(
            to_hex(&replacement_message),
            required_fixture_str(rotation_expected, "replacement_possession_bytes_hex")
        );
        assert_eq!(
            to_hex(&parsed_rotation.replacement_key_possession_proof),
            required_fixture_str(rotation_expected, "replacement_possession_proof")
        );
    }

    fn valid_identity_create_fixture(
        verification_reference: Option<[u8; 32]>,
    ) -> (
        Value,
        AuthoredEventCandidate,
        SigningKey,
        ProfileV0AdmissionPureState,
    ) {
        let event_id = parse_uuid(EVENT_ID, "event", "test").unwrap();
        let sponsor_id = parse_uuid(SPONSOR_ID, "sponsor", "test").unwrap();
        let target_id = parse_uuid(TARGET_ID, "target", "test").unwrap();
        let period_id = parse_uuid(CAPACITY_PERIOD_ID, "period", "test").unwrap();
        let rulebook_id = parse_uuid(RULEBOOK_ID, "rulebook", "test").unwrap();
        let sponsor = SigningKey::from_bytes(&[0x11; 32]);
        let applicant = SigningKey::from_bytes(&[0x22; 32]);
        let descriptor = KeyDescriptorV0 {
            key_profile_version: SIGNATURE_PROFILE_ED25519_V0.to_string(),
            signature_algorithm: SIGNATURE_ALGORITHM_ED25519.to_string(),
            raw_public_key_bytes: applicant.verifying_key().as_bytes().to_vec(),
            owning_identity_id: target_id,
        };
        let initial_ref = public_key_ref_from_descriptor_v0(&descriptor).unwrap();
        let rulebook = RulebookReferenceV0 {
            rulebook_id,
            rulebook_version: "profile_v0".to_string(),
            rulebook_hash: [0x44; 32],
        };
        let authorization = admission_authorization_reference_v0(
            PROFILE_V0_ADMISSION_PROFILE,
            sponsor_id,
            period_id,
            &rulebook,
        )
        .unwrap();
        let root_plan = root_plan();
        let proof_message = initial_key_possession_bytes_v0(&InitialKeyPossessionInputV0 {
            identity_create_event_id: event_id,
            target_identity_id: target_id,
            admission_profile_version: PROFILE_V0_ADMISSION_PROFILE,
            initial_key_descriptor: &descriptor,
            initial_public_key_ref: initial_ref,
            sponsor_identity_id: sponsor_id,
            admission_authorization_reference: authorization,
            verification_reference,
            identity_structural_root_plan: &root_plan,
        })
        .unwrap();
        let proof = applicant.sign(&proof_message).to_bytes();
        let payload = json!({
            "identity_id": target_id.to_string(),
            "initial_key_descriptor": descriptor_json(&descriptor),
            "initial_public_key_ref": to_hex(&initial_ref),
            "admission_profile_version": PROFILE_V0_ADMISSION_PROFILE,
            "capacity_period_id": period_id.to_string(),
            "rulebook_reference": {
                "rulebook_id": rulebook.rulebook_id.to_string(),
                "rulebook_version": rulebook.rulebook_version,
                "rulebook_hash": to_hex(&rulebook.rulebook_hash),
            },
            "admission_authorization_reference": to_hex(&authorization),
            "identity_structural_roots": root_plan.roots.iter().map(|root| json!({
                "role": root.role as u8,
                "idea_id": root.idea_id.to_string(),
            })).collect::<Vec<_>>(),
            "identity_structural_root_membership_connection_ids": root_plan.membership_connection_ids.iter().map(Uuid::to_string).collect::<Vec<_>>(),
            "initial_key_possession_proof": to_hex(&proof),
        });
        let mut payload = payload;
        if let Some(reference) = verification_reference {
            payload.as_object_mut().unwrap().insert(
                "verification_reference".to_string(),
                Value::String(to_hex(&reference)),
            );
        }
        let parsed = parse_profile_v0_identity_create_payload(&payload).unwrap();
        let sponsor_ref =
            public_key_ref_v0(sponsor.verifying_key().as_bytes(), sponsor_id).unwrap();
        let unsigned = AuthoredEventCandidate {
            signature_profile: SIGNATURE_PROFILE_ED25519_V0.to_string(),
            event_id,
            event_type: "identity_create".to_string(),
            author_identity_id: sponsor_id,
            speaker_identity_id: None,
            public_key_ref: sponsor_ref,
            payload_hash: to_hex(&hash_bytes(
                &canonical_identity_create_payload_bytes_v0(&parsed).unwrap(),
            )),
            payload_binding_mode: PAYLOAD_BINDING_EMBEDDED.to_string(),
            payload_ref: None,
            author_observed_at: None,
            signature: String::new(),
        };
        let candidate = AuthoredEventCandidate {
            signature: to_hex(
                &sponsor
                    .sign(&signed_candidate_bytes_v0(&unsigned).unwrap())
                    .to_bytes(),
            ),
            ..unsigned
        };
        (
            payload,
            candidate,
            sponsor,
            ProfileV0AdmissionPureState::default(),
        )
    }

    fn descriptor_json(descriptor: &KeyDescriptorV0) -> Value {
        json!({
            "key_profile_version": descriptor.key_profile_version,
            "signature_algorithm": descriptor.signature_algorithm,
            "raw_public_key": to_hex(&descriptor.raw_public_key_bytes),
            "owning_identity_id": descriptor.owning_identity_id.to_string(),
        })
    }

    fn candidate_from_fixture(value: &Value) -> AuthoredEventCandidate {
        let object = value.as_object().expect("candidate object");
        let speaker_identity_id = match object.get("speaker_identity_id") {
            None | Some(Value::Null) => None,
            Some(Value::String(value)) => {
                Some(parse_uuid(value, "speaker_identity_id", "test").unwrap())
            }
            _ => panic!("speaker_identity_id must be null or UUIDv7"),
        };
        assert!(
            matches!(object.get("payload_ref"), Some(Value::Null)),
            "fixture Profile-v0 admission candidate must have absent payload_ref"
        );
        assert!(
            matches!(object.get("author_observed_at"), Some(Value::Null)),
            "fixture Profile-v0 admission candidate must have absent author_observed_at"
        );
        AuthoredEventCandidate {
            signature_profile: required_fixture_str(object, "signature_profile").to_string(),
            event_id: parse_uuid(required_fixture_str(object, "event_id"), "event_id", "test")
                .unwrap(),
            event_type: required_fixture_str(object, "event_type").to_string(),
            author_identity_id: parse_uuid(
                required_fixture_str(object, "author_identity_id"),
                "author_identity_id",
                "test",
            )
            .unwrap(),
            speaker_identity_id,
            public_key_ref: required_fixture_str(object, "public_key_ref").to_string(),
            payload_hash: required_fixture_str(object, "payload_hash").to_string(),
            payload_binding_mode: required_fixture_str(object, "payload_binding_mode").to_string(),
            payload_ref: None,
            author_observed_at: None,
            signature: required_fixture_str(object, "signature").to_string(),
        }
    }

    fn required_fixture_str<'a>(object: &'a Map<String, Value>, field: &str) -> &'a str {
        object
            .get(field)
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("fixture field {field} must be a string"))
    }

    fn replace_initial_key_reference(payload: &mut Value) {
        let descriptor = parse_key_descriptor(
            payload
                .as_object()
                .unwrap()
                .get("initial_key_descriptor")
                .unwrap(),
            "test",
        )
        .unwrap();
        let reference = public_key_ref_from_descriptor_v0(&descriptor).unwrap();
        payload.as_object_mut().unwrap().insert(
            "initial_public_key_ref".to_string(),
            Value::String(to_hex(&reference)),
        );
    }

    fn resign_identity_create_candidate(
        candidate: &AuthoredEventCandidate,
        payload: &Value,
        sponsor: &SigningKey,
    ) -> AuthoredEventCandidate {
        let parsed = parse_profile_v0_identity_create_payload(payload).unwrap();
        let mut unsigned = candidate.clone();
        unsigned.payload_hash = to_hex(&hash_bytes(
            &canonical_identity_create_payload_bytes_v0(&parsed).unwrap(),
        ));
        unsigned.signature.clear();
        AuthoredEventCandidate {
            signature: to_hex(
                &sponsor
                    .sign(&signed_candidate_bytes_v0(&unsigned).unwrap())
                    .to_bytes(),
            ),
            ..unsigned
        }
    }

    fn resign_key_candidate(
        candidate: &AuthoredEventCandidate,
        event_type: &str,
        payload_bytes: &[u8],
        author: &SigningKey,
    ) -> AuthoredEventCandidate {
        let mut unsigned = candidate.clone();
        unsigned.event_type = event_type.to_string();
        unsigned.payload_hash = to_hex(&hash_bytes(payload_bytes));
        unsigned.signature.clear();
        AuthoredEventCandidate {
            signature: to_hex(
                &author
                    .sign(&signed_candidate_bytes_v0(&unsigned).unwrap())
                    .to_bytes(),
            ),
            ..unsigned
        }
    }

    fn root_plan() -> IdentityStructuralRootPlanV0 {
        IdentityStructuralRootPlanV0 {
            roots: [
                root(
                    IdentityStructuralRootRoleV0::Mindgarden,
                    "00000000-0000-7000-8000-00000000e001",
                ),
                root(
                    IdentityStructuralRootRoleV0::BackyardOfRelationships,
                    "00000000-0000-7000-8000-00000000e002",
                ),
                root(
                    IdentityStructuralRootRoleV0::SelfTree,
                    "00000000-0000-7000-8000-00000000e003",
                ),
                root(
                    IdentityStructuralRootRoleV0::Anthill,
                    "00000000-0000-7000-8000-00000000e004",
                ),
            ],
            membership_connection_ids: [
                parse_uuid("00000000-0000-7000-8000-00000000f001", "root", "test").unwrap(),
                parse_uuid("00000000-0000-7000-8000-00000000f002", "root", "test").unwrap(),
                parse_uuid("00000000-0000-7000-8000-00000000f003", "root", "test").unwrap(),
            ],
        }
    }

    fn root(role: IdentityStructuralRootRoleV0, idea_id: &str) -> IdentityStructuralRootV0 {
        IdentityStructuralRootV0 {
            role,
            idea_id: parse_uuid(idea_id, "root", "test").unwrap(),
        }
    }
}
