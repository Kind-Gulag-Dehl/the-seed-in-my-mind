---
doc_id: canonical_event_authorship_signature_profile_v0
title: Canonical Event Authorship and Signature Profile v0
status: authoritative
version: v0.1
last_reviewed: 2026-07-11
authority:
  subordinate_to:
    - docs/protocol v5.md Section 0
    - docs/canonical-encoding-and-hashing-spec.md
    - docs/deterministic-replay-and-merge-spec.md
    - docs/protocol v5-appendix-a.md
  authoritative_for:
    - authored canonical event candidates
    - signature_profile ed25519_v0
    - canonical signed bytes for human-authored events
    - public_key_ref descriptor hashing
    - replay-derived identity key state
    - authorship-signature conformance-vector requirements
  not_authoritative_for:
    - private-key storage
    - wallet user experience
    - private account recovery
    - session authentication
    - mechanical boundary-event signatures
---

# Canonical Event Authorship and Signature Profile v0

## 0. Purpose and Authority

This document is the canonical home for ordinary human-authored canonical event signatures in Signature Profile v0.

It defines:

- the authored event candidate signed by a human;
- the published canonical event wrapper that binds that signed candidate into canonical order;
- `signature_profile = ed25519_v0`;
- the exact signed-byte layout for Profile v0;
- deterministic `public_key_ref` construction;
- replay-derived identity key state;
- authorship-signature conformance-vector requirements.

This document composes with, and does not replace:

- Protocol v5 Section 0 for constitutional invariants;
- the Canonical Encoding and Hashing Specification for primitive encodings, UUIDv7 `id` encoding, hash32 bytes, canonical text, and canonical payload hashing;
- the Deterministic Replay and Merge Specification for prefix validation and replay;
- the POD Consensus and Canonical Publication Specification for publication finality and finalized prefix certificates;
- Protocol v5 Appendix A for event-family schemas.

Where another document uses older signature terminology, this document supplies the normative mapping in Section 10.

## 1. Submission and Finality Stages

Canonical authorship has these stages:

1. `local_draft`
2. `signed_authored_candidate`
3. `validated_candidate`
4. `finalized_publication`
5. `canonical_event`
6. `deterministic_replay`

A valid signature makes a candidate attributable. It does not make the candidate canonical.

Canonical status begins only when the exact signed authored candidate is included in a valid finalized publication under the active publication profile.

Canonical order is determined only by finalized publication order and finalized prefix certificates. Canonical order is never determined by:

- UUID ordering;
- author-observed timestamps;
- local receipt order;
- signature generation time;
- private database insertion order;
- node-local scheduler order.

## 2. Two-Layer Canonical Event Model

### 2.1 Authored Event Candidate

An authored event candidate is the structure created and signed by the human before canonical publication.

It contains only fields known and controlled at authorship time:

| Field | Required | Canonical term | Notes |
|---|---:|---|---|
| Signature profile | yes | `signature_profile` | For Profile v0 this value is exactly `ed25519_v0`. |
| Event identifier | yes | `event_id` | Canonical UUIDv7 `id` encoding. It identifies the candidate and does not define order. |
| Event kind | yes | `event_type` | Canonical event-family name from Appendix A or the event registry. |
| Author identity | yes | `author_identity_id` | Canonical identity that signs and is responsible for the event. |
| Speaker identity | conditional | `speaker_identity_id` | Present only when the schema permits or requires a speaker distinct from the author. |
| Signing key reference | yes | `public_key_ref` | Hash32 of the key descriptor defined in Section 5. |
| Payload hash | yes | `payload_hash` | Hash32 of the canonical payload bytes under the applicable event schema. |
| Payload binding mode | yes | `payload_binding_mode` | Either `embedded_payload` or `payload_ref`. |
| Payload reference | conditional | `payload_ref` | Required only when `payload_binding_mode = payload_ref`; encoded as the canonical payload-reference bytes defined by the event schema. |
| Author-observed wall-clock value | optional | `author_observed_at` | Non-semantic human-observed value when an event schema requires or permits one. It never determines order, validity window, or cycle position. |
| Signature | yes | `signature` | Raw Ed25519 signature encoded as specified in Section 4. It is not included in the signed bytes. |

The payload itself may be carried with the candidate or retrieved through a canonical payload reference according to the event schema. In both cases, validation recomputes `payload_hash` from the canonical payload bytes before signature acceptance.

### 2.2 Published Canonical Event

A published canonical event wraps or references the exact signed authored candidate and adds publication-derived data.

Publication-derived fields may include:

- finalized canonical sequence position;
- exposed `(block_height, event_index)` address;
- finalized-prefix-certificate reference;
- active rulebook reference set;
- safety or payload classification;
- canonical publication chain references;
- node- or publication-derived metadata required by an authoritative publication profile.

Publication-derived fields are not part of the ordinary human-authorship signature because they do not exist when the human signs the candidate.

The finalized publication certificate and its commitments bind the exact signed candidate bytes to canonical order. A node MUST reject a publication artifact that:

- alters the embedded signed candidate;
- alters the payload while preserving the old payload hash;
- associates a signature with different candidate bytes;
- claims a canonical position for bytes other than the committed signed candidate bytes.

## 3. Signature Profile v0

Signature Profile v0 is exactly:

| Parameter | Value |
|---|---|
| `signature_profile` | `ed25519_v0` |
| Algorithm | Ed25519 |
| Ed25519 variant | RFC 8032 Ed25519, without Ed25519ph or Ed25519ctx |
| Public key length | 32 raw bytes |
| Signature length | 64 raw bytes |
| Canonical public key representation | raw 32 bytes in canonical bytes; lowercase hex in JSON/test-vector surfaces |
| Canonical signature representation | raw 64 bytes in validation; lowercase hex in JSON/test-vector surfaces |

Conforming Profile-v0 implementations MUST NOT select another algorithm locally or by implementation preference.

Future signature profiles MAY add other algorithms only through explicit versioned specification changes. An implementation that receives an unsupported `signature_profile` MUST reject the authored candidate for Profile-v0 validation.

Malformed public keys and signatures are rejected. No normalization, alternate encodings, compressed alternatives, multibase prefixes, PEM wrappers, whitespace-tolerant decoding, or implementation-local key formats are permitted in canonical validation.

Verification succeeds only when Ed25519 verification over the exact Profile-v0 signed bytes succeeds for the raw public key identified by `public_key_ref` and that key is active for `author_identity_id` at the applicable publication point.

## 4. Canonical Signed Bytes

### 4.1 Primitive Encodings Used Here

This section uses primitive encodings from the Canonical Encoding and Hashing Specification. For clarity, Profile v0 fixes the following field encodings:

- `u8(x)`: one unsigned byte.
- `u32be(n)`: four-byte unsigned big-endian integer.
- `ascii(s)`: `u32be(byte_length(s)) || ASCII_BYTES(s)`. Non-ASCII strings are invalid in fields declared as `ascii`.
- `text(s)`: the canonical text bytes defined by the Canonical Encoding and Hashing Specification, preceded by `u32be(byte_length)`.
- `id(x)`: the canonical UUIDv7 identifier encoding defined by the Canonical Encoding and Hashing Specification: `u32be(36) || ASCII_BYTES(canonical_uuidv7_string)`.
- `hash32(h)`: exactly 32 raw bytes.
- `bytes(b)`: `u32be(byte_length(b)) || b`.
- `bytes32(b)`: exactly 32 raw bytes.
- `bytes64(b)`: exactly 64 raw bytes.
- `opt_none`: `u8(0)`.
- `opt_some`: `u8(1)` followed by the field encoding.

The fixed signed-candidate domain tag is:

`seed.canonical_event.authored_candidate.v0`

The fixed key-descriptor hash domain tag is:

`seed.identity.public_key_descriptor.v0`

The fixed authored-candidate hash domain tag is:

`seed.canonical_event.authored_candidate_hash.v0`

### 4.2 Profile-v0 Signed-Byte Layout

The Profile-v0 signed bytes for an authored event candidate are exactly:

```text
signed_candidate_bytes_v0 =
    ascii("seed.canonical_event.authored_candidate.v0")
 || ascii(signature_profile)
 || id(event_id)
 || ascii(event_type)
 || id(author_identity_id)
 || optional_id(speaker_identity_id)
 || hash32(public_key_ref)
 || hash32(payload_hash)
 || ascii(payload_binding_mode)
 || optional_bytes(payload_ref)
 || optional_text(author_observed_at)
```

Where:

```text
optional_id(x) = opt_none when absent
optional_id(x) = opt_some || id(x) when present

optional_text(x) = opt_none when absent
optional_text(x) = opt_some || text(x) when present

optional_bytes(x) = opt_none when absent
optional_bytes(x) = opt_some || bytes(x) when present
```

The signature field itself is excluded from `signed_candidate_bytes_v0`.

The signed bytes include every candidate field whose mutation would change the human-authored assertion or the key used to verify it. They include the profile identifier, event kind, author identity, signing key reference, payload hash, payload binding mode, and optional speaker, payload reference, and author-observed time fields.

### 4.3 Fields Excluded From Human-Signed Bytes

The human-authorship signature MUST NOT include:

- canonical event index;
- block height;
- cycle index;
- finalized-prefix-certificate data;
- canonical publication position;
- node-local receipt time;
- database identifiers;
- private account or session information;
- node-local safety, cache, or operational metadata;
- publication wrapper fields.

Any document that requires the ordinary human signature to bind a future publication-assigned `event_index`, block height, prefix certificate, or canonical position is superseded by this document.

### 4.4 Candidate Hash

When a candidate hash is required for conformance or publication commitments, it is:

```text
authored_candidate_hash_v0 =
    HASH(
        ascii("seed.canonical_event.authored_candidate_hash.v0")
     || signed_candidate_bytes_v0
     || bytes64(signature)
    )
```

`HASH` is the active canonical hash32 function defined by the Canonical Encoding and Hashing Specification.

The publication layer may commit to `authored_candidate_hash_v0` or to an enclosing event-record hash that includes it, but it MUST bind the exact signed candidate bytes and signature to the finalized order.

## 5. Public Key References

Profile v0 defines:

```text
public_key_ref = HASH(
    ascii("seed.identity.public_key_descriptor.v0")
 || key_descriptor_bytes_v0
)
```

The key descriptor bytes are:

```text
key_descriptor_bytes_v0 =
    ascii(key_profile_version)
 || ascii(signature_algorithm)
 || bytes32(raw_public_key_bytes)
 || id(owning_identity_id)
```

For Signature Profile v0:

- `key_profile_version = "ed25519_v0"`;
- `signature_algorithm = "ed25519"`;
- `raw_public_key_bytes` is exactly 32 bytes;
- `owning_identity_id` is the canonical identity that owns the key.

The same descriptor MUST produce the same `public_key_ref` in every conforming implementation.

Validation MUST reject:

- public key references whose descriptor cannot be reconstructed from canonical identity key state;
- malformed public key bytes;
- a key descriptor whose `owning_identity_id` differs from `author_identity_id`;
- a key descriptor whose profile or algorithm does not match `signature_profile`;
- an otherwise valid key that is not active at the applicable publication point.

## 6. Replay-Derived Identity Key State

Signature validation depends only on replay-derived canonical identity key state.

Private account tables, session databases, email addresses, cookies, bearer tokens, hosted recovery state, and product-owned identity adapters are never authoritative for canonical signature validation.

### 6.1 Initial Registration

An identity's initial key registration is bound to canonical identity creation or to a required linked canonical key-registration event.

The canonical identity creation path MUST provide enough canonical data to reconstruct:

- `owning_identity_id`;
- raw public-key bytes;
- key profile version;
- signature algorithm;
- resulting `public_key_ref`.

If initial key material is absent or cannot reconstruct the descriptor, that key cannot authorize new Profile-v0 human-authored writes.

### 6.2 Rotation

Key rotation introduces a new key descriptor for the same canonical identity.

A rotation is valid only when authorized by a key that is active for that identity at the rotation event's applicable publication point, unless a separately specified canonical recovery process authorizes it.

The new key becomes active at the finalized canonical position of the valid rotation event. It does not authorize events published before that position.

### 6.3 Revocation

Key revocation targets a specific `public_key_ref`.

A revocation is valid only when authorized by another active key for the same identity or by a separately specified canonical recovery process.

The revoked key becomes inactive at the finalized canonical position of the valid revocation event.

Revocation is non-retroactive. It does not invalidate canonical events that were validly signed and finalized before the revocation became effective.

A revocation that would leave no active key and no specified canonical recovery path MUST be rejected unless the active rulebook explicitly defines a non-private recovery mechanism for that identity class.

### 6.4 Active-Key State

Replay derives, for each identity and `public_key_ref`, whether the key is:

- unknown;
- active;
- revoked;
- superseded but historically preserved;
- invalid because its descriptor is malformed or owned by another identity.

Unknown keys, revoked keys used after revocation, and keys owned by another identity are rejected deterministically.

Historical key records remain replay-visible so old finalized events can be verified against the key state that existed at their publication point.

## 7. Authorship Validity

An ordinary human-authored canonical event candidate has valid authorship only when all of the following are true:

1. The candidate declares a recognized `signature_profile`.
2. The candidate's Profile-v0 signed bytes can be reconstructed exactly.
3. The `signature` verifies against the raw public key identified by `public_key_ref`.
4. The key descriptor resolves to `author_identity_id`.
5. The key was active and authorized under replay-derived canonical identity key state at the candidate's applicable publication point.
6. The `author_identity_id` is an eligible verified human identity for that event family.
7. The event passes all other schema, causal, rulebook, payload, duplicate, and publication validation.

AI provenance may be recorded as metadata where permitted by a payload schema, but AI provenance never satisfies human authorship, signature authority, verified-human eligibility, or canonical-writer eligibility.

`system_boundary_emitter` remains a separate mechanically constrained exception for authorized boundary events. It does not use ordinary human-authorship semantics unless an explicit boundary-event signature profile says so.

## 8. Publication and Replay Requirements

A publication wrapper validates after candidate validation.

The wrapper MUST bind:

- the exact signed candidate bytes;
- the signature;
- the candidate hash or enclosing event-record hash;
- the finalized publication order;
- the finalized prefix certificate.

Replay uses finalized canonical order, not event identifiers, author-observed timestamps, local receipt timestamps, or signature-generation time.

If the same valid signed candidate is submitted more than once and the finalized publication profile treats it as already accepted, replay MUST produce the same canonical effects as one accepted candidate.

If the same `event_id` is associated with different signed bytes or a different signature, validation MUST reject the later conflicting candidate or publication artifact according to the duplicate-event rules in the replay specification.

## 9. Required Authorship-Signature Conformance Vectors

Conformance suites for Signature Profile v0 MUST include vectors for:

- valid Ed25519 signature;
- invalid signature;
- altered payload hash;
- altered event type;
- altered author identity;
- wrong key owner;
- unknown key;
- revoked key used after revocation;
- historically valid event signed and finalized before later revocation;
- rotation from old key to new key;
- malformed public key;
- malformed signature;
- unsupported signature profile;
- identical candidate encoded by two implementations producing identical signed bytes;
- publication wrapper altering signed candidate bytes;
- same candidate assigned a canonical position without changing authored bytes;
- attempted use of publication-assigned `event_index` as part of the author signature.

Each vector MUST include:

- structured input;
- exact `signed_candidate_bytes_v0` in lowercase hex;
- public key in lowercase hex;
- signature in lowercase hex;
- expected `authored_candidate_hash_v0` where applicable;
- expected accept or reject result;
- the replay-derived key state needed to evaluate the vector.

Conformance fixtures MUST use deterministic public test vectors. They MUST NOT contain private keys that protect real funds, real identities, production infrastructure, or private user accounts.

## 10. Canonical Terminology and Deprecated Names

Appendix A terminology is canonical for event authorship:

| Canonical term | Meaning |
|---|---|
| `event_id` | Authored candidate identifier. |
| `author_identity_id` | Canonical identity that signs and is responsible for the event. |
| `speaker_identity_id` | Optional represented speaker when distinct from the author. |
| `signature` | Raw Profile-v0 Ed25519 signature over Profile-v0 signed bytes. |
| `public_key_ref` | Hash32 of the Profile-v0 key descriptor. |

Deprecated terms map one way to canonical terms:

| Deprecated term | Canonical term | Status |
|---|---|---|
| `global_ulid` | `event_id` | Deprecated. It MUST NOT be used in new normative event-envelope definitions. |
| `human_confirmation_proof` | `signature` | Deprecated. It MUST NOT imply a separate proof system. |
| `signer_key_id` | `public_key_ref` | Deprecated. It MUST NOT imply implementation-local key IDs. |
| `human_author_id` | `author_identity_id` | Deprecated. |
| `speaker_identity` | `speaker_identity_id` | Deprecated where used as a field name. |
| `created_at` for ordering | none | Deprecated as an ordering input. Use `author_observed_at` only when a schema permits a non-semantic human-observed value. |

`event_index`, block height, cycle index, and finalized-prefix-certificate references are publication-derived fields, not authored-candidate fields.

## 11. Non-Goals

This document does not define:

- private key custody;
- wallets;
- hosted account login;
- product account recovery;
- user interface flows;
- private DTOs;
- Tempo claims;
- cycle predicates;
- challenge voting;
- governance or token effects;
- mechanical boundary-event signatures.
