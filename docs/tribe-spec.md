---
doc_id: tribe_spec
title: Tribe Specification
status: authoritative
version: v0
last_reviewed: 2026-01-27

scope:
  - Defines tribe formation, membership, and public importance overlays.

authoritative_for:
  - Tribe semantics and public overlays.
  - Tribe-scoped deliberation constraints.

not_authoritative_for:
  - Governance mechanics beyond tribe scope.

depends_on:
  - protocol v5.md
  - governance-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of roles-and-stewardship-spec.md.

reader_path:
  - prereq: offline-and-mindseed-spec.md
  - next: roles-and-stewardship-spec.md

keywords:
  - tribes
  - membership
  - public overlays
---

> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

# Tribe Specification

---

## 0. Purpose, Scope, and Design Goals

This specification defines **tribes** as the system's mechanism for structured social coordination, contextual identity, and scoped deliberation, built entirely from existing protocol primitives. Tribes are not a separate object type, not a private space, and not a belief system. They are an emergent interpretation of ideas as shared importance anchors around which people coordinate and act, while remaining fully legible within the single canonical graph.

The purpose of tribes is to allow humans to organize meaningfully around ideas without fragmenting the epistemic commons, concentrating authority globally, or allowing isolated ideological enclaves to form. Tribes provide local structure while guaranteeing global context, so that difference can exist without separation.

A core design constraint of the system is that **no idea, group, or identity may exist without structural connection to the collective**. Tribes exist to express specialization and local alignment, not to create sovereign sub-communities. Every tribe is therefore embedded within broader contexts, and every member of a narrow tribe is always, structurally, also part of larger shared tribes up to the collective of all identities.

Tribes operate entirely within the canonical rules of Protocol v5. They reuse the existing idea model, relative importance connections, challenges, cycles, and deterministic replay semantics. No new consensus mechanisms, object types, or exception rules are introduced by this specification. Tribes do not introduce tribe-level governance systems or rulebooks. When interaction is restricted, it is restricted only through eligibility constraints applied to ordinary challenges, arguments, and votes.

This specification composes with and depends on:
- Protocol v5 (canonical ideas, identities, connections, importance, cycles)
- Challenge Engine Specification (challenge lifecycle, eligibility pools, voting determinism)
- Identity and Human Verification Specification (identity constraints and verification tiers)
- Token Specification (POD / POINT boundaries and incentive constraints)
- Rot and Burn Specification (lifecycle rules for living-map visibility)
- Offline and Mindseed Specification (offline operation and deterministic merge)

Nothing in this specification permits deviation from constitutional invariants defined elsewhere. Tribe-scoped constraints are strictly subordinate to protocol-level constraints.

The primary design goals of tribes are:
- To ground identity, authority, and visibility in **importance**, not popularity, permanence, or global reputation.
- To allow restricted coordination without restricting visibility.
- To make authority local, contextual, earned, and revocable, without leaking across unrelated domains.
- To ensure every individual and every idea remains structurally connected to the collective.
- To align social structure, incentive flow, and epistemic structure into a single coherent graph.

### 0.1 Composition and Non-Governance Clarification

Tribes do not introduce independent governance systems, constitutions, or rulebooks.

All actions that occur within tribes--idea creation, connection creation, challenges, arguments, voting, and verdicts--use exactly the same canonical primitives and lifecycle rules as the public system defined in Protocol v5 and the Challenge Engine Specification.

Tribes may restrict **who is eligible to perform actions**, but they may not introduce new forms of governance, override protocol rules, or define sovereign decision-making authority.

Any references elsewhere in the system to "rules," "admission," or "procedures" within tribes refer strictly to:
- eligibility constraints enforced through ordinary challenges and votes, and
- protocol-defined parameters (such as time-weighting and mana),

and never to tribe-local rulebooks or autonomous governance structures.

Tribes are coordination filters, not governing bodies.


## 1. Definition of a Tribe

A tribe is not a distinct entity type. A tribe is the **interpretation of an idea as a coordination anchor**. Every tribe-relative importance context is anchored to that public idea, while participation eligibility is derived from public membership state.

Every idea in the system implicitly defines a tribe. This is a fundamental invariant.

An idea's tribe exists whether or not anyone explicitly treats it as such. In most cases, the tribe associated with an idea will consist only of the identity that created it. In other cases, many identities may align importance to the same idea, forming a shared coordination context.

The idea itself is referred to as the **tribe anchor**. The tribe anchor is the sole canonical object; all tribe behavior is derived from the graph structure around it.

### 1.1 Open tribes

Every idea defines an **open tribe**.

The open tribe:
- Always exists.
- Is publicly visible.
- Has publicly declared/replay-derived membership plus the creator exemption.
- Cannot be disabled, hidden, or replaced.
- Serves as the canonical public context for the idea.

There is no such thing as an idea without an open tribe. Even ideas with only one member still define an open tribe.

### 1.2 Closed tribes as gatekept overlays

Closed tribes are **gatekept overlays** on top of an open tribe. They do not replace the open tribe and cannot suppress or modify it.

A gatekept overlay:
- Restricts **who may act** within a specific coordination context.
- Does not restrict visibility of any ideas, connections, arguments, or votes.
- Does not introduce new mechanics, authorities, or governance layers.

Gatekeeping decisions are made using ordinary challenges, arguments, and votes, with eligibility limited to members of the gatekept overlay. No special voting weights, offices, or permanent roles exist.

Gatekept overlays:
- Cannot waive public membership requirements.
- Cannot bypass time-weighted membership activation.
- Cannot override protocol-level invariants.

The open tribe always remains active and publicly accessible, regardless of any gatekept overlays that exist alongside it.


### 1.3 Tribe creation and the creator invariant

Creating an idea implicitly creates its open tribe. There is no separate "create tribe" action.

Every tribe has at least one creator identity: the identity that created the tribe anchor idea.

This creator invariant is permanent and immutable. The creator is always recorded in canonical history as the origin of the tribe anchor.

At creation time:
- The creator is automatically a member of the open tribe.
- No private importance position is authored, inferred, or exposed.
- This automatic inclusion does **not** count against tribe membership limits.

This exemption exists solely to prevent penalizing idea creation. It does not grant permanent authority, governance rights, or immunity from later loss of status.

Creator status:
- Does not grant permanent membership in closed tribes.
- Does not override importance qualification rules.
- Does not prevent rot, burn, or loss of influence.
- Persists only as historical provenance, not ongoing power.

### 1.4 Visibility and incentive invariants

All canonical ideas in the system are publicly viewable, subject only to protocol-defined safety and jurisdictional projection rules that affect payload exposure, not canonical existence.

Tribe membership MUST affect only action eligibility within tribe overlays. Tribe membership MUST NOT create tribe-private canonical object classes.

Tribe overlays:
- MUST remain publicly legible and auditable as canonical overlay state.
- MUST NOT hide underlying canonical ideas, representations, arguments, or challenge records.
- MUST remain a strict subset of universal canonical semantics.

Only canonical actions that satisfy universal incentive eligibility rules may affect POD and POINT. Tribe overlay activity is coordination state; it does not create a separate economic object domain.

Legacy references to "tribe-only ideas" or "mirroring into public ideas" are DEPRECATED in this specification and MUST be interpreted as historical vocabulary for overlay publication intent, not as active object semantics.

## 2. Tribe Membership: Core Concepts

Tribe membership expresses a person's **publicly attributable alignment** with the idea that anchors the tribe. It is recorded through the ordinary canonical membership connection, not inferred from the person's private importance map.

The system does not need separate "join tribe" or "leave tribe" object types: creating or ending the protocol-defined membership relationship expresses that transition. Active membership state is then derived, replayed, and audited from those public events and the applicable activation or decay rules.

### 2.1 Membership as a graph property

An identity's eligibility relationship to a tribe is determined entirely by:

- public canonical membership relationships to the tribe anchor,
- their activation, decay, or termination history, and
- the protocol and rulebooks active at the relevant cycle.

Relative-importance rankings may explain why a person cares about the tribe, but they do not create or revoke membership. No private map, hidden invitation, social contract, charter, or off-log agreement can substitute for public canonical membership state.

### 2.2 Explicit vs derived membership (high-level)

The system distinguishes between two structurally different kinds of membership:

- **Explicit membership**: public canonical membership state that grants the ability to act within a tribe after applicable activation rules.
- **Derived membership**: automatically implied membership required for structural context and continuity, which does not consume membership capacity.

This section defines the conceptual distinction only. The detailed mechanics of explicit membership are specified in Section 6, and the mechanics of derived membership are specified in Section 11.

### 2.3 Explicit public enrollment

An identity joins an open tribe by authoring the protocol's ordinary public membership relationship to the tribe anchor, subject to rate limits and activation rules. This is an attributable public action, not a disclosure of the identity's private relative-importance map.

Gatekept admission may add member-restricted eligibility checks, but no hidden invitation, private rank inference, or off-log decision may create canonical membership.

### 2.4 Membership is continuously recomputed

Tribe membership is not fixed at creation time and is not stored as a static flag.

Instead:

- membership states are recomputed from canonical membership events,
- time-weighted activation and decay apply, and
- all membership transitions are deterministic and replayable.

This ensures that tribes remain reflective of ongoing priorities rather than historical artifacts.

### 2.5 Visibility of membership state

An identity's membership state relative to a tribe is always publicly inspectable.

Observers may see:
- Whether an identity is an explicit member
- Whether membership is pending activation or decaying
- Which timeframe(s) grant membership
- How long the tribe anchor has remained important to the identity

This transparency is essential to accountability and trust.

---
### 2.6 Affiliation (interaction-based membership)

In addition to explicit and derived membership, the system recognizes **affiliation** as a weak, interaction-based relationship between an identity and a tribe.

Affiliation is derived automatically from observable interaction with tribe-related content, including:
- Viewing,
- Referencing,
- Participating in public challenges related to the tribe anchor or its descendants,
- Publishing or updating tribe-scoped overlay state.

Affiliation:
- Is unlimited.
- Grants no interaction rights.
- Confers no authority or privileges.
- Does not consume membership capacity.
- Does not satisfy eligibility requirements for tribe overlay writes or tribe-scoped challenge participation.

Affiliation exists solely for:
- Contextual framing,
- Analytics,
- Explainability,
- Understanding participation gradients.

Affiliation is descriptive, not normative, and has no effect on incentives, governance, or eligibility.

## 3. Structural Context and Relative Importance (Overview)

Tribes are not flat, isolated groups. They exist within a single, continuous structure defined by relative importance relationships among ideas.

This section provides a **conceptual overview** of that structure. The full formal definition of global tiering and mandatory upward membership chaining is specified later, in Section 11.

### 3.1 Relative importance as the sole structuring mechanism

All structure within the system--epistemic, social, and incentive-related--is derived from relative importance connections.

There is no separate clustering algorithm, taxonomy, or hierarchy system for tribes. Any apparent grouping or layering emerges directly from how ideas are connected by importance.

This applies equally to:
- Public ideas
- Tribe-scoped relative-importance overlays
- Tribe-scoped display override overlays
- Tribe maps
- Identity visibility
- POD routing

### 3.2 Reference-relative structure

All structure is interpreted relative to a **reference idea**.

Examples include:
- Any public idea as the reference for a public-relative rank context
- A tribe anchor as the reference idea for a tribe map
- An actionable idea as the reference idea for execution context

The same idea may occupy different structural positions depending on the reference idea used. These 10-axis relative contexts are distinct from the public 20-axis universal-importance product and its derived overall universal rank.

### 3.3 Local coherence and global embedding

Tribe-local structure (as seen in tribe maps) and global structure (as seen from the collective) are not separate systems.

They are different projections of the same graph.

This guarantees that:
- Tribe-local coordination remains coherent around its anchor idea.
- Every tribe is structurally embedded within broader contexts.
- No tribe can exist outside the collective frame.

### 3.4 Preview of upward chaining

One of the system's core anti-fragmentation guarantees is that membership in narrow tribes implies structural membership in broader tribes.

This property:
- Is derived from relative importance relationships between ideas
- Ensures that identities remain connected to shared contexts
- Prevents absolute social isolation

The formal rules governing this behavior are defined in Section 11.

### 3.5 Alignment with incentives

The same relative importance graph that structures tribes also determines how POD flows.

As a result:
- Structural position, social context, and incentive routing remain aligned
- No parallel hierarchies or shadow structures can form
- Changes to importance automatically reshape all three domains

This alignment is intentional and foundational to the system's design.


## 4. Tribe Overlays and Scoped Actions

Tribe activity in canonical state MUST be represented as scoped overlays on the universal canonical substrate.

A tribe overlay is anchored to a tribe anchor idea and MAY include:
- Tribe-scoped relative_importance overlay connections, and
- Tribe-scoped display overrides that reference already-canonical representation candidates.

Tribe overlays do not create private knowledge. They create restricted-action coordination over public canonical substrate.

### 4.1 Overlay semantics

A tribe overlay is identified by (scope_kind = tribe, anchor_id = tribe_anchor_idea_id).

Tribe overlays:
- MUST be represented in canonical events and deterministic replay.
- MUST remain publicly readable and auditable.
- MUST NOT create tribe-only canonical idea objects.
- MUST NOT duplicate canonical ideas through mirroring-as-copy semantics.

Restriction applies only to **who may act** in the overlay, never to **what canonical substrate exists**.

### 4.2 Eligibility matrix for tribe overlays

The following matrix is normative.

| Action class | Minimum eligibility | Additional constraints |
| --- | --- | --- |
| Tribe overlay writes (relative_importance create/update/delete) | Explicit membership in the open tribe anchored at anchor_id | Time-weighted activation complete; gatekept overlay rules MAY further restrict |
| Tribe overlay writes (scoped display override set/clear) | Explicit membership in the open tribe anchored at anchor_id | Target representation MUST already be canonical; gatekept overlay rules MAY further restrict |
| Tribe overlay challenge creation | Eligibility for the same (scope_kind=tribe, anchor_id) overlay | Challenge framing MUST use ordinary challenge domains and lifecycle rules |
| Tribe overlay challenge participation (arguments/votes) | Eligibility for the same (scope_kind=tribe, anchor_id) overlay | Voter and argument eligibility MUST be derived and replay-deterministic |

Gatekept overlays MAY restrict eligible identities further, but MUST NOT:
- Grant eligibility to identities lacking required public membership state.
- Accelerate or bypass time-weighted activation.
- Override protocol-level constraints.

All eligibility decisions MUST remain deterministic, replayable, and challengeable through ordinary canonical processes.

### 4.3 Visibility and auditability

All tribe overlay state MUST be:
- Indexed,
- Searchable,
- Auditable,
- Fully reconstructible through deterministic replay.

Non-members MAY observe:
- Overlay existence,
- Overlay authorship and provenance,
- Overlay challenge and argument history,
- Effective overlay state at any replay height.

Non-members MUST NOT gain write privileges solely by observing overlay state.

### 4.4 Incentive boundary

Tribe overlays are coordination state. They MUST NOT define a separate incentive-bearing object class.

Overlay activity MAY influence canonical deliberation only through ordinary public, challengeable mechanisms defined in protocol-level specs.

### 4.5 Historical terminology and deprecation

The following historical semantics are DEPRECATED in this specification and MUST NOT be used for new canonical interpretation:
- "Tribe-only idea" as a distinct canonical object category.
- "Mirroring a tribe-only idea into a public idea" as object-copy semantics.

Historical records that contain these terms MUST be interpreted as legacy vocabulary for scoped overlay publication or migration context.

## 5. Tribe Maps and Reference-Idea Structure

Tribe maps are structured, scoped views of the canonical idea graph used for coordination within a tribe. A tribe map does not introduce new data, objects, or mechanics. It is a constrained projection of the existing graph, rendered and interpreted relative to a specific reference idea.

The purpose of tribe maps is to ensure that all tribe-scoped coordination remains **coherent, legible, and structurally anchored** to the idea the tribe is formed around.

### 5.1 Reference idea invariant

Every tribe map is defined with the **tribe anchor idea as the reference idea**.

This is a hard invariant.

When viewing a tribe map:
- The tribe anchor idea is always the reference and conceptual center.
- All other ideas shown in the tribe map MUST be connected, directly or indirectly, to the tribe anchor via relative importance connections.
- No disconnected subgraphs are permitted.

A tribe map is therefore not an arbitrary collection of ideas. It is a structured field of relevance around a single coordinating idea.

### 5.2 Allowed structure within tribe maps

Within a tribe map:
- Only **relative importance connections** define structural relationships.
- Every idea must have at least one relative importance path that ultimately leads to the tribe anchor.

Other connection types that may exist globally (e.g., equivalence or membership connections) do not define placement or structure in tribe maps.

This constraint ensures that tribe maps answer a single question consistently:
### 5.3 Tribe overlay constraints

### 5.3 Tribe overlay constraints

Tribe maps MUST operate on universal canonical substrate ideas.

Within a tribe map, permitted tribe-scoped state is limited to:
- relative_importance overlays anchored to the tribe anchor, and
- tribe-scoped display overrides referencing canonical representation candidates.

Tribe maps MUST NOT introduce tribe-only canonical idea objects.

All tribe-scoped overlay state:
- MUST include deterministic anchoring to the tribe map reference idea,
- MUST remain structurally connected to the tribe anchor,
- MAY connect to multiple ideas within the map when those connections remain protocol-valid.

Any item that cannot be meaningfully connected to the tribe anchor via valid overlay state does not belong in that tribe map.

### 5.3A Tribe-relative rank lists [anchor: tribe_relative_rank_lists]

Every public idea may serve as a tribe anchor. The tribe's importance map ranks public candidate ideas relative to that anchor on ten axes:

- `important_to_reference` across the five protocol timeframes; and
- `important_for_reference` across the five protocol timeframes.

Each axis is an ordinal list. A valid lower-ranked candidate may challenge a higher-ranked candidate in the same complete context. The tribe anchor is the reference and is not a contestant. Eligible tribe members may contribute arguments and enter the deterministic juror-selection process; nonmembers may inspect the public challenge and resulting rank history but may not vote.

If the challenger wins and remains below the target when the verdict applies, it moves immediately above the target. No token balance, tribe mana, status, or popularity measure weights a ballot or supplies a rank score.

Tribe-relative rank state is a public overlay over the canonical substrate. It does not alter the 20-axis universal product, the derived overall universal rank, a public-relative context outside the tribe, or another tribe's rank state.

### 5.4 Tier formation within a tribe map

Tiers within a tribe map are defined **relative to the tribe anchor as the reference idea**, using the same primary-parent logic used elsewhere in the system.

For a tribe anchor idea T:

- Tier 0: the tribe anchor idea T itself.
- Tier 1: ideas whose highest relative importance connection is to T.
- Tier 2: ideas whose highest relative importance connection is to a Tier 1 idea rather than directly to T.
- Tier N: ideas whose highest relative importance connection is to a Tier (N-1) idea.

Tier numbering within tribe maps always follows this convention:
- The reference idea is Tier 0.
- The first ring surrounding it is Tier 1.

This produces concentric layers of increasing specificity and dependency radiating outward from the tribe anchor.

### 5.5 Multiple connections and tie handling

An idea may have relative importance connections to multiple ideas within the tribe map.

Tier assignment is determined by:
- The single strongest relative importance connection, or
- A deterministic tie-handling rule when multiple connections are within a defined threshold.

If multiple candidate parents are within threshold:
- The idea is assigned the lowest valid tier consistent with those parents.
- Tie-breaking rules MUST be deterministic and globally consistent.

This ensures that tribe maps remain stable, reproducible, and resistant to manipulation.

### 5.6 Structural meaning of tribe-local tiers

Tier position within a tribe map conveys **structural relationship**, not authority.

Tiers indicate:
- How directly an idea relates to the tribe's core concern.
- Whether an idea is foundational or derivative within the tribe's coordination space.
- How discussion and planning naturally flow from core concepts to refinements.

Tier position:
- Does not grant additional permissions.
- Does not override membership requirements.
- Does not affect POD or POINT eligibility.

### 5.7 Relationship to global structure

Tribe-local tiers are reference-relative views of the same canonical graph.

An idea may simultaneously:
- Be Tier 1 relative to a tribe anchor.
- Be Tier 3 or Tier 6 relative to the collective.

Both are correct within their respective contexts.

This reinforces a core system principle:
Structure is always interpreted relative to a reference idea. There is no single absolute hierarchy.

### 5.8 UI and explainability requirements

When a tribe map is presented, the system MUST make clear:
- Which idea is the reference idea.
- Why each idea appears at its assigned tier.
- Which relative importance connection determined that placement.

Participants MUST be able to inspect:
- The relevant relative importance values.
- The parent relationship used for tier assignment.

This guarantees that tribe maps remain transparent, inspectable, and understandable, preserving trust in their structure.

### 5.9 No divergence from the canonical graph

Tribe maps do not create parallel graphs or alternate histories.

All structure visible in a tribe map:
- Exists in the canonical idea graph.
- Is publicly viewable by non-members.
- Is fully reconstructible via deterministic replay.

Restriction applies only to **who may modify the structure**, never to who may observe it.

This preserves the system's foundational ethic: coordination without concealment.


## 6. Public Tribe Membership and Private-Rank Separation

Tribe challenge eligibility MUST be derived from public canonical membership state, never from an individual's private importance map.

An identity may express public alignment with a tribe through the protocol's ordinary `membership` connection to the tribe anchor. That public act is distinct from privately judging the anchor important. A node MUST NOT inspect, publish, or infer private rank positions to determine tribe membership.

Membership state and any activation/decay metadata are replay-derived from canonical membership events and the active protocol rules. A private owner may rank the same anchor highly, lowly, or not at all without changing public membership.

### 6.1 Membership as the sole eligibility signal

For tribe-scoped challenge creation, arguments, and voting, the eligibility signal is:

- an active canonical membership connection from the identity to the tribe anchor; or
- the narrowly defined creator exemption below.

There is no eligibility path based on private top-X lists, shared screenshots, AI inference, token balances, popularity, or hidden account state.

### 6.2 Explicit membership definition

An identity is an explicit member of a tribe anchored at idea T if and only if at least one of the following is true:

- an active canonical membership connection links the identity to T; or
- the identity is the creator of T under the creator exemption.

Explicit membership:
- Grants the right to perform tribe-scoped actions in the open tribe.
- Is required for participation in gatekept overlays associated with the tribe.
- Is required to perform tribe-scoped overlay writes within the tribe map.

Membership rate limits and activation rules MAY bound opportunistic mass joining, but membership scarcity MUST NOT be inferred from or enforced by exposing a private importance map.

### 6.3 Creator exemption

When an identity creates an idea, canonical provenance grants the creator the narrow membership exemption for the open tribe anchored by that idea. This does not author or reveal a private rank.

This grants the creator:
- Explicit membership in the corresponding open tribe.
- Eligibility to perform tribe-scoped actions.
- No requirement to publish a private importance judgment.

The creator exemption is **permanent with respect to the open tribe anchor**. Private reprioritization has no effect on it.

However, the exemption:
- Does not grant eligibility for gatekept overlays.
- Does not bypass time-weighting for other tribes.
- Does not confer permanent authority, visibility, or immunity from rot and burn.

Creator status persists solely as structural provenance and minimal participation eligibility in the open tribe.


### 6.4 Membership commitment metadata

An active rulebook MAY attach public commitment-horizon or activation metadata to a canonical membership connection. Such metadata is membership state, not a copy of an individual's private ten-axis ranking. If multiple public commitment conditions apply, the most conservative applicable activation condition governs.

### 6.5 Loss of explicit membership

Explicit membership is continuously replay-derived. It ends only through a valid membership-removal, suspension, lifecycle, or decay transition defined by the protocol. Private rank changes MUST NOT revoke it.

---

## 7. Time-Weighted Joining and Leaving

Creating a public membership connection need not grant immediate tribe-scoped power. Tribe membership MAY be **time-weighted** to prevent rapid cycling, opportunistic capture, and short-term manipulation.

Time-weighting applies uniformly to all tribes and is enforced at the protocol level.

### 7.1 Activation dwell time

The protocol defines a minimum dwell duration that may begin when the canonical membership connection becomes effective.

In general:
- Shorter timeframes have shorter dwell times.
- Longer timeframes have longer dwell times.

Until the dwell time has elapsed:
- The membership is visible as "pending."
- The identity may not perform tribe-scoped actions that require explicit membership.

### 7.2 Decay and removal time

Similarly, when a valid public membership-removal or expiry transition occurs, explicit membership need not vanish instantly.

Each timeframe defines a minimum decay duration:
- During decay, membership remains visible but is marked as expiring.
- After decay completes, explicit membership is fully revoked.

This asymmetry ensures that:
- Commitment must be sustained to gain power.
- Power does not disappear instantly due to transient reordering.

### 7.3 Transparency requirements

All tribe-scoped actions MUST display:
- The public membership basis through which the actor qualifies.
- The canonical duration for which that membership has remained active.

This makes opportunistic behavior legible without exposing the actor's private importance map.

### 7.4 Interaction with gatekept overlays

Time-weighting applies before any gatekeeping logic.

An identity must:
- First hold the required public membership state.
- Then satisfy dwell-time requirements.
- Only then may participate in gatekept tribe overlays.

Gatekeeping cannot accelerate or bypass time-weighting.

### 7.5 No acceleration mechanisms

There is no mechanism by which:
- Membership dwell time can be shortened.
- Decay time can be bypassed.
- Membership activation can be voted on or granted early.

All time-weighting behavior is protocol-defined, deterministic, and challengeable only through changes to the protocol itself.

This ensures that action-eligible membership reflects sustained public commitment, not a momentary or hidden signal.


## 8. Gatekept Tribes and Restricted Interaction

Tribes do not have governance systems, rulebooks, or internal constitutions. All ideas, connections, challenges, arguments, and votes follow **exactly the same rules** as the public system. The only distinction a tribe introduces is **who is permitted to perform those actions**.

Everything remains publicly visible at all times.

### 8.1 Open tribes (default)

Every idea defines an open tribe.

In an open tribe:
- Membership is derived from public canonical membership state plus the creator exemption.
- Any explicit member may perform tribe-scoped actions.
- No additional admission or approval exists.

Open tribes are the default and most common case.

### 8.2 Gatekept tribes

A gatekept tribe is a restricted overlay associated with an open tribe.

Gatekept tribes:
- Do not replace the open tribe.
- Do not hide the open tribe.
- Do not introduce new mechanics.

They only restrict **who may act** within that gatekept context.

### 8.3 Admission and exclusion

Gatekept tribes may choose who is allowed to participate.

Admission decisions:
- Are made by existing members of the gatekept tribe.
- Use the same challenge, argument, and voting mechanisms as the public system.
- Are fully public and auditable.

There are no special voting weights, roles, or authorities. Gatekeeping decisions are just ordinary challenges with a restricted eligible voter set.

### 8.4 Scope of restriction

Restriction in gatekept tribes applies only to overlay actions and associated challenge participation.

It MAY restrict:
- Creation, update, or deletion of tribe-scoped relative_importance overlay connections.
- Setting or clearing tribe-scoped display overrides.
- Opening tribe-scoped challenges tied to the same tribe overlay.
- Participation in tribe-scoped challenges (arguments and votes).

It MUST NOT restrict:
- Access to public canonical ideas.
- Read visibility of tribe overlay state.
- Interaction in the open tribe where eligibility permits.

### 8.5 Loss of eligibility

If an identity loses explicit membership in the open tribe through a valid membership transition:
- They immediately lose eligibility for all associated gatekept tribes.
- No gatekept decision can override this loss.

Gatekeeping is always subordinate to public membership eligibility.

### 8.6 Failure, stagnation, and decay

When a gatekept overlay loses active participation:
- Tribe-scoped overlay actions naturally decline.
- Overlay challenge throughput declines under mana and eligibility limits.
- The gatekept overlay becomes inert as a write surface.
- The open tribe remains unaffected and publicly visible.

No gatekept overlay can preserve activity without sustained eligible participation.

## 9. Identity, Pseudonymity, and Contextual Visibility

Identity within tribes is intentionally **contextual, earned, and local**. The system does not support global reputation, permanent status, or universal name recognition. An identity's visibility and recognition are always tied to **where** and **why** that identity is relevant.

### 9.1 Default pseudonymity

All identities are pseudonymous by default.

The underlying verified-human credential is required for participation but is **not public by default**.

By default:
- An identity is represented by a stable pseudonymous identifier.
- The identity's real-world name or persistent public persona is not shown.
- A non-identifying verification level MAY be shown (if available).
- Other participants may inspect the identity's public mindspace, including:
  - the ideas the identity has created,
  - the ideas the identity considers important,
  - the tribes the identity belongs to,
  - the challenges and actions the identity has participated in.

This ensures accountability through history and structure without enforcing global exposure.

### 9.2 Contextual identity reveal

An identity MAY choose to reveal a real name or public persona within a context. Importance level MAY permit such reveal but MUST NOT cause automatic disclosure. Default attribution remains pseudonymous unless the identity explicitly opts in.

Identity revelation is:
- Context-specific.
- Scope-limited.
- Revocable.
- Always opt-in; absence of a persona attachment preserves minimal attribution.

An identity MAY be shown non-pseudonymously when:
- The identity has high relative importance to a given tribe anchor idea, or
- The identity has high relative importance to an ancestor idea in the tier chain relevant to the current context.

Outside of those contexts, the identity remains pseudonymous, even if the same person is well-known elsewhere.

This ensures:
- You can be known where you have contributed meaningfully.
- You are anonymous everywhere else.
- Authority does not leak across unrelated domains.

### 9.3 Contextual roles

Roles are not global titles. They are contextual interpretations of relative importance.

The system recognizes conceptual roles such as:
- **Ant**: the default state for all identities in most contexts.
- **Lion**: an identity with high relative importance in an actionable or execution-oriented context.
- **Entling**: an identity with high relative importance in stewardship, maintenance, or long-term structural contexts.

These roles:
- Are derived from relative importance, not assigned.
- Exist only within specific idea or tribe contexts.
- Carry no protocol-level privileges beyond visibility and framing.
- Are lost automatically if relative importance declines.

Roles are descriptive, not prescriptive. They exist to help participants understand *why* a person is visible or influential in a given context.

### 9.4 Anti-celebrity invariant

The system explicitly forbids global fame.

There is no mechanism by which:
- An identity's name is universally visible.
- Reputation accumulates across unrelated domains.
- Authority in one area confers authority elsewhere.

Every instance of visibility must be justified by local importance.

---

## 10. Rot, Burn, and Lifecycle Effects in Tribes

Tribes do not alter lifecycle rules of canonical ideas or connections. Rot and burn apply uniformly, with tribe effects interpreted through overlay state rather than tribe-only object classes.

### 10.1 Overlay-state decay and inactivity

Tribe-scoped overlay state is subject to the same deterministic lifecycle and validity rules as other canonical state.

When tribe overlay state becomes inactive, superseded, or invalidated:
- Its historical record remains visible.
- Its active effect on current derived views ceases according to protocol rules.
- No hidden removal or discretionary erasure is permitted.

### 10.2 Interaction loss and decay

If participation in a tribe overlay declines:
- Eligible write throughput decreases,
- Overlay maintenance declines,
- Overlay relevance may decay naturally via ordinary challenge and lifecycle dynamics.

This behavior MUST be deterministic and replayable.

### 10.3 Decay dynamics

Overlay activity MUST NOT receive lifecycle immunity based on tribe size, gatekeeping status, or social prominence.

No decay behavior may be accelerated or slowed based on:
- Tribe size,
- Tier position,
- Breadth or narrowness of scope,
- Legacy terminology such as "tribe-only" versus "public".

Restriction limits *who may act*, never *whether lifecycle rules apply*.

### 10.4 Explainability and transparency

Whenever a tribe overlay surface is hidden, deprioritized, or removed from default views due to lifecycle or validity effects, the system MUST provide a clear explanation.

Explanations include:
- The rule that applied.
- The relevant activity or inactivity history.
- The cycle boundaries involved.

### 10.5 No immunity through restriction

No tribe, gatekept or open, may:
- Prevent lifecycle effects,
- Freeze overlay state in perpetuity,
- Shield overlay activity from deterministic evaluation.

This preserves integrity of the shared living map.

## 11. Global Tier Structure and Mandatory Collective Embedding

This section defines the **global structural invariants** that ensure all tribes, identities, and coordination contexts remain embedded within the collective. These rules are normative, protocol-level constraints. They are not UI conventions, social norms, or optional interpretations.

All other sections that reference "tiers," "parents," or "upward chaining" depend on the definitions in this section.

### 11.1 The collective as the distinguished reference idea

The system defines a distinguished idea representing **the collective of all identities**.

The collective:
- Is a canonical idea.
- Exists at all times.
- Serves as the root reference for global structural interpretation.

Every identity is a member of the collective by definition. As a result, any idea that is important to at least one identity is necessarily important to the collective in some degree.

### 11.2 Required relative importance anchoring

For every idea X in the system:

- There MUST exist a relative importance connection expressing that X is important to at least one identity (typically the creator).
- There MUST exist a relative importance connection expressing that X is important to the collective.

The collective-importance connection may be explicitly recorded or deterministically derived, but it MUST exist for purposes of structural computation.

This invariant guarantees that no idea is structurally unanchored.

### 11.3 Primary parent definition

Using the collective as the reference idea, define the **primary parent** of any idea X (where X is not the collective) as follows:

- Consider all ideas Y to which X has a relative importance connection, including the collective.
- Identify the idea Y for which the relative importance of X to Y is maximal.
- That idea Y is the primary parent of X.

If multiple candidate parents are within a deterministic threshold of maximal importance, a globally consistent tie-breaking rule MUST be applied.

### 11.4 Global tier assignment

Global tiers are defined recursively:

- The collective is Tier 0.
- For any other idea X:
  - `tier(X) = tier(primary_parent(X)) + 1`

Lower tiers represent broader, more encompassing ideas. Higher tiers represent narrower, more specific ideas.

Global tiers are not manually assigned, voted on, or curated. They emerge entirely from relative importance relationships.

### 11.5 Mandatory upward membership chaining

Global tiers impose a hard constraint on tribe membership.

If an identity is an explicit member of a tribe anchored at idea X:
- The identity MUST also be considered a member of the tribe anchored at the primary parent of X.
- This requirement applies recursively along the primary-parent chain.
- The chain always terminates at the collective.

This rule ensures that:
- Membership in a narrow tribe implies membership in broader tribes.
- No identity can exist solely within an isolated subgroup.
- All coordination is structurally contextualized within shared frames.

### 11.6 Derived versus explicit membership

Membership induced by upward chaining is **derived membership**.

Derived membership:
- Is automatic and non-optional.
- Does not consume explicit membership capacity.
- Exists solely to enforce structural continuity and interaction prerequisites.
- Confers no additional privileges beyond contextual inclusion.

Explicit membership:
- Is limited and scarce.
- Is derived from public canonical membership state (Section 6).
- Grants the ability to perform tribe-scoped actions.

This distinction preserves both scarcity and structural coherence.

### 11.7 Relationship to tribe-local structure

Global tier structure and tribe-local tier structure are derived from the same relative importance graph but use different reference ideas.

- Global tiers use the collective as the reference idea.
- Tribe-local tiers use the tribe anchor as the reference idea.

An idea may therefore occupy different tier positions in different contexts simultaneously. This is expected and correct.

### 11.8 Relationship to POD flow

The global tier structure is a static interpretation of the same graph used for POD routing.

POD flows:
- From ideas of higher universal importance,
- Down relative importance connections,
- Toward identities and actions.

Tier structure explains where an idea sits in relation to the collective. POD flow explains how value moves through that structure.

No separate hierarchy or clustering mechanism exists.

### 11.9 Anti-fragmentation invariant

The rules in this section enforce a non-negotiable system invariant:

For every identity and every tribe membership, there exists a continuous, inspectable structural path to the collective.

This invariant cannot be overridden by:
- Gatekept overlays,
- Deprecated tribe-only coordination semantics,
- Identity visibility rules,
- Offline operation,
- Or any other mechanism.

All disagreement, specialization, and coordination occurs within a shared collective frame.


## 12. System-Level Guarantees and Anti-Fragmentation Properties

The tribe system exists to enable coordination without allowing fragmentation, capture, or hidden power accumulation. This section summarizes the guarantees enforced by the preceding rules.

### 12.1 No hidden coordination

At no point can a tribe:
- Hide its ideas.
- Hide its discussions.
- Hide its structure.
- Hide its membership history.

Restriction applies only to who may act, never to what exists or what can be seen.

### 12.2 No sovereign subgroups

Tribes do not have:
- Independent governance systems.
- Independent rulebooks.
- Independent consensus mechanisms.
- Independent incentive structures.

All actions within tribes use the same primitives as the public system:
ideas, connections, challenges, arguments, votes, and cycles.

The only distinction is eligibility.

### 12.3 No global authority leakage

Authority, visibility, and recognition are always contextual.

An identity may be:
- Highly visible in one tribe.
- Anonymous in another.
- Pseudonymous in most contexts.

There is no mechanism by which importance or authority in one domain automatically transfers to another.

### 12.4 No permanent identity capture

Tribe membership:
- Is importance-derived.
- Is time-weighted.
- Decays naturally without reinforcement.

No identity can permanently occupy a tribal role without sustained alignment and participation.

### 12.5 Guaranteed collective embedding

For every identity and every tribe membership:
- There exists a continuous structural path to the collective.
- That path is inspectable.
- That path is enforced.

This ensures that all disagreement, specialization, and conflict occurs within a shared human frame.

### 12.6 Summary invariant

The tribe system guarantees that:
- All ideas are visible.
- All coordination is legible.
- All identities are accountable.
- All groups are contextualized.
- The collective is never bypassed.

Tribes allow people to differ, not to disappear from one another.


## 13. Offline, Mindseed, and Deterministic Merge Behavior

Tribe behavior is fully compatible with offline operation, delayed synchronization, and deterministic merge. Tribe scoping does not introduce special cases, forks, or discretionary resolution during replay. All tribe-related effects emerge from canonical events and are recomputed deterministically.

### 13.1 Offline participation in tribes

An identity operating offline MAY:
- Reorder importance lists.
- Create tribe-scoped overlay writes (within cached eligibility), including relative-importance overlay edits and scoped display override edits.
- Participate in tribe-scoped challenges, arguments, and votes.

Offline clients MUST:
- Enforce tribe eligibility based on the locally known state.
- Record all actions as canonical events.
- Preserve full provenance for later merge.

No offline action may bypass importance alignment or tribe eligibility rules.

### 13.2 Merge-time re-evaluation

Upon merge:
- Importance lists are recomputed.
- Explicit membership is recalculated.
- Derived (tier-based) memberships are recomputed.
- Tribe mana availability is recalculated.
- Tier placement in tribe maps is recomputed.

If an offline action is found to violate eligibility at merge time:
- The action remains in history.
- Its effects are voided or rolled back according to protocol rules.
- The invalidity is explicitly recorded and explainable.

There is no discretionary judgment during merge. Outcomes are deterministic.

### 13.3 Tribe maps under offline conditions

Tribe maps do not diverge offline.

Offline clients MAY display tribe maps using locally available data, but:
- The reference-idea invariant still applies.
- Tier placement remains derived from relative importance.
- Any uncertainty MUST be marked as provisional.

After merge, all tribe maps are recomputed from the canonical graph.

### 13.4 No offline privilege escalation

Offline operation:
- Does not grant additional tribe privileges.
- Does not allow bypassing gatekept eligibility.
- Does not allow stockpiling tribe mana.

All privileges remain subject to recomputation.

---

## 14. Security, Abuse Resistance, and Failure Modes

The tribe system is designed to resist capture, fragmentation, and covert coordination while preserving freedom of association and expression.

### 14.1 Anti-brigading guarantees

The system resists brigading through:
- Rate-limited public membership actions.
- Replay-derived activation and eligibility.
- Time-weighted activation and decay.
- Tribe mana throttling.
- Mandatory upward membership chaining.

Rapid influxes of coordinated identities cannot instantly acquire tribe control.

### 14.2 Anti-sockpuppet properties

Sockpuppet amplification is constrained because:
- Public membership and activation requirements must be sustained.
- Membership slots are scarce.
- Tribe mana limits throughput.
- Identity verification affects capacity but not authority.
- All actions are publicly visible and inspectable.

Sockpuppets may exist, but they cannot cheaply dominate coordination.

### 14.3 No hidden power accumulation

Tribes cannot:
- Accumulate hidden authority.
- Create private decision structures.
- Shield actions from public inspection.
- Convert internal coordination into incentives without publication.

All power remains legible.

### 14.4 Failure and decay behavior

If a tribe becomes inactive:
- Tribe overlay state naturally becomes stale or inert under ordinary lifecycle and challenge dynamics.
- Tribe mana pools lie dormant.
- Gatekept overlays become inert as write surfaces.
- The open tribe remains as a public context.

No tribe can permanently freeze a coordination structure.

### 14.5 Attack surface minimization

Tribes introduce no new primitives.

They reuse:
- Ideas
- Relative importance
- Challenges
- Voting
- Cycles
- Deterministic replay

This minimizes the attack surface and simplifies verification.

### 14.6 Final security invariant

The tribe system ensures that:
- Coordination never implies concealment.
- Restriction never implies secrecy.
- Association never implies isolation.
- Power never escapes visibility.

All disagreement occurs in the open, and all structure remains accountable to the collective.

## 15. Tribe Mana

Tribe mana is a protocol-level rate-limiting mechanism that governs how quickly identities may perform **tribe-scoped actions**. Its purpose is to prevent internal spam, force prioritization of coordination, and ensure that restricted interaction spaces cannot be used to overwhelm the system or simulate hidden authority.

Tribe mana does not grant power. It constrains throughput.

### 15.1 Purpose and design principles

Tribe mana exists to enforce the following constraints:

- Coordination within a tribe must be scarce and deliberate.
- Restricted interaction must never be cheaper or faster than public interaction.
- Narrower, more specialized coordination spaces must face stronger throughput limits.
- No tribe may bypass protocol-level rate limits through internal structuring.

Tribe mana is strictly subordinate to all protocol invariants. It cannot override importance alignment, time-weighting, eligibility rules, or deterministic replay.

### 15.2 Mana pools

Each identity has multiple mana pools:

- **Personal mana**, used for public, non-tribe-scoped actions.
- **Per-tribe mana**, scoped to each tribe in which the identity has explicit membership.

Per-tribe mana pools:
- Exist only while explicit membership is active.
- Are distinct for each tribe.
- Cannot be transferred, pooled, shared, or delegated.
- Cannot be converted into POD, POINT, or any other asset.

Derived (upward-chained) memberships do not create mana pools.

### 15.3 Actions that consume tribe mana

Tribe mana is consumed only by tribe-scoped overlay actions and tribe-scoped challenge participation, including:

- Creating, updating, or deleting tribe-scoped relative_importance overlay connections.
- Setting or clearing tribe-scoped display overrides.
- Opening tribe-scoped challenges.
- Submitting arguments in tribe-scoped challenges.
- Voting in tribe-scoped challenges.

Tribe mana MUST NOT be consumed by read-only actions, inspection, or viewing of public ideas or tribe overlay state.

Deprecated historical note: references to spending tribe mana on creating "tribe-only ideas" or on "mirroring" actions are legacy semantics and MUST NOT apply to new rule interpretations.

### 15.4 Regeneration rules

Tribe mana regenerates deterministically over cycles.

Tribe mana is not primarily balanced by making some tribes regenerate faster or slower. Instead, it is balanced by making **the per-action mana cost depend on tribe scale**, so that total throughput does not grow unfairly as a tribe gains more members.

The regeneration rate is protocol-parameterized and may depend on:
- The timeframe through which explicit membership is granted (longer timeframes may regenerate more slowly than shorter timeframes).

However, the dominant scaling mechanism is **cost**, not regeneration.

As a general structural effect:

- In smaller tribes, tribe-scoped actions cost less tribe mana per action, because fewer members exist to contribute total mana to the tribe's coordination capacity.
- As a tribe becomes larger (more eligible members), the per-action cost increases, approaching the equivalent public-action cost, because the tribe's total available mana (summed across members) increases.

This ensures that:
- Small tribes can coordinate meaningfully without being throttled into uselessness.
- Large tribes do not gain disproportionate total throughput simply by having many members.
- Tribe-overlay coordination cannot become cheaper public action at scale.

The cost scaling function is protocol-parameterized and deterministic, and it must be based only on publicly inspectable values (such as the current eligible member count for the relevant tribe context).

Deprecated historical note: mirroring-as-copy cost rules are retained only for legacy interpretation and MUST NOT define new canonical behavior.

No tribe-scoped action, at full scale, is cheaper or faster than the public system allows for the equivalent action.


### 15.5 Interaction with time-weighted membership

Tribe mana becomes available only after explicit membership has fully activated.

While membership is:
- **Pending**: no tribe mana is available.
- **Active**: tribe mana regenerates normally.
- **Decaying**: tribe mana regeneration halts.
- **Revoked**: any remaining tribe mana is discarded.

There is no mechanism to preserve, bank, or carry over tribe mana across membership loss.

### 15.6 Gatekept overlays and mana

Gatekept overlays do not introduce new mana pools.

All tribe-scoped actions within a gatekept overlay:
- Consume mana from the identity's per-tribe mana pool for the parent open tribe.
- Are subject to the same regeneration and depletion rules.

Gatekeeping may restrict who is eligible to spend mana, but it cannot:
- Increase available mana.
- Accelerate regeneration.
- Create overlay-specific reserves.

### 15.7 No accumulation or amplification

Tribe mana cannot be:
- Accumulated across tribes,
- Amplified by membership stacking,
- Converted into influence or incentives,
- Traded, delegated, or lent.

An identity active in many tribes is constrained by having many small, slow-regenerating mana pools, not by having greater total capacity.

This enforces genuine prioritization of attention and effort.

### 15.8 Transparency and explainability

For every tribe-scoped action, the system MUST make inspectable:
- The mana pool consumed.
- The remaining mana balance.
- The regeneration rate.
- The rule that determined the cost.

If an action is blocked due to insufficient mana, the system MUST explain:
- Which pool was exhausted.
- When regeneration will permit the action.
- Why that rate applies.

### 15.9 Relationship to incentives

Tribe mana has no relationship to POD or POINT.

- Spending tribe mana never earns POD or POINT.
- Holding tribe mana conveys no authority.
- Mana depletion does not affect incentive routing.

This separation ensures that coordination capacity cannot be converted directly into economic or epistemic power.

### 15.10 Invariant

Tribe mana enforces a hard system invariant:

Restricted coordination can never be cheaper, faster, or less accountable than public action.

All coordination remains bounded by time, visibility, and human limits.


## 16. Summary of Tribe Invariants

The tribe system enforces the following non-negotiable invariants:

- Every idea implicitly defines an open tribe.
- Every tribe has a creator recorded immutably.
- Tribe membership is replay-derived from public canonical membership state, never from private importance rank.
- Tribe activity in canonical state is overlay-only (relative_importance + scoped display overrides).
- Tribe overlays are publicly readable and auditable.
- Restricted interaction never implies secrecy.
- No tribe has independent governance or rulebooks.
- Time-weighted activation and decay apply uniformly.
- Tribe mana applies only to overlay actions and tribe-scoped challenge participation.
- No identity may exist solely within an isolated subgroup.
- All tribe membership chains structurally to the collective.
- All coordination remains inspectable, replayable, and accountable.

Tribes allow difference without fragmentation and coordination without concealment.

## 17. Deprecated Semantics

The following semantics no longer apply as active protocol behavior in this specification:

- Tribe-only idea objects as a distinct canonical object class.
- Mirroring-as-copy from tribe-only ideas into public ideas.
- Incentive-splitting rules tied to mirroring copied objects.
- Mana spending rules tied to creating tribe-only objects.

Historical records MAY still contain this terminology. Such records MUST be interpreted as legacy context and MUST NOT override the overlay-only tribe model defined in this document.
