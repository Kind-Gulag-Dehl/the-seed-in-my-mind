---
doc_id: cycle_spec
title: Cycle Specification
status: authoritative
version: v0
last_reviewed: 2026-01-27

scope:
  - Defines cycle derivation and any mechanics gated by cycles.

authoritative_for:
  - Cycle boundaries and derivation rules.
  - Which behaviors are cycle-gated across the system.

not_authoritative_for:
  - Tempo enforcement and time-only mode (see tempo-spec.md).

depends_on:
  - protocol v5.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of tempo-spec.md and challenge-engine-spec.md.

reader_path:
  - prereq: node-and-conformance-spec.md
  - next: tempo-spec.md

keywords:
  - cycles
  - pacing
  - derivation
  - gating
---

# Cycle Specification

## 0. Purpose, Scope, and Design Invariants [anchor: 0_purpose_scope_and_design_invariants]

### 0.1 Purpose [anchor: purpose]

This specification defines **cycles** as the protocol’s primary coordination and recomputation mechanism. Cycles provide a deterministic, replayable structure for pacing collective deliberation, adapting to participation changes, and synchronizing derived state such as mana availability, economic recomputation, and governance effects.

Cycles are not units of time, not blocks, and not rewards epochs. They are **semantic batch boundaries** in the canonical event log.

The design goal of the cycle system is to:

* pace collective reasoning without trusting clocks,
* prevent both acceleration and permanent stall,
* adapt automatically to growth, collapse, and partitions,
* allow structural progress to continue while legitimacy and power lag behind verification,
* provide deterministic recomputation points for all derived state.

---

### 0.2 Scope [anchor: scope]

This specification **normatively owns**:

* the definition of a cycle and cycle boundary,
* how cycles advance under normal and degraded conditions,
* the non-time metrics used to determine normal cycle completion,
* forced (liveness) cycle advancement,
* what happens at cycle boundaries,
* how cycle boundaries interact with mana, POD, and POINT,
* how lagged time legitimacy gates power without blocking liveness.

This specification **consumes**, but does not define:

* time predicates and tempo modes (from the Tempo Specification),
* the internal calculation of importance and POD routing (from the Token Specification),
* burn/rot eligibility rules (from the Lifecycle/Burn Specification).

This specification is the **single authoritative source** for when and how cycles advance and what cycle advancement means.

Cycles provide semantic pacing and coordination boundaries but do not define or trigger snapshot emission. Snapshot creation remains governed exclusively by block-height intervals in snapshot-format-v0.md.

---

### 0.3 Core Design Invariants [anchor: core_design_invariants]

The following invariants MUST hold at all times:

1. **Cycles are not time units**
   Cycles do not represent hours, days, or any external temporal measure.

2. **Time does not advance cycles**
   Time is used only as a guardrail to prevent cycles from closing too fast or stalling forever.

3. **Deliberation advances cycles**
   Normal cycle completion is driven by completed human deliberation, not raw activity.

4. **Boundary creation is separate from boundary consequences**
   A cycle boundary may be created before legitimacy is fully established; power and rewards may not.

5. **The system must never permanently stall**
   Under any participation regime, the system must be able to advance cycle boundaries.

6. **The system must never silently accelerate**
   No mechanism may allow sustained cycle advancement to directly mint economic or governance power without lagged, challengeable legitimacy.

---

## 1. Core Concepts and Definitions [anchor: 1_core_concepts_and_definitions]

### 1.1 Cycle [anchor: cycle]

A **cycle** is the interval between two canonical cycle boundaries in the deterministic event log.

Within a cycle:

* canonical events accumulate,
* deliberation occurs (ideas, challenges, arguments, votes),
* no derived state is finalized.

A cycle has no intrinsic duration and no intrinsic value; it is purely a coordination interval.

---

### 1.2 Cycle Boundary [anchor: cycle_boundary]

A **cycle boundary** is a deterministic marker in the canonical log that indicates the end of one cycle and the start of the next.

Cycle boundaries are:

* replayable from the canonical log,
* identical for all honest nodes,
* classified by how they were produced (see §1.3).

A cycle boundary is not itself a truth claim, reward event, or governance decision.

---

### 1.2.1 Canonical Boundary Event (`cycle_close`) [anchor: canonical_boundary_event_cycle_close]

Each cycle boundary MUST be recorded as a canonical `cycle_close` event anchored to a specific block height `H_close`.

`cycle_close` is emitted automatically by deterministic protocol logic when closure predicates are satisfied. Human identities and operators MUST NOT manually submit `cycle_close`.

The event author MUST be the reserved non-human canonical identity `system_boundary_emitter` defined by Protocol v5. `system_boundary_emitter` is restricted to mechanically verifiable boundary events and has no voting or governance authority.

The `cycle_close` event is the sole authoritative marker that:

* ends the current cycle,
* begins the next cycle,
* binds all boundary recomputation to block height `H_close`.

All cycle-based recomputations MUST occur at `H_close`, including at least:

* rank recomputation,
* POD recomputation,
* POINT mint/redistribution/melt,
* mana recharge,
* eligibility rollovers.

`cycle_close` records structural closure only; legitimacy gating still controls whether downstream effects are finalized.

For cycle `r`, a `cycle_close` event is valid only at the earliest canonical log position `p` where:

```
cycle_age_ge_dmin == true
AND (event_target_reached == true OR cycle_age_ge_dmax == true)
```

where:

```
event_target_reached := (W >= W_target)
```

Any later `cycle_close` for the same cycle is invalid and MUST be deterministically rejected.

---

### 1.3 Types of Cycle Boundaries [anchor: types_of_cycle_boundaries]

There are two types of cycle boundaries:

#### 1.3.1 Deliberative Boundary (Normal Seal) [anchor: deliberative_boundary_normal_seal]

A **deliberative boundary** is created when the system determines that sufficient collective deliberation has been completed during the cycle and that minimum-time guardrails are satisfied.

This represents the normal, healthy progression of the system.

#### 1.3.2 Forced Boundary (Liveness Seal) [anchor: forced_boundary_liveness_seal]

A **forced boundary** is created when maximum-time guardrails are satisfied but deliberative completion is not possible, typically due to participation collapse, partition, or extreme inactivity.

Forced boundaries exist to guarantee liveness and adaptation. They are explicitly marked and carry restricted consequences.

---

### 1.4 Boundary Creation vs Boundary Consequences [anchor: boundary_creation_vs_boundary_consequences]

This distinction is fundamental.

* **Boundary creation**: advancing the cycle index and ending the current cycle.
* **Boundary consequences**: recomputation of derived state and authorization of power.

Boundary creation MAY occur without legitimacy.
Boundary consequences MUST NOT authorize economic or governance power unless legitimacy conditions are met.

---

### 1.5 Legitimacy Lag [anchor: legitimacy_lag]

The system explicitly allows **structure to advance faster than legitimacy**.

Legitimacy (especially time legitimacy) is established through slow, challengeable processes and may lag behind cycle boundaries by a fixed number of cycles.

This lag is intentional and is used to prevent acceleration attacks while preserving liveness.

---

## 2. Cycle Progression Metrics (Non-Time) [anchor: 2_cycle_progression_metrics_non_time]

Normal cycle progression is determined exclusively by **deliberative metrics**, not time.

Time predicates only gate or force closure; they never measure progress.

---

### 2.1 Distinct Human Participation (V) [anchor: distinct_human_participation_v]

#### Definition [anchor: definition]

`V` is the number of **distinct eligible human identities** that participated meaningfully during the cycle.

#### Counting Rules [anchor: counting_rules]

* Each identity may contribute at most **one unit** to `V` per cycle.
* Participation is primarily defined as **casting at least one vote** in an eligible challenge.
* Creating ideas, arguments, or evidence alone does not count toward `V`.

#### Rationale [anchor: rationale]

This metric exists to:

* ensure breadth of human involvement,
* prevent automation or spam from accelerating cycles,
* cause cycles to naturally slow when participation drops.

`V` scales directly with the size of the active community and is insensitive to activity volume per user.

---

### 2.2 Completed Deliberations (C) [anchor: completed_deliberations_c]

#### Definition [anchor: definition_2]

`C` is the number of **challenges that reached a final verdict** during the cycle.

#### Counting Rules [anchor: counting_rules_2]

* A challenge counts once, at the moment its verdict is finalized.
* Partial progress, open argument phases, or abandoned challenges do not count.

#### Rationale [anchor: rationale_2]

This metric exists to:

* measure completed reasoning rather than activity,
* incentivize resolution and closure,
* prevent cycles from advancing due to endless debate or unchecked creation.

`C` captures epistemic progress independent of community size.

---

### 2.3 Work Score (W) [anchor: work_score_w]

#### Definition [anchor: definition_3]

The **work score** for a cycle is defined as:

```
W = V + C
```

#### Rationale [anchor: rationale_3]

This combination is intentional:

* `V` provides legitimacy through participation breadth.
* `C` provides epistemic progress through resolved deliberation.

Together, `W` represents the minimum sufficient statistic for “enough collective work has been done to justify advancing the system.”

No other activity types contribute to `W`.

---

### 2.4 Adaptive Work Target (W_target) [anchor: adaptive_work_target_w_target]

#### Definition [anchor: definition_4]

`W_target` is the required work score for a cycle to close normally via a deliberative boundary.

#### Computation [anchor: computation]

* `W_target` is computed from recent cycle history (e.g., via an exponential moving average).
* It is recalculated only at cycle boundaries.
* It is clamped to governance-defined minimum and maximum bounds.

#### Rationale [anchor: rationale_4]

The adaptive target allows the system to:

* automatically scale with growth,
* recover after catastrophic participation drops,
* avoid fixed assumptions about community size,
* converge to stable pacing without manual intervention.

After a collapse, `W_target` will decrease within one or two cycles, allowing normal progression to resume for a smaller community.

---

## 3. Time Bounds (Guardrails, Not Progress Metrics) [anchor: 3_time_bounds_guardrails_not_progress_metrics]

### 3.1 Purpose of Time Bounds [anchor: purpose_of_time_bounds]

Time bounds exist solely to constrain cycle closure. They do **not** measure progress, reward activity, or imply correctness.

Time bounds serve two functions:

1. **Acceleration prevention**
   Ensure cycles cannot close faster than a minimum acceptable pace.

2. **Liveness enforcement**
   Ensure cycles cannot remain open indefinitely when deliberative completion becomes impossible.

Time bounds never *advance* a cycle by themselves. They only **permit** or **force** boundary creation when combined with the rules in §§4–5.

---

### 3.2 Minimum Time Bound (Dmin) [anchor: minimum_time_bound_dmin]

#### Definition [anchor: definition_5]

`Dmin` is a protocol-defined minimum elapsed time that must pass after a cycle boundary before the current cycle may close *normally*.

#### Rule [anchor: rule]

A cycle MUST NOT close via a deliberative boundary unless:

```
cycle_age_ge_dmin == true
```

where `cycle_age_ge_dmin` is a boolean predicate produced by the Tempo Specification with sufficient certainty.

#### Rationale [anchor: rationale_5]

* Prevents rapid-fire cycles driven by automation or coordination attacks.
* Ensures each cycle represents a meaningful deliberative window.
* Forces time to act as a *brake*, not an accelerator.

---

### 3.3 Maximum Time Bound (Dmax) [anchor: maximum_time_bound_dmax]

#### Definition [anchor: definition_6]

`Dmax` is a protocol-defined maximum acceptable elapsed time that a cycle may remain open.

#### Rule [anchor: rule_2]

A cycle MUST close if:

```
cycle_age_ge_dmax == true
```

regardless of whether deliberative completion criteria are met.

#### Rationale [anchor: rationale_6]

* Guarantees liveness under collapse, partition, or inactivity.
* Prevents permanent deadlock.
* Allows the system to adapt its targets after shocks.

---

### 3.4 Source of Time Predicates [anchor: source_of_time_predicates]

This specification does not compute time directly.

All time-related inputs MUST be consumed exclusively from the **Tempo Specification**, which produces:

* `cycle_age_ge_dmin`
* `cycle_age_ge_dmax`
* tempo mode flags (e.g., constrained mode)

#### Explicit Prohibitions [anchor: explicit_prohibitions]

Implementations MUST NOT:

* use local system clocks,
* use raw timestamps directly,
* infer elapsed time outside of Tempo predicates,
* treat timestamps as authoritative truth.

---

### 3.5 Certainty Requirements [anchor: certainty_requirements]

* Time predicates are boolean but have associated certainty scores.
* Governance defines certainty thresholds for predicates to be considered `true`.
* This specification treats predicates as `false` until thresholds are met.

---

### 3.6 Time Bounds and Legitimacy [anchor: time_bounds_and_legitimacy]

Satisfying `cycle_age_ge_dmin` or `cycle_age_ge_dmax`:

* permits or forces **boundary creation**,
* does **not** by itself authorize:

  * mana spendability,
  * POD recomputation,
  * POINT distribution,
  * governance changes.

Legitimacy gating is defined in §7.

---

## 4. Normal Cycle Closure (Deliberative Seal) [anchor: 4_normal_cycle_closure_deliberative_seal]

### 4.1 Definition [anchor: definition_7]

A **normal cycle closure**, also called a **deliberative boundary**, represents healthy system operation in which sufficient deliberative work has occurred within acceptable time bounds.

A deliberative boundary is a structural closure event. It does not, by itself, authorize economic, governance, or other irreversible effects.

---

### 4.2 Closure Conditions [anchor: closure_conditions]

A cycle MUST close via a deliberative boundary at the earliest canonical log position where **all** of the following conditions are met:

1. **Sufficient Work**

W ≥ W_target

where `W_target` is the adaptive target defined elsewhere in this specification.

2. **Minimum Time Satisfied**

cycle_age_ge_dmin == true

3. **Earliest-Valid Boundary Rule**

No valid `cycle_close` for the same cycle has occurred earlier in canonical log order.

If any condition is false, a deliberative boundary MUST NOT occur at that log position.

---

### 4.3 Properties of a Deliberative Boundary [anchor: properties_of_a_deliberative_boundary]

A deliberative boundary:

* indicates sufficient collective reasoning has completed,
* is considered a **legitimate structural progression**,
* is eligible (subject to §7) to authorize downstream effects,
* is distinguishable from forced boundaries in the canonical log.

---

### 4.4 Effects of a Deliberative Boundary (Structural Only) [anchor: effects_of_a_deliberative_boundary_structural_only]

Creation of a deliberative boundary:

* ends the current cycle,
* increments the cycle index,
* triggers boundary recomputation hooks (see §8–§10),
* records participation metrics for adaptive targeting.

It does **not** by itself guarantee that all boundary consequences are authorized; legitimacy gating still applies.

---

### 4.5 Failure to Reach Deliberative Closure [anchor: failure_to_reach_deliberative_closure]

If `W < W_target`:

* the cycle remains open,
* deliberation may continue,
* time bounds continue to be evaluated.

Low participation is expected to manifest as reduced realized work `W`. Participation metrics (e.g., `V`) MAY be tracked for diagnostics and adaptive smoothing, but MUST NOT be used as a closure gate.

If the cycle remains open until `cycle_age_ge_dmax == true`, forced closure rules apply (see §5).


### 4.6 Design Guarantees [anchor: design_guarantees]

This closure mechanism guarantees that:

* cycles reflect completed reasoning, not activity volume,
* the system naturally slows when participation drops,
* growth and collapse are handled symmetrically,
* no single actor or small group can advance cycles under normal conditions.

### 4.7 Participation Metrics Are Non-Gating [anchor: participation_metrics_are_non_gating]

The system MAY track participation metrics (e.g., number of distinct eligible identities that produced activity during a cycle) for:

* diagnostics,
* transparency and monitoring,
* adaptive smoothing inputs.

Participation metrics MUST NOT be used as a hard closure requirement and MUST NOT prevent a cycle from closing if all other closure conditions are satisfied.

No minimum participation threshold is required for correctness. In extreme collapse scenarios, the system MUST remain operable even if participation drops to a single surviving identity.


## 5. Forced Cycle Closure (Liveness Seal) [anchor: 5_forced_cycle_closure_liveness_seal]

### 5.1 Purpose [anchor: purpose_2]

Forced cycle closure exists to guarantee **liveness**.

It ensures that the system can advance cycle boundaries even when normal deliberative completion becomes impossible due to:

* catastrophic participation loss,
* prolonged inactivity,
* network partitions,
* adversarial stalling behavior.

Forced closure prioritizes **structural continuity** over legitimacy and power.

---

### 5.2 Trigger Condition [anchor: trigger_condition]

A cycle MUST close via a forced boundary at the earliest canonical log position when:

```
cycle_age_ge_dmin == true
AND
cycle_age_ge_dmax == true
AND
event_target_reached == false
```

This requirement applies regardless of participation level and any minimum participation floors.

Any later forced `cycle_close` for the same cycle is invalid and MUST be deterministically rejected.

---

### 5.3 Properties of a Forced Boundary [anchor: properties_of_a_forced_boundary]

A forced boundary:

* is explicitly marked as **forced** in the canonical log,
* advances the cycle index deterministically,
* exists solely to allow the system to adapt and recover,
* does **not** imply sufficient deliberation occurred.

Forced boundaries MUST be distinguishable from deliberative boundaries during replay.

---

### 5.4 Prohibited Consequences of Forced Closure [anchor: prohibited_consequences_of_forced_closure]

A forced boundary MUST NOT, by itself, authorize:

* unrestricted mana spendability,
* POD recomputation finalization,
* POINT minting or distribution,
* irreversible governance effects,
* burn/rot execution with permanent effect.

These consequences are gated by legitimacy rules defined in §7.

---

### 5.5 Allowed Consequences of Forced Closure [anchor: allowed_consequences_of_forced_closure]

A forced boundary MAY:

* terminate the current cycle,
* create the next cycle,
* recompute adaptive targets (`W_target`) using the completed (forced) cycle’s metrics,
* update internal participation baselines used for diagnostics and adaptive smoothing,
* allow the system to re-enter normal closure behavior after adjustment.

Forced closure MUST NOT mint authority. It exists to preserve liveness and to allow adaptive targets to recalibrate under collapse conditions.


### 5.6 Design Rationale [anchor: design_rationale]

Forced closure is intentionally **weak**.

It allows the system to:

* escape deadlock,
* recalibrate expectations after collapse,
* continue producing deterministic structure,

without granting unearned power to remaining participants.

---

## 6. Internal Anti-Stall Advancement [anchor: 6_internal_anti_stall_advancement]

### 6.1 Purpose [anchor: purpose_3]

The internal anti-stall mechanism exists to ensure that `cycle_age_ge_dmax` can be satisfied **within the same cycle**, even under extreme low participation.

Without this mechanism, Dmax would itself require sufficient participation to verify, recreating the stall problem it is meant to solve.

---

### 6.2 Role and Authority [anchor: role_and_authority]

The internal anti-stall mechanism:

* evaluates Tempo outputs and weak passive evidence,
* determines whether Dmax has plausibly been exceeded,
* may trigger a forced boundary.

It is a **liveness aid**, not a source of truth.

---

### 6.3 Authority Limits [anchor: authority_limits]

The internal anti-stall mechanism MAY:

* trigger forced cycle closure,
* allow one or more provisional cycles to advance,
* feed participation metrics into adaptive target recalibration.

It MUST NOT:

* produce or elevate time anchor beacons,
* unlock economic or governance power,
* override legitimacy gating,
* replace or bypass Tempo predicates.

---

### 6.4 Determinism Requirements [anchor: determinism_requirements]

Anti-stall logic MUST be:

* deterministic,
* replayable from the canonical log,
* identical for all honest nodes.

Any heuristics or thresholds used MUST be explicitly specified and governance-controlled.

---

### 6.5 Scope of Use [anchor: scope_of_use]

Anti-stall advancement is expected to be rare and temporary.

Under normal operation:

* deliberative closure should occur before Dmax,
* anti-stall logic remains dormant.

Under collapse:

* one or more forced cycles may occur,
* adaptive targets rapidly recalibrate,
* deliberative closure resumes once feasible.

---

## 7. Lagged Time Legitimacy and Constrained Mode [anchor: 7_lagged_time_legitimacy_and_constrained_mode]

### 7.1 Principle of Lagged Legitimacy [anchor: principle_of_lagged_legitimacy]

Cycle boundaries may advance **before** legitimacy is fully established.

Legitimacy—especially time legitimacy—is established through slow, challengeable processes and is intentionally allowed to **lag** behind structural progression.

This separation is critical to preventing acceleration attacks.

---

### 7.2 Legitimacy Lag Window (K) [anchor: legitimacy_lag_window_k]

A fixed, governance-defined lag window `K` (in cycles) is defined.

For cycle `r`, full authorization of boundary consequences requires that:

```
cycle (r − K) has a high-certainty time anchor beacon
```

Only beacon-level time claims (as defined by the Tempo Specification) qualify.

---

### 7.3 Authorization Rule [anchor: authorization_rule]

Boundary consequences at cycle `r` are authorized only if the above condition is met.

If the condition is met:

* the system operates in **normal mode**.

If the condition is not met:

* the system operates in **constrained mode**.

---

### 7.4 Constrained Mode [anchor: constrained_mode]

In constrained mode:

* mana MAY recharge but is **restricted in spendability**,
* POD recomputation is provisional or deferred,
* POINT minting and distribution are locked or paused,
* irreversible governance and lifecycle effects are blocked.

Allowed actions in constrained mode are limited to:

* time repair (attestations, challenges, evidence),
* governance recovery actions,
* system stabilization and maintenance.

The exact allowed action set is governance-defined but MUST preserve recovery capability.

---

### 7.5 Exit from Constrained Mode [anchor: exit_from_constrained_mode]

The system exits constrained mode automatically when:

* sufficient beacon-level time anchors exist for cycles ≥ `r − K`,
* legitimacy catches up with structure.

No manual intervention is required.

---

### 7.6 Security Guarantees [anchor: security_guarantees]

This model guarantees that:

* attackers cannot extract value by accelerating cycles,
* forced progression does not mint power,
* legitimacy failures result in restriction, not acceleration,
* recovery is always possible without trusted clocks.

---

### 7.7 Design Summary [anchor: design_summary]

Lagged legitimacy allows the system to:

* move forward structurally under any conditions,
* defer power until verification completes,
* maintain auditability and challengeability,
* preserve both liveness and safety.


## 8. Mana and Cycle Interaction [anchor: 8_mana_and_cycle_interaction]

### 8.1 Role of Mana [anchor: role_of_mana]

Mana is the protocol’s **rate-limiting resource** for canonical actions. Its purpose is to:

* prevent unbounded action throughput,
* pace deliberation at the individual level,
* guarantee that recovery actions remain possible under degraded conditions.

Mana is **not** a reward and is **not** a store of value.

---

### 8.2 Mana Issuance [anchor: mana_issuance]

Mana MUST recharge at **every cycle boundary**, regardless of:

* whether the boundary was deliberative or forced,
* whether time legitimacy is currently sufficient,
* whether the system is in constrained mode.

This rule is absolute.

#### Rationale [anchor: rationale_7]

* Guarantees liveness and recovery capacity.
* Prevents deadlock during partitions or catastrophic participation loss.
* Ensures remaining participants can repair time legitimacy and governance.

---

### 8.3 Mana Spendability Modes [anchor: mana_spendability_modes]

Mana spendability is gated by the system’s legitimacy state (see §7).

#### 8.3.1 Normal Mode [anchor: normal_mode]

When time legitimacy requirements are satisfied:

* Mana is fully spendable on all canonical actions.
* Normal creation, challenge, voting, and governance activity is allowed.

#### 8.3.2 Constrained Mode [anchor: constrained_mode_2]

When time legitimacy requirements are not satisfied:

* Mana MAY still recharge,
* Mana MUST be restricted in how it can be spent.

Allowed uses in constrained mode MUST include:

* time attestations and challenges,
* governance recovery actions,
* system stabilization and maintenance actions.

Disallowed uses in constrained mode MUST include at least:

* high-volume idea or connection creation,
* actions that materially expand the living map,
* actions that would generate irreversible economic consequences.

The exact allowlist is governance-defined but MUST preserve recovery.

---

### 8.4 Design Guarantees [anchor: design_guarantees_2]

This model guarantees that:

* mana never disappears entirely,
* recovery actions are always possible,
* accelerated cycles cannot be used to generate unlimited effective activity,
* restriction always targets **power**, not **capacity to repair**.

---







## 9. Long-Horizon Tempo Regulation [anchor: 9_long_horizon_tempo_regulation]

### 9.1 Purpose [anchor: purpose_4]

This section defines a deterministic, low-gain regulation mechanism that biases cycle time bounds toward a long-horizon target pace without introducing trusted clocks or short-term predictability.

This mechanism exists solely to correct slow drift in average cycle duration over extended horizons. It MUST NOT be used to schedule or force individual cycle closures.

---

### 9.2 Target Pace Definition [anchor: target_pace_definition]

Governance MAY define a long-horizon target cycle rate:

R_target = expected number of cycles per governance-defined long period


`R_target` is a descriptive goal, not an entitlement. Failure to meet the target MUST NOT be treated as a protocol violation.

---

### 9.3 Measurement Window [anchor: measurement_window]

Let `N` be a governance-defined measurement window expressed in cycles.

For any cycle `r`, the system MAY compute a realized pace estimate using only beacon-level time anchors:

ΔT_beacon(r, N) = elapsed time between beacon anchors associated with cycles r and r-N


Only beacon-level time claims MAY be used for this computation.

If sufficient beacon coverage does not exist for the full window, this mechanism MUST be disabled for that cycle.

---

### 9.4 Realized Pace Estimation [anchor: realized_pace_estimation]

When valid beacon data exists, a realized pace estimate is computed conservatively:

R_realized = N / midpoint(ΔT_beacon(r, N))


If beacon ranges overlap or are asymmetric, the midpoint MUST be chosen conservatively toward slower pace estimates.

No node-local clocks, wall-clock timestamps, or external time sources MAY be used.

---

### 9.5 Deviation Signal [anchor: deviation_signal]

A deviation signal is computed as:

ΔR = R_realized - R_target


The sign of `ΔR` indicates drift direction only:

- `ΔR > 0`: cycles occurring faster than target
- `ΔR < 0`: cycles occurring slower than target

The magnitude of `ΔR` MUST be capped to a governance-defined maximum before further use.

---

### 9.6 Parameter Modulation [anchor: parameter_modulation]

The deviation signal MAY be used to apply a small, bounded bias to cycle time bounds as consumed by downstream cycle rules:

D_min_next = D_min_base × (1 + k_min × f(ΔR))
D_max_next = D_max_base × (1 + k_max × f(ΔR))


Where:

- `D_min_base` and `D_max_base` are baseline values computed from normal cycle rules,
- `k_min` and `k_max` are governance-defined gains,
- `|k_min| > |k_max|` MUST hold,
- `f(ΔR)` is a monotonic, bounded normalization function.

Adjustments MUST be clamped to governance-defined envelopes.

---

### 9.7 Gain and Stability Constraints [anchor: gain_and_stability_constraints]

The following constraints MUST hold:

1. Adjustments MUST be low-gain.
2. Adjustments MUST be monotonic and continuous.
3. No single cycle MAY adjust bounds by more than a governance-defined percentage.
4. Adjustments MUST be derived only from sealed, replayable data.
5. Adjustments MUST NOT depend on the current cycle’s incomplete data.
6. Adjustments MUST NOT introduce oscillatory behavior.

---

### 9.8 Applicability Conditions [anchor: applicability_conditions]

Long-horizon tempo regulation MUST be disabled when any of the following hold:

- tempo is in a constrained or degraded mode,
- beacon certainty falls below a governance-defined minimum,
- the system is recovering from partition or collapse,
- fewer than `N` eligible cycles have completed since genesis or reset.

When disabled:

D_min_next = D_min_base
D_max_next = D_max_base


---

### 9.9 Non-Goals and Explicit Limits [anchor: non_goals_and_explicit_limits]

This mechanism MUST NOT:

- force a specific cycle to close earlier or later,
- override cycle sealing rules,
- serve as evidence for predicate satisfaction,
- create or revoke beacon claims,
- act as a liveness guarantee.

Its sole role is slow statistical bias correction.

---

### 9.10 Determinism and Replay [anchor: determinism_and_replay]

All computations defined in this section MUST be:

- deterministic,
- replayable from the canonical log,
- invariant under node restarts,
- auditable by external observers.

Any divergence constitutes a protocol violation.



## 10. POD and POINT Recompute Cadence [anchor: 10_pod_and_point_recompute_cadence]

### 10.1 Role of Cycles in the Economic Layer [anchor: role_of_cycles_in_the_economic_layer]

Cycles are the **only moments** at which economic state is recomputed.

Within a cycle:

* canonical actions accumulate,
* importance relationships may change,
* no economic balances are finalized.

At cycle boundaries:

* economic state is recomputed deterministically from the canonical log.

---

### 10.2 POD Recompute [anchor: pod_recompute]

#### Definition [anchor: definition_8]

POD is a **living, non-transferable measure of importance attribution** tied to identities and derived from the living idea graph.

#### Recompute Rule [anchor: recompute_rule]

At each cycle boundary:

* POD MUST be recomputed deterministically,
* using the living map as of the boundary,
* after burn/rot eligibility evaluation.

POD recomputation MAY be provisional if legitimacy is insufficient.

---

### 10.3 POINT Distribution [anchor: point_distribution]

#### Definition [anchor: definition_9]

POINT is the transferable economic token whose issuance and redistribution are derived from POD.

#### Distribution Rule [anchor: distribution_rule]

POINT minting, redistribution, melting, and inheritance MUST occur only at cycle boundaries.

---

### 10.4 Legitimacy Gating for Economic Effects [anchor: legitimacy_gating_for_economic_effects]

If time legitimacy requirements (§7) are satisfied:

* POD recompute is finalized,
* POINT distribution proceeds normally.

If time legitimacy requirements are not satisfied:

* POD recompute MUST be marked provisional or deferred,
* POINT minting and redistribution MUST be locked or paused,
* no irreversible economic balances may change.

Locked or provisional balances MAY be displayed but MUST NOT be spendable.

---

### 10.5 Rationale [anchor: rationale_8]

Economic effects are the most sensitive outputs of the system.

By gating them on lagged legitimacy:

* attackers cannot extract value from accelerated cycles,
* forced boundaries cannot be abused economically,
* verification failures result in delay, not loss.

---

## 11. Burn, Rot, and the Living Map [anchor: 11_burn_rot_and_the_living_map]

### 11.1 Role of Burn and Rot [anchor: role_of_burn_and_rot]

Burn and rot are lifecycle mechanisms that:

* remove inactive or low-importance ideas and connections from the **living map**,
* prevent unbounded growth and stagnation,
* keep the living map reflective of current collective attention.

Burn and rot NEVER delete history; they only affect living eligibility.

---

### 11.2 Timing [anchor: timing]

Burn and rot evaluation MUST occur at cycle boundaries.

The evaluation uses:

* importance rankings,
* inactivity measures,
* governance-defined thresholds.

---

### 11.3 Independence from Cycle Progression [anchor: independence_from_cycle_progression]

Burn, rot, and living-map size:

* MUST NOT affect whether a cycle closes,
* MUST NOT accelerate or delay cycle boundaries,
* MUST NOT substitute for deliberative metrics.

Living-map contraction or growth is **never** a cycle progression input.

---

### 11.4 Use as Feedback Signals [anchor: use_as_feedback_signals]

Burn-related metrics MAY be used to:

* tune future burn aggressiveness,
* tune future cycle targets,
* inform governance decisions,
* generate UI health indicators.

Such tuning MUST apply only to **future cycles** and MUST NOT retroactively affect progression.

---

### 11.5 Design Guarantees [anchor: design_guarantees_3]

This separation guarantees that:

* cycles are driven by reasoning, not map size,
* attackers cannot manipulate burn to stall or accelerate cycles,
* lifecycle pruning remains a maintenance mechanism, not a pacing lever.



## 12. Adaptive Recovery After Collapse (Normative Scenario) [anchor: 12_adaptive_recovery_after_collapse_normative_scenario]

### 12.1 Purpose [anchor: purpose_5]

This section describes the **intended behavior** of the cycle system under catastrophic participation loss. It is normative in outcome (what MUST happen), not in narrative detail.

The goal is to demonstrate that the system:

* never stalls permanently,
* never grants unverified power,
* automatically adapts to new participation regimes,
* returns to normal operation without manual intervention.

---

### 12.2 Collapse During an Active Cycle [anchor: collapse_during_an_active_cycle]

Consider a cycle `r` in which:

* participation drops suddenly and severely,
* normal deliberative closure becomes difficult or impossible because realized work falls below target:

  * `W < W_target`.

In this state:

* the cycle remains open,
* deliberation MAY continue with remaining participants,
* time predicates continue to be evaluated.


### 12.3 Forced Closure via Dmax [anchor: forced_closure_via_dmax]

If the cycle remains open until:

```
cycle_age_ge_dmin == true
AND
cycle_age_ge_dmax == true
AND
event_target_reached == false
```

then:

* the system MUST create a **forced boundary**,
* cycle `r` ends,
* cycle `r+1` begins.

This forced boundary:

* does not imply sufficient deliberation,
* does not authorize unrestricted power,
* exists solely to restore liveness.

---

### 12.4 Target Recalibration [anchor: target_recalibration]

At the forced boundary:

* completion metrics from cycle `r` are recorded,
* adaptive targets (`W_target`) are recomputed.

Because realized work in cycle `r` was low:

* `W_target` MUST decrease (subject to governance-defined clamps and smoothing).

Participation metrics MAY be recorded and used to update internal baselines for diagnostics and smoothing, but MUST NOT introduce any minimum participation gate.


### 12.5 Multiple Forced Cycles (If Necessary) [anchor: multiple_forced_cycles_if_necessary]

If participation remains too low in cycle `r+1` to reach deliberative closure:

* the system MAY repeat forced closure when Dmax is reached again,
* each forced cycle contributes new data to adaptive recalibration.

In practice, after one or two forced cycles:

* targets converge to levels achievable by the remaining participants.

---

### 12.6 Resumption of Normal Operation [anchor: resumption_of_normal_operation]

Once targets are sufficiently reduced:

* remaining participants’ normal activity satisfies:

  * `W ≥ W_target`,
  * `cycle_age_ge_dmin == true`.

The system then resumes **deliberative boundaries**.

No external intervention is required.


### 12.7 Economic and Legitimacy Behavior During Recovery [anchor: economic_and_legitimacy_behavior_during_recovery]

Throughout this process:

* mana continues to recharge each cycle,
* mana spendability remains restricted if time legitimacy lags,
* POD and POINT remain locked or provisional until legitimacy catches up.

This guarantees recovery without economic exploitation.

---

## 13. Determinism and Replay Guarantees [anchor: 13_determinism_and_replay_guarantees]

### 13.1 Deterministic Replay [anchor: deterministic_replay]

Given the same canonical event log, all honest nodes MUST:

* identify identical cycle boundaries,
* classify each boundary as deliberative or forced,
* compute identical participation metrics,
* recompute identical targets and derived state.

No nondeterministic inputs are permitted.

---

### 13.2 Explicit Boundary Classification [anchor: explicit_boundary_classification]

Each cycle boundary MUST record:

* cycle index,
* boundary type (deliberative or forced),
* triggering condition (e.g., `W_target met`, `Dmax forced`).

This information MUST be derivable during replay and auditable.

---

### 13.3 No Retroactive Modification [anchor: no_retroactive_modification]

Once a cycle boundary exists:

* it MUST NOT be removed or altered,
* even if later legitimacy challenges overturn time claims.

Legitimacy failures affect **future authorization**, not past structure.

---

### 13.4 Auditability [anchor: auditability]

The system MUST preserve:

* participation metrics per cycle,
* target values per cycle,
* time predicate certainty histories,
* legitimacy gating decisions.

These records MUST be sufficient for an external auditor to reconstruct:

* why each cycle ended,
* why power was or was not authorized.

---

## 14. Interfaces and Governance Parameters [anchor: 14_interfaces_and_governance_parameters]

### 14.1 Interfaces to Dependent Specifications [anchor: interfaces_to_dependent_specifications]

#### 14.1.1 Tempo Specification [anchor: tempo_specification]

This spec consumes from Tempo:

* `cycle_age_ge_dmin` (boolean + certainty),
* `cycle_age_ge_dmax` (boolean + certainty),
* `tempo_mode` flags,
* beacon-level time anchor identifiers.

Tempo MUST NOT:

* seal cycles,
* unlock mana,
* authorize economic effects.

---

#### 14.1.2 Token Specification [anchor: token_specification]

This spec invokes Token Specification logic at cycle boundaries to:

* recompute POD (provisionally or finally),
* mint, redistribute, or lock POINT,
* apply melt and inheritance rules.

The Token Specification MUST respect legitimacy gating signals from this spec.

---

#### 14.1.3 Lifecycle / Burn Specification [anchor: lifecycle_burn_specification]

This spec invokes lifecycle rules at cycle boundaries to:

* determine living eligibility,
* apply burn/rot rules.

Lifecycle outputs MUST NOT affect cycle progression decisions.

---

### 14.2 Governance-Controlled Parameters [anchor: governance_controlled_parameters]

The following parameters MUST be governance-configurable:

* `Dmin`, `Dmax` (time guardrails),
* adaptive target smoothing constants,
* minimum and maximum clamps for `W_target`,
* legitimacy lag window `K`,
* mana spendability restrictions in constrained mode,
* thresholds for provisional vs final POD/POINT computation,
* parameters for long-horizon time-bound modulation (if enabled).

All parameter changes MUST take effect only at cycle boundaries.


### 14.3 Invariants (Restated) [anchor: invariants_restated]

Governance MUST NOT change the following invariants:

* cycles are not time units,
* time never advances cycles directly,
* forced closure never grants power,
* mana always exists,
* economic value requires lagged legitimacy,
* the system must never permanently stall.

---

## **15. Explicit Separation Between Cycle Progression and Authority** [anchor: 15_explicit_separation_between_cycle_progression_and_authority]

### **15.1 Structural Progress vs Authorized Effects** [anchor: 15_1_structural_progress_vs_authorized_effects]

Cycle progression and authority are explicitly decoupled.

A cycle boundary represents **structural progression only**.

No cycle boundary—deliberative or forced—MAY be interpreted as implicit authorization for:

* economic finalization,
* governance effect activation,
* lifecycle irreversibility,
* visibility or importance dominance.

All authorization requires satisfaction of legitimacy conditions defined in §7.

---

### **15.2 No Implicit Authority from Repetition** [anchor: 15_2_no_implicit_authority_from_repetition]

Repeated cycle advancement, even over many cycles, MUST NOT accumulate implicit legitimacy.

Specifically:

* multiple forced boundaries do not converge into authority,
* repeated provisional recomputation does not become final by inertia,
* time spent in constrained mode does not relax legitimacy requirements.

Authority is granted only by explicit satisfaction of lagged legitimacy conditions.

---

### **15.3 Boundary Creation Is Not Consent** [anchor: 15_3_boundary_creation_is_not_consent]

A cycle boundary MUST NOT be interpreted as:

* collective agreement,
* majority approval,
* consensus,
* or endorsement of the actions taken within the cycle.

Consent and endorsement exist only through challenge resolution and legitimacy-gated effects.

---

## **16. Anti-Acceleration and Anti-Capture Guarantees** [anchor: 16_anti_acceleration_and_anti_capture_guarantees]

### **16.1 No Activity-Based Acceleration** [anchor: 16_1_no_activity_based_acceleration]

Cycle progression MUST NOT be accelerated by:

* increased per-identity activity,
* automation of allowed actions,
* parallelization of deliberation steps.

Only `V`, `C`, and their adaptive target `W_target` may contribute to normal closure.

---

### **16.2 No Minority Capture via Collapse** [anchor: 16_2_no_minority_capture_via_collapse]

A small surviving population MAY adapt the system to its new scale **only after**:

* forced boundaries occur,
* adaptive targets recalibrate downward,
* legitimacy lag rules are satisfied.

The same population MUST NOT gain equivalent authority while a larger active population still exists.

This property MUST arise automatically from adaptive targets and legitimacy lag, not from special-case logic.

---

### **16.3 No Retroactive Capture** [anchor: 16_3_no_retroactive_capture]

Actions taken during cycles that later fail legitimacy checks:

* MUST remain recorded,
* MUST remain auditable,
* MUST NOT retroactively gain authority or economic effect.

Later legitimacy cannot “repair” earlier unauthorized power.

---

## **17. Partition and Rejoin Behavior (Normative)** [anchor: 17_partition_and_rejoin_behavior_normative]

### **17.1 Independent Structural Progression** [anchor: 17_1_independent_structural_progression]

During a network partition:

* each partition MAY advance cycles independently,
* boundaries are computed deterministically within each partition,
* no partition gains authority over another.

---

### **17.2 Rejoin Reconciliation** [anchor: 17_2_rejoin_reconciliation]

Upon rejoin:

* all cycle boundaries coexist in the canonical log,
* forced vs deliberative classification remains visible,
* legitimacy gating applies uniformly across merged history.

No partition’s boundaries automatically override another’s.

---

### **17.3 Authority After Rejoin** [anchor: 17_3_authority_after_rejoin]

After rejoin:

* authority depends solely on post-merge legitimacy satisfaction,
* earlier structural advancement in one partition does not confer dominance.

---

## **18. Survivor-Scale Continuity Guarantee** [anchor: 18_survivor_scale_continuity_guarantee]

### **18.1 Minimum Viable Continuity** [anchor: 18_1_minimum_viable_continuity]

If participation collapses to a very small number of identities:

* cycles MUST remain advanceable,
* mana MUST continue to recharge,
* deliberation MUST remain possible,
* forced boundaries MUST allow adaptive recalibration.

The system MUST remain operable by any non-zero human population.

---

### **18.2 No Survivor Privilege** [anchor: 18_2_no_survivor_privilege]

Survivors gain no privileges beyond what adaptive recalibration naturally allows.

Specifically:

* no reduced legitimacy lag,
* no lowered authority thresholds,
* no bypass of time or challenge requirements.

---

## **19. Explicit Non-Goals** [anchor: 19_explicit_non_goals]

### **19.1 What Cycles Are Not** [anchor: 19_1_what_cycles_are_not]

Cycles MUST NOT be interpreted as:

* time units,
* blocks,
* reward epochs,
* governance rounds,
* safety oracles,
* or snapshot boundaries or canonical distribution units.

---

### **19.2 What Cycles Do Not Prevent** [anchor: 19_2_what_cycles_do_not_prevent]

The cycle system does not claim to prevent:

* total identity capture,
* coercion of all humans,
* malicious governance decisions made legitimately.

Its purpose is to ensure such events are **slow, visible, challengeable, and recoverable**.

---

## **20. Invariant Summary** [anchor: 20_invariant_summary]

The following invariants are mandatory:

1. Cycles pace structure, not authority.
2. Time constrains cycles but never advances them.
3. Liveness is preserved without granting power.
4. Authority is always lagged, challengeable, and explicit.
5. Structural progress never implies consent.



