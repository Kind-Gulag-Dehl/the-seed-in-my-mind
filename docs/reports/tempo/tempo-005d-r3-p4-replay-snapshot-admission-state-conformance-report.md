# TEMPO-005D-R3-P4 Replay and Snapshot Admission-State Conformance Report

## 1. Scope and Stop Boundary

TEMPO-005D-R3-P4 implemented a database-free Profile-v0 admission replay
projection and its deterministic snapshot commitment. The work consumes the
typed candidates and pure validation established by P2 and preserves P3 as the
stateful storage transition boundary.

This pass does not write storage, run migrations, use a database, change
`ReplayDriver` database materialization, change the Stage-0 snapshot builder or
binaries, add HTTP ingress, alter DTOs, implement capacity generation or
qualifying-cycle policy, derive VH/VI, or change private-product code.

## 2. Controlling Hash Confirmation

`docs/identity-admission-and-invitation-capacity-spec-v0.md` remained unchanged
before and after the work:

`DD28615FB10D80D9D38BC2FB989973788627784A56AEA911472B32E8D42F73B1`

## 3. Baseline and Final Hashes

| File | Baseline SHA-256 | Final SHA-256 |
| --- | --- | --- |
| `backend/crates/replay/Cargo.toml` | `636F28001BFEB5C57A78125B52D8E65A408648B6071E8AA5ADDC614286B84691` | `65B0187E396672C005A0CC446A32D868DA522D2A23B0ED3C6AAFFA9B06DE7A9C` |
| `backend/crates/replay/src/lib.rs` | `FF55FC844445A23D4099D7560B700A75B5CCA9495C8EEB043D0035E76C7924EA` | `40E541F2FE6798ABD83DD2DA66E89142DBEAB6C26586C3A83CA3D09F8C8A883B` |
| `backend/crates/replay/src/profile_v0_admission.rs` | not present at baseline | `EE1816D494C313FE31B2944066D0D4E8EDA76C86130352DDE540DD5C8532BA93` |
| `backend/crates/snapshot/Cargo.toml` | `6C7BC6D225FD13DE0A0D9CB407B31DD40F43B5250793CBD5A44BA1FBBDB65C73` | `AF69651E202F99560E5A09DE1FEEC1DFD5A14785F9B594E300E4AAD47FE786EE` |
| `backend/crates/snapshot/src/format.rs` | `DF981FD4DE5BE6B2A8BCDF4565CFCC936AD70BBF7DF3AC7E62C1B63DDC2FE11E` | `0A62B4757F1762335E4EF748A16782F71CEE048D34DE3675884F12CD75494507` |
| `backend/crates/snapshot/src/lib.rs` | `24E8164A200EDB60579B09D5E1BB8B1792B7EE8F24BC58AC08F9CE142494972F` | `933C07690D20A7D60F95CF0DB0D97C25E301A764B3796EA64A6E1194ED6974AC` |
| `backend/crates/snapshot/src/profile_v0_admission.rs` | not present at baseline | `FFE3CFD38CD6F6B24A217C1D21AA07F7A6369514802B4F616A0BB1CA7E497BCD` |

## 4. Files Inspected and Changed

Inspected: the P1, P2, and P3 reports; admission, replay, snapshot, encoding,
verification, cycle, Tempo, and conformance specifications; the Profile-v0
event-log and verification implementations; the existing database-backed
`ReplayDriver`; Stage-0 snapshot format; snapshot tooling; and existing tests.

Substantive changes:

- `backend/crates/replay/src/profile_v0_admission.rs`: pure replay state,
  candidate application, compatibility records, and tests.
- `backend/crates/replay/src/lib.rs` and `Cargo.toml`: public replay module and
  existing verification dependency.
- `backend/crates/snapshot/src/profile_v0_admission.rs`: deterministic
  Profile-v0 admission snapshot build and verification.
- `backend/crates/snapshot/src/lib.rs`, `format.rs`, and `Cargo.toml`: exported
  snapshot surface, comparable `SnapshotSection`, and test-only dependency.

Coordination changes are this report, the active-task ledger, devlog, and code
notes. Pre-existing P2/P3 and specification work remained untouched.

## 5. Replay Event Integration

`ProfileV0AdmissionReplay` now consumes ordered canonical candidates without a
database and applies only these Profile-v0 event families:

- `identity_create`;
- `identity_key_rotate`;
- `identity_key_revoke`;
- manifest-authorized compatibility-only `identity_verification_update`.

Events are ordered by `(block_height, event_index, event_id)`. Event-ID retries
with equal candidate material are idempotent; conflicting reuse of an event ID
or canonical position is rejected. Failed validation returns a stable error and
does not replace the current derived state. The module has no database, private
account, API-arrival-order, wall-clock, AI, or operator input.

## 6. Identity, Key, Root, and Lineage Projection

A successful `identity_create` atomically derives an event-derived human
identity with `CanonicalAdmittedIdentity` semantics, the accepted active direct
key, all four structural roots, their three membership connections, sponsor
provenance, direct sponsor lineage, and one capacity-debit fact.

The fixed roots are preserved as ordinary graph identifiers with the exact
roles from P2: Mindgarden, Backyard of Relationships, Self Tree, and Anthill.
Incomplete or colliding roots leave no partial replay state. Sponsor lineage is
recorded only as admission provenance; it establishes neither verification nor
ordinary authority.

The projection retains direct-key history. Rotation supersedes the previously
active direct key and activates the replacement only after P2 validation.
Revocation follows the P2 sole-active-key restriction. The state model keeps
active, superseded, revoked, invalid, and compatibility-only key classifications
separate from historical signature validity.

## 7. Compatibility Treatment

Seed identities are explicit provenance-classed compatibility inputs:
`genesis_admitted`, `legacy_operator_provisioned`, or
`future_profile_derived`; they are never rewritten as Profile-v0 sponsored
admissions. Missing sponsor, applicant proof, debit, lineage, and verification
history is not fabricated.

An ordinary `identity_verification_update` is rejected as
`compatibility_event_not_authorized`. The only accepted form is an explicit
versioned manifest compatibility record with a non-event-derived provenance
class. Such a record remains historical/readable metadata and grants no VH, VI,
writer, inviter, voter, governance, Tempo, or economic authority.

## 8. Capacity and Liveness Projection Boundary

Every accepted admission produces one immutable debit fact containing the
admission event, sponsor, admitted identity, capacity period, rulebook
reference, debit quantity, and canonical position. An exact retry produces no
second debit. Rejected candidates produce none.

This slice intentionally does not create credits, balances, qualifying periods,
maturation, restoration, or emergency capacity. The replay state represents
`invitation_capacity_balance`, `invitation_suspension`, `maturation`, and
`admission_liveness_blocked` as `NotYetDerived` until the qualifying
cycle/rulebook implementation supplies authoritative inputs. It therefore
cannot manufacture capacity or authority from Dmax, forced, degraded, survivor,
record-only, AI, system, clock, or machine-only activity.

## 9. Snapshot Fields and Commitments

`ProfileV0AdmissionSnapshot` is a standalone deterministic pack with four
ordered sections:

1. identity kind, provenance, admission reference, sponsor, and structural
   roots/membership;
2. direct-key descriptors, references, lifecycle state, and transition events;
3. lineage/provenance facts;
4. eligibility lanes, capacity debits, compatibility records, and available
   derivation statuses.

The pack uses existing canonical encodings and domain-separated section,
state-root, and snapshot hashes. Build and verification compare the whole
deterministic projection. A change to admission lineage, roots, key state,
capacity debit, or liveness derivation invalidates verification. Full canonical
history and active rulebooks remain authoritative; the pack is a commitment to
the supplied pure projection, not a replacement for them.

The existing Stage-0 database-backed snapshot pack and its binaries remain
unchanged. P5/integration work must select this Profile-v0 pack when exposing
or publishing Profile-v0 admission state.

## 10. Executable Replay and Snapshot Vector Coverage

Database-free tests now cover:

- valid sponsor-authored admission and fixed human projection;
- absent-speaker, non-human, invalid-proof, duplicate-key, and root-collision
  rejection without mutation;
- atomic four-root and membership projection;
- sponsor lineage with only restricted initial authority;
- exactly one debit and idempotent exact retry;
- deterministic ordering across equivalent input order;
- direct-key rotation, invalid lifecycle protection, and last-active-key
  revocation rejection;
- ordinary verification-update rejection and manifest-only compatibility
  classification;
- deterministic snapshot build/verify and commitment changes for lineage,
  roots, direct keys, debit, and liveness;
- honest `NotYetDerived` capacity/liveness boundary with no boundary-generated
  capacity.

The existing P2 static crypto fixtures continue to execute through event-log
and verification tests. This pass does not change vector IDs or vector data.

## 11. Deferred Vectors and Dependencies

The following remain outside P4:

- P3 PostgreSQL migration and atomic transaction cases;
- execution against the storage-backed canonical event log;
- qualifying human-deliberative period, credit, carryover, maturity,
  suspension/restoration, and positive-capacity vectors;
- VH/VI and rulebook eligibility formulas;
- restricted verification event authorization beyond the initial lane state;
- snapshot-builder/snapshot-verify binary integration with Profile-v0 history;
- canonical write ingress, public reads, DTOs, API finality, and frontend work.

These require P3 integration validation plus the P5 API/DTO/ingress slice and
subsequent capacity/rulebook implementation work. They are not represented as
granted authority in the P4 projection.

## 12. P3 Database-Validation Gate

Release blocker retained verbatim:

> P3 PostgreSQL migration and atomic-transaction test matrix has not yet run
> because no explicit disposable PostgreSQL admin URL was configured.

P4 did not run database tests, migrations, or database setup. Source-level P3
tests are not a substitute for the guarded disposable PostgreSQL matrix. Full
storage conformance, production readiness, and runtime identity-admission
completion remain blocked until that matrix passes.

## 13. Test Commands and Results

- `cargo fmt --all`: passed.
- `cargo check -p replay -p snapshot`: passed.
- `cargo test -p replay --lib`: passed, 23 tests.
- `cargo test -p snapshot --lib`: passed, 5 tests.
- `cargo test -p event-log profile_v0_admission --lib`: passed, 8 tests.
- `cargo test -p verification --lib`: passed, 5 tests.
- `cargo test -p encoding --lib`: passed, 0 tests.
- `npm run conformance`: passed, Tempo/Cycle 29/29 fixtures.
- `npm run verify:boundaries`: passed.
- `npm run verify:canonical-dto`: passed.

Compiler output included pre-existing incremental-cache hard-link warnings for
the workspace volume. No test failure or database access resulted.

The P4 code, report, active-task lifecycle edit, appended devlog entry, and
notes entry have no trailing whitespace or control characters. The full
append-only devlog retains four unrelated pre-existing trailing-space lines in
an older R2D entry; they were not normalized. `git diff --check` has no
whitespace errors.

## 14. Remaining API/DTO/Ingress Work and Recommendation

`ReplayDriver` and the existing Stage-0 snapshot builder remain database-backed
legacy paths and do not yet dispatch this pure Profile-v0 projection. Public
canonical ingress, planned write finality, read DTOs, capacity explanation
surfaces, and Profile-v0 snapshot publication remain unimplemented.

Recommended next task: **TEMPO-005D-R3-P5 - Canonical Profile-v0 Identity
Admission Ingress, Public Read DTOs, and API Integration**. It must consume the
P2 validator, P3 atomic storage transition, and P4 pure replay/snapshot
projection without restoring account-coupled admission authority.

## 15. Readiness Assessment

- Pure schema and cryptographic validation: complete.
- Stateful storage transition: implemented at source level; guarded PostgreSQL
  matrix remains pending.
- Replay and snapshot admission projection: complete at the database-free P4
  boundary.
- API/DTO/public ingress implementation: not yet complete.
- Ready for R3-P5: yes.
- Ready to claim runtime identity admission complete: no.
