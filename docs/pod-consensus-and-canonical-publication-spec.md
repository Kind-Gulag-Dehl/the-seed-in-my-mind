---
doc_id: pod_consensus_and_canonical_publication_spec
title: Staged Canonical Publication Specification
status: authoritative
version: v1
last_reviewed: 2026-03-05

scope:
  - Defines staged canonical publication profiles for single-node bootstrap, witnessed single-publisher operation, and full multi-node prefix finality.
  - Defines canonical publication authority objects, deterministic ordering of prefix extensions, availability certification, omission auditability, and finalized-prefix continuity.
  - Defines how derived publication blocks preserve the public `(block_height, event_index)` address surface without acting as the authority source for order.

authoritative_for:
  - Canonical publication order across Profiles 0, 1, and 2.
  - Availability attestations, prefix certificates, omission proofs, and profile transition rules.
  - Deterministic mapping from finalized canonical sequence to derived publication blocks.

not_authoritative_for:
  - Truth, importance, action, governance, safety, token, or challenge semantics beyond assigning canonical publication order.
  - Cycle derivation, rulebook activation timing, or snapshot contents.

depends_on:
  - protocol v5.md
  - protocol v5-appendix-a.md
  - deterministic-replay-and-merge-spec.md
  - node-and-conformance-spec.md
  - canonical-encoding-and-hashing-spec.md
  - canonical-preservation-and-provenance-spine-spec.md
  - snapshot-format-v0.md
  - offline-and-mindseed-spec.md
  - governance-spec.md
  - verification-spec.md
  - challenge-engine-spec.md
  - cycle-spec.md

conflicts:
  - This document supersedes the earlier proposer-led PoD block-finality model while retaining the historical file path for compatibility.

change_rules:
  - Any change here requires review of protocol v5.md, protocol v5-appendix-a.md, node-and-conformance-spec.md, deterministic-replay-and-merge-spec.md, offline-and-mindseed-spec.md, snapshot-format-v0.md, canonical-preservation-and-provenance-spine-spec.md, and canonical-encoding-and-hashing-spec.md.

reader_path:
  - prereq: protocol v5.md
  - prereq: deterministic-replay-and-merge-spec.md
  - prereq: node-and-conformance-spec.md
  - next: snapshot-format-v0.md

keywords:
  - canonical publication
  - prefix finality
  - availability attestation
  - prefix certificate
  - derived publication block
  - omission proof
  - profile 0
  - profile 1
  - profile 2
---

# Staged Canonical Publication Specification

## 1. Purpose and non-goals [anchor: purpose_and_non_goals]

### 1.1 Purpose [anchor: purpose]

This specification defines how independent nodes assign one shared canonical publication order to already-authored events.

The publication system is staged:

- **Profile 0**: bootstrap single-publisher
- **Profile 1**: witnessed single-publisher
- **Profile 2**: multi-node availability-certified prefix finality

All three profiles use the same authority objects and the same deterministic ordering rules. Profiles 0 and 1 are strict restricted cases of Profile 2.

### 1.2 Non-goals [anchor: non_goals]

This specification does not:

- decide truth, importance, action, governance, safety, token, or challenge outcomes,
- use wall-clock time, timestamps, or local receipt order as canonical authority,
- create alternate canonical histories for offline partitions,
- grant privileged semantic authority to publishers, witnesses, committees, operators, or infrastructure actors,
- or replace deterministic replay as the source of meaning.

## 2. Definitions and terminology [anchor: definitions_and_terminology]

- **AuthoredEvent**
  A canonical event authored and signed by a verified human identity according to Protocol v5 Appendix A.

- **AvailabilityAttestation**
  A signed statement by an eligible witness that it possesses the exact canonical bytes for one `AuthoredEvent`.

- **PrefixCertificate**
  The canonical publication finality object. A valid `PrefixCertificate` finalizes one ordered extension to the previously finalized prefix.

- **DerivedPublicationBlock**
  A deterministic packaging artifact derived from the finalized canonical sequence after prefix finality. It preserves the public `(block_height, event_index)` address surface but is not the authority source for order.

- **Ready frontier**
  The set of non-canonical events that are both availability-certified under the active profile and dependency-ready against a given parent prefix.

- **Dependency-ready**
  An event is dependency-ready when all explicit dependencies, all required same-author predecessor edges, and all required `PublicationPack` predecessor edges are already canonical or appear earlier in the same candidate prefix extension.

- **Readiness ordinal**
  The first certificate ordinal at which an event becomes both availability-certified and dependency-ready.

- **Witness**
  A verified human identity eligible to sign `AvailabilityAttestation` objects.

- **Committee**
  The equal-weight set of verified human identities sampled deterministically for one certificate ordinal in Profile 2. Profiles 0 and 1 use a singleton committee.

- **Publisher**
  The sole committee seat in Profiles 0 and 1. The publisher finalizes certificates in those profiles but does not gain semantic authority beyond publication order.

- **Omission proof**
  A minimal challenge artifact showing that an event was in the committed ready frontier for a certificate but was excluded in violation of deterministic ordering and capacity rules.

## 3. Canonical authority objects [anchor: canonical_authority_objects]

### 3.1 AuthoredEvent [anchor: authored_event]

`AuthoredEvent` is the existing canonical event envelope plus payload defined by Protocol v5 Appendix A.

Normative requirements:

- canonical bytes, hash, and signature verification are defined exclusively by Protocol v5 Appendix A and the Canonical Encoding and Hashing Specification,
- authored events are human-authored only,
- an authored event is **not canonical** merely because it is validly signed,
- an authored event becomes canonical only when included in a valid finalized `PrefixCertificate`.

### 3.2 AvailabilityAttestation [anchor: availability_attestation]

`AvailabilityAttestation` is the canonical byte sequence of the following fields in order:

1. `attestation_version` (`u16`) = `1`
2. `profile_id` (`u8`)
3. `basis_certificate_hash` (`hash32`)
4. `event_hash` (`hash32`)
5. `attestor_identity_id` (`id`)
6. `public_key_ref` (`id`)
7. `statement_kind` (`u8`) = `1` for `have_exact_event_bytes_v1`

The attestation hash is:

```text
availability_attestation_hash =
  HASH("availability_attestation_v1" || encode_fields_in_order)
```

The signature is a valid signature by `public_key_ref` over `availability_attestation_hash`.

Validation rules:

- `event_hash` MUST match the canonical event hash of the referenced `AuthoredEvent`,
- `basis_certificate_hash` MUST reference the finalized prefix view against which the attestor claims availability,
- `attestor_identity_id` MUST be a verified human identity eligible to witness under the active profile,
- `public_key_ref` MUST be an authorized key for `attestor_identity_id` at `basis_certificate_hash`,
- an attestation states exact-byte availability only; it does not attest truth, quality, importance, or semantic validity.

### 3.3 PrefixCertificate [anchor: prefix_certificate]

`PrefixCertificate` is the canonical byte sequence of the following body fields in order:

1. `certificate_version` (`u16`) = `1`
2. `profile_id` (`u8`)
3. `certificate_ordinal` (`u64`)
4. `parent_certificate_hash` (`hash32`)
5. `parent_canonical_event_count` (`u64`)
6. `extension_event_count` (`u32`)
7. `extension_events_root` (`hash32`)
8. `ready_frontier_event_count` (`u32`)
9. `ready_frontier_root` (`hash32`)
10. `committee_basis_snapshot_hash` (`hash32`)
11. `committee_seed` (`hash32`)
12. `committee_root` (`hash32`)
13. `tie_break_salt` (`hash32`)
14. `quorum_numerator` (`u16`)
15. `quorum_denominator` (`u16`)
16. `diversity_floor` (`u16`)

The certificate body hash is:

```text
prefix_certificate_hash =
  HASH("prefix_certificate_v1" || encode_fields_in_order)
```

The full `PrefixCertificate` additionally contains ordered signer tuples:

1. `signer_count` (`u16`)
2. `signer_tuples` in canonical committee order, where each tuple is:
   - `signer_identity_id` (`id`)
   - `public_key_ref` (`id`)
   - `signature` (`bytes`)

Each signer signs `prefix_certificate_hash`.

Validation rules:

- `certificate_ordinal = parent.certificate_ordinal + 1`,
- `parent_canonical_event_count + extension_event_count` MUST equal the finalized canonical event count after this certificate,
- `extension_events_root` MUST commit to the ordered extension event hashes exactly as finalized,
- `ready_frontier_root` MUST commit to the ordered ready frontier used for omission auditability,
- `committee_basis_snapshot_hash`, `committee_seed`, and `committee_root` MUST match deterministic committee derivation under the active profile,
- signer tuples MUST be distinct by identity,
- signer tuples MUST satisfy quorum and diversity requirements for the active profile,
- signer keys MUST be authorized for the signing identities at the parent prefix.

### 3.4 DerivedPublicationBlock [anchor: derived_publication_block]

`DerivedPublicationBlock` is optional transport and storage packaging built *after* canonical publication order is finalized.

Its canonical byte sequence contains:

1. `block_version` (`u16`) = `1`
2. `block_height` (`u64`)
3. `parent_block_header_hash` (`hash32`)
4. `start_canonical_event_index` (`u64`)
5. `event_count` (`u32`)
6. `block_event_merkle_root` (`hash32`)

The block header hash is:

```text
derived_publication_block_hash =
  HASH("derived_publication_block_v1" || encode_fields_in_order)
```

Derived blocks:

- MUST be derived deterministically from the finalized canonical sequence using the configured block size,
- MUST NOT create, change, or delay canonical order,
- MAY span certificate boundaries,
- MAY be omitted, regenerated, or re-served without changing canonical meaning.

## 4. Event states [anchor: event_states]

Every event moves through the following states:

- **authored**
  The `AuthoredEvent` exists, is validly signed, and may circulate.

- **availability-certified**
  The event has the required number of valid `AvailabilityAttestation` objects under the active profile.
  - Profile 0: not required
  - Profile 1: required
  - Profile 2: required

- **canonical**
  The event appears in the ordered extension of a valid finalized `PrefixCertificate`.

No event becomes canonical by attestation alone.

## 5. Profiles [anchor: profiles]

### 5.1 Shared profile invariants [anchor: shared_profile_invariants]

All profiles share the following invariants:

- canonical order comes only from valid finalized `PrefixCertificate` objects,
- deterministic ordering is identical across profiles,
- blocks are derived packaging only,
- offline work remains delayed publication,
- no profile may use wall-clock time, timestamps, or local receipt order as authority,
- governance activates profile changes only at deterministic cycle boundaries.

### 5.1.1 Active publication parameters [anchor: active_publication_parameters]

The active rulebook MUST expose or deterministically imply the following publication parameters:

- `bootstrap_publisher_identity_id`
- `committee_target_size`
- `prefix_extension_event_cap`
- `availability_threshold(profile_id)`
- `diversity_floor`

Defaults, unless superseded by governance:

- `committee_target_size = 31`
- `prefix_extension_event_cap = 50`

In Profiles 0 and 1, `committee_target_size` is interpreted as `1`.

### 5.2 Profile 0: bootstrap single-publisher [anchor: profile_0_bootstrap_single_publisher]

Profile 0 is the bootstrap mode for a single operating publisher.

Rules:

- committee size = `1`,
- quorum = `1/1`,
- diversity floor = `1`,
- the singleton committee seat is the `bootstrap_publisher_identity_id` active at the parent prefix,
- availability attestations are optional and not required for conformance,
- the publisher finalizes a `PrefixCertificate` directly.

Conformance note:

- Profile 0 is explicitly centralized and MUST be presented as such.

### 5.3 Profile 1: witnessed single-publisher [anchor: profile_1_witnessed_single_publisher]

Profile 1 preserves the singleton publisher but requires availability plumbing.

Rules:

- committee size = `1`,
- quorum = `1/1`,
- diversity floor = `1`,
- the singleton committee seat remains the active publisher identity,
- every non-boundary event included in the certificate extension MUST be availability-certified,
- the certificate MUST commit to a `ready_frontier_root`,
- repeated omission of ready, availability-certified events is objectively challengeable.

Profile 1 exists to prove transport, witness, and omission-auditability surfaces before activating multi-node finality.

### 5.4 Profile 2: multi-node availability-certified prefix finality [anchor: profile_2_multi_node_availability_certified_prefix_finality]

Profile 2 is the end-state canonical publication profile.

Rules:

- the committee is sampled deterministically from canonical state,
- quorum is `>= 2/3`,
- diversity floor is rulebook-defined and MUST be satisfied in addition to quorum,
- every included non-boundary event MUST be availability-certified,
- any node may assemble and relay candidate prefix proposals,
- only threshold-signed `PrefixCertificate` objects finalize canonical order.

There is no canonical proposer schedule and no trusted scheduler role.

### 5.5 Profile transition rules [anchor: profile_transition_rules]

Profile transition is governance-controlled.

Rules:

- genesis defaults to Profile 0 unless the genesis rulebook states otherwise,
- transitions activate only at deterministic cycle boundaries,
- a transition MUST specify the new `profile_id`, the effective boundary, and any updated quorum / committee parameters,
- Profile 0 -> Profile 1 -> Profile 2 is the normal forward path,
- silent downgrade is forbidden,
- if Profile 2 loses quorum, canonical publication stalls rather than reverting automatically to Profile 1 or Profile 0,
- downgrade or recovery requires explicit governance or an explicit catastrophe-restart lineage procedure defined elsewhere.

## 6. Candidate event intake and dissemination [anchor: candidate_event_intake_and_dissemination]

Nodes MAY disseminate the following non-final artifacts:

- `AuthoredEvent` bytes,
- `AvailabilityAttestation` objects,
- `PublicationPack` bundles,
- candidate prefix proposals,
- omission-proof and equivocation-proof materials.

Transport, gossip, batching, retransmission, and local timeout policy are implementation-defined.

Canonical validity depends only on the finalized objects defined in this specification.

`PublicationPack` requirements:

- preserves the authored event order declared by the pack,
- may include offline provenance, custody manifests, and witness receipts,
- is not canonical by itself,
- creates deterministic predecessor edges between adjacent events in the pack order.

## 7. Deterministic ordering rules for prefix extensions [anchor: deterministic_ordering_rules_for_prefix_extensions]

### 7.1 Inputs [anchor: ordering_inputs]

For parent certificate `P`, compute the candidate frontier from all non-canonical authored events known to the node.

An event is eligible for the ready frontier only if:

- its canonical bytes and signature are valid,
- it satisfies the profile-specific availability requirement,
- it is not already canonical,
- it is not invalid under replay against parent state,
- and all explicit dependencies are either canonical in `P` or are also candidates for earlier placement.

### 7.2 Ordering constraints [anchor: ordering_constraints]

Within each finalized extension, nodes MUST preserve the following precedence constraints:

1. explicit causal dependency edges,
2. same-author declared sequence edges, when the event type defines them,
3. `PublicationPack` sequence edges,
4. mechanical boundary events, including `cycle_close`, at the earliest replay-valid position.

### 7.3 Earliest-valid mechanical insertion [anchor: earliest_valid_mechanical_insertion]

Mechanical boundary events are not discretionary.

If replay of parent state plus already-selected extension events reaches the earliest point at which a mechanical boundary event becomes valid, that boundary event MUST be inserted before any later-ready non-boundary event.

### 7.4 Readiness ordinal [anchor: readiness_ordinal]

For each eligible event `E`, define `readiness_ordinal(E)` as the smallest certificate ordinal at which:

- `E` was availability-certified under the active profile, and
- all required predecessor edges were satisfied by the parent prefix or by earlier events in the same extension.

`readiness_ordinal` is a canonical fairness input. It is not wall-clock time.

### 7.5 Selection algorithm [anchor: selection_algorithm]

Let `cap` be the active `prefix_extension_event_cap`.

The ordered extension is constructed iteratively:

1. Initialize working replay state from parent certificate `P`.
2. Compute the ready frontier `R(P)` as all dependency-ready events against the current working state.
3. If an earliest-valid mechanical boundary event exists, select it next.
4. Otherwise, among all events in `R(P)`, choose the minimum ordering key:
   - `readiness_ordinal(E)` ascending,
   - `HASH("prefix_tie_break_v1" || P.prefix_certificate_hash || event_hash(E))` ascending.
5. Apply the event to working replay state and continue until:
   - no ready events remain, or
   - `cap` events have been selected.

If two events would create a cycle in the precedence graph, neither is ready until the cycle is broken by prior canonical history or governance-defined invalidation.

### 7.6 Ready frontier commitment [anchor: ready_frontier_commitment]

The `ready_frontier_root` in a certificate commits to the entire ordered ready frontier *before* truncation by `cap`.

Construction:

- order the ready frontier by the same deterministic ordering rules used for unbounded selection,
- hash the ordered event hashes with the canonical Merkle rules,
- use the resulting root as `ready_frontier_root`.

If the ready frontier is empty, use:

```text
HASH("empty_ready_frontier_v1")
```

## 8. Availability certification [anchor: availability_certification]

### 8.1 Eligible witness pool [anchor: eligible_witness_pool]

The eligible witness pool at parent certificate `P` is the set of verified human identities that:

- appear in the latest finalized universal voter snapshot at or before `P`,
- possess at least one currently authorized signing key,
- are not suspended from publication participation under active governance,
- are not AI or synthetic identities.

### 8.2 Thresholds [anchor: availability_thresholds]

The active rulebook defines `availability_threshold(profile_id)`.

Defaults:

- Profile 0: `0`
- Profile 1: `2`
- Profile 2: `max(3, ceil(committee_target_size / 3))`

An event is availability-certified when it has valid attestations from at least `availability_threshold(profile_id)` distinct eligible witnesses.

### 8.3 Witness duties [anchor: witness_duties]

An eligible witness SHOULD attest only if it can serve the exact event bytes for a reasonable verification window.

Witnesses MUST NOT:

- attest to bytes they do not have,
- mutate payload bytes before attesting,
- or treat attestation as semantic endorsement.

### 8.4 Storage expectations [anchor: availability_storage_expectations]

Nodes claiming publication conformance MUST retain or be able to retrieve:

- all authored event bytes included in finalized certificates they serve,
- all attestations needed to prove availability certification for those events,
- and any omission-proof material they publish or answer.

## 9. Committee selection and quorum [anchor: committee_selection_and_quorum]

### 9.1 Basis snapshot [anchor: committee_basis_snapshot]

For certificate ordinal `c`, the basis snapshot is the latest finalized universal voter snapshot visible from the parent certificate.

Its snapshot hash is recorded as `committee_basis_snapshot_hash`.

### 9.2 Seed [anchor: committee_seed]

The committee seed is:

```text
committee_seed(c) =
  HASH("prefix_committee_seed_v1" ||
       parent_certificate_hash ||
       committee_basis_snapshot_hash)
```

### 9.3 Committee derivation [anchor: committee_derivation]

For each eligible identity `I` in the basis snapshot, compute:

```text
committee_score(I, c) =
  HASH("prefix_committee_member_v1" ||
       committee_seed(c) ||
       encode_id(I))
```

Sort ascending by:

1. `committee_score(I, c)`
2. canonical bytewise `identity_id`

Select the first `committee_target_size` identities.

`committee_root` is the Merkle root of the ordered list of committee identity ids.

Profiles 0 and 1 use a singleton committee whose root is the Merkle root of the single publisher identity id.

### 9.4 Quorum [anchor: quorum]

Profile 2 quorum requires:

- signatures from at least `ceil(2 * committee_size / 3)` distinct committee seats,
- and satisfaction of the active diversity floor.

Profiles 0 and 1 require the singleton signature only.

### 9.5 Diversity floor [anchor: diversity_floor]

The diversity floor is rulebook-defined and MUST be computable deterministically from canonical identity metadata visible at the parent prefix.

This specification does not define the social categories themselves. It only requires deterministic, replay-verifiable counting when a rulebook enables such a floor.

### 9.6 Equivocation and double-signing [anchor: equivocation_and_double_signing]

A publication participant objectively equivocates if it:

- signs two distinct `PrefixCertificate` bodies for the same `(parent_certificate_hash, certificate_ordinal)`, or
- signs two incompatible availability attestations claiming conflicting exact-byte hashes for the same authored event identifier where the underlying event bytes differ.

Objective equivocation is challengeable and MUST be sanctionable by eligibility suspension.

No financial slashing is assumed by this specification.

## 10. Finality and fork handling [anchor: finality_and_fork_handling]

### 10.1 Finalized means canonical [anchor: finalized_means_canonical]

An event becomes canonical only when included in a valid finalized `PrefixCertificate`.

Unfinalized proposals, partial signature sets, and transport-layer votes are not canonical.

### 10.2 Canonical certificate chain selection [anchor: canonical_certificate_chain_selection]

Canonical replay considers only the chain of valid finalized prefix certificates descended from genesis.

Selection rule:

1. follow valid parent hashes from genesis,
2. extend along the unique finalized child when one exists,
3. if a later certificate strictly extends an earlier finalized certificate, the longer finalized prefix is canonical,
4. if two conflicting finalized children exist for the same parent, canonical advancement halts at the common ancestor.

There is no longest-chain rule over unfinalized proposals.

### 10.3 Conflicting finalized certificates [anchor: conflicting_finalized_certificates]

Conflicting finalized certificates indicate a safety breach.

When observed, nodes MUST:

- preserve all conflicting certificates and signatures,
- stop advancing canon past the last uncontested ancestor,
- surface the conflict as objective equivocation evidence,
- and await governance-defined recovery or a later uncontested continuation anchored at the last safe prefix.

## 11. Omission / censorship auditability [anchor: omission_censorship_auditability]

### 11.1 Objective omission condition [anchor: objective_omission_condition]

An omission is objectively challengeable when:

- event `E` is included in the challenged certificate's committed `ready_frontier_root`,
- `E` is not included in the finalized extension,
- `E` was dependency-ready against the parent prefix,
- and either:
  - the extension terminated with unused capacity, or
  - `E` precedes at least one included event under the deterministic ordering rules.

### 11.2 Minimal omission proof [anchor: minimal_omission_proof]

A minimal omission proof consists of:

1. the challenged `PrefixCertificate`,
2. the omitted `AuthoredEvent` bytes,
3. the availability attestations required to show `E` was availability-certified,
4. a Merkle inclusion proof for `event_hash(E)` in `ready_frontier_root`,
5. dependency references sufficient to verify that `E` was ready,
6. either:
   - proof of spare capacity, or
   - a witness included event `F` such that deterministic ordering requires `E` before `F`.

If the proof verifies, the omission is objective.

### 11.3 Sanctions [anchor: omission_sanctions]

Rulebooks MAY define sanctions for objective omission, including:

- temporary suspension from witness eligibility,
- temporary suspension from committee eligibility,
- public breach recording,
- forced profile review or emergency governance action.

This specification requires challengeability and visibility, not financial penalties.

## 12. Offline integration [anchor: offline_integration]

Offline work remains delayed publication.

Rules:

- offline-authored events are ordinary `AuthoredEvent` objects,
- `PublicationPack` order creates deterministic predecessor edges during canonical publication,
- offline provenance may contribute availability attestations and witness material,
- offline authorship does not grant retroactive placement,
- offline publication does not create parallel canonical histories,
- and reintegration never performs averaging, merge-time reconciliation, or silent conflict resolution.

Canonical order is assigned only when offline events are included in finalized certificates.

## 13. Interaction with cycles, tempo, snapshots, Atlas, and L1 [anchor: interaction_with_cycles_tempo_snapshots_atlas_and_l1]

- **Cycles / tempo**
  - remain replay-derived and challengeable,
  - do not schedule certificates by clock,
  - may determine when `cycle_close` becomes mechanically insertable.

- **Snapshots**
  - remain derived verification checkpoints,
  - are keyed to derived block height,
  - do not create canonical order.

- **Atlas / packs / preservation**
  - are downstream packaging and preservation surfaces,
  - are derived from the finalized canonical sequence.

- **L1 / external anchoring**
  - may anchor finalized canonical sequence commitments,
  - MUST remain downstream of finalized prefix certificates,
  - MUST NOT become the authority source for canonical order.

## 14. Migration and compatibility notes [anchor: migration_and_compatibility_notes]

- Historical deployments may begin at Profile 0 with one publisher and no required attestations.
- Profile 1 adds witness attestations and omission-auditability without changing event bytes, replay rules, or derived block addressing.
- Profile 2 changes only the finality rule and committee derivation; it does not change event schema or deterministic ordering rules.
- Nodes SHOULD store and serve:
  - finalized prefix certificates,
  - included authored event bytes,
  - required availability attestations,
  - omission and equivocation proofs,
  - derived publication blocks if they expose `(block_height, event_index)` externally.
- Derived block mapping remains stable across profiles:
  - first finalize canonical sequence,
  - then group consecutive events into fixed-size derived blocks,
  - then assign `(block_height, event_index)`.

## 15. Node conformance checklist by profile [anchor: node_conformance_checklist_by_profile]

### 15.1 Profile 0 [anchor: node_conformance_profile_0]

A conformant Profile 0 node MUST:

- validate authored event bytes and signatures,
- finalize only through singleton `PrefixCertificate` objects,
- preserve deterministic ordering rules,
- derive blocks only after finality,
- and expose Profile 0 status clearly.

### 15.2 Profile 1 [anchor: node_conformance_profile_1]

A conformant Profile 1 node MUST additionally:

- verify availability attestations,
- require availability certification for included non-boundary events,
- commit to a ready frontier root,
- preserve omission-proof materials,
- and keep publisher finality separate from witness attestations.

### 15.3 Profile 2 [anchor: node_conformance_profile_2]

A conformant Profile 2 node MUST additionally:

- derive committees deterministically,
- verify quorum and diversity floor,
- reject certificates with inconsistent ready-frontier or ordering proofs,
- detect conflicting finalized certificates,
- and halt safely on finalized conflict rather than auto-selecting a branch.

## 16. Security considerations and failure modes [anchor: security_considerations_and_failure_modes]

- **Quorum loss**
  - Profile 2 may stall. Stalling is safer than inventing order.

- **Publisher compromise**
  - In Profiles 0 and 1, conflicting singleton certificates are objective equivocation and MUST trigger recovery review.

- **Witness collusion**
  - False availability claims are challengeable because exact event bytes and signatures are committed.

- **Payload withholding**
  - Availability attestations and omission proofs make withholding visible even when payload serving degrades.

- **Committee capture**
  - Equal-human committee weighting, deterministic sampling, and diversity floors reduce but do not eliminate capture risk.

- **Network asynchrony**
  - Transport may produce competing candidate proposals.
  - Canonical validity depends only on finalized certificates.

- **Catastrophe / restart**
  - If all public nodes disappear, surviving carriers need:
    - the last trusted finalized prefix certificates,
    - the corresponding authored event bytes,
    - the snapshot / bundle artifacts needed for verification,
    - and the governance rules required to resume publication.
