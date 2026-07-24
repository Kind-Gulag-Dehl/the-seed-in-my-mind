# Active Codex Tasks

This file coordinates Codex work currently in progress in the open-core repository.

It is mutable operational state. Completed work belongs in the append-only
`docs/codex-devlog.md`.

## Rules

- Every modifying Codex run must register here before editing.
- Read every active entry before registering a task.
- Use a unique task ID beginning with:
  - `OPENCORE-`
  - `TEMPO-`
  - `CONFORMANCE-`
  - `DOCS-`
  - `INTEGRATION-`
- State the intended files or directories before editing.
- State any shared or cross-repository surfaces.
- Do not modify a file or tightly coupled surface claimed by another active task.
- If overlap is required, stop and report the conflict before editing.
- Preserve unrelated uncommitted changes.
- Do not stage, restore, revert, overwrite, delete, or reformat another task's work.
- At successful completion, append the completed-run devlog entry and then remove the active entry.
- Blocked tasks remain listed with `Status: blocked` and an explanation.
- Cross-repository tasks must use the same `INTEGRATION-*` task ID in both repositories.

## Active

### TEMPO-005D - Tempo-specific follow-on authority after DEC-043 handoff

- Track: Canonical Tempo and Stage 1
- Repository: the-seed-in-my-mind-open-core
- Repository role: authoritative public/open-core
- Codex conversation label: Canonical Tempo Codex
- Status: blocked
- Started: 2026-07-11 19:29:17 -04:00
- Last updated: 2026-07-24 09:45:00 -04:00
- Goal: Retain only unresolved Tempo-specific authority work that follows, and does not overlap, the accepted DEC-043 Profile-v0 identity-admission implementation and ordering handoff.
- Intended files/directories:
  - `docs/tempo-spec.md`
  - `docs/cycle-spec.md`
  - `docs/planning/tempo-cycle-canonical-schema-and-replay-resolution.v1.md`
  - `docs/codex-active-tasks.md`, `docs/codex-devlog.md`, `docs/codex-notes.md` (coordination)
- Shared surfaces touched:
  - Tempo and certified-cycle authority only; Profile-v0 identity admission, ordering, and their runtime surfaces are explicitly excluded.
- Counterpart repository task:
  - none; private repository must not be modified.
- Dependencies:
  - TEMPO-005D-R3-P3-DBV completed; its narrow admission-storage validation claim is released.
  - DEC-043 and `INTEGRATION-ORDERING-001` are complete; released cross-repository Ordering surfaces remain outside this Tempo task.
- Required merge order:
  - Completed DEC-043 native Ordering cutover -> later separately scoped Tempo-specific task, if still required.
- Notes:
  - Completed outside this task: all released authority/index/protocol admission paths; `backend/crates/encoding/**`, `event-log/**`, `identity/**`, `verification/**`, `storage/**`, `replay/**`, `snapshot/**`, `api-types-canonical/**`, `common/**`, `tests/**`, and `tooling/**`; `backend/bins/api-server/**`, `seed-importer/**`, `snapshot-builder/**`, `snapshot-verify/**`, and `verify-replay/**`; `backend/migrations/postgres/**`; frontend canonical type/client paths; API/DTO, conformance, implementation-status, boundary, export, Ordering-script, and related coordination surfaces.
  - The released dirty work was preserved through the completed Ordering cutover without reset, restore, staging, deletion, or mass-formatting.
- Blocker:
  - Do not resume this parent task against released DEC-043 or ordering surfaces. A separate, explicitly scoped Tempo authority task is required before any retained-path implementation work.

### TEMPO-005A - Clean open-core canonical reasoning write substrate

- Track: Canonical Tempo and Stage 1
- Repository: the-seed-in-my-mind-open-core
- Repository role: authoritative public/open-core
- Codex conversation label: Canonical Tempo Codex
- Status: blocked
- Started: 2026-07-11 11:11:06 -04:00
- Last updated: 2026-07-11 11:57:56 -04:00
- Goal: Implement the smallest secure, deterministic, default-open-core canonical reasoning write substrate for signed `idea_create` and `connection_create`.
- Intended files/directories:
  - `docs/codex-active-tasks.md` (coordination)
  - `docs/codex-devlog.md` (coordination)
- Shared surfaces touched:
  - none; runtime/source/spec/test files were inspected read-only and not modified.
- Counterpart repository task:
  - none.
- Dependencies:
  - Authority dependency resolved by TEMPO-005B: `docs/canonical-event-authorship-and-signature-profile-v0.md` now defines Signature Profile v0, signed authored-candidate bytes, `public_key_ref`, and replay-derived key state.
- Required merge order:
  - specification/authority decision before open-core runtime implementation; private downstream integration after open-core substrate.
- Blocker:
  - Original TEMPO-005A run stopped before implementation because authority was incomplete at that time. That authority blocker is resolved by TEMPO-005B; runtime work should resume as a new task, recommended ID `TEMPO-005A-R1`, rather than continuing the old stopped run.

## Entry Template

### TASK-ID — Short title

- Track:
- Repository:
- Repository role:
- Codex conversation label:
- Status: active
- Started:
- Last updated:
- Goal:
- Intended files/directories:
  - path
- Shared surfaces touched:
  - none
- Counterpart repository task:
  - none
- Dependencies:
  - none
- Required merge order:
  - none
- Notes:
