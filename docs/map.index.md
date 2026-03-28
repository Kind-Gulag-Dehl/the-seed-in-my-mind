---
doc_id: map_index
title: Map: Public Docs Index
status: derived
version: v0
last_reviewed: 2026-03-26

scope:
  - Navigation entry point for the curated public open-core documentation set.

authoritative_for:
  - Navigation only.

not_authoritative_for:
  - Protocol semantics
  - Implementation status

depends_on:
  - authoritative-index.md
  - open-core-implementation-status.md
  - open-core-reviewer-guide.md

conflicts:
  - none known

change_rules:
  - Keep this file aligned with the docs that actually ship in the public repo.

reader_path:
  - prereq: authoritative-index.md
  - next: none

keywords:
  - map
  - index
  - navigation
  - open-core
---

Purpose: quick navigation for the public open-core docs set.

Public note: this repo is a curated documentation subset. Some broader specs are public for architectural transparency, but `open-core-implementation-status.md` remains the source of truth for what is implemented now.

## start here

- `open-core-architecture-overview.md` - what the public package is.
- `open-core-implementation-status.md` - what is implemented now versus planned/spec-only.
- `open-core-reviewer-guide.md` - how to review the public package.
- `open-core-demo-flow.md` - what the demo proves.
- `open-core-boundary-manifest.md` - what belongs in the public package.

## public maps included in this repo

- `map.protocol-v5.md` - protocol root structure and delegated surfaces.
- `map.token-governance.md` - governance and token docs now included publicly.
- `map.node-and-replay.md` - node behavior, replay, and conformance.
- `map.encoding-snapshots-bundles.md` - encoding, snapshots, and bundle/distribution surfaces.
- `map.offline-and-preservation.md` - offline semantics plus preservation/provenance docs included in this public repo.

## authoritative public specs

### core substrate

- `protocol v5.md`
- `protocol v5-appendix-a.md`
- `canonical-encoding-and-hashing-spec.md`
- `deterministic-replay-and-merge-spec.md`
- `node-and-conformance-spec.md`
- `pod-consensus-and-canonical-publication-spec.md`

### snapshots, bundles, and preservation

- `snapshot-format-v0.md`
- `shared-map-and-payload-bundles-spec.md`
- `canonical-preservation-and-provenance-spine-spec.md`
- `offline-and-mindseed-spec.md`

### broader public architecture docs

- `cycle-spec.md`
- `tempo-spec.md`
- `challenge-engine-spec.md`
- `governance-spec.md`
- `token-spec.md`
- `verification-spec.md`
- `safety-spec.md`
- `safety-rulebook-interface-mechanics-spec.md`

## how to interpret the broader public specs

- `tempo-spec.md`, `challenge-engine-spec.md`, `governance-spec.md`, `token-spec.md`, `verification-spec.md`, `safety-spec.md`, and `safety-rulebook-interface-mechanics-spec.md` are public for architectural transparency.
- Those docs carry explicit status banners at the top.
- Their publication does not imply full runtime implementation.
- Use `open-core-implementation-status.md` for the current implemented surface.

## intentionally not included in this public repo

- planning, audit, archive, and internal working docs
- product/frontend private docs
- excluded subordinate docs such as `canonical-offline-mode.md`, `full-offline-canonical-mode.md`, and `1v1-debate-spec.md`
