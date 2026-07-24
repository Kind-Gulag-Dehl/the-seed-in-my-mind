# TEMPO-005D-R3-P2 Pure Schema and Cryptographic Validation Report

## 1. Scope and Stop Boundary

Implemented the database-free Profile-v0 identity-admission schema and cryptographic
validation foundation. This pass adds typed payload parsing, canonical byte construction,
Ed25519 possession-proof verification, sponsor candidate verification, pure supplied-state
checks, and executable fixed crypto fixtures.

It does not write canonical state, query a database, alter storage or migrations, change
replay or snapshots, debit invitation capacity, derive VH/VI or eligibility, add API/DTO
surfaces, or alter frontend or private-repository code. The existing legacy/account-coupled
identity path remains unchanged and is not accepted by the explicit Profile-v0 validator.

## 2. Controlling Hash Confirmation

The controlling identity-admission specification SHA-256 matched before implementation and
after all edits:

```text
DD28615FB10D80D9D38BC2FB989973788627784A56AEA911472B32E8D42F73B1
```

## 3. Files Inspected and Changed

Inspected the R3-P1 plan; the controlling admission specification; Appendix A; canonical
authorship, encoding, registry, API-contract, and static-vector specifications; the existing
`event-log`, `verification`, and `encoding` crates; current signature vectors; and Cargo
workspace conventions.

Substantive changes:

- `backend/crates/verification/src/admission.rs`: admission commitments, root-plan encoding,
  key-reference derivation, and direct Ed25519 possession-proof primitives.
- `backend/crates/verification/src/lib.rs`: exposes the admission module.
- `backend/crates/event-log/src/profile_v0_admission.rs`: typed Profile-v0 payload parsing,
  canonical payload bytes, pure candidate validation, pure-state inputs, and tests.
- `backend/crates/event-log/src/lib.rs` and `backend/crates/event-log/Cargo.toml`: expose the
  pure module and reuse the existing verification crate/Ed25519 dependency.
- `backend/Cargo.lock`: records those existing workspace dependencies for `event-log`.
- `docs/conformance/profile-v0-identity-admission.{vectors.json,schema.json,vectors.md}`:
  fixed public P2 crypto fixtures and their schema/guide description.

## 4. Implemented Schema Types

The event-log module implements strict typed parsing for `identity_create`,
`identity_key_rotate`, and `identity_key_revoke`. The Profile-v0 identity-create parser
requires the closed field set, fixed sponsored admission profile, UUIDv7 identifiers,
Ed25519 descriptor, reduced authorization reference, omitted-or-hash32
`verification_reference`, four-root plan, three membership IDs, and bytes64 applicant proof.

`identity_kind` is a profile invariant rather than a payload field: attempted non-human
assertions return `invalid_target_identity_kind`; other payload extras are rejected.
`identity_verification_update` has a pure ordinary-ingress rejection helper returning
`compatibility_event_not_authorized`; manifest-authorized compatibility handling remains
stateful later work.

## 5. Implemented Canonical Encodings

The verification admission module implements the R2B/Profile-v0 byte layouts for:

- descriptor bytes and domain-separated public-key reference;
- the four-field BLAKE3 reduced admission-authorization commitment;
- `verification_reference` as exactly `0x00` absent or `0x01 || hash32` present;
- the explicit, ordered Mindgarden, Backyard of Relationships, Self Tree, and Anthill root
  plan plus three membership connection IDs;
- applicant initial-key possession bytes; and
- replacement-key possession bytes.

The event-log module serializes the three Profile-v0 payloads in Appendix A field order and
recomputes their payload hashes before sponsor-signature validation. JSON `null`, empty, all
zero, and alternate absence forms are rejected for `verification_reference`.

## 6. Proof and Signature Validation Behavior

Applicant and replacement proofs use strict RFC 8032 Ed25519 verification over the exact
domain-separated message. The applicant message binds event ID, target, profile, complete
descriptor, initial reference, sponsor, authorization reference, exact optional verification
reference, roots, and membership IDs. It excludes itself, sponsor signature, completed-candidate
bytes, payload hash containing itself, and mutable replay state.

The sponsor candidate is verified only after the completed identity-create payload hash is
reconstructed; therefore the ordinary Profile-v0 signature binds the applicant proof. Key
reference/descriptor equality is checked independently. The pure validators accept immutable
identity/key/root state inputs solely for duplicate and collision checks; they perform no I/O.

## 7. Speaker and Author Enforcement

The explicit Profile-v0 candidate validators require absent `speaker_identity_id` for identity
creation and both direct-key events. The sponsor is the candidate author for `identity_create`;
the applicant is the target/key controller only. Direct-key rotation and revocation require the
controlled identity as author and its active direct key in supplied pure state.

The generic legacy event validator was deliberately not changed: current storage still invokes it
for the legacy self/speaker-based identity path. That preserves P2's pure stop boundary. P3 must
quarantine or replace that stateful ingress path rather than treating it as Profile-v0 support.

## 8. Structural-Root Pure Validation

The root plan requires exactly one ordered role each for Mindgarden, Backyard of Relationships,
Self Tree, and Anthill; exactly three UUIDv7 membership connection IDs; and pairwise-distinct
root/connection IDs. The validator rejects missing, duplicate, unordered, malformed, or occupied
supplied root IDs with stable `incomplete_identity_structural_roots` or
`structural_root_collision` errors. It creates no ideas or connections.

## 9. Key-Lifecycle Pure Validation

The direct-key validator models one active direct key through supplied immutable key state.
Rotation requires exactly one supplied active authorizing key, a descriptor/reference/raw public
key not present in supplied history, and a valid replacement proof. It returns pure errors for
superseded/revoked/invalid authorizing keys without applying a transition.

Revocation permits only an owned superseded target; it rejects an already revoked target and
forbids revocation of the sole active key. Supersession, activation, historical signature
position, idempotence, and persistence remain P3/replay responsibilities.

## 10. Error and Precedence Mapping

The pure layer uses Appendix A stable errors for unsupported profile, malformed identity/key
payloads, fixed-kind violations, speaker presence, malformed descriptor/reference, invalid
verification-reference representation, invalid or binding-mismatched proofs, supplied duplicate
identity/key/root state, direct-key authorization, supersession, revocation, last-key revoke,
and ordinary compatibility-only verification updates.

It separates structural/cryptographic failures from replay-only errors. It intentionally does
not decide sponsor human classification, active author state at canonical position, period
staleness, inviter eligibility, suspension, available capacity, atomic debit, duplicate-event
idempotence, or manifest authorization. Those facts require P3 storage/replay state.

## 11. Executable Vector Coverage

`profile-v0-identity-admission.vectors.json` now carries two fixed public crypto fixtures:

- `identity_create_primary` covers the pure schema/byte/proof portions of IA-002 through
  IA-013, including omitted speaker, fixed human profile, absent/present optional-reference
  encodings, reduced commitment, applicant proof, non-recursion, and completed sponsor signature.
- `identity_key_rotate_primary` covers the cryptographic portion of IA-025.

`static_profile_v0_admission_crypto_fixtures_match_runtime` reads those fixtures and checks
canonical payload bytes, commitment bytes, possession messages/proofs, and sponsor signatures.
Focused pure tests additionally cover IA-014/015/017 supplied state, IA-026/027/028 direct-key
rejections, and IA-029 ordinary compatibility rejection.

## 12. Deferred Stateful Vector Coverage

IA-001's canonical application and IA-016 through IA-035 remain unexecuted at their full
stateful scope. In particular, capacity debit/idempotence, period/rulebook staleness, sponsor
key/eligibility/suspension checks, compatibility manifests, qualifying cycles/liveness,
provenance materialization, restricted verification lane, replay, snapshots, API behavior, and
database atomicity require later implementation slices.

## 13. Compatibility Quarantine

No legacy self-authored, target-as-speaker, account-coupled, bootstrap/import, or mutable
`canonical_writer_level` input is accepted by the explicit Profile-v0 validators. P2 does not
delete or alter legacy behavior. The later stateful quarantine must route only complete
sponsor-authored candidates into the new path and classify legacy/import records through their
explicit compatibility authority.

## 14. Test Commands and Results

All commands below were database-free:

- `cargo fmt --all -- --check`: passed.
- `cargo test -p event-log -p verification`: passed, 41 event-log and 5 verification tests.
- `cargo test -p encoding`: passed, 10 tests.
- `cargo check -p encoding -p verification -p event-log`: passed.
- `npm run conformance`: passed, 29/29 Tempo/Cycle fixtures.
- `npm run verify:boundaries`: passed.
- `npm run verify:canonical-dto`: passed.
- Node JSON parse of the changed identity-admission vector and schema: passed.

An initial `cargo test -p encoding` invocation from the repository root reported no Cargo
manifest; the same command was rerun from `backend` and passed. Windows incremental-build
hard-link warnings were emitted during Cargo commands; they did not affect compilation or tests.
Two intermediate compile/test failures while tightening the new raw-key and active-key-count
checks were corrected before the final formatter, test, and cargo-check runs above; there are no
unresolved failures.

## 15. Remaining P3 Dependencies

P3 must add replay-position state validation, canonical transactionality, exact capacity debit,
identity/key/root/lineage persistence, additive compatibility provenance, legacy account-path
quarantine, and deterministic error precedence across stateful conditions. It must retain this
pure module as the schema/proof boundary and use only disposable databases guarded by
`common::test_db_guard::require_disposable_database_url` for database tests.

## 16. Readiness Assessment

- Pure Profile-v0 admission schema and cryptographic validation: implemented and tested.
- Static public crypto fixtures: implemented and exercised by the pure test suite.
- Stateful admission, storage, replay, capacity, API, DTO, and snapshot behavior: not implemented.
- Database use, migrations, exports, private-repository edits, and runtime ingress changes: none.
- Recommended next task: `TEMPO-005D-R3-P3 - Stateful Admission Validation, Atomic Storage Transition, and Compatibility Quarantine`.
