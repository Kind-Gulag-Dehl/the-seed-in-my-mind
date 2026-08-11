# Codex Notes

This file records Codex changes that affect code, configuration, scripts, API definitions, build behavior, exports, or runtime behavior.

Documentation-only coordination work does not require a notes entry unless it changes operational build or tooling behavior.

- 2026-07-10 21:26:16 -04:00 — TEMPO-004
  - Updated `scripts/tempo-cycle-fixture-harness.mjs` to validate the reconciled Tempo/Cycle conformance model: `T_allow` is derived structural support rather than truth certainty; passive timestamp evidence is normalized, deduplicated, capped, and never sufficient alone; zero-human state reports `record_only`.
  - Updated `docs/conformance/tempo-cycle-fixtures.v1.json` and schema to add structural-support profile fields and passive-evidence/zero-human vectors. `npm run conformance` and `npm run conformance:tempo-cycle` both pass 29/29 fixtures.

- 2026-07-11 12:41:21 -04:00 - TEMPO-005A-R1
  - Added the default-open-core signed canonical write substrate for ordinary `idea_create` and `connection_create` through a self-authenticating Profile-v0 candidate ingress at `POST /api/v1/canonical/events`.
  - Added Ed25519 Profile-v0 canonical byte construction, key descriptor hashing, strict signature verification, canonical JSON payload hashing, additive candidate/publication storage fields, and replayable identity key-state storage.
  - New public writes require identity-bound writer eligibility and active Profile-v0 key state; legacy identities without canonical key state remain readable but cannot submit new Profile-v0 writes through this ingress.
  - The ingress explicitly leaves Tempo predicates, cycle closure, beacons, certification, frontier, token, governance, mana, and lifecycle behavior unchanged.

- 2026-07-11 14:12:37 -04:00 - TEMPO-005C
  - Added deterministic Signature Profile-v0 conformance artifacts in `docs/conformance/canonical-event-signature-profile-v0.*` and a Rust verification test that reads the JSON vectors.
  - Added an env-gated isolated Postgres HTTP integration suite for signed `idea_create` and `connection_create` ingress; it requires `SEED_TEST_DATABASE_ADMIN_URL` and skips rather than touching a development database when that variable is absent.
  - Extended public canonical event-log DTOs to expose Profile-v0 audit fields and legacy/unsigned row classification.
  - Documented that current key-state and writer-eligibility rows are bootstrap/operator/test-provisioned open-core state until public identity/key and eligibility lifecycles are implemented.

- 2026-07-11 18:03:46 -04:00 - TEMPO-005C-R1
  - Hardened the signed-ingress isolated Postgres test harness so parallel test database names use a UUIDv7 suffix instead of timestamp-only uniqueness and each created isolated DB prints its generated name.
  - Fixed the isolated signed-ingress bootstrap fixture to use the seed bootstrap verifier identity for `canonical_writer_grant` and to materialize that verifier identity before inserting writer state, matching replay and storage FK constraints.
  - Corrected the duplicate-conflict route test so it submits a freshly signed candidate with the same `event_id` and different candidate bytes, and aligned the unsupported identity-idea negative case with the public `unsupported_event_type` error code.

- 2026-07-11 18:43:00 -04:00 - TEMPO-005C-R2
  - Added a reusable `common::test_db_guard` that rejects protected or ordinary database targets such as `seed_dev`, `seed_open_core`, and `postgres`, and accepts only approved disposable test targets.
  - Wired the guard into storage DB-backed tests, storage account/session DB tests, API-server shared test DB setup, and Stage 1 psql helpers so test code skips/fails closed before opening protected development databases.
  - Forensics matched the protected database contamination to API-server DB-backed test fixtures; final proof remains blocked by an external `cargo test`/`api_server` process outside the open-core worktree that was actively writing to `seed_dev`.

- 2026-07-11 19:18:00 -04:00 - TEMPO-005C-R2
  - Completed the protected-database isolation proof after the external writer exited: every required Rust package test was rerun with a fresh disposable database and preserved the protected `seed_dev` all-table count signature.
  - Executed the signed-ingress HTTP/database suite with no `SKIP` paths; valid signed `idea_create` and `connection_create`, negative validation, atomicity, idempotency, replay rebuild, and legacy migration cases passed against disposable databases.
  - Proved the generated open-core export and smoke build using only `seed_export_smoke`; `seed_dev` was unchanged, while the export reset/import/snapshot steps were confined to the disposable export-smoke database.

- 2026-07-13 17:10:34 -04:00 - TEMPO-005D-R3-P2
  - Added pure Profile-v0 admission cryptographic primitives and typed validators for sponsored `identity_create`, direct key rotation, and restricted direct-key revocation. The implementation includes canonical admission/root payload bytes, reduced authorization commitments, strict Ed25519 applicant/replacement proofs, completed-payload sponsor signature validation, and fixed public crypto fixtures.
  - This is not ingress or state application: legacy self/speaker and account-coupled identity paths remain unchanged until the P3 stateful storage and compatibility-quarantine slice. No database or network access is involved in the new module.

- 2026-07-13 18:22:07 -04:00 - TEMPO-005D-R3-P3
  - Added storage-only Profile-v0 sponsored-admission persistence with append-only provenance, direct-key history/state, structural-root, lineage, capacity-debit, and explicitly transitional replay-bridge records.
  - Removed legacy one-source-event uniqueness constraints that prevented an admission from creating four roots and three memberships, replacing them with source-event indexes. Public ingress, replay, snapshots, API/DTOs, capacity generation, and verification eligibility remain deferred.

- 2026-07-13 19:13:00 -04:00 - TEMPO-005D-R3-P4
  - Added a database-free Profile-v0 admission replay projector for sponsored identity creation, direct-key rotation/revocation, and manifest-only compatibility verification records. It derives human identity/provenance, keys, the complete structural-root set, memberships, sponsor lineage, restricted initial authority, and exactly one immutable capacity-debit fact from canonical candidates.
  - Added a deterministic Profile-v0 admission snapshot pack that commits identity/provenance, roots, key history, lineage, debit facts, compatibility records, lanes, and honest not-yet-derived capacity/liveness statuses. Existing database-backed replay and Stage-0 snapshot flows remain unchanged pending API/integration work.

- 2026-07-23 20:42:40 -04:00 - TEMPO-005D-R3-P3-DBV
  - Test-only disposable PostgreSQL isolation now uses the exact `seed_admission_p3_test_` prefix in the P3 storage helper, and `common::test_db_guard` recognizes that prefix.
  - The helper refuses cleanup for any name outside that prefix and emits `ISOLATED_DB_CLEANUP` status. This does not change application, migration, replay, or admission semantics.

- 2026-07-23 20:42:40 -04:00 - TEMPO-005D-R3-P3-DBV corrective update
  - Isolated admission tests now use fallible checks inside their asynchronous bodies, so `db.cleanup().await` runs before a failure is surfaced by the outer test. This preserves task-prefixed cleanup evidence even when a test condition fails.
  - The atomic test asserts one new identity relative to its fixture baseline. The 64-byte corrupted-proof case uses the established `applicant_proof_binding_mismatch` boundary; malformed proof encoding remains the separate invalid-proof path.

- 2026-07-23 22:00:29 -04:00 - TEMPO-005D-R3-P3-DBV validation complete
  - The authenticated guarded PostgreSQL matrix passed 10/10. All fourteen task-prefixed databases reported `differs_from_seed_dev=true` and matching `ISOLATED_DB_CLEANUP ... dropped=true` lines. No missing-admin skip occurred.

- 2026-07-24 09:45:00 -04:00 - INTEGRATION-ORDERING-001 / OPENCORE-ORDERING-001
  - Replaced the live authored Rail substrate with native `Ordering` validation, encoding/hashing, storage, replay, snapshot, import/export, verification, API/DTO, frontend-contract, and conformance surfaces.
  - Authored events are `ordering_create` and `ordering_fork`; profiles are `vine`, `evidence_rail`, and `action_rail`. Representation pointers remain in the snapshot representation index and outside the authored Ordering state root.
  - Added migration `0024_native_ordering_cutover.sql`, native Ordering conformance fixtures/harness, deterministic replay/snapshot/API/importer tests, and reviewer-facing implementation/conformance documentation.
  - Historical migrations and negative conformance cases retain old tokens only to document or prove rejection; there is no live compatibility layer.

- 2026-07-26 18:13:35 -04:00 - OPENCORE-IDENTITY-SOURCE-INTEGRITY-001 / INTEGRATION-SEED-V4-PILOT-001
  - Repaired only the eight-pass UTF-8/Windows-1252 mojibake in the authoritative Profile-v0 identity-admission specification and normalized its single CR to LF. No wording, headings, protocol semantics, or source classification changed.
  - Added a focused source-integrity harness that pins the repaired UTF-8/NFC/LF bytes and Unicode inventory, rejects BOM/replacement/mojibake/control patterns, proves reconstruction normalization is byte-stable, and proves the repaired source forward-round-trips to the frozen V3 hash.
  - Added one explicit non-authoritative negative fixture containing a single exact historical corrupted token; the harness requires it to fail with `known_mojibake_pattern`.

- 2026-07-26 18:35:00 -04:00 - OPENCORE-SEED-V4-PROFILE0-VALIDATION-001 / INTEGRATION-SEED-V4-PILOT-001
  - Added a separate `--validate-only` importer path for the exact unsigned, noncanonical Seed V4 pilot manifest. The path verifies the full manifest/component commitment set, deterministic UUIDv7 projections, record shapes, native Orderings, review/provenance state, DEC-037 derived ranks, relative contexts, and unsigned Profile-v0 event templates.
- V4 packages are rejected without `--validate-only`, and `--force` cannot be combined with validation. Successful validation returns before database configuration or canonical mutation setup; ordinary legacy Seed import behavior is unchanged.

- 2026-07-27 10:24:00 -04:00 - INTEGRATION-GATE0-RATIFICATION-001
  - Before genesis, ratified source documents and the bootstrap profile guide construction. After genesis, canonical event history, replay-derived graph state, and activated ordinary rulebook ideas are semantic authority; Markdown is provenance, conformance material, or a generated reading projection.
  - Rulebook identifiers reference ordinary rulebook ideas. Snapshots may commit the derived active set but never activate rules themselves.
  - Persistent public human graph content is canonical and rate constrained. Private journals/queues and Public AI/model/agent realms remain noncanonical product state and cannot affect replay.

- 2026-07-28 12:02:07 -04:00 - INTEGRATION-SEED-CONFORMANCE-BINDINGS-001 Open Core phase
  - Canonical `representation_create` now distinguishes one `title` slot from `description` cells. Title omits tier and vocabulary fields; descriptions use three named lengths by four named complexities; `vocabulary_version_id` is required exactly for canonical complexity.
  - Representation authorship is explicit and bound to the already-existing event speaker across validation, materialized storage, replay, snapshots, importer checks, and canonical reads.
  - Native Evidence/Action Rails now carry typed `subject_idea_id` and aligned roles. Standardized forks preserve subject, retained-item roles, and the Action Rail lane; Vines continue to omit standardized subject/role metadata.
  - Added migration `0025_seed_conformance_bindings.sql`, a canonical representation read endpoint, shared native Ordering and representation conformance suites, and exact commitment vectors. Disposable PostgreSQL application of migration 0025 remains pending because no guarded database URL was present.

- 2026-08-10 12:16:48 -04:00 - OPENCORE-M1-REVIEWER-REPAIR-001
  - Files changed: snapshot builder/verifier/library export; guarded migration/reviewer/backend/demo/export scripts; export manifest; package scripts; current Seed fixtures.
  - Description: aligned Stage-0 verification with representation-bearing snapshots, added a fixed golden regression and external artifact root, repaired bundled idea types, added a 19-case exact-prefix migration matrix, removed global process killing, rejected ordinary database targets, and isolated frontend/Cargo/snapshot/export outputs.
  - Commands/results: Rust bootstrap pass; snapshot-verifier tests 2/2 pass; script/JSON/fixture syntax checks pass. Guarded PostgreSQL/demo/export/full-suite execution remains pending because no process-local administrator URL was available and stored-secret access was rejected.
  - Result: code/tooling repair implemented; run blocked before database-backed completion.
- 2026-08-10 13:20:41 -04:00 - OPENCORE-M1-REVIEWER-REPAIR-001 complete
  - Files changed: snapshot builder/verifier/library; migration 0025 and guarded matrix; Seed fixtures; reviewer/backend/demo/export tooling; public status/registry/report/coordination docs.
  - Description: aligned representation-bearing state roots, enforced non-null description complexity, made reviewer and export execution exact-prefix/outside-repository isolated, and completed the inherited conformance gate.
  - Commands/results: full Rust, 67/67 conformance, 22/22 DTO, boundaries, identity integrity, frontend, 19/19 PostgreSQL matrix, source demo, exported demo, export verifier, and cleanup/catalog gates passed.
  - Result: implementation complete; no staging or checkpoint action performed.

- 2026-08-10 16:03:44 -04:00 - OPENCORE-PRODUCT-READ-CONTRACT-001 complete
  - Files changed: API handlers/router/helpers/mapping/tests/types; canonical Rust and TypeScript DTOs; storage read/query/types; reset/demo scripts; API contract/drift/export tooling; read-contract, boundary, status, and coordination docs.
  - Description: added API contract v1.0.0 capabilities and bounded, deterministic, snapshot-pinned product reads with a shared commitment-bearing basis; completed representation list/detail; added canonical-ID batch resolution and exact title/sentence lookup; added Open Core-owned schema and drift checks. No migration, write kind, canonical policy, or private behavior was added.
  - Commands/results: Rust fmt/build/all-target tests passed; focused guarded API database acceptance passed; PostgreSQL matrix 19/19 passed; conformance, 31-interface DTO drift, 19-endpoint API drift, boundaries, frontend 2/2/build, source demo, full external export verification, and exported smoke passed.
  - Result: complete on local branch; all disposable databases cleaned, protected database count preserved at 2, and no private repository or external system changed.
- 2026-08-10 21:12:47 -04:00 - OPENCORE-CANONICAL-HISTORY-TRANSFER-001 complete
  - Added the Open Core-owned canonical-history package/CLI with deterministic export, offline validate-only, guarded fresh-target import, idempotent retry, replay/snapshot equality, bounded validation, and negative fixtures.
  - Added exact task-prefixed PostgreSQL verification, reviewer/export integration, and canonical-only boundary enforcement; derived and private authority remain excluded.
  - Reconciled the package with committed recovery authority: it is a narrow canonical_history artifact, not a .seedpkg profile or Full Recovery implementation, and the recovery profile/schema/vectors are required public export members.
  - Commands/results: Rust format/build/all-target tests, focused 4/4 negatives, 67/67 conformance, identity integrity, 31 DTO contracts, 19 API endpoints, boundaries, frontend 2/2/build, guarded 499-event round trip, source demo, external export/smoke, provenance, and cleanup all passed.
