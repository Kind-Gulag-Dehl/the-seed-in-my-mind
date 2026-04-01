# Protocol v5 – Appendix A  

*(Normative appendix — subordinate to Protocol v5 Sections 0–11)*

---

## A0. Purpose, scope, and authority [anchor: a0_purpose_scope_and_authority]

### A0.1 Purpose [anchor: a0_1_purpose]

This appendix defines the **canonical data model and event schemas** required for Protocol v5 conformance. Its purpose is to eliminate ambiguity and implementation drift by specifying:

- the exact canonical objects that may exist in the universe,
- the required and optional fields for each object,
- the event types that may create, modify, or transform canonical state,
- the deterministic validation rules that govern acceptance or rejection of events.

This appendix is the binding interface between:

- the normative protocol text,
- governance and rulebook logic,
- offline and Mindseed operation,
- deterministic replay and fork resolution,
- and all conformant implementations (including reference clients and nodes).

Any conformant implementation MUST be able to serialize, validate, replay, and reconstruct canonical state using only the definitions in this appendix and the main Protocol v5 text.

---

### A0.2 Scope [anchor: a0_2_scope]

This appendix governs **canonical structure and mechanics only**. It defines *what exists*, *how it is represented*, and *how it changes*, but not *why* particular rules exist or *which* rules are adopted.

#### A0.2.1 Canonical objects [anchor: a0_2_1_canonical_objects]

This appendix defines schemas for the following canonical object classes:

- **identity**
- **idea**
- **connection** (edge)
- **challenge**
- **argument** (either as a distinct object or as an idea with constrained semantics, as specified herein)
- **description / representation artifacts** (where distinct from ideas)
- **rail** (ordered sequence object; vines are rail specializations)
- **block**
- **snapshot**
- **rulebook reference objects** (interface only; rulebook content is governed elsewhere)

Each object class defined here MUST have a stable, deterministic schema.

#### A0.2.2 Canonical events [anchor: a0_2_2_canonical_events]

This appendix defines the canonical event schemas used to:

- create objects,
- update object state,
- link objects,
- transition lifecycle states,
- apply challenge outcomes,
- record governance selections.

Only events defined in this appendix MAY mutate canonical state.

Events not conforming to these schemas MUST be rejected by conformant nodes.

---

### A0.3 Authority and conflict resolution [anchor: a0_3_authority_and_conflict_resolution]

This appendix is **subordinate** to the main Protocol v5 specification.

In the event of any conflict or ambiguity:

1. **Protocol v5 Section 0 invariants are authoritative.**
2. Deterministic replay requirements override convenience or interpretation.
3. Explicit schema definitions in this appendix override informal descriptions elsewhere.
4. No implementation behavior MAY rely on unspecified fields, implicit defaults, or out-of-band assumptions.

This appendix MAY be extended by future protocol versions, but MUST NOT be selectively implemented.

---

### A0.4 Normative conventions [anchor: a0_4_normative_conventions]

#### A0.4.1 Requirement language [anchor: a0_4_1_requirement_language]

This appendix uses the following normative terms:

- **MUST / SHALL** — absolute requirement for conformance.
- **MUST NOT / SHALL NOT** — absolute prohibition.
- **SHOULD** — strong recommendation; deviation requires explicit justification.
- **MAY** — optional behavior.

#### A0.4.2 Field presence rules [anchor: a0_4_2_field_presence_rules]

Each schema field is designated as one of:

- **REQUIRED** — MUST be present in all instances.
- **OPTIONAL** — MAY be present.
- **CONDITIONAL** — REQUIRED only if specified conditions are met.
- **FORBIDDEN** — MUST NOT appear.

Fields not explicitly defined in a schema are FORBIDDEN.

#### A0.4.3 Deterministic encoding and hashing [anchor: a0_4_3_deterministic_encoding_and_hashing]

All canonical objects and events MUST be:

- serialized deterministically,
- hashed using protocol-specified algorithms,
- ordered and validated using only canonical data.

No canonical meaning may depend on:

- local timestamps,
- node-specific metadata,
- user interface state,
- non-canonical sandbox data.

Serialization, hashing, domain separation, Merkle construction, and signature encoding and verification MUST follow the Canonical Encoding and Hashing Specification (v0).

---

### A0.5 Canonical minimalism invariant [anchor: a0_5_canonical_minimalism_invariant]

This appendix intentionally defines the **minimum sufficient structure** required to support:

- truth determination,
- importance determination,
- action deliberation,
- governance,
- safety and visibility,
- offline resilience.

If a concept can be expressed using existing objects and events, no new canonical object type SHALL be introduced. All higher-level constructs MUST be reducible to these primitives.

---

### A0.6 Relationship to other appendices [anchor: a0_6_relationship_to_other_appendices]

This appendix composes with, but does not duplicate:

- cryptographic and signature specifications,
- deterministic replay algorithms,
- governance rulebook content,
- safety classification semantics.

Those documents MUST reference the schemas defined here and MUST NOT redefine canonical fields independently.

---

## A1. Canonical identifiers, ordering, and cryptographic primitives [anchor: a1_canonical_identifiers_ordering_and_cryptographic_primitives]

This section defines the foundational identifiers, ordering rules, and cryptographic bindings used throughout the canonical event log. These rules apply uniformly to all canonical objects and events defined in this appendix.

No canonical meaning may depend on implementation-specific identifiers, local clocks, database row order, or non-deterministic generation methods.

---

### A1.1 Canonical identifiers [anchor: a1_1_canonical_identifiers]

All canonical objects and events MUST be identified using globally unique, collision-resistant identifiers.

#### A1.1.1 Identifier format [anchor: a1_1_1_identifier_format]

Canonical identifiers MUST satisfy all of the following properties:

- globally unique with overwhelming probability,
- stable and immutable once assigned,
- serializable deterministically.

Canonical identifiers MUST be UUIDv7 strings in canonical textual form (36 characters, lowercase hex with hyphens).
Fields explicitly defined as hashes/commitments are not identifiers and MUST be encoded as lowercase hex strings.

#### A1.1.2 Ordering guarantees [anchor: a1_1_2_ordering_guarantees]

Canonical ordering of events is determined exclusively by the ordered event log. Live ordering and finality are defined by finalized prefix certificates in `pod-consensus-and-canonical-publication-spec.md`; when derived blocks are exposed, they provide the public `(block_height, intra-block position)` address surface only after prefix finality.

Identifier ordering MUST NOT be used to reorder events, resolve conflicts, or infer causality.

If deterministic tie-breaking is required between otherwise equivalent items (e.g., stable ordering of lists), it MUST use canonical log order or other explicitly defined canonical ordering, not lexicographic identifier ordering.

---

### A1.2 Agent identity linkage [anchor: a1_2_agent_identity_linkage]

All canonical statements in the system are interpreted as **“an agent says …”**. This invariant is enforced structurally through explicit identity linkage.

#### A1.2.1 Identity references [anchor: a1_2_1_identity_references]

Canonical events and objects that represent assertions, actions, or endorsements MUST reference a canonical identity.

Two distinct identity reference fields are used:

- `author_identity_id` — the identity responsible for creating an object or event.
- `speaker_identity_id` — the identity whose claim or position is being expressed, if distinct.

Unless explicitly stated otherwise, these fields MUST reference the same identity.

#### A1.2.2 Required identity linkage [anchor: a1_2_2_required_identity_linkage]

The following canonical elements MUST include an identity reference:

- all canonical events,
- all ideas,
- all challenges,
- all arguments,
- all votes,
- all descriptions or representations.

Objects or events lacking a required identity reference MUST be rejected.

#### A1.2.3 Anonymous-but-verified identities [anchor: a1_2_3_anonymous_but_verified_identities]

The protocol MAY support identities whose public presentation is anonymous, provided that:

- the identity corresponds to a unique, verified human agent,
- the identity is cryptographically bound and persistent,
- accountability is preserved within the canonical log.

Anonymous presentation MUST NOT relax authorship, signature, or validation requirements.

---

### A1.3 Signatures [anchor: a1_3_signatures]

Cryptographic signatures bind canonical content to author identities and ensure integrity and non-repudiation.

#### A1.3.1 Signature requirements [anchor: a1_3_1_signature_requirements]

All canonical events MUST include a valid cryptographic signature.

Signatures MUST:

- be verifiable using the public key associated with `author_identity_id`,
- be generated using an approved cryptographic algorithm,
- cover the exact canonical byte representation of the signed content.

Unsigned or improperly signed events MUST be rejected.

#### A1.3.2 What is signed [anchor: a1_3_2_what_is_signed]

At minimum, the following MUST be signed:

- the complete canonical event envelope, including:
  - event type,
  - author identity reference,
  - canonical ordering references,
  - payload.

If blocks are used, the canonical block header MAY also be signed, as defined elsewhere.

The signed byte sequence is defined canonically by the Canonical Encoding and Hashing Specification (v0); this section specifies *what* is signed, not how bytes are formed.

#### A1.3.3 Verification and failure modes [anchor: a1_3_3_verification_and_failure_modes]

If signature verification fails, the event MUST be rejected.

Nodes MUST NOT attempt to repair, reinterpret, or partially apply invalidly signed events.

---

### A1.4 Time and clocks [anchor: a1_4_time_and_clocks]

Canonical meaning MUST NOT depend on wall-clock time.

#### A1.4.1 Logical time vs descriptive time [anchor: a1_4_1_logical_time_vs_descriptive_time]

Canonical ordering is determined exclusively by:

- event log order,
- block height and intra-block position, if applicable, as assigned by the derived packaging rules defined in `pod-consensus-and-canonical-publication-spec.md`.

Fields representing time observations (e.g., `observed_at`, `claimed_time`) are descriptive only and MUST NOT be used to reorder events unless explicitly permitted by a canonical rule.

#### A1.4.2 Prohibited time dependencies [anchor: a1_4_2_prohibited_time_dependencies]

Nodes MUST NOT:

- infer ordering from local clocks,
- reject events based on local time drift,
- mint tokens or advance epochs based on node-local time.

All time-dependent behavior MUST be derived from canonical events and deterministic rules.

---

## A2. Canonical object schemas (state objects) [anchor: a2_canonical_object_schemas_state_objects]

This section defines the canonical **state objects** that may exist in the protocol universe.
State objects represent the durable, replayable result of applying canonical events.
No state object may be created, modified, or removed except through events defined elsewhere in this appendix.

All fields not explicitly listed are FORBIDDEN.

---

### A2.0 Representation terminology (normative) [anchor: a2_0_representation_terminology_normative]

For representation semantics in this appendix, the following terms are normative:

- **representation object**
  A separate canonical object that references one textual payload for one target object (`idea` or `rail`) at one `(tier_length, tier_complexity)` slot.

- **candidate representation** (also called **competing representation**)
  A representation object present in canonical history that is not currently selected by the canonical pointer for its slot.

- **canonical representation pointer**
  The object-level pointer from a specific `(tier_length, tier_complexity)` slot to the currently selected `representation_id`.

- **proposed description**
  Synonymous with a candidate representation object not currently selected by the canonical representation pointer.

Representations are canonical objects; pointer selection changes over time only through replay of finalized representation challenge verdicts, while representation objects remain preserved in history.

---

### A2.1 Identity object [anchor: a2_1_identity_object]

The identity object represents a single real agent capable of authorship and accountability within the system.

#### A2.1.1 Required fields [anchor: a2_1_1_required_fields]

REQUIRED:
- `identity_id`  
  Canonical identifier (UUIDv7 string) for the identity.

- `verification_status`  
  A structured status indicator or reference proving that the identity corresponds to a verified real human agent.  
  This field MAY be a reference or hash pointing to verification proof material defined outside this appendix.

- `created_event_id`  
  Reference to the canonical event that created this identity.

#### A2.1.2 Optional fields [anchor: a2_1_2_optional_fields]

OPTIONAL:
- `public_display_name`  
  A human-readable name or label chosen by the identity holder.

- `public_visibility_flags`  
  Privacy-preserving flags indicating which aspects of the identity may be publicly rendered.

- `profile_metadata`  
  Non-authoritative descriptive metadata (e.g., biography text, links, avatar references).  
  This metadata MUST NOT affect canonical behavior, ranking, governance, or validation.

#### A2.1.3 Invariants [anchor: a2_1_3_invariants]

- Only identities corresponding to **verified human agents** MAY author canonical events, except the reserved non-human identity `system_boundary_emitter` for mechanically emitted boundary events (`cycle_close`, and `snapshot_commit` if enabled by active profile/rulebooks).
- AI identities, if represented at all, MUST be explicitly marked as non-human and:
  - MUST NOT author canonical events,
  - MUST NOT vote,
  - MUST NOT govern,
  - MUST NOT mint or receive POD or POINT.
- All human-authored canonical events are interpreted as **“this identity says …”**.

Identities violating these invariants MUST be rejected at event validation time.
---

### A2.2 Idea object (unified idea model) [anchor: a2_2_idea_object_unified_idea_model]

The idea object is the unified representation for all epistemic and practical content in the system.

#### A2.2.1 Required core fields [anchor: a2_2_1_required_core_fields]

REQUIRED:
- `idea_id`  
  Canonical identifier (UUIDv7 string) for the idea.

- `idea_type`  
  Enumeration with the following allowed values:
  - `truth_claim`
  - `conceptual_idea`
  - `actionable_idea`
  - `action`
  - `identity`

- `speaker_identity_id`  
  Reference to the identity whose statement or position the idea represents.

- `created_event_id`  
  Reference to the canonical event that created the idea.

#### A2.2.2 Optional core fields [anchor: a2_2_2_optional_core_fields]

OPTIONAL:
- `title`  
  Short human-readable label.

- `short_label`  
  Extremely compact label for dense views or summaries.

- `canonical_representation_ids`
  Deterministic mapping from `(tier_length, tier_complexity)` slots to the currently canonical `representation_id` values for this idea.

- `tags`  
  Rulebook-extensible string tags used for filtering or categorization.  
  Tags MUST NOT affect canonical truth, importance, or governance semantics.

#### A2.2.3 Conditional fields by idea_type [anchor: a2_2_3_conditional_fields_by_idea_type]

CONDITIONAL:

- If `idea_type` is `truth_claim`:
  - `truth_subtype` MUST be present and MUST be one of:
    - `existence_or_boundary`
    - `observation_or_measurement`
    - `relation_or_pattern`
    - `causal_or_mechanistic`
    - `test_or_validation`
    - `prediction`
    - `model_or_integration`

- If `idea_type` is `action`:
  - `execution_window` MAY be present, describing claimed or observed time bounds.
  - `affected_scope` MAY be present.
  - `execution_proof_refs` MAY be present, referencing evidence or validation claims.

- If `idea_type` is `identity`:
  - The idea MUST reference an existing `identity_id`.
  - The protocol MUST enforce that no idea of type `identity` represents gods, fictional characters, abstract collectives, or non-agent entities.

#### A2.2.4 Invariants [anchor: a2_2_4_invariants]

- Every idea is interpreted as **“speaker_identity says …”**.
- Ideas MUST NOT exist without a speaker identity.
- Canonical ideas MUST represent statements made by real agents.

---

### A2.3 Description / representation objects [anchor: a2_3_description_representation_objects]

Descriptions are represented as separate canonical **representation objects**.

The embedded-on-idea alternative model is not canonical in Protocol v5 Appendix A.
Conformant implementations MUST model canonical descriptions as representation objects referenced by canonical representation pointers.

#### A2.3.1 Required fields [anchor: a2_3_1_required_fields]

REQUIRED:
- `representation_id`
  Canonical identifier (UUIDv7 string) for the representation object.
- `target_kind`
  Enumeration for representation target:
  - `idea`
  - `rail`
- `target_object_id`
  Canonical identifier of the represented target object.
- `tier_length`
  Enumeration for description length tier:
  - `sentence`
  - `paragraph`
  - `full`
- `tier_complexity`
  Enumeration for complexity tier:
  - `fundamental`
  - `standard`
  - `advanced`
  - `canonical`
- `payload_hash` (`hash32`)
  Deterministic hash of the canonical representation payload bytes.
- `author_identity_id`
  Identity that authored this representation payload.
- `created_event_id`
  Reference to the canonical event that created this representation object.

#### A2.3.2 Optional fields [anchor: a2_3_2_optional_fields]

OPTIONAL:
- `language_locale`
- `provenance`
  Indicates whether the content was human-authored or AI-drafted and later adopted.
- `safety_classification_refs`
  References to safety classification objects.

#### A2.3.3 Invariants [anchor: a2_3_3_invariants]

- Canonical selection of which representation is active for any `(tier_length, tier_complexity)` slot MUST be determined by replay of finalized representation challenge verdicts (`challenge_finalize_verdict`) and canonical pointer updates, not mutable local flags.
- Representation objects MUST NOT modify the semantic identity of the target object they describe.
- Representation payload bytes are content-addressed blobs keyed by `payload_hash`; events and state objects reference payloads by hash.

---

### A2.4 Connection (edge) object [anchor: a2_4_connection_edge_object]

Connections encode all relationships between ideas.

#### A2.4.1 Required fields [anchor: a2_4_1_required_fields]

REQUIRED:
- `connection_id`  
  Canonical identifier (UUIDv7 string) for the connection.
- `from_idea_id`
- `to_idea_id`
- `connection_type`  
  Enumeration with exactly the following allowed values:
  - `same_as`
  - `relative_importance`
  - `membership`

- `created_by_event_id`

#### A2.4.2 Conditional metadata by connection_type [anchor: a2_4_2_conditional_metadata_by_connection_type]

CONDITIONAL:

- If `connection_type` is `same_as`:
  - Merge semantics MUST be defined by the governing rulebooks.

- If `connection_type` is `membership`:
  - Scope and membership semantics MAY be present.
  - Membership roles, if any, MUST be declarative and non-authoritative.

- If `connection_type` is `relative_importance`:
  REQUIRED:
  - `usage`
  - `axis`
  - `timeframe`
  - `scope`

  OPTIONAL:
  - `value_representation`
  - `certainty_band`

#### A2.4.3 Invariants [anchor: a2_4_3_invariants]

- Rulebooks MUST NOT introduce new connection types.
- All connection semantics MUST be reducible to the fixed connection_type set.

---

#### A2.4.3A Rail object (ordered sequence primitive) [anchor: a2_4_3a_rail_object_ordered_sequence_primitive]

A rail is a first-class canonical object for ordered traversal context.

REQUIRED:
- `rail_id`
  Canonical identifier (UUIDv7 string) for the rail.
- `rail_kind`
  Enumeration:
  - `vine`
- `speaker_identity_id`
  Identity that authors the rail object.
- `created_event_id`
  Reference to the canonical event that created the rail.
- `canonical_representation_ids`
  Deterministic mapping from `(tier_length, tier_complexity)` slots to the currently canonical `representation_id` values for this rail.
- `item_idea_ids`
  Ordered list of `idea_id` values.

CONDITIONAL:
- If `rail_kind = vine`, `vine_type` MUST be present and MUST be one of:
  - `pathway_vine`
  - `narrative_vine`

OPTIONAL:
- `base_rail_id`
  Reference rail for a fork lineage.
- `step_meta`
  Ordered metadata aligned to adjacent steps. Each step metadata entry MAY include `via_connection_id`.

Invariants:
- Rails/vines MUST preserve item order exactly as recorded.
- Rails/vines MUST NOT introduce new connection types or alter base connection semantics.
- For `pathway_vine`, each adjacent step MAY include optional `via_connection_id` provenance.
- For `narrative_vine`, no underlying-edge requirement exists.

---

### A2.5 Challenge object [anchor: a2_5_challenge_object]

The challenge object represents an active or resolved dispute.

#### A2.5.1 Required fields [anchor: a2_5_1_required_fields]

REQUIRED:
- `challenge_id`  
  Canonical identifier (UUIDv7 string) for the challenge.
- `challenge_domain`  
  Enumeration:
  - `truth_challenge`
  - `importance_challenge`
  - `action_challenge`
  - `representation_challenge`

- `subject_idea_ids`  (required for idea-targeted challenges)
- `created_by_identity_id`
- `lifecycle_state`

#### A2.5.2 Optional and conditional fields [anchor: a2_5_2_optional_and_conditional_fields]

OPTIONAL:
- `subject_rail_ids`  (required for rail-targeted representation challenges)
- `framing_representation_id`
- `eligibility_pool_ref`
- `timeline_windows`
- `linked_argument_refs`
- `linked_evidence_refs`

#### A2.5.3 Deterministic lifecycle fields [anchor: a2_5_3_deterministic_lifecycle_fields]

REQUIRED:
- `opened_event_id`
- `closed_event_id` (if closed)
- `verdict_id` (if finalized)

Lifecycle state transitions MUST be deterministic and event-driven.

---

### A2.6 Verdict object [anchor: a2_6_verdict_object]

Verdicts record the resolved outcome of a challenge.

#### A2.6.1 Required fields [anchor: a2_6_1_required_fields]

REQUIRED:
- `verdict_id`  
  Canonical identifier (UUIDv7 string) for the verdict.
- `challenge_id`
- `outcome`
- `tally_summary`
- `finalization_event_id`

For governance/rulebook verdicts, REQUIRED metadata fields:
- `decision_cycle_index`
- `change_class` (`emergency` | `standard` | `major` | `constitutional`)
- `delay_policy_version`
- `activation_cycle_index`

#### A2.6.2 Transformation fields [anchor: a2_6_2_transformation_fields]

OPTIONAL:
- `state_transformations`  
  Explicit references to canonical state changes applied as a result of the verdict.

Verdicts MUST NOT retroactively modify history. All effects MUST be applied via new events.

---

### A2.7 Rank snapshots / importance state objects [anchor: a2_7_rank_snapshots_importance_state_objects]

Importance MAY be derived purely from replay or stored periodically as snapshots.

If stored, rank snapshots MUST conform to the following schema.

REQUIRED:
- `snapshot_id`
- `ranking_scope`
- `axis`
- `timeframe`
- `ordered_idea_ids`
- `created_event_id`

Deterministic tie-breaking rules MUST be specified and stable.

---

### A2.8 Safety classification objects (interface-level) [anchor: a2_8_safety_classification_objects_interface_level]

Safety classification objects provide visibility control without altering canonical meaning.

REQUIRED:
- `classification_id`
- `rulebook_id`
- `jurisdiction_lens`
- `explanation_ref`

Safety classifications MUST NOT affect canonical replay.

---

### A2.9 Rulebook reference objects (interface-level) [anchor: a2_9_rulebook_reference_objects_interface_level]

Rulebook objects are referenced canonically but defined elsewhere.

REQUIRED:
- `rulebook_id`
- `rulebook_version`
- `rulebook_hash`
- `activation_event_id`

Snapshots MUST record the active rulebook set deterministically.

---

## A3. Canonical event envelope (applies to all events) [anchor: a3_canonical_event_envelope_applies_to_all_events]

This section defines the **canonical event envelope** shared by all canonical events in the protocol.
The event envelope provides authorship, integrity, ordering, and validation guarantees independent of event payload semantics.

Every canonical event defined later in this appendix MUST be wrapped in an envelope conforming exactly to this section.

Fields not explicitly defined here are FORBIDDEN at the envelope level.

---

### A3.1 Event envelope fields [anchor: a3_1_event_envelope_fields]

All canonical events MUST include the following envelope fields.

#### A3.1.1 Required fields [anchor: a3_1_1_required_fields]

REQUIRED:

- `event_id`  
  Canonical identifier (UUIDv7 string) for the event.  
  MUST be globally unique and immutable.

- `event_type`  
  String identifier naming the canonical event type.  
  MUST match one of the event types defined in Section A4.

- `author_identity_id`  
  Canonical identity of the event author.  
  MUST reference a verified human identity for human-authored events. For mechanically emitted boundary events, MUST reference the reserved `system_boundary_emitter` identity.

- `created_at`  
  Descriptive timestamp supplied by the author or client.  
  This field is **non-authoritative** and MUST NOT be used for ordering, validation, replay, or epoch derivation.

- `ordering_ref`  
  Canonical ordering reference, expressed as either:
  - an explicit `prev_event_ref`, or
  - an implicit position derived from block inclusion rules, if blocks are used.

  The ordering mechanism MUST be deterministic and globally consistent.

- `payload_hash`  
  Cryptographic hash of the canonical payload bytes after deterministic encoding.

- `signature`  
  Cryptographic signature over the canonical envelope and payload hash.

- `public_key_ref`  
  Reference to the public key associated with `author_identity_id` used to verify `signature`.

#### A3.1.2 Optional fields [anchor: a3_1_2_optional_fields]

OPTIONAL:

- `client_info`  
  Non-authoritative metadata describing the originating client or software version.  
  This field MUST NOT affect validation, replay, ordering, or canonical state.
  This field MUST NOT include network identifiers (e.g., IP addresses, routing headers, transport metadata, or device fingerprints).

---

### A3.2 Event payload determinism [anchor: a3_2_event_payload_determinism]

All event payloads MUST be encoded deterministically prior to hashing and signing.

#### A3.2.1 Canonical encoding [anchor: a3_2_1_canonical_encoding]

Canonical payload encoding MUST satisfy all of the following:

- deterministic field ordering,
- deterministic numeric encoding (no locale-dependent formats),
- deterministic boolean and null encoding,
- explicit handling of empty values,
- no implicit defaults.

If JSON is used, implementations MUST enforce:

- lexicographic ordering of object keys,
- UTF-8 encoding,
- Unicode normalization to a single canonical form,
- prohibition of NaN, Infinity, or implementation-specific numeric values.

Binary encodings MAY be used if and only if the encoding rules are fully deterministic and universally specified.

#### A3.2.2 Hash computation [anchor: a3_2_2_hash_computation]

`payload_hash` MUST be computed over the exact canonical byte sequence produced by deterministic encoding.

Any discrepancy in encoding MUST result in a different hash and therefore a different signature.

Nodes MUST NOT attempt to reinterpret or normalize payloads after hashing.

---

### A3.3 Validation pipeline [anchor: a3_3_validation_pipeline]

All conformant nodes MUST validate canonical events using the following pipeline, in the specified order.
Failure at any step MUST cause the event to be rejected.

#### A3.3.1 Signature verification [anchor: a3_3_1_signature_verification]

Nodes MUST:

- verify that `signature` is valid for the signed bytes,
- verify that `public_key_ref` corresponds to `author_identity_id`,
- verify that the author identity is eligible to author canonical events.

Events with invalid or unverifiable signatures MUST be rejected.

#### A3.3.2 Schema validation [anchor: a3_3_2_schema_validation]

Nodes MUST:

- verify that the event envelope conforms to Section A3,
- verify that the payload schema matches the declared `event_type`,
- reject any event containing forbidden or unknown fields.

#### A3.3.3 Invariant checks [anchor: a3_3_3_invariant_checks]

Nodes MUST enforce all applicable Protocol v5 Section 0 invariants, including but not limited to:

- human-first authorship,
- no AI-authored canonical events,
- no reliance on wall-clock time,
- no unauthorized state mutation.

Violations of protocol invariants MUST result in rejection.

#### A3.3.4 Rulebook constraints [anchor: a3_3_4_rulebook_constraints]

Nodes MUST evaluate the event against the active rulebook set, as determined at the event’s canonical ordering position.

Rulebooks MAY impose additional constraints on:
- who may perform certain actions,
- rate limits,
- eligibility requirements,
- domain-specific restrictions.

Rulebook evaluation MUST be deterministic.

#### A3.3.5 State transition validation [anchor: a3_3_5_state_transition_validation]

Finally, nodes MUST verify that:

- the event represents a valid transition from the current canonical state,
- all referenced objects exist and are in compatible states,
- the event does not imply retroactive modification of history.

If the state transition is invalid, the event MUST be rejected without partial application.

---

## A4. Canonical event types (catalog) [anchor: a4_canonical_event_types_catalog]

This section enumerates **all canonical event types** permitted by the protocol.
It is the **single source of truth** for events that may mutate canonical state.

No canonical state transition may occur except through events defined in this section.
Events not listed here MUST be rejected by conformant nodes.

Each event type defined below:
- uses the canonical event envelope defined in Section A3,
- has a strictly defined payload schema,
- may only produce the effects explicitly described.

Fields not listed in a payload schema are FORBIDDEN.

---

## A4.1 Identity events [anchor: a4_1_identity_events]

### A4.1.1 `identity_create` [anchor: a4_1_1_identity_create]

Creates a new canonical identity.

REQUIRED payload fields:
- `identity_id`
- `initial_public_key_ref`
- `verification_reference` (interface-level pointer)

Effects:
- Creates a new identity object.
- Identity is inactive for canonical authorship until verified.

---

### A4.1.2 `identity_verification_update` [anchor: a4_1_2_identity_verification_update]

Updates the verification status of an identity.

REQUIRED payload fields:
- `identity_id`
- `verification_status`
- `verification_reference`

Effects:
- Updates verification status.
- Enables or disables eligibility for canonical authorship.

---

### A4.1.3 `identity_visibility_update` [anchor: a4_1_3_identity_visibility_update]

Updates public visibility flags for an identity.

REQUIRED payload fields:
- `identity_id`
- `public_visibility_flags`

Effects:
- Affects presentation only.
- MUST NOT affect canonical validation, governance, or ranking.

---

### A4.1.4 `identity_key_rotate` / `identity_key_revoke` [anchor: a4_1_4_identity_key_rotate_identity_key_revoke]

Manages cryptographic keys associated with an identity.

REQUIRED payload fields:
- `identity_id`
- `key_ref`
- `action` (`rotate` or `revoke`)

Effects:
- Updates key validity for future signatures.
- Does not invalidate past events.

---

## A4.2 Idea events [anchor: a4_2_idea_events]

### A4.2.1 `idea_create` [anchor: a4_2_1_idea_create]

Creates a new canonical idea.

REQUIRED payload fields:
- `idea_id`
- `idea_type`
- `speaker_identity_id`
- `initial_representation_refs` (if any)

CONDITIONAL:
- If `idea_type` is `truth_claim`, `truth_subtype` MUST be present.

Effects:
- Creates a new idea object.
- `initial_representation_refs`, if present, MUST reference already-existing canonical representation objects whose `target_kind = idea` and `target_object_id = idea_id`.
- `idea_create` MUST NOT create representation objects directly.

---

### A4.2.2 `idea_update_metadata` [anchor: a4_2_2_idea_update_metadata]

Updates non-substantive metadata.

REQUIRED payload fields:
- `idea_id`
- `metadata_patch`

Effects:
- Updates allowed metadata fields only.
- MUST NOT change meaning, truth status, or importance.

---

### A4.2.3 `idea_update_representation` [anchor: a4_2_3_idea_update_representation]

Deprecated compatibility alias of `representation_create` for idea targets.

REQUIRED payload fields:
- `representation_id`
- `target_kind`  // MUST be `idea` for this alias
- `target_object_id`
- `tier_length`
- `tier_complexity`
- `payload_hash` (`hash32`)
- `author_identity_id`

OPTIONAL payload fields:
- `language_locale`
- `provenance`

Effects:
- Creates a new candidate/competing representation object for the target idea.
- MUST NOT update canonical representation pointers.
- Payload encoding and semantics are identical to `representation_create`.

---

### A4.2.4 `idea_deprecate` / `idea_retract` [anchor: a4_2_4_idea_deprecate_idea_retract]

Marks an idea as deprecated or retracted.

REQUIRED payload fields:
- `idea_id`
- `reason_representation_ref`

Effects:
- Idea remains in history.
- Deprecation affects visibility and downstream reasoning only.

---

### A4.2.5 `representation_create` [anchor: a4_2_5_representation_create]

Creates a new canonical candidate representation object.

REQUIRED payload fields:
- `representation_id`
- `target_kind`  // enum: `idea`, `rail`
- `target_object_id`
- `tier_length`
- `tier_complexity`
- `payload_hash` (`hash32`)
- `author_identity_id`

OPTIONAL payload fields:
- `language_locale`
- `provenance`

Effects:
- Creates a new representation object in canonical history.
- Associates that representation with the target object's representation set for deterministic replay.
- DOES NOT update canonical representation pointers.
- Canonical representation pointers for ideas and rails are updated only when replay applies a finalized representation challenge verdict (`challenge_finalize_verdict`).

Invariants:
- `representation_create` is the authoritative canonical path for creating candidate/competing representations for ideas and rails.
- Payload bytes are content-addressed blobs keyed by `payload_hash` and MAY be distributed independently of the event log.

---

## A4.2B Rail events [anchor: a4_2b_rail_events]

### A4.2B.1 `rail_create` [anchor: a4_2b_1_rail_create]

Creates a new rail object.

REQUIRED payload fields:
- `rail_id`
- `rail_kind`  // enum: `vine`
- `speaker_identity_id`
- `item_idea_ids` (ordered list)

CONDITIONAL payload fields:
- If `rail_kind = vine`, `vine_type` MUST be present and MUST be one of `pathway_vine`, `narrative_vine`.

OPTIONAL payload fields:
- `initial_representation_refs`
- `step_meta` (ordered metadata aligned to adjacent steps; each entry MAY include `via_connection_id`)

Effects:
- Creates a new rail object with the ordered item list.
- Does not create or modify base graph connections.

### A4.2B.2 `rail_fork` [anchor: a4_2b_2_rail_fork]

Creates a new rail derived from an existing rail.

REQUIRED payload fields:
- `base_rail_id`
- `rail_id`
- `speaker_identity_id`
- `item_idea_ids` (full ordered replacement list)

OPTIONAL payload fields:
- `vine_type` (if omitted, inherits from base rail when base rail is a vine)
- `step_meta`

Effects:
- Creates a new rail object in canonical history.
- Records lineage through `base_rail_id`.
- Fork-only mutation model: existing rails are not edited in place.

### A4.2B.3 `rail_update_representation` [anchor: a4_2b_3_rail_update_representation]

Deprecated compatibility alias of `representation_create` for rail targets.

REQUIRED payload fields:
- `representation_id`
- `target_kind`  // MUST be `rail` for this alias
- `target_object_id`
- `tier_length`
- `tier_complexity`
- `payload_hash` (`hash32`)
- `author_identity_id`

OPTIONAL payload fields:
- `language_locale`
- `provenance`

Effects:
- Creates a new candidate/competing representation object for the target rail.
- MUST NOT update canonical representation pointers.
- Payload encoding and semantics are identical to `representation_create`.

---

## A4.3 Connection events [anchor: a4_3_connection_events]

### A4.3.1 `connection_create` [anchor: a4_3_1_connection_create]

Creates a new connection between ideas.

REQUIRED payload fields:
- `connection_id`
- `from_idea_id`
- `to_idea_id`
- `connection_type`

CONDITIONAL:
- If `connection_type` is `relative_importance`, all required metadata
  (`usage`, `axis`, `timeframe`, `scope`) MUST be present.

Effects:
- Creates a new connection object.

---

### A4.3.2 `connection_update` [anchor: a4_3_2_connection_update]

Updates allowed mutable fields on a connection.

REQUIRED payload fields:
- `connection_id`
- `update_patch`

Effects:
- Only fields explicitly declared mutable MAY change.

---

### A4.3.3 `connection_remove` [anchor: a4_3_3_connection_remove]

Tombstones a connection.

REQUIRED payload fields:
- `connection_id`
- `reason_representation_ref`

Effects:
- Connection is marked inactive.
- Historical presence is preserved.

---

### A4.3.4 `same_as_resolution` [anchor: a4_3_4_same_as_resolution]

Finalizes equivalence resolution between ideas.

REQUIRED payload fields:
- `canonical_idea_id`
- `merged_idea_ids`

Effects:
- Establishes canonical representative.
- Does not delete merged ideas.

---

## A4.4 Challenge lifecycle events [anchor: a4_4_challenge_lifecycle_events]

### A4.4.1 `challenge_create` [anchor: a4_4_1_challenge_create]

Creates a new challenge.

REQUIRED payload fields:
- `challenge_id`
- `challenge_domain`
- `framing_representation_ref`

CONDITIONAL payload fields:
- `subject_idea_ids` (required for idea-targeted challenges)
- `subject_rail_ids` (required for rail-targeted representation challenges)

Effects:
- Creates a challenge object in draft state.

---

### A4.4.2 `challenge_open_arguments` [anchor: a4_4_2_challenge_open_arguments]

Opens the argument submission phase.

REQUIRED payload fields:
- `challenge_id`

---

### A4.4.3 `challenge_close_arguments` [anchor: a4_4_3_challenge_close_arguments]

Closes the argument submission phase.

REQUIRED payload fields:
- `challenge_id`

---

### A4.4.4 `challenge_open_voting` [anchor: a4_4_4_challenge_open_voting]

Opens voting.

REQUIRED payload fields:
- `challenge_id`
- `eligibility_pool_ref`

---

### A4.4.5 `challenge_close_voting` [anchor: a4_4_5_challenge_close_voting]

Closes voting.

REQUIRED payload fields:
- `challenge_id`

---

### A4.4.6 `challenge_finalize_verdict` [anchor: a4_4_6_challenge_finalize_verdict]

Finalizes the verdict.

REQUIRED payload fields:
- `challenge_id`
- `verdict_id`

If the verdict applies a governance/rulebook change, REQUIRED payload metadata fields:
- `decision_cycle_index`
- `change_class`
- `delay_policy_version`
- `activation_cycle_index`

Effects:
- Locks verdict outcome.
- Enables downstream state transformations.
- For `representation_challenge` verdicts, deterministic replay MUST update canonical representation pointer selection for the target (`idea` or `rail`) slot by selecting exactly one canonical representation and superseding prior selection without deleting history.
- Canonical representation pointers MUST NOT be updated by `representation_create`, `idea_update_representation`, or `rail_update_representation`.

---

### A4.4.7 `challenge_cancel` / `challenge_supersede` [anchor: a4_4_7_challenge_cancel_challenge_supersede]

Terminates or replaces a challenge.

REQUIRED payload fields:
- `challenge_id`
- `reason_representation_ref`

Effects:
- Challenge ends without verdict.
- History preserved.

---

## A4.5 Argument and evidence events [anchor: a4_5_argument_and_evidence_events]

### Modeling note [anchor: modeling_note]

Conformant implementations MUST choose **exactly one** of the following models.

---

## A4.5 Argument and evidence modeling (no special event types) [anchor: a4_5_argument_and_evidence_modeling_no_special_event_types]

Protocol v5 treats **arguments, evidence, and importance-arguments as ordinary ideas** connected using the fixed connection primitives.

Accordingly, Appendix A defines **no dedicated canonical event types** for “argument creation,” “argument attachment,” “evidence attach,” or “evidence detach.”

Instead, conformant implementations MUST represent argumentation and evidence placement using only:

- `idea_create` (to create the argument/evidence idea), and
- `connection_create` / `connection_remove` with `connection_type = relative_importance` (to place it).

No additional canonical object type named “argument” is introduced.

### A4.5.1 Argument-as-idea (MANDATORY model) [anchor: a4_5_1_argument_as_idea_mandatory_model]

An argument MUST be represented as an **idea** authored by a speaker identity.

- The argument idea MAY be any canonical `idea_type` permitted by rulebooks, but conformant implementations SHOULD use `conceptual_idea` unless the argument is itself a truth claim (in which case it SHOULD be `truth_claim` with an appropriate `truth_subtype`).

Arguments MUST be attached to the relevant target(s) using `relative_importance` connections with `usage = importance_argument`.

#### Canonical placement pattern [anchor: canonical_placement_pattern]

To submit an importance argument within a challenge between subject ideas:

1. Create the argument idea via `idea_create`.
2. Create one or more connections via `connection_create`:
   - `connection_type = relative_importance`
   - `usage = importance_argument`
   - `from_idea_id = <argument_idea_id>`
   - `to_idea_id = <subject_idea_id>` (one or more)

#### Scoping an argument to a specific challenge (CONDITIONAL) [anchor: scoping_an_argument_to_a_specific_challenge_conditional]

If rulebooks require that an argument be explicitly scoped to a specific challenge instance, the placement connection payload MAY include:

OPTIONAL (rulebook-controlled):
- `context_challenge_id`

If present, `context_challenge_id` MUST reference an existing canonical challenge object.
If absent, the argument is interpreted as a general-purpose argument attached to the subject idea(s), and challenge UIs MAY choose to display it when relevant.

### A4.5.2 Evidence placement (truth challenges) [anchor: a4_5_2_evidence_placement_truth_challenges]

Evidence MUST be represented as an ordinary idea and attached using `relative_importance` connections:

- `usage = evidence_for`
- `usage = evidence_against`

#### Canonical placement pattern [anchor: canonical_placement_pattern_2]

To attach evidence to a truth claim:

1. Create the evidence idea via `idea_create` (often a `truth_claim` or `conceptual_idea`).
2. Create a placement connection via `connection_create`:
   - `connection_type = relative_importance`
   - `usage = evidence_for` or `usage = evidence_against`
   - `from_idea_id = <evidence_idea_id>`
   - `to_idea_id = <truth_claim_idea_id>`

#### Scoping evidence to a specific challenge (CONDITIONAL) [anchor: scoping_evidence_to_a_specific_challenge_conditional]

As with arguments, rulebooks MAY require explicit scoping using an OPTIONAL `context_challenge_id` field on the placement connection.

### A4.5.3 Removal / withdrawal semantics [anchor: a4_5_3_removal_withdrawal_semantics]

Arguments and evidence are never deleted.

To withdraw an argument or detach evidence, implementations MUST use one of:

- `connection_remove` (tombstone semantics) on the placement connection, and/or
- creation of new counter-arguments / counter-evidence ideas and connections.

Idea objects themselves SHOULD NOT be “withdrawn” in a way that removes them from history; any “retraction” MUST be recorded using `idea_deprecate` / `idea_retract` and remains challengeable.

---

## A4.6 Governance / rulebooks (idea-first; activation is cycle-scheduled) [anchor: a4_6_governance_rulebooks_idea_first_activation_is_snapshot_derived]

Protocol v5 treats governance artifacts and rulebooks as **ordinary ideas** and their relationships, not privileged object families.

Appendix A therefore defines **no special canonical event type** for “rulebook proposal” as a separate governance-only primitive.

### A4.6.1 Governance proposals and debate [anchor: a4_6_1_governance_proposals_and_debate]

A governance proposal MUST be represented as an **actionable idea** (and/or associated conceptual ideas), created via `idea_create`.

Debate about governance proposals MUST use the same argument mechanisms as all other domains:

- arguments are ordinary ideas,
- attached via `relative_importance` connections with:
  - `usage = importance_argument`
  - and where applicable `usage = evidence_for` / `usage = evidence_against`.

### A4.6.2 Rulebook references (interface-level) [anchor: a4_6_2_rulebook_references_interface_level]

A rulebook’s **content** is defined outside Appendix A, but its **identity and commitment** MUST be representable canonically.

A conformant implementation MUST represent a rulebook commitment using one of:

- a rulebook idea whose canonical representation commits to a `rulebook_hash`, or
- a dedicated rulebook reference object as defined in Section A2.9, created via events defined by the governing rulebooks.

At minimum, the canonical universe MUST have a deterministic way to reference:
- `rulebook_id`
- `rulebook_version`
- `rulebook_hash`

### A4.6.3 Rulebook activation and deactivation (no standalone activation events) [anchor: a4_6_3_rulebook_activation_and_deactivation_no_standalone_activation_events]

Rulebook activation MUST be **derived deterministically** at cycle boundaries from canonical history, not introduced by discretionary mid-stream events.

Specifically:

- Rulebook activation MUST NOT take effect within an interval.
- The “active rulebook set” for any segment of the log MUST be recoverable deterministically from:
  - governance challenge outcomes recorded in the log,
  - any required implementation/completion truth claims recorded in the log,
  - and the computed `activation_cycle_index` for the rule change.

Accordingly, Appendix A treats rulebook activation as:

- a derived state transition that becomes effective at cycle boundaries, and
- a value recorded in the derived snapshot header and indexed canonically by `snapshot_commit` as `active_rulebook_set_hash` (see A4.9).

### A4.6.4 Suspension / quarantine (OPTIONAL, policy-interface) [anchor: a4_6_4_suspension_quarantine_optional_policy_interface]

If a universe supports policy events for emergency suspension or quarantine of a rulebook deployment, those events MUST be:

- strictly interface-level,
- snapshot-bound in effect,
- and authorized by governance mechanisms defined by active rulebooks.

If supported, such events MUST NOT alter canonical history and MUST NOT delete prior rulebook commitments.

(If you want these events, they should be added later as OPTIONAL event types with explicit authorization requirements. They are not required for Protocol v5 core conformance.)

---


## A4.7 Token accounting events (interface-level) [anchor: a4_7_token_accounting_events_interface_level]

### A4.7.1 `pod_mint` (if explicit) [anchor: a4_7_1_pod_mint_if_explicit]

REQUIRED payload fields:
- `identity_id`
- `amount`
- `source_event_id`

---

### A4.7.2 `point_mint` / `point_distribute` [anchor: a4_7_2_point_mint_point_distribute]

REQUIRED payload fields:
- `identity_id`
- `amount`
- `epoch_id`

---

### A4.7.3 `payout_epoch_finalize` [anchor: a4_7_3_payout_epoch_finalize]

REQUIRED payload fields:
- `epoch_id`
- `snapshot_id`

Effects:
- Finalizes issuance for the epoch.

---

## A4.8 Safety / visibility events (interface-level) [anchor: a4_8_safety_visibility_events_interface_level]

### A4.8.1 `safety_classify` [anchor: a4_8_1_safety_classify]

REQUIRED payload fields:
- `classification_id`
- `target_id`
- `rulebook_id`
- `jurisdiction_lens`
- `explanation_ref`

---

### A4.8.2 `safety_appeal` [anchor: a4_8_2_safety_appeal]

REQUIRED payload fields:
- `classification_id`
- `appeal_representation_ref`

---

### A4.8.3 `safety_override` [anchor: a4_8_3_safety_override]

REQUIRED payload fields:
- `classification_id`
- `governance_authorization_ref`

---

## A4.9 Snapshot artifacts and commit event (MUST) [anchor: a4_9_snapshot_events_must]

Snapshots are deterministic replay checkpoints derived from the canonical event log. Snapshot artifacts are accelerators, not authorities. Canonical truth remains what is reconstructible from replay of the ordered event log under the active protocol/rulebooks.

The only canonical snapshot-related event defined by this appendix is `snapshot_commit`. It is a mechanical boundary/index event authored by `system_boundary_emitter` that references a derived snapshot artifact. The artifact itself is not a canonical event.

---

### A4.9.1 `snapshot_commit` [anchor: a4_9_1_snapshot_commit]

Records the canonical index/attestation for a derived snapshot artifact at a deterministic block boundary.

A conformant `snapshot_commit` MUST allow any conformant node to:
- identify the derived snapshot artifact for a given block height,
- verify the artifact’s commitments against deterministic replay, and
- confirm the active rulebook set hash that applies at that boundary.

REQUIRED payload fields:
- `block_height`
  Deterministic block-height boundary for the snapshot. This MUST align with the active snapshot interval profile.

- `snapshot_hash`
  Canonical hash of the derived snapshot bytes.

- `state_root_hash`
  Deterministic commitment to the canonical derived state at this snapshot boundary.

- `title_sentence_payload_root`
  Deterministic payload-root commitment for the shared/pocket map at this boundary.

- `shared_map_commitment`
  Deterministic combined commitment over state and payload roots for this boundary.

- `last_event_id`
  Event ID of the last canonical event included in the derived snapshot artifact.

- `event_count`
  Total canonical event count through `last_event_id`, inclusive.

- `active_rulebook_set_hash`
  Deterministic hash of the active rulebook set immediately following this snapshot boundary.

OPTIONAL:
- `snapshot_id`
  Legacy alias for external references. When present, it MUST equal `hex(snapshot_hash)`.

INVARIANTS:
- `snapshot_commit` MUST be authored by `system_boundary_emitter`.
- `snapshot_commit` is a replay no-op for canonical semantic state. It is an index/attestation event only.
- The referenced snapshot artifact MUST remain fully derivable from the canonical log plus active rulebooks.
- Any derived block-boundary reference used for scheduling or packaging MUST be mechanically mappable to the same replay prefix and MUST NOT introduce new ordering or validity rules.

#### A4.9.1.1 Deterministic rulebook state at snapshots (derived requirement) [anchor: a4_9_1_1_deterministic_rulebook_activation_at_snapshots_derived_requirement]

The `active_rulebook_set_hash` recorded in `snapshot_commit` MUST be derivable deterministically from canonical history.

Specifically, a snapshot boundary’s active rulebook set MUST be computed from:
- governance challenge verdicts recorded prior to that snapshot boundary,
- any required implementation actions and completion truth claims recorded prior to that boundary,
- cycle-based activation scheduling (`decision_cycle_index`, `change_class`, `delay_policy_version`, `activation_cycle_index`),
- supersession semantics defined by governance rulebooks,
- and Protocol v5 Section 0 invariants.

If required implementation/completion truth claims are missing or invalid at the scheduled activation cycle boundary, the previously active rulebook set MUST remain in force.

Nodes MUST reject `snapshot_commit` events whose `active_rulebook_set_hash` does not match the deterministically derived set.

---

### A4.9.2 `snapshot_create` / `snapshot_adopt` (deprecated or interface-level only) [anchor: a4_9_2_snapshot_adopt_checkpoint_optional_interface_level]

`snapshot_create` is a deprecated legacy alias from earlier drafts. It is NOT REQUIRED for conformance and SHOULD NOT be emitted as a canonical event by new implementations.

`snapshot_adopt` / `checkpoint` MAY exist as a local or interface-level accelerator marker.

Effects:
- Signals that a derived snapshot artifact is usable for faster bootstrapping.
- MUST NOT alter canonical history, object semantics, or derived outcomes.

Invariants:
- Snapshot artifacts are accelerators, not authorities.
- Canonical truth remains what is reconstructible from replay of the ordered event log.

---

### A4.9.3 `cycle_close` (MUST, cycle boundary event) [anchor: a4_9_3_cycle_close_must_cycle_boundary_event]

Records the canonical boundary between cycle `r` and cycle `r+1`.

`cycle_close` is a mechanically emitted boundary event and MUST be authored by `system_boundary_emitter`.

REQUIRED payload fields:
- `cycle_index`
- `closure_kind`  
  Enum: `deliberative` or `forced`.
- `forced_seal`  
  Boolean that MUST match `closure_kind`.
- `closure_boundary_ref`  
  Deterministic boundary reference (block height `H_close`, or equivalent deterministic boundary reference when block heights are unavailable).

Invariants:
- Valid only at the earliest canonical log position where cycle closure predicates are satisfied under Cycle Specification rules.
- Any later `cycle_close` for the same cycle index is invalid.
- MUST NOT be authored by human identities.


---

## A5. Snapshot and block structures (OPTIONAL) [anchor: a5_snapshot_and_derived_block_structures_optional]

This section describes how implementations use finalized prefix certificates and any additional derived structures for integrity, chunking, transport, and replication efficiency. In the staged publication model, prefix certificates assign canonical publication order; derived blocks and other packaging artifacts MUST NOT define canonical ordering, validity, governance, or payout rules.

---

### A5.1 Derived publication blocks (OPTIONAL, Stage 2+) [anchor: a5_1_derived_block_structure_optional_non_canonical]

If a universe enables explicit blocks, blocks are derived publication units defined by `pod-consensus-and-canonical-publication-spec.md`.

Blocks:
- expose canonical publication addresses through `(block_height, intra-block position)`,
- derive those addresses only from already-finalized prefix certificates,
- do not determine truth, importance, governance activation, token payouts, or safety semantics beyond event position,
- and remain replay-verifiable from their ordered canonical contents plus the finalized prefix history.

Nodes MUST still be able to validate canonical history from finalized prefix certificates plus replay, and any additional derived packaging artifact MUST remain mechanically mappable to the same replay prefix without altering validity or ordering.

Any descriptive fields such as timestamps or operator labels that are not explicitly consumed by the canonical publication mechanism are non-authoritative metadata and MUST NOT affect validity, ordering, activation, or payouts.


#### A5.1.1 Block header fields (compatibility mapping) [anchor: a5_1_1_block_header_fields_compatibility_mapping]

If blocks are implemented, the block header fields exposed by an implementation MUST be a lossless representation of the derived publication block header defined in `pod-consensus-and-canonical-publication-spec.md`.

REQUIRED:
- `block_header_hash`
  Block header hash commitment (hex string; not a UUIDv7 identifier).

- `block_height`
  Monotonically increasing derived publication height.

- `parent_block_hash`
  Cryptographic hash of the previous derived block header.

- `event_count`
  The number of canonical events included in the block.

- `events_root`
  Merkle root committing to the ordered list of canonical event hashes included in the block.

- `source_prefix_certificate_hash`
  Hash of the latest finalized prefix certificate whose sequence contributes events to or through this block.

OPTIONAL (non-authoritative metadata only):
- `block_timestamp`  
  Descriptive timestamp only. MUST NOT affect ordering, cycles, snapshots, activation, or payouts.

- `assembler_identity_id`  
  Identity that assembled or served the block header, if recorded.  
  This field confers no authority and MUST NOT affect validity.

#### A5.1.2 Event inclusion rules (derived consistency requirements) [anchor: a5_1_2_event_inclusion_rules_derived_consistency_requirements]

- Events within a block MUST be included in the same deterministic order as canonical ordering in the event log.
- Blocks MUST NOT reorder events or define alternative ordering rules.
- Any block presented for verification MUST be deterministically mappable to an event range, and that event range MUST correspond exactly to the events committed by the block’s Merkle root.

#### A5.1.3 Deterministic encoding and verification [anchor: a5_1_3_deterministic_encoding_and_verification]

Block headers and inclusion proofs MUST be encoded and verified using the canonical rules defined by the Canonical Preservation and Provenance Spine Specification (and its Appendix A).

If an implementation exposes an alternative serialization, it MUST be a pure view over the canonical representation and MUST NOT introduce ambiguity in verification.

---

### A5.2 Snapshot structure [anchor: a5_2_snapshot_structure]

Snapshots provide deterministic replay checkpoints and governance verification commitments.

Snapshots MUST NOT introduce new semantics.
They only summarize and commit to state derivable from the event log.

#### A5.2.1 Snapshot header [anchor: a5_2_1_snapshot_header]

REQUIRED:
- `snapshot_id`
  Canonical identifier (UUIDv7 string) for the snapshot.

- `included_range`  
  Deterministic reference to the portion of the chain covered, expressed as:
  - block height range, or
  - canonical event range.

- `state_root_hash`  
  Deterministic hash of the canonical state at the snapshot boundary.

- `active_rulebook_set`  
  The rulebook set governing the interval immediately following the snapshot.

OPTIONAL:
- `snapshot_kind`  
  Enumerated value such as `full` or `delta`, provided reconstruction rules are defined.

#### A5.2.2 Snapshot payload [anchor: a5_2_2_snapshot_payload]

A snapshot payload MUST contain sufficient data to reconstruct canonical state.

Depending on snapshot kind, this MAY include:

- full canonical state objects:
  - identities
  - ideas
  - connections
  - challenges
  - verdicts
  - representations

- or modular snapshot packs, as defined by the Offline & Mindseed Specification.

If included, snapshots MAY also contain:

- deterministic rank lists by axis / timeframe / scope,
- certainty band assignments,
- POD and POINT balances per identity.

These values MUST be reproducible by replay.

#### A5.2.3 Canonical vs non-canonical data [anchor: a5_2_3_canonical_vs_non_canonical_data]

Snapshots MAY include references to non-canonical metadata, such as:
- Ent training or growth ring metadata,
- client-side optimization data.

Such data MUST be clearly marked as non-canonical and MUST NOT affect replay or validation.

---

### A5.3 Snapshot validity rules [anchor: a5_3_snapshot_validity_rules]

A snapshot is valid if and only if all of the following hold:

- Replaying the ordered event log up to the snapshot boundary produces a canonical state whose hash matches `state_root_hash`.
- The `active_rulebook_set` matches the deterministically derived set at that boundary.
- No field in the snapshot introduces new semantics or overrides replay results.

Nodes MUST reject snapshots that fail any validity rule.

---

## A6. Cross-spec interfaces (bridges) [anchor: a6_cross_spec_interfaces_bridges]

This section defines the **minimal, normative interfaces** between Appendix A and other protocol specifications.
These interfaces ensure separation of concerns while maintaining deterministic behavior.

---

### A6.1 Offline & Mindseed interface [anchor: a6_1_offline_mindseed_interface]

The Offline & Mindseed system enables local operation and later reintegration without breaking canonical determinism.

#### A6.1.1 Seed package contents [anchor: a6_1_1_seed_package_contents]

An offline seed package MUST include, at minimum:
- a sequence of canonical events authored locally,
- identity references sufficient to verify authorship,
- any required representations referenced by those events.

Optional:
- snapshot references used as local starting points.

#### A6.1.2 Packaging and verification [anchor: a6_1_2_packaging_and_verification]

Local logs MUST be packaged with:
- deterministic ordering,
- cryptographic signatures preserved,
- no reliance on external clocks.

Upon reintegration, nodes MUST:
- verify signatures,
- validate events against current rulebooks,
- merge logs deterministically according to replay rules.

---

### A6.2 AI & Ent interface [anchor: a6_2_ai_ent_interface]

AI systems operate strictly outside the canonical universe until human adoption.

#### A6.2.1 Non-canonical drafts [anchor: a6_2_1_non_canonical_drafts]

AI-generated content MUST exist as non-canonical drafts:
- not included in the canonical event log,
- not eligible for POD or POINT,
- not affecting canonical state.

#### A6.2.2 Adoption boundary [anchor: a6_2_2_adoption_boundary]

AI drafts become canonical only when:
- a verified human identity authors a canonical event adopting the content,
- the adopted content satisfies all schema and invariant rules.

When non-canonical/outer-layer content is adopted into the canonical universe:
- the adopting identity MUST be the canonical signed author of the adoption event,
- canonical adoption artifacts MUST NOT include transport metadata or origin network identifiers (e.g., IP addresses, routing headers, or device identifiers),
- source anonymity SHOULD be preserved by default.

Implementations MAY include a non-identifying source reference commitment (e.g., hash/commitment) for provenance without deanonymization, where permitted by active rulebooks and interface schemas.
See `privacy-and-high-risk-submission-spec.md` §6.3 (Adoption Provenance Constraints).

All canonical impact MUST be attributable to a human identity.

---

### A6.3 Roles & Stewardship interface [anchor: a6_3_roles_stewardship_interface]

Roles are **claims**, not powers.

#### A6.3.1 Role representation [anchor: a6_3_1_role_representation]

Role claims MUST be represented using existing primitives:
- role descriptions as ideas,
- role relationships as connections (typically `relative_importance` or `membership` with rulebook-defined usage values).

No new object or connection types are introduced for roles.

#### A6.3.2 Stewardship semantics [anchor: a6_3_2_stewardship_semantics]

Roles such as Ent, Entling, or Lion:
- confer no canonical authority,
- may influence eligibility pools or visibility rules only if explicitly defined by rulebooks,
- remain fully challengeable.

---

### A6.4 Safety specification interface [anchor: a6_4_safety_specification_interface]

Safety affects **visibility and presentation**, not canonical truth.

#### A6.4.1 Required inputs [anchor: a6_4_1_required_inputs]

To support safety classification and explanation, Appendix A guarantees availability of:
- idea and connection identifiers,
- classification rulebook references,
- jurisdiction lenses,
- explanation representation references.

#### A6.4.2 "Why am I seeing this?" [anchor: a6_4_2_why_am_i_seeing_this]

Nodes MUST be able to compute "why am I seeing this?" explanations deterministically from:
- safety classification events,
- active rulebooks,
- snapshot state.

Safety interfaces MUST NOT suppress, alter, or delete canonical history.

---

## A7. Deterministic error handling and rejection semantics [anchor: a7_deterministic_error_handling_and_rejection_semantics]

This section defines how conformant implementations MUST handle invalid, conflicting, or forward-incompatible data.
All error handling MUST be deterministic. No node-local discretion, heuristics, or recovery logic may alter canonical outcomes.

---

### A7.1 Invalid event handling [anchor: a7_1_invalid_event_handling]

Nodes MUST classify invalid events into deterministic categories and respond accordingly.

#### A7.1.1 Reject [anchor: a7_1_1_reject]

An event MUST be **rejected** if any of the following are true:

- envelope validation fails (Section A3),
- signature verification fails,
- schema validation fails,
- required fields are missing,
- forbidden fields are present,
- referenced canonical objects do not exist or are incompatible,
- the author identity is not eligible to author the event,
- protocol invariants (Protocol v5 §0) are violated,
- rulebook constraints deterministically disallow the event,
- the event attempts a state transition not defined in Appendix A.

Rejected events:
- MUST NOT be added to the canonical log,
- MUST NOT affect canonical state,
- MAY be retained locally for diagnostics but MUST NOT influence replay.

#### A7.1.2 Quarantine (OPTIONAL, interface-level) [anchor: a7_1_2_quarantine_optional_interface_level]

Implementations MAY support a **quarantine pool** for events that are syntactically valid but temporarily unverifiable, such as:

- missing referenced objects not yet observed,
- deferred verification dependencies permitted by rulebooks.

If quarantine is supported:

- quarantine behavior MUST be deterministic,
- quarantine MUST NOT affect canonical ordering,
- quarantined events MUST either transition to accepted or rejected deterministically.

Quarantine is an optimization mechanism and MUST NOT be required for protocol conformance.

#### A7.1.3 Ignore [anchor: a7_1_3_ignore]

Nodes MAY **ignore** data that is explicitly non-canonical, including:

- AI drafts not adopted via canonical events,
- client metadata,
- non-canonical snapshot acceleration data.

Ignoring such data MUST NOT affect canonical validation or replay.

---

### A7.2 Conflicting events [anchor: a7_2_conflicting_events]

Canonical conflicts MUST be resolved deterministically using protocol-defined mechanisms.

#### A7.2.1 Direct conflicts [anchor: a7_2_1_direct_conflicts]

If multiple events attempt to mutate the same object in incompatible ways:

- Events MUST NOT be resolved by last-write-wins semantics.
- Events MUST NOT be resolved by author privilege or timing heuristics.

Instead, conflicts MUST be resolved through:

- challenge-mediated processes defined by the protocol, or
- explicit supersession semantics defined by rulebooks.

If no valid conflict-resolution path exists, conflicting events MUST be rejected.

#### A7.2.2 Concurrent attempts [anchor: a7_2_2_concurrent_attempts]

If two valid events are authored concurrently but only one may succeed:

- canonical ordering determines which event is evaluated first,
- the second event MUST be evaluated against the updated state,
- if invalid in the updated state, it MUST be rejected.

Nodes MUST NOT reorder events to resolve conflicts.

---

### A7.2.3 Duplicate challenge creation (deterministic rejection) [anchor: a7_2_3_duplicate_challenge_creation_deterministic_rejection]

A `challenge_create` event MUST be rejected if, at the event’s canonical ordering position, there already exists a non-finalized challenge representing the same **challenge instance**.

A challenge instance is uniquely defined by the tuple:

- `challenge_domain`, and
- the complete set of domain-specific subject anchors, including:
  - subject idea identifiers, and
  - for importance challenges: axis, timeframe, and scope.

For the purposes of this rule, a challenge is considered **non-finalized** if its lifecycle state is any of:

- created,
- open for arguments,
- closed for arguments,
- open for voting,
- closed for voting.

Accordingly:

- At most **one public challenge per challenge instance** MAY exist at any given time.
- Any attempt to create a second concurrent challenge with an identical instance-defining tuple MUST be rejected deterministically.
- Rejection MUST occur during state transition validation and MUST NOT depend on node-local discretion.

Once a challenge instance reaches a finalized terminal state (verdict finalized, cancelled, or superseded):

- a new `challenge_create` event for the same instance-defining tuple MAY be accepted,
- representing a new deliberative round.

This rule prevents vote-splitting, challenge spam, and ambiguity while preserving full revisability over time.


### A7.3 Forward compatibility and schema evolution [anchor: a7_3_forward_compatibility_and_schema_evolution]

Appendix A permits evolution without semantic drift.

#### A7.3.1 Schema versioning [anchor: a7_3_1_schema_versioning]

Canonical events MAY include:

OPTIONAL:
- `schema_version`

If present, `schema_version` MUST:
- be an integer or structured identifier,
- be interpreted deterministically.

Nodes MUST reject events whose schema version is unsupported.

#### A7.3.2 Rulebook-based extensions [anchor: a7_3_2_rulebook_based_extensions]

Rulebooks MAY define extension fields ONLY if:

- extension fields are namespaced,
- extension fields do not alter canonical semantics,
- extension fields are ignored safely by nodes that do not recognize them.

Rulebooks MUST NOT introduce:
- new canonical object types,
- new connection types,
- new event side effects.

#### A7.3.3 Semantic invariants [anchor: a7_3_3_semantic_invariants]

All extensions MUST preserve:

- deterministic replay,
- challengeability of all claims,
- human-first authorship,
- fixed canonical primitives.

Any extension that violates these invariants MUST be rejected.

---

## A8. Conformance requirements for Appendix A [anchor: a8_conformance_requirements_for_appendix_a]

This section defines the minimum requirements for claiming conformance with Appendix A.

---

### A8.1 Node conformance (schema and validation layer) [anchor: a8_1_node_conformance_schema_and_validation_layer]

A conformant node implementation MUST:

- validate all event envelopes per Section A3,
- validate all event payload schemas per Section A4,
- enforce all invariants defined in Protocol v5 §0,
- reject AI-authored canonical events,
- implement deterministic serialization and hashing,
- compute derived fields deterministically,
- validate snapshots against replay results,
- reject snapshots with incorrect rulebook activation.

A node MUST produce identical canonical state given identical input logs.

---

### A8.2 Client conformance (authoring layer) [anchor: a8_2_client_conformance_authoring_layer]

A conformant client implementation MUST:

- construct canonical events that satisfy all schema and envelope requirements,
- bind authorship explicitly to a verified human identity,
- present clear authorship and adoption boundaries,
- prevent AI systems from authoring canonical events directly,
- surface validation failures clearly prior to submission.

Clients MAY assist users with drafting, simulation, or previewing effects, but MUST NOT bypass canonical validation rules.

---

## A9. Test vectors and reference examples (non-normative but essential) [anchor: a9_test_vectors_and_reference_examples_non_normative_but_essential]

This section provides **non-normative** reference examples intended to support
implementation testing, interoperability, and debugging.

Nothing in this section introduces new semantics.
All examples MUST be consistent with the normative requirements in Sections A1–A8.

Implementations MAY use these examples as fixtures, but MUST NOT rely on them as authoritative definitions.

---

### A9.1 Minimal example canonical log [anchor: a9_1_minimal_example_canonical_log]

This example demonstrates the smallest meaningful end-to-end flow:
identity creation → idea creation → connection → challenge → voting → verdict → snapshot.

#### A9.1.1 Event sequence (logical order) [anchor: a9_1_1_event_sequence_logical_order]

1. `identity_create`
2. `identity_verification_update`
3. `idea_create` (truth claim)
4. `idea_create` (counter-idea)
5. `connection_create` (relative_importance argument)
6. `challenge_create` (truth_challenge)
7. `challenge_open_arguments`
8. `challenge_close_arguments`
9. `challenge_open_voting`
10. `challenge_close_voting`
11. `challenge_finalize_verdict`
12. `cycle_close`
13. `snapshot_commit`

#### A9.1.2 Example events (abridged) [anchor: a9_1_2_example_events_abridged]


{
  "event_id": "0191f3d1-2a3b-7c4d-8e5f-1234567890ab",
  "event_type": "identity_create",
  "author_identity_id": "0191f3d1-2a3b-7c4d-8e5f-abcdef123456",
  "ordering_ref": { "block_height": 1, "index": 0 },
  "payload_hash": "hash(identity_create)",
  "signature": "sig(identity_create)"
}



## A10. Voting events (normative) [anchor: a10_voting_events_normative]

This section defines the canonical event schemas used for voting, including optional commit–reveal semantics. These events MUST be interpreted and validated according to the deterministic replay rules in Section A11.

All voting events MUST:

- use the canonical event envelope defined in Section A3,
- be signed by `author_identity_id`,
- reference an existing canonical `challenge_id`,
- reference the voting identity via `author_identity_id` (and MAY redundantly include `voter_identity_id` in payload if a rulebook requires it).

No vote event may be accepted outside the voting window for its associated challenge.

### A10.1 `vote_cast` (single-phase voting) [anchor: a10_1_vote_cast_single_phase_voting]

A `vote_cast` event represents a direct vote submission (no commit–reveal).

#### A10.1.1 Required payload fields [anchor: a10_1_1_required_payload_fields]

REQUIRED:
- `challenge_id`
- `vote_choice`  
  A canonical encoding of the voter’s choice, as defined by the active rulebook for the challenge domain.  
  Examples include: `option_a` / `option_b`, `yes` / `no`, or a canonical subject identifier.

#### A10.1.2 Optional payload fields [anchor: a10_1_2_optional_payload_fields]

OPTIONAL:
- `ballot_style_ref`  
  Reference to a rulebook-defined ballot encoding or schema if multiple ballot formats exist.

- `client_ballot_commitment`  
  Optional client-side commitment or UX checksum intended for user verification only.  
  MUST NOT affect canonical validity.

#### A10.1.3 Effects [anchor: a10_1_3_effects]

Effects:
- Records a vote attempt by `author_identity_id` for `challenge_id`.

A `vote_cast` event does not, by itself, guarantee acceptance. Acceptance is determined by Section A11 replay rules.

---

### A10.2 `vote_commit` (commit–reveal voting) [anchor: a10_2_vote_commit_commit_reveal_voting]

A `vote_commit` event represents a cryptographic commitment to a future vote reveal.

#### A10.2.1 Required payload fields [anchor: a10_2_1_required_payload_fields]

REQUIRED:
- `challenge_id`
- `commit_hash`  
  A cryptographic commitment computed as:

```

commit_hash = Hash(vote_choice || nonce)

```

Where `nonce` is a voter-chosen secret value.

#### A10.2.2 Optional payload fields [anchor: a10_2_2_optional_payload_fields]

OPTIONAL:
- `commit_scheme`  
A rulebook-defined identifier of the commitment scheme version, if multiple exist.

#### A10.2.3 Effects [anchor: a10_2_3_effects]

Effects:
- Records a commitment by `author_identity_id` for `challenge_id`.

A `vote_commit` event does not count as a vote. It only enables a later reveal.

---

### A10.3 `vote_reveal` (commit–reveal voting) [anchor: a10_3_vote_reveal_commit_reveal_voting]

A `vote_reveal` event discloses a previously committed vote.

#### A10.3.1 Required payload fields [anchor: a10_3_1_required_payload_fields]

REQUIRED:
- `challenge_id`
- `vote_choice`
- `nonce`

#### A10.3.2 Verification requirement [anchor: a10_3_2_verification_requirement]

During deterministic replay, implementations MUST verify:

```

Hash(vote_choice || nonce) == commit_hash

```

Where `commit_hash` is taken from the accepted `vote_commit` by the same `author_identity_id` for the same `challenge_id`.

If verification fails, the reveal is invalid.

#### A10.3.3 Effects [anchor: a10_3_3_effects]

Effects:
- Records a reveal attempt by `author_identity_id` for `challenge_id`.

Acceptance is determined by Section A11 replay rules and applicable rulebook windows.

---

## A11. Deterministic vote acceptance rules (normative) [anchor: a11_deterministic_vote_acceptance_rules_normative]

This section defines the binding replay rules that determine which vote attempts become effective.

These rules apply regardless of packaging (blocks/segments), transport, or node implementation details.

### A11.1 Eligibility requirement [anchor: a11_1_eligibility_requirement]

A vote attempt MUST be rejected during replay if the voting identity was not eligible for that challenge according to the active rulebook at the deterministic eligibility freeze boundary.

Eligibility freeze boundary:

- non-governance challenges: voting-open boundary (the cycle boundary at which `challenge_open_voting` becomes effective),
- governance challenges: challenge-open boundary.

Eligibility is a rulebook-defined gate and MUST NOT weight votes.

Eligibility pool membership is independent of current mana/session capacity. A vote attempt (or vote-session capacity reservation, where applicable) MUST be rejected if the voting identity lacks sufficient deterministic voting capacity at that canonical position.

### A11.2 Voting window requirement [anchor: a11_2_voting_window_requirement]

A vote attempt MUST be rejected during replay if it is outside the challenge’s voting window as defined by the active rulebook.

For commit–reveal voting, rulebooks MUST define:

- a commit window, and
- a reveal window.

Votes MUST satisfy the relevant window(s) to be accepted.

### A11.3 One-vote-per-identity rule [anchor: a11_3_one_vote_per_identity_rule]

For any challenge, **at most one vote per eligible identity MAY be accepted**.

For each `(challenge_id, author_identity_id)` pair:

- the first vote that satisfies all validity rules MUST be accepted, and
- all subsequent vote attempts for that pair are invalid.

This rule applies across all voting event types (`vote_cast`, `vote_commit`/`vote_reveal`) and regardless of ordering, packaging, or inclusion context.

### A11.4 Commit–reveal acceptance rule (conditional) [anchor: a11_4_commit_reveal_acceptance_rule_conditional]

If commit–reveal voting is required by the active rulebook:

- `vote_cast` MUST be rejected for that challenge.
- A vote is accepted only if:
  - a valid `vote_commit` exists for the pair `(challenge_id, author_identity_id)` within the commit window, and
  - a valid `vote_reveal` exists within the reveal window whose `(vote_choice, nonce)` matches the committed hash.

Failure to reveal within the reveal window MUST be treated as defined by the active rulebook (e.g., abstention or invalid).

### A11.5 Invalid votes remain in history [anchor: a11_5_invalid_votes_remain_in_history]

Invalid vote events remain part of immutable canonical history but MUST have no effect on tallies, verdicts, or state transformations.

---

## A12. Identity status changes and post-hoc fraud handling (normative) [anchor: a12_identity_status_changes_and_post_hoc_fraud_handling_normative]

This section defines how post-hoc discovery of fraudulent, automated, or misverified identities interacts with voting history.

### A12.1 Identity status events (interface-level, canonical) [anchor: a12_1_identity_status_events_interface_level_canonical]

If supported by the active rulebooks, the protocol MAY include canonical identity status events such as:

- `identity_flagged`
- `identity_revoked`
- `identity_restored`

These events MUST:

- be challengeable through the protocol’s standard challenge mechanisms,
- affect future eligibility and confidence overlays,
- not rewrite or delete historical events.

(If your Appendix A already enumerates identity status event types in A4.1, this section is normative guidance for their effects.)

### A12.2 No retroactive invalidation [anchor: a12_2_no_retroactive_invalidation]

Discovery of fraudulent, automated, or misverified identities MUST NOT retroactively invalidate:

- previously accepted votes,
- finalized challenge verdicts,
- historical state transitions applied due to those verdicts.

Canonical history is immutable.

Any correction MUST occur via new canonical events (e.g., new challenges, new verdicts, new governance decisions), not retroactive deletion or reinterpretation.

---

## A13. Derived confidence annotations for fraud exposure (normative) [anchor: a13_derived_confidence_annotations_for_fraud_exposure_normative]

This section defines mandatory derived outputs that help users and downstream systems assess risk when fraud is discovered, without altering canonical replay outcomes.

### A13.1 Required derived annotations [anchor: a13_1_required_derived_annotations]

Conformant implementations MUST compute derived, non-canonical confidence annotations that can be attached (in UI/lenses) to:

- challenges and verdicts, and
- ideas or rankings that depend on those verdicts.

At minimum, annotations MUST include:

- `flagged_voter_count`  
  Number of accepted votes cast by identities later flagged or revoked.

- `flagged_voter_share`  
  Proportion of accepted votes attributable to flagged or revoked identities.

- `affected_outcome_margin` (if applicable)  
  A computed indicator of whether flagged votes could plausibly have changed the outcome, as defined deterministically by rulebook tally semantics.

- `recommended_followup`  
  A deterministic suggestion (e.g., “re-challenge recommended”) computed from rulebook thresholds, if configured.

### A13.2 No effect on deterministic replay [anchor: a13_2_no_effect_on_deterministic_replay]

Derived confidence annotations:

- MUST NOT alter deterministic replay results,
- MUST NOT change tallies or verdicts,
- MUST NOT modify canonical objects.

They exist solely to surface risk and trigger human/governance follow-up through normal challenge processes.

### A13.3 Rulebook-triggered review thresholds (optional) [anchor: a13_3_rulebook_triggered_review_thresholds_optional]

Rulebooks MAY define deterministic thresholds such that:

- if `flagged_voter_share` exceeds a threshold, or
- if `affected_outcome_margin` indicates potential outcome sensitivity,

then the system MAY:

- mark the verdict as “review recommended,” and/or
- make a re-challenge eligible under reduced friction, as defined by the rulebook.

Any such trigger MUST be implemented as a visibility / process recommendation and MUST NOT rewrite canonical outcomes.
```





