---
doc_id: deterministic_replay_and_merge_spec
title: Deterministic Replay and Merge Specification
status: authoritative
version: v0
last_reviewed: 2026-06-22

scope:
  - Defines deterministic replay rules and merge semantics that preserve conflicts explicitly.

authoritative_for:
  - Deterministic state derivation from the canonical log and snapshots.
  - Merge ingestion rules and conflict preservation guarantees.

not_authoritative_for:
  - Byte-level encoding details (see canonical-encoding-and-hashing-spec.md).

depends_on:
  - protocol v5.md
  - canonical-encoding-and-hashing-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of node-and-conformance-spec.md conformance language.
  - Any change here requires review of offline-and-mindseed-spec.md reintegration rules.

reader_path:
  - prereq: canonical-encoding-and-hashing-spec.md
  - next: node-and-conformance-spec.md

keywords:
  - deterministic replay
  - merge
  - ingestion
  - conflicts
  - canonical log
---

## Deterministic Replay & Merge Specification [anchor: deterministic_replay_merge_specification]

(Normative - subordinate to Protocol v5; defines the algorithmic rules that guarantee identical state reconstruction and safe reintegration of offline logs.)

---

## 0. Purpose, scope, and authority [anchor: 0_purpose_scope_and_authority]

### 0.1 Purpose [anchor: purpose]

This specification defines the **deterministic replay procedures** required so that any two conformant implementations, given the same canonical event log and rulebook set, reconstruct **identical canonical state** at every height.

It further defines **deterministic merge and publication procedures** for offline publication logs and Seed/Publication Packages such that delayed or disconnected activity can be reintegrated into the canonical universe **without ambiguity, reordering discretion, or semantic drift**. Mutable private Mindseed journals are outside this replay boundary.

The intent of this document is to make all algorithmic assumptions implicit in Protocol v5 explicit, testable, and interoperable. Before genesis, Protocol v5 and its ratified subordinate specifications are the source of semantic authority. After genesis, the canonical event log and replay-derived active graph-native rulebook commitments are authoritative, and this document is their conformance projection.

### 0.2 Scope [anchor: scope]

This specification governs the **procedural and algorithmic rules** for:

- canonical event ordering and application
- deterministic replay and state derivation
- validation and rejection semantics
- challenge-driven state transitions as they affect replay
- snapshot derivation, structure, and verification
- offline log packaging and deterministic reintegration
- lineage tracking and continuity across constitutional breaches and forks

This specification applies to **full canonical nodes**, **verifying nodes**, and any client or tool that claims conformance with Protocol v5 replay guarantees.

It does **not** define new idea types, connection types, governance powers, safety semantics, or token semantics.

### 0.3 Authority [anchor: authority]

Protocol v5 Section 0 invariants are supreme.

If any algorithmic rule in this specification conflicts with Protocol v5, **Protocol v5 SHALL prevail**, and the conflicting rule in this document SHALL be considered invalid.

This specification defines **procedures only**. It introduces **no new semantics**, authorities, or rulebooks, and does not modify the meaning of any concept defined in Protocol v5.

---

## 1. Definitions and primitives [anchor: 1_definitions_and_primitives]

### 1.1 Core terms [anchor: core_terms]

**canonical event log**
The single, ordered sequence of canonical events that defines the authoritative history of the system. The canonical event log is the sole input to deterministic replay and is never edited retroactively.

**block / epoch / snapshot interval**
A protocol-defined **block-height boundary** used for ordering, accounting, governance activation, or snapshot generation. If blocks exist, they provide an ordering container only; semantics remain event-defined.

**cycle boundary (`cycle_close`, `H_close`)**
A protocol-defined structural boundary anchored by a canonical `cycle_close` event at a deterministic replay prefix, addressable by block height `H_close` where block heights are exposed. Cycle-based derived outputs (rankings, POD, POINT mint/melt, mana) may be structurally recomputed at `H_close` using the replay prefix through and including the `cycle_close` event, but consequential effects remain provisional or pending until cycle certification and the lagged authorization frontier authorize them. Events after `H_close` MUST NOT affect structural cycle outputs for that boundary.

**deterministic replay**
The process by which canonical state is reconstructed by applying canonical events in canonical order, under the active rulebook set, using only deterministic algorithms and canonical inputs.

**canonical state**
The complete authoritative state derived from replay, consisting of canonical facts (stored/event-derived objects) plus deterministically derived outputs (rankings, balances, safety classifications, indexes). Only canonical facts are committed by `state_root_hash` (see Snapshot Format v0).

**stored state vs derived state**
Stored state consists of canonical facts explicitly recorded in canonical events or snapshots.
Derived state consists of outputs deterministically computable from canonical facts during replay (e.g., rankings, balances, safety classifications, indexes). Derived state MUST NOT introduce new information and is not committed by `state_root_hash`.

**rulebook set**
The complete, ordered set of rulebooks active at a given replay height, as determined by Protocol v5 governance rules and snapshot boundaries.
Replay height corresponds to block height as defined in snapshot-format-v0.md. State reconstruction at snapshot boundaries uses the specified `state_root_hash`, `title_sentence_payload_root` (equal to `pocket_map_payload_root`), and derived `shared_map_commitment`.

**lineage**
A continuous canonical history defined by adherence to Protocol v5 Section 0 invariants. Lineage continuity is determined by invariant compliance, not majority agreement or network dominance.

**seed package / mindseed log**
An append-only offline event log produced while disconnected from the canonical network. These logs follow the same event construction rules as online events but are not canonical until validated and reintegrated.

**publication pack**
A deterministic bundle of events (and optional proofs) extracted from a seed or offline log and submitted for reintegration into the canonical event log.

**breach / constitutional fork**
A breach occurs when a rulebook or event violates Protocol v5 Section 0 invariants. A constitutional fork is the resulting divergence in histories, where only invariant-compliant lineages are considered legitimate.

### 1.2 Inputs and outputs of replay [anchor: inputs_and_outputs_of_replay]

**Inputs**

- the ordered canonical event log as finalized by prefix certificates (or derived blocks containing that already-finalized order)
- the active rulebook set at each replay height
- snapshot checkpoints (optional accelerators only; never authoritative)

**Outputs**

- the canonical state at replay height *H*
- a deterministic state hash or checksum representing that state
- an optional replay audit log (non-canonical, local-only, discardable)

### 1.2A Tempo/Cycle Replay Obligations [anchor: tempo_cycle_replay_obligations]

Replay MUST reproduce the following Tempo/Cycle derived outputs using the Appendix A Tempo/Cycle schemas:

- derived Dmin/Dmax target keys;
- ordinary `truth_claim` ideas with conditional `tempo_claim` metadata;
- Tempo-context potential evidence ideas and actual evidence ideas;
- `evidence_for`, `evidence_against`, and `same_as` connections in Tempo context;
- evidence-placement and certainty-band challenge outcomes;
- Tempo mana balances;
- target-level certainty-band state;
- immutable Tempo profile references;
- current eligible-human structural stances;
- passive evidence normalization, deduplication, outlier, and capped structural contribution state;
- target-level structural-support state;
- Dmin/Dmax predicates;
- structural Dmax liveness predicate state;
- derived beacon states;
- structural cycle boundaries;
- cycle certification status;
- lagged authorization frontier, initialized as `initial_authorization_frontier = -1`;
- normal, constrained, and record-only operating modes, with time-repair substate/reason codes;
- provisional, pending, authorized, and blocked downstream outputs.

Replay MUST NOT create or infer new top-level canonical object types for evidence, attestations, time claims, targets, or beacons. Evidence, observations, attestations, testimony, source statements, arguments, and measurements are identity-authored ideas in roles, connected by existing connection usages and adjudicated through ordinary challenge verdicts.

Replay MUST reject or ignore, according to Appendix A rejection rules, any input that attempts to derive Tempo truth certainty from node-local clocks, server time, client timestamps, receipt time, block height, scheduler observations, local uncommitted observations, AI-generated observations, or publication volume unless that input is represented as valid canonical ideas and connections under explicit protocol rules.

Tempo/Cycle replay MUST use deterministic certainty-band ordering or its canonical integer encoding for ordinary truth certainty only. `T_allow` is a separate structural-support threshold. Floating-point truth certainty, local clocks, server clocks, client timestamps, block-height time authority, passive evidence as event ordering authority, and AI observations as authority are forbidden.

Replay order for Tempo/Cycle state:

1. At genesis, set `initial_authorization_frontier = -1` and derive the immutable bootstrap basis if one is explicitly present in genesis data. Otherwise begin constrained.
2. At each structural cycle start, derive and freeze the active Tempo profile reference and derive `tempo_target(cycle_index, dmin)` and `tempo_target(cycle_index, dmax)` from the anchor event, cycle index, and frozen profile.
3. Validate target-bound time claims as ordinary `truth_claim` ideas with conditional `tempo_claim` metadata. Reject mismatched target keys, profiles, durations, non-target-bound low-threshold claims, and unauthorized authors.
4. Validate Tempo-context evidence ideas and `evidence_for`, `evidence_against`, or `same_as` connections using the ordinary idea and connection schemas plus Tempo-lane eligibility and mana rules. Invalid Tempo-context evidence connections deterministically reject with `ERR_TEMPO_EVIDENCE_CONNECTION_INVALID` and do not contribute to placement challenges, certainty-band derivation, predicates, beacons, certification, or frontier state.
5. At each structural boundary, recharge Tempo mana, cap it, then process valid Tempo-lane spends in canonical order. Invalid events do not spend mana.
6. Aggregate equivalent target-bound claims by `target_key` while preserving separate authorship and visible contradictions.
7. Apply evidence-placement challenge verdicts to determine where actual evidence ideas belong against potential evidence ideas.
8. Apply certainty-band challenge verdicts to derive each target-bound time claim's operative truth-certainty band. Verdict effects do not delete claims.
9. Apply contradiction blocking when contradictory time claims reach the profile's `contradiction_block_band`.
10. Derive current eligible-human structural stances (`support`, `oppose`, `none`) per identity and target from ordinary canonical evidence/stance records. Later valid stances supersede earlier stances for current structural counting only.
11. Normalize passive evidence, deduplicate by profile rule, apply deterministic outlier handling, and cap passive contribution below `T_allow`.
12. Derive structural-support state for Dmin and Dmax from eligible-human support, eligible-human margin, capped passive contribution, `T_allow`, and contradiction blockers. Passive evidence alone cannot satisfy any predicate.
13. Derive `cycle_age_ge_dmin` and `cycle_age_ge_dmax` from structural-support state. Dmax mechanically implies structural Dmin for the same anchor/profile.
14. Derive `structural_dmax_liveness_predicate` for the current Dmax target from nonzero eligible-human survivor support, valid target-bound Dmax claims or stances, required capped passive plausibility evidence, Tempo mana spend acceptance, accepted contradictory target-bound claims, unresolved blocking truth challenges, contradictory certainty-band verdicts, and constrained/time-repair conditions. The status is `true`, `false`, or `blocked`; it is not ordinary truth certainty.
15. Evaluate the earliest-valid structural boundary: close deliberatively when Dmin is true and `W_score >= W_target`; otherwise close forcibly when Dmax is true and `W_score < W_target`; otherwise close forcibly with `trigger = dmax_structural_liveness_forced` when `structural_dmax_liveness_predicate == true` and `W_score < W_target`. Zero eligible-human record-only state emits no universal `cycle_close`.
16. Derive beacon status from ordinary time-claim certainty bands, diversity, independence, stability, contradiction checks, and challenge survivability. The structural Dmax liveness predicate and passive evidence alone do not contribute to beacon status.
17. Derive deterministic beacon coverage and cycle certification: deliberative boundaries require Dmin target certification; forced boundaries require Dmax target certification. A `dmax_structural_liveness_forced` boundary remains pending until normal Dmax beacon certification exists and remains forced permanently.
18. Derive the contiguous lagged authorization frontier:
    `eligible_by_lag = current_cycle - K`;
    `candidate_frontier = min(largest_contiguous_certified_cycle, eligible_by_lag)`;
    `authorization_frontier = max(previous_frontier, candidate_frontier)`.
19. Derive `normal`, `constrained`, or `record_only` mode, with optional `time_repair_priority` constrained substate, from current frontier coverage, constrained allowlist, certification gaps, beacon coverage, publication availability, and human repair availability.
20. Classify downstream outputs as `provisional`, `pending`, `authorized`, or `blocked`. Later certification may finalize explicit pending outputs only and MUST NOT retroactively validate forbidden actions or backfill ordinary mana/rate-limit authority.

All conformant implementations MUST produce identical outputs from identical inputs.
Replay height corresponds to a finalized canonical sequence boundary. When snapshots or APIs surface `block_height`, that height is the derived packaging address defined in snapshot-format-v0.md and `pod-consensus-and-canonical-publication-spec.md`. State reconstruction at snapshot boundaries uses the specified `state_root_hash`, `title_sentence_payload_root` (equal to `pocket_map_payload_root`), and derived `shared_map_commitment`.

### 1.2B Profile-v0 identity-admission replay obligations [anchor: profile_v0_identity_admission_replay_obligations]

For a valid Profile-v0 `identity_create`, replay applies the exact Appendix A and
Signature Profile-v0 schema only at the event's finalized canonical position. The
single atomic transition derives all of the following, or derives none of them:

- canonical existence of the target `CanonicalAdmittedIdentity` with `identity_kind = human`;
- `event_derived` admission provenance, the accepted initial direct key, and key-registration provenance;
- the complete Mindgarden, Backyard of Relationships, Self Tree, and Anthill root set;
- sponsor/admission provenance and the direct sponsor-to-admitted lineage relation;
- exactly one debit of the sponsor's replay-derived invitation-capacity balance; and
- only the restricted verification and direct-key-control lanes permitted by the active
  event-family and rulebook rules.

Admission does not derive VH, VI, human uniqueness, ordinary writing or challenge
eligibility, voting, governance, Tempo eligibility, inviter eligibility, capacity,
economic authority, a private account, or a mutable universal identity status.

The same identity event, key transition, or capacity debit MUST be idempotent when the
canonical event model recognizes an exact already-accepted retry. A conflicting
duplicate, rejected candidate, or invalid event derives no identity, key, root, lineage,
eligibility, or capacity effect. Replay MUST use finalized log order and the active
canonical rulebook only; database row order, API arrival order, wall-clock time,
operator discretion, private-account state, local storage behavior, and AI output are
forbidden inputs.

### 1.3 Deterministic serialization and hashing dependencies [anchor: deterministic_serialization_and_hashing_dependencies]

All deterministic replay, state hashing, snapshot verification, and authorship-signature checks depend on **canonical serialization, hashing, and authored-candidate signature rules**.

This specification defers low-level encoding, canonical byte ordering, hashing algorithms, and primitive field formats to the **Canonical Encoding and Hashing Specification (v0)**. It defers ordinary human-authored event candidates, `signature_profile`, exact signed bytes, `public_key_ref`, and replay-derived identity key state to **`canonical-event-authorship-and-signature-profile-v0.md`**.

Implementations MUST:

- use the canonical byte formats defined in the Canonical Encoding and Hashing Specification (v0)
- use only the hash function(s) specified in the Canonical Encoding and Hashing Specification (v0)
- reject any event, snapshot, or package that does not conform to those definitions
- reject any ordinary human-authored event candidate that does not conform to `canonical-event-authorship-and-signature-profile-v0.md`

No implementation MAY substitute alternate encodings, hashes, or serialization shortcuts, even if functionally equivalent.



## 2. Canonical event ordering [anchor: 2_canonical_event_ordering]

### 2.1 Ordering invariants [anchor: ordering_invariants]

All conformant nodes **MUST** apply canonical events in **identical order**.

Canonical ordering **MUST NOT** depend on:

- local receipt time
- wall-clock time or timestamps subject to drift
- network topology, peer identity, or propagation path
- node-local heuristics or optimization strategies

Canonical ordering is a **property of the canonical log itself**, not of how or when a node receives events.

Any implementation that applies the same set of canonical events in a different order is **non-conformant**, even if the resulting state appears superficially similar.

### 2.2 Ordering source of truth [anchor: ordering_source_of_truth]

The source of truth for ordering is the finalized prefix-certificate chain defined by `pod-consensus-and-canonical-publication-spec.md`.

- Events are first ordered by their position in the finalized canonical sequence.
- If a deployment exposes derived blocks, the public canonical address is then surfaced as:
  1. `block_height` (ascending), then
  2. `event_index` within the derived block (ascending).

Derived block boundaries and block heights are deterministic packaging outputs of the finalized canonical sequence. They are replay-verifiable, but they are not the root authority for order.

**Tie handling:**

- Ties (e.g., two events claiming the same position) MUST be resolved deterministically **only if explicitly permitted by Protocol v5**.
- When permitted, tie-breaking MUST use canonical log order or other explicitly defined canonical ordering, not lexicographic identifier ordering.
- If tie resolution is not permitted by the active rulebook set, the situation MUST be treated as a validation failure.

Nodes MUST NOT invent or infer ordering rules beyond those defined by the protocol and rulebooks.

### 2.3 Forbidden ordering inputs [anchor: forbidden_ordering_inputs]

The following inputs are **explicitly forbidden** from influencing canonical event ordering:

- local or remote timestamps
- wall-clock time or time synchronization mechanisms
- peer priority, reputation, or trust weighting
- subjective importance, ranking, or popularity
- AI scoring, heuristic sorting, or probabilistic ordering

Any implementation that uses forbidden inputs to determine ordering violates deterministic replay guarantees.

---

## 3. Replay state model [anchor: 3_replay_state_model]

### 3.1 Canonical state partitions [anchor: canonical_state_partitions]

Deterministic replay operates over a single canonical state composed of the following **partitions**. Canonical facts are committed by `state_root_hash` as defined in Snapshot Format v0; derived outputs are not.

- **identities**
  Canonical identity anchors, identity kind, provenance, admission facts, structural-root
  references, and registered direct-key history. Current key and eligibility outputs are
  replay-derived rather than private-account or implementation-owned state.

- **ideas**
  All canonical ideas, including truth claims, conceptual ideas, actionable ideas, actions, and identity-typed ideas.
  Replay MUST treat ideas as a single universal canonical substrate. Tribe-only canonical idea object classes are forbidden and MUST NOT be created, inferred, or reconstructed during replay.

- **orderings / vines**
  Canonical ordering objects (including `pathway_vine` and `narrative_vine`) with ordered `idea_id` sequences and ordering representation pointers.

- **connections**
  Canonical connections (`same_as`, `relative_importance`, `membership`) with all protocol-defined metadata.

- **challenges**
  Challenge objects, arguments, votes, verdicts, and lifecycle state.

- **representations / descriptions**
  Canonical representation objects (including candidate/competing representations) and canonical representation pointers selected through replay of finalized representation challenge verdicts.

- **governance / rulebook state**
  Adopted rulebooks, governance proposals, activation boundaries, and supersession history.

- **token balance outputs (POD / POINT)**
  Deterministically derived balances and accounting metadata computed from canonical facts and the active token rulebooks. These are derived outputs and are not committed by `state_root_hash`.

- **safety classification outputs (derived)**
  Deterministically derived classification and visibility outputs computed from canonical facts and active safety rulebooks. These are derived outputs and are not committed by `state_root_hash`.

  **Important:**
  Visibility labels, gating decisions, blurring, hiding, or jurisdictional presentation outcomes are **non-canonical** and MUST NOT be included in canonical replay state or state hashes.

- **derived indexes (non-authoritative)**
  Rank lists, eligibility pools, search indexes, caches, and other accelerators that are recomputable from canonical state and excluded from `state_root_hash`.

### 3.2 Stored vs derived state [anchor: stored_vs_derived_state]

Each partition is classified as either **stored** or **derived**:

**Stored state** includes:
- identities
- ideas
- orderings / vines
- representations / descriptions (representation objects plus canonical representation pointers)
- connections
- challenges and verdicts
- governance and rulebook state

Stored state is introduced **only** through canonical events or verified snapshots.

**Derived state** includes:
- importance rankings and ordered lists
- POD and POINT balances
- safety classifications and visibility state
- voter eligibility pools
- identity verification artifacts' VH and VI certainty, event-family eligibility lanes,
  invitation capacity, invitation suspension, maturation, and admission-liveness state
- search and navigation indexes
- UI-oriented aggregates and summaries

Derived state MUST be:
- deterministically computable from stored state, and
- safely discardable and recomputable at any time.

Snapshots MAY include derived state for convenience or historical record, but derived state is excluded from `state_root_hash` per Snapshot Format v0.

Derived indexes **MUST NOT** be treated as authoritative unless:
- they are explicitly anchored to canonical events or snapshots, and
- their derivation algorithm is deterministic and specified.

For Profile v0, replay maintains separate derived identity lanes rather than one
mutable lifecycle status: canonical existence; identity kind; key control and key
history; structural-root completeness; sponsor/admission provenance and lineage;
verification state; VH and VI certainty; restricted-verification, ordinary-writer,
ordinary-challenge, voter, governance, Tempo, and inviter eligibility; invitation
capacity balance; invitation suspension; maturation; dormancy or recovery state; and
the applicable `admission_liveness_blocked` status. Stored compatibility fields such as
`canonical_writer_level` are historical/materialized inputs only. They are not final
protocol authority and cannot override replay-derived lanes.

### 3.3 Deterministic state hash [anchor: deterministic_state_hash]

A **deterministic state hash** represents the canonical facts commitment at a given replay height (i.e., `state_root_hash` as defined in Snapshot Format v0).
Replay height corresponds to block height as defined in snapshot-format-v0.md. State reconstruction at snapshot boundaries uses the specified `state_root_hash`, `title_sentence_payload_root` (equal to `pocket_map_payload_root`), and derived `shared_map_commitment`.

**Requirements:**

- All required canonical fact partitions MUST be included.
- Derived outputs (e.g., rankings, POD/POINT balances, safety classifications/visibility state, indexes) MUST be excluded.
- Keys and collections MUST be ordered deterministically.
- Encoding MUST use the canonical byte formats defined in the Canonical Encoding and Hashing Specification (v0).
- Hashing MUST use the reference hash function(s) defined in the Canonical Encoding and Hashing Specification (v0).

**Purpose of the state hash:**

- snapshot verification
- node-to-node consistency checks
- interoperability and conformance testing
- deterministic replay audits

Any two conformant implementations replaying the same canonical inputs to the same height MUST compute **identical state hashes**.



## 4. Event validation pipeline (deterministic) [anchor: 4_event_validation_pipeline_deterministic]

All canonical events MUST pass through the same deterministic validation pipeline prior to application.

Validation is performed **during replay**, not only at ingestion time. An event that was previously accepted by a node MUST still be rejected if replayed under the same inputs and rules.

Validation outcomes MUST be identical across all conformant implementations.

### 4.1 Envelope validation [anchor: envelope_validation]

Envelope validation verifies that an event is structurally admissible before inspecting its semantic payload.

Each event envelope MUST satisfy all of the following:

- **Signature verification**
  - An ordinary human-authored event candidate MUST carry a valid cryptographic `signature`.
  - The signature MUST verify against the claimed `author_identity_id` using the Profile-v0 signed bytes and replay-derived identity key state defined in `canonical-event-authorship-and-signature-profile-v0.md`.
  - Publication-derived fields such as `event_index`, block height, finalized-prefix-certificate data, and local receipt metadata MUST NOT be required to reconstruct the human-authorship signed bytes.

- **Author identity verification**
  - The author identity referenced by the event MUST exist in canonical state at the time of application.
  - The author identity MUST meet the verification requirements defined by the active rulebook set (e.g., human verification where required).

- **Schema version check**
  - The event MUST declare a schema/version identifier.
  - The schema version MUST be supported and valid under the active protocol and rulebook set.
  - Unknown or deprecated schema versions MUST cause deterministic rejection.

Events that fail envelope validation MUST NOT proceed to payload or invariant validation.

### 4.2 Payload validation [anchor: payload_validation]

Payload validation verifies that the event content is internally consistent and structurally correct.

Each event payload MUST satisfy all of the following:

- **Schema validation**
  - The payload MUST conform exactly to the canonical schema for its declared event type.
  - Missing required fields or invalid field formats MUST cause rejection.

- **Referential integrity**
  - All referenced identifiers (ideas, connections, challenges, identities, rulebooks, etc.) MUST exist at the replay height where the event is applied.
  - Forward references are forbidden unless explicitly permitted by Protocol v5 or an active rulebook.

- **Type correctness**
  - All referenced objects MUST be of the expected canonical type.
  - For example, a `relative_importance` connection MUST NOT reference an invalid idea type or an incompatible scope.

Payload validation MUST be performed deterministically and MUST NOT rely on external context, caches, or heuristics.
Replay height corresponds to block height as defined in snapshot-format-v0.md. State reconstruction at snapshot boundaries uses the specified `state_root_hash`, `title_sentence_payload_root` (equal to `pocket_map_payload_root`), and derived `shared_map_commitment`.

### 4.3 Invariant validation (Protocol v5 §0) [anchor: invariant_validation_protocol_v5_0]

Invariant validation enforces the **constitutional constraints** defined in Protocol v5 Section 0.

Each event MUST be rejected if it violates any of the following invariants:

- **Human-first authorship**
  - All canonical events MUST be authored by exactly one verified human identity.
  - AI systems MAY draft or propose events but MUST NOT be the canonical author.

- **No retroactive edits**
  - Events MUST NOT modify or erase prior canonical events.
  - All removals, reversals, or corrections MUST be represented via new events (tombstones or superseding events).

- **No forbidden identity types**
  - Events MUST NOT introduce identities that violate Protocol v5 constraints (e.g., fictional entities, abstract collectives, gods).

- **No new connection types**
  - Events MUST NOT introduce new canonical connection types beyond those defined in Protocol v5.
  - Specialization MUST occur only through permitted metadata (e.g., usage, axis, timeframe, scope).

- **No tribe-only idea object classes**
  - Events, rulebooks, and replay procedures MUST NOT introduce a tribe-only canonical idea object class.
  - Tribe semantics in canonical replay MUST be expressed only through scope-constrained overlays on universal canonical ideas.

- **AI non-authority enforcement**
  - Events MUST NOT grant voting, governance, or canonical authority to AI agents.

Any violation of Protocol v5 Section 0 invariants constitutes a **constitutional breach** and MUST result in deterministic rejection.

### 4.4 Rulebook validation [anchor: rulebook_validation]
#### 4.4.1 Governance activation scheduling (normative bridge) [anchor: governance_activation_scheduling_normative_bridge]

Replay MUST apply governance rule changes using canonical scheduling inputs only:
- `decision_event` confirmed at cycle close,
- `decision_cycle_index` from canonical log order,
- `change_class` from verdict metadata,
- `delay_policy_version` active at `decision_cycle_index`,
- a valid canonical implementation-completion claim and the evidence status required by the active rulebook.

Nodes MUST deterministically compute:
- `delay_cycles = delay_policy(change_class)` under `delay_policy_version`,
- `activation_cycle_index = decision_cycle_index + delay_cycles`.

Rule changes become active at the start of `activation_cycle_index` (inclusive) only when the required completion claim remains valid at that boundary. A successful governance verdict without qualifying implementation-completion evidence MUST NOT activate a rule change. Snapshot boundaries are verification/checkpoint artifacts and MUST NOT define governance activation boundaries. This bridge is authoritative for replay/conformance and aligns with Protocol v5 + Governance specs.

After invariant validation, events MUST be validated against the **active rulebook set**.

Rulebook validation follows these rules:

- The applicable rulebook set is derived from ordinary rulebook ideas, governance verdicts, qualifying implementation-completion evidence, and cycle-boundary activation history at the event’s replay position.
- Rulebook supersession and activation MUST follow the governance rules defined in Protocol v5.
- Events that depend on permissions, thresholds, or procedures defined by rulebooks MUST satisfy those conditions exactly.
Replay height corresponds to block height as defined in snapshot-format-v0.md. State reconstruction at snapshot boundaries uses the specified `state_root_hash`, `title_sentence_payload_root` (equal to `pocket_map_payload_root`), and derived `shared_map_commitment`.

**Constitutional override handling:**

- If a rulebook or governance proposal violates Protocol v5 Section 0 invariants, it is invalid.
- Events relying on an invalid rulebook MUST also be rejected.
- Invalid rulebooks MUST NOT activate, even if adopted by majority vote.

### 4.5 Deterministic failure semantics [anchor: deterministic_failure_semantics]

All validation failures MUST be handled deterministically and identically by all conformant nodes.

Canonical replay recognizes **only** the following outcomes:

- **Reject**
  - The event is invalid and MUST NOT be applied to canonical state.
  - Rejected events MUST NOT affect canonical state, replay ordering, state hashes, or any derived state.

- **Ignore non-canonical payloads**
  - Payloads explicitly marked as non-canonical (e.g., local annotations, UI hints, operator notes) MAY be ignored during replay.
  - Ignoring such payloads MUST NOT affect canonical state, replay ordering, or state hashes.

Any concept of **quarantine**, operator inspection, temporary holding, or visibility labeling applies **exclusively to non-canonical, local storage layers**.

Quarantine mechanisms are **strictly operational conveniences** and MUST NOT:

- be applied to canonical state,
- influence replay ordering,
- influence state hash computation,
- influence validation outcomes,
- influence acceptance or rejection decisions,
- or introduce any branch, delay, or ambiguity in canonical replay.

All conformant nodes replaying the same canonical inputs MUST reach the same accept/reject decisions regardless of any local quarantine or inspection behavior.


## 5. Event application semantics (state transition rules) [anchor: 5_event_application_semantics_state_transition_rules]

### 5.1 General application rules [anchor: general_application_rules]

All valid canonical events MUST be applied according to the following rules:

- Events are applied **atomically**.
- Each event defines a single, discrete state transition.
- State transitions MUST be deterministic, total, and pure functions of:
  - the prior canonical state, and
  - the event payload.

Partial application, best-effort application, or implementation-defined side effects are forbidden.

### 5.2 Idempotency and replay safety [anchor: idempotency_and_replay_safety]

Canonical events MUST be safe to reapply during replay from genesis.

To ensure replay safety:

- Event application MUST NOT produce side effects outside canonical state.
- Event application MUST NOT depend on external I/O, system clocks, randomness, or network state.
- Applying the same event twice in the same replay context MUST NOT produce divergent results.

Implementations MUST be able to discard all in-memory state and reconstruct canonical state solely by replaying the canonical event log.

### 5.2A Profile-v0 identity and direct-key application [anchor: profile_v0_identity_and_direct_key_application]

`identity_create`, `identity_key_rotate`, and `identity_key_revoke` use the exact
schemas, proofs, validation precedence, and non-reuse rule in Protocol v5 Appendix A,
the Canonical Encoding and Hashing Specification, and the Profile-v0 Authorship and
Signature Specification. Replay MUST NOT reinterpret their bytes or replace their
errors.

The direct-key model has one active key per identity. A valid rotation atomically
supersedes the current active key and activates the proven replacement. A valid
revocation has only the narrow Appendix A purpose of marking a superseded key revoked;
it cannot revoke the sole active key in Profile v0. Historical signatures remain valid
when their key was active at their own finalized position. A later supersession or
revocation does not rewrite accepted history. Replay records active, superseded,
revoked, malformed/invalid, and historically reused-key rejection states with the
event/provenance that caused each transition.

No direct-key recovery, reassignment, hidden operator override, or multiple-active-key
extension is implied by this section. Those require a later profile.

### 5.2B Compatibility-only identity verification records [anchor: compatibility_only_identity_verification_records]

`identity_verification_update` is not an ordinary post-genesis validation or authority
transition. Replay accepts it only when the exact versioned genesis, import, or legacy
manifest required by Appendix A authorizes the compatibility record. It preserves its
closed provenance class and historical status as compatibility information only.

Such a record MUST NOT be translated into Profile-v0 sponsored admission, an applicant
proof, a sponsor, a capacity debit, an admission lineage edge, ordinary truth/evidence
material, VH, VI, or any ordinary writer, inviter, voter, governance, Tempo, or economic
eligibility. Ordinary long-term verification derives from canonical claims, evidence,
contradictions, challenges, responses, outcomes, active rulebooks, and activation
boundaries. A manifest cannot create a continuing operator-controlled status setter.

### 5.3 Tombstones and reversals [anchor: tombstones_and_reversals]

Canonical state supports removal and reversal through **forward-only events**.

Rules:

- No canonical object is ever deleted from history.
- Removal, invalidation, or supersession is represented by a new event (e.g., a tombstone or status-change event).
- The historical record always retains the original object and all subsequent modifications.

The **current-view state** at any replay height is determined by applying the latest valid status according to deterministic rules defined by the active rulebook set.
Replay height corresponds to block height as defined in snapshot-format-v0.md. State reconstruction at snapshot boundaries uses the specified `state_root_hash`, `title_sentence_payload_root` (equal to `pocket_map_payload_root`), and derived `shared_map_commitment`.

Reversals or corrections MUST occur only through new canonical events and MUST NOT modify prior events retroactively.


### 5.4 Ordering event application semantics [anchor: ordering_event_application_semantics]

Ordering/vine events are applied as deterministic stored-state transitions:

- `ordering_create`
  - Adds a new Ordering object with ordered `item_idea_ids`, explicit `ordering_profile`, and profile-valid metadata.
  - A Vine has no subject or standardized roles. An Evidence Rail references an already-existing `truth_claim` subject and has one aligned `potential_evidence`/`actual_evidence` role per item. An Action Rail references an already-existing `actionable_idea` subject and is one role-homogeneous `potential_action` or `proposed_action` lane.

- `ordering_fork`
  - Adds a new Ordering object that references `base_ordering_id`, repeats the base Ordering's `ordering_profile`, and carries a full replacement ordered item list.
  - A fork whose profile or subject differs from its base is invalid. A retained item whose role differs from its base role is invalid, and an Action Rail fork whose lane differs from its base lane is invalid.
  - Existing Orderings are not edited in place (fork-only model).

- `representation_create`
  - Creates candidate/competing representation objects for both ideas and orderings, without pointer selection changes.
  - Replay requires the payload and materialized row to preserve `author_identity_id`; the author MUST equal the event speaker and MUST already exist at the event position.
  - `vocabulary_version_id` is required exactly for `tier_complexity = canonical`, forbidden otherwise, and MUST reference an already-existing ordinary idea. Replay never substitutes an active, latest, title-derived, or text-derived vocabulary.

`representation_create` is the only live representation-creation event for Ordering targets. No Ordering-specific compatibility alias is valid.

Deterministic event ordering is sufficient to merge concurrent Ordering activity. No additional Ordering-specific merge heuristic is permitted.

Subjects, roles, authors, and vocabulary references are canonical facts. They are
included in replay state and snapshot commitments, and divergence in any one produces a
different state commitment. Selected/completed action state remains derived from
challenge and action events; replay MUST NOT infer it from Action Rail membership.

### 5.5 Overlay event application semantics [anchor: overlay_event_application_semantics]

Overlay events are replayed as deterministic stored-state transitions over universal canonical substrate. Overlay replay MUST use only canonical inputs (event payloads, canonical ordering, active rulebooks) and MUST NOT depend on client-local caches, UI state, or private drafts.

Overlay scope is identified by:
- `scope_key = (scope_kind, anchor_id)`

#### 5.5.1 Overlay connection events

The following event family is recognized:
- `overlay_connection_create`
- `overlay_connection_update`
- `overlay_connection_delete`

For deterministic replay, each overlay connection event MUST resolve a stable merge key:
- `overlay_connection_merge_key = (scope_kind, anchor_id, from_idea_id, to_idea_id, usage, axis, timeframe)`

Application rules:
- `overlay_connection_create`
  - Appends a new canonical overlay-connection state record for the resolved merge key.
  - If prior state exists for the same merge key, replay preserves history and computes effective state by deterministic resolution order (see §5.5.3).

- `overlay_connection_update`
  - Applies to the resolved merge key.
  - Updates the effective payload for that merge key according to rulebook-defined field semantics.
  - History remains append-only; no prior event is edited.

- `overlay_connection_delete`
  - Applies a forward-only tombstone to the resolved merge key.
  - Prior records remain historical; effective state becomes inactive unless later re-created by a subsequent valid event.

#### 5.5.2 Scoped display override events

The following event family is recognized:
- `scoped_display_override_set`
- `scoped_display_override_clear`

For deterministic replay, each scoped display override event MUST resolve a stable merge key:
- `scoped_display_override_merge_key = (scope_kind, anchor_id, target_kind, target_id, display_slot_key)`

Application rules:
- `scoped_display_override_set`
  - Sets or replaces the effective override payload for the resolved merge key.
  - Referenced representation candidates MUST already exist in canonical state at apply height.

- `scoped_display_override_clear`
  - Applies a forward-only clear/tombstone to the resolved merge key.
  - Prior override records remain historical and replay-auditable.

#### 5.5.3 Deterministic resolution order and key convergence

For each overlay merge key (`overlay_connection_merge_key` or `scoped_display_override_merge_key`), effective state MUST be derived as follows:

1. Collect all valid events for the merge key in canonical order as defined by §2.
2. Apply events in that order with no local reordering.
3. The latest valid event for the key determines effective state:
   - create/update/set => active effective value
   - delete/clear => inactive effective value
4. Exact duplicate events (identical canonical hash) are idempotent under §10.3.

No alternative merge heuristic (including wall-clock precedence, peer-arrival order, client preference, or AI ranking) is permitted.

#### 5.5.4 Legacy compatibility requirements

Legacy events remain valid.

- Logs that do not contain overlay event families defined in this section MUST replay exactly as before under existing event semantics.
- Rulebooks MAY define deterministic normalization for legacy payload variants, but normalization MUST be canonical, replay-verifiable, and identical across nodes.
- Replay MUST NOT require client-local migration state to interpret legacy events.


## 6. Challenge-driven state transitions [anchor: 6_challenge_driven_state_transitions]

Challenges are the sole mechanism by which contested questions produce canonical transformations. All challenge effects MUST be replayable, deterministic, and anchored to explicit canonical events.

### 6.1 Challenge lifecycle state machine (algorithmic view) [anchor: challenge_lifecycle_state_machine_algorithmic_view]

Each challenge progresses through the following deterministic lifecycle:
Replay height corresponds to block height as defined in snapshot-format-v0.md. State reconstruction at snapshot boundaries uses the specified `state_root_hash`, `title_sentence_payload_root` (equal to `pocket_map_payload_root`), and derived `shared_map_commitment`.

1. **Creation**
   - A challenge creation event introduces the challenge object.
   - The challenge domain (truth, importance, action, representation) and parameters are fixed at creation.

2. **Argument window**
   - Arguments may be submitted according to the active rulebook set.
   - Argument submission closes at a deterministic boundary (defined by rulebook and replay height, not wall-clock time).

3. **Voter selection**
   - The eligible voter set is computed deterministically (see §6.2).
   - The voter set is fixed for the remainder of the challenge lifecycle unless explicitly allowed otherwise by rulebook.

4. **Voting window**
   - Votes may be submitted by eligible voters.
   - Voting closes at a deterministic boundary defined by rulebook and replay height.

5. **Verdict**
   - Votes are aggregated deterministically (see §6.3).
   - A verdict is computed but does not affect canonical state until finalized.

6. **Transformation**
   - A verdict finalization event applies the canonical state transformation defined by the challenge domain.

**Valid transitions** are those explicitly defined above and permitted by the active rulebook set.
Any event that attempts to skip, reorder, or repeat lifecycle stages MUST be rejected.

### 6.2 Voter eligibility computation [anchor: voter_eligibility_computation]

Voter eligibility MUST be computed deterministically and reproducibly by all nodes.

**Inputs to eligibility computation:**

- the rulebook set active at the challenge's defined reference point (typically challenge creation or argument close)
- identity verification status at that same reference point
- participation constraints defined by rulebooks (e.g., exclusions, minimum activity requirements)

**Rules:**

- Eligibility MUST NOT depend on local time, node discretion, or post-hoc state.
- Eligibility MUST be derived solely from canonical state reachable by replay at the reference point.
- The resulting eligibility list MUST be identical across all conformant implementations.

The computed eligibility list becomes part of the challenge's canonical context and MUST NOT change unless explicitly permitted by rulebook.

### 6.3 Vote aggregation and tally rules [anchor: vote_aggregation_and_tally_rules]

Vote aggregation MUST be deterministic and defined per challenge domain and ballot type.

**Aggregation requirements:**

- The tally method (e.g., majority, weighted rulebook-defined method) MUST be specified by the active rulebook set.
- All votes MUST be validated for:
  - voter eligibility,
  - ballot format correctness,
  - submission timing within the voting window.

**Handling rules:**

- **Ties**
  - Ties MUST be resolved deterministically according to rulebook-defined procedures.
  - If no tie-breaking rule is defined, the outcome MUST be a deterministic “no-change” or equivalent neutral verdict.

- **Invalid ballots**
  - Invalid ballots MUST be excluded deterministically and MUST NOT affect tallies.

- **Late ballots**
  - Ballots submitted after the voting window closes MUST be ignored.

- **Revoked identities**
  - If identity revocation effects are permitted, their impact on eligibility and ballot validity MUST be defined by rulebook and applied deterministically.

### 6.4 Verdict finalization [anchor: verdict_finalization]

A verdict affects canonical state **only** through a verdict finalization event.

Rules:

- The finalization event is the sole canonical trigger for state transformation.
- Once finalized, a verdict becomes part of immutable canonical history.
- Verdicts MUST NOT be edited, amended, or retroactively altered.

Any reversal, correction, or override of a verdict MUST occur via a **new challenge**, producing a new verdict and new transformation events.

### 6.5 Transformation mapping [anchor: transformation_mapping]

Verdict finalization events map to deterministic state transformations based on challenge domain.

**Truth challenges**
- Update truth-status fields, certainty bands, and permitted linkage outcomes.
- No truth object is deleted; status changes are forward-only.

**Importance challenges**
- Apply the Protocol v5 immediate-above bubble-up rule to the exact declared universal or relative rank context.
- Preserve the challenge, ordinary idea-based arguments, ballots, verdict, eligible connection metadata, and prior ordering history.
- Recompute derived universal aggregate state only when a universal axis changes.
- No new connection types are introduced.

**Action challenges**
- Update action acceptance states, disbursement triggers, or compliance markers as defined by rulebook.
- Actions remain historical records regardless of acceptance outcome.

**Representation challenges**
- Replay of `challenge_finalize_verdict` updates canonical representation pointer(s) for an idea or ordering tier slot.
- Prior representation objects remain part of historical record.
- No representation object is deleted or rewritten by verdict application.

---

## 7. Importance ranking derivation (deterministic) [anchor: 7_importance_ranking_derivation_deterministic]

Importance rankings are **derived state**, computed deterministically from canonical inputs.

### 7.1 Ranking input surfaces [anchor: ranking_input_surfaces]

The inputs to importance ranking derivation are limited to:

- the declared `rank_kind` and complete rank-context fields,
- eligible `relative_importance` connections and their metadata for relative contexts,
- verdict outputs from importance challenges,
- deterministic baseline and tie rules defined by the active rulebook set,
- the Protocol v5 exact twenty-axis universal aggregation rule.

No other inputs (e.g., popularity metrics, UI behavior, AI inference) MAY influence rankings.

### 7.2 Deterministic ranking algorithm [anchor: deterministic_ranking_algorithm]

The ranking algorithm MUST satisfy the following:

- Universal-axis rankings are computed per `(rank_kind = universal, universal_orientation, timeframe, scope = universal)`.
- Relative rankings are computed per `(rank_kind = relative, reference_idea_id, usage = general, relative_axis, timeframe, scope, scope_anchor_id_if_any)`.
- For each successful verdict, a challenger that remains below its target is removed from its current position and inserted immediately above the target. Every other idea preserves relative order.
- A loss or a challenger that is no longer below the target produces no movement under the base rule.
- After all affected universal-axis lists are replayed, each idea's exact integer `universal_position_sum`, exact mean `sum / 20`, and `overall_universal_rank` are recomputed.
- Public-relative and tribe-relative lists MUST NOT contribute to the universal aggregate.
- Individual-private rank state MUST NOT enter canonical replay.
- All lists MUST be generated using a deterministic ordering procedure.
- Tie-breaking MUST follow explicit, deterministic rules defined by rulebook.
- If rank movement history fields are stored, they MUST be derived deterministically from prior rankings and verdicts.

Two conformant implementations replaying the same canonical inputs MUST produce identical ranking outputs.

### 7.3 Snapshot-anchored rankings [anchor: snapshot_anchored_rankings]

If rank snapshots are emitted:

- They MUST be derived directly from replayed canonical state at the snapshot boundary.
- They MUST be reproducible from the snapshot's canonical facts at the same boundary.
- Any mismatch between replay-derived rankings and snapshot-stored rankings indicates a derived-view discrepancy and MUST NOT invalidate `state_root_hash` verification.

If rankings are **derived-only** and not stored:

- Implementations MAY cache rankings locally for performance.
- Cached rankings MUST be discardable and recomputable at any time.
- Caching MUST NOT affect canonical replay, state hashes, or conformance behavior.



## 8. Snapshot derivation and verification [anchor: 8_snapshot_derivation_and_verification]

Snapshots are **deterministic accelerators**, not sources of authority. Canonical truth is always defined by replay of the canonical event log under the active rulebook set.

### 8.1 Snapshot intervals and triggers [anchor: snapshot_intervals_and_triggers]

Snapshots MAY be generated at deterministic boundaries, including:

- **derived block intervals** or other protocol-defined grouping boundaries
- **payout epochs** or accounting checkpoints
- **rulebook change boundaries**, where a new rulebook set becomes active

Snapshot generation MUST be deterministic with respect to replay height.
Nodes MUST NOT generate snapshots based on local time, operator preference, or heuristic conditions.
Replay height corresponds to a finalized canonical sequence boundary. Where snapshots expose `block_height`, that height is the derived block address of the last included canonical event.

The snapshot height (i.e., the derived **block height** of the last included canonical event) MUST be explicit and unambiguous.

### 8.2 Snapshot content requirements [anchor: snapshot_content_requirements]

A conformant snapshot MUST include, at minimum. Snapshot Format v0 is authoritative for what is committed by `state_root_hash`; derived outputs MAY be included for convenience or historical record but are excluded from `state_root_hash`.

- **full canonical stored state**, including:
  - identities
  - ideas
  - orderings / vines
  - representations and canonical representation pointers
  - connections
  - challenges and verdicts
  - governance and rulebook state

- **active rulebook set**
  - the exact rulebook identifiers and versions active immediately after the snapshot boundary

- **balances**
  - POD and POINT balances as derived at the snapshot height (derived output; excluded from `state_root_hash`)

- **ranking lists**
  - if stored, the complete set of deterministic importance rankings at the snapshot height (derived output; excluded from `state_root_hash`)

- **certainty bands**
  - truth-status and certainty band state at the snapshot height (derived output; excluded from `state_root_hash`)

- **growth ring metadata references (if required)**
  - references to Ent or stewardship growth metadata where defined by Protocol v5
  - growth metadata itself MUST NOT introduce new canonical semantics

Snapshots MUST include a header containing:
- the covered event range,
- the prior snapshot reference (if any),
- the canonical state hash,
- and a cryptographic signature per Appendix A.

### 8.3 Snapshot verification procedure [anchor: snapshot_verification_procedure]

Snapshot verification MUST be deterministic and identical across nodes.

The verification procedure is:

1. **Replay**
   - Replay canonical events from the last trusted snapshot (or genesis) through the snapshot height.

2. **Compute**
   - Compute the deterministic state hash from replayed state using the Canonical Encoding and Hashing Specification (v0).

3. **Compare**
   - Compare the computed hash with the snapshot header's declared state hash.

4. **Handle mismatch**
   - If hashes match, the snapshot is valid.
   - If hashes differ, the snapshot is invalid and MUST NOT be trusted.

**Failure handling:**

- Invalid snapshots MUST be rejected deterministically.
- Nodes MUST fall back to replay from an earlier valid snapshot or from genesis.
- Invalid snapshots MUST NOT affect canonical ordering or state.

### 8.4 Snapshot acceleration rules [anchor: snapshot_acceleration_rules]

Nodes MAY use verified snapshots to accelerate synchronization and replay.

Constraints:

- Nodes MUST be capable of auditing canonical state **from genesis** given full log availability.
- Snapshots MUST NOT replace or obscure the canonical event log.
- Trust models (e.g., trusted snapshot providers) are **optional overlays** and MUST NOT introduce canonical authority or semantic changes.

Any optimization that prevents full auditability violates conformance.

---

## 9. Offline logs, Mindseed packages, and reintegration [anchor: 9_offline_logs_mindseed_packages_and_reintegration]

Offline operation is treated as **delayed publication**, not alternate semantics.

All offline-produced canonical events are subject to the same validation, ordering, and replay rules as online events.

### 9.1 Offline log structure [anchor: offline_log_structure]

Offline publication activity is recorded as:

- **append-only local event logs**
  - exact signed candidates follow the same schema, signing, and authorship rules as online events

- **deterministic local replay**
  - local state may be replayed deterministically for user feedback and drafting

- **local snapshots (non-canonical accelerators)**
  - local snapshots MAY exist for performance
  - local snapshots are non-authoritative and MUST NOT be treated as canonical

A user-controlled private Mindseed journal is separate from these replayable publication logs. Its idea-compatible records MAY be edited, deleted, pruned, or compacted under private-product policy and are outside deterministic protocol replay until a human separately approves and signs new canonical publication candidates.

Offline logs MAY contain additional non-canonical material (drafts, annotations, private structures) that is ignored during canonical reintegration.

### 9.2 Publication pack composition [anchor: publication_pack_composition]

A **publication pack** is a deterministic bundle submitted for reintegration.

It MUST include:

- **selected event ranges**
  - one or more contiguous ranges of offline-produced canonical events

- **required proofs**
  - cryptographic signatures for all events
  - identity continuity proofs where required by rulebook

- **snapshot references (optional)**
  - references to canonical or local snapshots, if included, used only as accelerators

Publication packs MUST NOT omit required events, reorder events, or include unverifiable payloads.

### 9.3 Reintegration pipeline [anchor: reintegration_pipeline]

Reintegration MUST treat offline-produced canonical events **as if they were received online**, subject to the same rules.

The pipeline is:

1. **Validation**
   - Perform full envelope, payload, invariant, and rulebook validation (§4) on each event.

2. **Publication ordering**
   - Offline events become canonical **only when included in a finalized prefix certificate through the canonical publication mechanism**.
   - A valid signed authored candidate remains non-canonical until publication finality binds the exact candidate bytes to canonical order.
   - Their canonical order is determined solely by finalized publication order and any derived block mapping, not by when or where the events were authored offline.

3. **Handling cases**
   - **Exact duplicates** (identical event hashes): ignored deterministically.
   - **Previously known events**: ignored deterministically.
   - **Invalid or non-conformant events**: rejected deterministically.

Offline origin MUST NOT grant priority, retroactive placement, exemption, or special handling.


### 9.4 Offline merge constraints [anchor: offline_merge_constraints]

Offline semantics MUST be identical to online semantics for all canonical effects.

Constraints:

- **No offline token minting**
  - POD and POINT are derived only through canonical replay and rulebook-defined events.

- **Offline tribe operation**
  - Tribes MAY operate fully while offline:
    - creating ideas,
    - submitting arguments,
    - participating in challenges,
    - and performing other tribe-level interactions permitted by rulebooks.
  - Offline tribe activity follows the same rules as online tribe activity, including public visibility semantics.

- **Visibility and privacy**
  - Offline operation does NOT grant tribes private canonical state.
  - All tribe-authored canonical events become public upon reintegration, exactly as if they were performed online.

- **Local-only material**
  - Offline environments MAY contain personal drafts, notes, or local annotations.
  - Such material is non-canonical and MAY be discarded during reintegration.

Offline operation provides **temporal disconnection**, not altered authority, visibility, or governance semantics.

## 10. Conflicts and convergence across disconnected histories [anchor: 10_conflicts_and_convergence_across_disconnected_histories]

Disconnected operation, delayed publication, and parallel activity naturally produce conflicts. The protocol treats conflicts as **first-class, representable facts**, not as errors to be automatically resolved.

### 10.1 Conflict types [anchor: conflict_types]

The following conflict types MAY arise during replay or reintegration:

- **Referential conflicts**
  - An event references an object that does not exist at the replay height.
  - Such conflicts are validation failures, not semantic disagreements.

- **Competing claims**
  - Multiple truth claims assert mutually incompatible statements.
  - All claims may coexist canonically until challenged and adjudicated.

- **Competing representations**
  - Multiple descriptions or representations exist for the same idea or ordering.
  - Canonical selection occurs only through replay of finalized representation challenge verdicts.

- **Competing orderings / vines**
  - Multiple orderings may encode different orderings over overlapping idea sets.
  - Orderings are fork-only canonical objects; coexistence is preserved unless superseded by later canonical events.

- **Competing importance relations**
  - Multiple `relative_importance` connections assert incompatible orderings.
  - These relations coexist and are reconciled only through importance challenges in their complete relative contexts.
  - Universal aggregation occurs only after the twenty universal-axis lists are replayed; it does not reconcile arbitrary relative relations.

- **Competing scoped overlay writes**
  - Multiple overlay events may target the same overlay merge key while disconnected.
  - Coexistence is preserved in history; effective state is selected only by deterministic key resolution order defined in §5.5.3.

Conflict existence alone does NOT imply invalidity.
Replay height corresponds to block height as defined in snapshot-format-v0.md. State reconstruction at snapshot boundaries uses the specified `state_root_hash`, `title_sentence_payload_root` (equal to `pocket_map_payload_root`), and derived `shared_map_commitment`.

### 10.2 Protocol-consistent conflict handling [anchor: protocol_consistent_conflict_handling]

The protocol **does not resolve conflicts by merge logic**.

Rules:

- Conflicting events and objects MAY coexist in canonical state.
- No automatic suppression, deletion, or override occurs due to conflict.
- Resolution occurs only through:
  - challenges,
  - arguments,
  - votes,
  - verdict finalization events.

Deterministic replay MUST preserve all conflicting canonical objects until explicitly transformed by valid challenge outcomes.

### 10.3 Deterministic duplicate handling [anchor: deterministic_duplicate_handling]

Duplicate handling MUST be deterministic and uniform across nodes.

- **Exact duplicates**
  - Events with identical canonical hashes are idempotent.
  - Reapplying such events MUST have no effect on state.

- **Semantic duplicates**
  - Events or objects with different identifiers but similar or identical content remain distinct.
  - Semantic equivalence is recognized only through explicit `same_as` connections and, if contested, resolved via challenges.

Nodes MUST NOT infer equivalence or collapse objects heuristically.

---

## 11. Lineages, constitutional forks, and continuity [anchor: 11_lineages_constitutional_forks_and_continuity]

Canonical legitimacy is determined by adherence to Protocol v5 Section 0 invariants, not by network dominance or consensus weight.

### 11.1 Lineage tracking (descriptive) [anchor: lineage_tracking_descriptive]

Lineage tracking MAY be recorded as **descriptive, non-authoritative metadata**.

Permitted lineage metadata includes:

- lineage identifiers
- parent lineage references
- fork point markers (event or snapshot references)
- descriptive annotations

Lineage metadata MUST NOT:
- affect canonical replay,
- alter validation outcomes,
- introduce authority or precedence.

### 11.2 Breach detection rules [anchor: breach_detection_rules]

During deterministic replay, nodes MUST identify violations of Protocol v5 Section 0 invariants.

Rules:

- Any event or rulebook violating §0 invariants constitutes a **constitutional breach**.
- Breached events or blocks MUST be rejected deterministically.
- Nodes MUST NOT apply breached events, regardless of majority adoption or network prevalence.

Breach detection MUST be reproducible by any conformant node replaying the same inputs.

### 11.3 Continuity rules under breach [anchor: continuity_rules_under_breach]

When a breach is detected:

- Canonical continuity proceeds from the **last pre-breach valid snapshot or event**.
- Events following the breach are excluded from canonical state.
- Legitimacy is defined solely by invariant adherence, not by vote counts or participant numbers.

Forks that violate §0 invariants are non-canonical, even if widely adopted.

### 11.4 Recording other lineages and external systems [anchor: recording_other_lineages_and_external_systems]

A conformant lineage MAY record information about:

- non-conformant lineages,
- breached forks,
- external or legacy systems,

by representing them as **canonical ideas, truth claims, or observations**.

Constraints:

- No external lineage or system state may be merged directly into canonical state.
- All such records are descriptive only and subject to challenge like any other claim.
- “Merge by reference” (treating external state as canonical without replay) is forbidden.

This ensures historical awareness without compromising canonical integrity.
### 11.5 continuity basis and external representation (normative clarification) [anchor: continuity_basis_and_external_representation_normative]

Canonical continuity is determined by replay conformance to protocol rules plus snapshot commitments (`state_root_hash`, `shared_map_commitment`), not by labels, popularity, or external authority.

Non-conformant or external deliberation systems MAY be represented inside canonical history as claims/evidence, but they remain descriptive/non-authoritative unless and until they are canonically adopted through conformant governance events.


## 12. Conformance requirements [anchor: 12_conformance_requirements]

Conformance ensures that independent implementations produce identical canonical outcomes and can interoperate without trust.

### 12.1 Node conformance [anchor: node_conformance]

A node claiming conformance with Protocol v5 and this specification MUST:

- **Implement canonical ordering rules**
  - Apply events in identical order according to §2.
  - Reject any ordering influenced by forbidden inputs.

- **Implement the validation pipeline identically**
  - Perform envelope, payload, invariant, and rulebook validation exactly as defined in §4.
  - Produce identical accept/reject outcomes for identical inputs.

- **Compute deterministic state hash identically**
  - Include the same canonical partitions.
  - Use the same ordering, encoding, and hash function per the Canonical Encoding and Hashing Specification (v0).

- **Support snapshot verification**
  - Verify snapshots according to §8.3.
  - Reject invalid snapshots deterministically.

- **Support offline reintegration validation**
  - Validate publication packs using the same rules as online events (§9).
  - Reject non-conformant offline submissions without special treatment.

Nodes MUST be capable of full replay from genesis given access to the complete canonical event log.

### 12.2 Client conformance [anchor: client_conformance]

A client (including UIs, SDKs, or tooling) claiming conformance MUST:

- **Construct valid events**
  - Produce events that conform to canonical schemas and signing rules.
  - Reference only valid identifiers and permitted object types.

- **Avoid reliance on non-deterministic sandbox data**
  - Client behavior MUST NOT assume AI drafts, local caches, or UI state are canonical.
  - All canonical effects MUST be explicitly submitted as events.

- **Represent adoption boundaries explicitly**
  - Clearly distinguish between:
    - drafted or proposed content, and
    - content adopted into canonical state by a human-authored event.

Clients MUST NOT implicitly promote non-canonical content into canonical effects.

### 12.3 Interoperability test requirements [anchor: interoperability_test_requirements]

To claim interoperability, implementations MUST pass a common suite of tests, including:

- **Minimum test vectors**
  - A defined set of canonical logs and expected outputs.

- **Replay checksum milestones**
  - State hash checkpoints at specified replay heights.

- **Offline reintegration tests**
  - Publication pack submission and deterministic acceptance/rejection outcomes.
Replay height corresponds to block height as defined in snapshot-format-v0.md. State reconstruction at snapshot boundaries uses the specified `state_root_hash`, `title_sentence_payload_root` (equal to `pocket_map_payload_root`), and derived `shared_map_commitment`.

Failure to pass required tests indicates non-conformance.

---

## 13. Test vectors and reference procedures [anchor: 13_test_vectors_and_reference_procedures]

This section is non-normative but REQUIRED for ecosystem maturity and interoperability.

### 13.1 Replay vectors [anchor: replay_vectors]

Replay test vectors MUST include:

- canonical event log slices
- the active rulebook set
- expected deterministic state hashes at specified heights
- Tempo/Cycle vectors covering:
  - time claims remain ordinary `truth_claim` ideas with `tempo_claim` metadata;
  - rejection of `time_claim`, `tempo_target`, `beacon`, `evidence`, `attestation`, `testimony`, or `source` idea types;
  - valid low-threshold target-bound Tempo claim creation by `tempo_contributor`;
  - Tempo evidence represented only as ordinary ideas plus `evidence_for` / `evidence_against` connections;
  - rejection of arbitrary canonical idea creation, challenge opening, challenge voting, or verdict finalization by `tempo_contributor` lacking ordinary eligibility;
  - rejection of AI/non-human Tempo claim, evidence, challenge, vote, verdict, beacon, cycle, governance, POD, POINT, or token authority;
  - invalid target key, target/profile mismatch, and insufficient Tempo mana rejection;
  - no certainty from raw author counts, equivalent-claim counts, hidden weights, model scores, external links alone, or heuristics;
  - evidence-placement and certainty-band challenge verdicts assigning Tempo certainty;
  - local/server/client time, receipt time, block height, scheduler observations, publication volume, and AI observations failing to affect certainty;
  - Dmax mechanically implying structural Dmin;
  - Dmin plus `W_target` closing deliberatively;
  - Dmax closing forcibly when `W_target` is unmet;
  - forced cycles producing no POD, POINT, governance, lifecycle, ordinary mana, ordinary rate-limit, or final-rank authority;
  - `K` and beacon requirements remaining fixed under collapse;
  - authorization frontier stopping at a certification gap;
  - later certification finalizing explicit pending outputs only;
  - replay reproducing targets, predicates, beacon state, certification, frontier, modes, and output status.
Replay height corresponds to block height as defined in snapshot-format-v0.md. State reconstruction at snapshot boundaries uses the specified `state_root_hash`, `title_sentence_payload_root` (equal to `pocket_map_payload_root`), and derived `shared_map_commitment`.

These vectors allow independent implementations to verify replay correctness.

### 13.2 Snapshot vectors [anchor: snapshot_vectors]

Snapshot test vectors MUST include:

- a snapshot (header + state)
- the corresponding delta event range
- the expected verification outcome (valid or invalid)

Vectors MUST cover both successful verification and deterministic failure cases.

### 13.3 Offline reintegration vectors [anchor: offline_reintegration_vectors]

Offline reintegration vectors MUST include:

- a local offline event log
- the derived publication pack
- the expected canonical validation and ordering outcomes

Vectors MUST demonstrate handling of duplicates, invalid events, and partial submissions.

### 13.4 Breach and fork vectors [anchor: breach_and_fork_vectors]

Breach and fork vectors MUST include:

- sample rulebooks or events that violate Protocol v5 Section 0 invariants
- the expected deterministic rejection behavior
- the expected canonical continuation point

These vectors ensure that breach detection and lineage continuity are implemented consistently.


# Appendix A: Replay and Merge Vectors (Normative Addition to deterministic-replay-and-merge-spec.md)

## A. Genesis to Single-Idea Replay [anchor: a_genesis_to_single_idea_replay]

- Input log: Events from single-idea snapshot fixture
- Replay steps: Apply idea_create, materialize
- Expected state_root_hash matches snapshot fixture

## B. Basic Offline Publication Pack [anchor: b_basic_offline_publication_pack]

- Local events: Two signed ordinary idea_create candidates
- Pack: Signed bundle
- Merge: Publication order ingestion
- Expected post-merge roots

## C. Duplicate Event Rejection [anchor: c_duplicate_event_rejection]

- Pack with duplicate idea_create
- Expected: Reject duplicate, state unchanged

## D. Invalid Signature Rejection [anchor: d_invalid_signature_rejection]

- Event with invalid signature
- Expected: Full rejection, no state change
- Profile-v0 authorship-signature vectors, including altered payload hash, altered event type, wrong key owner, unknown key, revoked key, and publication-wrapper mutation, are required by `canonical-event-authorship-and-signature-profile-v0.md`.

(Provide 5-6 vectors with input events, pack formats, and expected outcomes/hashes.)
