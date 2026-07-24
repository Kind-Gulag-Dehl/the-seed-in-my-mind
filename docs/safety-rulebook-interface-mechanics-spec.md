> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

# Safety Rulebook Interface & Visibility Mechanics

---

---

## 0. Purpose, scope, and authority [anchor: 0_purpose_scope_and_authority]

### 0.1 Purpose [anchor: purpose]

This specification defines **how safety and visibility decisions are expressed, applied, inspected, and challenged** within the Protocol v5 ecosystem, without deleting, rewriting, fragmenting, or otherwise mutating canonical history.

Its purpose is to ensure that:

- safety decisions are legible, challengeable, and reversible,
- visibility controls do not create divergent canonical realities,
- individuals can understand *why* content is gated, abstracted, or withheld,
- jurisdictional constraints can be respected without erasing truth or existence.

This document is explicitly concerned with the **interface layer** between governance-defined safety logic and user-visible representations. It does not redefine truth, importance, or action semantics.

### 0.2 Scope [anchor: scope]

This specification governs:

- safety rulebooks as protocol-governed instruments,
- safety classification events and their canonical representation,
- abstraction versus redaction mechanics,
- jurisdictional and local lenses,
- conformance requirements for nodes and clients.
- interaction between safety handling and challenge mechanics,
- conformance requirements for nodes and clients.

Anything not explicitly listed above is out of scope unless referenced as a dependency.

### 0.3 Authority [anchor: authority]

- Protocol v5 Section 0 constitutional invariants are supreme.
- Safety mechanisms MUST NOT alter canonical facts, event ordering, or replay-derived semantics.
- Safety applies only to **visibility and presentation**, never to canonical existence.

No safety mechanism defined here may override, reinterpret, or weaken Protocol v5 invariants.

### 0.4 Normative precedence and conflict handling [anchor: normative_precedence_and_conflict_handling]

   - canonical semantics are determined by Protocol v5 together with Appendix A schema and event semantics,
2. If this specification conflicts with other subordinate specifications (including but not limited to Governance, Challenge Engine, Deterministic Replay & Merge, Offline & Mindseed, or Token Specification), then:
   - canonical semantics are determined by Protocol v5 together with Appendix A schema and event semantics,
   - this document governs only the safety and visibility interface layer,
   - the conflict MUST be resolved through governance (e.g., rulebook amendment or supersession), not through node-local or client-local interpretation.

### 0.5 Definitions and terms (normative) [anchor: definitions_and_terms_normative]

For the purposes of this specification:

- **Canonical state**  
  The state derived by deterministic replay of the canonical event log under the active rulebook set at each snapshot boundary.

- **Visibility and presentation**  
  Any rendering, disclosure, filtering, warning, abstraction, gating, redaction placeholder, or UI/transport decision that does not mutate canonical objects or their replay-derived semantics.

- **Lens**  
  A local or jurisdictional presentation constraint applied by a node or client to a user-visible view, disclosed as such and not treated as canonical truth.

- **Safety floor**  
  A rulebook-defined minimum safety-handling requirement that nodes MUST enforce (e.g., non-distributable payload thresholds, mandatory gating of certain specific content).

- **Non-distributable payload**  
  A payload specificity level or content category that active rulebooks prohibit from being served or transmitted in a given distribution context, while still preserving canonical existence and safe-level descriptions.

### 0.6 Non-goals (normative exclusions) [anchor: non_goals_normative_exclusions]

This specification does not define:

- how truth, importance, action, or representation verdicts are computed,
- how votes are tallied or who is eligible to vote,
- how tokens are minted, distributed, or accrued,
- how identities are verified or authenticated,
- how indexing, ranking, or search algorithms operate (except insofar as visibility gating must be explainable).

These concerns are addressed by other specifications and remain authoritative there.

### 0.7 Threat model (non-exhaustive, normative expectations) [anchor: threat_model_non_exhaustive_normative_expectations]

Nodes and clients MUST treat the following as expected adversarial pressures:

- coercive jurisdictions attempting to force silent deletion, denial of existence, or historical revision,
- coordinated campaigns attempting to weaponize safety labels to suppress ideas rather than protect users,
- attempts to launder disallowed payload specificity into canonical descriptions or abstractions,
- classifier bias, drift, or opaque decision-making (human or AI),
The interface defined in this specification MUST remain legible, deterministic, and auditable under these pressures.

The interface defined in this specification MUST remain legible, deterministic, and auditable under these pressures.

---

## 1. Safety as visibility, not erasure [anchor: 1_safety_as_visibility_not_erasure]

### 1.1 Core invariant [anchor: core_invariant]

No safety rule may delete, rewrite, nullify, or retroactively alter a canonical event, idea, connection, challenge, or verdict.

All safety actions operate exclusively by **controlling visibility and representation**, not canonical state.

Canonical history is immutable; safety governs how it is seen, not whether it exists.

### 1.2 Consequences [anchor: consequences]

- Canonical replay MUST ignore safety rules.
- Two nodes replaying the same canonical event log MUST derive identical canonical state regardless of safety configuration, lenses, or UI policies.
- Safety differences MAY change what users see, but MUST NOT change what exists.

For avoidance of doubt, “Canonical replay MUST ignore safety rules” refers specifically to post-admission visibility and presentation handling. The precise requirements are:

- Canonical replay MUST ignore node-local and client-local visibility preferences, UI settings, personalized filters, and jurisdictional lens settings when deriving canonical state.

- Canonical replay MUST ignore node-local and client-local visibility preferences, UI settings, personalized filters, and jurisdictional lens settings when deriving canonical state.
- Canonical replay MUST apply all rulebook-defined validity rules, including safety floors that govern event admission (for example, prohibitions on accepting events containing certain prohibited payload specificity levels, or requirements for mandatory abstraction/withholding during submission).
- Once an event has been validly admitted to the canonical event log, canonical replay MUST NOT drop, omit, nullify, or otherwise alter that event or its semantic effects due to safety considerations. Safety mechanisms operate exclusively on payload visibility, abstraction, placeholder generation, and explanation surfaces after admission.
- Safety handling MUST NOT introduce alternate canonical universes. All conformant nodes MUST derive identical canonical state from the same event log under the same active rulebook set at cycle boundaries.

This distinction ensures that safety floors protect users and comply with law at the point of event acceptance, while preserving the integrity and universality of the canonical historical record thereafter.

### 1.4 Canonical existence vs safe representation (normative) [anchor: canonical_existence_vs_safe_representation_normative]

- Canonical existence MUST remain discoverable at a safe level of representation, even when full details are gated, abstracted, or withheld.
The system MUST never pretend that something does not exist merely because it cannot be fully shown.

The system MUST never pretend that something does not exist merely because it cannot be fully shown.

### 1.5 Determinism and portability requirements (normative) [anchor: determinism_and_portability_requirements_normative]

- Applying the same canonical event log and the same active rulebook set MUST produce the same safety classification state and the same required minimum visibility actions (safety floors).

- Safety handling MUST remain portable across nodes, clients, and offline contexts, relying only on snapshot-visible data and declared lenses.

### 1.6 No silent suppression (normative) [anchor: no_silent_suppression_normative]


- If something is withheld due to a safety floor or lens, users MUST be able to see that withholding occurred and understand why, without requiring privileged access or moderator tools.



## 2. Safety rulebooks [anchor: 2_safety_rulebooks]

### 2.1 Definition [anchor: definition]

A safety rulebook is a **governance-controlled specification** that defines how safety-related judgments are translated into visibility and distribution constraints, without altering canonical state.

A safety rulebook defines, at minimum:

- **classification criteria**  
  The conditions under which canonical objects or artifacts receive safety labels.

- **permitted visibility actions**  
  The set of allowed presentation outcomes (e.g., full, abstracted, gated, jurisdictionally hidden, warning overlays) that may result from a classification.

- **abstraction requirements**  
  Rules governing how unsafe or non-distributable payloads are transformed into safe-level representations while preserving meaning and structure.

- **jurisdictional applicability**  
  How safety handling varies across legal or regulatory contexts, including which lenses apply and under what conditions.

A safety rulebook does not itself classify content; it defines the rules under which classification events are interpreted and enforced.

### 2.2 Rulebook scope [anchor: rulebook_scope]

Safety rulebooks MAY govern:

- content categories (e.g., violence, sexual content, extremism),
- participant protection requirements (e.g., minors, vulnerable populations),
- legal or regulatory compliance by jurisdiction,
- distribution constraints tied to context (public feeds, default views, export, offline bundles).

Safety rulebooks MUST NOT govern:

- truth value or truth-challenge outcomes,
- importance rankings or importance-challenge outcomes,
- voting eligibility, tallying, or verdict computation,
- authorship legitimacy or identity validity.

Safety rulebooks operate strictly at the **visibility and distribution interface**, not at the epistemic or governance core.

### 2.3 Activation and supersession (normative) [anchor: activation_and_supersession_normative]

Safety rulebooks are activated and superseded exclusively through governance processes, and their effective application MUST be deterministic.

- A safety rulebook (or a specific version of a rulebook) becomes operative only through a successful governance adoption process for the safety domain.
- Adoption and supersession take effect at **cycle boundaries**. Nodes MUST deterministically derive the active safety rulebook set for each cycle interval from the canonical event log, including relevant adoption actions and resolved challenges.
- Multiple safety rulebooks MAY coexist if governance has adopted them under a defined composition scheme.
- Conflict resolution between coexisting safety rulebooks MUST be deterministic and explicit (for example, via a defined precedence order or merge semantics), such that independent implementations converge on the same safety floors and outcomes for the same cycle interval.

Node-local policy or client preferences MUST NOT alter which safety rulebooks are active.

### 2.4 Rulebook identity, versioning, and immutability (normative) [anchor: rulebook_identity_versioning_and_immutability_normative]

- A safety rulebook MUST have a stable identifier and an immutable content hash.

- Nodes MUST be able to prove which safety rulebook version(s) were active at a given snapshot boundary using only canonical data.

Historical rulebook versions remain inspectable and challengeable even after supersession.

### 2.5 Rulebook composition and precedence (normative) [anchor: rulebook_composition_and_precedence_normative]

When multiple safety rulebooks coexist, the active set MUST define:

- a deterministic precedence order or deterministic merge semantics,
- an explicit list of permitted visibility actions and their constraints.
- deterministic tie-breaking rules,
- an explicit list of permitted visibility actions and their constraints.

Composition rules MUST be explicit enough that two independent nodes, given the same snapshot and classification events, derive the same safety floor outcomes.

### 2.6 Safety floors vs optional guidance (normative) [anchor: safety_floors_vs_optional_guidance_normative]

A safety rulebook MUST clearly distinguish between:

- **safety floors**  
  Mandatory minimum handling requirements that nodes MUST enforce. These include any rule that affects event admissibility, payload distribution, or minimum gating/abstraction.

- **guidance**  
  Recommended client or UI behaviors that MAY vary across implementations without breaking conformance.

Any requirement that affects whether content may be admitted, transmitted, or minimally gated MUST be declared as a safety floor.

### 2.7 Non-distributable payload thresholds (normative) [anchor: non_distributable_payload_thresholds_normative]

Safety rulebooks MAY define thresholds at which certain payload specificity becomes **non-distributable** in a given context (e.g., to minors, to a jurisdiction, or in a default public view).

When such thresholds apply:

- canonical existence MUST remain intact,
- safe-level descriptions MUST remain available,
- the visibility outcome MUST record that higher-specificity payload exists and is withheld,
- the system MUST NOT present the content as if it does not exist.

Non-distributable thresholds constrain distribution and presentation, not canonical history.

### 2.8 Rulebook transparency and auditability (normative) [anchor: rulebook_transparency_and_auditability_normative]

- Safety rulebooks MUST be human-readable and machine-parseable.
- If classifiers (human or AI) are referenced, the rulebook MUST specify decision criteria and label semantics sufficient to audit classification outcomes.
- Governance MUST NOT adopt safety mechanisms whose outcomes cannot be independently inspected, reasoned about, and challenged by conformant nodes.

Opaque or non-auditable safety logic is non-conformant.

---

## 3. Safety classification events [anchor: 3_safety_classification_events]

### 3.1 Classification as canonical metadata [anchor: classification_as_canonical_metadata]

Safety classification is expressed through **explicit canonical events** that attach metadata to existing canonical objects or artifacts.

Safety classification events:

- do not modify the underlying canonical object,
- reference the classified target by stable identifier or hash,
- specify the applicable safety rulebook(s) and label(s).

Classification events are part of canonical history and participate in deterministic replay.

### 3.2 Classification object structure (conceptual) [anchor: classification_object_structure_conceptual]

A safety classification event MUST include, at minimum:

- a target object reference,
- one or more classification labels,
- a rulebook reference (identifier and version),
- a jurisdiction scope (if applicable),
- a timestamp and author identity.

The exact schema is defined in Protocol v5 Appendix A, but all fields above MUST be representable.

### 3.3 Challengeability [anchor: challengeability]

All safety classification events MUST be challengeable.

Safety disputes MUST be expressed using the existing Protocol v5 challenge primitive and its domains, without introducing a new safety-specific challenge type.

Challenges MAY take the form of:

- **Representation challenges**  
  Used to contest the *representation-layer* outcome of safety handling, such as whether an abstraction, redaction placeholder, warning interstitial, or other permitted visibility outcome:
  - preserves maximal safe meaning,
  - avoids semantic mutation,
  - avoids silent suppression,
  Representation challenges MUST target representation artifacts (e.g., competing safe abstractions, competing redaction placeholders, competing warning/summary representations) consistent with the Challenge Engine Specification’s rules for representation challenges.

- **Governance challenges (action challenges with governance constraints)**  

- **Governance challenges (action challenges with governance constraints)**  
  Used to contest the *rule-level* causes of a safety outcome, including:
  - the text of a safety rulebook,
  - classification criteria, thresholds, or label semantics,
  - rulebook composition / precedence rules,
  - and any governance-controlled configuration that determines safety floors.

No safety classification is final or immune from challenge. Reclassification and supersession MUST occur only through new canonical events; there is no retroactive erasure.

No safety classification is final or immune from challenge. Reclassification and supersession MUST occur only through new canonical events; there is no retroactive erasure.

### 3.4 Canonical ordering and clock-independence (normative clarification) [anchor: canonical_ordering_and_clock_independence_normative_clarification]

- Timestamps MUST NOT be treated as authoritative wall-clock time for canonical semantics.
- Canonical ordering MUST be derived from event log ordering rules and cycle boundaries, not from external clocks.
- Implementations MAY store timestamps for user display or audit convenience, but MUST NOT use them to resolve conflicts, precedence, or determinism.

### 3.5 Targets and attachment rules (normative) [anchor: targets_and_attachment_rules_normative]

Safety classification events MAY target, at minimum:

- ideas,
- connections,
- descriptions or representation artifacts,
- arguments or evidence attachments,
- challenges (as containers),
- specific challenge-phase artifacts.

A classification event MUST identify its target unambiguously and MUST NOT rely on mutable fields such as titles or free-text labels.

### 3.6 Label semantics and namespaces (normative) [anchor: label_semantics_and_namespaces_normative]

- Classification labels MUST be defined by the referenced safety rulebook, including their meaning, severity (if applicable), and permitted visibility actions.
- Labels MUST be namespaced or otherwise unambiguous across multiple active rulebooks.
- Classification labels MUST NOT be treated as truth judgments, importance judgments, or governance outcomes.

Labels describe safety handling, not epistemic validity.

### 3.7 Authorship and accountability (normative) [anchor: authorship_and_accountability_normative]

- Canonical safety classification events MUST be authored by a **verified human identity** as required by Protocol v5 Appendix A. AI systems MAY assist by proposing classifications, but an AI MUST NOT directly author canonical safety classification events.

- Canonical safety classification events MUST be authored by a **verified human identity** as required by Protocol v5 Appendix A. AI systems MAY assist by proposing classifications, but an AI MUST NOT directly author canonical safety classification events.

- If AI-assisted classification is used, the system MUST represent the distinction between:
  - **non-canonical proposals** (draft/advisory outputs), and
  - **canonical adoption** (a human-authored classification event that records the chosen label(s), referenced rulebook version(s), and any required visibility floor outcomes).

- Classification events MUST remain challengeable regardless of who proposed them, and regardless of whether the classification was made manually or with AI assistance.

- Classification accountability MUST remain legible at a safe level. At minimum, clients and nodes MUST be able to disclose:
  - the author identity reference of the canonical classification event,
  - the referenced rulebook identifier + immutable hash/version,
  - the applied label namespace and label(s),
  - and the derived floor visibility action (as deterministically computed under the active rulebook set).




### 3.8 Minimal safe description requirement (normative) [anchor: minimal_safe_description_requirement_normative]

If a safety classification results in gating, abstraction, or withholding of content:

- a safe-level description MUST remain available, and
Safety handling MUST never eliminate the user’s ability to discover that something exists and understand, at a safe level, why its visibility changed.





## 4. Visibility actions [anchor: 4_visibility_actions]

### 4.1 Permitted visibility actions (normative) [anchor: permitted_visibility_actions_normative]

Safety rulebooks MAY specify only the following visibility actions for user-facing representations. These actions define **how** content is shown, not whether it exists canonically.

1. **Full visibility**  
   The content is rendered without safety-driven transformation, other than standard formatting or layout differences. No abstraction, gating, or withholding is applied.

2. **Abstracted visibility**  
   The content is rendered as a safe-level abstraction that preserves meaning at the maximum permissible level while removing disallowed specificity. The abstraction MUST preserve topic identity, role, and structural context.

3. **Gated visibility (explicit opt-in)**  
   - Gating MUST always allow refusal without penalty.  
   - The interface MUST display a safe-level summary and a warning before opt-in.  
   - Gating MUST always allow refusal without penalty.  
   - Gating MUST NOT be used to create silent invisibility or denial of existence.

4. **Jurisdictionally hidden (with disclosure)**  
   The content is withheld due to a jurisdictional lens or node distribution policy tied to a jurisdictional rule.  
   - The interface MUST disclose that the content exists.  
   - The interface MUST state that it is withheld under an active jurisdictional lens.  
   - The governing rulebook and jurisdiction label MUST be identifiable at a safe level.

5. **Warning overlay / interstitial**  
   A warning or interstitial layer is rendered prior to showing either full or abstracted content.  
   - The warning MUST convey relevant safety context.  
   - The warning MUST present clear next actions (continue, view abstracted, view full if permitted, learn why).

No other visibility actions are permitted as safety rulebook outputs.

Rulebook-defined **distribution or admissibility floors** (e.g., non-distributable payload thresholds) are not additional visibility action types. They are constraints that may require withholding payload detail while still requiring placeholders, abstraction, and explanation surfaces.

### 4.2 Separation: visibility actions vs event admissibility (normative) [anchor: separation_visibility_actions_vs_event_admissibility_normative]

Visibility actions govern how content is rendered or disclosed to a user. Separately, safety rulebooks MAY define **distribution or admissibility floors**.

- Admissibility floors MAY cause a node to refuse to serve or transmit specific payload details in a given context.
- Admissibility floors MUST NOT delete, invalidate, or erase canonical objects or events.
- Admissibility floors MUST still support safe-level discovery, placeholders, and explanation paths.
- Admissibility floors MUST be applied deterministically under the active rulebook set at the relevant snapshot boundary.

Visibility actions and admissibility constraints serve distinct roles and MUST NOT be conflated.

### 4.3 Action selection and precedence (normative) [anchor: action_selection_and_precedence_normative]

Given a target object and the active safety rulebook set:

- The minimum required visibility action (the **floor action**) MUST be derived deterministically from:
  - safety classification events targeting the object or its attached artifacts,
  - the active safety rulebook set at the relevant snapshot boundary,
  - any explicitly applied jurisdictional lens.
- If multiple applicable rules yield different actions, conflict resolution MUST follow the deterministic composition and precedence rules defined by the active rulebook set.
- If a node or client applies additional strictness beyond the floor action, that additional strictness MUST be disclosed as a local or client lens choice and MUST NOT be represented as a canonical safety outcome.

### 4.4 Action applicability and target granularity (normative) [anchor: action_applicability_and_target_granularity_normative]

Visibility actions MUST be applicable at multiple granularities, including at minimum:

- idea-level (title and description tiers),
- representation artifact level (sentence, paragraph, full; fundamental, standard, advanced, canonical),
- argument or evidence attachment level,
- challenge container level (without affecting voting, tallying, or verdict mechanics),
- connection display level (the connection may be visible while the payload of a label or description is abstracted).

### 4.5 Prohibited actions (normative) [anchor: prohibited_actions_normative]

### 4.5 Prohibited actions (normative) [anchor: prohibited_actions_normative]

Safety rulebooks MUST NOT permit:

- **silent suppression**, where content disappears without disclosure,
- **unmarked removal**, where no placeholder or explanation path is provided,
- **irreversible hiding**, where no challenge path or later reclassification is possible,
- **user-specific personalization of canon**, where canonical state differs per user rather than per lens,
- **semantic mutation**, where safety labels or abstractions alter truth, importance, or action meaning.

### 4.6 Required disclosure behaviors (normative) [anchor: required_disclosure_behaviors_normative]

Whenever content is not fully visible due to safety handling, nodes and clients MUST:

- render a placeholder indicating that withholding occurred,

- show at least a safe-level description sufficient to understand the general nature of what is withheld.

If even a safe-level description is prohibited, the placeholder MUST explicitly state that visible detail is legally or safely constrained and identify the governing rulebook at a safe level.

---

## 5. Abstraction vs redaction [anchor: 5_abstraction_vs_redaction]

### 5.1 Abstraction (preferred) (normative) [anchor: abstraction_preferred_normative]

Abstraction replaces detailed content with a higher-level representation that preserves, to the maximum permissible degree:

- **topic identity** (what the object is broadly about),
- **existence of disagreement or contestation** (when applicable),
- **argumentative role** (claim, counterclaim, evidence, plan step, action report, etc.),
- **existence of disagreement or contestation** (when applicable),
- **graph continuity** (links, rankings, challenge containers, and navigability).

Abstraction MUST NOT introduce new factual claims. It may describe the category, shape, or role of the content without providing disallowed specificity.

### 5.2 Abstraction tiers and description layers (normative) [anchor: abstraction_tiers_and_description_layers_normative]

When an object supports multi-tier descriptions (sentence, paragraph, full; fundamental, standard, advanced, canonical), safety handling MUST define:

- which tiers are permitted under each visibility action,
- whether abstraction is generated deterministically from an allowed tier (preferred) or provided as a separate representation artifact,
Fallback behavior MUST be deterministic and explainable.

Fallback behavior MUST be deterministic and explainable.

### 5.3 Redaction (last resort) (normative) [anchor: redaction_last_resort_normative]

Redaction MAY be used only when abstraction itself would violate a safety floor or binding legal constraint.

When redaction is applied:

- a placeholder MUST remain indicating that content was withheld,
- the placeholder MUST include the governing rulebook reference and reason category at a safe level,
- the redacted artifact MUST remain challengeable.

Challenges to redaction concern the classification or visibility decision and the adequacy of the safe-level representation, not the disclosure of prohibited details.

Redaction MUST be treated as an exceptional measure, not the default.

### 5.4 Canonical persistence and referential integrity (normative) [anchor: canonical_persistence_and_referential_integrity_normative]

Both abstraction and redaction MUST preserve canonical references such that:

- challenges continue to point to the same targets,
- connection topology remains intact,
- importance relations and rank computations remain stable,
- downstream reasoning can refer to the existence of withheld content without requiring access to the withheld payload.

Canonical structure MUST survive safety handling unchanged.

### 5.5 Deterministic transformation requirements (normative) [anchor: deterministic_transformation_requirements_normative]

- If abstraction or redaction is performed automatically by a node, the transformation rules MUST be deterministic under the active rulebook set.
- If abstraction or redaction is authored as a separate representation artifact, that artifact MUST be attributable to an identity and MUST be challengeable like other representations.

### 5.6 Avoiding meaning-loss attacks (normative) [anchor: avoiding_meaning_loss_attacks_normative]

At minimum, the system MUST preserve:

At minimum, the system MUST preserve:

- that something exists,
- the general category of what it is,
- how it relates to nearby ideas, arguments, or challenges,
- and why its visibility changed.



## 6. Jurisdictional lenses [anchor: 6_jurisdictional_lenses]

### 6.1 Definition (normative) [anchor: definition_normative]

A **jurisdictional lens** is a deterministic mapping from canonical content to visible content under a specified legal or regulatory context. A jurisdictional lens constrains **distribution and presentation**, not canonical existence or semantics.

A jurisdictional lens may be applied by:


A jurisdictional lens MUST be explicit, named, and inspectable. It MUST be possible for a user to identify when a lens is active and what legal or regulatory context it represents.

A jurisdictional lens MUST be explicit, named, and inspectable. It MUST be possible for a user to identify when a lens is active and what legal or regulatory context it represents.

### 6.2 Properties (normative) [anchor: properties_normative]

Jurisdictional lenses:

- MUST NOT alter canonical objects, event validity semantics, or deterministic replay outcomes,
- MAY restrict distribution of payload specificity via non-distributable thresholds,
- MUST be disclosed whenever applied to a user-visible view,
- MUST be deterministic given:
  - the active safety rulebook set at the relevant snapshot boundary, and
  - the applicable safety classification events for the target objects.

A node MAY apply a default jurisdictional lens as a matter of legal compliance. If it does so, the lens identity and scope MUST be visible to users and MUST NOT be silently applied.

### 6.3 Lens identity and auditable configuration (normative) [anchor: lens_identity_and_auditable_configuration_normative]

- Each jurisdictional lens MUST have a stable identifier.
- Each lens MUST declare the safety rulebook(s) and clauses it depends on.
- Nodes MUST be able to report which jurisdictional lens (if any) is active for a given session, request, or rendered view.
- Clients MUST be able to display the active lens identity and allow inspection of its governing rules at a safe level.

Lens configuration MUST be auditable using snapshot-visible data and declared node/client settings, without reliance on proprietary or opaque logic.

### 6.4 Cross-jurisdiction awareness (normative) [anchor: cross_jurisdiction_awareness_normative]

Even when content is hidden or gated under a jurisdictional lens, interfaces MUST indicate:

- that the content exists,
- that it is withheld due to a jurisdictional constraint,
- which safety rulebook and jurisdiction label govern the restriction,
- whether the restriction is due to:
  - a mandatory safety floor, or
  - an additional local or node-specific strictness policy.

### 6.5 Lens stacking and “stricter-than-floor” behavior (normative) [anchor: lens_stacking_and_stricter_than_floor_behavior_normative]

If multiple lenses or local policies apply simultaneously:

If multiple lenses or local policies apply simultaneously:

- the resulting visibility MUST be at least as restrictive as the safety floor actions required by the active rulebooks,
- any additional strictness beyond the floor MUST be disclosed as such,
- the explanation path MUST clearly separate:
  - rulebook-required handling, and
  - locally chosen or user-selected restrictions.

Lens stacking MUST be deterministic and MUST NOT produce ambiguous or contradictory explanations.

### 6.6 Portability and offline views (normative) [anchor: portability_and_offline_views_normative]

Jurisdictional lenses MUST be usable in offline contexts.

- A lens MUST NOT depend on live external services to determine applicability.
- If a lens requires a jurisdiction label, that label MUST be representable as part of:
  - local configuration, and/or
  - packaged metadata included in offline bundles.
- Offline views MUST still support disclosure, placeholders, and explanation paths consistent with online behavior.

---

### 7.1 Mandatory explainability (normative) [anchor: mandatory_explainability_normative]

### 7.1 Mandatory explainability (normative) [anchor: mandatory_explainability_normative]

- the applied safety rulebook set (or the specific rulebooks relevant to the decision),

- the applied safety rulebook set (or the specific rulebooks relevant to the decision),
- the relevant safety classification event(s) and label(s),
- the visibility action taken,
- whether the action is a rulebook-required floor action or a stricter local or client lens action,
- the target object identity in a safe, non-payload-leaking form,
- the available challenge path (including challenge domain and initiation point),
- the applied jurisdictional lens identity (if any).

Explainability is a core safety requirement, not an optional UI feature.

### 7.2 Explanation determinism (normative) [anchor: explanation_determinism_normative]

Explanations MUST be derivable deterministically from:

- canonical safety classification events,
- the active safety rulebook set at the relevant snapshot boundary,
- snapshot-visible governance state needed to interpret those rulebooks (e.g., adoption lineage),
- the explicitly declared lens configuration (node and/or client).

Given the same inputs, independent implementations MUST converge on the same explanation content at the floor level.

### 7.3 Minimal explanation availability (normative) [anchor: minimal_explanation_availability_normative]

Even when the underlying payload is non-distributable:

- the explanation view MUST remain renderable,
- the explanation MUST state that something was withheld,
- the explanation MUST include the reason category at a safe level,
- the governing rulebook reference MUST be shown,
- the explanation MUST indicate how to challenge the classification or representation decision.

Lack of payload access MUST NOT block access to the explanation itself.

### 7.4 No privileged-only explanations (normative) [anchor: no_privileged_only_explanations_normative]

- Ordinary users MUST be able to view the baseline explanation path at a safe level.

- Ordinary users MUST be able to view the baseline explanation path at a safe level.
- Privileged tools for moderators or stewards MAY exist, but MUST NOT replace or hide the baseline explanation.
- Safety reasoning MUST NOT be locked behind authority, role, or payment.

### 7.5 Explanation stability across implementations (normative) [anchor: explanation_stability_across_implementations_normative]

Independent nodes and clients MUST converge on the same floor explanation outcome for the same:

- snapshot boundary,
- classification events,
- active rulebook set,
- declared lens configuration.

UI layout, wording, or presentation MAY differ, but the explanation content and reasoning chain MUST be reconstructible, consistent, and auditable across implementations.



## 8. Interaction with challenges [anchor: 8_interaction_with_challenges]

### 8.1 Safety during active challenges (normative) [anchor: safety_during_active_challenges_normative]

Safety rules MAY affect the visibility and representation of challenge-related artifacts, including:

- arguments,
- evidence attachments and references,
- descriptions and representation tiers of challenge targets,
- embedded media or external links,
- user-generated summaries or explanations of a dispute.

Safety rules MUST NOT affect any canonical challenge mechanics, including:

- vote eligibility,
- voter selection or assignment (where applicable),
- tally computation,
- quorum or threshold rules,
- verdict finalization,
- the deterministic state transformation that follows a finalized verdict.

If safety handling results in gating, abstraction, or redaction of challenge artifacts:

- the challenge container MUST remain discoverable and navigable,
- users MUST still be able to see, at a safe level:
  - that a challenge exists,
  - the challenge domain (truth, importance, action, or representation),
  - which idea(s) or artifact(s) are involved,
  - the current phase (e.g., open arguments, voting window, finalized),
  - the existence of arguments for and against (counts MAY be shown even if contents are gated).

Safety handling MUST NOT be used to structurally disadvantage one side of a challenge by erasing its visible existence.

### 8.2 Safety floors for challenge participation surfaces (normative) [anchor: safety_floors_for_challenge_participation_surfaces_normative]

Safety rulebooks MAY define floors that constrain what can be submitted, served, or distributed during a challenge (for example, prohibiting certain payload specificity for minors or in a given jurisdiction).

If such floors apply:

- submissions MAY be rejected, required to be abstracted, or required to be transformed at submission time,
- any rejection or abstraction MUST be deterministic under the active rulebook set at the relevant snapshot boundary,

Safety floors MUST regulate payload handling, not the existence of disagreement or participation.

Safety floors MUST regulate payload handling, not the existence of disagreement or participation.

### 8.3 Safety as challenge subject (normative) [anchor: safety_as_challenge_subject_normative]

Safety decisions themselves MAY be challenged, but only in ways that do not mutate truth, importance, or action semantics.

Permitted challenge pathways include:

- **Representation challenges**  
  To contest whether the *resulting representation-layer artifacts* (abstractions, redaction placeholders, summaries, warnings, or other representation outputs permitted by rulebooks) are correct and maximally meaning-preserving under the active safety floors.

- **Governance challenges (action challenges with governance constraints)**  

- **Governance challenges (action challenges with governance constraints)**  
  To contest the *governance-layer causes* of a safety outcome, including:
  - safety rulebook text,
  - classification criteria/thresholds,
  - label semantics,
  - and deterministic composition/precedence rules among coexisting rulebooks.

  Governance challenges MUST be expressed as action challenges under the Governance Specification, and any adoption/supersession effects MUST take effect only at cycle boundaries, consistent with deterministic replay.

The challenge subject MUST be explicitly identified, including one or more of:

- the specific safety classification event(s) being disputed,
- the specific rulebook clause(s) or version(s) claimed to be incorrect or unsafe,
- the specific floor visibility action outcome for a given target under a declared lens.

Safety challenges MUST be legible as safety disputes. They MUST NOT be used as disguised truth or importance challenges, and MUST NOT alter vote eligibility, tally computation, or verdict finalization for non-safety disputes.


### 8.4 Handling disputed safety during a challenge (normative) [anchor: handling_disputed_safety_during_a_challenge_normative]

When a safety classification or rulebook application is under active dispute:

- the system MUST continue to apply the current safety floor until a governance or representation verdict changes the applicable classification or rulebook state,
- any change in classification, rulebook text, or visibility outcome MUST be represented as new canonical events; there is no retroactive erasure,
Dispute handling MUST preserve continuity, auditability, and user comprehension throughout the challenge lifecycle.

Dispute handling MUST preserve continuity, auditability, and user comprehension throughout the challenge lifecycle.

---

## 9. Minority universes and safety divergence [anchor: 9_minority_universes_and_safety_divergence]

### 9.1 Allowed divergence boundaries (normative) [anchor: allowed_divergence_boundaries_normative]

Nodes and clients MAY differ in:

- local lens configuration (including stricter-than-floor default UI behavior),
- jurisdictional lens selection when explicitly declared,
- whether explicit user opt-in is offered to view content beyond the floor action (only when the floor permits opt-in),
- additional presentation filtering above the safety floor, provided it is disclosed as a lens.

Nodes and clients MUST NOT differ in:

- the active safety rulebook set derived via governance at cycle boundaries,
- event validity rules and safety floors that affect admissibility and minimum distribution constraints,
- canonical replay outcomes.

Divergence is permitted only at the presentation layer, not at the canonical or governance layer.

### 9.2 Canonical continuity preserved (normative) [anchor: canonical_continuity_preserved_normative]

All nodes remain part of the same canonical universe as long as they:

- replay the same canonical event log under the same deterministic replay rules,
- deterministically derive and apply the same active rulebook set at the same cycle boundaries,
- enforce the same rulebook-defined safety floors and validity constraints,
- disclose any stricter-than-floor handling as a lens or local policy rather than presenting it as canonical protocol behavior.

Safety divergence MUST NOT create multiple hidden canonical realities.

### 9.3 Disclosure requirement for divergence (normative) [anchor: disclosure_requirement_for_divergence_normative]

If a node or client applies handling that is stricter than the safety floor:

- it MUST disclose that additional strictness is being applied,
- it MUST identify the lens or policy responsible for that strictness,
- it MUST preserve canonical existence indicators and safe-level placeholders and explanations.

Users MUST be able to tell the difference between protocol-required safety and optional local strictness.

### 9.4 Forks and governance splits (normative clarification) [anchor: forks_and_governance_splits_normative_clarification]

If governance produces a fork resulting in two competing canonical universes, safety rulebooks MAY diverge because the universes themselves have diverged.

This is not safety divergence within a single universe; it is a governance-level fork.

In such cases:

- each universe MUST remain internally deterministic,
- nodes MUST disclose which universe or fork they are serving,




## 10. Individual rights and refusal [anchor: 10_individual_rights_and_refusal]

### 10.1 Right to disengage (normative) [anchor: right_to_disengage_normative]

No individual is required to:

- view restricted, gated, abstracted, or jurisdictionally withheld content,
- opt in to see full or higher-specificity details,
- participate in challenges,
- engage with safety-gated material,
Clients MAY offer additional user-selected lenses or opt-in paths where permitted by the safety floor, but MUST NOT force deeper viewing, expanded disclosure, or engagement as a condition of access to unrelated canonical functionality.

Clients MAY offer additional user-selected lenses or opt-in paths where permitted by the safety floor, but MUST NOT force deeper viewing, expanded disclosure, or engagement as a condition of access to unrelated canonical functionality.

Disengagement MUST NOT incur penalties, reduced standing, or adverse canonical effects.

### 10.2 Right to silence (normative) [anchor: right_to_silence_normative]

No identity is obligated to:

- publish thoughts or drafts,
- defend claims or classifications,
- respond to challenges,
- disclose private drafts or non-canonical working material,
- disclose real-world identity details beyond the verification level required by the protocol for canonical participation.

Safety rules, classifications, and challenges MUST NOT compel participation, justification, or disclosure. Silence MUST NOT be interpreted as concession, admission, or fault within canonical semantics.

### 10.3 Non-coercion of attention (normative) [anchor: non_coercion_of_attention_normative]

Safety mechanisms MUST NOT be used to force exposure or engagement.

In particular:

- interfaces MUST NOT require viewing gated or abstracted content in order to proceed with unrelated actions,
- users MUST NOT be required to respond, argue, or appeal in order to remove a safety label,
- warning overlays and interstitials MUST always allow refusal or exit without penalty.

Opt-in MUST be explicit, reversible, and free of coercive design patterns.

### 10.4 Respect for minors and protected populations (normative) [anchor: respect_for_minors_and_protected_populations_normative]

If minors or other protected populations are participants or viewers, safety rulebooks MAY impose stricter safety floors for:

- content visibility,
- opt-in mechanics,
- distribution thresholds and payload specificity.

Such floors:

- MUST be explicit and rulebook-defined,
- MUST be deterministic under the active rulebook set,
- MUST NOT alter canonical existence, ordering, or challengeability of content.

Protection of vulnerable populations MUST be achieved through visibility and distribution controls, not through erasure or semantic distortion.

---

## 11. Security considerations [anchor: 11_security_considerations]

### 11.1 Abuse prevention (normative) [anchor: abuse_prevention_normative]

Safety mechanisms MUST resist, at minimum, the following classes of abuse:

- **covert suppression**: hiding content without disclosure, placeholders, or explanation paths,
- **narrative laundering**: replacing disallowed payloads with misleading abstractions that alter meaning or argumentative role,
- **selective visibility manipulation**: applying inconsistent lenses without disclosure to create plausible-deniability censorship,
- **label spam or brigading**: mass classification intended to suppress ideas rather than protect participants,
- **classifier drift attacks**: changing classification behavior without governance-visible rulebook updates,
- **jurisdiction spoofing**: misrepresenting applied jurisdiction to gain access to restricted details or to suppress content.

Rulebooks, classification events, and explanation surfaces MUST be designed to surface and resist these attack patterns.

### 11.2 Auditability (normative) [anchor: auditability_normative]

All safety actions MUST leave an inspectable trail sufficient for independent reconstruction, including:

- the safety classification event(s) and their author identity,
- the rulebook version(s) referenced (hash-identifiable),
- the derived floor visibility action,
- the applied lens configuration (if any),
- any stricter-than-floor handling declared by the node or client,
Audit trails MUST be:

Audit trails MUST be:

- portable across nodes,
- usable in offline contexts,
- independent of proprietary or opaque services.

### 11.3 Deterministic reproducibility (normative) [anchor: deterministic_reproducibility_normative]

Given the same snapshot boundary state, active rulebook set, and safety classification events:

- independent nodes MUST be able to reproduce the same floor visibility action and explanation outcome,
- any stochastic or model-driven classifier outputs MUST be either:
  - advisory-only until adopted by an explicit human-authored canonical event, or
  - constrained and specified by rulebooks so that outcomes are deterministic and reproducible.

Non-deterministic safety outcomes MUST NOT affect canonical admissibility, ordering, or replay.

### 11.4 Privacy and doxxing resistance (normative) [anchor: privacy_and_doxxing_resistance_normative]

Safety mechanisms MUST avoid forcing disclosure of private identity details.

Explanation and audit views MUST be designed so that:

- they do not leak sensitive personal data,
- they do not require revealing private drafts or non-canonical material,
Safety infrastructure MUST strengthen resistance to doxxing, not weaken it.

Safety infrastructure MUST strengthen resistance to doxxing, not weaken it.



## 12. Conformance requirements [anchor: 12_conformance_requirements]

### 12.1 Node conformance (normative) [anchor: node_conformance_normative]

A conformant node MUST satisfy all of the following requirements.

#### Canonical replay determinism [anchor: canonical_replay_determinism]

A conformant node MUST:

- derive canonical state exclusively via deterministic replay of the canonical event log under the active rulebook set at each snapshot boundary,
- ignore node-local and client-local visibility preferences, UI settings, personalization, and lens choices when deriving canonical state,
- enforce all rulebook-defined validity constraints and safety floors deterministically during event validation and replay,
- MUST NOT drop, omit, rewrite, or nullify already-logged canonical events during replay; safety affects only distributable payload detail, required safe-level representations, and explanation surfaces.

Canonical replay outcomes MUST be identical across conformant nodes given the same event log and rulebook activations.

#### Active rulebook set correctness [anchor: active_rulebook_set_correctness]

A conformant node MUST:

- deterministically derive the active rulebook set (including safety rulebooks) from governance events and cycle boundaries,
- be able to prove which specific rulebook versions (hash-identifiable) were active for any given cycle boundary,
- apply rulebook composition, precedence, and conflict-resolution rules deterministically and explicitly.

Rulebook activation, supersession, and coexistence MUST be replay-safe and auditable.

#### Safety classification handling [anchor: safety_classification_handling]

A conformant node MUST:

- accept, store, index, and serve safety classification events as canonical metadata,
- apply classifications to their targets without mutating the underlying canonical objects,
- support namespaced classification labels and explicit rulebook references,
- preserve classification challengeability and attribution.

Classification handling MUST NOT introduce non-determinism or hidden state.

#### Floor visibility enforcement for distribution [anchor: floor_visibility_enforcement_for_distribution]

A conformant node MUST:

- enforce all rulebook-defined safety floors for distribution (including non-distributable payload thresholds) deterministically,
- when withholding payload details, preserve canonical existence indicators and safe-level discovery,
- provide placeholders, abstractions, and safe-level descriptions as required by the active rulebook set.

Withholding MUST be explicit and inspectable.

#### Explainability and audit outputs [anchor: explainability_and_audit_outputs]

- references to relevant safety classification events,

- references to relevant safety classification events,
- referenced rulebook identifiers and versions,
- derived floor visibility actions,
- applied jurisdictional lens identity (if any),
- disclosure of any stricter-than-floor node policy.

Nodes MUST NOT require privileged access to retrieve explanation-critical metadata.

#### No silent suppression [anchor: no_silent_suppression]

A conformant node MUST ensure that any node-rendered or node-supplied view can represent withheld content as withheld (via placeholders or markers), rather than omitting it without trace.

---

### 12.2 Client conformance (normative) [anchor: client_conformance_normative]

A conformant client MUST satisfy all of the following requirements.

#### Visibility marking [anchor: visibility_marking]

A conformant client MUST:

- clearly mark gated, abstracted, redacted, withheld-by-floor, or jurisdictionally withheld content,
#### Explainability interface [anchor: explainability_interface]

#### Explainability interface [anchor: explainability_interface]

A conformant client MUST:


- display, at a safe level, the applicable rulebook references, classification labels, visibility action taken, lens identity (if any), and the challengeability path.

#### No silent suppression [anchor: no_silent_suppression_2]

A conformant client MUST prevent silent suppression in presentation.

If the client applies filtering, hiding, or stricter handling beyond the safety floor, it MUST:

- label that handling as a client-local lens,
- disclose that it is stricter than the rulebook-required floor.

#### Opt-in and refusal UX [anchor: opt_in_and_refusal_ux]

A conformant client MUST:

- implement gated visibility as explicit opt-in,
- allow refusal without penalty or forced exposure,
- avoid coercive attention or engagement mechanics.

#### Consistency with floor actions [anchor: consistency_with_floor_actions]

A conformant client MUST:

- apply at least the floor visibility actions required by node-supplied metadata and the active rulebook set,
- if additional strictness is applied, disclose it as a client lens and preserve the explanation path.

---

### 12.3 Conformance testability (normative) [anchor: conformance_testability_normative]

A conformant implementation MUST support test cases enabling an independent auditor to verify that:

- canonical replay outcomes are identical across nodes given the same event log and rulebook activations,
- floor visibility actions and explanation outputs are reproducible given the same snapshot state, classification events, and active rulebooks,
- lens-induced strictness is disclosed and does not alter canonical semantics,
- withheld payloads still leave discoverable placeholders and safe-level descriptions where permitted.

Conformance MUST be testable without privileged access or proprietary tooling.

---

### 12.4 Failure modes and reporting (normative) [anchor: failure_modes_and_reporting_normative]

If a node or client cannot apply a required safety floor (e.g., missing rulebook, missing classification metadata, incompatible version):

- it MUST fail closed with respect to the restricted payload (do not serve disallowed details),
- it MUST still preserve canonical existence indicators and explanation placeholders,
Failure handling MUST NOT silently degrade safety or canonical integrity.

Failure handling MUST NOT silently degrade safety or canonical integrity.

---

## 13. Non-goals and exclusions [anchor: 13_non_goals_and_exclusions]

This specification does not:

- prioritize comfort over truth; canonical existence and challengeability are preserved,
- adjudicate legality beyond applying explicitly declared jurisdictional lenses and rulebook-defined distribution floors,
- prioritize comfort over truth; canonical existence and challengeability are preserved,
- personalize canonical reality feeds or create user-specific canonical universes,
- define ranking, ordering, or search behavior except insofar as withheld items must remain discoverable as withheld,
- define private draft handling (which remains out of canonical scope), except that safety rules MUST NOT compel disclosure of drafts.



## 14. Relationship to other specifications [anchor: 14_relationship_to_other_specifications]

### 14.1 Composition and authority (normative) [anchor: composition_and_authority_normative]

This specification is explicitly subordinate to Protocol v5 and composes with the following specifications, each of which governs a distinct layer of the system.

- **Protocol v5**  
  Defines constitutional invariants, canonical object roles, fixed connection types, scope rules, authorship constraints, and the fundamental separation between canonical state and presentation.  
  In any conflict, Protocol v5 is authoritative.

- **Protocol v5 Appendix A**  
  Defines the canonical schema and event semantics used to encode safety-related artifacts, including safety classification events, rulebook references, identifiers, and any explanation-relevant metadata.  
  This specification assumes Appendix A schemas and MUST NOT redefine canonical event structures.

- **Governance Specification**  
  Defines how safety rulebooks are proposed, adopted, superseded, composed, and challenged, including deterministic activation at cycle boundaries.  
  This specification relies on governance to determine which safety rulebooks are active, but does not define governance procedures itself.

- **Challenge Engine Specification**  
  Defines challenge domains, lifecycles, eligibility rules, tally computation, verdict finalization, and deterministic state transformations.  
  This specification constrains how safety interacts with challenges but does not modify challenge mechanics or outcomes.

- **Deterministic Replay & Merge Specification**  
  Defines replay rules, snapshot derivation, conflict resolution, and offline merge guarantees.  
  This specification assumes that safety classifications and rulebook activations are replay-safe and merge-safe under those rules.

- **Offline & Mindseed Specification**  
  Defines offline operation, packaging, portability, delayed synchronization, and explainability under disconnected or degraded conditions.  
  This specification requires that safety lenses, placeholders, and explanation paths remain usable offline.

- **Token Specification**  
  Defines POD, POINT, and any incentive mechanics and their strict non-use in governance weighting.  
  This specification assumes that safety decisions and rulebook adoption are not influenced or weighted by token balances.

- **Roles & Stewardship / AI Boundaries Specifications**  
  Define role responsibilities, stewardship expectations, and AI advisory boundaries.  
  This specification relies on those documents to constrain who may propose safety classifications, under what conditions AI assistance is permitted, and how accountability is preserved.

In all cases, Protocol v5 remains authoritative, and this document MUST NOT be interpreted in a way that alters canonical semantics, authorship rules, or replay determinism.

---

### 14.2 Explicit delegation map (normative) [anchor: explicit_delegation_map_normative]

The responsibilities across specifications are explicitly delegated as follows:

- **This specification (Safety Rulebook Interface & Visibility Mechanics)** defines:
  - safety labels and their semantics,
  - permitted visibility actions and safety floors,
  - abstraction and redaction mechanics,
  - jurisdictional and local lenses,

  - node and client conformance requirements for safety visibility.

- **Governance Specification** defines:
  - how safety rulebooks are created, adopted, superseded, and composed,
  - how safety-related governance challenges are initiated and resolved,
  - when rulebook changes become active (cycle boundaries).

- **Challenge Engine Specification** defines:
  - how safety decisions are contested through representation and governance challenges,
  - how disputes progress through challenge phases,
  - how verdicts modify rulebook state or classification state without retroactive erasure.

- **Protocol v5 Appendix A** defines:
  - the canonical event and object schemas used to encode safety classifications,
  - rulebook identifiers and references,
  - metadata required for deterministic explanation reconstruction.

- **Deterministic Replay & Merge Specification** and **Offline & Mindseed Specification** define:
  - how safety-related artifacts remain portable, verifiable, and deterministic,
  - how offline nodes apply lenses, safety floors, and explanations without external dependencies,
  - how merged logs preserve safety semantics.

## 15. Profile-v0 identity-admission boundary [anchor: profile_v0_identity_admission_boundary]

Safety rulebooks and interfaces are subordinate to the Profile-v0 identity-admission architecture. They MAY impose deterministic, disclosed, and reviewable presentation or process safeguards. They MUST NOT create a canonical `identity_create`, act as a sponsor, fabricate sponsor lineage, fabricate verification or an eligibility lane, reserve or mint invitation capacity, or use private accounts, relay records, AI output, wall-clock time, system emitters, or machine-only boundaries as admission authority.

Safety treatment of a non-canonical admission request does not make that request canonical. Safety restrictions MUST preserve canonical history and replay-derived authority. An authorized emergency rule may freeze capacity only where constitutional and rulebook authority expressly permits; it MUST NOT create emergency invitation capacity or a privileged admission path.


