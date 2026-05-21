---
doc_id: stage0_runtime_configuration
title: Stage 0 Runtime Configuration
status: derived
version: v0
last_reviewed: 2026-04-25
scope:
  - Defines the practical runtime prerequisites for the current public read-only/open-core evaluation path.
authoritative_for:
  - Current runtime configuration guidance only.
not_authoritative_for:
  - Protocol semantics
depends_on:
  - api-contract-read-only.md
---

# Stage 0 Runtime Configuration

## 1. Purpose

This document defines the current runtime prerequisites for the public open-core evaluation path.

## 2. Required environment

- Postgres is required.
- `DATABASE_URL` must be set in the environment or provided through `backend/.env`.
- `psql` must be available on PATH for the scripted reset/demo flows.

Optional:
- `PGPASSFILE`

## 3. Minimal local setup (quickstart)

Local/dev example only:

- example database name: `seed_open_core`
- example user: `seed_local`
- example password: `seed_local_pw`
- exact `DATABASE_URL`:

```text
postgres://seed_local:seed_local_pw@127.0.0.1:5432/seed_open_core
```

PowerShell example:

```powershell
$env:DATABASE_URL="postgres://seed_local:seed_local_pw@127.0.0.1:5432/seed_open_core"
```

This example is for local reviewer setup only. It is not production guidance.

## 4. Backend environment file

Create `backend/.env` from `backend/.env.example` and set at minimum:

```powershell
DATABASE_URL=postgres://USER:PASSWORD@HOST:PORT/DBNAME
```

`backend/.env` example:

```text
DATABASE_URL=postgres://seed_local:seed_local_pw@127.0.0.1:5432/seed_open_core
```

The public export intentionally excludes a real `.env` file. It includes `backend/.env.example` only.

## 5. Developer verification commands

From the repo root:

```powershell
npm run verify
```

`npm run verify` is the public open-core working-tree check. It runs:

- open-core boundary checks,
- canonical DTO drift checks,
- reference frontend boundary/test/build,
- backend build/test through `backend/scripts/verify-backend.ps1`.

The root `lint`, `test`, and `build` scripts are real commands and should not be called with `--if-present` as a substitute for verification:

```powershell
npm run lint
npm run test
npm run build
```

Use these narrower commands when isolating failures:

```powershell
npm run verify:boundaries
npm run verify:canonical-dto
npm run verify:frontend
npm run verify:backend
npm run verify:generated-export
npm run verify:open-core-export
npm run verify:export-working-tree
npm run verify:all
```

`npm run verify:public` and `npm run verify` check the local public working tree as a developer checkout. `npm run verify:generated-export` checks `_export/open-core` after `npm run extract:open-core`; `npm run verify:open-core-export` is a compatibility alias for that generated-export check. `npm run verify:export-working-tree` intentionally treats the current repo tree as if it were a shippable export package; it is expected to fail if local-only generated artifacts such as `node_modules`, `dist`, `backend/var`, or `tsconfig.tsbuildinfo` are present.

Backend verification uses a temporary Cargo target directory when `CARGO_TARGET_DIR` is not already set:

```powershell
$env:TEMP\the-seed-in-my-mind-open-core-backend-verify-target
```

This keeps large Rust build output off the repo drive during scripted verification. Direct `cargo` commands run from `backend/` still create `backend/target/` unless you set `CARGO_TARGET_DIR` yourself.

## 6. Safe cleanup of generated artifacts

Generated artifacts are safe to remove when no dev server, test run, or build is using them. Do not remove source, docs, seed data, migrations, or package manifests.

Common cleanup targets:

```powershell
Remove-Item -LiteralPath .\backend\target -Recurse -Force
Remove-Item -LiteralPath (Join-Path $env:TEMP "the-seed-in-my-mind-open-core-backend-verify-target") -Recurse -Force
Remove-Item -LiteralPath .\backend\var -Recurse -Force
Remove-Item -LiteralPath .\frontend\open-core-reference\dist -Recurse -Force
Remove-Item -LiteralPath .\frontend\open-core-reference\node_modules -Recurse -Force
Remove-Item -LiteralPath .\frontend\open-core-reference\package-lock.json -Force
Remove-Item -LiteralPath .\frontend\open-core-reference\tsconfig.tsbuildinfo -Force
Remove-Item -LiteralPath .\_export\open-core -Recurse -Force
Remove-Item -LiteralPath .\tools\open-core\dist -Recurse -Force
```

`backend/var` may contain local runtime state and logs. Remove it only for local cleanup, not during an investigation where those artifacts are needed.

## 7. Reference frontend alignment

The current reference frontend expects the API server at:

```text
http://127.0.0.1:3000
```

The Vite development server proxies `/api/*` to that backend target by default.

## 8. Public export path

To generate and verify the public open-core export:

```powershell
npm run extract:open-core
```

That flow builds a clean export, writes `EXPORT_INFO.txt`, runs the exported smoke flow, and produces the zip artifact under `tools/open-core/dist/`.
