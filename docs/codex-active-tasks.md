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

### TEMPO-005D - Canonical identity and Profile-v0 key lifecycle

- Track: Canonical Tempo and Stage 1
- Repository: the-seed-in-my-mind-open-core
- Repository role: authoritative public/open-core
- Codex conversation label: Canonical Tempo Codex
- Status: blocked
- Started: 2026-07-11 19:29:17 -04:00
- Last updated: 2026-07-11 19:39:00 -04:00
- Goal: Implement the smallest deterministic canonical identity and Ed25519 Profile-v0 key lifecycle required before future verified-human canonical contributions.
- Intended files/directories:
  - authority documents listed by TEMPO-005D (read-only unless a blocker requires reporting; no spec semantic edits intended)
  - `backend/crates/encoding/**` (inspect/edit only if existing canonical encoding helpers need event payload support)
  - `backend/crates/event-log/**` (event validation/registry support)
  - `backend/crates/replay/**` (replay-derived identity/key state)
  - `backend/crates/storage/**` (additive key lifecycle storage/materialization/migrations/tests)
  - `backend/crates/verification/**` (Profile-v0 descriptor/signature validation reuse; edit only if lifecycle support needs it)
  - `backend/crates/api-types-canonical/**` (public DTO/read fields for safe identity/key state)
  - `backend/bins/api-server/src/server/**` (canonical ingress dispatch, public reads, tests)
  - `backend/migrations/postgres/**` (additive migrations only if required)
  - `frontend/src/shared/types/canonical.ts` and DTO drift manifests if public DTOs change
  - `docs/open-core-implementation-status.md` and `docs/api-contract-read-only.md` if implementation status/API claims change
  - `docs/codex-active-tasks.md`, `docs/codex-devlog.md`, `docs/codex-notes.md` (coordination)
- Shared surfaces touched:
  - signed canonical event ingress, identity/key-state projection, replay validation, public canonical DTOs, database-backed test isolation.
- Counterpart repository task:
  - none; private repository must not be modified.
- Dependencies:
  - TEMPO-005B, TEMPO-005A-R1, TEMPO-005C, TEMPO-005C-R1, and TEMPO-005C-R2 completed.
- Required merge order:
  - open-core identity/key lifecycle before canonical verification/writer lifecycle; Tempo claim recording remains later.
- Notes:
  - Stop before implementation if current authority does not settle identity-create author/authorization, initial key descriptor fields, bootstrap signature validation, rotation replacement/addition, revocation authorization, last-key/recovery, or key-management eligibility.
  - All DB-backed tests must use disposable databases accepted by `common::test_db_guard`.
- Blocker:
  - Current authority does not settle the exact `identity_create` author/authorization rule. Appendix A defines payload fields/effects but not who authors or authorizes the event; Verification says identities may exist only after invite; Protocol v5 still contains older wording where a new user registers/emits the identity.
  - Current authority does not settle `identity_create` signature bootstrap. It does not choose between inviter/verifier-authored creation, self-signed bootstrap exception, or bootstrap publisher authority.
  - Current authority does not settle whether `identity_key_rotate` adds another simultaneously active key or supersedes/replaces the previous key. The authorship profile mentions active and superseded states, while Appendix A only says the new key becomes active.
  - Current authority does not clearly state whether direct key-management events require ordinary canonical-writer eligibility or only active key control for the identity.

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
