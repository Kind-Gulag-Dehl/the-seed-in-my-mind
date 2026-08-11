---
doc_id: open_core_implementation_status
title: Open-Core Implementation Status
status: derived
version: v0
last_reviewed: 2026-08-10
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
| Canonical-history portability | Implemented and guarded-PostgreSQL verified | Versioned manifest plus canonical block/event byte export, offline validate-only, fresh-target exact import, idempotent retry, replay/snapshot/root/count/fixed-height API equality, bounded negative fixtures, and reviewer/export integration are implemented. Import rebuilds projections only through Open Core replay, never copies derived or private rows, and fails closed on unsupported event families or incompatible migrations. This narrow history artifact is not a `.seedpkg`, closed SeedPackage profile, or Full Recovery implementation. |
| Deterministic replay over canonical events | Implemented | Public backend crates and verification scripts are included in the open-core export. |
| Snapshot generation | Implemented and guarded-reviewer verified | Public `snapshot-builder` commits ideas, connections, native Orderings, and representations. The current fixture import and isolated artifact-root reviewer path passed without writing snapshot output into the source tree. |
| Snapshot verification | Implemented and guarded-reviewer verified | Public `snapshot-verify` requires and commits the same four represented section sets as the builder. The fixed representation-bearing root, missing-representations rejection, source demo, and exported demo all pass. |
| Read-only canonical HTTP API | Implemented and guarded-PostgreSQL verified | The versioned product-read contract exposes runtime capabilities, verified-snapshot pinning and commitment bases, bounded idea batch resolution, bounded exact title/sentence matching, complete idea representation list/detail reads, and replay-resolved historical Ordering representation pointers. The Open-Core-owned JSON artifact and Rust/TypeScript/router drift verifier ship in the public export. |
| Open-core reference viewer | Implemented | Minimal public frontend for browsing snapshot-backed ideas. |
| Open-core export generation | Implemented and guarded-export verified | Deterministic extraction requires an explicit outside-repository output root, includes all compile/runtime conformance dependencies, and passes exported conformance, DTO, boundaries, Rust, frontend, demo, and cleanliness verification. Optional zip output is external and was not generated in this run. |
| Boundary enforcement between public and private code | Implemented | Automated boundary and DTO drift checks are included in CI and local verification. |
| Native authored Ordering substrate (DEC-043) | Implemented | `ordering_create` and `ordering_fork` use explicit `ordering_profile` values (`vine`, `evidence_rail`, or `action_rail`) across validation, canonical encoding, hashing, storage, replay, snapshots, import/export, verification, read APIs, DTOs, and conformance. Evidence Rail and Action Rail are standardized profiles rather than separate substrate types; no live `rail_*` compatibility API or dual-read/write path remains. |
| Current-profile representation and Ordering bindings | Implemented and guarded-PostgreSQL verified | Representation kind, author, vocabulary version, title/description slot rules, typed Ordering subjects, aligned item roles, Action-lane homogeneity, duplicate rejection, and fork preservation are implemented across validation, replay, storage, snapshots, APIs/DTOs, 18 representation-binding vectors, 20 Ordering vectors, and a passing 19-case migration-0025 matrix. |
| Public canonical write surface in the exported open-core package | Partial: signed idea/connection substrate | The default open-core API exposes the self-authenticating `/api/v1/canonical/events` signed authored-candidate ingress for ordinary `idea_create` and `connection_create` only. The implementation includes an env-gated isolated Postgres HTTP integration suite for accepted writes, negative writes, idempotency, candidate preservation, event-log/readback, replay equality, and pre-0022 legacy migration compatibility. Challenges, voting, Tempo claims, cycle closure, beacons, certification, authorization, governance, and token effects remain disabled or outside this surface. |
| Canonical event authorship/signature Profile v0 | Partial runtime implementation | `canonical-event-authorship-and-signature-profile-v0.md` defines Ed25519 Profile v0, signed authored-candidate bytes, `public_key_ref`, replay-derived key state, and required conformance vectors. Runtime Profile-v0 byte construction, key-reference hashing, strict Ed25519 verification, candidate preservation, public event-log audit fields, and identity-bound writer eligibility are implemented for the narrow signed idea/connection ingress only. Current key and writer-eligibility rows used by this ingress are bootstrap/operator/test-provisioned open-core state until the public identity/key lifecycle and writer-eligibility lifecycle are implemented; they must not be described as fully canonical event-derived lifecycle state yet. |
| Profile-v0 sponsored identity admission | Partial internal implementation | Pure validation and cryptography, migration 0023, an atomic storage transition, guarded storage evidence, and standalone replay/snapshot projection exist. Missing are public signed admission ingress, public admission reads/DTOs, main ReplayDriver/Stage-0 binary integration, and cycle-derived capacity/liveness. Existing self/speaker-based, account-coupled, bootstrap/seed-import, and stored `canonical_writer_level` paths remain transitional or compatibility behavior, not complete Profile-v0 conformance. |
| Tempo/Cycle structural support, passive evidence, beacons, certification, and authorization frontier | Spec/conformance only; runtime transitional | The reconciled specs and conformance fixtures define the intended deterministic model. Current runtime `tempo_predicates` side-table booleans, legacy internal `cycle_close` materialization, and limited status reads are transitional and MUST NOT be treated as the final protocol implementation. |
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
