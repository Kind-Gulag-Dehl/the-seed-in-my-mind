---
doc_id: canonical_encoding_and_hashing_spec
title: Canonical Encoding and Hashing Specification
status: authoritative
version: v0
last_reviewed: 2026-02-11

scope:
  - Defines canonical encodings and hashing/commitment rules used by the protocol.

authoritative_for:
  - Canonical serialization formats and normalization rules.
  - What bytes are hashed and how commitments are computed.

not_authoritative_for:
  - Governance rules and pacing policy.
  - Challenge lifecycle semantics beyond encoding surfaces.
  - Ordinary human-authorship signature algorithms, signed event-candidate fields, or replay-derived key state.

depends_on:
  - protocol v5.md
  - canonical-event-authorship-and-signature-profile-v0.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of snapshot-format-v0.md and shared-map-and-payload-bundles-spec.md.
  - Any change here requires review of node-and-conformance-spec.md test expectations.

reader_path:
  - prereq: protocol v5.md
  - next: deterministic-replay-and-merge-spec.md

keywords:
  - canonical encoding
  - hashing
  - commitments
  - serialization
  - normalization
---

## Canonical Encoding and Hashing Specification [anchor: canonical_encoding_and_hashing_specification]

### 0. Purpose and Scope [anchor: 0_purpose_and_scope]

This document defines the **authoritative, canonical rules** for encoding data into bytes and computing cryptographic commitments within the system.

Its purpose is to ensure that:

* All compliant implementations (across languages, platforms, and time) produce **identical byte streams** for the same canonical data.
* All cryptographic hashes, Merkle roots, and commitments are **deterministic, comparable, and verifiable** across nodes, offline partitions, and archival snapshots.
* Canonical facts are cleanly separated from derived views, indexes, caches, or local representations.

This document is **normative**. Where conflicts arise, this specification takes precedence over informal descriptions, implementation details, or derived artifacts.

This document defines:

* Primitive byte encodings
* Identifier encoding
* Text and payload canonicalization
* Hashing inputs and domain separation prerequisites

This document does **not** define:

* Governance processes
* Rulebook semantics
* Ranking algorithms
* Token economics
* Human-authorship signature algorithms or identity key-state semantics
* UI or presentation-layer behavior

Relationship to other specifications:

* **Snapshot Format v0** relies on this document for all section hashes, Merkle roots, and commitments.
* **Canonical Event Authorship and Signature Profile v0** relies on this document for primitive field encodings, canonical UUIDv7 `id` encoding, hash32 bytes, canonical text bytes, payload hashes, and domain-separated hash primitives, but it owns ordinary authored-event signature profiles and signed-byte layouts.
* **Offline and Mindseed Specification** relies on this document for portable verification, custody manifests, and reconciliation.
* **Node and Conformance Specification** relies on this document for test vectors and determinism guarantees.
* **Preservation Spine** defines higher-level archival strategy but MUST use the encodings defined here.

Normative keywords **MUST**, **MUST NOT**, **SHOULD**, **MAY** are to be interpreted as described in RFC 2119.

---

### 1. Canonical Byte Model [anchor: 1_canonical_byte_model]

The canonical representation of all data in the system is a **sequence of bytes**.

All higher-level structures (objects, lists, trees, sections) are reduced to bytes according to deterministic rules defined in this document and referenced specifications.

Rules:

1. There is **no implicit structure** beyond what is explicitly encoded.
2. No encoding may depend on:

   * programming language object layout,
   * platform endianness defaults,
   * serialization libraries,
   * locale, timezone, or system settings.
3. Any ambiguity in encoding MUST result in rejection.

A canonical byte sequence MUST:

* Be reproducible exactly from canonical inputs.
* Be verifiable without reference to external state.
* Be identical across all compliant implementations.

Encodings that allow multiple byte representations for the same logical value are **forbidden**.

---

### 2. Primitive Encodings [anchor: 2_primitive_encodings]

#### 2.1 Integers [anchor: integers]

The system uses the following unsigned integer primitives:

* `u8`  — 8-bit unsigned integer
* `u16` — 16-bit unsigned integer
* `u32` — 32-bit unsigned integer
* `u64` — 64-bit unsigned integer

Encoding rules:

* All multi-byte integers MUST be encoded in **big-endian** (network byte order).
* Integer primitive fields (`u8`, `u16`, `u32`, `u64`) MUST be encoded using their full declared width (no variable-length encoding for primitive integer values).
* Values outside the representable range MUST be rejected.
* Negative integers are not permitted unless explicitly defined elsewhere.

There is no implicit integer sizing. The integer width is determined solely by the schema using it.

Clarification on length framing:

* Variable-size byte containers (for example UTF-8 strings and other byte vectors where a schema requires a length prefix) MAY use unsigned varint length framing.
* In this specification, `varint` means unsigned LEB128/base-128 with 7-bit groups and continuation bit `0x80`, encoded in minimal form.
* Varint framing is used only for container lengths where explicitly defined by schema; it does not change fixed-width primitive integer encoding requirements above.
* `encode_id` remains explicitly `u32(length) || ASCII_BYTES` as defined in section 2.2.

---

#### 2.2 Identifiers (`id`) [anchor: identifiers_id]

All canonical identifiers in the system use **UUIDv7 strings** with strict constraints.

Identifier requirements:

* MUST be exactly **36 characters**
* MUST use **lowercase hexadecimal** with hyphens in the canonical 8-4-4-4-12 form
* MUST have UUID version **7** and RFC 4122 variant (8, 9, a, or b)
* MUST NOT use uppercase letters or alternate encodings, aliases, or transformations

Canonical encoding of an `id`:

```
id := u32(length) || ASCII_BYTES
```

Where:

* `length` MUST be `36`
* `ASCII_BYTES` are the 36 ASCII characters of the UUIDv7 string

Validation rules:

* If `length ≠ 36`, reject.
* If any character is not valid lowercase hex or a hyphen at the required positions, reject.
* If the UUID version is not 7 or the variant is not RFC 4122, reject.
* If uppercase or mixed-case encodings are encountered, reject.
* Implementations MUST NOT normalize or auto-correct IDs.

Ordering rules:

* When comparing identifiers, comparisons MUST be performed on the **encoded bytes exactly as stored**, including the `u32` length prefix, unless a specification explicitly states otherwise.
* Identifier comparison MUST NOT be used to derive canonical ordering; canonical ordering is defined by the canonical log.

This encoding ensures:

* Stable ordering
* Explicit length binding
* Resistance to subtle cross-language differences

---

#### 2.3 Enums [anchor: enums]

Enumerations are encoded as unsigned integers.

Rules:

* Enums MUST be encoded as `u8` unless otherwise specified.
* Enum values are **stable and append-only**.
* Reordering or renumbering enum values is forbidden.
* Unknown enum values MUST be rejected unless explicitly marked as forward-compatible in the referencing specification.

Enum meaning is defined by the specification that introduces it, not by this document.

---

### 3. Text and Payload Canonicalization [anchor: 3_text_and_payload_canonicalization]

#### 3.1 Character Encoding [anchor: character_encoding]

All textual payloads MUST be encoded in **UTF-8**.

Rules:

* UTF-8 byte sequences MUST be valid.
* Invalid or overlong UTF-8 encodings MUST be rejected.
* No alternate encodings (UTF-16, UTF-32, locale-specific encodings) are permitted.

There is no implicit BOM. Presence of a BOM MUST be rejected.

---

#### 3.2 Normalization Policy [anchor: normalization_policy]

To preserve human meaning while preventing ambiguity, the following normalization rules apply:

* Unicode normalization form: **NFC**
* Line endings MUST be normalized to `\n` (LF, byte `0x0A`)
* Carriage return characters (`\r`) MUST NOT appear after normalization
* No trimming, collapsing, or rewrapping of whitespace is performed

Whitespace is **significant**. Two payloads differing only by whitespace are distinct unless a higher-level process explicitly challenges or reconciles them.

---

#### 3.3 Payload Hashing [anchor: payload_hashing]

Textual payloads are canonicalized as follows before hashing:

1. Interpret bytes as UTF-8
2. Apply Unicode NFC normalization
3. Normalize line endings to LF
4. Encode back to UTF-8 bytes
5. Hash the resulting byte sequence

Rules:

* The hash input MUST be exactly the normalized byte sequence.
* Empty payloads are permitted and hash the empty byte sequence.
* Missing payloads are not equivalent to empty payloads and MUST be explicitly represented as absent in schemas that allow omission.

Payload hashes represent **human-readable content**, not derived interpretations.

For representations where `payload_embedded = true`, the embedded bytes MUST match the canonicalized payload bytes exactly.

### 4. Structured Object Encoding [anchor: 4_structured_object_encoding]

This section defines how structured data objects are reduced to canonical byte sequences.

All structured encodings MUST be deterministic, explicit, and fully specified. No implicit defaults or schema-inferred behavior is permitted.

---

#### 4.1 Field Ordering [anchor: field_ordering]

For any structured object with multiple fields:

* Fields MUST be encoded in a fixed, specification-defined order.
* Field order MUST NOT depend on:

  * insertion order,
  * alphabetical sorting,
  * language-level map or dictionary behavior,
  * runtime reflection or serialization libraries.

If a specification defines a structure, it MUST define the exact field order used for encoding.

Encoding rules:

* Each field is encoded sequentially according to its defined type.
* No field names or keys are included in the byte stream unless explicitly specified.
* Field boundaries are determined solely by the encoding rules of each field’s type.

If two implementations encode the same logical object but produce different byte orders, at least one is non-conformant.

---

#### 4.2 Optional Fields [anchor: optional_fields]

Optional fields MUST be encoded using an explicit presence marker.

Rules:

* Each optional field MUST be preceded by a `u8` presence flag:

  * `0x00` = field absent
  * `0x01` = field present
* If the field is absent, no bytes for that field follow.
* If the field is present, the field value MUST be encoded immediately after the presence flag using its canonical encoding.

Optional fields MUST NOT be omitted implicitly.

Explicit absence is part of the canonical encoding and affects hashes and commitments.

---

#### 4.3 Lists and Repeated Fields [anchor: lists_and_repeated_fields]

Lists and repeated fields are encoded as follows:

```
list := u32(count) || element_1 || element_2 || ... || element_n
```

Rules:

* `count` MUST equal the number of encoded elements.
* Elements MUST be encoded in the order defined by the specification that introduces the list.
* Lists MUST NOT be implicitly sorted unless explicitly stated.
* Empty lists are permitted and encoded with `count = 0`.

Lists with differing element orders are distinct unless ordering is explicitly defined as irrelevant.

---

#### 4.4 Section Encoding [anchor: section_encoding]

Canonical sections (as used in snapshots and archives) are encoded as:

```
section := u16(section_id) || u32(section_length) || section_body_bytes
```

Rules:

* `section_id` uniquely identifies the section type.
* `section_length` is the byte length of `section_body_bytes`.
* `section_body_bytes` MUST be encoded deterministically according to this document and the referencing specification.
* Sections MUST NOT overlap or nest unless explicitly defined.

Section IDs and their meanings are defined by the Snapshot Format and related specifications.

---

### 5. Hashing Primitives [anchor: 5_hashing_primitives]

This section defines the cryptographic hashing rules used throughout the system.

All cryptographic commitments rely on these primitives.

---

#### 5.1 Hash Algorithm [anchor: hash_algorithm]

The system uses a single fixed cryptographic hash algorithm.

Rules:

* The hash algorithm MUST be explicitly named and versioned.
* The algorithm MUST produce a fixed-length output.
* The same algorithm MUST be used everywhere unless a specification explicitly defines a transition mechanism.

Hash outputs are treated as opaque byte sequences.

The hash algorithm is assumed to provide:

* preimage resistance,
* second-preimage resistance,
* collision resistance.

No additional cryptographic properties are assumed.

---

#### 5.2 Domain Separation [anchor: domain_separation]

All hashes MUST be domain-separated.

A domain-separated hash is computed as:

```
HASH(domain_tag || payload_bytes)
```

Rules:

* `domain_tag` MUST be a fixed ASCII byte string defined by the specification invoking the hash.
* Domain tags MUST be unique across distinct hash purposes.
* Domain tags MUST NOT be reused for different semantic roles.
* The domain tag MUST be included exactly once and MUST precede the payload bytes.

Domain separation prevents cross-context hash reuse and ambiguity.

---

### 6. Merkle Construction Rules [anchor: 6_merkle_construction_rules]

Merkle trees are used to commit to ordered sets of canonical byte sequences.

This section defines the authoritative construction rules.

---

#### 6.1 Leaf Construction [anchor: leaf_construction]

Each Merkle leaf is constructed from a canonical byte sequence defined by the referencing specification.

Rules:

* Leaf bytes MUST be exactly the canonical encoding defined for that leaf.
* Leaf hashes are computed as:

  ```
  leaf_hash = HASH(domain_tag_leaf || leaf_bytes)
  ```
* Leaves MUST be sorted according to the rules defined by the referencing specification before tree construction.

If no explicit sorting rule is defined, leaves MUST be sorted by raw bytewise comparison of `leaf_bytes`.

---

#### 6.2 Internal Node Hashing [anchor: internal_node_hashing]

Internal nodes are computed pairwise.

Rules:

* Adjacent child hashes are concatenated in order:

  ```
  node_bytes = left_child_hash || right_child_hash
  ```
* The parent hash is computed as:

  ```
  parent_hash = HASH(domain_tag_node || node_bytes)
  ```
* Tree construction proceeds upward until a single root hash is produced.

Odd-node handling:

* If a level has an odd number of nodes, the final node is paired with itself.
* No padding, reordering, or special casing is permitted.

---

#### 6.3 Root Semantics [anchor: root_semantics]

A Merkle root hash commits to:

* the exact set of leaf byte sequences,
* their exact ordering,
* the full tree structure as constructed under these rules.

A Merkle root hash does **not** commit to:

* any interpretation of the data,
* derived views, indexes, or caches,
* external metadata not included in the leaf bytes.

Any change to a single leaf byte, ordering rule, or construction step MUST produce a different root hash.


### 7. Canonical Commitments [anchor: 7_canonical_commitments]

This section defines the canonical cryptographic commitments used to bind system state, payloads, and shared reality.

All commitments defined here MUST be computable deterministically from canonical encodings defined in this document and referenced specifications.

---

#### 7.1 Section Hashes [anchor: section_hashes]

Each canonical section is hashed independently to produce a section hash.

Section hash computation:

```
section_hash = HASH(domain_tag_section || section_id || section_body_bytes)
```

Rules:

* `section_id` MUST be encoded as `u16` using big-endian byte order.
* `section_body_bytes` MUST be the exact canonical encoding of the section body.
* `domain_tag_section` MUST be unique to section hashing and MUST NOT be reused for other purposes.

Section hashes bind:

* the section type,
* the section contents,
* the exact byte encoding.

Any change to section contents or encoding MUST change the section hash.

---

#### 7.2 `state_root_hash` [anchor: state_root_hash]

The `state_root_hash` commits to the complete set of **canonical facts** produced by deterministic replay of the canonical event log under the active rulebooks at the snapshot boundary.

The `state_root_hash` MUST be computed as a Merkle root over the following canonical sections:

* identities
* ideas
* representations
* connections
* challenges
* verdicts
* rulebook_set

The following sections are explicitly **excluded** from `state_root_hash`:

* rankings
* token_balances
* safety_classifications
* idea_tags
* indexes, caches, or performance-oriented structures

Rules:

* Each included section MUST be hashed using the section hashing rules defined in §7.1.
* Section hashes MUST be sorted by ascending `section_id` before Merkle construction.
* The Merkle root of these section hashes is the `state_root_hash`.

The `state_root_hash` represents the immutable factual substrate of the system.

Changes to:

* ranking algorithms,
* token formulas,
* safety or visibility rulebooks,
* derived view computation logic

MUST NOT affect the `state_root_hash`.

---

#### 7.3 Payload Roots [anchor: payload_roots]

Payload roots commit to sets of human-readable payloads associated with represented canonical objects (ideas and rails for Tier 0).

Payload roots are defined per payload tier.

For Tier 0 (title and sentence payloads), the Tier 0 payload root is computed as a Merkle root over leaf entries defined as:

```
leaf_bytes = u8(object_kind) || encode_id(object_id) || u8(tier_enum) || hash32(payload_hash)
```

Rules:

* `object_kind` is `0` for `idea` and `1` for `rail`.
* `encode_id(object_id)` MUST follow the identifier encoding defined in §2.2.
* `tier_enum` is `0` for `title` and `1` for `sentence`.
* `payload_hash` MUST be the hash of the canonicalized payload bytes (§3).
* Leaves MUST be sorted by raw bytewise comparison of `leaf_bytes` prior to Merkle construction.
* Merkle construction MUST follow §6.

Payload roots commit to:

* which payloads exist,
* which ideas they belong to,
* which tier they represent,
* the exact canonicalized payload bytes.

Payload roots do not commit to presentation, ranking, or interpretive metadata.

---

#### 7.4 Canonical publication artifacts [anchor: canonical_publication_artifacts]

The canonical publication artifacts defined by `pod-consensus-and-canonical-publication-spec.md` MUST use canonical field ordering and canonical primitive encodings from this specification.

The following artifacts are normative:

* `AvailabilityAttestation`
* `PrefixCertificate`
* `DerivedPublicationBlock`

Rules:

* Each artifact MUST serialize fields exactly in the order defined by its owner specification.
* Each artifact MUST use a distinct domain separation tag for its primary hash:
  * `"availability_attestation_v1"`
  * `"prefix_certificate_v1"`
  * `"derived_publication_block_v1"`
* Merkle roots embedded in publication artifacts MUST use the Merkle construction rules in §6.
* Signatures over publication artifacts MUST sign the artifact hash derived from the canonical encoded body, not a local transport wrapper.
* Ordinary human-authored event signatures are not publication-artifact signatures. They MUST follow `canonical-event-authorship-and-signature-profile-v0.md` and MUST NOT sign publication-assigned positions.
* Transport-specific envelopes, compression layers, or batching containers MUST NOT affect the canonical bytes or hashes of these artifacts.

These rules ensure that publication finality, omission proofs, and derived block packaging remain replay-verifiable across implementations.

---

### 8. Shared Map Commitment [anchor: 8_shared_map_commitment]

The shared map commitment represents the system's collectively agreed, human-readable map of reality.

The shared map commitment at height `H` is defined as:

```
shared_map_commitment(H) =
  HASH("shared_map_commitment_v0" ||
       state_root_hash(H) ||
       pocket_map_payload_root(H))
```

Where:

* `state_root_hash(H)` commits to canonical facts (§7.2),
* `pocket_map_payload_root(H)` commits to the selected set of payloads distributed as the shared map.

Properties:

* The shared map commitment changes if and only if:

  * canonical facts change, or
  * the committed payload set changes.
* Changes to derived views, rulebooks, or interpretation logic MUST NOT affect the shared map commitment unless they introduce new canonical facts.

The shared map commitment enables:

* global comparability of shared reality,
* offline verification,
* censorship-resistant replication of meaning.

---

### 9. Ordering and Comparison Semantics [anchor: 9_ordering_and_comparison_semantics]

This section defines the authoritative ordering and comparison rules used throughout the system.

Ordering MUST be deterministic and based solely on canonical encodings.

---

#### 9.1 Identifier Ordering [anchor: identifier_ordering]

Identifiers are ordered by bytewise comparison of their canonical encoded form.

Rules:

* Comparisons MUST operate on the full encoded `id` byte sequence, including the `u32` length prefix.
* No locale, numeric, or timestamp-based interpretation is permitted during comparison.
* Identifier ordering MUST NOT be used as a canonical ordering source; the canonical log defines ordering.

---

#### 9.2 Bytewise Ordering [anchor: bytewise_ordering]

Where ordering of arbitrary canonical byte sequences is required:

* Ordering MUST be performed using unsigned bytewise lexicographic comparison.
* Comparison proceeds from the first byte to the last.
* Shorter byte sequences are considered smaller only if all shared prefix bytes are equal.

---

#### 9.3 Stability Guarantees [anchor: stability_guarantees]

All ordering rules defined here are stable across:

* implementations,
* programming languages,
* platforms,
* time.

Any deviation from these ordering rules results in non-conformant behavior and MUST be rejected by verification logic.


### 10. Validation and Rejection Rules [anchor: 10_validation_and_rejection_rules]

This section defines mandatory validation checks and rejection conditions for canonical encoding, hashing, and verification.

All compliant implementations MUST enforce these rules strictly.

---

#### 10.1 Hard Rejection Conditions [anchor: hard_rejection_conditions]

An implementation MUST reject data if any of the following are observed:

* Invalid or ambiguous byte encodings.
* Integers encoded with incorrect width or endianness.
* Identifiers that:

  * are not exactly 36 characters,
  * contain uppercase characters,
  * do not match canonical UUIDv7 lowercase hex with hyphens,
  * are not UUID version 7 or RFC 4122 variant,
  * use alternate or normalized encodings.
* Invalid UTF-8 sequences or presence of a BOM.
* Payload bytes that do not match their declared payload hash.
* Canonical sections whose declared lengths do not match actual byte length.
* Merkle roots that do not match recomputed roots under the rules of this specification.
* Section hashes, state roots, or shared map commitments that cannot be recomputed exactly.
* Use of a domain tag outside its defined purpose.

Hard rejections invalidate the entire artifact (snapshot, bundle, manifest, or verification transcript).

---

#### 10.2 Soft Rejection and Forward Compatibility [anchor: soft_rejection_and_forward_compatibility]

Soft rejection MAY apply only where explicitly allowed by a referencing specification.

Rules:

* Unknown enum values MUST be rejected unless explicitly marked as forward-compatible.
* Unknown optional sections MAY be ignored only if:

  * they are explicitly marked as optional, and
  * they are excluded from canonical commitments.
* Implementations MUST NOT silently coerce, normalize, or reinterpret invalid data.

Soft rejection MUST NOT allow canonical facts to be altered or inferred.

---

#### 10.3 Validation Scope [anchor: validation_scope]

Validation MUST be performed over:

* canonical encodings,
* payload canonicalization,
* section hashes,
* Merkle constructions,
* commitment recomputation.

Validation MUST be possible:

* offline,
* without external services,
* without trust in the producing implementation.

---

### 11. Security and Adversarial Considerations [anchor: 11_security_and_adversarial_considerations]

This section outlines known adversarial risks and the guarantees provided by this specification.

---

#### 11.1 Ambiguity Attacks [anchor: ambiguity_attacks]

Ambiguity attacks attempt to produce multiple encodings for the same logical content.

Mitigations:

* Single canonical encoding per value.
* Explicit field ordering and presence markers.
* Strict rejection of alternate or normalized encodings.

Any ambiguity is treated as a verification failure.

---

#### 11.2 Cross-Implementation Drift [anchor: cross_implementation_drift]

Different implementations may diverge if rules are underspecified.

Mitigations:

* Byte-level encoding definitions.
* Fixed ordering semantics.
* Mandatory domain separation.
* Explicit rejection conditions.

Conformance fixtures SHOULD be used to detect drift early.

---

#### 11.3 Offline Partition Risks [anchor: offline_partition_risks]

Offline partitions may compute derived views under different rulebooks.

Mitigations:

* Canonical facts are immutable and committed via `state_root_hash`.
* Derived views are excluded from canonical commitments.
* Reconciliation relies on factual replay, not derived outputs.

Offline recomputation MUST NOT alter canonical history.

---

#### 11.4 Payload Withholding [anchor: payload_withholding]

An adversary may distribute commitments without corresponding payload bytes.

Mitigations:

* Payload roots bind payload existence.
* Custody manifests and bundle tiers define expected payload coverage.
* Verification can detect missing payloads even if content is unavailable.

Payload availability affects usability, not factual integrity.

---

### 12. Conformance Requirements [anchor: 12_conformance_requirements]

This section defines the minimum requirements for an implementation to claim conformance.

---

#### 12.1 Required Capabilities [anchor: required_capabilities]

A conformant implementation MUST be able to:

* Encode all canonical primitives defined in this specification.
* Canonicalize and hash textual payloads.
* Compute section hashes, Merkle roots, and commitments.
* Verify snapshots, bundles, and manifests deterministically.
* Reject non-conformant encodings.

---

#### 12.2 Optional Capabilities [anchor: optional_capabilities]

An implementation MAY additionally support:

* Payload bundle generation and management.
* Offline reconciliation tooling.
* Conformance test generation.
* Visualization or inspection utilities.

Optional capabilities MUST NOT alter canonical behavior.

---

#### 12.3 Conformance Testing [anchor: conformance_testing]

Implementations SHOULD be tested against shared conformance fixtures.

Conformance fixtures MAY include:

* canonical encodings with expected byte output,
* known payload hashes,
* Merkle trees with fixed roots,
* full snapshot verification cases.

Failure to pass required fixtures indicates non-conformance.

---

#### 12.4 Evolution and Stability [anchor: evolution_and_stability]

This specification is versioned.

Rules:

* Backward-incompatible changes MUST require a new major version.
* Canonical encodings and hashing rules MUST NOT change retroactively.
* New features MUST preserve existing commitments unless explicitly superseded by governance.

Once adopted, this specification defines the permanent cryptographic foundation of the system.


### 13. Versioning and Evolution [anchor: 13_versioning_and_evolution]

This specification defines the canonical encoding and hashing rules that underpin all cryptographic commitments in the system. Stability and long-term verifiability are primary design goals.

---

#### 13.1 Version Identification [anchor: version_identification]

This specification has an explicit version identifier.

Rules:

* Every artifact (snapshot, bundle, manifest, or verification transcript) that relies on this specification MUST record the version identifier it conforms to.
* For artifacts whose format version is explicitly bound to this specification version (e.g., Snapshot Format v0), that format_version satisfies this requirement.
* Version identifiers MUST be explicit and MUST NOT be inferred from context.

---

#### 13.2 Backward Compatibility [anchor: backward_compatibility]

Backward compatibility guarantees are strict.

Rules:

* Canonical encodings, hashing rules, and Merkle construction rules defined in this specification MUST NOT change retroactively.
* Any change that would alter:

  * canonical byte encodings,
  * hash inputs,
  * Merkle tree structure,
  * ordering semantics,
    is a **backward-incompatible change**.

Backward-incompatible changes MUST require a new major version of this specification.

---

#### 13.3 Forward Evolution [anchor: forward_evolution]

Forward evolution is permitted only under the following constraints:

* New sections, fields, or encodings MAY be added only if:

  * they are explicitly optional, and
  * they are excluded from existing canonical commitments.
* New enum values MAY be appended if explicitly defined as forward-compatible.

No forward evolution may weaken determinism, ambiguity resistance, or verification guarantees.

---

#### 13.4 Deprecation [anchor: deprecation]

Deprecation rules:

* Deprecated features MUST remain verifiable indefinitely.
* Deprecated encodings MUST continue to be recognized for verification.
* Removal of deprecated features is forbidden unless a new major version is adopted and explicitly referenced.

Historical artifacts MUST remain verifiable forever.

---

### Appendix A. Example Encodings (Non-Normative) [anchor: appendix_a_example_encodings_non_normative]

This appendix provides illustrative examples to aid implementers. These examples are not authoritative.

---

#### A.1 Identifier Encoding Example [anchor: a_1_identifier_encoding_example]

Logical ID:

```
018f5c2a-7c4d-7b2e-8f9a-9d2b5a0c3f1d
```

Canonical byte encoding:

```
00 00 00 24 30 31 38 66 35 63 32 61 2D 37 63 34 64 2D 37 62 32 65 2D 38 66 39 61 2D 39 64 32 62 35 61 30 63 33 66 31 64
```

---

#### A.2 Payload Canonicalization Example [anchor: a_2_payload_canonicalization_example]

Original text:

```
Hello\r\nWorld
```

Canonicalized payload bytes:

```
48 65 6C 6C 6F 0A 57 6F 72 6C 64
```

---

### Appendix B. Example Merkle Construction (Non-Normative) [anchor: appendix_b_example_merkle_construction_non_normative]

This appendix illustrates Merkle tree construction.

---

#### B.1 Leaf Example [anchor: b_1_leaf_example]

Leaf bytes:

```
u8(object_kind) || encode_id(object_id) || u8(tier_enum) || hash32(payload_hash)
```

Leaf hash:

```
HASH(domain_tag_leaf || leaf_bytes)
```

---

#### B.2 Internal Node Example [anchor: b_2_internal_node_example]

Given two child hashes `L` and `R`:

```
node_bytes = L || R
node_hash = HASH(domain_tag_node || node_bytes)
```

If a level has an odd number of nodes, the final node is duplicated and hashed with itself.

---

### Appendix C. Known Pitfalls and Explicit Non-Goals (Non-Normative) [anchor: appendix_c_known_pitfalls_and_explicit_non_goals_non_normative]

This appendix documents design boundaries.

---

#### C.1 Known Pitfalls [anchor: c_1_known_pitfalls]

* Treating derived views as canonical facts.
* Normalizing or correcting invalid identifiers instead of rejecting them.
* Allowing multiple encodings for the same logical value.
* Recomputing history under new rulebooks.

---

#### C.2 Explicit Non-Goals [anchor: c_2_explicit_non_goals]

This specification does **not** attempt to:

* Define governance processes.
* Define ranking, token, or safety logic.
* Optimize storage or transmission size.
* Provide human-friendly serialization formats.

Its sole purpose is deterministic, verifiable canonical encoding and hashing.


### Appendix D: Conformance Test Vectors (Normative) [anchor: appendix_d_conformance_test_vectors_normative]

This appendix defines mandatory test vectors that conformant implementations MUST reproduce exactly. Expected outputs use BLAKE3-256 with domain separation as defined in the main specification.

#### D.1 Primitive Encodings [anchor: d_1_primitive_encodings]

##### Vector D.1.1: Empty String
- Input: `""`
- Length prefix: varint 0 (`00`)
- Canonical bytes: `00`
- Expected hex: `00`

##### Vector D.1.2: UTF-8 String "hello"
- Input: `"hello"`
- Length prefix: varint 5 (`05`)
- Payload: `68 65 6c 6c 6f`
- Canonical bytes: `05 68 65 6c 6c 6f`
- Expected hex: `0568656c6c6f`

##### Vector D.1.3: Fixed Placeholder UUIDv7 ID
- Input: `018f5c2a-7c4d-7b2e-8f9a-9d2b5a0c3f1d`
- Canonical bytes: `u32` length prefix (`36`) + ASCII bytes
- Expected hex: `0000002430313866356332612d376334642d376232652d386639612d396432623561306333663164`

#### D.2 Domain-Separated Hashing [anchor: d_2_domain_separated_hashing]

Domain tags are UTF-8 encoded strings defined in implementation (e.g., "seed-primitive").

(Provide 3–5 vectors with computed BLAKE3 outputs once reference implementation available.)

#### D.3 Merkle Tree Constructions [anchor: d_3_merkle_tree_constructions]

##### Vector D.3.1: Single Leaf
- Leaf data: defined placeholder
- Domain: "seed-merkle-leaf"
- Expected leaf hash: [computed hex]

##### Vector D.3.2: Balanced Internal Node
- Child hashes L and R provided
- Expected node hash: [computed hex]

##### Vector D.3.3: Odd-Level Duplication
- Demonstration of final node duplication

(Provide at least 4 complete computed examples.)
