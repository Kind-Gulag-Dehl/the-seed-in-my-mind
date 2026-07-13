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
- statement: only eligible humans may author ordinary canonical events. The narrow Tempo repair lane allows eligible human `tempo_contributor` identities to create only target-bound ordinary `truth_claim` ideas with conditional `tempo_claim` metadata and, where explicitly allowed by the active Tempo profile, Tempo-context evidence ideas and `relative_importance` connections using existing `evidence_for`, `evidence_against`, or `same_as` usages. AI may analyze and propose, but cannot vote, govern, create canonical Tempo claims or evidence, or directly create canonical events without explicit human adoption.
- canonical home: `protocol v5.md`, `ai-boundaries-spec.md`
- scope: all documents
- enforcement: node validation + conformance tests

### 1.1A canonical event authorship signatures
- statement: ordinary human-authored canonical events use the two-layer event model. The human signs a Profile-v0 authored event candidate with `signature_profile = ed25519_v0`; Profile v0 uses Ed25519 only. The signed bytes include the candidate fields defined by `canonical-event-authorship-and-signature-profile-v0.md` and exclude publication-derived fields such as `event_index`, block height, finalized-prefix-certificate data, canonical publication position, local receipt time, database identifiers, and private account/session data. The finalized publication certificate binds the exact signed candidate bytes to canonical order. `public_key_ref` is the hash32 of the versioned key descriptor and resolves through replay-derived identity key state. Key revocation is non-retroactive.
- canonical home: `canonical-event-authorship-and-signature-profile-v0.md`
- scope: protocol root, Appendix A, encoding, publication, replay, node/conformance, verification, offline, privacy, implementation status, conformance fixtures
- enforcement: signature-profile conformance vectors, event-envelope validation, publication-wrapper validation, replay-derived key-state validation

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

### 5.0 idea-only deliberative content
- statement: all canonical deliberative content is expressed as identity-authored ideas using existing base idea types. Evidence, arguments, attestations, observations, testimony, source statements, potential evidence, and source descriptions are roles or uses of ideas, not separate canonical content-object types. Canonical relationships use existing connection types and usages, and canonical resolution uses the unified challenge, vote, verdict, and cycle processes. No document may introduce top-level `evidence`, `attestation`, `testimony`, `source`, `time_claim`, `tempo_target`, or `beacon` idea types.
- canonical home: `protocol v5.md`, `protocol v5-appendix-a.md`, `challenge-engine-spec.md`
- scope: all documents
- enforcement: schema validation, conformance fixtures, and search checks for forbidden content-object types

### 5.1 cycles gate pacing, not semantics
- statement: Protocol v5 owns root cycle invariants and sealing semantics; `cycle-spec.md` provides the detailed subordinate normative algorithm. Cycle boundaries gate pacing and structural replay, not truth semantics or consequential authority by themselves.
- canonical home: `protocol v5.md`, `cycle-spec.md`
- scope: `tempo-spec.md`, `challenge-engine-spec.md`, `governance-spec.md`, `token-spec.md`, `offline-and-mindseed-spec.md`
- enforcement: conformance tests

### 5.2 tempo, constrained mode, and time repair
- statement: Tempo owns target-bound ordinary truth-claim metadata, Tempo-context evidence rules, ordinary certainty-band interpretation for time truth claims, `T_allow` structural-support derivation for Dmin/Dmax predicates, the Dmax-only `structural_dmax_liveness_predicate`, derived beacons, and modes. Tempo does not create a separate evidence, attestation, challenge, or truth-certainty system. `T_allow` is structural support, not ordinary truth certainty. `T_allow` predicates may be consumed same-cycle only for structural boundary evaluation; `structural_dmax_liveness_predicate` may be consumed only for forced Dmax structural closure. Consequential authority requires certification and the lagged authorization frontier.
- canonical home: `tempo-spec.md`
- scope: `cycle-spec.md`, `node-and-conformance-spec.md`, `governance-spec.md`, offline specs
- enforcement: node/client behavior requirements

### 5.3 automatic cycle_close boundary emission
- statement: `cycle_close` is emitted automatically by the system boundary emitter when structural closure predicates are satisfied; it is not human/operator-submitted and is not proof of consequential authority.
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

### 5.7A lagged authorization frontier
- statement: consequential authority advances only through a contiguous, monotonic authorization frontier derived from certified cycles and governance lag `K`. Certification gaps stop advancement; revocation blocks future advancement without rewriting authorized history; genesis starts with `initial_authorization_frontier = -1` unless immutable independently verifiable genesis data defines a safe bootstrap basis.
- canonical home: `protocol v5.md`, `cycle-spec.md`
- scope: protocol, cycle, tempo, governance, token, POD, POINT, lifecycle, replay/merge, node/conformance
- enforcement: replay of certification status, frontier state, deferred-output finalization, and rejection of forbidden retroactive effects

### 5.7B forced cycles and anti-collapse
- statement: deliberative close requires Dmin structural readiness and `W_score >= W_target`; forced close requires either Dmax structural readiness with unmet work target or `structural_dmax_liveness_predicate` true with unmet work target. Ordinary Dmax structural readiness mechanically implies structural Dmin for the same anchor/profile; `structural_dmax_liveness_predicate` does not. The survivor predicate is Dmax-only, forced-closure-only, not ordinary certainty, not beacon certainty, not certification, and not authorization. Forced boundaries remain forced forever, never accumulate legitimacy, and never grant authority. `W_target` may adapt downward in nonzero-human constrained operation, but `K`, `T_beacon`, beacon diversity, independence, and stability requirements do not automatically shrink during population collapse. Zero eligible humans produce record-only posture and no universal `cycle_close`.
- canonical home: `protocol v5.md`, `cycle-spec.md`, `tempo-spec.md`
- scope: protocol, cycle, tempo, governance, token, POD, POINT, lifecycle, replay/merge
- enforcement: cycle-close classification tests, anti-collapse conformance tests, deferred-output replay checks

### 5.7C Tempo targets, claims, evidence, and equality
- statement: Dmin/Dmax targets are derived deterministic target keys, not authored canonical objects, events, ideas, or connection types. Humans author target-bound ordinary `truth_claim` ideas using the existing truth-subtype enum and conditional `tempo_claim` metadata. Tempo evidence and attestations are ordinary identity-authored ideas, usually `truth_claim` ideas, connected through existing `relative_importance` usages such as `evidence_for` or `evidence_against`; they are not separate events or content-object types. Verification may gate eligibility or diversity, but must not weight truth, challenge, governance, or Tempo influence.
- canonical home: `tempo-spec.md`, `protocol v5.md`, `verification-spec.md`
- scope: protocol, appendices, tempo, verification, challenge, API contracts, node/conformance
- enforcement: schema validation, replay aggregation, rejection of AI/non-human Tempo events and weighted-identity formulas

### 5.7C.1 Tempo evidence connection validity
- statement: Tempo-context `evidence_for`, `evidence_against`, and `same_as` connections use existing connection types and must reference valid identity-authored ideas in the relevant Tempo context. Invalid endpoints, incompatible target-bound time-claim metadata, direct external URL/hash/payload evidence, derived target/beacon endpoints, or attempts to create certainty outside ordinary evidence-placement and certainty-band challenge flow reject with `ERR_TEMPO_EVIDENCE_CONNECTION_INVALID`.
- canonical home: `protocol v5-appendix-a.md`, `tempo-spec.md`
- scope: tempo, challenge, replay/merge, node/conformance, fixtures
- enforcement: schema validation and conformance fixtures

### 5.7D canonical evidence only
- statement: Tempo truth certainty may use evidence only when represented by identity-authored canonical evidence ideas, explicit allowed connections, and challenge verdicts with inspectable provenance and reproducible replay inputs. Tempo structural support may additionally use profile-admitted passive machine timestamp evidence only when the source is canonically committed or canonically anchored, normalized deterministically, deduplicated, outlier-handled by profile rule, and capped below `T_allow`. Node-local time, server time, client timestamps, receipt time, background schedulers, block height, publication volume, AI-generated observations, external links alone, or uncommitted observations never affect Tempo truth certainty or structural support except through the explicit passive-evidence channel and valid canonical ideas/connections.
- canonical home: `tempo-spec.md`, `protocol v5.md`
- scope: protocol, tempo, replay/merge, node/conformance, API contracts
- enforcement: replay input validation and rejection of hidden-clock certainty inputs

### 5.7E structural support vs truth certainty
- statement: `T_allow` operates on derived provisional structural support from current eligible-human stances plus capped passive evidence. `T_beacon` operates on ordinary canonical truth-certainty bands plus beacon diversity, challenge-survival, and contradiction requirements. Structural support may close a cycle structurally in the same cycle, but does not assign truth certainty, create a beacon, certify a cycle, advance the frontier, or finalize consequences.
- canonical home: `tempo-spec.md`, `protocol v5.md`, `cycle-spec.md`
- scope: protocol, tempo, cycle, challenge, replay/merge, node/conformance, fixtures
- enforcement: conformance fixtures separating structural readiness from certainty-band verdicts and beacon/certification outputs

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

### 7.3 relative importance axis vocabulary
- statement: relative_importance connection metadata uses relative axes (`important_to_reference`, `important_for_reference`) crossed with the five protocol timeframes. Universal orientation values (`important_to_current_individual`, `important_for_current_individual`, `important_to_collective`, `important_for_collective`) belong to universal importance profiles and MUST NOT be used as ordinary relative_importance axis values unless a future rulebook explicitly defines a deterministic projection.
- canonical home: `protocol v5.md` (`scoped_importance_universal_tribal_and_personal_contexts`), `protocol v5-appendix-a.md` (`a2_4_2_conditional_metadata_by_connection_type`)
- scope: protocol, appendices, challenge, tribe, governance, token, replay/merge, node/conformance, api docs
- enforcement: schema validation + challenge framing validation + rank snapshot validation

### 7.4 no mirroring-as-copy
- statement: mirroring-as-copy semantics (copying tribe-local ideas into separate public canonical objects) are forbidden. Cross-scope publication semantics must be represented by canonical substrate + overlays, not object duplication.
- canonical home: `protocol v5.md` (`canonical_substrate_and_scoped_overlays`), `tribe-spec.md`
- scope: protocol, tribe, replay/merge, token, api docs
- enforcement: invariant validation + deprecation checks in conformance suite

### 7.5 canonical read/write access policy
- statement: canonical substrate state is always publicly readable. Ordinary canonical writes require ordinary canonical-writer verification requirements (current deployment: Seed verifier-issued `canonical_writer_level`). The only low-threshold exception is the narrow human Tempo repair lane for target-bound ordinary `truth_claim` ideas with conditional `tempo_claim` metadata and, if explicitly permitted, Tempo-context evidence ideas and evidence/same_as connections by eligible `tempo_contributor` identities. All canonical claims remain publicly challengeable, and challenge creation/voting use ordinary challenge eligibility unless a future explicit Tempo-only challenge capability is adopted; Tempo contributor status alone is insufficient.
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
