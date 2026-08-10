# INTEGRATION-SEED-CONFORMANCE-BINDINGS-001 — Open Core handoff

Status: Open Core implementation, conformance, and guarded PostgreSQL validation complete. The private counterpart remains downstream for the exact snapshot-verifier and migration-0030 obligations recorded below.

## Settled current-profile contract

- A target has one separate title representation and twelve description cells.
- `representation_kind` is exactly `title` or `description`.
- Title omits `tier_length`, `tier_complexity`, and `vocabulary_version_id`.
- Descriptions use `sentence | paragraph | full` and
  `fundamental | standard | advanced | canonical`.
- `vocabulary_version_id` is present exactly for canonical-complexity descriptions,
  references an already-existing ordinary idea, and is never inferred or defaulted.
- `author_identity_id` equals the representation event speaker and that identity
  already exists at the event position.
- A Vine omits standardized subject and role fields.
- An Evidence Rail has an existing `truth_claim` subject and aligned
  `potential_evidence | actual_evidence` roles.
- An Action Rail has an existing `actionable_idea` subject and one homogeneous
  `potential_action` or `proposed_action` lane. Potential and proposed remain separate
  Orderings.
- Standardized forks preserve profile, subject, retained-item roles, and Action lane,
  even when the fork retains no base item.

## Implemented surfaces

Authority, canonical payload encoding/hashing, event validation, storage migration and
read projections, deterministic replay, snapshot state commitments, public canonical
API/DTO mapping, frontend canonical contracts, Seed importer validation, and executable
conformance fixtures were updated as one upstream contract.

Migration `0025_seed_conformance_bindings.sql` adds representation vocabulary binding,
nullable title complexity with a strict slot-shape constraint, author/vocabulary
pre-use checks, Ordering subjects, item roles, typed subjects, standardized duplicate
rejection, and fork-preservation checks.

The public read surface now includes
`GET /api/v0/representation/:representation_id`. Title responses omit tier and
vocabulary fields; canonical descriptions preserve the explicit vocabulary idea ID.

## Conformance and validation

- Full Rust workspace: 147 passed, 0 failed.
- Workspace all-target compile check: pass.
- Event log: 44/44.
- Replay: 24/24.
- Snapshot: 9/9.
- Seed importer: 6/6.
- JavaScript conformance: 67/67:
  - Tempo/Cycle: 29/29.
  - Native Ordering: 20/20.
  - Representation bindings: 18/18, including 12/12 description cells.
- Canonical DTO agreement: 22/22.
- Open Core frontend boundary check, tests 2/2, and production build: pass.
- Exact V4 pilot validate-only: pass with `canonical_writes=0`; 50 ideas,
  600 representations, 84 connections, three Orderings, 50 universal profiles,
  120 relative contexts, and digest
  `8cca9190326dd6afe4519b344c15b9274c146bee74398efa212a16f981862880`.

The native Ordering hash vector commits the explicit subject and item-role list at
BLAKE3
`92dca3941dfc608931972b0bf2de810ffbab9de0502b2275ccceda4e0bd6a42f`.
Representation commitment vectors independently change when author or vocabulary
changes.

## Focused proof-gap closure

The follow-up evidence review found no missing runtime rule and required no policy or
migration expansion.

Canonical-description vocabulary binding is enforced by event position, not by UUID
shape or by presence somewhere in a materialized input map:

- Seed importer test
  `canonical_description_rejects_a_forward_vocabulary_reference` proves that a
  syntactically valid UUID is rejected until the referenced ordinary idea is in the
  importer's already-seen idea set.
- Replay test `replay_rejects_a_materialized_but_later_vocabulary_idea` supplies the
  vocabulary row in the complete materialized map but places its `idea_create` after
  the representation event; replay rejects it with `missing_vocabulary`.
- Unchanged migration 0025 function
  `seed_validate_representation_conformance_bindings()` requires the referenced
  ordinary `ideas` row position to be strictly earlier than the representation
  position. Its SHA-256 remains
  `66970ECE9B95391DA0792901CF29D47514DE9D31A565AAF7C813B2995E88FEB1`.

No runtime "ratified vocabulary" state was added. The accepted current-profile
contract requires an explicit pre-existing ordinary idea. Selection or ratification
of which ordinary idea is eligible to serve as the Seed vocabulary version remains a
governance and generation gate.

A positive title representation is now covered explicitly at each non-database
boundary:

1. Importer test `title_representation_projects_as_a_separate_slot`.
2. Replay test `title_representation_replays_as_a_separate_slot`.
3. Snapshot test
   `title_representation_is_committed_as_a_distinct_snapshot_record`.
4. API/DTO test
   `title_representation_maps_to_the_public_dto_without_description_fields`.

Focused results: Seed importer 8/8, replay 26/26, snapshot 10/10, and API server
27/27. Exact-file `rustfmt --check` passed for the four touched Rust files. The
workspace-wide format check remains affected only by unrelated pre-existing
formatting differences in `api-server` private-handler/helper/test files; those files
were not modified by this follow-up.

## Pending guarded database evidence

This process had neither `SEED_TEST_DATABASE_ADMIN_URL` nor `DATABASE_URL`. No
database was contacted. Migration 0025 was statically reviewed, and an Action Rail
trigger defect found during review was corrected so base-lane comparison applies only
to actual forks. Before integration is claimed database-complete, apply migrations
through 0025 twice to a fresh approved disposable database and exercise valid and
invalid title/description, author/vocabulary pre-use, typed subject, aligned role, and
fork-preservation cases.

## Preservation and boundaries

Historical migrations and frozen/previous Seed artifacts were not rewritten. Existing
Gate 0, identity-source-integrity, V4, Tempo, and unrelated dirty work remains in
place. No private repository file, canonical database, event log, Seed import, genesis,
signature, publication, bundle, deployment, Git index, commit, branch, push, or PR was
changed.

## Exact downstream handoff

The private counterpart may now bind its importer/API/frontend consumption to this
contract while preserving the zero-write V4 path. It must not claim the shared task
complete until migration 0025 has guarded disposable-PostgreSQL evidence and the
private bindings pass their own test matrix.

## 2026-08-10 reviewer-repair update

The authoritative Open Core snapshot verifier omitted the representations section even though the builder committed it. The verifier now requires the same four state sections as the builder, and a fixed representation-bearing golden regression passes 2/2. Both bundled current-profile Seed fixtures now carry valid idea_type values for every idea_create.

Reviewer, backend, demo, and export tooling now rejects ordinary database targets, owns only spawned processes, preserves source frontend dependencies and lockfiles, and uses temporary Cargo, frontend, snapshot, and export outputs. A 19-case migration-0025 PostgreSQL matrix covers valid/invalid title and description slots, author/vocabulary pre-use, typed subjects, roles, duplicates, Action lane homogeneity, and fork preservation.

The database gate is still pending. This process did not inherit SEED_TEST_DATABASE_ADMIN_URL, and safety review correctly rejected reading the administrator credential from the private repository because the task forbids touching secrets. Run the following from an already authenticated user-owned PowerShell process without pasting the URL:

    npm run verify:seed-bindings-db

The inherited task remains open until that command reports 19 cases, exact seed_opencore_m1_reviewer_repair_001_* cleanup with dropped=true, and unchanged seed_admission_p3_test_32944_% counts.
## 2026-08-10 guarded reviewer completion

- Migration 0025 applied twice on an exact `seed_opencore_m1_reviewer_repair_001_*` disposable database. Its 19-case matrix passed valid/invalid title and description slots, author/vocabulary pre-use, typed subjects, item roles, duplicates, Action-lane homogeneity, and fork preservation.
- The matrix exposed and repaired a PostgreSQL CHECK/NULL defect: descriptions now require `tier_complexity IS NOT NULL`, matching the already-authoritative representation contract.
- The current reviewer fixture imported 22 events, 9 ideas, and 12 connections; independent snapshot verification passed with builder/verifier agreement over ideas, connections, Orderings, and representations.
- Source and exported reviewer demos, read-only API checks, reference frontend 2/2 tests/build, conformance 67/67, DTO 22/22, boundaries, identity-source integrity, full Rust workspace build/tests, and export cleanliness passed.
- Exact task-prefix catalog absence was verified after the runs. The two pre-existing `seed_admission_p3_test_32944_*` databases remained present and unchanged in count.
- No private file, persistent database, `seed_dev`, secret, Git index/history, release zip, or genesis artifact was touched.

Downstream private obligations: add representations to the authoritative snapshot verifier section set and golden/missing-artifact regressions; add the explicit non-null description-complexity guard to private migration 0030; then rerun private guarded import/replay/snapshot verification and shared DTO checks.