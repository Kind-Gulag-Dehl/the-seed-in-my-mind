# seed data v0

This folder contains deterministic canonical seed files used for local development and public open-core verification.

- `seed-data-v0.json` is the larger development seed.
- `reviewer-demo.seed-data-v0.json` is the tiny public dataset used by the reviewer demo and export smoke flow.
- The seed importer ingests these events into Postgres deterministically.
- To force a re-import, run `cargo run -p seed-importer -- --file <path> --force`.
