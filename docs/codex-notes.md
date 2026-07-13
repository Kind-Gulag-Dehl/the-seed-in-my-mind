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
