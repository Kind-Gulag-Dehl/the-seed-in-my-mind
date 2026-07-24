---
title: INTEGRATION-ORDERING-001 / OPENCORE-ORDERING-001 Cutover Report
status: complete
date: 2026-07-24
decision: DEC-043
authority: Open Core canonical protocol and runtime
---

# Native Ordering Cutover Report

## Outcome

DEC-043 is implemented as one authored `Ordering` substrate. The only authored event types are `ordering_create` and `ordering_fork`; every object has an `ordering_id` and explicit named `ordering_profile`. `vine` is the open-ended ecosystem-facing profile. `evidence_rail` and `action_rail` are standardized profiles, not separate substrate types.

There is no live `rail_*` compatibility API, event alias, DTO, storage path, replay path, or dual-read/write path. Derived relative-importance, navigation, chronology, visual-layout, and event-position orderings remain derived.

## Cutover surfaces

- Authority: Protocol v5, Appendix A, canonical encoding/hashing, replay, snapshot, event registry, API, node/conformance, publication/boundary, implementation-status, maps, and cross-document invariants.
- Runtime: event validation and registry agreement, canonical payload hashing, importer/exporter and verifier paths, replay state/application, and snapshot construction/verification.
- Storage: `orderings`, `ordering_items`, `private_orderings`, and `private_ordering_items`; native profile fields and constraints; no compatibility views.
- API/DTO: `/api/v0/ordering/:ordering_id`, `/api/v0/idea/:idea_id/orderings`, and `/api/v0/me/orderings`; canonical/private Ordering DTOs and shared TypeScript contracts.
- Conformance: native Ordering JSON Schema, ten shared vectors, JavaScript harness, and Rust event-log consumption of the same vectors.
- Migration: `backend/migrations/postgres/0024_native_ordering_cutover.sql`.

## Handed-off work preserved

The existing dirty Profile-v0 admission implementation, specifications, conformance vectors, migration `0023`, Tempo reports, and all unrelated worktree changes were preserved in place. No reset, restore, deletion, staging, mass-format, or overwrite was performed.

## Verification

- Open Core Rust workspace: 139 passed, 0 failed (`--all-targets --test-threads=1`).
- Native Ordering conformance: 10/10 passed.
- Tempo/Cycle conformance regression: 29/29 passed.
- Canonical DTO agreement: 20/20 interface contracts.
- Open Core reference frontend: boundary check passed, 2/2 tests passed, production build passed.
- Canonical Ordering hash vector: BLAKE3 `318597c0fd62c1971cb954153edd24faa061150efd35db0bcc0119bc96fc81f1`.
- Replay/import/snapshot tests cover named profiles, numeric-profile rejection, deterministic create/fork reconstruction, representation-index exclusion from the state root, and invalid profile metadata.
- Clean disposable Postgres `seed_test_ordering_001_open_core`: migrations `0001` through `0024` applied from empty, a second bootstrap applied nothing, native tables were present, legacy tables were absent, and the database-backed private Ordering isolation test passed.

## Intentional Rail terminology

- Ecosystem profile names **Evidence Rail** and **Action Rail**, encoded as `evidence_rail` and `action_rail`.
- Explicit negative conformance cases proving `rail_create`, `rail_fork`, and the removed representation-update aliases are rejected.
- Historical migration filenames, migration SQL/notes, completed reports/devlogs, and historical archives.
- Ordinary English or UI-layout terms such as “guardrail” or a visual rail are not protocol substrate identifiers.

## DEC-043 Seed row dispositions

Sixteen stable rows are regenerated as native Ordering planning members:

1. `rail-object`
2. `protocol-v5-appendix-a-section-041-a4-2b-rail-events`
3. `protocol-v5-appendix-a-section-042-a4-2b-1-rail-create`
4. `protocol-v5-appendix-a-section-043-a4-2b-2-rail-fork`
5. `deterministic-replay-and-merge-spec-section-030-5-4-rail-event-application-semantics`
6. `rail-add`
7. `canonical-rail-object`
8. `rail-fork-lineage`
9. `fork-only-rail-mutation-model`
10. `rail-targeted-representation-challenges`
11. `rail-create-event`
12. `rail-fork-event`
13. `rail-concurrent-activity-merged-by-event-order`
14. `rail-representation-index-is-index-only`
15. `rail-step-metadata-via-connection-reference`
16. `snapshot-rail-record-ordered-items`

Two removed alias rows remain historical-only:

17. `protocol-v5-appendix-a-section-044-a4-2b-3-rail-update-representation`
18. `rail-update-representation-compatibility-alias-event`

Stable slugs and provenance remain unchanged for audit continuity. Final Seed content generation has not begun.

## Protected Tempo boundary

`docs/tempo-spec.md`, `docs/cycle-spec.md`, and `docs/planning/tempo-cycle-canonical-schema-and-replay-resolution.v1.md` were not modified by this task. Their final SHA-256 hashes exactly match the pre-edit baseline:

- `tempo-spec.md`: `3407193F0E5547202F0424E6DA32E8283FD654F9D9AA2902B61BC01BB110829D`
- `cycle-spec.md`: `905EC16DC60F0C23EC8E28A578AAB67B67DA9CA67E8E7E089C557A82626FBA79`
- Tempo planning resolution: `7CC60A89C3A1ADCC3F17551F2F3A6999818A5C172047C86AB5C20FA52ACCA7AB`
