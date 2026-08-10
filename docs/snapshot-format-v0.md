---
doc_id: snapshot_format_v0
title: Snapshot Format v0
status: authoritative
version: v0
last_reviewed: 2026-06-22

scope:
  - Defines snapshot artifacts, metadata, and verification rules.

authoritative_for:
  - Snapshot structure and metadata fields.
  - Snapshot verification and integrity guarantees.

not_authoritative_for:
  - Canonical encoding and hashing rules (see canonical-encoding-and-hashing-spec.md).
  - Deterministic replay semantics (see deterministic-replay-and-merge-spec.md).

depends_on:
  - canonical-encoding-and-hashing-spec.md
  - deterministic-replay-and-merge-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of shared-map-and-payload-bundles-spec.md.
  - Any change here requires review of node-and-conformance-spec.md.

reader_path:
  - prereq: deterministic-replay-and-merge-spec.md
  - next: shared-map-and-payload-bundles-spec.md

keywords:
  - snapshots
  - state capture
  - verification
  - metadata
---

# Snapshot Format v0 (Stage L0 / Stage 0)

This document defines the derived, verifiable Snapshot Format v0 used by Stage L0 and Stage 0. It is implementation-ready and deterministic. It composes with:
- Protocol v5 Appendix A (`snapshot_commit` and canonical object schemas)
- Canonical Preservation & Provenance Spine (canonical record preservation)
- Canonical Encoding and Hashing Specification (v0) (canonical encoding, payload normalization, hashing, Merkle rules, and domain tags)
- Shared Map of Reality and Payload Bundles (Tier 0 / pocket_map_payload_root and shared_map_commitment)
- Offline & Mindseed specification (snapshot boundaries and packaging)


---

## 0. scope and non-goals

### scope
- Defines the on-disk, byte-level snapshot artifact used by Stage L0 and Stage 0.
- Snapshots are derived, verifiable artifacts emitted at deterministic derived block-height intervals (default: every 100 blocks, where each derived block contains a fixed number of events, default 50). For convenience, implementations MAY prioritize scheduling at block heights immediately following a block that contains a `cycle_close` event, but snapshots remain block-keyed and cycles do not define snapshot identity, keys, or boundaries.
- The canonical log MAY include a mechanical `snapshot_commit` boundary event that indexes a derived snapshot artifact. The artifact itself remains derived-only and replay-verifiable.
- Provides a deterministic serialization format, hashing rules, and verification procedure.
- Requires embedded title-representation text plus the selected Tier 0
  sentence-description text for all ideas and orderings present in snapshot state.
- Includes sufficient tables and indexes to serve the read-only API without guessing.

### stage 0 snapshot scope (minimal canonical materialization)
Stage 0 is permitted to emit a **reduced snapshot profile** derived from the minimal materialized tables currently required by the read-only API. This profile is authoritative for Stage 0 only and MUST be replaced by the full Snapshot Format v0 sections once the canonical materialization tables exist.

**Stage 0 canonical facts scope (state_root_hash):**
The current pre-genesis Stage 0 profile commits to the following materialized fact
sections, concatenated by ascending section ID:
1) `orderings_s0` (`0x000F`)
2) `ideas_s0` (`0x8001`)
3) `connections_s0` (`0x8002`)
4) `representations_s0` (`0x8003`)

The `ordering_representation_index` (`0x0010`) is included in the artifact but remains
an index-only section excluded from `state_root_hash`. Identities, challenges, verdicts,
rulebook_set, and other full-profile sections remain outside this reduced Stage 0 state
root until their current-profile materializations are implemented.

**Stage 0 section hashing:**
Each Stage 0 section hash MUST be computed as:
```
section_hash = HASH("snapshot_section" || u16(section_id) || section_bytes)
```
using the canonical encoding rules defined in `canonical-encoding-and-hashing-spec.md`.

Stage 0 section IDs (extension range reserved for temporary profiles):
- `orderings_s0` = `0x000F`
- `ordering_representation_index` = `0x0010` (index-only; excluded from state root)
- `ideas_s0` = `0x8001`
- `connections_s0` = `0x8002`
- `representations_s0` = `0x8003`

**Stage 0 state_root_hash:**
```
state_root_hash = HASH("snapshot_state_root" || concat(section_hashes_in_id_order))
```
where `section_hashes_in_id_order` is
`[orderings_s0, ideas_s0, connections_s0, representations_s0]`.

**Stage 0 section contents and ordering:**
Stage 0 sections MUST be encoded using canonical primitive encodings (big-endian integers, varint string lengths, UTF-8, NFC normalization, LF line endings, explicit presence flags for optional fields). Records MUST be ordered by canonical log order, not by identifiers.

`ideas_s0` records (one per idea in `ideas` table with `created_block_height <= H`), ordered by `(created_block_height ASC, created_event_index ASC)`:
1) `idea_id` (id)
2) `idea_type` (string)
3) `speaker_identity_id` (id)
4) `created_event_id` (id)
5) `created_block_height` (u64)
6) `created_event_index` (u32)

`connections_s0` records (one per connection in `connections` table with `created_block_height <= H`), ordered by `(created_block_height ASC, created_event_index ASC)`:
1) `connection_id` (id)
2) `from_idea_id` (id)
3) `to_idea_id` (id)
4) `connection_type` (string)
5) `usage_present` (u8) + `usage` (string, if present)
6) `axis_present` (u8) + `axis` (string, if present)
7) `timeframe_present` (u8) + `timeframe` (string, if present)
8) `scope_present` (u8) + `scope` (string, if present)
9) `created_by_event_id` (id)
10) `created_block_height` (u64)
11) `created_event_index` (u32)

`orderings_s0` records (one per Ordering at `H`), ordered by
`(created_block_height ASC, created_event_index ASC)`:
1) `ordering_id` (id)
2) `ordering_profile` (u8)
3) `vine_type_present` (u8) + `vine_type` (u8, if present)
4) `subject_idea_id_present` (u8) + `subject_idea_id` (id, if present)
5) `speaker_identity_id` (id)
6) `created_event_id` (id)
7) `base_ordering_id_present` (u8) + `base_ordering_id` (id, if present)
8) `item_count` (u32)
9) ordered item records: `idea_id` (id), `item_role_present` (u8) +
   `item_role` (u8, if present)
10) aligned step-metadata count and optional `via_connection_id` values

Stage 0 uses these exact `u8` enum codes:
- `ordering_profile`: `0 = vine`, `1 = evidence_rail`, `2 = action_rail`;
- `vine_type`: `0 = pathway_vine`, `1 = narrative_vine`;
- `item_role`: `0 = potential_evidence`, `1 = actual_evidence`,
  `2 = potential_action`, `3 = proposed_action`;
- representation `target_kind`: `0 = idea`, `1 = ordering`;
- representation `representation_kind`: `0 = title`, `1 = description`;
- description `tier_length`: `0 = sentence`, `1 = paragraph`, `2 = full`;
- representation `tier_complexity`: `0 = fundamental`, `1 = standard`,
  `2 = advanced`, `3 = canonical`.

All other `u8` values are invalid. The profile/subject/role and fork invariants are those
in Appendix A; snapshots MUST reject mismatched materialized rows and duplicate item IDs
in standardized Rails.

`representations_s0` records (one per representation at `H`), ordered by
`(created_block_height ASC, created_event_index ASC)`:
1) `representation_id` (id)
2) `target_kind` (u8)
3) `target_object_id` (id)
4) `representation_kind` (u8)
5) conditional kind fields:
   - for `title`: no tier fields;
   - for `description`: `tier_length` (u8), then `tier_complexity` (u8)
6) `vocabulary_version_id_present` (u8) + `vocabulary_version_id` (id, if present)
7) `author_identity_id` (id)
8) `payload_hash` (hash32)
9) `payload_embedded` (u8) + `payload_bytes` (bytes, if present)
10) `language_locale_present` (u8) + `language_locale` (string, if present)
11) `provenance_present` (u8) + `provenance` (string, if present)
12) `created_event_id` (id)
13) `created_block_height` (u64)
14) `created_event_index` (u32)

For a title row, `vocabulary_version_id_present` MUST be zero. It MUST be one exactly for
a canonical-complexity description and zero for every other description. A title row
carrying a tier or vocabulary identifier, a canonical-complexity description without a
vocabulary reference, a noncanonical-complexity description with one, or a row whose
author differs from its event speaker is invalid. Title and selected Tier 0 sentence
payloads MUST be embedded. No snapshot builder may repair or infer any field.

**Stage 0 title_sentence_payload_root:**
For Stage 0, `title_sentence_payload_root` MUST be computed over title-representation
payloads and selected Tier 0 sentence-description payloads for all included ideas and
orderings:
- For each included object (`idea` and `ordering`) at height `H`, resolve the canonical
  title representation and the selected Tier 0 sentence-description representation
  payload bytes for that object.
- Canonicalize each payload per `canonical-encoding-and-hashing-spec.md` (UTF-8, NFC, LF normalization).
- Compute `payload_hash = HASH(canonical_bytes)` (no domain tag).
- Build leaf bytes as `u8(object_kind) || encode_id(object_id) || u8(payload_kind) || hash32(payload_hash)` where:
  - `object_kind` is `0` for idea and `1` for ordering.
  - `payload_kind` is `0` for title and `1` for the selected sentence description.
    These are payload-root leaf codes, not description length-tier codes.
- Sort leaves by raw bytewise comparison of full `leaf_bytes`.
- Compute the Merkle root using the Merkle construction rules in `canonical-encoding-and-hashing-spec.md`.

If any included idea or ordering at height `H` lacks a title payload or the selected
Tier 0 sentence-description payload, the Stage 0 snapshot is invalid.

**Stage 0 empty-root constants:**
If there are zero included ideas and orderings at height `H`, the `title_sentence_payload_root` MUST be:
```
BLAKE3("seed-empty-payload")
```
as defined in Appendix A of this document. The `state_root_hash` MUST be computed from the empty `ideas_s0` and `connections_s0` sections using the standard section hash and state_root_hash formulas above.

**Stage 0 shared_map_commitment:**
Stage 0 uses the same formula:
```
shared_map_commitment(H) =
  HASH("shared_map_commitment_v0" || state_root_hash(H) || title_sentence_payload_root(H))
```

### non-goals
- Does not redefine canonical event schemas or replay rules (see Protocol v5 Appendix A).
- Does not define transport, compression wrappers, or network distribution.
- Does not define payload pack rules beyond the Tier 0 root reference (see Shared Map spec).
- The shared map commitment is defined as `HASH("shared_map_commitment_v0" || state_root_hash(H) || pocket_map_payload_root(H))` in the Shared Map spec.
- The authoritative definition of `shared_map_commitment` is in the Canonical Encoding and Hashing Specification (v0); this document references that definition and does not redefine it.
- The `HASH(...)` notation and domain tag strings are symbolic references; their concrete byte encoding, hash algorithm, and domain separation behavior are defined exclusively by the Canonical Encoding and Hashing Specification (v0).
- Does not define cryptographic primitives, domain tags, canonical ID encoding rules, or Merkle construction rules beyond referencing them (see the Canonical Encoding and Hashing Specification v0).

---

## 1. snapshot identity (block height, snapshot tiers)

A snapshot is a derived, verifiable read-only state artifact for a single derived block height.

### 1.1 canonical identity
A snapshot is identified by:
- All numeric types in this document (e.g., u64, u32) refer to internal canonical binary encodings. When exposed via APIs or transport schemas, these values MUST be transmitted as decimal strings per the API Contract.
- `block_height` (u64): the derived publication block height at which the snapshot boundary occurs.
- `snapshot_hash` (32 bytes): the canonical hash of the snapshot bytes, defined in Section 7.

For all APIs and external references in Stage L0/0, `snapshot_id` is defined as:

```
snapshot_id := hex(snapshot_hash)
```

This avoids non-deterministic identifiers and allows direct verification.

### 1.2 snapshot tier
A snapshot MAY include an optional `snapshot_tier_id` string. This is a label used only for packaging and distribution (e.g., Tier 0 Pocket Map). It has no effect on canonical meaning. If absent, the snapshot is a full/default tier for its block height.

### 1.3 block-height basis
Snapshots MUST be keyed to derived block height, not cycles. Any cycle index included elsewhere is advisory only and MUST NOT affect snapshot validity.

### 1.4 snapshot role in governance activation (verification-only) [anchor: snapshot_boundary_activation_definition]

Snapshots are keyed to derived block height and are used for deterministic verification, checkpointing, and distribution.

Governance/rulebook activation is cycle-based and MUST be derived from governance verdict metadata (`decision_cycle_index`, `change_class`, `delay_policy_version`) under the active delay policy.

Snapshots MUST NOT define governance activation boundaries. This document defines snapshot identity and verification mechanics only.
---

## 2. Commitment Hierarchy

Each snapshot defines:
- `state_root_hash`: Merkle root over canonical facts only (identities, ideas, orderings, representations with payload_hash pointers, connections, challenges, verdicts, active rulebook set).
- `title_sentence_payload_root`: Merkle root over Tier 0 leaf bytes
  `(object_kind, object_id, payload_kind, payload_hash)` for title-representation and
  selected sentence-description payloads across ideas and orderings, ordered
  deterministically by full encoded leaf bytes.
- `shared_map_commitment`: HASH("shared_map_commitment_v0" || state_root_hash || pocket_map_payload_root) — baseline commitment for verifying shared canonical reality and Tier 0 meaning preservation (as defined in Canonical Encoding and Hashing Specification (v0); referenced here).
- `snapshot_hash`: HASH(domain_snapshot || entire_snapshot_bytes) — content-addressable artifact identifier.

---

## 3. header schema (fields, types, canonical order)

All fields are encoded deterministically using the primitive types defined in Section 6. The header is a fixed-order sequence with length-prefixes for variable-sized fields.

### 3.1 primitive types (used in header)

All primitive encodings (integer endianness, length-prefix rules, UTF-8 requirements, normalization/rejection rules, and byte layout for hash32, id, string, bytes, and list<T>) MUST follow the Canonical Encoding and Hashing Specification (v0). This document only specifies which primitive type is used for each field and any additional constraints (e.g., id MUST be a canonical UUIDv7 string).

IDs MUST be canonical UUIDv7 strings in the 36-character lowercase hex with hyphens form; uppercase or alternate encodings MUST be rejected.

### 3.2 header fields (canonical order)
1) `magic` (8 bytes)
   - ASCII: `MCCSNAP0`

2) `format_version` (u16)
   - MUST be `0` for this spec.
   - This `format_version` also serves as the Canonical Encoding and Hashing Specification version identifier for snapshot artifacts (v0).

3) `header_flags` (u16)
   - MUST be `0` in v0.

4) `header_byte_len` (u32)
   - Number of bytes from the start of `block_height` to the end of the section directory.

5) `block_height` (u64)

6) `snapshot_kind` (u8)
   - `0` = full snapshot (REQUIRED in v0)
   - `1` = delta snapshot (RESERVED)

7) `snapshot_tier_id` (string)
   - Empty string if not provided.

8) `last_event_id` (id)
   - Event ID of the final canonical event included up to this block boundary.

9) `event_count` (u64)
   - Total number of canonical events from genesis through `last_event_id`, inclusive.

10) `active_rulebook_set_hash` (hash32)
    - Deterministic hash of the active rulebook set at this boundary.

11) `state_root_hash` (hash32)
    - Deterministic hash of canonical facts only (Section 7).

12) `title_sentence_payload_root` (hash32)
    - Merkle root over all title + sentence payload hashes embedded in this snapshot (Section 7).
    - This is the Tier 0 `pocket_map_payload_root` for the same block height.

13) `section_count` (u16)

14) `section_directory` (repeated `section_count` times)
    - `section_id` (u16)
    - `section_item_count` (u32)
    - `section_byte_len` (u64)
    - `section_hash` (hash32)

The header ends immediately after the section directory. The body begins at the next byte.

---

## 4. body schema (sections/tables, canonical ordering)

The snapshot body is a concatenation of sections in the exact order listed in the header's section directory. Each section is raw bytes of its records; no section header appears in the body.

### 4.1 section IDs (v0)
Required sections in v0 (must appear, may be empty):
- `0x0001` identities
- `0x0002` ideas
- `0x0003` representations
- `0x0004` idea_representation_index (index-only; excluded from state_root_hash)
- `0x000F` orderings
- `0x0010` ordering_representation_index (index-only; excluded from state_root_hash)
- `0x0005` connections
- `0x0006` connection_from_index (index-only; excluded from state_root_hash)
- `0x0007` connection_to_index (index-only; excluded from state_root_hash)
- `0x0008` challenges
- `0x0009` verdicts
- `0x000B` rulebook_set
- `0x0011` identity_direct_keys
- `0x0012` identity_admission_lineage
- `0x0013` identity_admission_derived
- `0x0014` admission_capacity_period_state

Optional derived-view sections in v0 (MAY be present):
- `0x000A` rankings
- `0x000C` safety_classifications
- `0x000D` token_balances
- `0x000E` idea_tags
- `0x000F` tempo_cycle_state

`identity_admission_derived` and `admission_capacity_period_state` are required
replay-derived explainability sections for Profile-v0 identity admission, but are
excluded from `state_root_hash`. Performance indexes and other derived views (rankings,
token balances, safety classifications, Tempo/Cycle state) MAY be included for
convenience or historical record and are likewise excluded from `state_root_hash`
computation.

### 4.1.1 derived state packs: ranks

Snapshots MAY include materialized rank data as **derived artifacts**, but derived rank values are not canonical truth. The canonical source of rank state remains deterministic replay of the canonical event log under the active rulebooks at the snapshot boundary.

Implementations MAY emit optional **rank packs** or **rank history packs** keyed to a specific snapshot height. Such packs MUST commit to:
- the snapshot identifier (`snapshot_id` or `snapshot_hash` as defined in this document),
- the relevant rulebook/version commitment for ranking (e.g., `active_rulebook_set_hash`), and
- the deterministic ordering and encoding rules used to serialize the rank data.

Rank packs MUST be reproducible from deterministic replay and MUST validate against the snapshot's existing commitments (including `state_root_hash` and `active_rulebook_set_hash`) using the same ordering and encoding rules recorded in the pack metadata. No new cryptographic commitments are introduced by this section.

Multiple resolutions and tiers are permitted. Frequent, small snapshots MAY omit rank materialization entirely. Less frequent, higher-tier snapshots MAY include full universal ranks and/or relative ranks. Regardless of inclusion, ranks are uniquely determined for any snapshot height by deterministic replay.

### 4.1.2 derived state packs: Tempo/Cycle

Snapshots MAY include materialized Tempo/Cycle derived state for read performance and historical inspection. This pack is optional, derived, and excluded from `state_root_hash`.

If present, a Tempo/Cycle derived-state pack MAY expose:
- current cycle index,
- active Dmin/Dmax target keys,
- predicate status,
- `structural_dmax_liveness_predicate` status,
- cycle certification status,
- authorization frontier,
- Tempo operating mode,
- beacon summaries,
- provisional/pending/authorized/blocked downstream-output status.

Tempo/Cycle state MUST be interpreted separately from the required Profile-v0 admission
derived sections. A Dmax, forced, degraded, survivor, record-only, or machine-only
boundary does not by itself make a qualifying capacity period, generate invitation
capacity, advance maturation, activate inviter eligibility, restore suspension, or
clear `admission_liveness_blocked`.

The pack MUST NOT expose private identity details beyond public canonical rules. It MUST NOT include node-local clocks, server time, client timestamps, receipt time, block-height time claims, scheduler observations, local uncommitted observations, AI observations, or private verification material as Tempo authority.

Tempo/Cycle packs MUST be reproducible from deterministic replay and MUST validate against the snapshot's existing commitments using the Appendix A Tempo/Cycle schemas. Snapshot `block_height` remains a packaging/address key only and MUST NOT substitute for Dmin/Dmax certainty, beacon state, cycle certification, or the lagged authorization frontier.

Extension sections MUST use IDs in the range `0x8000`-`0xFFFF`.

### 4.2 canonical record ordering
- Records within each section MUST be sorted by their primary key(s) in lexicographic byte order of the canonical encoded ID(s).
- Lists inside a record MUST be sorted lexicographically by the same rule, unless explicitly ordered by the specification (e.g., rankings sections if present).
- Ties MUST be broken by lexicographic order of the smallest available canonical ID.
- Ordering comparisons MUST be performed on the canonical encoded `id` bytes exactly as defined by the Canonical Encoding and Hashing Specification (v0). Implementations MUST NOT apply alternate string collations or locale-dependent ordering.
- These ordering rules define snapshot serialization order only and MUST NOT be interpreted as canonical ordering. Canonical ordering derives from canonical log order (block height and event_index), not identifier sort order.

### 4.3 section schemas
All fields are encoded in the order listed. Optional fields use a `bool` presence flag followed by the value when present.

#### 4.3.1 identities (`0x0001`)
Primary key: `identity_id`

Fields:
- `identity_id` (id)
- `identity_kind` (u8)  // Profile-v0 `0x01 = human`
- `admission_provenance_class` (u8)  // Appendix A closed provenance enum
- `created_event_id_present` (bool)
- `created_event_id` (id, if present)
- `compatibility_manifest_ref_present` (bool)
- `compatibility_manifest_ref` (hash32, if present)
- `structural_root_count` (u32)
- `structural_roots` (ordered `(root_role u8, idea_id id)` list)
- `public_display_name_present` (bool)
- `public_display_name` (string, if present)

`structural_roots` is exactly the four Profile-v0 roles for an `event_derived` identity
and is absent only where a compatibility provenance is explicitly classified as lacking
that history. A snapshot MUST NOT fabricate roots, sponsors, proofs, capacity debits,
lineage, or verification artifacts for genesis/import/legacy identities.

#### 4.3.1A identity_direct_keys (`0x0011`)
Primary key: (`identity_id`, `public_key_ref`)

Fields:
- `identity_id` (id)
- `public_key_ref` (hash32)
- `key_descriptor_bytes` (bytes)
- `key_state` (u8: active, superseded, revoked, invalid)
- `registration_provenance_class` (u8)
- `activation_event_id` (id)
- `supersession_event_id_present` (bool)
- `supersession_event_id` (id, if present)
- `revocation_event_id_present` (bool)
- `revocation_event_id` (id, if present)

Records are sorted by canonical `identity_id` then raw `public_key_ref` bytes. This
section commits complete registered-key history; the active key is replay-derived and
must agree with the exact Appendix A/Profile-v0 lifecycle.

#### 4.3.1B identity_admission_lineage (`0x0012`)
Primary key: `admitted_identity_id`

Fields:
- `admitted_identity_id` (id)
- `sponsor_identity_id_present` (bool)
- `sponsor_identity_id` (id, if present)
- `admission_event_id_present` (bool)
- `admission_event_id` (id, if present)
- `admission_profile_version_present` (bool)
- `admission_profile_version` (string, if present)
- `provenance_class` (u8)

For `event_derived` Profile-v0 admission all sponsor and admission fields are required.
For compatibility provenance they are absent unless an explicitly historical canonical
record supplies them; replay MUST NOT invent them.

#### 4.3.1C identity_admission_derived (`0x0013`)
Primary key: (`identity_id`, `admission_profile_version`)

Fields:
- `identity_id` (id)
- `admission_profile_version` (string)
- `verification_state_commitment` (hash32)
- `vh_certainty` (rulebook-defined canonical scalar encoding)
- `vi_certainty` (rulebook-defined canonical scalar encoding)
- `eligibility_lane_flags` (u16)
- `invitation_capacity_balance` (i64, non-negative)
- `invitation_suspension` (u8)
- `maturation_state_commitment` (hash32)
- `capacity_derivation_commitment` (hash32)

The lane flags are an ordered bitset defined by the active rulebook for restricted
verification, ordinary writer, ordinary challenge, voter, governance, Tempo, and
inviter eligibility. The record is derived-only: canonical event history and active
rulebooks remain authoritative, and its fields must be reproducible from them.

#### 4.3.1D admission_capacity_period_state (`0x0014`)
Primary key: (`admission_profile_version`, `rulebook_id`, `capacity_period_id`)

Fields:
- `admission_profile_version` (string)
- `rulebook_id` (id)
- `rulebook_version` (string)
- `rulebook_hash` (hash32)
- `capacity_period_id` (id)
- `boundary_kind` (u8)
- `qualifying_capacity_period` (bool)
- `admission_liveness_blocked` (bool)
- `maturation_advanced` (bool)
- `new_capacity_generated` (bool)
- `existing_capacity_spendable` (bool)

This required derived section makes the public, rulebook-scoped, period-scoped admission
liveness result explainable. It does not expose private capacity; Profile-v0 capacity is
publicly replay-derivable.

#### 4.3.2 ideas (`0x0002`)
Primary key: `idea_id`

Fields:
- `idea_id` (id)
- `idea_type` (u8)  // enum per Protocol v5 Appendix A
- `truth_subtype_present` (bool)
- `truth_subtype` (u8, if present)  // enum per Appendix A
- `speaker_identity_id` (id)
- `created_event_id` (id)
- `lifecycle_state` (u8)  // enum per active rulebook
- `short_label_present` (bool)
- `short_label` (string, if present)

#### 4.3.3 representations (`0x0003`)
Primary key: `representation_id`

Each record represents a canonical title or description representation object. The
canonical title representation and selected Tier 0 sentence-description representation
MUST be present for every idea and ordering and MUST include embedded payload bytes
(Section 5).

Fields:
- `representation_id` (id)
- `target_kind` (u8)  // `0 = idea`, `1 = ordering`
- `target_object_id` (id)
- `representation_kind` (u8)  // `0 = title`, `1 = description`
- if `representation_kind = description`, `tier_length` (u8)
  // `0 = sentence`, `1 = paragraph`, `2 = full`
- if `representation_kind = description`, `tier_complexity` (u8)
  // `0 = fundamental`, `1 = standard`, `2 = advanced`, `3 = canonical`
- `vocabulary_version_id_present` (bool)
- `vocabulary_version_id` (id, if present)
- `author_identity_id` (id)
- `payload_hash` (hash32)
- `payload_class` (u8)  // enum per safety rulebooks: normal, sensitive_abstracted, non_distributable_blocked
- `payload_embedded` (bool)
- `payload_bytes` (bytes, if payload_embedded = true)
- `language_locale_present` (bool)
- `language_locale` (string, if present)
- `provenance_present` (bool)
- `provenance` (string, if present)
- `created_event_id` (id)
- `created_block_height` (u64)
- `created_event_index` (u32)

Rules:
- A title has no `tier_length` or `tier_complexity`; the canonical title payload MUST be
  embedded.
- A description has both tier fields. The selected Tier 0 sentence-description payload
  MUST be embedded; other description payloads MAY be omitted when `payload_hash` is
  present.
- `vocabulary_version_id_present` MUST be false for title, true exactly for a
  canonical-complexity description, and false for every other description complexity.
- `author_identity_id` MUST equal the source `representation_create` event speaker, and
  both that identity and the vocabulary idea when present MUST predate the representation.
- Enum values outside the exact code sets above are invalid. Snapshot builders MUST NOT
  infer, default, or repair any binding.

#### 4.3.4 idea_representation_index (`0x0004`)
Primary key: `idea_id`

Fields:
- `idea_id` (id)
- `title_representation_id` (id)  // canonical pointer for the one title slot
- `sentence_representation_id` (id)  // rulebook-selected Tier 0 sentence-description pointer
- `other_representation_count` (u32)
- `other_representation_ids` (id list, sorted)

The title pointer MUST reference `representation_kind = title`. The sentence pointer
MUST reference `representation_kind = description` with `tier_length = sentence`.
Together with `other_representation_ids`, the index preserves the active pointers for
the twelve description cells; each description ID's row supplies its exact length and
complexity. Candidate representations remain in `representations` even when not selected.

#### 4.3.4A orderings (`0x000F`)
Primary key: `ordering_id`

Fields:
- `ordering_id` (id)
- `ordering_profile` (u8)  // 0=vine, 1=evidence_rail, 2=action_rail
- `vine_type_present` (bool)
- `vine_type` (u8, if present)  // `0 = pathway_vine`, `1 = narrative_vine`
- `subject_idea_id_present` (bool)
- `subject_idea_id` (id, if present)
- `speaker_identity_id` (id)
- `created_event_id` (id)
- `base_ordering_id_present` (bool)
- `base_ordering_id` (id, if present)
- `item_count` (u32)
- `item_idea_ids` (id list, ordered)
- `item_role_count` (u32)
- `item_roles` (u8 list, ordered and aligned one-for-one with `item_idea_ids`)
- `step_meta_count` (u32)
- `step_meta` (list; each entry MAY include `via_connection_id` as optional id)

Profile invariants:
- `vine_type_present` MUST be true for an `ordering_create` Vine and MAY inherit from the base for an `ordering_fork` Vine.
- `vine_type_present` MUST be false for Evidence Rail and Action Rail profiles.
- A Vine MUST have `subject_idea_id_present = false` and `item_role_count = 0`.
- An Evidence Rail MUST reference a `truth_claim`; its aligned roles use only
  `0 = potential_evidence` or `1 = actual_evidence`.
- An Action Rail MUST reference an `actionable_idea`; all aligned roles in one Ordering
  use exactly one lane, either `2 = potential_action` or `3 = proposed_action`.
- Standardized Rails require at least one item and forbid duplicate item IDs.
- A fork's `ordering_profile`, subject, retained-item roles, and Action Rail lane MUST
  equal its base Ordering's corresponding values.
- Enum values outside the exact code sets above are invalid. Snapshot builders MUST NOT
  infer subjects or roles.

#### 4.3.4B ordering_representation_index (`0x0010`)
Primary key: `ordering_id`

Fields:
- `ordering_id` (id)
- `title_representation_id` (id)  // canonical pointer for the one title slot
- `sentence_representation_id` (id)  // rulebook-selected Tier 0 sentence-description pointer
- `other_representation_count` (u32)
- `other_representation_ids` (id list, sorted)

The kind, Tier 0 selection, twelve-cell mapping, and candidate-preservation rules for
`idea_representation_index` apply identically to this Ordering index.

#### 4.3.5 connections (`0x0005`)
Primary key: `connection_id`

Fields:
- `connection_id` (id)
- `from_idea_id` (id)
- `to_idea_id` (id)
- `connection_type` (u8)  // enum per Protocol v5 Appendix A
- `created_event_id` (id)
- `lifecycle_state` (u8)
- `usage_present` (bool)
- `usage` (string, if present)
- `axis_present` (bool)
- `axis` (string, if present)
- `timeframe_present` (bool)
- `timeframe` (string, if present)
- `scope_present` (bool)
- `scope` (string, if present)

#### 4.3.6 connection_from_index (`0x0006`)
Primary key: `from_idea_id`

Fields:
- `from_idea_id` (id)
- `connection_count` (u32)
- `connection_ids` (id list, sorted)

#### 4.3.7 connection_to_index (`0x0007`)
Primary key: `to_idea_id`

Fields:
- `to_idea_id` (id)
- `connection_count` (u32)
- `connection_ids` (id list, sorted)

#### 4.3.8 challenges (`0x0008`)
Primary key: `challenge_id`

Fields:
- `challenge_id` (id)
- `challenge_domain` (u8)  // enum per Protocol v5 Appendix A
- `subject_idea_count` (u32)
- `subject_idea_ids` (id list, sorted)
- `subject_ordering_count_present` (bool)
- `subject_ordering_count` (u32, if present)
- `subject_ordering_ids` (id list, sorted, if present)
- `created_by_identity_id` (id)
- `lifecycle_state` (u8)
- `importance_context_present` (bool)
- `importance_rank_kind` (u8, if present)
- `challenger_idea_id` (id, if present)
- `target_idea_id` (id, if present)
- `importance_scope` (string, if present)
- `importance_reference_idea_id_present` (bool, if importance context is present)
- `importance_reference_idea_id` (id, if present)
- `importance_scope_anchor_id_present` (bool, if importance context is present)
- `importance_scope_anchor_id` (id, if present)
- `importance_usage_present` (bool, if importance context is present)
- `importance_usage` (string, if present)
- `importance_axis` (string, if importance context is present)
- `importance_timeframe` (string, if importance context is present)
- `opened_event_id` (id)
- `closed_event_id_present` (bool)
- `closed_event_id` (id, if present)
- `verdict_id_present` (bool)
- `verdict_id` (id, if present)

When `importance_context_present = true`, the fields MUST encode exactly the Appendix A universal or relative challenge context. For universal challenges, reference, scope anchor, and usage are absent and `importance_axis` is a universal orientation. For relative challenges, reference and `usage = general` are present and `importance_axis` uses the relative vocabulary. Private individual ordering MUST NOT be encoded as a canonical challenge.

#### 4.3.9 verdicts (`0x0009`)
Primary key: `verdict_id`

Fields:
- `verdict_id` (id)
- `challenge_id` (id)
- `outcome` (string)
- `tally_summary_hash` (hash32)
- `finalization_event_id` (id)

#### 4.3.10 rankings (`0x000A`)
Primary key: canonical `rank_context_key`

Fields:
- `ranking_record_kind` (u8: `axis_list` or `universal_overall`)
- `rank_kind` (u8: `universal` or `relative`)
- `scope` (string)
- `reference_idea_id_present` (bool)
- `reference_idea_id` (id, if present)
- `scope_anchor_id_present` (bool)
- `scope_anchor_id` (id, if present)
- `usage_present` (bool)
- `usage` (string, if present)
- `axis_present` (bool)
- `axis` (string, if present)
- `timeframe_present` (bool)
- `timeframe` (string, if present)
- `ordered_idea_count` (u32)
- `ordered_idea_ids` (id list, ordered)  // order is semantically significant
- `universal_position_sums_present` (bool)
- `universal_position_sums` (u64 list aligned with `ordered_idea_ids`, if present)

For a universal `axis_list`:

- `rank_kind = universal`;
- `scope = universal`;
- reference, scope anchor, and usage are absent;
- `axis` is one universal orientation;
- timeframe is present; and
- aggregate sums are absent.

For a relative `axis_list`:

- `rank_kind = relative`;
- reference, usage, relative axis, timeframe, and scope are present;
- `axis` is `important_to_reference` or `important_for_reference`;
- `scope = tribe` additionally requires `scope_anchor_id = reference_idea_id`; and
- aggregate sums are absent.

For `universal_overall`:

- `rank_kind = universal`;
- `scope = universal`;
- axis and timeframe are absent;
- `ordered_idea_ids` are sorted by ascending exact twenty-axis position sum plus the active deterministic tie-break; and
- `universal_position_sums` is present. The exact displayed mean is each sum divided by twenty and need not be separately encoded.

Private individual and simulated AI ranks MUST NOT appear in canonical snapshot sections. A rank-context key MUST encode the discriminators above and MUST NOT depend on an idea title or UI label.

#### 4.3.11 rulebook_set (`0x000B`)
Primary key: (`governance_domain`, `rulebook_id`)

Fields:
- `governance_domain` (string)
- `rulebook_id` (id)
- `rulebook_version` (string)
- `rulebook_hash` (hash32)
- `activation_event_id` (id)

#### 4.3.12 safety_classifications (`0x000C`)
Primary key: `classification_id`

Fields:
- `classification_id` (id)
- `rulebook_id` (id)
- `jurisdiction_lens` (string)
- `explanation_representation_id` (id)

#### 4.3.13 token_balances (`0x000D`)
Primary key: `identity_id`

Fields:
- `identity_id` (id)
- `pod_balance` (i64)
- `point_balance` (i64)
- `melt_counter` (i64)

#### 4.3.14 idea_tags (`0x000E`)
Primary key: (`idea_id`, `tag`)

Fields:
- `idea_id` (id)
- `tag` (string)


## 5. embedded text rules (title representation + selected sentence description)

The snapshot MUST embed the canonical title-representation text and selected Tier 0
sentence-description text for every idea and every ordering present in snapshot state.

Requirements:
- Each idea MUST have exactly one canonical title representation and one selected
  Tier 0 sentence-description representation.
- Each ordering MUST have exactly one canonical title representation and one selected
  Tier 0 sentence-description representation.
- These representations MUST appear in `representations` with `payload_embedded = true` and their payload bytes present.
- The embedded bytes MUST be the canonical payload bytes whose hash equals the `payload_hash` field.
- Embedded payload bytes MUST be canonicalized and validated exactly as defined by the Canonical Encoding and Hashing Specification (v0), including all normalization and rejection rules.
- `idea_representation_index` MUST reference title/sentence representation IDs for ideas.
- `ordering_representation_index` MUST reference title/sentence representation IDs for orderings.

If any idea or ordering lacks either required representation, or if embedded bytes do
not hash to the declared `payload_hash`, the snapshot is invalid.



## 6. canonical serialization rules (byte-level)

This section defines the **structural determinism** requirements for snapshot serialization. All primitive encodings (integer endianness and width, signed integer representation, string/bytes framing, UTF-8 and normalization/rejection rules, boolean/enum encoding, list framing, and canonical map/list rules) MUST follow the Canonical Encoding and Hashing Specification (v0).

Snapshot Format v0 MUST NOT define any competing primitive encodings.

### 6.1 integer encoding
All integer encodings (including endianness, width, range validation, and signed representation) MUST follow the Canonical Encoding and Hashing Specification (v0).

### 6.2 string, id, and bytes encoding
All encodings for `string`, `id`, and `bytes` (including UTF-8 requirements, normalization/rejection rules, and length framing) MUST follow the Canonical Encoding and Hashing Specification (v0).

Additional constraints:
- `id` values MUST be valid canonical identifiers (UUIDv7 strings) per the Canonical Encoding and Hashing Specification (v0) and MUST be rejected if not canonical.

### 6.3 boolean and enum encoding
All encodings for `bool` and `enum` MUST follow the Canonical Encoding and Hashing Specification (v0).

Additional constraints:
- `bool` values MUST be valid and MUST be rejected if outside the permitted set.
- `enum` values MUST be valid enumerators per the referenced specification and MUST be rejected otherwise.

### 6.4 list encoding
All encodings for `list<T>` (including count framing and item encoding) MUST follow the Canonical Encoding and Hashing Specification (v0).

### 6.5 ordering and determinism
- All sections MUST appear in the order given by the section directory.
- All records and lists MUST be ordered as specified in Section 4.2.
- No field may be omitted, reordered, or conditionally suppressed.
- No padding bytes are permitted.
- Implementations MUST reject snapshots that violate ordering requirements, omit required fields, include unexpected padding, or otherwise fail deterministic structure rules.

### 6.6 compression and transport wrappers
Compression is permitted only as an external wrapper. The canonical snapshot bytes defined by this specification MUST be recoverable exactly, byte-for-byte. Compression metadata and any transport framing MUST NOT affect any hash, signature, or Merkle commitment.


## 7. hashing rules (snapshot_hash, state_root_hash, payload roots)

All hashes, domain separation tags, canonical byte encodings, and Merkle construction rules referenced by this document MUST follow the Canonical Encoding and Hashing Specification (v0). The Canonical Preservation & Provenance Spine defines how these commitments are used for long-term preservation and packaging, but it does not override the byte-level rules in the Canonical Encoding and Hashing Specification (v0).

### 7.1 section_hash

All domain tag strings and their byte-level treatment are defined exclusively by the Canonical Encoding and Hashing Specification (v0); this document references those tags symbolically.

For each section, compute:

section_hash = HASH("snapshot_section" || u16(section_id) || section_bytes)
`HASH(...)` and the domain tag shown are symbolic references; the hash algorithm, domain separation, and byte encoding rules are defined exclusively by the Canonical Encoding and Hashing Specification (v0).

The `section_hash` is recorded in the header section directory.

### 7.2 state_root_hash
`state_root_hash` commits only to canonical facts, not to indexes or derived views. It includes exactly the sections listed below and excludes `idea_representation_index`, `ordering_representation_index`, `connection_from_index`, `connection_to_index`, rankings, safety_classifications, token_balances, idea_tags, and all extension sections.

Compute in order:
1) Determine the canonical-facts sections:
   - identities, identity_direct_keys, identity_admission_lineage, ideas, orderings,
     representations, connections, challenges, verdicts, rulebook_set.
2) Collect the `section_hash` values for those sections in canonical section-id order.
3) Compute:

state_root_hash = HASH("snapshot_state_root" || concat(section_hashes))

Derived views (rankings, token_balances, safety_classifications, idea_tags,
identity_admission_derived, and admission_capacity_period_state) are computed from
canonical facts plus active rulebooks. They are required where Profile-v0 admission is
active for independent explanation, but are not part of `state_root_hash` and never
replace full replay.

### 7.2.1 active_rulebook_set_hash
`active_rulebook_set_hash` is derived from the `rulebook_set` section bytes:

active_rulebook_set_hash = HASH("snapshot_rulebook_set" || rulebook_set_section_bytes)

### 7.3 title_sentence_payload_root (Tier 0 root)

Let `P` be the ordered list of leaves for all canonical title representations and
selected Tier 0 sentence-description representations across all ideas and orderings,
where each leaf commits to the tuple
`(object_kind, object_id, payload_kind, payload_hash)` in canonical byte order.

- `object_kind` is a u8 with:
  - `0 = idea`
  - `1 = ordering`
- `payload_kind` is a u8 with:
  - `0 = title`
  - `1 = selected sentence description`

`payload_kind` classifies Tier 0 payload-root leaves. It is not the description
`tier_length` enum, and title is not a description tier.

Leaf encoding:
- leaf bytes MUST be
  `u8(object_kind) || encode_id(object_id) || u8(payload_kind) || hash32(payload_hash)`.
- `encode_id(object_id)` MUST use the canonical `id` encoding defined in the Canonical Encoding and Hashing Specification (v0).
- `hash32(payload_hash)` MUST be the canonical 32-byte hash encoding as defined in the Canonical Encoding and Hashing Specification (v0).

Ordering:
- The leaf list MUST be sorted lexicographically by the encoded leaf bytes (bytewise ascending).
- Sorting MUST be performed on the full leaf bytes; implementations MUST NOT sort by `payload_hash` alone.

Merkle construction:
- `title_sentence_payload_root` MUST be computed as the Merkle root over `P` using the Merkle construction rules and domain separation tags defined in the Canonical Encoding and Hashing Specification (v0).

This value equals `pocket_map_payload_root(H)` as defined in the Shared Map specification.

### 7.4 snapshot_hash
The snapshot hash commits to the full header and body:

snapshot_hash = HASH("snapshot" || header_bytes || body_bytes)

`header_bytes` includes all bytes from `magic` through the end of the section directory.


## 8. verification procedure

A conformant verifier MUST perform the following steps in order:

1) Read and validate header fields (magic, version, flags, lengths).
2) Parse the section directory and read each section by byte length.
3) For each section, compute `section_hash` and compare to the directory entry.
4) Recompute `state_root_hash` and compare to the header field.
5) Recompute `title_sentence_payload_root` and compare to the header field.
6) Recompute `snapshot_hash` and compare to the expected value (e.g., `snapshot_id`).
7) If the verifier has access to the canonical event log:
   - deterministically replay through `block_height` and verify the derived state hash matches `state_root_hash`.
   - confirm that rulebook activation at this boundary matches `rulebook_set` and `active_rulebook_set_hash`.

Any mismatch renders the snapshot invalid.

---

## 9. extension and compatibility rules

### 9.1 forward compatibility
- New optional sections MUST use IDs in `0x8000`-`0xFFFF`.
- Unknown extension sections MUST be skipped by v0 readers but MUST still be hashed into `snapshot_hash`.
- Extension sections MUST NOT affect `state_root_hash` unless and until a new snapshot format version explicitly includes them.

### 9.2 backward compatibility
- A reader MUST reject any snapshot whose `format_version` is not `0`.
- A reader MAY accept snapshots with additional extension sections, as long as required sections are present and valid.

### 9.3 schema evolution
- New enum values MUST be introduced only via rulebook or protocol updates and MUST be version-gated.
- Any change to required sections or hashing rules requires a new snapshot format version.

---

## 10. conformance fixtures (format + required vectors; values may be TODO)

Conformance fixtures MUST include:

1) Minimal snapshot fixture
   - One identity, one idea, title + sentence, no connections.
   - Valid hashes and roots.

2) Multi-idea snapshot fixture
   - At least two ideas with connections and rankings.
   - Valid title/sentence payload root.

3) Safety classification fixture
   - Includes at least one `sensitive_abstracted` representation.

4) Token fixture
   - Includes at least one non-zero POD/POINT balance.

5) Replay verification fixture
   - Event log + snapshot pair where replay reproduces `state_root_hash` byte-for-byte.

Each fixture MUST provide:
- the full snapshot file bytes
- the expected `snapshot_hash`, `state_root_hash`, and `title_sentence_payload_root`
- the expected `shared_map_commitment`

Values MAY be marked TODO initially, but the fixture structure and required fields are normative.

---

# Appendix A: Conformance Fixtures 

## A. Minimal Genesis Snapshot (Height 0)

- Events: empty log
- state_root_hash: fixed empty state root computed via the canonical HASH rules with a v0 domain tag (see Canonical Encoding and Hashing Specification (v0)); any literal algorithm names here are non-normative.
- title_sentence_payload_root: fixed empty payload root (BLAKE3("seed-empty-payload"))
- shared_map_commitment: HASH("shared_map_commitment_v0" || empty_state_root || empty_payload_root)
- snapshot_hash: computed as `HASH("snapshot" || header_bytes || body_bytes)` per Canonical Encoding and Hashing Specification (v0) (symbolic reference; concrete hashing is defined there).
- Expected values:
  - state_root_hash: [computed hex, e.g., all zeros placeholder normalized]
  - shared_map_commitment: [computed hex]

## B. Single-Idea Snapshot

- Materialized state contains one idea plus two selected representation objects:
  - `representation_kind = title`, payload "Fundamental Truth", with no tier fields;
  - `representation_kind = description`, `tier_length = sentence`, with a valid
    complexity tier and payload "Collective reasoning requires verifiable shared
    reality."
- Embedded Tier 0 payloads normalized
- state_root_hash: Merkle over single idea entry
- title_sentence_payload_root: Merkle over two payload-kind leaves (title + selected sentence)
- Full header + sections bytes provided
- Expected:
  - state_root_hash: [hex]
  - title_sentence_payload_root: [hex]
  - shared_map_commitment: [hex]
  - snapshot_hash: [hex]

## C. Multi-Idea with Connection

- Two ideas + one relative_importance connection (weight 1.0)
- Derived rank preview (non-canonical)
- Full computation chain with hex outputs

(Provide complete byte examples and hashes for at least these three fixtures.)

---

# Appendix B: Stage 0 implementation plan (non-normative)

This appendix outlines the minimal implementation steps needed to produce Stage 0 snapshots that conform to the **Stage 0 snapshot scope** section above.

1) Materialize a stable Stage 0 snapshot height `H` (latest block height or explicit operator-specified height).
2) Encode `ideas_s0` and `connections_s0` sections using canonical encoding rules and canonical log order.
3) Compute `section_hash` for each section and derive `state_root_hash`.
4) Compute `title_sentence_payload_root` by canonicalizing title/sentence payloads and hashing leaf bytes.
5) Compute `shared_map_commitment` using the standard formula.
6) Persist snapshot metadata with `state_root_hash`, `title_sentence_payload_root`, and `shared_map_commitment` populated.
7) At scheduled block-height boundaries, emit `snapshot_commit` as the canonical index event for the derived artifact.

Schema additions required for Stage 1+ (not part of Stage 0):
- identities
- representations
- challenges
- verdicts
- rulebook_set
