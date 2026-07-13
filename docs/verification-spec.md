---
doc_id: verification_spec
title: Verification Specification
status: authoritative
version: v0
last_reviewed: 2026-06-22

scope:
  - Defines human identity verification and eligibility guarantees.

authoritative_for:
  - Verification mechanisms and identity legitimacy constraints.
  - Rules for who may author canonical events.

not_authoritative_for:
  - Governance mechanics beyond eligibility surfaces.
  - Canonical authored-candidate signed bytes, signature algorithms, or public-key-reference construction.

depends_on:
  - protocol v5.md
  - canonical-event-authorship-and-signature-profile-v0.md
  - node-and-conformance-spec.md
  - tempo-spec.md
  - challenge-engine-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of governance-spec.md and offline-and-mindseed-spec.md.

reader_path:
  - prereq: node-and-conformance-spec.md
  - next: offline-and-mindseed-spec.md

keywords:
  - verification
  - identity
  - eligibility
---

> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

# title
verification spec

## 0. purpose and scope

The Human Verification Specification defines how the platform establishes practical confidence for two narrow claims:

1. the claim that there is a human behind a digital account, referred to as VH,  
2. the claim that a particular identity in the graph corresponds to the specific human behind that account, referred to as VI.

Verification in this protocol is not an administrative approval. It is a collection of truth claims produced by agents and nodes, with evidence claims supporting or contradicting them. The specification governs only how those claims accumulate and how certainty is derived from them.

This document explicitly does not attempt to:

- reveal or store personally identifiable information as canonical data,  
- determine moral worth or social rank of individuals,  
- create any weighting for governance votes,  
- or replace peer witnessing with institutional identity authority.

Any conformant node must be able to evaluate verification state deterministically using this specification and the ordered canonical event log defined in protocol v5.

---

### 0.2 Hierarchy of responsibility

The verification layer composes in a fixed responsibility order:

Anthill hub → evidence ideas → VH/VI certainty tracks → VL eligibility gates.

1) Anthill hub  
   The Anthill is the mandatory graph location in which verification artifacts are anchored and indexed. It provides storage topology only and confers no governance or economic power.

2) Evidence ideas
   All confidence in verification originates from accumulated evidence claims created by real agents. Evidence ideas use existing base idea types, usually `truth_claim`, and are themselves perpetually challengeable.

3) Certainty tracks  
   Implementations MUST maintain two independent quantitative tracks:  
   - vh_certainty — confidence that there exists a human behind the account.  
   - vi_certainty — confidence that the declared identity corresponds to that specific human.

4) VL eligibility gates  
   Read/write permissions, voter pools, and invite friction are derived deterministically from the VH and VI tracks via thresholds that map to VL tiers.

In all cases:  
- verification NEVER weights votes,  
- verification NEVER determines payouts,  
- no subsystem may alter this hierarchy.


## 1. definitions and terms

### 1.1 identity versus account

An **identity** is a canonical idea in the graph representing a real agent capable of saying things. An **account** is the key material and login surface used to submit events on behalf of that identity.

The protocol treats verification as statements about identities, not accounts alone. Evidence about account control must always be modeled as what an agent or node is willing to publicly claim it observed.

### 1.2 verification tracks

- **VH (human presence)**  
  A track accumulating claims and evidence that a real human interacted with the system or another human in a verification-relevant context.

- **VI (identity correspondence)**  
  A separate track accumulating claims that link a graph identity to the human controlling an account.

These tracks are **separable claims**. The protocol must never collapse them into a single fuzzy status. Certainty is evaluated per track before any mapping to discrete levels.

### 1.3 certainty and verification levels

Certainty is a **derived scalar** computed during replay. Verification Levels, VL, are discrete tiers deterministically mapped from certainty values.

VL tiers may gate only:

- ordinary canonical-writer eligibility,
- narrow Tempo `tempo_contributor` eligibility,
- `beacon_qualified_identity` and diversity-gate eligibility,
- how many invites an account may issue,  
- how much personal mana the account may hold,  
- and eligibility to participate in voter pools.

VL tiers must never be used as weights for:

- truth evaluation outside verification,  
- Tempo claim or evidence influence,
- importance rankings,  
- governance decisions,  
- or cryptocurrency payouts.

### 1.3A canonical writer eligibility gate

Canonical write submission eligibility is derived from verification state and is separate from canonical read access.

- Canonical reads remain publicly readable and do not require verification.
- Ordinary canonical writes require satisfaction of the active canonical-writer verification gate.
- The only exception is the narrow Tempo repair lane defined in Protocol v5 and the Tempo Specification: an eligible human `tempo_contributor` may create only target-bound ordinary `truth_claim` ideas with valid `tempo_claim` metadata and, if the active Tempo profile permits it, Tempo-context evidence ideas and evidence/same_as connections.
- `tempo_contributor` eligibility does not grant arbitrary canonical idea creation, evidence creation outside Tempo context, connection creation outside Tempo context, challenge creation, voting, verdict finalization, governance authority, POD, POINT, or token authority.
- Challenge creation and voting are canonical write actions and therefore use ordinary challenge eligibility; `tempo_contributor` status alone is insufficient.

Current deployment profile:
- The canonical writer gate is implemented as `canonical_writer_level` and is currently issued through the Seed verifier role.
- This is a gate for write eligibility, not an authority override of canonical challengeability or replay semantics.

Verification gates eligibility but MUST NOT weight influence. In Tempo, verification may affect `tempo_contributor` and `beacon_qualified_identity` eligibility, but it MUST NOT multiply certainty, challenge, governance, or Tempo influence.

### 1.4 evidence ideas

Typical evidence classes to be consumed later include:

- p2p meetup attestations between humans,  
- attestations from nodes observing provider logins,  
- institutional issuer credential attestations,  
- operator verification attestations,  
- and ideas asserting negative fraud signals.

The actual identifiers, images, or databases used in any check must remain off-system or in the subject’s local vault. The canonical graph references only an identity-authored idea attesting what was observed.

---

## 2. goals and invariants

### 2.1 human-first determinism

- the canonical authority of the platform is the append-only event log plus protocol rules,  
- verification certainty must be computable identically by every node,  
- all verification claims and evidence remain perpetually challengeable,  
- and any derived block or snapshot boundary must stall rather than introduce nondeterminism.

### 2.2 privacy boundary

- no event may contain plaintext personally identifying fields,  
- no institutional system may be privileged,  
- and any assurance requiring raw documents must be performed privately by the user.

### 2.3 sybil resistance through friction

The design relies on social and economic friction:

- limited invites,  
- requirement for diverse human counterparties,  
- recency evaluated in cycles,  
- diminishing returns,  
- and taint propagation forward through lineage when fraud is proven.

These defenses restrict only permissions and never dehumanize participants or entrench irreversible privilege.

### 2.4 failure and correction

If evidence is missing, invalid, or successfully challenged:

- certainty drops automatically,  
- VH/VI gates recompute only at boundaries,  
- the previously active rulebook set remains in force,  
- and routing edges may burn while preserving the permanent historical record.

The platform must continue functioning safely offline and across jurisdictions without relying on provider policies.

---

## 3. Canonical graph patterns for verification

Verification state accumulates exclusively through ideas in which an agent says something relevant to VH or VI. During deterministic replay, nodes interpret those artifacts as holding or failing according to uniform algorithms.

The interpretation pathway is:

- agents produce verification-relevant claims,  
- nodes record those claims as canonical events,  
- additional claims operate as evidence supporting or contradicting earlier ones,  
- replay computes a certainty scalar per track,  
- and VL tiers are mapped from those scalars.

No provider database, document image, or off-platform identifier may be interpreted as part of the canonical graph. The protocol reasons only over attestations and their cryptographic proofs that identify who made the statement.

Evidence concentration inside a small social cluster is treated neutrally by the algorithm and may trigger diminishing returns or ordinary fraud challenges, but never administrative discretion.

### 3.1 Anthill anchoring requirement

All verification artifacts MUST be representable as ideas in the graph and anchored to the Anthill hub. The Anthill provides storage topology and legibility only and confers no governance or economic power.

### 3.2 Required linkage fields

Every verification artifact MUST include:
- subject_identity_id — the identity being verified,
- attester_identity_id — the identity asserting evidence,
- schema_type — one of the standard verification schemas,
- referenced_cycle_or_block_id (coarse, optional) — deterministic reference only.

These fields establish provenance while conferring no authority over ordering or supply.

### 3.3 Chronological chaining (legibility only)

Implementations SHOULD provide non-authoritative vines that chain:
verification_prediction → artifact_observed → followup_evidence.

Such vines are presentation aids only; the canonical authority remains the event log + snapshots. Nodes may ignore vines entirely during validation.


## 4. challenges to verification claims

Any identity in the platform may initiate a challenge against verification ideas or evidence ideas using the ordinary challenge primitive.

A verification challenge may concern:

- whether a human presence interaction actually occurred,  
- whether the correspondence between an identity and a human holds,  
- whether an attestation violated the raw-PII prohibition,  
- or whether negative fraud signals should apply.

Outcomes of challenges affect only derived state:

- certainty scalars for VH or VI recompute automatically,  
- VL eligibility gates update,  
- invite lineage may become tainted forward in time,  
- and previously active rules remain until a compatible activation boundary.

No verification challenge may:

- change event ordering in the log,  
- delete any artifact,  
- or convert attestations into vote weighting.

If an attester refuses or cannot defend the claim, the idea simply fails verification during replay and certainty drops without human judgment.

Appeals and reopen flows follow the same deterministic rules as any other domain; the verification system never finalizes permanently.


### 4.1 VH vs VI track separation

Nodes MUST keep VH and VI as independent lanes. Evidence accumulation in one track may not implicitly alter the other.

### 4.2 Track outputs

Each derived snapshot MUST expose:
- vh_certainty,
- vi_certainty,
- vl_tier,
- eligibility_flags including ordinary canonical-writer eligibility, `tempo_contributor`, `beacon_qualified_identity`, can_invite, and voter_pool_member where applicable.

These outputs are computed deterministically from evidence ideas under this specification.

---

## 5 Standard schemas and workflows

### 5.1 Referenced schemas

This specification relies on the following standard verification schemas. Exact byte layout and canonical field definitions live exclusively in Appendix A.

- p2p_meetup_attestation  
- gov_id_witness_attestation  
- provider_login_attestation  
- issuer_credential_attestation  
- operator_verification_attestation  
- negative_fraud_claim.

### 5.2 Onboarding workflow

After account creation, nodes route:
p2p_meetup_attestation → vh_certainty updates,  
issuer/credential or provider_login_attestation → vi_certainty updates.

### 5.3 Challenge workflow

Any identity with ordinary challenge eligibility may open:
truth_challenge on a verification artifact,  
importance_challenge on evidence class,  
representation_challenge on VL mapping.

Tempo contributor status alone does not authorize verification challenges, time-related truth challenges, or voting in any challenge.

### 5.4 Taint workflow

If duplicates or fraud are alleged, nodes apply:
negative_fraud_claim → quarantine gating,  
verdict_reopened → recompute eligibility.

### 5.5 Centralized lanes

ID-checked attestations and operator verifications are optional complements that feed the same VH/VI tracks. They MAY affect invite friction only and never govern reading or speaking.


## 6. p2p meetup protocol

### 6.1 human presence friction

The peer-to-peer meetup protocol raises VH certainty through direct human-scale interaction. What becomes canonical is only the statement each side is willing to say, signed by the identity that says it.

### 6.2 attestation handshake

A physical meetup produces two independent attestations. Each participant signs their own statement about what they personally witnessed.

Meetups MAY optionally include exchange of custody manifests and generation of state witness attestations as defined in Section 6A.

Minimum fields for `p2p_meetup_attestation`:

- attester_identity_id  
- subject_identity_id  
- meetup_type = p2p_liveness  
- timestamp_range (coarse only)  
- nonce  
- optional: mutual_meetup_nonce  
- signature(attester_private_key)

`timestamp_range` is intentionally coarse. It is a bounded interval such as “within this hour” or “within this day,” chosen to support replay determinism and avoid precise tracking. It MUST NOT be used for canonical ordering or payout rules.

`nonce` is generated by the attester to prevent replay or copy-paste reuse. `mutual_meetup_nonce` is optional and may be shared during the meetup so that both attestations can explicitly link to a single shared encounter without becoming a single jointly-authored artifact.

The protocol records only the two claims:

- “identity A says they met identity B in person during time range R,”  
- “identity B says they met identity A in person during time range R.”

The protocol does not require that both attestations exist, but certainty computation may apply stronger weights when a linked pair exists.

### 6.3 privacy and coercion-safe invariant

No location coordinates, names, government identifiers, photographs, biometrics, or other raw PII may be included in canonical meetup attestations. Any off-system checks of documents remain private to the user and are not required by the protocol.

The protocol also treats refusal as safe. No participant is required to create an attestation during or after a meetup. A refusal, or a choice to delay, must not be interpreted as negative evidence by itself. Claims about coercion or pressure are handled only through explicit challengeable artifacts and rulebook-defined procedures.

### 6.4 challenges

Anyone may challenge a meetup attestation. If fraud is proven, the artifact fails verification and certainty drops without deletion. The historical record remains intact; only derived certainty and derived gates change.


## **6A. P2P State and Log Verification Protocol**

### **6A.1 Purpose and scope**

This section defines a peer-to-peer protocol by which two verified human identities may exchange, compare, and attest to their **view of canonical system state**, including snapshots, seedpacks, and event-log ranges.

This protocol is **orthogonal** to human verification (VH/VI):

* It does not establish identity.
* It does not confer authority.
* It does not determine truth, importance, or ordering.
* It produces only *witnessed claims about data possession and observation*.

The goal is to allow humans to verify not only *who* they met, but *what state of the system they observed each other holding*, in a manner compatible with offline operation and deterministic replay.

---

### **6A.2 Non-authoritative nature**

P2P state and log verification:

* MUST NOT grant ordering authority
* MUST NOT advance cycles
* MUST NOT finalize challenges
* MUST NOT resolve conflicts
* MUST NOT influence VH or VI scores directly

All artifacts produced under this protocol are **descriptive**, not **decisive**.

They record observations, not judgments.

---

### **6A.3 Custody manifest**

A **custody manifest** is a structured summary of the canonical artifacts an identity claims to possess at a given moment.

Custody manifests are **optional** and **non-authoritative**, but when exchanged they MUST follow a deterministic schema.

A custody manifest MAY include:

* one or more `snapshot_id`
* corresponding `snapshot_hash`
* one or more `seedpack_id`
* highest known `event_log_tip_hash`
* declared `log_range` held (if partial)
* optional declared custody role(s) (e.g., personal, collective, archival)
* optional declared storage class (pointer / working / archival)

Custody manifests MUST:

* be canonically encoded
* be hashable
* exclude precise timestamps
* exclude location data
* exclude personal identifiers beyond identity IDs

A custody manifest is a **claim of possession only**, not a claim of correctness or completeness.

---

### **6A.4 State witness attestation**

A new verification artifact is defined:

**`p2p_state_witness_attestation`**

This attestation records that one identity observed another identity presenting a specific custody manifest.

#### Required fields

* `attester_identity_id`
* `observed_identity_id`
* `observed_manifest_hash`
* `observation_scope`
* `nonce`
* `signature`

#### Observation scope

`observation_scope` MUST be one of:

* `manifest_only`
* `snapshot`
* `seedpack`
* `log_range`

The scope specifies *what level of verification the attester claims to have performed*, not what the observed identity claimed.

---

### **6A.5 Mutual witnessing (optional)**

As with P2P meetup attestations, state witness attestations MAY be mutual.

If two identities exchange attestations referencing:

* the same `observed_manifest_hash`
* a shared `nonce`

then the replay engine MAY interpret this as **mutual observation**, but no additional authority or trust weighting is implied.

Mutuality is optional.
Lack of mutuality MUST NOT be interpreted as negative evidence.

---

### **6A.6 Divergence acknowledgment**

If two identities compare custody manifests and observe disagreement, either party MAY issue a state witness attestation indicating divergence.

In this case:

* `observation_scope` MUST be `manifest_only`
* the attestation records *that divergence was observed*, not which view is correct

Divergence is an expected and normal condition during offline or partitioned operation and MUST NOT be treated as fraud or error.

---

### **6A.7 Relationship to VH and VI scoring**

State witness attestations:

* MUST NOT directly affect VH certainty
* MUST NOT directly affect VI certainty
* MUST NOT be converted into trust weights

They MAY be consumed by higher-level systems (offline replication, custody assignment, archival audits), but **never as identity truth signals**.

Any system that treats state possession as identity trust is non-conformant.

---

### **6A.8 Challenges and invalidation**

A `p2p_state_witness_attestation` MAY be challenged.

Valid grounds for challenge include:

* invalid signature
* malformed manifest hash
* provable fabrication (e.g., manifest does not decode to a valid schema)

If invalidated:

* the attestation becomes ineffective
* no automatic VH or VI penalty applies
* penalties MAY occur only if a separate fraud claim is upheld

---

### **6A.9 Privacy and refusal safety**

Participation in P2P state verification is strictly optional.

An identity MAY:

* refuse to present a custody manifest
* refuse to sign a state witness attestation
* refuse mutual witnessing

Refusal MUST:

* carry no negative inference
* carry no score penalty
* be treated as a neutral outcome

All privacy protections applicable to P2P meetup verification apply equally here.

---

### **6A.10 Deterministic replay requirements**

All custody manifests and state witness attestations MUST:

* be deterministically serializable
* replay identically across nodes
* remain valid independent of when or where they are observed

Replay MUST preserve attestations even if their referenced data later becomes unavailable.

---

## 7. invite-only onboarding and lineage accountability

### 7.1 identity creation is limited

Invites are scarce and create accountability chains. An identity may exist only after being invited by another agent willing to say so, under the applicable rulebooks.

### 7.2 lineage taint, negative evidence, and boundary conditions

If an invited identity is later proven fraudulent, compromised, or non-human, the inviter’s lineage becomes tainted only forward in time. Forward taint affects derived state only, including:

- future invite rates,  
- derived eligibility pools,  
- probation relief,  
- and any rulebook-defined quarantine gates.

Negative evidence is expressed as explicit, challengeable artifacts (for example `negative_fraud_claim`). Negative evidence MAY reduce certainty or trigger quarantine only when it holds under the challenge system.

Boundary conditions:

- A lack of evidence is not negative evidence. If no new artifacts exist, certainty may decay by rulebook-defined decay rules, but it must not “flip” into fraud.  
- A refusal to attest is not negative evidence.  
- Provider failures or missing centralized proofs are not negative evidence unless a specific claim is made and upheld that fraud occurred.  
- Any taint or quarantine effect MUST be reversible by new evidence and/or successful challenges; nothing is permanent.

### 7.3 no permanence

Lineage signals confer no authority or weighting. They exist only to restrict surface breath and reduce the effectiveness of sybil and automation attacks.

### 7.4 appeals

Appeals and reopen flows follow ordinary deterministic rules. Mistakes do not disappear, but they stop causing harm when overturned.

### 7.5 centralized tracks and probation effects (informational)

Centralized lanes (government-ID witnessing, provider logins, issuer credentials, operator verification) may be used to shorten onboarding probation or reduce invite friction when attested.

These lanes:

- MAY affect only derived VH/VI certainty and the resulting VL gates,  
- MUST NOT be required for reading or speaking,  
- MUST NOT create privileged citizenship or permanent access,  
- and MUST NOT be interpreted as vote weighting or payout eligibility.

---

## 8. certainty computation (deterministic scoring overview)

### 8.1 derived scalars per track

During replay, nodes compute two certainty scalars independently:

- vh_certainty — confidence that a human exists behind the account,  
- vi_certainty — confidence that the graph identity corresponds to that specific human.

These are derived only from canonical verification artifacts and their challenge outcomes.

### 8.2 evidence weights and schema classes

Each evidence schema contributes according to rulebook-defined weights. The scoring model MUST support distinct base weights per schema class, such as:

- p2p meetups,  
- issuer credentials,  
- provider logins,  
- operator verifications,  
- government-ID witnessing,  
- and negative fraud claims.

Weights must be applied deterministically and may be updated only by rulebook activation at defined boundaries.

### 8.3 recency, decay, and cadence

Certainty computation MUST incorporate time as cycles, not wall clock time. Recency effects MUST be derived from cycle distance, and certainty MAY decay when an identity becomes stale relative to the rulebook parameters.

Decay reduces certainty gradually and deterministically; it must not introduce any non-deterministic “judgment” step.

### 8.4 diversity and diminishing returns

To prevent trivial amplification inside a single cluster, certainty computation MUST incorporate:

- diversity constraints (distinct counterparties, distinct attesters, or other deterministic diversity measures), and  
- diminishing returns for repeated attestations from the same relationship neighborhood or the same evidence class.

These mechanisms are deterministic and parameterized by rulebooks. They never require administrative discretion.

### 8.5 negative evidence and recomputation boundaries

Negative evidence is accumulated through explicit artifacts and takes effect only when upheld by challenges. When negative evidence holds, replay recomputes certainty and derived gates deterministically at the defined boundary.

The presence of contradicting artifacts triggers recomputation only at boundaries. No admin approval is required.

### 8.6 VL mapping and gates

Verification Levels are mapped deterministically from vh_certainty and vi_certainty using rulebook-defined thresholds.

VL gates may restrict only:

- invite issuance,  
- maximum personal mana,  
- and voter-pool eligibility.

VL gates must never:

- weight votes,  
- alter governance standing,  
- influence truth/importance evaluation outside verification,  
- or influence POD/POINT payouts.

### 8.7 delegation to Appendix A

The exact algorithms for:

- scoring functions,  
- decay curves,  
- diversity constraints,  
- diminishing-returns rules,  
- threshold values,  
- and required derived outputs,

are normative only in Appendix A of this document. Any implementation variance MUST be treated as non-conformant.

---

## 9. third-party and centralized verification

### 9.1 general principle

Nodes MAY use external or centralized systems to generate additional evidence claims when a witness, operator, or issuer is willing to say it observed a procedure succeed. Such systems include government ID checks, major social-media logins, or operator verification similar to how exchanges and platforms such as x.com conduct onboarding.

The canonical graph reasons only over the attestation artifact. The existence of a centralized check does not create authority; it creates an optional signal.

### 9.2 mapping outputs to schemas

Any external source lane must be translated by the observing node into protocol evidence types defined here:

- `gov_id_witness_attestation`  
  A witness says it observed a government-ID verification complete off-system and binds only the resulting VL numbers to the graph.

- `provider_login_attestation`  
  A node or witness says a social-media or Google/Facebook login succeeded and that the account surface was controlled during that session.

- `issuer_credential_attestation`  
  A trusted institution says it issued or verified a credential relevant to identity correspondence.

- `operator_verification_attestation`  
  A platform operator says it verified account control using non-PII procedures.

- `negative_fraud_claim`  
  An agent says it has evidence of automation, duplication, or key compromise.

Other nodes may ignore these lanes completely and remain fully conformant.

### 9.3 domains affected by VL gating

Centralized signals may affect only:

- invite issuance rate,  
- maximum personal mana allocation,  
- eligibility to participate in voter pools,  
- and onboarding probation relief when attested.

Centralized signals MUST NOT affect:

- evaluation of ordinary truth claims outside VH/VI,  
- importance or ranking of any idea,  
- governance participation or rulebook verdicts,  
- or POINT/POD cryptocurrency payouts.

### 9.4 anchor independence

No single third-party system is privileged. Multiple providers may coexist, and failure in one does not require rejection in others unless a node is willing to say so.

---

## 10. user permissions and responsibilities

### 10.1 what users can claim

Any user may create ideas stating:

- that they witnessed another human in person,  
- that they believe an identity corresponds to them,  
- or that they observed centralized procedures succeed.

The protocol records only what the user is willing to say; it never forces deeper engagement.

### 10.2 user privacy boundary

Users MUST keep any raw documents privately if they require personal assurance. The protocol contains no mechanism to publish or weight those documents.

### 10.3 no dignity transforms

Verification numbers never encode moral worth. Even when fraud is proven, artifacts persist as canonical history; only gates recompute.

### 10.4 counterparties

Users should prefer diverse human counterparties when raising VH certainty to avoid ordinary diminishing returns. This preference is normative guidance only and remains challengeable.

---

## 11. node interoperability and verification

### 11.1 requirements for nodes using institutions

Nodes implementing centralized observation must:

- map outputs to the schemas listed above,  
- ensure their artifacts can be verified by any other node using appendix algorithms,  
- and maintain the ability to validate and replay the universe without relying on provider databases.

### 11.2 proof-of-serving challenges

Any node claiming to host a verification pack must be able to:

- receive a request referencing a specific payload hash and optional pack_profile_id,  
- return the raw bytes or an inclusion proof,  
- and produce failure evidence if it cannot serve.

The spine itself assigns no penalties; it only makes verification possible.

### 11.3 deletion prohibition

No node may introduce a rule that deletes artifacts or converts verification signals into vote weighting.

### 11.4 merge independence

During offline operation or after deterministic merges, nodes must recompute VH and VI identically before VL gates update.

### 11.5 banner of authority

The immutable event log plus protocol rules remain the sole authority over verification. Blocks, epochs, or institutions never supersede this.

---

## 12. Offline / Mindseed integration

### 12.1 Independence from providers

The wider protocol assumes that participants and nodes may operate without access to centralized databases. This specification therefore requires that VH and VI confidence can be established solely from canonical attestations created by real agents. Provider logins, government-ID checks, or exchange-style verifications are optional complements only when a specific agent is willing to publish a statement that converts those signals into one of the schemas defined here.

### 12.2 Boundary-based recomputation

During deterministic replay, all effects of verification occur only at defined boundaries such as cycles or snapshots. Nodes MUST interpret the following identically:

- ingestion of new verification artifacts as ordinary canonical events,  
- gradual decay of certainty according to cycle distance,  
- quarantine triggers derived from upheld negative fraud claims,  
- and VL tier mapping from vh_certainty and vi_certainty.

No human or digital clock outside the system is consulted as authority.

### 12.3 Transport between offline groups

Individuals or gatherings MAY create attestations while offline and later exchange Mindseed logs physically or digitally. Offline transport MAY include both P2P meetup attestations and P2P state witness attestations, which replay as verification artifacts. When such logs are merged, the protocol requires that:

- all artifacts identify who made the statement and what schema was used,  
- supporting and contradicting claims remain preserved,  
- and certainty is recomputed deterministically after the merge.

The platform must remain usable for establishing confidence even when many lanes of activity originate off-line.

### 12.4 Refusal-safe and reversible gates

Offline operation may produce fewer attestations. A lack of artifacts, or a refusal to attest, MUST NOT be interpreted as negative evidence by itself. Any effect on eligibility or probation is reversible by new evidence and/or successful challenges. Nodes MAY ignore provider lanes entirely and remain fully interoperable with the wider protocol.

### 12.5 Mapping to the mechanical commons

If a universe implements derived blocks or downloadable bundles, nodes must be able to map any block or bundle reference to an event range in the canonical log and validate VH/VI state without reliance on those blocks. Such artifacts express legibility only and confer no authority over ordering or economic supply.

## 13. How verification affects system permissions

### 13.1 VH and VI as quantitative inputs

Permissions are derived from the two certainty tracks and from no other source. Nodes MUST apply thresholds uniformly to map those tracks to VL tiers before any gating decision.

### 13.2 VL gating table

Verification Levels may gate only:

- ordinary_canonical_writer_eligibility
- tempo_contributor_eligibility
- beacon_qualified_identity_or_diversity_gate_eligibility
- invite_rate  
- personal_mana_max  
- voter_pool_eligibility  
- onboarding_probation_relief.

Verification Levels must never gate or weight:

- importance_ranking  
- governance_votes  
- Tempo claim or evidence influence
- truth_evaluation outside verification  
- POD or POINT payouts.

### 13.3 Challengeable provenance complements

Centralized lanes—when attested by a specific operator—are permitted only as complements that MAY reduce probation or affect invite friction. They are never required for reading or speaking, and any node may ignore them.

### 13.4 Allowed challenge domains

- challenging_vh_claim  
- challenging_vi_claim  
- challenging_evidence_object  
- challenging_institutional_signal  
- challenging_provider_login_observation  
- challenging_operator_verification_attestation.

### 13.5 Taint and quarantine outcomes

If a negative_fraud_claim is upheld, forward surface permissions may be gated and the affected eligibility pools recomputed. The decision is deterministic and reversible; no artifact is deleted and no administrative discretion is involved.

### 13.6 Deletion prohibition banner

No algorithm or operator may delete verification artifacts. No mechanism may convert VL tiers into governance or payout weighting. Any relief mechanism must rely only on what a real agent has signed and on deterministic recomputation.

No mechanism may convert VL tiers, provider lanes, institutions, jurisdictions, POD, POINT, wealth, reputation, or roles into unequal Tempo truth certainty, structural support, challenge, governance, or repair influence.

### 13.7 User understanding goal

The intent of these tables is to help users see step by step:

- which agents said they witnessed something,  
- how many claims support or contradict VH and VI,  
- and what rule produced the current eligibility decision.

Such understanding is a transparency aim only and confers no extra power.

---

## 14. Deterministic data model

### 14.1 Artifact classes and schema typing

All verification-relevant information MUST enter the canonical log only as structured, signed artifacts created by real agents. Each artifact MUST declare a `schema_type` that selects a canonical field set defined in Appendix A.

Canonical reasoning is performed only over:
- artifact fields,
- signatures and key provenance,
- and challenge outcomes that uphold or invalidate those artifacts.

No off-platform database, document image, or provider record is part of the canonical graph. Such materials may be consulted privately by an attester, but only the attester’s signed statement as an identity-authored idea becomes a canonical evidence idea.

### 14.2 Required identity link fields

Every verification artifact MUST include, at minimum:
- attester_identity_id,
- subject_identity_id,
- schema_type,
- nonce,
- signature(attester_private_key).

Artifacts MAY include:
- timestamp_range (coarse only),
- mutual_meetup_nonce (optional),
- referenced_cycle_or_block_id (coarse only, descriptive only).

No optional field may affect canonical ordering, validity, governance, or payouts.

### 14.3 Canonical link patterns and anchoring

Nodes MUST be able to materialize a deterministic verification subgraph such that:

- each subject identity has an Anthill hub anchor,
- each verification artifact is anchored under that hub,
- and artifacts may reference prior artifacts as evidence links.

If evidence links are present, they MUST use deterministic fields (for example: referenced_artifact_id(s)). If links are absent, replay MUST still compute certainty from the set of artifacts alone.

Non-authoritative vines MAY be used to render chronological legibility, but vines MUST NOT be required for correctness.

### 14.4 Required derived outputs

At each boundary where derived state is updated, replay MUST produce:

- vh_certainty
- vi_certainty
- vl_tier
- eligibility_flags.

Implementations MAY also output:
- vh_components and vi_components (breakdown of evidence class contributions),
- taint_flags and quarantine_flags,
All derived outputs MUST be reproducible from the canonical log plus Appendix A.

All derived outputs MUST be reproducible from the canonical log plus Appendix A.

### 14.5 Deterministic invalidation and state transitions

Artifacts are never deleted. An artifact becomes ineffective only by:
- failing signature validation,
- being invalidated by a challenge verdict,
- or being discounted by deterministic scoring rules (such as decay or diminishing returns).

Taint and quarantine effects, when applicable, are derived state only and apply forward in time. They MUST be reversible by successful appeals or new evidence according to deterministic boundary rules.

---
## 15. Relationship to other specifications

### 15.1 Protocol v5

Protocol v5 defines the canonical event log, cycle boundaries, deterministic replay, and the constitutional invariants of the system. This verification specification is a constrained pillar inside that framework.

Verification artifacts are ordinary canonical events and MUST be ingested identically by every node. Verification may only produce derived certainty values and derived eligibility gates.

Verification MUST NOT introduce:
- any alternative authority over ordering,
- any special administrative discretion,
- any deletion or retroactive erasure,
- any vote weighting,
- or any payout weighting.

### 15.2 Challenge Engine specification

The Challenge Engine defines how challenges are created, argued, voted, and finalized. Verification relies on the Challenge Engine only to resolve disputes over:
- VH-relevant artifacts,
- VI-relevant artifacts,
- negative fraud claims,
- and any rulebook-defined verification procedures.

Challenge verdicts may invalidate artifacts and thereby change derived certainty at boundaries. No other mechanism may invalidate evidence.

### 15.2A Tempo Specification

Tempo defines target-bound time truth claims, Tempo-context evidence ideas/connections, derived target truth-certainty band state, structural-support state, and beacon qualification. Verification supplies only deterministic eligibility gates for `tempo_contributor`, `beacon_qualified_identity`, and diversity counting.

An eligible human `tempo_contributor` may author the Dmax truth claim used by `structural_dmax_liveness_predicate`, but verification status only gates eligibility. It does not make the claim certain, does not satisfy beacon diversity by itself, and does not grant challenge, governance, POD, POINT, token, ordinary mana, rate-limit, or authorization-frontier authority.

Verification MUST NOT:
- multiply Tempo claim, evidence, challenge, or governance influence,
- let non-human or AI actors create canonical Tempo claims or Tempo-context evidence,
- grant ordinary challenge creation or voting rights through Tempo contributor status,
- reduce beacon or authorization thresholds during population collapse.

### 15.3 Deterministic replay and merge specification

Replay and merge define how multiple lanes of history are reconciled into a single canonical state. Verification requires that, after any merge:
- all verification artifacts remain preserved,
- certainty is recomputed deterministically,
- and VL gates update only at defined boundaries.

No provider or institutional lane may supersede merge/replay correctness.

### 15.4 Governance specification

Governance defines how rulebooks are proposed, challenged, approved, and activated. Governance may adjust verification parameters such as:
- evidence weights,
- decay and recency rules,
- diminishing returns,
- diversity constraints,
- and VL thresholds.

Any such changes MUST activate only at defined boundaries and must be fully deterministic.

Governance MUST NOT introduce vote weighting or payout weighting through verification, even indirectly.

### 15.5 Token specification

The Token specification defines POD and POINT. Verification interacts with tokens only by gating eligibility to participate (invite issuance, personal mana maximum, voter-pool membership). Verification MUST NOT:
- change POD or POINT accounting,
- change reward allocation,
- or change the truth/importance ranking logic outside verification.

### 15.6 Safety specification and safety rulebook interface mechanics

Safety rulebooks govern visibility and payload abstraction. Verification artifacts are subject to the same safety lens mechanisms as any other content. Safety may:
- restrict exposure of sensitive metadata,

- and control UI presentation.

Safety MUST NOT alter verification scoring outputs beyond what is implied by canonical artifact availability and deterministic replay.

### 15.7 Canonical preservation and provenance spine

The preservation spine and its pack/bundle mechanisms define how payloads and history are replicated, committed, and served. Verification requires only that:
- artifacts can be hashed and verified like any other payload,
- pack commitments can include verification artifacts,
- and nodes can validate proofs of serving if those mechanisms are supported.

Preservation mechanisms remain non-authoritative over ordering and do not affect verification scoring other than ensuring artifacts are retrievable and verifiable.

---
## 16 Random meetups and gatherings (guidance)

### 16.1 Pairing mechanisms

To improve diversity of witnessing, implementations MAY support random pairing for voluntary encounters. Random pairing is a transport and social-coordination feature only and produces no authority or economic advantage.

- A pairing action creates an opportunity for agents to say something about VH or VI under the schemas already defined.  
- The pairing system MUST NOT generate attestations automatically; only the human identity may choose to submit a statement.

### 16.2 Large gatherings

Meetups MAY occur as organized gatherings rather than one-to-one meetings.

- A gathering can be modeled as an actionable idea or as a concrete action in the graph.  
- Many independent `p2p_meetup_attestation` artifacts may reference that gathering via coarse identifiers.  
- Certainty upgrades are delegated to deterministic scoring; stronger effects MAY occur when numerous distinct attesters participate.

### 16.3 Anti-coercion guidance

Refusal-safe norms remain paramount.

- No participant should be pressured to attest.  
- Any claim that coercion occurred must be expressed only as explicit, challengeable artifacts and never inferred from silence.  
- Operator or pairing systems MUST provide clear messaging that participation is voluntary and reversible.

### 16.4 Ordinary effects

Random meetups or gatherings MAY:

- raise VH or VI certainty when agents are willing to say so,  
- reduce onboarding probation according to rulebooks,  
- and provide new material for challenges.

They MUST NOT weight votes or affect POINT/POD payouts.

---

## 17 Privacy, safety, and abuse prevention

### 17.1 Canonical boundary

The protocol reasons only over what real agents have signed. This specification therefore enforces a strict prohibition on canonical inclusion of sensitive tracking material.

- No coordinates, photographs, names, addresses, government numbers, biometrics, or raw provider databases may be stored in canonical verification artifacts.  
- Attesters may consult such materials privately, but publication MUST contain only the resulting signed statement and hashes defined by Appendix A.

### 17.2 Doxxing and harassment safeguards

Any mechanism that encourages exposure of personal details is non-conformant.

- Interfaces MUST provide refusal options and delays.  
- Identities may challenge any artifact that appears to target or shame another person.  
- Negative fraud claims must focus on procedural contradiction, not humiliation.

### 17.3 Taint semantics

Taint applies only to derived eligibility.

- A taint flag signals uncertainty about VH or VI and may increase friction for invites.  
- Taint is reversible by new evidence or appeals.  
- Taint MUST NOT be interpreted as proof of guilt.

### 17.4 Abuse challenges

Agents may open challenges concerning:

- violation of privacy invariant,  
- coercive verification behavior,  
- or harassment tied to evidence accumulation.

If upheld, the abusive artifact fails effectiveness without deletion.

### 17.5 Social belief vs mechanical authority

Uniform algorithms, not discretion, determine certainty.

- Evidence concentration in a single neighborhood triggers only diminishing returns.  
- No early relief or upgrade may occur outside boundaries.  
- The ethic that no human is the enemy guides interpretation of conflicts.

---

## 18 Appendix V1 — Example flows (informative)

### 18.1 Onboarding

identity created →  
agents submit `issuer_credential_attestation` or `provider_login_attestation` →  
replay computes vh_certainty and vi_certainty →  
vl_tier gates eligibility.

### 18.2 One-to-one meetup

A meets B in person →  
A submits `p2p_meetup_attestation` → VH lane updates;  
B MAY submit their own artifact → paired upgrades delegated to scoring.

### 18.3 ID-checked meetup (attested)

Operator or witness performs private ID check →  
attester submits `gov_id_witness_attestation` →  
only derived results may upgrade VH or VI tiers.

### 18.4 Duplicate discovered

`negative_fraud_claim` holds under challenge →  
quarantine gates recomputed →  
certainty drops →  
effects remain reversible.

### 18.5 Lineage tainted

fraud upheld on invited identity →  
inviter invite_rate reduced only forward →  
no effect on reading/speaking.

### 18.6 Provider observation ignored

Any raw provider database imported → rejected by light or archive nodes as non-conformant.

These examples illustrate intended use only. Exact math, weights, and thresholds remain defined exclusively in Appendix A.



## 19. identity control, key authority, and recovery

### 19.1 identity persistence and key authority

An identity is a canonical idea representing a real agent capable of saying things.  
Control of that identity is expressed operationally through cryptographic key material used to sign canonical events.

The protocol distinguishes between:

- the **identity** (persistent canonical anchor), and  
- the **active controlling keyset** (replay-derived signing authority).

Keys are not the identity. Keys are replaceable instruments used to speak for the identity.

Nodes MUST treat ordinary human-authored event signatures as valid only when they satisfy `canonical-event-authorship-and-signature-profile-v0.md`: the signed bytes reconstruct exactly, `public_key_ref` resolves to a key owned by `author_identity_id`, and that key is active according to canonical state derived from replay. Verification status controls whether the identity is eligible to use that key for the event family; it does not redefine signature bytes or key-reference construction.

---

### 19.2 canonical control state

Control of an identity is represented as derived state computed during replay.

Replay MUST be able to determine:

- the currently accepted controlling keyset(s) for an identity, and  
- the chronological history of prior control assignments.

Control state changes only through canonical artifacts that assert or dispute identity control.

No operator, node, or institution may alter control state outside the canonical log and challenge process.

---

### 19.3 control claims and recovery claims

The protocol supports canonical artifacts asserting identity control transitions.

Typical control-related artifacts MAY include:

- `identity_control_claim`  
  An identity or witness asserts that a specified keyset controls an identity.

- `identity_control_rotation_claim`  
  An identity asserts that control should move from one keyset to another.

- `identity_control_recovery_claim`  
  A claimant asserts that control of an identity should be reassigned due to key loss, compromise, or theft.

- `identity_control_dispute`  
  An agent asserts that the currently recognized controlling keyset is invalid or compromised.

All such artifacts:

- MUST be signed by the asserting identity or witness,  
- MUST remain perpetually challengeable,  
- MUST NOT delete historical control evidence,  
- and affect only derived control state after replay.

---

### 19.4 evidence for recovery or reassignment

Recovery or reassignment of identity control relies on the same verification primitives used elsewhere in this specification.

Evidence MAY include:

- p2p meetup attestations,
- issuer credential attestations,
- operator verification attestations,
- provider login attestations,
- lineage continuity evidence,
- previously issued credentials,
- or any other canonical evidence idea.

Raw documents, identifiers, or biometric materials MUST remain off-system or in the subject’s local vault. Only the signed identity-authored idea describing what was observed may enter the canonical graph.

Recovery therefore consists of producing new canonical evidence ideas sufficient to support a control reassignment claim.

---

### 19.5 deterministic reassignment of control

During replay:

- nodes MUST evaluate control-related artifacts using deterministic rules,
- upheld recovery or dispute outcomes MUST update the derived controlling keyset,
- invalid or challenged claims MUST be ignored without deletion.

Control reassignment applies only forward in time.

Historical events remain attributed to the keyset that signed them at the time.

No reassignment may alter past ordering, invalidate existing artifacts, or erase history.

---

### 19.6 theft, compromise, and dispute handling

If a keyset is alleged to be compromised:

- any agent MAY submit a dispute artifact,
- the dispute proceeds through the ordinary challenge lifecycle,
- if upheld, replay recomputes control state,
- the compromised keyset becomes ineffective for future events.

The protocol never deletes compromised artifacts.  
It only prevents further authority from the invalidated keyset.

---

### 19.7 multi-key and recovery-friendly configurations

Implementations MAY support:

- multiple simultaneous controlling keysets,
- hardware backup keys,
- recovery contacts represented as identities,
- or other redundancy strategies.

These mechanisms remain implementation-level conveniences and must still resolve through canonical artifacts and deterministic replay.

No implementation may rely on hidden administrator override, private databases, or undisclosed recovery procedures.

---

### 19.8 offline recovery and mindseed compatibility

Recovery claims and supporting evidence MAY be created while offline.

When offline logs are later merged:

- all control-related artifacts MUST replay deterministically,
- derived control state MUST recompute identically across nodes,
- and eligibility gates update only at defined boundaries.

The system must remain capable of recovering identity control without reliance on centralized services.

---

### 19.9 invariants

Identity control mechanisms MUST satisfy:

- identity persistence independent of any single wallet or device,
- recoverability through challengeable canonical evidence ideas,
- absence of administrative override authority,
- preservation of full historical record,
- deterministic replay across nodes,
- and privacy preservation through off-system storage of sensitive materials.

Any implementation that allows identity control to be reset by administrative action, provider database authority, or undisclosed procedures is non-conformant.
