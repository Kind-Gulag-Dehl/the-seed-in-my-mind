---
doc_id: governance_spec
title: Governance Specification
status: authoritative
version: v0
last_reviewed: 2026-06-22

scope:
  - Defines governance processes, rulebooks, and modification procedures.

authoritative_for:
  - Governance decision-making mechanisms.
  - Rulebook creation, amendment, and enforcement.

not_authoritative_for:
  - Token issuance mechanics beyond governance hooks (see token-spec.md).

depends_on:
  - protocol v5.md
  - tempo-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of safety-spec.md and token-spec.md.

reader_path:
  - prereq: challenge-engine-spec.md
  - next: token-spec.md

keywords:
  - governance
  - rulebooks
  - voting
  - amendments
---

> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

# Governance Specification (v5)

### status: draft normative specification [anchor: status_draft_normative_specification]
This document defines the governance subsystem of the Seed protocol, including how rulebooks evolve, how proposals and challenges operate, how votes are conducted, how rulebooks activate at cycle boundaries, and how governance interacts with safety, identity, token, and action systems.

---


## 0. scope, purpose, and status [anchor: 0_scope_purpose_and_status]

### 0.1 purpose [anchor: purpose]
This specification defines the canonical governance subsystem of the Seed. Governance is internal to the graph: rulebooks, proposals, challenges, verdicts, implementations, and activations are all represented as ideas and events within the canonical universe. This document specifies:

- how governance rules evolve through deliberation,
- how governance proposals are authored, challenged, voted on, verified, and activated,
- how rulebooks are created, versioned, superseded, and applied,
- how governance interacts with the unified challenge framework and action system,
- how universal governance and tribal governance interrelate,
- how participation rules apply to identities (humans only),
- how the safety, identity, and token subsystems connect to governance,
- how deterministic replay and constitutional invariants constrain governance actions.

### 0.2 normative vs informative text [anchor: normative_vs_informative_text]

### 0.2 normative vs informative text [anchor: normative_vs_informative_text]
Normative sections define conformance requirements:

- schemas for governance proposals, rulebooks, and governance actions,
- lifecycle rules for governance challenges,
- quorum and threshold semantics,
- parameters for voter eligibility,
- executor eligibility and voluntary-action constraints,
- deterministic activation rules at cycle boundaries,
- state-transition semantics for rulebook supersession and activation.

Informative sections provide:

- UI patterns and explainers,
- examples and templates,
- commentary on rule design,
- recommended advisory practices for Ents in their non-authoritative role.

Informative text MUST NOT override normative requirements.

### 0.3 dependencies [anchor: dependencies]

Governance is a dependent subsystem. Correct governance behavior requires:

- **Protocol v5**  
  (idea ontology, challenge primitive, constitutional invariants, action system, cycle semantics, rulebook applicability semantics)

- **Challenge and Event Specification**  
  (structure of challenge creation, argumentation, voter selection, voting windows, verdicts, and state updates)


  (action declarations, completion truth claims, verification workflow, POD routing)

- **Safety Specification**  
  (safety rulebooks, classifier configuration governance, payload class constraints, jurisdiction overlays)

- **Identity Specification**  
  (verification, quarantine, suspension, reintegration; constraints on voting eligibility)

- **Token Specification**  
  (POD non-transferability, POINT-generation invariants, governance limits on token parameters)

- **Node & Conformance Specification**  
  (event-log integrity, block hashing, snapshot structure and tiers, replay determinism, invalid-event handling)

Governance MAY also reference Tribe Specification and AI Boundaries Specification for additional constraints and advisory behavior, but the governance processes defined here remain authoritative with respect to rulebook evolution and activation.

Governance in this specification additionally depends on the following Protocol v5 concepts and MUST remain consistent with them:

- **Cycle-anchored activation semantics**  
  Governance activation scheduling MUST be expressed in cycle indices (for example `activation_cycle_index`) derived deterministically from the canonical event log. Governance MUST NOT use wall-clock time, block height, snapshot frequency, or snapshot tiers as activation boundaries.

- **Blocks and snapshots as non-semantic packaging**  
  Blocks (fixed-size event bundles) and snapshots (including tiered snapshots) provide cryptographic anchoring, distribution, and performance acceleration only. They MUST NOT determine rulebook applicability or activation timing.

- **Derived `lifecycle_state` and living-map eligibility**  
  Governance MUST treat eligibility and inclusion as derived and replay-verifiable. Governance rulebooks and governance actions MUST NOT introduce authored eligibility flags that override derived eligibility, `lifecycle_state`, or living-map inclusion rules.

- **Cycle export packs**  
- **Offline & Mindseed semantics**  

- **Offline & Mindseed semantics**  
  Governance events, voter eligibility determination, and activation scheduling MUST remain replay-verifiable under offline publication and canonical ingestion semantics, independent of wall-clock time.


---

### 0.5 activation semantics (normative) [anchor: activation_semantics_harmonization_normative_override]

This section is normative.

Canonical governance activation semantics are defined in Section 2.4, anchor `normative_activation_semantics_decision_and_snapshot_boundary`.

`activation_cycle_index` is the authoritative activation boundary. Rule changes become active at the start of that cycle index (inclusive).

Delay scheduling is computed from canonical log data only: `decision_cycle_index`, `change_class`, and `delay_policy_version` active at decision time.
## 1. governance model overview [anchor: 1_governance_model_overview]

This section defines the structural and constitutional foundations of governance inside the Seed.  
as defined in Protocol v5. No mechanism outside the event log may modify governance outcomes.
operates entirely through ideas, challenges, events, and replay-deterministic state transitions, 
as defined in Protocol v5. No mechanism outside the event log may modify governance outcomes.

### 1.1 governance as first-class ideas [anchor: governance_as_first_class_ideas]

All governance artifacts exist **inside the graph** and follow the same ontological rules as any
other idea or event. Governance introduces no new primitives beyond those already defined in 
Protocol v5; instead, it uses existing constructs in structured combinations:

- **Rulebooks** are conceptual + actionable ideas that encode normative rules for a domain  
  (governance, safety, identity, tokens, tribes, or protocol semantics).

- **Governance proposals** are **actionable ideas** that contain draft rulebooks or rulebook
  modifications. Like all actionable ideas, they do not execute automatically; they must be 
  endorsed through an action challenge and then voluntarily implemented by human identities.

- **Governance challenges** are **action challenges** with additional constraints defined in this
  specification. They drive the deliberation and voting process that determines whether a 
  rulebook proposal is accepted, rejected, or superseded.

- **Verdicts** are canonical events that enshrine the outcome of a governance challenge. A 
  verdict does not itself activate a rulebook; it schedules activation contingent on 
  implementation and snapshot rules.

- **Implementations** are **real-world actions** carried out voluntarily by human identities and 
  recorded as **completion truth claims**. Governance rulebooks only activate when their 
  implementation actions withstand truth verification.

Governance MUST NOT rely on any out-of-band mechanism, privileged operator role, or implicit 
authority. All authority arises from canonical events, and all governance processes are 
expressed, recorded, and replayed deterministically within the system.

### 1.2 foundational governance principles [anchor: foundational_governance_principles]


These principles constrain what governance MAY do and what governance MAY NOT override:

- **Determinism** — nodes MUST agree on which rulebooks were active for which events under deterministic replay.

- **Human-first authorship** — only humans initiate canonical governance actions; AI cannot hold autonomous authority.

- **Equal voting power (one person, one vote)** — POD, POINT, wealth, seniority, or reputation MUST NOT influence vote weight.

- **Voluntary action only (Protocol §0.44)** — no identity may be compelled to implement a governance action; participation in governance execution MUST always be voluntary.

- **Perpetual challengeability** — any rulebook, governance decision, or representation of a rule may be challenged under the appropriate domain.

- **Immutable history** — governance MAY create new rulebooks or supersede prior ones, but MAY NOT delete or modify historical events.

- **Replay determinism** — nodes MUST reconstruct the same governance state when replaying the log, using only deterministic derivation from the canonical event log and the rule applicability semantics defined here and in Protocol v5.

- **Anti-capture & decentralization** — governance MUST NOT introduce mechanisms that centralize authority, give special privileges, or allow durable capture by any entity or subgroup.

- **Canonical processes only** — rule changes and rulebook supersession occur only through the ordinary canonical pipeline: challenge → verdict → (optional) implementation → cycle-anchored activation.

These principles form the boundary conditions for every governance rulebook and every governance challenge.

These principles form the boundary conditions for every governance rulebook and every governance challenge.

For avoidance of doubt:

- Blocks and snapshots are cryptographic anchoring and packaging artifacts only and MUST NOT be used as governance activation boundaries.

Governance MUST NOT use wall-clock time, block height, snapshot timing, or snapshot tiers as activation boundaries. Cycles MAY constrain pacing and participation, and governance activation, rulebook supersession, and rulebook applicability during replay are determined solely by cycle-anchored activation semantics derived deterministically from the canonical event log.



### 1.3 classes of governance [anchor: classes_of_governance]

Governance inside the Seed operates under a single, unified structure. All identities, groups, 
and tribes use the same challenge primitive, the same voting rules, the same quorum and 
threshold semantics, the same action system, and the same cycle-based activation rules. 
governance process defined in Protocol v5 and this specification.
Any governance action recorded in the canonical universe MUST follow the exact same universal 
governance process defined in Protocol v5 and this specification.


mechanisms, but to distinguish *what* a governance action concerns:

- **Universal governance**  
  Governance concerning the canonical universe as a whole, including rulebooks for protocol 
  semantics, safety floors, identity verification, token mechanics, classifier configuration, 
  and the governance system itself. Universal decisions bind all identities and all nodes.

- **Tribe-relevant governance**  
  Governance actions that specifically *concern* a tribe, its membership, its public projects, 
  or its publicly recorded activities. These actions still use the **same universal governance 
  machinery**, including one-person-one-vote, the same challenge lifecycle, the same quorum 
  and threshold semantics, and the same rulebook activation processes.  
  Tribes MAY meet, deliberate, coordinate, and make decisions outside the protocol using any 
  procedures they choose, but when these decisions are recorded inside the canonical universe, 
  Tribal governance therefore describes a *topic domain*, not a separate ruleset.
  were performed by any other subset of humans.  
  Tribal governance therefore describes a *topic domain*, not a separate ruleset.

- **Hybrid governance**  
  Governance actions whose effects influence both a tribe and the broader system. As with 
  tribe-relevant governance, hybrid governance uses the same universal mechanism. A hybrid 
  decision is one whose content affects both a local tribe context and universal rulebooks or 
  universal importance structures, but the mechanics used to decide it remain unchanged.

Because all governance actions share one unified mechanism, tribes DO NOT have:
- separate voting weights,
- separate quorum or threshold rules,
- separate challenge rules,
- separate action or implementation mechanics,
- separate rulebook activation procedures,
- private or hidden governance histories,
- or any authority to override universal protocol invariants.

Tribes may organize, deliberate, and coordinate however they choose outside the protocol, but 
inside the canonical universe every governance action is processed identically for all 
participants. This preserves deterministic replay, prevents governance fragmentation, and 

logic.

When governance actions concern tribes, any eligibility roster or domain-qualified subset used for voter selection MUST be reconstructable deterministically from canonical events and active rulebooks, independent of wall-clock time.

Tribe-relevant governance MUST NOT introduce private or hidden governance histories, private rulebook activation semantics, or tribe-private activation boundaries. Any tribe-scoped eligibility constraints must still be represented and validated as replay-verifiable canonical state.


### 1.4 human-first constraint [anchor: human_first_constraint]

All governance operations are performed through the standard challenge framework

rulebook, rule change, or governance action is at issue, participants use:

- **action challenges** to decide whether a proposed governance action (a rulebook,
  configuration change, or structural adjustment) should be endorsed,
- **truth challenges** to verify completion claims for governance actions,
- **representation challenges** to merge or distinguish governance ideas or clarify
  canonical descriptions.

Only verified human identities may perform any canonical governance operation.
Specifically, only humans may:

- author governance-related actionable ideas,
- open or participate in any challenge concerning governance,
- vote in any challenge, including governance-related ones,
- declare or complete governance implementation actions,
- attest to evidence or implementation truth claims.

AI identities MAY:

- draft proposed rulebooks or analyses for humans to adopt,
- summarize or contrast arguments,
- generate hypothetical actions or structural implications,
- produce advisory reports.

AI identities MAY NOT:

- vote,
- open challenges,
- declare or complete actions,
- activate rulebooks,
- serve as executors for governance implementations.

All governance actions remain subject to the **voluntary-action invariant**
implementation activities must be undertaken by willing human volunteers.
procedure, vote, or mechanism to perform an action. All execution and
implementation activities must be undertaken by willing human volunteers.

vote**, except where a governance rulebook specifies randomly sampled juror
pools drawn from the same equal-weight set of eligible humans.
vote**, except where a governance rulebook specifies randomly sampled juror
pools drawn from the same equal-weight set of eligible humans.

---
## 2. governance rulebooks [anchor: 2_governance_rulebooks]

### 2.1 rulebooks as versioned ideas [anchor: rulebooks_as_versioned_ideas]

A rulebook is an immutable **idea** of type `actionable_idea` paired with a structured rulebook object in its metadata. A rulebook specifies the active procedures that nodes and participants MUST use when interpreting governance actions, voting rules, challenge rules, and activation conditions.

Each rulebook instance MUST include:

- a **version number**, unique within its family,
- a **scope = universal** (tribes MAY NOT define separate governance rules),
- a list of **predecessor rulebooks** it supersedes,
- definitions and parameters relevant to its rule domain,
- **quorum rules** and **threshold rules** for the challenges it governs,
- **voter eligibility rules**, if domain-specific,
- **activation timing rules** (cycle-anchored),
- **executor requirements** for implementing approved governance actions.

Rulebooks become effective only at cycle boundaries as scheduled by governance verdicts and expressed via `activation_cycle_index` (§2.4).




### 2.2 rulebook families [anchor: rulebook_families]

The system maintains several distinct but structurally identical rulebook
families, each governing a particular normative domain:

- **governance rulebooks** — challenge lifecycles, voting rules, quorum and
  canonical semantics, and idea ontology.
- **safety rulebooks** — classifier definitions, abstraction rules, specificity
  threshold mechanics, voter pool definitions, and implementation requirements.
- **identity rulebooks** — identity verification requirements, credential
  boundaries, jurisdictional lenses, and safety floors.
- **token rulebooks** — parameters governing POD routing, POINT generation, and
  decay rules, quarantine procedures, and eligibility checks.

  token-cycle timing.

Tribes DO NOT have separate rulebooks or modified versions of these rulebooks.
A tribe may deliberate internally however it wishes, but any canonical record of
tribe decisions MUST use the same rulebook families and challenge mechanics used
by the universal system.

### 2.3 rulebook invariants (non-overrideable) [anchor: rulebook_invariants_non_overrideable]

- human-first authorship of all canonical events,
- equality of governance voting (one human = one vote, except when using
  equal-weight juror pools),
- non-transferability of POD,
- immutable canonical history,
- deterministic replay rules,
- prohibition on AI authority or AI governance participation,
- system-wide safety floors on globally illegal or harmful payloads,
- challengeability of all propositions and rule changes.

These invariants are non-overrideable by ordinary rulebooks, emergency procedures, deployment settings, or implementation convenience.

### 2.3A Tempo Profile Governance Boundary [anchor: tempo_profile_governance_boundary]

Governance owns the active Tempo profile as rulebook configuration. Governance MAY set and amend bounded parameters such as:

- `Dmin`;
- `Dmax`;
- lag `K`;
- `required_human_support_dmin`;
- `required_human_margin_dmin`;
- `required_human_support_dmax`;
- `required_human_margin_dmax`;
- `survivor_dmax_min_human_support`;
- `T_allow`;
- `passive_evidence_cap`;
- `passive_source_dedup_policy`;
- `passive_source_class_policy`;
- `passive_outlier_policy`;
- `contradiction_block_band`;
- `T_beacon`;
- `T_beacon_revoke`;
- `beacon_minimum_certainty_band`;
- `minimum_beacon_identities`;
- `minimum_independence_domains`;
- `beacon_stability_cycles`;
- `beacon_challenge_survival_cycles`;
- `authorization_lag_k`;
- `tempo_mana_cap`;
- `tempo_mana_recharge`;
- `time_claim_create_cost`;
- `tempo_evidence_claim_create_cost`;
- `tempo_evidence_connection_cost`;
- `tempo_same_as_connection_cost`;
- `time_challenge_cost`;
- `certainty_band_order`;
- `structural_dmax_liveness_rule`;
- `constrained_mode_allowlist`;
- `profile_hash`.

Governance MAY define structural-support units for `T_allow`, but it MUST NOT redefine `T_allow` as truth certainty. Passive evidence parameters MUST remain deterministic, canonically committed, and capped below `T_allow`.

Governance MUST NOT override:

- no trusted clock authority;
- human-only canonical authorship;
- idea-only deliberative content: evidence, arguments, attestations, observations, source statements, and time claims remain identity-authored ideas in roles;
- ordinary truth-claim and truth-challenge semantics for target-bound time claims;
- certainty-band challenge semantics for Tempo claims;
- the separation between `T_allow` structural support and ordinary truth certainty;
- the Dmax-only and forced-closure-only limits of `structural_dmax_liveness_predicate`;
- verification and independence as eligibility/diversity gates only, not weights;
- forced-cycle non-authority;
- anti-collapse threshold non-shrinkage;
- no backfill or retroactive authority;
- AI non-authority.

Tempo profile changes MUST activate only through ordinary governance lifecycle and cycle-anchored activation. Governance MUST NOT use block height, publication count, server time, client timestamps, scheduler observations, or AI-generated observations as Tempo authority.

### 2.4 activation at cycle boundaries [anchor: activation_at_snapshot_boundaries]

#### Normative activation semantics (decision cadence and cycle-based activation) [anchor: normative_activation_semantics_decision_and_snapshot_boundary]

This subsection is normative and supersedes conflicting wording elsewhere in this document.

- `decision_event`: the canonical event that confirms a governance change at cycle close.
- `decision_cycle_index`: the cycle index in which `decision_event` is confirmed.
- `delay_policy`: the active governance rulebook mapping `change_class -> delay_cycles`.
- `change_class`: one of `emergency`, `standard`, `major`, `constitutional`.
- `delay_cycles`: deterministic cycle delay derived from `delay_policy(change_class)` under the `delay_policy_version` active at `decision_cycle_index`.
- `activation_cycle_index = decision_cycle_index + delay_cycles`.

Normative lifecycle:
1. Governance challenges conclude at cycle end and emit `decision_event`.
2. Activation scheduling is computed from `decision_cycle_index`, `change_class`, and the `delay_policy_version` active at decision time.
3. The change activates at the start of `activation_cycle_index` (inclusive) for deterministic replay and conformance.
4. Nodes MUST compute identical `activation_cycle_index` from canonical replay inputs only.
5. Snapshot boundaries MUST NOT define governance activation boundaries.

Delay-policy constraints:
- The rulebook MUST publish minimum delay bounds per `change_class`.
- Recommended delay ranges are: `emergency` 0-1 cycle, `standard` 1-2 cycles, `major` 3-5 cycles, `constitutional` 10+ cycles.
- Governance MAY evolve exact delay mappings by rulebook update, but activation MUST remain cycle-boundary deterministic.

A rulebook change becomes effective only when the required conditions are satisfied and the scheduled activation cycle is reached.

A rulebook change becomes effective only when the required conditions are satisfied and the scheduled activation cycle is reached.

A rulebook becomes active only when all required conditions are satisfied:

1. A governance-related **action challenge** endorses a rulebook adoption, amendment, or supersession (Yes verdict).
2. If the rulebook family requires implementation, a human identity successfully completes the required **implementation action(s)**, attesting via completion truth claim(s) that the rulebook was implemented.
A governance verdict MUST specify:

A governance verdict MUST specify:
- `verdict_cycle_index`, and
- `activation_cycle_index`, where `activation_cycle_index > verdict_cycle_index`.

Nodes MUST apply rulebooks according to cycle-anchored applicability:

- For any canonical event `E`, compute the derived `cycle_index(E)`.
- The new rulebook version applies if and only if `cycle_index(E) e activation_cycle_index`.

This ensures deterministic replay across all nodes without reliance on wall-clock time, block height, or snapshot timing.

Rulebooks MAY NOT activate mid-cycle.


### 2.5 supersession [anchor: supersession]

A new rulebook MUST clearly specify:

- the exact rulebook(s) it supersedes,
- the successor version number,
- the cycle boundary at which it becomes active (expressed via `activation_cycle_index`, or as a rulebook-defined cycle delay to be instantiated into `activation_cycle_index` at verdict time).

Nodes MUST load and apply the correct rulebook version when replaying events according to deterministic cycle-anchored applicability:

- for any canonical event `E`, the applicable rulebook version is determined by the derived `cycle_index(E)`,
- a new rulebook version applies if and only if `cycle_index(E) e activation_cycle_index` for that supersession.

Supersession is strictly forward-only:  
rulebooks cannot be retroactively modified, merged, or corrected. Attempts to retroactively alter rulebooks MUST be represented as new actionable ideas and evaluated through standard challenges and cycle-anchored activation rules.



## 3. proposal lifecycle (full governance flow) [anchor: 3_proposal_lifecycle_full_governance_flow]

(protocol, governance, safety, identity, token). It follows the same lifecycle
A governance proposal represents a prospective modification to a rulebook family
(protocol, governance, safety, identity, token). It follows the same lifecycle
as all other actionable ideas but carries additional requirements concerning
compatibility, justification, and implementation before activation.

### 3.1 proposal definition [anchor: proposal_definition]

A governance proposal MUST be represented as an actionable idea containing:

- the full **proposed rulebook content** or modifications,
- a clear **rationale** explaining the necessity or benefit,
- a list of **implementation actions** required prior to activation (if any),
- the intended **activation cycle schedule** (expressed as a rulebook-defined delay in cycles, or as an explicit `activation_cycle_index` relative to the eventual verdict cycle),
- a list of **implementation actions** required prior to activation (if any),
- optional AI-generated material (permitted only as a draft and MUST be explicitly marked in metadata).

The actionable idea MUST be authored by a verified human.  
AI MAY assist in drafting but cannot create canonical proposals.

### 3.2 lifecycle states [anchor: lifecycle_states]

Governance proposals move through the standard challenge lifecycle, resulting in seven canonical states:

1. **draft**  
2. **arguments open**  
3. **voter pool selection** (if required by rulebook)  
4. **voting window**  
5. **verdict issued**  
6. **implementation actions performed**  
7. **activation at cycle boundary**

These states are represented as explicit events in the canonical universe.

Lifecycle transitions for governance proposals MUST be reconstructable via deterministic replay from canonical events and rulebooks, without dependence on wall-clock timestamps.

Lifecycle transitions for governance proposals MUST be reconstructable via deterministic replay from canonical events and rulebooks, without dependence on wall-clock timestamps.


### 3.3 draft stage [anchor: draft_stage]

- The proposal is created by a verified human identity.
- The draft is immediately visible to all users.
- Arguments MAY begin informally, but voting cannot begin until an
  **action challenge** has been formally opened on the proposal.
- Drafts MAY incorporate AI-authored summaries, comparisons, or structured
  revisions, but the decision to adopt AI text MUST be explicitly performed by a
  human identity.

### 3.4 argument stage [anchor: argument_stage]

Once an action challenge is opened against the governance actionable idea:

- Participants submit **arguments** as ordinary ideas,
- They MAY attach them to the proposal using relative_importance connections
  (usage = `importance_argument`) or any rulebook-defined usage for governance,
- Evidence supporting or challenging the proposal MAY be attached via
  `evidence_for` or `evidence_against`,
- Rulebooks MAY enforce:
  - minimum argument-duration windows,
  - minimum number of perspectives or identity diversity,
  - requirements for referencing prior rulebooks.

The argument stage MUST be fully recorded and replayable.

### 3.5 voter selection [anchor: voter_selection]

- **Humans only** may vote.

- **Humans only** may vote.
- The eligible pool is defined by the **active governance rulebook**, not by the
  tribe or by influence metrics.
- Pools MAY be:
  - the entire verified human population,
  - a random juror subset (equal-weight sampling),
  - a domain-qualified subset (e.g., identity-verified node operators), when
    explicitly allowed by the active rulebook.
- No delegation, no staking, no coercion.
Eligibility for voting in governance proposals MUST be derived and replay-verifiable.

Eligibility for voting in governance proposals MUST be derived and replay-verifiable.

Any rulebook-defined eligibility pool (entire verified-human set, juror subset, or domain-qualified subset) MUST be:
- determinable from canonical state and active rulebooks at challenge-open time,
- independent of wall-clock time,
- and recorded in a way that permits deterministic reconstruction during replay.

Where a governance action is scope-constrained, voter eligibility MUST be derived from the action's resolved `scope_key = (scope_kind, anchor_id)` at challenge-open time.

- **Verified Identity / Uniqueness (VI)**
- **Verified Human (VH)** and
- **Verified Identity / Uniqueness (VI)**

and the mapping from certainty values to discrete eligibility tiers.

This specification does not define those calculations, but governance rulebooks MUST reference them only as gating criteria and MUST NOT introduce vote weighting.


### 3.6 voting stage [anchor: voting_stage]

- Votes are binary: **Yes / No**.
- Optional human-readable rationales MAY be added.
- Rulebooks define quorum and threshold conditions.
- Voting windows MUST be fixed-duration and deterministic.
- AI identities MAY NOT vote or influence quorum or threshold conditions.

When the voting window closes, the challenge proceeds to verdict computation.

Wall-clock timestamps may be recorded as non-authoritative evidence, but MUST NOT determine voting-window opening, closing, quorum evaluation, or threshold evaluation.

Wall-clock timestamps may be recorded as non-authoritative evidence, but MUST NOT determine voting-window opening, closing, quorum evaluation, or threshold evaluation.


### 3.7 verdict stage (normative) [anchor: verdict_stage_normative]

A **verdict** is a canonical event produced by deterministic tallying of a completed governance challenge.

The verdict records the outcome of deliberation and schedules any resulting rule changes, but it does not itself immediately alter canonical behavior.

#### 3.7.1 verdict outcomes [anchor: verdict_outcomes]

- A **Yes** verdict means the proposal is adopted *in principle*.
- A **No** verdict means the proposal remains inert and produces no changes.
#### 3.7.2 verdict contents [anchor: verdict_contents]

#### 3.7.2 verdict contents [anchor: verdict_contents]

A governance verdict MUST specify:

- the identity of the challenged actionable idea (the governance proposal),
- the derived `verdict_cycle_index` at which the verdict was finalized,
- the scheduled `activation_cycle_index`,
- the rulebook(s) or rule sections affected,
- any dependencies on other rulebook changes or prerequisite conditions.

The `activation_cycle_index` MUST be strictly greater than the `verdict_cycle_index`.

Normative addition [anchor: verdict_fields_snapshot_boundary_addition]: governance verdict metadata MUST include `decision_cycle_index`, `change_class`, `delay_policy_version`, and computed `activation_cycle_index`. `activation_cycle_index` MUST equal `decision_cycle_index + delay_policy(change_class)` under the `delay_policy_version` active at decision time.

#### 3.7.3 activation semantics [anchor: activation_semantics]

This subsection is consistent with Section 2.4 `normative_activation_semantics_decision_and_snapshot_boundary` and uses the same normative lifecycle.

A successful governance verdict does not activate rule changes immediately.

A rulebook change becomes effective at the start of the scheduled activation cycle.

Formally:
- For any canonical event `E`, the applicable rulebook version is determined by the derived `cycle_index(E)`.
- The new rulebook version applies if and only if `cycle_index(E) e activation_cycle_index`.

This activation rule is deterministic, replay-verifiable, and independent of wall-clock time, block height, or snapshot timing.

#### 3.7.4 relationship to blocks and snapshots [anchor: relationship_to_blocks_and_snapshots]

Blocks and snapshots do not determine when governance changes activate.

They serve only as:
- cryptographic commitment anchors,
- packaging and distribution boundaries,
- performance accelerators for replay.

Governance activation MUST NOT be keyed to block height, snapshot frequency, or snapshot tier.

#### 3.7.5 implementation and auditability [anchor: implementation_and_auditability]

Verdict events MAY be referenced by later implementation, completion, or audit claims.

All governance verdicts MUST be:
- publicly inspectable,
- fully replayable from the canonical event log,
- accompanied by deterministic explanations of why and when activation occurred.


### 3.8 implementation stage [anchor: implementation_stage]

Rulebooks do not self-apply.  
Implementation requires real-world action performed voluntarily by a human.

A rulebook endorsed by verdict MUST be implemented through the standard action pipeline:

1. A human identity issues an **action declaration** stating the intent to implement the rulebook.
2. The human performs all required implementation steps (publishing rulebook schemas, updating classifier configs, producing reproducible builds, updating node software, etc.).
3. The human submits a **completion truth claim**, attaching:
   - links to commits, configuration updates, or specification documents,
   - diffs or reproducible build artifacts,
   - evidence showing that the rulebook is fully implemented and ready to be activated.

Completion truth claims are ordinary truth claims and MAY be challenged via truth challenges.  
A successful falsification of the completion claim prevents the rulebook from activating.

If no valid and verified completion truth claim exists before the scheduled activation cycle is reached:

- the rulebook **does not activate**,  
- the preceding rulebook remains in force,  
- the governance proposal retains its historical record but has no operative effect.

Only after a valid completion claim exists and the scheduled activation cycle boundary is reached does the new rulebook become active.

For avoidance of doubt:
- blocks and snapshots do not determine activation timing,
- they may be used to package or distribute post-activation state, but they do not control rule applicability.


## 4. governance as deliberation over actionable ideas (normative) [anchor: 4_governance_as_deliberation_over_actionable_ideas_normative]

Governance in this system is not a separate mechanism or authority layer.  
All governance activity is expressed through **actionable ideas**, **challenges**, **votes**, and **verdicts**, using the same canonical processes defined elsewhere in Protocol v5.

All governance activity is expressed through **actionable ideas**, **challenges**, **votes**, and **verdicts**, using the same canonical processes defined elsewhere in Protocol v5.

There are no governance-specific challenge types, voting systems, or event primitives.

### 4.1 rulebooks as ideas [anchor: rulebooks_as_ideas]

A **rulebook** is an idea whose descriptive content defines rules that affect canonical interpretation, eligibility, safety, token behavior, or system configuration.

Rulebooks:
- are represented as ordinary ideas,
- use the same multi-tier description system as all other ideas,
- are publicly viewable and challengeable,
- may be referenced, duplicated, or forked like any other idea.

The system recognizes certain ideas as rulebooks solely for the purpose of **deterministic replay and activation semantics**.  
This recognition does not create a separate ontology or governance layer.

### 4.2 governance proposals [anchor: governance_proposals]

A **governance proposal** is an actionable idea whose intent is to modify, amend, supersede, or interpret one or more rulebook ideas.

Governance proposals:
- reference the rulebook(s) they affect,
- specify the proposed change (textual, parametric, or structural),
- participate in challenges exactly like any other actionable idea.

A governance proposal does not alter system behavior merely by existing or being approved in principle.

Only a completed challenge with a canonical verdict may schedule a rule change.

### 4.3 verdicts and activation semantics [anchor: verdicts_and_activation_semantics]

This subsection is consistent with Section 2.4 `normative_activation_semantics_decision_and_snapshot_boundary` and uses the same normative lifecycle.

A successful governance challenge produces a **verdict** that records:

- the cycle index at which the verdict was finalized,
- the scheduled **activation_cycle_index**,
- the rulebook change to be applied.

Rulebook changes do **not** activate immediately upon verdict.

A rulebook change becomes effective at the start of the scheduled activation cycle.

Formally:
- For any canonical event `E`, the applicable rulebook version is determined by the derived `cycle_index(E)`.
- A rulebook change applies if and only if `cycle_index(E) e activation_cycle_index`.

This ensures:
- deterministic replay,
- independence from wall-clock time,
- resistance to activity-rate manipulation.

### 4.4 blocks and snapshots in governance [anchor: blocks_and_snapshots_in_governance]

Blocks and snapshots do not determine governance activation.

They serve only as:
- cryptographic anchors,
- packaging boundaries,
- performance and distribution artifacts.

Governance semantics MUST NOT depend on block height, snapshot frequency, or snapshot tier.

### 4.5 the system governing itself [anchor: the_system_governing_itself]

Its rulebooks:

Its rulebooks:
- define how governance operates,
- define how those definitions may be changed,
- and are themselves governed by the same deliberative process.

There is no external or privileged governance authority.

Any apparent bootstrapping authority arises solely from current eligibility and participation conditions, which are themselves subject to future governance.

---

## 5. voting mechanics (normative) [anchor: 5_voting_mechanics_normative]

Voting in governance challenges follows the same canonical mechanics as voting in any other challenge.

There are no governance-specific voting rules beyond those defined by applicable rulebooks.

### 5.1 voter eligibility [anchor: voter_eligibility]

Only identities that meet **derived eligibility criteria** may vote.

Eligibility is computed deterministically during replay and MAY depend on:
- verification status,
- quarantine or suspension state,
- scope (universal, tribe, personal),
- rulebook-defined constraints.

Eligibility is never authored or manually assigned.

Artificial or automated identities MUST NOT vote in governance challenges.

For governance actions that target writable canonical surfaces, eligibility MUST be bound to a deterministic **scope key**:
- `scope_key = (scope_kind, anchor_id)`

The governing rulebook MUST define how `scope_key` is resolved from the challenged action, and all conformant nodes MUST derive the same `scope_key` from canonical inputs at challenge-open time.

### 5.1.1 overlay and substrate write eligibility matrix [anchor: overlay_and_substrate_write_eligibility_matrix]

The governance engine remains unified across all scopes. The following matrix defines minimum write eligibility classes used by governance challenges and endorsements:

| Write class | Scope key | Minimum write eligibility |
| --- | --- | --- |
| Universal substrate writes | `(universal, universal_anchor)` | Eligible verified human under active universal governance rulebooks. |
| Personal overlay writes | `(personal, owner_identity_id)` | Owner-only: actor identity MUST equal `owner_identity_id` and satisfy baseline derived eligibility constraints. |
| Tribe overlay writes | `(tribe, tribe_anchor_idea_id)` | Membership-gated: actor identity MUST satisfy deterministic tribe membership eligibility for `tribe_anchor_idea_id` under active rulebooks. |

Additional constraints:
- `universal_anchor` MUST be a deterministic, rulebook-defined constant identifier for universal-scope eligibility resolution.
- Rulebooks MAY add stricter predicates, but MUST NOT relax owner-only personal overlay authority or membership-gated tribe overlay authority.
- Overlay read visibility MUST NOT be treated as write authority.
- Challenge voter/participant eligibility for any scoped governance action MUST be derived from the same `scope_key` at challenge-open time and replayed unchanged for that challenge instance.

### 5.2 vote forms [anchor: vote_forms]

Votes MAY include:
- **yes**
- **no**
- **abstain** (if permitted by the applicable rulebook)

Votes MAY also include an optional **rationale**, which is treated as evidence and remains publicly inspectable.

Rationales do not alter vote weight.

### 5.3 quorum requirements [anchor: quorum_requirements]

Quorum rules are defined by rulebooks.

Rulebooks MAY specify:
- minimum participation thresholds,
- whether abstentions count toward quorum,
- distinct quorum rules for different scopes or rulebook families.

Quorum evaluation is performed deterministically at verdict time.

### 5.4 decision thresholds [anchor: decision_thresholds]

Decision thresholds are defined by rulebooks and MAY include:
- simple majority,
- absolute majority,
- supermajority requirements,
- tiered thresholds based on rulebook scope or impact.

Thresholds MUST be explicit and replay-verifiable.

### 5.5 voting windows [anchor: voting_windows]

Voting windows are defined in **cycles**, not wall-clock time.

Rulebooks MAY specify:
- minimum and maximum voting window lengths in cycles,
- extended windows for high-impact or constitutional changes,
- shortened windows for narrowly scoped or tribe-level governance.

Voting windows advance only as cycles advance.

If cycles do not advance, voting windows do not progress.

### 5.6 uniformity principle [anchor: uniformity_principle]

Governance voting is not privileged.

The same:
- rate limits,
- eligibility derivation,
- visibility rules,
- safety abstractions,
- explainability requirements

apply to governance voting as to all other challenges.

This preserves a single, unified deliberative system.


## 6. selecting which proposed actions become real actions [anchor: 6_selecting_which_proposed_actions_become_real_actions]

### 6.1 the actionable idea as the anchor of deliberation [anchor: the_actionable_idea_as_the_anchor_of_deliberation]

### 6.1 the actionable idea as the anchor of deliberation [anchor: the_actionable_idea_as_the_anchor_of_deliberation]
Every governance proposal is an **actionable idea**.  
The actionable idea is the *center* of the action-selection process:

- All proposed implementation methods attach to the actionable idea as **relative_importance (usage = general)** edges.
- Arguments for or against each proposal attach as ideas with **usage = importance_argument** edges.
- No proposal has special status until its relative importance is determined through challenges.

### 6.2 proposed actions as actionable ideas in their own right [anchor: proposed_actions_as_actionable_ideas_in_their_own_right]

### 6.2 proposed actions as actionable ideas in their own right [anchor: proposed_actions_as_actionable_ideas_in_their_own_right]

This keeps the ontology simple and consistent:


Each proposed action has:

Each proposed action has:

- full description tiers,
- supporting arguments,
- related truth and conceptual ideas,
- relative importance edges to the parent actionable idea,
- optional sub-actions if further broken down.

By modeling proposals as actionable ideas rather than special objects, the system avoids adding new semantics.

### 6.3 ranking proposed actions by relative importance [anchor: ranking_proposed_actions_by_relative_importance]

Before any proposal can move into an executable action, the system must determine which proposed actions matter most.

- Any proposed action may challenge a higher-ranked sibling.

- Any proposed action may challenge a higher-ranked sibling.
- Arguments articulate why it better implements the governance change.
- Voters consider:
  - effectiveness,
  - safety,
  - feasibility,
  - risk of capture or centralization.
  - clarity of implementation pathway,
  - risk of capture or centralization.

The result is a well-ordered set of proposed actions, ranked deterministically.


Importance challenges already do everything needed.

Where action sequencing is required (dependencies, prerequisites, ordering constraints), those constraints MUST be expressed as ordinary ideas and connections and evaluated through the same challenge and ranking mechanisms, rather than introducing any separate governance-only workflow.


### 6.4 mutually exclusive vs compatible proposed actions [anchor: mutually_exclusive_vs_compatible_proposed_actions]
Some proposed actions **cannot** both be executed (mutually exclusive), e.g.:


Some proposed actions **can** both be executed (compatible), e.g.:

Some proposed actions **can** both be executed (compatible), e.g.:

- “Add new conformance tests”

Compatible proposals simply appear as multiple top-ranked options.  

Compatible proposals simply appear as multiple top-ranked options.  
Mutually exclusive proposals compete in importance challenges until only one remains above the threshold of plausibility.

### 6.5 selecting the candidate action(s) via ranking [anchor: selecting_the_candidate_action_s_via_ranking]

### 6.5 selecting the candidate action(s) via ranking [anchor: selecting_the_candidate_action_s_via_ranking]
Once importance challenges settle the ordering, rulebooks may specify:

- **top-1 selection** (the highest-ranked proposal becomes the candidate)
- **top-k selection** (multiple proposals move forward if compatible)
These rules do *not* alter challenge semantics; they interpret the ranking.

These rules do *not* alter challenge semantics; they interpret the ranking.

### 6.6 moving a proposed action into an action challenge [anchor: moving_a_proposed_action_into_an_action_challenge]
An action challenge asks:

An action challenge asks:

The result determines whether the proposed action becomes authorized for execution.

The result determines whether the proposed action becomes authorized for execution.

- implementation must be deterministic,

- implementation must be deterministic,
- proposals may be complex,
- completion verification later requires a single well-defined target.

### 6.7 conversion into a real-world action [anchor: conversion_into_a_real_world_action]

If an action challenge endorses the proposed action:

1. A human identity voluntarily issues an **action declaration**.  
2. They perform the real-world implementation (updating configs, publishing rulebooks, etc.).
3. They submit a **completion truth claim** describing what was actually done.
4. The completion claim may be challenged via truth challenges.
5. If the completion claim withstands challenges, the action is confirmed.

Only then may the rulebook activate at its scheduled activation cycle boundary (as specified by `activation_cycle_index` in the governance verdict).


### 6.8 why we do not use ranked-choice voting [anchor: why_we_do_not_use_ranked_choice_voting]
Ranked-choice voting was considered but rejected for governance because:

- It creates non-deterministic interpretive edge cases across nodes.
- Importance challenges already represent the pairwise preference structure.
- Ranked-choice cannot be challenged mid-process; challenges allow incremental refinement.
- Importance-based sorting integrates directly with the epistemic graph and POD routing.

Importance challenges are incremental, transparent, contestable, and replay-stable.  
Ranked-choice is not.

### 6.9 safeguards against extreme or unserious proposals [anchor: safeguards_against_extreme_or_unserious_proposals]
The system does **not censor** extreme proposals.  
Instead, it demotes them epistically:

- Their importance ranking relative to the actionable idea is usually low.
- Arguments highlight infeasibility, harm, or violation of invariants.
- Extreme proposals rarely survive pairwise challenges.

### 6.10 relationship to the spectrum of potential actions [anchor: relationship_to_the_spectrum_of_potential_actions]

### 6.10 relationship to the spectrum of potential actions [anchor: relationship_to_the_spectrum_of_potential_actions]
- For governance, this spectrum clarifies the range of conceivable implementations.

- For governance, this spectrum clarifies the range of conceivable implementations.
- It helps voters understand extremeness and relative feasibility.
- It does *not* automatically determine which proposal wins; that remains deliberative.

This keeps the action-selection layer aligned with the truth-claim evidence-ordering architecture.

### 6.11 determinism and replay guarantees [anchor: determinism_and_replay_guarantees]
- explicit,

- explicit,
- challengeable,
- auditable,
- recorded in the canonical universe.

No ambiguity exists in replay:

- The same proposals appear,
- the same challenges run,
- the same ranking emerges,
- the same endorsed candidate appears,
- the same completion claim is verified,
- the same rulebook activates.

Governance cannot fork through interpretation.

---
## 7. governance scope: universal vs tribal [anchor: 7_governance_scope_universal_vs_tribal]

### 7.1 universal governance (canonical) [anchor: universal_governance_canonical]

### 7.1 universal governance (canonical) [anchor: universal_governance_canonical]
Universal governance governs:

- the protocol rulebooks,
- governance rulebooks,
- safety rulebooks,
- identity rulebooks,
- token rulebooks,
- classifier configurations,
- canonical semantics for ideas, challenges, and actions,
- deterministic replay rules,
- POD/POINT parameters,
- global safety floors.

Universal governance applies to *all* nodes, identities, and tribes.  
Only universal governance can change the semantics or invariants of the system.

Universal rulebooks MUST:

- enforce deterministic replay,
- adhere to non-transferability of POD,
- enforce deterministic replay,
- preserve the voluntary-action invariant,
- forbid AI authority,
- maintain equality of human votes.

### 7.2 tribal governance (strict subset of universal governance) [anchor: tribal_governance_strict_subset_of_universal_governance]

Within the canonical universe, tribes MAY:

Within the canonical universe, tribes MAY:

- form around any idea via membership connections,
- maintain tribe-scoped **importance maps** (public overlays where only tribe members may vote or attach certain importance edges),
- open challenges whose voter eligibility is limited to members of the tribe (for tribe-scoped decisions),
- record tribe decisions, norms, or leadership choices as ordinary ideas,
- attach arguments expressing tribe endorsements or objections to universal proposals.

Within the canonical universe, tribes MAY NOT:

- introduce new challenge domains or lifecycles,
- alter rulebook activation rules,
- create alternate voting systems (e.g., weighted votes),
- modify POD/POINT semantics,
- override safety floors or identity rules,
- create private canonical ideas that only the tribe can see.

All tribe activity that touches the canonical universe remains public, challengeable, and structurally identical to universal activity; only **eligibility scopes** and **importance overlays** differ.

For avoidance of doubt, public visibility of tribe or personal overlays does not grant authority to write those overlays. Write authority MUST be derived from scope-constrained eligibility predicates, not from readability.

A tribe’s “internal rules” (who they consider leaders, how they decide things off-chain, what processes they follow in meetings) are not a separate rulebook family. They are simply **ideas** that describe how the tribe claims to operate. If the tribe wants these internal rules to be visible in the system, they MUST record them as ideas and connect them using the same ontology as any other content.

Examples:

Examples:

- “Tribe X endorses governance proposal G” → an idea linked to G via importance_argument or endorsement-style usage.

The protocol does not grant these internal rules any special canonical authority. Their only power comes from:

The protocol does not grant these internal rules any special canonical authority. Their only power comes from:

- voluntary adherence by tribe members,
- public visibility and criticism,
- and the same challenge and ranking mechanisms that apply to all ideas.

Tribes may therefore experiment socially and organizationally, but they can do so only *through* the universal epistemic substrate, not alongside or outside it within the canonical universe.


### 7.4 tribal decision-making in canonical processes [anchor: tribal_decision_making_in_canonical_processes]
A tribe MAY:

- encourage its members to vote a certain way,
- create arguments supporting a proposal,
- form consensus off-chain and record it via ideas,
- coordinate execution of actions (voluntarily).

But the final canonical decision is **always a universal human vote** in the applicable pool defined by the active rulebook, not a tribal vote.

Tribal deliberation is informational, not authoritative.

### 7.5 tribes and executor selection [anchor: tribes_and_executor_selection]
A tribe MAY:

- encourage members to volunteer as executors,
- run internal discussions to determine who is most qualified,
- attach arguments to candidate executors.

A tribe MAY NOT:

- compel a member to execute an action,
- assign canonical authority to an executor,
- override voluntary-action invariants.

### 7.6 visibility and transparency requirements [anchor: visibility_and_transparency_requirements]

### 7.6 visibility and transparency requirements [anchor: visibility_and_transparency_requirements]
Tribes MUST:

- keep all tribe rulebooks public,
- keep all tribe governance actions public,
- expose tribe-specific importance maps publicly,
- record tribe decisions as canonical events if they intend them to affect the system.

Nothing related to tribe governance may be hidden from the global canonical universe.

---

## 8. ents in governance (advisory-only identities) [anchor: 8_ents_in_governance_advisory_only_identities]

### 8.1 identity and purpose of an ent [anchor: identity_and_purpose_of_an_ent]

### 8.1 identity and purpose of an ent [anchor: identity_and_purpose_of_an_ent]
An Ent is:

- a human identity with high epistemic standing,
- a steward of historical memory,
- a guide to newcomers,
- an advisor in governance arguments,
- a curator of conceptual coherence,
- a narrator of the genealogies of ideas.

Ents do not have elevated voting rights, leadership authority, or inherent ability to execute governance actions.

Their influence is **persuasive**, not structural.

### 8.2 ent privileges and non-privileges [anchor: ent_privileges_and_non_privileges]
Ents MAY:

- draft rulebooks (non-canonically),
- submit arguments,
- advise on challenge design,
- help interpret genealogies,
- maintain narrative continuity,
- produce long-term planning documents,
- assist in translating complex proposals for newcomers.

Ents MAY NOT:

- vote with more weight than any other human,
- bypass challenge mechanisms,
- activate rulebooks,
- serve as executors through privilege (still voluntary + subject to eligibility rules),
- block proposals,
- enforce outcomes,
### 8.3 transformation from founder to ent [anchor: transformation_from_founder_to_ent]

### 8.3 transformation from founder to ent [anchor: transformation_from_founder_to_ent]
The founder identity begins with temporary bootstrap authority defined by initial rulebooks.  
Over time, the community uses canonical governance processes to:

3. dissolve all formal authority,  
2. remove final vetoes and transition powers,  
3. dissolve all formal authority,  
4. transform the founder into the first Steward-Ent.

This transition demonstrates:

- the model for soft conversion of centralized systems into distributed governance,
- the replacement of authority with stewardship,
- the model for soft conversion of centralized systems into distributed governance,
- the principle that no identity retains permanent centralized power.

### 8.4 ents and challenge processes [anchor: ents_and_challenge_processes]
Ents participate exactly like normal humans:

- They may submit arguments,
- open challenges,
- vote,
- declare and complete actions (voluntarily),
- or serve as advisors to others writing proposals.

Ents have no special challenge modes, no unique challenge authority, and no ability to bypass or accelerate deliberation.

### 8.5 ent responsibilities (informative) [anchor: ent_responsibilities_informative]
While not normative, Ents are culturally encouraged to:

- maintain long-term epistemic memory,
- clarify historical reasoning paths,
- ensure conceptual integrity,
- help prevent governance capture through analysis and transparency,
- elevate new contributors,
They are symbolic guardians, not rulers.

They are symbolic guardians, not rulers.

### 8.6 ent continuity across generations [anchor: ent_continuity_across_generations]
The Ent role is not exclusive:

- multiple humans may become Ents over time,
- the role is emergent and social rather than formalized,
Ents eventually form a lineage of custodianship, giving the system a cultural heritage that outlasts individual contributors while remaining fully compatible with canonical governance equality.

Ents eventually form a lineage of custodianship, giving the system a cultural heritage that outlasts individual contributors while remaining fully compatible with canonical governance equality.

---
## 9. identity governance [anchor: 9_identity_governance]

### 9.1 identity rulebooks (universal only) [anchor: identity_rulebooks_universal_only]

### 9.1 identity rulebooks (universal only) [anchor: identity_rulebooks_universal_only]
Identity rulebooks define:

- verification requirements and permitted verification methods,
- standards for continuous authentication (periodic checks, liveness, re-verification),
- rules for when identities enter or exit *quarantine*,
- processes for appeals,
- rules for restoring voting and challenge rights,
- criteria for suspension from voting pools (never from authorship),
- eligibility predicates for identity-related challenges.

Identity rulebooks are **universal rulebooks only**.  
Tribes may NOT create tribe-specific identity rulebooks, identity types, or verification pathways.

The identity rulebook MUST remain consistent with:

- one-human-one-vote governance equality,
- human-first authorship,
- one-human-one-vote governance equality,
- voluntary-action invariants,
- non-deletion of canonical history,
- no identity may ever gain permanent privileged governance rights.

### 9.2 identity verification [anchor: identity_verification]
Verification procedures are defined in identity rulebooks and implemented via:

- completion truth claims (“verification completed with method M”).
- evidence (video call attestations, signatures, cryptographic proofs),
Verification MAY be challenged:

Verification MAY be challenged:

- via action challenges (to change verification rules).
- via representation challenges (if identity mapping is incorrect),
- via action challenges (to change verification rules).

All verification evidence is public unless restricted by safety rulebooks for privacy-protected payloads (e.g., face images stored abstractly).

Identity verification within governance operates on two distinct, challengeable truth-claim tracks:


Each track yields a deterministic certainty value in the range \[0,1\], derived from evidence ideas and their challenge status. These certainty values are mapped into discrete verification tiers used solely for **eligibility gating**, never for vote weighting, authority, or reputation.

Each track yields a deterministic certainty value in the range \[0,1\], derived from evidence ideas and their challenge status. These certainty values are mapped into discrete verification tiers used solely for **eligibility gating**, never for vote weighting, authority, or reputation.

Governance MUST NOT authorize or require storage of raw personally identifiable information (PII) in the canonical event log.

Verification evidence recorded canonically MUST consist only of:
- cryptographic commitments,
- attestations,
- issuer references,
- or other non-reversible proofs.

Statements elsewhere in this specification that identity evidence is “publicly viewable” SHALL be interpreted as referring to the existence and challengeability of verification claims and attestations, not disclosure of underlying private data.

### 9.3 identity quarantine [anchor: identity_quarantine]

### 9.3 identity quarantine [anchor: identity_quarantine]
Quarantine is a **state**, not an identity type.  
An identity may enter quarantine if:

- verification fails,
- contradictory evidence shows impersonation,
- multiple identities appear linked to the same human,
- governance rulebooks specify additional conditions.

Quarantine effects:

- MAY suspend voting eligibility,  
- MAY restrict challenge initiation,  
- MUST NOT prevent authorship of ideas  
  (because all ideas must still be attributable to the real identity).

Quarantine is reversible.  
Exiting quarantine requires:

- a corrective action (re-verification),
- a completion truth claim,
- or a challenge resolving the dispute.

Quarantine is a derived eligibility state applied to identities whose verification claims, evidence, or behavior are under active dispute or investigation.

Quarantine:
- does not delete or hide identity history,
- does not remove authored events from the canonical log,
- deterministically affects **eligibility pools** used for governance, challenges, voting, and other gated actions.

Eligibility under quarantine is computed during replay and MUST NOT be authored as a discretionary flag.

Systems MUST provide a user-facing diagnostic explanation indicating:
- that the identity is quarantined,
- which eligibility pools are affected,
---

---

### 9.4 identity suspension and reinstatement [anchor: identity_suspension_and_reinstatement]
Suspension only affects **governance participation**.  
A suspended identity:

- may not vote,
- may not open challenges requiring voting eligibility,
- may still create ideas, arguments, evidence, and conceptual contributions.

Suspension occurs only through:

- endorsed actionable ideas,
- action challenges that approve them,
- truth-challenge verification of the corresponding implementation.

Reinstatement follows the same flow.

There is no automatic or silent suspension.

Suspension is a derived exclusion state resulting from a completed canonical process (e.g., resolved challenge, rulebook outcome).

Suspension:
- deterministically removes the identity from defined eligibility pools,
- does not retroactively delete events or verdicts,
- is applied and removed solely through canonical replay rules.




### 9.5 identity appeals [anchor: identity_appeals]
Any identity governance decision may be appealed through:

- truth challenges (evidence disputes),
- representation challenges (identity equivalence or misassignment),
- action challenges (changing identity rules).

Appeals MUST be replay-deterministic and MUST reference the rulebook and evidence grounding the original decision.

### 9.6 identity merging and splitting [anchor: identity_merging_and_splitting]
Identity merges or splits are purely representational and use **same_as** and **representation challenges**:


Past authorship CANNOT be modified.  

Past authorship CANNOT be modified.  
The historical record remains immutable.

### 9.7 identity obligations during governance actions [anchor: identity_obligations_during_governance_actions]
Identities performing governance actions MUST:

- adhere to voluntary-action invariants,
- provide truthful completion claims,
- not falsify verification evidence,
- not create deceptive representational mappings.

Identity misconduct does not delete past contributions but may reduce voting rights through challenge outcomes.

---

Safety and governance interact in two principal ways:

Safety and governance interact in two principal ways:

1. Governance controls *which safety rulebooks and classifiers are active*.  
2. Safety constraints limit what governance may legally adopt.

This section formalizes their relationship.

### 10.1 governance authority over safety rulebooks [anchor: governance_authority_over_safety_rulebooks]

Safety rulebooks are universal rulebooks that define:

- classifier architectures,
- payload classification boundaries,
- abstraction/sanitization rules,
- jurisdiction-sensitive visibility rules,
- emotional-load scoring and normalization,
- thresholds for non-distributable payloads.

Governance MAY adopt new classifiers but MUST:

Governance MAY adopt new classifiers but MUST:

- describe them in canonical ideas,
- provide reproducible specifications,
- supply test vectors in appendices,
- schedule activation at cycle boundaries (expressed via `activation_cycle_index`), never by block height, snapshot timing, or wall-clock time.


### 10.2 limits on governance power over safety [anchor: limits_on_governance_power_over_safety]
Governance MAY NOT:

- allow globally illegal specificity to be distributed,
- override the non-distributable classification category,
- adopt opaque proprietary classifiers that cannot be audited or reproduced.
- weaken the requirement that all abstraction rules be deterministic under replay,
- adopt opaque proprietary classifiers that cannot be audited or reproduced.

These are universal invariants and cannot be superseded by rulebooks.

### 10.3 classifier governance (selection, audit, replacement) [anchor: classifier_governance_selection_audit_replacement]
Classifier models and rules are governed in three layers:

1. **Specification layer**  
   A rulebook defines what the classifier *must do*, including determinism and reproducibility.

2. **Model layer**  
   A specific classifier model (e.g., a neural model) is proposed as an actionable idea.  
   Implementation involves:
   - publishing model weights or reproducible training pipelines,
   - providing safety test suites,
   - declaring implementation and completing the action.

3. **Audit layer**  
   Auditors (human volunteers, elected via eligibility rules) perform verification actions such as:
   - re-running classifiers on the test corpus,
   - evaluating determinism,
   - ensuring no hidden behavior violates rulebook semantics.

Audit actions are recorded as completion truth claims.

### 10.4 classifier challenges [anchor: classifier_challenges]
Identities MAY challenge:

- classifier misbehavior,
- misclassification of specific payloads,
- violation of determinism,
- misuse of jurisdiction lenses,
- incorrect emotional-load scoring.

These take the form of:

- truth challenges (evidence of incorrect classification),
- representation challenges (mislabeling of rules),
- action challenges (proposals to adopt a new classifier or modify rulebooks).

### 10.5 governance constraints under safety [anchor: governance_constraints_under_safety]
Governance proposals that modify safety rules MUST include:

- demonstration that abstraction rules remain safe,
- demonstration that replay determinism is preserved,
- demonstration that abstraction rules remain safe,
- evidence that the classifier is auditable and reproducible.

Governance verdicts that violate these constraints are invalid under deterministic replay and MUST be discarded.

### 10.6 voluntary implementation of safety changes [anchor: voluntary_implementation_of_safety_changes]
Implementing a new safety rulebook or classifier requires:

- a completion truth claim including reproducible build artifacts,
- execution of the required changes,
- a completion truth claim including reproducible build artifacts,
- surviving any truth challenges.

Until the completion claim is accepted, the old safety rulebook remains active.

### 10.7 governance transparency for safety changes [anchor: governance_transparency_for_safety_changes]
All governance decisions related to safety MUST:

- show rulebook diffs,
- surface classifier version numbers,
- show rulebook diffs,
- display which ideas motivated the safety modification.

Transparency is mandatory; safety MAY NOT be used as a tool for censorship or hidden suppression.

---
Token governance governs how POD and POINT operate at the **rulebook and parameter level**—not how much any individual earns, and not the epistemic logic by which importance, truth, or action verification are determined.

Governance MAY modify token-system parameters **only** within the hard boundaries established by Protocol v5 constitutional invariants. In particular:

Governance MAY modify token-system parameters **only** within the hard boundaries established by Protocol v5 constitutional invariants. In particular:

- governance MUST NOT weight voting, truth, importance, or eligibility by POD, POINT, wealth, reputation, or tenure,
- governance MUST NOT introduce mechanisms that allow token balances to influence governance authority,
- governance MUST NOT override derived eligibility, derived `lifecycle_state`, or living-map inclusion rules.

All token-related governance changes MUST follow the standard canonical flow:

Token governance controls *rules*, not outcomes, and remains fully subordinate to Protocol v5 semantics.

Token governance controls *rules*, not outcomes, and remains fully subordinate to Protocol v5 semantics.

---

### 11.1 what governance may and may not modify [anchor: what_governance_may_and_may_not_modify]
Governance MAY update:

- the **frequency** of POD/POINT snapshot cycles,  
- the **reward windows** for completion claims relative to snapshots,  
- the **global conditions** under which POD flows into actionable ideas,  
- the **reward windows** for completion claims relative to snapshots,  
- how **POINT** may be donated, spent, or used to fund actionable ideas,  

- how **expired, abandoned, or incomplete** actionable ideas dissipate POD.

Governance MAY NOT:

- alter the **non-transferability** of POD,  
- retroactively change POD already assigned to past human actions,  
- assign POD/POINT to AI identities,  
- give POD or POINT any influence over voting weight, eligibility, or governance power,  
- assign POD/POINT to AI identities,  
- distort deterministic replay of past POD flows,  
- create new token types that function as governance weight.  

POD/POINT MUST remain **epistemic and economic**, never **political**.

- **block-height keyed snapshot schedules**, and

- **block-height keyed snapshot schedules**, and
- **snapshot tier intervals** within the snapshot ladder.

Governance MUST NOT redefine snapshots as cycle-based activation boundaries.

Additionally, governance rulebooks MUST respect derived lifecycle and eligibility constraints defined in Protocol v5.

In particular:
- burned or rotted ideas and connections MUST remain excluded from supply-affecting computations,
- governance MUST NOT indirectly re-include such objects via token, accounting, or routing rule changes.

Any governance proposal that would cause burned or rotted objects to contribute to importance propagation, POD/POINT issuance, or routing is invalid.


### 11.2 token rulebooks [anchor: token_rulebooks]
Token rulebooks are universal rulebooks that define:

- POD routing parameters (how importance rankings allocate POD to actionable ideas),  
- POINT minting parameters (how POINT is generated from POD over time),  
- spend/donate/burn rules for POINT (for funding actions),  
- expiration rules for unclaimed or incomplete actions,  
- interaction rules between POD routing and certainty bands (informative),  
- constraints on token-ledger representations for deterministic replay.

A token rulebook MAY introduce more detailed formulas or structures for these processes **as long as** they remain:

- human-auditable,  
- deterministic under replay,  
- transparent to all nodes,  
- challengeable through governance actions,  
All token issuance, routing, and accounting formulas defined by governance MUST respect living-map eligibility and derived lifecycle_state exclusions.

All token issuance, routing, and accounting formulas defined by governance MUST respect living-map eligibility and derived lifecycle_state exclusions.

Token rulebooks MUST NOT:
- count burned or rotted ideas or connections,
- bypass eligibility computation,
- or indirectly resurrect excluded objects through accounting mechanisms.

Lifecycle_state and eligibility are derived during deterministic replay and SHALL take precedence over any token-level rule definitions.


### 11.3 governance proposals affecting token rules [anchor: governance_proposals_affecting_token_rules]

A governance proposal that affects POD or POINT behavior MUST be represented as an actionable idea and MUST contain:

- a precise specification of the proposed parameter or rule change,
- a justification grounded in idea importance, system incentives, or long-term systemic health,
  - POD routing,
- an analysis of expected impacts on:
  - POD routing,
  - POINT creation rates,
  - secondary market dynamics (if applicable),
- explicit implementation instructions (configuration changes, code updates, schema changes),
- reproducible calculations, simulations, or models where quantitative effects are claimed.

The proposal lifecycle is identical to any other governance change:

1. proposal authored as an actionable idea,  
2. argumentation phase,  
3. voter eligibility selection (if required by the active rulebook),  
4. voting window,  
5. verdict issuance,  
6. voluntary implementation action by a human identity,  
7. activation at the scheduled cycle boundary (`activation_cycle_index`).

Governance proposals affecting token rules MUST NOT:

- treat snapshots, block height, or wall-clock time as activation anchors,
- introduce authored eligibility flags or token-based participation weighting,
- indirectly resurrect burned or rotted ideas or connections by counting them in routing or supply calculations.

All routing and issuance logic MUST respect derived `lifecycle_state` and living-map eligibility as defined in Protocol v5.

---

### 11.4 action verification for token changes [anchor: action_verification_for_token_changes]
Implementing a token rule change requires:

- explicit declaration by a human executor,  
- modification of token-distribution code, configuration, or parameters,  
- a completion truth claim with:

  - diff or commit hashes,  
  - reproducible calculations,  
  - updated test suites for routing/POINT generation,  
  - supporting evidence that implementation matches the approved proposal.

Completed implementations MAY be challenged, ensuring token changes cannot be silently misapplied.

### 11.5 token invariants and safety [anchor: token_invariants_and_safety]
Governance MUST uphold:

- **human-first token ownership**  
  (POD permanently tied to human-created canonical actions),

- **safety constraints**  
  (no POINT incentives may be set that encourage illegal or harmful behavior),

- **identity constraints**  
  (no POD assigned to unverifiable or impersonated identities),

- **deterministic replay**  
  (token flows must reconstruct identically across nodes),

- **no governance-weight tokens**  
  (POD and POINT cannot be used to weight votes or eligibility).

Violated invariants invalidate the governance proposal at replay time.

### 11.6 POINT funding of actionable ideas [anchor: point_funding_of_actionable_ideas]
Governance MAY adjust how POINT can fund actionable ideas, provided:

- POINT remains tradable,  
- POINT cannot influence voting,  
- funding flows are publicly visible,  
- budget allocations are challengeable,  
- funded actions must still be voluntarily executed by human identities.

Governance MAY NOT repurpose POINT into a coercive governance mechanism.

### 11.7 transparency and auditability [anchor: transparency_and_auditability]

Every change to POD or POINT rulebooks MUST:

- publish a clear, human-readable rationale,
- expose all parameter diffs relative to the prior rulebook version,
- document expected POD routing and POINT creation behavior for upcoming cycles,
- include test vectors demonstrating correct routing, exclusion, and edge-case handling,
- remain fully visible and challengeable by all eligible identities.

Nodes MUST:

- record token rulebook versions together with their activation cycle indices,
- replay POD and POINT flows deterministically from the canonical event log,
Explanations MUST account for derived exclusions, including:

Explanations MUST account for derived exclusions, including:
- burned or rotted ideas or connections,
- ineligible edges,
- safety or jurisdictional abstraction,
- identity quarantine or suspension.

Curated artifacts, summaries, export packs, or readable state bundles MUST NOT be treated as authoritative for token accounting and MUST be fully regenerable from canonical replay.


### 11.8 future token rulebook evolution [anchor: future_token_rulebook_evolution]
Token rulebooks MAY evolve as:

- new POD/POINT use cases emerge,  
- AI agents require economic constraints in the sandbox,  
- new forms of long-term project funding appear,  
- or unforeseen market behaviors need correction.

Future rulebooks MUST maintain:

- the distinction between POD (non-transferable, epistemic legitimacy)  
  and POINT (tradable, usable for funding),  
- the voluntary-action invariant,  
- the no-token-governance-power invariant,  
- deterministic replay,  
- complete transparency.

---

---
## 12. governance replay, validity, and deterministic state transitions [anchor: 12_governance_replay_validity_and_deterministic_state_transitions]

### 12.1 replay of governance proposals [anchor: replay_of_governance_proposals]

### 12.1 replay of governance proposals [anchor: replay_of_governance_proposals]
During replay, a node SHALL:

1. reconstruct every governance proposal as an actionable idea,  
2. attach all arguments, connections, and supporting ideas in event order,  
3. reconstruct voter eligibility sets from the resolved `scope_key = (scope_kind, anchor_id)` as they existed at challenge creation time,  
4. replay votes and verdicts exactly as they appear in the canonical universe,  
5. schedule rulebook activations according to the verdict instructions.

A node MUST NOT reinterpret, approximate, or substitute voter sets, quorum rules, or threshold rules. All such data is explicit in the governance rulebooks active at the time of the challenge.

### 12.2 replay of action challenges for governance proposals [anchor: replay_of_action_challenges_for_governance_proposals]
Governance action challenges follow the same replay rules as all action challenges:

- argument windows follow canonical timestamps,  
- eligibility predicates are derived from identity state and resolved `scope_key` at challenge creation,  
- the verdict event determines endorsement or rejection,  
- rejected proposals leave no governance effect,  
- endorsed proposals move to implementation.

Nodes MUST NOT infer or apply additional semantics not present in the log.

### 12.3 implementation verification and replay [anchor: implementation_verification_and_replay]

Rulebooks do **not** activate on endorsement.  
They activate only if:

1. a human identity voluntarily declares an implementation action,  
2. the same identity submits a completion truth claim,  
3. the completion claim withstands any truth challenges,  
4. the scheduled **activation cycle boundary** is reached (as specified by `activation_cycle_index` in the governance verdict).

During replay:

- if a completion truth claim is falsified by a truth challenge, activation MUST be cancelled,  
- if no valid completion claim exists before the scheduled activation cycle boundary, activation MUST fail,  
- if multiple implementations are attempted, only the successful completion(s) matter.

The canonical universe MUST NOT activate rulebooks that were not properly implemented.

Deterministic replay and governance verification operate over:
- the canonical event log,
- deterministic cycle derivation from that log,
- and any cryptographic anchoring and snapshotting mechanisms used for performance and distribution.

Blocks (hash-chained event bundles) and snapshots (including tiered snapshots) MAY be used as cryptographic anchors and replay accelerators, but MUST NOT be treated as semantic activation boundaries.

Cycle scheduling defines when a governance decision or rulebook version is eligible to become active. Where a governance effect is consequential authority under Protocol v5/Cycle semantics, replay MUST also respect cycle certification and the lagged authorization frontier before finalizing that effect. A structural `cycle_close` alone is not governance activation authority, and forced boundaries do not create missing deliberative reward or control manifests.

Replay MUST treat curated, summarized, or export-oriented artifacts as non-authoritative.

In particular:
- cycle export packs,
- readable state bundles,
- or other human-facing summaries

MUST NOT be used as inputs to governance replay, validity checking, voter eligibility derivation, or activation decisions.

All such artifacts MUST be fully regenerable from the canonical event log and any authoritative replay anchors, and MUST NOT introduce new semantics.


### 12.4 invalid governance events [anchor: invalid_governance_events]
A governance proposal is **invalid** (and MUST NOT produce state changes) if:

- it violates non-transferability of POD,  
- it assigns governance power based on wealth, stake, POD, or POINT,  
- it violates non-transferability of POD,  
- it attempts to override safety floors for globally illegal content,  
- it introduces non-human voting entities,  
- it modifies identity authorship of past ideas,  
- it breaks replay determinism.

Invalid proposals are *still recorded* as ideas and events, but state transitions they attempt MUST be discarded during replay.

Governance proposals are invalid if they attempt to:

- use cycles as governance activation boundaries,
- introduce private or restricted governance visibility,
- override or suppress required explainability for derived exclusions, including burn, rot, taint, quarantine, suspension, or safety abstraction.

Such proposals MUST be rejected during canonical evaluation and preserved as non-effective events for auditability.

### 12.5 governance forks and resolution [anchor: governance_forks_and_resolution]
Because governance is fully explicit, forks can arise only from:

- local node bugs,  
- non-deterministic classifier behavior (which is forbidden),  
- incomplete data sync.

Replay rules resolve forks by:

- applying rulebook versions exactly at their activated cycle ranges,  
- reconstructing challenge outcomes exactly as recorded,  
- ignoring actions whose completion claims are invalidated,  
- applying suppression of illegitimate state transitions.

Correct replay ALWAYS converges to a single canonical governance history.

### 12.6 rulebook lineage reconstruction [anchor: rulebook_lineage_reconstruction]
Under replay, nodes MUST reconstruct:

- rulebook version genealogy,  
- supersession chains,  
- activation windows,  
- deactivation windows,  
- any failed or abandoned rulebook proposals,  
- evidence used to justify rulebook adoption or rejection.

---

---

## 13. governance security and anti-capture design [anchor: 13_governance_security_and_anti_capture_design]

Governance must be robust against manipulation, coercion, identity fraud, rapid mobilization attacks, and subtle long-term capture attempts. The system prevents these threats not by restricting speech or preventing proposals, but by enforcing structural constraints that ensure fair, transparent, slow, and challengeable decision-making.

### 13.1 no token-weighted power, no plutocracy, no stake governance [anchor: no_token_weighted_power_no_plutocracy_no_stake_governance]
The system forbids:

- token-weighted voting,  
- stake-based voting,  
- delegation-of-votes to representatives,  
- governance privileges based on POINT, POD, wealth, reputation, or seniority.

All governance is strictly one-human-one-vote.  
This eliminates entire classes of plutocratic capture attacks.

### 13.2 voluntary-action invariant [anchor: voluntary_action_invariant]
Identities MUST NOT be compelled to:

- vote,  
- argue,  
- implement rulebooks,  
- or execute actions.

This prevents governance from being weaponized to force individuals to perform labor, implement dangerous rules, or serve as executors against their will.

### 13.3 mandatory argument windows (slow governance) [anchor: mandatory_argument_windows_slow_governance]
Governance rulebooks MAY define:

- minimum argument windows,  
- minimum number of arguments required,  
- maximum voting acceleration rate.

All challenge windows are canonical and replay-deterministic.

All challenge windows are canonical and replay-deterministic.

### 13.4 transparent identity verification and quarantine [anchor: transparent_identity_verification_and_quarantine]
Identity rulebooks ensure:

- periodic re-verification,  
- quarantine of potentially compromised identities,  
- suspension of voting rights if impersonation or fraud is detected,  
- full transparency about verification states.

This mitigates:

- botnets,  
- sybil attacks,  
- identity hijacking,  
- coordinated fraud.

All identity governance actions are challengeable.

### 13.5 No private governance, no hidden decision-making [anchor: no_private_governance_no_hidden_decision_making]

Governance occurs only through:

- public proposals,  
- public arguments,  
- public voting,  
- public implementation actions.

Tribes cannot run private governance inside the canonical universe. Tribe-local rulebooks and tribe-scoped governance processes are fully public canonical objects, visible and challengeable by any participant. They may influence only tribe-level importance rankings or internal tribe procedures and are subject to universal-scope challenges or supersession if they attempt broader effects.  
Secret committees or weighted councils cannot exist canonically.

There are no hidden levers of power.
### 13.6 challenge-based anti-capture [anchor: challenge_based_anti_capture]
If any governance outcome appears suspicious, participants may:

- open truth challenges on implementation claims,  
- open representation challenges on identity misassignments,  
- open action challenges to reverse, modify, or supersede rulebooks.

The system is designed so that:

> Capture attempts create visible anomalies in the idea graph that become fuel for counter-challenges.

Capture leaves evidence, and evidence becomes contestable argument.

### 13.7 auditability of governance decisions [anchor: auditability_of_governance_decisions]
- clear diffs,  

- clear diffs,  
- test cases,  
- reproducible reasoning,  
- references to arguments and evidence,  
- activation cycle schedules.

Anyone may audit:

- classifier changes,  
- token rulebook changes,  
- identity rule changes,  
- governance rulebook changes,  
- protocol rulebook changes.

Audit actions themselves are canonical and challengeable.

Governance proposals that modify:

- rot or burn thresholds,
- snapshot ladder tiers or intervals,
- cycle export pack selection criteria,
- or other mechanisms affecting visibility, eligibility, or routing

MUST include:
- explicit parameter definitions,
- deterministic test vectors,
These requirements exist because such changes directly alter what users see, what participates in importance propagation, and what contributes to token routing.

These requirements exist because such changes directly alter what users see, what participates in importance propagation, and what contributes to token routing.

Failure to include sufficient audit and explainability material renders the proposal invalid.


### 13.8 protection from long-term slow capture [anchor: protection_from_long_term_slow_capture]
The system allows continuous counter-governance through:

- perpetual challengeability (no decision is final forever),  
- rulebook supersession,  
- re-verification of identities,  
- Ent stewardship and long-term memory roles,  
- a transparent genealogical tree of rulebooks.

If slow capture occurs, the system surfaces it through:

- shifts in importance maps,  
- anomalous rulebook proposals,  
- contradictory evidence against executors,  
- trends in identity quarantines.

Challenges can be raised at any time to counter long-term drift.

### 13.9 anti-coercion architecture [anchor: anti_coercion_architecture]
Because governance relies on voluntary execution:

- a malicious majority cannot force a minority to perform an action,  
- a harmful rulebook cannot be activated without a voluntary human executor,  
- executors may refuse to act or publicly resign,  
- the system never binds people to unwanted responsibilities.

### 13.10 separation of epistemic, economic, and political layers [anchor: separation_of_epistemic_economic_and_political_layers]

### 13.10 separation of epistemic, economic, and political layers [anchor: separation_of_epistemic_economic_and_political_layers]
The system intentionally separates:

- epistemic weight (certainty, evidence),  
- economic weight (POINT),  
- epistemic-economic weight (POD),  
- political weight (one-human-one-vote).

This separation prevents feedback loops in which:

- epistemic elites gain political power,  
- wealthy actors gain epistemic power,  
- highly active users gain control of governance,  
- high-POD identities dominate political decisions.

No layer may bleed into another in a way that produces unequal governance authority.

### 13.11 Ents as anti-capture stabilizers (informative) [anchor: ents_as_anti_capture_stabilizers_informative]
Ents do not hold formal power, but they:

- preserve historical memory,  
- identify governance capture attempts early,  
- contextualize proposals for newcomers,  
- critique reasoning lapses,  
This creates a soft cultural defense layer without introducing ruling classes.

This creates a soft cultural defense layer without introducing ruling classes.

### 13.12 governance security under replay [anchor: governance_security_under_replay]
During replay, nodes MUST:

- discard any governance transitions that violate invariants,  
- discard any rulebook activation lacking valid implementation,  
- reconstruct accurate voter sets,  
- re-evaluate challenge outcomes exactly as recorded.

Replay enforces the true canonical governance state even if:

- malicious actors attempted manipulation,  
- a node applied invalid transitions,  
- off-chain governance mimicked canonical processes.

Replay is the final arbiter of truth for governance history.

## 14. Profile-v0 identity-admission governance boundary [anchor: profile_v0_identity_admission_governance_boundary]

Governance MUST NOT create a permanent founder, operator, institution, expert, tribe, or genesis inviter class. It MUST NOT make political agreement, viewpoint, social status, wealth, token ownership, private-account status, or AI approval a condition of ordinary Profile-v0 admission eligibility.

Within the constitutional and replay constraints, rulebooks MAY define objective eligibility thresholds, evidence diversity, maturation, capacity rates above the minimum, caps, carryover, expiration, suspension, restoration, abuse controls, and transition grace periods. They MUST NOT assign zero invitation capacity indefinitely to an otherwise inviter-eligible and unsuspended human, transfer, sell, delegate, or tokenize capacity, or use admission lineage as governance or economic authority.

An explicitly authorized emergency rule MAY freeze existing capacity prospectively. Governance MUST NOT mint emergency capacity through an operator, AI, system emitter, wall-clock process, or machine-only boundary, and MUST NOT fabricate verification, sponsor signatures, applicant proofs, sponsors, capacity debits, lineage, or Profile-v0 admission history for genesis, import, or legacy identities.

---
