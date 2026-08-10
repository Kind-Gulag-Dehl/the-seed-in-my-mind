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
- precedence rules between public docs during the pre-genesis drafting and
  ratification phase.

It is not authoritative for:
- protocol semantics themselves,
- implementation status beyond the reviewer-facing status split in `open-core-implementation-status.md`.

Authority changes at the ratified graph-native genesis boundary:

- before genesis, the checked-in specifications identified below are the
  ratified source material used to construct and validate the genesis
  candidate;
- genesis commits the initial canonical event sequence, graph-native protocol
  and rulebook ideas, their ordinary connections and representations, and the
  exact bootstrap/conformance profile required to replay them;
- after genesis, the canonical event log and the replay-derived active
  graph-native rulebook commitments are the semantic authority;
- Markdown specifications are then human-readable and conformance-oriented
  projections of that authority. Source documents retained for provenance are
  hashed archives, not a parallel source of protocol meaning.

Until the graph-native genesis package is generated, ratified, and activated,
this transition has not occurred and the current pre-genesis precedence rules
remain in force.

---

## 1. public document classes

### 1.1 pre-genesis authoritative public specs

These documents define protocol or system behavior within the public repo
during pre-genesis construction. After the graph-native authority transition,
their maintained successors are projections of canonical graph state and MUST
NOT override the canonical event log or active rulebook commitments.

- `protocol v5.md`
- `protocol v5-appendix-a.md`
- `canonical-encoding-and-hashing-spec.md`
- `canonical-event-authorship-and-signature-profile-v0.md`
- `deterministic-replay-and-merge-spec.md`
- `node-and-conformance-spec.md`
- `pod-consensus-and-canonical-publication-spec.md`
- `snapshot-format-v0.md`
- `canonical-preservation-and-provenance-spine-spec.md`
- `offline-and-mindseed-spec.md`
- `cycle-spec.md`
- `shared-map-and-payload-bundles-spec.md`

### 1.2 pre-genesis authoritative public specs published for transparency

These documents are part of the intended open-core architecture and are public in this repo for architectural transparency. They are not, by themselves, evidence that the described runtime is fully implemented.

- `tempo-spec.md`
- `challenge-engine-spec.md`
- `governance-spec.md`
- `token-spec.md`
- `verification-spec.md`
- `identity-admission-and-invitation-capacity-spec-v0.md`
- `safety-spec.md`
- `safety-rulebook-interface-mechanics-spec.md`
- `privacy-and-high-risk-submission-spec.md`
- `tribe-spec.md`
- `roles-and-stewardship-spec.md`
- `ai-boundaries-spec.md`

Each of these documents carries an explicit status banner. `open-core-implementation-status.md` remains the authoritative current-state implementation reference.

`identity-admission-and-invitation-capacity-spec-v0.md` is the scoped authority for Profile-v0 identity preparation and admission, sponsor-authored `identity_create` architecture, applicant initial-key possession architecture, initial admitted authority, identity structural roots, invitation eligibility and capacity, admission lineage, admission liveness, and genesis/legacy admission classification. It is subordinate to Protocol v5 constitutional invariants and does not settle exact encodings, domain separators, payload schemas, key-lifecycle payloads, numeric verification thresholds, numeric capacity rates above the constitutional minimum, or runtime implementation status.

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
- `canonical-identity-admission-api-contract-v0.md` (scoped planned public contract; subordinate to protocol, schema, signature, replay, and implementation-status authorities)
- `cross-doc-invariants.md`

---

## 2. precedence rules

When public documents conflict, resolve them in this order.

### 2.1 mechanics and determinism

1) `canonical-encoding-and-hashing-spec.md` wins for canonical primitive encodings, hashing rules, byte-level primitive formats, and commitment primitives.
2) `canonical-event-authorship-and-signature-profile-v0.md` wins for authored event candidates, ordinary human-authorship signature profiles, exact signed bytes, public-key references, and replay-derived identity key state.
3) `deterministic-replay-and-merge-spec.md` wins for deterministic replay and merge semantics.
4) `pod-consensus-and-canonical-publication-spec.md` wins for canonical publication profiles, finalized-prefix rules, and derived block mapping.
5) `node-and-conformance-spec.md` wins for conformance expectations, but must not redefine canonical mechanics.

### 2.2 root protocol semantics

Before the graph-native genesis authority transition:

1) `protocol v5.md` is the public root for constitutional invariants and core semantics.
2) `protocol v5-appendix-a.md` is subordinate to `protocol v5.md`.
3) Subsystem specs are authoritative within their scope, but may not contradict `protocol v5.md`.

After that transition:

1) the canonical event log is the historical authority;
2) replay-derived active graph-native rulebook commitments govern prospective
   semantics at each canonical position;
3) generated protocol/specification projections explain and test that state but
   do not independently change it;
4) archived source documents provide provenance and reconstruction evidence,
   not competing semantic authority.

Cycle/Tempo scope split:
- `protocol v5.md` owns root normative cycle invariants, structural/consequential authority separation, and sealing semantics.
- `cycle-spec.md` owns the detailed subordinate structural cycle-close, certification, and lagged authorization-frontier algorithm.
- `tempo-spec.md` owns target-bound time truth claims, Tempo-context evidence rules using ordinary ideas/connections, certainty-band interpretation, Dmin/Dmax predicates, derived beacons, and Tempo modes.

Identity admission scope split:
- `protocol v5.md` owns constitutional human-first authorship, anti-gatekeeping, identity-root, admission-not-verification, and no-machine-authority invariants.
- `identity-admission-and-invitation-capacity-spec-v0.md` owns Profile-v0 identity preparation and admission architecture, sponsorship, admitted initial authority, `identity_structural_roots`, invitation eligibility and capacity, admission lineage, admission liveness, and genesis/legacy admission classification.
- `protocol v5-appendix-a.md` owns exact canonical event schemas and effects after reconciliation with the identity-admission specification.
- `canonical-encoding-and-hashing-spec.md` owns exact bytes, primitive encodings, hashes, commitments, and canonical no-value encodings.
- `canonical-event-authorship-and-signature-profile-v0.md` owns exact authored-candidate signature construction, key descriptors, `public_key_ref`, signature verification, and replay-derived identity key-state mechanics.
- Replay, verification, cycle, snapshot, API, node/conformance, and subsystem specifications retain authority within their scoped domains and must reconcile with the Profile-v0 admission architecture without redefining it.

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
3) `canonical-event-authorship-and-signature-profile-v0.md`
4) `deterministic-replay-and-merge-spec.md`
5) `node-and-conformance-spec.md`
6) `pod-consensus-and-canonical-publication-spec.md`
7) `cycle-spec.md`
8) `tempo-spec.md`
9) `challenge-engine-spec.md`
10) `governance-spec.md`
11) `token-spec.md`
12) `safety-spec.md`
13) `safety-rulebook-interface-mechanics-spec.md`
14) `snapshot-format-v0.md`
15) `shared-map-and-payload-bundles-spec.md`
16) `offline-and-mindseed-spec.md`
17) `verification-spec.md`
18) `identity-admission-and-invitation-capacity-spec-v0.md`
19) `privacy-and-high-risk-submission-spec.md`
20) `tribe-spec.md`
21) `roles-and-stewardship-spec.md`
22) `ai-boundaries-spec.md`

---

## 4. maintenance rules

- If a public doc is added, removed, or reclassified, update this index.
- If a public map references a document that is not present in this repo, either remove the reference or mark it clearly as not included in this public repo.
- If implementation status changes, update `open-core-implementation-status.md` rather than implying implementation through map or index wording.
- Derived docs must not introduce new semantics.
- A generated post-genesis protocol projection MUST identify the canonical
  height, active rulebook-set commitment, and projection format/version from
  which it was produced.
- Editing a projection or archived source file does not amend the protocol.
  Semantic amendment must use ordinary canonical governance, implementation,
  completion-verification, and cycle-activation procedures.
