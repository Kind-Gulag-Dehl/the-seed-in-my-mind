---
doc_id: canonical_identity_admission_api_contract_v0
title: Canonical Identity Admission API Contract v0
status: draft contract
version: v0
scope:
  - Defines planned public transport and read projections for Profile-v0 canonical identity admission.
  - Separates canonical admission from non-canonical applicant request transport and private product APIs.
authoritative_for:
  - Language-neutral public API and DTO contract requirements after Profile-v0 runtime implementation begins.
not_authoritative_for:
  - Canonical event schemas, signed bytes, replay algorithms, privacy primitives, or current runtime implementation status.
depends_on:
  - identity-admission-and-invitation-capacity-spec-v0.md
  - protocol v5-appendix-a.md
  - canonical-event-authorship-and-signature-profile-v0.md
  - canonical-encoding-and-hashing-spec.md
  - deterministic-replay-and-merge-spec.md
  - verification-spec.md
  - cycle-spec.md
  - snapshot-format-v0.md
  - api-contract-read-only.md
---

# Canonical Identity Admission API Contract v0

## 1. Scope and status

This is a scoped implementation contract. It does not add protocol authority. Appendix A owns event schemas and errors; the Profile-v0 signature and encoding specifications own bytes and proofs; replay, verification, cycle, Tempo, and snapshot specifications own derived state.

The current exported runtime does **not** implement the Profile-v0 routes or projections in this document. `api-contract-read-only.md` remains the current implemented public API reference. This contract tells a later implementation what it must expose without treating a route, table, account registration, or private request as canonical authority.

## 2. Boundary model

```text
local identity/key preparation
    -> non-canonical applicant request
    -> sponsor-authored signed identity_create candidate
    -> canonical validation, publication, and replay
    -> public replay-derived identity admission projection
```

Local preparation and applicant requests may be exchanged directly, through a relay, or through a future request-discovery mechanism. They are not canonical resources. They do not reserve capacity, obligate a sponsor, establish identity or verification, or grant authority.

Private product accounts, sessions, contact records, raw private evidence, documents, messages, relay-local records, storage IDs, secrets, and AI prompts or outputs are outside this contract and MUST NOT be accepted as canonical authority or returned in canonical DTOs.

## 3. Planned canonical write ingress

### 3.1 Route and status

The planned route is the existing public canonical candidate ingress:

```text
POST /api/v1/canonical/events
```

Profile-v0 support is planned, not currently implemented. The route accepts a complete sponsor-authored signed candidate only. It MUST NOT accept an applicant request, account registration, client-provided capacity balance, eligibility result, verification outcome, mutable writer level, or a private transport reference as authority.

### 3.2 Request shape

```json
{
  "candidate": {
    "signature_profile": "ed25519_v0",
    "event_id": "UUIDv7",
    "event_type": "identity_create | identity_key_rotate | identity_key_revoke",
    "author_identity_id": "UUIDv7 sponsor or controlled identity",
    "speaker_identity_id": "UUIDv7 | null (event-schema controlled)",
    "public_key_ref": "hash32 lowercase hex",
    "payload_hash": "hash32 lowercase hex",
    "payload_binding_mode": "embedded_payload",
    "payload_ref": null,
    "author_observed_at": null,
    "signature": "64-byte Ed25519 signature as lowercase hex"
  },
  "payload": "exact Appendix A event payload"
}
```

`identity_create`, `identity_key_rotate`, and `identity_key_revoke` require absent `speaker_identity_id`; neither applicant nor sponsor is represented as speaker. The sponsor is the `identity_create` candidate author. The payload is exactly Appendix A's Profile-v0 payload and includes the applicant possession proof, optional `verification_reference` using the one canonical presence encoding, root plan, and reduced authorization commitment. This contract does not duplicate those field schemas.

### 3.3 Transport and finality

A transport acknowledgement confirms receipt only. It is not capacity reservation, canonical acceptance, publication, or finality. Where the active publication profile separates acceptance from finality, the response MUST expose the distinction.

```json
{
  "event": {
    "event_id": "UUIDv7",
    "event_type": "identity_create",
    "candidate_status": "received | accepted | finalized | rejected",
    "canonical_event_reference": "UUIDv7 | null",
    "publication_reference": "string | null",
    "idempotent": "boolean"
  },
  "rejection": {
    "error_code": "stable Appendix A identifier | null",
    "diagnostic_category": "safe public category | null"
  }
}
```

An accepted response MAY include a canonical event or commitment reference. It MUST NOT return private evidence or request data. A rejection exposes a stable error identifier and a safe category, not secrets, private documents, or node-local reasoning.

### 3.4 Stable rejection surface

The planned ingress MUST expose the Appendix A error identifiers unchanged, including `unsupported_admission_profile`, `malformed_identity_create_payload`, `invalid_target_identity_kind`, `identity_already_exists`, `malformed_initial_key_descriptor`, `public_key_already_registered`, `invalid_applicant_possession_proof`, `applicant_proof_binding_mismatch`, `speaker_not_permitted`, `malformed_admission_authorization`, `stale_admission_authorization`, `author_key_inactive`, `author_key_revoked`, `inviter_ineligible`, `inviter_suspended`, `insufficient_invitation_capacity`, `incomplete_identity_structural_roots`, `structural_root_collision`, and the direct-key lifecycle and compatibility errors.

The ingress MUST preserve Appendix A validation precedence. In particular, stale authorization is not a substitute for inactive key, ineligibility, suspension, capacity exhaustion, duplicate identity, or duplicate key errors.

## 4. Planned public read projections

All public projections are derived from verified canonical replay at an explicit snapshot basis. Full canonical history and active rulebooks remain authoritative; a summary or index is not a separate source of authority.

### 4.1 Identity detail

```json
{
  "identity": {
    "identity_id": "UUIDv7",
    "canonical_existence": true,
    "identity_kind": "human",
    "provenance_class": "genesis_admitted | legacy_operator_provisioned | event_derived | future_profile_derived",
    "admission": {
      "event_id": "UUIDv7 | null",
      "profile_version": "string | null",
      "sponsor_identity_id": "UUIDv7 | null",
      "lineage_event_id": "UUIDv7 | null",
      "compatibility_manifest_reference": "string | null"
    },
    "structural_roots": [
      { "role": "Mindgarden | Backyard of Relationships | Self Tree | Anthill", "idea_id": "UUIDv7", "created_event_id": "UUIDv7" }
    ],
    "direct_keys": {
      "active_public_key_ref": "hash32 | null",
      "history": [
        { "public_key_ref": "hash32", "state": "active | superseded | revoked | invalid", "activation_event_id": "UUIDv7", "transition_event_id": "UUIDv7 | null" }
      ]
    },
    "verification": {
      "verification_state": "canonical summary",
      "vh_certainty": "rulebook-derived summary | null",
      "vi_certainty": "rulebook-derived summary | null"
    },
    "eligibility": {
      "restricted_verification_lane_eligibility": "boolean",
      "ordinary_writer_eligibility": "boolean",
      "ordinary_challenge_eligibility": "boolean",
      "voter_eligibility": "boolean",
      "governance_eligibility": "boolean",
      "tempo_eligibility": "boolean",
      "inviter_eligibility": "boolean"
    },
    "invitation": {
      "capacity_balance": "decimal string | omitted",
      "capacity_explanation_reference": "canonical event or rulebook reference | null",
      "invitation_suspension": "boolean",
      "maturation_state": "canonical summary",
      "qualifying_capacity_period": "canonical summary",
      "admission_liveness_blocked": "boolean"
    },
    "history_references": ["canonical event or snapshot reference"]
  }
}
```

### 4.2 Field authority labels

| Field group | Classification | Rule |
| --- | --- | --- |
| Identity, kind, provenance, keys, roots, lineage, eligibility, capacity, suspension, maturation, liveness | Canonical directly replay-derived or canonical summary/index derived from replay | Must be reproducible from canonical history, active rulebooks, and snapshot basis. |
| Key and admission history | Historical-only canonical data | May be paginated but must preserve canonical references and ordering. |
| Display labels, explanations, and compacted capacity presentation | Optional presentation data | Not authoritative; omission does not change state. |
| Capacity omitted or bucketed for UI coercion concerns | Intentionally omitted presentation data | Not cryptographic secrecy: exact Profile-v0 capacity remains publicly replay-derivable. |
| Private request/evidence/account/contact/relay data | Intentionally excluded | Never a canonical DTO field. |

Publicly allowed verification summaries MUST distinguish raw artifacts, VH/VI certainty, and eligibility lanes. They MUST NOT imply that sponsorship, lineage, roots, a `verification_reference`, or a compatibility row is verification.

### 4.3 Planned reads

An implementation MAY expose identity detail and history through versioned endpoints such as:

```text
GET /api/v1/canonical/identities/{identity_id}
GET /api/v1/canonical/identities/{identity_id}/keys
GET /api/v1/canonical/identities/{identity_id}/admission-history
GET /api/v1/canonical/admission/status
```

Exact path selection remains an implementation compatibility decision. Any route MUST expose the snapshot basis, canonical explanation references, and the authority classifications above.

## 5. Compatibility quarantine

The current runtime's self/speaker-based identity validation, account-coupled creation, mutable `canonical_writer_level`, account/session-driven writer handlers, bootstrap and seed-import identity/root paths, legacy operator-provisioned rows, and signed-write support limited to `idea_create` and `connection_create` are transitional, compatibility-only, non-authoritative, or not-yet-conformant as applicable. They do not satisfy Profile-v0 admission merely because a route, table, migration, DTO, or test exists.

## 6. Implementation and conformance sequence

Before exposing these routes as implemented, the public runtime must add validated event ingestion, storage and migrations, replay and snapshots, public read projections, language-specific DTOs, static-vector execution, database-backed atomicity/idempotence tests, compatibility migration, and boundary checks. `docs/conformance/profile-v0-identity-admission.vectors.json` is the static contract vector inventory; it is not a runtime-harness success claim.
