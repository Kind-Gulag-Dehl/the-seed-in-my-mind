---
doc_id: tempo_spec
title: Tempo Specification
status: authoritative
version: v0
last_reviewed: 2026-01-27

scope:
  - Defines tempo limits, acceleration control, and constrained/time-repair mode behavior (legacy label: time-only mode).

authoritative_for:
  - Tempo constraints and enforcement surfaces.
  - Constrained/time-repair mode triggers, restrictions, and exit conditions.

not_authoritative_for:
  - Canonical encoding/hashing (see canonical-encoding-and-hashing-spec.md).

depends_on:
  - protocol v5.md
  - cycle-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of node-and-conformance-spec.md and governance-spec.md.

reader_path:
  - prereq: cycle-spec.md
  - next: challenge-engine-spec.md

keywords:
  - tempo
  - acceleration
  - constrained mode
  - time-repair-priority mode
  - pacing
---

> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

# Tempo Specification

*(Time Evidence, Time Claims, and Predicate Production)*

---

## 0. Purpose, Scope, and Design Invariants [anchor: 0_purpose_scope_and_design_invariants]

### 0.1 Purpose [anchor: purpose]

This specification defines **Tempo**: the subsystem responsible for representing, evaluating, and adjudicating time-related information inside the protocol.

Tempo’s sole function is to transform **challengeable, in-protocol time evidence** into **deterministic predicates and mode flags** that the Cycle Specification consumes as guardrails.

Tempo does **not** advance cycles.
Tempo does **not** authorize power.
Tempo does **not** mint economic effects.

Tempo exists to ensure that time enters the system **only as a socially contestable fact**, never as infrastructure authority.

---

### 0.2 Scope [anchor: scope]

This specification **normatively owns**:

* the definition of time anchors and time claims,
* the admissible forms of time evidence,
* the lifecycle of time claims (creation, challenge, resolution),
* certainty aggregation for time claims,
* production of cycle-level time predicates:

  * `cycle_age_ge_dmin`
  * `cycle_age_ge_dmax`
* production of tempo mode flags used for restriction and recovery.

This specification **does not own**:

* cycle progression metrics,
* cycle sealing rules,
* mana issuance or spendability rules,
* POD or POINT recomputation,
* burn/rot logic.

Those are owned by the Cycle, Token, and Lifecycle Specifications respectively.

---

### 0.3 Core Design Invariants [anchor: core_design_invariants]

The following invariants MUST hold at all times:

1. **No trusted clocks**
   Tempo MUST NOT trust wall clocks, node clocks, block timestamps, or external calendars.

2. **Time enters only as claims and evidence**
   All time information MUST be represented as challengeable claims supported by admissible evidence.

3. **Predicates, not actions**
   Tempo outputs predicates and mode flags only. It MUST NOT trigger cycle boundaries or economic effects.

4. **Deterministic replay**
   Given the same canonical event log, all honest nodes MUST compute identical Tempo outputs.

5. **Weak evidence cannot become authority**
   Passive or automatically derived evidence MUST NOT, by itself, produce high-certainty time anchors.

---

## 1. Core Concepts and Definitions [anchor: 1_core_concepts_and_definitions]

### 1.1 Time Anchor [anchor: time_anchor]

A **time anchor** is a canonical reference point against which elapsed time is evaluated.

Valid anchors include:

* a specific **cycle boundary identifier** (the canonical `cycle_close` event at block height `H_close`),
* a previously accepted **beacon-level time claim**,
* any other governance-approved canonical anchor type.

Anchors MUST be:

* globally referencable,
* immutable once created,
* replayable from the canonical log.

---

### 1.2 Time Claim [anchor: time_claim]

A **time claim** is a truth claim asserting a **bounded elapsed-time relation** relative to an anchor.

Time claims MUST be expressed as inequalities, not exact timestamps.

Canonical forms include:

* “Since anchor A, elapsed time ≥ X”
* “Since anchor A, elapsed time ≤ Y” (optional, if enabled)

Exact wall-clock timestamps (e.g., “it is now 14:32 UTC”) MUST NOT appear as claim content.

---

### 1.3 Cycle Time Predicates [anchor: cycle_time_predicates]

Tempo produces only the following **cycle-level predicates**, each with an associated certainty score:

* `cycle_age_ge_dmin`
* `cycle_age_ge_dmax`

These predicates are consumed by the Cycle Specification as guardrails.

Tempo MUST NOT produce any predicate that directly implies “seal now,” “advance cycle,” or “authorize effects.”

Predicates are evaluated against the most recent `cycle_close` anchor at block height `H_close`, as defined in the Cycle Specification.

---

### 1.4 Time Evidence [anchor: time_evidence]

**Time evidence** is any admissible information that supports or refutes a time claim.

Evidence is not truth. Evidence contributes weight toward certainty.

Tempo recognizes two evidence channels with different authority levels:

* **Voluntary attestations** (strong)
* **Passive timestamp evidence** (weak)

These channels are defined normatively in §3.

---

### 1.5 Certainty [anchor: certainty]

**Certainty** is a deterministic confidence score in the range `[0, 1]` associated with a time claim.

Certainty represents how strongly the system currently believes a claim is true, given:

* accepted evidence,
* challenge outcomes,
* identity eligibility,
* contradiction history.

Certainty is:

* monotonic only in the absence of counterevidence,
* reversible under successful challenge,
* never equivalent to truth itself.

---

### 1.6 Beacon-Level Time Claim [anchor: beacon_level_time_claim]

A **beacon** is a time claim that has achieved **high certainty** and is eligible to serve as a durable time reference.

Beacon-level claims are the **only** time objects eligible for:

* lagged legitimacy checks in the Cycle Specification,
* authorizing exit from constrained mode.

The criteria for beacon elevation are defined in §6.

---

## 2. Time Claim Types [anchor: 2_time_claim_types]

### 2.1 Guardrail Lower-Bound Claims (Elapsed ≥ X) [anchor: guardrail_lower_bound_claims_elapsed_x]

#### Definition [anchor: definition]

A **lower-bound time claim** asserts that at least `X` time has elapsed since a given anchor.

Canonical form:

```
elapsed(anchor) ≥ X
```

These claims are used to support:

* satisfaction of `cycle_age_ge_dmin`,
* satisfaction of `cycle_age_ge_dmax`.

---

#### Constraints [anchor: constraints]

* `X` MUST be a governance-defined duration unit.
* Claims MUST reference a valid anchor.
* Claims MUST NOT assert exact durations.

---

### 2.2 Guardrail Upper-Bound Claims (Elapsed ≤ Y) *(Optional)* [anchor: guardrail_upper_bound_claims_elapsed_y_optional]

#### Definition [anchor: definition_2]

An **upper-bound time claim** asserts that no more than `Y` time has elapsed since a given anchor.

Canonical form:

```
elapsed(anchor) ≤ Y
```

These claims are optional and MAY be disabled by governance.

---

#### Purpose [anchor: purpose_2]

When enabled, upper-bound claims:

* help detect implausible or malicious “too much time passed” assertions,
* provide contradiction evidence against false lower-bound claims,
* improve certainty discrimination.

Upper-bound claims MUST NOT be required for normal operation.

---

### 2.3 Anchor Consolidation Claims (Beacons) [anchor: anchor_consolidation_claims_beacons]

#### Definition [anchor: definition_3]

An **anchor consolidation claim** is a time claim explicitly intended to become a beacon.

Examples include:

* “Cycle r lasted at least Dmin”
* “Cycle r was not sealed before Dmin elapsed”

These claims:

* are evaluated like all other time claims,
* require stronger evidence and challenge survivability,
* are elevated to beacon status only when certainty thresholds are met.

---

### 2.4 Placeholder Claims [anchor: placeholder_claims]

At the start of each cycle, the system MAY create **placeholder time claims** for:

* `elapsed(cycle_start) ≥ Dmin`
* `elapsed(cycle_start) ≥ Dmax`

Placeholder claims:

* begin with zero certainty,
* carry no implicit truth,
* exist only to aggregate evidence and challenges.

They MUST NOT be auto-accepted.

---

### 2.5 Prohibited Claim Types [anchor: prohibited_claim_types]

Tempo MUST NOT accept:

* claims asserting exact timestamps,
* claims referencing node-local or external clocks directly,
* claims that directly request cycle advancement or economic effects.


## 3. Evidence Channels and Weighting [anchor: 3_evidence_channels_and_weighting]

### 3.1 Principle of Evidence Plurality [anchor: principle_of_evidence_plurality]

Tempo does not infer time from a single source.

All time certainty MUST arise from the aggregation of **multiple, independently challengeable evidence contributions**.

No single piece of evidence is sufficient to establish high certainty.

---

### 3.2 Voluntary Human Attestations (Strong Evidence) [anchor: voluntary_human_attestations_strong_evidence]

#### Definition [anchor: definition_4]

A **voluntary attestation** is an explicit human action asserting support for or opposition to a specific time claim.

Examples:

* “Yes, at least Dmin has elapsed since cycle r began.”
* “No, Dmax has not yet elapsed.”

Attestations MUST:

* reference a specific time claim,
* be cast deliberately by an eligible identity,
* be cryptographically attributable to that identity.

---

#### Weighting [anchor: weighting]

* Voluntary attestations are the **strongest evidence type**.
* Each eligible identity contributes at most one unit of attestation weight per claim.
* Attestations MAY be affirmative or negative.

---

#### Rationale [anchor: rationale]

This channel ensures that:

* time enters the system through conscious human judgment,
* collusion is visible and challengeable,
* legitimacy is socially constructed, not infrastructural.

---

### 3.3 Passive Timestamp Evidence (Weak Evidence) [anchor: passive_timestamp_evidence_weak_evidence]

#### Definition [anchor: definition_5]

**Passive timestamp evidence** consists of timestamps recorded automatically when canonical events occur.

These timestamps include:

* event creation times,
* event ordering metadata,
* block inclusion times (if applicable).

---

#### Role and Limits [anchor: role_and_limits]

Passive timestamps:

* MAY be admitted as weak evidence,
* MAY contribute to certainty aggregation,
* MUST NEVER be sufficient on their own to:

  * satisfy Dmin or Dmax,
  * elevate a claim to beacon level,
  * exit constrained mode.

---

#### Rationale [anchor: rationale_2]

Passive timestamps:

* provide weak corroboration,
* help discriminate extreme falsehoods,
* prevent total information vacuum under low participation,

but are explicitly insufficient to establish authority.

---

### 3.4 Evidence Weight Classes [anchor: evidence_weight_classes]

Evidence is divided into weight classes:

| Evidence Type         | Weight Class |
| --------------------- | ------------ |
| Voluntary attestation | Strong       |
| Challenge verdict     | Strong       |
| Passive timestamp     | Weak         |

Governance MAY tune numeric weights but MUST preserve this ordering.

---

### 3.5 Evidence Aggregation [anchor: evidence_aggregation]

Certainty for a time claim is computed as a deterministic function of:

* total affirmative weight,
* total negative weight,
* contradiction history,
* identity eligibility filters.

The aggregation function MUST:

* be deterministic,
* be monotonic only absent counterevidence,
* degrade certainty when credible contradiction appears.

---

## 4. Evidence Admissibility and Identity Constraints [anchor: 4_evidence_admissibility_and_identity_constraints]

### 4.1 Identity Eligibility [anchor: identity_eligibility]

Only **eligible identities** may contribute voluntary attestations.

Eligibility criteria are governance-defined and MAY include:

* human verification status,
* stake or reputation requirements,
* participation history.

Bots or automated agents MUST NOT contribute voluntary attestations.

---

### 4.2 One-Identity-One-Attestation Rule [anchor: one_identity_one_attestation_rule]

For any given time claim:

* each eligible identity MAY contribute at most one attestation,
* later attestations by the same identity MUST replace earlier ones.

This prevents weight amplification through repetition.

---

### 4.3 Admissibility of Passive Evidence [anchor: admissibility_of_passive_evidence]

Passive timestamp evidence is admissible only if:

* it is derived from canonical events,
* it is replayable deterministically,
* it is not selectively included or excluded.

Nodes MUST include all admissible passive timestamps uniformly.

---

### 4.4 Contradictory Evidence [anchor: contradictory_evidence]

Evidence MAY contradict existing claims.

When contradiction occurs:

* certainty MUST be recomputed,
* claims MAY lose beacon eligibility,
* downstream predicates MAY revert from true to false.

Contradiction is a normal and expected part of Tempo operation.

---

### 4.5 Fraud and Collusion Handling [anchor: fraud_and_collusion_handling]

Tempo does not attempt to detect intent.

Instead:

* coordinated false attestations lower certainty under challenge,
* contradiction accumulates evidence against claims,
* legitimacy gating in the Cycle Specification limits damage.

This preserves neutrality and determinism.

---

## 5. Time Claim Lifecycle and Challenges [anchor: 5_time_claim_lifecycle_and_challenges]

### 5.1 Claim Creation [anchor: claim_creation]

Time claims MAY be created by:

* any eligible identity,
* the system as placeholder claims (see §2.4).

All claims begin with **zero certainty**.

Creation alone carries no authority.

---

### 5.2 Evidence Accumulation Phase [anchor: evidence_accumulation_phase]

After creation:

* identities MAY submit attestations,
* passive timestamps accumulate automatically,
* claims remain open to challenge.

There is no fixed “voting window”; accumulation is continuous.

---

### 5.3 Challenge Initiation [anchor: challenge_initiation]

Any eligible identity MAY challenge a time claim.

Challenges are structured deliberations governed by the protocol’s challenge mechanism.

Challenges MAY introduce:

* counterevidence,
* expert testimony,
* cross-claim contradiction.

---

### 5.4 Challenge Resolution [anchor: challenge_resolution]

Upon challenge resolution:

* verdicts are treated as strong evidence,
* certainty is updated deterministically,
* claims MAY gain or lose beacon eligibility.

Challenge verdicts do not delete claims; they alter certainty.

---

### 5.5 Claim Finality [anchor: claim_finality]

Time claims are **never final**.

Even beacon-level claims:

* MAY be challenged later,
* MAY lose certainty,
* MAY be superseded by better anchors.

Finality is replaced by durability under continued scrutiny.

---

### 5.6 Predicate Production [anchor: predicate_production]

Tempo MUST evaluate all relevant time claims and output:

* `cycle_age_ge_dmin`
* `cycle_age_ge_dmax`

Tempo predicates constrain cycle advancement but have no authority over snapshot boundaries or emission scheduling, which remain block-height deterministic per snapshot-format-v0.md.

Predicates are true only if:

* at least one supporting claim exceeds the certainty threshold,
* no contradictory claim with sufficient certainty exists.

---

### 5.7 Failure Modes [anchor: failure_modes]

If no claim reaches sufficient certainty:

* predicates remain false,
* the system may enter or remain in constrained mode,
* cycle sealing behavior is determined by the Cycle Specification.

Tempo never forces advancement.


## 6. Beacon Elevation, Persistence, and Decay [anchor: 6_beacon_elevation_persistence_and_decay]

### 6.1 Purpose of Beacons [anchor: purpose_of_beacons]

Beacon-level time claims exist to provide **durable, high-confidence temporal anchors** that can be safely consumed by other parts of the protocol, especially for **lagged legitimacy checks**.

Beacons are not absolute truth. They are the system’s strongest available time references under continuous challengeability.

---

### 6.2 Beacon Eligibility Criteria [anchor: beacon_eligibility_criteria]

A time claim MAY be elevated to **beacon status** if and only if all of the following conditions are met:

1. **Certainty Threshold Met**
   The claim’s certainty MUST exceed a governance-defined beacon threshold `T_beacon`.

2. **Diversity of Support**
   Certainty MUST be supported by attestations from a minimum number of distinct eligible identities.

3. **Challenge Survivability**
   The claim MUST have survived at least one completed challenge cycle OR remained unchallenged for a governance-defined duration.

4. **No Strong Contradiction**
   There MUST NOT exist any contradictory claim with certainty above a defined rejection threshold.

---

### 6.3 Beacon Elevation Process [anchor: beacon_elevation_process]

Beacon elevation is **deterministic and automatic**.

When eligibility criteria are met:

* the claim is marked as a beacon,
* its anchor identifier becomes admissible for legitimacy checks,
* its elevation is recorded in the canonical log.

No human action is required to “approve” a beacon beyond normal attestations and challenges.

---

### 6.4 Beacon Persistence [anchor: beacon_persistence]

Once elevated:

* a beacon persists across cycles,
* it MAY be referenced by future claims,
* it MAY be used in lagged legitimacy checks.

Beacon status does **not** imply permanence.

---

### 6.5 Beacon Decay and Revocation [anchor: beacon_decay_and_revocation]

Beacon status MUST be revoked if:

* credible contradictory evidence accumulates,
* a successful challenge materially reduces certainty below `T_beacon`,
* governance-defined decay rules apply (optional).

Upon revocation:

* the claim reverts to a normal time claim,
* downstream predicates MAY change,
* legitimacy gating MAY re-enter constrained mode.

No retroactive erasure occurs.

---

### 6.6 Multiple Beacons and Conflict [anchor: multiple_beacons_and_conflict]

Multiple beacon claims MAY coexist.

If conflicting beacons exist:

* both remain visible,
* certainty aggregation resolves their relative strength,
* downstream consumers MUST rely only on predicates, not raw beacons.

Tempo never selects a “winner” arbitrarily.

---










## 7. Evidence Rate Limits and Tempo Mana [anchor: 7_evidence_rate_limits_and_tempo_mana]

### 7.1 Purpose [anchor: purpose_3]

Tempo requires explicit rate limiting to prevent temporal legitimacy from being manufactured through volume, coordination, or automation.

This section defines **tempo mana**, a dedicated, non-transferable capacity that limits how frequently an identity may contribute voluntary time attestations.

Tempo mana exists solely to protect the integrity of time certainty.

---

### 7.2 Tempo Mana Definition [anchor: tempo_mana_definition]

Each eligible identity has a **tempo mana pool**.

Tempo mana:

* is separate from all other mana types,
* cannot be transferred, delegated, or stored,
* exists only to rate-limit voluntary time attestations,
* recharges slowly and deterministically.

Tempo mana is **not** a reward and confers no advantage beyond participation.

---

### 7.3 Tempo Mana Costs [anchor: tempo_mana_costs]

Each voluntary attestation to a time claim consumes tempo mana.

Rules:

* An identity MUST have sufficient tempo mana to submit an attestation.
* Attestations submitted without sufficient tempo mana MUST be rejected.
* Replacing a previous attestation counts as a new attestation and consumes tempo mana again.

Governance defines:

* tempo mana capacity,
* tempo mana recharge rate,
* minimum recharge delay between attestations.

---

### 7.4 Rationale [anchor: rationale_3]

Tempo mana ensures that:

* no identity can flood time claims,
* coordination costs scale with the certainty sought,
* low-participation systems remain safe,
* time legitimacy grows slowly and visibly.

---

## 8. Passive Timestamp Evidence: Caps and Outlier Rejection [anchor: 8_passive_timestamp_evidence_caps_and_outlier_rejection]

### 8.1 Passive Evidence Ceiling [anchor: passive_evidence_ceiling]

Passive timestamp evidence MUST be subject to a **hard influence ceiling**.

Rules:

* Passive timestamps may contribute to certainty only up to a fixed maximum fraction `P_max`.
* `P_max` MUST be strictly less than the minimum certainty required for:

  * predicate truth (`T_allow`),
  * beacon elevation (`T_beacon`).

Passive evidence MUST NEVER, by itself, cause:

* `cycle_age_ge_dmin == true`,
* `cycle_age_ge_dmax == true`,
* beacon elevation.

---

### 8.2 Deterministic Outlier Rejection [anchor: deterministic_outlier_rejection]

Passive timestamps MUST be filtered deterministically.

Outlier rejection rules MAY include:

* rejecting timestamps outside governance-defined plausibility bounds,
* trimming extreme quantiles,
* rejecting isolated spikes unsupported by other evidence.

All nodes MUST apply identical outlier rules during replay.

---

### 8.3 Rationale [anchor: rationale_4]

These rules ensure that:

* infrastructure does not become time authority,
* botnets cannot fabricate elapsed time,
* partitions do not silently dominate certainty.

Passive evidence remains **contextual**, never decisive.

---

## 9. Certainty Thresholds and Predicate Semantics [anchor: 9_certainty_thresholds_and_predicate_semantics]

### 9.1 Dual Threshold Requirement [anchor: dual_threshold_requirement]

Tempo MUST define **two distinct certainty thresholds**:

1. **Predicate Threshold (`T_allow`)**
   The minimum certainty required for a time predicate to evaluate as `true`.

2. **Beacon Threshold (`T_beacon`)**
   A strictly higher certainty required for beacon elevation.

It MUST always hold that:

```
T_allow < T_beacon
```

---

### 9.2 Predicate Truth Rules [anchor: predicate_truth_rules]

A predicate (e.g. `cycle_age_ge_dmin`) MAY evaluate to `true` if and only if:

* at least one supporting claim has certainty ≥ `T_allow`,
* no contradictory claim has certainty ≥ `T_allow`,
* passive evidence ceilings are respected.

Predicate truth does **not** imply durability, legitimacy, or authorization.

---

### 9.3 Anti-Stall Minimum Evidence Floor (Dmax Only) [anchor: anti_stall_minimum_evidence_floor_dmax_only]

For `cycle_age_ge_dmax` predicates only, the following exception applies:

If:

* passive evidence exceeds a governance-defined plausibility minimum,
* attestations exist from at least a minimal number of distinct eligible identities,
* no strong contradictory claim exists,

then `cycle_age_ge_dmax` MAY reach `T_allow` **without** reaching `T_beacon`.

This rule exists solely to prevent permanent stall.

It MUST NOT apply to:

* `cycle_age_ge_dmin`,
* beacon elevation,
* legitimacy authorization.

---

### 9.4 Rationale [anchor: rationale_5]

This separation ensures that:

* cycles can advance structurally under collapse,
* power cannot be extracted without high-certainty time,
* liveness and safety are decoupled.

---

## 10. Tempo Modes and System Signaling [anchor: 10_tempo_modes_and_system_signaling]

### 10.1 Mode Taxonomy [anchor: mode_taxonomy]

Tempo MUST expose the following mode flags:

1. **Normal Tempo Mode**
2. **Constrained Tempo Mode**
3. **Time-Repair-Priority Mode**

Modes are diagnostic signals consumed by downstream systems.

---

### 10.2 Normal Tempo Mode [anchor: normal_tempo_mode]

Entered when:

* sufficient recent beacon-level claims exist,
* time predicates are stable,
* contradiction density is low.

Downstream systems MAY operate normally.

---

### 10.3 Constrained Tempo Mode [anchor: constrained_tempo_mode]

Entered when:

* predicates exist but lack beacon support,
* contradictions remain unresolved,
* legitimacy lags structural progression.

Downstream systems SHOULD restrict power and irreversible actions.

---

### 10.4 Time-Repair-Priority Mode [anchor: time_repair_priority_mode]

Entered when:

* beacon coverage is absent for multiple cycles,
* contradictions block predicate stability,
* time legitimacy is severely degraded.

In this mode:

* time attestations and challenges SHOULD be prioritized,
* non-essential actions SHOULD be further restricted,
* recovery actions MUST remain available.

---

### 10.5 Determinism [anchor: determinism]

Mode transitions MUST be:

* deterministic,
* replayable,
* derived solely from canonical data.

---

## 11. Failure Scenarios and Partition Handling [anchor: 11_failure_scenarios_and_partition_handling]

### 11.1 Coordinated Attestation Attacks [anchor: coordinated_attestation_attacks]

If coordinated false attestations occur:

* tempo mana limits bound their influence,
* contradictions reduce certainty,
* predicates may temporarily reach `T_allow`,
* beacon elevation will fail,
* downstream legitimacy gating prevents value extraction.

---

### 11.2 Catastrophic Participation Loss [anchor: catastrophic_participation_loss]

Under sudden participation collapse:

* passive evidence may support Dmax predicates,
* forced cycles may occur (Cycle Specification),
* tempo remains constrained or degraded,
* recovery depends on surviving identities rebuilding beacons.

The system MUST remain live.

---

### 11.3 Network Partition and Merge [anchor: network_partition_and_merge]

During partitions:

* each partition accumulates independent time evidence,
* no partition gains authority over another.

Upon merge:

* all claims coexist,
* contradictions are evaluated deterministically,
* certainty recomputes without arbitration.

No partition automatically overrides another.

---

## 12. Determinism, Replay, and Auditability (Extended) [anchor: 12_determinism_replay_and_auditability_extended]

### 12.1 Replay Completeness [anchor: replay_completeness]

The canonical log MUST be sufficient to reconstruct:

* all time claims,
* all attestations and challenges,
* certainty values over time,
* mode transitions.

---

### 12.2 External Auditability [anchor: external_auditability]

An external auditor MUST be able to determine:

* why a predicate was true or false at a given cycle,
* why a beacon was or was not elevated,
* why legitimacy was restricted or restored.

Opacity is a protocol failure.

---

### 12.3 Invariant Restatement [anchor: invariant_restatement]

Tempo MUST NEVER:

* advance cycles,
* authorize economic or governance power,
* trust clocks or infrastructure,
* create irreversibility.

Tempo exists only to **measure contested time** and expose it safely.


## 13. Interaction with the Cycle Specification (Normative Bridge) [anchor: 13_interaction_with_the_cycle_specification_normative_bridge]

### 13.1 Non-Authority Clause [anchor: non_authority_clause]

Tempo outputs **informational predicates and mode flags only**.

Under no circumstances may any Tempo output—including time predicates, beacon-level claims, or tempo mode flags—be interpreted as authorization to:

* seal a cycle,
* advance a cycle,
* mint, revoke, or unlock mana,
* authorize governance effects,
* bypass Cycle-owned requirements.

All such actions remain exclusively governed by the Cycle Specification.

Cycle closure validity is determined only by Cycle-owned closure rules. Tempo’s role is limited to certifying predicate truth values consumed by Cycle (including `cycle_age_ge_dmin` and `cycle_age_ge_dmax`) and never extends to boundary emission authority.

---

### 13.2 Lagged Consumption Requirement [anchor: lagged_consumption_requirement]

The Cycle Specification MUST consume Tempo predicates and beacon references only under **lagged evaluation rules**, such that:

* predicates derived during cycle `r` MAY influence behavior no earlier than cycle `r+1`,
* beacon elevation MUST precede any legitimacy-granting effect by a governance-defined delay.

Tempo outputs MUST NOT be consumed within the same cycle in which they are derived for legitimacy or authorization purposes.

---

### 13.3 Failure-Safe Bias [anchor: failure_safe_bias]

If Tempo outputs are ambiguous, contradictory, unstable, or below certainty thresholds:

* downstream systems MUST bias toward restriction rather than progression,
* lack of time certainty MUST NOT be interpreted as permission to accelerate or finalize effects.

---

## 14. Anti-Acceleration and Anti-Normalization Guarantees [anchor: 14_anti_acceleration_and_anti_normalization_guarantees]

### 14.1 No Self-Normalizing Time [anchor: no_self_normalizing_time]

Tempo MUST NOT infer acceptable tempo from recent system behavior.

Specifically:

* shorter recent cycles,
* increased event volume,
* faster convergence of claims

MUST NOT, by themselves, alter acceptable elapsed-time expectations.

Only explicit governance changes to tempo parameters may redefine acceptable tempo.

---

### 14.2 Prediction Non-Authority [anchor: prediction_non_authority]

Predictions, expectations, or forecasts about when cycles should complete:

* MAY exist as ordinary truth claims elsewhere in the protocol,
* MUST NOT be admissible as time evidence,
* MUST NOT influence Tempo certainty aggregation.

Tempo evaluates observed elapsed time only.

---

### 14.3 Automation Asymmetry Invariant [anchor: automation_asymmetry_invariant]

Tempo MUST preserve the following invariant:

> Increased speed or coordination MUST NOT increase authority.

Accordingly:

* high-volume attestations do not accelerate certainty,
* rapid convergence does not bypass contradiction handling,
* automation gains no structural advantage over slow, diverse participation.

---

## 15. Low-Participation and Survivor Scenarios [anchor: 15_low_participation_and_survivor_scenarios]

### 15.1 Liveness Under Participation Collapse [anchor: liveness_under_participation_collapse]

If eligible participation drops sharply:

* Tempo MUST continue to evaluate time predicates using admissible evidence,
* passive evidence MAY contribute within strict ceilings,
* `cycle_age_ge_dmax` MAY satisfy `T_allow` without beacon elevation as defined elsewhere.

Tempo MUST NOT permanently stall the system.

---

### 15.2 Survivor Non-Capture Guarantee [anchor: survivor_non_capture_guarantee]

If a small number of identities remain active:

* they MAY accumulate sufficient certainty to form new beacons only when no larger contradicting population exists,
* the same identities MUST NOT be able to do so while a broader active population remains.

This guarantee MUST arise solely from:

* diversity requirements,
* contradiction aggregation,
* eligibility constraints.

No special survivor privileges are permitted.

---

### 15.3 No Retroactive Legitimization [anchor: no_retroactive_legitimization]

Time claims or beacons formed during degraded or low-participation periods:

* MUST NOT retroactively legitimize prior actions,
* MAY only support forward progress.

---

## 16. Explicit Non-Goals and Prohibited Interpretations [anchor: 16_explicit_non_goals_and_prohibited_interpretations]

### 16.1 Non-Goals [anchor: non_goals]

Tempo MUST NOT be interpreted as:

* a trusted clock,
* a scheduler,
* an emergency override,
* a governance authority,
* or a substitute for human judgment.

---

### 16.2 Limits of Protection [anchor: limits_of_protection]

Tempo does not claim to prevent:

* total identity capture,
* universal coercion,
* or compromised governance authority.

Tempo exists to preserve auditability, contestability, and human-reactable pacing.

---

## 17. Invariant Summary [anchor: 17_invariant_summary]

The following invariants are mandatory:

1. Time enters the system only as contestable claims.
2. Uncertainty slows the system.
3. Speed never grants authority.
4. Deterministic replay is never violated to restore liveness.
5. Tempo guards legitimacy conditions, not outcomes.




