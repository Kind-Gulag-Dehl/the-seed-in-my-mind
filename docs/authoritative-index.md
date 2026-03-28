<!-- file: the-seed-in-my-mind-open-core/docs/authoritative-index.md -->

# authoritative index

## 0. purpose

This document is the authority and navigation index for the public `docs/` set in this repo.

It defines:
- which documents are part of the public open-core documentation set,
- which public documents are authoritative protocol/spec sources,
- which public documents are derived maps or reviewer guides,
- which public documents are future/spec-only,
- the recommended public reading order.

This repo intentionally publishes a curated public subset of the wider source-tree documentation family. Documents not listed here should be treated as outside the current public documentation set.

This index is authoritative for:
- public document classification,
- public reading order,
- precedence rules between public docs.

It is not authoritative for:
- protocol semantics themselves,
- implementation status beyond the reviewer-facing status split in `open-core-implementation-status.md`.

---

## 1. public document classes

### 1.1 authoritative public specs

These documents define protocol or system behavior within the public repo.

- `protocol v5.md`
- `protocol v5-appendix-a.md`
- `canonical-encoding-and-hashing-spec.md`
- `deterministic-replay-and-merge-spec.md`
- `node-and-conformance-spec.md`
- `pod-consensus-and-canonical-publication-spec.md`
- `snapshot-format-v0.md`
- `canonical-preservation-and-provenance-spine-spec.md`
- `offline-and-mindseed-spec.md`
- `cycle-spec.md`
- `shared-map-and-payload-bundles-spec.md`

### 1.2 authoritative public specs published for transparency

These documents are part of the intended open-core architecture and are public in this repo for architectural transparency. They are not, by themselves, evidence that the described runtime is fully implemented.

- `tempo-spec.md`
- `challenge-engine-spec.md`
- `governance-spec.md`
- `token-spec.md`
- `verification-spec.md`
- `safety-spec.md`
- `safety-rulebook-interface-mechanics-spec.md`

Each of these documents carries an explicit status banner. `open-core-implementation-status.md` remains the authoritative current-state implementation reference.

### 1.3 derived public maps and navigation docs

These documents help readers navigate the public spec set. They are not authoritative for protocol semantics.

- `map.index.md`
- `map.protocol-v5.md`
- `map.token-governance.md`
- `map.node-and-replay.md`
- `map.encoding-snapshots-bundles.md`
- `map.offline-and-preservation.md`

### 1.4 reviewer, boundary, and runtime framing docs

These documents explain the public package, reviewer workflow, implementation status, and public boundary.

- `open-core-architecture-overview.md`
- `open-core-implementation-status.md`
- `open-core-reviewer-guide.md`
- `open-core-demo-flow.md`
- `open-core-boundary-manifest.md`
- `open-core-split-and-data-boundary-spec.md`
- `stage0-runtime-configuration.md`
- `api-contract-read-only.md`
- `cross-doc-invariants.md`

---

## 2. precedence rules

When public documents conflict, resolve them in this order.

### 2.1 mechanics and determinism

1) `canonical-encoding-and-hashing-spec.md` wins for canonical encodings, hashing rules, byte-level formats, and commitment primitives.
2) `deterministic-replay-and-merge-spec.md` wins for deterministic replay and merge semantics.
3) `pod-consensus-and-canonical-publication-spec.md` wins for canonical publication profiles, finalized-prefix rules, and derived block mapping.
4) `node-and-conformance-spec.md` wins for conformance expectations, but must not redefine canonical mechanics.

### 2.2 root protocol semantics

1) `protocol v5.md` is the public root for constitutional invariants and core semantics.
2) `protocol v5-appendix-a.md` is subordinate to `protocol v5.md`.
3) Subsystem specs are authoritative within their scope, but may not contradict `protocol v5.md`.

### 2.3 reviewer and navigation documents

1) `open-core-implementation-status.md` is authoritative for current implementation status in the public package.
2) Map files and reviewer guides are derived and must not introduce new semantics.
3) If a public future/spec-only doc appears broader than the runnable package, defer to its status banner plus `open-core-implementation-status.md`.

---

## 3. recommended public reading order

### 3.1 reviewer-first path

1) `map.index.md`
2) `open-core-architecture-overview.md`
3) `open-core-implementation-status.md`
4) `open-core-reviewer-guide.md`
5) `open-core-demo-flow.md`

### 3.2 protocol-first path

1) `protocol v5.md`
2) `canonical-encoding-and-hashing-spec.md`
3) `deterministic-replay-and-merge-spec.md`
4) `node-and-conformance-spec.md`
5) `pod-consensus-and-canonical-publication-spec.md`
6) `cycle-spec.md`
7) `tempo-spec.md`
8) `challenge-engine-spec.md`
9) `governance-spec.md`
10) `token-spec.md`
11) `safety-spec.md`
12) `safety-rulebook-interface-mechanics-spec.md`
13) `snapshot-format-v0.md`
14) `shared-map-and-payload-bundles-spec.md`
15) `offline-and-mindseed-spec.md`
16) `verification-spec.md`

---

## 4. maintenance rules

- If a public doc is added, removed, or reclassified, update this index.
- If a public map references a document that is not present in this repo, either remove the reference or mark it clearly as not included in this public repo.
- If implementation status changes, update `open-core-implementation-status.md` rather than implying implementation through map or index wording.
- Derived docs must not introduce new semantics.
