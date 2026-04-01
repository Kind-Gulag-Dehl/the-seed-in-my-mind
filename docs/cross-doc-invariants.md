<!-- file: the-seed-in-my-mind/docs/cross-doc-invariants.md -->

# cross-document invariants registry

## 0. purpose

This document lists invariants that MUST remain consistent across all authoritative specifications in `docs/`.

If any authoritative document contradicts an invariant listed here, that document is wrong and MUST be corrected.

For each invariant, record:
- statement: the invariant
- canonical home: where the definitive definition lives
- scope: which documents it constrains
- enforcement: how conformance/tests detect violations

---

## 1. determinism and canonical authorship

### 1.1 human-first canonical authorship
- statement: only verified humans may author canonical events. AI may analyze and propose, but cannot vote, govern, or directly create canonical events without explicit human adoption.
- canonical home: `protocol v5.md`, `ai-boundaries-spec.md`
- scope: all documents
- enforcement: node validation + conformance tests

### 1.2 append-only history
- statement: canonical history is append-only. No retroactive edits; corrections are new events.
- canonical home: `protocol v5.md`, `deterministic-replay-and-merge-spec.md`
- scope: all documents
- enforcement: deterministic replay; log validation

### 1.3 deterministic replay and merge
- statement: given the same canonical log and snapshots, every conformant node derives the same state. Merge preserves conflicts explicitly and never silently reconciles disagreement.
- canonical home: `deterministic-replay-and-merge-spec.md`
- scope: all documents
- enforcement: replay/merge test vectors; conformance suite

---

## 2. ordering and identifiers

### 2.1 ordering derives from canonical log position
- statement: ordering authority derives from canonical log position assigned by finalized canonical publication, not timestamps, not ULID/UUIDv7 sort order, and not claimed creation time. Finalized prefix certificates are the authority source for publication order; derived blocks may expose `(block_height, event_index)` as a packaging address only.
- canonical home: `deterministic-replay-and-merge-spec.md`, `pod-consensus-and-canonical-publication-spec.md`
- scope: all documents
- enforcement: ingestion rules; reject time-based ordering assumptions
note: IDs do not define canonical ordering; ordering is derived from the canonical log.

### 2.4 derived blocks are packaging only
- statement: derived blocks are deterministic packaging artifacts built after canonical publication finality. They MUST NOT be treated as the root authority for ordering, validity, truth, governance, or token semantics.
- canonical home: `pod-consensus-and-canonical-publication-spec.md`, `canonical-preservation-and-provenance-spine-spec.md`
- scope: protocol, appendices, snapshots, preservation, node/conformance, api contracts
- enforcement: conformance checks reject any implementation that lets block packaging alter publication order or replay semantics

### 2.2 single identifier policy
- statement: the protocol MUST define one identifier family and use it consistently wherever IDs are normative (e.g., ULID vs UUIDv7). Mixed semantics are forbidden.
- canonical home: `protocol v5-appendix-a.md` and `canonical-encoding-and-hashing-spec.md` (explicit winner must be recorded)
- scope: `protocol v5.md`, appendices, encoding spec, node/conformance
- enforcement: schema validation + conformance checks

### 2.3 canonical identifier format
- statement: Canonical Identifier Format: UUIDv7.
- canonical home: `protocol v5-appendix-a.md`, `canonical-encoding-and-hashing-spec.md`
- scope: `protocol v5.md`, appendices, encoding spec, node/conformance
- enforcement: schema validation + encoding validation

---

## 3. canonical encoding and hashing

### 3.1 canonical encoding is defined in one place
- statement: canonical byte encodings, field ordering, normalization rules, and hashing inputs are defined only by the canonical encoding spec. Other docs may reference but must not redefine.
- canonical home: `canonical-encoding-and-hashing-spec.md`
- scope: snapshots, bundles, nodes, offline, api contracts
- enforcement: encoding test vectors; hash commitment verification

### 3.2 numeric representation
- statement: numeric representation in canonical artifacts (headers/metadata) must be explicitly defined and consistent (e.g., decimal strings where specified).
- canonical home: `canonical-encoding-and-hashing-spec.md`
- scope: `snapshot-format-v0.md`, `shared-map-and-payload-bundles-spec.md`, `api-contract-read-only.md`
- enforcement: parser/validator tests

---

## 4. snapshots, bundles, and commitments

### 4.1 snapshot semantics
- statement: snapshot format, metadata, and verification rules are defined by the snapshot spec. Other docs must not redefine snapshot metadata.
- canonical home: `snapshot-format-v0.md`
- scope: nodes, bundles, offline
- enforcement: snapshot validation

### 4.2 commitment surfaces
- statement: the set of commitments (e.g., state roots, payload roots, shared map commitments) must be defined once and referenced consistently.
- canonical home: `canonical-encoding-and-hashing-spec.md`, `shared-map-and-payload-bundles-spec.md`
- scope: snapshot spec, node spec, offline spec, api contracts
- enforcement: verifier checks

---

## 5. cycles and tempo

### 5.1 cycles gate pacing, not semantics
- statement: cycle boundaries and cycle-gated mechanics are defined in `cycle-spec.md` and used consistently across challenge/governance/token/offline.
- canonical home: `cycle-spec.md`
- scope: `tempo-spec.md`, `challenge-engine-spec.md`, `governance-spec.md`, `token-spec.md`, `offline-and-mindseed-spec.md`
- enforcement: conformance tests

### 5.2 tempo and time-only mode
- statement: tempo exists to cap acceleration; constrained/time-repair mode restrictions, triggers, and exit conditions (legacy label: time-only mode) must be consistent across node behavior and governance.
- canonical home: `tempo-spec.md`
- scope: `node-and-conformance-spec.md`, `governance-spec.md`, offline specs
- enforcement: node/client behavior requirements

### 5.3 automatic cycle_close boundary emission
- statement: `cycle_close` is emitted automatically when cycle closure predicates are satisfied; it is not human/operator-submitted.
- canonical home: `cycle-spec.md` (`canonical_boundary_event_cycle_close`), `protocol v5.md` (`cycle_sealing_mechanisms_normative`)
- scope: cycle, replay/merge, node/conformance, protocol appendices
- enforcement: replay validation rejects manual/non-mechanical cycle-close submissions

### 5.4 earliest-valid boundary rule
- statement: for each cycle, only the earliest canonical log position where closure predicates hold may produce a valid `cycle_close`; later duplicates are deterministically invalid.
- canonical home: `cycle-spec.md` (`canonical_boundary_event_cycle_close`, `closure_conditions`, `trigger_condition`)
- scope: cycle, replay/merge, node/conformance
- enforcement: deterministic duplicate-boundary rejection during replay

### 5.5 reserved boundary emitter identity
- statement: `system_boundary_emitter` is the only non-human canonical emitter and is limited to mechanically verifiable boundary events (`cycle_close`, optional `snapshot_commit` if enabled); it has no voting/governance/authorship rights for ordinary canonical actions.
- canonical home: `protocol v5.md` (`human_primacy_agent_constraints_and_canonical_authorship`), `protocol v5-appendix-a.md` (`a2_1_3_invariants`)
- scope: protocol, cycle, replay/merge, appendices, node/conformance
- enforcement: author-role validation rules and event-type allowlist checks

### 5.6 canonical voter assignment model
- statement: vote-session pull is canonical voter assignment semantics; any per-challenge preselection is derived/cache-only and MUST NOT alter outcomes.
- canonical home: `protocol v5.md` (`voter_eligibility_juror_selection_and_rate_limited_vote_sessions`), `challenge-engine-spec.md` (`voter_eligibility_and_selection_normative`, `verifiable_random_selection_normative`)
- scope: challenge, replay/merge, API semantics, node/conformance
- enforcement: replay of vote-session candidate generation and anti-grinding checks

### 5.7 eligibility freeze and mana gate separation
- statement: for non-governance challenges, eligibility freezes at the deterministic voting-open boundary; eligibility pool membership is independent of current mana/session capacity, but vote submission requires sufficient deterministic voting capacity.
- canonical home: `challenge-engine-spec.md` (`eligibility_pools`, `eligibility_computation`, `voting_rights_and_constraints`), `protocol v5.md` (`voter_eligibility_juror_selection_and_rate_limited_vote_sessions`), `protocol v5-appendix-a.md` (`a11_1_eligibility_requirement`)
- scope: challenge, protocol appendices, replay/merge, node/conformance
- enforcement: vote acceptance tests validating eligible-but-insufficient-capacity rejection behavior

### 5.8 importance challenge duplicate uniqueness
- statement: Stage 1 importance duplicate uniqueness key is `(domain=importance, context_key, ordered target tuple)`; identical active keys are forbidden, while same pairs in different contexts/domains may coexist.
- canonical home: `challenge-engine-spec.md` (`uniqueness_of_concurrent_challenges_no_duplicate_instances`, `parallel_challenges_on_distinct_instances`)
- scope: challenge, replay/merge, API validation
- enforcement: deterministic duplicate challenge rejection keyed by canonical tuple

---

## 6. governance, safety, tokens

### 6.1 governance equality
- statement: governance is human-equal (one person, one vote within eligibility pools). No weighting by POD, POINT, wealth, or reputation. No permanent elites.
- canonical home: `protocol v5.md`, `governance-spec.md`
- scope: all documents
- enforcement: rulebook constraints; governance audit

### 6.2 safety focuses on payload specificity
- statement: safety redacts/gates harmful specificity, not the existence of ideas. History is preserved; exposure may be gated with transparent "why am I seeing this?" explanations.
- canonical home: `safety-spec.md`, `safety-rulebook-interface-mechanics-spec.md`
- scope: UI docs and any safety-adjacent specs
- enforcement: rulebook mechanics and UI requirements

### 6.3 POD/POINT do not confer authority
- statement: POD/POINT must not weight governance, truth, or importance processes. They may fund actions or incentives, but never decision authority.
- canonical home: `token-spec.md`, `protocol v5.md`
- scope: governance + token + UI docs
- enforcement: rulebook constraints; conformance expectations

---

## 7. canonical substrate and overlays

### 7.1 universal canonical substrate
- statement: all published non-draft ideas and all published representation candidates belong to one universal canonical substrate. No separate tribe-only or personal-only canonical idea plane is permitted.
- canonical home: `protocol v5.md` (`canonical_substrate_and_scoped_overlays`), `tribe-spec.md`
- scope: protocol, tribe, governance, replay/merge, api docs
- enforcement: event/schema validation + replay conformance checks

### 7.2 overlays-only scopes
- statement: `personal`, `tribe`, and `universal` scopes are overlays only. Scoped state is limited to overlay surfaces (relative_importance overlays and scoped display overrides) keyed by deterministic scope identifiers.
- canonical home: `protocol v5.md` (`canonical_substrate_and_scoped_overlays`, `scoped_display_overrides_normative`)
- scope: protocol, tribe, challenge, governance, replay/merge, api docs
- enforcement: scope-key validation + deterministic replay of overlay events

### 7.3 no mirroring-as-copy
- statement: mirroring-as-copy semantics (copying tribe-local ideas into separate public canonical objects) are forbidden. Cross-scope publication semantics must be represented by canonical substrate + overlays, not object duplication.
- canonical home: `protocol v5.md` (`canonical_substrate_and_scoped_overlays`), `tribe-spec.md`
- scope: protocol, tribe, replay/merge, token, api docs
- enforcement: invariant validation + deprecation checks in conformance suite

### 7.4 canonical read/write access policy
- statement: canonical substrate state is always publicly readable; canonical writes are allowed to any identity that satisfies canonical-writer verification requirements (current deployment: Seed verifier-issued `canonical_writer_level`); all canonical claims remain publicly challengeable, and challenge creation/voting use the same canonical-writer eligibility gate.
- canonical home: `protocol v5.md` (`canonical_read_write_access_policy`)
- scope: protocol, API contracts, challenge spec, verification spec, roadmap/planning docs
- enforcement: API access tests (public canonical GET; auth-gated canonical POST), replay validation of challenge/vote author eligibility

---

## 8. conflict log (fill as discovered)

When a conflict is found, add an entry here:

- conflict_id: <short stable id>
  statement: <what conflicts>
  docs: [<file1>, <file2>, ...]
  winner: <file>
  resolution: <what changed / what should change>
  date: <yyyy-mm-dd>
