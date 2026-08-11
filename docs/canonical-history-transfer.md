---
doc_id: canonical_history_transfer
title: Canonical History Transfer
status: derived
version: v1
last_reviewed: 2026-08-10
scope:
  - Documents the Open Core-owned canonical-history package and guarded transfer CLI.
authoritative_for:
  - Tool operation and package layout only.
not_authoritative_for:
  - Protocol semantics
  - Canonical event acceptance
depends_on:
  - canonical-encoding-and-hashing-spec.md
  - canonical-event-authorship-and-signature-profile-v0.md
  - deterministic-replay-and-merge-spec.md
  - node-and-conformance-spec.md
  - collective-seedpackage-and-recovery-profile-v0.md
  - snapshot-format-v0.md
---

# Canonical History Transfer

<a id="scope_and_authority"></a>
## 1. Scope and authority

This tool moves the canonical event history into a fresh Open Core database without treating projections or private product data as authority. It implements existing canonical-byte, authored-candidate, ordering, replay, and snapshot rules; it introduces no new event family or protocol meaning.

The governing requirements are canonical byte preservation and payload hashing in canonical-encoding-and-hashing-spec.md sections 1 (`canonical_byte_model`), 4 (`payload_hashing`), and 5 (`canonical_authored_event_payload_binding`); canonical order in deterministic-replay-and-merge-spec.md section 2 (`canonical_event_ordering`); authored-candidate byte preservation in canonical-event-authorship-and-signature-profile-v0.md sections 4.2 and 4.4; and snapshot verification in snapshot-format-v0.md.

`canonical-history-package-v1` is a narrow Open Core tooling format for the `canonical_history` artifact role. It is not a `.seedpkg` container, one of the closed SeedPackage profile identifiers, or an implementation of `full_recovery_v0` from collective-seedpackage-and-recovery-profile-v0.md. A future conformant SeedPackage may list this output as a history artifact, but this tool does not implement SeedPackage manifests, certificates, custody, Archive Shards, catastrophe-successor labeling, or recovery completeness.

<a id="package_layout"></a>
## 2. Package v1

A package directory contains exactly:

- `manifest.json`
- `blocks.ndjson`
- `events.ndjson`

The machine schema is `tools/open-core/canonical-history-package.v1.schema.json`. Each NDJSON component is LF-terminated and contains one compact JSON object per line.

The manifest records:

- package and event-record versions,
- source height, event count, last event ID, and optional latest source snapshot,
- the exact Open Core migration ledger count, head, and fingerprint,
- component byte lengths and domain-separated BLAKE3-256 hashes,
- a domain-separated whole-package BLAKE3-256 hash over the manifest core.

Each event record carries its exact canonical position, UUIDv7 event ID, event type, speaker, canonical payload bytes, signature and authored-candidate audit fields, publication audit fields, and a domain-separated record hash. Hash domains are fixed by package version and are not protocol hash domains.

<a id="guarded_commands"></a>
## 3. Guarded commands

From `backend`:

    cargo run -p canonical-history-transfer -- export --database-url <source-url> --output <empty-directory>
    cargo run -p canonical-history-transfer -- validate --package <package-directory>
    cargo run -p canonical-history-transfer -- import --package <package-directory> --validate-only
    cargo run -p canonical-history-transfer -- import --package <package-directory> --database-url <fresh-target-url> --confirm-fresh-target <exact-database-name>

`validate` and import `--validate-only` open no database connection and perform no database writes. Import rejects protected database names, migration drift, non-fresh targets, resource-limit violations, unsupported versions or event families, incomplete authored-candidate audit data, and any package-byte, order, ID, candidate, or hash mismatch. An exact repeat import reports `already_present`; a different non-empty history is rejected.

<a id="replay_and_comparison"></a>
## 4. Replay and comparison

Import inserts only canonical blocks and events, then rebuilds the supported Open Core projections through event replay logic. It never imports projection rows. Post-import verification rebuilds replay and the snapshot checkpoint and compares:

- event count and last event ID,
- state root,
- title/sentence payload root,
- shared-map commitment,
- active rulebook-set hash,
- snapshot hash,
- idea, connection, representation, and Ordering counts.

The guarded verifier additionally performs byte-identical repeat exports, a source-to-target package comparison, persisted snapshot comparison, and fixed-snapshot API response comparison.

<a id="exclusions"></a>
## 5. Explicit exclusions

The package must not contain or authorize derived ideas, connections, Orderings, representations, ranks, snapshots as database rows, projections, accounts, authentication sessions, documents, private evidence, private storage identifiers, AI data, prompts, outputs, messages, or secrets. Snapshot commitments may appear only as verification evidence in the manifest or canonical `snapshot_commit` event payload.

Unsupported canonical event families fail closed until their existing Open Core replay authority is portable and tested.

<a id="downstream_extractor"></a>
## 6. Downstream private extractor requirements

A downstream extractor for the current combined private database must:

1. Read the canonical `blocks` and `events` surfaces only, in exact `(block_height, event_index)` order.
2. Preserve canonical payload bytes and every populated authorship, candidate, signature, publication, and audit field exactly.
3. Use this Open Core producer and schema; it must not fork hashing, validation, ordering, or replay behavior.
4. Refuse unknown event families, migration mismatch, missing audit bytes, and any private or derived table in its authority path.
5. Write only a new package directory and never mutate the source database.
6. Run validate-only and the Open Core import/round-trip verifier before any migration cutover decision.

Seed fixture content is not part of this package format. Tests may consume the existing public fixture without modifying or accepting Seed content.
