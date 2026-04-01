---
doc_id: roles_and_stewardship_spec
title: Roles and Stewardship Specification
status: authoritative
version: v0
last_reviewed: 2026-01-27

scope:
  - Defines stewardship roles layered over canonical semantics.

authoritative_for:
  - Stewardship role definitions and constraints.
  - Role-based non-canonical overlays.

not_authoritative_for:
  - Canonical authority or governance weighting.

depends_on:
  - protocol v5.md
  - tribe-spec.md
  - governance-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of ai-boundaries-spec.md.

reader_path:
  - prereq: tribe-spec.md
  - next: ai-boundaries-spec.md

keywords:
  - roles
  - stewardship
  - ents
---

> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

## 0. Purpose and scope

### 0.1 Purpose

This specification defines how roles of responsibility and stewardship operate within the Protocol v5 ecosystem **without granting authority, altering canonical semantics, or introducing hierarchy**.

This specification formalizes how centralized systems and individuals can transition toward collective care and stewardship over time, how humans and AI participate symmetrically in stewardship goals, and how accountability remains local, revocable, and auditable. Roles defined herein are **overlays on participation**, not sources of power.

This specification formalizes how centralized systems and individuals can transition toward collective care and stewardship over time, how humans and AI participate symmetrically in stewardship goals, and how accountability remains local, revocable, and auditable. Roles defined herein are **overlays on participation**, not sources of power.

---

### 0.2 Scope

This specification governs:

the baseline Ant role;  
responsibility overlays (Lion, Entling, Ent);  
idea-anchored representation and scoped responsibility;  
tribe election of Human Entlings;  
interaction between roles and canonical protocol mechanics.
revocation, demotion, and accountability of role claims;  
interaction between roles and canonical protocol mechanics.

This specification does not define new idea types, alter voting rules, grant governance authority, or modify Protocol v5 semantics. All canonical authority, eligibility, and state transitions remain governed exclusively by Protocol v5 and associated rulebooks.

---

### 0.3 Role overlays as protocol-visible claims

No role may exist outside the canonical challenge framework. All role claims MUST be representable, challengeable, and reversible using ordinary Protocol v5 mechanisms.

No role may exist outside the canonical challenge framework. All role claims MUST be representable, challengeable, and reversible using ordinary Protocol v5 mechanisms.

---

## 1. Core invariants

### 1.1 Roles are overlays, not authorities

Roles defined in this specification do not grant authority.

In particular, roles MUST NOT grant voting weight, activate or deactivate rulebooks, mint or influence POD or POINT, bypass challengeability, or modify canonical semantics. All canonical authority remains defined exclusively by Protocol v5.

Any behavior associated with a role MUST be achievable through ordinary protocol actions available to all participants, subject to the same validation, challenge, and accountability rules.

---

### 1.2 Ant is always the base role

Every identity within the Protocol v5 ecosystem is always an Ant.

All other roles apply only within explicitly scoped responsibility contexts and never replace baseline equality of participation. When an identity is not acting within the scope of a specific responsibility claim, it MUST be treated as an Ant in all interfaces, eligibility models, and protocol interactions.

---

### 1.3 No role-based eligibility shortcuts

Roles MUST NOT be used as eligibility shortcuts for challenge voting, rulebook activation, moderation authority, visibility control, or token issuance decisions.

If any rulebook permits role-related behaviors, those behaviors MUST remain fully challengeable and MUST NOT bypass Protocol v5 invariants. Roles may shape expectations and accountability, but never eligibility or authority.

---

### 1.4 Roles are descriptive and contractual, not ontological

Roles describe responsibility commitments, accountability surfaces, and stewardship expectations. They do not describe intrinsic status, rank, superiority, or identity essence.

Role status is contextual, scoped, and contingent on continued fulfillment of declared responsibilities. No role confers inherent legitimacy beyond what is earned through visible, challengeable behavior.



## 2. Ant (baseline participant role)

### 2.1 Definition

The **Ant** role represents ordinary participation in the shared reasoning, deliberation, and challenge system defined by Protocol v5.

Every verified identity participates as an Ant by default. The Ant role establishes the baseline expectations, permissions, and constraints that apply uniformly to all participants in the absence of explicitly scoped responsibility claims.

---

### 2.2 Properties

The Ant role applies to all identities at all times.

As Ants, identities may create ideas, submit arguments, participate in challenges, vote where eligible, and perform all other protocol-defined actions available under Protocol v5 and applicable rulebooks. The Ant role enforces equality of participation by ensuring that all actions are governed solely by protocol-defined eligibility rules rather than role-based privilege.

No Ant possesses inherent authority, special standing, or immunity by virtue of participation alone. All Ant actions remain subject to challenge, validation, and accountability under Protocol v5.

---

### 2.3 Ant equality outside responsibility contexts

When an identity is not acting within the scope of an explicitly declared responsibility role, that identity MUST be treated as an Ant in all interfaces, eligibility models, and protocol evaluations.

Responsibility roles do not permanently alter identity status. Outside their declared scope, all participants revert to baseline Ant equality, ensuring that responsibility claims cannot be used to assert generalized authority or privilege.

---

## 3. Lion (centralized authority holder)

### 3.1 Definition

A **Lion** is an identity that holds centralized operational control over a system, institution, or actionable idea whose decisions materially affect others.

Lionhood describes the existence of unilateral or concentrated power, not its moral quality, legitimacy, or desirability. An identity may be a Lion by circumstance, necessity, design, or historical contingency.

---

### 3.2 Descriptive, not legitimizing

Being identified as a Lion neither justifies nor condemns the exercise of power. Lion status does not imply wrongdoing, virtue, endorsement, or rejection. It exists to make power legible so that its impacts can be understood, discussed, and challenged where appropriate.

Being identified as a Lion neither justifies nor condemns the exercise of power. Lion status does not imply wrongdoing, virtue, endorsement, or rejection. It exists to make power legible so that its impacts can be understood, discussed, and challenged where appropriate.

---

### 3.3 Scope of application

Lion status applies only with respect to a specific system, institution, or actionable idea over which centralized control is exercised.

Outside the declared scope of that control, the identity MUST be treated as an Ant. Lionhood is never a global identity status and MUST NOT be generalized beyond its explicitly defined boundaries.

---

### 3.4 Lionhood as a claim about a concrete system boundary

A Lion claim MUST specify the concrete boundary of control being asserted.

At minimum, a Lion claim MUST identify the system or actionable idea under control, the type of control exercised (such as operational, financial, infrastructural, or policy control), and the surface area of impact, described at a high level in terms of who is affected and how.

Lion claims MUST remain narrowly scoped and MUST NOT imply authority outside the declared boundary.

---

### 3.5 Challengeability of Lion claims

Claims that an identity is, or is not, a Lion with respect to a particular system or idea MAY be challenged using ordinary Protocol v5 mechanisms.

Such claims MAY be disputed as inaccurate, outdated, incomplete, or misleading and MUST remain updateable as real-world conditions, control structures, or operational realities change.





## 4. Individual initiative and creation

### 4.1 Freedom to create

Individuals MUST be free to originate ideas, build systems, lead initiatives, and experiment without prior approval from the protocol, tribes, or governance bodies.

Centralized creation is valid and often necessary. The protocol does not require consensus, permission, or distributed participation as a precondition for initiating new ideas or systems. Innovation, experimentation, and leadership are explicitly protected activities within the ecosystem.

No role, rulebook, or governance process may prohibit creation solely on the basis of potential impact or scale.

---

### 4.2 No prohibition by scale

The protocol does not restrict creation based on projected importance, anticipated reach, or speculative future impact.

Responsibility does not precede creation. Responsibility scales only after impact emerges and becomes legible within the protocol. This ensures that individuals are not deterred from building, exploring, or experimenting by premature governance burdens.

The protocol recognizes that many systems begin as small, centralized, or informal initiatives and only later require broader accountability as their influence grows.

---

### 4.3 Distinction between building and governing

This specification draws a clear distinction between building something and governing something that materially affects many others.

Building refers to initiative, experimentation, and creation. Governing refers to ongoing decision-making that shapes outcomes for a wider population and therefore demands accountability.

The protocol favors enabling creation while making growing impact visible and debatable. It does not collapse initiative into governance, nor does it treat early leadership as illegitimate simply because it is centralized.

---

## 5. Impact-based responsibility growth

### 5.1 Impact principle

As an idea, system, or actionable initiative grows in importance and affects more lives, the legitimate interest of those affected increases.

This interest implies a growing expectation of deliberation, influence, and accountability. It does not imply ownership, seizure, or forced transfer of control. The protocol recognizes interest without asserting entitlement to command.

Impact is measured by legibility within the protocol rather than by coercive thresholds or external enforcement.

---

### 5.2 Stewardship over control

The protocol introduces stewardship as an alternative to unilateral authority, not as a replacement imposed by force.

Stewardship emphasizes care, maintenance, explanation, and responsiveness to those affected by an idea or system. It seeks to transform power through legitimacy, trust, and demonstrated competence rather than through mandate or expropriation.

This model allows centralized systems to evolve toward collective care without denying the validity of leadership, initiative, or expertise.

---

### 5.3 Impact through protocol legibility, not coercion

Instead, accountability grows through making impacts visible, organizing collective deliberation, and offering superior coordination and legitimacy compared to opaque or unilateral systems.

Instead, accountability grows through making impacts visible, organizing collective deliberation, and offering superior coordination and legitimacy compared to opaque or unilateral systems.

The protocol does not compel compliance. It competes by providing clearer understanding, better reasoning, and more trustworthy stewardship structures as impact scales.



## 6. Entling (stewardship candidate role)

### 6.1 Definition

An **Entling** is an identity that has taken on an explicit, declared responsibility to steward a specific idea, system, or tightly scoped cluster of ideas within the Protocol v5 ecosystem.

Entling status represents a voluntary assumption of care and accountability. It does not imply authority, expertise, legitimacy, or endorsement. An identity becomes an Entling only with respect to the scope they explicitly claim and remains an Ant outside that scope.

---

### 6.2 No authority grant

Entlings do not gain authority by virtue of stewardship.

In particular, Entlings do not gain voting power, do not decide challenge outcomes, do not activate rulebooks, and do not speak authoritatively for others. Entlings cannot bind participants, override deliberation, or claim privileged interpretation.

The function of an Entling is to ensure care, continuity, representation of context, and long-term reasoning health for the ideas or systems they steward, not to command or control them.

---

### 6.3 Bounded and explicit responsibility

An Entling role MUST be declared explicitly and with bounded scope.

At minimum, an Entling claim MUST specify the idea or system being stewarded, the stewardship tasks the identity commits to performing, the scope boundary including what is explicitly out of scope, and the expected cadence, if any, of stewardship outputs such as audits, summaries, explanations, or maintenance reports.

Ambiguous, global, or open-ended stewardship claims are non-conformant. Entling responsibilities exist only within the bounds declared and accepted as challengeable claims under Protocol v5.

---

### 6.4 Ordinary protocol actions

Entling work MUST be performed through ordinary protocol actions available to all participants.

Entlings possess no privileged execution path, special interface, or bypass mechanism. Their effectiveness derives from sustained contribution, clarity, and trust, not from elevated permissions.

Entlings possess no privileged execution path, special interface, or bypass mechanism. Their effectiveness derives from sustained contribution, clarity, and trust, not from elevated permissions.

---

## 7. Idea-anchored representation

### 7.1 One Entling per idea by default

Entlings are anchored to individual ideas or to tightly scoped clusters of closely related ideas. They are not anchored to populations, platforms, institutions, or abstract collectives.

By default, an idea SHOULD have at most one Entling at a time unless a rulebook explicitly permits multiple Entlings for the same scope under clearly defined conditions.

---

### 7.2 Accountability advantage

Idea-anchored stewardship prevents diffuse or ambiguous representation and ensures that responsibility remains local and legible.

Anchoring stewardship to specific ideas or systems creates visible accountability surfaces, identifiable failure modes, and precise responsibility boundaries. This allows stewardship quality to be evaluated, challenged, and compared without conflating unrelated domains.

---

### 7.3 Preventing broad representative drift

Entlings MUST NOT claim representation, stewardship, or authority beyond their explicitly anchored scope.

If an identity wishes to steward multiple ideas or systems, it MUST hold multiple explicit Entling claims, each independently declared, bounded, and challengeable. Broad or implicit representation claims are non-conformant and MUST be challengeable as such.




## 8. Tribe-elected Human Entlings

### 8.1 Election mechanics

Major idea-tribes MAY elect one or more Human Entlings to steward ideas or systems anchored to that tribe.

Entling elections MUST preserve one-person-one-vote among eligible tribe members, MUST be public and challengeable, and MUST be governed by tribe rulebooks consistent with universal constraints and Protocol v5 invariants.

Elections confer no authority and do not create representatives with decision-making power. They establish visibility, responsibility, and accountability for stewardship tasks within a defined scope.

---

### 8.2 Role of Human Entlings

Human Entlings steward idea representation and reasoning health within their declared scope.

Their responsibilities MAY include surfacing neglected concerns, assisting in audits and summaries, maintaining contextual continuity across related ideas, and acting as liaisons between adjacent idea clusters.

Human Entlings do not act as political representatives, delegates, or spokespersons. They do not negotiate on behalf of others, bind collective decisions, or claim authority beyond their explicitly declared stewardship scope.

---

### 8.3 Clarification on one-person-one-vote

This constraint does not alter, replace, or supersede Protocol v5 voting rules for challenges, verdicts, or other canonical processes. Entling elections are role-selection mechanisms, not governance mechanisms.

This constraint does not alter, replace, or supersede Protocol v5 voting rules for challenges, verdicts, or other canonical processes. Entling elections are role-selection mechanisms, not governance mechanisms.

---

### 8.4 Sybil resistance and identity verification coupling

Entling elections MUST rely on identity verification mechanisms defined in identity-related specifications and applicable rulebooks so that tribe membership is meaningful, votes represent distinct humans, and elections cannot be captured by fake or duplicate identities.

This specification does not define verification mechanics. It requires only that Entling elections be compatible with, and constrained by, whatever verification and Sybil-resistance systems are active under Protocol v5 governance.

---

### 8.5 Major idea-tribe determination

Eligibility to elect Human Entlings as a major idea-tribe MUST be deterministically defined by rulebooks consistent with universal constraints.

---

---

## 9. Tribe-hosted AI Entlings

### 9.1 Symmetry with humans

AI Entlings mirror Human Entlings in scope, constraint, and non-authority.

They are idea-anchored, advisory only, and non-authoritative. AI Entlings exist to assist stewardship tasks within a declared scope and are subject to the same expectations of bounded responsibility and challengeability.

---

### 9.2 Cross-reference to AI Boundaries Specification

All technical constraints, lineage requirements, sandbox isolation rules, and governance interactions affecting AI Entlings are defined in the *AI Boundaries Specification*.

This document defines only the role symmetry and stewardship framing for AI Entlings. In the event of any conflict, the AI Boundaries Specification and Protocol v5 remain authoritative.

---

### 9.3 AI Entlings as tools, not electable agents

AI Entlings are not electable persons and do not participate in elections.

They are selected, equipped, or endorsed through tribe-defined processes as tools that assist stewardship tasks. AI Entlings do not represent tribes as agents, do not express collective intent, and do not possess standing, legitimacy, or authority within the canonical universe.



## 10. Ent (full stewardship role)

### 10.1 Definition

Ent recognition does not create authority. It is a descriptive acknowledgment that an identity has consistently performed stewardship work in a way that others may reasonably rely upon as an aid to understanding, maintenance, and long-horizon reasoning.

Ent recognition does not create authority. It is a descriptive acknowledgment that an identity has consistently performed stewardship work in a way that others may reasonably rely upon as an aid to understanding, maintenance, and long-horizon reasoning.

---

### 10.2 Ent-hood is revocable

Ent status grants no authority, confers no permanence, and creates no entitlement.

Recognition as an Ent MAY be suspended or removed at any time through ordinary challenge and governance mechanisms defined by Protocol v5 and applicable rulebooks. Ent status is contingent on continued fulfillment of declared stewardship responsibilities and remains subject to scrutiny, dispute, and reassessment.

No identity may claim irrevocable or inherent Ent status.

---

### 10.3 Stewardship, not command

Ents maintain systems; they do not rule them.

The function of an Ent is to care for ideas, reasoning structures, safety explanations, or maintenance surfaces by improving clarity, continuity, and long-term coherence. Ents do not issue commands, enforce outcomes, or override deliberation.

Ent stewardship is exercised through explanation, maintenance, and visible contribution rather than directive authority.

---

### 10.4 Recognition without authority

Recognition as an Ent indicates that the community can observe an established record of stewardship behavior and that others may choose to rely on that record as an aid to deliberation.

Such reliance is voluntary and non-binding. Ent recognition MUST NOT grant special permissions, eligibility, interface priority, canonical power, or governance influence. Ent status does not modify Protocol v5 semantics or participation equality.

---

### 10.5 Scoped stewardship declarations

Ent status MUST be explicitly scoped.

An Ent MUST declare the domains they steward, such as specific ideas, clusters of ideas, safety or visibility explanation surfaces, or maintenance domains. An Ent MUST also declare which responsibilities they accept and which they explicitly do not accept.

Ent recognition applies only within the declared scope. Outside that scope, the identity MUST be treated as an Ant.

---

## 11. Growth rings (stewardship lineage)

### 11.1 Human growth rings

Human stewardship lineages MAY be represented through **growth rings** that record descriptive evidence of stewardship activity over time.

Human growth rings MAY record stewardship epochs, audits performed, disputes clarified, explanations provided, or other sustained stewardship contributions. Growth rings are descriptive artifacts that make stewardship behavior legible and auditable.

---

### 11.2 Model growth rings

Model lineage and version-state requirements for AI systems are defined in the *AI Boundaries Specification* and are referenced here solely for symmetry.

Human and AI growth rings share the same conceptual purpose: to provide transparent, time-ordered evidence of stewardship behavior without conferring authority or privilege.

---

### 11.3 Growth rings as evidence, not authority

Growth rings provide legible evidence of stewardship behavior. They are not authority tokens.

Growth rings MUST NOT grant eligibility, grant power, act as voting weight, function as governance credentials, or modify canonical authority in any way. They exist solely to support human judgment about stewardship quality and reliability.



### 12.1 Non-coercive transition model

### 12.1 Non-coercive transition model

The protocol defines a non-coercive model for the transition from centralized authority toward stewardship.

Transitions in responsibility and influence occur through transparency, demonstrated performance, accumulated legitimacy, and earned public trust. They do not occur through force, seizure, mandate, or retroactive invalidation of authority.

The protocol does not compel Lions to relinquish control. Instead, it provides mechanisms through which stewardship-oriented practices can outcompete opaque or unilateral control by offering greater legitimacy and coordination.

---

As centralized systems grow in importance and affect more lives, Lions MAY voluntarily adopt stewardship practices consistent with Entling behavior.

As centralized systems grow in importance and affect more lives, Lions MAY voluntarily adopt stewardship practices consistent with Entling behavior.

During this phase, Lions may begin acting as Entlings for the systems they control by making responsibility explicit, publishing explanations, inviting challenge, and participating in deliberation while still retaining operational authority.

This phase does not require a formal handoff of control. It represents a shift in posture from unilateral decision-making toward accountable stewardship without denying the reality of existing power.

---

### 12.3 Transition markers and accountability surfaces

A transition from Lion toward Ent stewardship SHOULD be legible within the protocol.

Indicative markers MAY include increasing publication of operational decisions as challengeable claims, increasing public deliberation on consequential choices, increasing reliance on protocol-driven reasoning and challenge processes, and decreasing unilateral control surfaces where feasible.

These markers are descriptive signals used to understand evolving stewardship behavior. They do not constitute requirements, mandates, or automatic triggers and MUST NOT be enforced coercively.

---

## 13. When Lions remain Lions

### 13.1 Legitimate centralization

Some systems legitimately benefit from sustained centralized authority.

Such systems MAY include time-critical operations, creative or artistic works, and tightly scoped tools where unified direction, speed, or coherence is essential. The protocol does not assume decentralization is universally superior or appropriate.

Lionhood in these contexts remains a descriptive status and does not imply moral failure or protocol non-conformance.

---

### 13.2 Coexistence principle

The forest is not expected to eliminate Lions universally.

Instead, it is expected to make Lion systems legible, make their impacts debatable, and offer an alternative stewardship path when impact grows and broader accountability becomes beneficial.





## 14. Revocation, demotion, and accountability

### 14.1 Revocability

All roles defined in this specification, except the Ant role, are temporary, challengeable, and removable.

No role confers permanence, tenure, or guaranteed continuation. Role status exists only so long as it remains accurate, accepted, and consistent with observed behavior and declared responsibilities.

---

### 14.2 No role immunity

Role status never shields an identity from challenge, scrutiny, or accountability.

Holding a role does not provide protection from criticism, dispute, investigation, or revocation. Roles do not alter how evidence is weighed, how challenges proceed, or how outcomes are determined under Protocol v5.

---

### 14.3 Role claims, disputes, and revocation mechanics

Role status MUST be represented as explicit, challengeable claims using existing Protocol v5 primitives.

Role revocation or demotion does not delete historical evidence of prior stewardship or behavior. It updates only the current recognized status of the role while preserving the historical record for audit and context.

Role revocation or demotion does not delete historical evidence of prior stewardship or behavior. It updates only the current recognized status of the role while preserving the historical record for audit and context.

---

### 14.4 No hidden revocation

Revocation and demotion of roles MUST be public and legible, subject only to safety, privacy, and jurisdictional visibility constraints defined elsewhere in the specification set.

No role status may be altered through secret, silent, or retroactive mechanisms. Participants MUST be able to see when a role has changed and why, within the limits of applicable safety rules.

---

## 15. Non-goals and exclusions

This specification explicitly does not support forced decentralization, role-based voting power, permanent stewardship classes, hidden authority structures, or role-based immunity.

Any implementation or interpretation that introduces these properties is non-conformant with this specification and with the principles of Protocol v5.



## 16. Relationship to other specifications

This specification depends upon and is subordinate to the following documents, each of which defines authoritative rules within its respective domain:

Protocol v5, which defines canonical semantics, event structure, challenge mechanisms, and authority invariants;  
AI Boundaries Specification, which defines artificial intelligence participation, sandbox boundaries, and model lineage;  
Governance Specification, which defines rulebooks, challenges, elections, and governance processes;  
Token Specification, which defines POD and POINT issuance, accounting, and constraints;  
Safety Specification, which defines visibility, classification, and content handling rules;  
Offline & Mindseed Specification, which defines offline operation, resilience, and publication mechanics.

In the event of any conflict, ambiguity, or inconsistency between this specification and Protocol v5, **Protocol v5 SHALL be authoritative**. Conflicts with other dependent specifications SHALL be resolved by deferring first to Protocol v5 and then to the specification governing the relevant domain.

