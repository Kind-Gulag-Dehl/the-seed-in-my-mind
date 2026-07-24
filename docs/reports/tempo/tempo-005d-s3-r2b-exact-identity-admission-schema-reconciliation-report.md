# TEMPO-005D-S3-R2B Exact Identity-Admission Schema Reconciliation Report

## 1. Task And Scope

R2B reconciles the exact Profile-v0 sponsored human-admission schema layer. It updates
Appendix A event schemas/effects, Profile-v0 authored-candidate and proof semantics,
canonical encodings and commitments, direct key lifecycle rules, event-registry
classification, compatibility provenance, and deterministic rejection precedence.

This is normative documentation only. It does not implement runtime behavior, replay,
verification formulas, cycles, snapshots, APIs, DTOs, migrations, fixtures, tests,
databases, or exports.

## 2. Authority And Dependency Method

The authority order applied was Protocol v5 Section 0, Canonical Encoding and Hashing,
Deterministic Replay and Merge, Protocol v5 Appendix A, the Profile-v0 Authorship and
Signature specification, then the completed Profile-v0 Identity Admission and Invitation
Capacity specification within its scoped authority. R2A already reconciled root protocol
semantics; R2B supplies the subordinate exact schemas and bytes without redefining
replay, verification, cycle, or snapshot ownership.

## 3. Baseline And Final Hashes

The controlling identity-admission specification was verified before and after this pass:

| File | Baseline SHA-256 | Final SHA-256 |
| --- | --- | --- |
| `docs/identity-admission-and-invitation-capacity-spec-v0.md` | `DD28615FB10D80D9D38BC2FB989973788627784A56AEA911472B32E8D42F73B1` | `DD28615FB10D80D9D38BC2FB989973788627784A56AEA911472B32E8D42F73B1` |
| `docs/protocol v5-appendix-a.md` | `49F1796FF48B7A3198A71FF91B35F9B4E84B37F13BD32D017928100463A2C575` | `418760888C08E04205B0E3D93A621255CF2270FA7A275DDC9EA40D2207A3D0A0` |
| `docs/canonical-event-authorship-and-signature-profile-v0.md` | `95B1D4F98755899051CF4587338D13F8B5C45DB25A72F50C5E83CCDE2452EBD6` | `6EB6A75DFC6B1715FEBBDAF47551AB550F661717623A28D208A88C4AB331029B` |
| `docs/canonical-encoding-and-hashing-spec.md` | `CA8EF2CFCE4CD89F29C08096F5B6E25A46E40AB62EE1DB8B5722CA5C353BE2BA` | `6FEAD64D5B84B91161BC2B50AA43B8A478C3D4D532AB93C734607E489D5FEEA7` |
| `docs/protocol-event-registry.v1.md` | `C0D96741D06D11F8D170EE5EC02AAFD309C9F2E34D19409996B1B9FC4F951C45` | `86F45FBD431CBE2E691FF994FC338106182932FEF60B5B7CFB5298838A5644E6` |

## 4. Files Inspected And Changed

Inspected authority, Protocol v5, Appendix A, the identity-admission target, canonical
encoding, Profile-v0 authorship/signature, the event registry, R2A report, R1 audit,
active-task ledger, devlog, notes, and existing maps.

Changed normative files:

- `docs/protocol v5-appendix-a.md`
- `docs/canonical-event-authorship-and-signature-profile-v0.md`
- `docs/canonical-encoding-and-hashing-spec.md`
- `docs/protocol-event-registry.v1.md`

No navigation update was required: R2A already established the authority and navigation
placement for this specification set.

## 5. Audit Findings Addressed

Fully addressed at R2B's normative schema layer:

- AD-004: exact sponsored `identity_create` schema/effects.
- AD-008: applicant and replacement possession proof bytes and construction rules.
- AD-009: reduced admission-authorization commitment and stale-context distinction.
- AD-010: one-active-key rotation, supersession, narrow revocation, and last-key rule.
- AD-015: registry classification and runtime-status accuracy.
- AD-023: Appendix A portion of exact event and encoding requirements.
- AD-028: stable admission/key errors and deterministic precedence.

Partially addressed:

- AD-005: `identity_verification_update` is compatibility-only. Replay, verification
  derivation, retirement, and migration processing remain R2C/R2D/E.
- AD-007: speaker absence is normative. Validators, importers, DTOs, and tests remain
  R2D/E and runtime work.
- AD-035: the closed provenance enum is normative. Replay, snapshots, storage, and
  migration manifestation remain later work.

## 6. Final `identity_create` Schema

The Profile-v0 payload is fixed-order and embedded only:

1. `identity_id` (`id`)
2. `initial_key_descriptor` (`key_descriptor_bytes_v0`)
3. `initial_public_key_ref` (`hash32`)
4. `admission_profile_version` (`ascii`, exactly `sponsored_public_admission_v0`)
5. `capacity_period_id` (`id`)
6. `rulebook_reference` (`id(rulebook_id) || ascii(rulebook_version) || hash32(rulebook_hash)`)
7. `admission_authorization_reference` (`hash32`)
8. optional `verification_reference` (`optional_hash32`)
9. explicit four-entry `identity_structural_roots`
10. explicit three-entry containment-connection ID list
11. `initial_key_possession_proof` (`bytes64`)

The sponsor is represented only by envelope `author_identity_id`; it is intentionally
not duplicated in the payload. `identity_kind` is a fixed Profile-v0 invariant:
`human`, not a free-form field. Admission does not establish VH, VI, uniqueness, civil
identity, trustworthiness, or ordinary participation authority.

## 7. Final Speaker Rule

For `identity_create`, `speaker_identity_id` MUST be absent. The sponsor is author; the
applicant is target/key controller, not author or speaker. `identity_key_rotate` and
`identity_key_revoke` also require absent speaker fields.

## 8. Final `verification_reference` Encoding

The only canonical no-reference encoding is one optional-field presence byte: `0x00`.
Presence is `0x01 || hash32(reference)`. JSON/schema-facing transport represents the
absent case by omitting the member; `null`, empty strings, zero hashes, and alternate
absence forms are forbidden.

When present, the reference may identify only an allowed canonical verification artifact
or privacy-safe canonical commitment. It cannot point to private admission transport,
account, relay, raw evidence, documents, communications, or mutable private storage. It
does not itself authorize admission or produce verification or eligibility.

## 9. Final Reduced Authorization Construction

The reduced commitment contains exactly the admission profile, sponsor identity, capacity
period ID, and structured rulebook reference. It is BLAKE3-256 over the unique ASCII
domain `seed.identity.admission_authorization.v0` plus their fixed canonical encodings.
It excludes eligibility snapshots, key state, suspension, capacity, reservation IDs, and
bearer authority.

A malformed reference, mismatched commitment, and structurally valid but stale context
have distinct errors. Replay at actual canonical position remains authoritative for
sponsor key state, eligibility, suspension, balance, identity/key uniqueness, and
structural-root feasibility.

## 10. Final Applicant-Proof Construction

The direct Ed25519 proof signs:

```text
ascii("seed.identity.initial_key_possession.v0")
|| id(event_id)
|| id(target_identity_id)
|| ascii(admission_profile_version)
|| key_descriptor_bytes_v0
|| hash32(initial_public_key_ref)
|| id(sponsor_identity_id)
|| hash32(admission_authorization_reference)
|| optional_hash32(verification_reference)
|| identity_structural_roots
|| identity_structural_root_membership_connection_ids
```

It is exactly 64 raw bytes, verifies against the descriptor's 32 raw Ed25519 bytes, and
is not prehashed. It excludes itself, the sponsor signature, final signed-candidate
bytes, a recursive payload hash, publication data, and mutable replay state. The event
ID is selected before proof creation; changing any bound field requires a new proof.

## 11. Final Sponsor-Signature Construction

After the applicant proof is inserted, the sponsor signs the ordinary existing
`signed_candidate_bytes_v0` with its active key. The normal signature binds the completed
payload hash and therefore binds the applicant proof, reference state, root plan, and
all other payload fields. There is no alternate sponsor-signature format.

## 12. Final Initial-Key Descriptor

The descriptor remains:

```text
ascii(key_profile_version)
|| ascii(signature_algorithm)
|| bytes32(raw_public_key_bytes)
|| id(owning_identity_id)
```

Profile v0 requires `ed25519_v0`, `ed25519`, exactly 32 raw public-key bytes, and target
ownership. `public_key_ref` is the BLAKE3-256 domain-separated descriptor hash. No
private material, alternate encoding, wrapper, or implementation-local key ID is valid.
A key descriptor, key, or reference once registered is globally non-reusable, including
by its former owner after supersession or revocation. Historical signatures remain valid
at their original final positions.

## 13. Final Structural-Root Representation

Because canonical object IDs are UUIDv7, Profile v0 selects explicit materialization:
the signed payload contains four root idea IDs and three containment connection IDs.
The roots are exactly Mindgarden, Backyard of Relationships, Self Tree, and Anthill, in
closed `u8` role order. Appendix A creates ordinary `conceptual_idea` objects with exact
titles, target as speaker, and non-epistemic root-role metadata, plus deterministic
Mindgarden-to-other-root membership connections.

Every root/connection is atomic with admission; a missing, duplicate, or pre-existing
entry rejects the whole event. No opaque root database or identity-as-idea endpoint was
introduced.

## 14. Final Admission Effects And Atomicity

An accepted event atomically creates the event-derived human identity, records sponsor
provenance and direct lineage, activates the initial key, materializes the complete root
set, debits one invitation-capacity unit, and enables only later-defined restricted
verification/direct-key lanes. Exact retry is idempotent; it cannot produce a second
debit or graph/key effect. Failure applies nothing.

## 15. Final Rotation And Revocation Semantics

Profile v0 has one active direct signing key per identity. Rotation is signed by that
identity's active direct envelope key and includes a separately signed replacement-key
proof over `event_id`, identity, authorizing key reference, descriptor, and replacement
reference. Rotation atomically supersedes the old key and activates the new key.

Revocation is a narrow same-identity action against a previously superseded key. It
records later compromise but cannot revoke the sole active key because Profile v0 defines
no recovery event. That action returns `last_active_key_revocation_forbidden`; an already
revoked target returns `key_already_revoked`. Recovery and keyless retirement remain
future-profile work.

## 16. `identity_verification_update` Classification

`identity_verification_update` is compatibility-only. Ordinary post-genesis ingress must
reject it unless an explicit versioned genesis/import/legacy manifest authorizes it. Its
payload records compatibility class, manifest reference, historical status, and optional
historical reference. It cannot directly set VH, VI, or any current eligibility, and it
must not fabricate claims, evidence, sponsorship, proofs, debits, or lineage.

## 17. Event-Registry Updates

The derived registry now identifies:

- `identity_create` as specification-reconciled but runtime-unimplemented sponsored
  admission with absent speaker and atomic root/key/capacity effects.
- `identity_key_rotate` and `identity_key_revoke` as specification-reconciled direct
  lifecycle events without ordinary-writer dependency.
- `identity_verification_update` as compatibility-only, not ordinary verification.

The registry explicitly labels older validator requirements for target speaker/title as
runtime drift rather than current protocol support.

## 18. Genesis, Import, And Legacy Provenance

The identity provenance enum is closed: `genesis_admitted`,
`legacy_operator_provisioned`, `event_derived`, and `future_profile_derived`.
`event_derived` is a replay effect of accepted `identity_create`, never author supplied.
Compatibility classes require an explicit manifest and cannot fabricate modern
sponsorship, applicant proofs, capacity debits, lineage, structural roots, or
verification attestations.

## 19. Admission Error Registry And Precedence

Appendix A now provides fixed errors for schema/profile, speaker, descriptor/reference,
proof, duplicate, authorization, key-state, inviter, capacity, root, lifecycle,
compatibility, and reserved restricted-lane failures. The 13-stage precedence is:
envelope; support; field/speaker; payload hash/signature; author key; proof; duplicate
identity/key; context applicability; inviter eligibility; suspension; capacity; root or
last-key feasibility; atomic application. `stale_admission_authorization` cannot mask a
specific key, eligibility, suspension, capacity, duplicate, or root failure.

## 20. Future Conformance-Vector Requirements

The later conformance task must cryptographically exercise at least:

- valid sponsor-authored `identity_create` and no-speaker enforcement;
- the exact `0x00` no-reference bytes and rejection of every alternate absence form;
- reduced admission-authorization construction, including wrong sponsor, capacity
  period, and rulebook-reference mutations;
- applicant proof success and mutation of every bound proof field, including event ID,
  target, descriptor, key reference, sponsor, authorization reference, verification
  reference, root plan, and root connection list;
- recursive-proof prohibition and sponsor signature over the completed payload that
  contains the proof;
- stale authorization distinct from inactive/revoked key, inviter ineligibility,
  suspension, and exhausted capacity;
- duplicate identity and global historical-key rejection; successful atomic capacity
  debit; and an exact retry without a second debit;
- complete structural-root creation, fixed root ordering, and root collision;
- valid key rotation, replacement-key proof failure, historical key reuse rejection,
  historical signature validity, and the sole-active-key revocation rule;
- compatibility-authorized and ordinary-ingress-rejected `identity_verification_update`.

No JSON vectors or fixtures were created in R2B.

## 21. Deferred R2C Work

R2C must reconcile deterministic replay and merge, verification ontology/derived VH and
VI, restricted verification event catalog, invitation eligibility/capacity generation,
qualifying periods, liveness, rulebook interfaces, cycle semantics, snapshots, and the
replay treatment/retirement of compatibility records.

The final terminology sweep assigns remaining normative `identity_verification_update`
and transitional `canonical_writer_level` material in Protocol v5 and the Verification
Specification to R2C. `snapshot-format-v0.md` still needs the resulting replay-derived
identity/key/provenance/root/lineage commitments. Historical task reports, devlog
entries, and the non-authoritative identity-admission outline retain historical terms
only and need no R2B correction.

## 22. Deferred Combined R2D/E Work

R2D/E must reconcile privacy/offline/safety/governance/AI boundaries, APIs, public DTOs,
node and conformance contracts, exact vector artifacts, boundary checks, implementation
status, and public-read explanations. It must propagate the speaker rule and public
capacity derivability without claiming private-state authority.

The final terminology sweep assigns `speaker_identity_id` handling in
`api-contract-read-only.md`, `node-and-conformance-spec.md`, and
`offline-and-mindseed-spec.md` to this batch, together with the runtime DTO and
validator drift recorded by the event registry.

## 23. Deferred Runtime Implementation Work

Runtime remains blocked. It needs replay/state schemas, storage/migrations, public route
and DTO updates, strict validator changes, atomic materialization, isolated integration
tests, vector execution, export verification, and compatibility migration work after the
remaining specification reconciliation.

## 24. Validation

- Required identity-admission SHA-256 matched before and after editing.
- Complete baseline and final SHA-256 values were recorded for every edited normative
  file.
- Targeted searches verified the exact payload fields, removed snapshot/legacy payload
  names, optional encoding, domain tags, root set, compatibility classification, and
  error names.
- The controlling identity-admission specification hash matched exactly before and
  after this pass.
- Appendix A heading-order and all changed-file duplicate-heading checks passed.
- Control-character scan passed for all changed R2B files. A whole-file scan found 45
  pre-existing trailing-whitespace lines in Appendix A outside R2B content; no added
  trailing whitespace was present, as confirmed by `git diff --check`.
- `git diff --check` passed. Git emitted only line-ending warnings for existing Markdown
  working-copy conversion behavior.

## 25. Readiness Assessment

Identity-admission target internally complete: yes.

Authority and Protocol root reconciliation complete: yes.

Exact Profile-v0 admission schema reconciled: yes.

Ready for R2C replay/verification/cycle/snapshot reconciliation: yes.

Ready for runtime identity-admission implementation: no. Replay, verification,
cycle/snapshot, privacy/API/DTO/conformance, and runtime reconciliation remain required.

## 26. Recommended Next Task

`TEMPO-005D-S3-R2C - Replay, Verification, Cycle, Rulebook, And Snapshot Reconciliation`

Scope it narrowly to replay-derived identity/key/provenance/root/lineage state,
compatibility-record treatment, restricted verification into the ordinary epistemic
ontology, inviter eligibility/capacity/liveness hooks, qualifying-cycle behavior, and
snapshot commitments. Do not implement runtime behavior in R2C.

## 27. No-Change Boundary

No runtime code, migration, API, DTO, test, fixture, conformance-vector artifact,
database, export, generated file, private-repository file, Protocol v5 root document,
cross-document invariant, or controlling identity-admission specification was edited in
R2B.
