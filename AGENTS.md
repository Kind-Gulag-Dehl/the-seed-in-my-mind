# filepath: AGENTS.md

# Codex Agent Operating Contract (The Seed in My Mind Open Core)

This file defines the default operating rules for Codex in the public open-core repository. It is designed for local-first development, dirty working trees, parallel Codex conversations, and deliberate coordination with the private companion repository.

This repository is the authoritative working location for public/open-core materials:

```text
A:\the-seed-in-my-mind-open-core
```

The private companion repository is:

```text
A:\the-seed-in-my-mind
```

Do not infer current implementation status from GitHub or remote branches. The local working tree is the source of truth for current implementation state.

## 0) Current Open-Core Workstreams

This repository is authoritative for:

- public protocol and open-core specifications
- canonical encoding and hashing
- deterministic replay and merge
- snapshots and commitments
- node and conformance behavior
- public canonical APIs and DTOs
- open-core reference tooling
- reviewer and demo tooling
- export and boundary enforcement
- future public canonical runtime work

Current workstreams:

- Track A - Open-core runtime and conformance:
  - hardening deterministic ingestion
  - replay
  - snapshots
  - snapshot verification
  - read-only APIs
  - conformance vectors
  - boundary verification
  - reviewer-runnable infrastructure
- Track B - Canonical Tempo and Stage 1 foundation:
  - constrained canonical write ingress
  - human-authored canonical time contributions
  - Tempo claims and evidence
  - deterministic certainty and predicates
  - cycle shadow mode
  - eventual canonical cycle boundaries
  - challenge, voting, verdict, and expanded replay prerequisites
- Track C - Public documentation, reviewer tooling, and boundary maintenance:
  - authoritative public specifications
  - implementation-status accuracy
  - maps and reviewer guides
  - demo and quickstart flows
  - export cleanliness
  - public/private dependency boundaries
- Track D - Explicit cross-repository integration:
  - open-core-owned public changes implemented here first
  - downstream private integration performed deliberately afterward
  - shared task identifiers and merge ordering when both repositories change

The following normally do not belong in this repository:

- private accounts and sessions
- private ideas and private descriptions
- private overlays
- Builder product UX
- proprietary AI sandbox implementation
- document-import product workflows
- private shared-AI-map product code
- hosted services
- billing, support, or proprietary integrations
- proprietary game code or assets
- internal private planning
- private Seed-generation working files

## 1) Local-First Development and Git Boundaries

The user performs ongoing work directly in the local repositories and only occasionally pushes accumulated work to GitHub for backup.

- Do not fetch, pull, push, rebase, merge, switch branches, create branches, commit, amend commits, reset, restore, clean, stash, or otherwise alter Git history or remote state unless the user explicitly requests that Git operation.
- Do not use remote differences to overwrite or synchronize local files.
- Do not assume that a dirty working tree is accidental.
- Existing modified and untracked files may represent valid work accumulated over many prior tasks.
- Treat all unrelated local changes as protected, regardless of whether they are committed.
- Do not stage files. The user decides when accumulated work should be committed and pushed.
- `git status --short`, `git diff`, and other read-only Git inspection commands are permitted and required where specified.
- Read-only Git history commands are permitted when useful, but local file contents and current implementation evidence take precedence over older commits or remote state.
- If a file contains existing changes and the current task also needs to modify it, inspect the diff carefully and preserve all unrelated edits.
- If the current task cannot safely distinguish its intended edit from pre-existing local work, stop and report the conflict before modifying the file.

For cross-repository tasks, "merge order" means the required implementation and integration order between the two local repositories. It does not authorize running Git merge commands.

## 2) Coordination Preflight Before Modifying Tasks

Before any modifying Codex task, complete this preflight in order:

A. Run:

- `git status --short`

B. Read:

- `docs/codex-active-tasks.md`
- the latest relevant entries in `docs/codex-devlog.md`
- `docs/codex-notes.md` when touching code, configuration, scripts, exports, APIs, replay, or related surfaces
- applicable authority and implementation-status documents

C. Inspect all existing uncommitted changes.

D. Treat all unrelated changes as protected.

- Never assume an existing uncommitted change belongs to the current run.
- Shared repository visibility does not grant ownership over another task's work.
- Never restore, revert, overwrite, delete, stage, reformat, rename, or clean unrelated changes.
- Do not use broad Git cleanup, reset, restore, or staging operations on a dirty tree.
- If the requested task overlaps an active task, a claimed file, a tightly coupled shared surface, or unexplained existing modifications, stop and report the conflict before editing.

E. Register the task in `docs/codex-active-tasks.md` before editing.

F. Produce the Impact Analysis required below.

## 3) Scope and Impact Controls

Read access may cover the whole repository. Writes must remain within declared scope.

Before making edits, produce an Impact Analysis:

- intended changes
- exact files expected to change
- commands expected to run
- risks and repository invariants touched

If an additional file becomes necessary after the Impact Analysis, stop and explain why it is needed before modifying it.

Default scope rules:

- no unrelated refactors
- no formatting churn
- no unrequested file moves or renames
- no silent semantic reconciliation
- authoritative specs must be consulted for canonical changes
- derived documents must not introduce protocol semantics
- implementation-status documents must be updated when public implementation claims materially change

## 4) Cross-Repository Authority and Integration

Every potentially cross-repository task must determine ownership before editing.

Before editing, read the relevant authority indexes and boundary documents. Determine whether the surface is owned by:

- the open-core repository
- the private repository
- or deliberate integration across both repositories

Open-core-owned surfaces include:

- protocol semantics
- public schemas
- canonical event definitions
- encoding and hashing
- deterministic replay
- snapshots and commitments
- node conformance
- public canonical APIs
- public canonical DTOs
- public reference tooling
- boundary and export tooling

Private-owned surfaces include:

- private product UX
- Builder
- private overlays
- private accounts and storage
- proprietary AI features
- document import
- shared non-canonical AI product features
- hosted integrations
- proprietary game experiences

An authoritative open-core change must not be implemented first in the private repository and later copied back. Private product behavior must not be copied into the public package merely because the private repository consumes the open core.

When both repositories must change:

- use the same `INTEGRATION-*` task ID in both active-task ledgers
- register the task in both repositories before editing either
- identify the authoritative repository
- state the repository role in each ledger
- state the merge order
- implement and validate the authoritative change first
- perform downstream integration second
- do not push either repository as part of the task

The normal merge order for open-core-owned changes is:

```text
authoritative public specification or contract
-> open-core implementation
-> open-core tests and conformance
-> public API or DTO
-> private repository integration
-> private product UI
```

The normal order for private-owned changes is:

```text
private product design
-> private storage/API
-> private frontend/Builder workflow
-> no open-core change unless a genuine public boundary is affected
```

If ownership is unclear, stop and ask before modifying either repository.

## 5) Boundary Protection Rules

Before any task that may alter the public/private split, cross-check:

- `docs/authoritative-index.md`
- `docs/open-core-boundary-manifest.md`
- `docs/open-core-split-and-data-boundary-spec.md`
- `docs/open-core-implementation-status.md`

Explicitly confirm that:

- private DTO crates do not enter open-core dependency direction
- private frontend modules do not enter the reference frontend
- private AI, Builder, accounts, overlays, or game code do not enter the export
- internal planning and private generated data do not enter public export surfaces
- open-core implementation claims remain aligned with the implementation-status document

## 6) Build and Verification Requirements

Run relevant checks based on changed surfaces. Do not run broad build or application checks for documentation-only coordination changes unless the user requests them.

### 6.1 Rust / Backend / Canonical Runtime

Before Rust tooling on Windows:

- `$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"`
- `where cargo`
- `cargo --version`
- `where rustc`
- `rustc --version`

Relevant checks:

- `cargo build`
- relevant `cargo test`
- replay/snapshot verification when applicable
- reviewer or Stage 0 verification when canonical ingest, replay, snapshots, API, ordering, encoding, or storage changes

### 6.2 Frontend / Reference Viewer

Relevant checks:

- relevant `npm run build`
- meaningful tests where present
- boundary checks
- basic smoke verification when practical

### 6.3 Public Boundary / Export

Relevant checks:

- `npm run verify:boundaries`
- `npm run verify:canonical-dto` when DTOs change
- appropriate export cleanliness or generated-export verification when export surfaces change

## 7) Completed-Run Logs

`docs/codex-devlog.md` is append-only completed-run history. Do not rewrite or normalize existing entries.

Every Codex run must append:

- local timestamp
- active task ID
- track name
- repository name
- repository role
- Codex conversation label when supplied
- whether active-task overlap was detected
- whether the task was cross-repository
- counterpart task ID when applicable
- task summary
- files changed
- commands and results
- unresolved risks and follow-ups

`docs/codex-notes.md` is required whenever code, configuration, scripts, API definitions, build behavior, exports, or runtime behavior changes.

Do not require a notes entry for documentation-only work unless the change affects operational build or tooling behavior. Preserve existing history.

## 8) End-of-Run Requirements

Before ending a successful modifying task:

- append the completed-run devlog entry
- append Codex notes when required
- remove or update the matching active-task entry
- run `git status --short`
- report unrelated remaining changes
- report any cross-repository follow-up and required merge order

For blocked work:

- keep the active task registered
- change status to blocked
- record the blocker
- do not overwrite another task's files

Final response structure:

1. Changed files
2. Key diffs
3. Commands run and results
4. Failures and resolution
5. Follow-ups and risks
6. Cross-repository impact and merge order
7. Pasteback block for ChatGPT

## 9) Optional Scoped Instruction Files

Scoped `AGENTS.md` files may be useful in directories such as:

- `backend/`
- `frontend/open-core-reference/`
- `docs/`
- `tools/open-core/`

Do not create them merely for completeness. Only create a scoped file if:

- the directory has materially different commands or invariants
- the root file would otherwise become confusing
- the scoped instructions are short and clearly useful

Prefer keeping coordination in this root file unless there is a strong reason to add scoped instructions.

## 10) No Complex Lock System

The active-task ledger is advisory coordination.

Do not add:

- background services
- Git hooks
- lock daemons
- new dependencies
- CI changes
- automation scripts
- scheduled processes
- database-backed locking
- network coordination

The goal is a small, inspectable workflow.

## 11) Safety and Confidentiality

- Do not introduce instructions or code that facilitate wrongdoing or unsafe behavior.
- Do not add secrets to the repo.
- Do not move private/product material into the public open-core repository.
