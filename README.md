# the-seed-in-my-mind

This repo is the public infrastructure layer for The Seed in My Mind. It contains the backend, reference frontend, docs, and export tooling needed to inspect and verify a shared idea graph without the private product layer. The package contains the code and guarded tooling for deterministic Seed import, replay, snapshot construction and verification, read-only serving, and export-boundary enforcement. The database-backed reviewer proof requires an explicitly disposable PostgreSQL administrator URL; ordinary runtime databases are rejected. It is a real runnable open-core package, not the full end-user product.

The Seed in My Mind open-core repo is the public infrastructure package for a deterministic canonical-history substrate and its reference reviewer surface.

This public open-core package contains the implemented and reviewer-runnable infrastructure layer:
- canonical event ingestion and validation,
- deterministic replay and snapshot generation,
- a read-only reference node/API,
- an open-core reference viewer,
- curated protocol and conformance docs,
- export and boundary verification tooling.

This repo also publishes some broader protocol/spec documents for architectural transparency. Those documents describe intended open-core architecture, but they do not imply that the described runtime is already implemented. Use [open-core-implementation-status.md](docs/open-core-implementation-status.md) as the authoritative current-state reference.

The private/product layer is the richer game/product experience built on top of the same canonical substrate. That code is intentionally outside this public repo.

## Repository authority model

This repository is the source of truth for open-core code, open-core docs/specs, public reference tooling, public reviewer/demo materials, and the open-core export manifest. Private companion repositories may consume this repo, mirror selected open-core materials, or package generated exports for integration checks, but they must not automatically overwrite newer public/open-core work.

Private/product/game work remains authoritative in private companion repositories. Changes should flow across the repo boundary deliberately: public/open-core changes originate here; proprietary integration changes originate outside this repo.

## What reviewers can verify today

Today's open core is not a finished decentralized network. It is a working Stage 0 substrate with a reviewer-ready evaluation path.

Implemented now:
- Rust backend workspace for canonical ingest, replay, snapshots, and read-only API serving.
- Snapshot builder and snapshot verifier binaries.
- Seed importer for deterministic canonical bootstrap/demo ingestion.
- Open-core boundary enforcement and canonical DTO drift checks.
- Public open-core export generation with cleanliness verification and zip packaging.
- Minimal reference frontend for inspecting verified snapshot state.

Public spec-only or not yet operational in the public runtime:
- multi-node consensus and committee-based canonical publication,
- broader public canonical write flows beyond the narrow signed idea/connection ingress,
- governance activation using the live system,
- token/economic runtime enforcement beyond specs and partial code paths,
- the full game layer and broader product UX.

## Fast reviewer paths

15-minute path:
```powershell
npm run review:quickstart
```

30-minute path:
```powershell
powershell -ExecutionPolicy Bypass -File scripts/grant-reviewer-quickstart.ps1 -Profile 30
```

If you do not already have a local Postgres database configured, see the minimal setup section below.

Both paths are described in [open-core-reviewer-guide.md](docs/open-core-reviewer-guide.md). For the implemented-versus-spec-only split, see [open-core-implementation-status.md](docs/open-core-implementation-status.md).

## What the demo proves

The deterministic demo path:
1. resets a Postgres database,
2. applies migrations,
3. ingests a tiny public canonical seed,
4. replays canonical state and builds a snapshot,
5. verifies the snapshot commitment,
6. serves that state through the read-only open-core API,
7. optionally builds the reference frontend against the same surface.

Run it directly:
```powershell
npm run review:demo
```

See [open-core-demo-flow.md](docs/open-core-demo-flow.md). The demo proves the implemented runtime path only; it is not evidence that the full public spec set is already operational.

## Architecture at a glance

```text
public seed/demo data
        |
        v
  seed-importer ----> canonical events table ----> deterministic replay ----> snapshot-builder
                                                                  |                 |
                                                                  |                 v
                                                                  |          snapshot-verify
                                                                  v
                                                        read-only api-server
                                                                  |
                                                                  v
                                                     open-core reference viewer
```

Boundary model:
- Implemented open core runtime: protocol-facing backend crates, replay/snapshot tooling, read-only API surface, reference viewer, export/boundary tooling.
- Public spec architecture: curated protocol and conformance docs, including some future/spec-only open-core documents published for transparency.
- Private/product layer: game logic, richer UX flows, private overlays, builder/product shells, and other downstream product-specific modules.

See [open-core-architecture-overview.md](docs/open-core-architecture-overview.md) and [open-core-boundary-manifest.md](docs/open-core-boundary-manifest.md).

## Repo structure

```text
backend/                      Rust workspace for canonical ingest, replay, snapshots, and API
frontend/open-core-reference/ Minimal public viewer for verified canonical state
docs/                         Curated specs and reviewer-facing documentation
scripts/                      Boundary, demo, and reviewer orchestration scripts
tools/open-core/              Export, packaging, DTO verification, and smoke tooling
seed/                         Deterministic public seed/demo data
```

## Prerequisites

- Windows PowerShell
- Rust stable
- Node.js 20+
- Postgres with `psql` on PATH
- SEED_TEST_DATABASE_ADMIN_URL set process-locally to the postgres maintenance database on a disposable local PostgreSQL server with create/drop privileges

Runtime details: [stage0-runtime-configuration.md](docs/stage0-runtime-configuration.md)

## Minimal local setup (quickstart)

Reviewer scripts never reset an ordinary DATABASE_URL. Set SEED_TEST_DATABASE_ADMIN_URL process-locally to the postgres maintenance database on a disposable PostgreSQL server. The scripts create only seed_opencore_m1_reviewer_repair_001_* databases, verify exact-name cleanup, and preserve existing databases.

Do not put the administrator URL in this repository or paste it into review evidence.
## Export the public open-core package

From the repo root:

```powershell
npm run extract:open-core:dir -- C:\Temp\seed-open-core-export
```

That command:
- creates the explicitly supplied isolated export directory,
- verifies the export is clean and publishable,
- runs build/test/smoke checks against the exported tree,
- writes `EXPORT_INFO.txt`,
- does not create a zip unless the separate zip path is explicitly requested.

## Verification commands

Use the root commands for developer checks:

```powershell
npm run lint
npm run test
npm run build
npm run conformance
npm run verify
```

`npm run verify` is the public open-core working-tree check. It runs boundary checks, canonical DTO drift checks, reference frontend checks, and backend verification.

`npm run conformance` runs the currently available docs/spec conformance harnesses without depending on the reference frontend test install. Today it runs:

```powershell
npm run conformance:tempo-cycle
```

The Tempo/Cycle harness validates `docs/conformance/tempo-cycle-fixtures.v1.json` against its schema and replays the fixture events. It is fixture-scoped conformance scaffolding, not the production Tempo runtime.

`npm run test` runs the reference frontend Vitest suite and requires `frontend/open-core-reference` dependencies to be installed.

Export-package checks are separate:

```powershell
npm run verify:generated-export
npm run verify:open-core-export
npm run verify:export-working-tree
```

`npm run verify:generated-export` is a read-only compatibility check for an already-present legacy `_export/open-core` tree. New exports must use an explicit external root, for example `npm run extract:open-core:dir -- C:\Temp\seed-open-core-export`, and the generator verifies that root directly. `npm run verify:export-working-tree` intentionally treats the current repo tree as if it were shippable and should fail when local build/runtime artifacts are present.

## Current status

Use [open-core-implementation-status.md](docs/open-core-implementation-status.md) for the honest implemented-vs-planned status matrix.

## Funding framing

This package is intended to support infrastructure funding for:
- hardening the public reference node/export,
- improving evaluator and contributor onboarding,
- extending the canonical publication path beyond the current bootstrap/runtime stage,
- preparing the system for independent operators and downstream applications.

The correct claim today is: there is a real deterministic substrate, a real public export path, and a credible staged path forward. The correct claim is not that the full long-term system is already complete.
