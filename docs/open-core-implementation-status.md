---
doc_id: open_core_implementation_status
title: Open-Core Implementation Status
status: derived
version: v0
last_reviewed: 2026-03-05
scope:
  - Provides an honest implemented-versus-planned status matrix for the public open-core package.
authoritative_for:
  - Reviewer-facing implementation status only.
not_authoritative_for:
  - Protocol commitments
  - Product roadmap commitments
depends_on:
  - open-core-architecture-overview.md
  - open-core-split-and-data-boundary-spec.md
  - pod-consensus-and-canonical-publication-spec.md
---

# Open-Core Implementation Status

## 1. Scope statement

This status page is intentionally conservative.

It distinguishes:
- implemented now,
- partially implemented or internal-only,
- planned/spec-only.

Some authoritative protocol/spec documents are published in this public repo for architectural transparency even where the runtime remains limited, partial, or not yet implemented. Their presence in `docs/` does not change the status matrix below.

## 2. Status matrix

| Area | Status | Notes |
| --- | --- | --- |
| Canonical event ingestion from deterministic seed files | Implemented | Public `seed-importer` binary exists and is used in the reviewer demo flow. |
| Deterministic replay over canonical events | Implemented | Public backend crates and verification scripts are included in the open-core export. |
| Snapshot generation | Implemented | Public `snapshot-builder` binary exists. |
| Snapshot verification | Implemented | Public `snapshot-verify` binary exists and is used in smoke/demo checks. |
| Read-only canonical HTTP API | Implemented | Public read-only surface is available through the open-core `api-server` build. |
| Open-core reference viewer | Implemented | Minimal public frontend for browsing snapshot-backed ideas. |
| Open-core export generation | Implemented | Deterministic export, cleanliness verification, zip packaging, and smoke flow are scripted. |
| Boundary enforcement between public and private code | Implemented | Automated boundary and DTO drift checks are included in CI and local verification. |
| Public canonical write surface in the exported open-core package | Not included in the public runtime | The current public package is intentionally centered on ingestion/replay/snapshot/read verification. |
| Multi-node canonical publication / consensus runtime | Planned/spec-only | Spec work exists; operational multi-node runtime is not yet shipped. |
| Governance self-activation using the live system | Planned/spec-only | Not yet operational in the public package. |
| Token/economic incentive runtime | Partial/spec-heavy | Important design work exists, but the public runtime does not yet present this as finished. |
| Full game/product experience | Proprietary/downstream | Intentionally outside the public open-core export. |
| Collapse/restart/disaster-recovery tooling beyond current carried-artifact concepts | Partial | Relevant specs exist; public runtime/demo focuses on deterministic export and replay. |

## 3. What a reviewer should infer

A reviewer should infer:
- there is real infrastructure here,
- the infrastructure is inspectable and testable,
- the public export is not a placeholder,
- the long-term system is larger than the currently shipped runtime.

A reviewer should not infer:
- that the full long-term protocol vision is already operational,
- that the project is already decentralized in production,
- that the public package already exposes every future canonical write/governance/economic feature.

## 4. Current milestone

The current milestone is:

- a grant-reviewer-ready open-core package,
- a deterministic ingest/replay/snapshot/read path,
- a clean public export boundary,
- a reference viewer and demo that prove the substrate is real,
- a staged path toward later canonical publication and decentralization.
