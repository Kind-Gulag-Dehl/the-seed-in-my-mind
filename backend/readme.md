# Backend

Stage 0 Rust backend workspace scaffold.

## Windows dev run

1) Copy `backend/.env.example` to `backend/.env` and set `DATABASE_URL` (optional: `PGPASSFILE`).
2) Run the bootstrap script:

```
powershell -ExecutionPolicy Bypass -File backend/scripts/dev-bootstrap.ps1
```

3) Run the binaries:

```
cargo run -p snapshot-builder
cargo run -p api-server
```

Optional helpers (loads env + runs):

```
powershell -ExecutionPolicy Bypass -File backend/scripts/run-snapshot-builder.ps1
powershell -ExecutionPolicy Bypass -File backend/scripts/run-api-server.ps1
```

Note: if `cargo` fails to remove `api-server.exe` on Windows, stop the running process and rerun.
