# Seed V4 Pilot Profile-v0 Validate-only Alignment

- Task: `OPENCORE-SEED-V4-PROFILE0-VALIDATION-001`
- Shared task: `INTEGRATION-SEED-V4-PILOT-001`
- Open Core baseline: `6068072160fc032eb1ec3b7641cb917c38f08776`
- Private baseline declared by the package: `ce8b0c6ac4dc97f155dda83fb9ed19bd735819fd`
- Status: focused Open Core alignment passing; shared task remains active

## Authority boundary

This change adds one separate package-validation path for the exact unsigned,
noncanonical, isolated V4 pilot. It does not admit events, connect to storage,
sign material, create genesis state, or alter the ordinary signed Profile-v0
candidate boundary.

The importer accepts `seed-v4-pilot-manifest-v1` only with `--validate-only`.
The flag is rejected with `--force`. An attempt to use the same package without
`--validate-only` fails before `DATABASE_URL` is read, before a pool or
transaction is created, and before canonical mutation is enabled. The existing
`seed-data-v0` path is otherwise unchanged.

No normative authority change was required. The repaired identity-admission
specification and its source-integrity artifacts belong to the coordinated
parallel sub-run and were not modified here.

## Exact validated contract

The validator independently checks:

- manifest payload SHA-256, all 15 non-manifest component lengths and SHA-256
  values, the sorted non-manifest component digest, component membership, safe
  relative paths, and the exact baseline/package-domain binding;
- explicit unfinished, unsigned, noncanonical, import-ineligible, unaccepted,
  0/50 owner-review status;
- the package-declared UUIDv7 algorithm against independently reproduced
  UUIDv7 values for ideas, representations, connections, Orderings, relative
  lenses, and Profile-v0 event templates;
- 50 unique ideas, the fixed DEC-044 speaker identity, actual provenance and
  review boundaries, and no human-authorship or owner-acceptance claim;
- 600 representations forming all 12 complexity/length cells for each idea,
  runtime text limits, text SHA-256, and explicit unresolved canonical BLAKE3
  payload hashes;
- connection endpoints/types; three native `Ordering` records using
  `ordering_create`/`ordering_fork`, `ordering_id`, `ordering_profile: vine`,
  Vine types, ordered idea membership, and valid fork lineage;
- 50 universal profiles with 49 complete 20-slot provisional rank profiles and
  the DEC-044 identity's one explicit wholly unresolved profile;
- 120 relative-importance contexts, plus DEC-037 aggregate scores, horizon
  subtotals, deep-time-first tie order, selection-order pilot surrogate, and
  cumulative ranks for the 49 comparable profiles;
- 50 unsigned `ed25519_v0` local-draft event templates with null author key,
  payload hash, signature, and observation fields, and blocked import
  eligibility;
- the import projection's validate-only, zero-canonical-write, isolated,
  unsigned, noncanonical, non-genesis state.

Ecosystem profile values such as `evidence_rail` remain valid terminology, but
recursive live `rail_*` substrate keys are rejected.

## Verification evidence

Exact manifest:

`A:\the-seed-in-my-mind\docs\planning\generated\seed-v4-pilot\seed-v4-pilot-manifest.v1.json`

- Independent private verification: 21/21 checks passed, 0 failed, 0 structural
  errors; identity-source corruption check false.
- Manifest file SHA-256:
  `a132bb341207b636700312253aa3b42a1ce4bdbede461cc3e7007a859c4afc8f`.
- Manifest payload SHA-256:
  `2050abff0306a2cfc4b5554967ccb169171c4c95d5c3818a60fde95ec5aefaba`.
- Non-manifest component digest:
  `8cca9190326dd6afe4519b344c15b9274c146bee74398efa212a16f981862880`.
- Open Core validate-only result:
  `pass canonical_writes=0`; ideas 50, representations 600, connections 84,
  Orderings 3, universal profiles 50, relative contexts 120.
- The same manifest without `--validate-only`, with `DATABASE_URL` absent,
  rejected as ineligible for canonical import, signing, or genesis.
- `cargo check -p seed-importer`: passed.
- `cargo test -p seed-importer`: 6 passed, 0 failed.

The only command warnings were Windows incremental-cache hard-link fallbacks;
they did not affect validation.

## Protected boundaries

No private-repository file was modified. No database was contacted. No
canonical import, signing, genesis, full-Seed generation, staging, commit, push,
or deployment occurred.

The retained Tempo files remained byte-identical:

- `docs/tempo-spec.md`:
  `3407193F0E5547202F0424E6DA32E8283FD654F9D9AA2902B61BC01BB110829D`
- `docs/cycle-spec.md`:
  `905EC16DC60F0C23EC8E28A578AAB67B67DA9CA67E8E7E089C557A82626FBA79`
- `docs/planning/tempo-cycle-canonical-schema-and-replay-resolution.v1.md`:
  `7CC60A89C3A1ADCC3F17551F2F3A6999818A5C172047C86AB5C20FA52ACCA7AB`

## Follow-up

The shared task remains active. The private materializer/verifier may now rerun
against this Open Core importer state to replace its importer-dependency
projection and recapture package commitments. That downstream regeneration does
not authorize owner review, canonical import, signing, genesis, or final Seed
generation.
