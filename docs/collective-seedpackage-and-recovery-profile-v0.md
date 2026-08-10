---
doc_id: collective_seedpackage_and_recovery_profile_v0
title: Collective SeedPackage and Recovery Profile v0
status: authoritative
version: v0
last_reviewed: 2026-08-10

scope:
  - Unifies the existing map-bundle, payload-pack, retention, custody, and node-role vocabularies.
  - Defines the interoperable SeedPackage v0 container and manifest.
  - Defines Full Recovery Bundle v0 and Archive Shard Set v0.
  - Defines packaging requirements for ordinary and catastrophe-successor lineages.

authoritative_for:
  - SeedPackage profile identifiers and required composition.
  - SeedPackage container paths, manifest fields, and local verification closure.
  - Full Recovery Bundle and deterministic archive-shard composition.

not_authoritative_for:
  - Canonical event bytes, identifiers, hashing primitives, or Merkle mechanics.
  - Snapshot internal structure.
  - Canonical publication order, ordinary finality, or catastrophe-successor authority.
  - Private Mindseed journal contents or private product behavior.

depends_on:
  - protocol v5.md
  - canonical-encoding-and-hashing-spec.md
  - snapshot-format-v0.md
  - shared-map-and-payload-bundles-spec.md
  - canonical-preservation-and-provenance-spine-spec.md
  - offline-and-mindseed-spec.md
  - pod-consensus-and-canonical-publication-spec.md
  - node-and-conformance-spec.md

conflicts:
  - none known

change_rules:
  - Changes to canonical field bytes or domain tags require canonical-encoding-and-hashing-spec.md review.
  - Changes to lineage authority or restart finality require pod-consensus-and-canonical-publication-spec.md and Protocol v5 review.
  - Changes to snapshot contents require snapshot-format-v0.md review.
---

# Collective SeedPackage and Recovery Profile v0

## 0. Purpose and authority [anchor: purpose_and_authority]

The system already defines Tier 0 Pocket Maps, Tier 1 Citizen Maps, Tier 2 Civic
Archives, Tier 3 Full Archives, standard payload packs, Personal Custody Sets
(PCS), Collective Custody Sets (CCS), offline retention profiles, and portable
Mindseeds / SeedPackages. This profile does not introduce a parallel naming
system. It defines how those existing artifacts compose into interoperable,
verifiable packages.

This profile governs packaging and recovery data. It MUST NOT change canonical
event validity, event ordering, deterministic replay, ordinary publication
finality, governance authority, or human eligibility. Where this profile
conflicts with an owner specification, the owner specification controls:

1. `canonical-encoding-and-hashing-spec.md` for exact bytes and hashes;
2. `pod-consensus-and-canonical-publication-spec.md` for publication and
   catastrophe-successor lineage authority;
3. `snapshot-format-v0.md` for snapshot structure;
4. `shared-map-and-payload-bundles-spec.md` for Tier 0-3 map membership;
5. `canonical-preservation-and-provenance-spine-spec.md` for payload packs and
   preservation evidence;
6. `offline-and-mindseed-spec.md` for private/offline lanes and reintegration.

## 1. Closed terminology [anchor: closed_terminology]

The following terms are distinct and MUST NOT be used interchangeably:

- **Genesis Seed data**: pre-genesis source material used to construct the
  initial canonical event sequence. It is not a portable SeedPackage profile.
- **Canonical snapshot**: replay-derived state at a canonical block height.
- **Payload pack**: a deterministic set of content-addressed payload blobs.
- **Map bundle**: a Tier 0-3 distribution selection anchored to one canonical
  snapshot and its `shared_map_commitment`.
- **SeedPackage**: a transport container holding one map bundle plus the exact
  history, certificates, packs, software, or recovery material required by its
  declared profile.
- **Private Mindseed**: user-controlled non-canonical private state and, when
  present, a separate append-only stream of exact signed publication candidates.
- **Full Recovery Bundle**: a SeedPackage profile sufficient to reconstruct,
  verify, inspect, and restart compatible operation without a live service.
- **Archive Shard Set**: a deterministic derived encoding of a Full Recovery
  Bundle that permits reconstruction after bounded shard loss.

`Pocket Seed`, `Living Seed`, and `Ark Seed` are not protocol profile names.

## 2. Normative artifact crosswalk [anchor: normative_artifact_crosswalk]

| Existing artifact | Canonical or derived role | Minimum package use | Custody expectation |
|---|---|---|---|
| Tier 0 Pocket Map | Canonical snapshot plus mandatory shallow meaning | `pocket_map_v0` and every larger map profile | Near-universal distribution; CCS baseline |
| Tier 1 Citizen Map | Tier 0 plus bounded deeper collective meaning | `citizen_map_v0`, default offline reading | Ordinary clients and full nodes |
| Tier 2 Civic Archive | Tier 1 plus explanatory closure and checkpoints | `civic_archive_v0` | Communities, schools, libraries, research groups |
| Tier 3 Full Archive | Complete public history and payload depth | `full_archive_v0`, `full_recovery_v0` | Multiple independent archival custodians |
| Core Library Pack | Deep payloads for universally important ideas | Tier 1-3 according to bundle rules | Broad CCS replication |
| Living Map Pack | Shallow payloads for the current living map | Tier 0-3 | Ordinary full nodes |
| Cycle Delta Pack | New or changed payloads since the prior cycle | Incremental synchronization and evaluation buffer | Retained across the rulebook-defined delta window |
| Archive Pack | All payloads across all history | Tier 3 and Full Recovery | Archival custodians and shard producers |
| PCS | Personally selected canonical custody responsibility | Any local retention profile | Multiple socially uncorrelated custodians where shared |
| CCS | Replay-derived collective custody responsibility | Tier 0, Core Library, governance, safety, and recovery material | Broad population replication |
| Light retention | Selected readable state and anchors | Pocket or Citizen package | Low-resource clients |
| Snapshot-based retention | Trusted basis snapshot plus exact continuation history | Playable Offline package | Ordinary offline nodes |
| Archival retention | Full deterministic history | Full Archive or Full Recovery | Archive nodes |

Node conformance and storage are orthogonal. A verifier MAY be conformant without
hosting a Full Archive, while an archival custodian MUST truthfully declare and
prove the exact artifacts it serves.

## 3. SeedPackage profiles [anchor: seedpackage_profiles]

The following profile identifiers are closed for version 0:

1. `pocket_map_v0`
2. `citizen_map_v0`
3. `civic_archive_v0`
4. `full_archive_v0`
5. `playable_offline_v1`
6. `full_recovery_v0`
7. `archive_shard_set_v0`

Additional profiles require a new identifier. Existing identifiers MUST NOT be
redefined retroactively.

The first six identifiers are SeedPackage container profiles.
`archive_shard_set_v0` is a derived distribution profile over one
`full_recovery_v0` package and is exempt from the basis-snapshot path requirement
for a `.seedpkg` container.

### 3.1 Pocket Map package [anchor: pocket_map_package]

`pocket_map_v0` MUST include:

- one valid Snapshot Format v0 full snapshot at basis block height `H`;
- every title representation and selected Tier 0 sentence description required
  by that snapshot;
- the `shared_map_commitment(H)`;
- the SeedPackage manifest.

It MAY include a non-canonical local search index and inert viewer assets. It is
readable and verifiable at its basis commitment but does not claim to contain
full deterministic history.

### 3.2 Citizen Map package [anchor: citizen_map_package]

`citizen_map_v0` MUST include:

- all `pocket_map_v0` contents;
- the Tier 1 Citizen Map selection;
- the applicable Living Map and Core Library pack commitments;
- every payload required by the Tier 1 explanatory closure in Section 4;
- the SeedPackage manifest.

### 3.3 Civic Archive package [anchor: civic_archive_package]

`civic_archive_v0` MUST include:

- all `citizen_map_v0` contents;
- the Tier 2 Civic Archive selection;
- complete direct explanatory closure for every selected subject;
- the rulebook-defined checkpoint window;
- the SeedPackage manifest.

### 3.4 Full Archive package [anchor: full_archive_package]

`full_archive_v0` MUST include:

- the complete canonical event log from genesis through the declared finalized
  prefix;
- all required authored-event bytes and finalized-prefix certificates;
- all canonical snapshots through the basis height, or a complete snapshot index
  whose referenced snapshot artifacts are included;
- the Archive Pack and every public payload required to verify it;
- active and historical rulebook commitments required for replay;
- the SeedPackage manifest.

Content that is canonically non-distributable under the safety specifications
MUST be represented by the permitted commitment, encapsulation, or safe artifact;
this profile does not override public-distribution safety rules.

### 3.5 Playable Offline package [anchor: playable_offline_package]

`playable_offline_v1` MUST include:

- at least `citizen_map_v0`;
- a declared trusted basis snapshot and every exact event/certificate delta
  needed to continue from that basis through the package head;
- a conformant offline interface capable of browsing, local reasoning, private
  drafting, and separate signed-candidate preparation without a live service;
- local verification instructions;
- the SeedPackage manifest.

The package MUST state whether its historical verification basis is genesis or
an externally trusted earlier snapshot. It MUST NOT imply full-history
reconstruction when it contains only snapshot continuation material.

### 3.6 Full Recovery package [anchor: full_recovery_package]

`full_recovery_v0` MUST include all `full_archive_v0` and
`playable_offline_v1` contents plus:

- the last uncontested finalized-prefix certificate and its full predecessor
  certificate chain;
- all availability attestations and omission/equivocation evidence required by
  the active publication profile;
- the current identity, key, eligibility, cycle, Tempo, and active rulebook state
  required to verify ordinary publication eligibility;
- every known valid catastrophe-successor declaration, its supporter envelope,
  recognition/bridge record if any, and preserved competing declaration
  evidence, without implying that any declaration is original canon;
- the conformance fixtures required by the active profiles;
- the complete human-readable authoritative specification set, schemas, and
  version/precedence index governing the included head;
- source code for at least one independently buildable conformant
  verifier/replayer and minimal publication node that can rebuild the included
  basis state and attempt compatible publication without a proprietary service;
- source code for the SeedPackage verifier/builder and Archive Shard
  constructor/reconstructor;
- database schemas, ordered migrations, genesis/import material, and
  configuration templates required by those implementations;
- pinned dependency manifests, all redistributable dependency sources, and an
  offline build toolchain for at least one declared target platform, together
  with its redistributable source or an included source-bootstrap closure;
- at least one non-auto-running verifier/replayer executable for a declared
  target platform, with its source and reproducible build identity;
- an operator recovery guide covering verification, replay, read-only startup,
  ordinary publication resumption, and catastrophe-successor handling;
- no private identity-vault secrets, account secrets, operator credentials, or
  unpublished private Mindseed material.

Additional target executables SHOULD be included for platform diversity, but
source, build closure, conformance data, and human-readable reconstruction
instructions remain mandatory. Executable bytes are non-canonical artifacts and
MUST be independently hash-identified in the manifest.

### 3.7 Catastrophe-successor package labeling [anchor: catastrophe_successor_package_labeling]

A SeedPackage based on a catastrophe-successor lineage MUST:

- use `lineage_mode = 1`;
- include the exact `CatastropheSuccessorDeclaration`;
- include its signed supporter envelope and every artifact committed by its
  `recovery_evidence_root`;
- include the frozen parent prefix certificate and parent snapshot named by that
  declaration;
- expose the successor lineage identifier prominently;
- preserve any known competing successor declarations;
- never label the successor head as an uninterrupted extension of the frozen
  parent canon.

Authority and later recognition are governed exclusively by
`pod-consensus-and-canonical-publication-spec.md`.

## 4. Exact higher-tier selection and explanatory closure [anchor: exact_higher_tier_selection]

### 4.1 Required rulebook parameters [anchor: required_rulebook_parameters]

Before a Tier 1 or Tier 2 bundle can be valid, the active packaging rulebook MUST
define integer values for:

- `citizen_top_ideas_k`
- `citizen_top_truth_claims_k`
- `citizen_reasoning_per_polarity_k`
- `civic_top_ideas_k`
- `civic_top_truth_claims_k`
- `civic_checkpoint_count`
- `cycle_delta_retention_cycles`

Genesis MUST commit initial values. A missing value makes the affected bundle
profile unavailable; implementations MUST NOT invent a local default.
All selection and checkpoint counts MUST be non-negative.
`cycle_delta_retention_cycles` MUST be at least `1`.

### 4.2 Subject ordering [anchor: subject_ordering]

Eligible ideas and truth claims MUST be ordered by ascending replay-derived
`overall_universal_rank`, with their canonical creation position
`(block_height, event_index)` as the first tie-break and canonical encoded
identifier bytes as the final tie-break. The last tie-break is an explicit
ordering rule for this derived package selection only. Bundle selection MUST NOT
use engagement, operator preference, or private curation.

The general-idea pool contains every publicly distributable idea eligible for
the living map at `H`. The truth-claim pool is the subset whose canonical idea
type is a truth-claim subtype. Each pool is ordered independently. A selector
takes the first `min(K, pool_count)` entries. The idea and truth selections are
then unioned by canonical idea identifier; a subject selected in both pools
appears once and receives one explanatory closure. Tier 1 uses the three
`citizen_*` counts. Tier 2 inherits Tier 1 and unions the independently computed
`civic_*` selections.

### 4.3 Citizen explanatory closure [anchor: citizen_explanatory_closure]

For every Tier 1 selected subject, the Citizen Map MUST include:

1. its active title, sentence, and paragraph representations;
2. the active challenge status and latest verdict summary for every direct
   challenge targeting it;
3. up to `citizen_reasoning_per_polarity_k` direct supporting items and the same
   number of direct opposing items, selected by replay-derived universal rank and
   then creation position and the Section 4.2 identifier tie-break;
4. the payloads and provenance commitments required to understand those items;
5. an explanation record identifying the active rulebook parameters, the
   subject's universal-axis positions, aggregate rank derivation, and selection
   boundary that caused inclusion.

This closure is a distribution rule, not an endorsement or rank mutation.
For this selection, a direct supporting or opposing item is an ordinary public
idea connected directly to the subject by an active canonical connection whose
type/usage the active explanation rulebook classifies for that polarity,
including ordinary ideas/connections used as importance arguments. Invalid,
inactive, private, or non-distributable connections are ineligible. Each polarity
is ordered independently by: (1) items attached to an active or unresolved
direct challenge, (2) items attached to the latest finalized direct challenge
that produced the subject's current verdict or rank relationship, and (3) other
eligible direct items. Within each bucket, use the item's universal rank,
then creation position and the Section 4.2 identifier tie-break. An item without
a valid rank sorts after ranked items in its bucket and remains ordered by
creation position and identifier. Fewer than `K` items means all eligible items
are included. Implementations MUST NOT substitute an AI-written summary for the
selected canonical reasoning bytes.

### 4.4 Civic explanatory closure [anchor: civic_explanatory_closure]

For every Tier 2 selected subject, the Civic Archive MUST include all direct:

- supporting and opposing reasoning items;
- challenges, votes, verdicts, and ordinary idea/connection material serving as
  arguments or evidence and permitted for public distribution;
- active and superseded descriptions required to understand the current result;
- provenance, universal-axis positions, selection boundary, and rulebook
  references required to recompute the result.

The closure is finite at a fixed canonical height and MUST be selected solely
from replay-derived public state. Indirect transitive closure is not required
unless an included canonical artifact explicitly depends on the indirect object
for validation.

### 4.5 New and not-yet-ranked material [anchor: new_material_retention]

Cycle Delta Packs MUST be retained for at least
`cycle_delta_retention_cycles`. This creates a bounded evaluation window for new
material without assigning it artificial universal importance. Material without
a valid `overall_universal_rank` is not locally inserted into a top-K selector;
it remains present in Tier 0 when otherwise eligible and in the applicable
Cycle Delta Pack while canonical ranking catches up. PCS, full
archives, and optional non-authoritative local preservation policies MAY provide
additional redundancy. No minority, novelty, language, or random-storage status
MAY alter universal rank.

### 4.6 Optional preservation insurance [anchor: optional_preservation_insurance]

A producer MAY add non-required, clearly labeled preservation-insurance
artifacts for material it believes is unusually vulnerable to loss, including
rare-language representations, unresolved challenges, anomalous claims,
historically rejected ideas, or a reproducible sample. This is optional
availability insurance, not a mandatory core pack and not collective judgment.

If included, the producer MUST list the artifacts as `required = 0`, include a
human-readable policy record naming the selection method and inputs, and commit
that record in the manifest. Presence, absence, or selection under such a policy
MUST NOT affect profile completeness, universal rank, canonical visibility,
governance weight, or the required contents of any Tier 0-3 profile. Any
optional artifact actually listed in a manifest still MUST be present and pass
ordinary length/hash verification.

## 5. SeedPackage v0 container [anchor: seedpackage_v0_container]

### 5.1 Wrapper [anchor: container_wrapper]

The standard transport extension is `.seedpkg` and the media type is
`application/vnd.seed.seedpackage+zip`.

The wrapper is a ZIP or ZIP64 archive. Wrapper bytes, compression choices, entry
order, and ZIP metadata are non-canonical. Package identity is the canonical
SeedPackage manifest hash and the hashes of manifest-listed artifacts.

Implementations MUST reject:

- absolute paths, drive-qualified paths, `..` path segments, backslash path
  separators, NUL bytes, or non-ASCII manifest paths;
- duplicate or case-colliding paths;
- symbolic links, hard links, devices, or other non-regular entries;
- an extracted byte count that differs from the manifest;
- undeclared required artifacts or missing declared artifacts;
- decompression that exceeds locally declared safety limits.

Opening a SeedPackage MUST NOT auto-run executables, start services, open network
connections, or publish data.

### 5.2 Required paths [anchor: required_paths]

Every package MUST contain:

```text
manifest/seedpackage-manifest.bin
snapshots/basis.snapshot-v0
```

Profile-dependent artifacts use these prefixes:

```text
certificates/
history/
packs/
payloads/
rules/
conformance/
runtime/
recovery/
shards/
```

Only paths listed in the manifest affect package verification. Optional
human-readable projections MAY be included but MUST be listed as non-required
artifacts.

### 5.3 Canonical manifest fields [anchor: canonical_manifest_fields]

The SeedPackage manifest fields, in canonical order, are:

1. `manifest_version` (`u32`, MUST equal `0`)
2. `profile_id` (length-prefixed ASCII)
3. `genesis_snapshot_hash` (`hash32`)
4. `basis_block_height` (`u64`)
5. `basis_snapshot_hash` (`hash32`)
6. `shared_map_commitment` (`hash32`)
7. `active_rulebook_set_hash` (`hash32`)
8. `canonical_encoding_profile` (length-prefixed ASCII)
9. `snapshot_format_profile` (length-prefixed ASCII)
10. `lineage_mode` (`u8`: `0` ordinary, `1` catastrophe successor)
11. `catastrophe_successor_declaration_hash` (optional `hash32`; present exactly
    when `lineage_mode = 1`)
12. `finalized_prefix_certificate_hash` (`hash32`)
13. `parent_manifest_hash` (optional `hash32`)
14. `pack_commitment_hashes` (counted list of `hash32`, sorted by raw bytes)
15. `artifacts` (counted list of artifact descriptors sorted by path bytes)

An artifact descriptor is encoded in this field order:

1. `path` (length-prefixed ASCII)
2. `artifact_kind` (length-prefixed ASCII)
3. `byte_length` (`u64`)
4. `artifact_hash` (`hash32`)
5. `required` (`u8`, `0` or `1`)

Version 0 `artifact_kind` is one of:

```text
snapshot_v0
canonical_history
publication_certificate
publication_evidence
payload_pack
payload_blob
rulebook_material
conformance_fixture
specification
source_code
dependency_source
build_toolchain
executable
database_material
configuration
recovery_guide
catastrophe_declaration
recovery_evidence
viewer_asset
search_index
human_projection
```

Required artifacts MUST use a value in this closed list. A non-required
extension MAY use an ASCII kind beginning `x-`; verifiers that do not understand
it retain and hash-check it but MAY otherwise ignore it. A new required kind
requires a new manifest version.

Lengths use fixed-width `u32` big-endian framing. Optional hashes use the
canonical presence byte from the Canonical Encoding and Hashing Specification.
The manifest hash and artifact hashes use the domain tags owned by that
specification.

The pack-hash list MUST be strictly increasing by raw hash bytes, and the
artifact list MUST be strictly increasing by raw ASCII path bytes. Duplicate
hashes, duplicate paths, case-colliding paths, non-minimal or alternate field
encodings, unknown trailing bytes, and out-of-order entries are invalid.

### 5.4 Package verification closure [anchor: package_verification_closure]

A package is valid only when:

1. the manifest bytes decode canonically and its hash verifies;
2. the profile identifier is known and every profile-required artifact exists;
3. every path is safe and unique;
4. every artifact length and hash matches its descriptor;
5. every included pack commitment and payload verifies;
6. the basis snapshot hash and `shared_map_commitment` verify;
7. the included history and certificates satisfy the profile's declared
   historical closure;
8. deterministic replay reaches the basis snapshot for replay-capable profiles;
9. catastrophe-successor labeling and declaration closure verify when present.

Verification failure MUST be explicit. Partial data MAY be opened only in a
clearly labeled incomplete inspection mode and MUST NOT be represented as a
valid complete package.

## 6. Archive Shard Set v0 [anchor: archive_shard_set_v0]

### 6.1 Purpose and authority [anchor: archive_shard_purpose]

Archive shards are derived availability artifacts. They do not grant authority,
change canonical state, or replace whole Full Recovery Bundles. Version 0 uses a
simple independently implementable XOR parity profile; later stronger erasure
profiles require new identifiers.

The set consists of `manifest/archive-shard-manifest.bin` plus the shard paths
defined below. Files MAY travel independently. An optional ZIP/ZIP64 wrapper MAY
use extension `.seedshards` and media type
`application/vnd.seed.archive-shards+zip`; its wrapper metadata is
non-canonical and the Section 5.1 safe-entry rules apply. Shard-set identity is
the canonical shard-manifest hash, not wrapper bytes.

### 6.2 Source stream [anchor: archive_shard_source_stream]

Given a valid `full_recovery_v0` package, construct the source stream as:

```text
u64(manifest_byte_length)
|| manifest_bytes
|| for each artifact descriptor in ascending path order:
     u64(artifact_byte_length) || artifact_bytes
```

Both profile-required and included optional artifacts are preserved. Integers
are unsigned big-endian. The shard manifest records the exact source-stream byte
length before padding.

### 6.3 Data and parity shards [anchor: data_and_parity_shards]

The `xor8_1m_v0` profile is:

- data shard size: `1,048,576` bytes;
- data shards per group: `8`;
- parity shards per group: `1`;
- `group_count = ceil(source_stream_byte_length / (8 * 1,048,576))` and MUST be
  at least `1`;
- every group emits all eight physical data-shard files and one parity-shard
  file;
- the final partial data shard and unused final-group data positions are
  right-padded with zero bytes to full physical shards;
- parity shard byte `j` is the XOR of byte `j` across all eight padded data
  shards in the group.

Any one missing data shard in a group can be reconstructed from the other seven
data shards and the parity shard. Two or more missing shards in the same group
are not recoverable under this profile and MUST be reported honestly.

Each shard is named:

```text
shards/<group_index_10_digit>/<d0..d7|p0>.shard
```

Each shard hash uses the canonical `archive_shard_v0` domain tag.

### 6.4 Shard manifest [anchor: archive_shard_manifest]

The shard manifest fields, in canonical order, are:

1. `shard_manifest_version` (`u32`, MUST equal `0`)
2. `shard_profile_id` (ASCII, MUST equal `xor8_1m_v0`)
3. `source_seedpackage_manifest_hash` (`hash32`)
4. `source_stream_byte_length` (`u64`)
5. `data_shard_size` (`u32`, MUST equal `1048576`)
6. `data_shards_per_group` (`u32`, MUST equal `8`)
7. `parity_shards_per_group` (`u32`, MUST equal `1`)
8. `group_count` (`u32`)
9. `shard_descriptors` (counted list in group/index order)

Each shard descriptor contains `group_index` (`u32`), `shard_kind` (`u8`: data
`0`, parity `1`), `shard_index` (`u8`), `byte_length` (`u32`), and
`shard_hash` (`hash32`). There MUST be exactly nine descriptors per group,
ordered as data indices `0..7` followed by parity index `0`; every descriptor
`byte_length` MUST equal `1048576`. The shard-manifest hash uses the canonical
`archive_shard_manifest_v0` domain tag.

### 6.5 Reconstruction closure [anchor: archive_shard_reconstruction]

A reconstruction is valid only if:

- the shard manifest verifies;
- all available shard hashes verify;
- no group has more than one missing shard;
- reconstructed shard hashes match their descriptors;
- source-stream truncation uses the recorded original byte length;
- the reconstructed SeedPackage manifest and every manifest-listed artifact verify;
- the resulting `full_recovery_v0` package passes Section 5.4.

## 7. Distribution and custody requirements [anchor: distribution_and_custody]

At every finalized snapshot boundary selected by the active packaging rulebook:

- Tier 0 MUST be generated and made trivially hostable;
- Tier 1 SHOULD be generated at the standard Citizen cadence;
- Tier 2 SHOULD be generated at the Civic cadence;
- Tier 3 and Full Recovery MUST exist at the rulebook-defined archival cadence;
- at least one Archive Shard Set MUST be generated for every Full Recovery
  Bundle and placed under independently controlled custody according to the
  active preservation rulebook;
- custody health MUST distinguish verified serving from untested claims.

The ecosystem MUST maintain multiple independently controlled whole or
reconstructible Full Recovery copies. Exact numeric targets and privacy-safe
independence evidence belong to active preservation rulebooks. Storage failure
does not mutate canonical truth, but missing required recovery coverage MUST be
visible as a system health failure.

## 8. Conformance [anchor: conformance]

Implementations claiming this profile MUST pass:

- `docs/conformance/collective-seedpackage-profile-v0.schema.json`;
- `docs/conformance/collective-seedpackage-profile-v0.vectors.json`;
- the applicable snapshot, canonical encoding, publication, replay, and payload
  pack vectors.

At minimum, vectors MUST cover:

- every closed profile identifier;
- safe and unsafe paths;
- missing required artifacts;
- duplicate paths;
- ordinary versus catastrophe-successor labeling;
- incomplete historical closure;
- artifact length/hash mismatch;
- valid XOR parity reconstruction;
- unrecoverable multiple loss in one shard group;
- complete Full Recovery verification.

The existence of fixtures is not evidence that runtime packaging is implemented.

## 9. Explicit limits [anchor: explicit_limits]

This profile cannot reconstruct bytes that are absent from every whole archive
and every recoverable shard set. Hashes prove identity and expose loss; they do
not recreate missing content.

Accordingly:

- a Pocket Map can verify and expose its included state but cannot reconstruct
  omitted history;
- a snapshot-based package can continue from its declared trusted basis but
  cannot prove or recreate omitted pre-basis event bytes by itself;
- only a complete Full Archive, Full Recovery Bundle, or sufficient Archive
  Shard Set can reconstruct the complete included historical record;
- continued human participation cannot be guaranteed if no eligible humans,
  valid keys, or compatible constitutional lineage survives.
