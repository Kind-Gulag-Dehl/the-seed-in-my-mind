---
doc_id: offline_and_mindseed_spec
title: Offline and Mindseed Specification
status: authoritative
version: v0
last_reviewed: 2026-01-27

scope:
  - Defines offline workspaces, mindseed packages, and reintegration rules.

authoritative_for:
  - Offline deliberation semantics.
  - Reintegration and merge constraints for offline work.

not_authoritative_for:
  - Snapshot format details (see snapshot-format-v0.md).

depends_on:
  - protocol v5.md
  - deterministic-replay-and-merge-spec.md
  - cycle-spec.md
  - tempo-spec.md
  - verification-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of canonical-offline-mode.md and full-offline-canonical-mode.md.

reader_path:
  - prereq: verification-spec.md
  - next: canonical-offline-mode.md

keywords:
  - offline
  - mindseed
  - reintegration
  - partitions
---

## 0. purpose and scope [anchor: 0_purpose_and_scope]

### 0.1 purpose [anchor: purpose]

The purpose of this specification is to define how the Protocol v5 system operates when fully or partially disconnected from the network, without altering protocol meaning, semantics, or authority.

This document specifies how individuals and groups may:

- create and evaluate ideas,
- deliberate through challenges, arguments, and votes,
- record local histories and snapshots,
- preserve reasoning structures under censorship or disconnection,
- and later publish those records for deterministic ingestion into the canonical public universe.

Offline operation is treated as **delayed publication**, not as a separate mode of reasoning.  
All offline activity uses the same primitives, lifecycle stages, and constraints defined in Protocol v5.

This specification further defines **Mindseeds / SeedPackages** as portable publication artifacts that allow reasoning to continue offline and later reintegrate without semantic divergence, forked meaning, or hidden authority.

This document does not define new protocol objects, meanings, or judgments.  
Protocol v5 remains the sole source of truth for idea semantics, challenge mechanics, importance logic, governance, safety, and token behavior.

---

### 0.2 scope [anchor: scope]

This specification governs the following areas:

- offline operation by individuals and offline groups,
- the structure and contents of Mindseeds / SeedPackages,
- local event logs and snapshot generation while disconnected,
- offline identity continuity and authorship preservation,
- reintegration of offline-produced events into the canonical network,
- publication packaging for different node storage classes, including:
- full-history archival nodes,
- snapshot-based nodes,
- light or readable-state clients.
- custody and replication obligations for offline survivability (personal custody sets and   collective custody sets),
- human-carried synchronization and exchange of snapshots, deltas, and manifests,
- universal partition mode as a degraded-but-valid offline operating mode with explicit local vs global finality semantics.

This document **does not redefine protocol semantics**.  
It constrains **how Protocol v5 events are produced, stored, packaged, and published** when network connectivity is unavailable or intentionally avoided.

Mindseeds / SeedPackages MAY include **optional non-canonical payloads** (such as packaged user interfaces, reference implementations, documentation, local search indexes, or installation assets) to support offline usability and portability.

Such payloads:

- are **not required** for node conformance,
- are **non-canonical**,
- MUST NOT introduce new semantics,
- MUST NOT affect deterministic replay, event validity, governance activation, token issuance, or canonical IDs,
- MUST be fully ignorable without changing any canonical outcome.

All canonical behavior MUST be determined solely by the canonical payload: Protocol v5 events, snapshots, deltas, anchors, and signatures, under the active rulebooks.

Private drafts are explicitly **out of protocol conformance scope**.

- Draft artifacts are not protocol objects.
- Drafts are not required for node conformance.
- Drafts are not canonical records.
- Drafts MUST NOT be treated as ideas, arguments, evidence, votes, or actions.

Drafts become ideas **only** when emitted as Protocol v5 events and published, either directly to the canonical network or into an offline event log explicitly intended for later canonical publication.

This specification composes with the Shared Map of Reality and Payload Bundle Specification, including Tier 0/1/2 bundle definitions and the shared_map_commitment(H) = HASH("shared_map_commitment_v0" || state_root_hash(H) || pocket_map_payload_root(H)) baseline commitment at block height H.
The authoritative definition of `shared_map_commitment` is in the Canonical Encoding and Hashing Specification (v0); this document repeats it only for readability and does not redefine byte-level hashing, tags, or encodings.
The `HASH(...)` notation and domain tag strings are symbolic references; their concrete byte encoding, hash algorithm, and domain separation behavior are defined exclusively by the Canonical Encoding and Hashing Specification (v0).

This specification also standardizes offline terminology to match Protocol v5:

- **blocks** are deterministic event-log segment boundaries used for integrity and packaging,
- **snapshots** are readable / replay-acceleration state artifacts keyed to block height (and may be tiered),
- **cycle export packs** are optional, non-authoritative, fully regenerable browsing bundles intended for offline usability.

Mindseeds / SeedPackages MAY include snapshots and/or cycle export packs, but:
- snapshots are authoritative only insofar as they are verifiable by deterministic replay from the canonical log,
- export packs are always non-authoritative, ignorable, and MUST NOT affect replay, validity, or governance.

---

## 1. core invariants (offline == online semantics) [anchor: 1_core_invariants_offline_online_semantics]

### 1.1 semantic equivalence [anchor: semantic_equivalence]

Offline operation MUST be semantically equivalent to online operation.

An offline-capable system MUST support the full canonical lifecycle defined in Protocol v5, including:

* idea creation,
* connection creation,
* challenge creation,
* vote casting,
* verdict finalization,
* snapshot generation,
* cycle derivation and rate-limit enforcement as defined by Protocol v5 §3.

Offline systems MAY delay publication and canonical ingestion, but MUST NOT alter:

* idea types,
* connection types,
* challenge domains,
* challenge lifecycle rules (including cycle-bounded windows),
* voting rules,
* verdict logic,
* importance semantics,
* governance semantics,
* safety semantics,
* cycle semantics or rate-limit semantics.

No offline-only meanings, shortcuts, exceptions, or alternative rule sets may exist.

Any Protocol v5 event that is valid online MUST be representable offline, and any event produced offline MUST be evaluated under the same rules when reintegrated.

References to “arguments” in offline operation refer to the Protocol v5 model in which reasons, evidence, and deliberative content are represented as ideas and connections, not as a separate protocol object type.

Offline conformance MUST treat governance activation and canonical anchoring exactly as Protocol v5 defines them (i.e., cycle-based activation derived from canonical governance verdict metadata). Optional cycle export packs MUST NOT be used as activation anchors, resume points, or inputs to deterministic replay.
#### Clarification: decision cadence and activation boundary [anchor: clarification_decision_cadence_and_activation_boundary]

Offline publication does not create alternate governance activation timelines.

- Governance `decision_event` confirmation remains cycle-end based.
- Activation remains cycle-boundary based and is scheduled by deterministic replay from `decision_cycle_index + delay_policy(change_class)` under `delay_policy_version`.
- The authoritative activation boundary is `activation_cycle_index` (start of cycle, inclusive).
- Workspace snapshots, offline cycle markers, and export packs remain non-authoritative and MUST NOT define alternate activation boundaries.

---

### 1.2 public-on-reintegration invariant [anchor: public_on_reintegration_invariant]

Any event reintegrated into the canonical universe MUST be publicly viewable in principle.

Visibility is governed exclusively by:

- canonical safety abstraction rules,
- jurisdictional or regulatory visibility lenses.

Offline privacy exists **only prior to publication**.

Public attribution after publication remains minimal by default: a stable pseudonymous author identity plus a non-identifying verification level. Civil identity disclosure is never required for publication or reintegration.

Once an event is published into the canonical universe:

- it is subject to universal challengeability,
- it participates in public reasoning,
- it is visible as existing even if its payload is abstracted or blocked.

Draft-only content, including private notes, speculative sketches, local prompts, and non-event artifacts:

- is not part of protocol semantics,
- MUST NOT be treated as an idea or canonical record,
- MUST NOT appear in snapshots or SeedPackages as protocol objects,
- remains entirely outside the protocol until explicitly emitted as a Protocol v5 event.

---

### 1.3 no private tribes [anchor: no_private_tribes]

Tribes do NOT have private ideas, private maps, or private snapshots.

All tribe-associated ideas are ordinary public ideas.
All tribe importance maps are public overlays constructed from public ideas using tribe-scoped judgments.

Offline operation MUST follow the same rule.

Offline mode MUST NOT introduce:

- private tribe ideas,
- hidden tribe-only snapshots,
- tribe-restricted canonical visibility.

Tribes may control voting eligibility and mutation rights for tribe-scoped rankings, but they do not control idea visibility, canonical challengeability, or public existence.

Offline systems MUST preserve this invariant exactly.



## 2. offline execution model [anchor: 2_offline_execution_model]

### 2.1 offline workspace [anchor: offline_workspace]

An **offline workspace** is a local execution environment in which one or more humans create, evaluate, and record Protocol v5 events without network connectivity.

An offline workspace MUST have the following properties:

- an **append-only local event log** containing Protocol v5 events intended for later canonical publication,
- **deterministic local replay** using the same semantics and ordering rules as canonical replay,
- **local cycle derivation** using the same cycle rules defined in Protocol v5 §3,
- **local rate-limit and mana enforcement** using the same per-identity caps and rules defined in Protocol v5 §3,
- **local snapshot generation** to support performance, inspection, and recovery,
- the ability to be **shared or transferred** physically or digitally among trusted participants (e.g., files, removable media, direct device transfer).
- the ability to generate and exchange custody manifests and state witness artifacts during P2P meetings (as defined by the verification specification).

An offline workspace represents a **potential future publication stream**, not a fork of the canonical universe.

Local state derived within an offline workspace is non-authoritative and remains inert until its events are published and ingested by canonical nodes.

Offline operation MUST NOT be used to obtain additional canonical throughput. Offline execution speed MUST NOT create additional valid canonical actions beyond what the same identities could have produced under the Protocol v5 cycle and rate-limit rules.

Offline systems MAY compute provisional local cycles for user experience, local gating, and planning purposes; however, such cycles are strictly non-authoritative and MUST NOT be treated as canonical time.

During reintegration, canonical cycle derivation and canonical per-identity limits determine which published events are immediately effective. Any excess events remain preserved in the canonical log but non-effective until later canonical cycles make them effective, as defined in §3.5.




### 2.2 individual vs group workspaces [anchor: individual_vs_group_workspaces]

Offline workspaces MAY be operated by a single individual or by multiple humans collaborating together.

An **individual workspace**:

- contains events authored by a single human identity,
- preserves authorship continuity for that identity,
- may later publish events independently.

A **group workspace**:

- contains events authored by multiple human identities,
- allows independent idea creation, challenges, arguments, and votes by each participant,
- preserves per-event authorship exactly as in the online system.

Group workspaces do NOT imply collective authorship.

- Every event retains a single speaker identity.
- No event is jointly authored.
- Group deliberation emerges from interaction between identities, not from shared authorship objects.

Offline group operation MUST NOT introduce:

- collective identities,
- shared-author events,
- alternative authorship semantics.

All authorship rules defined in Protocol v5 apply identically offline.

In group workspaces, all per-identity rate limits, mana pools, earning caps, and action costs MUST remain strictly per-identity.

A group workspace MUST NOT allow participants to:
- pool mana,
- bypass per-identity caps,
- or convert group coordination into higher per-identity throughput.
---

## 3. event log and replay (offline) [anchor: 3_event_log_and_replay_offline]

### 3.1 local event log [anchor: local_event_log]

Offline systems MUST maintain an append-only **local event log** composed exclusively of valid Protocol v5 event types that are intended for later canonical publication.

Each event in the local event log MUST include:

- a `client_event_id` that uniquely identifies the event within the local workspace,
- a `speaker_identity`, which MAY be:
  - a verified human identity,
  - a pseudonymous identity,
  - an anonymous placeholder identity suitable for non-canonical publication or later canonical adoption by a verified-human identity,
- an event payload identical in structure and semantics to the corresponding online Protocol v5 event,
- a locally recorded wall-clock timestamp for user-interface and evidentiary purposes only.

Local timestamps are **non-authoritative** and MUST NOT be treated as canonical ordering signals during reintegration.

Offline systems MAY store additional non-canonical artifacts (including drafts, annotations, prompts, UI state, or speculative actions) for user-experience purposes.

Such non-canonical artifacts:
- MUST be stored outside the protocol event log,
- MUST NOT be interpreted as protocol events,
- MUST NOT appear in snapshots or SeedPackages as canonical objects,
- MUST NOT be published unless explicitly emitted as Protocol v5 events.

Offline systems MAY store non-authoritative canonical anchor references for verification and portability, including:
- a basis snapshot identifier keyed to block height,
- an optional snapshot tier marker,
- the active rulebook set identifier or hash at the basis point,
- hashes or integrity proofs for any included canonical payload blobs.

All such references are advisory metadata only:
- they MUST NOT introduce new semantics,
- they MUST NOT affect event meaning, ordering, validity, eligibility, or governance activation.

Offline systems MUST NOT discard, rewrite, or retroactively modify events once recorded in the local event log.


### 3.2 deterministic local replay [anchor: deterministic_local_replay]

Offline nodes MUST support deterministic replay of their local event log.

Deterministic local replay MUST allow a node to:

- reconstruct the complete local state from the local event log,
- evaluate challenges, arguments, votes, and verdicts using Protocol v5 semantics,
- generate identical local snapshots on different devices that share the same event log and rulebook set.

Local replay MUST use the same deterministic rules as canonical replay, including:

- event ordering logic,
- verdict aggregation rules,
- importance computation logic,
- snapshot derivation rules.

Determinism is required so that:

- offline collaboration remains coherent,
- verification is possible prior to publication,
- reintegration does not introduce semantic drift.

---

### 3.3 local invalidity handling [anchor: local_invalidity_handling]

Offline systems MUST preserve all locally produced events, including events that are invalid under Protocol v5 semantics.

Offline systems MUST classify locally produced events into at least two non-authoritative categories:

1. **structurally/semantically invalid (inert)**
   - violates schema, signatures, or Protocol v5 rulebooks as evaluated locally,
   - MUST be retained in the local log,
   - MUST be marked as non-effective for local state derivation.

2. **valid-but-not-yet-effective (deferred)**
   - structurally valid Protocol v5 events that would exceed locally derived per-identity limits, mana availability, or other pacing constraints at the current local cycle index,
   - MUST be retained in the local log,
   - MUST be marked as non-effective for local state derivation unless and until it becomes effective under locally derived cycles and mana,
   - MUST remain publishable (subject to reintegration semantics in §3.5).

Local invalidity or deferral does NOT imply erasure, censorship, or suppression.

During canonical reintegration:
- the receiving canonical node re-evaluates all published events under the active rulebooks and canonical cycle derivation,
- locally invalid events MAY become valid, remain invalid, or be rejected,
- deferred events MAY become effective immediately or remain deferred depending on canonical cycles and per-identity limits,
- canonical evaluation always takes precedence over local evaluation.

Offline systems MUST NOT discard or rewrite history in response to local invalidity or deferral.



### 3.4 offline pacing, cycles, and rate limits (normative) [anchor: offline_pacing_cycles_and_rate_limits_normative]

Offline operation MUST preserve the canonical pacing and rate-limit properties of Protocol v5 §3.

An offline workspace:

- MUST derive cycle boundaries using the Protocol v5 cycle rules from its local event log,
- MUST enforce per-identity action limits and mana rules using the same rules as a canonical node,
- MUST reject or prevent creation of Protocol v5 events that would be invalid under Protocol v5 §3 at the current derived local cycle index.

Offline systems MUST NOT permit “cycle grinding” or “cycle churn” to function as a free reset of action limits.

In particular:

- Cycle advancement MUST NOT increase per-cycle allowances except as permitted by the Protocol v5 scaling rules.
- Mana pools MUST persist across cycles and MUST respect maximum caps exactly as defined in Protocol v5.
- Challenge creation MUST remain gated by deliberation mana earned via voting, subject to per-cycle earning caps and persistence rules.

Offline-derived cycles exist only to enforce pacing constraints while disconnected.
They MUST NOT be interpreted as canonical time and MUST NOT be used to claim entitlement to canonical actions.

Universal Partition Mode (§7C) MAY define additional offline procedures (e.g., cycle boundary claims and quorum receipts) to coordinate groups during full disconnection; however, such procedures MUST NOT create entitlement to canonical actions and MUST remain subject to deterministic validation upon publication.

Offline systems MAY support drafting, simulation, or speculative actions that exceed local pacing limits for user-experience purposes; however:
- such actions MUST remain drafts or non-event artifacts,
- such actions MUST NOT be emitted into the local event log as Protocol v5 events,
- such actions MUST NOT be included in publishable SeedPackages.

Any Protocol v5 event recorded in the local event log MUST be structurally valid and locally permitted at the time of recording.

Canonical pacing, effectiveness, and eligibility are enforced only during canonical ingestion and deterministic replay.

### 3.5 deferred effectiveness on reintegration (normative) [anchor: deferred_effectiveness_on_reintegration_normative]

Offline publication is delayed submission into the canonical event log, not parallel progression.

When offline-produced Protocol v5 events are ingested into the canonical universe, canonical replay MUST enforce canonical cycles and per-identity pacing exactly as if those identities had authored the events online.

Accordingly:

- If an ingested event is valid and permitted under canonical pacing at its replay position, it becomes **effective** normally.
- If an ingested event is structurally valid but would exceed canonical per-identity limits, mana availability, or other pacing constraints at that point in replay, the event MUST remain in the canonical log but MUST be treated as **non-effective (deferred)**.
- Deferred events MUST become effective automatically once deterministic replay reaches a later point where canonical cycles and per-identity pacing make them permitted, without rewriting, reordering, or selectively dropping history.

Deferred effectiveness MUST be:
- deterministic,
- auditable,
- and identical across all conformant nodes given the same canonical log and rulebooks.

This rule guarantees that offline operation cannot:
- accelerate canonical cycles,
- exceed per-identity throughput,
- or retroactively alter canonical pacing.



## 4. identity and authorship (offline) [anchor: 4_identity_and_authorship_offline]

### 4.1 identity continuity [anchor: identity_continuity]

Offline systems MUST support continuity of identity across time, devices, and disconnection.

Offline identity support MUST include the ability to record:

- key continuity claims,
- key rotation claims,
- device loss or compromise recovery claims.

These records are **claims**, not authority grants.

- Recording a continuity, rotation, or recovery claim does not itself confer legitimacy.
- All such claims are subject to challenge and verification after publication.
- Canonical acceptance of identity continuity is determined exclusively through Protocol v5 mechanisms.

Offline systems MUST preserve identity continuity claims in the local event log exactly as authored, without local elevation to authority.

Mindseed implementations MUST support a distinct **identity vault** capability for preserving identity continuity across devices, jurisdictions, and offline operation.

The Mindseed identity vault:
- MAY store private credentials, proofs, recovery materials, and other identity-related secrets,
- MUST store such materials encrypted locally,
- MUST NOT publish raw PII or raw credential contents into the canonical event log.

Public attribution MUST remain minimal: offline publication uses the pseudonymous author identity and (optionally) a non-identifying verification level, never raw credentials or civil identity.

Canonical publication of identity-related facts MUST use:
- attestations,
- commitments,
- references to issuers or verification processes,
- and other non-PII artifacts that are replay-verifiable,
rather than raw identity documents or raw credential payloads.

---

### 4.2 multi-author workspaces [anchor: multi_author_workspaces]

Each Protocol v5 event authored offline MUST name **exactly one** `speaker_identity`.

Offline group workspaces MAY contain events authored by multiple identities, but:

- no event may have more than one speaker,
- no event may be authored “on behalf of” another human,
- no collective or proxy authorship is permitted.

All authorship rules defined in Protocol v5 apply identically offline.

Group deliberation arises through interaction between independently authored events, not through shared or delegated authorship.

---

### 4.3 anonymous and pseudonymous authorship [anchor: anonymous_and_pseudonymous_authorship]

Offline users MAY create local pre-publication events anonymously or under a pseudonymous identity.

Upon publication into the canonical universe, canonical authorship rules apply: canonical events MUST be signed by a verified-human identity, and anonymously authored material remains in the outer layer until explicitly adopted by a verified-human identity.
See Privacy and High-Risk Submission Spec §6 (Anonymous Outer-Layer Semantics) and §7.4 (Store-and-Forward / Offline Publication Profile).

When such events are reintegrated into the canonical universe:

- presentation MAY remain anonymous or pseudonymous, but canonical authorship remains the adopting verified-human identity where canonical publication occurs,
- no hidden or implied real-world identity is assumed,
- optional persona attachments (if any) are opt-in and presentation-only.

Anonymous or pseudonymous events MAY later be linked to a verified identity through **explicit, challengeable identity-linking claims**.

Identity linkage:

- is optional,
- is never retroactive authority,
- remains fully challengeable under normal Protocol v5 rules.

Offline systems MUST NOT require identity disclosure as a condition of authorship or later publication.

---

## 5. ideas, scopes, and visibility (offline) [anchor: 5_ideas_scopes_and_visibility_offline]

### 5.1 ideas [anchor: ideas]

All ideas and rails in Protocol v5 are canonical public objects by default.

Offline systems MAY create ideas locally and defer their publication, but:

- an idea becomes a protocol object only when expressed as a Protocol v5 event intended for publication,
- unpublished ideas remain local and non-canonical,
- private drafts are not ideas and are not protocol objects.

A draft becomes an idea **only** when it is emitted as a Protocol v5 event, even if that event is stored offline and published later.

Offline systems MUST clearly distinguish drafts from protocol events.

---

### 5.2 personal scope [anchor: personal_scope]

Individuals MAY maintain **personal-scope maps** offline.

Personal-scope judgments:

- reference public canonical ideas,
- use the same challenge, voting, and importance mechanics defined in Protocol v5,
- are anchored to a single owning identity,
- are subject to the same cycle-based pacing and rate-limit rules defined in Protocol v5 §3.

Personal-scope judgments MAY be published as canonical personal-scope events if allowed by Protocol v5 rulebooks.

Personal scope MUST support the following visibility modes:

1. **private personal map (default)**  
   Visible only to the owning identity.  
   No other identity may view, interact with, or mutate the map.

2. **public-readable personal map**  
   The map and its contents are publicly viewable.  
   Only the owning identity may:
   - create challenges,
   - vote,
   - modify rank lists within the personal scope.

3. **fully public personal map (optional)**  
   The owner MAY additionally allow public challenge participation against objects shown in the map, subject to applicable rulebooks.  
   Voting and rank mutation remain owner-controlled unless a rulebook explicitly defines a different eligibility model for personal scope.

In all cases:

- the underlying ideas referenced by a personal map are ordinary public ideas,
- those ideas remain universally challengeable in universal scope under normal Protocol v5 rules,
- personal scope never shields ideas from universal scrutiny.

Offline operation MUST preserve these distinctions exactly.

When offline personal-scope content is published into the canonical universe, clients MUST preserve canonical explainability requirements.

In particular, when any personal-scope object is excluded or altered in default views, clients MUST be able to present “why am I seeing this?” diagnostics that distinguish:
- safety-based abstraction or redaction,
- jurisdictional or regulatory visibility lenses,
- derived exclusions (e.g., lifecycle_state exclusion such as rotted/burned, or taint-derived exclusions),
- and user-applied filters.

Offline packaging MAY include private personal drafts and local-only annotations, but once a personal-scope event is published canonically, it is subject to the same explainability, safety, and lens semantics as any other canonical event.


### 5.3 tribe scope (public overlay) [anchor: tribe_scope_public_overlay]

Tribe-associated ideas are ordinary public ideas.

Tribe maps are projections derived from tribe-scoped judgments applied to public ideas.

Offline systems MAY create tribe-scoped challenges, arguments, votes, and verdicts locally, subject to later publication.

At no stage does tribe scope introduce private visibility.

- There are no tribe-private ideas.
- There are no tribe-private snapshots.
- There are no tribe-restricted canonical views.

Offline operation MUST preserve tribe scope as a **public overlay** exactly as defined in Protocol v5.



## 6. challenges, voting, and tribe maps (offline) [anchor: 6_challenges_voting_and_tribe_maps_offline]

### 6.1 challenge scopes [anchor: challenge_scopes]

Challenges MAY be created offline with an explicit scope.

Supported scopes are identical to those defined in Protocol v5:

- **universal scope**
- **tribe scope**
- **personal scope**

Challenge scope determines **eligibility and mutation rights**, not the existence or visibility of the underlying ideas.

All ideas referenced by a challenge are ordinary public ideas once published, regardless of challenge scope.

Challenge lifecycle progression (argument windows, voting windows, and finalization) MUST be evaluated using **cycle indices** as defined in Protocol v5 §3, including for challenges created offline.

In Universal Partition Mode (§7C), offline groups MAY proceed with universal-scope challenge activity under degraded eligibility and quorum constraints; any locally-finalized outcomes remain provisional until publication, and canonical ingestion MAY mark some events non-effective if eligibility, selection, or pacing constraints cannot be validated against the receiving canonical node’s authoritative state.

Visibility defaults upon publication are as follows:

- **universal scope:** publicly visible
- **tribe scope:** publicly visible (with voting restricted to eligible tribe members)
- **personal scope:** visibility determined by the personal map’s visibility mode:
  - private,
  - public-readable,
  - optional fully public.

Personal-scope mutation rights:

- MUST remain restricted to the owning identity,
- MUST NOT be transferred implicitly through visibility changes,
- MAY only be modified if a future rulebook explicitly defines a delegate mechanism.

Offline systems MUST preserve these scope semantics and lifecycle rules exactly.


### 6.2 tribe-scoped challenges [anchor: tribe_scoped_challenges]

Tribe-scoped challenges are governed by tribe membership.

For tribe-scoped challenges:

- only identities that are members of the tribe MAY vote,
- all arguments are publicly visible,
- all votes are publicly visible,
- eligibility rosters MUST be recorded as part of the challenge state to enable deterministic replay and later verification.

Offline systems MUST record sufficient membership and eligibility information to allow a canonical node to independently verify that voting eligibility rules were followed at the time of authorship.

Any tribe-scoped challenge that restricts participation to a membership set MUST record eligibility in a replay-verifiable manner independent of wall-clock time.

Tribe eligibility rosters MUST be reconstructable from canonical events and applicable rulebooks at deterministic boundaries (including boundaries keyed to snapshots at block height), so that any node can validate:
- who was eligible to participate,
- under which rulebook version,
- and at which canonical point in replay.

Offline or local roster representations MAY be used for UX, but canonical verification MUST rely only on replay-verifiable data derived from the canonical log.

---

### 6.3 public interaction with tribe ideas [anchor: public_interaction_with_tribe_ideas]

Tribe membership restricts **voting eligibility**, not participation in public reasoning.

Accordingly:

- non-members MAY create **universal-scope challenges** involving ideas that appear in tribe maps,
- non-members MAY submit arguments to tribe-scoped challenges, subject to applicable tribe rulebooks,
- non-members MUST NOT vote in tribe-scoped challenges unless explicitly permitted by a rulebook.

Offline systems MUST support these interaction patterns and MUST NOT introduce additional access restrictions.

---

### 6.4 tribe maps [anchor: tribe_maps]

A **tribe map** is a deterministic projection defined by:

- a public set of ideas,
- tribe-scoped verdict edges produced through tribe-scoped challenges,
- an optional tribe lens configuration, if such a lens is defined by rulebooks.

Tribe maps:

- do not duplicate ideas,
- do not create tribe-private objects,
- do not alter universal idea identity.

Offline systems MUST construct tribe maps using the same deterministic projection rules as online systems.

---

## 7. snapshots (offline) [anchor: 7_snapshots_offline]

### 7.1 local snapshots [anchor: local_snapshots]

Offline systems MAY generate **local snapshots** to accelerate replay and support usability.

Local snapshots are **non-canonical workspace artifacts**.

Local snapshots:

- MUST NOT be treated as authoritative canonical snapshots,
- MAY include private drafts, local caches, and derived state,
- are invalid as canonical snapshots unless their contents and hash exactly match a canonical snapshot boundary produced by a conformant canonical node.

Offline systems MAY produce snapshots using multiple **retention profiles** aligned to node storage classes:

- **archival profile**  
  Retains sufficient material to reconstruct full deterministic history locally.

- **snapshot-based profile**  
  Retains snapshots and deltas sufficient to reconstruct current state from a trusted basis snapshot forward.

- **light profile**  
  Retains only a selected subset (e.g., readable state snapshots, anchors, and user-selected material), optimized for portability and dissemination.

Retention profiles affect **storage and distribution only**.

They MUST NOT introduce offline-only semantics, alter protocol meaning, or change replay rules.

Offline systems MAY create local snapshots for workspace convenience, storage management, or recovery.

Local snapshots MUST be distinguished from canonical snapshots defined in Protocol v5:
- **canonical snapshots** are tiered, block-height keyed artifacts derived from the canonical event log and include all required derived state,
- **workspace (local) snapshots** are non-authoritative artifacts created for offline usability.

Any snapshot intended to be comparable to a canonical snapshot (for verification or reintegration purposes) MUST include all derived fields required by Protocol v5, including `lifecycle_state` and any other deterministically derived eligibility fields.

Local snapshots MUST NOT be treated as canonical unless they can be verified by deterministic replay from the canonical log.

---

### 7.2 workspace snapshots [anchor: workspace_snapshots]

A **workspace snapshot** captures the local execution state of an offline workspace at a point in time.

A workspace snapshot MAY include:
- the local event log head,
- the derived local replay state,
- local caches, indexes, or auxiliary data used for performance or user experience.

Workspace snapshots are strictly local artifacts.

Workspace snapshots SHOULD be constructible in custody-aligned profiles (PCS/CCS and archival tiers) to support human-carried synchronization (§7B) and durability targets (§7A).

Workspace snapshots:
- MUST NOT override canonical state,
- MUST NOT be published as canonical snapshots,
- MUST NOT be mistaken for canonical snapshot tiers,
- MUST NOT be used as activation boundaries,
- MUST NOT affect governance, pacing, eligibility, or replay semantics,
- MAY be discarded, regenerated, or replaced without affecting protocol semantics.

Offline systems MUST ensure that workspace snapshots never supersede, masquerade as, or are interpreted as canonical snapshots.

A workspace snapshot MAY reference a canonical snapshot commitment (for example, a block-height-keyed snapshot hash or tier marker) as an advisory anchor for verification or navigation; however, such references do not confer authority and MUST NOT replace deterministic replay from the canonical event log.


# **7A. Custody and Replication**

## 7A.1 Purpose and scope [anchor: 7a_1_purpose_and_scope]

Custody and replication define how canonical material is **physically preserved, distributed, and redundantly stored** when the system operates offline or under partial connectivity.

This section governs:

* who is responsible for storing which parts of the canonical payload,
* how redundancy targets are defined and monitored,
* how under-replication is detected and addressed,
* how human agents (including Entlings) assist replication.

Custody affects **durability only**.
It does **not** affect authorship, authority, eligibility, voting power, or semantics.

---

## 7A.2 Canonical Storage Classes [anchor: 7a_2_canonical_storage_classes]

Canonical payload is divided into two storage responsibility classes:

### 7A.2.1 Personal Custody Set (PCS) [anchor: 7a_2_1_personal_custody_set_pcs]

A **Personal Custody Set (PCS)** is the subset of canonical ideas, rails, connections, challenges, and histories that an identity is primarily responsible for storing.

An identity’s PCS MUST include:

* all ideas (and associated rails, where applicable) that the identity ranks above a configurable personal-importance threshold,
* all challenges, votes, and arguments authored by the identity,
* all downstream structures required to replay those items deterministically.

PCS selection rules:

* are user-controlled,
* are transparent and auditable,
* MAY evolve over time through personal importance judgments.

PCS responsibility does **not** imply ownership, authority, or control—only custodial duty.

---

### 7A.2.2 Collective Custody Set (CCS) [anchor: 7a_2_2_collective_custody_set_ccs]

A **Collective Custody Set (CCS)** consists of ideas, rails, and histories deemed highly important to the collective.

The CCS MUST include:

* ideas above a defined universal importance threshold,
* governance histories and rulebooks,
* global cycle boundary claims and anchors,
* safety and eligibility rule histories.

CCS membership:

* is determined exclusively by canonical importance logic,
* is identical for all conformant nodes at a given replay state,
* is challengeable through ordinary Protocol v5 mechanisms.

---

## 7A.3 Custody Tiers [anchor: 7a_3_custody_tiers]

Custody responsibilities are expressed in tiers:

1. **Primary custodians**
   Identities for whom an item is in their PCS or CCS.

2. **Secondary custodians**
   Identities who replicate items opportunistically.

3. **Archival custodians**
   Nodes or identities that retain full-history replicas.

The protocol does **not** require all identities to be archival custodians.

---

## 7A.4 Redundancy Targets [anchor: 7a_4_redundancy_targets]

Each canonical object SHOULD meet minimum redundancy targets:

* PCS items: replicated across multiple socially uncorrelated custodians,
* CCS items: replicated broadly across the population,
* governance and safety history: high redundancy.

Redundancy targets:

* are advisory but strongly recommended,
* MAY be adjusted by governance rulebooks,
* MUST be computed deterministically from canonical state.

---

## 7A.5 Under-Replication Detection [anchor: 7a_5_under_replication_detection]

Nodes MAY locally detect under-replication by comparing:

* known custody manifests,
* observed availability,
* expected redundancy targets.

Under-replication signals:

* MUST NOT invalidate canonical state,
* MUST NOT block publication or replay,
* MAY trigger replication assistance workflows.

---

## 7A.6 Entling-Assisted Replication [anchor: 7a_6_entling_assisted_replication]

Entlings MAY be tasked with:

* identifying under-replicated CCS material,
* requesting replication from custodians,
* assisting with packaging and transfer.

Entlings:

* have no authority to modify canonical content,
* act only as facilitators,
* operate under auditable task logs.

---

# **7B. Human-Carried Synchronization**

## 7B.1 Purpose [anchor: 7b_1_purpose]

Human-carried synchronization defines how canonical data is exchanged **through direct human interaction**, without reliance on network infrastructure.

This includes:

* in-person meetings,
* physical media exchange,
* local device-to-device transfer.

---

## 7B.2 Exchange Artifacts [anchor: 7b_2_exchange_artifacts]

During a human-carried sync, participants MAY exchange:

1. **Custody manifests**
   Lists of canonical objects held.

2. **Pack headers**
   Snapshot or delta identifiers and hashes.

3. **Event deltas**
   Missing canonical events.

4. **Witness attestations**
   Signed claims of observed state.

No raw authority is transferred by exchange alone.

---

## 7B.3 Priority Rules [anchor: 7b_3_priority_rules]

When bandwidth or time is limited, exchanges MUST prioritize:

1. PCS obligations,
2. CCS obligations,
3. Opportunistic replication.

This ordering ensures both personal continuity and collective durability.

---

## 7B.4 Verification During Exchange [anchor: 7b_4_verification_during_exchange]

Participants MAY verify:

* event log continuity,
* snapshot hash agreement,
* signature validity.

Disagreements do not halt operation; they are recorded for later resolution.

---

## 7B.5 Failure and Partial Exchange [anchor: 7b_5_failure_and_partial_exchange]

Partial exchanges:

* MUST be safe,
* MUST be resumable,
* MUST NOT corrupt local state.

Incomplete syncs are expected and normal.

---

# **7C. Universal Partition Mode**

## 7C.1 Definition [anchor: 7c_1_definition]

**Universal Partition Mode (UPM)** applies when:

* no globally reachable online nodes exist,
* the system operates entirely through offline and P2P interaction.

UPM is a **degraded but valid** operating mode.

---

## 7C.2 Canonical Activity Under Partition [anchor: 7c_2_canonical_activity_under_partition]

Under UPM:

* events MAY continue to be authored,
* challenges MAY progress locally,
* votes MAY be cast with limited pools,
* cycles advance approximately using agreed time bounds.
* P2P exchange MUST NOT require civil identity disclosure; default attribution remains the pseudonymous author identity plus a non-identifying verification level.

All outcomes remain **provisional** until later convergence.

---

## 7C.3 Independence-Quorum Voting [anchor: 7c_3_independence_quorum_voting]

Certain canonical actions MAY require votes from identities that:

* exceed a verification threshold,
* are not socially adjacent to the proposer.

Eligibility is verified later through replay and identity attestations.

---

## 7C.4 Double Voting and Conflict [anchor: 7c_4_double_voting_and_conflict]

Double voting MAY occur during partition.

Resolution rules:

* detected during reintegration,
* invalidate excess votes,
* MAY penalize identities under governance rules.

No retroactive rewriting occurs.

---

## 7C.5 Cycle Boundary Claims [anchor: 7c_5_cycle_boundary_claims]

During partition:

* participants MAY emit cycle boundary claims,
* claims are later clustered and evaluated,
* quorum receipts establish approximate alignment.

Cycle boundaries are reconciled deterministically during merge.

---

## 7C.6 Local vs Global Finality [anchor: 7c_6_local_vs_global_finality]

Partition-local finality:

* enables continued operation,
* does not grant global authority.

Offline events may accumulate in authored state and, where the active publication profile supports it, MAY also accumulate availability attestations. Neither state grants global canonical order.

Conflicts escalate to governance challenges upon reintegration.

---

## 7C.7 Merge Escalation [anchor: 7c_7_merge_escalation]

When partitions reconnect:

* divergent histories are published,
* conflicts surface explicitly,
* resolution proceeds through normal challenge mechanisms.

There is no silent reconciliation.


## 8. safety and visibility (offline) [anchor: 8_safety_and_visibility_offline]

### 8.1 local safety handling [anchor: local_safety_handling]

Offline clients MAY apply local safety filters for usability, legal compliance, or user protection while operating without network connectivity.

Local safety handling:

- MUST NOT delete protocol events,
- MUST NOT alter event payloads,
- MUST NOT rewrite, redact, or mutate canonical content,
- MUST NOT determine canonical validity or effect.

Local safety filters MAY:

- hide or abstract payloads in the user interface,
- restrict interaction with certain content locally,
- annotate content with local warnings or notices.

All local safety handling is **non-authoritative**.

Offline systems MUST preserve the original event payloads exactly as authored so that canonical safety classifiers can be applied correctly upon publication.

---

### 8.2 reintegration visibility [anchor: reintegration_visibility]

Upon publication and canonical ingestion of offline-originated events:

- canonical safety classifiers apply,
- jurisdictional and regulatory visibility lenses are enforced,
- abstraction, blocking, or payload redaction is performed strictly according to Protocol v5 rules.

When visibility is gated or modified:

- the existence of the event MUST remain visible,
- relationships involving the event MUST remain visible,
- explanatory metadata (e.g., “why am I seeing this?” or “why is this hidden?”) MUST be generated in accordance with protocol rules.

Offline systems MUST NOT suppress or pre-empt these canonical visibility explanations.

User-facing reintegration views MUST provide explainability not only for safety-based abstraction, but also for derived exclusions.

When content is hidden, excluded, or deprioritized during reintegration or replay, clients MUST be able to explain whether this is due to:
- safety abstraction or redaction,
- jurisdictional or regulatory visibility lenses,
- lifecycle_state exclusion (e.g., rotted or burned),
- taint-derived exclusion or correction,
- or user-applied filters.

Derived exclusions MUST be distinguishable from safety redactions in user-facing explanations.

---

## 9. reintegration and publication [anchor: 9_reintegration_and_publication]

### 9.1 publication model [anchor: publication_model]

Reintegration is defined as **publication** of a selected subset of an offline event log to the canonical network.

Reintegration is NOT consensus merging.
There is no fork resolution, averaging, reconciliation of state, or negotiation of outcomes.

Offline partitions MAY independently publish overlapping or conflicting event histories. Canonical nodes MUST ingest each publication deterministically; conflicts are preserved as explicit competing claims and MUST be resolved through ordinary challenge mechanisms rather than by any reintegration-time merge negotiation.

Publication consists solely of submitting authored Protocol v5 events for canonical evaluation under the currently active rulebooks.

Offline-produced events may therefore exist in three distinct states during reintegration:

- `authored` while only the signed event bytes exist,
- `availability-certified` once the active publication profile’s witness threshold is met,
- `canonical` only after inclusion in a finalized prefix certificate.

Offline systems MUST support multiple **publication packaging modes** to serve different node storage classes:

- **full-history publication packs**  
  Intended for archival nodes.  
  Contain complete deterministic event history sufficient for full replay.

- **snapshot + delta publication packs**  
  Intended for snapshot-based nodes.  
  Contain subsequent events after a trusted basis. Offline packages and mindseeds anchor to block-height snapshots verified via `shared_map_commitment`.

- **readable-state publication packs**  
  Intended for light clients and dissemination.  
  Contain human-readable state representations accompanied by cryptographic anchors. Offline packages and mindseeds anchor to block-height snapshots verified via `shared_map_commitment`.

Publication packs:
- MUST contain Protocol v5 events,
- MUST contain required cryptographic signatures and integrity proofs,
- MAY contain availability attestations and omission-auditability material required by the active publication profile,
- Offline packages and mindseeds anchor to block-height snapshots verified via `shared_map_commitment`,
- MAY be aligned to block boundaries for convenience.

Canonical nodes MUST verify publication packs exclusively through deterministic replay and MUST NOT trust any included derived state.

Draft artifacts:
- MUST NOT be included in any publication pack,
- MUST NOT appear as protocol objects,
- MAY only be included if they have been explicitly emitted as Protocol v5 events intended for publication.

Offline systems MAY also produce **cycle export packs**, which are distinct from publication packs.

Cycle export packs:
- are curated, non-authoritative artifacts intended solely for offline browsing, search, and human-readable inspection,
- MAY include selected ideas, connections, histories, and payloads,
- MAY include derived summaries for readability,
- MUST NOT be used for replay, validation, governance activation, or canonical ingestion,
- MUST be fully regenerable from the canonical event log. Offline packages and mindseeds anchor to block-height snapshots verified via `shared_map_commitment`.

Export packs MAY be distributed independently of publication packs.

### 9.1.1 bundle tiers and shared_map_commitment [anchor: bundle_tiers_and_shared_map_commitment]

Mindseeds / SeedPackages MAY include standardized bundle tiers that package canonical-readable state and text payloads for offline browsing and dissemination.

In particular:

SeedPackages MAY contain Tier 0 / Tier 1 / Tier 2 bundles as defined by the Shared Map of Reality and Payload Bundle Specification (Pocket Map, Citizen Map, Civic Archive), including any required payload packs and optional non-canonical UI assets. Offline packages and mindseeds anchor to block-height snapshots verified via `shared_map_commitment`.

SeedPackages MAY include optional rank packs aligned with bundle tiers (e.g., smaller bundles include sparse rank samples; larger bundles include denser or full rank materializations). Offline browsing SHOULD surface rank history at the resolution provided by included packs, and MAY compute finer-grained history by deterministic replay when the required log segments and checkpoints are available.

When a workspace, partition, or delayed-publication stream declares a basis point at block height H, it SHOULD record the following basis commitments:

- `state_root_hash(H)` (canonical facts commitment),
- `pocket_map_payload_root(H)` (Tier 0 / Pocket Map payload-root commitment),
- `shared_map_commitment(H) = HASH("shared_map_commitment_v0" || state_root_hash(H) || pocket_map_payload_root(H))` (single baseline “shared reality” commitment).
  The authoritative definition of `shared_map_commitment` is in the Canonical Encoding and Hashing Specification (v0); this document repeats it only for readability and does not redefine byte-level hashing, tags, or encodings.

Partition provenance preserves block-height basis and `shared_map_commitment` for reconciliation.

The basis point MAY also record `snapshot_hash(H)` if a concrete Snapshot Format v0 artifact is included. Offline packages and mindseeds anchor to block-height snapshots verified via `shared_map_commitment`. The minimum interoperability requirement is the pair `(state_root_hash(H), pocket_map_payload_root(H))` and the derived `shared_map_commitment(H)`.

Offline partitions fork from a declared basis point in the following sense:

- offline execution and packaging MAY reference a basis block height H and the corresponding `shared_map_commitment(H)`,
- offline-produced events remain delayed-publication content until canonically ingested,
- the basis commitment is used for verification, reconciliation transcripts, and survivability auditing, not for authority.

Custody manifests MAY declare bundle tier coverage and MAY include the roots needed to verify that coverage. For example, a custody manifest MAY declare:

- “stores Tier 0 at H” (declaring `pocket_map_payload_root(H)`),
- “stores Tier 1 for range [H1..H2]” (declaring the Tier 1 payload-root commitments for those heights, if defined),
- “stores full archive” (declaring the highest-tier coverage and/or full pack roots).

Such declarations MAY be used for survivability analysis, redundancy tracking, and replication assistance; however, custody manifests remain non-authoritative and MUST NOT alter canonical semantics, validity, replay, or governance activation.


### 9.2 canonical ingestion [anchor: canonical_ingestion]

Canonical ingestion is deterministic and authoritative.

Upon receiving a publication pack, canonical nodes MUST:
- assign `canonical_event_id` values,
- validate each event under the currently active rulebooks,
- validate each event against Protocol v5 cycle and rate-limit rules,
- validate any required availability attestations and prefix-certificate prerequisites under the active publication profile,
- preserve invalid or non-effective events without deletion,
- append accepted events to the canonical event log.

Publication packs MAY include optional custody manifests and state witness artifacts used for replication accounting and survivability analysis; such artifacts MUST NOT alter canonical semantics or confer authority, and MUST be treated as descriptive verification-layer payloads.

Cycle indices, rate-limit accounting, mana effects, and all derived state MUST be computed **after ingestion** using deterministic replay of the merged canonical event log.

Offline verdicts, including offline governance outcomes:
- SHALL NOT be treated as canonical,
- SHALL NOT activate rulebooks,
- SHALL NOT change canonical state.

Only verdicts that appear in the global canonical event log and complete the full Protocol v5 challenge lifecycle may activate rulebooks, mint tokens, or modify canonical rankings.

Offline-produced events fall into the following categories during canonical ingestion:

1. **Structurally invalid events**  
   Events that violate schema, signatures, or active rulebooks.  
   These events:
   - MUST remain visible as existing,
   - MUST be marked non-effective,
   - MUST NOT produce state transitions,
   - MUST NOT contribute to cycles, mana accrual, verdicts, or importance updates,
   - MAY become effective only if later rulebooks render them valid.

2. **Structurally valid but over canonical limits**  
   Events that exceed canonical per-identity pacing, mana availability, or other constraints at their replay position.  
   These events:
   - MUST remain in the canonical log,
   - MUST be treated as non-effective (deferred),
   - MUST become effective automatically once later canonical cycles make them permitted.

3. **Structurally valid and permitted events**  
   Events that become effective immediately upon replay.

During canonical ingestion and replay:
- cycle indices are derived from the canonical event log,
- `lifecycle_state` values are derived from canonical replay,
- eligibility, importance propagation, and POD/POINT routing are derived deterministically.

Canonical event order is assigned only by finalized publication, never by offline authorship time, local sequence numbers, or exchange order.

Any cycle indices, lifecycle_state values, eligibility markers, importance annotations, or verdict outcomes included in offline artifacts are advisory only and MUST NOT bind canonical evaluation.

Nodes and the network SHOULD encourage broad archival replication:
- at least one full-history replica MUST exist within the network,
- the ecosystem SHOULD sustain many independent archival nodes,
- archival nodes SHOULD continuously cross-verify hashes, anchors, and cycle derivations to detect discrepancies or corruption. Offline packages and mindseeds anchor to block-height snapshots verified via `shared_map_commitment`.

This ingestion and replay process MUST be applied identically by all conformant nodes.



### 9.3 basis snapshot reference [anchor: basis_snapshot_reference]

Offline packages and mindseeds anchor to block-height snapshots verified via `shared_map_commitment`.

Offline events and publication packs MAY include:

- a `basis_snapshot_index`,
- a `cycle_index_at_height`,
- non-authoritative rulebook hashes,
- advisory metadata describing the local evaluation context.

Such references are **advisory only**.

They MUST NOT:

- override canonical rulebooks,
- override canonical snapshot ordering,
- override canonical cycle derivation,
- alter canonical validation outcomes.

For **snapshot + delta publication packs**:

- the basis snapshot reference MUST be explicit,
- the pack MUST include cryptographic anchors sufficient to verify that the referenced basis snapshot corresponds to a canonical snapshot recognized by the network.

Canonical nodes MUST independently verify basis references before accepting deltas.

Any reference to a “basis snapshot” MUST align with Protocol v5 snapshot semantics.

A basis snapshot reference:
- SHOULD identify a snapshot keyed to block height,
- MAY include a snapshot tier marker,
- MAY include a snapshot hash or commitment for integrity verification.

Any `cycle_index_at_height` included in offline artifacts is advisory only and MAY be omitted.

Canonical replay and validation MUST NOT depend on `cycle_index_at_height` values and MUST rely exclusively on deterministic replay from the canonical event log and applicable snapshot commitments.


## 10. tokens and economic effects [anchor: 10_tokens_and_economic_effects]

### 10.1 offline neutrality [anchor: offline_neutrality]

Offline systems MUST remain economically neutral.

Offline operation:

- MUST NOT mint POD,
- MUST NOT mint POINT,
- MUST NOT issue, simulate, advance, or settle token balances,
- MUST NOT perform any token-affecting state transitions,
- MUST NOT advance token accrual schedules tied to cycles or verdicts.

Offline work MAY be:

- authored,
- stored,
- replayed,
- organized,
- locally evaluated for informational and planning purposes.

However, **all token effects** occur only:

- after canonical ingestion,
- during deterministic replay of the canonical event log,
- at issuance boundaries defined by the active token rulebooks.

Offline systems MAY compute **provisional or advisory annotations** related to tokens (e.g., “would earn POD if canonical”), but such annotations:

- are non-authoritative,
- MUST NOT be treated as balances,
- MUST NOT unlock actions, mana, or privileges,
- MUST NOT influence local rate limits or cycle advancement,
- MUST NOT be published as canonical events.

Ethical quarantine and exclusion rules apply equally to offline-originated events.

Offline origin does not grant immunity from:

- ethical exclusion,
- eligibility revocation,
- or quarantine-based suppression of token effects during canonical replay.

Offline systems MUST respect canonical lifecycle eligibility when presenting any provisional or informational token-related annotations.

In particular:
- burned or rotted ideas and connections MUST be excluded from any provisional POD/POINT routing, supply, or attribution annotations,
- “would earn,” “potential,” or similar preview indicators MUST respect lifecycle_state and eligibility rules,
- offline UI MUST NOT suggest that excluded structures participate economically.

All token-related annotations in offline mode are informational only and MUST NOT contradict canonical eligibility rules defined in Protocol v5.


### 10.2 canonical issuance [anchor: canonical_issuance]

Canonical token issuance occurs only when all of the following conditions are met:

- events have been ingested into the canonical event log,
- events have passed validation under the currently active token rulebook,
- a deterministic issuance boundary (e.g., snapshot or epoch boundary) is reached.

Token issuance:

- is replayable,
- is deterministic,
- is governed exclusively by canonical rulebooks,
- applies equally to online-originated and offline-originated events.

No token effects may occur retroactively or locally.

Offline systems MUST NOT attempt to pre-commit, reserve, or escrow token effects prior to canonical issuance.

---

## 11. authoritarian-resilience guarantees [anchor: 11_authoritarian_resilience_guarantees]

### 11.1 offline survivability [anchor: offline_survivability]

The system MUST be capable of functioning indefinitely without network connectivity.

Custody and replication obligations (§7A) and human-carried synchronization (§7B) are the primary mechanisms by which survivability is achieved without dependence on centralized node operators.

Offline survivability requires that:

- no external services are required for core reasoning operations,
- no centralized authority checks are required to create or evaluate events,
- no mandatory online verification is required to continue deliberation,
- no central registry or permission gate is required for authorship,
- cycle progression and local replay can proceed using only locally available events.

An offline workspace MUST be sufficient to:

- create ideas and connections,
- create and deliberate on challenges,
- submit arguments and votes,
- record a complete local event history,
- generate local snapshots and cycle boundaries,
- preserve identity continuity claims for later publication.

Offline operation is treated as **delayed publication**, not as an alternative universe.

Accordingly:

- offline cycles are local and non-authoritative,
- offline cycle advancement does not grant additional canonical power,
- offline activity cannot exceed canonical limits upon reintegration.

When offline histories are merged into the canonical universe:

- all events are evaluated under canonical rules,
- cycle indices, rate limits, and token effects are recomputed deterministically,
- excess or non-effective events remain visible but inert.

The system MUST degrade gracefully under censorship, partition, or shutdown, without semantic collapse, loss of meaning, or dependency on continuous connectivity.

Offline survivability requires only a minimal set of artifacts.

At minimum, an offline-capable system MUST be able to preserve:
- the canonical event log (or a contiguous segment thereof),
- cryptographic signatures and integrity proofs required for replay.

Optionally, an offline system MAY also preserve:
- canonical snapshots (tiered, block-height keyed),
- cycle export packs for browsing, search, and inspection.

Export packs are always optional and MUST be treated as non-authoritative and fully regenerable from the canonical event log and snapshots.

Loss of export packs MUST NOT compromise canonical recoverability or survivability.


### 11.2 deferred publication [anchor: deferred_publication]

Users MAY delay publication of offline-authored events indefinitely.

Deferred publication:

- does not invalidate events,
- does not weaken their auditable history,
- does not alter their semantics.

When publication eventually occurs, the published record MUST be:

- complete, including all events intended for publication,
- auditable, with preserved ordering, authorship, and metadata,
- replayable under canonical deterministic rules,
- publicly legible, subject only to canonical safety and visibility lenses.

The protocol MUST allow a complete epistemic history to survive suppression locally and re-enter the public canonical universe intact when conditions permit.



## 12. non-goals and exclusions [anchor: 12_non_goals_and_exclusions]

This specification explicitly does NOT attempt to provide or support the following:

- **offline consensus with the global network**  
  Offline operation does not attempt to reconcile, merge, average, or negotiate canonical state with the network. Reintegration is publication, not consensus.

- **private canonical tribes**  
  There are no private tribes, private tribe ideas, private tribe maps, or tribe-restricted canonical visibility, offline or online.

- **offline token issuance**  
  Offline systems do not mint, issue, reserve, simulate, or advance POD or POINT balances. All token effects are canonical-only.

- **hidden rulebook activation**  
  Rulebooks cannot activate offline, implicitly, or without appearing in the canonical event log and completing the full governance lifecycle.

- **event deletion or retroactive modification**  
  Offline systems must preserve authored events. Events may be marked non-effective, but MUST NOT be deleted, rewritten, or retroactively altered.

Any behavior that would introduce these properties is out of scope and non-conformant.

---

## 13. relationship to other specifications [anchor: 13_relationship_to_other_specifications]

This specification composes with and is subordinate to the following documents. In the event of any conflict, precedence is determined by the authority rules defined in the governing documents, with Protocol v5 remaining the ultimate source of canonical meaning.

- **Protocol v5**  
  Defines canonical semantics, event types, idea ontology, connection types, challenge domains, importance logic, cycle-based pacing and rate limits, block-height keyed snapshot tiers, derived `lifecycle_state` and living-map eligibility, resurrection actions, cycle export packs, and deterministic replay requirements.

- **Deterministic Replay & Merge specification**  
  Defines the canonical replay order, merge semantics, ingestion validation, snapshot derivation rules, and cross-node convergence guarantees for online and offline event logs.

- **Challenge Engine specification**  
  Defines challenge lifecycle, eligibility determination, voter selection, voting and tally rules, verdict finalization, and deterministic application of challenge-driven state transformations.

- **Governance specification**  
  Defines governance challenges, rulebook lifecycle, activation at deterministic cycle boundaries, implementation claims, supersession rules, and how tunable parameters are adopted and applied.

- **Token specification**  
  Defines POD and POINT issuance, deterministic issuance boundaries, routing constraints, eligibility exclusions (including effects of derived `lifecycle_state`), ethical quarantine, and audit requirements.

- **Safety specification**  
  Defines safety classifiers, abstraction rules, jurisdictional visibility lenses, emotional load handling, and required explanatory metadata (including “why am I seeing this?” requirements that apply during reintegration).

- **Identity specification**  
  Defines identity verification and continuity (including local identity vault / Mindseed handling), eligibility and authorship constraints, and challengeable identity claims and attestations.

- **Verification specification**
Defines P2P meetup attestations and P2P state/log witness artifacts (including custody manifests and state witness attestations) used by offline groups to reconcile and evidence replication without conferring authority.

This specification introduces no independent canonical semantics. It specifies offline packaging, local execution, and reintegration behavior that MUST remain semantically equivalent to Protocol v5 under the constraints defined by the specifications above.



