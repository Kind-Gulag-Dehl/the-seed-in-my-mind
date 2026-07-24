use crate::signatures::{
    key_descriptor_bytes_v0, KeyDescriptorV0, SIGNATURE_ALGORITHM_ED25519,
    SIGNATURE_PROFILE_ED25519_V0,
};
use ed25519_dalek::{Signature, VerifyingKey};
use encoding::canonical::{encode_id, encode_u32, encode_u8, validate_id};
use encoding::hash::hash_bytes;
use std::collections::BTreeSet;
use uuid::Uuid;

pub const PROFILE_V0_ADMISSION_PROFILE: &str = "sponsored_public_admission_v0";
pub const ADMISSION_AUTHORIZATION_DOMAIN: &str = "seed.identity.admission_authorization.v0";
pub const INITIAL_KEY_POSSESSION_DOMAIN: &str = "seed.identity.initial_key_possession.v0";
pub const REPLACEMENT_KEY_POSSESSION_DOMAIN: &str = "seed.identity.replacement_key_possession.v0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionCryptoError {
    pub code: &'static str,
    pub message: String,
}

impl AdmissionCryptoError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for AdmissionCryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for AdmissionCryptoError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulebookReferenceV0 {
    pub rulebook_id: Uuid,
    pub rulebook_version: String,
    pub rulebook_hash: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IdentityStructuralRootRoleV0 {
    Mindgarden = 0x01,
    BackyardOfRelationships = 0x02,
    SelfTree = 0x03,
    Anthill = 0x04,
}

impl IdentityStructuralRootRoleV0 {
    pub const ORDERED: [Self; 4] = [
        Self::Mindgarden,
        Self::BackyardOfRelationships,
        Self::SelfTree,
        Self::Anthill,
    ];

    pub fn from_u8(value: u8) -> Result<Self, AdmissionCryptoError> {
        match value {
            0x01 => Ok(Self::Mindgarden),
            0x02 => Ok(Self::BackyardOfRelationships),
            0x03 => Ok(Self::SelfTree),
            0x04 => Ok(Self::Anthill),
            _ => Err(AdmissionCryptoError::new(
                "incomplete_identity_structural_roots",
                "unsupported Profile-v0 identity structural-root role",
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityStructuralRootV0 {
    pub role: IdentityStructuralRootRoleV0,
    pub idea_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityStructuralRootPlanV0 {
    pub roots: [IdentityStructuralRootV0; 4],
    pub membership_connection_ids: [Uuid; 3],
}

impl IdentityStructuralRootPlanV0 {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, AdmissionCryptoError> {
        validate_identity_structural_root_plan_v0(self)?;
        let mut out = Vec::new();
        out.extend_from_slice(&encode_u32(4));
        for root in &self.roots {
            out.extend_from_slice(&encode_u8(root.role as u8));
            push_id(&mut out, root.idea_id)?;
        }
        out.extend_from_slice(&encode_u32(3));
        for connection_id in &self.membership_connection_ids {
            push_id(&mut out, *connection_id)?;
        }
        Ok(out)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitialKeyPossessionInputV0<'a> {
    pub identity_create_event_id: Uuid,
    pub target_identity_id: Uuid,
    pub admission_profile_version: &'a str,
    pub initial_key_descriptor: &'a KeyDescriptorV0,
    pub initial_public_key_ref: [u8; 32],
    pub sponsor_identity_id: Uuid,
    pub admission_authorization_reference: [u8; 32],
    pub verification_reference: Option<[u8; 32]>,
    pub identity_structural_root_plan: &'a IdentityStructuralRootPlanV0,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplacementKeyPossessionInputV0<'a> {
    pub identity_key_rotate_event_id: Uuid,
    pub identity_id: Uuid,
    pub authorizing_public_key_ref: [u8; 32],
    pub replacement_key_descriptor: &'a KeyDescriptorV0,
    pub replacement_public_key_ref: [u8; 32],
}

pub fn validate_key_descriptor_v0(
    descriptor: &KeyDescriptorV0,
) -> Result<(), AdmissionCryptoError> {
    if descriptor.key_profile_version != SIGNATURE_PROFILE_ED25519_V0 {
        return Err(AdmissionCryptoError::new(
            "malformed_key_descriptor",
            "key_profile_version must be ed25519_v0",
        ));
    }
    if descriptor.signature_algorithm != SIGNATURE_ALGORITHM_ED25519 {
        return Err(AdmissionCryptoError::new(
            "malformed_key_descriptor",
            "signature_algorithm must be ed25519",
        ));
    }
    if descriptor.raw_public_key_bytes.len() != 32 {
        return Err(AdmissionCryptoError::new(
            "malformed_key_descriptor",
            "Profile-v0 public keys must be exactly 32 bytes",
        ));
    }
    validate_uuid_v7(descriptor.owning_identity_id, "owning_identity_id")?;
    Ok(())
}

pub fn key_descriptor_bytes_from_descriptor_v0(
    descriptor: &KeyDescriptorV0,
) -> Result<Vec<u8>, AdmissionCryptoError> {
    validate_key_descriptor_v0(descriptor)?;
    key_descriptor_bytes_v0(
        &descriptor.raw_public_key_bytes,
        descriptor.owning_identity_id,
    )
    .map_err(|error| AdmissionCryptoError::new("malformed_key_descriptor", error.message))
}

pub fn public_key_ref_from_descriptor_v0(
    descriptor: &KeyDescriptorV0,
) -> Result<[u8; 32], AdmissionCryptoError> {
    let descriptor_bytes = key_descriptor_bytes_from_descriptor_v0(descriptor)?;
    let mut bytes = Vec::new();
    push_ascii(
        &mut bytes,
        "seed.identity.public_key_descriptor.v0",
        "domain",
    )?;
    bytes.extend_from_slice(&descriptor_bytes);
    hash32(&bytes)
}

pub fn optional_hash32_bytes_v0(value: Option<[u8; 32]>) -> Vec<u8> {
    match value {
        Some(value) => {
            let mut out = Vec::with_capacity(33);
            out.push(0x01);
            out.extend_from_slice(&value);
            out
        }
        None => vec![0x00],
    }
}

pub fn rulebook_reference_bytes_v0(
    rulebook_reference: &RulebookReferenceV0,
) -> Result<Vec<u8>, AdmissionCryptoError> {
    let mut out = Vec::new();
    push_id(&mut out, rulebook_reference.rulebook_id)?;
    push_ascii(
        &mut out,
        &rulebook_reference.rulebook_version,
        "rulebook_version",
    )?;
    out.extend_from_slice(&rulebook_reference.rulebook_hash);
    Ok(out)
}

pub fn admission_authorization_reference_v0(
    admission_profile_version: &str,
    sponsor_identity_id: Uuid,
    capacity_period_id: Uuid,
    rulebook_reference: &RulebookReferenceV0,
) -> Result<[u8; 32], AdmissionCryptoError> {
    if admission_profile_version != PROFILE_V0_ADMISSION_PROFILE {
        return Err(AdmissionCryptoError::new(
            "unsupported_admission_profile",
            "unsupported Profile-v0 admission profile",
        ));
    }
    let mut bytes = Vec::new();
    push_ascii(&mut bytes, ADMISSION_AUTHORIZATION_DOMAIN, "domain")?;
    push_ascii(
        &mut bytes,
        admission_profile_version,
        "admission_profile_version",
    )?;
    push_id(&mut bytes, sponsor_identity_id)?;
    push_id(&mut bytes, capacity_period_id)?;
    bytes.extend_from_slice(&rulebook_reference_bytes_v0(rulebook_reference)?);
    hash32(&bytes)
}

pub fn initial_key_possession_bytes_v0(
    input: &InitialKeyPossessionInputV0<'_>,
) -> Result<Vec<u8>, AdmissionCryptoError> {
    if input.admission_profile_version != PROFILE_V0_ADMISSION_PROFILE {
        return Err(AdmissionCryptoError::new(
            "unsupported_admission_profile",
            "unsupported Profile-v0 admission profile",
        ));
    }
    let mut out = Vec::new();
    push_ascii(&mut out, INITIAL_KEY_POSSESSION_DOMAIN, "domain")?;
    push_id(&mut out, input.identity_create_event_id)?;
    push_id(&mut out, input.target_identity_id)?;
    push_ascii(
        &mut out,
        input.admission_profile_version,
        "admission_profile_version",
    )?;
    out.extend_from_slice(&key_descriptor_bytes_from_descriptor_v0(
        input.initial_key_descriptor,
    )?);
    out.extend_from_slice(&input.initial_public_key_ref);
    push_id(&mut out, input.sponsor_identity_id)?;
    out.extend_from_slice(&input.admission_authorization_reference);
    out.extend_from_slice(&optional_hash32_bytes_v0(input.verification_reference));
    out.extend_from_slice(&input.identity_structural_root_plan.canonical_bytes()?);
    Ok(out)
}

pub fn replacement_key_possession_bytes_v0(
    input: &ReplacementKeyPossessionInputV0<'_>,
) -> Result<Vec<u8>, AdmissionCryptoError> {
    let mut out = Vec::new();
    push_ascii(&mut out, REPLACEMENT_KEY_POSSESSION_DOMAIN, "domain")?;
    push_id(&mut out, input.identity_key_rotate_event_id)?;
    push_id(&mut out, input.identity_id)?;
    out.extend_from_slice(&input.authorizing_public_key_ref);
    out.extend_from_slice(&key_descriptor_bytes_from_descriptor_v0(
        input.replacement_key_descriptor,
    )?);
    out.extend_from_slice(&input.replacement_public_key_ref);
    Ok(out)
}

pub fn verify_initial_key_possession_proof_v0(
    input: &InitialKeyPossessionInputV0<'_>,
    proof: &[u8],
) -> Result<(), AdmissionCryptoError> {
    let expected_reference = public_key_ref_from_descriptor_v0(input.initial_key_descriptor)?;
    if expected_reference != input.initial_public_key_ref {
        return Err(AdmissionCryptoError::new(
            "public_key_ref_mismatch",
            "initial_public_key_ref does not match the descriptor",
        ));
    }
    verify_possession_proof(
        &initial_key_possession_bytes_v0(input)?,
        &input.initial_key_descriptor.raw_public_key_bytes,
        proof,
    )
}

pub fn verify_replacement_key_possession_proof_v0(
    input: &ReplacementKeyPossessionInputV0<'_>,
    proof: &[u8],
) -> Result<(), AdmissionCryptoError> {
    let expected_reference = public_key_ref_from_descriptor_v0(input.replacement_key_descriptor)?;
    if expected_reference != input.replacement_public_key_ref {
        return Err(AdmissionCryptoError::new(
            "public_key_ref_mismatch",
            "replacement_public_key_ref does not match the descriptor",
        ));
    }
    verify_possession_proof(
        &replacement_key_possession_bytes_v0(input)?,
        &input.replacement_key_descriptor.raw_public_key_bytes,
        proof,
    )
}

pub fn validate_identity_structural_root_plan_v0(
    plan: &IdentityStructuralRootPlanV0,
) -> Result<(), AdmissionCryptoError> {
    let mut ids = BTreeSet::new();
    for (index, root) in plan.roots.iter().enumerate() {
        if root.role != IdentityStructuralRootRoleV0::ORDERED[index] {
            return Err(AdmissionCryptoError::new(
                "incomplete_identity_structural_roots",
                "identity structural roots must use the fixed Profile-v0 order",
            ));
        }
        validate_uuid_v7(root.idea_id, "identity_structural_roots idea_id")?;
        if !ids.insert(root.idea_id) {
            return Err(AdmissionCryptoError::new(
                "incomplete_identity_structural_roots",
                "identity structural-root idea IDs must be distinct",
            ));
        }
    }
    for connection_id in &plan.membership_connection_ids {
        validate_uuid_v7(
            *connection_id,
            "identity_structural_root_membership_connection_ids",
        )?;
        if !ids.insert(*connection_id) {
            return Err(AdmissionCryptoError::new(
                "incomplete_identity_structural_roots",
                "identity structural-root IDs must be pairwise distinct",
            ));
        }
    }
    Ok(())
}

fn verify_possession_proof(
    message: &[u8],
    raw_public_key_bytes: &[u8],
    proof: &[u8],
) -> Result<(), AdmissionCryptoError> {
    if raw_public_key_bytes.len() != 32 {
        return Err(AdmissionCryptoError::new(
            "malformed_key_descriptor",
            "Profile-v0 public keys must be exactly 32 bytes",
        ));
    }
    if proof.len() != 64 {
        return Err(AdmissionCryptoError::new(
            "invalid_possession_proof",
            "Profile-v0 possession proofs must be exactly 64 bytes",
        ));
    }
    let mut public_key = [0_u8; 32];
    public_key.copy_from_slice(raw_public_key_bytes);
    let verifying_key = VerifyingKey::from_bytes(&public_key).map_err(|_| {
        AdmissionCryptoError::new("malformed_key_descriptor", "malformed Ed25519 public key")
    })?;
    let mut signature_bytes = [0_u8; 64];
    signature_bytes.copy_from_slice(proof);
    verifying_key
        .verify_strict(message, &Signature::from_bytes(&signature_bytes))
        .map_err(|_| {
            AdmissionCryptoError::new(
                "possession_proof_binding_mismatch",
                "possession proof does not bind the reconstructed Profile-v0 message",
            )
        })
}

fn hash32(bytes: &[u8]) -> Result<[u8; 32], AdmissionCryptoError> {
    let hash = hash_bytes(bytes);
    hash.try_into().map_err(|_| {
        AdmissionCryptoError::new("invalid_hash", "BLAKE3-256 must produce exactly 32 bytes")
    })
}

fn push_ascii(out: &mut Vec<u8>, value: &str, field: &str) -> Result<(), AdmissionCryptoError> {
    if !value.is_ascii() {
        return Err(AdmissionCryptoError::new(
            "malformed_identity_create_payload",
            format!("{field} must be ASCII"),
        ));
    }
    out.extend_from_slice(&encode_u32(value.len() as u32));
    out.extend_from_slice(value.as_bytes());
    Ok(())
}

fn push_id(out: &mut Vec<u8>, value: Uuid) -> Result<(), AdmissionCryptoError> {
    let encoded = encode_id(&value.to_string()).map_err(|error| {
        AdmissionCryptoError::new("invalid_id", format!("invalid UUIDv7: {error}"))
    })?;
    out.extend_from_slice(&encoded);
    Ok(())
}

fn validate_uuid_v7(value: Uuid, field: &str) -> Result<(), AdmissionCryptoError> {
    validate_id(&value.to_string()).map_err(|error| {
        AdmissionCryptoError::new("invalid_id", format!("{field} must be UUIDv7: {error}"))
    })
}
