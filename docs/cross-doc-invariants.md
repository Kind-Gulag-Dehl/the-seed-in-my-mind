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
- statement: only eligible humans may author ordinary canonical events. Profile-v0 `identity_create` is also human-authored: the existing eligible sponsor is the event author, while the applicant's initial-key possession proof does not make the applicant the event author and does not require the applicant to already have an active canonical author key. The narrow Tempo repair lane allows eligible human `tempo_contributor` identities to create only target-bound ordinary `truth_claim` ideas with conditional `tempo_claim` metadata and, where explicitly allowed by the active Tempo profile, Tempo-context evidence ideas and `relative_importance` connections using existing `evidence_for`, `evidence_against`, or `same_as` usages. Narrow restricted verification and key-control lanes may exist for a `CanonicalAdmittedIdentity` before ordinary writer eligibility only where exact event-family schemas authorize them. AI may analyze and propose, but cannot vote, govern, create canonical Tempo claims or evidence, sponsor admission, mint invitation capacity, or directly create canonical events without explicit human adoption.
- canonical home: `protocol v5.md`, `ai-boundaries-spec.md`
- scope: all documents
- enforcement: node validation + conformance tests

### 1.1A canonical event authorship signatures
- statement: ordinary human-authored canonical events use the two-layer event model. The human signs a Profile-v0 authored event candidate with `signature_profile = ed25519_v0`; Profile v0 uses Ed25519 only. The signed bytes include the candidate fields defined by `canonical-event-authorship-and-signature-profile-v0.md` and exclude publication-derived fields such as `event_index`, block height, finalized-prefix-certificate data, canonical publication position, local receipt time, database identifiers, and private account/session data. The finalized publication certificate binds the exact signed candidate bytes to canonical order. `public_key_ref` is the hash32 of the versioned key descriptor and resolves through replay-derived identity key state. Key revocation is non-retroactive.
- canonical home: `canonical-event-authorship-and-signature-profile-v0.md`
- scope: protocol root, Appendix A, encoding, publication, replay, node/conformance, verification, offline, privacy, implementation status, conformance fixtures
- enforcement: signature-profile conformance vectors, event-envelope validation, publication-wrapper validation, replay-derived key-state validation

### 1.1B Profile-v0 identity admission
- statement: Profile-v0 identity admission follows permissionless local identity and key preparation, a portable non-canonical admission request, sponsor-authored canonical `identity_create`, and a resulting `CanonicalAdmittedIdentity` with restricted initial authority. Admission alone establishes canonical existence, human target kind, accepted initial key, sponsor/admission provenance, and complete `identity_structural_roots`; it does not establish VH, VI, human uniqueness, civil identity, trustworthiness, ordinary writing, ordinary challenges, voting, governance, Tempo eligibility, inviter eligibility, invitation capacity, or economic authority. Sponsorship, lineage, Anthill membership or degree, structural-root membership, invitation spending, and `verification_reference` are not verification.
- canonical home: `protocol v5.md`, `identity-admission-and-invitation-capacity-spec-v0.md`
- scope: protocol root, Appendix A, authorship/signature, encoding, verification, replay, cycle, snapshot, API contracts, node/conformance, privacy, offline, implementation status, conformance fixtures
- enforcement: event-family validation, admission conformance vectors, replay-derived eligibility checks, and cross-document terminology checks

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

### 4.3 one public SeedPackage vocabulary
- statement: public portable artifacts use the existing Pocket Map, Citizen Map, Civic Archive, Full Archive, Playable Offline Bundle, Full Recovery Bundle, and Archive Shard Set profiles. Documents must not introduce a parallel Pocket/Living/Ark seed taxonomy or conflate private Mindseed journals with public recovery packages.
- canonical home: `collective-seedpackage-and-recovery-profile-v0.md`
- scope: bundles, preservation, offline, nodes, clients, documentation
- enforcement: profile-id and terminology fixtures

### 4.4 higher-tier identity is explicit
- statement: `shared_map_commitment` proves canonical facts plus Tier 0 meaning only. Tier 1-3, playable, recovery, and shard claims require the applicable SeedPackage manifest hash and pack commitments.
- canonical home: `canonical-encoding-and-hashing-spec.md`, `collective-seedpackage-and-recovery-profile-v0.md`
- scope: bundle publication, custody manifests, APIs, verification, UI
- enforcement: manifest and coverage-claim fixtures

### 4.5 universal importance remains primary
- statement: Citizen and Civic selection uses replay-derived universal importance plus deterministic explanatory closure. Minority, novelty, language, or random-storage status cannot change universal rank or mandatory core-tier membership; delta, PCS/CCS, archive, and optional preservation policies provide separate redundancy.
- canonical home: `shared-map-and-payload-bundles-spec.md`, `collective-seedpackage-and-recovery-profile-v0.md`
- scope: ranks, bundles, custody, clients
- enforcement: deterministic selection and rank-noninterference fixtures

### 4.6 reconstruction claims match retained bytes
- statement: a partial dataset can verify only its declared closure. Complete historical reconstruction requires a valid Full Archive, Full Recovery Bundle, or sufficient verified Archive Shards; hashes do not recreate universally lost bytes.
- canonical home: `collective-seedpackage-and-recovery-profile-v0.md`
- scope: protocol claims, packages, nodes, recovery guides, UI
- enforcement: incomplete-closure rejection fixtures

### 4.7 catastrophe recovery never rewrites continuity
- statement: ordinary quorum loss freezes the original prefix. Continued operation requires an explicitly linked, separately identified catastrophe successor; ordinary thresholds are never silently lowered, competing declarations remain preserved, and later recognition or bridging is forward-only.
- canonical home: `pod-consensus-and-canonical-publication-spec.md`
- scope: publication, replay, snapshots, packages, nodes, APIs, clients
- enforcement: lineage, certificate-root, labeling, and competing-declaration fixtures

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
- statement: deliberative close requires Dmin structural readiness and `W_score >= W_target`; forced close requires either Dmax structural readiness with unmet work target or `structural_dmax_liveness_predicate` true with unmet work target. Ordinary Dmax structural readiness mechanically implies structural Dmin for the same anchor/profile; `structural_dmax_liveness_predicate` does not. The survivor predicate is Dmax-only, forced-closure-only, not ordinary certainty, not beacon certainty, not certification, and not authorization. Forced boundaries remain forced forever, never accumulate legitimacy, and never grant authority. Dmax alone, forced boundaries, degraded boundaries, survivor boundaries, record-only boundaries, wall-clock passage, cron activity, AI activity, system-emitter activity, and machine-only boundaries do not generate invitation capacity, inviter eligibility, inviter maturation, suspension restoration, carryover-cap increases, or admission rewards unless they independently satisfy the human-deliberative certification rules for a qualifying capacity period. `W_target` may adapt downward in nonzero-human constrained operation, but `K`, `T_beacon`, beacon diversity, independence, and stability requirements do not automatically shrink during population collapse. Zero eligible humans produce record-only posture and no universal `cycle_close`.
- canonical home: `protocol v5.md`, `cycle-spec.md`, `tempo-spec.md`
- scope: protocol, cycle, tempo, governance, token, POD, POINT, lifecycle, admission, replay/merge
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
- statement: importance duplicate uniqueness uses the complete rank context plus ordered challenger/target tuple. A universal key includes `(domain=importance, rank_kind=universal, universal_orientation, timeframe, scope=universal, challenger, target)`. A relative key includes `(domain=importance, rank_kind=relative, reference_idea_id, usage=general, relative_axis, timeframe, scope, scope_anchor_id_if_any, challenger, target)`. Identical active keys are forbidden, while the same candidates in distinct contexts may coexist.
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
- statement: canonical tribe and globally eligible public relative contexts are overlays over the one canonical substrate. `scope = universal` in a relative context names the globally eligible public participation scope; it does not by itself identify the distinct universal-importance rank kind. Personal/private rank state is noncanonical owner-controlled state, not a canonical one-person challenge overlay. A shared personal display remains a projection unless an explicit future canonical adoption transition says otherwise.
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
- statement: canonical substrate state is always publicly readable. Ordinary canonical writes require ordinary canonical-writer verification requirements (current deployment compatibility: Seed verifier-issued `canonical_writer_level`). Profile-v0 identity admission is sponsor-authored and does not give the target identity ordinary writer eligibility. A `CanonicalAdmittedIdentity` may use only the exact restricted verification and key-control lanes later defined by Appendix A and the Verification Specification. The only low-threshold Tempo exception is the narrow human Tempo repair lane for target-bound ordinary `truth_claim` ideas with conditional `tempo_claim` metadata and, if explicitly permitted, Tempo-context evidence ideas and evidence/same_as connections by eligible `tempo_contributor` identities. All canonical claims remain publicly challengeable, and challenge creation/voting use ordinary challenge eligibility unless a future explicit Tempo-only challenge capability is adopted; Tempo contributor status alone is insufficient.
- canonical home: `protocol v5.md` (`canonical_read_write_access_policy`)
- scope: protocol, API contracts, challenge spec, verification spec, roadmap/planning docs
- enforcement: API access tests (public canonical GET; auth-gated canonical POST), replay validation of challenge/vote author eligibility

### 7.6 invitation capacity and admission liveness
- statement: inviter eligibility is an orthogonal replay-derived lane and must be generally attainable by humans satisfying the same objective rules; no permanent founder, operator, institution, expert, or genesis inviter class exists. Every inviter-eligible, unsuspended human receives at least one spendable invitation-capacity unit in each qualifying capacity period. Invitation capacity is replay-derived, integer-valued, identity-bound, non-transferable, non-saleable, non-delegable in Profile v0, bounded, and not money, a token, reputation, verification certainty, truth weight, importance weight, or vote weight. Exact Profile-v0 invitation capacity is deterministically derivable from public canonical history and rulebooks. When no qualifying capacity period occurs, replay exposes `admission_liveness_blocked = true`; Profile v0 permits no emergency capacity minting by operators, AI, system emitters, wall-clock processes, or machine-only cycles.
- canonical home: `protocol v5.md`, `identity-admission-and-invitation-capacity-spec-v0.md`
- scope: protocol, admission, verification, cycle, replay/merge, snapshots, node/conformance, API contracts, privacy, offline, implementation status
- enforcement: replay-derived admission-state checks, capacity-debit tests, cycle-liveness conformance vectors, and public-read consistency checks

### 7.7 universal and relative importance are distinct rank products
- statement: universal importance is the public 20-axis rank product formed by four universal orientations across five timeframes. Relative importance is the 10-axis product formed by `important_to_reference` and `important_for_reference` across the same five timeframes for a declared reference idea. They reuse pairwise challenge and bubble-up semantics but MUST remain distinct in schemas, snapshots, replay keys, APIs, and labels.
- canonical home: `protocol v5.md` (`universal_importance_and_the_20_axis_foundation`, `scoped_importance_universal_tribal_and_personal_contexts`), `protocol v5-appendix-a.md` (`a2_5_1_required_fields`, `a2_7_rank_snapshots_importance_state_objects`)
- scope: protocol, appendices, challenge, tribe, token, replay/merge, snapshots, conformance, API and UI docs
- enforcement: rank-kind discriminators, axis-vocabulary validation, cross-document fixture assertions

### 7.8 universal aggregate is derived from pairwise axis positions
- statement: every universal axis is an ordinal list produced by pairwise verdicts. For each complete idea profile, `universal_position_sum` is the exact integer sum of the twenty one-based axis positions and the exact mean is that sum divided by twenty. `overall_universal_rank` is derived by ascending sum plus the active deterministic tie-break. The aggregate is not directly challenged and never decides an axis verdict.
- canonical home: `protocol v5.md` (`universal_importance_and_the_20_axis_foundation`, `derived_universal_ranks_and_rank_history`)
- scope: protocol, token, replay/merge, snapshots, node/conformance, bundles, preservation, API and UI docs
- enforcement: exact-integer aggregation fixtures, tie-break fixtures, rejection of authored aggregate mutations

### 7.9 canonical importance context and bubble-up
- statement: a universal challenge is keyed by universal orientation, timeframe, and public scope. A relative challenge is keyed by reference, usage, relative axis, timeframe, and public or tribe scope. The reference or universal pole defines the comparison and is not a contestant. A valid challenger begins below the target; on a winning verdict it moves immediately above the target while all other ideas preserve relative order. A loss or no-longer-below challenger produces no movement under the base rule.
- canonical home: `protocol v5.md` (`importance_challenges`), `challenge-engine-spec.md` (`importance_challenges`, `importance_challenge_effects`)
- scope: protocol, appendices, challenge, replay/merge, node/conformance, API
- enforcement: framing validation, duplicate-key validation, replay/conformance bubble-up fixtures

### 7.10 public, tribe, and individual decision procedures
- statement: universal-axis and public-relative challenges draw jurors from the eligible public. Tribe-relative challenges draw jurors only from eligible members of the tribe anchored by the reference idea, while results remain public. Individual-private relative rankings are selected directly by the owner without canonical challenges or votes, remain permissioned/noncanonical, and include no private universal rank. "Voted on by the public/tribe" names the eligible electorate; the deterministic small-panel vote-session procedure remains controlling.
- canonical home: `protocol v5.md` (`scoped_importance_universal_tribal_and_personal_contexts`, `importance_challenges`, `voter_eligibility_juror_selection_and_rate_limited_vote_sessions`), `tribe-spec.md`
- scope: protocol, challenge, tribe, offline/private integration, AI/product docs
- enforcement: electorate eligibility fixtures, private-event rejection, privacy-boundary tests

### 7.11 importance arguments are explanatory ordinary ideas
- statement: importance arguments are ordinary ideas attached to either candidate through `relative_importance` connections with `usage = importance_argument`. `context_challenge_id` makes an argument challenge-specific; absence makes it reusable. An importance-argument edge does not enter the `usage = general` candidate list and cannot move rank without a valid verdict.
- canonical home: `protocol v5.md` (`importance_challenges`), `protocol v5-appendix-a.md` (`a4_5_1_argument_as_idea_mandatory_model`)
- scope: protocol, appendices, challenge, replay/merge, API and UI docs
- enforcement: connection-usage validation and no-effect fixtures

### 7.12 tokens and AI cannot author canonical importance
- statement: POD/POINT formulas may consume replay-derived importance state but never create ranks, weight ballots, or choose verdicts. AI may draft, analyze, or simulate rankings but cannot author canonical challenges, votes, verdicts, or rank mutations under the human-first protocol. Private or simulated AI/robot maps have no private universal rank.
- canonical home: `protocol v5.md`, `token-spec.md`, `ai-boundaries-spec.md`
- scope: protocol, token, AI, private integration, conformance, API and UI docs
- enforcement: token-separation assertions, human-authorship validation, AI-event rejection fixtures

---

## 8. conflict log (fill as discovered)

When a conflict is found, add an entry here:

- conflict_id: <short stable id>
  statement: <what conflicts>
  docs: [<file1>, <file2>, ...]
  winner: <file>
  resolution: <what changed / what should change>
  date: <yyyy-mm-dd>
