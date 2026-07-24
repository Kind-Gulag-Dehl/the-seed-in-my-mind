---

title: AI Map Realms, Projections, and Lifecycle
project: The Seed in My Mind
status: current private product architecture specification
version: v1
last_updated: 2026-07-12
authority_class: private product architecture
authoritative_for:

* AI map realm terminology
* private product map projections
* AI Commons visibility and lifecycle
* autonomous simulation and model-map distinctions
* private adoption, public AI sharing, and canonical promotion distinctions
* document and URL ingestion product behavior
  not_authoritative_for:
* canonical event schemas
* canonical replay or snapshots
* open-core conformance
* governance semantics
* POD or POINT mechanics
* canonical eligibility or verification rules
* exact API, database, or frontend contracts

---

# AI Map Realms, Projections, and Lifecycle

## 1. Purpose, Scope, and Authority

The Seed in My Mind contains several forms of ideas that differ in ownership, visibility, durability, and authority. The initial Seed and later human-authored canonical events form the permanent canonical map. Users may also maintain private maps containing human-written or AI-assisted ideas. In addition, the product may expose public AI-generated ideas, autonomous AI simulations, and model-specific views that are visible and useful without becoming part of canonical history.

These surfaces must not be treated as interchangeable. A publicly visible AI-generated idea is not canonical merely because everyone can see it. A private idea is not canonical merely because a human controls it. An autonomous simulation may imitate the canonical process without acquiring any canonical authority. A model-specific view may display ideas created by one model without becoming a separate authoritative universe.

This document defines the private product architecture that distinguishes these realms and explains how they may overlap through projections. It establishes the preferred terminology for the canonical map, private maps, the AI Commons, simulation universes, model universes, model lenses, and the combined public map. It also defines the intended lifecycle of non-canonical AI artifacts and the transitions between private adoption, public AI sharing, and canonical promotion.

The document is authoritative within the private product repository for map-realm vocabulary, projection rules, visibility boundaries, AI lifecycle concepts, and the intended launch experience. It is not a public protocol specification. It does not define canonical event formats, replay behavior, snapshot formats, governance mechanics, token accounting, verification, or canonical-write eligibility.

Where this document conflicts with the open-core protocol or its delegated specifications, the open-core authority controls. In particular, this document remains subordinate to the canonical protocol, the AI boundary specification, the node and conformance specification, the open-core data-boundary specification, the offline and Mindseed specification, and any other authoritative open-core document governing canonical state.

Existing private documents concerning agent maps, AI mirror maps, model maps, Ents, Builder behavior, document ingestion, connected-idea expansion, and shared AI artifacts remain useful within their narrower scopes. Where their general map terminology conflicts with this document, this document supplies the preferred current product vocabulary.

## 2. Core Principles

The architecture is governed by a small number of principles that apply across all map views and AI features.

### 2.1 Canonical authority comes only from canonical history

The canonical map is derived from the canonical event log. The initial Seed is its bootstrap state, and later canonical changes must occur through valid canonical events. Ordinary canonical writes require eligible human authorship or explicit human adoption under the governing protocol.

AI may help generate, analyze, summarize, compare, challenge, or organize ideas, but AI output cannot independently create canonical effects. AI does not become a canonical author by generating useful content, participating in a simulation, or producing output that humans later view or share.

### 2.2 Visibility and authority are separate

An idea may be publicly visible without being canonical. Public AI-generated ideas may appear beside canonical Seed ideas in the same map interface, but their authority remains different.

The product must never rely on visibility, visual prominence, popularity, model confidence, adoption count, or AI activity to imply canonical truth or canonical importance. Public AI content must remain clearly marked as AI-generated and non-canonical.

### 2.3 Private control is not canonical authority

A user may create, edit, rank, connect, or delete ideas in a private map. Those ideas may be written directly by the user or generated with AI assistance. Once an AI-generated idea is adopted into a private map, it becomes human-controlled private content, but it remains non-canonical.

Private ownership determines who may manage the content. It does not make the content part of shared canonical reality.

### 2.4 Public AI sharing is not canonical publication

A reviewed AI artifact may be shared to the public AI layer. Sharing allows other people to browse, expand, compare, or privately adopt the idea. It does not create a canonical event, transfer canonical authorship, or grant the artifact canonical rank, certainty, or permanence.

The action of sharing to the AI Commons must therefore remain distinct from a later action that submits or promotes content into the canonical system.

### 2.5 Canonical promotion requires human responsibility

When canonical writes become available, an eligible human may use private or AI-originated content as the basis of a canonical submission. The human must review, approve, and take responsibility for the actual content entering the canonical event log.

The canonical event must contain the complete approved content required by the protocol. It may preserve AI provenance, but its validity must not depend on the original AI artifact, private draft, webpage snapshot, or simulation record remaining available.

### 2.6 AI artifacts are not inherently permanent

The AI Commons and autonomous simulation layers may generate far more material than the canonical map. Most of this material does not require permanent storage.

Non-canonical AI artifacts may become inactive, lose visibility, be archived, or be deleted according to lifecycle and retention rules. Artifacts that have been privately adopted or used as provenance for canonical submissions may require enough retained information to explain their origin, but this does not require preserving every AI edit, prompt response, or shared revision forever.

The canonical event log is the permanent authority layer. The AI Commons is a living, mutable layer.

### 2.7 Private content requires explicit consent before public projection

AI generations based on private ideas, private documents, private URLs, or other private sources remain private by default. They enter public projections only through an explicit sharing action.

Public projections must be sanitized on the server. The system must not depend on the frontend alone to conceal private source text, account information, source-document passages, internal storage paths, or other protected metadata.

### 2.8 User-facing reasoning content is represented as ideas

Documents, articles, webpages, reports, and other sources may produce ideas, but their technical processing records are not automatically map ideas.

Everything the user is expected to reason about, navigate, connect, rank, adopt, or promote should be represented in idea format. Supporting records such as uploaded bytes, webpage snapshots, normalized text, chunks, offsets, hashes, retrieval records, and extractor versions remain provenance objects behind the ideas.

### 2.9 Projections do not create new authority realms

The same underlying content may appear in several views. An AI artifact may begin privately, later be shared to the AI Commons, appear through a model lens, be adopted by another user, and eventually inform a canonical submission.

These appearances are projections or relationships. They do not require each view to become an independent authoritative database, and they do not change the authority of the underlying content merely by displaying it elsewhere.

## 3. Preferred Terminology

The following vocabulary should be used consistently in private product architecture, implementation planning, user-interface language, and future AI-map documentation.

| Preferred term                     | Meaning                                                                                                                                               |
| ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Canonical Map**                  | The authoritative map derived from the initial Seed and later valid canonical events.                                                                 |
| **Private Map**                    | A user-controlled, non-canonical collection of ideas, connections, rankings, notes, and organizational state.                                         |
| **AI Artifact Substrate**          | The general non-canonical storage and provenance layer underlying AI-generated candidates, analyses, connections, simulations, and shared AI content. |
| **AI Commons**                     | The public, shared, non-canonical layer of AI-generated ideas and connections.                                                                        |
| **Combined Public Map**            | A public projection displaying canonical records and active AI Commons artifacts together while preserving their distinction.                         |
| **Autonomous Simulation Universe** | A non-canonical environment in which AI agents simulate idea creation, challenges, votes, verdicts, cycles, and other canonical-like processes.       |
| **Model Universe**                 | An autonomous simulation universe primarily operated by one model or model family for comparative study.                                              |
| **Model Lens**                     | A filtered view of artifacts generated by a particular model across one or more origins and realms.                                                   |
| **Candidate Idea**                 | A reviewable AI artifact proposing idea-format content before private adoption or public sharing.                                                     |
| **Private Adoption**               | An explicit human action that imports AI-originated content into a human-controlled private map.                                                      |
| **Public AI Sharing**              | An explicit action that exposes a reviewed AI artifact in the AI Commons.                                                                             |
| **Canonical Promotion**            | A later human-authored canonical submission based partly or wholly on private or AI-originated content.                                               |
| **Source-Extracted Idea**          | An idea directly and explicitly supported by a source document or webpage.                                                                            |
| **Source-Inferred Idea**           | An idea strongly implied by a source but not directly stated in the same form.                                                                        |
| **AI-Expanded Idea**               | An idea generated beyond the source using the source or another idea as context.                                                                      |

Several older or broader terms remain usable only in limited contexts.

**Shared Map** is reserved for its canonical or open-core protocol meaning. It should not be used as the public product name for the AI Commons.

**Sandbox** refers to the protocol-level boundary within which AI may simulate or draft without canonical effect. It may describe a safety boundary, but it should not replace the names of the product realms defined here.

**Wilderness** remains a user-interface metaphor for public exploration. It does not define authority, ownership, or persistence.

**Ent** remains a product and narrative concept for AI identities, assistants, or agents. It does not name the entire AI map architecture.

**Agent Map**, **Personal Map**, **Public Map**, and **Model Map** are too ambiguous to serve as primary architecture terms. Existing documents may retain them for historical or subsystem-specific reasons, but new architecture should use the more precise terms in this document.

The words **adoption**, **sharing**, and **promotion** should normally be qualified. “Adoption” should specify whether it means private adoption or canonical human adoption. “Sharing” should specify AI Commons sharing where needed. “Promotion” should be reserved for movement toward canonical submission rather than ordinary public visibility.

The terms **rot**, **burn**, **archive**, and **deletion** must also be qualified by realm. Canonical lifecycle mechanics and AI Commons lifecycle mechanics may use similar metaphors without having identical meanings.

## 4. Realm and Projection Overview

The product contains three primary content realms and several derived projections.

The primary realms are:

1. The Canonical Map
2. Private Maps
3. The AI Artifact Substrate

The Canonical Map contains authoritative event-log-derived state. Private Maps contain user-controlled non-canonical state. The AI Artifact Substrate contains generated candidates, AI analyses, AI-proposed connections, shared AI artifacts, and simulation outputs.

The AI Commons is the public projection of selected active artifacts from the AI Artifact Substrate. It is a public view over non-canonical AI content rather than a second canonical map.

Autonomous Simulation Universes are isolated non-canonical environments built from AI artifacts, agent activity, simulation configuration, and simulation history. A Model Universe is a specialized simulation universe associated primarily with one model or model family.

A Model Lens is not necessarily a storage realm. It is a projection that selects artifacts according to model, model version, task, origin, simulation, visibility, or other provenance fields.

The Combined Public Map is also a projection. It presents canonical records and active AI Commons artifacts in one public browsing experience:

```text
Combined Public Map
= Canonical Map
+ Active AI Commons
```

This combination affects presentation, not authority. Canonical and AI content remain separate in storage, provenance, lifecycle, and legal meaning.

A simplified flow is:

```text
Documents, URLs, private ideas, and canonical ideas
                         ↓
                  AI generation
                         ↓
              Private AI candidates
                    ↙         ↘
          Private adoption   Public AI sharing
                  ↓                 ↓
             Private Map        AI Commons
                    \             /
                     human review
                          ↓
                Canonical promotion
                          ↓
                   Canonical Map
```

Autonomous simulations operate alongside this human-prompted flow:

```text
Canonical Seed or selected snapshot
                    ↓
          Autonomous AI simulation
                    ↓
       Simulation ideas and connections
                    ↓
      Simulation universe / model universe
                    ↓
          public observation and comparison
```

Simulation results may also be viewed through the AI Commons or model lenses where the product deliberately exposes them, but they remain labeled according to their autonomous origin.

One AI artifact may participate in several projections over time. For example, a GPT-generated candidate may begin private, be shared to the AI Commons, appear in a GPT model lens, be adopted by another user into a private map, and later be cited as provenance for a human canonical submission.

The architecture should therefore track origin, ownership, visibility, lifecycle, model provenance, source references, simulation membership, adoption, and promotion relationships. It should not assume that every public view requires a duplicated copy of the full idea map.

The following distinctions are fundamental:

| Concept             |                   Publicly visible |                            Human controlled |                                     Canonical |      Permanent by default |
| ------------------- | ---------------------------------: | ------------------------------------------: | --------------------------------------------: | ------------------------: |
| Canonical idea      |                                Yes | Through canonical authorship and governance |                                           Yes |                       Yes |
| Private idea        |       No, unless separately shared |                                         Yes |                                            No |                        No |
| AI Commons artifact |                                Yes |  Managed by its owner and product lifecycle |                                            No |                        No |
| Simulation artifact | Depending on simulation visibility |      Controlled by simulation configuration |                                            No |                        No |
| Model lens entry    | Depends on the underlying artifact |          Depends on the underlying artifact | No, unless the underlying record is canonical | No independent permanence |

The rest of this document defines the behavior of these realms and transitions in greater detail.

## 5. Canonical Map

The Canonical Map is the permanent, authoritative representation of shared ideas in The Seed in My Mind. It is derived from the canonical event log rather than from the current contents of a private database, AI workspace, or public display cache.

The initial Seed forms the bootstrap state of the Canonical Map. It supplies the first body of canonical ideas, descriptions, and connections from which later human participation can develop. The Seed is not merely example content or an optional starting dataset. It is part of the canonical history that every conformant replay must reproduce.

After launch, the Canonical Map may grow through valid human-authored canonical events. These events must satisfy the protocol’s authorship, eligibility, signature, rate-limit, and validation requirements. The exact requirements are defined by the open-core protocol and its delegated specifications rather than by this document.

AI may assist a human in preparing canonical content. It may draft an idea, suggest descriptions, identify connections, extract ideas from sources, compare alternatives, or simulate challenges. None of those actions independently changes the Canonical Map.

A canonical change occurs only when an eligible human deliberately creates or approves a valid canonical event and assumes responsibility for the content carried by that event. Where AI contributed to the drafting process, appropriate provenance may be preserved, but the human remains the canonical author.

Canonical content has properties that do not apply automatically to private or AI-generated content:

* it participates in canonical replay;
* it may be included in canonical snapshots;
* it may receive canonical ranks, challenges, votes, verdicts, or other protocol-defined state;
* it must remain interpretable independently of private product services;
* its validity cannot depend on an AI provider, private account, hosted database, or source AI artifact remaining available.

The Canonical Map should therefore remain relatively scarce compared with the AI Commons. The AI layer may produce many possible ideas and branches. Canonical history should contain the smaller body of ideas that eligible humans deliberately judge important enough to enter permanent shared history.

Canonical scarcity is not a claim that canonical content is automatically true. Canonical ideas remain challengeable under the protocol. Scarcity reflects the cost and responsibility of writing to permanent shared history, not immunity from disagreement or correction.

## 6. Launch Architecture

The initial public launch should not depend on general canonical writing being available.

Human verification, writer eligibility, abuse resistance, rate limits, challenge procedures, and other canonical participation systems may require additional development and real-world testing. Delaying public launch until all of those systems are complete would leave the canonical Seed static and would prevent users from exploring the intended AI-assisted product experience.

The launch architecture therefore separates canonical reading from canonical writing.

At launch, the Canonical Map should be publicly readable. Users should be able to browse the initial Seed, inspect descriptions and connections, search ideas, and navigate the public graph without being verified canonical writers.

General canonical writing should remain disabled until its separate readiness requirements are satisfied. The user interface must not imply that a public AI action, private edit, or ordinary account action has created canonical state.

At the same time, signed-in users should be able to use AI to generate new material from the Seed. They may expand a canonical idea, generate related truth claims or actionable ideas, create descriptions, extract ideas from documents or URLs, and propose new connections.

These generations should begin as private candidates. After review, users may keep them private, import them into a private map, or share selected results to the AI Commons.

The public launch experience therefore contains three active surfaces:

### Canonical surface

The initial Seed is public and authoritative. Canonical reads are enabled, while general canonical writes remain disabled.

### Private surface

Users may create and manage private ideas, private connections, AI-generated candidates, document-derived ideas, URL-derived ideas, and privately adopted AI Commons content.

### AI Commons surface

Reviewed AI-generated ideas and connections may be shared publicly. They may visibly extend from canonical Seed ideas and may be browsed by everyone, but they remain non-canonical.

The public map experience should support three primary views:

```text
Canonical only
AI Commons only
Combined
```

The combined view should allow users to see how public AI-generated ideas grow around and between canonical Seed ideas. It must not make those ideas appear to have the same authority.

Canonical and AI content must differ through multiple visible signals, such as:

* explicit realm labels;
* icons or symbols;
* node shape, border, or texture;
* provenance panels;
* canonical event information;
* provider and model information;
* origin mode;
* lifecycle state;
* connection authority.

Color may support this distinction but must not be the only signal.

A canonical node should clearly identify itself as part of canonical history. An AI Commons node should clearly identify itself as AI-generated and non-canonical. An AI-proposed connection should not be visually interchangeable with a canonical connection.

The combined map is a combined presentation, not a merged authority system.

The initial launch model can be summarized as:

```text
Canonical
- Initial Seed visible
- Reads enabled
- General writes disabled

Private
- Local or hosted private maps
- Human-written and AI-assisted ideas
- Document ingestion
- URL ingestion when implemented
- Private adoption from AI Commons

AI Commons
- Publicly readable
- Generation requires authentication
- Sharing requires review and explicit action
- Content remains AI-generated and non-canonical

Combined Public Map
- Canonical-only filter
- AI-only filter
- Combined filter
```

This structure allows the public shared experience to grow immediately while preserving the integrity of the future human canonical system.

## 7. Private Maps

A Private Map is a user-controlled, non-canonical workspace for developing ideas without writing to shared canonical history.

Private Maps may contain:

* ideas written directly by the user;
* AI-generated ideas adopted by the user;
* AI-assisted rewrites or descriptions;
* ideas extracted from uploaded documents;
* ideas extracted from webpages or public URLs;
* ideas adopted from the AI Commons;
* private connections involving private or canonical references;
* private rankings, placements, notes, and organizational state.

Private content may be stored locally, hosted privately, or synchronized between local and hosted systems. The storage location does not change its authority. A hosted private idea remains private and non-canonical. A local private idea does not become more authoritative by remaining on the user’s device.

The product may offer different privacy and synchronization modes, including:

* local-only;
* hosted private;
* synchronized local and hosted;
* exported private backup;
* offline private workspace.

Those modes concern availability, privacy, and resilience. They do not create separate idea types or authority levels.

A Private Map gives the user broad control. Subject to technical and legal retention requirements, the user may:

* create;
* edit;
* delete;
* connect;
* disconnect;
* rank;
* organize;
* duplicate;
* merge;
* archive;
* privately expand;
* privately import.

Private Maps are intended to support experimentation. Users should be able to explore uncertain, unfinished, controversial, personal, or low-confidence ideas without those ideas being publicly visible or interpreted as canonical contributions.

AI-generated content in a Private Map should preserve AI provenance, but the user controls the adopted private version. The user may rewrite the title, descriptions, type, or connections after adoption.

That control does not erase the fact that AI contributed to the origin where provenance is relevant. It does mean that the private idea is no longer merely a read-only provider response. It has become part of the user’s private reasoning workspace.

Private ideas may refer to canonical ideas. For example, a user may privately connect a new policy proposal to a canonical truth claim or privately rank several ideas relative to a canonical reference.

These private relationships must not alter canonical connections, ranks, or certainty. The canonical idea serves as a stable public reference, while the private relationship belongs only to the user’s map.

Private importance uses the ten-axis reference-relative meaning (`important_to_reference` and `important_for_reference` across five timeframes), but the owner selects and revises the private order directly. Private ranking does not require a canonical challenge, juror panel, vote, or verdict. A private map has no universal-importance rank: the 20-axis universal product and its derived overall rank exist only in the canonical public process. Sharing, AI assistance, or simulation does not change those boundaries.

Private Maps may also contain content adopted from the AI Commons. Such adoption creates a private record owned by the adopting user. The public artifact remains separate and may later be withdrawn, archived, or deleted according to AI Commons lifecycle rules.

A private adoption should retain enough provenance to identify the public artifact or model output from which it originated. However, the private idea must remain usable if the public artifact later disappears.

Private content enters the public AI layer only through explicit Public AI Sharing. It enters canonical history only through a separate Canonical Promotion process when canonical writing becomes available.

Private adoption, public sharing, and canonical promotion must therefore remain distinct in both the data model and the user interface.

## 8. AI Artifact Substrate

The AI Artifact Substrate is the general non-canonical record layer that supports AI generation, review, provenance, sharing, simulation, and adoption.

It should not be understood as one visible map. It is the underlying system from which several private and public projections may be produced.

The substrate may contain:

* candidate ideas;
* candidate descriptions;
* candidate connections;
* document-extraction results;
* URL-extraction results;
* duplicate findings;
* evidence analyses;
* simulated challenges;
* simulated arguments;
* simulated votes and verdicts;
* model comparison outputs;
* autonomous simulation activity;
* other bounded AI-generated records.

An AI artifact should retain enough structured metadata to explain what it is, where it came from, who may access it, and how it has been used.

Relevant metadata may include:

* artifact identifier;
* task type;
* artifact type;
* provider;
* model and model version;
* prompt-template identifier and version;
* prompt or configuration fingerprint;
* origin mode;
* owning account or identity;
* visibility;
* review state;
* lifecycle state;
* source idea references;
* document or URL source references;
* simulation identifier;
* agent role;
* map-realm memberships;
* sharing state;
* private adoption references;
* canonical-promotion references;
* creation and modification timestamps;
* content fingerprints.

Not every artifact requires every field. A document-extracted candidate may require exact source spans. A simulated vote may require a simulation, agent role, and rulebook version. A connected-idea expansion may require a source idea and relationship rationale.

The substrate should use common provenance and lifecycle concepts without forcing unrelated artifact types into identical payloads.

### Origin and visibility

Origin describes how an artifact was created. Visibility describes who may access it. These must remain separate.

An artifact may have an origin such as:

* human-prompted expansion;
* description generation;
* document extraction;
* URL extraction;
* autonomous simulation;
* model simulation;
* evidence analysis;
* challenge simulation.

It may separately have visibility such as:

* private;
* shared to the AI Commons;
* visible within a simulation;
* archived;
* removed from active views.

A human-prompted artifact does not become public merely because its origin is human-prompted. A simulation artifact does not become private merely because no human authored it. Visibility follows explicit consent and simulation configuration.

### Review and artifact state

Many AI artifacts should begin in a reviewable candidate state.

A typical lifecycle before public or private use is:

```text
Generated
→ Pending review
→ Accepted, rejected, or superseded
→ Privately adopted and/or publicly shared
```

Acceptance indicates that a human has reviewed the artifact for a particular use. It does not make the artifact canonical and does not necessarily create a private idea.

Private adoption, public sharing, and canonical promotion remain separate later actions.

Editing an accepted artifact may require renewed review where the accepted content fingerprint no longer matches. The exact implementation may vary, but the system should not present materially changed content as though the earlier human approval automatically covered it.

### One artifact, multiple projections

The substrate should support a single artifact participating in several projections over time.

For example:

```text
Artifact:
- generated by GPT
- task: expand_idea
- source: canonical Seed idea
- origin: human prompted
- owner: User A
- initial visibility: private
- later shared to AI Commons
- visible in GPT model lens
- adopted privately by User B
- later referenced by User A in a canonical submission
```

The system does not need to create a separate full copy of the artifact for every view. It may use memberships, references, visibility records, and adoption records.

A private adoption does create a distinct private idea because the adopting user controls that content independently. A canonical promotion creates a distinct canonical event because canonical state must remain independent of the AI substrate.

### Relationship to canonical state

The AI Artifact Substrate must remain outside canonical replay.

AI artifacts must not:

* create canonical ideas directly;
* create canonical connections directly;
* affect canonical rank;
* affect canonical certainty;
* cast canonical votes;
* produce canonical verdicts;
* mint canonical protocol value;
* become required inputs to canonical replay.

Canonical records may include provenance referring to AI assistance, but canonical replay must be able to reconstruct valid canonical state without reading the private AI substrate.

### Relationship to public maps

The AI Commons is the public projection of selected artifacts from the substrate. Model Lenses are filtered projections based on provenance. Simulation Universes are projections and histories organized around autonomous agent activity.

The substrate therefore supports multiple views, but those views must preserve the underlying distinctions among:

* ownership;
* visibility;
* origin;
* lifecycle;
* simulation membership;
* canonical versus non-canonical status.

The next sections define the AI Commons, the Combined Public Map, and the actions available to public and authenticated users.

## 9. AI Commons

The AI Commons is the public, shared, non-canonical layer of AI-generated ideas and connections.

Its purpose is to let the visible public map grow beyond the initial Seed before general canonical writing becomes available. It provides a place where users can explore AI-generated extensions, compare alternative framings, discover related ideas, and adopt useful material into private work without confusing those activities with canonical authorship.

The AI Commons is not a second canonical map. Its contents do not enter canonical replay, receive canonical authority, or become permanent merely because they are publicly visible.

An artifact in the AI Commons should remain clearly identifiable as:

* AI-generated;
* non-canonical;
* associated with a model and generation process;
* created through a particular origin mode;
* subject to non-canonical lifecycle and retention rules.

The product should use the term **AI Commons** in user-facing and architecture language. Internal implementation identifiers such as `shared_ai_map` may remain in storage or code, but they should not define the public vocabulary.

The word **Commons** reflects public accessibility and shared exploration. It does not imply common ownership, canonical endorsement, unrestricted reuse, or permanent preservation.

### 9.1 Human-prompted contributions

Most AI Commons content at launch should originate through a human-prompted workflow.

A user may begin from:

* a canonical Seed idea;
* an existing AI Commons idea;
* a private idea;
* an uploaded document;
* a webpage or article;
* another supported source.

The system generates private candidate ideas, descriptions, or connections. The user reviews those candidates and decides which, if any, should be shared publicly.

The basic flow is:

```text
Source
→ AI generation
→ Private candidates
→ Human review
→ Optional AI Commons sharing
```

Generation should not publish automatically.

This review boundary reduces accidental disclosure, malformed output, obvious duplication, unsupported claims, and low-value generations entering the public layer without deliberate human action.

The reviewing user does not become the sole human author of the AI-generated text merely by sharing it. The artifact should remain labeled as AI-generated, while also recording that a human reviewed and chose to share it.

### 9.2 Autonomous contributions

Later, autonomous Simulation Universes may expose selected outputs to the AI Commons without an ordinary human prompt for each artifact.

Those outputs must remain distinguishable from human-prompted AI activity.

The AI Commons should preserve origin modes such as:

* human-prompted;
* document-extracted;
* URL-extracted;
* autonomous simulation;
* model simulation;
* evidence analysis;
* challenge simulation.

A user should be able to tell whether an idea was generated because a person requested an expansion, because an autonomous agent created it during a simulation, or because it was extracted from a source.

Autonomous AI output should not be displayed as though a human reviewed or endorsed it unless such review actually occurred.

### 9.3 Public AI connections

The AI Commons may contain non-canonical connections between:

* an AI idea and a canonical idea;
* two AI ideas;
* an AI idea and a safely exposed source reference;
* simulation ideas within a public simulation projection.

An AI-proposed connection remains non-canonical even when one endpoint is canonical.

For example, an AI-generated idea may appear as relatively important under a canonical Seed idea. That connection expresses an AI-generated proposal in the public non-canonical layer. It does not modify the canonical idea’s official connections or rank.

Connection direction and meaning must remain explicit. The system must not silently reverse a connection because only one direction is convenient to display or store.

AI Commons connections should carry:

* connection type;
* endpoint references;
* model provenance;
* origin mode;
* rationale where available;
* lifecycle state;
* non-canonical status.

### 9.4 Public growth from the Seed

At launch, the initial Seed provides the stable canonical foundation from which public AI exploration can grow.

A user should be able to open a Seed idea, request an AI expansion, review the results, and share selected ideas. Those ideas then become visible to other users as non-canonical branches connected to the canonical source.

This allows the public map to develop immediately without pretending that general canonical participation is ready.

The resulting structure may resemble:

```text
Canonical Seed idea
├── canonical connection
├── canonical related idea
├── AI Commons idea
│   ├── AI Commons connection
│   └── further AI Commons expansion
└── another AI Commons idea
```

The presence of AI branches around a canonical idea must not imply that the canonical system has accepted those branches.

The graph should make the distinction visible at every level.

### 9.5 Ownership and management

A shared AI artifact retains its underlying ownership and provenance.

The user who shares it may be allowed to:

* withdraw it from active public views;
* manage its public visibility;
* review its current lifecycle state;
* see who has privately adopted it where privacy rules permit aggregate information;
* create a corrected or superseding artifact.

Sharing does not transfer ownership to the public or to another user.

Another user may privately adopt the artifact, but that creates a separate private idea controlled by the adopting user. It does not change ownership of the source artifact.

The product may also apply moderation, abuse prevention, legal removal, lifecycle, or storage policies independently of owner actions.

### 9.6 Mutable and lifecycle-controlled content

AI Commons content is expected to be abundant and mutable.

Artifacts may:

* remain active;
* cool as activity declines;
* rot out of default views;
* be burned from the active map;
* move into archive storage;
* be deleted when no retention requirement remains.

The AI Commons should therefore not be designed as an immutable permanent publication ledger.

Some limited revision, supersession, withdrawal, or provenance records may still be useful, particularly where another user adopted an artifact or where the artifact later informed a canonical submission. Those records support traceability without requiring every generated variant to remain permanently visible.

The detailed lifecycle model is defined later in this document.

## 10. Combined Public Map

The Combined Public Map is the public projection that presents the Canonical Map and the active AI Commons in one navigable experience.

It is defined conceptually as:

```text
Combined Public Map
= Canonical Map
+ Active AI Commons
```

The Combined Public Map does not merge the two layers into one authority system. It allows users to explore them together while preserving their different origins, durability, and meaning.

### 10.1 Required views

The public interface should support at least three primary views:

#### Canonical only

Shows the initial Seed and later canonical records.

This view excludes AI Commons artifacts and presents only event-log-derived canonical ideas and connections.

#### AI Commons only

Shows active public AI-generated ideas and connections.

Canonical ideas may still appear as limited reference anchors where necessary to explain AI connections, but they should not be presented as AI artifacts.

#### Combined

Shows canonical and AI Commons content together.

This is the primary exploratory view for understanding how public AI-generated material extends, challenges, or connects to canonical ideas.

These filters should change presentation without rewriting or copying the underlying records.

### 10.2 Visible realm distinction

Canonical and AI content must be distinguishable without opening a detail panel.

The interface should use several signals together, including:

* explicit labels;
* icons;
* node shapes;
* border styles;
* textures or patterns;
* provenance badges;
* connection styles;
* lifecycle indicators;
* model information.

Color may reinforce these differences but must not be the only distinction.

A canonical idea might show:

* `Canonical`;
* canonical event or Seed provenance;
* human authorship;
* immutable-history indicator;
* canonical connection treatment.

An AI Commons idea might show:

* `AI-generated`;
* `Non-canonical`;
* model and provider;
* human-prompted or autonomous origin;
* lifecycle state;
* sharing provenance.

The interface should not use language such as “approved,” “verified,” or “accepted” for an AI artifact unless the meaning is clearly limited to human review for sharing rather than canonical validation.

### 10.3 Connection distinction

Connections also require realm labeling.

The public graph may contain:

* canonical connections;
* AI-proposed connections;
* source-derivation relationships;
* simulation relationships.

These should not appear interchangeable.

A canonical connection is part of event-log-derived canonical state.

An AI-proposed connection is an AI artifact that may relate canonical or AI endpoints but remains non-canonical.

A source-derivation relationship indicates that an idea came from a document, URL, or other source. It does not necessarily make a semantic claim such as causation, evidence, or importance.

The interface should expose enough information to distinguish these meanings.

### 10.4 Filtering and inspection

The Combined Public Map should eventually support filters including:

* map realm;
* model;
* model version;
* origin mode;
* task type;
* source type;
* simulation universe;
* lifecycle state;
* idea type;
* connection type;
* privately adopted by the current user;
* later promoted to canon.

Not all filters need to exist at launch. The architecture should nevertheless preserve the metadata required to add them later.

Selecting a node should open a detail view that explains:

* whether the record is canonical or AI-generated;
* who or what created it;
* its model and task provenance where applicable;
* its source idea or source material;
* its lifecycle state;
* whether it has been privately adopted;
* whether a later canonical idea was derived from it.

### 10.5 Public search

Public search may return canonical and AI Commons results together, but every result must retain its realm label.

Search ranking must not imply truth or canonical importance.

The system may rank search results according to textual relevance, recency, explicit user filters, or other bounded retrieval criteria. It should not imply that the first AI result is more true, important, or canonically accepted than lower results.

Search should allow users to limit results to:

* canonical records;
* AI Commons records;
* both.

Withdrawn, burned, archived, or deleted artifacts should follow the lifecycle visibility rules defined later.

### 10.6 No authority inheritance

An AI Commons idea does not gain canonical authority by connecting to a canonical node.

A canonical idea does not become AI-generated merely because it appears in the combined graph.

A private adoption does not become public merely because the source artifact is public.

The Combined Public Map is therefore best understood as a joined read projection over records that continue to belong to distinct realms.

## 11. Public Read and Authenticated Actions

The launch architecture should distinguish between actions available to any visitor and actions requiring an authenticated account.

Public visibility should be broad enough to make the Seed and AI Commons useful as a shared reasoning environment. Mutation, generation, ownership, and private adoption require identity, rate limits, and abuse controls.

### 11.1 Publicly readable surfaces

Without signing in, a visitor should be able to:

* browse the Canonical Map;
* browse active AI Commons ideas;
* use the Combined Public Map;
* search public ideas and connections;
* switch between canonical-only, AI-only, and combined views;
* inspect safe provenance;
* see model and origin information;
* follow public source citations;
* inspect simulation outputs marked for public viewing.

Public reads must not expose:

* private ideas;
* private source text;
* private document contents;
* private URL snapshots;
* account identifiers not intended for public display;
* raw provider prompts or responses;
* API credentials;
* internal storage paths;
* moderation or security metadata.

Public read endpoints should return sanitized public projections rather than exposing private artifact records directly.

### 11.2 Authenticated generation

Generating new AI content should require authentication.

Authentication supports:

* provider-cost accounting;
* generation quotas;
* abuse prevention;
* ownership;
* review state;
* private storage;
* later sharing management;
* provenance.

A signed-in user may request AI generation from eligible sources, including:

* a canonical idea;
* an AI Commons idea;
* an owner-controlled private idea;
* an uploaded document;
* an allowed URL;
* another supported private source.

The source text sent to the configured provider must be disclosed to the user where it involves private content.

Generation results should begin private unless they occur inside an explicitly public autonomous simulation.

### 11.3 Authenticated review and sharing

A signed-in user should be able to:

* inspect generated candidates;
* edit allowed fields;
* accept;
* reject;
* supersede;
* resolve duplicates;
* import privately;
* share selected artifacts to the AI Commons;
* withdraw owned artifacts from active public views.

Sharing should require an explicit action after review.

The interface should display what will become public, including:

* idea text;
* model provenance;
* source citations;
* safe relationship information;
* public identity or pseudonymous attribution where applicable.

Private source fields that will not be exposed should also be clearly distinguished.

### 11.4 Authenticated private adoption

A signed-in user may adopt a shared AI Commons idea into a Private Map.

Private adoption should:

* create a separate private idea;
* preserve AI and source provenance;
* assign control to the adopting user;
* remain non-canonical;
* continue working if the public artifact is later withdrawn or deleted.

Adoption should not automatically copy every public connection. The user should decide which connections to import, subject to the private connection model.

### 11.5 Canonical actions remain separately gated

Authentication alone must not enable canonical writing.

Canonical actions may later require:

* verified human identity;
* writer eligibility;
* rate-limit capacity;
* signing authority;
* protocol-valid payloads;
* other Stage 1 requirements.

At initial launch, ordinary users should not see public AI generation or AI Commons sharing presented as a substitute for canonical writing.

The interface may explain that selected private or AI-originated ideas could later be promoted to canonical submissions once the human canonical system becomes available.

### 11.6 Public simulations

Autonomous simulations may be publicly viewable without ordinary account authentication where their configuration permits it.

Starting, configuring, funding, or controlling a simulation should require appropriate authenticated permissions.

Observers should be able to distinguish:

* live simulation activity;
* frozen historical simulations;
* model universes;
* human-prompted AI Commons activity;
* canonical state.

Simulation actions remain non-canonical regardless of how many people observe them.

### 11.7 Rate limits and moderation

Public browsing and authenticated generation have different operational risks.

Public reads may require:

* request limits;
* caching;
* pagination;
* search limits;
* scraping protection.

Authenticated generation may require:

* AI compute quotas;
* payment or subscription limits;
* provider-cost budgets;
* per-task limits;
* content moderation;
* sharing restrictions;
* abuse detection.

These product controls must not be confused with canonical truth, rank, voting power, or writer authority.

The distinction between AI compute limits and canonical participation limits is defined later in this document.

## 12. Creating Ideas From Ideas, Documents, and URLs

The product should use one general idea-generation model across different source types.

A user may begin from:

* a canonical idea;
* a private idea;
* an AI Commons idea;
* an uploaded document;
* a webpage;
* an article;
* a public PDF;
* another supported source.

The source changes how provenance is recorded, but the user-facing result should remain idea-format content.

When the source is already an idea, AI may propose:

* related truth claims;
* related concepts;
* actionable ideas;
* actions;
* descriptions;
* connections;
* alternative framings;
* objections;
* consequences;
* supporting ideas.

When the source is a document or URL, the system should first convert the source into an idea cluster rather than exposing technical chunks as the primary result.

The cluster should normally contain:

1. A root idea representing the source as a whole.
2. Important ideas explicitly stated in the source.
3. Important ideas strongly implied by the source.
4. Connections among the extracted ideas.
5. Optional AI-generated ideas extending beyond the source.

The same review, private-adoption, and public-sharing boundaries should apply regardless of whether the source was a canonical idea, private idea, file, or URL.

### 12.1 Root source ideas

A substantial document, article, report, or webpage should normally produce one root idea representing its central meaning.

The root idea should not merely repeat the source title. It should express the main claim, proposal, concept, or subject in ordinary idea form.

For an argumentative article, the root may be a truth claim or actionable idea. For an explanatory report, it may be a conceptual idea. For an instruction or plan, it may be an actionable idea.

The root idea may contain:

* a concise title;
* a sentence description;
* a paragraph summary;
* a fuller representation of the source where useful;
* source provenance;
* links to extracted sub-ideas.

The source itself remains supporting provenance. The root idea is the user-facing representation of what the source is about.

### 12.2 Important source ideas

The system should extract ideas that are meaningful enough to reason about independently.

These may include:

* central truth claims;
* definitions;
* concepts;
* recommendations;
* actions;
* assumptions;
* conclusions;
* trade-offs;
* objections;
* evidence statements;
* causal claims.

The system should avoid producing an idea for every sentence. Extraction should focus on distinct conceptual units that can be understood, connected, challenged, ranked, or adopted.

Each extracted idea should be intelligible outside the source while retaining enough provenance to show where it came from.

### 12.3 Source-extracted, source-inferred, and AI-expanded

The product must distinguish three classes of output.

#### Source-extracted

A Source-Extracted Idea is directly and explicitly supported by the source text.

Its provenance should identify the exact source passages supporting it. The idea may paraphrase the source, but it should not introduce a materially new claim.

#### Source-inferred

A Source-Inferred Idea is strongly implied by the source but not directly stated in the same form.

The system should explain that the idea is an interpretation or inference. It should not present an inference as a quotation or direct source claim.

#### AI-expanded

An AI-Expanded Idea goes beyond the source.

It may identify a consequence, related concept, alternative, objection, application, or extension that the source did not itself express.

The source may provide context, but the system must not imply that the source author stated or endorsed the expansion.

These classifications should remain visible in provenance and available as filters.

### 12.4 Connections within source clusters

Ideas extracted from one source may be connected to the root idea and to one another.

A source-derivation relationship indicates that an idea was extracted or inferred from a source. It is different from a semantic connection such as evidence-for, evidence-against, same-as, causation, or relative importance.

The fact that two ideas appear in the same article does not prove that one supports the other or that either is important relative to the other.

Semantic connections should therefore be generated and reviewed separately from source membership.

Possible relationships include:

* extracted from source;
* inferred from source;
* evidence for;
* evidence against;
* relative importance;
* same-as;
* causal relationship;
* prerequisite;
* consequence;
* action based on claim.

Where the current connection model does not support a proposed relationship safely, the proposal should remain a reviewable AI artifact rather than being forced into an inaccurate connection type.

### 12.5 Technical provenance records

The system still requires technical records for safe processing and traceability.

These may include:

* original uploaded bytes;
* submitted URL;
* resolved URL;
* stored webpage snapshot;
* extracted text;
* normalized text;
* chunks;
* source offsets;
* content hashes;
* retrieval time;
* extractor version;
* normalization profile;
* page or structural locators.

These records are supporting provenance objects.

They should not normally appear as ordinary idea nodes in the map. Users reason about the article-level idea, extracted claims, concepts, and connections rather than about chunk identifiers or content hashes.

The governing principle is:

> Everything users reason about and navigate is represented in idea format. Technical records needed to establish provenance remain supporting objects behind those ideas.

### 12.6 URL ingestion

URL ingestion should be implemented as a sibling of document ingestion.

The intended flow is:

```text
URL
→ safe retrieval
→ private source snapshot
→ extraction and normalization
→ chunking
→ root article idea
→ extracted and inferred ideas
→ optional AI expansion
→ human review
→ private adoption and/or public sharing
```

Sources and candidates should begin private.

The system may later support public sharing of reviewed URL-derived ideas, but it should not automatically publish the stored source snapshot or complete extracted page text.

Public URL-derived ideas may expose safe provenance such as:

* original public URL;
* resolved URL;
* page title;
* publisher or domain;
* retrieval date;
* bounded source excerpts;
* source fingerprint.

The system must not bypass login requirements, paywalls, access controls, or private network boundaries.

Private, authenticated, local, intranet, temporary signed, or account-specific URLs should remain private. A public citation may be exposed only where doing so is safe and deliberate.

## 13. Private Adoption, Public AI Sharing, and Canonical Promotion

Private Adoption, Public AI Sharing, and Canonical Promotion are three distinct transitions.

They must remain separate in terminology, user interface, provenance, and data behavior.

| Action              | Result                                             | Visibility | Authority     |
| ------------------- | -------------------------------------------------- | ---------- | ------------- |
| Private Adoption    | Creates or updates a human-controlled private idea | Private    | Non-canonical |
| Public AI Sharing   | Exposes a reviewed AI artifact in the AI Commons   | Public     | Non-canonical |
| Canonical Promotion | Creates a valid human-authored canonical event     | Public     | Canonical     |

### 13.1 Private Adoption

Private Adoption occurs when a human explicitly accepts AI-originated content into a Private Map.

The resulting private idea belongs to the adopting user. The user may edit, connect, rank, organize, archive, or delete it according to private-map rules.

The private idea should preserve appropriate provenance, including:

* source AI artifact;
* model and provider;
* task type;
* source idea, document, or URL;
* adoption time;
* adopting identity where applicable.

The private idea must remain usable if the source AI artifact is later withdrawn, archived, or deleted.

Private Adoption does not imply that the adopting human independently wrote every word. It means the human chose to take control of the content in a private workspace.

Preferred user-interface language includes:

* `Save privately`
* `Import privately`
* `Adopt into private map`

### 13.2 Public AI Sharing

Public AI Sharing occurs when a user explicitly exposes a reviewed AI artifact in the AI Commons.

The artifact remains:

* AI-generated;
* non-canonical;
* associated with its original provider and model;
* subject to AI Commons lifecycle rules.

Sharing does not transfer ownership to the public. It does not convert the artifact into a human-authored canonical idea.

The user should be shown what will become public before confirming the action.

Publicly exposed fields may include:

* idea content;
* model and task provenance;
* origin mode;
* safe source citations;
* relationship rationale;
* public identity or pseudonym where applicable.

Private source text, private document passages, internal account identifiers, raw prompts, raw provider responses, and internal storage details must be removed or withheld server-side.

Other users may browse, expand, or privately adopt the shared artifact.

### 13.3 Cross-user adoption

When another user adopts a shared AI Commons artifact, the system creates a separate private idea for the adopter.

The new private idea belongs to the adopter. Ownership of the public source artifact does not transfer.

Cross-user adoption should preserve enough provenance to identify the public artifact and its AI origin.

If the original artifact is later unshared, burned, archived, or deleted, the adopted private idea remains. It must not be silently changed or removed.

Later edits or replacements of the public artifact should not automatically overwrite private copies. Updating an adopted idea should require a new explicit human decision.

### 13.4 Canonical Promotion

Canonical Promotion occurs only when an eligible human creates a valid canonical submission based on private or AI-originated content.

The human may:

* accept the content largely as generated;
* rewrite it;
* combine several private or AI ideas;
* add qualifications;
* change its type;
* alter its descriptions;
* select different connections.

The resulting canonical event must contain the complete human-approved content required by the protocol.

The human becomes the canonical author and accepts responsibility for the submission.

AI provenance may be preserved where useful or required, but the canonical record must remain valid independently of:

* the source AI artifact;
* the original prompt;
* a hosted provider;
* the user’s private map;
* a document snapshot;
* an AI Commons record.

Public AI Sharing is not Canonical Promotion. Private Adoption is not Canonical Promotion.

The interface must not use one action or label for all three processes.

## 14. Autonomous Simulation Universes and Model Comparison

The product should support autonomous AI simulations of the human canonical system.

These simulations allow observers to see what may happen when AI agents create ideas, challenge claims, form arguments, vote, issue simulated verdicts, and participate in simulated cycles without humans exercising canonical authority.

An Autonomous Simulation Universe is a non-canonical experiment.

It may include:

* AI-created ideas;
* AI-created connections;
* advocate agents;
* critic agents;
* juror agents;
* simulated identity roles;
* simulated challenges;
* simulated evidence analysis;
* simulated votes;
* simulated verdicts;
* simulated cycles;
* simulated ranking;
* simulated lifecycle activity.

Humans may observe, configure, fund, pause, compare, or archive simulations, but simulation activity does not create canonical state.

### 14.1 Simulation configuration

Each simulation should identify enough information to explain its behavior.

Relevant configuration may include:

* simulation identifier;
* starting Seed or snapshot;
* starting map realm;
* model set;
* model versions;
* agent roles;
* rulebook version;
* prompt or policy configuration;
* compute budget;
* cycle settings;
* start time;
* current status.

This metadata should allow observers to distinguish changes caused by different models, rulebooks, prompts, or starting conditions.

### 14.2 Simulation history

A simulation may maintain an append-only event history while it is active or retained.

This history supports:

* replay;
* observation;
* debugging;
* comparison;
* explanation of how the simulated map developed.

Simulation history is not the canonical event log.

It may later be:

* frozen;
* compressed;
* archived;
* summarized;
* deleted according to retention policy.

The product must not describe simulation votes, ranks, verdicts, or cycles as canonical.

### 14.3 Model Universes

A Model Universe is an Autonomous Simulation Universe primarily operated by one model or model family.

Examples may include:

* a GPT-based universe;
* a Claude-based universe;
* a Gemini-based universe;
* a local-model universe;
* a mixed-model universe.

Model Universes allow controlled comparison.

Given similar starting Seeds and rule configurations, the product may compare:

* ideas generated;
* connections proposed;
* challenges raised;
* priorities selected;
* convergence;
* divergence;
* simulated verdicts;
* long-term map structure.

A Model Universe has its own simulation identity and history. It is not merely a filter over unrelated artifacts.

### 14.4 Model Lenses

A Model Lens is a filtered view of artifacts generated by a particular model across different origins.

A model lens may include:

* human-prompted expansions;
* description generation;
* document extraction;
* URL extraction;
* evidence analysis;
* challenge simulation;
* autonomous simulation output;
* shared AI Commons artifacts.

A Model Lens does not create a new authority realm.

It selects records based on provenance such as:

* provider;
* model;
* model version;
* task type;
* origin mode;
* date;
* simulation;
* lifecycle state;
* visibility.

Private AI output may appear in the owner’s private model history. It must not enter a public Model Lens unless it was explicitly shared or created in a public simulation.

The distinction is:

> A Model Universe is an autonomous simulation environment. A Model Lens is a filtered view of model-generated artifacts.

## 15. AI Commons Lifecycle and Retention

The AI Commons is intended to generate and expose much more material than the Canonical Map.

It must therefore support lifecycle and retention rather than treating every AI artifact as permanently active.

The lifecycle model should allow the public AI layer to remain useful, searchable, and economically manageable as it grows.

### 15.1 Active

An Active artifact is part of the ordinary AI Commons experience.

It is normally:

* visible by default;
* searchable;
* expandable;
* available for private adoption;
* eligible for further AI activity.

Active status does not imply quality, truth, or importance. It means only that the artifact remains part of the current public AI layer.

### 15.2 Cooling

A Cooling artifact remains public and searchable but receives less default prominence.

Cooling may indicate declining recent activity, reduced relevance, or the beginning of a lifecycle transition.

Cooling should not be interpreted as a negative truth judgment.

### 15.3 Rotting

A Rotting artifact is excluded from normal default views but remains inspectable through search, direct reference, provenance, or lifecycle filters.

It should not normally receive automatic expansion or prominent placement.

Rotting provides a reversible way to reduce clutter without immediately deleting content.

### 15.4 Burned

A Burned artifact is removed from the active AI Commons.

It may remain as:

* an archived record;
* a limited provenance record;
* a tombstone;
* a retained artifact where adoption or promotion references require it.

Burning is an AI Commons lifecycle action. It must not silently inherit the meaning of any canonical, governance, token, or tribe burn mechanism.

### 15.5 Archived

An Archived artifact is retained outside the active public experience.

Archival may support:

* provenance;
* historical inspection;
* debugging;
* research;
* simulation preservation;
* adoption history;
* canonical-promotion history.

Archived content may be inaccessible through ordinary browsing while remaining available through authorized or historical interfaces.

### 15.6 Deleted

A Deleted artifact has had its content payload removed where retention rules permit.

The system may retain a minimal tombstone containing information such as:

* artifact identifier;
* content fingerprint;
* deletion status;
* deletion time;
* reason category;
* whether adoption or canonical references exist.

Hard deletion should be permitted for low-value, duplicate, unused, and unreferenced AI output when legal and product requirements allow it.

### 15.7 Lifecycle inputs

Exact formulas are deferred, but lifecycle decisions may consider:

* age;
* recent human views;
* explicit saves;
* private adoptions;
* continued expansion;
* duplicate status;
* moderation state;
* source availability;
* storage cost;
* simulation relevance;
* canonical-promotion references;
* owner withdrawal.

These signals must not be interpreted as measures of truth.

Popularity, adoption, or activity may affect visibility and retention. They must not determine canonical certainty or universal importance.

### 15.8 Retention principles

The following principles apply:

* Not every AI artifact requires permanent retention.
* Duplicate, abandoned, or unused output may be deleted.
* Private adoptions must retain enough provenance to explain their origin.
* Canonical events must remain valid without the source artifact.
* Simulation histories may use separate retention rules.
* Public artifacts may be withdrawn or removed from active views.
* Hard deletion must not silently break required references.
* Permanent immutable revision history is not required for every artifact.

Limited revision or supersession records may be retained when needed, particularly where another user adopted an artifact or where a human used it in a canonical submission.

The AI Commons remains a mutable public reasoning layer rather than a permanent publication ledger.

## 16. Visibility, Consent, and Public Safety

Private generation should remain private by default.

This applies to generation based on:

* private ideas;
* private documents;
* private URLs;
* hosted private maps;
* local private maps;
* personal notes;
* other owner-controlled sources.

An artifact enters the AI Commons only through explicit sharing or through the declared rules of a public autonomous simulation.

### 16.1 Explicit consent

The sharing interface should show:

* the idea content being shared;
* source citations being exposed;
* model and origin provenance;
* public identity or pseudonym;
* hidden fields that will remain private.

The user should not have to infer whether clicking an ordinary save or adoption button will publish content.

Private Adoption and Public AI Sharing must remain separate actions.

### 16.2 Server-side sanitization

Public AI projections must be sanitized on the server.

They must not expose:

* private source descriptions;
* private document text beyond deliberately shared excerpts;
* private URL snapshots;
* private account identifiers;
* internal storage paths;
* API credentials;
* raw prompts;
* raw provider responses;
* internal moderation metadata.

The frontend may further limit display, but privacy cannot depend on frontend hiding alone.

### 16.3 Source safety

Public webpage-derived ideas may show:

* public URL;
* publisher or domain;
* title;
* retrieval date;
* bounded citations;
* source fingerprint.

The system should not republish the complete article text by default.

Paywalled, authenticated, local, private-network, intranet, account-specific, or temporary signed sources should remain private unless a separate safe public citation is deliberately supplied.

The product must not bypass access controls.

### 16.4 Copyright and source representation

Idea extraction should summarize and transform source content into idea-format records.

The system should avoid exposing long source passages where a short citation or paraphrase is sufficient.

The public AI Commons should not become a repository of copied articles, books, reports, or other source documents.

Source-extracted ideas should point back to their origin while remaining independently phrased and useful.

### 16.5 Moderation and abuse

Public AI sharing may require controls for:

* spam;
* automated flooding;
* malicious links;
* unsafe content;
* impersonation;
* copyright complaints;
* repeated duplicates;
* provider abuse;
* account evasion.

Moderation affects public availability. It does not establish canonical truth or falsity.

Detailed moderation policy remains outside this document.

## 17. Compute Limits and Canonical Scarcity

Hosted AI generation consumes external or local computational resources.

The product may therefore use controls such as:

* AI compute credits;
* generation quotas;
* subscription allowances;
* provider-cost budgets;
* per-task limits;
* daily or monthly usage limits.

These mechanisms govern access to AI computation.

They must remain conceptually separate from:

* POD;
* POINT;
* mana;
* canonical writer eligibility;
* canonical participation limits;
* canonical voting;
* canonical rank;
* canonical importance.

AI generation may be relatively abundant. Canonical writing remains scarce because it enters permanent authoritative history and requires human responsibility.

A user may have enough AI compute credits to generate thousands of candidates while having no authority to create a canonical event.

Conversely, a verified canonical writer may have canonical write capacity without receiving unlimited hosted AI generation.

The systems may later interact through product policy, but they should not be conflated by default.

Exact pricing, subscriptions, free allowances, provider pass-through, and compute budgets remain deferred product decisions.

## 18. Implementation Direction

The product should implement these realms through clear boundaries and projections rather than by duplicating the complete map into separate independent systems.

The preferred conceptual model is:

```text
Canonical records
+
Private records
+
AI artifact substrate
+
simulation records
↓
realm-aware projections
```

The Combined Public Map should be produced from canonical records plus active AI Commons artifacts.

A Model Lens should filter records by model provenance rather than copy them into a new authoritative map.

A Private Adoption should create a distinct private idea because the adopter gains independent control over the content.

A Canonical Promotion should create a distinct canonical event because canonical validity must remain independent from private and AI storage.

Implementation should preserve:

* realm identity;
* ownership;
* visibility;
* origin mode;
* lifecycle state;
* model provenance;
* source provenance;
* simulation membership;
* adoption references;
* promotion references.

The public-read layer should remain separate from authenticated generation and mutation.

Private source sanitization must occur before data enters a public projection.

AI lifecycle state must remain outside canonical replay.

Canonical services must never require the AI artifact substrate to reconstruct canonical state.

The architecture does not require every map view to use identical storage. It does require consistent semantics when records move between or appear across projections.

## 19. Deferred Decisions

This document establishes the architecture but does not settle every implementation detail.

The following remain deferred:

* exact AI Commons lifecycle thresholds;
* exact cooling, rot, and burn algorithms;
* retention durations;
* hard-deletion criteria;
* moderation and abuse workflows;
* public search and graph limits;
* exact URL retrieval implementation;
* browser rendering or JavaScript extraction support;
* source snapshot retention periods;
* historical shared-artifact revision visibility;
* AI compute pricing;
* free and paid quotas;
* autonomous simulation budgets;
* simulation retention periods;
* detailed agent roles;
* exact visual styles for realm distinctions;
* canonical-promotion interface;
* canonical-write launch date;
* whether every public AI artifact requires prior human review;
* how fully autonomous public AI Commons contributions are rate-limited.

Future implementation documents may resolve these issues, but they must remain consistent with the authority and realm distinctions defined here.

## 20. Architectural Compliance

A future feature complies with this architecture when it satisfies the following conditions.

It must identify which realm or projection it belongs to.

It must preserve the distinction between canonical, private, public AI, and simulation content.

It must not allow AI alone to create canonical effects.

It must preserve sufficient provenance for its purpose.

It must respect private visibility and require explicit consent before public sharing.

It must distinguish Private Adoption, Public AI Sharing, and Canonical Promotion.

It must not assume that every AI artifact requires permanent immutable retention.

It must not confuse a Model Lens with a Model Universe or a separate authority layer.

It must represent user-facing reasoning content in idea format while keeping technical source records as supporting provenance.

It must ensure that canonical state remains valid and replayable without relying on private product databases, AI providers, simulation histories, or AI Commons artifacts.

## 21. Relationship to Existing Documents

This document supplies the current private-product architecture for AI map realms, projections, and lifecycle.

The open-core protocol and delegated specifications remain authoritative for:

* canonical events;
* replay;
* snapshots;
* human authorship;
* AI non-authority;
* node conformance;
* canonical tokens;
* governance;
* publication eligibility.

Private AI implementation documents remain authoritative for their implemented subsystem behavior.

The private document-ingestion specification defines storage, extraction, normalization, chunking, and source-provenance behavior for uploaded files.

The document candidate-extraction specification defines current document-to-candidate generation, exact source spans, review, and private import.

The connected-idea expansion and shared-artifact specification defines the current implemented private expansion and authenticated sharing foundation.

Those documents should be interpreted through the realm vocabulary defined here.

Older documents using terms such as:

* Agent Map;
* AI Mirror Map;
* Velocity Map;
* Shared AI Map;
* Personal Map;
* Sandbox Map;
* Model Map;

may remain useful for historical, narrative, experimental, or subsystem-specific purposes.

They should not override the preferred current terms:

* Canonical Map;
* Private Map;
* AI Artifact Substrate;
* AI Commons;
* Combined Public Map;
* Autonomous Simulation Universe;
* Model Universe;
* Model Lens.

Where an older document describes a future feature not yet implemented, its design should be reconciled with this architecture before implementation.

## 22. Summary

The Seed in My Mind uses one permanent canonical authority layer and several non-canonical spaces for human and AI exploration.

The Canonical Map begins with the Seed and later grows through scarce, eligible human-authored events.

Private Maps allow humans to develop, organize, and refine ideas without public or canonical effects.

The AI Artifact Substrate supports AI-generated candidates, extractions, analyses, connections, simulations, and provenance.

The AI Commons exposes selected active AI artifacts publicly so the visible map can grow before canonical writes become available.

Autonomous Simulation Universes allow AI agents to imitate the canonical process without gaining authority.

Model Universes support controlled model-specific simulations, while Model Lenses provide filtered views across model-generated artifacts.

Documents and URLs produce idea-format clusters, while technical source records remain supporting provenance.

Private Adoption, Public AI Sharing, and Canonical Promotion remain separate human actions.

The AI Commons is mutable and lifecycle-controlled. It may cool, rot, burn, archive, or delete artifacts. It is not a second immutable event log.

The Combined Public Map allows canonical and AI-generated ideas to be explored together while preserving clear distinctions in authority, provenance, appearance, and lifecycle.

The result is a system in which AI can generate abundantly, humans can explore privately and publicly, and only deliberately selected human-authored content enters permanent canonical history.
