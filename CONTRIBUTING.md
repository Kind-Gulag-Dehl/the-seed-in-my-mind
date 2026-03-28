# Contributing

This repo is split between a public open-core substrate and a downstream proprietary product layer. Contributions should preserve that boundary and keep the public package reviewer-friendly.

## Before you open a PR

- Read [README.md](README.md).
- Read [docs/open-core-boundary-manifest.md](docs/open-core-boundary-manifest.md).
- Read [docs/open-core-implementation-status.md](docs/open-core-implementation-status.md) so implementation claims stay honest.

## Prerequisites

- Rust stable
- Node.js 20+
- Postgres with `psql` available on PATH
- Windows PowerShell for the repo’s current scripted workflows

If you need local database configuration, copy `backend/.env.example` to `backend/.env` and set `DATABASE_URL`.

## Baseline checks

From the repo root:

```powershell
npm run verify:boundaries
npm run verify:canonical-dto
npm run verify:backend
```

If you changed export tooling or public packaging:

```powershell
npm run extract:open-core
powershell -ExecutionPolicy Bypass -File tools/open-core/smoke-export.ps1
```

## Boundary rules

Do not move proprietary code into the open-core surface.

In practice this means:
- the open-core reference frontend must not import private/product modules,
- exported backend crates must not depend on `api-types-private`,
- the public export must not contain secrets, local runtime artifacts, caches, or internal planning material.

The automated checks that enforce this are:
- `npm run verify:boundaries`
- `npm run verify:canonical-dto`
- `npm run verify:open-core-export`

## Scope discipline

- Keep diffs narrowly scoped.
- Avoid unrelated refactors and formatting churn.
- Preserve deterministic replay, canonical encoding, snapshot, and boundary invariants.
- Do not oversell unimplemented subsystems in public docs.

## Public docs

If your change affects the public open-core package, update the relevant reviewer-facing docs:
- [docs/open-core-reviewer-guide.md](docs/open-core-reviewer-guide.md)
- [docs/open-core-architecture-overview.md](docs/open-core-architecture-overview.md)
- [docs/open-core-implementation-status.md](docs/open-core-implementation-status.md)
- [docs/open-core-demo-flow.md](docs/open-core-demo-flow.md)
