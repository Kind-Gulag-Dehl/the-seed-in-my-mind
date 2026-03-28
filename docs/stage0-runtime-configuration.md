---
doc_id: stage0_runtime_configuration
title: Stage 0 Runtime Configuration
status: derived
version: v0
last_reviewed: 2026-03-05
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

## 5. Basic local verification path

From the repo root:

```powershell
npm run verify:backend
npm run review:demo
```

## 6. Reference frontend alignment

The current reference frontend expects the API server at:

```text
http://127.0.0.1:3000
```

The Vite development server proxies `/api/*` to that backend target by default.

## 7. Public export path

To generate and verify the public open-core export:

```powershell
npm run extract:open-core
```

That flow builds a clean export, writes `EXPORT_INFO.txt`, runs the exported smoke flow, and produces the zip artifact under `tools/open-core/dist/`.
