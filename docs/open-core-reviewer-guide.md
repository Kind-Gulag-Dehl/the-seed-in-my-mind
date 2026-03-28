---
doc_id: open_core_reviewer_guide
title: Open-Core Reviewer Guide
status: derived
version: v0
last_reviewed: 2026-03-05
scope:
  - Gives grant reviewers a short, deterministic evaluation path for the public open-core package.
authoritative_for:
  - Reviewer workflow only.
not_authoritative_for:
  - Protocol semantics
depends_on:
  - open-core-demo-flow.md
  - open-core-architecture-overview.md
  - open-core-implementation-status.md
  - stage0-runtime-configuration.md
---

# Open-Core Reviewer Guide

## 1. What to evaluate

The public package is meant to prove four concrete things:

1. canonical data can be ingested deterministically,
2. replay and snapshot commitments are reproducible,
3. a read-only reference node can serve verified state,
4. the open-core boundary is enforced and exportable.

Some additional protocol/spec documents in `docs/` are published for architectural transparency. Reviewers should use this guide and [open-core-implementation-status.md](open-core-implementation-status.md) to distinguish runnable public surfaces from future/spec-only design material.

## 2. Prerequisites

- Windows PowerShell
- Rust stable
- Node.js 20+
- Postgres with `psql` on PATH
- `DATABASE_URL` set in the environment, or `backend/.env` created from `backend/.env.example`

When you run the repo-level reviewer scripts from the source tree, they will load `backend/.env` automatically if the variable is not already set. The exported package does not ship a real `.env`; reviewers should copy `backend/.env.example` to `backend/.env` or set `DATABASE_URL` directly before running the exported runtime demo.

## 3. 15-minute path

From the repo root:

```powershell
npm run review:quickstart
```

If you do not already have a local Postgres database configured, see the minimal setup section in `README.md` or `docs/stage0-runtime-configuration.md`.

This path runs:
- boundary verification,
- canonical DTO drift verification,
- the deterministic open-core demo report.

What you should see:
- boundary checks pass,
- DTO checks pass,
- the demo resets the database, imports a tiny public seed, builds and verifies a snapshot, starts the read-only API, and prints the imported idea titles and resulting snapshot commitments.

## 4. 30-minute path

From the repo root:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/grant-reviewer-quickstart.ps1 -Profile 30
```

This path adds:
- reference frontend install/test/build,
- broader open-core backend surface verification.

What you should see:
- the same deterministic demo report,
- a successful build of the reference frontend,
- a successful open-core backend verification pass.

## 5. Public export verification

To verify the publishable open-core artifact:

```powershell
npm run extract:open-core
```

That command must produce:
- `_export/open-core/`
- `_export/open-core/EXPORT_INFO.txt`
- `tools/open-core/dist/open-core-export.zip`

It also re-runs:
- export cleanliness verification,
- exported backend build/test,
- exported reference frontend build/test,
- exported-runtime smoke verification.

## 6. What is intentionally out of scope

This review package does not claim that the project is already:
- a live multi-node decentralized network,
- a finished governance runtime,
- a finished token/economics runtime,
- the full game/product experience.

For that status split, see [open-core-implementation-status.md](open-core-implementation-status.md).
