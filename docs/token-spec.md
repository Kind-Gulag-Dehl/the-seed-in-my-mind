---
doc_id: token_spec
title: Token Specification
status: authoritative
version: v0
last_reviewed: 2026-06-22

scope:
  - Defines POD and POINT semantics, issuance, and constraints.

authoritative_for:
  - POD and POINT definitions and lifecycle.
  - Restrictions on token use in governance and truth processes.

not_authoritative_for:
  - Governance voting mechanics (see governance-spec.md).

depends_on:
  - protocol v5.md
  - governance-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of governance-spec.md and safety-spec.md.

reader_path:
  - prereq: governance-spec.md
  - next: safety-spec.md

keywords:
  - POD
  - POINT
  - tokens
  - incentives
---

> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

# Token System Specification  
## POD (Proof of Deliberation) and POINT [anchor: pod_proof_of_deliberation_and_point]
This document defines the deterministic semantics of the protocol’s dual-token system. It specifies how POD and POINT are computed, how they change over time, how they interact with governance and safety mechanisms, and how they remain replayable, auditable, and resistant to capture.

This specification is **normative**. Any conformant node MUST implement the behaviors described here exactly and MUST derive identical POD and POINT balances from the same canonical event log, snapshots, and active rulebooks.

This specification is **normative**. Any conformant node MUST implement the behaviors described here exactly and MUST derive identical POD and POINT balances from the same canonical event log, snapshots, and active rulebooks.

---

## 0. Scope, authority, and deployment posture [anchor: 0_scope_authority_and_deployment_posture]

### 0.1 Purpose of this specification [anchor: purpose_of_this_specification]

The purpose of this specification is to define the **token layer** of the protocol as a deterministic, replay-derived system that:

- reflects the current universal importance of human deliberation and action,
- provides an economic coordination mechanism that aligns with that importance,
- remains strictly separated from truth determination and governance authority,
- prunes itself naturally as attention fades, while preserving complete history.

This document specifies:
- what POD and POINT are,
- how they are computed and updated,
- how they respond to growth, decline, fraud, death, and inactivity,
- how governance may parameterize but never directly control token behavior.

This specification does **not** define UI behavior, client presentation, storage layout, database schemas, or chain-specific implementations. It defines **semantic behavior only**.

---

### 0.2 Authority and precedence [anchor: authority_and_precedence]

This specification is subordinate to and composes with the following documents, in descending order of authority:


   Defines non-negotiable constraints such as human equality, deterministic replay, and separation of epistemic and economic power.

2. **Deterministic Replay & Merge Specification**  
   Defines how canonical state is reconstructed from event logs and snapshots.

3. **Challenge Engine Specification**  
   Defines how challenges, arguments, voting, and verdicts operate and become canonical.

In the event of conflict:
- Protocol v5 invariants take absolute precedence.
- Deterministic replay rules override local optimizations or interpretations.
- This specification overrides any non-normative documentation, UI logic, or chain-layer assumptions.

No client, node operator, rulebook, or governance action may override the requirements defined here.

---

### 0.3 Canonical versus derived token state [anchor: canonical_versus_derived_token_state]

The token system is explicitly divided into **canonical inputs** and **derived outputs**.

Canonical inputs include:
- the ordered canonical event log,
  - governance rulebook activation at cycle boundaries (as defined by the governance system), and
- the active rulebook set as determined by deterministic replay, including:
  - governance rulebook activation at cycle boundaries (as defined by the governance system), and
  - token rulebook effectiveness at cycle boundaries (as defined by this specification).

POD balances, POINT balances, derived `lifecycle_state`, derived eligibility, mint amounts, melt pools, redistributions, and routing outcomes are **derived state**. They MUST NOT be stored as authoritative values and MUST be recomputed deterministically during replay.

No node may:
- persist token balances as canonical truth,
- introduce non-deterministic shortcuts.
- skip recomputation steps,
- introduce non-deterministic shortcuts.

All token-related outcomes MUST be explainable as a direct function of the canonical record.

For avoidance of doubt:
- **cycles** are the canonical pacing and window mechanism for token accounting and recomputation,
- **blocks** are hash-chained batching of events for integrity and transport,
- **snapshots** are cryptographic anchors and replay accelerators,
- snapshots MAY include derived state (including derived `lifecycle_state` where required by other specifications) for comparability and performance, but deterministic replay from canonical events remains the source of truth,
- token rulebooks MUST NOT redefine governance activation semantics, and governance rulebooks MUST NOT retroactively rewrite token outcomes.

Any implementation that treats derived balances or derived eligibility as canonical truth is non-conformant.


### 0.4 Early-stage deployment and regulatory posture [anchor: early_stage_deployment_and_regulatory_posture]

During early deployment phases, the protocol MAY operate under constrained exposure of its token layer in order to minimize regulatory risk and premature economic signaling.

During such phases:
- POINT transfers MAY be disabled entirely.
- POINT balances MAY be hidden or abstracted in user interfaces.
- Rate limits MAY be severe.
- Public-facing documentation MAY omit or minimize discussion of POINT.

However, the following invariant applies:

> **All token mechanics defined in this specification MUST be fully implemented and active from genesis, regardless of UI exposure or transfer availability.**

Minting, melting, redistribution, inheritance, fraud handling, and replay derivation MUST occur exactly as specified, even if some features are temporarily inaccessible to users.

Any relaxation or tightening of early-stage constraints MUST:
- occur only via standard governance procedures,
- activate at defined cycle boundaries,
- remain fully challengeable,
- never alter historical semantics.

---

## 1. Conceptual model and design goals [anchor: 1_conceptual_model_and_design_goals]

### 1.1 Why the token layer exists [anchor: why_the_token_layer_exists]

The token layer exists to solve a specific problem:

The protocol answers this by separating:

The protocol answers this by separating:
- **epistemic power** (truth, importance, governance), and
- **economic power** (coordination, funding, incentives).

The token system does not decide what is true, what is important, or what rules apply. It reflects and responds to those decisions after they have been made through human deliberation.

---

### 1.2 Dual-token architecture [anchor: dual_token_architecture]

The protocol uses exactly two tokens with strictly separated roles:

- **POD (Proof of Deliberation)**  
- **POINT**  

- **POINT**  
  A transferable economic token that circulates across cycles, minted and redistributed at cycle boundaries in proportion to POD, and used for coordination, bounties, and funding.

This separation is fundamental. Any system that allows economic assets to influence truth, importance, or governance outcomes is constitutionally invalid.

---

### 1.3 POD as a living share, not a permanent ledger balance [anchor: pod_as_a_living_share_not_a_permanent_ledger_balance]

POD is not a historical reward, reputation score, or badge of honor.

POD represents:
- each living verified human identity's current non-transferable share of universal importance routed through that identity's eligible canonical contributions,
- derived from the *alive* portion of the idea graph,
- continuously recomputed at cycle boundaries.

As ideas lose relevance, connections rot, or contributions are superseded, the POD associated with those contributions naturally declines.

History is never erased. Recognition is not permanent.

This ensures that:
- early participants do not accumulate perpetual dominance,
- importance reflects present consensus rather than past timing,
- the system remains responsive to new evidence and new contributors.

---

### 1.4 POINT as a circulatory economic alignment layer [anchor: point_as_a_circulatory_economic_alignment_layer]

POINT exists to translate epistemic importance into economic coordination **without feedback into epistemic authority**.

POINT:
- is minted at each cycle boundary from POD,
- partially melts each cycle,
- is redistributed in proportion to current POD shares.

This creates a circulatory system in which economic value:
- flows toward currently important contributors,
- slowly drains from inactive or obsolete positions,
- cannot be hoarded indefinitely without continued relevance.

---

---

### 1.5 Bounded growth and anti-entrenchment [anchor: bounded_growth_and_anti_entrenchment]

Human attention is finite. The token system is explicitly designed to reflect this constraint.

The living graph is bounded by:
- rate limits,
- challenge requirements,
- rot and burn mechanics,
- cycle-based recomputation.

As deliberation expands, token circulation expands.
As attention fades, circulation contracts.

This ensures that:
- the system does not grow without bound,
- obsolete structures lose economic weight,
- long-term capture by early elites is structurally prevented.

The result is a token system that behaves less like a speculative asset and more like the circulatory system of a living, deliberating organism.


## 2. Constitutional invariants [anchor: 2_constitutional_invariants]

These invariants are not preferences or defaults; they are structural constraints. Any system behavior that contradicts them is invalid, regardless of intent or outcome.

These invariants are not preferences or defaults; they are structural constraints. Any system behavior that contradicts them is invalid, regardless of intent or outcome.

---

### 2.1 POD invariants [anchor: pod_invariants]

1. **Non-transferability**  
   POD MUST NOT be transferred, sold, delegated, inherited, pooled, pledged, or otherwise reassigned between identities. POD is bound exclusively to the verified human identity that generated the qualifying events from which it is derived.

2. **Human-only origin**  
   Only verified human identities MAY accrue POD. AI identities, automated agents, tribes, organizations, or abstract entities MUST NOT hold POD under any circumstance.

3. **Replay-derived state**  
   POD balances MUST be derived deterministically from the canonical event log, snapshots, and active rulebooks. POD MUST NOT be stored as authoritative state.

4. **Explainable lineage**  
   Every unit of POD MUST be traceable to specific canonical events, routing paths, and alive importance structures. Black-box attribution is prohibited.

5. **No governance or epistemic weight**  
   POD MUST NOT influence:
   - voting weight,
   - challenge eligibility,
   - verdict aggregation,
   - rule adoption,
   - truth determination,
   - importance ranking logic,
   - safety or visibility decisions.

POD reflects importance; it does not grant authority.

---

### 2.2 POINT invariants [anchor: point_invariants]

1. **No epistemic influence**  
   POINT ownership or balance MUST NOT affect truth outcomes, importance rankings, challenge mechanics, voter selection, or certainty bands.

2. **No governance influence**  
   POINT MUST NOT be used to weight votes, proposals, vetoes, or any governance process.

3. **Purely economic semantics**  
   POINT exists solely as an economic coordination mechanism: for transfer, pooling, bounties, funding, and other non-epistemic uses.

4. **Replay determinism**  
   POINT balances MUST be deterministically derived from:
   - prior balances,
   - cycle minting,
   - melt and redistribution,
   - transfers (when enabled),
   - inheritance rules (on identity death).

No discretionary minting or adjustment is permitted.

---

### 2.3 Separation invariants [anchor: separation_invariants]

The token system enforces strict separation between:
- truth,
- importance,
- governance,
- economics.

Under no circumstances may:
- economic assets buy epistemic outcomes,
- importance metrics grant economic authority,
- governance rules create token-weighted power,
- safety or moderation decisions depend on token balances.

Any such coupling constitutes a constitutional breach.

---

### 2.4 Anti-capture guarantees [anchor: anti_capture_guarantees]

The system MUST remain resistant to long-term capture by early participants, wealthy actors, or coordinated groups.

To this end:
- POD decays naturally as relevance fades.
- POINT melts and redistributes at cycle boundaries.
- No token accumulates perpetual control.
- Governance cannot entrench token holders.

The token layer MUST reinforce adaptability, not hierarchy.

---

### 2.5 Mana lane invariants for substrate and overlays [anchor: mana_lane_invariants_for_substrate_and_overlays]

Mana pricing in the token system MUST be interpreted through two rulebook-controlled lanes:

1. **Lane U (universal substrate creation lane)**  
   Applies to canonical universal substrate creation actions (for example, publishing a non-draft idea or publishing a representation candidate as canonical substrate).

2. **Lane O (scoped overlay operations lane)**  
   Applies to scoped overlay operations that modify context-relative overlay state (for example, scoped `relative_importance` overlay edits and scoped display override set/clear actions).

Lane assignment and lane-specific costs:
- MUST be defined by active rulebooks,
- MUST remain deterministic and replay-verifiable,
- MUST NOT depend on discretionary node behavior,
- MUST NOT introduce hidden or implementation-local cost classes.

Scope-specific mana source constraints:
- Tribe mana MUST apply only to tribe-scoped Lane O overlay actions and associated tribe-scoped challenge participation.
- Canonically published personal projection or display-overlay writes MUST consume personal mana under Lane O. Direct edits to noncanonical private rank state are outside token accounting and consume no protocol mana.
- Universal substrate creation under Lane U MUST consume universal creation capacity as defined by rulebooks and MUST NOT be reclassified as a tribe-only object-creation lane.

Legacy terminology that treats "tribe-only object creation" as a distinct mana cost class is DEPRECATED and MUST NOT define active cost semantics.

---

## 3. Canonical objects relevant to token flow [anchor: 3_canonical_objects_relevant_to_token_flow]

This section defines which canonical objects participate in POD and POINT computation, and how they are treated by the token system. Objects not explicitly included here MUST NOT affect token behavior.

---

### 3.1 Identities [anchor: identities]

An **identity** represents a real agent capable of authorship and action within the protocol.
Public attribution uses a pseudonymous author identity (and optional non-identifying verification level); underlying verification credentials are not public by default.

For token purposes, identities fall into the following categories:

1. **Verified human identities**  
   - MAY accrue POD.
   - MAY hold and transfer POINT.
   - Are eligible sinks for POD routing.

2. **AI advisory identities**  
   - MUST NOT accrue POD.
   - MAY be referenced as authors of drafts or analyses.
   - MAY receive delegated execution context, but never POD.

3. **Collective or abstract entities** (tribes, organizations, concepts)  
   - MUST NOT accrue POD.
   - MUST NOT act as POD sinks.
   - MAY participate only as ideas or structural groupings.

Identity death, inactivity, or revocation is handled explicitly in later sections and does not retroactively alter canonical history.

---

### 3.2 Ideas [anchor: ideas]

An **idea** is a canonical object representing a claim, concept, plan, action, or identity reference.

For token purposes:
- Ideas do not hold POD or POINT.
- Ideas act as **routing structures** for POD flow.
- Only ideas in the **alive** state participate in routing.

Each idea includes:
- a type (truth claim, conceptual idea, actionable idea, action, identity),
- a lifecycle state (alive, rotting, burned),
- visibility and safety metadata (non-token-affecting).

Structural roles (e.g., backyard, anthill, action record) do not alter token semantics unless explicitly stated elsewhere.

Token semantics that reference ideas MUST respect derived `lifecycle_state` and living-map eligibility.

In particular:
- burned ideas and burned connections MUST be excluded from all supply-affecting computations and routing,
- rotted ideas MAY be excluded or treated according to the active token rulebooks, but any such treatment MUST be deterministic, replay-verifiable, and consistent with Protocol v5 living-map eligibility rules,
- any idea that is excluded by derived eligibility rules (burn/rot/taint/safety/quarantine) MUST be treated as excluded for token routing and issuance purposes.

### 3.3 Connections [anchor: connections]

### 3.3 Connections [anchor: connections]

A **connection** is a canonical relationship between ideas.

Only specific connection types and usages participate in POD routing. In general:
- POD routes through **relative_importance** connections that are alive and eligible.
- Connections marked as burned or excluded are ignored for routing.

Each connection relevant to token flow includes:
- source idea,
- destination idea,
- usage metadata,
- scope, axis, and timeframe (for importance),
- lifecycle state.

The exact eligibility rules for connections are defined in later sections.

Token semantics that reference connections MUST treat connection participation as derived and replay-verifiable.

In particular:
- `relative_importance` connections have independent derived `lifecycle_state` and MAY rot or burn independently of endpoint ideas,
- burned `relative_importance` connections MUST be excluded from routing and supply-affecting computations,
- structural and provenance connections that are active only when their endpoint idea is active (e.g., `created_by`, `same_as` patterns, or other provenance/representation links defined by the protocol) MUST be treated as active when the idea is active, but MUST NOT be used to bypass exclusions on burned `relative_importance` connections.

Connection eligibility for token routing MUST be derived during replay and MUST be explainable; it MUST NOT be authored.


### 3.4 Events as the sole POD attachment point [anchor: events_as_the_sole_pod_attachment_point]

POD attaches **only** to canonical events.

Examples of POD-eligible events include:
- creating an idea,
- creating or updating a description,
- submitting an argument or evidence,
- opening or participating in a challenge,
- voting in a challenge,
- performing or verifying an action.

Ideas and connections themselves do not earn POD. They merely define the structure through which POD flows.

This invariant ensures that:
- POD reflects human agency, not static objects,
- attribution remains traceable,
- replay semantics remain simple and deterministic.

No implementation may award POD directly to ideas, connections, or aggregates.

---

### 3.5 Eligibility filtering [anchor: eligibility_filtering]

Not all events, ideas, or connections are eligible for POD routing.

Eligibility depends on:
- challenge exposure and outcomes,
- fraud or falsification findings,
- lifecycle state (alive vs burned),
- rulebook-defined exclusions consistent with constitutional invariants.

Eligibility filtering removes objects from **future** token participation without erasing history or mutating past states.

The mechanics of eligibility, fraud handling, and edge cutting are defined in subsequent sections.

All token routing, issuance, and redistribution steps MUST apply eligibility filtering as a derived function of canonical replay.

Eligibility filtering MUST be:
- derived, not authored,
- deterministic and replay-verifiable,
- aligned with Protocol v5 derived `lifecycle_state`, living-map eligibility, identity quarantine/suspension, taint handling, and safety abstraction rules.

If an idea, connection, identity, vote, or action is excluded by derived rules, it MUST NOT contribute to:
- token issuance,
- token routing,
- token redistribution,
- supply-affecting computations.

## 4. Idea lifecycle and pruning semantics (token-relevant) [anchor: 4_idea_lifecycle_and_pruning_semantics_token_relevant]

## 4. Idea lifecycle and pruning semantics (token-relevant) [anchor: 4_idea_lifecycle_and_pruning_semantics_token_relevant]

The canonical record of the protocol is immutable, but the **living system** that participates in importance, POD routing, and POINT circulation is intentionally pruned. This section defines how derived lifecycle states interact with the token system.

Pruning affects only **future participation** in token computation. It never alters historical events, past attribution, or canonical truth.

Lifecycle state and eligibility are **derived during deterministic replay** from the canonical event log under the active rulebooks. They are not authored, not discretionary, and not administratively applied.

---

### 4.1 Lifecycle states [anchor: lifecycle_states]

Every idea exists in exactly one derived lifecycle state at any given cycle boundary:

1. **Active**  
   The idea is eligible to participate in:
   - importance ranking and propagation,
   - POD routing,
   - growth and supply-affecting computations,
   - POINT minting dynamics.

2. **Rotted**  
   The idea remains historically intact and inspectable but is excluded from the default living map.
   - It MAY remain visible under non-default lenses, search, or historical views.
   - It MUST be treated according to living-map eligibility rules for all supply-affecting computations.
   - If Protocol v5 defines rotted objects as excluded from living-map participation, then rotted ideas MUST be excluded from token routing and issuance exactly as burned ideas are.

3. **Burned**  
   The idea is excluded from the living system.
   - It MUST NOT participate in POD routing.
   - It MUST NOT contribute to growth or supply-affecting computations.
   - It MUST NOT receive new token-relevant attribution via routing.

Burned ideas remain part of the immutable historical record and may still be inspected, cited, or challenged, but they are economically inert.

For avoidance of doubt:
- lifecycle states are derived at cycle boundaries from canonical replay,
- blocks and snapshots do not determine lifecycle semantics; they MAY record derived lifecycle state for comparability and performance.

---

### 4.2 Effects of lifecycle state on POD routing [anchor: effects_of_lifecycle_state_on_pod_routing]

Lifecycle state directly affects whether and how POD flows through an idea.

- **Active ideas**  
  - Eligible as routing intermediaries and endpoints (subject to other derived eligibility rules).
  - Count toward any routing neighborhood or share-curve computation defined by token rulebooks.

- **Rotted ideas**  
  - MUST be treated as excluded from routing and supply-affecting computations whenever rotted objects are excluded from the living map by Protocol v5 living-map eligibility rules.
  - If a rulebook family permits limited handling of rotted objects for display or diagnostics, such handling MUST NOT affect issuance, routing, redistribution, or supply.

- **Burned ideas**  
  - MUST be excluded entirely from routing.
  - MUST be ignored for neighborhood size, share curves, and all growth/supply computations.

No lifecycle state may retroactively alter past POD attribution. All effects apply forward from the cycle boundary at which the derived lifecycle state becomes active.

---

### 4.3 Burn semantics [anchor: burn_semantics]

Burning an idea is a strong, forward-only exclusion from the living map.

- the idea’s contribution weight is treated as zero for all future token computations,
- all token-relevant routing paths through that idea are severed for future cycles,
- and no supply-affecting computations may count that idea as eligible.
- no new POD may be routed through that idea,
- and no supply-affecting computations may count that idea as eligible.

Burn does not imply falsehood, immorality, or erasure. It signifies only that the idea is no longer part of the active, living structure that receives economic recognition.

---

### 4.4 Restoration predicates and limits (resurrection) [anchor: restoration_predicates_and_limits_resurrection]

An idea MAY return from burned to active participation only via an explicit **resurrection action** as defined by Protocol v5.

Restoration:
- MUST occur via canonical events (an ordinary creation-cost action, not a vote-weighted privilege),
- MUST consume the rulebook-defined Lane U universal substrate creation cost for resurrection actions,
- MUST be fully replayable and auditable from the canonical event log,
- MUST NOT retroactively reinstate routing, attribution, or supply-affecting effects.

Restoration changes eligibility only for **future** participation starting at the cycle boundary at which the resurrection becomes effective under deterministic replay.

For avoidance of doubt:
- restoring an idea MUST NOT implicitly restore any incident `relative_importance` connections,
- structural/provenance connections that are defined to be active when the idea is active (e.g., created-by and representation/provenance links) become active automatically with the idea,
- `relative_importance` connections remain independently lifecycle-scoped and must be maintained or resurrected independently.

---

### 4.5 Relationship to token supply dynamics [anchor: relationship_to_token_supply_dynamics]

The size of the living eligible idea graph directly affects:
- POD routing topology,
- routing neighborhood computations,
- and dynamic POINT minting and redistribution behavior.

As ideas rot and burn:
- routing neighborhoods shrink,
- total eligible routing mass contracts,
- and supply-affecting computations reflect the reduced living structure.

---

---

### 4.6 Derived exclusions are not authored (normative) [anchor: derived_exclusions_are_not_authored_normative]

All exclusions that affect token accounting are derived during replay and MUST NOT be authored as discretionary flags.

This includes exclusions derived from:
- idea and connection `lifecycle_state` (active / rotted / burned),
- living-map eligibility rules,
- identity quarantine or suspension,
- taint handling,
- and safety or jurisdictional abstraction rules.

A conformant node MUST:
- recompute eligibility and exclusions deterministically from canonical replay,
- treat excluded objects as non-participating in issuance, routing, redistribution, and supply-affecting computations,
- preserve excluded objects in history and search views according to applicable lenses,



## 5. POD (Proof of Deliberation) semantics [anchor: 5_pod_proof_of_deliberation_semantics]

POD is not a stored balance, reward, or credential. It is a continuously recomputed measure of participation in present relevance.

POD is not a stored balance, reward, or credential. It is a continuously recomputed measure of participation in present relevance.

---

### 5.1 Definition and normative role of POD [anchor: definition_and_normative_role_of_pod]

Formally, POD is defined as:

\[
\text{POD}_u(c) = \sum_{e \in E_u(c)} \text{RoutedPOD}_c(e \rightarrow u)
\]

where \( c \) is the qualifying cycle boundary, \( E_u(c) \) is the set of POD-eligible canonical events attributed to living verified human identity \( u \) at that boundary, and routing uses the then-current universal-importance, lifecycle, eligibility, and active-rulebook state defined in Sections 6 and 7. Therefore \( \text{POD}_u(c) \) may be greater than, less than, or equal to its value at an earlier boundary, including zero.

POD serves three purposes:
1. to make importance legible and auditable,
2. to parameterize POINT distribution,
3. to provide a non-economic signal of current contribution.

POD does **not**:
- grant authority,
- grant voting power,
- grant moderation rights,
- grant special visibility.

---

### 5.2 Categories of POD-eligible contribution [anchor: categories_of_pod_eligible_contribution]

POD is attributed only to specific categories of human action. These categories are exhaustive.

---

#### 5.2.1 Epistemic POD (E-POD) [anchor: epistemic_pod_e_pod]

Epistemic POD is attributed to actions that advance collective understanding.

Examples include:
- creating ideas,
- writing or updating descriptions,
- submitting arguments or evidence,
- opening challenges,
- participating as a selected voter,
- voting in challenges,
- producing verdicts that endure challenge.

Epistemic POD reflects contribution to determining what is true and important.

---

#### 5.2.2 Action POD (A-POD) [anchor: action_pod_a_pod]

Action POD is attributed to actions that advance collective outcomes.

Examples include:
- performing an action linked to an actionable idea,
- verifying or validating real-world execution,
- maintaining infrastructure required for canonical operation,
- executing tasks that implement decided plans.

Action POD reflects contribution to doing what the system has determined matters.

---

#### 5.2.3 Explicit exclusions [anchor: explicit_exclusions]

Certain actions MUST NOT accrue POD, even if they are important or visible.

Exclusions include:
- actions prohibited by constitutional or safety rulebooks,
- actions that cause harm but are recorded for epistemic reasons,
- actions performed by non-human agents,
- actions that bypass challenge exposure when challenge is required.

Importance and reward are intentionally decoupled.

---

### 5.3 Challenge-gated eligibility [anchor: challenge_gated_eligibility]

POD eligibility is gated by challenge exposure.

An action or contribution:
- MUST have passed through the appropriate challenge process,
- MUST NOT be under active dispute or quarantine,
- MUST remain valid under current rulebooks.

This ensures that POD reflects *survived deliberation*, not untested assertion.

---

### 5.4 POD attribution granularity [anchor: pod_attribution_granularity]

POD is attributed at the level of **canonical events**.

Each eligible event:
- contributes POD weight,
- is attributed to exactly one verified human identity,
- participates in routing through the importance structure.

Ideas and connections define routing topology, but they do not themselves earn POD.

---

### 5.5 Temporal nature of POD [anchor: temporal_nature_of_pod]

POD is recomputed at each cycle boundary.

As a result:
- POD may increase, decrease, or vanish over time,
- no POD balance is permanent,
- sustained contribution is required to sustain flow.

This temporal nature enforces fairness across generations of contributors.

---

### 5.6 Relationship between POD and pruning [anchor: relationship_between_pod_and_pruning]

POD routing respects lifecycle states:

- routing ignores burned ideas,
- routing may be reduced or altered by rotting ideas,
- routing adapts automatically as the living graph evolves.

This closes the loop between deliberation, relevance, and economic alignment.

This closes the loop between deliberation, relevance, and economic alignment.


## 6. POD routing and attribution mechanics [anchor: 6_pod_routing_and_attribution_mechanics]

This section defines how POD flows from canonical events through the living importance structure and is attributed to verified human identities. These rules are deterministic, replayable, and exhaustive. Any implementation that deviates from these mechanics is non-conformant.

---

### 6.1 Event-level attribution [anchor: event_level_attribution]

Every POD-eligible canonical event is attributed to exactly one verified human identity (the event author).

For each eligible event \( e \):
- The event contributes a unit weight \( W_e \) to POD routing.
- \( W_e \) MAY be parameterized by rulebooks (e.g., different weights for argument submission vs voting), but MUST be deterministic and bounded.
- \( W_e \) MUST NOT depend on token balances, identity status, or governance roles.

Events authored by non-human identities contribute zero POD.

---

### 6.2 Routing topology [anchor: routing_topology]

POD routes through the **alive importance structure** defined by:
- alive ideas,
- alive relative_importance connections,
- active scope/axis/timeframe selections.

Routing reads canonical importance state only after that state has been replayed. Universal routing inputs use the distinct 20-axis universal product and its derived `overall_universal_rank`. Eligible `relative_importance` edges may define local paths, but public-relative and tribe-relative positions MUST NOT be arithmetically folded into universal importance. Private and simulated ranks are never routing inputs.

---

---

### 6.3 The downhill rule (mandatory constraint) [anchor: the_downhill_rule_mandatory_constraint]

POD routing MUST satisfy the downhill rule. Because one-based position `1` is the highest overall universal rank:

\[
\operatorname{overall\_universal\_rank}(\text{source})
<
\operatorname{overall\_universal\_rank}(\text{destination})
\]

The routing comparison uses the replay-derived overall universal ordinal position after sorting ideas by ascending `universal_position_sum` and the active deterministic tie-break. `universal_position_mean = universal_position_sum / 20` MAY be displayed, but no floating-point display value, token balance, or private/tribe-relative rank may replace the ordinal comparison.

If the inequality is not satisfied, the routing path is invalid and MUST be excluded from computation.

This rule ensures:
- POD flows from more important ideas to less important ones,
- cycles are prevented,
- attribution converges toward human identities as terminal sinks.

No rulebook may disable or invert the downhill rule.

---

### 6.4 Terminal sinks [anchor: terminal_sinks]

Verified human identities act as **terminal sinks** for POD.

Routing proceeds until:
- an identity-linked idea is reached, or
- no further valid downhill edges exist.

At a terminal sink:
- all remaining routed POD is attributed to the associated human identity,
- no further routing occurs.

Identities do not route POD onward.

---

### 6.5 Neighborhood definition [anchor: neighborhood_definition]

For a given routing step, the **local neighborhood** is defined as:

- the set of eligible outgoing relative_importance connections
- originating from the current idea
- that satisfy lifecycle and downhill constraints.

---

---

### 6.6 Eligibility filtering and edge cutting [anchor: eligibility_filtering_and_edge_cutting]

Routing excludes any edge or idea that is:
- burned,
- quarantined due to fraud,
- explicitly excluded by a constitutionally valid rulebook.

When an edge is cut:
- its routing weight is set to zero for all future cycles,
- it is removed from neighborhood counts and normalization,
- past attribution remains unchanged.

No retroactive mutation of routing history is permitted.

---

### 6.7 Deterministic aggregation [anchor: deterministic_aggregation]

For each identity \( u \), POD is computed as:

\[
\text{POD}_u = \sum_{e \in E_u} \text{RoutedPOD}(e \rightarrow u)
\]

Where:
- \( E_u \) is the set of eligible events authored by \( u \),
- \( \text{RoutedPOD}(e \rightarrow u) \) is the amount of POD from event \( e \) that reaches \( u \) through valid routing paths.

Aggregation MUST be deterministic and order-independent.

---

## 7. POD share curves and local allocation math [anchor: 7_pod_share_curves_and_local_allocation_math]

This section defines how POD is split among multiple routing paths within a local neighborhood. Share curves are mathematical functions that assign proportions to each eligible outgoing edge in the neighborhood.

All share curves MUST be:
- deterministic,
- normalized (shares sum to 1.0 within the neighborhood),
- monotonically non-increasing by rank,
- independent of identity attributes, POD, POINT, reputation, wealth, or any non-canonical data.

---

---

### 7.1 Local ranking within a neighborhood [anchor: local_ranking_within_a_neighborhood]

Within a routing neighborhood:

1. All eligible outgoing edges are ranked by their relative importance ordering (highest first), using the canonical connection semantics for the relevant scope, axis, and timeframe.
2. The rank index \( k \) starts at 1 for the most important edge.
3. Let \( n \) be the number of eligible outgoing edges in the neighborhood.

#### 7.1.1 Deterministic tie-breaking (normative) [anchor: deterministic_tie_breaking_normative]

If two or more outgoing edges are tied under the relative importance ordering, ties MUST be broken deterministically using the following sequence:

1. **Earlier ordinal position in the complete canonical relative context**, when the edges are distinguishable there.
2. **Lexicographic ascending order of the canonical edge identifier**.

If a tie-breaking field is unavailable, the implementation MUST skip that criterion and proceed to the next without introducing nondeterminism.

No numeric relative-importance strength exists in the base protocol. A token rulebook MUST NOT introduce one as a substitute for pairwise challenge history.

---

### 7.2 Linear decay share curve (canonical) [anchor: linear_decay_share_curve_canonical]

The protocol uses a **linear decay share curve** for all POD routing.

Weights are assigned as:

\[
w_k = (n - k + 1)
\]

The normalized POD share for edge \( k \) is:

\[
\text{share}_k = \frac{w_k}{\sum_{i=1}^{n} i}
\]

This curve:
- prioritizes higher-ranked edges,
- preserves participation for lower-ranked edges,
- avoids extreme concentration of flow,
- remains stable as neighborhood sizes change due to pruning.

No alternative share curve may be enabled.

---

### 7.3 Governance constraints on share curves [anchor: governance_constraints_on_share_curves]

Governance MUST NOT:
- replace the linear decay share curve,
- introduce alternative functional forms,
- introduce per-identity or per-edge exceptions,
- modify weight ordering or normalization rules.

Governance MAY:
- adjust upstream importance inputs that affect ranking,
- adjust pruning rules that affect neighborhood membership.

The share curve itself is a constitutional constant of the token system.

---

### 7.4 Edge cases and stability guarantees [anchor: edge_cases_and_stability_guarantees]

The following cases are handled deterministically:

- **Single-edge neighborhood** (\( n = 1 \)):  
  The sole edge receives 100% of routed POD.

- **Empty neighborhood** (\( n = 0 \)):  
  Routing terminates and remaining POD is attributed to the current sink.

- **All edges excluded**:  
  Equivalent to an empty neighborhood.

These guarantees ensure routing always terminates and never oscillates.

---

### 7.5 Interaction with pruning [anchor: interaction_with_pruning]

Share curve computation ignores:
- burned edges,
- burned ideas,
- cut edges,
- quarantined edges when exclusion is active under the current rulebook.

As pruning progresses:
- neighborhood sizes shrink,
- normalization adapts automatically,
- total routed POD contracts smoothly.

This ensures mathematical stability as the living graph evolves.



## 8. Fraud, error, and correction handling (operational semantics) [anchor: 8_fraud_error_and_correction_handling_operational_semantics]

The token system assumes that error, deception, and good-faith mistakes are inevitable in any large deliberative system. This section defines how such cases are handled **without retroactive mutation**, **without discretionary intervention**, and **without breaking deterministic replay**.

All correction mechanisms operate forward-only and are mediated through canonical challenges.

---

### 8.1 Fraud detection via challenges [anchor: fraud_detection_via_challenges]

All allegations of fraud, falsification, manipulation, or improper conduct MUST be expressed as canonical challenges under the Challenge Engine Specification. These challenges may concern:

All allegations of fraud, falsification, manipulation, or improper conduct MUST be expressed as canonical challenges under the Challenge Engine Specification. These challenges may concern:

- false claims,
- fabricated evidence,
- invalid actions,
- misrepresentation of execution,
- improper authorship or attribution.

A finding of fraud exists only when a challenge reaches a canonical verdict under the rules in force at the time.

---

### 8.2 Forward-only correction principle [anchor: forward_only_correction_principle]

Under no circumstances may the system:

- delete historical events,
- rewrite past POD or POINT attribution,
- retroactively alter balances,
- invalidate already-executed cycles.

Instead, all corrections apply **prospectively** from the cycle boundary following the relevant verdict.

This preserves:
- auditability,
- replay determinism,
- historical integrity.

---

### 8.3 Edge cutting semantics [anchor: edge_cutting_semantics]

When a challenge establishes that a claim, connection, or action is fraudulent or invalid, the system MAY apply **edge cutting** as a corrective measure.

Edge cutting has the following effects:

  - routing neighborhoods,
- The connection is removed from:
  - routing neighborhoods,
  - share curve normalization,
  - growth and activity metrics.
- No new POD may route through that edge.

Past routing and attribution remain unchanged.

Edge cutting is the primary mechanism by which fraudulent structures are economically neutralized without erasing history.

---

### 8.4 POD handling under fraud findings [anchor: pod_handling_under_fraud_findings]

When fraud is established:

- Future POD attribution via the affected structure MUST cease.
- At the next qualifying cycle boundary, current POD MUST be recomputed without any structure or event made ineligible by the finding.
- The affected identity's current POD MAY consequently decrease or become zero.
- Rulebooks MAY prospectively quarantine or exclude affected routing inputs, but MUST NOT freeze an identity's current POD at a prior value.

Rulebooks MAY define bounded penalties, but MUST NOT:
- alter the POD derivation recorded for an earlier completed cycle,
- introduce discretionary punishments,
- target identities outside canonical findings.

---

### 8.5 Quarantine and cooling states (optional) [anchor: quarantine_and_cooling_states_optional]

Governance MAY define **quarantine** or **cooling** states as intermediate measures.

These states MAY:
- temporarily suspend routing through an idea or connection,
- exclude structures from growth metrics,
- signal uncertainty to downstream processes.

Such states:
- MUST be time-bounded or verdict-bounded,
- MUST activate only at cycle boundaries,
- MUST be fully replayable.

---

### 8.6 Error correction without blame [anchor: error_correction_without_blame]

Not all corrections imply misconduct.

Good-faith errors, superseded claims, or obsolete actions MAY be resolved via:
- challenges resulting in downgrade or exclusion,
- natural rot and burn,
- re-routing of POD away from outdated structures.

The token system distinguishes **invalidity** from **malice** and applies the least invasive correction necessary to preserve integrity.

---

## 9. POINT semantics [anchor: 9_point_semantics]

All POINT behavior is derived deterministically at cycle boundaries from:

All POINT behavior is derived deterministically at cycle boundaries from:
- the canonical event log,
- the active token rulebooks (bounded by constitutional invariants).
- prior cycle POINT balances,
- the active token rulebooks (bounded by constitutional invariants).

POINT MUST NOT influence truth, importance, governance, challenge mechanics, or visibility decisions.

---

### 9.1 Definition and role of POINT [anchor: definition_and_role_of_point]

POINT represents **discrete, transferable bundles of economic energy** derived from Energy Flow (POD) at cycle boundaries.

POINT:
- MAY be transferred between verified human identities (when transfers are enabled),
- MAY be pooled or spent,
- MAY be used for bounties, funding, and coordination,
- MUST NOT affect truth, importance, or governance.

POINT is always subordinate to POD:
- POD determines where POINT is minted and redistributed,
- POINT never feeds back into POD routing or importance computation.

---

### 9.2 Cycle-based base minting from POD (canonical) [anchor: cycle_based_base_minting_from_pod_canonical]

At each structural cycle boundary (anchored by `cycle_close` at the deterministic replay prefix, addressable by `H_close` where block heights are exposed), the system computes the total active POD using the replay prefix through and including `cycle_close`. Block height is an address for the replay prefix, not a source of time legitimacy. Minting applies only to balances after that boundary and MUST NOT retroactively change prior cycles.

\[
\text{TotalPOD} = \sum_{u} \text{POD}_u
\]

The system then computes the **base mint** for the cycle:

\[
\text{BaseMint} = M \cdot \text{TotalPOD}
\]

Where:
Each identity \( u \) receives newly minted POINT proportional to their POD share:

Each identity \( u \) receives newly minted POINT proportional to their POD share:

\[
\text{Mint}_u = 
\begin{cases}
\text{BaseMint} \cdot \frac{\text{POD}_u}{\text{TotalPOD}} & \text{if } \text{TotalPOD} > 0 \\
0 & \text{if } \text{TotalPOD} = 0
\end{cases}
\]

Minting is performed only at cycle boundaries and is fully replay-derived.

### 9.2A Authorization-Frontier Token States [anchor: authorization_frontier_token_states]

Token-derived effects MUST be classified during replay as:

- **provisional**: computed from structural cycle data but not yet cycle-certified through the lagged authorization frontier;
- **pending**: eligible for later finalization if certification and frontier conditions are satisfied;
- **authorized**: finalized because the required certified cycle is at or behind the authorization frontier;
- **blocked**: unavailable because certification failed, contradiction blocks certification, or the effect was forbidden when attempted.

POD may be provisional while cycle certification is pending. POINT minting, distribution, spendability, melt, and redistribution remain pending or paused until the lagged authorization frontier authorizes the relevant cycle. Ordinary mana and rate-limit authority likewise remain pending or blocked outside the constrained allowlist.

Forced cycles do not create token value extraction. Later certification MAY finalize outputs that were explicitly pending or provisional. Later certification MUST NOT retroactively validate forbidden actions, make ineligible votes valid, create ordinary mana or rate-limit burst capacity, or backfill missed POINT/POD effects beyond those explicitly preserved as pending outputs.

---

### 9.3 Dynamic supply adjustment tied to living-graph growth (canonical) [anchor: dynamic_supply_adjustment_tied_to_living_graph_growth_canonical]

To reflect expansion or contraction of meaningful deliberation, the system applies a deterministic dynamic adjustment based on the size of the living POD-distributing structure.

Let:
- \( E_t \) = the number of **eligible POD-distributing edges** at cycle \( t \)
- \( E_{t-1} \) = the same quantity at cycle \( t-1 \)
The growth rate is:

The growth rate is:

\[
g = \frac{E_t - E_{t-1}}{\max(E_{t-1}, 1)}
\]

The dynamic adjustment is:

\[
\text{DynamicAdjust} = k \cdot g \cdot \text{TotalPOINT}_{t-1}
\]

Where:

- \( \text{DynamicAdjust} \) MAY be negative during contraction.

#### 9.3.1 Eligible POD-distributing edge definition [anchor: eligible_pod_distributing_edge_definition]

For the purpose of \( E_t \), an edge counts if and only if it is:
- a `relative_importance` connection,
- alive (not burned),
- not cut,
- not excluded or quarantined from routing under the active rulebook,
- valid under the downhill rule at cycle \( t \).

This definition is intentionally strict so that inflation tracks the living, routable structure rather than raw graph size.

---

### 9.4 POINT melt mechanics (canonical) [anchor: point_melt_mechanics_canonical]

POINT decays via cycle-based melt.

\[

\[
\text{Melt}_u = m \cdot \text{POINT}_u
\]

Where:
Melted POINT is removed from individual balances and contributes to the cycle’s redistribution pool (§9.5).

Melt MUST NOT be 100% in a single cycle. Rulebooks MUST enforce an upper bound strictly less than 1.

Melt MUST NOT be 100% in a single cycle. Rulebooks MUST enforce an upper bound strictly less than 1.

---

### 9.5 Redistribution pool and payout (canonical) [anchor: redistribution_pool_and_payout_canonical]

The redistribution pool for the cycle is constructed from melted POINT plus the dynamic supply adjustment.

Let:

\[
\text{MeltTotal} = \sum_u \text{Melt}_u
\]

The raw pool is:

\[
\text{PoolRaw} = \text{MeltTotal} + \text{DynamicAdjust}
\]

#### 9.5.1 Pool non-negativity (normative clamp) [anchor: pool_non_negativity_normative_clamp]

The redistribution pool used for payouts MUST NOT be negative. The payable pool is:

\[
\text{Pool} = \max(\text{PoolRaw}, 0)
\]

If \( \text{PoolRaw} < 0 \), the negative remainder is treated as a **system contraction** that reduces total supply by reducing redistribution to zero for that cycle; it MUST NOT create negative payouts or negative balances.

#### 9.5.2 Payouts [anchor: payouts]

If \( \text{TotalPOD} > 0 \), the pool is redistributed proportionally to POD:

\[
\text{Payout}_u =
\text{Pool} \cdot \frac{\text{POD}_u}{\text{TotalPOD}}
\]

If \( \text{TotalPOD} = 0 \), then:

- \(\text{Mint}_u = 0\) for all \(u\),
- \(\text{Payout}_u = 0\) for all \(u\),
- \(\text{Pool}\) MUST be handled deterministically as follows:
  - by default it is **burned** (removed from circulation),
  - governance MAY choose **carry-forward** into the next cycle via rulebook,
  - in either case the handling MUST be explicit, deterministic, and cycle-bounded.

---

### 9.6 Total per-identity cycle update (derived) [anchor: total_per_identity_cycle_update_derived]

For each identity \( u \), the cycle update (excluding transfers and inheritance) is:

\[
\text{POINT}_u^{\text{after}} =
\text{POINT}_u^{\text{before}}
- \text{Melt}_u
+ \text{Mint}_u
+ \text{Payout}_u
\]

Transfers (when enabled) and inheritance (on identity death) apply in their own phases in §11 and §10.

---

---

### 9.7 Interaction with pruning, fraud, and exclusion [anchor: interaction_with_pruning_fraud_and_exclusion]

When structures are pruned (rot/burn) or cut due to fraud:

- future POINT mint and payout to affected identities decreases naturally via POD recomputation,
- existing POINT remains subject to melt and redistribution,
- no retroactive confiscation occurs.

- therefore their future mint and payout decrease,
- their future POD share decreases,
- therefore their future mint and payout decrease,
- but previously minted POINT is not retroactively removed.

This preserves immutable history while ensuring forward economic neutrality.

---

### 9.8 Default parameter set for mint, melt, and redistribution (genesis defaults) [anchor: default_parameter_set_for_mint_melt_and_redistribution_genesis_defaults]

At genesis, the protocol MUST initialize a complete deterministic default parameter set for POINT. Unless superseded by a later valid token rulebook (effective only at cycle boundaries), the following defaults apply:

1. **Mint coefficient**
   \[
   M = 1
   \]
   (One unit of POINT base mint per unit of TotalPOD per cycle.)

2. **Dynamic adjustment constant**
   \[
   k = 0.1
   \]

3. **Melt rate**
   \[
   m = 0.01
   \]
   (1% of each balance melts per cycle.)

4. **Pool non-negativity**
5. **TotalPOD = 0 handling**

5. **TotalPOD = 0 handling**
   - By default, the pool is **burned** when \(\text{TotalPOD}=0\).
   - Governance MAY switch to carry-forward by rulebook.

6. **Governance bounds (hard limits)**
   Any token rulebook specifying values outside these bounds is invalid and MUST be rejected during replay:
   - \( 0 < m \le 0.20 \)
   - \( 0 \le k \le 0.50 \)
   - \( 0 \le M \le 10 \)

These genesis defaults provide complete determinism for independent implementations while allowing bounded tuning over time.



## 10. POINT transfers, inheritance, and identity death [anchor: 10_point_transfers_inheritance_and_identity_death]

This section defines how POINT may be transferred between identities, how POINT is handled when an identity ceases to exist, and how these processes remain deterministic, replayable, and non-capturing. POINT transferability is explicitly constrained to preserve the separation between economic coordination and epistemic or governance power.

All rules in this section are forward-only and MUST NOT retroactively alter past cycles.

---

### 10.1 POINT transfer semantics [anchor: point_transfer_semantics]

POINT transfers are canonical economic events that move POINT from one verified human identity to another.

When transfers are enabled by governance:
- POINT MAY be transferred between verified human identities.
- Transfers MUST be recorded as canonical events.
- Transfers MUST be validated deterministically.
- Transfers MUST NOT affect POD, importance rankings, challenge eligibility, governance rights, voter selection, or verdict outcomes.

Transfers are balance movements only. They confer no standing, authority, or epistemic weight.

---

### 10.2 Transfer enablement and disablement [anchor: transfer_enablement_and_disablement]

Governance MAY:
- enable transfers,
- disable transfers,
- apply deterministic rate limits,
- restrict transfers to specific contexts (e.g., bounties-only).

Such changes:
- MUST activate at cycle boundaries,
- MUST be fully challengeable,
- MUST be replayable from canonical events and snapshots.

When transfers are disabled:
- attempted transfers MUST fail deterministically,
- all other POINT mechanics (mint, melt, redistribution, inheritance) remain active.

---

### 10.3 Identity death (canonical termination) [anchor: identity_death_canonical_termination]

An identity may cease to exist due to:
- verified human death,
- voluntary account termination,
- irreversible identity revocation under constitutional rules.

Identity death MUST be represented by a canonical event.

Identity death:
- permanently halts all future POD attribution to that identity,
- removes the identity as a terminal POD sink for future cycles,
- prevents any future transfers authored by or directed from that identity,
Historical events authored by the identity remain part of the canonical record.

Historical events authored by the identity remain part of the canonical record.

---

### 10.4 POD handling on identity death [anchor: pod_handling_on_identity_death]

On identity death:
- POD associated with the identity ceases to exist in future cycles (via recomputation),
- routing that would terminate at the identity instead terminates earlier according to the routing rules,
- no POD is inherited, transferred, reassigned, or preserved as a balance.

This ensures that POD remains strictly tied to living human agency.

---

### 10.5 POINT inheritance hierarchy [anchor: point_inheritance_hierarchy]

POINT held by a deceased identity MUST be resolved deterministically according to an inheritance hierarchy.

The default inheritance order is:

1. **Explicit beneficiary**  
   If the identity designated a beneficiary via a canonical event, that beneficiary receives the inheritable POINT.

2. **Burn fallback**
   If no beneficiary exists, inheritable POINT follows the active rulebook's non-lineage burn or public redistribution rule. Profile-v0 sponsor or invitation lineage MUST NOT be a beneficiary, fallback recipient, ownership interest, or economic entitlement.

Invitation capacity is not money, reputation, ownership, truth weight, vote weight, governance influence, or economic authority. Sponsorship records admission provenance only and creates no claim on an admitted identity's future POINT, earnings, assets, or inheritance. Any future sponsor-lineage economic mechanism requires a separately governed profile and has no active Profile-v0 effect.
Governance MAY parameterize this hierarchy but MUST preserve determinism and MUST NOT allow discretionary assignment.

Governance MAY parameterize this hierarchy but MUST preserve determinism and MUST NOT allow discretionary assignment.

---

### 10.6 Inheritance timing and determinism [anchor: inheritance_timing_and_determinism]

Inheritance:
- MUST occur at the cycle boundary following the identity death event,
- MUST be replayable solely from canonical events and the active rulebooks,
- MUST NOT retroactively affect prior cycles.

No manual intervention is permitted.

---

### 10.7 Anti-entrenchment guarantees [anchor: anti_entrenchment_guarantees]

Inheritance rules MUST NOT:
- allow indefinite accumulation of POINT across generations,
- create dynastic economic power,
- reintroduce token-based authority.

Decay and redistribution continue to apply to inherited POINT immediately and normally.

---

### 10.8 Transfer validation and anti-double-spend rules (normative) [anchor: transfer_validation_and_anti_double_spend_rules_normative]

Transfers MUST be validated deterministically to prevent double-spends and ordering ambiguity.

Each transfer event MUST include:
- `from_identity_id`
- `to_identity_id`
- `amount`
- `nonce` (monotonic per `from_identity_id`)
- `cycle_index` (the cycle in which the transfer is submitted; used only for validation context)

#### 10.8.1 Nonce rules [anchor: nonce_rules]

For each `from_identity_id`:
- the first valid transfer MUST have `nonce = 1`,
- each subsequent valid transfer MUST have `nonce = previous_nonce + 1`,
- any transfer with a nonce that is:
  - less than or equal to the last accepted nonce, or
  - greater than the last accepted nonce + 1
  MUST be rejected.

This ensures a single deterministic transfer chain per sender.

#### 10.8.2 Balance rules [anchor: balance_rules]

A transfer MUST be rejected if:
- `amount <= 0`,
- `from_identity_id == to_identity_id`,
- the sender is dead or ineligible at validation time,
#### 10.8.3 Canonical ordering for simultaneous transfers [anchor: canonical_ordering_for_simultaneous_transfers]

#### 10.8.3 Canonical ordering for simultaneous transfers [anchor: canonical_ordering_for_simultaneous_transfers]

When multiple transfers from the same sender appear in the same cycle segment:
- transfers MUST be evaluated in canonical event-log order,
- only those that satisfy the nonce chain and balance constraints are accepted,
- invalid transfers are rejected and do not affect balances.

No client may reorder transfers. No node may use arrival time or network observation as ordering.

For avoidance of doubt:
- `cycle_index` is derived during deterministic replay and is used only as a validation context label,
- canonical ordering is always the event-log order; cycle membership does not create an alternate ordering mechanism.

### 10.9 Transfer quarantines and freezes (optional, rulebook-controlled) [anchor: transfer_quarantines_and_freezes_optional_rulebook_controlled]

Governance MAY define rulebook-controlled quarantines or freezes for POINT transfers in response to:
- detected exploits,
- systemic attacks,
- emergency safety conditions.

If enabled, such controls MUST satisfy:

- **Deterministic scope**: freezes must specify exactly which transfers are disallowed (e.g., all transfers, transfers above a threshold, transfers from flagged identities).
- **Cycle-bound activation**: activation and release occur only at cycle boundaries, scheduled via governance verdicts (e.g., an `activation_cycle_index` and, where applicable, an `expiration_cycle_index`).
- **Sunset requirement**: emergency freezes MUST include a defined expiration cycle unless renewed by a new governance action.
- **No epistemic coupling**: freezes MUST NOT alter POD, truth, importance, governance voting, or challenge outcomes.

When a freeze applies:
- disallowed transfer events MUST be rejected deterministically,
- balances continue to update via mint, melt, redistribution, and inheritance.

Freeze logic affects only transfer validity, not the underlying economic layer.

For avoidance of doubt:
- blocks and snapshots may be used to package or anchor state, but do not determine when a freeze becomes effective,
- freeze effectiveness is determined solely by cycle-anchored scheduling under deterministic replay.


## 11. Cycle mechanics and deterministic ordering [anchor: 11_cycle_mechanics_and_deterministic_ordering]

This section defines the temporal structure of the token system. All token computations occur in discrete, deterministic **cycles**. Cycle boundaries are the only moments at which POD and POINT derived balances change.

Cycle semantics are replay-derived: given the same canonical inputs, all conformant nodes MUST compute identical cycle boundaries, identical cycle state transitions, and identical POD/POINT results.

Cycles may approximate wall-clock time at the ecosystem level, but are defined entirely by canonical events and rulebook parameters, not by clocks.

---

### 11.1 Cycle boundaries [anchor: cycle_boundaries]

A **cycle** is a fixed interval of the canonical event log within which:
- canonical events accumulate,
- no token balances change,
- and all token state transitions are applied only at the boundary.

Cycle boundaries:
- MUST be globally computable from the canonical log and active rulebooks,
- MUST partition the event log into deterministic segments,
- MUST NOT depend on wall-clock time observed by a node,
- MUST NOT depend on network arrival order or connectivity.

All token updates occur only at cycle boundaries.

All token updates occur only at cycle boundaries.

Cycle boundaries MUST be derived using the Protocol v5 cycle derivation rules (including any event-count targets, smoothing parameters, participation thresholds, and diversity thresholds). This specification does not redefine cycle derivation; it consumes the derived cycle index as an input to token accounting.

---

### 11.2 Order of operations per cycle [anchor: order_of_operations_per_cycle]

At each cycle boundary, conformant nodes MUST perform the following operations in the exact order listed:

1. **Importance recomputation**  
   Replay all twenty universal-axis orderings from the current living map, compute each idea's exact position sum/mean, and derive `overall_universal_rank`.

2. **Lifecycle evaluation**  
   Apply rot and burn rules to ideas and eligible connections.

3. **Eligibility filtering**  
   Apply derived exclusions, including fraud findings, identity quarantine/suspension, taint rules, and lifecycle-based ineligibility. Safety abstraction affects visibility but MUST NOT be treated as a token-eligibility override unless an explicit rulebook-defined eligibility rule is in force.

4. **POD routing and attribution**  
5. **POINT base minting**  

5. **POINT base minting**  
6. **Dynamic supply adjustment**  

6. **Dynamic supply adjustment**  
7. **POINT melt**  

7. **POINT melt**  
8. **Redistribution**  

8. **Redistribution**  
9. **Inheritance processing**  

9. **Inheritance processing**  
10. **Finalize cycle commitment**  

10. **Finalize cycle commitment**  
No step may be skipped, reordered, merged, or partially executed.

No step may be skipped, reordered, merged, or partially executed.

---

### 11.3 Deterministic replay guarantees [anchor: deterministic_replay_guarantees]

Given:
- the same canonical event log,
- the same snapshots (when used),
- the same active rulebooks and their effective cycles,

all conformant nodes MUST:
- compute identical cycle boundaries,
- compute identical POD values,
- compute identical POINT balances,
- apply identical pruning, eligibility filtering, minting, melting, redistribution, and inheritance.

Any divergence constitutes non-conformance.

---

### 11.4 Stability and termination guarantees [anchor: stability_and_termination_guarantees]

Cycle computation MUST:
- terminate in finite time,
- avoid oscillation or feedback loops,
- remain stable under pruning and contraction.

Routing terminates because:
- routing neighborhoods are finite,
- the routing/propagation rules are monotonic and bounded,
---

---

### 11.5 Explainability requirements [anchor: explainability_requirements]

Nodes MUST be able to explain, for any identity:

- why their POD has a given value at a given cycle,
- which events contributed to it and how they routed,
- why their POINT changed at a given cycle (mint, melt, redistribution, transfers, inheritance),
- how pruning, lifecycle transitions, quarantines/suspensions, tainting, or other derived exclusions affected routing.

Explanations MUST reference:
- canonical events,
- the applicable routing neighborhood and tie-break outcomes,
- and the derived inclusion/exclusion predicates that applied at replay time.
- effective rulebook parameters and effective cycles,
- and the derived inclusion/exclusion predicates that applied at replay time.

Black-box implementations are non-conformant.

---

### 11.6 Cycle boundary definition and canonical representation (normative) [anchor: cycle_boundary_definition_and_canonical_representation_normative]

Cycle boundaries MUST be determined and representable such that any conformant node can identify:
- the exact last event included in cycle \(c\),
- the exact first event included in cycle \(c+1\),
- the active rulebook set effective for cycle \(c+1\),
- the commitment data sufficient to validate replay outputs.

#### 11.6.1 Boundary determination [anchor: boundary_determination]

Cycle boundaries are defined as a deterministic function of the canonical event log and the Protocol v5 cycle derivation parameters.

The active rulebook set MUST specify (or reference the protocol-defined defaults for):
- which event types count toward cycle advancement,
- the target event density and smoothing parameters (e.g., EMA-based pacing),
- any minimum participation or diversity thresholds required for a cycle transition,
- how boundary conditions are evaluated deterministically during replay.

Because cycle boundaries may depend on participation properties in addition to event counts, the exact event at which a cycle boundary occurs MAY NOT be predictable in advance, but MUST be determinable unambiguously during replay.

Token implementations MUST NOT introduce any alternate cycle boundary definition. If a node computes token cycles using rules that differ from Protocol v5 cycle derivation, it is non-conformant.

#### 11.6.2 Boundary representation [anchor: boundary_representation]

Each cycle boundary MUST be representable in the canonical record via at least one replay-verifiable anchor.

A conformant implementation MUST support identifying cycle boundaries using one or both of the following:

1. **Derived boundary with verifiable anchors**  
   Cycles are derived by replay from the canonical event log under Protocol v5 rules, while blocks and snapshots provide cryptographic anchoring and replay acceleration.

2. **Explicit boundary marker events (optional)**  
   If the protocol defines an explicit boundary marker, each cycle boundary MAY also be committed by a canonical marker that records:
   - the cycle index,
   - the last included event pointer,
   - the active rulebook set hash/identifier,
   - and any required replay commitments.

Boundary markers, if present, MUST be consistent with the boundary derived from replay. A boundary marker that conflicts with replay-derived cycle boundaries is invalid.

A conformant node MUST reject any boundary representation that is ambiguous or unverifiable from canonical data.


### 11.7 Cycle activation, challengeability, and forward-only correction (normative) [anchor: cycle_activation_challengeability_and_forward_only_correction_normative]

Cycle boundary rules and token rulebook activations MUST be challengeable through normal governance mechanisms. However:

- cycles MUST NOT be rolled back,
- past cycle computations MUST NOT be rewritten,
- corrections MUST be forward-only.

If an error is discovered (e.g., an exploit, misconfigured rulebook, or incorrect parameter adoption):
- governance MAY adopt a corrective rulebook,
- the corrective rulebook MUST activate at a future cycle boundary,
- the system MUST proceed from the corrected future state.

#### 11.7.1 Disputes near boundary time [anchor: disputes_near_boundary_time]

If two nodes disagree during synchronization about whether a cycle boundary has been finalized:
- the dispute MUST resolve by referencing the canonical record (snapshot or boundary marker),
This preserves the requirement that system time is derived from shared canonical commitments, not local authority.

This preserves the requirement that system time is derived from shared canonical commitments, not local authority.

#### 11.7.2 Emergency handling without rollback [anchor: emergency_handling_without_rollback]

Emergency actions (e.g., transfer freezes, quarantine activation) MAY be adopted, but:
- MUST be explicit rulebooks,
- MUST have effective cycles,
- MUST include sunset or expiration constraints where required,
- MUST NOT retroactively invalidate transfers or cycle results already committed.

All emergency response is forward-only.

---

### 11.8 Relationship between cycles, blocks, and snapshots (normative) [anchor: relationship_between_cycles_blocks_and_snapshots_normative]

Cycles define structural recomputation points for the token system. The lagged authorization frontier controls when consequential token effects become final.

Blocks and snapshots serve different roles:
- **Blocks** group events for hashing and transport efficiency.
- **Snapshots** accelerate replay and provide cryptographic anchors.
- **Cycles** determine when token state is structurally recomputed.
- **The authorization frontier** determines when consequential token state becomes finalized.

Blocks and snapshots MUST NOT be treated as substitutes for cycles in token computation.
Cycle membership is derived independently during replay and MAY span multiple blocks or partial blocks.

Snapshots MAY coincide with cycle boundaries, but this is an optimization, not a requirement.
Token correctness depends only on deterministic cycle derivation, cycle certification, and the lagged authorization frontier, not on snapshot frequency, block height, publication count, local clocks, or server/client timestamps.

## 12. Governance interaction and operational rulebooks [anchor: 12_governance_interaction_and_operational_rulebooks]

The token system is parameterized by governance but never controlled by it. This section defines how governance may interact with POD and POINT mechanics without violating constitutional invariants, replay determinism, or human equality.

Governance operates exclusively through **rulebooks**. Rulebooks may tune bounded parameters, enable or disable optional operational features, and impose emergency constraints, but they may not alter core token semantics.

Rulebooks are not discretionary. They are explicit, replayable parameter commitments.

---

### 12.1 Rulebook scope and authority [anchor: rulebook_scope_and_authority]

Token-related rulebooks define **parameter values and configuration choices** for the token system. They do not define new mechanics, introduce new execution paths, or grant discretionary authority.

All token-related rulebooks:

- MUST be adopted through the standard governance process  
  (one verified human identity, one equal vote within the applicable eligibility pool).
- MUST be represented as canonical governance events.
- MUST specify an explicit **effective cycle**.
- MUST activate **only at cycle boundaries**, never mid-cycle.
- MUST be fully replayable, auditable, and challengeable from the canonical event log.

Rulebooks:
- parameterize logic already defined in this specification and its dependencies,
- select between allowed boundary forms, thresholds, caps, and coefficients,
- MAY enable or disable optional mechanisms explicitly defined in this specification,
- MUST NOT introduce new token mechanics not already specified canonically.

No token rulebook MAY:
- execute logic directly,
- override deterministic replay rules,
- alter past cycle results,
- introduce discretionary or identity-specific behavior,
- introduce private parameters or non-public configuration,
All rulebook effects MUST be:

All rulebook effects MUST be:
- forward-only,
- cycle-indexed,
- derived solely during deterministic replay.

If a rulebook attempts to specify behavior outside the bounds of this specification or Protocol v5 invariants, it MUST be treated as invalid during replay and MUST NOT activate.

For avoidance of doubt:
- rulebooks govern **how the system behaves**, not **who benefits**,
- they may adjust parameters that affect aggregate dynamics,
- they MUST NOT privilege specific identities, groups, or ideas,
- and they MUST NOT bypass lifecycle_state, eligibility, or living-map exclusion rules.


---

### 12.2 Token rulebook categories (expanded) [anchor: token_rulebook_categories_expanded]

Governance MAY define token rulebooks in the following categories. Categories are normative labels that constrain what a rulebook is allowed to modify.

1. **POINT minting and redistribution parameters**
   - Pool handling when \( \text{TotalPOD} = 0 \) (burn vs carry-forward) (see §9.5.2)

2. **POINT decay parameters**

2. **POINT decay parameters**

   - Upper bounds preventing total melt in a single cycle

3. **Eligibility and exclusion filters (token-relevant)**
   - Definitions of identity quarantine / suspension effects as inputs to token eligibility
   - Deterministic exclusion rules consistent with Protocol v5 invariants
   - Fraud-verdict-triggered taint/exclusion rules that cut routing prospectively
   - Explicit requirement that exclusions respect derived `lifecycle_state` and living-map eligibility (burned/rotted objects MUST NOT be counted)

4. **Operational transfer constraints**
   - Transfer enablement or disablement
   - Rate limits and deterministic throttling
   - Context restrictions (e.g., bounty-only transfers)
5. **Inheritance policy parameters**

5. **Inheritance policy parameters**
   - Inheritance hierarchy option set (beneficiary / inviter / anthill / burn vs pool)
   - Eligibility criteria for recipients (alive/verified/not-frozen)
   - Deterministic proportional distribution rules for anthill fallback

6. **Cycle-accounting parameters (token-relevant)**
   - Parameters that affect token computations *given* the protocol-defined cycle boundaries, such as:
     - minimum eligibility requirements for counting certain events as token-relevant,
     - caps or clamps applied at cycle recomputation time,
     - cycle-level accounting presentation rules that do not change ordering or validity.

This category MUST NOT be used to redefine cycle boundary derivation.

In particular:
- Token rulebooks MUST NOT select a boundary form, boundary interval, or boundary marker mechanism for cycles.
- Cycle boundary derivation is defined by Protocol v5 and its composed specifications.
Each token rulebook MUST explicitly declare:

Each token rulebook MUST explicitly declare:
- its category,
- its effective cycle,
- its full parameter set and bounds.

If a token rulebook attempts to modify cycle boundary derivation, snapshot scheduling, governance activation boundaries, or any non-token protocol semantics, it MUST be treated as invalid and MUST NOT activate.


### 12.3 Governance limitations (hard prohibitions) [anchor: governance_limitations_hard_prohibitions]

Governance MUST NOT:

- introduce identity-weighted mechanics (by POD, POINT, reputation, wealth, role, or account age),
- create permanent exemptions from decay,
- introduce identity-weighted mechanics (by POD, POINT, reputation, wealth, role, or account age),
- create permanent exemptions from decay,
- grant POD or POINT to non-human identities,
- enable token-weighted voting, governance, moderation, or visibility control,
- retroactively modify balances, attribution, or past cycle results,
Any rulebook that violates these constraints is invalid and MUST be rejected during replay.

Any rulebook that violates these constraints is invalid and MUST be rejected during replay.

---

### 12.4 Emergency rulebooks (bounded) [anchor: emergency_rulebooks_bounded]

Emergency rulebooks MAY be defined to respond to:
- systemic attacks,
- discovered exploits,
- catastrophic implementation flaws,
Emergency rulebooks:

Emergency rulebooks:
- MUST be time-bounded or sunset-bound,
- MUST specify explicit activation and expiration cycles,
- MUST be challengeable under expedited procedures,
- MUST NOT override constitutional invariants,
- MUST NOT roll back past cycles or invalidate already-committed results.

Emergency powers are constrained by design and cannot become permanent.

---

### 12.5 Transparency and auditability (normative) [anchor: transparency_and_auditability_normative]

All active token rulebooks MUST be:
- visible to all participants,
- included in snapshots (or otherwise committed at cycle boundaries),
- referenced in replay explanations.

No hidden parameters, unpublished adjustments, or client-specific defaults are permitted.

Nodes MUST expose, at minimum:
- the active token rulebook set for any cycle,
- the effective-cycle history of token rulebooks,
- the parameter values used for any POD/POINT computation.

---

### 12.6 Rulebook lifecycle semantics for tokens (normative) [anchor: rulebook_lifecycle_semantics_for_tokens_normative]

Token rulebooks MUST follow a deterministic lifecycle:

1. **Propose**
   - A rulebook proposal MUST include:
     - category,
     - complete parameter set,
     - bounds (where applicable),
     - effective cycle,
     - sunset cycle if emergency.

2. **Adopt**
   - Adoption occurs via canonical governance action.
   - Adoption MUST commit a stable identifier (hash) of the full rulebook content.

3. **Activate**
   - Activation occurs only at the declared effective cycle boundary.
   - A rulebook with an effective cycle earlier than the current cycle MUST be rejected.

4. **Supersede**
   - Supersession is achieved only by adopting a new rulebook with a later effective cycle.
   - No rulebook may partially override another unless the protocol defines deterministic merge rules for parameter namespaces.

#### 12.6.1 Conflict resolution (normative) [anchor: conflict_resolution_normative]

If multiple token rulebooks are adopted with overlapping parameter namespaces and the same effective cycle:
- conflicts MUST be resolved deterministically by canonical adoption order, and
- the superseded rulebook MUST be treated as inactive for that cycle.

If deterministic conflict resolution cannot be performed from canonical data, the rulebook set is invalid and MUST be rejected.

---

### 12.7 Rulebook transparency requirements (operational) [anchor: rulebook_transparency_requirements_operational]

For any cycle and any identity, a node MUST be able to produce an explanation bundle that includes:

For any cycle and any identity, a node MUST be able to produce an explanation bundle that includes:
- which token rulebooks were active,
- any exclusions/quarantines/freeze rules that affected:
- all parameter values actually used,
- any exclusions/quarantines/freeze rules that affected:
  - routing neighborhoods,
  - minting,
  - transfers,
  - inheritance.

This requirement applies equally offline: rulebook explanations must be computable from the same mindseed package used for replay.

---

### 12.8 Multi-client and offline compatibility requirements [anchor: multi_client_and_offline_compatibility_requirements]

Token rulebooks MUST be interoperable across clients and nodes.

Therefore:
- rulebooks MUST be self-contained and fully specified (no external links as authoritative content),
- rulebooks MUST NOT depend on proprietary services, vendor APIs, or private schemas,
- rulebooks MUST be interpretable in offline deterministic replay using only canonical data.

If a rulebook cannot be evaluated offline from canonical data, it is invalid.

---

### 12.9 Emergency controls catalog (bounded, replayable) [anchor: emergency_controls_catalog_bounded_replayable]

Emergency controls are permitted only in a fixed catalog of actions that affect economics but never epistemics.

Emergency token rulebooks MAY do the following:

1. **Freeze POINT transfers**
   - Temporarily reject all transfer events (or transfers above a deterministic threshold).

2. **Enable transfer throttles**
   - Enforce deterministic per-cycle limits on transfer count or amount.

3. **Quarantine routing through flagged structures**
   - Exclude specified edges/ideas from routing neighborhoods, prospectively, when they are under active fraud challenge or emergency review.

4. **Freeze optional subsystems**
   - Disable optional modules such as bounty payout execution if an exploit is detected in that subsystem.

Emergency token rulebooks MUST NOT:
- change POD routing rules,
- change the linear share curve,
- change importance computation,
- change governance eligibility or voting rules,
- confiscate balances retroactively,
- roll back cycles.

All emergency controls MUST include:
- an explicit activation cycle,
- an explicit expiration cycle (sunset),
- a deterministic scope definition (what exactly is frozen/quarantined and how that set is identified canonically).

---

## 13. Safety, prohibited value capture, and ethical constraints [anchor: 13_safety_prohibited_value_capture_and_ethical_constraints]

The token system explicitly separates **importance** from **reward**. This section defines how the system prevents economic recognition of harmful actions while preserving epistemic visibility and historical integrity.

---

### 13.1 Importance does not imply reward [anchor: importance_does_not_imply_reward]

An action or idea may be:
- highly important,
- widely discussed,
- central to deliberation,

and still be **ineligible for POD or POINT**.

This distinction is intentional. The system is designed to understand the world, not to reward harm.

---

### 13.2 POD exclusion classes [anchor: pod_exclusion_classes]

The following categories of actions MUST NOT accrue POD:

- actions that directly cause prohibited harm,
- actions that violate constitutional safety constraints,
- actions performed under coercion or automation,
- actions that bypass required challenge exposure.

Such actions may still be recorded, analyzed, and debated.

---

### 13.3 Handling of harmful but important actions [anchor: handling_of_harmful_but_important_actions]

When harmful actions are important to understand:

- they may remain visible in the graph,
- they may receive high importance rankings,
- they may attract challenges and analysis,

but:
- POD routing MUST bypass them,
- economic recognition MUST cease at that boundary.

This prevents moral hazard while preserving epistemic completeness.

---

### 13.4 No laundering of harm into reward [anchor: no_laundering_of_harm_into_reward]

The system MUST prevent indirect reward through structural manipulation.

Specifically:
- POD MUST NOT be earned by supporting structures whose sole purpose is to amplify prohibited harm,
- edge cutting MAY be applied to isolate such structures,
- routing MUST respect exclusion boundaries strictly.

---

### 13.5 Ethical neutrality of the mechanism [anchor: ethical_neutrality_of_the_mechanism]

The token system does not encode moral judgments directly. Instead, it enforces **structural constraints** that prevent reward capture while allowing deliberation.

Ethical conclusions are reached through:
- challenges,
- governance,
- collective judgment,

not through token mechanics.

---

### 13.6 Alignment with long-term human survival [anchor: alignment_with_long_term_human_survival]

The ultimate safety constraint is this:

> The token system MUST NOT incentivize outcomes that systematically undermine long-term human flourishing or collective survival.

When such risks are identified:
- governance may intervene via rulebooks,
- pruning and exclusion may apply,
- but history and explainability must remain intact.

This ensures that the system remains both truthful and safe.


## 14. Offline, mindseed, and survivability semantics [anchor: 14_offline_mindseed_and_survivability_semantics]

The token system is designed to remain valid, reconstructable, and trustworthy under conditions of partial connectivity, prolonged disconnection, censorship, or civilizational disruption. This section defines how POD and POINT behave in offline contexts and how they are reconstructed through mindseed exchange and deterministic merge.

Offline survivability is a **hard requirement**, not an optimization.

---

### 14.1 Offline operation principles [anchor: offline_operation_principles]

Offline operation is a first-class design requirement.

Under offline conditions, the token system MUST satisfy the following properties:

- POD and POINT balances MUST be reconstructable without access to any global authority.
- No live oracle, external price feed, trusted timestamp service, or centralized ledger is required.
- All token semantics derive exclusively from:
  - canonical events,
  - canonical snapshots and/or canonical boundary markers,
  - active rulebooks and their effective cycles.

Offline nodes MAY continue to:
- record canonical events locally,
- create challenges and arguments,
- perform actions,
- accumulate publishable canonical logs.

Offline operation MUST NOT grant additional economic throughput.

In particular:
- offline activity MUST NOT advance canonical cycles beyond what canonical replay permits,
- offline-derived cycle indices or derived balances are advisory UX data only and MUST NOT bind canonical ingestion,
- during canonical ingestion and replay, cycle boundaries and all derived token state are recomputed deterministically from the merged canonical event log.

No minting, melting, redistribution, or inheritance resolution occurs outside cycle boundaries, but all inputs required for those operations MUST be preserved for later deterministic replay.


### 14.2 Mindseed packaging and token-relevant data [anchor: mindseed_packaging_and_token_relevant_data]

A **mindseed** is a portable, self-contained package of canonical data sufficient to reconstruct a portion of the system deterministically under Protocol v5 semantics.

For token reconstruction purposes, a mindseed MUST include, at minimum:

- the canonical events relevant to the included graph subset, in canonical order,
- the most recent **completed cycle boundary** represented by:
  - a snapshot and/or
  - an explicit cycle-boundary marker sufficient for replay verification,
- all token-relevant rulebooks that were active at or before that boundary,
- identity metadata required to evaluate authorship, attribution, eligibility, quarantine, and suspension status,
- sufficient importance and connection data to recompute:
  - universal importance,
  - relative-importance neighborhoods,
  - routing topology required for POD flow.

Mindseeds MUST NOT include:

- precomputed POD balances,
- precomputed POINT balances,
- cached minting, melt, redistribution, or inheritance results,
- discretionary annotations, summaries, or overrides that are not derivable from canonical data.

All POD and POINT balances, flows, and lifecycle effects MUST be recomputed deterministically upon import. Any mindseed that attempts to assert balances or economic outcomes directly is non-conformant.

---

### 14.3 Token reconstruction from mindseeds [anchor: token_reconstruction_from_mindseeds]

Upon importing a mindseed, a conformant node MUST execute the following deterministic reconstruction procedure:

1. **Canonical integrity validation**  
   Validate the integrity of all included canonical events, snapshots, and cycle-boundary markers, including hash chaining and signature verification.

2. **Cycle boundary identification**  
   Identify the most recent completed cycle boundary contained within the mindseed. Partial or provisional boundaries MUST NOT be treated as authoritative.

3. **Importance recomputation**  
   Replay all twenty universal-axis orderings from the alive graph at the identified cycle boundary, then derive the exact position sums/means and `overall_universal_rank`.

4. **Lifecycle evaluation**  
   Re-evaluate lifecycle_state (alive, rotting, burned) for all ideas and eligible connections using the rulebooks effective at that boundary.

5. **Eligibility filtering**  
   Apply fraud findings, quarantines, suspensions, and other derived exclusions as of that boundary.

6. **POD routing**  
7. **POINT computation**  

7. **POINT computation**  
   Recompute POINT minting, dynamic adjustment, melt, redistribution, and inheritance from the identified cycle onward.

If multiple mindseeds are imported that contain overlapping data:

- overlapping events MUST be deduplicated deterministically by canonical identity,
- conflicting non-canonical annotations MUST be discarded,
- replay MUST converge to identical POD and POINT results on all conformant nodes.

Any divergence indicates non-conformance.

---

### 14.4 Merge determinism across offline branches [anchor: merge_determinism_across_offline_branches]

When offline branches rejoin the canonical universe:

- token outcomes MUST converge identically on all conformant nodes,
- no manual reconciliation, averaging, or discretionary resolution is permitted,
- no branch may assert privileged authority based on isolation, uptime, or local progress.

Cycle boundaries guarantee that:

- minting, melting, redistribution, and inheritance occur only after a shared canonical boundary is established,
- isolated or partially observed branches cannot mint, redistribute, or settle independently.

Merge determinism relies exclusively on:

- canonical event ordering,

Any implementation that attempts to “merge balances” or reconcile outcomes directly violates deterministic replay.

---

---

### 14.5 Survivability under extreme conditions [anchor: survivability_under_extreme_conditions]

The token system MUST remain operable and well-defined under extreme conditions, including:

- prolonged or permanent network outages,
- censorship, partitioning, or jurisdictional isolation,
- collapse of centralized infrastructure,
- partial or degraded computing and storage capacity.

Under such conditions:

- nodes MAY pause cycle advancement until sufficient canonical data is available,
- token semantics MUST remain invariant,
- isolation MUST NOT grant economic advantage or accelerated minting.

The system explicitly prefers **delay over divergence**. Stalled progress is acceptable; inconsistent economic state is not.

---

### 14.6 Canonical timekeeping dependency statement (normative) [anchor: canonical_timekeeping_dependency_statement_normative]

Token cycles MUST NOT depend on:

- local wall-clock time,
- trusted timestamps,
- external time authorities,
Instead:

Instead:

- cycle boundaries MUST be derived solely from canonical commitments and replay-derivable conditions defined elsewhere in the protocol,
- any implementation that advances cycles based on unverifiable time sources is non-conformant.

Canonical commitments and conditions that MAY be used to support cycle derivation include:
- canonical event-log order,
- rulebook-defined boundary conditions evaluated during replay,
- cryptographic anchors such as hash-chained blocks and block-height keyed snapshots,
- optional canonical boundary markers that commit cycle index and event pointers.

If precise global time becomes unavailable or disputed:

- cycle progression MAY slow or pause,
- but no node may unilaterally advance cycles or assert temporal authority.

This constraint is mandatory to ensure resistance to:

- authoritarian manipulation of time,
- coordinated misinformation about elapsed cycles,
- economic advantage via temporal spoofing.


### 14.7 Post-catastrophic continuity guarantees [anchor: post_catastrophic_continuity_guarantees]

Even in post-catastrophic scenarios where:
- only partial mindseeds survive,
- nodes operate intermittently,
- communication is sporadic,

the following guarantees MUST hold:

- historical token outcomes remain reconstructable,
- future token outcomes remain deterministic,
- no hidden minting or balance mutation can occur.

The token system is designed to degrade **gracefully**, preserving correctness even when liveness is temporarily lost.


## 15. Conformance requirements [anchor: 15_conformance_requirements]

This section defines what it means for a node, client, or implementation to be **conformant** with the token system specification.

Conformance is binary: an implementation either conforms or it does not.

---

### 15.1 Mandatory computations [anchor: mandatory_computations]

A conformant implementation MUST:

- recompute POD at every cycle boundary,
- recompute POINT mint, melt, redistribution, and inheritance,
- apply pruning, fraud findings, and exclusions deterministically,
- respect rulebook parameters exactly,
- follow the cycle operation order precisely.

Skipping, approximating, or caching these computations is prohibited.

---

### 15.2 Forbidden shortcuts [anchor: forbidden_shortcuts]

A conformant implementation MUST NOT:

- store POD or POINT as authoritative state,
- adjust balances manually,
- use heuristics in place of defined equations,
- reorder cycle operations,
- condition token outcomes on non-canonical data,
- depend on external services for correctness.

Performance optimizations are allowed only if they preserve identical results.

---

### 15.3 Replay equivalence [anchor: replay_equivalence]

Given identical canonical inputs, all conformant implementations MUST:

- compute identical POD values,
- compute identical POINT balances,
- apply identical pruning outcomes,
- reach identical final state.

Any divergence constitutes non-conformance.

---

### 15.4 Explainability and auditability [anchor: explainability_and_auditability]

A conformant node MUST be able to explain, for any identity:

- why their POD has a given value,
- which events contributed to it,
- how routing paths were selected,
- how POINT minting and redistribution occurred.

Explanations MUST reference:
- canonical events,
- routing neighborhoods,
- share curves,
- rulebook parameters.

Black-box or opaque implementations are not permitted.

---

### 15.5 Test vectors and verification [anchor: test_vectors_and_verification]

Implementations SHOULD provide:

- deterministic test vectors for token computation,
- cycle-by-cycle verification outputs,
- tools for independent replay and audit.

Reference implementations MAY be provided, but conformance does not require using any specific codebase.

---

### 15.6 Partial implementations [anchor: partial_implementations]

Clients or lightweight nodes MAY omit:
- transfer UI,
- economic displays,
- optimization layers.

However, any component that claims to compute or display POD or POINT MUST fully conform to this specification.




## 16. Non-normative extensions and optional integrations [anchor: 16_non_normative_extensions_and_optional_integrations]

This section describes optional extensions and integrations that may be layered on top of the token system. These extensions are **non-normative**: they do not change canonical semantics, replay determinism, or constitutional guarantees. A conformant implementation may omit all extensions described here and remain fully valid.

---

### 16.1 Optional on-chain mirroring of POINT [anchor: optional_on_chain_mirroring_of_point]

POINT balances MAY be mirrored onto an external blockchain for purposes such as:
- external settlement,
- liquidity experimentation,
- audit anchoring,
- interoperability with other systems.

When enabled:
- the blockchain MUST be treated as a mirror, not an authority,
- only aggregated or batched updates MAY be published,
- no blockchain state may influence canonical token computation.

Typical approaches include:
- periodic Merkle roots of POINT balances,
- batched net-delta transactions per cycle,
- claim-based withdrawal systems.

All canonical truth remains off-chain and replay-derived.

---

### 16.2 Treasury pools and infrastructure funding [anchor: treasury_pools_and_infrastructure_funding]

Governance MAY define optional treasury pools funded by:
- a portion of POINT mint,
- voluntary contributions,
- unclaimed inheritance.

Treasury pools:
- MUST be transparent,
- MUST be rulebook-defined,
- MUST NOT influence POD, importance, or governance weight.

Treasury use cases include:
- infrastructure maintenance,
- development bounties,
- audits and security reviews.

Treasuries are economic tools, not political ones.

---

### 16.3 External economic interfaces [anchor: external_economic_interfaces]

POINT MAY be used as an interface to:
- marketplaces,
- grant systems,
- cooperative funding mechanisms.

Such integrations:
- MUST remain opt-in,
- MUST not affect internal token mechanics,
- MUST not introduce economic pressure into epistemic processes.

External valuation has no canonical meaning.

---

### 16.4 UI metaphors and narrative layers [anchor: ui_metaphors_and_narrative_layers]

User interfaces MAY present POD and POINT using metaphorical or narrative language, such as:
- energy flow and energy,
- sap and fruit,
- nutrients and harvest.

These metaphors:
- MUST faithfully reflect underlying mechanics,
- MUST NOT misrepresent authority or power,
- MUST be clearly distinguishable from canonical semantics.

---

## 17. Summary guarantees [anchor: 17_summary_guarantees]

This section summarizes the guarantees provided by the token system as a whole. These are emergent properties derived from the mechanics defined throughout this specification.

---

### 17.1 What POD guarantees [anchor: what_pod_guarantees]

POD guarantees that:
- importance remains a living, dynamic measure,
- recognition follows present relevance, not historical timing,
- no identity can hoard epistemic influence,
- all attribution is traceable and explainable.

POD provides visibility without power.

---

### 17.2 What POINT guarantees [anchor: what_point_guarantees]

POINT guarantees that:
- economic capacity aligns with current contribution,
- unused value decays and redistributes,
- coordination is possible without authority,
- no permanent economic elite can form.

POINT provides utility without control.

---

### 17.3 What the combined system guarantees [anchor: what_the_combined_system_guarantees]

Together, POD and POINT guarantee that:
- truth and importance are determined by human deliberation,
- economics follows understanding rather than shaping it,
- the system remains adaptive across generations,
- history is preserved without fossilizing power.

The token layer functions as the circulatory system of a collective mind: alive, self-regulating, and resistant to capture.



---

## Rationale (Short)

- The Lane U / Lane O split aligns mana cost semantics with the universal-substrate versus scoped-overlay architecture while remaining rulebook-controlled.
- Explicit tribe and published-personal-overlay constraints prevent reintroduction of tribe-only object creation as an active cost class without treating private owner ordering as canonical token activity.
- Deterministic lane assignment preserves replay equivalence across conformant nodes and avoids discretionary cost behavior.
