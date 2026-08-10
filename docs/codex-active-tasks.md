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
