---
doc_id: open_core_split_and_data_boundary_spec
title: Open-Core Split and Data-Boundary Specification
status: draft normative specification
version: v0.1
last_reviewed: 2026-02-14

scope:
  - Defines OPEN / NODE / INDIVIDUAL realm boundaries.
  - Defines canonical vs non-canonical data placement.
  - Defines code and dependency boundaries for open-core separation.
  - Defines build matrix and guardrails for core-only vs full product.

authoritative_for:
  - Open-core split boundaries and dependency-direction rules.
  - Data placement policy for canonical, node-private, and individual-private state.
  - Build-separability guardrails for open-core operation.

not_authoritative_for:
  - Protocol semantics beyond what is delegated by protocol/governance/replay specs.
  - Licensing legal advice.

depends_on:
  - protocol v5.md
  - governance-spec.md
  - deterministic-replay-and-merge-spec.md
  - snapshot-format-v0.md
  - cycle-spec.md
  - protocol v5-appendix-a.md
  - api-contract-read-only.md

conflicts:
  - none known

change_rules:
  - Any change to OPEN/NODE/INDIVIDUAL boundaries MUST preserve one-way dependency direction.
  - Any change to governance activation wording MUST remain cycle-based and aligned to governance-spec.md.

reader_path:
  - prereq: authoritative-index.md
  - next: open-core-boundary-manifest.md

keywords:
  - open core
  - boundary
  - canonical
  - node
  - individual
  - dependency
  - data placement
---

# open-core split and data-boundary spec

## 0. purpose [anchor: purpose]
Define a clean split across:
1. OPEN (open-source open core): the verifiable canonical substrate and minimal reference surfaces.
2. NODE (operator realm; initially corporation-run first node): availability, accounts, and optional convenience services.
3. INDIVIDUAL (user realm): user-controlled keys and private local state.

This specification defines:
- data placement boundaries (what is stored where),
- code boundaries (what belongs to open core vs proprietary layer),
- dependency rules (allowed arrows and forbidden arrows),
- build/run matrix (core-only vs full),
- enforcement guardrails that keep the split clean.

Normative pointers:
- governance activation semantics are cycle-based and authoritative in `docs/governance-spec.md` section `normative_activation_semantics_decision_and_snapshot_boundary`.
- cycle boundary derivation is defined in `docs/cycle-spec.md` section `canonical_boundary_event_cycle_close`.
- snapshot role is verification/checkpoint only, not activation boundary, per `docs/snapshot-format-v0.md` section `snapshot_boundary_activation_definition`.
- governance verdict metadata schema home is `docs/protocol v5-appendix-a.md` sections `a2_6_1_required_fields` and `a4_4_6_challenge_finalize_verdict`.

## 1. definitions [anchor: definitions]
- canonical: participates in deterministic replay and affects shared truth/importance/action/governance outcomes.
- non-canonical: does not affect canonical replay; includes drafts, personalization, account/session data, and caches.
- decision cadence: cycle-based (governance decisions confirmed at cycle close).
- activation: cycle-based (`activation_cycle_index = decision_cycle_index + delay_cycles`).
- snapshot: verification/checkpoint artifact keyed to block height; not a semantic activation boundary.

## 2. data boundary model (open / node / individual) [anchor: data_boundary_model_open_node_individual]

### 2.1 data tiers [anchor: data_tiers]
`open / canonical replicated`:
- canonical event log (append-only ordered events),
- canonical snapshots + commitments (`state_root_hash`, `shared_map_commitment`),
- protocol/conformance artifacts (test vectors, verification scripts),
- canonical read API contract surface,
- canonical write surface only when protocol-defined and replay-deterministic.

`node / operator non-canonical`:
- accounts, sessions, auth,
- rate limits, abuse controls, moderation queues (operational state),
- rebuildable indexes/caches derived from canonical state,
- optional convenience storage (explicitly non-canonical):
  - encrypted blob sync (opaque to node),
  - private drafts,
  - bookmarks/saved searches/UI preferences,
- hosted glue services (billing/support/analytics).

`individual / user-controlled`:
- private keys/identity vault material,
- local-only notes, annotations, and UI state,
- private drafts before publication,
- private overlays not published to canonical log,
- optional encrypted backups controlled by the user.

### 2.2 data placement matrix (normative) [anchor: data_placement_matrix_normative]
### 2.2 data placement matrix (normative) [anchor: data_placement_matrix_normative]
| --- | --- | --- | --- | --- | --- |
| canonical event log | yes | yes | yes (host/mirror) | optional cache | OPEN defines meaning and conformance |
| canonical snapshots + commitments | yes | yes | yes (host/mirror) | optional cache | verification artifacts only |
| canonical query indexes | no (derived) | optional | yes | optional | rebuildable caches |
| accounts/sessions/auth | no | no | yes | optional | MUST NOT affect replay |
| rate limits/mod queues | no | no | yes | no | operational only |
| private drafts (server convenience) | no | no | yes | yes | MUST be labeled non-canonical |
| private drafts (local) | no | no | no | yes | default behavior |
| private notes/annotations | no | no | optional (encrypted blob) | yes | user-controlled |
| personal overlays (published) | depends | yes (if published) | yes | yes | canonical only when published |
| keys/identity vault | no | no | no (except opaque blob) | yes | NODE custody never required |

## 3. code boundary model (open core vs proprietary) [anchor: code_boundary_model_open_core_vs_proprietary]

### 3.1 open core includes [anchor: open_core_includes]
`specs + conformance`:
- `docs/` authoritative Tier 1-3 specs,
- `docs/map.*.md` entrypoints and `docs/authoritative-index.md`,
- reproducibility verification scripts.

`backend canonical engine + reference node`:
- deterministic primitives: encoding, event-log validation, replay, snapshot, canonical storage, verification,
- reference API server for canonical reads (and protocol-defined gated writes),
- snapshot builder/verify tooling,
- seed import tooling.

`minimal reference viewer` (recommended open):
- read-only browsing of canonical ideas/connections/ranks/challenges,
- consumes canonical read endpoints only,
- no private overlays/auth/product-game dependency.

### 3.2 corporation/proprietary includes [anchor: corporation_proprietary_includes]
`product experience ui`:
- builder mode and advanced UX,
- game layer (for example pixi/gardencanvas),
- proprietary assets.

`hosted services not required for canonical validation`:
- accounts/billing/support tooling,
- AI helper product features (advisory only).

`optional node-only premium features`:
- private draft storage,
- premium indexing/search,
- private collaboration tools.

All above remain non-canonical.

## 4. repository mapping (current -> target) [anchor: repository_mapping_current_to_target]

### 4.1 current observed situation (high-level) [anchor: current_observed_situation_high_level]
- backend is structurally close to separable open core.
- frontend reference surfaces have some coupling risk with builder/private concerns.
- ordering/domain utilities include both canonical and private overlay concerns in places.
- the active repository model is now two-repo: this public open-core repo is authoritative for open-core materials, while private companion repositories are authoritative for private/product materials and integration glue.

### 4.2 target layout (recommended) [anchor: target_layout_recommended]
`option a (single repo, folder-separated)`:
- `open/backend/`
- `open/docs/`
- `open/frontend-reference/`
- `proprietary/frontend-product/`
- `proprietary/assets/`
- `proprietary/hosted/`

`option b (two repos)`:
- Public open-core repo: open-core backend, open-core docs/specs, reference viewer, public verification scripts, public reviewer/demo materials, and open-core export manifest.
- Private companion repositories: proprietary/product frontend, game and private overlay code, hosted/private integration, private docs, and glue that consumes or mirrors open-core contracts.

Both options use identical boundary rules.

In the active two-repo model, the public repo is authoritative for open-core changes. The private repo may mirror or consume those materials, but private copies must be labeled as mirrors/integration references when they duplicate public open-core docs or tooling.

## 5. dependency rules (normative) [anchor: dependency_rules_normative]

### 5.1 allowed arrows [anchor: allowed_arrows]
- proprietary -> open core,
- node services -> open core,
- shared dto/transport -> both (dto-only, no business logic).

### 5.2 forbidden arrows [anchor: forbidden_arrows]
- open core -> proprietary,
- canonical/reference viewer -> private overlays/auth/builder modules,
- deterministic replay/encoding -> non-deterministic sources unless protocol-defined.

### 5.3 enforcement mechanisms [anchor: enforcement_mechanisms]
`frontend`:
- boundary check via `frontend/open-core-reference/scripts/check-reference-boundaries.mjs`:
  - `frontend/open-core-reference/src/**` MUST NOT import `domains/private/**`, `domains/private-overlay/**`, or `api/private/**`,
  - `frontend/open-core-reference/src/**` MUST NOT import builder/game modules such as `app/workspaceshell`, `app/tabstate`, or `components/viewers/builderview`.

`backend`:
- canonical dto crates MUST NOT depend on private/auth dto crates.
- optional feature gates for private endpoints.

`build`:
- separate targets:
  - core-only backend,
  - reference-only frontend,
  - full product frontend.

## 6. api contract boundaries [anchor: api_contract_boundaries]

### 6.1 canonical read apis (open) [anchor: canonical_read_apis_open]
- public canonical reads (Stage 0 profile),
- contract aligns with `docs/api-contract-read-only.md`.

### 6.2 canonical write apis (open, gated) [anchor: canonical_write_apis_open_gated]
- protocol-defined writes only,
- replay-deterministic validation only,
- semantic validity MUST NOT depend on NODE-private account/session state.

Eligibility checks may use canonical eligibility state and protocol-authorized identity proofs.

### 6.3 node-private apis (node) [anchor: node_private_apis_node]
- auth/session/account,
- private drafts and encrypted blob sync,
- rate limit/moderation controls,
- premium indexing/search.

These surfaces MUST be explicitly labeled non-canonical.

## 7. clean-split work plan (implementation direction) [anchor: clean_split_work_plan_implementation_direction]

### 7.1 frontend: break current coupling [anchor: frontend_break_current_coupling]
- create a dedicated reference viewer shell with no builder/auth imports,
- route reference tab/surface to that shell,
- consume canonical read endpoints only.

### 7.2 frontend: split canonical ordering vs private overlay [anchor: frontend_split_canonical_ordering_vs_private_overlay]
- `domains/canonical-ordering/**`: canonical transforms + canonical fetches,
- `domains/private-overlay/**`: saved ideas/personal workspace state,
- reference viewer consumes canonical-ordering only,
- product builder may consume both.

### 7.3 shared types and contracts [anchor: shared_types_and_contracts]
- canonical dto package for OPEN endpoints,
- private dto package for NODE/private endpoints,
- shared transport package remains dto/transport-only.

## 8. build matrix (minimum) [anchor: build_matrix_minimum]
`core-only`:
- backend build + deterministic verification,
- run api server exposing canonical reads.

`reference-only`:
- build reference viewer,
- connect to running core-only backend.

`full`:
- build/run product frontend + node-private services.

`ci guardrail`:
- core-only and reference-only jobs MUST pass without proprietary packages/assets present.

## 9. dependency and data guardrail checklist [anchor: dependency_and_data_guardrail_checklist]
- no forbidden imports from OPEN into proprietary code.
- no canonical/reference imports of private overlays/auth/builder modules.
- canonical replay outputs depend only on canonical log + rulebooks + deterministic rules.
- snapshots are treated as verification artifacts only.
- NODE-private loss does not alter canonical replay results.
- INDIVIDUAL key custody remains user-controlled.

## 10. acceptance criteria [anchor: acceptance_criteria]
- reference viewer builds/runs without private/builder/auth imports.
- core-only backend builds and passes deterministic verification scripts.
- open core excludes proprietary assets and premium-only UX logic.
- lint/build guardrails block boundary regressions.
- docs clearly state canonical vs node-private vs individual-private data placement.

## 11. non-goals and legal note [anchor: non_goals_and_legal_note]
- this document is technical structure guidance, not legal advice.
- this document does not select a final software license.
- this document does not redefine protocol semantics outside delegated authoritative specs.

## 12. Profile-v0 canonical identity-admission boundary [anchor: profile_v0_canonical_identity_admission_boundary]

The public open core owns protocol-defined sponsored identity admission: canonical event validation, deterministic replay, snapshots, public canonical read contracts, and the protocol-defined write ingress. Non-canonical applicant requests may be exchanged directly or through relays, but remain NODE or INDIVIDUAL data and have no canonical authority.

Accounts, sessions, private requests, relay-local data, private evidence, private documents, private messages, private contact details, private storage identifiers, secrets, and private AI prompts or outputs MUST NOT determine canonical admission, verification, invitation capacity, key control, or eligibility. A private product integration may consume the public canonical contract, but it MUST NOT become a hidden canonical decision-maker.
