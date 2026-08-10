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

### OPENCORE-CANONICAL-HISTORY-TRANSFER-001 ? Canonical history transfer tooling

- Track: Track A / Track C canonical runtime and reviewer tooling
- Repository: the-seed-in-my-mind-open-core
- Repository role: authoritative public/open-core producer
- Codex conversation label: OPENCORE-CANONICAL-HISTORY-TRANSFER-001
- Status: active
- Started: 2026-08-10 16:41:55 -04:00
- Last updated: 2026-08-10 16:41:55 -04:00
- Goal: Add guarded deterministic canonical-event history export, offline validation, fresh-database import, replay/snapshot comparison, negative fixtures, and reviewer/export integration without copying derived or private authority.
- Intended files/directories:
  - backend/Cargo.toml
  - backend/Cargo.lock
  - backend/crates/common/src/test_db_guard.rs
  - backend/crates/canonical-history/**
  - backend/bins/canonical-history-transfer/**
  - scripts/verify-canonical-history-transfer-postgres.ps1
  - scripts/check-open-core-boundaries.mjs
  - scripts/grant-reviewer-quickstart.ps1
  - package.json
  - tools/open-core/export-manifest.json
  - tools/open-core/canonical-history-package.v1.schema.json
  - docs/conformance/canonical-history-transfer-negative.v1.json
  - docs/canonical-history-transfer.md
  - docs/open-core-reviewer-guide.md
  - docs/open-core-implementation-status.md
  - docs/open-core-boundary-manifest.md
  - docs/codex-active-tasks.md
  - docs/codex-notes.md
  - docs/codex-devlog.md
- Shared surfaces touched:
  - canonical event ordering, authored-candidate audit bytes, replay/snapshot verification, public export/reviewer packaging
- Counterpart repository task:
  - downstream private extractor integration required after this Open Core producer commit; private repository is not modified by this task
- Dependencies:
  - exact clean producer commit 4483b66cb8bf66a42c18cbe7881c513a3bc0f262
- Required merge order:
  - Open Core package/tooling and conformance first -> downstream private read-only extractor/integration second
- Notes:
  - No active-task or dirty-worktree overlap detected at registration.
  - Fail closed on unsupported event families, nonportable signing authority, migration mismatch, non-fresh targets, or any canonical encoding/replay authority conflict.


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
