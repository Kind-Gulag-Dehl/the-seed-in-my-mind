---
doc_id: challenge_engine_spec
title: Challenge Engine Specification
status: authoritative
version: v0
last_reviewed: 2026-06-22

scope:
  - Defines the lifecycle, voting, and resolution of challenges.

authoritative_for:
  - Challenge creation, argument phases, voting windows, and resolution.
  - Canonical state transformations resulting from challenge outcomes.

not_authoritative_for:
  - Cycle derivation rules (see cycle-spec.md).
  - Tempo enforcement mechanics (see tempo-spec.md).

depends_on:
  - protocol v5.md
  - cycle-spec.md
  - tempo-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of governance-spec.md and token-spec.md.

reader_path:
  - prereq: tempo-spec.md
  - next: governance-spec.md

keywords:
  - challenges
  - voting
  - deliberation
  - resolution
---

> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

# Challenge Engine Specification

*(Normative — subordinate to Protocol v5; defines the atomic deliberation and state-transformation mechanism used for truth, importance, action, and representation.)*

---

## 0. Purpose, scope, and authority [anchor: 0_purpose_scope_and_authority]

### 0.1 Purpose [anchor: purpose]

This specification defines the **challenge engine**: the single canonical mechanism by which contested claims, comparisons, proposals, and representations are evaluated and resolved within Protocol v5.

Its purpose is to:

- make all consequential determinations **human-authored, challengeable, and auditable**,
- ensure that every canonical state change resulting from disagreement is **deterministic and replayable**,
- prevent parallel or ad-hoc resolution mechanisms from fragmenting the canonical universe,
- define a uniform lifecycle and outcome model that applies across truth, importance, action, and representation.

The challenge engine is the protocol’s **deliberative core**. All meaningful resolution flows through it.

---

### 0.2 Scope [anchor: scope]

This specification governs:

- the definition of a challenge as a canonical object,
- the permitted challenge domains and their semantics,
- challenge framing and anchoring requirements,
- the deterministic lifecycle state machine,
- argument and evidence handling using existing protocol primitives,
- voter eligibility computation and selection,
- vote casting and tally rules,
- verdict finalization,
- and the mapping from verdicts to canonical state transformations.

This specification does **not**:

- introduce new idea types, connection types, or authorities,
- redefine token issuance, POD/POINT economics, or governance powers,
- specify UI, UX, or application-layer workflows,
- define safety visibility rules (see Safety Specification),
- define deterministic replay algorithms (see Deterministic Replay & Merge Specification).

---

### 0.3 Authority and conflict resolution [anchor: authority_and_conflict_resolution]

This specification is **subordinate to Protocol v5**.

In the event of conflict or ambiguity:

1. Protocol v5 Section 0 invariants are authoritative.
2. Appendix A canonical schemas and event rules prevail over informal descriptions.
3. Deterministic replay requirements override convenience or interpretation.
4. No behavior may be inferred from omission or convention.

This specification introduces **no new canonical powers**. It constrains how existing protocol primitives may be composed to resolve disagreement.

Canonical access policy for challengeability and write eligibility is defined in `protocol v5.md` (`canonical_read_write_access_policy`).

---

## 1. Challenge as the sole resolution primitive [anchor: 1_challenge_as_the_sole_resolution_primitive]

### 1.1 Definition [anchor: definition]

A challenge is a structured, bounded deliberation process in which verified human identities evaluate a clearly framed question and produce a canonical verdict.

A challenge exists to resolve disagreement. It does not create truth, importance, or legitimacy by fiat; it records the best available collective judgment at a point in the canonical history, subject to future challenge.

A challenge is always:

explicitly anchored to existing canonical objects,

authored by a verified human identity,

cycle-bounded using canonical cycle indices derived from the event log (Protocol v5 §3),

finalized by an explicit verdict event,

and permanently recorded in the canonical event log.

### 1.2 Challenge domains [anchor: challenge_domains]

Protocol v5 defines **exactly four challenge domains**:

1. **Truth Challenge**  
   Evaluates whether a truth-claim idea holds, fails, or remains uncertain under available evidence and arguments.

2. **Importance Challenge**  
   Evaluates the relative importance between two ideas under a specified axis, timeframe, and scope.

3. **Action Challenge**  
   Evaluates whether an action or actionable idea should proceed, be recognized, or be accepted under defined criteria.

4. **Representation Challenge**  
   Evaluates which description or representation of a target object (`idea` or `ordering`) should be canonical.

No additional challenge domains may be introduced by rulebooks, implementations, or applications.

Each challenge domain has domain-specific verdict semantics, but **shares the same lifecycle, eligibility model, and finalization rules**.

**Duplicate challenge restriction**  
Conformant nodes MUST reject any attempt to create a new challenge that duplicates an existing open or recently closed challenge with identical domain and anchors. This rule (defined in Appendix A §A7.2.3) prevents vote-splitting, challenge spam, and ambiguity while preserving the ability to re-challenge resolved issues after a completed lifecycle or significant new evidence.

Resurrection is not a challenge domain and does not require a challenge.

Resurrection is a canonical creation-like action that restores a burned idea or burned relative_importance connection to active participation in the living map. Resurrection actions are subject to the same post-hoc challengeability as all other canonical actions.



### 1.3 Exclusivity of canonical resolution [anchor: exclusivity_of_canonical_resolution]

All canonical changes to the following MUST occur **only** through finalized challenge verdicts:

- truth status or certainty of truth claims,
- relative importance relations used for ranking,
- acceptance or recognition of actions,
- selection or replacement of canonical descriptions or representations.

Direct modification of canonical state outside a challenge verdict is prohibited.

Events that attempt to bypass this rule MUST be rejected during deterministic replay.

---

### 1.4 Challenges as transformations, not authorities [anchor: challenges_as_transformations_not_authorities]

A challenge does not grant authority, legitimacy, or permanence.

A challenge verdict:

- records a state transformation at a specific point in canonical history,
- is always subject to later challenge and revision,
- does not prevent future evidence, arguments, or reinterpretation.

The protocol treats challenge outcomes as **current best judgments**, not final truths.

---

### 1.5 Human-first deliberation invariant [anchor: human_first_deliberation_invariant]

Only verified human identities may:

- create challenges,
- submit arguments or evidence into a challenge,
- vote in challenge ballots,
- finalize challenge verdicts.

AI systems MAY assist humans in drafting, analysis, or simulation, but MUST NOT author, vote, finalize, or otherwise directly affect canonical challenge outcomes.

This invariant is absolute and SHALL NOT be bypassed.

Because challenge operations are canonical writes, challenge creation and voting use the same canonical-writer eligibility gate as other canonical write events.
All canonical claims remain publicly challengeable even when write eligibility is gated.

The narrow Tempo repair lane is not a challenge lane. `tempo_contributor` status alone does not authorize challenge creation, argument submission, voting, or verdict finalization. Time-related truth challenges remain ordinary truth challenges and require ordinary challenge eligibility unless a later explicit rulebook creates a separate `tempo_challenger` capability.

Tempo evidence and attestations are ideas in roles, usually `truth_claim` ideas, connected through existing `relative_importance` usages such as `evidence_for` and `evidence_against`. They are not challenge votes, not separate non-idea records, and not separate content-object types. Verification level, identity certainty, provider, institution, jurisdiction, wealth, POD, POINT, reputation, model, or role MUST NOT multiply truth, challenge, governance, or Tempo influence.

Tempo-context evidence placement uses the ordinary evidence connection model. A placement connection that claims Tempo relevance but fails Tempo target, claim, `same_as`, provenance, or schema validation is rejected under Appendix A with `ERR_TEMPO_EVIDENCE_CONNECTION_INVALID`; it must not enter an evidence-placement challenge as valid support or opposition.

The Tempo `structural_dmax_liveness_predicate` is not a challenge verdict and does not assign certainty. It may permit only a Dmax forced structural boundary under the Tempo and Cycle specifications. Any contradictory target-bound claim or unresolved blocking truth challenge blocks that liveness path until resolved by ordinary challenge process unless a rulebook explicitly defines a deterministic non-authoritative tie behavior.

---

### 1.6 Uniformity across the protocol [anchor: uniformity_across_the_protocol]

The challenge engine is intentionally universal.

Truth disputes, importance disputes, action decisions, governance questions, safety appeals, and representation conflicts all reduce to the same core mechanism:

> **a framed question → arguments → eligible voters → deterministic tally → explicit verdict → canonical transformation**

No parallel or “lighter” resolution paths are permitted for canonical state.

This uniformity is a core anti-capture and anti-ambiguity property of the system.

---
## 2. Challenge object and framing requirements [anchor: 2_challenge_object_and_framing_requirements]

### 2.1 Challenge object definition [anchor: challenge_object_definition]

A **challenge** is a canonical object created by a `challenge_create` event (as defined in Appendix A).  
It binds a framed question to specific canonical objects and declares the parameters under which deliberation occurs.

A challenge MUST include:

- a unique `challenge_id`,
- a `challenge_domain` (truth, importance, action, representation),
- one or more **subject anchors** (idea IDs and/or connection IDs, as permitted by domain),
- a `created_by_identity_id` (verified human),
- deterministic lifecycle parameters (defined in Section 3),
- references to the applicable rulebook set.

A challenge MUST be immutable after creation except for lifecycle state transitions and the final verdict reference.



### 2.2 Framing invariants [anchor: framing_invariants]

Every challenge MUST be framed such that:

1. **The question is precise**  
   The challenge MUST pose a question whose possible outcomes are enumerable and whose effects on canonical state are fully defined by the protocol and active rulebooks.

2. **The scope is explicit**  
   The challenge MUST explicitly declare:
   - affected ideas or connections,
   - relevant axes, timeframes, and scopes (for importance challenges),
   - or affected representations (for representation challenges), including an optional representation `scope_key = (scope_kind, anchor_id)` when the contested effect is scoped display behavior.

3. **The transformation is defined**  
   For each possible verdict outcome, the resulting canonical state transformation MUST be deterministic and specified in advance.

Challenges that fail to meet these requirements MUST be rejected at creation time.

---

### 2.3 Subject anchoring by domain [anchor: subject_anchoring_by_domain]

Each challenge domain imposes strict anchoring rules:

#### 2.3.1 Truth challenges [anchor: truth_challenges]
- MUST reference exactly one `truth_claim` idea as the primary subject.
- MAY reference additional ideas as contextual anchors.
- MUST NOT introduce new truth claims implicitly.
- MAY challenge a target-bound Tempo time claim, but the challenge remains an ordinary truth challenge.
- MUST NOT treat Tempo-context evidence ideas or attestations as votes or as a substitute for challenge arguments, ballots, or verdicts.

#### 2.3.2 Importance challenges [anchor: importance_challenges]
- MUST reference exactly two ideas whose relative importance is being evaluated.
- MUST declare `rank_kind`, timeframe, participation scope, challenger, and target.
- The two subject ideas are candidate contestants. The reference idea or universal pole defines the question and MUST NOT be encoded as a third contestant.
- `challenger_idea_id` MUST currently be below `target_idea_id` in the complete declared context.
- For `rank_kind = universal`, `scope` MUST be `universal` and `universal_orientation` MUST be one of `important_to_current_individual`, `important_for_current_individual`, `important_to_collective`, or `important_for_collective`.
- For `rank_kind = relative`, `reference_idea_id` and `usage = general` are REQUIRED, and `axis` MUST be either `important_to_reference` or `important_for_reference`.
- For relative `scope = universal`, the eligible public is the potential electorate. For relative `scope = tribe`, `scope_anchor_id` MUST equal the tribe `reference_idea_id` and only eligible tribe members form the potential electorate.
- `timeframe` MUST be one of `near_term`, `mid_term`, `long_term`, `very_long_term`, or `trans_generational`.
- Canonical importance challenges MUST NOT use personal/private scope. An individual directly orders private relative rank state outside this engine.
- MUST NOT attempt to set absolute importance values.

Universal importance and reference-relative importance are distinct rank products that reuse this challenge lifecycle. A public relative connection to an idea named "humanity", "the individual", or "the collective" does not become universal importance by title or reference alone.

Examples:

- **Universal axis:** if lower-ranked idea B challenges idea A in `(rank_kind = universal, universal_orientation = important_for_collective, timeframe = long_term)`, eligible public jurors compare B with A. If B wins, B moves immediately above A in that one axis list. The other nineteen universal axes do not move; the exact overall universal aggregate is then recomputed.
- **Public relative:** if B challenges A in `(rank_kind = relative, reference_idea_id = R, axis = important_for_reference, timeframe = long_term, scope = universal)`, R frames what "important for" means but is not a contestant. Eligible public jurors decide between B and A.
- **Tribe relative:** the same relative challenge with `scope = tribe` uses R as both reference and tribe anchor. Only eligible members of R's tribe may vote, while the resulting tribe-relative ordering remains public.
- **Individual private:** an owner may place B above A in the equivalent ten-axis private context directly. That edit creates no challenge, ballot, canonical verdict, or universal rank.

#### 2.3.3 Action challenges [anchor: action_challenges]
- MUST reference exactly one `action` or `actionable_idea`.
- MUST declare the decision being evaluated (e.g., recognition, authorization, acceptance).

#### 2.3.4 Representation challenges [anchor: representation_challenges]
- MUST reference exactly one target object whose representation is contested (`idea` or `ordering`).
- MUST reference two or more competing representations already present in canonical history.
- Competing representations MUST be canonical representation objects (created via the Appendix A representation creation path) and not informal/off-log submissions.
- MAY include an optional `scope_key = (scope_kind, anchor_id)` in challenge framing.
- If `scope_key` is omitted, the challenge is interpreted as a universal-pointer representation challenge.
- If `scope_key` is present, the challenge is interpreted as a scoped-display-override representation challenge.
- Representation challenges remain in the representation domain in all cases; no new challenge domain is introduced.

---

### 2.4 Prohibited framing patterns [anchor: prohibited_framing_patterns]

The following framing patterns are forbidden:

- multi-question challenges,
- challenges that implicitly create or delete ideas,
- challenges that bundle unrelated decisions,
- challenges whose outcomes depend on future, unspecified events,
Such challenges MUST be rejected deterministically.

Such challenges MUST be rejected deterministically.

---

### 2.5 Relationship to arguments and evidence [anchor: relationship_to_arguments_and_evidence]

Challenges do not contain arguments or evidence internally.

Arguments and evidence are:

- ordinary ideas authored independently,
- connected to the challenge or its subject using `relative_importance` connections with rulebook-defined usage values,
- never embedded or duplicated inside the challenge object.

This separation preserves immutability and replay determinism.

For importance challenges, an argument MAY support either candidate. It is an ordinary idea attached with a `relative_importance` connection whose `usage = importance_argument`. A present `context_challenge_id` scopes it to this challenge; absence makes it a reusable general argument. The argument edge is explanatory only: it MUST NOT enter the candidates' `usage = general` rank list or move rank without a verdict.

---

### 2.6 Uniqueness of concurrent challenges (no duplicate instances) [anchor: uniqueness_of_concurrent_challenges_no_duplicate_instances]

For any given **challenge instance**, at most **one active public challenge** MAY exist at a time.

A *challenge instance* is defined by the tuple:

- `challenge_domain`, and
- the complete set of **subject anchors** required by that domain (including axis, timeframe, and scope where applicable).

Accordingly:

- For an **importance challenge**, a challenge instance is uniquely identified by:
  - `challenge_domain = importance`,
  - a deterministic **context key** (for example: universal context, or relative-importance-under-reference context),
  - and an ordered target tuple (idea A, idea B, plus reference idea identifier when the context requires it).
- For a **truth challenge**, a challenge instance is uniquely identified by:
  - the challenged truth-claim identifier.
- For a **representation challenge**, a challenge instance is uniquely identified by:
  - the target object identifier (`idea_id` or `ordering_id`),
  - the target `(tier_length, tier_complexity)` slot,
  - and the resolved representation scope key.

For representation-challenge uniqueness, scope resolution MUST be deterministic:
- If framing declares `scope_key`, use that value.
- If framing omits `scope_key`, resolve to `(scope_kind = universal, anchor_id = universal_anchor)`, where `universal_anchor` is the deterministic universal-scope constant defined by the active rulebook set.
- For an **action challenge**, a challenge instance is uniquely identified by:
  - the action or actionable-idea identifier.

While a challenge instance is in any **non-finalized lifecycle state** (Created, Open for Arguments, Closed for Arguments, Open for Voting, Closed for Voting):

- no other challenge with the same instance-defining tuple MAY be created,
- attempts to create such a duplicate challenge MUST be rejected deterministically.

Once a challenge instance is **finalized**, any identity MAY create a **new challenge** with the same instance-defining tuple, representing a re-evaluation based on new arguments, evidence, or changed context.

---

### 2.7 Parallel challenges on distinct instances [anchor: parallel_challenges_on_distinct_instances]

The uniqueness constraint applies **only** to identical challenge instances.

It is explicitly permitted for multiple challenges to exist concurrently involving the same ideas, provided they differ in at least one instance-defining parameter.

Examples of permitted parallel challenges include:

- an importance challenge between idea A and idea B (universal scope),
  concurrently with an importance challenge between idea A and idea B relative to a third idea C,
- an importance challenge between idea A and idea B,
  concurrently with truth challenges concerning claims made *about* A or B,
- an importance challenge between idea A and idea B,
Each such challenge is treated as an independent deliberative process with its own lifecycle and verdict.

Each such challenge is treated as an independent deliberative process with its own lifecycle and verdict.

---

### 2.8 Rationale (non-normative) [anchor: rationale_non_normative]

This constraint ensures that:

- deliberation is focused rather than fragmented,
- participants converge on a single shared arena for argument and voting,
- outcomes are legible and attributable,
- and the system avoids race conditions, vote splitting, and strategic duplication.

Revisability is preserved by allowing repeated challenges over time, while simultaneity is constrained to maintain coherence.


## 3. Challenge lifecycle and state machine [anchor: 3_challenge_lifecycle_and_state_machine]

### 3.1 Lifecycle overview [anchor: lifecycle_overview]

Every challenge progresses through the same deterministic lifecycle:

1. **Created**
2. **Open for arguments**
3. **Closed for arguments**
4. **Open for voting**
5. **Closed for voting**
6. **Finalized**

Lifecycle transitions occur only via explicit canonical events and MUST follow the permitted transition table defined below.

---

### 3.2 Lifecycle states [anchor: lifecycle_states]

#### 3.2.1 Created [anchor: created]
- The challenge exists but accepts no arguments or votes.
- Eligibility pools and windows are not yet active.

#### 3.2.2 Open for arguments [anchor: open_for_arguments]
- Eligible identities MAY submit arguments and evidence.
- Voting is prohibited.

#### 3.2.3 Closed for arguments [anchor: closed_for_arguments]
- No new arguments or evidence may be attached.
- Existing arguments remain visible and challengeable.

#### 3.2.4 Open for voting [anchor: open_for_voting]
- Eligible voters MAY cast exactly one vote.
- Arguments are frozen.

#### 3.2.5 Closed for voting [anchor: closed_for_voting]
- No further votes may be cast.
- Tally computation becomes possible.

#### 3.2.6 Finalized [anchor: finalized]
- A verdict has been recorded.
- Canonical state transformations have been applied.
- The challenge is immutable thereafter.

---

### 3.3 Deterministic window definition [anchor: deterministic_window_definition]

All lifecycle windows MUST be defined exclusively using **cycle index deltas** derived from the canonical event log (Protocol v5 §3).

Windows are anchored to specific lifecycle transition events (e.g., `challenge_open_arguments`) and are measured in cycles from the cycle index at which the anchor event occurs.

Wall-clock time, timestamps, calendars, block heights, or event index deltas MUST NOT be used to define lifecycle windows.

---

### 3.3.1 Cycle anchoring of lifecycle phases (Normative) [anchor: cycle_anchoring_of_lifecycle_phases_normative]

Let:

Let:

* `c(X)` be the cycle index at which lifecycle event `X` occurs.
* `A_cycles` be the argument-phase length in cycles (rulebook-defined).
* `V_cycles` be the voting-phase length in cycles (rulebook-defined).

Then the lifecycle phase rules MUST be:

1. **Open for arguments**

   * Begins at `challenge_open_arguments` (cycle `c(open_args)`).
   * Remains open while:

   ```
   current_cycle < c(open_args) + A_cycles
   ```

2. **Closed for arguments**

   * Occurs at the first valid `challenge_close_arguments` event at or after cycle `c(open_args) + A_cycles`.

3. **Open for voting**

   * Begins at `challenge_open_voting` (cycle `c(open_vote)`).
   * Remains open while:

   ```
   current_cycle < c(open_vote) + V_cycles
   ```

4. **Closed for voting**

   * Occurs at the first valid `challenge_close_voting` event at or after cycle `c(open_vote) + V_cycles`.

All compliant implementations MUST evaluate lifecycle phase validity using the cycle indices derived from deterministic replay.

---

### 3.4 Permitted state transitions [anchor: permitted_state_transitions]

The only permitted transitions are:

* Created → Open for arguments
* Open for arguments → Closed for arguments
* Closed for arguments → Open for voting
* Open for voting → Closed for voting
* Closed for voting → Finalized

Any event attempting to skip, repeat, or reverse a transition MUST be rejected.

---

### 3.5 Cancellation and supersession [anchor: cancellation_and_supersession]

A challenge MAY be cancelled or superseded only if:

* no votes have been cast, and
* the active rulebook explicitly permits cancellation.

Cancellation MUST be recorded as a canonical event and MUST NOT erase history.

Superseded challenges remain visible and immutable.

---

### 3.6 Single-finalization invariant [anchor: single_finalization_invariant]

A challenge MAY be finalized exactly once.

Any attempt to finalize a challenge more than once MUST be rejected.

This invariant ensures that each challenge produces at most one canonical transformation.

---

### 3.7 Voter Eligibility and Selection (Normative) [anchor: voter_eligibility_and_selection_normative]

#### 3.7.1 Eligibility Pools [anchor: eligibility_pools]

For each challenge, the eligible voter pool MUST be defined by the active rulebook at a deterministic freeze boundary.

For non-governance challenges, the freeze boundary is the deterministic **voting-open boundary**: the cycle-boundary transition at which `challenge_open_voting` becomes effective.

For governance challenges, eligibility freeze remains challenge-open scoped as defined by Protocol v5 governance sections.

Eligibility rules MAY include, but are not limited to:

human verification level

jurisdiction or tribe membership

minimum account age or participation thresholds

explicit exclusions (e.g. challenge authors, argument authors, conflicts of interest)

Eligibility determines who may be selected, not vote weight. All eligible voters are equal.

Eligibility pool membership MUST NOT depend on current mana or current vote-session capacity.

#### 3.7.2 Deterministic Selection Boundary [anchor: deterministic_selection_boundary]

Canonical voter assignment uses voter-initiated **vote-session pull** semantics (Protocol v5 §6.5). Candidate selection for each vote session MUST occur at a deterministic boundary defined in cycle terms and committed state, such as:

* the first cycle boundary after the challenge enters a specified lifecycle state, or
* a specific cycle offset from a lifecycle anchor event (rulebook-defined).

Selection MUST NOT occur at an arbitrary time chosen by the challenge creator or any other participant.

If snapshot boundaries are used as the selection boundary, they MUST be deterministically derived from the canonical log, and the selection boundary MUST still be expressible in cycle terms.

---

### 3.8 Verifiable Random Selection (Normative) [anchor: verifiable_random_selection_normative]

#### 3.8.1 Selection Seed Construction [anchor: selection_seed_construction]

The vote-session candidate seed MUST be derived from values committed prior to session candidate computation.

The seed MUST be computed as:

vote_session_seed = Hash(
selection_boundary_commitment ||
voter_identity_id ||
session_index ||
selection_boundary_id ||
randomness_beacon_value (if configured)
)

Where:

selection_boundary_commitment is the committed root of canonical state at the deterministic selection boundary (e.g., snapshot root) or an equivalent protocol-defined commitment

voter_identity_id uniquely identifies the voter opening the session

session_index is a monotonically increasing per-identity vote-session index

selection_boundary_id is a block-height anchored boundary identifier. It MUST be either:
(a) snapshot_id for a snapshot whose identity is keyed to block height H (preferred), or
(b) the block height H itself (if snapshot_id is unavailable).
Cycles MUST NOT be used to derive snapshot identity; cycle_index_at_height may be included as metadata only.

randomness_beacon_value is optional and MUST be independently verifiable if used

#### 3.8.2 Deterministic Shuffle [anchor: deterministic_shuffle]

For each vote session, the open challenge set for which the voter is eligible and which still requires jurors MUST be deterministically shuffled using `vote_session_seed`.

The first `M` unique challenge candidates in the shuffled list constitute the session candidate set, where `M` is rulebook-defined (base profile: 3).

The voter may choose one candidate challenge (or decline) per session according to rulebook rules. All compliant implementations MUST derive identical candidate sets from identical inputs.

#### 3.8.3 Anti-Grinding Constraint [anchor: anti_grinding_constraint]

A vote-session seed MUST depend on at least one value unknown at challenge creation time.

Any attempt to restart, replay, or re-request the same session index to fish for different candidates MUST be deterministically rejected.

Challenge-centric preselection caches MAY exist as implementation accelerators only if they are fully derivable from canonical vote-session seed inputs and cannot alter accepted outcomes.

---

## 3.9 Vote Integrity and Uniqueness (Normative) [anchor: vote_integrity_and_uniqueness_normative]

### 3.9.1 One-Vote-Per-Identity Rule [anchor: one_vote_per_identity_rule]

For any challenge, **at most one vote per eligible identity MAY be accepted** during deterministic replay.

If multiple vote events exist for the same `(challenge_id, identity_id)` pair:

* the first valid vote encountered during deterministic replay is accepted, and
* all subsequent vote events for that pair are invalid.

This rule applies regardless of ordering, packaging, or inclusion context.

### 3.9.2 Voting Window Enforcement [anchor: voting_window_enforcement]

Votes MUST be submitted within the voting window defined by the active rulebook in **cycle terms**, anchored to the cycle index at which the challenge enters the Open for voting state.

Votes submitted outside the cycle-bounded voting window are permanently invalid and MUST NOT be accepted during replay.

### 3.9.3 Verdict Finalization [anchor: verdict_finalization]

Once a challenge verdict is finalized:

* the voting outcome is immutable, and
* no additional votes may affect the result.

Later discoveries of fraud, identity revocation, or rule changes MUST NOT retroactively alter finalized verdicts.

---

## 3.10 Commit–Reveal Voting (Optional, Rulebook-Controlled) [anchor: commit_reveal_voting_optional_rulebook_controlled]

### 3.10.1 Commit Phase [anchor: commit_phase]

When required by the active rulebook, voting MUST occur using a commit–reveal process.

During the commit phase, a selected voter MUST submit a `vote_commit` event containing a cryptographic commitment of the form:

commit_hash = Hash(vote_choice || nonce)

The commit event MUST NOT reveal the vote choice.

The commit phase window MUST be defined in **cycle terms**, anchored to a lifecycle transition event specified by the active rulebook (e.g., `challenge_open_voting`).

### 3.10.2 Reveal Phase [anchor: reveal_phase]

During the reveal phase, the voter MUST submit a `vote_reveal` event disclosing:

* `vote_choice`, and
* `nonce`.

The reveal MUST cryptographically match the previously submitted commit.

The reveal phase window MUST be defined in **cycle terms**, anchored deterministically to the end of the commit phase (or other rulebook-defined anchor).

### 3.10.3 Failure to Reveal [anchor: failure_to_reveal]

Failure to submit a valid reveal within the reveal window MUST be treated as defined by the active rulebook, such as:

* abstention, or
* invalid vote.


## 4. Arguments, evidence, and deliberative inputs [anchor: 4_arguments_evidence_and_deliberative_inputs]

### 4.1 Single content model (ideas only) [anchor: single_content_model_ideas_only]

The challenge engine recognizes **one content model** for deliberation inputs:

- **Arguments and evidence are ordinary ideas**.
- No separate canonical object types for “argument” or “evidence” exist.
- No challenge-local payloads or embedded content are permitted.

Arguments and evidence MUST be expressed as ideas authored by verified human identities and positioned relative to a challenge or its subject using canonical connection primitives.

This invariant ensures that all deliberative content is:
- attributable,
- rankable,
- challengeable,
- replayable,
- and composable across challenges.

---

### 4.2 Argument ideas [anchor: argument_ideas]

An **argument** is an idea that attempts to persuade eligible voters regarding the framed question of a challenge.

Arguments MAY be expressed as:
- `conceptual_idea` (normative or interpretive reasoning),
- `truth_claim` (claims about facts, causality, or evidence),
- or other permitted idea types, provided they do not violate domain constraints.

Arguments are associated with a challenge by creating a `relative_importance` connection with a rulebook-defined usage value indicating argumentative role (e.g., `importance_argument`, `supports`, `opposes`).

Arguments:
- MAY be submitted only while the challenge is **open for arguments**,
- MUST remain immutable once authored,
- MUST remain visible after submission, even if later contested.

---

### 4.3 Evidence ideas [anchor: evidence_ideas]

**Evidence** is an idea intended to support or undermine a truth claim relevant to a challenge. Evidence, testimony, attestations, observations, source statements, arguments, and measurements are roles of ideas, not additional canonical content-object types.

Evidence MUST be expressed as an idea (typically a `truth_claim`) and associated to its target using a `relative_importance` connection with one of the following usages:
- `evidence_for`
- `evidence_against`
(or rulebook-defined equivalents that do not introduce new connection types).

Evidence:
- MUST be authored by a verified human identity,
- MUST NOT be deleted or altered after creation,
- MAY be countered only by:
  - additional evidence ideas, or
  - tombstoning the placement connection (if permitted by rulebook).

---

### 4.4 Immutability and historical integrity [anchor: immutability_and_historical_integrity]

Once created, argument and evidence ideas are immutable.

Deliberative correction occurs only through:
- counter-arguments,
- counter-evidence,
- subsequent challenges.

The challenge engine MUST NOT permit:
- editing argument content,
- replacing evidence content,
- retroactively altering deliberative history.

This preserves full auditability and deterministic replay.

---

### 4.5 Submission validity rules [anchor: submission_validity_rules]

Nodes MUST reject any deliberative input event that:

- is authored outside the argument window,
- attempts to embed content directly into the challenge object,
- attempts to reference non-canonical or future objects,
- violates domain-specific framing constraints.

Rulebooks MAY define additional admissibility constraints (e.g., relevance filters, formatting requirements), provided they are deterministic and uniformly applied.

---

## 5. Eligibility, voting, and verdict aggregation [anchor: 5_eligibility_voting_and_verdict_aggregation]

### 5.1 Eligibility computation [anchor: eligibility_computation]

Eligibility to participate in a challenge (as voter or contributor) MUST be computed deterministically from:

the ordered canonical event log,

the active rulebook set,

the active rulebook set,

protocol invariants.

For non-governance challenges, the eligibility reference point is the deterministic voting-open boundary (the cycle-close transition that makes `challenge_open_voting` effective). For governance challenges, the eligibility reference point is challenge-open as defined by Protocol v5 governance rules.

Eligibility MUST NOT depend on:

node-local state,

external identity attributes,

discretionary moderation,

wall-clock time.

Eligibility pool membership MUST NOT depend on current mana balance or current vote-session capacity.

Rulebooks MAY define eligibility criteria such as:

Rulebooks MAY define eligibility criteria such as:

exclusion of the challenge creator,

exclusion of identities with conflicts of interest,

inclusion of identities meeting contribution, participation, or verification thresholds,

domain-specific eligibility pools (e.g., jurisdiction, tribe membership), provided such membership is represented canonically.

### 5.2 Voting rights and constraints [anchor: voting_rights_and_constraints]

During the open for voting window:

Each eligible identity MAY cast exactly one vote per challenge.

Votes MUST be authored by verified human identities.

Vote events MUST be rejected if:

the voter is ineligible,

the voter has already voted in that challenge.

the voter has already voted in that challenge.

Additionally, vote events MUST be rejected if they violate canonical per-identity rate limits for voting as defined by Protocol v5 §3 (cycles, pacing, and rate limits).

Eligibility and ability are distinct:

- identities may remain in the eligibility pool while temporarily unable to vote due to insufficient voting mana/session capacity,
- each vote submission (or vote-session capacity reservation event, if used) MUST be rejected when the voter lacks sufficient deterministic voting capacity at that canonical position.

Unless an active rulebook states a stricter policy, the default voting-capacity cost is positive (`> 0`) per accepted vote session or vote attempt.

Votes are immutable once cast.

### 5.3 Vote semantics by domain [anchor: vote_semantics_by_domain]

Each challenge domain defines the semantic interpretation of votes:

- **Truth challenges**: votes express support for predefined outcome options (e.g., upheld, overturned, uncertain).
- **Importance challenges**: votes express which of the two candidate ideas should rank higher in the complete declared context. The reference or universal pole is not a vote option.
- **Action challenges**: votes express approval, rejection, or selection as defined by framing.
- **Representation challenges**: votes select among competing representations.

The set of valid vote options MUST be fixed at challenge creation.

---

### 5.4 Tally computation [anchor: tally_computation]

Vote tallies MUST be computed deterministically using:

- the complete set of valid votes,
- the challenge’s domain,
Rulebooks MAY define:

Rulebooks MAY define:
- majority thresholds,
- supermajority requirements,
- quorum requirements,
- tie-breaking rules,

provided all such rules are:
- deterministic,
- explicitly defined,
- replayable from the event log.

---

### 5.5 Verdict determination [anchor: verdict_determination]

A **verdict** MUST be recorded as a canonical event when:

- the voting window is closed, and
- tally computation produces a valid outcome under rulebook rules.

Verdict recording MUST be explicit. Outcomes MUST NOT be inferred implicitly.

A verdict event MUST reference:
- the challenge,
- the computed outcome,
- the tally summary sufficient for independent verification.

---

### 5.6 Canonical state transformation [anchor: canonical_state_transformation]

Each verdict produces a deterministic canonical state transformation defined by:

- the challenge domain,
- the framing parameters,
- the active rulebook set.

State transformations MAY include:
- creating or updating connections,
- updating derived certainty or importance state,
- activating a selected representation,
- recognizing or rejecting an action.

State transformations MUST NOT:
- delete canonical history,
- retroactively alter prior events,
- bypass future challengeability.

---

### 5.7 Post-verdict finality and revisability [anchor: post_verdict_finality_and_revisability]

A finalized verdict is final **only at its position in the canonical log**.

Subsequent challenges MAY:
- revisit the same subject,
- introduce new evidence,
- produce new verdicts that supersede earlier ones.

The system treats verdicts as **current best judgments**, not immutable truths.

---
## 6. Verdict effects, state mutation, and replay guarantees [anchor: 6_verdict_effects_state_mutation_and_replay_guarantees]

### 6.1 Verdicts as the only mutation gateway [anchor: verdicts_as_the_only_mutation_gateway]

A finalized challenge verdict is the **only** mechanism by which a challenge may affect canonical state.

Accordingly:

- All canonical state mutations attributable to deliberation MUST be produced by a finalized verdict event.
- No intermediate lifecycle event (argument submission, vote casting, window transitions) may mutate canonical state beyond the challenge’s own lifecycle fields.
- Nodes MUST reject any event that attempts to apply deliberative effects without referencing a finalized verdict.

This rule guarantees that all contested changes are auditable and replayable.

---

### 6.2 Domain-specific mutation rules [anchor: domain_specific_mutation_rules]

Each challenge domain maps verdict outcomes to deterministic state mutations as follows.

#### 6.2.1 Truth challenge effects [anchor: truth_challenge_effects]
A truth challenge verdict MAY:
- update derived certainty bands associated with the challenged truth claim,
- update derived truth-status labels (e.g., upheld, overturned, uncertain),
- affect downstream importance calculations through rulebook-defined mappings.

A truth challenge verdict MUST NOT:
- delete or rewrite the challenged claim,
- retroactively alter evidence or argument history,
- prevent future truth challenges on the same claim.

#### 6.2.2 Importance challenge effects [anchor: importance_challenge_effects]
An importance challenge verdict MUST:
- apply only within the exact declared rank kind, orientation/reference, usage, axis, timeframe, and scope;
- if the challenger wins and remains below the target, remove it from its current position and insert it immediately above the target while preserving the relative order of every other idea;
- preserve the list when the challenger loses or is no longer below the target under the base conflict rule;
- trigger recomputation of universal aggregate state only when a universal axis changed.

Importance challenge verdicts MUST NOT:
- set absolute importance values,
- affect unrelated axes or scopes,
- directly author `universal_position_sum`, `universal_position_mean`, or `overall_universal_rank`,
- use POD, POINT, popularity, model confidence, reputation, or weighted ballots to determine movement,
- bypass future importance challenges.

#### 6.2.3 Action challenge effects [anchor: action_challenge_effects]
An action challenge verdict MAY:
- recognize or reject an action or actionable idea,
- update derived action status fields (as defined by rulebooks),
- affect downstream POD routing as a consequence of importance flows.

Action challenge verdicts MUST NOT:
- execute actions automatically,
- assert real-world completion without accompanying truth claims,
- prevent future action challenges.

#### 6.2.4 Representation challenge effects [anchor: representation_challenge_effects]
A representation challenge verdict MUST:
- select exactly one representation outcome for the referenced target (`idea` or `ordering`) and the challenge's resolved representation scope,
- become effective only when replay applies the corresponding `challenge_finalize_verdict` event,
- supersede any previously active representation without deleting history.

Effect surface is scope-dependent:
- If resolved scope is universal, finalization updates the universal canonical representation pointer for the target slot.
- If resolved scope is non-universal, finalization updates scoped display-override state for that `scope_key` and target slot, and MUST NOT mutate the universal canonical representation pointer.

Representation challenges MUST NOT:
- modify the underlying target object content,
- prevent future representation challenges.

---

### 6.3 Deterministic application requirements [anchor: deterministic_application_requirements]

Nodes MUST apply verdict effects deterministically using only:

- the ordered canonical event log,
- the finalized verdict event,
- the active rulebook set at the verdict’s position,
- protocol constants.

Node-local configuration, timing, or discretion MUST NOT affect verdict application.

---

### 6.4 Replay invariants [anchor: replay_invariants]

During deterministic replay:

- Applying the same event log MUST always yield identical challenge states, verdict outcomes, and resulting canonical state.
- Nodes MUST detect and reject:
  - duplicate verdict application,
  - verdicts referencing non-finalized challenges,
  - verdicts whose declared effects contradict the challenge framing.

Any replay divergence constitutes a protocol violation.

---

### 6.5 Supersession and historical continuity [anchor: supersession_and_historical_continuity]

Later verdicts MAY supersede earlier verdicts by:
- creating new connections that dominate earlier ones under ranking rules,
- updating derived certainty or importance state.

Supersession MUST:
- preserve historical records,
- never erase prior verdicts,
- remain itself subject to future challenge.

The canonical log therefore records a continuous deliberative history rather than a single “final truth.”

---

## 7. Safety, governance, and cross-domain interaction [anchor: 7_safety_governance_and_cross_domain_interaction]

### 7.1 Safety interface [anchor: safety_interface]

The challenge engine is safety-agnostic.

Safety systems MAY:
- restrict visibility of arguments, evidence, or votes,
- require additional procedural steps for certain domains or jurisdictions,
- label or contextualize challenge outcomes.

Safety systems MUST NOT:
- alter challenge lifecycle rules,
- modify vote tallies or verdict outcomes,
- introduce hidden or discretionary state changes.

All safety effects MUST operate as overlays on canonical state.

---

### 7.2 Governance interface [anchor: governance_interface]

Governance operates through the same challenge engine.

Accordingly:

- Governance proposals MUST be expressed as actionable ideas.
- Governance decisions MUST be resolved via action challenges.
- Rulebook adoption, modification, suspension, or removal MUST be gated by finalized challenge verdicts and activated at deterministic boundaries (e.g., snapshot boundaries).

The challenge engine itself MUST NOT encode governance authority or policy content.

---

### 7.3 Token and incentive interface [anchor: token_and_incentive_interface]

The challenge engine does not mint or transfer tokens.

However:

- Verdict outcomes MAY affect importance rankings.
- Importance rankings MAY affect POD routing.
- POD routing MAY affect POINT generation according to the Token Specification.

These effects MUST be indirect and rulebook-defined. The challenge engine MUST NOT contain token logic.

---

### 7.4 Infrastructure and meta-deliberation interface [anchor: infrastructure_and_meta_deliberation_interface]

Challenges MAY be used to evaluate claims about:
- infrastructure operation,
- replay correctness,
- availability,
- censorship allegations,
- protocol maintenance actions.

Such challenges:
- MUST obey the same lifecycle and eligibility rules,
- MUST NOT affect canonical acceptance or ordering,
- MAY affect importance and long-term rewards.

This enables the system to reason about its own operation without creating circular authority.

---

### 7.5 Final invariants [anchor: final_invariants]

The challenge engine SHALL preserve the following invariants across all domains:

- **Human-first deliberation**: only verified humans author, vote, and finalize challenges.
- **Single resolution primitive**: all contested change flows through challenges.
- **Determinism**: all outcomes are replayable from the event log.
- **Revisability**: no verdict is beyond future challenge.
- **Non-coercion**: no challenge outcome enforces compliance outside the protocol.
- **Tempo separation**: Tempo evidence and attestations are ordinary ideas in evidential roles; Tempo contributor status does not grant challenge authority.

---

---
## 8. Verdict finalization [anchor: 8_verdict_finalization]

### 8.1 Finalization event [anchor: finalization_event]

A challenge is resolved **only** by a `challenge_finalize_verdict` event, as defined in Appendix A.

Accordingly:

- A challenge MUST NOT be considered resolved, applied, or effective until a `challenge_finalize_verdict` event has been recorded.
- Finalization MUST reference a challenge that has completed all required lifecycle stages.
- Finalization is **irreversible**. Once recorded, a finalized verdict MUST NOT be revoked, edited, or replaced.

Any attempt to apply verdict effects without a corresponding finalization event MUST be rejected.

---

### 8.2 Verdict structure [anchor: verdict_structure]

A finalized verdict MUST encode, at minimum:

- the `challenge_id`,
- the resolved outcome (encoded according to the challenge domain),
- a deterministic tally summary sufficient for independent verification,
- references to the governing rulebook versions used to compute the outcome,
- the canonical ordering reference at which finalization occurs.

The verdict payload MUST NOT include discretionary or interpretive fields. All semantic meaning MUST be derivable from:
- the challenge framing,
- the recorded votes,
- and the referenced rulebooks.

---

### 8.3 Disputes after finalization [anchor: disputes_after_finalization]

Finalized verdicts are part of immutable canonical history.

Accordingly:

- A finalized verdict MUST remain visible and replayable indefinitely.
- No event MAY retroactively alter or invalidate a finalized verdict.

Disagreement with a finalized verdict MAY be expressed only by:
- initiating a new challenge that re-evaluates the same instance after finalization,
- introducing new arguments or evidence into subsequent challenges.

This preserves revisability without erasing history.

---

## 9. State transformation mapping [anchor: 9_state_transformation_mapping]

This section defines how finalized verdicts produce canonical state transformations.  
All transformations are **verdict-applied only** and MUST be deterministic.

---

### 9.1 Truth challenge effects (verdict-applied only) [anchor: truth_challenge_effects_verdict_applied_only]

Truth challenges produce canonical effects **only** through a finalized verdict.

Accordingly:

- Any change to a claim’s derived truth-state (e.g., confidence class, certainty band, upheld/contested/overturned labeling) MUST occur only as the deterministic transformation specified for the finalized verdict.
- No participant MAY directly mutate truth-state by submitting an update event outside the challenge lifecycle.

Rulebooks MAY define deterministic mappings from:
- vote outcomes and tallies,
- admissible evidence conditions,
- challenge integrity checks,
to:
- derived certainty bands,
- derived truth-status labels,
- and other permitted truth-state outputs.

For target-bound Tempo time claims, evidence-placement and certainty-band challenge verdicts determine operative truth certainty under the Tempo Specification. Tempo `T_allow` structural support is separate: it may permit provisional structural cycle closure from current eligible-human stances plus capped passive evidence while truth challenges continue. Challenge verdicts MUST NOT delete the claim, mark a selected winning time claim as authoritative, create a beacon as an authored object, infer truth certainty from counts or hidden weights, or bypass the Tempo Specification's derived beacon and lagged authorization-frontier rules.

Nodes MUST reject (or deterministically ignore, if rulebooks specify no-effect acceptance) any event that attempts to directly set truth-state fields without passing through a truth challenge and verdict finalization.

This ensures truth-state remains a product of deliberation rather than direct write access.

---

### 9.2 Importance challenge effects [anchor: importance_challenge_effects_2]

An importance challenge verdict MUST produce its effects by:

- applying the Protocol v5 immediate-above bubble-up rule to the exact universal or relative context declared in the challenge;
- preserving the relative order of all non-challenger ideas;
- retaining every argument, ballot, verdict, and prior ordering state in history.

For a universal-axis change, replay MUST recompute the affected idea's exact twenty-position sum/mean and the overall universal order. That arithmetic is downstream derived state and is not a challenge target. Relative verdicts affect only their declared public-relative or tribe-relative context.

Importance challenge verdicts MUST NOT:
- set absolute importance values,
- alter unrelated axes or scopes,
- reinterpret a reference idea as a contestant,
- convert private owner ordering into canonical state,
- bypass future importance challenges.

---

### 9.3 Action challenge effects [anchor: action_challenge_effects_2]

An action challenge verdict MAY:

- recognize or reject an action or actionable idea for canonical consideration,
- update derived action-status indicators defined by rulebooks,
- trigger downstream eligibility for POD routing based on importance flows.

Action challenge verdicts MUST NOT:
- execute actions automatically,
- assert real-world completion without accompanying truth claims,
- equate importance with moral approval or endorsement.

This preserves the separation between *importance*, *recognition*, and *normative judgment*.

---

### 9.4 Representation challenge effects [anchor: representation_challenge_effects_2]

A representation challenge verdict MUST:

- select exactly one representation outcome for the referenced target (`idea` or `ordering`) and the challenge's resolved representation scope,
- activate that representation deterministically only when replay applies `challenge_finalize_verdict`,
- supersede prior canonical representations without deleting or rewriting them.

Scope-dependent activation semantics:
- Universal-scope representation challenges update universal canonical representation pointers.
- Scoped representation challenges update only scoped display-override state for the framed `scope_key`.
- Scoped representation challenges MUST NOT alter universal canonical representation pointers.

Representation challenge verdicts MUST NOT:
- modify the underlying target object content,
- erase historical representations,
- prevent future representation challenges.

This preserves interpretive evolution while maintaining historical continuity.



## 10. POD relevance and importance signaling [anchor: 10_pod_relevance_and_importance_signaling]

### 10.1 Challenge importance [anchor: challenge_importance]

A challenge itself MAY be assigned derived importance within the canonical system.

Challenge importance is **derived**, not authored, and MAY be computed using rulebook-defined mappings that consider factors such as:

- the derived importance of the ideas or connections affected by the challenge,
- the breadth and diversity of verified human participation,
- the magnitude and scope of downstream canonical state transformations,
- the persistence or durability of the verdict over subsequent challenges.

Challenge importance MUST be computed deterministically from:
- the ordered canonical event log,
- finalized verdicts,
- and the active rulebook set.

Challenge importance MUST NOT be manually assigned, directly edited, or asserted by participants.

---

### 10.2 POD hooks and indirect reward signaling [anchor: pod_hooks_and_indirect_reward_signaling]

Challenges do not mint, transfer, or guarantee POD.

However, challenge outcomes MAY indirectly influence POD routing by:

- altering importance relations between ideas,
- stabilizing or destabilizing high-impact claims,
- enabling or disabling downstream actions that themselves carry importance.

POD issuance and POINT generation depend on:
- the importance of ideas and actions affected by challenges,
- the durability of those importance relations over time,
- and rulebook-defined epoch mechanics.

Accordingly:

- **Winning a challenge does not guarantee POD.**
- POD accrues only if the effects of a challenge remain important and unchallenged over time.
- Fraudulent, low-quality, or unstable challenge outcomes will lose importance and therefore lose POD relevance.

This ensures that incentives favor sustained epistemic contribution rather than short-term tactical victories.

---

## 11. Failure modes and edge cases [anchor: 11_failure_modes_and_edge_cases]

### 11.1 Insufficient participation [anchor: insufficient_participation]

Rulebooks MAY define minimum participation requirements for challenges, such as:

- minimum number of eligible voters,
- minimum quorum thresholds,
- or minimum diversity constraints.

If a challenge fails to meet required participation thresholds:

- the challenge MUST NOT produce a canonical verdict,
- the challenge MUST be deterministically closed without effect,
Such challenges remain part of immutable history and MAY be re-initiated later under the same instance-defining parameters.

Such challenges remain part of immutable history and MAY be re-initiated later under the same instance-defining parameters.

---

### 11.2 Withdrawals [anchor: withdrawals]

#### 11.2.1 Challenge creator withdrawal [anchor: challenge_creator_withdrawal]

Withdrawal by the challenge creator:

- MUST NOT erase the challenge,
- MUST NOT invalidate already-submitted arguments or votes,
- MAY affect lifecycle progression only if permitted by the active rulebook.

Rulebooks MAY permit creator withdrawal only before voting begins. If permitted, the challenge MUST be deterministically closed and marked as withdrawn without producing a verdict.

#### 11.2.2 Participant withdrawal [anchor: participant_withdrawal]

Participants MAY withdraw arguments or votes only if:

- the active rulebook explicitly permits withdrawal, and
- withdrawal is expressed via canonical events (e.g., tombstoning a placement connection).

Withdrawals MUST NOT retroactively alter tallies that have already closed.

---

### 11.3 Rulebook changes mid-challenge [anchor: rulebook_changes_mid_challenge]

Challenge behavior uses deterministic freeze boundaries:

- governance challenges freeze eligibility/quorum/threshold rule references at challenge-open (Protocol v5 governance rule),
- non-governance challenges freeze eligibility and voter-selection references at voting-open boundary.

Accordingly:

Rulebook changes MUST NOT retroactively affect:

eligibility criteria,

lifecycle window durations (including cycle-bounded phase lengths),

voter selection rules,

vote aggregation rules,

verdict thresholds,
for challenges whose relevant freeze boundary has already been reached.

If a rulebook changes while a challenge is active:

the challenge MUST continue under the rulebook snapshot captured at the applicable freeze boundary,

any new challenges MUST use the updated rulebook set.

This prevents governance manipulation and preserves procedural fairness.

### 11.4 Final invariants for failure handling [anchor: final_invariants_for_failure_handling]

Across all failure modes:

- history MUST remain immutable,
- outcomes MUST be explicit,
- behavior MUST be deterministic,
- and all unresolved or failed challenges MUST remain challengeable in the future.

Failure to reach resolution is itself a canonical outcome, not an error condition.


## 12. Safety and jurisdictional overlays [anchor: 12_safety_and_jurisdictional_overlays]

### 12.1 Safety classification during challenges [anchor: safety_classification_during_challenges]

Safety mechanisms operate strictly as **visibility and presentation overlays** on top of canonical challenge processes.

Accordingly:

- Safety classifications MUST NOT affect:
  - challenge creation validity,
  - lifecycle progression,
  - eligibility computation,
  - vote tallying,
  - verdict finalization,
  - or canonical state transformations.

- Safety rules MAY:
  - restrict visibility of arguments, evidence, votes, or representations,
  - abstract or summarize deliberative content,
  - require additional user acknowledgments or warnings,
  - provide jurisdiction-specific explanations for restricted visibility.

If an argument or evidence item is abstracted or hidden due to safety rules:

- it MUST still be counted canonically for the purposes of:
  - eligibility determination,
  - vote tallies,
  - verdict computation,
  - deterministic replay.

Nodes MUST NOT drop, alter, or reinterpret canonical challenge data due to safety classification.

---

### 12.2 Jurisdictional lenses [anchor: jurisdictional_lenses]

Challenges are **global and single-canon**.

Accordingly:

- Every challenge, vote, and verdict exists exactly once in canonical history.
- Jurisdictional or regulatory differences MAY affect what is *shown* to users, but MUST NOT affect what *exists* canonically.

Jurisdictional lenses MAY:
- hide or abstract specific arguments or evidence,
- annotate challenges with regulatory context,
- explain why certain content is unavailable in a given jurisdiction.

Jurisdictional lenses MUST NOT:
- fork challenge outcomes,
- modify vote tallies,
- create alternate verdicts,
- suppress the existence of a challenge itself.

All users, regardless of jurisdiction, participate in the same canonical challenge universe.

---

## 13. Conformance requirements [anchor: 13_conformance_requirements]

### 13.1 Node conformance [anchor: node_conformance]

A conformant node MUST:

- enforce the challenge lifecycle state machine exactly as specified,
- reject invalid lifecycle transitions deterministically,
- enforce uniqueness of concurrent challenge instances,
- compute eligibility using only canonical data and rulebooks,
- accept at most one valid vote per eligible identity per challenge,
- compute vote tallies and verdict outcomes deterministically,
- apply verdict effects only through finalized verdict events,
- preserve all canonical history during replay and synchronization.

A node MUST NOT:
- infer outcomes implicitly,
- apply discretionary moderation to canonical state,
- use wall-clock time or external signals for challenge evaluation.

Failure to meet these requirements constitutes non-conformance.

---

### 13.2 Client conformance [anchor: client_conformance]

A conformant client MUST:

- represent challenge lifecycle states accurately and without ambiguity,
- distinguish clearly between:
  - open vs closed stages,
  - arguments vs votes,
  - finalized vs non-finalized outcomes,
- present verdicts as outcomes of deliberation, not authoritative truths,
- respect jurisdictional and safety overlays without implying canonical differences,
- avoid misrepresenting abstracted or hidden content as absent from canon.

Clients MUST NOT:
- suggest that safety-hidden content did not participate canonically,
- imply that challenge outcomes are irreversible or beyond dispute,
- enable users to bypass lifecycle or eligibility constraints.

Clients that misrepresent canonical challenge behavior undermine protocol integrity and are non-conformant.



## 14. Non-goals and exclusions [anchor: 14_non_goals_and_exclusions]

This specification intentionally excludes the following mechanisms and behaviors.

### 14.1 No weighted voting by status or tokens [anchor: no_weighted_voting_by_status_or_tokens]

The challenge engine MUST NOT support weighted voting based on:

- POD balance,
- POINT balance,
- wealth, reputation, tenure, or rank,
- prior challenge outcomes,
- or any other status-derived metric.

All voting power within a challenge is equal among eligible participants, as determined by rulebooks. Governance influence is procedural, not economic.

Tempo contributor status is separate from challenge authority. Challenge voting eligibility MUST NOT be inferred from Tempo contributor status, and Tempo contributor status MUST NOT be inferred from challenge voter status.

---

### 14.2 No AI adjudication or autonomous resolution [anchor: no_ai_adjudication_or_autonomous_resolution]

AI systems MUST NOT:

- adjudicate challenges,
- cast votes,
- determine eligibility,
- finalize verdicts,
- or apply canonical state transformations.

AI MAY assist humans in drafting, analysis, simulation, or summarization, but all canonical challenge actions MUST be explicitly authored and confirmed by verified human identities.

---

### 14.3 No shortcut resolution mechanisms [anchor: no_shortcut_resolution_mechanisms]

The protocol forbids any shortcut paths that bypass the challenge lifecycle.

Accordingly, there SHALL be:

- no administrative overrides,
- no privileged resolution channels,
- no emergency backdoors,
- no implicit or inferred outcomes,
- no direct state writes that substitute for a challenge verdict.

All contested canonical change flows through the challenge engine or does not occur.

---

### 14.4 No deletion or rewriting of challenge history [anchor: no_deletion_or_rewriting_of_challenge_history]

Challenge history is immutable.

The system MUST NOT permit:

- deletion of challenges,
- deletion of votes,
- rewriting or editing of verdicts,
- retroactive alteration of lifecycle events.

Correction and disagreement occur only through subsequent challenges and verdicts, never by erasing the past.

---

## 15. Relationship to other specifications [anchor: 15_relationship_to_other_specifications]

This specification composes with and depends on the following documents:

- **Protocol v5**  
  Defines constitutional invariants, canonical authority, identity requirements, and human-first authorship rules.

- **Canonical Data Model & Event Schemas (Appendix A)**  
  Defines all canonical objects, event types, validation rules, and deterministic replay requirements used by the challenge engine.

- **Governance Specification**  
  Defines rulebooks, eligibility constraints, adoption procedures, and governance-controlled parameters referenced by the challenge engine.

- **Token Specification**  
  Defines POD and POINT issuance, epoch mechanics, and how challenge outcomes indirectly influence incentives.

- **Safety Specification**  
  Defines visibility, abstraction, jurisdictional overlays, and appeal mechanisms that operate on top of canonical challenge processes.

In the event of any conflict:

- **Protocol v5 is authoritative**,  
- followed by **Appendix A**,  
- followed by this specification.

This document introduces no independent authority and derives all legitimacy from the core protocol.


### 15.1 Challenge Engine as Sole Path for Consequential Change [anchor: challenge_engine_as_sole_path_for_consequential_change]

Any canonical state transformation that resolves disagreement, comparison, ranking, or representation MUST occur exclusively through a finalized challenge verdict.

No parallel mechanism - administrative, automated, or exceptional—MAY produce equivalent effects outside this engine.
