---
doc_id: open_core_demo_flow
title: Open-Core Demo Flow
status: derived
version: v0
last_reviewed: 2026-03-05
scope:
  - Documents the deterministic reviewer demo path and what each step proves.
authoritative_for:
  - Demo instructions only.
not_authoritative_for:
  - Canonical semantics
depends_on:
  - stage0-runtime-configuration.md
  - api-contract-read-only.md
---

# Open-Core Demo Flow

## 1. Purpose

The demo flow is designed to prove a real end-to-end path, not just static documentation.

It shows:
- canonical event ingestion,
- deterministic replay,
- snapshot generation,
- snapshot verification,
- read-only API serving of verified state,
- optional reference-frontend build against that API surface.

It does not prove the full public spec set. It proves the currently implemented open-core runtime path described in `open-core-implementation-status.md`.

## 2. Command

From the repo root:

```powershell
npm run review:demo
```

Optional:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/open-core-demo.ps1 -BuildReferenceFrontend
```

## 3. What the script does

The script:

1. resets the local Postgres schema,
2. applies the backend migrations,
3. imports `seed/reviewer-demo.seed-data-v0.json`,
4. runs `snapshot-builder`,
5. runs `snapshot-verify --latest --profile stage0`,
6. starts `api-server` in `open_core` mode,
7. queries `/api/v0/snapshot/latest`, `/api/v0/ideas/top`, and a specific `/api/v0/idea/{id}` route,
8. prints a short verification report with snapshot commitments and the imported demo idea.

If `-BuildReferenceFrontend` is used, it also:
- installs the reference frontend dependencies,
- runs its tests,
- builds the frontend bundle.

## 4. Demo dataset

The demo uses a tiny public seed file:

- [reviewer-demo.seed-data-v0.json](../seed/reviewer-demo.seed-data-v0.json)

That file is intentionally small so reviewers can inspect it quickly and understand what was imported.

## 5. What success looks like

Success means the script prints:
- snapshot height,
- snapshot hash,
- shared map commitment,
- event count,
- the imported demo idea title and sentence,
- a short statement of what the run proved.

This is the minimal credible proof that the public open core is real and executable.

It is not proof that multi-node canonical publication, full governance/token runtime, or the private/product layer are already operational in this repo.
