---
doc_id: map_offline_and_preservation
title: Map: Offline and Preservation
status: derived
version: v0
last_reviewed: 2026-06-22

scope:
  - Navigation map for the offline and preservation documents included in this public repo.

authoritative_for:
  - Navigation only.

not_authoritative_for:
  - Protocol semantics
  - Implementation status

depends_on:
  - offline-and-mindseed-spec.md
  - canonical-preservation-and-provenance-spine-spec.md
  - shared-map-and-payload-bundles-spec.md
  - snapshot-format-v0.md

conflicts:
  - none known

change_rules:
  - Keep this map limited to the offline and preservation docs that are actually present in the public repo.

reader_path:
  - prereq: authoritative-index.md
  - next: none

keywords:
  - map
  - offline
  - preservation
  - provenance
  - navigation
---

Purpose: quick navigation of the offline, preservation, and distribution docs that are part of the public open-core package.

Public note: this repo intentionally does not include `canonical-offline-mode.md` or `full-offline-canonical-mode.md`. This map covers only the offline and preservation docs that actually ship in the public repo.

## source documents

- `offline-and-mindseed-spec.md`
- `canonical-preservation-and-provenance-spine-spec.md`
- `shared-map-and-payload-bundles-spec.md`
- `snapshot-format-v0.md`

## suggested reading order

1. `offline-and-mindseed-spec.md`
   - offline execution model
   - reintegration and publication
   - authoritarian-resilience guarantees
2. `canonical-preservation-and-provenance-spine-spec.md`
   - block packaging and provenance spine
   - payload integrity and availability
   - conformance and threat model
3. `snapshot-format-v0.md`
   - snapshot structure and commitment rules
4. `shared-map-and-payload-bundles-spec.md`
   - public distribution artifacts
   - bundle tiers and shared-map commitments

## interpretation note

- These docs explain the open-core preservation and offline architecture.
- They should be read together with `open-core-implementation-status.md` for what is implemented in the public runtime today.

## related links

- `map.index.md`
- `open-core-implementation-status.md`
- `open-core-split-and-data-boundary-spec.md`
