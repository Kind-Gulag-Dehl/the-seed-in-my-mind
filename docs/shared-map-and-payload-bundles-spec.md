---
doc_id: shared_map_and_payload_bundles_spec
title: Shared Map and Payload Bundles Specification
status: authoritative
version: v0
last_reviewed: 2026-01-27

scope:
  - Defines shared map commitments and payload bundle formats for distribution.

authoritative_for:
  - Bundle tier definitions and contents.
  - Shared map commitment surfaces.

not_authoritative_for:
  - Snapshot internal structure (see snapshot-format-v0.md).

depends_on:
  - canonical-encoding-and-hashing-spec.md
  - snapshot-format-v0.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of snapshot-format-v0.md and node-and-conformance-spec.md.

reader_path:
  - prereq: snapshot-format-v0.md
  - next: challenge-engine-spec.md

keywords:
  - shared map
  - payload bundles
  - distribution
  - commitments
---

# Shared Map of Reality and Payload Bundle Specification

## Deterministic Text Availability, Distribution, and Verification

---

## 0. Purpose

This document defines how the system ensures that **human-readable content** (text payloads describing ideas, truth claims, and actions) is:

* widely replicated,
* practically usable offline,
* verifiably identical across users,
* and aligned with collective determinations of importance.

The goal is not merely to prove that reasoning occurred, but to ensure that **the content of what humans determined to be important, true, and actionable is actually available to people**.

This specification introduces:

* deterministic **bundle tiers** of increasing completeness,
* a single **shared map commitment** that allows anyone to verify sameness,
* custody and redundancy expectations,
* and clear separation between canonical authority and availability mechanisms.

---

## 1. Design principles

1. **Integrity before availability**
   Canonical events commit to text payloads by hash. No transport or storage mechanism is authoritative.

2. **Availability must be explicit**
   The system MUST define what text is expected to be present, not merely allow it optionally.

3. **Importance drives replication**
   The most important ideas and truth claims MUST be the most widely available.

4. **Partial storage is acceptable, blindness is not**
   Not everyone must store everything, but everyone must have access to a shared minimal map.

5. **Determinism over curation**
   Bundle membership MUST be computable deterministically from canonical state.

The shared map commitment is defined as:
The authoritative definition of `shared_map_commitment` is in the Canonical Encoding and Hashing Specification (v0); this document repeats it only for readability and does not redefine byte-level hashing, tags, or encodings.
The `HASH(...)` notation and domain tag strings are symbolic references; their concrete byte encoding, hash algorithm, and domain separation behavior are defined exclusively by the Canonical Encoding and Hashing Specification (v0).
shared_map_commitment(H) = HASH("shared_map_commitment_v0" || state_root_hash(H) || pocket_map_payload_root(H))
This commitment serves as the sole primitive for verifying identical canonical facts and embedded Tier 0 meaning across nodes, snapshots, and bundles.

---

## 2. Canonical text payload model (recap)

* All text (titles, descriptions, arguments, evidence, etc.) is serialized canonically and hashed.
* Canonical events reference text **only by hash**.
* Text bytes may be obtained from any transport (HTTP, IPFS, P2P, physical media).
* Verification is always against the hash committed in the event log.

This document does not change that model.

All payload hashing, Merkle construction, and domain separation tags referenced by this specification MUST follow the Canonical Encoding and Hashing Specification (v0) (including canonical payload normalization rules and rejection rules for alternate encodings).

---

## 3. Bundle tiers (standardized distribution artifacts)

The system defines a small number of **standard bundle tiers**. Each tier is a deterministic set of payloads selected from canonical state at a given block height. Bundles MUST record the Canonical Encoding and Hashing Specification version identifier (v0), satisfied by including a Snapshot Format v0 header with format_version = 0.

### 3.1 Tier 0 — “Pocket Map”

**Purpose**
Near-universal distribution. Small enough to fit on phones and low-bandwidth devices.

**Required contents**

* A Snapshot Format v0 full snapshot at block height H.
* Title tier text for all ideas and orderings present in the snapshot's state.
* Sentence-tier description for all ideas and orderings present in the snapshot's state.
* Optional minimal UI assets and local search index (non-canonical).

**Properties**

* All bundles anchor to a snapshot's `shared_map_commitment`. Tier 0 bundles include embedded title/sentence payloads matching the basis snapshot.
* Guarantees basic legibility of the entire living map.
* No deep reasoning or evidence included.

---

### 3.2 Tier 1 — “Citizen Map”

**Purpose**
Default offline encyclopedia for ordinary users.

**Required contents**

* Tier 0 contents.
* Paragraph-tier descriptions for the top-K ideas by universal importance at H.
* Paragraph-tier descriptions for top-K truth claims (across truth subtypes).
* Minimal reasoning summaries (deterministically selected tiers, not full history).

**Properties**

* All bundles anchor to a snapshot's `shared_map_commitment`. Tier 0 bundles include embedded title/sentence payloads matching the basis snapshot.
* Explains what is important and true, in readable form.
* Does not require full event history.

---

### 3.3 Tier 2 — “Civic Archive”

**Purpose**
Communities, schools, libraries, research groups.

**Required contents**

* Tier 1 contents.
* Full-tier descriptions for top-K ideas and truth claims.
* Argument and evidence text payloads that justify the current rankings/verdicts for top-K.
* Multiple snapshot checkpoints (e.g., last N large snapshots).

**Properties**

* All bundles anchor to a snapshot's `shared_map_commitment`. Tier 0 bundles include embedded title/sentence payloads matching the basis snapshot.
* Preserves the “why,” not just the conclusions.
* Suitable for education and institutional memory.

---

### 3.4 Tier 3 — “Full Archive”

**Purpose**
Long-term preservation and deep auditing.

**Required contents**

* All payload bundles.
* Full canonical event log (or complete snapshot chain + deltas).
* All description tiers and historical reasoning artifacts.

**Properties**

* All bundles anchor to a snapshot's `shared_map_commitment`. Tier 0 bundles include embedded title/sentence payloads matching the basis snapshot.
* Maximum durability.
* Not required for ordinary participation.

---

## Publication and Retention Schedule

This section defines a dissemination policy over existing canonical artifacts (Snapshot Format v0 snapshots and payload packs). It does not introduce any new canonical format or snapshot kind.

### Variables (normative)

- **N_pocket_blocks**: block interval for Pocket Map (Tier 0) publication.
- **N_citizen_blocks**: block interval for Citizen Map (Tier 1) publication.
- **N_archive_blocks**: block interval for Civic Archive (Tier 2) publication.
- **keep_last_pocket**: number of most recent Pocket bundles to retain.
- **keep_last_citizen**: number of most recent Citizen bundles to retain.
- **keep_last_archive**: number of most recent Archive bundles to retain.

### Defaults (non-normative example)

Example defaults aligned to the Snapshot Format v0 snapshot interval:

```
N_pocket_blocks = the Snapshot Format v0 snapshot interval (see snapshot-format-v0.md)
N_citizen_blocks = 10 * N_pocket_blocks
N_archive_blocks = 100 * N_pocket_blocks
keep_last_pocket = 30
keep_last_citizen = 12
keep_last_archive = 24
```

### Normative rules

- A bundle published at height H MUST use the Snapshot Format v0 snapshot keyed to height H (see snapshot-format-v0.md).
- If snapshots exist only at certain heights, bundle publication heights MUST align to those snapshot heights.
- Payload pack inclusion cadence MUST follow the pack cadence rules in canonical-preservation-and-provenance-spine-spec.md.
- "Lighter" and "heavier" refer to bundle contents and frequency, not snapshot kind.

### Retention semantics

Retention is defined as "keep last K bundles of each tier." Retention policy MUST NOT affect canonical validity; it only affects availability and distribution.

Offline nodes MAY use additional retention profiles for local storage (see offline-and-mindseed-spec.md); this section defines the standard public bundle schedule.

## 4. Deterministic bundle selection rules

For each tier, bundle membership MUST be computable from:

* snapshot state at block height H,
* importance ranks at H,
* idea type and description tier metadata.

No human curation or discretionary inclusion is permitted for core tiers.

This ensures:

* independent nodes compute identical bundle contents,
* custody claims are verifiable,
* disagreements are detectable.

---

## 5. Shared map commitment

The hash() function, byte concatenation rules, and the domain separation tag used for shared_map_commitment MUST be exactly those defined in the Canonical Encoding and Hashing Specification (v0).

To allow anyone to verify that they share the same “map of reality,” the system defines a **Shared Map Commitment**.

At block height H:
The authoritative definition of `shared_map_commitment` is in the Canonical Encoding and Hashing Specification (v0); this document repeats it only for readability and does not redefine byte-level hashing, tags, or encodings.

```
shared_map_commitment(H) =
  HASH("shared_map_commitment_v0"
    || state_root_hash(H)
    || pocket_map_payload_root(H)
  )
```

Where:

* `state_root_hash(H)` is the canonical facts commitment.
* `pocket_map_payload_root(H)` is a Merkle root over Tier 0 payload leaf bytes `(object_kind, object_id, tier_enum, payload_hash)` at H, where `object_kind` is `idea` or `ordering`, per the Canonical Encoding and Hashing Specification (v0).

title_sentence_payload_root(H) equals pocket_map_payload_root(H) and MUST be identical to the Snapshot Format v0 header field title_sentence_payload_root at the same block height H.

This commitment is stable across derived view recomputation and changes only when canonical facts or the Tier 0 payload set changes.

**Interpretation**

* If two people share the same commitment, they have:

  * the same state,
  * the same minimal human-readable content,
  * derived from the same collective decisions.

Higher tiers extend this, but Tier 0 defines the baseline shared reality.

---

## 6. Snapshots and mandatory embedded text

To prevent “state without meaning,” snapshots MUST embed:

* title tier text for each idea and ordering in snapshot state,
* sentence-tier description for each idea and ordering in snapshot state.

These embedded bytes are:

* deterministic,
* hash-verifiable,
* a convenience cache (not authoritative beyond the committed hash).

This guarantees that any snapshot is immediately legible without fetching external packs.

---

## 7. Custody, replication, and redundancy

### 7.1 Node classes and obligations

Different node classes MAY be defined (light, standard, archival), but:

* At least one node class MUST be required to store Tier 1 bundles.
* Tier 0 MUST be trivially hostable by ordinary users.

### 7.2 Custody manifests

Nodes MAY publish **custody manifests** declaring:

* which bundle tiers they store,
* Merkle roots of stored payload sets.

Manifests are verifiable but non-authoritative.

### 7.3 Redundancy targets

The system SHOULD define redundancy targets such as:

* minimum independent custodians per region for Tier 1,
* higher thresholds for Tier 2.

Detection of under-replication MAY trigger:

* human alerts,
* entling assistance,
* incentives (defined elsewhere).

---

## 8. Transport independence

Bundles may be distributed via:

* HTTP mirrors,
* IPFS or similar content-addressed networks,
* peer-to-peer meetups,
* physical media.

Transport choice does not affect verification or authority.

---

## 9. Offline and authoritarian environments

In offline or censored environments:

* Tier 0 and Tier 1 bundles are the primary survival artifacts.
* Offline partitions fork from known snapshot + bundle roots.
* Reconciliation later verifies payload hashes against canonical commitments.

This ensures that even under prolonged isolation, communities retain:

* a shared map,
* the text that defines it,
* and the ability to verify integrity later.

---

## 10. Non-goals

This specification does NOT:

* require everyone to store everything,
* guarantee perfect global availability at all times,
* replace the canonical event log.

It guarantees **meaningful availability of what matters most**, not total completeness.

---

## 11. Summary

The system’s mission requires more than an immutable log.
It requires **durable, shared access to the content of human understanding**.

By defining:

* deterministic bundle tiers,
* a shared map commitment,
* mandatory minimal text inclusion,
* and explicit custody expectations,

the system ensures that:

* people can always obtain the most important ideas,
* verify they match everyone else’s,
* and carry that knowledge forward even when infrastructure fails.

This completes the bridge between **proof of reasoning** and **shared human meaning**.



