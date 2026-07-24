# TEMPO-005D-R3-P3 Stateful Admission Storage Transition Report

## 1. Scope and Stop Boundary

Implemented the storage-backed Profile-v0 sponsored `identity_create` transition. The
new storage-only entry point validates a completed P2 candidate against canonical state
at the allocated application position, then atomically records the identity, initial
direct key, four structural roots, three containment memberships, sponsor lineage,
event-derived provenance, and one invitation-capacity debit.

This pass does not add public ingress, API/DTOs, replay, snapshots, capacity generation,
qualifying-cycle behavior, VH/VI derivation, ordinary writer eligibility, restricted
verification events, key-rotate/revoke ingress, migrations for private accounts, or
frontend behavior.

## 2. Controlling Hash Confirmation

The controlling identity-admission specification SHA-256 matched before implementation
and remains unchanged after implementation:

```text
DD28615FB10D80D9D38BC2FB989973788627784A56AEA911472B32E8D42F73B1
```

## 3. Files Inspected and Changed

Inspected the P1/P2 reports; Profile-v0 Appendix A, encoding, authorship, replay,
verification, cycle, and API-contract specifications; existing storage/event-log/P2
modules; all current PostgreSQL migrations; account paths; and guarded database-test
conventions.

Changed implementation files:

- `backend/migrations/postgres/0023_profile_v0_identity_admission_storage.sql`
- `backend/crates/storage/src/profile_v0_admission.rs`
- `backend/crates/storage/src/lib.rs`
- `backend/crates/storage/src/types.rs`

The new migration is additive except for removing only the incompatible one-row-per-event
constraints on `ideas` and `connections`. It replaces them with non-unique source-event
and source-position indexes so one canonical admission event can create all four roots
and three memberships. Existing object identifiers and append-only behavior are preserved.

## 4. Stateful Validation Model

`Storage::apply_profile_v0_identity_create` accepts only a completed sponsor-authored
P2 `identity_create` candidate. It rejects `identity_verification_update` as
`compatibility_event_not_authorized` and rejects all other event types.

At the canonical application position it:

1. checks exact-event idempotence before current-state validation;
2. locks sponsor, target, initial-key reference, root IDs, and membership IDs;
3. requires a canonical sponsor with human-kind provenance;
4. requires the sponsor direct key to be active, non-revoked, and owner/reference
   consistent;
5. invokes P2 pure parsing, payload/signature/proof checks, and supplied-state
   duplicate/collision validation;
6. reads one explicit replay-replaceable sponsor materialization for profile, period,
   rulebook, inviter eligibility, invitation suspension, and available capacity;
7. keeps stale authorization distinct from key, eligibility, suspension, capacity,
   identity, key, and root errors.

The temporary `profile_v0_admission_state_materializations` table is append-only and has
no public, account, API, or operator-write path in this slice. Its only permitted
materialization class is `compatibility_replay_bridge`. P4 and later replay/cycle work
must replace it with replay-derived state; it is not `canonical_writer_level` and cannot
be treated as a durable authority override.

## 5. Storage Schema and Migration Summary

Migration 0023 introduces:

- `canonical_identity_provenance_v0`: closed human provenance classification;
- `profile_v0_identity_admissions`: Profile-v0 event facts and reduced admission context;
- `canonical_identity_structural_roots_v0`: the role, exact fixed title, and canonical
  root idea reference for Mindgarden, Backyard of Relationships, Self Tree, and Anthill;
- `canonical_identity_admission_lineage_v0`: direct sponsor-to-admitted lineage;
- `canonical_profile_v0_direct_key_history` and append-only key-state history;
- `profile_v0_admission_state_materializations`: temporary stateful-validation bridge;
- `profile_v0_invitation_capacity_debits`: one-unit, event-keyed debit ledger.

All new canonical tables have append-only triggers. Key references and raw key bytes are
globally unique in the new Profile-v0 registry. Validation also checks the existing
`canonical_identity_key_states` table so historical legacy key registration blocks reuse.

## 6. Atomic Transaction Behavior

After all validation succeeds, one database transaction writes the canonical event with
an absent speaker, target identity, event-derived human provenance, admission facts,
initial direct-key registry and active state, legacy key-state compatibility record, four
ordinary `conceptual_idea` roots, three ordinary `membership` connections, root projection,
lineage, and one debit-ledger row.

Every failure rolls the transaction back. An exact retry returns the original canonical
position and never creates another debit. A duplicate identity, historical key reuse, or
occupied root/connection is rejected before any Profile-v0 admission effect is committed.

## 7. Capacity-Debit Representation

Invitation capacity is not stored as money or a mutable account balance. The transition
records one append-only debit per accepted admission event. The temporary materialized
spendable amount is reduced by the sum of matching period/rulebook debit records under a
sponsor lock. Capacity generation, carryover, expiration, restoration, and liveness remain
replay/cycle work for P4 and later.

## 8. Root and Lineage Representation

The explicit P2 root plan is materialized exactly as:

```text
Mindgarden -> Backyard of Relationships
Mindgarden -> Self Tree
Mindgarden -> Anthill
```

Each root remains an ordinary `ideas` row with `conceptual_idea` type and the admitted
identity as speaker. The root projection records the closed structural role and exact
fixed title; it does not create a separate opaque identity graph. The three connection
roles are derived by list position, never author supplied.

## 9. Provenance and Compatibility Classification

New Profile-v0 admissions receive `event_derived` provenance. The closed schema also
reserves `genesis_admitted`, `legacy_operator_provisioned`, and `future_profile_derived`.
No migration fabricates Profile-v0 sponsors, applicant proofs, capacity debits, lineage,
or verification history for existing bootstrap/import/legacy rows. Existing identities
remain readable from `identities_s0` until an explicit replay/migration classification is
implemented.

## 10. Quarantined Old Paths

The new transition has no account ID parameter and makes no `accounts` write. It requires
absent `speaker_identity_id`; the sponsor is author and the applicant is only the target.
The existing account-coupled `create_canonical_identity` path remains legacy compatibility
behavior and cannot create a `profile_v0_identity_admissions` row, Profile-v0 roots,
lineage, or capacity debit. A guarded test exercises private account creation and verifies
that it produces none of those records.

`canonical_writer_level` remains outside this transition and is not read as admission,
verification, or inviter authority.

## 11. Database-Test Isolation Evidence

P3 database tests create an isolated `seed_test_tempo_005d_r3_p3_*` database in-process
only when `SEED_TEST_DATABASE_ADMIN_URL` is set. The helper permits only the `postgres`
maintenance database as its admin target, derives an isolated `seed_test_` URL, calls
`common::test_db_guard::require_disposable_database_url` before opening its application
pool, and never uses `seed_dev`, `seed_open_core`, or another application database.

This environment did not provide `SEED_TEST_DATABASE_ADMIN_URL` or `DATABASE_URL`; the
database matrix therefore skipped before opening any database connection. The protected
database guard and migration-discovery tests executed without a database.

## 12. Test Cases and Results

Implemented guarded storage coverage for:

- accepted admission, complete atomic effect set, and idempotent retry;
- invalid applicant possession proof with a valid sponsor signature and no writes;
- stale context, inactive key, revoked key, inviter ineligibility, suspension, and capacity
  exhaustion with distinct errors and no writes;
- duplicate identity, historical key reuse, and root collision with no partial writes;
- speaker presence with no admission writes;
- ordinary `identity_verification_update` rejection;
- private-account non-admission behavior;
- legacy row readability without fabricated Profile-v0 history;
- protected database URL rejection and migration-catalog discovery.

Executed checks:

- `cargo fmt --all`: passed.
- `cargo check -p storage --all-targets`: passed.
- `cargo test -p storage --lib -- --nocapture`: passed, 17 tests. Database-backed cases
  skipped safely because the explicit admin URL was absent.
- `cargo test -p event-log -p verification`: passed, 41 and 5 tests.
- `cargo test -p encoding`: passed, 10 tests.

Cargo emitted existing Windows incremental-cache hard-link warnings; compilation and tests
completed successfully.

## 13. Deferred Replay, Snapshot, and API Work

P4 must replace the temporary materialized sponsor state with deterministic replay,
rebuild Profile-v0 identity/provenance/root/key/lineage/debit state from history, and add
snapshot commitments and verification. Public write ingress, DTOs, public reads, and API
finality behavior remain deferred. Key rotate/revoke application, verification lanes,
qualifying cycles, capacity generation, VH/VI, invitation liveness, and public explanation
also remain out of scope.

## 14. Risks and Follow-Up

- The migration must be applied once in an explicitly isolated disposable PostgreSQL
  database to prove full SQL application and transaction behavior; no database service was
  configured for this run.
- P4 must make the materialization bridge replay-derived before exposing public admission
  ingress. It must not introduce an administrative eligibility path.
- Existing legacy account/import rows need a separately reviewed compatibility-classification
  migration. Their historical data must remain readable without invented Profile-v0 facts.
- The legacy account-coupled identity creator must be disabled or more strongly quarantined
  before public Profile-v0 ingress is exposed.

## 15. Recommended Next Task

`TEMPO-005D-R3-P4 - Deterministic Replay, Snapshot Projection, and Admission-State
Conformance`.

It should consume the P3 append-only event/projection facts, replace transitional admission
materializations with replay authority, and stop before public API/DTO changes.
