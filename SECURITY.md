# Security policy

If you discover a security issue, do not publish exploit details in a public issue.

## What to report privately

Please include:
- affected component or path,
- impact summary,
- reproduction steps,
- any logs or traces needed to reproduce,
- proposed mitigation if you have one.

If there is no dedicated private reporting address published with the current distribution, contact the repository owner through a non-public channel first and keep the initial message minimal.

## Scope notes for this open-core package

The public export is intended to exclude:
- `.env` files,
- local database/runtime artifacts,
- logs and caches,
- internal planning material,
- private/product-only code and DTO crates.

If you find any of those in a generated export, treat it as a release-hygiene/security issue and report it.

## Local secret hygiene

For local development checks:

```powershell
powershell -ExecutionPolicy Bypass -File backend/scripts/scan-secrets.ps1
```

This file is process guidance only and does not create a legal support obligation or SLA.
