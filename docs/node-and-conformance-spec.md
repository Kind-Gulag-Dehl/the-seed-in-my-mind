---
doc_id: node_and_conformance_spec
title: Node and Conformance Specification
status: authoritative
version: v0
last_reviewed: 2026-06-22

scope:
  - Defines node responsibilities and conformance requirements for validation and replay.

authoritative_for:
  - Conformance requirements and validation behaviors for nodes.
  - How implementations prove they follow the protocol and deterministic replay.

not_authoritative_for:
  - Core semantics beyond conformance framing (see protocol v5.md).
  - Encoding specifics beyond what is referenced (see canonical-encoding-and-hashing-spec.md).

depends_on:
  - protocol v5.md
  - canonical-encoding-and-hashing-spec.md
  - deterministic-replay-and-merge-spec.md

conflicts:
  - none known

change_rules:
  - Any change here should be mirrored into conformance tests/spec harness expectations.

reader_path:
  - prereq: deterministic-replay-and-merge-spec.md
  - next: cycle-spec.md

keywords:
  - node
  - conformance
  - validation
  - replay
  - tests
---

# THE SEED IN MY MIND - NODE & CONFORMANCE SPEC
**Normative Sub-Specification**

This specification defines what it means to operate a *conformant node* in The Seed in My Mind ecosystem:
how nodes ingest and validate events, how they store and index the canonical universe, how they apply rulebooks at cycle boundaries, how they manage sandbox/AI boundaries, how they implement safety constraints, and how they ensure deterministic replay and interoperability across all implementations.

The Node Spec describes the **minimum required behavior** for any implementation, independent of programming language, database, hardware, or execution environment. Any system that behaves as specified in this document is a conformant Seed node.

---

## 0. Scope, Purpose, Definitions [anchor: 0_scope_purpose_definitions]

### 0.1 Scope [anchor: scope]

The Node & Conformance Specification defines the operational requirements for all node implementations, including:

- **Data retention requirements**
  Nodes MUST support conformance under multiple storage classes. Baseline-conformant nodes are NOT required to retain the complete canonical event log. Instead, baseline-conformant nodes MUST retain either (a) the complete canonical event log, or (b) a verifiably sufficient subset of deterministic history, such as readable state snapshots plus the cryptographic anchors and verification material required to confirm correspondence to a canonical snapshot.
  Separately, the protocol defines an **archival responsibility** requirement at the network level: at least one—and preferably many—independent archival replicas of the complete canonical event log (or a verifiably equivalent compressed representation) MUST exist in-network at all times. Implementations MAY define explicit “archival node” or “full-history node” classes whose conformance profile includes mandatory full-history retention, in order to encourage wide decentralized redundancy and discrepancy detection across multiple independently operated replicas.

- **Event validation rules**
  Nodes must validate schema, identity authorship, rulebook constraints, connection semantics, and challenge lifecycle rules before accepting an event.

- **Snapshot behavior**
  Nodes must generate, ingest, and interpret snapshots that commit rulebook state, safety rules, governance state, and token-rule conditions. **Rulebook activation occurs at scheduled cycle boundaries only after a valid voluntary human implementation/execution action has been recorded in the canonical log, as defined in the Governance Specification.**

- **Replay determinism**
  Deterministic replay is a property of the canonical universe and MUST remain possible in principle from genesis through all events and snapshots. Conformant nodes are NOT required to personally store or distribute full deterministic history. However, conformant nodes MUST be able to (a) deterministically apply and interpret snapshots and the rulebooks active at the corresponding cycle boundaries, and (b) verify that the state they present or operate on corresponds to a canonically anchored snapshot under the active verification rules. Archival/full-history nodes MUST be able to replay from genesis through all events and snapshots to produce an identical canonical universe.

- **Storage classes**
  Nodes must support three canonical payload visibility classes as defined by active safety rulebooks:
  - `normal`
  - `sensitive_abstracted`
  - `non_distributable_blocked`

- **Canonical publication integration**
  Nodes must integrate with the staged canonical publication system by verifying finalized prefix certificates, derived block commitments where exposed, and anchored snapshot summaries. Publication logic itself is defined in `pod-consensus-and-canonical-publication-spec.md`.

- **Identity and rulebook enforcement**
  Nodes must apply identity verification, human-first constraints, authorship rules, and rulebook-defined behavior exactly at cycle boundaries.

- **Sandbox handling**
  Nodes must ignore sandbox (AI map) events during canonical replay. Sandbox data is non-canonical and never affects the universal graph.

- **Interaction with safety and token subsystems**
  Nodes enforce safety classification and token execution rules at event-acceptance and snapshot-application time.

This specification **does not** define frontend interaction, UI metaphors, or network protocols for event distribution; it defines only what a node *must do* to be considered conformant.


---

### 0.2 Purpose [anchor: purpose]

The primary purposes of the Node & Conformance Spec are:

- **Ensure all conformant nodes produce the same canonical universe.**
  A node must arrive at an identical state (ideas, connections, challenges, rankings, rulebook activation, certainty bands, safety flags, token flows) when replaying the same event log.

- **Ensure deterministic replay from genesis.**
  Replay must be provably deterministic: any deviation indicates a defect in the implementation, not ambiguity in the spec.

- **Ensure rulebook-defined safety, governance, and token rules are applied identically.**
  Rulebooks are ordinary ideas but become active only through governance adoption actions.
  Nodes must interpret all events using the rulebooks active at their nearest snapshot boundary.

- **Facilitate offline use, replication, snapshots, reconciliation.**
  Nodes must support:
  - exporting/importing mindseeds,
  - offline-only logs,
  - delayed synchronization,
  - conflict-free deterministic merges via snapshot anchoring.

- **Provide a minimal, interoperable foundation across implementations.**
  Rust, Go, TypeScript, Python, or database-specific implementations must all converge on the *same* deterministic state when following this specification.

---

### 0.3 Key Definitions [anchor: key_definitions]

**Canonical Universe**
The globally replicated, deterministic idea/event graph built exclusively from human-authored events.
All state relevant to truth, importance, governance, identity, actions, and tokens arises from this universe.

**Sandbox Universe (AI Map)**
A speculative, non-canonical environment where AI agents operate without affecting the canonical universe.
Sandbox events never appear in the canonical log and are ignored during replay.

**Snapshot**
A frozen view of the canonical universe at a block boundary.
Snapshots determine:
- which rulebooks are active,
- which safety rules apply,
- which token rules are in force,
- what compression/compaction rules future replay uses.

Snapshots are binding: replay must treat them as authoritative boundaries.

**Replay Determinism**
The requirement that any node can start from genesis, ingest the event log and snapshots in order, apply rulebooks at the correct boundaries, and reconstruct *exactly* the same state.

**Payload Class**
A visibility and distribution classification applied by safety rulebooks:
- `normal` - fully distributable
- `sensitive_abstracted` - abstracted for distribution, full content locally retained
- `non_distributable_blocked` - cannot be distributed; abstract only

Nodes must enforce payload class behavior exactly.

**Rulebook**
A set of governance ideas adopted via a governance adoption action.
Rulebooks define:
- safety policies
- challenge parameters
- juror selection rules
- token rules
- storage/visibility rules
- event validation rules
Nodes must apply the rulebooks active at each snapshot.

**Valid Event**
An event that:

conforms to schema under the active rulebook version,

is authored by a valid human identity, with a verifiable Profile-v0 `signature` under `canonical-event-authorship-and-signature-profile-v0.md`,

may include AI provenance metadata, but AI identities MUST NOT appear as authors,

satisfies all rulebook constraints active at the relevant snapshot boundary,

references only existing canonical entities (ideas, identities, connections),

does not violate protocol-level invariants (e.g., cannot disable challenges, cannot introduce private governance, cannot break human-first identity).

Only valid events may be appended to the canonical event log or used in deterministic replay.

**Conformance Scope (Normative)**
Conformance scope includes, at minimum:
- deterministic custody manifest encoding and hashing (§8A)
- deterministic P2P reconciliation transcript generation (§8B)
- verification of state witness and receipt artifacts (§8C)
- deterministic ingestion of partitioned publications without implicit resolution (§8D)

---
## 1. Node Roles and Modes [anchor: 1_node_roles_and_modes]

### 1.1 Full Canonical Node (Normative) [anchor: full_canonical_node_normative]

A **full canonical node** is the only node class that qualifies as *conformant* under this specification.
It MUST:

- **Store the complete canonical event log** from genesis (or from a verified trusted snapshot if bootstrapping).
- **Retain all snapshots** since genesis or since its configured bootstrap epoch.
- **Validate all incoming events**, including schema validation, identity/authorship validation, rulebook checks, challenge lifecycle checks, and safety-class checks.
- **Execute deterministic replay** on demand or during synchronization.
- **Apply rulebook activation at cycle boundaries** exactly as specified in the Protocol.
- **Perform local safety classification** according to the active safety rulebooks.
- **Track all importance rankings**, certainty bands, challenge states, token balances, and governance activation.
- **Maintain identity provenance**, including authorship references and key-rotation chains.
- Optionally **store AI map / sandbox data**, but this is non-normative and MUST NOT affect canonical replay.

#### Canonical storage requirement [anchor: canonical_storage_requirement]
A full canonical node **MUST NOT apply editorial or topic-based filters** to the canonical event log or to canonical storage of ideas, descriptions, or connections.
The only permissible filters are:

- **Global safety rules** (Sections 8.1-8.3 of the Protocol), which define *payload_class* (`normal`, `sensitive_abstracted`, `non_distributable_blocked`).
- **Jurisdictional overlays** (§8.4 of the Protocol), which may conditionally restrict *distribution*, but not canonical storage.

Any omission, redaction, or deletion of canonical content-other than global safety transformations-renders the node **non-conformant**.

View-layer filtering (e.g., UI hiding sensitive ideas) is permitted, but **MUST NOT alter stored canonical state**.

---

### 1.2 Partial Nodes (Informative Only) [anchor: partial_nodes_informative_only]

Partial nodes are useful for many applications but are **non-conformant** for canonical responsibilities.
They MAY selectively store subsets of the universe, but they cannot serve as canonical witnesses.

Types include:

- **Light Node**
  Stores snapshots only (no full event log). Supports browsing, but cannot validate history.

- **Audit Node**
  Stores full events but performs **replay only**; does not sign or produce blocks.

- **Tribe Node**
  Hosts a tribe's internal importance map, internal rulebooks, and visibility scopes.
  Tribe data is public but scoped; tribe nodes DO NOT replace canonical nodes.

- **Ent-Tethered Node**
  Stores required data for running a local Ent training ring or equipped-helper model.
  These nodes interact heavily with the sandbox but do not control canonical state.

Partial nodes MUST NOT claim canonical conformance unless they meet all requirements of §1.1.

---

### 1.3 Offline Node Mode [anchor: offline_node_mode]

All node implementations MUST support offline operation.

An offline node MUST allow:

- **Full local read-only browsing** of the canonical universe as of its last synchronized snapshot.
- **Creation of offline event logs**, including idea creation, connections, drafts, and challenge openings.
- **Offline snapshots**, produced as local-only artifacts until reconnected.
- **Event accumulation** when disconnected, with eventual reconciliation at reconnection time.

Offline nodes MUST:

- Preserve full event history integrity.
- Postpone event validation until they reconnect to canonical peers.
- Merge offline logs deterministically using canonical publication order; canonical ordering of events is (block_height, event_index) in the canonical event log. Any event_id (UUIDv7) is an identifier only and MUST NOT be used as an ordering authority.
- Reject events that fail validation once full rulebook checks are applied at reconnection.

Offline publications from different partitions may overlap or conflict; nodes MUST preserve all valid events and surface conflicts explicitly. Merge behavior MUST conform to §8D and MUST NOT depend on arrival order.

Offline nodes MAY provide local-only sandbox computation, drafts, and Ent interactions.

---

## 2. Canonical Data Model Requirements [anchor: 2_canonical_data_model_requirements]

### 2.1 Mandatory Data Structures [anchor: mandatory_data_structures]

A conformant canonical node MUST store, index, and expose at least the following structures:

Event Log

Append-only, ordered by canonical log position (block_height, event_index).

Contains every canonical event from genesis (or from the node's configured epoch, if it bootstraps from a trusted snapshot).

If the node operates as a baseline-conformant node (non-archival), it is NOT required to retain or distribute the complete canonical event log. In that case, the node MUST still retain and expose whatever portion of the event log it holds (for example, events after its bootstrap snapshot) and MUST retain the verification material required to prove that its readable state corresponds to a canonically anchored snapshot (see “Chain Anchors” and “Snapshots” below).

If the node operates as an archival/full-history node, it MUST retain the complete canonical event log (or a verifiably equivalent compressed representation) and MUST be capable of replaying deterministically from genesis.

Snapshots (Readable State + Verification Material)

Readable state snapshots at canonical snapshot boundaries, sufficient to reconstruct the current live state of the canonical universe at that boundary (for example: current canonical descriptions, active connections, rank list state summaries, and verdict summaries), as defined by protocol rulebooks.

Cryptographic anchors and verification material sufficient to verify that a readable snapshot corresponds to a canonical snapshot commitment (for example: snapshot hash, anchor references, and any required inclusion proofs or commitments).

Snapshot boundary metadata freezing rulebook activation state, governance state, safety classifier versions, and token-rule conditions in force at that boundary.

A baseline-conformant node MAY store readable state snapshots without storing full deterministic history, provided it can verify snapshot correspondence to a canonical snapshot anchor under the active rulebooks.

Idea Table

All idea types, including:

truth claims

conceptual ideas

actionable ideas

actions

identity ideas

structural roles such as backyard, garden of relationships, anthill, vine, shrub (implemented via idea type + connections, not special types).

The Idea Table contains only canonical ideas created by canonical events. Private drafts that have not been published as canonical events are out of protocol conformance scope; nodes are not required to store, interpret, or expose draft artifacts, and drafts become ideas only when published into the canonical universe as canonical events.

Connection Table

All relative_importance usages (general, importance_argument, evidence_for, evidence_against, etc.).

Tiered same_as equivalence links.

Membership links (tribe membership, garden shrubs).

Authorship / provenance links.

Identity Table

Human identities (canonical actors).

AI identities with constraints (Ents, helper models, sandbox-only roles).

Key-rotation, succession, freeze/unfreeze metadata.

Action Table

Actionable ideas (plans).

Action declarations and completion truth claims.

Linkage to verification challenges.

(Implementation note: “action declarations” and “completion truth claims” are canonical events and/or canonical ideas depending on the active rulebooks; conformant nodes MUST store them in a form that preserves deterministic interpretation during replay and snapshot application.)

Rulebook Table

All rulebook ideas.

Adoption actions and activation history per governance domain.

Safety Metadata

payload_class (normal, sensitive_abstracted, non_distributable_blocked).

Abstraction data and visibility flags.

Safety classifier versions and references.

POD / POINT Balance Table

POD balances and flows.

POINT emission and melt accounting.

Token invariants enforced according to active rulebooks.

Chain Anchors

Finalized prefix certificates and their canonical commitments.

Derived block references and snapshot anchor hashes, where exposed.

Sufficient verification material to validate that locally stored snapshots and post-snapshot events correspond to canonically anchored commitments under active rulebooks.

Growth Ring Metadata for Ents (Optional)

Ent training ring lineage.

Ring-snapshot associations.

Non-canonical content references (training corpora, model hashes, etc.).

Nodes MAY implement additional indexes or caches (e.g., materialized rank lists, search indexes) but these MUST NOT alter canonical semantics or observable replay results.


### 2.2 Canonical Record Ordering [anchor: canonical_record_ordering]

Canonical encoding rules also apply to verification-layer artifacts (custody manifests, reconciliation transcripts, and state witness attestations); see §8A and §8B.

Nodes MUST follow the ordering rules defined by Protocol v5:

Canonical Ordering
Canonical ordering of events is determined first by finalized prefix-certificate order. Where the public packaging surface is exposed, that order is represented as `(block_height, event_index)` in the canonical event log. Any event_id (UUIDv7) is an identifier only and MUST NOT be used as an ordering authority.

event_index
A monotonically increasing index assigned within each derived block after finalization.
This index is part of the public canonical address surface and MUST be consistent with finalized canonical sequence ordering during replay.

Block Boundaries & Snapshot Generation

Derived publication blocks group ranges of already-finalized events.

Snapshots occur at derived block boundaries according to rulebooks.

Rulebook activations, governance changes, and certain safety changes take effect at cycle boundaries, not per-event.

Deterministic Merges for Offline Forks
In the presence of offline creation or duplicated histories, the canonical universe MUST converge to a single linear history. Conformant nodes participate in this convergence according to their storage class:

Archival/full-history nodes MUST:

Merge all candidate events into a single sequence ordered by finalized canonical publication order, surfaced externally as `(block_height, event_index)` where derived blocks are exposed.

Re-apply validation and rulebooks as of the appropriate cycle boundaries.

Reject invalid events and produce a single linear canonical history.

Persist and serve the resulting linear history and the snapshot anchors derived from it, enabling discrepancy detection against other independent archival nodes.

Baseline-conformant nodes (non-archival) MAY:

Ingest a trusted snapshot and then ingest post-snapshot events in canonical publication order, surfaced externally as `(block_height, event_index)` where derived blocks are exposed, validating each event under the rulebooks active at the most recent snapshot boundary.

Verify that the snapshot they bootstrapped from corresponds to a canonical snapshot anchor, and verify continuity of subsequent block commitments and snapshot anchors they observe.

They are NOT required to perform full-network merges across all candidate events from genesis, provided the network maintains sufficient archival/full-history replication for such merges to be performed and audited.

This ensures convergence of all fully conformant nodes under deterministic replay, even if they temporarily diverge while offline.
Compatibility note: nodes MAY store UUIDv7s or other local event_ids for convenience (dedupe, local references), but replay/verification MUST ignore identifier ordering.

During merge, nodes MUST NOT perform implicit reconciliation, averaging, or tie-breaking; disagreement remains visible and challengeable (see §8D).


### 2.3 Event Envelope and Schema Requirements [anchor: event_envelope_and_schema_requirements]

Every ordinary human-authored canonical event MUST first exist as a signed authored candidate and then, after publication finality, as a published canonical event wrapper.

The signed authored candidate MUST include the canonical fields defined in `canonical-event-authorship-and-signature-profile-v0.md`, including:

- `signature_profile`
- `event_id`
- `event_type`
- `author_identity_id`
- optional `speaker_identity_id`
- `public_key_ref`
- `payload_hash`
- `payload` or `payload_ref`
- `payload_binding_mode`
- optional non-semantic `author_observed_at`
- `signature`

Deprecated field names MUST be interpreted only as historical aliases:

- `global_ulid` maps to `event_id`;
- `human_confirmation_proof` maps to `signature`;
- `signer_key_id` maps to `public_key_ref`;
- `human_author_id` maps to `author_identity_id`.

New normative envelope definitions MUST use the Appendix A canonical terms and MUST NOT introduce interchangeable names.

The published canonical event wrapper supplies publication-derived fields such as:

- `event_index`
- `block_height`
- parent prefix-certificate hash or chain reference
- derived block identifier
- finalized-prefix-certificate reference
- rulebook reference set
- payload classification

Those publication-derived fields MUST reflect finalized canonical publication ordering and MUST NOT be part of ordinary human-authorship signed bytes.

`author_identity_id`

MUST reference a valid, active human identity in replay-derived canonical identity state for ordinary human-authored events.

This is the canonical **pseudonymous** author of the event content (public by default). Civil identity disclosure MUST NOT be required for validity.

`speaker_identity_id` (optional)

Optional represented speaker identity attached to this event when the event schema permits or requires a speaker distinct from the author.

`signature`

A Profile-v0 Ed25519 signature proving that the referenced human explicitly confirmed the authored candidate.

MUST bind the authored candidate fields and `payload_hash` to `author_identity_id` according to `canonical-event-authorship-and-signature-profile-v0.md`.

`public_key_ref`

Hash32 key descriptor reference used to verify the event candidate signature.

MUST resolve to an active key owned by `author_identity_id` through replay-derived identity key state.

`payload_hash`

Cryptographic hash of the event payload (idea, connection, challenge, action, rulebook, etc.).

Used for prefix commitments, derived block anchoring, snapshot verification, and offline integrity checks.

payload_class

One of:

normal

sensitive_abstracted

non_distributable_blocked

Determined by safety rulebooks at ingestion (§3.2, Safety Spec).

rulebook_version (or structured rulebook reference set)

Identifiers of the rulebook versions that were active at the most recent snapshot preceding this event.

Used during replay to ensure events are validated under the correct governance and safety context.

ai_provenance (optional, non-canonical metadata)

MAY describe AI systems or Ents that contributed drafts or suggestions.

MUST NOT be treated as authorship.

MUST NOT affect event validity.

local_metadata (optional, implementation-specific)

Non-canonical hints (indexes, caching hints, UI tags).

MUST NOT affect replay semantics.

Validation Rule: Human-Authored Canonical Events Only

For every canonical event E:

E.author_persona_id, if present, is presentation-only and MUST NOT affect validity. Absence of a persona attachment MUST NOT be treated as invalid.

E.author_identity_id MUST correspond to a human identity, not an AI, Ent, or synthetic actor.

E.signature MUST be valid and MUST cryptographically bind all Profile-v0 signed fields defined in `canonical-event-authorship-and-signature-profile-v0.md`, including:

the event's payload_hash,

the event_id,

the author_identity_id,

the public_key_ref,

and any optional speaker, payload-reference, or author-observed fields present in the candidate.

The `public_key_ref` listed in E MUST resolve to an active key owned by author_identity_id according to replay-derived identity key state.

Nodes MUST reject any event that:

attributes authorship to a non-human identity,

lacks a valid `signature`,

has a `public_key_ref` that does not resolve to an active key owned by the claimed human identity,

or contains inconsistent or missing authorship information.

ai_provenance, if present, MUST appear only in non-canonical metadata fields and MUST NOT influence authorship validity checks.

Any node that accepts canonical events violating this rule is non-conformant.

Nodes MUST also validate:

Schema correctness against the active rulebook at the event's epoch.

Safety compatibility of payload_class with current rulebooks.

Chain continuity via parent_hash / chain_reference.

Referential integrity for referenced ideas, connections, identities, rulebooks, and snapshots (no forward references beyond sandbox or clearly declared “proposal-of-future” patterns defined in rulebooks).

Only events that pass all checks MAY be appended to the canonical event log or used in deterministic replay.

## 3. Event Validation Pipeline [anchor: 3_event_validation_pipeline]

A conformant node MUST validate every incoming canonical event through a strict multi-stage pipeline.
No event may enter the canonical log without passing all validation layers.

### 3.1 Submission Intake [anchor: submission_intake]

Upon receiving a new event (local submission or from the network), the node MUST perform the following steps in order:

Envelope & Schema Validation

Verify all required authored-candidate fields are present:
`event_id`, `event_type`, `author_identity_id`, `public_key_ref`, `payload_hash`, payload binding fields, and `signature`.

Publication-derived fields such as `event_index`, block height, finalized-prefix-certificate references, and chain reference fields are validated only when evaluating the publication wrapper. They MUST NOT be required to reconstruct human-authorship signed bytes.

Check that the payload structure conforms to the schemas defined by the active rulebooks at the last snapshot.

Signature Validation

Verify that `signature` is a valid Profile-v0 signature over the exact authored-candidate bytes from author_identity_id.

Verify that `public_key_ref` is an active replay-derived key reference for the human identity, respecting canonical key rotation, key revocation, and identity succession rules.

Reject any canonical event that:

is unsigned,

is signed only by AI keys,

or fails cryptographic verification.

Human Authorship Check

Confirm that author_identity_id is a human identity in the Identity Table.

If ai_provenance is present, ensure it is treated as metadata only and not as an author.

Enforce the “Human-Authored Canonical Events Only” rule from §2.3.

Safety Classification & Payload Class Assignment

Apply safety rulebooks and classifiers to the payload.

Assign payload_class as:

normal,

sensitive_abstracted, or

non_distributable_blocked.

Perform any required abstraction transformations, recording them in safety metadata.

Identity Reference Validation

Ensure all referenced identities exist and are valid at the time of the event.

Validate any identity succession, freeze/unfreeze, or posthumous flags that affect the event's validity.

Rulebook Reference Validation

Check the rulebook_version field (or equivalent reference set) matches the rulebooks that were active at the most recent snapshot boundary.

Reject events that attempt to apply future or non-existent rulebooks.

Timeline and Publication Validation

Ensure that event ordering, once finalized, matches the canonical publication order defined by finalized prefix certificates and any derived block mapping.

Check parent_hash or chain_reference against the node's current canonical publication view.

Reject events that would create inconsistent publication histories or violate canonical publication invariants.

Rate-Limit and Budget Checks

Apply rulebook-defined rate limits to:

event submissions,

challenge openings,

action declarations.

Enforce that autopilot-assisted events consume the human's action budgets.

Reject events that exceed rate limits as invalid canonical events.

Append to Candidate Store / Canonical Log

After intake validation succeeds, the node MAY store the event as a signed authored candidate.

The event enters the canonical log only after it is included in a valid finalized prefix certificate. At that point the node assigns or verifies the derived `(block_height, event_index)` address and updates internal indexes.

If any check fails, mark the submission as rejected and optionally record a local diagnostic log (non-canonical).

### 3.2 Safety Classification Enforcement [anchor: safety_classification_enforcement]

Nodes MUST enforce safety rulebooks (as defined in the Safety & Visibility Spec and referenced in Protocol §8) at event-ingestion time and during deterministic replay.

#### 3.2.1 classifier execution [anchor: classifier_execution]

Before accepting any event into the canonical chain, a node MUST:

1. Run the **active safety classifier version** referenced by the currently active safety rulebook.
2. Execute classification **locally**; nodes MUST NOT rely on external oracles or remote APIs for canonical classification.
3. Ensure classifier behavior is **deterministic** under replay, given the same:
   - classifier version,
   - model and configuration hashes,
   - rulebook references,
   - hardware architecture constraints.
4. Record all classifier-related metadata for each payload, including:
   - classifier_version,
   - model_hashes,
   - rulebook_version,
   - inference configuration,
   - explanation metadata (as required by the Safety Spec).

If a node cannot execute the classifier deterministically, it MUST NOT accept the event.

#### 3.2.2 payload_class assignment [anchor: payload_class_assignment]

Nodes MUST assign **payload_class** based on classifier output and rulebook-defined thresholds:

- **normal** - fully distributable and viewable.
- **sensitive_abstracted** - content is stored canonically but distributed only in abstracted form.
- **non_distributable_blocked** - original content cannot be legally or safely distributed; only minimal metadata and safe-summary information MAY be shared.

All payload_class assignments MUST be reproducible under replay.

#### 3.2.3 handling of sensitive_abstracted payloads [anchor: handling_of_sensitive_abstracted_payloads]

For payloads classified as **sensitive_abstracted**, nodes MUST:

1. Store the **original payload** locally in canonical storage, unless prohibited by global illegality rules.
2. Distribute only the **abstracted form** of the payload to peers and clients.
3. Attach rulebook references, classifier metadata, and abstraction identifiers.
4. Ensure that clients receive the correct safe-summary or abstracted text defined by the active rulebook.

Nodes MUST NOT distribute the unabstracted payload to any peer or client under any circumstances.

#### 3.2.4 handling of non_distributable_blocked payloads [anchor: handling_of_non_distributable_blocked_payloads]

For payloads classified as **non_distributable_blocked**, nodes MUST:

1. **NOT** distribute the original payload to any peers or clients.
2. Store only what the rulebook permits:
   - `payload_hash`,
   - classifier metadata,
   - minimal safe-summary or placeholder text,
   - submission context needed for audit and challenge.
3. Discard the original payload **if** global or jurisdictional law forbids storage.

Nodes MAY retain the original payload locally *only if* global legality rules allow storage but forbid distribution.

#### 3.2.5 blocked_submission event [anchor: blocked_submission_event]

When a submission is classified as non_distributable_blocked:

1. The node MUST emit a canonical `blocked_submission` event as defined by Appendix A, containing
   only safe accountability metadata:
   - reference to the submitting identity,
   - rulebook and classifier references,
   - safe-summary or placeholder text,
   - explanation pointer for "Why am I seeing this?" surfaces.
2. The blocked_submission event MUST appear in the canonical universe so that:
   - the existence of the attempted payload is visible,
   - users may challenge the classification,
   - history remains intact and auditable.
3. The event MUST NOT admit or distribute the blocked payload and MUST NOT mutate truth certainty,
   importance, governance authority, token authority, Tempo/Cycle certification, or the
   authorization frontier.

Nodes MUST NEVER silently drop dangerous content without recording a canonical trace.

#### 3.2.6 replay verification [anchor: replay_verification]

During deterministic replay:

1. Nodes MUST re-run the applicable classifier version for each historical payload.
2. Classification results MUST match the stored safety metadata exactly.
3. Any mismatch MUST be surfaced as:
   - a potential non-conformance issue, OR
   - grounds for a representation challenge.

Nodes MUST halt canonical synchronization if safety metadata cannot be reproduced exactly, unless a governance-approved migration or rulebook revision explicitly allows reclassification.

#### 3.2.7 anti-capture and substitution constraints [anchor: anti_capture_and_substitution_constraints]

Nodes MUST:

- execute **only** the classifier defined in the active safety rulebook,
- MUST NOT substitute local, altered, or proprietary classifiers,
- MUST NOT modify rulebook-defined specificity boundaries or abstraction rules,
- MUST NOT bypass safety logic by pre-filtering or altering payloads before classification.

Classifier definitions and updates are governed exclusively through the governance and rulebook processes.



### 3.3 Governance and Rulebook Checks [anchor: governance_and_rulebook_checks]

Nodes MUST reject events that violate:

Protocol Meta-Invariants

No event may:

disable challenges,

make identities or ideas unchallengeable,

render governance private,

disable evidence or verification requirements,

centralize irrevocable authority in a single identity or tribe,

remove universal importance.

Active Rulebooks

Challenge structure and lifecycle (opening, arguments, juror selection, verdict).

Voting parameters and quorum requirements.

Safety-handling rules and jurisdictional overlays (for distribution).

Token accounting rules for POD and POINT.

Governance adoption mechanics (rulebook activation conditions).

Identity Constraints

Human-first authorship requirements.

Identity freeze/unfreeze rules.

Succession and key rotation rules.

Prohibitions on AI-only canonical authorship.

Canonical Publication Invariants and Continuity Rules

Proper parent-hash or chain-reference continuity.

Consistency with the active canonical publication profile and finalized-prefix rules.

No introduction of illegitimate parallel chains as canonical.

Rate Limit Constraints

Events MUST respect rulebook-defined rate limits and budgeting rules.

Autopilot and AI helpers MUST NOT bypass human budgets.

Events failing any of these checks MUST NOT enter the canonical log and MUST be treated as invalid for replay.

### 3.4 AI Source Rules [anchor: ai_source_rules]

Nodes MUST enforce strict human-first boundaries between canonical and sandbox universes:

Canonical Events MUST be Human-Authored

`author_identity_id` MUST always be a human identity for ordinary human-authored events.

`signature` MUST always be present and valid under `canonical-event-authorship-and-signature-profile-v0.md`.

AI identities (including Ents) MAY appear in ai_provenance, but NEVER as authors.

AI-Assisted Drafts (Permitted)

AI systems MAY propose drafts, suggestions, or structures in:

the AI sandbox universe,

local helper workflows.

These drafts become canonical only when:

a human identity reviews them,

the human explicitly confirms them,

and a canonical event with that human's authorship and signature is recorded.

AI-Originated Canonical Events (Forbidden)
Nodes MUST reject any canonical event that:

is authored by an AI identity, Ent, or synthetic entity;

lacks a valid human-authorship `signature`;

attempts to treat ai_provenance as authorship;

or uses AI-signed keys as the only signer for the canonical event.

Sandbox Events (Non-Canonical)

Events in the AI sandbox are outside canonical replay.

Nodes MAY store, sync, or drop sandbox data as they choose, but MUST NOT mix sandbox events into canonical history.

Sandbox events MUST be clearly distinguished at storage and networking layers.

By enforcing these rules, nodes ensure that the canonical universe is always the expression of human-authored, human-confirmed reasoning, even when AI systems assist in drafting and analysis.


---

## 4. Snapshot Requirements [anchor: 4_snapshot_requirements]

### 4.1 Snapshot Generation [anchor: snapshot_generation]

Full canonical nodes MUST generate snapshots at the defined **derived block intervals**.

Other conformant nodes (including baseline-conformant nodes that do not retain full deterministic history) MAY generate snapshots, and if they do, they MUST follow the same deterministic snapshot rules for the state they claim to represent.

Each snapshot MUST contain:

- **Complete canonical state**, including:
  - all ideas
  - all connections
  - all descriptions
  - all challenge states
  - all importance ranking lists
  - all certainty bands

- **Identity balances**
  - POD and POINT totals according to active token rules.

- **Active rulebook set**
  - The rulebooks selected via adoption actions prior to this snapshot.

- **Growth rings for Ents**
  - Ent training lineage, snapshot-visible metadata (non-canonical content excluded).

- **Safety metadata**
  - Payload classes and abstraction markers.

Snapshots MUST be deterministic: two nodes with the same event log MUST produce identical snapshot hashes.

Readable State vs Deterministic History

This specification distinguishes between:

- **Readable state snapshots**, which represent the current live state of the canonical universe at a snapshot boundary, optimized for access and comprehension, and
- **Deterministic replay history**, which consists of the full ordered event log plus sufficient snapshot/delta material to reconstruct canonical state exactly from genesis.

Conformant nodes and clients are NOT required to store or distribute full deterministic history. A node MAY be conformant while retaining only a trusted snapshot plus subsequent events, provided it can verify correspondence to canonically anchored snapshot commitments and can validate all events it does ingest under the rulebooks active at the relevant cycle boundaries.

Private drafts that have not been published as canonical events are out of protocol conformance scope and SHALL NOT be required snapshot contents; drafts become ideas only when published into the canonical universe as canonical events.

State witness attestations and reconciliation artifacts are validated only for syntax, signature, and canonical encoding, and MUST NOT affect canonical ordering, eligibility, or semantics (see §8C).

---

### 4.2 Snapshot Anchoring [anchor: snapshot_anchoring]

Nodes MUST:

- bind each canonical snapshot hash to the corresponding finalized canonical sequence boundary and any derived block header exposed for that boundary,
- verify snapshot continuity when syncing from peers,
- reject peers whose snapshots do not match expected hashes.

Anchor hashes guarantee:

- replay correctness,
- finalized-prefix continuity,
- offline reconciliation integrity.

Conformance Note on Anchoring Without Full History

A baseline-conformant node that does not retain full deterministic history MUST still be able to verify that any snapshot it imports corresponds to a canonically anchored snapshot hash, and MUST be able to verify continuity for any later finalized prefixes, derived blocks, and snapshots it accepts. It is not required to possess earlier history to perform this verification, provided it possesses (or can obtain) the relevant anchor references and verification material required by the active rulebooks.

---

### 4.3 Snapshot Structure [anchor: snapshot_structure]

A snapshot MUST contain:

- **Canonical state root hash**
- **Complete idea/connection/challenge/action structures**
- **Rulebook versions active at this boundary**
- **Safety classifier versions**
- **Token-rule versions**
- **Identity state, including POD/POINT balances**
- **Ent ring metadata relevant at snapshot time**

Snapshots MUST be self-sufficient:
A node should be able to import a snapshot + subsequent events to reconstruct full state.

Readable State Snapshot Distribution

Readable state snapshots MAY be distributed independently of full deterministic history, provided they include cryptographic anchors sufficient to verify correspondence to a canonical snapshot. Nodes MAY choose to distribute only readable snapshots (and required verification material) to reduce bandwidth and storage requirements, without altering canonical semantics.

---

### 4.4 Snapshot Import [anchor: snapshot_import]

Nodes MUST allow importing:

- **Remote snapshots**
- **Tribe snapshots**
- **Offline snapshots**

Import is only allowed after:

- Full rulebook verification
- Identity reference validation
- Safety and jurisdictional checks
- Hash equality validation (snapshot hash must match expected lineage)

If a snapshot is valid:

- It becomes the new base state for replay.
- Subsequent events are replayed deterministically on top.

If a snapshot fails validation:

- It MUST be rejected.
- The node MUST fall back to local snapshots or trusted peer snapshots.

Archival Compatibility

When importing an offline snapshot, a node MUST treat it as non-authoritative unless its hash corresponds to a canonically anchored snapshot in the finalized canonical publication lineage. Offline snapshots MAY still be imported as local readable state for analysis or proposal development, but SHALL NOT be treated as canonical unless and until the corresponding canonical events and anchored snapshot lineage exist in the global event log and finalized publication history.


### 4.4 Snapshot Import [anchor: snapshot_import_2]

Nodes MUST allow importing:

- **Remote snapshots**
- **Tribe snapshots**
- **Offline snapshots**

Import is only allowed after:

- Full rulebook verification
- Identity reference validation
- Safety and jurisdictional checks
- Hash equality validation (snapshot hash must match expected lineage)

If a snapshot is valid:

- It becomes the new base state for replay.
- Subsequent events are replayed deterministically on top.

If a snapshot fails validation:

- It MUST be rejected.
- The node MUST fall back to local snapshots or trusted peer snapshots.

---

## 5. Deterministic Replay System [anchor: 5_deterministic_replay_system]

### 5.1 Replay Goals [anchor: replay_goals]

Deterministic replay is the core integrity mechanism of The Seed in My Mind.
A conformant node MUST guarantee that:

- **Any two conformant nodes reconstruct identical canonical state** from the same event log and snapshots.
- **Snapshots stabilize governance, safety, and token rules** at fixed boundaries during replay.
- **No divergence occurs**, except:
  - sandbox-only operations (non-canonical),
  - non-conformant forks (which do not count as the canonical universe).

Replay MUST be bit-for-bit deterministic in all normative state transitions:
- truth-system updates (certainty bands),
- importance rankings,
- rulebook activation,
- token accounting,
- authorship & identity structure,
- visibility/safety classifications.

---

### 5.2 Replay Algorithm Requirements [anchor: replay_algorithm_requirements]

A conformant node MUST:

1. **Apply events strictly in finalized canonical order, surfaced externally as `(block_height, event_index)` where derived blocks are exposed**
   Event_id values (UUIDv7) are identifiers only and MUST NOT affect replay ordering.

2. **Recompute certainty, importance, and rankings exactly**
   - Re-run all successful challenge verdicts.
   - Apply the bubble-up rule for importance challenges.
   - Apply certainty-band changes from truth challenges.
   - Apply representation merges or equivalence results.

3. **Recompute POD flows**
   POD derivation from importance rankings MUST be deterministic and rulebook-governed. POD effects remain provisional or pending until the relevant cycle is authorized by certification and the lagged authorization frontier.

4. **Recompute POINT emissions**
   Using the active token rulebook at each structural cycle or snapshot boundary. POINT mint, melt, distribution, or spendability effects remain pending or blocked until the relevant cycle is authorized by certification and the lagged authorization frontier.

5. **Apply safety classifications deterministically**
   Payload classification and abstraction MUST yield the same results across nodes.

6. **Apply rulebook logic exactly as active at the time of each event**
   - Rulebooks are activated via adoption actions and only at cycle boundaries.
   - Replay MUST use the rulebook version active at the moment the event originally occurred.
   - This ensures stable interpretation even if governance later modifies rulebook behavior.

7. **Resolve merges deterministically**
   Representation challenges may create same_as relationships that collapse ideas into canonical equivalents; nodes MUST derive identical results.

No implementation-specific heuristics may alter replay logic.

---

### 5.2A Tempo/Cycle Conformance Hooks [anchor: tempo_cycle_conformance_hooks]

Conformant nodes MUST implement the Appendix A Tempo/Cycle schemas and preserve the following boundaries:

- AI identities, non-human actors, local automation, publication participants, or node operators MUST NOT become canonical Tempo contributors, authors of canonical Tempo claims/evidence, beacon supporters, cycle authorities, challengers, voters, verdict finalizers, or governance actors.
- Low-threshold `tempo_contributor` eligibility permits only target-bound ordinary `truth_claim` creation with valid `tempo_claim` metadata and explicitly allowed Tempo-context evidence ideas/connections. It does not grant ordinary canonical write access, evidence creation outside Tempo context, connection creation outside Tempo context, challenge opening, challenge voting, verdict finalization, governance, POD, POINT, token, or ordinary mana authority.
- Time-related challenge creation, challenge voting, and verdict finalization remain governed by the ordinary challenge eligibility rules unless a later authoritative rulebook explicitly defines a separate `tempo_challenger` capability.
- Nodes MUST NOT admit hidden trusted-clock inputs into Tempo truth certainty or structural support. Node-local clocks, server time, client timestamps, receipt time, block height, scheduler observations, local uncommitted observations, AI-generated observations, publication volume, or availability timing have no Tempo effect unless represented as valid canonical ideas, canonical connections, or profile-admitted passive evidence inputs under explicit protocol rules.
- Nodes MUST deterministically reproduce derived Dmin/Dmax target keys, immutable Tempo profile references, target-level truth certainty, human structural stances, passive evidence aggregation, structural-support state, Dmin/Dmax predicates, derived beacon states, structural cycle boundaries, cycle certification, the lagged authorization frontier, downstream output status, and normal/constrained/record-only modes.
- Nodes MUST deterministically reproduce `structural_dmax_liveness_predicate` as a Dmax-only forced-closure predicate derived from nonzero eligible-human survivor support, valid target-bound Dmax claims or stances, required capped passive plausibility evidence, constrained/time-repair conditions, and blocker checks. Nodes MUST reject any use of that predicate for Dmin, deliberative closure, beacon status, certification, authorization-frontier advancement, or consequential authority.
- Nodes MUST reject invalid Tempo-context `evidence_for`, `evidence_against`, or `same_as` connections with `ERR_TEMPO_EVIDENCE_CONNECTION_INVALID` when endpoints, target-bound time-claim metadata, compatibility, provenance-as-idea requirements, or ordinary evidence-placement/certainty flow requirements are not satisfied.
- Nodes MUST reject any interpretation that treats forced structural cycles as consequential authority, accumulated legitimacy, payout authorization, governance activation, lifecycle finalization, final rank authority, ordinary mana authority, or rate-limit reset authority.
- Nodes MUST reject any automatic reduction of `K`, `T_beacon`, beacon diversity, independence, or stability requirements under population collapse.
- Nodes MUST reject backfilled ordinary mana, ordinary rate-limit, POD, POINT, token, or governance effects from uncertified or previously constrained cycles.
- Nodes MUST verify that Tempo truth certainty comes from explicit idea, connection, challenge, vote, and verdict history. Nodes MUST also verify that `T_allow` structural support is separate from truth certainty and comes only from current eligible-human stances plus profile-capped passive evidence. Verification, identity certainty, provider, institution, jurisdiction, wealth, POD, POINT, reputation, role, model, or operator status MUST NOT multiply influence.
- Nodes MUST expose deterministic rejection causes that map to Appendix A Tempo/Cycle rejection codes for conformance testing, without requiring any specific runtime exception class or API error format.

---

### 5.3 AI Activities Ignored [anchor: ai_activities_ignored]

Replay MUST explicitly **exclude**:

- **Sandbox events**
  AI map events are never canonical and MUST NOT influence replay.

- **AI-only interactions**
  Drafts, autonomous behaviors, speculative graph edits are ignored.

- **Non-canonical drafts**
  All drafts-AI or human-remain invisible unless submitted as canonical events.

- **Helper drafts not submitted by humans**
  If AI generates a draft and the human does not sign it, replay MUST NOT include it.

Replay strictly reconstructs the **human-authored canonical universe**.

---

### 5.4 Rulebook Versioning During Replay [anchor: rulebook_versioning_during_replay]

Nodes MUST interpret historical events using **the rulebooks active at that canonical replay position**, not the rulebooks active now.

- Every event stores a `rulebook_version` reference.
- Snapshots commit rulebook activation state for deterministic verification at discrete derived block boundaries.
- Replay MUST restore the exact rulebooks active at each historical replay boundary.

This ensures:

- governance changes do not retroactively rewrite history,
- identical replay across node implementations,
- deterministic interpretation of challenges and safety rules.

---

## 6. Canonical Publication Profiles [anchor: 6_proof_of_deliberation_chain_integration]

### 6.1 Profile-aware publication [anchor: pod_block_formation]

Full canonical nodes MUST integrate with the staged canonical publication system as follows:

- **Validate authored events before finality**
  Nodes validate authored candidate bytes, Profile-v0 human-authorship signatures, replay-derived key state, rulebook compatibility, and safety classification before treating an event as a candidate for canonical publication.

- **Follow the active publication profile**
  Canonical publication MUST follow the active profile defined in `pod-consensus-and-canonical-publication-spec.md`:
  - Profile 0: singleton publisher finality,
  - Profile 1: singleton publisher plus availability attestations,
  - Profile 2: committee-signed prefix finality.

- **Derive blocks only after finality**
  Where blocks are exposed, nodes MUST derive them from the already-finalized canonical sequence. Blocks are not the authority source for order.

- **Include snapshot anchors at derived boundaries**
  Snapshot generation MUST occur at deterministic derived block heights.

Canonical publication is an ordering-and-availability system, not an authority system for meaning.

---

### 6.2 Publication validation rules [anchor: block_validation_rules]

When ingesting canonical publication artifacts, nodes MUST validate:

- **Prefix certificate signatures**
  Signatures MUST originate from valid human identities eligible under the active publication profile.

- **Committee / publisher derivation**
  The signer set MUST match the active singleton publisher or deterministic committee rules for the parent prefix.

- **Availability requirements**
  Events included in a profile that requires availability certification MUST carry the required attestation support.

- **Ready-frontier commitments**
  The committed ready frontier MUST match the deterministically recomputed ready set for the parent prefix and provided candidate materials.

- **Deterministic event ordering**
  The extension events MUST match the deterministic ordering rules defined by the canonical publication spec.

- **Snapshot anchors**
  Any snapshot hash associated with the finalized sequence or exposed derived block header MUST match the computed canonical snapshot hash.

If a publication artifact fails any validation rule, it MUST be rejected.

---

### 6.3 No AI in Publication Roles [anchor: no_ai_in_pod_roles]

Nodes MUST enforce the human-first constraint in canonical publication:

- **AI identities cannot be singleton publishers**
- **AI identities cannot sign availability attestations**
- **AI identities cannot be committee signers**
- **AI-assisted publication tools must be human-confirmed**

Any publication artifact produced or finalized by an AI-only identity is invalid and MUST be rejected.

---

### 6.4 Conflicting Finality and Stall Behavior [anchor: failed_forks]

Nodes MUST classify publication conflicts as follows:

- A branch that **violates rulebooks**, **breaks replay determinism**, or **introduces non-human-authored canonical events** becomes a **non-conformant universe**, not a canonical branch.

- A branch that presents **conflicting finalized prefix certificates** for the same parent is a safety breach. Nodes MUST:
  - preserve the conflicting certificates and signatures,
  - halt canonical advancement at the last uncontested ancestor,
  - reject automatic longest-chain selection,
  - and continue syncing only on uncontested finalized history until recovery is defined.

No conformant node may diverge from canonical replay outcomes.
Any divergence is treated as a **different system**, not a competing canonical universe.

---

## 7. Identity and Authorization Requirements [anchor: 7_identity_and_authorization_requirements]

### 7.1 Human Identity Anchoring [anchor: human_identity_anchoring]

All canonical activity in the universe MUST ultimately be attributable to a **real human identity**.
Nodes therefore MUST enforce:

- **Proof-of-humanity requirements** defined in the active identity rulebooks.
- **That every canonical event includes a valid human authorship reference**, even if the event was AI-assisted.
- **That no event authored solely by an AI identity enters the canonical event log.**

A conformant node MUST reject any event that:

- originates from an AI-only identity,
- lacks a valid human-authorship `signature`,
- references an identity whose proof-of-humanity is missing, expired, or invalid.

Identity anchoring ensures the canonical universe remains grounded in human deliberation.

---

### 7.2 Autopilot Transparency [anchor: autopilot_transparency]

Autopilot mode allows AI helpers to perform limited epistemic actions **under the authority of a human identity**.
Nodes MUST record this distinction explicitly.

For any event generated with autopilot assistance, nodes MUST:

- Mark the event as **autopilot-assisted** in event metadata.
- Record the **responsible human identity** who provided confirmation.
- Preserve this metadata during snapshots and replay.

During deterministic replay, autopilot metadata MUST NOT affect canonical outcomes, but MUST remain visible for audit, safety, and transparency requirements.

---

### 7.3 Identity Continuity [anchor: identity_continuity]

Identity is a long-lived structure. Nodes MUST support the following continuity operations:

- **Succession events**
  Allow an identity to rotate keys, recover from compromise, or migrate devices without losing historical authorship.

- **Identity freeze / unfreeze**
  As defined by rulebooks, an identity MAY be frozen (e.g., due to suspected compromise) and later unfrozen by proper challenge or verification events.

- **Posthumous identity preservation**
  Human identities do not disappear on death. Their authored ideas, actions, and contributions remain part of the canonical universe forever.
  A conformant node:
  - MUST NOT overwrite or delete identity history,
  - MUST preserve POD/POINT balances at time of death,
  - MUST apply any rulebook-defined behaviors for inheritance or stewardship of dormant identities.

Identity continuity ensures the universe remains historically faithful and resistant to capture, impersonation, or revisionism.

---

## 8. Safety Rules Enforcement [anchor: 8_safety_rules_enforcement]

### 8.1 Payload Classes [anchor: payload_classes]

Nodes MUST apply safety-classification rulebooks to all incoming content and assign each canonical event one of:

- **normal** - fully distributable, no abstraction required
- **sensitive_abstracted** - content must be stored but abstracted according to rulebook-defined transformations
- **non_distributable_blocked** - content must be stored in metadata-only form (e.g., hash, classifier output) but full payload MUST NOT be distributed

This classification MUST occur *prior* to accepting an event into the canonical log.

Rules for classification, abstraction, and blocking are defined entirely by **active safety rulebooks** at the current snapshot boundary.

---

### 8.2 Blocked Submission Events [anchor: blocked_submission_events]

Nodes MUST:

- Create an explicit Appendix A `blocked_submission` event when dangerous or non-distributable payloads are submitted.
- **Never silently drop** such content.
- Store:
  - event ID and any non-semantic local receipt metadata,
  - human identity attempting submission,
  - classifier reasoning (abstracted as required),
  - rulebook references justifying the block.

This ensures the event log remains historically complete while preventing unsafe content distribution.

---

### 8.3 "Why Am I Seeing This?" Support [anchor: why_am_i_seeing_this_support]

Nodes MUST support the protocol's visibility-explanation requirements.

For any hidden, abstracted, or omitted content-whether due to:

- safety rules,
- jurisdictional overlays,
- personal visibility constraints,

nodes MUST attach or expose:

- **rulebook references** that triggered the visibility decision,
- **safety classifier metadata**,
- **explanation pointers** to the decision lineage (e.g., which rulebook clause, which classifier version).

This ensures transparency of safety decisions and supports governance oversight, audits, and long-term epistemic accountability.

---

### 8.4 Hard Floor Enforcement [anchor: hard_floor_enforcement]

“Hard floors” are safety boundaries that **no rulebook, idea, or adoption action may cross**.

Nodes MUST enforce:

- When a payload is classified as **non_distributable_blocked**,
  that classification CANNOT be overridden except through governance-defined mechanisms that modify the classifier or its criteria at future snapshots.

- Hard floors MUST apply *only* through governance-controlled rulebooks.
  No local configuration, no operator judgment, and no external legal or platform pressure may bypass rulebook-defined safety hard floors.

This ensures:

- deterministic enforcement across all conformant nodes,
- immunity to platform-level editorial capture,
- transparent governance of safety boundaries.


## **8A. Custody Manifest Canonical Encoding and Hashing** [anchor: 8a_custody_manifest_canonical_encoding_and_hashing]

### **8A.1 Purpose** [anchor: 8a_1_purpose]

This section defines conformance requirements for the canonical encoding and hashing of **custody manifests**, as used in P2P state verification and offline replication workflows.

All conformant nodes MUST produce identical hashes for semantically identical custody manifests.

---

### **8A.2 Canonical Encoding Requirements** [anchor: 8a_2_canonical_encoding_requirements]

A conformant node MUST ensure that custody manifests:

* use a single, deterministic serialization format,
* normalize field ordering,
* normalize numeric and boolean representations,
* exclude non-semantic whitespace,
* exclude local-only metadata (timestamps, device identifiers, paths).

Semantically identical manifests MUST serialize to byte-identical representations.

---

### **8A.3 Hashing Rules** [anchor: 8a_3_hashing_rules]

The manifest hash MUST:

* be computed over the canonical serialized form,
* use the system-standard cryptographic hash function,
* be stable across platforms and implementations.

Any deviation in encoding or hashing MUST be treated as a conformance failure.

---

### **8A.4 Conformance Fixtures** [anchor: 8a_4_conformance_fixtures]

A conformant implementation MUST pass fixtures that include:

* minimal manifests,
* manifests with partial log ranges,
* manifests with PCS-only claims,
* manifests with PCS + CCS claims,
* manifests containing optional fields omitted or reordered.

Each fixture MUST specify:

* input structure,
* canonical serialized bytes,
* expected hash.

---

## **8B. Reconciliation Transcript Determinism** [anchor: 8b_reconciliation_transcript_determinism]

### **8B.1 Purpose** [anchor: 8b_1_purpose]

This section defines conformance requirements for **deterministic reconciliation transcripts** generated during P2P state comparison and exchange.

A reconciliation transcript records *what was compared and observed*, not what was decided.

---

### **8B.2 Transcript Definition** [anchor: 8b_2_transcript_definition]

A reconciliation transcript MAY include:

* exchanged custody manifest hashes,
* declared log ranges,
* detected overlaps and gaps,
* verification outcomes (match / divergence),
* optional witness references.

The transcript MUST be a deterministic artifact derived solely from exchanged inputs.

---

### **8B.3 Determinism Requirements** [anchor: 8b_3_determinism_requirements]

Given identical inputs, a conformant node MUST:

* produce an identical reconciliation transcript,
* serialize it identically,
* compute identical transcript hashes.

Ordering of observations MUST be deterministic and stable.

---

### **8B.4 Conformance Fixtures** [anchor: 8b_4_conformance_fixtures]

A conformant implementation MUST pass fixtures covering:

* identical manifests (perfect match),
* partial overlap,
* complete divergence,
* asymmetric knowledge (one side has more data),
* optional fields present vs absent.

Fixtures MUST assert byte-identical transcripts and hashes across implementations.

---

## **8C. State Witness Receipt Verification** [anchor: 8c_state_witness_receipt_verification]

### **8C.1 Purpose** [anchor: 8c_1_purpose]

This section defines verification requirements for **P2P state witness attestations** and related receipt artifacts.

Receipt verification ensures that attestations are **syntactically valid and cryptographically authentic**, without conferring authority.

---

### **8C.2 Verification Rules** [anchor: 8c_2_verification_rules]

A conformant node MUST verify that each receipt:

* references a valid manifest hash,
* includes required fields,
* carries a valid signature from the attesting identity,
* binds attestation scope correctly,
* is canonically serializable.

Invalid receipts MUST be rejected deterministically.

---

### **8C.3 Replay Stability** [anchor: 8c_3_replay_stability]

Receipt verification MUST:

* produce identical results under replay,
* be independent of wall-clock time,
* remain valid even if referenced data is later unavailable.

---

### **8C.4 Conformance Fixtures** [anchor: 8c_4_conformance_fixtures]

A conformant implementation MUST include fixtures for:

* valid receipts,
* malformed receipts,
* incorrect signatures,
* mismatched manifest hashes,
* duplicate or replayed receipts.

Each fixture MUST assert a deterministic accept/reject result.

---

## **8D. Partitioned Merge Behavior** [anchor: 8d_partitioned_merge_behavior]

### **8D.1 Purpose** [anchor: 8d_1_purpose]

This section defines conformance requirements for canonical ingestion when **offline partitions publish overlapping or conflicting histories**.

The goal is to ensure deterministic preservation of disagreement.

---

### **8D.2 Required Merge Behavior** [anchor: 8d_2_required_merge_behavior]

When ingesting partitioned publications, a conformant node MUST:

* ingest all valid events,
* preserve conflicting claims explicitly,
* avoid implicit resolution or ordering changes,
* surface conflicts as challengeable state.

Silent reconciliation is strictly forbidden.

---

### **8D.3 Determinism Requirements** [anchor: 8d_3_determinism_requirements]

Given identical publication inputs, a conformant node MUST:

* produce identical merged canonical state,
* assign identical identifiers,
* surface identical conflict sets.

Merge outcomes MUST NOT depend on ingestion order.

---

### **8D.4 Conformance Fixtures** [anchor: 8d_4_conformance_fixtures]

A conformant implementation MUST pass fixtures including:

* two partitions with overlapping but non-conflicting events,
* two partitions with conflicting votes,
* partitions advancing cycles differently,
* delayed publication of older events,
* multi-partition fan-in merges.

Fixtures MUST assert identical canonical outcomes across implementations.


## 9. Token Accounting on Nodes [anchor: 9_token_accounting_on_nodes]

### 9.1 POD Calculations [anchor: pod_calculations]

Proof-of-Deliberation (POD) represents **epistemic significance**, not value, not currency.
Nodes MUST compute POD exactly as defined by the active token rulebooks at each structural cycle and snapshot boundary, but consequential POD effects remain provisional or pending until cycle certification and the lagged authorization frontier authorize them.

A conformant node MUST:

- **Recompute POD flows each cycle**
  Using:
  - universal-importance rankings,
  - relative_importance pathways,
  - rulebook-defined POD propagation rules.

- **Enforce downward POD routing**
  POD flows from:
  - universally important ideas
  -> intermediary ideas
  -> actionable ideas
  -> actions
  -> identities.

  Nodes MUST ensure POD never flows *upward* or sideways except through rulebook-defined transitional edges.

**POD Routing Constraints**

Nodes MUST:

Respect the rulebook-defined POD routing rules,

Use the relative_importance graph (with its usage metadata and axis/timeframe/scope) as the only permitted network for POD flow,

Ignore or reject any routing attempt that relies on:

invalid or disallowed connections,

sandbox-only ideas,

unmerged representation-duplicate ideas,

identity ideas as intermediate nodes (identities may only receive POD, not route it).

POD computation MUST be fully deterministic.
Any two conformant nodes replaying the same canonical universe MUST derive identical POD distributions and identical provisional, pending, authorized, or blocked POD status.

---

### 9.2 POINT Emission and Melt [anchor: point_emission_and_melt]

A conformant node MUST implement POINT tokens exactly as defined in rulebooks.

Nodes MUST:

- **Apply melt rules**
  Melt reduces POINT supply based on:
  - time decay,
  - inactivity,
  - misalignment corrections,
  - or other governance-defined factors.

- **Apply cycle-based emission rules**
  At each structural cycle or snapshot boundary, identities holding POD derive pending POINT outputs according to:
  - emission rate,
  - holding-to-yield curve,
  - governance parameters.

  Derived POINT outputs MUST NOT become authorized, spendable, melt-effective, or distributable until the lagged authorization frontier authorizes the relevant cycle.

- **Enforce token invariants**
  - **POD is non-transferable** under all circumstances.
    Attempts to transfer POD MUST be rejected.
  - **POINT does not influence governance.**
    POINT cannot:
    - weight votes,
    - grant special powers,
    - override human identities.

Nodes MUST also record POINT balances and pending/authorized status at each snapshot to support deterministic replay. Forced cycles, constrained operation, or later certification MUST NOT create burst capacity, backfilled ordinary rate-limit resets, or token value extraction for actions that were forbidden when attempted.

---

### 9.3 Identity-Level Storage [anchor: identity_level_storage]

Token storage rules are strict:

- **Tokens belong only to identities**
  Never to ideas, actions, tribes, challenges, or rulebooks.

- **Nodes MUST maintain for each identity**:
  - current POD balance,
  - current POINT balance,
  - melt counters,
  - token-related metadata snapshots.

- **Tribes never hold tokens.**
  Tribes influence importance, not token flows.

Any violation of identity-only token storage renders a node non-conformant.

---

## 10. Rulebook Enforcement [anchor: 10_rulebook_enforcement]

### 10.1 Rulebook Versions [anchor: rulebook_versions]

Conformant nodes MUST:

- **Store every historical rulebook version**, including superseded ones.
- **Apply the correct version based on canonical replay position and scheduled cycle activation**
  Rulebooks activate only via adoption actions, and only at cycle boundaries.
- **Record rulebook lineage**
  Nodes must maintain:
  - predecessor,
  - successor,
  - version identifiers,
  - adoption action references.

A node replaying history MUST reconstruct the active rulebook versions at each replay position exactly. Where derived block heights are exposed, block height is only a packaging/address coordinate for the already-finalized canonical sequence and MUST NOT substitute for cycle, Tempo, or governance authority.

---

### 10.2 Protocol Invariants [anchor: protocol_invariants]

Certain invariants are non-negotiable and MUST be enforced regardless of rulebook content:

- **Human primacy**
  Only humans can author canonical events; AI cannot vote, propose rulebooks, mint tokens, shape governance, contribute canonical Tempo authority, or support beacon elevation.

- **POD non-transferability**
  POD cannot be moved, sold, staked, or delegated.

- **Canonical/sandbox separation**
  Sandbox ideas never influence canonical truth, importance, or token flow.

- **Deterministic replay**
  Any node whose interpretation diverges from deterministic replay is non-conformant.

Nondeterministic encoding, hashing, reconciliation, or merge behavior for verification-layer artifacts is a hard conformance failure even if replay otherwise succeeds.

Nodes MUST reject any rulebook that attempts to violate these invariants.

---

### 10.3 Rulebook Activation Events [anchor: rulebook_activation_events]

Nodes MUST enforce the entire rulebook adoption lifecycle:

- **Validate proposals**
  Adoption actions must:
  - be authored by a human,
  - reference a valid rulebook for a governance domain,
  - satisfy rulebook-defined prerequisites.

- **Enforce delays / quorum**
  If rulebooks specify:
  - minimum deliberation periods,
  - quorum counts,
  - multi-stage reviews,
  nodes MUST enforce these before marking the adoption action complete.

- **Record rulebook lineage**
  Upon adoption:
  - mark referenced rulebook as active starting at its scheduled activation cycle boundary,
  - mark previous rulebook as superseded,
  - store lineage links for replay.

Only completed adoption actions can activate new rulebooks.

---

## 11. Sandbox Handling & AI Boundaries [anchor: 11_sandbox_handling_ai_boundaries]

### 11.1 Sandbox Universe [anchor: sandbox_universe]

Nodes MAY store or ignore sandbox (AI-only) data, but MUST enforce strict boundaries between the sandbox universe and the canonical universe.

A conformant node MUST:

- **NEVER treat sandbox data as canonical.**
  Sandbox ideas, challenges, actions, connections, or drafts MUST NOT enter the canonical event log.

- **NEVER allow sandbox events into deterministic replay.**
  Replay covers only human-authored canonical events. Sandbox content MUST be ignored completely during replay.

- **NEVER allow sandbox outputs to bypass human confirmation.**
  Any draft generated by an AI or Ent MUST:
  - remain non-canonical until signed/submitted by a human,
  - be rejected if submitted without human authorship.

Nodes MUST provide an explicit boundary object or namespace ensuring sandbox content cannot be mistaken for canonical content.

---

### 11.2 Helper Drafts [anchor: helper_drafts]

Nodes MUST distinguish sharply between:

- **Canonical human-submitted events**
  Events signed by a human identity that enter the canonical log.

- **AI-generated drafts**
  These MAY:
  - appear in UI,
  - be stored locally,
  - serve as suggestions or scaffolding,
  - feed sandbox training,
  BUT MUST NOT:
  - appear in the canonical event log,
  - be treated as submissions,
  - influence truth, importance, certainty, or tokens.

Every ordinary human-authored canonical event candidate must explicitly contain:
- `author_identity_id`
- `signature`
- `public_key_ref`
- `ai_assist_flag` (optional)
- `ai_model_reference` (optional)

Drafts lacking human confirmation MUST be ignored during canonical ingestion and replay.

---

### 11.3 Ent Information [anchor: ent_information]

Nodes that support Ents MUST store:

- **Growth rings** - snapshot-indexed training datasets packaged for Ent training.
- **Ring metadata** - version, timestamp, training lineage, classifier/safety overlays.
- **Ring activation events** - canonical events indicating which ring becomes visible to which Ent identity.

Nodes MAY provide enhanced APIs for Ent-tethered clients, but MUST NOT:

- allow Ents to author canonical events,
- allow Ents to bypass human-first rulebooks,
- let Ent actions influence POD, POINT, or governance.

Ent information is always **non-canonical metadata**.

---

## 12. Networking & Replication [anchor: 12_networking_replication]

### 12.1 Gossip / Receive Rules [anchor: gossip_receive_rules]

Nodes participating in replication MUST enforce strict validation rules when receiving data from peers.

A conformant node MUST:

- **Reject invalid snapshots**
  - Snapshots whose hash doesn't match computed canonical state,
  - Snapshots referencing impossible rulebook versions,
  - Snapshots with missing metadata.

- **Reject non-conformant forks**
  - Any chain diverging due to broken replay,
  - Any universe violating protocol invariants,
  - Any fork introducing unauthorized governance behavior.

- **Reject AI-signed canonical events**
  - AI-only signatures are invalid,
  - Events missing human-authorship MUST be dropped,
  - Blocks signed by AI identities MUST be rejected.

Nodes MAY use any networking protocol, but canonical validation MUST follow these rules.

---

### 12.2 Bandwidth Optimization [anchor: bandwidth_optimization]

Bandwidth and storage optimizations are **allowed** if they do not alter canonical semantics.

Implementations MAY:

- **Prune sandbox data**
  Sandbox content is optional and may be discarded without affecting canonical correctness.

- **Compress snapshots**
  Compression is allowed as long as decompression yields a byte-for-byte accurate canonical state.

- **Use delta synchronization**
  Nodes may sync:
  - incremental deltas between snapshots,
  - partial segments of the chain,
  - compressed historical archives,
  as long as reconstruction yields full canonical state.

Caching, indexing, and pruning strategies MUST NOT modify canonical event ordering or rulebook logic.

---

### 12.3 Tribe Replication [anchor: tribe_replication]

Tribal nodes operate as semi-autonomous local contexts but MUST remain tethered to canonical data.

A tribal node MAY:

- **Publish internal importance maps**
  Tribe-level significance maps are public and replicable.

- **Publish tribal rulebooks**
  These rulebooks apply only within the tribe and do not affect universal governance unless escalated through canonical adoption actions.

- **Sync canonical data from full nodes**
  Tribal nodes MUST:
  - import canonical snapshots,
  - import canonical events,
  - respect rulebook activation boundaries,
  - reject non-conformant forks.

Tribal nodes are **not** full canonical nodes.
They are application nodes with public-readable internal maps, relying on full nodes for authoritative chain state.

---

### 12.4 Operator Metadata Minimization and Privacy Declarations [anchor: operator_metadata_minimization_and_privacy_declarations]

Nodes MUST enforce the following privacy and metadata constraints for networking and submission handling:

- Canonical artifacts MUST NOT include network identifiers (e.g., IP addresses, routing metadata, transport headers, or device fingerprints).
- Canonical artifacts and operational logs MUST remain separate; operational logs are non-canonical.
- Default behavior for public-service nodes SHOULD minimize retention of correlating operational metadata; if retained for abuse control, retention MUST be explicitly bounded and documented.
- Public-service nodes MUST publish a metadata retention policy and MUST declare supported privacy capabilities, including confidential transport availability, indirect-routing compatibility, and minimal logging mode.
- Replay/conformance logging requirements apply to canonical and conformance-relevant artifacts and do not require retention of network-identifier operational logs.

Nodes SHOULD expose these declarations via an implementation-defined public node-info surface with fields such as:

- `supported_privacy_profiles`
- `confidential_transport_supported`
- `indirect_routing_supported`
- `metadata_retention_policy_url_or_text`
- `minimal_logging_mode_default`

For profile definitions and declaration requirements, see `privacy-and-high-risk-submission-spec.md` Sections 7-9.

---

## 13. Conformance Testing & Compliance Levels [anchor: 13_conformance_testing_compliance_levels]

### 13.1 Compliance Tiers [anchor: compliance_tiers]

The Node & Conformance Specification defines multiple compliance levels to accommodate a wide spectrum of implementations while preserving a strict definition of canonical correctness.

#### **Full Node Conformance (FNC)** - *Normative* [anchor: full_node_conformance_fnc_normative]
A node is “fully conformant” only if it:

- Stores the **entire canonical event log**,
- Stores **all snapshots** since genesis or its configured epoch,
- Performs **full deterministic replay**,
- Applies **all rulebooks** as active at the relevant cycle boundaries,
- Enforces **human-first authorship**,
- Enforces **safety rulebooks**,
- Computes **POD** and **POINT** deterministically,
- Rejects **non-conformant forks**, **AI-signed events**, and **invalid rulebook activations**.

Only FNC nodes are recognized as authoritative for the canonical universe.

---

#### **Replay-Only Conformance (ROC)** - *Normative but reduced* [anchor: replay_only_conformance_roc_normative_but_reduced]
A replay-only node:

- Stores snapshots and enough history to deterministically replay them,
- Reconstructs the canonical universe accurately,
- May not retain or store the full event log,
- May not participate in PoD block formation.

These nodes are useful for:

- auditing,
- verification of governance actions,
- blockchain-like reconstruction.

ROC nodes are conformant but **not authoritative** for chain growth.

---

#### **Snapshot-Only Conformance (SOC)** - *Informative* [anchor: snapshot_only_conformance_soc_informative]
A snapshot-only node:

- Stores periodic canonical snapshots,
- May discard intermediate events,
- Does **not** perform deterministic replay,
- Relies on trusted full nodes for hash verification.

Snapshot-only nodes are **not conformant** in the strict sense but are still recognized as valid consumers of the canonical universe.

---

#### **Non-Conformant (Sandbox-Only)** [anchor: non_conformant_sandbox_only]
A node is non-conformant if it:

- Stores **only sandbox data**,
- Produces its own speculative universes,
- Does not enforce rulebooks, human authorship, or deterministic replay.

These nodes play an important role in:

- experimentation,
- AI training,
- UI prototyping,

but MUST NOT advertise themselves as canonical.

---

### 13.2 Mandatory Test Vectors [anchor: mandatory_test_vectors]

All Full Conformant and Replay-Only nodes MUST pass a standardized conformance suite, including:

Conformance testing includes, but is not limited to:
- §8A custody manifest canonical encoding and hashing
- §8B reconciliation transcript determinism
- §8C state witness and receipt verification
- §8D partitioned merge behavior determinism

- **Genesis -> Snapshot replay**
  Nodes must reproduce identical state from the genesis block through multiple snapshot boundaries.

- **Safety classification tests**
  Nodes must apply safety classes (normal, sensitive_abstracted, non_distributable_blocked) identically to reference implementations.

- **Rulebook switching tests**
  Nodes must show correct activation of rulebooks at the proper cycle boundaries using recorded adoption actions.

- **Canonical authorship-signature tests**
  Nodes must pass the Profile-v0 authorship-signature vectors required by `canonical-event-authorship-and-signature-profile-v0.md`, including valid Ed25519 signatures, invalid signatures, altered payload hash, altered event type, altered author identity, wrong key owner, unknown key, revoked key use, historically valid pre-revocation signatures, key rotation, malformed key and signature encodings, unsupported signature profiles, byte-identical cross-implementation candidates, publication-wrapper mutation, unchanged signed bytes under canonical-position assignment, and attempted use of `event_index` in the human-authorship signed bytes.

- **POD/POINT cycle simulation tests**
  Nodes must:
  - compute POD flows deterministically,
  - compute POINT emissions identically,
  - enforce melt/invariants and identity token storage rules.

- **Tempo/Cycle schema and replay tests**
  Nodes must:
  - accept target-bound Tempo time claims only as ordinary `truth_claim` ideas with valid `tempo_claim` metadata,
  - reject any `time_claim`, `tempo_target`, `beacon`, `evidence`, `attestation`, `testimony`, or `source` idea type,
  - accept valid low-threshold target-bound Tempo claim creation and explicitly permitted Tempo-context evidence ideas/connections from eligible human `tempo_contributor` identities,
  - reject arbitrary idea creation, evidence creation outside Tempo context, connection creation outside Tempo context, challenge opening, challenge voting, verdict finalization, governance, POD, POINT, token, ordinary mana, or ordinary rate-limit authority from `tempo_contributor` status alone,
  - reject AI and non-human canonical Tempo claims, evidence ideas, challenges, votes, verdict authority, beacon support, cycle authority, and governance authority,
  - reject invalid target keys, target/profile mismatches, and insufficient Tempo mana,
  - reject invalid Tempo-context evidence or `same_as` connections with `ERR_TEMPO_EVIDENCE_CONNECTION_INVALID`,
  - verify evidence is represented by identity-authored ideas and existing `evidence_for` / `evidence_against` connections,
  - verify certainty-band challenge verdicts, not raw author counts or hidden weights, determine Tempo truth certainty,
  - verify `T_allow` structural support does not assign truth certainty and can permit structural closure only,
  - verify passive evidence alone cannot cross `T_allow`, `T_beacon`, Dmin, Dmax, survivor Dmax, certification, or frontier thresholds,
  - verify passive evidence plus profile-required eligible-human support may contribute to Dmin/Dmax structural readiness within caps,
  - verify repeated equivalent support from one identity counts once and later valid stance supersedes only current structural stance,
  - verify zero eligible humans produce record-only posture and no universal `cycle_close`,
  - verify one eligible human `tempo_contributor` can satisfy `structural_dmax_liveness_predicate` for forced Dmax structural closure only when the active profile's survivor minimum, passive plausibility, constrained/time-repair condition, and blocker checks are satisfied,
  - verify one eligible human cannot use `structural_dmax_liveness_predicate` for Dmin deliberative closure, beacon status, cycle certification, authorization-frontier advancement, or consequential authority,
  - verify accepted contradictory target-bound claims and unresolved blocking truth challenges block `structural_dmax_liveness_predicate`,
  - verify external links alone cannot satisfy `T_allow` and cannot satisfy beacon diversity,
  - verify local/server/client timestamps, receipt time, block height, scheduler observations, publication volume, local observations, and AI observations do not affect certainty,
  - verify Dmax mechanically implies structural Dmin,
  - verify Dmin plus `W_score >= W_target` closes deliberatively,
  - verify Dmax plus `W_score < W_target` closes forcibly,
  - verify `structural_dmax_liveness_predicate` plus `W_score < W_target` closes forcibly with trigger `dmax_structural_liveness_forced`,
  - verify forced cycles do not create POD, POINT, governance, lifecycle, final-rank, ordinary mana, ordinary rate-limit, or token authority,
  - verify `K`, beacon thresholds, diversity, independence, and stability requirements do not automatically shrink under collapse,
  - verify authorization frontier advancement stops at certification gaps and never decreases,
  - verify later certification finalizes explicit pending outputs only and does not retroactively validate forbidden actions or backfill ordinary mana/rate-limit authority,
  - reproduce derived targets, immutable profile references, structural stances, passive evidence aggregation, predicates, beacon coverage, cycle certification, authorization frontier, Tempo mode, and provisional/pending/authorized/blocked output status.

Nodes failing any vector are **not conformant**.

---

### 13.3 Tooling [anchor: tooling]

The conformance ecosystem SHOULD provide:

- **A formal test suite**
  Reference cases for:
  - replay logic,
  - safety classification,
  - rulebook activation,
  - POD/POINT calculations.

- **Replay state hash references**
  Each reference implementation SHOULD publish:
  - hash of replayed state after key snapshots,
  - hashes of importance maps,
  - token distribution hashes.

- **Example non-conformant forks**
  Demonstrating:
  - AI-signed events,
  - rulebook corruption,
  - replay divergence,
  - POD violations.

This allows developers to test their implementations against both valid and invalid behaviors.

---

## 14. Security Requirements [anchor: 14_security_requirements]

### 14.1 Attack Mitigation [anchor: attack_mitigation]

Nodes MUST enforce the following protections:

- **Hidden AI injections**
  Reject any canonical event whose human authorship is missing or compromised.

- **Safety bypasses**
  No event may bypass safety rulebooks or classifier decisions.

- **Rulebook corruption**
  Adoption actions must be verified with:
  - proper human authorship,
  - required quorum,
  - correct cycle-boundary activation semantics.

- **Snapshot tampering**
  Nodes must validate snapshot hashes and continuity with finalized prefixes and any exposed derived block headers.

- **Non-canonical event replay**
  Sandbox events, altered histories, and foreign-chain events MUST be rejected.

These mitigations form the core of canonical integrity.

---

### 14.2 Chain Integrity [anchor: chain_integrity]

Nodes MUST maintain canonical publication integrity by validating:

- **Finalized prefix history**
  Divergent finalized prefix histories MUST be rejected unless they result from a recognized canonical recovery procedure.

- **Hash continuity**
  Every finalized prefix commitment, derived block hash, and snapshot hash MUST match deterministic reconstruction.

- **Publication signer authenticity**
  Finality artifacts MUST be:
  - signed by valid human identities,
  - eligible under the active publication profile,
  - free of any AI-originated signatures.

Canonical publication integrity rules ensure that adversaries cannot introduce replay divergence or false histories.

---

### 14.3 Compromise Response [anchor: compromise_response]

When detecting malicious behavior or systemic compromise, nodes MUST:

- **Freeze suspicious identities**
  According to rulebook-defined identity-governance policies.

- **Quarantine non-conformant forks**
  Nodes MUST:
  - stop ingesting,
  - stop syncing,
  - stop signing or serving conflicting canonical publication artifacts to those universes.

- **Escalate to governance**
  Nodes SHOULD issue adoption or clarification proposals when:
  - rulebook corruption is detected,
  - identity fraud is widespread,
  - canonical publication instability is suspected,
  - safety infractions exceed thresholds.

Security response flows MUST be fully logged and replay-deterministic.

---

## 15. Node Conformance Accessibility Guarantee

### 15.1 Purpose

This section guarantees that operating a conformant node remains accessible to ordinary individuals using consumer-grade hardware and standard internet connectivity.

Conformance SHALL NOT depend on institutional resources, specialized infrastructure, or full archival hosting capacity.

The purpose of this guarantee is to preserve:

- Permissionless verification
- Broad decentralization
- Resistance to infrastructural capture
- The ability for individuals to independently validate canonical state

---

### 15.2 Principle: Verification Over Hosting

Conformance is defined by the ability to verify canonical state, not by the obligation to host or serve all data.

A node SHALL be considered conformant if it:

1. Deterministically replays canonical history from genesis or from a verified snapshot.
2. Applies rulebooks and safety floors deterministically.
3. Verifies canonical event hashes, signatures, and state roots.
4. Verifies payload hashes when payload blobs are present.
5. Produces identical canonical state roots as other conformant nodes.
6. Truthfully declares which payload tiers and bundle classes it stores or serves.

A conformant node SHALL NOT be required to:

- Store all payload blobs.
- Host full archival bundles.
- Serve public APIs.
- Accept public submissions.
- Maintain constant uptime.
- Provide high-bandwidth serving capacity.

---

### 15.3 Minimal Conformant Node (MCN)

The protocol SHALL recognize a Minimal Conformant Node profile.

An MCN MUST:

1. Verify canonical history using deterministic replay or verified snapshot continuation.
2. Validate event signatures and ordering.
3. Enforce rulebook-defined state transitions.
4. Enforce deterministic safety floors.
5. Support verification of Tier 0 payload bundles (or their commitments).

An MCN MAY:

- Store only Tier 0 payloads.
- Prune higher-tier payloads.
- Operate offline.
- Serve only local users.
- Refuse public ingress.

An MCN MUST remain capable of detecting canonical divergence.

---

### 15.4 Resource Accessibility Constraints

The protocol SHALL preserve the following constraints:

1. Deterministic replay MUST remain computationally feasible on consumer-grade hardware.
2. Snapshot artifacts MUST remain within reasonable bounds for broadband download.
3. Cryptographic primitives MUST rely on widely available algorithms implementable without specialized hardware.
4. Conformance MUST NOT depend on proprietary infrastructure or external blockchain access.
5. Conformance MUST NOT require persistent public hosting.

Any future rulebook or protocol modification that materially increases baseline storage, CPU, or bandwidth requirements SHALL include:

- An accessibility impact analysis.
- A justification under decentralization invariants.
- A defined fallback Minimal Conformant Node profile preserving accessibility.

---

### 15.5 Node Role Separation

The protocol SHALL distinguish between roles:

- Verifier Node (minimal)
- Tier Host Node
- Archival Node
- Ingress Node
- Relay Node

Only the Verifier Node role is mandatory for conformance.

No governance authority, canonical validity, ranking weight, or rulebook activation SHALL depend on operating an extended node role.

---

### 15.6 Anti-Centralization Guardrail

No rulebook, upgrade, or extension MAY:

- Require full archival storage for conformance.
- Require high-bandwidth serving capacity.
- Require institutional hardware.
- Require geographic or jurisdictional presence.
- Require recurring payment to a centralized operator.

If such requirements are introduced, they SHALL invalidate conformance claims for that rulebook version.

---

### 15.7 Summary Guarantee

The protocol guarantees:

Any technically competent individual using consumer-grade hardware can operate a fully conformant verifier node.

If this condition ceases to hold, the system SHALL be considered structurally centralized and in violation of its decentralization invariants.


## 16. Informative Appendices [anchor: 15_informative_appendices]

The following appendices are **non-normative**.
They illustrate patterns, recommended implementations, reference diagrams, and example workflows.
Nothing in this section imposes additional requirements beyond those stated in Sections 0-14.

---

### 16.1 Node Storage Diagrams [anchor: node_storage_diagrams]

This appendix SHOULD include schematic diagrams illustrating how a conformant node stores canonical state, including but not limited to:

- **Event Log (append-only)**
  Indexed by event_id (UUIDv7 or other local identifier), referencing:
  - signer identity
  - payload hash
  - payload class
  - rulebook version

- **Idea Table**
  Mapped by:
  - idea_id -> descriptions, ideatype, subtype(s), metadata

- **Connection Table**
  Representing:
  - relative_importance connections
  - same_as tiered equivalence links
  - authorship references
  - membership links
  - representation pointers

- **Challenge Table**
  Tracking:
  - open challenges
  - resolved verdicts
  - domain (truth, importance, action, representation)
  - contestant ideas
  - juror pool selection entries

- **Identity Table**
  Storing:
  - human identity metadata
  - verification proofs
  - key rotation history
  - posthumous markers
  - POD + POINT balances

- **Token Tables**
  Storing:
  - POD flows
  - POINT emissions
  - melt counters

- **Snapshot Storage**
  Frozen canonical state representing:
  - rulebooks active at boundary
  - importance rankings
  - safety classifier versions
  - identity token balances

- **Ent Ring Storage (Optional)**
  Training ring snapshots, non-canonical.

These diagrams demonstrate **clean separation** between canonical and sandbox data.

---

### 16.2 Replay Walkthrough Examples [anchor: replay_walkthrough_examples]

This appendix SHOULD include step-by-step examples of deterministic replay:

1. **Genesis -> First Snapshot**
   - Creating first ideas
   - Applying initial rulebooks
   - Generating snapshot_0

2. **Importance Challenge Replay**
   - Reproducing bubble-up ranking
   - Ensuring identical list ordering across implementations

3. **Truth Challenge Replay**
   - Reconstructing evidence rails
   - Applying certainty-band movements
   - Evaluating prediction claims with completion events

4. **Rulebook Adoption Example**
   - Adoption action
   - Rulebook activation at scheduled cycle boundary
   - Replay verifying correct rule activation

5. **Safety Classification Replay**
   - Example payload classified as sensitive_abstracted
   - Replay produces identical abstraction

6. **Token Replay Example**
   - POD flow reconstruction
   - POINT emission according to active rulebook

These examples illustrate the **end-to-end determinism** required for conformance.

---

### 16.3 Rate Limit Implementation Guide [anchor: rate_limit_implementation_guide]

This appendix SHOULD describe guidance for implementing rate limits:

- **Per-identity limits** on:
  - event submissions,
  - challenge openings,
  - action declarations.

- **Autopilot constraints**
  - Autopilot actions count toward human rate limits.
  - Autopilot cannot exceed human throughput.

- **Recommended mechanisms**
  - sliding window rate limits,
  - exponential backoff,
  - challenge congestion management.

- **Replay behavior**
  Rate-limit evaluation is **not replayed directly**; instead:
  - the node must verify that rate-limit conditions at event time were valid,
  - using rulebook definitions active in that event's epoch.

This section is informative, since implementations may vary, but **canonical rulebook checks are normative**.

---

### 16.4 Safety Enforcement Flowcharts [anchor: safety_enforcement_flowcharts]

This appendix SHOULD show example flowcharts describing:

- **Submission intake pipeline**
  1. receive event
  2. validate schema
  3. apply safety classifier
  4. assign payload class
  5. accept or generate blocked_submission

- **Payload class transformations**
  - how sensitive_abstracted content is transformed,
  - how non_distributable_blocked is stored in hash-only form.

- **Visibility explanations**
  ("Why am I seeing this?")
  - pointer to rulebook clause
  - classifier label
  - lineage graph

These diagrams help implementers verify correct classification logic.

---

### 16.5 Chain Anchor Examples [anchor: chain_anchor_examples]

This appendix SHOULD include examples of:

- **Prefix certificates and derived block headers containing snapshot hashes**
- **Canonical publication anchor verification** workflow
- **Detecting tampering**
- **Availability attestation / prefix-certificate proof format**
- **Expected prefix / block validation hashes**

Also recommended:

- Reference examples showing how a node should reject:
  - publication artifacts signed by AI identities,
  - finalized prefixes or derived blocks referencing impossible snapshot hashes,
  - conflicting finalized certificates or forks that violate deterministic replay.

---

### 16.6 Tribe Node Deployment Patterns [anchor: tribe_node_deployment_patterns]

This appendix SHOULD demonstrate how a tribe node is expected to operate:

- **Mirrors canonical data**
  - imports snapshots
  - accepts canonical events
  - rejects invalid forks

- **Maintains tribe-local rulebooks**
  - applies only to tribe-level importance challenges
  - never override universal governance

- **Publishes tribe maps**
  - tribe importance rankings
  - internal deliberations
  - internal action proposals

- **Bridging behavior**
  - tribe ideas may escalate to universal relevance when challenged in universal scope
  - tribe forks are always non-canonical and bounded to internal usage

Deployment patterns might include:
- tribe teaching nodes,
- tribe discussion servers,
- local offline tribe archives.

---

### 16.7 AI Sandbox Integration Examples [anchor: ai_sandbox_integration_examples]

This appendix SHOULD explain common sandbox integration patterns:

- **AI speculative idea generation**
  - stored separately from canonical universe
  - never included in deterministic replay

- **Ent training using rings**
  - ingest snapshot-based training data
  - respect safety walls
  - avoid cross-contamination into canonical state

- **Autopilot assistance**
  - AI proposes structure
  - human signs and submits canonical events

- **Non-canonical simulations**
  - AI-only worlds running accelerated reasoning
  - used for scenario testing, evolutionary strategy generation, or large-scale what-if analyses
  - ultimately feeding insights back into the canonical system via human-authored ideas

These examples ensure implementers build **safe, human-first AI integrations**.

---
