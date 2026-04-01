# Open-Core Publish Checklist

## 1) Generate and verify the public export

```powershell
npm run extract:open-core
```

Expected outputs:
- export tree: `_export/open-core/`
- export metadata: `_export/open-core/EXPORT_INFO.txt`
- release zip: `tools/open-core/dist/open-core-export.zip`

Verification note:
- `node scripts/verify-open-core-export.mjs --export-root _export/open-core` checks the generated export tree directly.
- Running `node scripts/verify-open-core-export.mjs` from the repo root may instead validate the repo working tree if a root `EXPORT_INFO.txt` is present.
- Failures about forbidden paths usually mean the checked tree contains local build/runtime artifacts that must not ship in the export.

## 2) Optional explicit rerun of the exported smoke path

```powershell
powershell -ExecutionPolicy Bypass -File tools/open-core/smoke-export.ps1
```

This verifies:
- a fresh database reset and migration path,
- import of the tiny public reviewer demo seed,
- snapshot build and snapshot verification,
- exported `api-server` build and read-only health/data responses,
- exported reference frontend build,
- cleanup of transient build artifacts from the exported tree.

## 3) What the export should contain

The public export is intended to contain:
- curated docs,
- canonical backend/runtime crates and scripts,
- reference frontend,
- seed/demo data,
- export and boundary tooling.

The public export must not contain:
- `.env` files,
- runtime logs,
- build outputs,
- `node_modules`,
- local DB/runtime state,
- internal planning or codex logs,
- private DTO crates or product-only frontend modules.
