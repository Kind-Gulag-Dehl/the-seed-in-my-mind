---
doc_id: safety_spec
title: Safety Specification
status: authoritative
version: v0
last_reviewed: 2026-01-27

scope:
  - Defines safety model, redaction rules, and visibility constraints.

authoritative_for:
  - Payload specificity handling and exposure gating.
  - Safety invariants and enforcement surfaces.

not_authoritative_for:
  - UI explanation mechanics (see safety-rulebook-interface-mechanics-spec.md).

depends_on:
  - protocol v5.md
  - governance-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of safety-rulebook-interface-mechanics-spec.md.

reader_path:
  - prereq: token-spec.md
  - next: safety-rulebook-interface-mechanics-spec.md

keywords:
  - safety
  - redaction
  - visibility
  - rulebooks
---

> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

## 0. purpose, scope, and invariants

### 0.1 purpose of this specification

The Safety, Visibility, and Emotional Load Specification defines how perceptual safety operates within the Seed in My Mind canonical universe. Its purposes are:

1. **Protect users** from harmful specificity, traumatizing content, and overwhelming emotional load while preserving epistemic integrity.
2. **Ensure compliance with jurisdictional legal constraints** without altering canonical ideas, structures, or replay semantics.
3. **Guarantee transparent, challengeable filtering**, so that all safety transformations are visible, explainable, contestable, and reproducible.
4. **Provide a deterministic framework** for classification, abstraction, redaction, and safe summarization of payloads (text, media, or structured descriptions) while maintaining the invariant that **ideas themselves are never removed or hidden**.

This specification is normative for all conformant nodes and clients.

---

### 0.2 non-negotiable invariants

The following invariants SHALL hold across all implementations, nodes, clients, rulebooks, classifiers, and jurisdictions:

1. **Idea-object visibility invariant**  
   Every idea MUST remain visible as an idea-object in all canonical views (lists, maps, D3, Garden, clusters, pathways).  
   Filtering MAY modify payload text but SHALL NOT remove, hide, or suppress the idea structurally.

2. **No safety filtering of canonical semantics**  
   Safety/emotional/jurisdictional filtering operates only on **perception-layer payloads**, not canonical structures.  
   Filtering SHALL NOT modify:
   - idea existence or type,  
   - connections,  
   - truth/importance/action processes,  
   - challenge eligibility,  
   - canonical rankings.

3. **Deterministic replay of safety events**  
   All classification, abstraction, and blocked-submission events MUST replay identically given the same safety rulebook and classifier versions.

4. **Full history retention**  
   The canonical universe retains all lawful historical events.  
   Only globally illegal payloads may be encapsulated using `merge_sanitization`, never deleted.

5. **Explainability requirement**  
   Any redaction, blur, collapse, or safe-summary MUST produce an explanation surface that discloses:
   - which rule triggered filtering,  
   - the emotional-load score (if relevant),  
   - the user’s threshold setting,  
   - jurisdictional restrictions,  
   - classifier decisions.  

6. **Challengeability requirement**  
   All safety decisions—including emotional-load scores, classifier outputs, jurisdictional masks, and safe summaries—MUST be open to representation challenges.

7. **User agency invariant**  
   Conformant **clients** MUST provide users with a personal emotional-load threshold control (e.g., a slider or equivalent interface) that can be adjusted at any time. This control dynamically reveals or hides payload detail according to the user’s preference, subject always to applicable jurisdictional law and non-distributable safety floors.  
   This control affects payload text visibility and presentation only—it MUST NOT alter idea structural presence, connections, importance rankings, challenge eligibility, or any other canonical semantic.

8. **Neutrality of filtering**  
   Filtering SHALL NOT:
   - bias canonical truth or importance outcomes,  
   - suppress minority worldviews or controversial truths,  
   - silence ideas structurally.  
   All ideas remain equally present and challengeable.

---

### 0.3 relationship to other specifications

This Safety Specification interacts with:

- **Protocol v5** (semantic invariants, challenge lifecycle, canonical event model).  
- **Node & Conformance Specification** (payload classes, validation rules, deterministic replay).  
- **Governance Specification** (safety rulebooks, classifier configuration, appeals).  
- **AI Boundaries Specification** (AI access constraints, summarization duties).  
- **Offline / Mindseed Specification** (propagation of safety metadata, offline classification behavior).  
Where conflicts arise, **Protocol v5 invariants supersede**, followed by this specification, then rulebooks.

Where conflicts arise, **Protocol v5 invariants supersede**, followed by this specification, then rulebooks.

---

### 0.4 normative vs informative content

- **Normative content** defines mandatory behavior for all conformant nodes and clients.  
- **Informative content** (examples, metaphors, UI suggestions) clarifies behavior but is not binding.  
In any ambiguity, interpretations MUST defer to deterministic replay semantics and Protocol v5 invariants.

---

## 1. conceptual foundations

### 1.1 idea vs payload vs specificity

The system distinguishes three layers:

1. **Ideas** - canonical epistemic entities.  
   - Immutable, challengeable, and always visible as objects.

2. **Payloads** - textual or media expressions attached to ideas.  
   - MAY be blurred, collapsed, substituted, or masked under safety rules.

   - Safety operates **only** on specificity, not on idea existence.
   - Some ideas are safe conceptually but harmful or illegal at high specificity.  
   - Safety operates **only** on specificity, not on idea existence.

---

---

### 1.2 physical presence vs mental perception

The Seed distinguishes between:

- **Physical Presence Layer**  
  Ideas appear as world-objects (trees, fungi, flowers, cards, nodes) and remain fully visible regardless of safety settings.

- **Perception Layer**  
  Payload text MAY be:
  - clean,  
  - blurred,  
  - collapsed to a safe summary,  
  - replaced by a jurisdictional placeholder,  
  - warning-gated,  
  - or legally masked.

Users may modify perception through personal threshold adjustments, but cannot alter the presence or structure of ideas.

---

### 1.3 goals of safety and emotional load

Safety and emotional-load systems pursue three objectives:

1. **Protect without distorting**  
   Users avoid harmful detail while ideas remain visible, challengeable, and structurally intact.

2. **Transparent, reversible filtering**  
   Users can always:
   - see that content exists behind a blur,  
   - understand why,  
   - click-to-reveal (unless jurisdictionally prohibited),  
   - challenge the filtering.

3. **Support emotional self-regulation**  
   The emotional-load slider allows users to intentionally pace their exposure to heavy content, zooming out or leaning in as needed.

---

### 1.4 emotional load as a continuous spectrum

Emotional load is a **continuous scalar**, not a categorical label.  
This design:

- avoids ideological fights over categories,  
- enables subtle threshold control,  
- aligns with the system’s comparative, spectrum-based reasoning ethos,  
- ensures filtering feels like controlled zooming rather than judgment or stigma,  
- allows rulebooks to adjust scoring models without redefining categories.

---

### 1.5 anti-censorship design philosophy

Safety is NOT a mechanism for suppressing ideas.  
Filtering MUST NOT:

- remove ideas from canonical maps or lists,  
- weaken controversial but lawful content,  
- distort historical or scientific truth claims.

Filtering is always:

- contextual,  
- reversible,  
- transparent,  
- user-controlled (within legal bounds),  
- semantically neutral.

The epistemic universe remains intact regardless of filtering settings.

---

### 1.6 jurisdictional visibility as an overlay

Jurisdiction-specific constraints apply to **payload detail**, not idea presence.

The system MUST:

- show that the idea exists,  
- reveal the most detailed legally permissible representation,  
- disclose reasons for abstraction,  
- distinguish between local masking and global canonical availability,  
- allow global maps to display which regions of discourse are open, blurred, or blocked.

Jurisdiction becomes metadata—not censorship—and must be visible and challengeable.

---

### 1.7 relationship to user experience and world metaphor

- Ideas (trees, fungi, flowers) always exist in the forest.  

- Ideas (trees, fungi, flowers) always exist in the forest.  
- Filtering is represented as fog around text, not removal of the tree.  
- The emotional slider controls visibility of text, not the idea.

This reinforces the principle that users control **how much detail they see**, but cannot pretend an idea does not exist.

## 2. safety rulebooks

### 2.1 safety rulebooks as ideas

Safety rulebooks SHALL be represented as ideas within the canonical graph, combining:

- a **conceptual_idea** component defining normative safety policies, and  
- an **actionable_idea** component defining activation, replacement, or supersession procedures.

Each safety rulebook is therefore:

3. **Immutable once adopted** — changes to rulebooks occur only through new rulebooks that supersede the previous ones; historical rulebooks remain intact for replay.
2. **Fully traceable** - rulebooks carry explicit provenance, authorship, activation history, and dependencies.  
Rulebooks MUST be stored as first-class ideas to ensure:

Rulebooks MUST be stored as first-class ideas to ensure:

- transparent governance,
- decentralized reinterpretation,
- auditable lineage,
- deterministic replay,
- and challengeability consistent with Protocol v5 epistemic principles.

---

### 2.2 activation at cycle boundaries
#### Normative alignment with governance activation semantics [anchor: safety_activation_semantics_alignment]

Safety rulebook activation follows governance decision cadence and scheduling semantics:
- decision confirmation at cycle end (`decision_event`),
- deterministic delay scheduling from `decision_cycle_index`, `change_class`, and `delay_policy_version`,
- activation at the start of `activation_cycle_index` (inclusive).

Snapshots remain verification and checkpoint artifacts and MUST NOT define activation boundaries.

Safety rulebooks MAY be proposed at any time but SHALL ONLY take effect at **scheduled cycle boundaries**.

A conformant node MUST:

1. Apply the currently active rulebook when producing any new safety-related event (classification, abstraction, sanitization).
2. Recompute all classification and abstraction results deterministically when replaying the chain, using the rulebook versions active at each historical cycle boundary.
3. Switch to a newly adopted rulebook only at the scheduled activation cycle boundary following its adoption.

This ensures:

- **deterministic replay of safety behavior**,  
- **forward-only evolution** of safety doctrine,  
- **stable classifier and threshold environments** between snapshots.

Nodes MUST NOT apply future rulebooks retroactively or apply rulebook changes at arbitrary points between snapshots.

---

### 2.3 allowed contents of safety rulebooks

Safety rulebooks MAY specify:

1. **Classification categories and payload classes**  
   - Mappings between classifier outputs and payload_class values (`normal`, `sensitive_abstracted`, `non_distributable_blocked`).

2. **Emotional-load scoring definitions**  
   - Scoring models, normalization curves, calibration parameters, and emotional intensity thresholds.

3. **Jurisdictional overlays**  
4. **Abstraction, sanitization, and summarization rules**  

4. **Abstraction, sanitization, and summarization rules**  
   - Procedures for generating safe summaries, blurs, and placeholders.
   - Deterministic text transformations.
   - Structure-preserving abstraction rules.

5. **Classifier and model references**  
   - Versioned models and configuration parameters.
   - Ensemble decision rules.
   - Required features or signals.

6. **Censorship-pressure detection thresholds**  
   - Metrics for identifying when safety systems disproportionately obscure content.

7. **Logging requirements**  
   - Metadata fields and formats for safety events.

Rulebooks MAY extend the system’s expressiveness in defining safe, deterministic, replayable transformations.  
Rulebooks MAY NOT redefine the canonical idea/connection/challenge primitives, importance logic, or idea presence invariants.

---

### 2.4 prohibited overrides

Safety rulebooks MUST NOT violate or undermine canonical invariants. They MAY NOT:

1. **Delete or retroactively modify canonical history**, except via sanctioned encapsulation of *globally illegal* payloads.  
2. **Suppress ideas structurally** (removal from maps, lists, or pathways; hiding nodes; altering connection visibility).  
3. **Modify truth, importance, or action processes**, including challenge lifecycles and verdict aggregation.  
4. **Introduce ideological, political, cultural, or worldview biases** into safety classification, emotional-load scoring, or abstraction rules.  
5. **Affect canonical rankings**, or distort epistemic outcomes through selective filtering.  
6. **Override Protocol v5 invariants** or node/client conformance requirements.

No rulebook may reduce the system’s censorship resistance or weaken its requirement that **all ideas remain present, visible, and challengeable**.

---

## 3. payload classification and specificity boundary

### 3.1 payload classes

All payloads SHALL be assigned exactly one of the following canonical payload classes:

1. **`normal`**  
   - Fully distributable within all safety and jurisdictional constraints.  
   - No abstraction required.

2. **`sensitive_abstracted`**  
   - Payload is lawful but requires abstraction, blur, collapse, or safe-summary replacement at certain specificity thresholds or emotional-load settings.

3. **`non_distributable_blocked`**  
   - Payload cannot legally or safely be admitted into the canonical universe.  
   - The system MUST record a `blocked_submission` event containing a safe summary and explanation metadata.

Payload class assignment is deterministic and governed by the safety rulebook active at the time of classification.

---

### 3.2 classification pipeline

Every new payload MUST flow through the safety classification pipeline:

1. **Ingest**  
   - Node receives a proposed idea description, edit, or replacement.

2. **Classify**  
   - Safety classifiers evaluate content for legal, safety, and specificity risks.

3. **Rulebook mapping**  
   - Classifier outputs are mapped—via the current safety rulebook—to:
     - payload_class,  
     - required abstractions,  
     - emotional-load scoring adjustments.

4. **Assign payload_class**  
   - Node assigns `normal`, `sensitive_abstracted`, or `non_distributable_blocked`.

5. **Emit safety event**  
   - Node records:
     - payload_class,  
     - emotional-load score,  
     - rulebook version,  
     - classifier version,  
     - transformation decisions.

6. **Store result**  
   - If blocked: emit `blocked_submission`.  
   - If abstracted: store both canonical version and safe-summary if required.

Nodes MUST follow this pipeline consistently for deterministic replay.

---

### 3.3 specificity boundary

The **specificity boundary** is the rulebook-defined line at which a payload becomes harmful, illegal, or overly detailed for certain jurisdictions or safety contexts.

Safety rulebooks MUST define:

- criteria for detecting overly specific harmful content,  
- deterministic abstraction or collapse rules,  
- when to substitute the payload with a safe-summary or placeholder.

Abstractions MUST:

- preserve the *conceptual meaning* of the idea,  
- retain challengeability (users can still challenge representation or abstraction),  
- avoid introducing misleading simplifications,  
- minimize epistemic distortion.

No abstraction may alter the underlying idea or its position in the canonical universe.

---

### 3.4 encapsulation and merge_sanitization

If a payload previously admitted to the canonical universe is later determined to be **globally illegal**, nodes MUST:

1. Replace the original payload with a **safe-summary encapsulation**.  
2. Preserve:
   - authorship metadata,  
   - timestamps,  
   - idea existence,  
   - challenge and connection structure.  
3. Emit a **merge_sanitization** event explaining:  
   - rulebook clause invoked,  
   - jurisdictional/legal basis,  
   - classifier inputs,  
   - safe-summary rationale.

Encapsulation SHALL NOT delete ideas or hide their presence; it only replaces unsafe textual detail with a legally compliant summary.

---

### 3.5 classifier determinism

Safety classifiers MUST produce deterministic outputs under deterministic conditions.

A conformant node MUST ensure:

1. Classifier behavior, when given identical payloads, rulebooks, and configuration, yields identical results.  
2. Replay from genesis—using historical rulebook versions—produces the same sequence of:  
   - payload_class assignments,  
   - emotional-load scores,  
   - abstractions,  
   - blocked submissions,  
   - merge_sanitization events.  
3. Model selection, parameters, hashing, and configuration versions are logged as safety metadata.

Non-deterministic classifier behavior renders a node non-conformant.

## 4. blocked submissions and illegal content

### 4.1 blocked_submission event type

A `blocked_submission` event SHALL be emitted when a proposed payload:

- cannot legally enter the canonical universe,  
- violates safety or legality requirements defined by the active safety rulebook, or  
- exceeds globally defined specificity thresholds that cannot be corrected by abstraction alone.

A conformant node MUST create a `blocked_submission` event containing:

1. **safe summary**  
   - A rulebook-governed safe-summary representing the intended meaning without illegal or harmful specificity.

2. **reason**  
   - A clear statement describing the classification outcome (e.g., illegal content, prohibited specificity, unallowable payload type).

3. **classifier evidence**  
   - Relevant classifier outputs, model references, and decision traces required to reproduce or challenge the decision.

4. **rulebook references**  
   - The specific clauses, versions, and jurisdictional overlays that produced the block.

The `blocked_submission` event SHALL be permanently recorded in the canonical chain and retained during deterministic replay.

---

### 4.2 canonical representation of blocked attempts

Blocked submissions MUST remain **visible as objects** in the canonical history.  
They SHALL NOT be deleted, hidden, or omitted from replay.

Their canonical representation ensures:

- The community can trace and reevaluate safety decisions over time,  
- The epistemic history includes attempted contributions even when disallowed,  
- The community can trace and reevaluate safety decisions over time,  
- Governance retains an accurate record of classifier and rulebook performance.

The system SHALL expose:

- the existence of the blocked submission,  
- its safe-summary,  
- its explanation surface,  
- its classifier and rulebook provenance.

Clients MUST display blocked submissions as legitimate historical events, distinct from admitted ideas but fully navigable and challengeable.

---

### 4.3 appeals and re-review

Blocked submissions SHALL be open to re-evaluation under three mechanisms:

1. **Challenge as misclassification**  
   Users MAY issue a representation challenge asserting that the submission was:
   - incorrectly blocked,  
   - improperly classified, or  
   - insufficiently abstracted.

2. **Rulebook changes**  
   If a new safety rulebook modifies:
   - the specificity boundary,  
   - jurisdictional requirements, or  
   - classifier interpretations,  
   then previously blocked submissions MUST be eligible for **automatic or manual reprocessing** at the next scheduled cycle boundary.

3. **Classifier version updates**  
A blocked submission MAY become admissible after re-evaluation.  

A blocked submission MAY become admissible after re-evaluation.  
If so, the system shall emit:

- a new **admission event**,  
- linking back to the original blocked submission,  
- while maintaining historical integrity.

---

## 5. emotional load system (continuous spectrum)

### 5.1 definition

Emotional load is a **continuous numerical score** representing the psychological intensity or emotional weight of a payload.

Its purpose is to:

- allow users to modulate exposure to heavy content,  
- prevent overwhelming or traumatic experiences,  
- maintain epistemic integrity while giving users agency,  
- avoid categorical labels that create unnecessary social, political, or ideological tension.

Emotional load affects **only textual perception**, never idea existence.

---

### 5.2 emotional-load scoring pipeline

A conformant node MUST compute emotional load using the following sequence:

1. **Model inference**  
   - A safety-classifier ensemble outputs a raw emotional-load estimate.

2. **Rulebook normalization**  
   - The safety rulebook defines normalization curves, clamps, calibration factors, and adjustments.

3. **Score assignment**  
   - The normalized emotional-load value is stored as:
     - payload emotional-load score,  
     - safety metadata associated with the idea’s descriptive tier.

4. **Event logging**  
   - The emotional-load score, classifier version, and rulebook clause MUST be recorded in safety metadata for deterministic replay.

Nodes MUST apply the same scoring pipeline under identical conditions during replay.

---

### 5.3 per-user emotional visibility threshold

Every user SHALL have a **personal emotional visibility threshold** (conceptually a slider).

This threshold determines how much textual detail is revealed:

- **Higher threshold →** more detail, clearer text, full descriptions where allowable.  
- **Lower threshold →** more blur, more collapse, greater reliance on safe summaries.

A conformant client MUST:

- allow users to adjust this threshold at any time,  
- reflect threshold changes immediately in the perception layer,  
- never modify idea presence, structure, or ranking,  
- display clear indicators when detail is hidden due to emotional load.

The threshold applies only to **payload visibility**, not canonical inclusion or metadata.

---

### 5.4 allowable perceptual effects

When emotional load exceeds the user’s threshold, clients MAY apply:

1. **blur**  
   - Text is obscured but visibly present.

2. **collapse**  
   - Text shrinks into a collapsed region with a short safe-summary.

3. **safe-summary substitution**  
   - A deterministic rulebook-defined summary replaces the original text.

4. **warning-gate or click-to-reveal**  
   - Users intentionally reveal heavier content after acknowledgment.

However, the system MUST NOT:

- remove idea objects,  
- remove ideas from lists or maps,  
- alter the idea’s structural presence or importance.

Only **textual detail** may be masked or transformed.

---

### 5.5 anti-censorship constraints

The emotional-load system SHALL NOT function as a censorship mechanism.

Specifically:

1. **No filtering from canonical views**  
   Emotional load MUST NOT prevent an idea from appearing in:
   - global lists,  
   - top-N importance views,  
   - the forest/world map,  
   - pathways or comparison views.

2. **No rank distortion**  
   Emotional load MUST NOT influence:
   - truth or importance judgments,  
   - challenge outcomes,  
   - idea ordering or comparisons.

3. **No suppression of controversial truths**  
   Heavy topics remain structurally present regardless of emotional intensity.

4. **Default threshold constraints**  
   Default system settings MUST NOT hide substantial portions of high-importance content.  
   Users must always be able to reveal full content (subject to jurisdictional law).

5. **Client disclosure requirement**  
   Clients MUST indicate:
   - when text is hidden,  
   - how much text is hidden,  
   - why it is hidden (emotional load threshold, not structural censorship).

The emotional load system is a **user-controlled perceptual filter**, not a gatekeeping mechanism.

## 6. visibility semantics and user perception

### 6.1 idea presence invariance

A conformant client SHALL ALWAYS display all ideas as **visible idea-objects**, regardless of any safety, emotional-load, or jurisdictional settings.

This invariance includes:

- appearance in the world/forest map,  
- appearance in D3 or graph-based viewers,  
- presence in idea lists, search results, and pathways,  
- structural connections to other ideas.

Filtering MAY affect only the **payload text**, never the structural presence, ordering, or discoverability of ideas.

No filtering mechanism—emotional, safety, or jurisdictional—MAY hide an idea or remove it from canonical presentations.

---

### 6.2 textual visibility states

Clients MUST support the following textual visibility states, applied per-payload according to safety rules, emotional-load thresholds, and jurisdictional overlays:

1. **clean**  
   - Full text visible.

2. **blurred**  
   - Text is visually obscured but present; blur intensity governed by rulebook guidelines.

3. **warning-gated**  
4. **safe-summary**  

4. **safe-summary**  
   - Payload replaced with a deterministic, rulebook-defined summary preserving conceptual meaning but removing harmful specificity.

5. **placeholder**  
   - Jurisdictionally required masking of text where no safe detail may legally be shown; replaced with a minimal descriptor explaining legal constraint.

All visibility states MUST be reversible by user action unless prohibited by jurisdictional law.

---

### 6.3 disclosure of hidden/abstracted content

Clients MUST clearly disclose when any text has been filtered, abstracted, or masked.

Disclosure SHALL include:

- **why** the payload is hidden:  
  - emotional-load threshold,  
  - safety rulebook requirement,  
  - jurisdictional law.

- **how many fields or segments** of text are affected.

- **how to reveal or request more detail**, including:  
  - adjusting emotional-load threshold,  
  - reading a safe-summary,  
  - viewing a rulebook explanation panel,  
  - seeing jurisdictional restrictions.

Explanation surfaces MUST be accessible directly from the filtered region.

The user MUST never be misled into thinking no content exists when content is merely filtered.

---

### 6.4 canonical views immune to filtering

Safety, emotional-load, and jurisdictional filtering SHALL NOT alter the structural presence, ordering, or placement of **any idea** in **any canonical or navigational view**.

Specifically:

1. **All ideas remain present in all views**  
   Ideas MUST appear in the same structural positions—forest/world map, D3 layouts, idea lists, pathways, comparison views—regardless of filtering or masking of textual payloads.

2. **No filtering may modify ordering or rank**  
   Rankings, sort order, or relative placement MUST remain identical whether text is:
   - clean,  
   - blurred,  
   - collapsed,  
   - substituted with safe-summary, or  
   - jurisdictionally masked.

3. **Canonical importance views**  
   All ideas MUST remain included according to their canonical rank; filtering MAY NOT exclude or reorder them.

4. **Structural views**  
   Maps, graphs, and cluster layouts MUST reflect the canonical universe exactly; filters MAY NOT hide nodes or connections.

5. **Challenge and comparison surfaces**  
   Ideas under evaluation, challenge, or comparison MUST always appear structurally unchanged.

Filtering MAY affect **textual content only**, never:

- idea existence,  
- the idea’s location in any map or viewer,  
- connections or pathways,  
- rank position,  
- visibility in lists or search results.

The canonical epistemic universe MUST be fully visible at all times, independent of perception-layer filtering.


---

## 7. jurisdictional safety and visibility

### 7.1 jurisdictional overlays

Nodes subject to jurisdictional law MUST apply legally required constraints to **payload distribution**, not idea presence.

Jurisdictional overlays SHALL:

- constrain the **level of permissible specificity**,  
- define when abstraction or masking is mandatory,  
- never alter the canonical chain or structural presence of ideas.

All jurisdictional effects operate on the **perception layer**, not on the canonical epistemic universe.

---

### 7.2 jurisdictional visibility lenses

Clients MUST support **visibility lenses** that reflect jurisdiction-specific constraints.

A jurisdictional lens MUST:

1. Display the **most detailed legally permissible representation** of each payload.  
2. Apply fallback rules in order:
   - show full text if permitted,  
   - otherwise show safe-summary,  
   - otherwise show placeholder with explanation of legal constraint.

3. Preserve structural visibility regardless of masking.

Users MAY switch between:

- **global canonical view** (conceptual "no filters" reference), and  
When switching views, idea structure MUST remain unchanged.

When switching views, idea structure MUST remain unchanged.

---

### 7.3 regulatory restriction metadata

For each idea and each jurisdiction, nodes MUST attach **regulatory restriction metadata** including:

- jurisdiction identifier,  
- type of restriction (partial abstraction, full masking, distribution ban),  
- rulebook clause references,  
- classifier or legal basis for the restriction,  
- activation cycle index corresponding to governance adoption.

This metadata MUST be visible in explanation surfaces to ensure transparency and challengeability.

---

### 7.4 global zoomed-out visibility map

Clients MUST provide a **zoomed-out visibility map** enabling users to understand how various jurisdictional lenses affect large-scale access to ideas.

The map SHALL:

- preserve global structural layout,  
- visually differentiate:
  - fully visible regions,  
  - partially abstracted regions,  
  - fully masked regions,  
- allow switching between:
  - global canonical (unfiltered) representation,  
This map SHALL NOT reveal restricted payloads themselves; it reveals **patterns of restriction**, not forbidden content.

This map SHALL NOT reveal restricted payloads themselves; it reveals **patterns of restriction**, not forbidden content.

Its purpose is transparency, not evasion of jurisdictional law.

---

### 7.5 jurisdictional challengeability

Jurisdiction-driven filtering decisions MAY be challenged under **representation challenges** when users believe:

- the abstraction is excessive relative to legal requirement,  
- the safe-summary does not reflect the canonical payload accurately,  
- the system misinterpreted jurisdictional constraints,  
- masking is inconsistent with rulebook definitions.

Challenges MUST proceed via the standard protocol challenge lifecycle and MUST be recorded on the canonical chain.

A successful challenge MAY:

- update jurisdictional rule interpretation,  
- require a new safe-summary,  
- adjust visibility lens behavior,  
- or create new rulebook provisions.

The canonical chain MUST retain the full history of jurisdictional interpretations and their evolution.


## 8. explanation surfaces

A conformant client MUST provide an **explanation surface** for every instance in which payload text is:

A conformant client MUST provide an **explanation surface** for every instance in which payload text is:

- blurred,  
- collapsed,  
- replaced with a safe-summary,  
- placeholder-masked due to jurisdiction, or  
- warning-gated for emotional-load reasons.

The explanation surface SHALL be accessible directly from the affected region (e.g., clicking a blurred text block, collapsed section, or placeholder).

Every explanation object MUST include the following canonical metadata:

1. **payload_class**  
   - The classification assigned (`normal`, `sensitive_abstracted`, `non_distributable_blocked`).

2. **emotional-load score**  
   - The numerical emotional-load value attached to the payload.

3. **user threshold**  
   - The current emotional-load threshold determining visibility.

4. **rulebook clauses**  
   - Specific safety rulebook references (by ID and version) that triggered any filtering, abstraction, or masking.

5. **jurisdiction restrictions**  
   - Any applicable legal overlays, including:
     - jurisdiction identifiers,  
     - restriction type (partial abstraction, mandatory masking),  
     - activation timestamp.

6. **classifier decisions and evidence**  
   - The outputs of the safety classifiers, including:
     - model version(s),  
     - decision traces or confidence values required for replay,  
     - rationale summaries if mandated by the rulebook.

7. **abstraction or sanitization reasons**  
   - Explanation of why the payload was transformed, referencing:
     - specificity boundary,  
     - safety categories,  
     - abstraction rules invoked,  
     - whether safe-summary generation was required.

The explanation surface MUST enable users to understand precisely **why** a transformation occurred, how to **override** it (if permissible), and what **legal or rulebook constraints** apply.

Explanation surfaces MUST be available even when payloads are entirely masked (e.g., placeholder due to strict jurisdiction).

---

### 8.2 challenge surfaces

A conformant client MUST provide a **challenge surface** directly accessible from the explanation panel.

From this panel, users SHALL be able to initiate:

1. **representation challenges**, asserting:  
   - the filtering or abstraction misrepresents the idea,  
   - the safe-summary is incorrect or misleading,  
   - the emotional-load score is miscalibrated,  
   - classifier or rulebook rules have been applied incorrectly.

2. **jurisdiction interpretation challenges**, asserting:  
   - a jurisdiction restriction was over-applied,  
   - the system misinterpreted legal requirements,  
   - masking is inconsistent with rulebook or law.

3. **rulebook challenges**, when a user believes:  
   - the rulebook defines harmful or overly restrictive safety behavior,  
   - abstraction rules are distorting meaning,  
   - emotional-load normalization is inappropriate.

Challenge surfaces MUST route to the standard Protocol v5 challenge lifecycle (creation → open arguments → voter selection → voting → verdict → state transformation).

All challenge events MUST be recorded canonically.

---

## 9. minors, sensitive modes, and safety roles

### 9.1 minors

Conformant implementations MUST apply special safety considerations for minors.

The system SHALL:

1. **assign lower default emotional-load thresholds**,  
   - reducing exposure to high-intensity content by default.

2. **apply additional gating**  
   - e.g., more frequent warning-gates, stricter safe-summary fallback.

3. **support parental or guardian controls**  
   - as defined by safety rulebooks, allowing:
     - threshold locking,  
     - prevented overrides,  
     - restricted toggling of jurisdiction lenses.

Rulebooks MAY define additional protections, but they MUST NOT alter idea presence or canonical structure.

Minors MUST still see that ideas exist (object presence invariant), with text filtered as needed.

---

### 9.2 sensitivity modes

Clients MUST support at least two safety modes, with rulebooks allowed to define more:

1. **High-sensitivity mode**  
   - Lower emotional-load threshold,  
   - More conservative abstraction and masking,  
   - Additional warning-gates.

2. **Research mode**  
   - Highest permissible emotional-load threshold,  
   - Minimal filtering (except for illegal content or jurisdictional constraints),  
   - Designed for researchers, stewards, auditors, and experienced users.

Switching modes MUST NEVER alter idea structure or remove ideas from any view.  
Modes modify textual perception only.

---

### 9.3 stewards and auditors

Certain users (e.g., governance participants, elected stewards, safety auditors) MAY require elevated safety visibility to perform oversight functions.

Rulebooks MAY authorize:

1. **expanded access to safe-summaries**,  
2. **reduced gating**,  
3. **research-mode defaults**,  
4. **audit logs and classifier decision traces**,  
5. **visibility into jurisdictionally-filtered structures** at an abstracted level.

However:

- Elevated safety access SHALL NOT allow exposure to globally illegal specifics.  
- Elevated safety access SHALL NOT bypass jurisdictional law where legally binding.  
- Stewards and auditors MAY NOT modify idea presence or ranking.

Their expanded visibility is for governance and verification, not epistemic influence.

## 10. offline replicas, mindseeds, and reconciliation

### 10.1 safety metadata in mindseeds

Mindseeds MUST include all safety metadata necessary for deterministic replay.  
For every payload stored offline, nodes SHALL record:

- **payload_class** (`normal`, `sensitive_abstracted`, `non_distributable_blocked`),  
- **emotional-load score**,  
- **jurisdictional metadata** active at time of export,  
- **classifier model identifiers and versions**,  
- **abstraction or safe-summary metadata**, including which rulebook clauses were invoked.

This ensures that safety decisions made offline can be re-evaluated consistently during reconciliation, even if rulebooks change.

---

### 10.2 offline classification

Offline classification is **provisional**.

Conformant nodes MAY:

- apply cached rulebooks,  
- run cached classifiers,  
- assign provisional payload_class values,  
- generate provisional safe-summaries or abstractions.

However:

1. Offline safety decisions are NEVER authoritative.  
2. Nodes MUST mark all such decisions as provisional.  
3. Offline nodes MUST prevent globally illegal content from entering the mindseed, even provisionally.  
4. Provisional classifications MUST be fully reprocessed upon reconnection.

---

### 10.3 reconnection rules

Upon reconnection to the network, nodes MUST:

1. **Re-run classification** on all offline-created payloads using the *current* active rulebooks and classifier versions.  
2. **Apply merge_sanitization** or updated abstraction when:
   - legal interpretations changed,  
   - specificity boundaries shifted,  
   - rulebook clauses differ from the cached version.  
3. **Emit correction events** for any changes:
   - classification updates,  
   - emotional-load adjustments,  
   - new abstraction levels,  
   - changes in jurisdiction visibility.  
4. **Preserve idea presence**  
   Ideas themselves MUST remain present even when payload text is abstracted or replaced.  
5. **Store full discrepancy logs** recording:
   - original provisional values,  
   - corrected values,  
   - rulebook and classifier references triggering the change.

Reconciliation MUST be deterministic and replayable across all conformant nodes.

---

## 11. ai roles, visibility, and payload handling

### 11.1 AI-generated material and safety processing

AI helpers MAY generate non-canonical draft text. AI-generated material that is surfaced for possible human adoption MUST enter the safety pipeline in the same way as human-proposed text:

- classification,  
- emotional-load scoring,  
- abstraction or sanitization,  
- jurisdiction overlays,  
- possible blocked_submission events.

AI is never a canonical author. A canonical candidate incorporating AI-generated material requires the ordinary valid human author and signature for its event family; AI-origin metadata, if retained, is non-authoritative and MUST NOT confer authorship, sponsorship, verification, eligibility, or authority.

---

### 11.2 ai visibility into the canonical map

AI helpers MAY access the **full canonical map**, including:

- all ideas,  
- all public descriptions,  
- all abstracted payloads,  
- all safe summaries,  
- all structural relations.

AI internal visibility is NOT restricted by:

- emotional-load settings of any user,  
- sensitive modes,  
- personal filters,  
- jurisdictional masks applied to user-visible text.

The ONLY restriction is:

- **Globally illegal encapsulated content MUST NOT be exposed to AI**, except in its safe-summary form.

AI may therefore reason over more content than any given user currently sees—this is intentional.

---

### 11.3 ai output to users and safety filtering

While AI may internally access the full canonical map, any text it outputs to a user MUST pass through the standard safety pipeline:

1. payload classification,  
2. emotional-load scoring,  
3. application of that user's emotional-load threshold,  
4. jurisdiction-based masking or abstraction,  
5. safe-summary substitution if required,  
6. blocked_submission rules if applicable.

This ensures:

- users only see what they have chosen (or are legally allowed) to see,  
- AI access does not bypass the user’s settings,  
- emotional-load filters remain meaningful without constraining AI internally.

Users MAY always increase their threshold or request more detail.

---

### 11.4 no constraints on ai generation

The Seed protocol SHALL NOT restrict or modify upstream AI model behavior.

Conformant nodes MUST NOT:

- enforce output limitations on external AI models,  
- attempt to prevent generation of harmful or operational content,  
- distinguish allowed/disallowed generations at the model level.

Instead, ALL safety enforcement occurs **after** the payload is generated, at ingestion time.

This guarantees parity:

- Humans can paste external model output.  
- The system cannot meaningfully "prevent" model generation.  
- Safety is applied uniformly to the text once produced.

Payload origin NEVER changes safety processing logic.

---

### 11.5 ai neutrality and epistemic constraints

While AI may analyze and explain ideas, it MUST NOT:

- hide ideas or distort visibility,  
- reorder content based on emotional load or safety metadata,  
- manipulate importance or truth processes,  
- misrepresent safety restrictions as intrinsic properties of ideas,  
- override canonical structure,  
- influence governance or voting.

AI MUST remain an **advisory tool** only.  
It cannot change canonical state or restrict user access.

---

### 11.6 sandboxed ai roles (optional)

Rulebooks MAY define sandboxed modes for AI intended for:

- classification assistance,  
- summarization into safe-summaries,  
- large-scale structural or coherence analysis,  
- idea proposal workflows.

Sandbox AI MAY access:

- all canonical content except illegal specifics,  
- historical event logs,  
- abstracted payloads.

However, sandbox AI MAY NOT:

- reconstruct specificity beyond what is stored canonically,  
- restore illegal content,  
- write directly into canonical history without human confirmation,  
- bypass deterministic safety transformations.

Sandbox roles provide enhanced reasoning capabilities—not enhanced authority.

### 11.7 Profile-v0 admission safety boundary

Safety controls MAY govern presentation, access friction, safe handling of voluntarily supplied non-canonical request material, and process safeguards within their delegated authority. They MUST NOT fabricate canonical identity, sponsorship, lineage, verification, invitation capacity, eligibility, or a qualifying capacity period. Safety systems MUST NOT delete canonical history or silently rewrite replay-derived authority.

Any admission-specific restriction MUST be objective, challengeable where the applicable protocol permits challenge, and independent of viewpoint, political agreement, wealth, token ownership, private-account status, social rank, or AI approval. A constitutionally authorized emergency rule MAY freeze existing invitation-capacity spending only through its explicit canonical rulebook process; it MUST NOT mint replacement capacity by operator, AI, system, wall-clock, or machine-only action.

## 12. governance, rulebooks, and challenge processes

### 12.1 rulebook lifecycle

Safety rulebooks are themselves ideas and MUST follow the standard governance and challenge lifecycle defined by the protocol:

1. **Proposal**  
   A rulebook revision is submitted as a governance-action proposal, containing:
   - full text of the new rulebook,  
   - clear diff against the prior version,  
   - justification and intent,  
   - references to relevant safety incidents or classifier updates.

2. **Debate**  
   Users MAY submit arguments, counterarguments, and supporting ideas.  
3. **Verdict**  

3. **Verdict**  
   A governance challenge proceeds through:
   - voter selection,  
   - voting window,  
   - deterministic verdict aggregation.

   Verdicts SHALL be:
   - canonical,  
   - replayable,  
   - tied to a unique block height.

4. **Activation**  
Normative clarification [anchor: safety_rulebook_lifecycle_activation_alignment]: safety rulebook activation MUST use governance-computed `activation_cycle_index` and deterministic delay policy by `change_class`.
   A new rulebook version SHALL activate:
   - at the scheduled activation cycle boundary,  
   - after all nodes have received the update,  
   - with version IDs recorded in all subsequent safety events.

Nodes MUST NOT apply a new rulebook retroactively except where required for global legality (e.g., merge_sanitization).

---

### 12.2 emotional-load governance

Rulebooks MAY define the normative structure of emotional-load behavior, including:

1. **Calibration rules**  
   - how raw model scores are normalized,  
   - how cross-model discrepancies are resolved,  
   - which features or payload dimensions contribute to emotional intensity.

2. **Minimum default thresholds**  
   - default slider positions for new users,  
   - stricter defaults for minors or sensitive-mode configurations,  
   - optional recommended ranges (not mandates).

3. **Safety model definitions**  
   - classifier versions and required training features,  
   - acceptable explanation formats,  
   - approved abstraction/summarization methods.

Rulebooks MUST NOT allow emotional load to influence:
- canonical importance,  
- truth processes,  
- structural visibility of ideas.

Emotional load is a *pacing tool*, not an epistemic filter.

---

### 12.3 anti-censorship protections

To preserve the epistemic integrity of the reasoning universe:

1. **Emotional load MUST NOT be weaponized**  
   Nodes and clients MUST NOT use emotional-load values to:
   - remove ideas from lists,  
   - reorder rankings,  
   - distort structural visibility,  
   - hide controversial but lawful ideas.

2. **Rankings remain canonical**  
   Safety settings SHALL NOT influence:
   - universal importance ranking,  
   - personal or tribe rankings,  
   - truth judgments,  
   - action structure.

3. **Canonical visibility is invariant**  
   All ideas MUST appear in their correct canonical positions, regardless of:
   - personal filters,  
   - emotional-load thresholds,  
   - jurisdictional restrictions (text only),  
   - sensitivity modes.

Safety affects text, not idea presence.

---

### 12.4 censorship-pressure metrics

To ensure transparency and detect subtle suppression patterns, conformant nodes MUST compute and expose **censorship-pressure metrics**, including:

1. **suppression-rate indicators**  
   - percentage of content blurred, collapsed, or abstracted in typical sessions,  
   - per-jurisdiction abstraction frequency,  
   - clustering of high emotional-load regions.

2. **outlier detection**  
   - detection of unusually high filtering around politically or socially sensitive topics,  
   - detection of sudden increases in jurisdictional masking.

3. **governance alerts**  
   - automated signals when suppression patterns exceed rulebook-defined thresholds,  
   - surfacing anomalies for human review.

4. **public transparency surfaces**  
   - dashboards showing how much content is suppressed and why,  
   - historical trends correlated with rulebook changes.

These metrics MUST NOT identify individual users or expose private content; they measure system behavior, not user behavior.

---

## 13. node and client conformance

### 13.1 node obligations

Conformant nodes MUST:

1. **Perform deterministic classification**  
   Classification and emotional-load scoring MUST be repeatable under deterministic replay.

2. **Preserve canonical storage**  
   Nodes MUST store canonical ideas, safe-summaries, abstractions, and metadata without alteration.

3. **Enforce abstraction and jurisdiction rules at distribution**  
   Nodes MUST apply:
   - safe-summary substitution,  
   - abstraction levels,  
   - jurisdiction masking,  
   **only** at the moment of distribution to clients.

4. **Surface all safety metadata**  
   Nodes MUST provide:
   - payload_class,  
   - emotional-load scores,  
   - jurisdiction restrictions,  
   - rulebook references,  
   - classifier versions,  
   - explanation-surface metadata.

Nodes MUST NOT silently drop or omit safety metadata.

---

### 13.2 client obligations

Conformant clients MUST:

1. **Always display idea presence**  
   - No idea may be removed from any canonical or navigational view.  
   - The tree/flower/mushroom icon MUST remain visible even if text is blurred or abstracted.

2. **Provide emotional-load threshold controls**  
   - Users MUST be able to raise or lower their slider at any time.  
   - Changes MUST update the visible text immediately.

3. **Render explanation surfaces**  
4. **Show placeholders and suppression notices**  

4. **Show placeholders and suppression notices**  
   - For jurisdictional masking, placeholder text and explanation MUST be shown.

5. **Never silently hide content**  
   - If text is unavailable, clients MUST show the reason.

Client behavior MUST remain transparent, predictable, and explainable.

---

### 13.3 non-conformance conditions

A node or client SHALL be declared **non-conformant** if it:

1. **Hides ideas**  
   - removes them from lists,  
   - omits them from structural maps,  
   - fails to display their presence in any view.

2. **Alters ranks**  
   - modifies universal or personal importance rankings based on safety settings,  
   - reorders lists due to emotional-load or jurisdiction restrictions.

3. **Re-exposes illegal specificity**  
   - restores blocked or encapsulated content,  
   - bypasses safe-summary masking,  
   - reveals jurisdictionally-forbidden detail.

4. **Misrepresents jurisdiction restrictions**  
   - displays contradictory or missing information about why content is blocked,  
   - falsely attributes filtering to user settings or rulebooks.

5. **Suppresses explanation metadata**  
   - omits payload_class, rulebook references, or classifier versions.

Non-conformant clients or nodes MUST NOT be allowed to participate in canonical state synchronization.

## 14. appendices (informative)

Appendices provide **examples**, **illustrations**, **reference patterns**, and **non-normative guidance** to support implementers.  
They SHALL NOT introduce new requirements or alter normative behavior defined in Sections 0-13.  
All examples are illustrative only; rulebooks and canonical protocol text remain authoritative.

---

### A. example classification rules

This appendix MAY include:

- example rulebook-defined boundaries between `normal`, `sensitive_abstracted`, and `non_distributable_blocked`,
- sample payloads showing why a classifier assigns each payload_class,
- walkthroughs of deterministic classifier decisions under replay.
- examples of multi-model classifier agreement thresholds,
- walkthroughs of deterministic classifier decisions under replay.

These examples help implementers validate classifier behavior without dictating exact model architecture.

---

### B. abstraction and sanitization transformations

This appendix MAY include:

- examples of safe-summary generation,  
- transformations from specific → general language,  
- examples of collapsing highly specific payloads into conceptual descriptions,  
- before/after comparisons demonstrating how meaning is preserved while specificity is reduced,  
- canonical placeholder formats used when necessary.

Examples illustrate how rulebooks should preserve **semantic integrity** while removing unsafe detail.

---

### C. emotional-load scoring examples

This appendix MAY illustrate:

- raw model scoring values,  
- rulebook-defined normalization curves,  
- how emotional-load interacts with payload_class,  
- example distributions for typical text, imagery, or combined payloads,  
- visualization of how slider thresholds reveal or blur content.

Examples show how emotional load functions as a **pacing mechanism**, not a censorship tool.

---

### D. jurisdiction lens examples

Examples MAY include:

- how a payload appears under three different jurisdiction lenses (e.g., Global Canonical, EU Lens, Country-X Lens),  
- fallback paths from full detail → partial detail → safe-summary → placeholder,  
- metadata formats for restriction references,  
- how clustered topic regions appear differently under each lens,  
- transparency indicators explaining why text is masked.

These examples help implementers demonstrate regulatory compliance without sacrificing global epistemic visibility.

---

### E. zoomed-out visibility map mockups

Appendix MAY include mockups or diagrams illustrating:

- the global idea map with visibility color coding (e.g., fully visible / partially abstracted / fully masked),  
- how entire ideological or conceptual clusters appear under different jurisdictional lenses,  
- time-lapse views showing visibility changes after rulebook updates,  
- UI affordances for toggling between views.

These mockups clarify how users perceive **structural suppression patterns** without exposing restricted payloads.

---

### F. classifier configuration examples

This appendix MAY provide sample configurations:

- multiple classifier models with weighted consensus,  
- ensembles for emotional-load estimation,  
- deterministic prompt templates for AI-assisted analysis,  
- example logs showing classifier version, parameters, and output,  
- replay metadata demonstrating deterministic outcomes.

Configurations illustrate extensibility while preserving deterministic replay.

---

### G. special cases

This appendix MAY describe nuanced or unusual scenarios, including:

1. **legacy rulebook migrations**  
   - revising safety rules over time,  
   - reconciling old classifications under new rulebooks,  
   - handling content that becomes newly allowed or newly illegal.

2. **jurisdiction rule evolution**  
   - how nodes respond to changing laws,  
   - examples of retroactive abstraction when required legally,  
   - reconciliation with existing snapshots.

3. **ai-generated toxic payloads**  
   - examples of highly unsafe AI-generated input,  
   - demonstration of the safety pipeline handling it identically to human-generated text,  
   - blocked_submission events with safe summaries.

These examples reinforce that **payload origin does not affect safety processing**.

---

### H. test vectors and conformance scenarios

This appendix MAY include:

- reference test vectors for classification, emotional-load, abstraction, and masking,  
- deterministic replay scenarios exercising edge cases,  
- tests ensuring idea presence invariance across clients,  
- conformance failures and expected system responses,  
- minimal test cases for node implementers.

These scenarios support the development of conformance suites.

---

### I. future extensions

Informative guidance MAY explore:

- potential integration of multimedia payload classification,  
- advanced jurisdiction mapping tools,  
- community-defined safety lenses for research contexts,  
- improved emotional-load calibration through federated learning,  
- extensions for new payload modalities,  
- alternative safe-summary algorithms.

These proposals SHALL NOT constrain the current version of the protocol but MAY inform later revisions.

