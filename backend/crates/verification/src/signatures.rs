use ed25519_dalek::{Signature, VerifyingKey};
use encoding::canonical::{canonicalize_string, encode_id, encode_u32, encode_u8};
use encoding::hash::hash_bytes;
use encoding::payload::to_hex;
use uuid::Uuid;

pub const SIGNATURE_PROFILE_ED25519_V0: &str = "ed25519_v0";
pub const SIGNATURE_ALGORITHM_ED25519: &str = "ed25519";
pub const PAYLOAD_BINDING_EMBEDDED: &str = "embedded_payload";
pub const PAYLOAD_BINDING_REF: &str = "payload_ref";

const SIGNED_CANDIDATE_DOMAIN: &str = "seed.canonical_event.authored_candidate.v0";
const KEY_DESCRIPTOR_DOMAIN: &str = "seed.identity.public_key_descriptor.v0";
const CANDIDATE_HASH_DOMAIN: &str = "seed.canonical_event.authored_candidate_hash.v0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureValidationError {
    pub code: &'static str,
    pub message: String,
}

impl SignatureValidationError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for SignatureValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for SignatureValidationError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoredEventCandidate {
    pub signature_profile: String,
    pub event_id: Uuid,
    pub event_type: String,
    pub author_identity_id: Uuid,
    pub speaker_identity_id: Option<Uuid>,
    pub public_key_ref: String,
    pub payload_hash: String,
    pub payload_binding_mode: String,
    pub payload_ref: Option<Vec<u8>>,
    pub author_observed_at: Option<String>,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyDescriptorV0 {
    pub key_profile_version: String,
    pub signature_algorithm: String,
    pub raw_public_key_bytes: Vec<u8>,
    pub owning_identity_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedCandidateV0 {
    pub signed_candidate_bytes_v0: Vec<u8>,
    pub authored_candidate_hash_v0: String,
    pub public_key_ref: String,
}

pub fn key_descriptor_bytes_v0(
    raw_public_key_bytes: &[u8],
    owning_identity_id: Uuid,
) -> Result<Vec<u8>, SignatureValidationError> {
    if raw_public_key_bytes.len() != 32 {
        return Err(SignatureValidationError::new(
            "malformed_public_key",
            "Profile-v0 public keys must be exactly 32 bytes",
        ));
    }
    let mut out = Vec::new();
    push_ascii(
        &mut out,
        SIGNATURE_PROFILE_ED25519_V0,
        "key_profile_version",
    )?;
    push_ascii(&mut out, SIGNATURE_ALGORITHM_ED25519, "signature_algorithm")?;
    out.extend_from_slice(raw_public_key_bytes);
    push_id(&mut out, owning_identity_id)?;
    Ok(out)
}

pub fn public_key_ref_v0(
    raw_public_key_bytes: &[u8],
    owning_identity_id: Uuid,
) -> Result<String, SignatureValidationError> {
    let descriptor = key_descriptor_bytes_v0(raw_public_key_bytes, owning_identity_id)?;
    let mut bytes = Vec::new();
    push_ascii(&mut bytes, KEY_DESCRIPTOR_DOMAIN, "domain")?;
    bytes.extend_from_slice(&descriptor);
    Ok(to_hex(&hash_bytes(&bytes)))
}

pub fn signed_candidate_bytes_v0(
    candidate: &AuthoredEventCandidate,
) -> Result<Vec<u8>, SignatureValidationError> {
    if candidate.signature_profile != SIGNATURE_PROFILE_ED25519_V0 {
        return Err(SignatureValidationError::new(
            "unsupported_signature_profile",
            "unsupported signature_profile",
        ));
    }
    if candidate.payload_binding_mode != PAYLOAD_BINDING_EMBEDDED
        && candidate.payload_binding_mode != PAYLOAD_BINDING_REF
    {
        return Err(SignatureValidationError::new(
            "invalid_payload_binding_mode",
            "payload_binding_mode must be embedded_payload or payload_ref",
        ));
    }

    let public_key_ref = decode_hash32(&candidate.public_key_ref, "public_key_ref")?;
    let payload_hash = decode_hash32(&candidate.payload_hash, "payload_hash")?;

    let mut out = Vec::new();
    push_ascii(&mut out, SIGNED_CANDIDATE_DOMAIN, "domain")?;
    push_ascii(&mut out, &candidate.signature_profile, "signature_profile")?;
    push_id(&mut out, candidate.event_id)?;
    push_ascii(&mut out, &candidate.event_type, "event_type")?;
    push_id(&mut out, candidate.author_identity_id)?;
    push_optional_id(&mut out, candidate.speaker_identity_id)?;
    out.extend_from_slice(&public_key_ref);
    out.extend_from_slice(&payload_hash);
    push_ascii(
        &mut out,
        &candidate.payload_binding_mode,
        "payload_binding_mode",
    )?;
    push_optional_bytes(&mut out, candidate.payload_ref.as_deref())?;
    push_optional_text(&mut out, candidate.author_observed_at.as_deref())?;
    Ok(out)
}

pub fn authored_candidate_hash_v0(
    signed_candidate_bytes_v0: &[u8],
    signature: &[u8],
) -> Result<String, SignatureValidationError> {
    if signature.len() != 64 {
        return Err(SignatureValidationError::new(
            "malformed_signature",
            "Profile-v0 signatures must be exactly 64 bytes",
        ));
    }
    let mut bytes = Vec::new();
    push_ascii(&mut bytes, CANDIDATE_HASH_DOMAIN, "domain")?;
    bytes.extend_from_slice(signed_candidate_bytes_v0);
    bytes.extend_from_slice(signature);
    Ok(to_hex(&hash_bytes(&bytes)))
}

pub fn verify_ed25519_v0(
    candidate: &AuthoredEventCandidate,
    raw_public_key_bytes: &[u8],
) -> Result<VerifiedCandidateV0, SignatureValidationError> {
    if raw_public_key_bytes.len() != 32 {
        return Err(SignatureValidationError::new(
            "malformed_public_key",
            "Profile-v0 public keys must be exactly 32 bytes",
        ));
    }
    let expected_ref = public_key_ref_v0(raw_public_key_bytes, candidate.author_identity_id)?;
    if candidate.public_key_ref != expected_ref {
        return Err(SignatureValidationError::new(
            "wrong_key_owner",
            "public_key_ref does not match the author-owned key descriptor",
        ));
    }

    let signature_bytes = decode_signature64(&candidate.signature)?;
    let mut public_key = [0_u8; 32];
    public_key.copy_from_slice(raw_public_key_bytes);
    let verifying_key = VerifyingKey::from_bytes(&public_key).map_err(|_| {
        SignatureValidationError::new("malformed_public_key", "malformed Ed25519 public key")
    })?;
    let signature = Signature::from_bytes(&signature_bytes);
    let signed_bytes = signed_candidate_bytes_v0(candidate)?;
    verifying_key
        .verify_strict(&signed_bytes, &signature)
        .map_err(|_| SignatureValidationError::new("invalid_signature", "invalid signature"))?;
    let candidate_hash = authored_candidate_hash_v0(&signed_bytes, &signature_bytes)?;

    Ok(VerifiedCandidateV0 {
        signed_candidate_bytes_v0: signed_bytes,
        authored_candidate_hash_v0: candidate_hash,
        public_key_ref: expected_ref,
    })
}

pub fn decode_hash32(
    value: &str,
    field: &'static str,
) -> Result<[u8; 32], SignatureValidationError> {
    let bytes = decode_hex_exact(value, 32, field)?;
    let mut out = [0_u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

pub fn decode_signature64(value: &str) -> Result<[u8; 64], SignatureValidationError> {
    let bytes = decode_hex_exact(value, 64, "signature")?;
    let mut out = [0_u8; 64];
    out.copy_from_slice(&bytes);
    Ok(out)
}

pub fn decode_public_key32(value: &str) -> Result<[u8; 32], SignatureValidationError> {
    let bytes = decode_hex_exact(value, 32, "public_key")?;
    let mut out = [0_u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

fn decode_hex_exact(
    value: &str,
    expected_len: usize,
    field: &'static str,
) -> Result<Vec<u8>, SignatureValidationError> {
    if value.len() != expected_len * 2 {
        return Err(SignatureValidationError::new(
            hex_error_code(field),
            format!(
                "{field} must be {} lowercase hex characters",
                expected_len * 2
            ),
        ));
    }
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(expected_len);
    let mut idx = 0;
    while idx < bytes.len() {
        let high = hex_value(bytes[idx]).ok_or_else(|| {
            SignatureValidationError::new(
                hex_error_code(field),
                format!("{field} must use lowercase hex"),
            )
        })?;
        let low = hex_value(bytes[idx + 1]).ok_or_else(|| {
            SignatureValidationError::new(
                hex_error_code(field),
                format!("{field} must use lowercase hex"),
            )
        })?;
        out.push((high << 4) | low);
        idx += 2;
    }
    Ok(out)
}

fn hex_error_code(field: &str) -> &'static str {
    match field {
        "signature" => "malformed_signature",
        "public_key" => "malformed_public_key",
        "public_key_ref" => "malformed_public_key_ref",
        _ => "invalid_hash",
    }
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

fn push_ascii(
    out: &mut Vec<u8>,
    value: &str,
    field: &'static str,
) -> Result<(), SignatureValidationError> {
    if !value.is_ascii() {
        return Err(SignatureValidationError::new(
            "invalid_field",
            format!("{field} must be ASCII"),
        ));
    }
    out.extend_from_slice(&encode_u32(value.len() as u32));
    out.extend_from_slice(value.as_bytes());
    Ok(())
}

fn push_id(out: &mut Vec<u8>, value: Uuid) -> Result<(), SignatureValidationError> {
    let encoded = encode_id(&value.to_string()).map_err(|err| {
        SignatureValidationError::new("invalid_id", format!("invalid UUIDv7: {err}"))
    })?;
    out.extend_from_slice(&encoded);
    Ok(())
}

fn push_optional_id(
    out: &mut Vec<u8>,
    value: Option<Uuid>,
) -> Result<(), SignatureValidationError> {
    match value {
        Some(value) => {
            out.extend_from_slice(&encode_u8(1));
            push_id(out, value)?;
        }
        None => out.extend_from_slice(&encode_u8(0)),
    }
    Ok(())
}

fn push_optional_bytes(
    out: &mut Vec<u8>,
    value: Option<&[u8]>,
) -> Result<(), SignatureValidationError> {
    match value {
        Some(value) => {
            out.extend_from_slice(&encode_u8(1));
            out.extend_from_slice(&encode_u32(value.len() as u32));
            out.extend_from_slice(value);
        }
        None => out.extend_from_slice(&encode_u8(0)),
    }
    Ok(())
}

fn push_optional_text(
    out: &mut Vec<u8>,
    value: Option<&str>,
) -> Result<(), SignatureValidationError> {
    match value {
        Some(value) => {
            out.extend_from_slice(&encode_u8(1));
            let bytes = canonicalize_string(value).map_err(|err| {
                SignatureValidationError::new("invalid_field", format!("invalid text: {err}"))
            })?;
            out.extend_from_slice(&encode_u32(bytes.len() as u32));
            out.extend_from_slice(&bytes);
        }
        None => out.extend_from_slice(&encode_u8(0)),
    }
    Ok(())
}

pub fn verify_signature(message: &[u8], public_key: &[u8], signature: &[u8]) -> bool {
    if public_key.len() != 32 || signature.len() != 64 {
        return false;
    }
    let mut public_key_bytes = [0_u8; 32];
    public_key_bytes.copy_from_slice(public_key);
    let Ok(verifying_key) = VerifyingKey::from_bytes(&public_key_bytes) else {
        return false;
    };
    let mut signature_bytes = [0_u8; 64];
    signature_bytes.copy_from_slice(signature);
    let signature = Signature::from_bytes(&signature_bytes);
    verifying_key.verify_strict(message, &signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use encoding::payload::{canonical_json_payload_bytes, canonical_json_payload_hash_hex};
    use serde_json::Value;
    use std::collections::BTreeSet;

    fn sample_candidate(signature: String, public_key_ref: String) -> AuthoredEventCandidate {
        AuthoredEventCandidate {
            signature_profile: SIGNATURE_PROFILE_ED25519_V0.to_string(),
            event_id: Uuid::parse_str("00000000-0000-7000-8000-000000000101").unwrap(),
            event_type: "idea_create".to_string(),
            author_identity_id: Uuid::parse_str("00000000-0000-7000-8000-00000000a001").unwrap(),
            speaker_identity_id: Some(
                Uuid::parse_str("00000000-0000-7000-8000-00000000a001").unwrap(),
            ),
            public_key_ref,
            payload_hash: to_hex(&hash_bytes(b"payload")),
            payload_binding_mode: PAYLOAD_BINDING_EMBEDDED.to_string(),
            payload_ref: None,
            author_observed_at: None,
            signature,
        }
    }

    #[test]
    fn public_key_ref_is_stable_for_descriptor() {
        let owner = Uuid::parse_str("00000000-0000-7000-8000-00000000a001").unwrap();
        let key = [7_u8; 32];
        let reference = public_key_ref_v0(&key, owner).expect("reference");
        assert_eq!(reference.len(), 64);
        assert_eq!(reference, public_key_ref_v0(&key, owner).unwrap());
    }

    #[test]
    fn valid_profile_v0_signature_verifies() {
        let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
        let public_key = signing_key.verifying_key();
        let public_key_ref =
            public_key_ref_v0(public_key.as_bytes(), sample_owner()).expect("public key ref");
        let unsigned = sample_candidate(String::new(), public_key_ref);
        let signed_bytes = signed_candidate_bytes_v0(&unsigned).expect("signed bytes");
        let signature = signing_key.sign(&signed_bytes);
        let candidate = AuthoredEventCandidate {
            signature: to_hex(&signature.to_bytes()),
            ..unsigned
        };

        let verified = verify_ed25519_v0(&candidate, public_key.as_bytes()).expect("verified");
        assert_eq!(verified.signed_candidate_bytes_v0, signed_bytes);
        assert_eq!(verified.authored_candidate_hash_v0.len(), 64);
    }

    #[test]
    fn altered_event_type_rejects_signature() {
        let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
        let public_key = signing_key.verifying_key();
        let public_key_ref =
            public_key_ref_v0(public_key.as_bytes(), sample_owner()).expect("public key ref");
        let unsigned = sample_candidate(String::new(), public_key_ref);
        let signed_bytes = signed_candidate_bytes_v0(&unsigned).expect("signed bytes");
        let signature = signing_key.sign(&signed_bytes);
        let mut candidate = AuthoredEventCandidate {
            signature: to_hex(&signature.to_bytes()),
            ..unsigned
        };
        candidate.event_type = "connection_create".to_string();

        let err = verify_ed25519_v0(&candidate, public_key.as_bytes()).expect_err("should reject");
        assert_eq!(err.code, "invalid_signature");
    }

    #[test]
    fn malformed_lengths_are_rejected() {
        let mut candidate = sample_candidate("00".repeat(63), "00".repeat(32));
        let err = decode_signature64(&candidate.signature).expect_err("short signature");
        assert_eq!(err.code, "malformed_signature");

        candidate.signature_profile = "ed25519ctx_v0".to_string();
        let err = signed_candidate_bytes_v0(&candidate).expect_err("unsupported");
        assert_eq!(err.code, "unsupported_signature_profile");

        let err = decode_public_key32(&"00".repeat(31)).expect_err("short key");
        assert_eq!(err.code, "malformed_public_key");
    }

    fn sample_owner() -> Uuid {
        Uuid::parse_str("00000000-0000-7000-8000-00000000a001").unwrap()
    }

    #[test]
    fn profile_v0_conformance_vectors_match_runtime() {
        let fixture: Value = serde_json::from_str(include_str!(
            "../../../../docs/conformance/canonical-event-signature-profile-v0.vectors.json"
        ))
        .expect("signature vector JSON");
        assert_eq!(
            fixture.get("schema").and_then(Value::as_str),
            Some("seed.conformance.canonical_event_signature_profile_v0.v1")
        );
        let vectors = fixture
            .get("vectors")
            .and_then(Value::as_array)
            .expect("vectors");
        let mut ids = BTreeSet::new();
        for vector in vectors {
            let id = required_str(vector, "id");
            assert!(ids.insert(id.to_string()), "duplicate vector id {id}");
            let candidate = candidate_from_vector(vector);
            let canonical_payload = vector
                .get("canonical_payload")
                .expect("canonical_payload")
                .clone();
            let payload_bytes =
                canonical_json_payload_bytes(&canonical_payload).expect("payload bytes");
            assert_eq!(
                to_hex(&payload_bytes),
                required_str(vector, "canonical_payload_bytes_hex"),
                "payload bytes mismatch for {id}"
            );

            let expected_payload_hash =
                canonical_json_payload_hash_hex(&canonical_payload).expect("payload hash");
            if id != "altered_payload_hash" {
                assert_eq!(
                    candidate.payload_hash, expected_payload_hash,
                    "payload hash mismatch for {id}"
                );
            } else {
                assert_ne!(
                    candidate.payload_hash, expected_payload_hash,
                    "altered_payload_hash vector must carry a mismatched payload hash"
                );
            }

            match vector.get("signed_candidate_bytes_v0") {
                Some(Value::String(expected_hex)) => {
                    let actual = signed_candidate_bytes_v0(&candidate)
                        .unwrap_or_else(|err| panic!("{id} signed bytes error: {err}"));
                    assert_eq!(to_hex(&actual), *expected_hex, "signed bytes for {id}");
                }
                Some(Value::Null) => {
                    assert!(
                        signed_candidate_bytes_v0(&candidate).is_err(),
                        "{id} should not produce signed bytes"
                    );
                }
                _ => panic!("{id} missing signed_candidate_bytes_v0"),
            }

            assert_eq!(
                required_str(vector, "public_key_ref"),
                candidate.public_key_ref,
                "public_key_ref mismatch for {id}"
            );

            let public_key_result = decode_public_key32(required_str(vector, "raw_public_key"));
            let expected_error = vector
                .pointer("/expected/error_code")
                .and_then(Value::as_str);
            match expected_error {
                Some("malformed_public_key") => {
                    let err = public_key_result.expect_err("malformed public key");
                    assert_eq!(err.code, "malformed_public_key");
                    continue;
                }
                _ => {}
            }
            let public_key = public_key_result.expect("public key");

            let verification = verify_ed25519_v0(&candidate, &public_key);
            match expected_error {
                Some("invalid_signature") if id == "event_index_in_signed_bytes_attempt" => {
                    verification.unwrap_or_else(|err| {
                        panic!("{id} candidate should verify; nonconforming event_index bytes are checked separately: {err}")
                    });
                }
                None => {
                    let verified = verification.unwrap_or_else(|err| {
                        panic!("{id} expected accepted but verification failed: {err}")
                    });
                    assert_eq!(
                        Some(verified.authored_candidate_hash_v0.as_str()),
                        vector
                            .get("authored_candidate_hash_v0")
                            .and_then(Value::as_str),
                        "candidate hash for {id}"
                    );
                }
                Some(
                    "invalid_signature"
                    | "wrong_key_owner"
                    | "malformed_signature"
                    | "unsupported_signature_profile",
                ) => {
                    let err = verification.expect_err("expected signature-level rejection");
                    assert_eq!(err.code, expected_error.unwrap(), "error for {id}");
                }
                Some("unknown_key" | "revoked_key" | "publication_wrapper_candidate_mismatch") => {
                    verification.unwrap_or_else(|err| {
                        panic!("{id} should pass cryptographic verification before external rejection: {err}")
                    });
                }
                Some(other) => panic!("{id} unexpected expected error code {other}"),
            }
        }

        assert!(ids.contains("valid_ed25519_idea_create"));
        assert!(ids.contains("event_index_in_signed_bytes_attempt"));
        let wrapper = fixture
            .get("publication_wrapper_checks")
            .expect("publication_wrapper_checks");
        let assigned_bytes = required_str(wrapper, "signed_bytes_after_assigned_position");
        let with_event_index =
            required_str(wrapper, "nonconforming_signed_bytes_with_event_index_hex");
        assert_ne!(
            assigned_bytes, with_event_index,
            "publication event_index must not be part of signed candidate bytes"
        );
        let valid = vectors
            .iter()
            .find(|vector| required_str(vector, "id") == "valid_ed25519_idea_create")
            .expect("valid vector");
        assert_eq!(
            assigned_bytes,
            required_str(valid, "signed_candidate_bytes_v0"),
            "assigned publication position must not mutate signed bytes"
        );
    }

    fn required_str<'a>(value: &'a Value, key: &str) -> &'a str {
        value
            .get(key)
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("missing string field {key}"))
    }

    fn optional_str<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
        value.get(key).and_then(Value::as_str)
    }

    fn candidate_from_vector(vector: &Value) -> AuthoredEventCandidate {
        let value = vector.get("candidate").expect("candidate");
        AuthoredEventCandidate {
            signature_profile: required_str(value, "signature_profile").to_string(),
            event_id: Uuid::parse_str(required_str(value, "event_id")).expect("event_id"),
            event_type: required_str(value, "event_type").to_string(),
            author_identity_id: Uuid::parse_str(required_str(value, "author_identity_id"))
                .expect("author_identity_id"),
            speaker_identity_id: optional_str(value, "speaker_identity_id")
                .map(|speaker| Uuid::parse_str(speaker).expect("speaker_identity_id")),
            public_key_ref: required_str(value, "public_key_ref").to_string(),
            payload_hash: required_str(value, "payload_hash").to_string(),
            payload_binding_mode: required_str(value, "payload_binding_mode").to_string(),
            payload_ref: None,
            author_observed_at: optional_str(value, "author_observed_at").map(str::to_string),
            signature: required_str(value, "signature").to_string(),
        }
    }
}
