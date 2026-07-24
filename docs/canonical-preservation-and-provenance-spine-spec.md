## Canonical Preservation & Provenance Spine Specification [anchor: canonical_preservation_provenance_spine_specification]

---

## 0. Purpose, scope, and framing [anchor: 0_purpose_scope_and_framing]

### 0.1 Purpose [anchor: purpose]

This specification defines the **Canonical Preservation & Provenance Spine** of the system. Its purpose is to ensure that the ideas (text payloads) and the deliberative history (canonical event log) are:

* immutable and tamper-evident,
* verifiable by any independent observer,
* widely distributable and preservable by humans,
* legible and explainable at a human level,
* survivable under partial failure, censorship, or isolation,
* replayable locally without reliance on centralized services.

The spine exists to preserve *what was said*, *when it was said*, *how it was deliberated*, and *why it mattered*, over long time horizons, while remaining fully compatible with the protocol's human-first authorship and deterministic replay invariants.

### 0.2 Scope [anchor: scope]

This specification defines:

* deterministic packaging of canonical events into hash-chained blocks for integrity and transport,
* immutable addressing of text payloads via content hashing,
* the relationship between blocks, cycles, snapshots, and derived state,
* the foundations for payload availability, replication, and offline portability,
* the conceptual framing required to expose mechanical processes as legible, inspectable artifacts.

This specification does **not** define:

* how truth, importance, or governance decisions are made,
* voting rules, challenge mechanics, or rulebooks,
* economic settlement or token transfer semantics,
* identity verification or eligibility logic,
* user interface or game mechanics beyond normative requirements.

Those concerns are handled by other specifications and are referenced normatively where required.

### 0.3 Framing and terminology [anchor: framing_and_terminology]

This document deliberately avoids traditional “blockchain” terminology that implies proposer selection, consensus authority, or semantic finality. The mechanisms defined here form a **meaning-preserving integrity spine**, not a consensus ledger.

Throughout this document:

* “Canonical” refers to data that is part of the authoritative event log or deterministically derived from it.
* “Derived” refers to data that is computed from the canonical log according to protocol rules and may be stored for efficiency or legibility.
* “Integrity” refers to the ability to detect tampering or inconsistency.
* “Availability” refers to the ability to obtain payload bytes, not merely verify their existence.

### 0.4 Explicit non-goals [anchor: explicit_non_goals]

This spine does **not**:

* decide truth, importance, or correctness,
* create authority through blocks, hashes, or anchoring,
* elect leaders or order events by competition,
* replace cycles as the system's temporal boundary,
* function as a financial ledger or settlement chain,
* require or assume external blockchains.

External anchoring and settlement chains may be referenced by this system, but they are strictly additive and never authoritative.

---

## 1. Position in the overall system architecture [anchor: 1_position_in_the_overall_system_architecture]

### 1.1 Relationship to Protocol v5 [anchor: relationship_to_protocol_v5]

Protocol v5 defines the system's core invariants:

* a single ordered canonical event log,
* human-first authorship of canonical events,
* deterministic replay to derive all state,
* cycles as the sole temporal boundary,
* corrections as new events rather than edits.

This spine operates **entirely downstream** of those invariants.

Nothing defined here alters event validity, ordering, or meaning. All structures introduced by this specification are either:

* direct views of the canonical log, or
* deterministic transformations of it.

### 1.2 Relationship to the canonical event log [anchor: relationship_to_the_canonical_event_log]

The canonical event log is the sole source of truth.

This spine does not introduce new event types. Instead, it defines how existing canonical events are:

* grouped deterministically,
* hashed for integrity proofs,
* packaged for transport and replication,
* referenced by higher-level provenance artifacts.

Any conformant node, given the same canonical event log, MUST derive identical spine artifacts.

### 1.3 Relationship to cycles [anchor: relationship_to_cycles]

Cycles remain the only semantic and temporal boundary in the system.

This spine respects cycles but does not redefine them. In particular:

* blocks may span cycle boundaries,
* cycle boundaries may occur within blocks,
* no block has semantic meaning independent of cycles.

Cycle boundaries continue to govern:

* derived state recomputation,
* lifecycle_state transitions,
* governance activation timing.

Snapshots MAY be scheduled at block heights immediately following a block that contains a cycle_close event for convenience, but snapshots are always keyed to block height (per Snapshot Format v0); cycles do not define snapshot identity, keys, or boundaries.

### 1.4 Relationship to snapshots [anchor: relationship_to_snapshots]

Snapshots are deterministic checkpoint artifacts keyed to a block height (per Snapshot Format v0) used for performance, indexing, and bootstrapping.

This spine complements snapshots by providing:

* integrity guarantees for the underlying history,
* chunked packaging for distribution,
* stable references for provenance and explanation.

Snapshots MAY reference block ranges and payload commitments defined by this spine, but snapshots do not depend on blocks for validity.

### 1.5 Relationship to offline operation and reseeding [anchor: relationship_to_offline_operation_and_reseeding]

Offline and Mindseed operation rely on the spine for:

* portable integrity proofs,
* verifiable partial history bundles,
* local replay without network access,
* deterministic merging after isolation.

This spine therefore defines the minimum structure necessary to carry the system's history and meaning across disconnected environments.

### 1.6 Relationship to external settlement chains [anchor: relationship_to_external_settlement_chains]

External chains (e.g., L1 settlement for POINT) are strictly outside this spine.

This spine:

* does not depend on external chains for integrity,
* does not require external data availability,
* does not accept external chains as authority.

External anchoring, if used, is treated only as additional evidence of existence at a time, never as canonical truth.

---

## 2. Canonical primitives and authority model [anchor: 2_canonical_primitives_and_authority_model]

### 2.1 Canonical primitives referenced [anchor: canonical_primitives_referenced]

This specification assumes the existence of the following canonical primitives defined elsewhere:

* **Canonical Event**: an immutable, signed record authored by a human identity.
* **Event Ordering**: a total order over canonical events.
* **Event Hash**: a cryptographic hash of an event's canonical representation.
* **Cycle**: a governed temporal boundary used for recomputation and activation.
* **Snapshot**: a deterministic checkpoint artifact keyed to a block height (per Snapshot Format v0).
* **Derived State**: any state computed deterministically from the canonical log.
* **Lifecycle State**: derived classification of ideas and connections (alive, rotting, burned).

No new canonical primitives are introduced here.

### 2.2 Authority model [anchor: authority_model]

Authority in the system derives exclusively from:

1. the existence of canonical events,
2. their ordering in the event log,
3. deterministic replay rules defined by protocol.

The spine introduces **no additional authority**.

In particular:

* A block hash does not make events valid.
* A snapshot does not override history.
* An anchor does not decide truth.
* A witness claim does not grant correctness.

All authority remains human-authored and protocol-defined.

### 2.3 Derived artifacts [anchor: derived_artifacts]

The following artifacts are defined by this spine and are **derived**, not authored:

* event blocks,
* block headers and hashes,
* block chains,
* payload commitments,
* pack definitions,
* provenance artifacts tied to blocks or cycles.

Derived artifacts MUST be computable by any conformant node and MUST NOT require discretionary choice.

### 2.4 Determinism requirement [anchor: determinism_requirement]

All derivations defined in this specification MUST satisfy:

* identical input canonical logs -> identical derived artifacts,
* no dependence on wall-clock time,
* no dependence on network topology,
* no dependence on node identity or role.

Any deviation from this requirement is a protocol violation.

---

## 3. Deterministic event block packaging [anchor: 3_deterministic_event_block_packaging]

### 3.1 Purpose of blocks [anchor: purpose_of_blocks]

Blocks exist to provide a **deterministic integrity and packaging layer** over the canonical event log. They make the history easier to verify, transport, reference, and preserve, without introducing any new authority, semantics, or ordering rules.

Blocks are not canonical objects. They are derived artifacts whose sole functions are:
- to provide hash-chained integrity proofs over the event log,
- to create stable, bounded chunks for replication and offline transport,
- to act as legible reference points for provenance, explanation, and witnessing.

### 3.2 Block definition [anchor: block_definition]

A block is a deterministic grouping of already-finalized canonical events.

- Blocks are formed by grouping canonical events in strict finalized event-log order.
- Each block contains exactly **N consecutive canonical events**, where N is a protocol constant (e.g. 50) defined by governance.
- Block boundaries depend only on event order and the fixed block size.

Given the same finalized canonical event log, every conformant node MUST derive the same blocks with identical boundaries.

### 3.3 Block identity and ordering [anchor: block_identity_and_ordering]

Blocks are identified by their **block ordinal**, starting from block 0.

The block ordinal is defined as:

- block 0 contains events `[0 ⬦ N-1]`,
- block 1 contains events `[N ⬦ 2N-1]`,
- and so on.

The block ordinal is purely positional. It does not imply time, authority, importance, semantic finality, or prefix-certificate authority.

### 3.4 Block header contents [anchor: block_header_contents]

Each block has a derived block header containing:

- the block ordinal,
- the hash of the previous block header (or a genesis constant for block 0),
- a cryptographic commitment to the ordered set of event hashes in the block,
- the inclusive range of event indices covered by the block.

The commitment to the block's events MAY be implemented as:
- a Merkle root over the ordered event hashes, or
- an equivalent rolling or tree-based commitment defined canonically.

The exact commitment scheme MUST be deterministic and specified unambiguously.

#### 3.4.1 Canonical block commitment scheme [anchor: canonical_block_commitment_scheme]

Each block MUST commit to the exact ordered set of canonical events it contains using a Merkle tree.

The Merkle root of this tree is included in the block header and is the sole commitment to block contents.

Rolling hashes or alternative accumulation schemes MUST NOT be used.

#### 3.4.2 Merkle ordering and leaf format [anchor: merkle_ordering_and_leaf_format]

The block's event Merkle root commits to the ordered sequence of canonical event hashes for the events included in the block.
The canonical event hash definition and the Merkle construction rules (leaf format, ordering, and padding) MUST follow the Canonical Encoding and Hashing Specification (v0). This document does not define byte-level Merkle rules.

#### 3.4.3 Hash function and domain separation [anchor: hash_function_and_domain_separation]

All block Merkle hashes MUST use the canonical hash function defined in the Canonical Encoding and Hashing Specification (v0).

Block Merkle hashing MUST use a distinct domain separation tag defined in the Canonical Encoding and Hashing Specification (v0) to prevent cross-domain collision with event hashes, payload hashes, or pack commitments.


### 3.5 Hash chaining and integrity guarantees [anchor: hash_chaining_and_integrity_guarantees]

Block headers are hash-chained by including the previous block's header hash in the current block header.

This hash chain provides:
- tamper evidence for the ordering and contents of the event log,
- efficient verification that two nodes share the same history prefix,
- a stable integrity backbone for transport and anchoring.

The hash chain does **not**:
- grant validity to events,
- resolve conflicts,
- finalize meaning,
- override deterministic replay.

Any node MAY independently verify the block chain against the finalized canonical event log.

### 3.6 Relationship between blocks and cycles [anchor: relationship_between_blocks_and_cycles]

Blocks and cycles are orthogonal.

- Blocks may span one or more cycle boundaries.
- A cycle boundary may occur within a block.
- No block has semantic meaning independent of cycles.

Cycles remain the only mechanism that governs:
- derived state recomputation,
- lifecycle_state transitions,
- governance activation timing.

Snapshots MAY be scheduled at block heights immediately following a block that contains a cycle_close event for convenience, but snapshots are always keyed to block height (per Snapshot Format v0); cycles do not define snapshot identity, keys, or boundaries.

Blocks exist only to package and secure the underlying finalized event history.

---

#### 3.7 Block identity and semantic neutrality [anchor: block_identity_and_semantic_neutrality]

Blocks are mechanical grouping artifacts only.

A block:
- does not assert truth,
- does not finalize meaning,
- does not imply consensus,
- does not imply endorsement.

Blocks exist solely to improve integrity verification, transport efficiency, and reference convenience.

Any interpretation of a block's contents or significance MUST occur through explicit truth claims and explanation layers, not through the block itself.


## 4. Text payload immutability and content addressing [anchor: 4_text_payload_immutability_and_content_addressing]

### 4.1 Text as canonical payload [anchor: text_as_canonical_payload]

Textual content authored by humans is part of the canonical record.

Whenever a canonical event introduces, updates, or references textual content, that content is treated as an immutable payload whose exact bytes matter.

The system guarantees immutability by **content addressing**, not by location or authority.

### 4.2 Content hashing [anchor: content_hashing]

Each distinct text payload MUST be hashed using the canonical hash function over the canonical byte representation defined in the Canonical Encoding and Hashing Specification (v0).

The resulting hash:
- uniquely identifies the payload,
- is included in the canonical event,
- is the sole authoritative reference to that payload.

Once a payload hash is referenced by a canonical event, the payload is immutable. Any change to the bytes necessarily produces a different hash and therefore a different payload.

#### 4.2.1 Payload hash algorithm [anchor: payload_hash_algorithm]

Payload hashing (canonicalization, hash input layout, and algorithm) MUST follow the Canonical Encoding and Hashing Specification (v0).

#### 4.2.2 Payload hash domain tag [anchor: payload_hash_domain_tag]

Payload hashing MUST use a domain separation tag defined in the Canonical Encoding and Hashing Specification (v0) that is distinct from those used for events, blocks, pack commitments, and bundle manifests.

This ensures that payload hashes cannot be misinterpreted as hashes of other artifact types.


### 4.3 Canonical byte representation [anchor: canonical_byte_representation]

All canonical byte representations (including structured payload canonicalization rules) MUST follow the Canonical Encoding and Hashing Specification (v0).
Preservation tooling MUST preserve and validate canonicalized bytes as defined there.

### 4.4 Payload blobs [anchor: payload_blobs]

Text payloads are stored and distributed as **content-addressed blobs**, keyed by their payload hash.

- Blobs are independent of events and blocks.
- Events reference blobs by hash; blobs do not reference events.
- Blobs may be stored, replicated, or omitted independently of the event log.

A node MAY possess an event without possessing the corresponding blob bytes, but it MUST be able to verify the blob if it is later obtained.

### 4.5 Integrity guarantees [anchor: integrity_guarantees]

This model guarantees that:

- any modification of text is detectable,
- payload substitution is impossible without detection,
- partial replicas remain verifiable,
- long-term preservation does not depend on trusted storage providers.

Immutability is cryptographic and mechanical, not social or institutional.

---

## 5. Multi-tier description model and replication flexibility [anchor: 5_multi_tier_description_model_and_replication_flexibility]

### 5.1 Description tiers [anchor: description_tiers]

Each idea and ordering in the system may have multiple description tiers, which are independent textual payloads.

Two orthogonal dimensions are defined:

- **Length tiers**: sentence, paragraph, full.
- **Complexity tiers**: fundamental, standard, advanced, canonical.

Each tier is represented as a distinct text payload with its own content hash.

### 5.2 Independence of tiers [anchor: independence_of_tiers]

Description tiers are independent:

- the presence or absence of one tier does not affect others,
- updating one tier does not alter the hashes of other tiers,
- different nodes may store different subsets of tiers.

This independence is essential for flexible replication and storage tradeoffs.

### 5.3 Breadth versus depth tradeoff [anchor: breadth_versus_depth_tradeoff]

The tiered model enables a deliberate tradeoff between:

- **breadth**: how many ideas are stored,
- **depth**: how much detail is stored per idea.

For a fixed storage budget, a node may choose to:
- store shallow descriptions for many ideas, or
- store deep descriptions for fewer ideas.

Both choices remain fully verifiable and interoperable.

### 5.4 Importance-weighted replication [anchor: importance_weighted_replication]

Because importance rankings are derived canonically elsewhere, they may be used to guide replication policy without affecting meaning.

This allows:
- the most important ideas to be replicated more deeply,
- less important ideas to be replicated more shallowly,
- storage effort to align with collective judgment.

The spine itself does not decide importance; it only enables importance-aware distribution.

### 5.5 Long-term preservation implications [anchor: long_term_preservation_implications]

The multi-tier model ensures that:

- essential ideas can be carried widely in minimal form,
- deeper reasoning can be preserved by archival nodes,
- offline bundles can balance size and informational value,
- historical understanding degrades gracefully rather than catastrophically.

This model directly supports the system's goal of preserving meaning across time, scale, and resource constraints.

## 6. Payload availability and replication model [anchor: 6_payload_availability_and_replication_model]

### 6.1 Integrity versus availability [anchor: integrity_versus_availability]

The spine explicitly separates **integrity** from **availability**.

- Integrity answers the question: "Can I be sure this text is exactly what was written?"
- Availability answers the question: ?Can I actually obtain the text bytes?

Integrity is guaranteed by:
- canonical events,
- content hashes,
- deterministic replay,
- block hash chaining.

Availability is not guaranteed automatically. It must be designed, incentivized, and verified. This section defines how availability is addressed without introducing new authority or centralization.

### 6.2 Payload packs as first-class objects [anchor: payload_packs_as_first_class_objects]

To make availability tractable and verifiable, the system defines **payload packs**.

A payload pack is a deterministic set of content-addressed blobs selected according to explicit rules. Packs are not canonical events; they are derived collections whose contents can be independently computed by any node.

Payload packs exist to:
- provide bounded, named units of replication,
- allow nodes to declare what they store,
- enable offline transport and reseeding,
- make “what should be widely preserved” objective and auditable.

### 6.3 Pack dimensions [anchor: pack_dimensions]

Every payload pack is defined along two orthogonal dimensions:

**Breadth** — which ideas (and orderings, where applicable) are included:
- all ideas across all history,
- only ideas in the current living map,
- top-ranked ideas by universal importance across all history,
- top-ranked ideas by universal importance in the living map,
- ideas added or modified within a specific cycle range.

**Depth** - which description tiers are included:
- sentence only,
- sentence + paragraph,
- all length tiers at a given complexity,
- all tiers (full archival).

The exact selection rules for each pack type MUST be deterministic and explicitly defined.

### 6.4 Standard pack categories [anchor: standard_pack_categories]

The protocol defines a small number of standard payload pack categories to support common preservation goals. These categories are normative examples intended to be widely interoperable; governance may introduce additional pack profiles without altering the underlying model.

Each pack category is defined by deterministic selection rules across two dimensions:

- breadth: which ideas and associated payloads are included (e.g., all history, living map, top-ranked),
- depth: which description tiers are included (e.g., sentence only, sentence+paragraph, all tiers).

Pack categories affect distribution and storage only. They MUST NOT affect canonical validity, event ordering, authorship, or deterministic replay rules.

The standard pack categories are:

- **Core Library Pack**
  Contains the most universally important ideas across all history, with deep description tiers. This pack represents the minimum “civilizational memory” set the system aims to preserve widely.

- **Living Map Pack**
  Contains all ideas whose lifecycle_state is alive, typically with shallow description tiers. This pack supports everyday interaction and current relevance.

- **Archive Pack**
  Contains all ideas and all description tiers across all history. This pack is intended for archival nodes and long-term custodians.

- **Cycle Delta Pack**
  Contains only payloads introduced or updated since a given cycle boundary. This pack supports incremental synchronization.

The exact deterministic selection rules for each standard pack category, including any thresholds, tie-breakers, and tier mappings, MUST be specified in Appendix A.

### 6.5 Pack commitments [anchor: pack_commitments]

At defined boundaries, the system computes **pack commitments** for one or more pack categories.

The default boundary is each cycle boundary. Governance MAY define a longer cadence for specific pack categories, but any cadence MUST be deterministic and MUST be expressed solely in terms of cycle indices.

#### Interaction with Snapshots and Bundle Publication

Snapshots are keyed to block height and emitted at deterministic block-height intervals (see snapshot-format-v0.md). Packs are computed at deterministic cycle boundaries (and any governance-defined longer cadence) as specified in this section, expressed in cycle indices. Bundle publication is defined in shared-map-and-payload-bundles-spec.md and occurs only at snapshot heights (see "Publication and Retention Schedule"). When constructing a bundle at snapshot height H, implementations MUST include the most recent eligible pack material available as of H (i.e., packs computed at or before the cycle boundary whose events are included in the snapshot at H), and MUST NOT invent recomputation steps at bundle time. None of these mechanisms use identifier ordering for precedence; ordering derives from canonical log order (block height + event_index).

A pack commitment consists of:

- an unambiguous identifier for the pack definition (pack_profile_id),
- the cycle index (or cycle range) for which the pack is computed,
- the deterministic ordered list of included payload hashes,
- a cryptographic commitment (e.g., a Merkle root) over the ordered payload hashes,
- the count of payload hashes committed.

Pack commitments are derived metadata. They define what the pack contains, not who stores it.

Pack commitments MUST be recorded in a deterministic, discoverable location. Conformant implementations MUST expose pack commitments as part of snapshot-associated metadata at the block height used to represent the applicable cycle index or range, such that independent verifiers can obtain:

- the pack_profile_id,
- the cycle index (or range),
- the commitment root,
- and any required verification parameters.

Any node can independently compute a pack commitment from the canonical log and verify that a received pack is complete and correct.

The commitment scheme for pack commitments, including leaf format, ordering, and domain separation, MUST be specified unambiguously in Appendix A.

### 6.6 Hosting roles and expectations [anchor: hosting_roles_and_expectations]

Nodes may choose to store different sets of payload packs depending on their capacity and role.

Typical roles include:

- **Light nodes**, which store minimal or no payload blobs but verify integrity using events, block headers, and pack commitments.
- **Full nodes**, which store at least the Living Map Pack and the Core Library Pack.
- **Archive nodes**, which store at least the Archive Pack and sufficient history to support long-term custodianship.

Role labels are descriptive and do not confer authority. If governance or rulebooks apply requirements or privileges based on claimed roles, those requirements MUST be expressed explicitly and MUST remain challengeable.

Nodes MAY publicly declare which packs they claim to host. Such declarations are informational and challengeable.

A hosting claim MUST be interpretable against published pack commitments. A node claiming to host a given pack for a given cycle (or cycle range) is implicitly claiming the ability to serve the exact payload blobs committed by the corresponding pack commitment.

### 6.7 Proof-of-serving challenges [anchor: proof_of_serving_challenges]

To prevent false hosting claims, the system supports **proof-of-serving challenges**.

Any identity may:

- request a specific payload blob by payload hash from a node claiming to host a pack that includes it,
- verify whether the returned blob matches the expected hash.

A proof-of-serving challenge MUST be based on objective, verifiable criteria:

- the requested payload hash is included in a published pack commitment for a pack the node claims to host,
- the node either serves the exact bytes matching the hash or it does not.

Failure to serve claimed content is publicly verifiable and may affect the credibility or eligibility of the node or identity under applicable rulebooks.

The spine itself does not assign penalties; it only makes verification possible.

The request/response protocol details and the canonical representation of proof-of-serving evidence SHOULD be specified in Appendix A to ensure interoperability across implementations.


## 7. Offline portability and reseeding [anchor: 7_offline_portability_and_reseeding]

### 7.1 Offline-first preservation goal [anchor: offline_first_preservation_goal]

A core goal of the system is that humans can preserve and interact with the record **without continuous network access**.

Offline operation is not an edge case; it is a first-class design requirement.

### 7.2 Portable seed bundles [anchor: portable_seed_bundles]

The system defines **portable seed bundles** as self-contained collections of data sufficient to:

- verify integrity locally,
- replay canonical history deterministically,
- browse and interact with ideas and explanations,
- resume participation after reconnection.

A seed bundle MAY include:
- a slice of the canonical event log,
- one or more snapshots and required indexes,
- selected payload packs,
- block headers and pack commitments,
- client software or a compatible UI/game build.

### 7.3 Local replay guarantee [anchor: local_replay_guarantee]

Given a valid seed bundle, a user MUST be able to:

- verify all included data against hashes and commitments,
- reconstruct derived state via deterministic replay,
- browse ideas, orderings, descriptions, and provenance,
- interact with the system locally within the limits of offline operation.

No trusted third party is required for local verification.

### 7.4 Reseeding and merge [anchor: reseeding_and_merge]

Events authored while offline are merged back into the canonical log according to the deterministic merge rules defined elsewhere.

Payload packs and seed bundles:
- do not grant authority,
- do not override canonical ordering,
- act only as carriers for data and integrity proofs.

This allows the system to survive fragmentation, censorship, or temporary capture and later reconstitute a shared record.

---

## 8. Human witness and provenance layer [anchor: 8_human_witness_and_provenance_layer]

### 8.1 Rationale [anchor: rationale]

Cryptographic integrity alone does not guarantee human understanding or trust.

The system therefore supports a **human witness and provenance layer** that records social observation of mechanical processes. This layer makes the system's operation legible and contestable by people, not just by software.

### 8.2 Blocks and cycles as conceptual objects [anchor: blocks_and_cycles_as_conceptual_objects]

Although blocks and cycles are derived artifacts, they MAY be represented in the idea graph as conceptual ideas.

These representations:
- are read-only reflections of derived artifacts,
- cannot be edited or redefined,
- exist to anchor explanation, witnessing, and navigation.

#### 8.2.1 Canonical mapping from derived artifact to graph object [anchor: canonical_mapping_from_derived_artifact_to_graph_object]

Derived mechanical artifacts, including blocks, cycles, snapshots, and pack commitments, MAY be represented in the idea graph as conceptual ideas.

When such a representation exists, the mapping from mechanical artifact to graph object identifier MUST be deterministic.

The canonical graph object identifier for a mechanical artifact MUST be derived from:

- the artifact type (e.g., block, cycle, snapshot, pack_commitment),
- the canonical artifact identifier (e.g., block hash, cycle index, snapshot id),
- a fixed namespace identifier defined in Appendix A.

This mapping ensures that all conformant nodes derive the same graph object identifier for the same mechanical artifact, independent of when or where the representation is created.

The absence of a graph object representation does not affect the canonical validity of the underlying artifact.

#### 8.2.2 Attached truth claims for mechanical artifacts [anchor: attached_truth_claims_for_mechanical_artifacts]

Truth claims MAY be attached to conceptual representations of mechanical artifacts to express human observation, interpretation, or verification.

A minimal recommended set of truth claims for a represented artifact includes:

- a claim that the artifact exists,
- a claim identifying the cycle or block context in which it was created,
- a claim identifying the observed creator set or participants, if applicable.

Such claims are OPTIONAL but RECOMMENDED for artifacts intended to serve as socially legible provenance anchors.

If attached, these truth claims are subject to the same challenge, evidence, and certainty mechanics as all other truth claims in the system.

Truth claims about mechanical artifacts do not grant authority to the artifact itself and MUST NOT alter deterministic replay or canonical state.


### 8.3 Witness truth claims [anchor: witness_truth_claims]

Human identities may author truth claims about derived artifacts, such as:

- the existence of a specific block,
- the contents of a block or pack,
- the occurrence of a cycle boundary,
- successful verification or replication.

These claims follow the same challengeable truth-claim mechanics as any other claim in the system.

### 8.4 Evidence for witnessing [anchor: evidence_for_witnessing]

Witness claims may attach evidence, including:
- block header hashes,
- inclusion proofs for events or payloads,
- pack commitments,
- proof-of-serving results,
- attestations of independent verification.

Mechanical evidence and human testimony coexist; neither replaces the other.

### 8.5 Certainty accumulation [anchor: certainty_accumulation]

Certainty about witnessed facts accumulates through:
- multiple independent witnesses,
- diversity of identities,
- consistency with mechanical evidence,
- absence of unresolved challenges.

Witnessing increases confidence and understanding but never confers canonical authority.

### 8.6 Provenance and explanation chains [anchor: provenance_and_explanation_chains]

Witness claims, explanations, and verifications may be linked chronologically using vine-like structures.

These chains allow observers to:
- trace how understanding of the system evolved,
- see who noticed what and when,
- distinguish mechanical fact from interpretation,
- build trust through transparent observation rather than blind faith.

This layer fulfills the system's goal of preserving not just history, but **the shared human understanding of that history**.


## 9. Explanation, prediction, and verification chains [anchor: 9_explanation_prediction_and_verification_chains]

### 9.1 Purpose of the explanation layer [anchor: purpose_of_the_explanation_layer]

Beyond preserving events and payloads, the system must preserve **understanding**.

The explanation layer exists to record how humans interpret, summarize, teach, and reason about what the system is doing mechanically and why it matters. This layer is explicitly human-authored and challengeable, and it evolves over time as understanding improves.

Explanation is not an optional UI feature; it is a preserved part of the deliberative record.

### 9.2 Explainer claims [anchor: explainer_claims]

An **explainer claim** is a truth claim authored by a human identity that describes:

- what happened mechanically (e.g., how a block was formed),
- how to interpret a derived artifact (block, cycle, pack),
- why a change or boundary matters,
- how the system arrived at a particular state.

Explainer claims are not authoritative interpretations. They are proposals for understanding, subject to challenge, revision, and replacement like any other truth claim.

### 9.3 Tiered explanations [anchor: tiered_explanations]

Explainer claims MAY use the same tiered description system as ideas:

- sentence-level explanations for quick orientation,
- paragraph-level explanations for general understanding,
- full or advanced explanations for technical audiences.

This ensures that explanations themselves are accessible at different levels of depth, without fragmenting the record.

### 9.4 Prediction claims [anchor: prediction_claims]

The system supports **predictive truth claims** about its own future behavior.

Examples include claims such as:
- a particular challenge will close in the next cycle,
- a specific event will appear in the next block,
- a pack commitment will include a given payload,
- a derived state transition will occur at a boundary.

Prediction claims are explicit, time-scoped, and falsifiable.

#### 9.4.1 Prediction scoping rules [anchor: prediction_scoping_rules]

A prediction truth claim about a future mechanical artifact MUST be scoped such that it is falsifiable.

A prediction MUST reference one or more of the following deterministic identifiers:

- a future cycle index or bounded cycle range,
- a future block position relative to a cycle boundary,
- a specific artifact type expected to exist by a given cycle.

Predictions that lack a bounded scope or cannot be evaluated against a future canonical artifact MUST be considered non-falsifiable and SHOULD be treated as low-certainty claims.

Prediction claims do not influence mechanical execution and cannot cause or prevent the creation of any artifact.


### 9.5 Verification claims [anchor: verification_claims]

After the relevant boundary or event occurs, identities may author **verification claims** that assess whether a prediction was fulfilled.

Verification claims may reference:
- canonical events,
- block headers,
- cycle boundaries,
- pack commitments,
- snapshots or derived state.

Verification claims increase or decrease certainty in the original prediction claim and in the predictive reliability of the identity making it.

#### 9.5.1 Verification claim linkage rules [anchor: verification_claim_linkage_rules]

A verification truth claim that evaluates a prior prediction MUST include deterministic linkage fields.

At minimum, a verification claim MUST reference:

- the identifier of the prediction truth claim being evaluated,
- the identifier of the observed mechanical artifact,
- one or more evidence references supporting the evaluation.

The linkage between prediction and verification claims MUST be explicit and machine-readable.

This linkage allows deterministic tracking of prediction accuracy, reliability of observers, and longitudinal epistemic performance without introducing new authority or weighting mechanisms.


### 9.6 Chronological provenance chains [anchor: chronological_provenance_chains]

Explainer, prediction, and verification claims may be linked into **chronological provenance chains** using vine-style connections.

These chains allow observers to:
- follow the unfolding of expectations and outcomes,
- see where predictions succeeded or failed,
- evaluate the reliability of observers over time,
- understand system behavior step by step.

The preservation spine treats these chains as first-class interpretive history, even though they do not affect canonical state.

---

## 10. External anchoring (optional, non-authoritative) [anchor: 10_external_anchoring_optional_non_authoritative]

### 10.1 Purpose of external anchoring [anchor: purpose_of_external_anchoring]

External anchoring exists solely to increase survivability and timestamp evidence.

Anchoring provides an additional signal that a particular state of the system existed at or before a certain time, independent of any single host or jurisdiction.

Anchoring is never required for correctness.

### 10.2 What may be anchored [anchor: what_may_be_anchored]

Only derived, integrity-focused artifacts MAY be anchored externally, such as:

- block header hashes,
- pack commitment roots,
- snapshot commitment hashes.

Canonical events, text payloads, or private data MUST NOT be directly anchored.

#### 10.2.1 Anchor payload schema [anchor: anchor_payload_schema]

An external anchor payload MUST consist of a minimal, self-contained commitment that can be independently verified.

At minimum, an anchor payload MUST include:

- the canonical artifact identifier being anchored (e.g., snapshot commitment hash),
- the cycle index at which the artifact was produced,
- a domain-separated hash of the committed data,
- a version identifier for the anchoring schema.

Anchor payloads MUST NOT include semantic interpretation, authorship claims, or authority assertions.

External anchoring provides immutability amplification only and MUST NOT be treated as a source of canonical truth.


### 10.3 Authority rule [anchor: authority_rule]

External anchors are strictly non-authoritative.

They:
- do not override deterministic replay,
- do not resolve disputes,
- do not finalize meaning,
- do not grant legitimacy.

If an anchor conflicts with the canonical log, the canonical log prevails.

### 10.4 Anchor registry (optional), optionality, and diversity [anchor: anchor_registry_optional_optionality_and_diversity]

Anchoring is optional and may be performed by any identity or node.

Multiple anchors from different external systems may coexist. No single anchoring system is privileged by the protocol.

Anchors are treated as evidence claims that can be cited, challenged, or ignored. External anchors do not confer canonical authority and do not override deterministic replay.

The system MAY maintain an anchor registry as a discoverability layer for external anchors. If implemented, an anchor registry MUST:

- index anchors by the canonical artifact identifier being anchored and the associated cycle index,
- record sufficient information to locate or reference the external anchor publication (e.g., system name and publication reference),
- remain fully challengeable and non-authoritative.

The absence, incompleteness, or failure of an anchor registry does not affect canonical validity, internal integrity guarantees, or deterministic replay.

## 11. Security properties, failure modes, and limits [anchor: 11_security_properties_failure_modes_and_limits]

### 11.1 Partial availability [anchor: partial_availability]

The system is designed to tolerate partial availability.

- Missing payload blobs are detectable via hash mismatch.
- Missing packs can be requested or reconstructed.
- Nodes can function with incomplete payloads as long as integrity metadata is present.

Availability degradation never results in silent corruption.

### 11.2 Malicious or incorrect witnesses [anchor: malicious_or_incorrect_witnesses]

Witness and explainer claims may be false, mistaken, or misleading.

This is expected.

Such claims are handled through:
- challenge mechanisms,
- evidence comparison,
- certainty adjustment.

No witness claim can alter canonical state, so malicious witnessing cannot corrupt the system's foundation.

### 11.3 Storage loss and recovery [anchor: storage_loss_and_recovery]

Loss of hosted data by individual nodes does not compromise the system as long as at least one copy remains recoverable.

The combination of:
- content addressing,
- pack commitments,
- wide replication,
- offline seed bundles,

allows recovery and reseeding even after significant data loss.

### 11.4 Capture and restart scenarios [anchor: capture_and_restart_scenarios]

In the event of capture, censorship, or shutdown:

- preserved packs and seed bundles may be carried forward independently,
- the canonical log can be reconstructed from surviving copies,
- the system can restart without loss of historical integrity.

The spine favors continuity of meaning over continuity of infrastructure.

### 11.5 Explicit limits [anchor: explicit_limits]

This spine cannot guarantee:
- that data will never be destroyed,
- that humans will always preserve it,
- that understanding will be correct or unanimous,
- that the system cannot be captured temporarily.

What it guarantees is:
- tampering is detectable,
- loss is visible,
- reconstruction is possible,
- preservation effort aligns with collective importance.

These guarantees are sufficient to support long-term deliberative continuity without relying on central authority.


## 12. Conformance requirements [anchor: 12_conformance_requirements]

### 12.1 General conformance principle [anchor: general_conformance_principle]

Conformance to the Canonical Preservation & Provenance Spine requires that a node, client, or tool correctly derives, verifies, and exposes the artifacts defined in this specification without introducing discretionary behavior or hidden authority.

A conformant implementation MUST:
- derive identical results from identical canonical inputs,
- treat all spine artifacts as derived, not authored,
- expose integrity and provenance information in a verifiable way,
- avoid introducing semantic meaning where none is defined.

### 12.2 Event log and block derivation conformance [anchor: event_log_and_block_derivation_conformance]

A conformant node MUST:

- ingest canonical events in their total order,
- compute event hashes using the canonical representation,
- derive block boundaries exactly according to the configured block size,
- compute block headers and block hashes deterministically,
- verify block hash chaining against the event log.

A conformant node MUST NOT:
- reorder events,
- omit events from blocks,
- introduce alternative block boundaries,
- treat block headers as authoritative state.

### 12.3 Payload hash and blob verification [anchor: payload_hash_and_blob_verification]

A conformant implementation MUST:

- compute payload hashes using the canonical byte representation,
- verify that any obtained payload blob matches its referenced hash,
- treat hash mismatch as an integrity failure.

A conformant implementation MAY:
- operate without possessing payload blobs,
- defer blob acquisition until requested,
- store only a subset of payload tiers.

### 12.4 Pack computation and verification [anchor: pack_computation_and_verification]

If an implementation supports payload packs, it MUST:

- compute pack membership deterministically from the canonical log and defined pack rules,
- compute pack commitments exactly as specified,
- verify that a received pack's contents match its declared commitment,
- expose verification failures clearly.

An implementation MUST NOT:
- accept partial packs as complete,
- silently ignore missing or mismatched blobs.

### 12.5 Hosting claims and proof-of-serving [anchor: hosting_claims_and_proof_of_serving]

If an implementation or identity makes a hosting claim, it MUST:

- accurately represent which packs or blobs are hosted,
- respond correctly to proof-of-serving requests within reasonable limits,
- expose failures to serve claimed content as observable outcomes.

The protocol does not mandate penalties, but conformance requires transparency.

### 12.6 Offline and local operation conformance [anchor: offline_and_local_operation_conformance]

A conformant implementation that supports offline operation MUST:

- allow local verification of all included data,
- allow deterministic replay from offline bundles,
- preserve provenance metadata even when disconnected.

Local operation MUST NOT depend on external timestamps, anchors, or services.

### 12.7 Explainability requirements [anchor: explainability_requirements]

A conformant UI or client SHOULD expose:

- block and pack provenance information,
- integrity verification status,
- reasons for missing or unavailable content,
- links to witness, explainer, and verification claims when present.

This requirement ensures that preservation is legible, not opaque.

---

### 12.8 Minimum interoperability set [anchor: minimum_interoperability_set]

To be considered minimally conformant, an implementation MUST support the following capabilities:

- derivation and verification of blocks from the canonical event log,
- verification of payload hashes against canonical byte representations,
- verification of pack commitments, regardless of whether the node hosts the associated payloads,
- verification of at least one standard seed bundle or pack profile defined in this specification or its appendices.

Implementations MAY support additional pack profiles, anchoring mechanisms, or storage roles, but MUST support the minimum set above to ensure interoperability across the network.


## 13. Relationship to other specifications [anchor: 13_relationship_to_other_specifications]

### 13.1 Normative dependencies [anchor: normative_dependencies]

This specification depends on and MUST be interpreted consistently with the following documents:

- **Protocol v5** - defines canonical authority, cycles, and deterministic replay.
- **Deterministic Replay & Merge Specification** - defines how canonical logs are replayed and merged.
- **Node & Conformance Specification** - defines baseline node behavior and network expectations.
- **Offline & Mindseed Specification** - defines offline operation and merge semantics.
- **Token Specification** - defines economic effects that may depend on derived state.
- **Tribe Specification** - defines scoped interaction without altering canonical preservation.
- **Safety and Safety Rulebook Specifications** - define visibility and redaction rules that may affect presentation but not integrity.

Where conflicts appear, Protocol v5 and Deterministic Replay rules take precedence.

### 13.2 Non-normative references [anchor: non_normative_references]

The following documents may reference this spine for explanatory or illustrative purposes but do not override it:

- system overview documents,
- technical architecture overviews,
- game or interface specifications,
- educational materials and tutorials.

### 13.3 Separation of concerns [anchor: separation_of_concerns]

This specification explicitly does not define:

- governance processes,
- identity verification mechanics,
- voting or challenge lifecycles,
- token minting, routing, or settlement,
- UI layout or rendering rules.

Those concerns are delegated to their respective specifications and must not be reintroduced here.

### 13.4 Forward compatibility [anchor: forward_compatibility]

Future specifications MAY extend:
- additional pack profiles,
- additional anchoring mechanisms,
- additional provenance visualization techniques,

provided that:
- all extensions remain derived from canonical events,
- no new authority is introduced,
- determinism is preserved,
- existing commitments remain verifiable.

Any extension that violates these constraints is non-conformant.


## 14. Security assumptions and threat model [anchor: 14_security_assumptions_and_threat_model]

### 14.1 Assumed adversaries [anchor: assumed_adversaries]

This specification assumes the presence of adversaries who may attempt to:

- alter or rewrite historical text or events,
- selectively withhold or destroy payload data,
- present incomplete or misleading replicas,
- impersonate preservation or hosting roles,
- confuse users about what is canonical versus derived,
- undermine trust by exploiting technical opacity.

The spine is designed to make such attacks **detectable, legible, and recoverable**, not impossible.

### 14.2 Trust assumptions [anchor: trust_assumptions]

The system makes the following minimal trust assumptions:

- cryptographic hash functions remain collision-resistant,
- digital signatures correctly bind ordinary human-authored event candidates to identities under `canonical-event-authorship-and-signature-profile-v0.md`,
- at least one honest copy of important data survives somewhere,
- deterministic replay rules are public and inspectable.

No trust is placed in:
- specific nodes,
- specific hosting providers,
- centralized storage,
- external blockchains,
- institutional continuity.

### 14.3 Attack surfaces and mitigations [anchor: attack_surfaces_and_mitigations]

**Event tampering**
Mitigated by signed canonical events and hash-chained block derivation. Any modification produces detectable divergence.

**Payload substitution**
Mitigated by content-addressed blobs. Incorrect bytes cannot masquerade as valid payloads.

**Selective history erasure**
Mitigated by pack commitments, offline bundles, and wide replication. Missing data is detectable and explicitly visible.

**False completeness claims**
Mitigated by deterministic pack definitions and proof-of-serving challenges. Nodes cannot claim to host what they cannot serve.

**Mechanical obscurity attacks**
Mitigated by the witness, explainer, and prediction layers. Human-authored interpretation is preserved alongside mechanical facts.

### 14.4 Non-mitigated risks [anchor: non_mitigated_risks]

This spine does not and cannot prevent:

- all copies of data being destroyed,
- widespread disinterest in preservation,
- coordinated misinformation in explainer claims,
- temporary system capture or shutdown.

The system prioritizes **recoverability and auditability** over absolute prevention.

### 14.5 Security posture summary [anchor: security_posture_summary]

The preservation spine provides:

- strong tamper evidence,
- explicit detection of loss or inconsistency,
- multiple independent recovery paths,
- a clear separation between fact, derivation, and interpretation.

It does not rely on secrecy, trust, or centralized enforcement.

---

## 15. Design rationale and invariants [anchor: 15_design_rationale_and_invariants]

### 15.1 Core design invariants [anchor: core_design_invariants]

This specification is constrained by the following invariants, which MUST hold for all compliant implementations:

1. **Human-first authorship**
   All canonical events originate from identifiable human identities. The spine introduces no automated authorship.

2. **Deterministic derivation**
   All spine artifacts are computed deterministically from the canonical event log.

3. **No new authority**
   Hashes, blocks, packs, witnesses, and anchors confer no authority over truth or meaning.

4. **Separation of integrity and availability**
   Integrity is guaranteed mechanically; availability is encouraged, verified, and made legible.

5. **Offline survivability**
   The system must remain verifiable and usable without continuous connectivity.

6. **Legibility over opacity**
   Mechanical processes must be explainable and observable by humans.

### 15.2 Why blocks exist without consensus [anchor: why_blocks_exist_without_consensus]

Blocks are used because:
- bounded chunks are easier to verify and transport,
- hash chains provide efficient integrity proofs,
- humans benefit from discrete reference points.

Blocks do not exist to decide ordering, validity, or meaning. Those functions are already fulfilled by finalized prefix certificates, the canonical event log, and deterministic replay.

### 15.3 Why text is content-addressed [anchor: why_text_is_content_addressed]

Text payloads are content-addressed because:
- meaning depends on exact wording,
- storage location is irrelevant to integrity,
- long-term preservation requires independence from infrastructure.

This approach allows the system to degrade gracefully under loss while remaining verifiable.

### 15.4 Why witnessing is preserved [anchor: why_witnessing_is_preserved]

Witnessing and explanation are preserved because:
- purely mechanical records are insufficient for human trust,
- understanding is part of the historical record,
- disagreement about interpretation should itself be visible.

Witness claims add social provenance without creating authority.

### 15.5 Why availability is tiered [anchor: why_availability_is_tiered]

Not all ideas can or should be stored everywhere.

Tiered replication:
- aligns preservation effort with collective importance,
- enables participation across resource constraints,
- increases resilience by encouraging many shallow copies.

This design maximizes the number of humans who can carry meaningful parts of the record.

### 15.6 Final rationale [anchor: final_rationale]

The Canonical Preservation & Provenance Spine exists to ensure that:

- the record of collective reasoning cannot be silently altered,
- loss is visible rather than hidden,
- recovery is possible without permission,
- meaning survives infrastructure failure,
- humans can understand not just *what* happened, but *how* and *why*.

This spine does not make the system perfect or invulnerable.
It makes it **honest, inspectable, and preservable**, which is sufficient for long-term deliberative continuity.




## Appendix A - Deterministic Commitments, Pack Profiles, and Bundle Manifests [anchor: appendix_a_deterministic_commitments_pack_profiles_and_bundle_manifests]

### A0. Appendix purpose and status [anchor: a0_appendix_purpose_and_status]

This appendix defines the exact deterministic algorithms, byte formats, and schemas referenced normatively in Sections 3 through 7 of this specification.

All rules in this appendix are normative unless explicitly stated otherwise.

Any implementation claiming conformance with this specification MUST implement the algorithms and formats defined herein exactly. Deviations result in non-conformance, even if higher-level behavior appears correct.

This appendix exists to eliminate ambiguity for implementers and to ensure that independent implementations produce byte-identical commitments, hashes, and verification results.

---

### A1. Cryptographic primitives and domain separation [anchor: a1_cryptographic_primitives_and_domain_separation]

#### A1.1 Hash algorithms [anchor: a1_1_hash_algorithms]

Hash algorithms are defined exclusively by the Canonical Encoding and Hashing Specification (v0).

This appendix does not define or restate any hash algorithm; implementations MUST use exactly the algorithm(s) specified there.

#### A1.2 Domain tag strings and byte layout rules [anchor: a1_2_domain_tag_strings_and_byte_layout_rules]

Domain separation tag strings and their byte layout rules are defined exclusively by the Canonical Encoding and Hashing Specification (v0).

This appendix only references canonical tags when describing commitments; it MUST NOT introduce alternative tag strings or byte layouts.

#### A1.3 Canonical encoding rules [anchor: a1_3_canonical_encoding_rules]

Canonical primitive encoding rules are defined exclusively by the Canonical Encoding and Hashing Specification (v0).

This appendix does not restate or modify those rules; implementations MUST use the canonical encodings specified there.

---

### A2. Canonical event hash representation reference [anchor: a2_canonical_event_hash_representation_reference]

#### A2.1 Event canonicalization [anchor: a2_1_event_canonicalization]

Event canonicalization, hashing, domain separation, and byte-level encoding are defined exclusively by the Canonical Encoding and Hashing Specification (v0). This document references those rules but does not redefine them.

This appendix does not redefine event canonicalization. It normatively references the event canonicalization rules defined there.

Any change to event canonicalization MUST be reflected consistently across all references in this appendix.

#### A2.2 Event hash bytes and inclusion leaf format [anchor: a2_2_event_hash_bytes_and_inclusion_leaf_format]

Event hashing inputs, domain tags, and inclusion-leaf byte formats MUST follow the Canonical Encoding and Hashing Specification (v0).
This section describes how event hashes are used in preservation proofs; it does not define byte-level formats.

---

### A3. Block commitment scheme (MUST) [anchor: a3_block_commitment_scheme_must]

#### A3.1 Block leaf format [anchor: a3_1_block_leaf_format]

Each block commits to an ordered list of canonical events.

Block leaf hashing inputs, domain tags, and leaf byte layouts MUST follow the Canonical Encoding and Hashing Specification (v0).

This section describes the role of block leaves in preservation, not their byte-level definition.

#### A3.2 Merkle tree ordering and padding [anchor: a3_2_merkle_tree_ordering_and_padding]

Merkle tree ordering and padding rules are defined exclusively by the Canonical Encoding and Hashing Specification (v0).

This appendix does not redefine those rules; implementations MUST use the canonical Merkle construction specified there.

#### A3.3 Block header canonical byte format [anchor: a3_3_block_header_canonical_byte_format]

The canonical block header byte representation MUST follow the Canonical Encoding and Hashing Specification (v0).
The logical header contains: block_version, block_ordinal, prev_block_header_hash, start_event_index, event_count, block_event_merkle_root.

#### A3.4 Block header hash computation [anchor: a3_4_block_header_hash_computation]

Block header hash computation (domain tag, canonical byte layout, and hash input construction) MUST follow the Canonical Encoding and Hashing Specification (v0).

The resulting hash uniquely identifies the block and serves as the canonical block identifier.

Block header hashes form a cryptographic hash chain through the prev_block_header_hash field, providing immutable ordering and integrity guarantees independent of cycle boundaries.

#### A3.5 Block and cycle orthogonality [anchor: a3_5_block_and_cycle_orthogonality]

Blocks and cycles are orthogonal constructs.

Blocks provide deterministic grouping, hashing, and chaining of canonical events.

Cycles provide deterministic state evaluation boundaries for derived state recomputation and governance effects.

No block header field MAY reference cycle indices, cycle boundaries, or snapshot identifiers.

---

### A4. Payload canonicalization and payload hash computation (MUST) [anchor: a4_payload_canonicalization_and_payload_hash_computation_must]

#### A4.1 Text encoding and Unicode normalization [anchor: a4_1_text_encoding_and_unicode_normalization]

Text encoding and Unicode normalization rules are defined exclusively by the Canonical Encoding and Hashing Specification (v0).

This appendix does not restate or modify those rules; implementations MUST apply the canonical text rules specified there.

#### A4.2 Line ending and whitespace rules [anchor: a4_2_line_ending_and_whitespace_rules]

Text canonicalization (including Unicode normalization, line ending rules, and whitespace handling) MUST follow the Canonical Encoding and Hashing Specification (v0). Preservation tooling MUST preserve and validate canonicalized bytes as defined there.

#### A4.3 Structured payload canonicalization [anchor: a4_3_structured_payload_canonicalization]

Structured payload canonicalization rules are defined exclusively by the Canonical Encoding and Hashing Specification (v0).

This appendix does not restate or modify those rules; any structured canonicalization MUST follow the canonical specification.

#### A4.4 Payload hash computation and blob naming [anchor: a4_4_payload_hash_computation_and_blob_naming]

Payload hashing (canonicalization, domain tags, and hash input layout) MUST follow the Canonical Encoding and Hashing Specification (v0). Blob naming MUST use the resulting payload hash as defined there.


### A5. Standard payload pack profiles (normative definitions) [anchor: a5_standard_payload_pack_profiles_normative_definitions]

This section defines the standard payload pack profiles referenced normatively by the specification.

Each pack profile is defined by deterministic rules along four axes:

- breadth: which ideas or artifacts are eligible for inclusion,
- depth: which description tiers are included,
- ordering: how eligible payloads are sorted,
- cadence: the cycle boundaries at which the pack is computed.

Unless otherwise stated, all pack profiles are computed at every cycle boundary.

Governance MAY introduce additional pack profiles, but MUST NOT modify the definitions below retroactively.

#### A5.1 Core Library Pack [anchor: a5_1_core_library_pack]

Breadth:
- Includes payloads associated with ideas whose universal importance exceeds a governance-defined Core Library threshold at the snapshot block height.
- Includes payloads required to interpret those ideas (e.g., descriptions, arguments, evidence).

Depth:
- Includes all description tiers (sentence, paragraph, full; all complexity levels).

Ordering:
- Sorted by descending universal importance.
- Ties are broken by canonical event order (block height, event_index).

Cadence:
- Computed at every cycle boundary.

#### A5.2 Living Map Pack [anchor: a5_2_living_map_pack]

Breadth:
- Includes payloads associated with all ideas whose lifecycle_state is alive at the snapshot block height.

Depth:
- Includes sentence-level descriptions.
- Paragraph-level descriptions MAY be included if governance enables them explicitly.

Ordering:
- Sorted by descending relative importance within the living map.
- Ties are broken by canonical event order (block height, event_index).

Cadence:
- Computed at every cycle boundary.

#### A5.3 Cycle Delta Pack [anchor: a5_3_cycle_delta_pack]

Breadth:
- Includes payloads introduced, modified, or newly referenced by canonical events since the previous cycle boundary.

Depth:
- Includes only the description tiers affected by the delta.

Ordering:
- Sorted by canonical event order.

Cadence:
- Computed at every cycle boundary.

#### A5.4 Archive Pack [anchor: a5_4_archive_pack]

Breadth:
- Includes all payloads across all history.

Depth:
- Includes all description tiers.

Ordering:
- Sorted by canonical payload hash ordering.

Cadence:
- Computed at governance-defined multi-cycle intervals.

#### A5.5 Shallow Everything Pack (optional) [anchor: a5_5_shallow_everything_pack_optional]

Breadth:
- Includes payloads associated with all ideas across all history.

Depth:
- Includes sentence-level descriptions only.

Ordering:
- Sorted by canonical event order (block height, event_index).

Cadence:
- Computed at governance-defined intervals.

#### A5.6 Tie-breakers and determinism rules [anchor: a5_6_tie_breakers_and_determinism_rules]

All sorting operations MUST be stable.

If two payloads are otherwise indistinguishable under a pack's ordering rules, the canonical payload hash MUST be used as the final tie-breaker.

Pack membership MUST be identical for all conformant nodes given the same canonical state at the snapshot block height.

---

### A6. Pack commitment objects and recording location (MUST) [anchor: a6_pack_commitment_objects_and_recording_location_must]

#### A6.1 Pack commitment object schema [anchor: a6_1_pack_commitment_object_schema]

A pack commitment object MUST include the following fields:

- All numeric types in this schema (e.g., uint64, uint32) refer to internal canonical binary encodings. When exposed via APIs or transport schemas, these values MUST be transmitted as decimal strings per the API Contract.
- pack_profile_id (string),
- cycle_start (uint64),
- cycle_end (uint64 or equal to cycle_start),
- payload_count (uint32),
- payload_merkle_root (32 bytes),
- commitment_version (uint32).

All fields MUST be encoded using canonical encoding rules defined in §A1.3.

#### A6.2 Recording location [anchor: a6_2_recording_location]

Pack commitments MUST be recorded as derived metadata associated with the snapshot at the block height used to represent the cycle_end field (per Snapshot Format v0).

Conformant implementations MUST expose pack commitments alongside snapshot headers in a discoverable and verifiable manner.

Pack commitments MUST NOT be recorded as canonical events.

#### A6.3 Verification procedure [anchor: a6_3_verification_procedure]

To verify a received pack, a node MUST:

1. obtain the corresponding pack commitment,
2. compute payload hashes for all received payloads,
3. construct the Merkle tree using the rules in §A3,
4. verify that the resulting root matches payload_merkle_root,
5. verify that payload_count matches the number of leaves.

Any mismatch invalidates the pack.

#### A6.4 Pack commitment Merkle construction [anchor: a6_4_pack_commitment_merkle_construction]

Pack commitment hashing (domain tags, Merkle construction rules, leaf formats) MUST follow the Canonical Encoding and Hashing Specification (v0). This section specifies what the pack commitment is intended to commit to, not how the bytes are hashed.

The resulting Merkle root is recorded as payload_merkle_root in the pack commitment object.


### A7. Proof-of-serving protocol (MUST if supported) [anchor: a7_proof_of_serving_protocol_must_if_supported]

#### A7.1 Request message format [anchor: a7_1_request_message_format]

A proof-of-serving request MUST include:

- payload_hash (32 bytes),
- optional pack_profile_id (string),
- optional nonce (opaque byte sequence).

The request MUST be unambiguous and reproducible.

#### A7.2 Response format [anchor: a7_2_response_format]

A response MUST include:

- the raw payload bytes,
- optional nonce echo,
- optional inclusion proof (e.g., Merkle branch).

The responder MUST NOT alter the payload bytes in any way.

#### A7.3 Failure evidence format [anchor: a7_3_failure_evidence_format]

If a node fails to serve a requested payload, failure evidence MAY include:

- timeout records,
- incorrect payload bytes,
- refusal messages.

Failure evidence MUST be verifiable by third parties.

---

### A8. Seed bundle profiles and bundle manifest (normative) [anchor: a8_seed_bundle_profiles_and_bundle_manifest_normative]

#### A8.1 Playable Offline Bundle v1 (required) [anchor: a8_1_playable_offline_bundle_v1_required]

The Playable Offline Bundle v1 MUST include:

- the canonical event log up to a defined cycle boundary,
- the snapshot at the block height used for that boundary,
- at least one standard pack profile sufficient to render the living map,
- the bundle manifest.

This bundle MUST be sufficient to run a conformant interactive interface fully offline.

#### A8.2 Archive Bundle (optional) [anchor: a8_2_archive_bundle_optional]

An Archive Bundle MAY include:

- the full canonical event log,
- all snapshots,
- the Archive Pack,
- the bundle manifest.

#### A8.3 Bundle manifest schema and hashing [anchor: a8_3_bundle_manifest_schema_and_hashing]

A bundle manifest MUST include the following fields:

- bundle_version (uint32),
- snapshot_identifier (fixed-length identifier),
- list of included block header hashes,
- list of included pack commitment identifiers,
- list of included payload hashes.

Bundle manifest canonicalization and hashing (including schema-to-bytes rules, domain tags, and hash input layout) MUST follow the Canonical Encoding and Hashing Specification (v0). This section defines required manifest fields and preservation semantics; hashing bytes are defined in the canonical spec.

The manifest MUST NOT include its own hash as a field. The manifest hash is treated as an external label and verification target.


#### A8.4 Local verification steps [anchor: a8_4_local_verification_steps]

To verify a bundle, a node MUST:

1. verify the manifest hash,
2. verify all included pack commitments,
3. verify all payload hashes,
4. replay the canonical event log to the snapshot block height.

#### A8.5 Bundle verification closure [anchor: a8_5_bundle_verification_closure]

A bundle is valid if and only if all of the following conditions hold:

1. the bundle manifest hash verifies correctly,
2. all referenced block headers verify against their predecessor headers,
3. all pack commitments verify against their payload Merkle roots,
4. all included payload hashes match the referenced payload bytes,
5. deterministic replay from the included event log and snapshot produces a consistent canonical state.

Failure of any condition invalidates the bundle.


### A9. Reference schemas and test vectors (recommended) [anchor: a9_reference_schemas_and_test_vectors_recommended]

#### A9.1 Reference schemas [anchor: a9_1_reference_schemas]

The following reference schemas SHOULD be provided:

- block_header,
- pack_commitment,
- bundle_manifest,
- proof_of_serving_request,
- proof_of_serving_response.

Schemas MAY be expressed as JSON schemas or equivalent canonical field lists.

#### A9.2 Example test vectors [anchor: a9_2_example_test_vectors]

At least one small deterministic example SHOULD be provided demonstrating:

- event hashing,
- block commitment,
- pack commitment,
- bundle verification.

Test vectors MUST be sufficient for independent implementers to validate correctness.
