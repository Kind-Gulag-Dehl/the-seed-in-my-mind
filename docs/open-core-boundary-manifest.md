---
doc_id: open_core_boundary_manifest
title: Open-Core Boundary Manifest
status: derived
version: v0
last_reviewed: 2026-03-05
scope:
  - Describes the practical in-repo boundary rules enforced for the public open-core package.
authoritative_for:
  - Boundary-check implementation guidance.
not_authoritative_for:
  - Licensing legal advice
  - Protocol semantics
depends_on:
  - open-core-split-and-data-boundary-spec.md
---

# Open-Core Boundary Manifest

This manifest summarizes the concrete public boundary currently enforced by local checks, export verification, and CI.

Authority note: this public open-core repo is the source of truth for the public boundary checker, public export manifest, reference viewer boundary, and reviewer/demo verification flow. The private repo may run these checks against its own integration tree, but private copies or generated exports are not authoritative over this repo.

## 1. Public open-core surfaces

The public package intentionally includes:

- `backend/**` excluding private-only crates, local env files, runtime state, and build outputs,
- `frontend/open-core-reference/**`,
- `frontend/src/shared/types/canonical.ts`,
- `scripts/check-open-core-boundaries.mjs`,
- `scripts/grant-reviewer-quickstart.ps1`,
- `scripts/open-core-demo.ps1`,
- `scripts/verify-open-core-export.mjs`,
- `tools/open-core/**`,
- curated docs listed by the export manifest,
- deterministic public seed/demo files under `seed/**`.

## 2. Forbidden frontend imports for the public reference viewer

The open-core reference frontend must not import:

- `domains/private/**`
- `domains/private-overlay/**`
- `api/private`
- `app/workspaceshell`
- `app/tabstate`
- `components/viewers/builderview/**`

These rules are enforced by:
- `frontend/open-core-reference/scripts/check-reference-boundaries.mjs`
- `npm run verify:boundaries`

## 3. Forbidden backend dependency direction

The exported open-core backend must not depend on the private DTO crate:

- `api-types-private`
- `api_types_private`

This is enforced by:
- `scripts/check-open-core-boundaries.mjs`
- the export manifest denylist
- export-time Cargo manifest sanitization

## 4. Export hygiene rules

The public export must not contain:

- `.env` files,
- `backend/var/**`,
- `node_modules/**`,
- `dist/**`,
- `coverage/**`,
- `_target/**`,
- runtime logs,
- local database files,
- internal planning material,
- codex logs/notes,
- archived and audit-only docs not required for public review.

These rules are enforced by:
- `tools/open-core/export-manifest.json`
- `scripts/verify-open-core-export.mjs`
- `npm run extract:open-core`

## 5. Main enforcement commands

- local boundary checks: `npm run verify:boundaries`
- canonical DTO drift guard: `npm run verify:canonical-dto`
- public working-tree verification: `npm run verify`
- generated export cleanliness: `npm run verify:generated-export`
- compatibility alias for generated export cleanliness: `npm run verify:open-core-export`
- working-tree-as-export hygiene check: `npm run verify:export-working-tree`
- full export + packaged artifact generation: `npm run extract:open-core`
