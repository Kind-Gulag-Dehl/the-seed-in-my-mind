# Identity Admission and Invitation Capacity Specification v0

**Status:** Draft
**Version:** 0.1.0
**Protocol profile:** Profile v0
**Applies to:** Protocol v5
**Authority:** Authoritative upon adoption and inclusion in the authoritative index
**Primary subject:** Canonical identity admission, sponsorship, invitation eligibility, invitation capacity, and the boundary between admission and human verification

## Related specifications

This specification MUST be read together with:

* `protocol v5.md`
* `protocol v5-appendix-a.md`
* `canonical-event-authorship-and-signature-profile-v0.md`
* `canonical-encoding-and-hashing-spec.md`
* `deterministic-replay-and-merge-spec.md`
* `verification-spec.md`
* `cycle-spec.md`
* `privacy-and-high-risk-submission-spec.md`
* `offline-and-mindseed-spec.md`
* `node-and-conformance-spec.md`
* `cross-doc-invariants.md`

Where this specification defines identity-admission semantics more specifically than general identity, registration, onboarding, or invitation language elsewhere, this specification controls admission behavior after adoption, subject to Protocol v5 Section 0 and the repository’s authoritative precedence rules.

This specification does not replace the Verification Specification’s authority over:

* verification claims;
* verification evidence;
* VH and VI certainty;
* challengeable verification artifacts;
* attestation schemas;
* derived verification levels;
* writer and inviter eligibility derivation.

It defines how a person enters canonical identity state so that those verification processes may subsequently occur.

## Change control

Changes to the following require protocol-level specification reconciliation rather than implementation-only modification:

* who may authorize canonical identity creation;
* whether canonical admission requires scarce capacity;
* whether an applicant may self-author ordinary Profile-v0 identity creation;
* whether admission grants ordinary participation authority;
* whether invitation authority may be permanently restricted to a privileged class;
* whether private or operator-controlled state may determine canonical admission;
* whether AI or system identities may sponsor ordinary human identities;
* whether unrestricted canonical self-registration is active under Profile v0;
* whether sponsorship itself constitutes human-verification evidence;
* whether social or Anthill relationships automatically confer verification authority;
* whether a newly admitted identity may participate in a restricted verification lane before ordinary writer eligibility.

Exact rates, maturation periods, certainty thresholds, capacity caps, and other explicitly rulebook-controlled parameters MAY change through the applicable canonical rulebook process without changing the constitutional boundaries in this specification.

---

# 0. Normative language and notation

## 0.1 Normative terms

The terms **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are normative.

Their meanings are:

* **MUST**, **REQUIRED**, or **SHALL**: the rule is mandatory for a conforming implementation.
* **MUST NOT** or **SHALL NOT**: the prohibited behavior is invalid.
* **SHOULD** or **RECOMMENDED**: the behavior is expected unless a documented and protocol-compatible reason justifies another implementation.
* **SHOULD NOT**: the behavior is discouraged and requires an explicit protocol-compatible justification.
* **MAY** or **OPTIONAL**: the behavior is permitted but not required.

Descriptive examples, explanatory notes, security analysis, and implementation guidance are non-normative unless they explicitly use normative language.

## 0.2 Identifiers and references

This specification uses the identifier, encoding, and canonical-position definitions established by the canonical encoding, event-authorship, replay, verification, and cycle specifications.

The following notation applies:

* `identity_id`: the canonical identifier of an identity.
* `event_id`: the canonical identifier of an authored event candidate.
* `author_identity_id`: the canonical identity that authored and signed an event.
* `subject_identity_id`: the canonical identity that a claim, attestation, challenge, or verification artifact concerns.
* `attester_identity_id`: the canonical identity making a verification-relevant claim or attestation.
* `public_key_ref`: the canonical reference to a public-key descriptor.
* `cycle_id`: the identifier of a canonical cycle.
* `rulebook_id`: the identifier or hash of the rulebook state governing a transition.
* `canonical_position`: the deterministic publication position at which an event becomes effective.
* `invitation_capacity`: the replay-derived number of identity admissions currently available to an inviter.
* `admission_liveness_blocked`: the replay-derived state indicating that no qualifying human-certified capacity period is available to generate new admission capacity, advance inviter maturation, activate new inviter eligibility, or restore invitation authority.
* `admission_authorization_reference`: the canonical value or deterministic reference committing to the intended admission profile, sponsor identity, capacity period, and rulebook reference.
* `sponsor_identity_id`: the identity that authorizes admission. Under Profile v0 this is normally derived from `author_identity_id`, not independently asserted.
* `target_identity_id`: the identity created by the admission event.
* `initial_public_key_ref`: the canonical reference to the target identity’s initial Profile-v0 signing key.
* `initial_key_possession_proof`: the target key’s proof that its controller possesses the corresponding private key.
* `verification_artifact_id`: the canonical identifier of a verification claim, attestation, evidence object, contradiction, challenge, or verification outcome.
* `identity_structural_roots`: the complete protocol-defined identity structural-root set that a canonical identity must have under the active profile. Under Profile v0, the required root set consists of Mindgarden, Backyard of Relationships, Self Tree, and Anthill. Exact canonical identifiers, byte encodings, derivation rules, structural-role constants, and containment mechanics remain deferred to Appendix A and structural-role reconciliation.
* `anthill_anchor`: the deterministically identifiable Anthill root or Anthill reference within `identity_structural_roots` under which the identity’s social and verification relationships may be organized and inspected.

Unless another authoritative specification explicitly states otherwise:

1. Canonical effects become valid at the event’s `canonical_position`.
2. Eligibility is evaluated against canonical state immediately before that position.
3. Event effects are applied atomically.
4. Arrival order, node-local time, network route, private account state, and private implementation state do not affect the canonical result.
5. References to wall-clock dates or transport timestamps are not canonical authorization inputs unless explicitly admitted as evidence under another specification.
6. A valid signature proves control of an authorized key at a canonical position; it does not prove human existence, uniqueness, civil identity, or truth.
7. A social or Anthill relationship does not constitute verification evidence unless an explicit verification artifact says what an attester claims to have observed or concluded.

## 0.3 Authority boundaries

This specification governs:

* the distinction between local identity preparation and canonical identity creation;
* the Profile-v0 admission model;
* the role and authority of the sponsor;
* invitation eligibility;
* invitation-capacity structure;
* the canonical authorization requirements for `identity_create`;
* admission lineage and sponsor provenance;
* the initial authority of a newly created identity;
* the required `identity_structural_roots` established by canonical admission;
* the boundary between admission, key control, verification, and later participation eligibility;
* the treatment of non-canonical admission requests;
* the constitutional limits on gatekeeping and admission centralization;
* the requirement that newly admitted identities have a restricted path to participate in their own verification without receiving ordinary canonical-writer authority;
* the distinction between identity structural-root topology, including Anthill topology, and explicit verification claims.

This specification does not replace the authority of:

* Appendix A over exact canonical event payload schemas and effects;
* the Profile-v0 authorship specification over signed candidate bytes, key descriptors, signatures, and possession-proof encodings;
* the Verification Specification over VH, VI, verification evidence, certainty derivation, attestation schemas, verification challenges, and verification-level outputs;
* the Cycle Specification over canonical cycle formation and boundary semantics;
* the Replay Specification over deterministic event application and conflict resolution;
* the Privacy and Offline Specifications over transport, relay, local storage, private evidence handling, and high-risk-submission behavior.

Where this specification requires a field, effect, rejection condition, restricted event family, or verification anchor to exist, Appendix A and the relevant encoding specifications MUST define its exact canonical representation.

Where this specification defers a numeric value, certainty threshold, maturation period, capacity amount, or eligibility formula to a rulebook, the rulebook MUST produce deterministic replay-visible outputs.

A private service, administrator setting, database flag, private social graph, popularity score, or opaque machine classification MUST NOT substitute for required canonical or rulebook-derived state.

## 0.4 Verification-boundary notation

This specification uses the following distinct verification concepts:

* **VH**, or human-presence verification: derived confidence that a real human controls or operates the relevant identity surface.
* **VI**, or identity-correspondence verification: derived confidence that the canonical identity corresponds to the particular human asserted by the relevant claims.
* **verification claim**: an ordinary attributable canonical truth claim concerning a verification predicate.
* **verification evidence**: ordinary canonical evidence material, represented through the protocol's authorized evidence ideas, evidence claims, evidence connections, contradiction relationships, or protected commitments, offered in support of or opposition to a verification claim.
* **verification attestation**: an ordinary human-authored verification truth claim under an attestation schema, describing a verification-relevant observation, interaction, procedure, credential, or continuity fact.
* **verification challenge**: an ordinary canonical challenge, or a constrained identity-verification profile of the ordinary challenge process, directed at a verification claim, evidence relationship, artifact, or derived verification state.
* **verification certainty**: replay-derived VH or VI confidence calculated from authorized claims, evidence, contradictions, challenges, outcomes, and rulebook rules.
* **verification eligibility output**: a replay-derived state such as ordinary-writer eligibility, inviter eligibility, or another event-family permission.

This specification does not define the full calculation of VH or VI. It requires admission behavior to preserve the inputs and boundaries needed by the Verification Specification.

## 0.5 Common verification ontology

Identity verification uses the protocol's ordinary canonical truth-claim, evidence, contradiction, and challenge ontology.

A verification claim is:

```text
ordinary canonical truth claim
+ verification predicate or schema
+ subject identity reference
```

Verification-specific event families MAY provide restricted authorization to create or relate those ordinary canonical objects.

They MUST NOT create a separate epistemic system in which verification facts become true through administrative status assignment.

Verification evidence MUST use the ordinary evidence model, including authorized evidence ideas, evidence claims, evidence connections, contradiction relationships, challenge objects, challenge responses, challenge outcomes, and privacy-preserving commitments where another specification permits them.

Verification-specific schemas MAY constrain subject identity, predicate, evidence type, author eligibility, privacy treatment, challenge rights, payload size, and rate limits.

Those constraints do not create a parallel verification-only object class.

## 0.6 Verification derivation chain

Verification authority follows this derivation chain:

```text
raw canonical verification artifacts
-> rulebook evaluation
-> derived VH or VI certainty
-> canonical boundary activation
-> event-family-specific eligibility output
```

Raw verification artifacts are historical canonical objects and relationships.

VH and VI certainty are replay-derived outputs.

Writer, challenge, voter, governance, Tempo, attester, and inviter eligibility are separate replay-derived lanes.

No raw verification artifact directly changes an eligibility lane unless an authoritative rule derives and activates that output.

---

# 1. Purpose

## 1.1 Problem statement

A civilizational-scale decentralized reasoning system must permit every human to seek participation without allowing arbitrary key generation to create unlimited canonical identities or unlimited access to shared human attention.

Identity admission therefore faces two simultaneous risks.

The first is **gatekeeping capture**. A system is not meaningfully open if admission can be controlled by:

* one government;
* one company;
* one identity provider;
* one founder group;
* one node operator;
* one permanent committee;
* one inherited class of early users;
* one social, political, economic, geographic, or cultural network.

A person without an existing relationship to a participant must still have a viable path to admission. This includes people who:

* live under censorship or authoritarian government;
* cannot safely disclose civil identity;
* lack recognized identity documents;
* are geographically isolated;
* have intermittent connectivity;
* must participate pseudonymously;
* are outside the dominant language or social graph;
* are unpopular with existing participants.

The second risk is **automated or coordinated flooding**. A system is not meaningfully open if real humans are overwhelmed by:

* mass-generated identities;
* bot-authored submissions;
* synthetic verification networks;
* invitation rings;
* challenge flooding;
* verification-request flooding;
* canonical-storage exhaustion;
* multiplication of per-identity allowances through Sybil identities;
* nation-state or commercial identity farms.

Permissionless key generation cannot by itself establish canonical identity existence. A cryptographic key proves control of a key, not the existence of a distinct human.

Likewise, strict verification after identity creation cannot by itself protect the admission layer if unlimited identity creation consumes permanent canonical storage or overwhelms verification attention.

The admission system must therefore distinguish among:

1. permissionless local identity preparation;
2. non-canonical requests for admission;
3. scarce canonical identity creation;
4. cryptographic key control;
5. explicit human-verification claims and evidence;
6. derived VH and VI certainty;
7. ordinary canonical-writer eligibility;
8. challenge, voting, governance, Tempo, and invitation eligibility.

## 1.2 Design objective

Profile v0 uses **sponsored public admission**.

The intended progression is:

```text
permissionless local identity preparation
→ non-canonical admission request
→ eligible human sponsor consumes invitation capacity
→ sponsor-authored canonical identity creation
→ CanonicalAdmittedIdentity with an active initial key
→ restricted identity-verification participation
→ separate human-verification progression
→ later orthogonal ordinary-writer eligibility
→ later orthogonal inviter eligibility
```

The design separates openness from canonical authority.

Any person MAY prepare an identity candidate locally. No sponsor, operator, government, institution, node, or existing participant may prevent local key generation.

Local preparation alone does not:

* create canonical identity state;
* consume permanent canonical storage;
* prove that the controller is human;
* establish VI;
* grant writing authority;
* grant ordinary challenge authority;
* grant voting or governance authority;
* grant Tempo eligibility;
* grant invitation capacity;
* grant economic authority.

Canonical identity creation requires a valid `identity_create` event authored by an eligible human sponsor with available invitation capacity.

The sponsor’s role is narrow. The sponsor authorizes the use of one unit of distributed admission capacity to create a canonical identity and bind its initial key.

The sponsor does not thereby certify:

* the applicant’s legal name;
* the applicant’s civil identity;
* the applicant’s political views;
* the truth of the applicant’s future claims;
* the applicant’s future conduct;
* the applicant’s current verified-human status;
* the applicant’s qualification for ordinary participation;
* the applicant’s uniqueness as a human.

The applicant separately proves possession of the initial key. This proof establishes key control only.

The created identity begins in a canonically inactive ordinary-participation state.

It MAY:

* control and rotate its keys;
* submit the identity-scoped verification claims allowed by the Verification Specification;
* submit verification evidence concerning itself where authorized;
* acknowledge or respond to verification attestations;
* respond to identity-scoped verification challenges;
* establish continuity evidence;
* maintain its `identity_structural_roots`, including the Anthill root, where authorized.

It MUST NOT automatically receive ordinary canonical bandwidth.

## 1.3 Verification through truth claims and evidence

Human verification MUST be represented through explicit, attributable, signed, challengeable ordinary canonical truth claims, evidence, contradictions, and challenges.

It MUST NOT use a parallel verification-only truth system.

Verification MUST NOT be represented solely through:

* an administrator approval;
* a private account flag;
* a hidden score;
* a social-graph connection count;
* an invitation relationship;
* possession of a key;
* payment;
* membership in an organization;
* AI classification;
* a node operator’s assertion.

Verification may include truth claims such as:

* a claim that a real human controls an identity;
* a claim that an identity corresponds to a particular human;
* a claim that two interactions involved the same continuing person;
* a claim that an identity is controlled by the same person who controlled it during an earlier event;
* a claim that two canonical identities are controlled by one human;
* a claim that an identity is automated, compromised, duplicated, or fraudulently represented.

Such claims are not treated as true merely because they were submitted.

They may be:

* supported;
* contradicted;
* challenged;
* defended;
* superseded;
* invalidated prospectively;
* incorporated into VH or VI certainty according to deterministic rulebook rules.

Each verification artifact is an ordinary canonical object or relationship in a verification role.

Each verification artifact MUST preserve:

* the subject identity;
* the attesting or authoring identity;
* the claimed verification predicate;
* the artifact schema;
* provenance;
* canonical position;
* relevant evidence references;
* challenge and outcome relationships.

Restricted verification authorization MAY allow a newly admitted identity to create or relate these ordinary objects before ordinary writer eligibility.

That restricted authorization does not make the objects true and does not grant ordinary canonical writing.

## 1.4 Identity structural roots and Anthill role

Each canonical identity MUST have a deterministically identifiable `identity_structural_roots` set.

Under Profile v0, `identity_structural_roots` consists of exactly:

1. Mindgarden;
2. Backyard of Relationships;
3. Self Tree;
4. Anthill.

A valid Profile-v0 `identity_create` MUST atomically create or deterministically derive the complete protocol-defined identity structural-root set required by the active authoritative protocol.

The Profile-v0 required root names are fixed here as Mindgarden, Backyard of Relationships, Self Tree, and Anthill.

Exact canonical identifiers, byte encodings, derivation rules, structural-role constants, containment relations, and whether each root is an explicit object or deterministic derivation remain delegated to Appendix A and the relevant structural-role specifications.

The Anthill is one specialized root within `identity_structural_roots`.

The Anthill provides a structural and navigational hub for:

* mutually acknowledged social relationships;
* verification claims concerning the identity;
* attestations concerning the identity;
* evidence supporting or contradicting verification claims;
* verification challenges;
* identity-continuity relationships;
* social and admission provenance relevant to inspection;
* explanation of how derived verification state was produced.

The Anthill itself does not establish truth.

The following do not automatically change VH or VI certainty:

* adding an identity to an Anthill;
* creating a mutual relationship;
* acknowledging that two identities know each other;
* being connected to many identities;
* being connected to highly verified identities;
* sharing an admission lineage;
* belonging to a tribe, organization, or social cluster.

Verification changes require explicit verification artifacts under authorized schemas.

The Anthill supplies topology, provenance visibility, and legibility.

The verification artifacts anchored to or organized through it supply epistemic content.

Whether the Anthill root is:

* explicitly created as part of `identity_create`; or
* deterministically derived from the canonical identity object

MUST be settled together with the rest of `identity_structural_roots` by Appendix A and the relevant structural specifications.

No implementation may create a circular dependency in which an identity requires ordinary writer eligibility to submit the verification artifacts needed to obtain ordinary writer eligibility.

## 1.5 Restricted verification participation

A newly admitted canonical identity MUST have a restricted verification-event lane.

This lane exists to prevent the following circularity:

```text
identity cannot write until verified
but
identity cannot participate in verification until it can write
```

The restricted lane MAY permit only event families needed to:

* prove current key control;
* assert self-correspondence or continuity;
* acknowledge an attestation;
* respond to an identity-specific challenge;
* submit defined identity-verification evidence;
* dispute a verification artifact concerning the identity;
* rotate or secure identity keys;
* maintain authorized identity structural roots.

The restricted lane MUST NOT grant authority to:

* create arbitrary ideas;
* create arbitrary connections;
* issue general truth challenges;
* vote;
* govern;
* submit Tempo claims;
* invite other identities;
* perform economic actions.

The exact event families, payloads, eligibility rules, and challenge rights remain controlled by Appendix A and the Verification Specification.

## 1.6 Civilizational-scale requirements

The admission system MUST satisfy the following long-term requirements.

### 1.6.1 No permanent admission class

Invitation authority MUST NOT remain permanently restricted to:

* genesis identities;
* founders;
* operators;
* delegates;
* experts;
* institutions;
* governance officeholders;
* wealthy participants;
* descendants of an original invitation network.

Every human identity that satisfies the same deterministic verification, continuity, maturation, key-control, and good-standing requirements MUST be capable of becoming inviter-eligible.

### 1.6.2 No single mandatory identity authority

The protocol MUST NOT require every applicant to use one:

* government identity system;
* biometric database;
* corporation;
* proof-of-personhood provider;
* institution;
* geographic jurisdiction;
* verification organization.

Such sources MAY provide optional evidence under the Verification Specification, but no single source may become the universal admission or verification authority.

### 1.6.3 Pseudonymous admission

A person MUST be able to seek canonical admission without universally disclosing:

* legal name;
* government identifier;
* precise location;
* political affiliation;
* private communications;
* persistent network address.

Profile v0 preserves the sponsor’s canonical authorship, but it does not require private transport metadata or civil-identity information to become canonical.

### 1.6.4 Admission of strangers

An eligible sponsor MAY sponsor an applicant with whom the sponsor has no prior personal relationship.

The protocol MUST NOT interpret lack of personal acquaintance as invalid sponsorship.

Public, relay-based, randomized, privacy-preserving, or offline request systems MAY assist applicants in reaching eligible sponsors.

These systems are transport and interface mechanisms, not independent canonical authorities.

### 1.6.5 Bounded canonical growth

Canonical identity creation MUST consume scarce, replay-derived invitation capacity.

Pending requests, failed requests, and local identity candidates MUST NOT become canonical identities merely because they exist or are signed.

### 1.6.6 Progressive authority

Canonical identity existence MUST NOT imply meaningful participation authority.

Verification, writing, challenges, voting, governance, Tempo, invitation capacity, and economic eligibility MUST be derived separately.

### 1.6.7 Deterministic enforcement

All canonical admission authorization, capacity generation, capacity consumption, verification eligibility, suspension, and restoration effects MUST be:

* deterministic;
* replayable;
* publicly inspectable;
* independent of node-local discretion;
* independent of private account state;
* independent of undisclosed machine judgments.

### 1.6.8 Practical inviter capacity

Inviter eligibility MUST have practical effect.

For each qualifying capacity period, every identity that is inviter-eligible and not suspended MUST receive at least one spendable invitation-capacity unit.

A rulebook MUST NOT make inviter eligibility nominal by assigning zero capacity indefinitely to an otherwise eligible and unsuspended class of human identities.

### 1.6.9 Admission liveness

A qualifying capacity period requires a properly certified human-deliberative cycle under the Cycle Specification.

When no qualifying capacity period occurs, replay MUST expose `admission_liveness_blocked = true` or an equivalent deterministic state.

That state is a visible liveness failure. It is not an authorization for operators, AI, system emitters, wall-clock timers, or machine-only boundaries to mint replacement capacity.

Previously generated spendable capacity remains spendable during stalled or non-qualifying periods unless it is suspended, expired under a rule already applicable before the stall, frozen by an authorized emergency rule, or blocked by another explicit constitutional rule.

### 1.6.10 Long-term extensibility

Profile v0 defines one active admission model: sponsored public admission.

Future profiles MAY add independent admission paths, including:

* bounded open admission;
* distributed human ceremonies;
* threshold sponsorship;
* anonymous credentials;
* other decentralized personhood evidence.

A future path MUST specify its own scarcity, authorization, privacy, storage, replay, abuse, verification, and initial-authority rules before becoming active.

Profile v0 MUST NOT be interpreted as permanently forbidding all future self-registration or alternate admission mechanisms.
---

# 2. Core invariants

## 2.1 Permissionless local identity preparation

Any person MAY locally generate:

* one or more cryptographic keypairs;
* a proposed `identity_id`;
* a Profile-v0 key descriptor;
* a `public_key_ref`;
* proof of possession;
* a signed admission request;
* private continuity or recovery material.

No canonical permission is required.

Local generation is outside canonical consensus until a valid canonical admission event is accepted.

The protocol MUST NOT attempt to prohibit local creation of cryptographic identities.

It constrains only:

* which identities enter canonical shared state;
* how canonical identity creation is authorized;
* which authorities a canonical identity receives.

## 2.2 Canonical identity creation is distinct from local preparation

A local identity candidate MUST NOT be treated as a canonical identity.

The following do not establish canonical identity existence:

* possession of a keypair;
* publication of a public key;
* possession of a signed admission request;
* appearance in a relay queue;
* a private product account;
* an operator-managed database row;
* a pending sponsorship conversation;
* a node-local identity record;
* an AI-generated identity proposal.

Canonical identity existence begins only through:

* authoritative genesis admission; or
* a valid finalized canonical `identity_create` event under the active admission profile.

## 2.3 Admission is distinct from verification and eligibility

The following states are distinct:

```text
identity existence
!= key control
!= verified-human status
!= verified-identity correspondence
!= ordinary canonical-writer eligibility
!= challenge eligibility
!= voter eligibility
!= governance eligibility
!= Tempo eligibility
!= inviter eligibility
!= economic eligibility
```

No implementation may infer one state solely from another unless an authoritative rule explicitly defines the transition.

In particular:

* identity creation establishes identity existence;
* initial key registration establishes key control;
* neither establishes VH;
* neither establishes VI;
* invitation sponsorship does not establish VH or VI;
* human verification does not automatically grant every event-family permission;
* writer eligibility does not automatically grant voting, governance, Tempo, or inviter eligibility;
* inviter eligibility does not increase the epistemic weight of the inviter’s claims;
* verification level does not increase vote weight, truth weight, importance weight, or Tempo influence.

## 2.4 Verification claims and Anthill separation

Human verification MUST be represented through explicit, signed, challengeable ordinary claims, evidence, contradiction relationships, and challenges.

Canonical identity admission, sponsor authorship, invitation lineage, identity structural-root membership, Anthill membership, social connection, and key possession do not by themselves establish verified-human status or verified-identity correspondence.

Verification MUST maintain at least two independent claim tracks:

* **VH:** confidence that a real human controls or operates the relevant identity surface;
* **VI:** confidence that the canonical identity corresponds to the particular human asserted by the relevant claims.

Verification-specific event families MAY create or relate ordinary canonical objects such as:

* truth claims;
* evidence ideas or evidence claims;
* evidence connections;
* attestations as truth claims under attestation schemas;
* contradiction relationships;
* challenge objects;
* challenge responses;
* verification outcomes.

Each verification artifact MUST identify:

* its subject identity;
* its author or attesting identity;
* its verification predicate or schema;
* its canonical provenance;
* relevant evidence references;
* any challenge or outcome relationships.

Each canonical identity MUST have a deterministically identifiable `identity_structural_roots` set.

The Anthill root within that set provides the location through which verification artifacts and relevant human relationships may be organized and inspected.

The Anthill provides:

* structural organization;
* provenance visibility;
* social topology;
* navigability;
* explanation of verification derivation.

The Anthill itself confers no:

* verification authority;
* writer authority;
* challenge authority;
* voting authority;
* governance authority;
* Tempo authority;
* economic authority;
* truth weight;
* importance weight;
* invitation authority.

A mutual Anthill connection, relationship edge, admission-lineage relationship, or social acknowledgment MUST NOT alter VH or VI certainty unless one or more identities separately submit verification artifacts under an authorized schema.

Verification certainty and eligibility MUST be derived deterministically from:

* signed verification claims;
* supporting evidence;
* contradicting evidence;
* challenge outcomes;
* active verification rulebooks;
* cycle-boundary activation rules.

The raw artifact, derived VH or VI certainty, and event-family eligibility output MUST remain distinct.

Private administrator flags, product-account records, social popularity, Anthill degree, and opaque machine classifications MUST NOT establish canonical verification state.

## 2.5 Restricted verification-lane invariant

A `CanonicalAdmittedIdentity` MUST be able to participate in the minimum identity-scoped verification processes necessary to become verified.

This restricted lane MUST NOT require ordinary canonical-writer eligibility.

The lane MAY authorize only specified event families needed for:

* self-correspondence claims;
* identity-continuity claims;
* key-control claims;
* acknowledgments of verification attestations;
* responses to verification challenges concerning the identity;
* submission of authorized verification evidence;
* disputes concerning verification artifacts attached to the identity;
* key rotation or revocation;
* maintenance of required identity structural state.

Those event families are restricted authorization paths into the ordinary canonical truth, evidence, contradiction, and challenge ontology.

They MUST NOT create opaque verification-only records disconnected from ordinary epistemic semantics.

The restricted lane MUST NOT authorize:

* ordinary idea creation;
* ordinary connection creation;
* general truth challenges;
* voting;
* governance;
* Tempo claims;
* invitation;
* economic issuance or transfer.

The exact restricted event families MUST be defined by Appendix A and the Verification Specification.

## 2.6 Human sponsorship

A normal post-genesis Profile-v0 `identity_create` event MUST be authored by an existing inviter-eligible human identity.

The target identity MUST NOT author its own ordinary Profile-v0 admission event.

The sponsor MUST:

* exist canonically before the admission event;
* control an active authorized signing key;
* satisfy inviter eligibility immediately before the event;
* possess sufficient invitation capacity;
* not be under active admission suspension.

The applicant MUST separately prove possession of the initial target key.

A sponsor’s signature and the applicant’s possession proof serve different purposes and MUST NOT be conflated.

Sponsorship itself MUST NOT be interpreted as a VH or VI attestation.

A sponsor MAY separately submit a verification attestation under the Verification Specification, but that attestation is a distinct canonical artifact subject to ordinary verification challenges and evidence rules.

## 2.7 No permanent privileged admission class

Inviter eligibility MUST be derived from general rules applicable to all human identities.

An implementation MUST NOT require discretionary approval from a permanent privileged body before an otherwise eligible human can become inviter-eligible.

Genesis status, operator status, institutional affiliation, wealth, governance office, expertise, or social prominence MUST NOT by themselves create permanent invitation authority.

Temporary bootstrap rules MAY establish initial state, but they MUST NOT become an enduring post-genesis admission mechanism.

## 2.8 Canonical admission scarcity

Each successful post-genesis Profile-v0 identity admission MUST consume invitation capacity.

Invitation capacity MUST be:

* replay-derived;
* identity-bound;
* non-transferable;
* non-saleable;
* non-delegable under Profile v0;
* bounded;
* deterministically consumed;
* unable to fall below zero.

An identical retry of the same accepted event MUST NOT consume capacity twice.

Concurrent admissions competing for limited capacity MUST resolve according to canonical order.

A transport-level admission request does not consume canonical capacity.

Capacity is consumed only when the corresponding canonical identity creation is validly applied.

## 2.9 Political and epistemic neutrality

Sponsorship authorizes admission into the identity-verification system.

It is not an endorsement.

Canonical admission MUST NOT require the sponsor to approve or attest to:

* an applicant’s political opinions;
* an applicant’s religion or worldview;
* an applicant’s future claims;
* an applicant’s social popularity;
* an applicant’s expertise;
* an applicant’s economic status;
* an applicant’s conformity with a dominant community.

An inviter MUST NOT automatically lose capacity merely because an invitee later:

* expresses unpopular ideas;
* makes an incorrect claim;
* disagrees with the inviter;
* fails to achieve high verification;
* commits isolated misconduct;
* becomes politically controversial.

Admission-related consequences require evidence of coordinated admission abuse under defined canonical procedures.

## 2.10 No private admission or verification authority

Canonical admission and canonical verification MUST NOT depend on:

* private account records;
* session state;
* passwords;
* cookies;
* email addresses;
* private operator allowlists;
* undisclosed reputation scores;
* private AI classifications;
* node-local trust settings;
* non-replayable administrative decisions;
* private Anthill edges not represented canonically where required.

A private system MAY facilitate transport or user experience, but it cannot determine canonical admission or verification validity.

All canonical eligibility, verification, and capacity inputs MUST be reconstructible from canonical or explicitly admitted genesis state.

## 2.11 No AI or system admission authority

AI and system identities MUST NOT sponsor ordinary human identity creation under Profile v0.

AI systems MAY:

* assist in formatting admission requests;
* detect possible spam;
* summarize public evidence;
* suggest possible abuse patterns;
* assist human reviewers;
* relay requests;
* help identify potentially relevant verification claims.

AI output MUST NOT independently:

* create a canonical identity;
* spend invitation capacity;
* establish VH;
* establish VI;
* suspend inviter eligibility;
* authorize sponsorship;
* determine canonical admission validity;
* determine canonical verification certainty;
* impose a canonical challenge outcome.

The constrained `system_boundary_emitter`, where defined elsewhere, is not an eligible human sponsor.

## 2.12 Non-retroactivity

A later change in:

* sponsor verification;
* sponsor eligibility;
* sponsor capacity;
* invitee conduct;
* admission-lineage assessment;
* VH or VI certainty;
* rulebook parameters

MUST NOT retroactively invalidate an identity creation that was valid at its canonical position, except where a separate constitutional invalidity rule explicitly applies.

Forward-looking effects MAY alter future eligibility or invitation capacity.

Historical identity existence, sponsor provenance, key history, verification artifacts, and valid authored events remain part of deterministic replay.

---

# 3. Scope

## 3.1 Included

This specification defines the Profile-v0 rules for:

1. Permissionless local identity preparation.
2. The boundary between local and canonical identity state.
3. Non-canonical admission requests.
4. Sponsor-authored canonical identity creation.
5. Sponsor authorization.
6. Applicant proof of initial-key possession.
7. Invitation eligibility.
8. Invitation-capacity structure.
9. Capacity generation and consumption boundaries.
10. Canonical admission provenance.
11. Admission lineage.
12. The initial ordinary-authority inactive status of a new identity.
13. Stranger sponsorship.
14. Pseudonymous and high-risk admission requirements.
15. Admission-related suspension and restoration boundaries.
16. Sybil-resistance requirements associated with identity creation.
17. Deterministic replay of admission state.
18. Genesis and legacy-admission classifications.
19. Profile-v0 compatibility requirements for future admission methods.
20. The requirement for deterministic `identity_structural_roots` associated with each canonical identity.
21. The distinction between Anthill relationships and explicit verification artifacts.
22. The minimum restricted verification lane required for `CanonicalAdmittedIdentity` identities.
23. The separation between sponsorship and verification attestation.
24. The requirement that identity verification be represented through challengeable claims and evidence rather than administrative approval.

## 3.2 Excluded

This specification does not fully define:

* human-verification evidence scoring;
* exact VH certainty formulas;
* exact VI certainty formulas;
* exact verification-level thresholds;
* the complete catalog of verification predicates;
* the complete catalog of attestation schemas;
* the complete catalog of verification challenges;
* certainty aggregation;
* challenge resolution;
* ordinary writing-rate formulas;
* ordinary challenge-rate formulas;
* voter selection;
* governance participation;
* Tempo contributor qualification;
* truth certainty outside verification;
* importance weighting;
* POD, POINT, token, mana, or economic issuance;
* private accounts;
* sessions;
* passwords;
* private-key custody;
* key recovery;
* social recovery;
* civil-identity databases;
* biometric identity systems;
* frontend request-pool design;
* relay discovery;
* transport spam economics;
* distributed ceremonies;
* anonymous credential protocols;
* globally bounded self-registration;
* open canonical self-registration;
* AI-based canonical enforcement;
* canonical identity deletion or pruning;
* detailed Anthill user-interface behavior;
* private social lists that do not affect canonical verification;
* the exact event payloads used by the restricted verification lane.

These subjects remain controlled by other specifications or future protocol profiles.

## 3.3 Profile-v0 implementation boundary

A conforming Profile-v0 implementation MUST support only the canonical admission path defined by this specification unless another admission profile has been separately standardized and activated.

Supporting local identity candidates or non-canonical request transport does not activate another canonical admission method.

An implementation MUST NOT introduce an undocumented canonical admission shortcut such as:

* operator-created post-genesis identities;
* direct database insertion;
* self-authored identity creation;
* private allowlisted admission;
* account-created identity state;
* payment-based identity creation;
* computational proof treated as proof of humanity;
* Anthill membership treated as verification;
* social popularity treated as verification;
* sponsor authorship treated as verified-human certification.

## 3.4 Verification-specification boundary

The Verification Specification controls:

* which verification claims are valid;
* which evidence schemas are recognized;
* how attestations are represented;
* how contradictions and challenges are evaluated;
* how VH and VI certainty are derived;
* how verification levels activate;
* how certainty affects event-family eligibility;
* how verification state changes at cycle boundaries;
* how verification claims become ineffective without historical deletion.

This specification controls only the admission-side requirement that those mechanisms remain available after identity creation.

It MUST NOT duplicate or replace the full verification algorithm.

## 3.5 Identity structural-root and Anthill boundary

This specification requires:

* one deterministically identifiable `identity_structural_roots` set for each canonical identity;
* the ability to associate verification artifacts with the subject identity through the Anthill root within that set;
* the separation of structural social relationships from verification claims;
* the visibility of provenance needed to inspect verification derivation.

This specification does not require every private social relationship to become canonical.

It does not define:

* private user lists;
* private relationship notes;
* social-feed behavior;
* relationship recommendation algorithms;
* private identity organization;
* all Anthill edge types.

Private social state MAY exist, but it MUST NOT determine canonical verification unless converted into an explicit authorized canonical artifact.

## 3.6 Restricted verification-lane boundary

This specification requires that `CanonicalAdmittedIdentity` identities have a limited path to participate in their own verification.

Appendix A and the Verification Specification MUST later define:

* the exact allowed event types;
* the exact author constraints;
* the exact subject constraints;
* which challenge rights are available;
* which evidence may be submitted;
* which structural updates are permitted;
* which actions remain prohibited.

The restricted lane MUST be narrow enough that it cannot be used as a substitute for ordinary canonical-writer eligibility.

---

# 4. Terminology

## 4.1 Local identity candidate

A **local identity candidate** is a proposed identity prepared outside canonical state.

It may include:

* a proposed `identity_id`;
* one or more keypairs;
* an initial key descriptor;
* a proposed `public_key_ref`;
* proof-of-possession material;
* a signed admission request;
* local continuity information;
* private recovery information.

A local identity candidate has no canonical existence or authority.

Multiple local candidates MAY exist for the same person before admission.

The protocol does not treat local generation as canonical identity duplication because no canonical identity has yet been created.

## 4.2 Admission request

An **admission request** is a signed, portable, non-canonical request asking an eligible sponsor to authorize canonical identity creation.

An admission request MAY be:

* transmitted directly;
* submitted to a public request pool;
* carried by a relay;
* stored and forwarded;
* prepared offline;
* mirrored by multiple transport providers;
* presented pseudonymously.

An admission request MUST NOT by itself:

* reserve an identity ID canonically;
* create an identity;
* consume invitation capacity;
* establish verification;
* establish lineage;
* grant any canonical action.

## 4.3 Applicant

The **applicant** is the person or human agent seeking canonical identity creation and controlling the proposed initial private key.

Profile v0 does not require the applicant to disclose a civil identity.

The applicant is not the canonical author of the normal `identity_create` event.

The applicant’s cryptographic role is to prove possession of the initial target key.

## 4.4 Sponsor

A **sponsor** is the inviter-eligible human identity that authors a canonical Profile-v0 `identity_create` event.

The sponsor:

* spends invitation capacity;
* authorizes creation of the target identity;
* binds the target identity to the supplied initial key descriptor;
* becomes part of the canonical admission provenance.

The sponsor does not certify:

* the applicant’s legal identity;
* the applicant’s opinions;
* the applicant’s truthfulness;
* the applicant’s uniqueness as a human;
* the applicant’s future conduct;
* the applicant’s VH or VI state.

A sponsor MAY separately submit a verification attestation.

That attestation is distinct from the sponsorship event.

## 4.5 Invitation eligibility

**Invitation eligibility** is the replay-derived state permitting a human identity to sponsor new canonical identities.

Invitation eligibility is distinct from:

* identity existence;
* key control;
* verified-human status;
* ordinary-writer eligibility;
* voter eligibility;
* governance eligibility;
* Tempo eligibility;
* possession of unused invitation capacity.

An identity may satisfy general verification requirements while remaining temporarily ineligible to invite because of maturation, continuity, suspension, or another specified rulebook condition.

## 4.6 Invitation capacity

**Invitation capacity** is the replay-derived quantity limiting how many canonical identity admissions an eligible sponsor may authorize.

Under Profile v0, one successful canonical `identity_create` consumes one unit of invitation capacity.

In each qualifying capacity period, the active rulebook MUST generate at least one spendable unit for each unsuspended inviter-eligible identity.

Invitation capacity is not:

* money;
* a transferable token;
* a saleable right;
* a vote;
* reputation;
* verification certainty;
* epistemic authority.

Invitation capacity exists only to bound canonical identity admission.

A stalled or non-qualifying period does not create new capacity, but it also does not silently destroy capacity generated earlier unless an already applicable expiration rule, active suspension, authorized freeze, or explicit constitutional rule applies.
## 4.7 Admission authorization

**Admission authorization** is the complete replay-validated basis that makes an `identity_create` valid.

It includes:

* the sponsor's canonical identity;
* the sponsor's active key;
* inviter eligibility;
* absence of an active invitation suspension;
* available invitation capacity;
* applicable capacity period;
* applicable rulebook state;
* the target identity;
* the applicant's initial key;
* the applicant's event-bound possession proof;
* the capacity debit.

The `admission_authorization_reference` commits only to the intended admission profile, sponsor, capacity period, and governing admission rulebook.

That reference does not prove current eligibility, active key state, unsuspended status, remaining capacity, target uniqueness, key uniqueness, applicant possession, or successful admission.

Those states remain independently replay-validated at the event's canonical application position.

## 4.8 Canonical admitted identity

A **CanonicalAdmittedIdentity** is an identity that:

* exists in canonical state;
* has Profile-v0 target kind `identity_kind = human`;
* has a valid active initial key;
* has recorded admission provenance;
* has the required `identity_structural_roots`, including the Anthill root;
* is not yet eligible for ordinary canonical participation.

A `CanonicalAdmittedIdentity` MAY perform only those identity-control, identity-structural, and identity-verification actions explicitly allowed by the applicable specifications.

The fixed human target kind records admission through the human identity profile.

It does not establish VH, VI, human uniqueness, ordinary writer eligibility, challenge eligibility, voting eligibility, governance eligibility, Tempo eligibility, inviter eligibility, invitation capacity, or economic authority.

AI, system, tribe, organization, and other non-human identities require other event families or admission profiles.

It does not automatically have authority to:

* create ordinary ideas;
* create ordinary connections;
* issue general challenges;
* vote;
* govern;
* participate in Tempo;
* invite other identities;
* exercise economic authority.

## 4.9 Verification artifact

A **verification artifact** is an ordinary signed, challengeable canonical object or relationship used in a verification role.

It may be:

* a truth claim;
* an evidence idea;
* an evidence claim;
* an evidence connection;
* a contradiction relationship;
* a challenge object;
* a challenge response;
* a challenge outcome;
* a verification attestation;
* a protected-evidence commitment or authorized privacy-preserving outcome.

It may concern:

* human presence;
* identity correspondence;
* account or key control;
* identity continuity;
* duplication;
* compromise;
* fraud;
* another verification predicate defined by the Verification Specification.

A verification artifact records what an identity claims to have observed, experienced, inferred, concluded, supported, opposed, challenged, or resolved.

It does not establish objective truth by declaration.

A verification artifact may contribute to derived VH or VI certainty only under:

* the active verification rulebook;
* supporting and contradicting evidence;
* challenge outcomes;
* boundary activation rules.

## 4.10 Verification claim

A **verification claim** is an ordinary canonical truth claim whose predicate concerns verification and whose subject references one or more identities.

Examples include claims that:

* a real human controls an identity;
* an identity corresponds to a particular human;
* the same human controlled an identity across two periods;
* two identities are controlled by the same person;
* an identity is automated;
* an identity has been compromised.

A verification claim remains a claim.

Its canonical presence does not make it true.

It becomes relevant to VH, VI, or event-family eligibility only through ordinary evidence, contradiction, challenge, outcome, rulebook, and boundary-activation rules.

## 4.11 Verification evidence

**Verification evidence** is ordinary canonical evidence material offered in support of or opposition to a verification claim.

Verification evidence MAY include:

* signed interaction records;
* continuity proofs;
* independently authored attestations;
* challenge outcomes;
* privacy-preserving credential proofs;
* evidence of duplicate control;
* evidence of automation;
* evidence of key continuity or compromise.

The Verification Specification determines which evidence classes are authorized and how they affect certainty.

Verification evidence MUST use the ordinary evidence ontology unless an authoritative specification explicitly defines a constrained profile of that ontology.

It MUST NOT create a parallel verification-only evidence graph.

## 4.12 Verification attestation

A **verification attestation** is an ordinary human-authored verification truth claim under an attestation schema in which an attesting identity states that it observed a verification-relevant:

* interaction;
* procedure;
* credential;
* continuity fact;
* human-presence indication;
* identity-correspondence indication;
* compromise indication;
* duplicate-control indication.

An attestation is attributable testimony, not administrative certification.

It remains challengeable.

It does not directly set VH, VI, writer eligibility, inviter eligibility, voting, governance, Tempo, or economic authority.

It MAY become ineffective prospectively without being deleted from history.

A mutual interaction normally produces separate attestations from each participant rather than one unquestionable shared fact.

## 4.13 Verification challenge

A **verification challenge** is an ordinary canonical challenge, or a constrained identity-verification profile of the ordinary challenge process, concerning:

* a verification claim;
* a verification artifact;
* an attestation;
* verification evidence;
* a derived VH or VI state;
* an identity-continuity claim;
* a duplicate-control claim;
* another authorized verification predicate.

Verification challenges may have distinct event-family authorization, subject, privacy, rate-limit, and response rules.

Those constraints do not create a second challenge system.

A `CanonicalAdmittedIdentity` MAY receive limited rights to respond to verification challenges concerning itself without gaining general challenge authority.

## 4.14 VH

**VH**, or **human-presence verification**, is replay-derived confidence that a real human controls or operates the relevant identity surface.

VH does not by itself establish:

* civil identity;
* legal name;
* uniqueness across all identities;
* correspondence to a particular claimed person;
* ordinary writer eligibility;
* inviter eligibility.

Those require separate rules or VI-related evidence.

## 4.15 VI

**VI**, or **identity-correspondence verification**, is replay-derived confidence that the canonical identity corresponds to the particular human asserted by the relevant verification claims.

VI is distinct from VH.

An identity may have evidence that a human controls it without sufficient evidence establishing which particular human that is.

Civil identity documents MAY contribute optional evidence, but they are not universally required.

## 4.16 Anthill

An **Anthill** is the identity-associated structural hub used to organize and display human relationships, verification artifacts, and verification provenance concerning a canonical identity.

Under Profile v0 the Anthill is a specialized identity structural root within `identity_structural_roots`, not the entire required root set.

The Anthill MAY provide a navigable web through which users inspect:

* which identities acknowledge relationships;
* which identities submitted attestations;
* which claims support or contradict VH;
* which claims support or contradict VI;
* which challenges affect those claims;
* which continuity claims exist;
* which admission relationships are recorded;
* how verification certainty was derived.

The Anthill provides topology, organization, and legibility only.

Anthill membership, connection count, or social position is not itself verification evidence.

A social connection contributes to verification only when an identity submits an explicit authorized verification artifact describing what it claims to know or have observed.

## 4.17 Anthill anchor

An **Anthill anchor** is the deterministically identifiable canonical structural location or reference associated with an identity’s Anthill root.

The anchor may be:

* an explicit structural object created with the identity; or
* a deterministic derivation from `identity_id`.

Appendix A and the structural specifications MUST choose one canonical rule as part of the complete `identity_structural_roots` reconciliation.

Every canonical identity MUST have the complete protocol-defined `identity_structural_roots` set under the active profile.

Within that set, each identity MUST have exactly one authoritative Anthill anchor unless a later structural-role reconciliation adopts a different explicit Anthill representation.

## 4.18 Restricted verification lane

A **restricted verification lane** is the limited set of event families available to a `CanonicalAdmittedIdentity` before ordinary canonical-writer eligibility.

The lane exists so the identity can participate in its own verification without receiving general canonical-writing authority.

The lane MAY include authorized events for:

* self-correspondence;
* continuity;
* key control;
* response to verification attestations;
* response to identity-scoped challenges;
* submission of identity-verification evidence;
* dispute of verification artifacts concerning the identity;
* identity key management.

The lane MUST NOT include ordinary ideas, arbitrary connections, general challenges, votes, governance, Tempo, invitations, or economic events.

Restricted verification-lane events are authorization paths into ordinary canonical truth, evidence, contradiction, and challenge objects or relationships.

They MUST NOT create opaque verification-only records disconnected from ordinary epistemic semantics.

## 4.19 Admission lineage

**Admission lineage** is the canonical provenance relationship between a sponsor and an identity created through that sponsor’s invitation capacity.

Admission lineage MAY inform:

* detection of concentrated sponsorship patterns;
* diversity calculations;
* diminishing returns for tightly connected verification evidence;
* investigation of coordinated admission abuse;
* future invitation-capacity restrictions.

Admission lineage MUST NOT by itself determine:

* guilt;
* truth;
* importance;
* vote weight;
* political legitimacy;
* verified-human status;
* automatic exclusion;
* retroactive invalidation.

## 4.20 Social relationship

A **social relationship** is a canonical or private relationship between identities indicating an acknowledged social connection or organizational relationship.

A social relationship is not automatically:

* a verification attestation;
* evidence of human presence;
* evidence of identity correspondence;
* evidence of trustworthiness;
* evidence of truth;
* a grant of authority.

An identity wishing to make a verification-relevant claim about another identity MUST create a distinct verification artifact.

## 4.21 Coordinated admission abuse

**Coordinated admission abuse** is deliberate conduct intended to circumvent identity-admission scarcity or create fraudulent human participation at scale.

Examples may include:

* knowingly sponsoring automated identities;
* operating a Sybil invitation ring;
* selling invitation capacity;
* transferring control of invitation authorizations;
* coordinating false human-verification evidence;
* repeatedly creating identities controlled by one actor;
* bypassing capacity limits through collusion;
* coercing or compromising inviters.

Ordinary disagreement, controversial speech, isolated misconduct, or failure by an invitee to attain higher verification is not coordinated admission abuse by itself.

## 4.22 Genesis-admitted identity

A **genesis-admitted identity** is an identity included through an authoritative, versioned genesis state rather than a normal post-genesis `identity_create` event.

Genesis admission MUST be:

* deterministic;
* publicly inspectable;
* replay-compatible;
* clearly marked as genesis provenance.

Genesis-admitted identities do not receive permanent special admission authority merely because of genesis status.

## 4.23 Event-derived identity

An **event-derived identity** is a canonical identity created by a valid post-genesis `identity_create` event.

Replay must be able to reconstruct:

* the identity;
* the sponsor;
* the creation position;
* the initial key;
* the admission profile;
* the capacity debit;
* `identity_structural_roots`, consisting of Mindgarden, Backyard of Relationships, Self Tree, and Anthill;
* the identity’s initial ordinary-authority inactive status.

## 4.24 Legacy operator-provisioned identity

A **legacy operator-provisioned identity** is an identity admitted by a pre-Profile-v0 or implementation-specific process that lacks complete event-derived admission provenance.

Such identities MAY remain readable for compatibility.

They MUST NOT be falsely represented as event-derived identities.

No implementation may fabricate:

* a sponsor;
* an admission event;
* a capacity debit;
* an applicant possession proof;
* a canonical lineage;
* a verification claim;
* identity structural-root history

to make legacy state appear Profile-v0 compliant.

# 5. Admission architecture

## 5.1 Architectural layers

Profile v0 separates identity admission into three authority layers:

1. local identity preparation;
2. non-canonical admission transport;
3. canonical identity creation.

These layers MUST remain distinct in specification, replay, storage, and implementation.

A single application MAY assist with all three layers, but application integration MUST NOT collapse their authority boundaries.

The layers are:

```text
local/private preparation
→ non-canonical request transport
→ canonical sponsor-authored admission
```

Only the final layer may create canonical identity state.

## 5.2 Local identity-preparation layer

The local identity-preparation layer includes:

* generating a proposed identity ID;
* generating one or more keypairs;
* constructing the initial public-key descriptor;
* calculating the initial `public_key_ref`;
* preparing proof-of-possession material;
* preparing a signed admission request;
* storing private identity-continuity material;
* storing private recovery material where supported.

This layer is permissionless.

No canonical identity, sponsor, node, operator, institution, government, or governance body may prevent a person from preparing identity material locally.

Local preparation creates no canonical rights, obligations, identity state, verification state, or admission lineage.

A local identity candidate is not required to be visible to any node or relay.

## 5.3 Non-canonical admission-transport layer

The non-canonical admission-transport layer carries admission requests between:

* applicants;
* relays;
* interfaces;
* request pools;
* potential sponsors;
* verification-support services.

This layer MAY include:

* direct person-to-person exchange;
* public request pools;
* privacy-preserving request pools;
* randomized request presentation;
* federated relays;
* offline packages;
* store-and-forward delivery;
* encrypted one-way communication;
* local community interfaces;
* mirrored request queues.

Admission transport is not canonical consensus.

A transport provider MAY decide which requests it:

* stores;
* forwards;
* indexes;
* presents;
* deprioritizes;
* expires;
* locally rate-limits.

These transport decisions have no canonical effect.

A transport provider MUST NOT thereby gain authority to:

* create a canonical identity;
* establish verified-human status;
* establish VI;
* establish inviter eligibility;
* consume invitation capacity;
* create admission lineage;
* deny an applicant access through all other transports;
* determine the validity of a future `identity_create` event.

An applicant MAY submit the same signed request through multiple independent transports.

## 5.4 Canonical-admission layer

The canonical-admission layer begins only when an eligible sponsor submits a valid signed `identity_create` authored candidate for canonical publication.

Canonical identity creation occurs only if all required validation conditions succeed at the event’s canonical position.

These conditions include:

* valid canonical encoding;
* valid sponsor signature;
* valid sponsor identity state;
* valid sponsor active-key state;
* valid inviter eligibility;
* sufficient invitation capacity;
* valid target identity;
* valid initial key descriptor;
* valid initial public-key reference;
* valid applicant key-possession proof;
* valid admission authorization;
* successful duplicate and conflict checks.

Only the canonical-admission layer may create:

* canonical identity existence;
* canonical sponsor provenance;
* canonical admission lineage;
* canonical initial-key state;
* canonical `identity_structural_roots` state, including Anthill root state;
* canonical invitation-capacity consumption;
* canonical admitted identity status.

Local or transport-layer state MUST NOT substitute for any of these effects.

## 5.5 Trust boundary

Relays, request pools, interfaces, and private products are untrusted transport participants for canonical purposes.

They MAY perform local functions such as:

* request-format checking;
* signature preflight;
* duplicate suppression;
* spam throttling;
* request-size enforcement;
* request expiration;
* sponsor-applicant matching;
* encrypted messaging;
* local moderation;
* local language or accessibility support.

These functions do not establish canonical validity.

A request rejected by one relay MAY be accepted by another.

A request hidden by one interface remains usable through another compatible transport.

No relay or interface may create a globally binding denial of admission unless that denial results from an independently authorized canonical rule.

Transport anti-spam systems MUST NOT be interpreted as proof that an applicant is human.

For example:

* proof of work may limit request spam;
* a CAPTCHA may protect a request pool;
* payment may fund relay operation;
* a local account may prioritize a queue;
* a local reputation score may help a sponsor triage requests.

None of these establishes:

* VH;
* VI;
* canonical admission authorization;
* canonical sponsor eligibility;
* canonical identity existence.

## 5.6 Canonical decision point

The canonical decision point is the deterministic application of a valid `identity_create` event.

Before that point:

* the applicant is not a canonical identity;
* the proposed identity ID is not canonically reserved;
* no invitation capacity has been canonically consumed;
* no canonical admission lineage exists;
* no initial key is active in canonical state;
* no canonical `identity_structural_roots` exist for the applicant;
* no ordinary writer or inviter eligibility exists for the applicant.

At that point, validation MUST evaluate the sponsor and applicant against canonical state immediately before the event’s canonical position.

If valid, the following effects MUST occur atomically:

1. create the canonical identity;
2. register the initial key descriptor;
3. activate the initial key;
4. establish canonical sponsor provenance;
5. establish canonical admission lineage;
6. establish or deterministically derive the complete protocol-defined `identity_structural_roots`, including the Anthill root;
7. consume invitation capacity exactly once;
8. establish canonical admitted identity status with ordinary participation lanes inactive;
9. initialize the restricted verification lane for the new identity.

If any required validation fails, none of these effects may occur.

## 5.7 Atomicity

An implementation MUST NOT partially apply an admission event.

It MUST NOT:

* create the identity without consuming capacity;
* consume capacity without creating the identity;
* activate the key without creating the identity;
* create admission lineage without creating the identity;
* create only some required identity structural roots or create a structural root without creating the identity;
* grant ordinary eligibility as a side effect;
* create private account state as part of canonical admission.

The event append and all canonical materialization effects MUST commit or fail as one deterministic transition.

## 5.8 Request acceptance versus canonical acceptance

A sponsor MAY agree to sponsor an applicant before canonical publication.

That agreement is not canonical acceptance.

A signed but unpublished `identity_create` candidate is not yet a canonical identity-creation event.

The sponsor’s state may change before publication. For example:

* its remaining invitation capacity may be consumed;
* inviter eligibility may be suspended;
* its active key may rotate;
* the target identity ID may be taken by an earlier event;
* the applicant key may become conflicting canonical state.

The event MUST therefore be validated against canonical state at its actual application position, not solely against the state that existed when:

* the request was submitted;
* the sponsor first reviewed it;
* the sponsor agreed privately;
* the sponsor signed the candidate;
* a relay accepted the candidate.

## 5.9 No hidden reservation authority

Profile v0 does not recognize private, interface-local, or node-local reservation of invitation capacity as canonical authority.

A sponsor wallet or interface MAY display capacity as tentatively assigned to a pending request.

That assignment is advisory only.

Canonical capacity is consumed only when a valid `identity_create` event is applied.

Two pending candidates MAY compete for the same remaining capacity.

Their result is determined by canonical order.

A future profile MAY define canonical reservation semantics. Profile v0 does not imply them.

## 5.10 Admission and verification separation

Canonical admission establishes:

* identity existence;
* initial key control;
* sponsor provenance;
* admission lineage;
* complete `identity_structural_roots`, including Anthill anchoring;
* access to the restricted verification lane.

Canonical admission does not establish:

* VH;
* VI;
* uniqueness of the human controller;
* ordinary writer eligibility;
* general challenge eligibility;
* voter eligibility;
* governance eligibility;
* Tempo eligibility;
* inviter eligibility;
* economic authority.

A sponsor MAY later submit a verification attestation concerning the applicant.

Such an attestation is a separate canonical verification artifact.

The sponsor’s `identity_create` signature MUST NOT be reused or interpreted as the attestation itself.

## 5.11 Identity structural-root and Anthill integration

Every newly created canonical identity MUST have the complete protocol-defined `identity_structural_roots` set.

The complete root set MUST be created or derived as part of the atomic identity-creation transition.

Failure to establish any required root MUST fail the complete admission transition.

The Anthill remains one specialized root in this set.

The Anthill MAY organize:

* verification claims;
* verification attestations;
* verification evidence;
* verification challenges;
* continuity claims;
* admission lineage;
* mutually acknowledged social relationships.

The Anthill MUST NOT itself calculate or confer VH or VI.

An Anthill relationship MAY make verification evidence easier to discover or inspect.

It MUST NOT become verification evidence unless an identity submits an explicit verification artifact under an authorized schema.

## 5.12 Admission-profile boundary

Profile v0 activates only sponsored public admission.

A conforming implementation MUST NOT treat any of the following as an additional active canonical admission path:

* unrestricted self-registration;
* operator-created identity insertion;
* private allowlisted creation;
* payment-based registration;
* proof-of-work registration;
* proof-of-reasoning admission;
* biometric-provider admission;
* government-document admission;
* private product-account creation;
* AI-selected admission;
* relay-selected admission.

These mechanisms MAY assist transport or contribute optional verification evidence where authorized.

They do not authorize canonical identity creation under Profile v0.

---

# 6. Admission state machine

## 6.1 Purpose

The admission state machine distinguishes:

* local state;
* transport state;
* canonical identity state;
* later eligibility state.

Only canonical states and canonical events affect deterministic replay.

Local and transport implementations MAY use different internal names, but they MUST preserve the authority boundaries defined here.

## 6.2 State overview

Profile v0 recognizes the following conceptual progression:

```text
LocalCandidate
→ AdmissionRequest
→ SponsoredPendingPublication
→ CanonicalAdmittedIdentity
→ independently derived orthogonal status lanes
```

The first three states are non-canonical.

`CanonicalAdmittedIdentity` is canonical.

Later eligibility, suspension, dormancy, and recovery outputs are replay-derived orthogonal lanes rather than mutually exclusive lifecycle stages.

At minimum, replay MUST distinguish these lanes or predicates:

```text
canonical_existence
identity_kind
key_control_state
identity_structural_root_state
verification_state
restricted_verification_lane_eligibility
ordinary_writer_eligibility
ordinary_challenge_eligibility
voter_eligibility
governance_eligibility
tempo_eligibility
inviter_eligibility
invitation_capacity_balance
invitation_suspension
identity_dormancy_or_recovery_state
```

An identity may simultaneously be writer-eligible, voter-eligible, Tempo-eligible, inviter-eligible, and subject to a separate invitation suspension when the applicable lane rules produce those outputs.

## 6.3 `LocalCandidate`

`LocalCandidate` describes locally prepared identity material.

It may contain:

* a proposed identity ID;
* one or more keypairs;
* an initial key descriptor;
* a public-key reference;
* request-stage possession proofs;
* private continuity information;
* private recovery information.

A `LocalCandidate`:

* is not globally reserved;
* is not a canonical identity;
* has no canonical authorizations;
* may be replaced or deleted locally;
* may be stored offline;
* may be duplicated across the applicant’s own devices.

The protocol does not enumerate local identity candidates.

## 6.4 `AdmissionRequest`

`AdmissionRequest` describes a signed non-canonical request presented to potential sponsors.

An admission request MAY be:

* public;
* encrypted;
* pseudonymous;
* addressed to a particular sponsor;
* addressed to a request pool;
* transported by a relay;
* prepared offline.

An admission request does not establish that:

* the applicant is human;
* the applicant is a unique human;
* the applicant corresponds to any civil identity;
* a sponsor has agreed;
* invitation capacity is available;
* the identity will become canonical.

## 6.5 `SponsoredPendingPublication`

`SponsoredPendingPublication` describes a non-canonical state in which a sponsor has prepared or signed an `identity_create` candidate that has not yet been canonically applied.

This state does not guarantee acceptance.

The candidate may fail because:

* the sponsor is no longer eligible;
* capacity has been consumed elsewhere;
* the sponsor key is no longer active;
* the target identity already exists;
* the public key conflicts with canonical state;
* the applicant proof is invalid;
* the event is malformed;
* an earlier canonical event resolves the conflict.

No permanent canonical effect occurs until application succeeds.

## 6.6 `CanonicalAdmittedIdentity`

A `CanonicalAdmittedIdentity`:

* exists canonically;
* has Profile-v0 target kind `identity_kind = human`;
* has canonical sponsor provenance;
* has canonical admission lineage;
* has an active initial key;
* has the required `identity_structural_roots`, including the Anthill root;
* has access to the restricted verification lane;
* is not eligible for ordinary canonical participation.

This is the required initial state of a valid post-genesis Profile-v0 admission.

The identity may perform only explicitly authorized identity-control, identity-structural, and identity-verification actions.

## 6.7 `ordinary_writer_eligibility`

`ordinary_writer_eligibility` is the replay-derived lane indicating that a canonical identity has independently satisfied the requirements for one or more ordinary canonical event families.

Writer eligibility is not granted by admission.

It is derived through:

* verification claims and evidence;
* VH and, where applicable, VI;
* challenge outcomes;
* the active verification rulebook;
* event-family-specific eligibility rules;
* applicable cycle-boundary activation.

Writer eligibility is not necessarily equivalent to:

* voter eligibility;
* governance eligibility;
* Tempo eligibility;
* inviter eligibility.

## 6.8 `inviter_eligibility`

`inviter_eligibility` is the replay-derived lane indicating that a canonical human identity satisfies the active rules for sponsoring new identities.

Inviter eligibility MAY depend on:

* sufficient VH;
* sufficient identity continuity;
* VI where required by the active rulebook;
* maturation across qualifying certified cycles;
* active key control;
* sufficiently diverse verification evidence;
* absence of unresolved severe identity challenges;
* absence of active admission suspension.

Inviter eligibility and invitation capacity are separate replay-derived states.

However, inviter eligibility MUST have practical effect: during each qualifying capacity period, every unsuspended inviter-eligible identity MUST receive at least one spendable invitation-capacity unit.

An inviter-eligible identity may have zero available invitation capacity only when:

* previously generated capacity was consumed;
* capacity expired under a rule already applicable before the current stalled or non-qualifying period;
* the identity is suspended;
* an authorized emergency rule freezes spending;
* no qualifying capacity period exists and `admission_liveness_blocked = true`.
## 6.9 `invitation_suspension`

`invitation_suspension` is the replay-derived lane indicating that an identity's ability to generate or spend invitation capacity has been restricted under an authorized canonical rule.

Suspension MUST be:

* replay-derived;
* based on canonical inputs;
* challengeable where required;
* prospective rather than retroactively destructive;
* subject to defined review or restoration.

Suspension from inviting does not automatically:

* erase the identity;
* revoke its keys;
* invalidate earlier admissions;
* remove ordinary writer eligibility;
* remove voter eligibility;
* remove governance eligibility;
* alter VH or VI;
* alter truth or importance weight.

Any such additional effect requires separate authority.

## 6.10 `identity_dormancy_or_recovery_state`

`identity_dormancy_or_recovery_state` is the replay-derived lane indicating that an identity remains part of canonical history but has a current dormancy, key-loss, compromise, or recovery condition under a rule defined elsewhere.

Dormancy may result from:

* lack of an active usable key;
* unresolved continuity state;
* voluntary dormancy;
* prolonged inactivity;
* another rulebook-defined condition.

Dormancy does not delete the identity, its `identity_structural_roots`, its Anthill root, its admission lineage, its key history, or its authored events.

## 6.11 Valid canonical transition

The canonical admission transition is:

```text
SponsoredPendingPublication
→ CanonicalAdmittedIdentity
```

This transition occurs only through successful application of `identity_create`.

The following later lane activations are governed by other specifications:

```text
CanonicalAdmittedIdentity
→ ordinary_writer_eligibility = active
```

and:

```text
CanonicalAdmittedIdentity
→ inviter_eligibility = active
```

The protocol MUST NOT assume that all forms of eligibility are one linear hierarchy.

Each eligibility lane may have distinct predicates.

Convenience labels such as "ordinary-authority inactive", "invitation suspended", or "dormant" MAY be used in UI or explanatory text only as summaries of these orthogonal derived predicates.

They MUST NOT be treated as mutually exclusive authoritative lifecycle states.

## 6.12 Restricted verification-lane transition

Upon canonical creation, the new identity becomes eligible only for the restricted verification lane defined by Appendix A and the Verification Specification.

This lane MAY permit the identity to:

* prove current key control;
* make an authorized self-correspondence claim;
* make continuity claims;
* submit verification evidence concerning itself;
* acknowledge an attestation;
* respond to an identity-specific challenge;
* dispute a verification artifact concerning itself;
* rotate or revoke keys under the key-lifecycle rules;
* maintain required `identity_structural_roots`, including Anthill state, where authorized.

The verification claims, evidence, contradiction relationships, challenge responses, and outcomes created or related through this lane remain ordinary canonical objects under constrained authorization.

The lane MUST NOT create a separate verification-only object model.

This lane MUST NOT transition the identity into ordinary writer eligibility merely because an event was accepted.

## 6.13 Invalid transitions

The following transitions are invalid under Profile v0.

### 6.13.1 Local candidate directly to canonical identity

A local candidate MUST NOT become canonical solely because it:

* possesses a valid key;
* publishes a request;
* passes a CAPTCHA;
* pays a fee;
* appears in a relay;
* creates a private account;
* receives an AI approval score.

### 6.13.2 Self-authored normal admission

The target identity MUST NOT author its own normal Profile-v0 `identity_create`.

The applicant’s possession proof is not sponsorship.

### 6.13.3 AI- or system-sponsored admission

AI identities, system identities, and constrained emitters MUST NOT sponsor normal human admission.

### 6.13.4 Operator-created admission

A node operator MUST NOT create a post-genesis canonical identity through direct database insertion or administrative privilege.

### 6.13.5 Capacity-free admission

A post-genesis Profile-v0 identity MUST NOT be created without successful invitation-capacity consumption.

### 6.13.6 Automatic verification

Identity creation MUST NOT automatically create:

* a VH claim result;
* a VI claim result;
* a raw verification artifact with self-executing status effect;
* a verification-level increase;
* writer eligibility;
* inviter eligibility.

Accepted verification artifacts after admission remain inputs to later derivation; they do not directly update verification status.

### 6.13.7 Automatic participation activation

Identity creation MUST NOT automatically grant:

* ordinary idea creation;
* ordinary connection creation;
* general challenge authority;
* voting;
* governance;
* Tempo participation;
* economic authority.

### 6.13.8 Structural-root and Anthill-based automatic verification

Creation of `identity_structural_roots`, an Anthill anchor, an Anthill relationship, or an admission-lineage edge MUST NOT automatically change VH or VI.

### 6.13.9 Same-cycle recursive invitation authority

A newly admitted identity MUST NOT generate invitation capacity or become inviter-eligible in the same canonical transition that created it.

## 6.14 Atomic transition requirements

A valid transition into `CanonicalAdmittedIdentity` MUST atomically:

1. create the identity;
2. register the initial key descriptor;
3. activate the initial key;
4. record the sponsor;
5. record the admission profile;
6. record admission lineage;
7. create or derive the complete protocol-defined `identity_structural_roots`, including the Anthill root;
8. debit invitation capacity exactly once;
9. initialize ordinary participation lanes as inactive;
10. initialize restricted verification-lane eligibility.

If any step fails, the complete transition fails.

## 6.15 Arrival-order independence

Admission results MUST depend only on:

* canonical ordering;
* canonical state;
* admitted genesis state;
* active rulebook state.

They MUST NOT depend on:

* which relay received a request first;
* which node first observed the event;
* private request-pool ordering;
* wall-clock submission time;
* private sponsor notes;
* opaque AI rankings.

---

# 7. Non-canonical admission requests

## 7.1 Purpose

An admission request allows a person to seek sponsorship without first becoming a canonical identity.

It provides a portable object that can be:

* verified by potential sponsors;
* transported through independent relays;
* prepared offline;
* presented pseudonymously;
* reused across compatible interfaces.

Keeping admission requests non-canonical reduces:

* permanent storage pressure;
* canonical identity spam;
* dependence on central registration services;
* the cost of failed or abandoned requests.

## 7.2 Required semantic content

A standardized Profile-v0 admission request SHOULD bind at least:

* request format version;
* proposed `identity_id`;
* initial key descriptor;
* proposed `initial_public_key_ref`;
* applicant request-stage possession proof;
* supported admission profile;
* optional expiration or freshness value;
* optional response or relay information;
* optional verification-material references.

The exact request encoding MAY be specified in a subordinate non-canonical transport profile.

Any standardized encoding MUST be:

* deterministic;
* versioned;
* domain-separated from canonical events;
* independent of private account identifiers;
* verifiable without the applicant’s private key.

An admission request MUST NOT include private-key material.

## 7.3 Request-stage proof of possession

An admission request SHOULD include proof that the requester controls the private key corresponding to the proposed initial public key.

This enables sponsors and relays to reject requests that attempt to bind an identity to a key not controlled by the applicant.

A request-stage proof may differ from the final canonical possession proof because the final proof may bind:

* the final `identity_create` event ID;
* the final sponsor identity;
* the final admission authorization.

A request-stage proof alone MUST NOT authorize canonical creation.

## 7.4 Non-effects

Creating, signing, transporting, storing, indexing, or accepting an admission request MUST NOT:

* create a canonical identity;
* reserve an identity ID canonically;
* reserve a key canonically;
* consume invitation capacity;
* establish VH;
* establish VI;
* establish continuity;
* establish sponsor lineage;
* create any required identity structural root;
* grant restricted or ordinary canonical eligibility;
* create a canonical rejection record.

A relay’s statement that a request is “accepted” means only that the relay accepted it for local processing.

## 7.5 Transport methods

Admission requests MAY be transported through:

* direct encrypted communication;
* public bulletin systems;
* privacy-preserving pools;
* randomized matching services;
* local community networks;
* federated relays;
* offline media;
* delayed store-and-forward systems;
* censorship-resistant publication channels.

No single transport provider is authoritative.

An applicant SHOULD be able to submit the same signed request through multiple transports without changing the proposed identity or initial key.

Transport systems MAY wrap the request in transport-specific envelopes.

Those envelopes MUST NOT alter the signed request payload.


## 7.6 Request pools

A request pool is a non-canonical service that presents requests to potential sponsors.

A request pool MAY:

* sort requests;
* randomize presentation;
* group requests by language;
* provide accessibility support;
* filter malformed requests;
* suppress duplicates;
* apply local rate limits;
* expire stale requests;
* protect sponsors from direct contact;
* provide one-way encrypted communication.

A request pool MUST NOT:

* create canonical identity state;
* spend sponsor capacity;
* establish inviter eligibility;
* establish VH or VI;
* represent local ranking as canonical priority;
* prevent the applicant from using another pool;
* require ideological conformity as a protocol condition.

A pool MAY have local moderation rules.

Those rules do not bind other pools or canonical replay.

## 7.7 Stranger sponsorship

An applicant MUST NOT be required to prove a pre-existing relationship with a sponsor.

A sponsor MAY select an applicant from a public or private request pool without prior acquaintance.

The protocol MUST NOT require the sponsor to assert that:

* the sponsor knows the applicant’s legal identity;
* the sponsor met the applicant physically;
* the sponsor agrees with the applicant;
* the sponsor guarantees uniqueness;
* the sponsor guarantees future behavior.

A sponsor MAY seek additional evidence before spending invitation capacity.

Such local sponsor practices MUST NOT become undeclared protocol requirements.

## 7.8 Verification material in admission requests

An applicant MAY attach or reference optional material relevant to future verification.

Such material may include:

* claims about human presence;
* claims about identity correspondence;
* continuity evidence;
* private high-risk evidence;
* optional credential proofs;
* references to prior interactions.

Admission-request material does not automatically become canonical verification evidence.

For material to affect VH or VI, it MUST later become or support an authorized verification artifact under the Verification Specification.

A sponsor’s review of private evidence does not itself convert that evidence into canonical truth.

## 7.9 Request privacy

An admission request MUST NOT universally require:

* legal name;
* government ID;
* exact physical location;
* home address;
* permanent contact information;
* source IP address;
* political views;
* religion;
* intended future claims;
* complete social graph;
* biometric data.

An applicant MAY voluntarily provide sensitive evidence through authorized privacy-preserving mechanisms.

Private evidence MUST NOT be copied into `identity_create` unless another authoritative specification explicitly requires a safe canonical commitment.

## 7.10 Relay privacy

Relays SHOULD minimize collection and retention of:

* source network address;
* location;
* device identifiers;
* contact metadata;
* request-routing history;
* social relationships;
* private verification materials.

Relay metadata is not canonical.

A relay is not the sponsor merely because it transported a request.

A relay is not the applicant merely because it submitted the request to a sponsor or node.

## 7.11 Spam and denial-of-service controls

Non-canonical request systems MAY use local anti-spam measures such as:

* connection limits;
* request-size limits;
* proof of work;
* temporary deposits;
* local account limits;
* queue limits;
* duplicate detection;
* CAPTCHAs;
* request expiration;
* human moderation.

These measures are transport controls.

They MUST NOT be treated as:

* proof of humanity;
* canonical verification evidence by default;
* canonical admission authorization;
* universal requirements across all transports.

An applicant blocked by one transport MAY use another.

## 7.12 No proof-of-reasoning admission

Admission requests MUST NOT be required to demonstrate intellectual sophistication, ideological conformity, rhetorical quality, or reasoning performance as proof of humanity.

Reasoning quality is not a reliable Sybil defense because:

* AI systems can produce persuasive reasoning;
* language and education differences would create exclusion;
* reviewers could discriminate based on viewpoint;
* human existence is not equivalent to argument quality.

An applicant may later submit ideas after becoming eligible under ordinary writer rules.

Reasoning quality does not determine canonical identity existence.

## 7.13 Request freshness and expiration

A standardized admission request MAY include expiration or freshness fields to reduce indefinite replay of abandoned requests.

Expiration affects only transport validity.

It does not affect an already finalized canonical identity.

The final `identity_create` event MUST bind the final target identity and key regardless of earlier request expiration semantics.

## 7.14 Duplicate and conflicting requests

Multiple requests may propose:

* the same identity ID;
* the same public key;
* different keys for one identity ID;
* the same key for different identities.

Transport systems MAY flag these conflicts.

Only canonical event application determines accepted state.

Non-canonical request order has no canonical effect.

## 7.15 Request retention

Profile v0 does not require universal permanent retention of:

* rejected requests;
* expired requests;
* abandoned requests;
* unsponsored requests.

Transport providers MAY retain requests under local policy, subject to privacy rules.

The protocol SHOULD favor expiration or deletion of unnecessary high-risk request metadata.

The disappearance of one copy does not invalidate another copy or the applicant’s local candidate.

## 7.16 Censorship resistance

Admission-request transport SHOULD support:

* portable signed requests;
* multiple independent relays;
* offline transport;
* mirrorable public requests where safe;
* encrypted delivery;
* sponsor discovery outside one social network;
* store-and-forward publication.

No applicant should depend on one operator or one relay for all possible admission.

This does not guarantee sponsorship.

It prevents one mandatory transport chokepoint.

---

# 8. Sponsor eligibility

## 8.1 Sponsor role

The sponsor is the canonical human author of a Profile-v0 `identity_create` event.

The sponsor spends one unit of invitation capacity to authorize creation of the target identity.

The sponsor’s role is narrower than verification.

The sponsor does not establish that the applicant:

* is already verified;
* is globally unique;
* corresponds to a disclosed civil identity;
* deserves writing authority;
* deserves challenge authority;
* deserves voting or governance authority;
* will behave correctly in the future.

The sponsor authorizes entry into canonical identity state and the verification process.

## 8.2 Required sponsor conditions

Immediately before the `identity_create` event’s canonical position, the sponsor MUST:

1. exist as a canonical identity;
2. be classified as a human identity;
3. control an active key authorized for admission events;
4. satisfy inviter eligibility under the active rulebook;
5. possess at least one available invitation-capacity unit;
6. not be under an active invitation suspension;
7. satisfy event-family-specific conditions defined by Appendix A;
8. differ from the target identity;
9. sign the exact authored-candidate bytes.

All conditions are evaluated at canonical application time.

A sponsor who was eligible when signing but became ineligible before application cannot authorize the event.

## 8.3 Sponsorship is not verification attestation

The sponsor’s signature means:

> The sponsor authorizes the use of one unit of invitation capacity to create the target canonical identity and bind its initial key.

It does not mean:

> The sponsor certifies that the target is a unique real human or corresponds to a particular civil person.

Sponsorship MUST NOT automatically contribute to VH or VI.

A sponsor MAY submit a separate verification attestation if the sponsor has verification-relevant evidence.

That attestation MUST:

* use an authorized verification schema;
* identify the subject identity;
* state the specific observation or claim;
* remain challengeable;
* be evaluated independently of sponsorship.

## 8.4 Eligibility versus capacity

Inviter eligibility and invitation capacity are distinct.

Inviter eligibility is the status lane. Invitation capacity is the spendable balance.

A rulebook MUST NOT make inviter eligibility merely nominal by assigning zero capacity indefinitely to otherwise eligible and unsuspended human identities.

In each qualifying capacity period:

```text
generated_capacity >= 1
```

for every unsuspended inviter-eligible identity.

An identity may be inviter-eligible but have zero available capacity because:

* all previously generated capacity was consumed;
* no qualifying capacity period exists and `admission_liveness_blocked = true`;
* rollover was exhausted;
* capacity expired under an already applicable rule;
* capacity is frozen by an authorized emergency rule;
* a capacity-specific restriction or suspension applies.

An identity may have a positive historical balance but be unable to spend it while invitation eligibility is suspended.

A valid admission requires both:

```text
inviter_eligible = true
```

and:

```text
available_invitation_capacity >= 1
```
## 8.5 Human requirement

Only human identities may sponsor ordinary human identity creation under Profile v0.

The following MUST NOT act as sponsor:

* AI identities;
* system identities;
* automated agents;
* tribes;
* organizations represented only as non-human identities;
* private product accounts;
* node operators acting without a canonical human identity;
* `system_boundary_emitter`;
* non-canonical identity candidates.

An organization MAY facilitate sponsorship through human participants.

The canonical sponsor must be one eligible human identity.

## 8.6 Active-key requirement

The sponsor’s authored candidate MUST be signed by a key active for the sponsor immediately before the event’s canonical position.

A signature from a key that is:

* unknown;
* inactive;
* superseded;
* revoked;
* owned by another identity;
* valid only after the event position

MUST be rejected.

Later rotation or revocation does not retroactively invalidate a valid admission.

## 8.7 General availability of inviter eligibility

Inviter eligibility MUST be attainable by every human identity satisfying the same objective requirements.

The protocol MUST NOT define inviter eligibility as membership in a permanent fixed class.

Permitted eligibility inputs MAY include:

* VH certainty;
* VI or identity-correspondence evidence where required;
* identity-continuity certainty;
* survival across certified cycles;
* active key control;
* diversity of verification evidence;
* resolved identity challenges;
* absence of active admission-abuse suspension;
* applicable good-standing requirements.

The same rules MUST apply without regard to:

* political viewpoint;
* popularity;
* wealth;
* geography;
* cultural group;
* institutional affiliation;
* genesis status;
* node-operator status.

## 8.8 Maturation requirement

The active rulebook MUST require maturation before inviter eligibility.

Maturation MUST be measured through qualifying certified human-deliberative cycles, not wall-clock waiting.

Dmax-only, forced, degraded, survivor, record-only, or machine-only boundaries MUST NOT advance Profile-v0 invitation maturation unless they separately satisfy the required human-deliberative certification rules.

A newly admitted identity MUST NOT become inviter-eligible in the same admission transition.

A newly admitted identity MUST NOT generate invitation capacity in the same admission transition or cycle unless a future profile explicitly defines otherwise.
## 8.9 Evidence diversity

Inviter eligibility MAY require diverse verification evidence.

Diversity calculations SHOULD reduce the effect of evidence concentrated within:

* one admission lineage;
* one tightly connected social cluster;
* one verification provider;
* one organization;
* one geographic or institutional source.

Diversity MUST NOT be implemented as an absolute barrier that makes isolated or censored populations impossible to verify.

The rulebook SHOULD allow multiple evidence combinations.

Cross-lineage evidence is a Sybil-resistance mechanism.

It is not a requirement that every human join a dominant social network.

## 8.10 Anthill evidence boundary

The sponsor’s Anthill connections MAY expose relevant relationships and prior attestations.

Anthill structure alone MUST NOT establish sponsor eligibility.

Sponsor eligibility MUST derive from explicit canonical state, including:

* verification artifacts;
* challenge outcomes;
* maturation state;
* active rulebook predicates;
* active key state;
* admission-suspension state.

A sponsor with many Anthill connections is not automatically more eligible than one with few connections.

## 8.11 Sponsor-target separation

The sponsor and target identity MUST differ.

An identity MUST NOT spend capacity to create itself.

The target identity MUST not already exist canonically.

A non-canonical candidate cannot sponsor another candidate because a sponsor must already be canonical and inviter-eligible.

## 8.12 Stranger sponsorship

A sponsor MAY admit a person the sponsor does not know personally.

The protocol MUST NOT require:

* physical meeting;
* civil-identity disclosure;
* long-term social relationship;
* shared organization;
* shared politics;
* shared geography.

A sponsor MAY rely on:

* a request pool;
* a relay;
* private communication;
* optional evidence;
* another lawful source.

The sponsor’s subjective willingness to spend capacity is not itself proof of VH or VI.

## 8.13 Prohibited qualification bases

Inviter eligibility MUST NOT be granted solely because an identity:

* operates a node;
* holds governance office;
* is designated an expert;
* possesses wealth;
* pays a fee;
* owns tokens;
* controls infrastructure;
* belongs to a favored institution;
* is a genesis identity;
* is popular;
* receives private operator approval.

These facts are not sufficient sponsor authorization.

## 8.14 Invitation suspension

An identity may lose or suspend inviter eligibility only through a canonically authorized rule.

Valid suspension inputs MAY include:

* adjudicated coordinated admission abuse;
* repeated capacity double-spend attempts;
* sale or transfer of invitation authorization;
* operation of a Sybil sponsorship ring;
* loss of required VH;
* loss of required identity continuity;
* severe key compromise;
* another specified admission violation.

Suspension MUST NOT be based solely on:

* the invitee’s opinions;
* ordinary factual error;
* unpopular speech;
* isolated later misconduct;
* disagreement with governance;
* failure of the invitee to reach higher verification.

## 8.15 Suspension effects

Unless another authoritative rule states otherwise, invitation suspension affects only:

* future capacity generation;
* future capacity spending;
* admission-event eligibility.

It does not automatically:

* revoke the sponsor identity;
* revoke sponsor keys;
* invalidate earlier admissions;
* erase sponsor provenance;
* remove ordinary writing rights;
* remove challenge rights;
* remove voting rights;
* remove governance rights;
* alter VH or VI;
* alter truth or importance weight.

Any additional effect requires separate authority.

## 8.16 Restoration

Invitation eligibility and capacity MAY be restored through rulebook-defined procedures.

Restoration MUST be:

* deterministic;
* replay-visible;
* challengeable where applicable;
* independent of private operator discretion;
* capable of distinguishing temporary caution from permanent exclusion.

Restoration MAY require:

* expiration of a suspension period;
* successful challenge resolution;
* new independent verification evidence;
* key rotation or recovery;
* evidence that coordinated abuse ended;
* another specified canonical condition.

## 8.17 No sponsor guarantee

A sponsor is not a guarantor of the invitee.

Canonical admission records that the sponsor spent scarce capacity.

It does not make the sponsor automatically responsible for all future invitee behavior.

This boundary is necessary to permit:

* stranger sponsorship;
* admission of dissidents;
* admission of socially isolated people;
* admission across political and cultural boundaries;
* participation by people who cannot safely disclose identity.

Without this boundary, participants would rationally avoid sponsoring unknown or high-risk applicants, recreating private gatekeeping.

## 8.18 Sponsor provenance

Profile v0 MUST preserve canonical sponsor authorship.

Public replay must determine:

* which identity authored admission;
* which identity was created;
* the canonical creation position;
* the active admission profile;
* the capacity debit;
* the resulting admission-lineage edge.

Private request transport, contact details, legal identity, and relay history MUST NOT be inferred as canonical facts from sponsor provenance.

A future privacy-preserving admission profile MAY hide or threshold sponsor identity only if it defines another deterministic authorization mechanism.

Profile v0 does not provide that mechanism.

# 9. Invitation eligibility

## 9.1 Purpose

Invitation eligibility determines whether a canonical human identity may sponsor the creation of additional canonical identities.

Invitation eligibility exists to distribute admission authority broadly while preventing arbitrary identity creation from multiplying canonical participation rights.

It is not:

* a permanent status;
* a social rank;
* a governance office;
* an expert designation;
* a token balance;
* a measure of epistemic authority;
* a guarantee that every sponsored identity is human;
* a reward for popularity.

Invitation eligibility is a replay-derived, event-family-specific authorization state.

It permits only the sponsorship of canonical identity admission under this specification.

## 9.2 Separation from other eligibility states

Invitation eligibility MUST remain distinct from:

* canonical identity existence;
* key control;
* VH;
* VI;
* ordinary writer eligibility;
* ordinary challenge eligibility;
* voter eligibility;
* governance eligibility;
* Tempo eligibility;
* possession of invitation capacity.

An identity MAY be eligible to write ordinary canonical content without being eligible to invite.

An identity MAY be inviter-eligible while temporarily having no invitation capacity.

An identity MAY lose invitation eligibility without automatically losing ordinary writing, voting, governance, or Tempo eligibility.

No implementation may represent all participation authority as one undifferentiated verification level.

The active rulebook MUST derive each event-family eligibility output separately.

## 9.3 Universal availability

Invitation eligibility MUST be attainable by every canonical human identity that satisfies the same deterministic requirements.

It MUST NOT be permanently reserved for:

* genesis identities;
* founders;
* operators;
* validators;
* node owners;
* delegates;
* experts;
* institutions;
* governments;
* wealthy participants;
* token holders;
* members of an inherited admission lineage;
* members of a favored political, cultural, religious, or geographic group.

No private administrator, committee, company, institution, or governance office may directly appoint an otherwise ineligible identity as an inviter through non-canonical state.

A governance process MAY change the general inviter-eligibility rulebook.

It MUST NOT privately exempt named identities from the rulebook unless a separate canonical exception mechanism is explicitly defined, publicly reviewable, and constitutionally valid.

## 9.4 Minimum eligibility classes

The active rulebook MUST require at least the following before invitation eligibility may become active:

1. the identity exists canonically;
2. the identity is classified as a human identity;
3. the identity controls an active authorized key;
4. the identity has sufficient VH certainty;
5. the identity has sufficient identity-continuity evidence;
6. the identity has matured across a strictly positive number of certified cycles;
7. the identity is not under active invitation suspension;
8. no unresolved canonical condition expressly blocks invitation eligibility.

The exact thresholds and maturation period are rulebook-controlled.

A rulebook MUST NOT set the maturation period to zero.

A newly admitted identity MUST survive at least one subsequent certified cycle before becoming inviter-eligible.

## 9.5 VH requirement

Invitation eligibility MUST require sufficient VH confidence that the identity is controlled by a real human.

VH certainty MUST derive from explicit verification claims, evidence, attestations, contradictions, challenges, and outcomes under the Verification Specification.

VH MUST NOT derive solely from:

* admission sponsorship;
* possession of a key;
* Anthill degree;
* social popularity;
* account age measured only by wall-clock time;
* payment;
* proof of work;
* an AI classification;
* a private operator approval;
* a single unchallengeable attestation.

The active rulebook MAY require stronger VH certainty for invitation eligibility than for limited ordinary writing.

This is RECOMMENDED because invitation authority can reproduce identities and therefore presents greater systemic Sybil risk than one bounded ordinary contribution.

## 9.6 VI and civil-identity boundary

A disclosed civil identity MUST NOT be universally required for invitation eligibility.

Invitation eligibility MUST remain attainable by pseudonymous humans who establish sufficient:

* VH;
* identity continuity;
* key continuity;
* evidence diversity;
* challenge survival;
* good standing.

VI evidence MAY contribute where an identity voluntarily makes correspondence claims or where a particular verification predicate requires correspondence to a known person.

VI MUST NOT become a universal requirement for:

* pseudonymous participation;
* ordinary writing;
* invitation eligibility;
* access by people without government documents.

No government, institution, or civil-identity provider may become the sole source capable of satisfying inviter eligibility.

## 9.7 Identity continuity

Invitation eligibility MUST require sufficient confidence that the same continuing human controls the identity over time.

Continuity MAY be supported by:

* sustained key control;
* authorized key rotations;
* repeated independent attestations;
* continuity claims;
* challenge survival;
* offline continuity evidence;
* privacy-preserving credential continuity;
* consistent participation across certified cycles;
* other rulebook-authorized evidence.

Continuity MUST NOT be inferred solely from inactivity-free wall-clock duration.

A bot-controlled key that remains active for a long period does not become human through age alone.

A legitimate human who must rotate a compromised key MUST NOT automatically lose all continuity if the rotation or recovery path preserves authorized identity provenance.

## 9.8 Maturation across certified cycles

Invitation eligibility MUST activate only after required maturation.

Maturation MUST be counted through qualifying certified human-deliberative cycles under the Cycle Specification.

Wall-clock time, cron jobs, AI activity, system emitters, and machine-only boundary production do not substitute for certified-cycle maturation.

When canonical cycles stall, invitation maturation also stalls.

Under Profile v0, a Dmax-only, forced, degraded, survivor, record-only, or machine-only boundary:

* generates no inviter maturation;
* does not activate new inviter eligibility;
* does not restore invitation suspension;
* does not create admission rewards or authority.

Such a boundary may become relevant to maturation only if it separately satisfies the required human-deliberative certification rules. Dmax status alone is insufficient.

A maturation rule MUST specify:

* counted cycle class;
* required number of qualifying periods;
* boundary at which maturation becomes effective;
* treatment of suspensions;
* treatment of rulebook transitions;
* treatment of invalidated cycles.
## 9.9 Evidence diversity

The active rulebook SHOULD require verification evidence from sufficiently independent sources before invitation eligibility activates.

Diversity MAY consider:

* admission lineage;
* Anthill topology;
* attestation provenance;
* social-cluster concentration;
* verification-provider concentration;
* geographic or institutional concentration where safely knowable;
* repeated use of the same evidence source;
* reciprocal attestation patterns;
* temporal independence.

Evidence from one tightly connected cluster SHOULD exhibit diminishing returns.

Evidence diversity MUST NOT be implemented as a rigid requirement that makes participation impossible for:

* isolated communities;
* censored populations;
* rural populations;
* minority-language groups;
* displaced people;
* fragmented or offline networks;
* communities newly entering the system.

The rulebook SHOULD permit multiple equivalent combinations of evidence.

For example, a person lacking broad social connections may compensate through stronger continuity, longer maturation, independent challenge survival, privacy-preserving credentials, or other authorized evidence classes.

Anthill topology MAY help measure concentration or diversity.

Anthill topology alone MUST NOT establish or deny invitation eligibility.

## 9.10 Challenge status

The active rulebook MAY consider unresolved verification challenges when deriving invitation eligibility.

A severe unresolved challenge concerning:

* human control;
* duplicate identity control;
* identity compromise;
* synthetic attestation;
* coordinated Sybil activity;
* continuity failure

MAY temporarily prevent inviter eligibility from activating.

A challenge MUST NOT affect eligibility merely because it exists.

The effect of a challenge must depend on:

* the challenge type;
* its admissibility;
* available evidence;
* the applicable certainty rules;
* challenge outcomes;
* active rulebook thresholds.

Low-quality or abusive challenges MUST NOT provide an inexpensive means to indefinitely block legitimate identities from becoming inviters.

## 9.11 Good-standing boundary

Where the active rulebook uses a good-standing condition, it MUST define that condition through explicit canonical predicates.

“Good standing” MUST NOT be an undefined discretionary judgment.

Permitted admission-related predicates MAY include:

* no active adjudicated invitation-abuse suspension;
* no unresolved severe identity-compromise state;
* no demonstrated capacity transfer or sale;
* no adjudicated participation in a Sybil sponsorship ring;
* compliance with required key-security transitions.

Good standing MUST NOT depend solely on:

* ideological conformity;
* popularity;
* social approval;
* controversial speech;
* lawful disagreement;
* criticism of governance;
* low importance rankings;
* truth claims later found incorrect;
* an invitee’s unrelated conduct.

## 9.12 Boundary activation

Inviter eligibility MUST become effective at a deterministic canonical boundary.

Profile v0 requires that new inviter eligibility activate only at a qualifying human-certified boundary required by the active rulebook.

It MUST NOT activate merely because:

* a local clock reached a date;
* a server applied a database migration;
* an administrator changed a private flag;
* a Dmax-only, forced, degraded, survivor, record-only, or machine-only boundary occurred.

Invitation capacity derived from newly activated eligibility MUST NOT be spendable before that activation.
## 9.13 No same-cycle recursive eligibility

An identity created during a cycle MUST NOT become inviter-eligible during that same cycle.

An identity that first satisfies a threshold during a cycle MUST NOT use newly acquired invitation eligibility until the rulebook-defined activation boundary.

Invitation capacity derived from newly activated eligibility MUST NOT be spendable before that activation.

This rule prevents a single admission event from beginning an unbounded same-cycle invitation chain.

## 9.14 Suspension

Invitation eligibility MAY be suspended by an authorized canonical outcome.

Suspension MUST be:

* prospective;
* replay-derived;
* attributable to a defined rule;
* based on canonical evidence or state;
* challengeable where applicable;
* separable from unrelated event-family eligibility;
* capable of restoration where the rule allows.

A suspension event or derived suspension state MUST identify:

* the affected identity;
* the authority or predicate producing the suspension;
* the canonical effective position;
* the applicable rulebook;
* the affected eligibility lane;
* the review, expiration, or restoration conditions where applicable.

## 9.15 Restoration

Invitation eligibility MAY be restored when the rulebook-defined restoration conditions are met.

Restoration becomes effective only through:

* an authorized canonical restoration outcome; and
* the qualifying activation boundary required by the active rulebook.

A forced, degraded, survivor, record-only, machine-only, or Dmax-only boundary MUST NOT restore invitation eligibility or capacity unless it separately satisfies the required human-deliberative certification rules.

Restoration MUST be:

* prospective;
* replay-derived;
* challengeable where required;
* publicly explainable.
---

# 10. Invitation capacity

## 10.1 Purpose

Invitation capacity bounds the rate at which inviter-eligible identities may create new canonical identities.

It protects:

* canonical storage;
* verification attention;
* public reasoning bandwidth;
* human challenge capacity;
* voter-pool integrity;
* the system’s resistance to self-reproducing Sybil lineages.

Invitation capacity limits admission quantity.

It does not determine whether an admitted identity is human.

Human verification remains a separate process.

## 10.2 Nature of capacity

Invitation capacity is a replay-derived integer quantity associated with one canonical identity.

It is not a transferable object.

It MUST NOT be represented as:

* a tradable token;
* currency;
* property transferable to another identity;
* a governance vote;
* reputation;
* truth weight;
* importance weight;
* a claim of human verification.

An implementation MAY internally materialize capacity balances for performance.

Canonical authority MUST derive from replay, not from the mutable materialized balance alone.

## 10.3 Capacity state

For each canonical identity and applicable capacity period, replay MUST be able to derive:

* whether the period is a qualifying capacity period;
* whether `admission_liveness_blocked` is active;
* whether the identity was inviter-eligible;
* capacity generated;
* capacity carried forward;
* capacity expired;
* capacity consumed;
* capacity suspended;
* capacity restored;
* whether existing capacity remains spendable during a stall;
* remaining spendable capacity.

The balance MUST satisfy:

```text
available_capacity
=
bounded_carryover
+ newly_generated_capacity
+ authorized_adjustments
- successful_admission_debits
- authorized_expirations
```

The resulting balance MUST NOT be negative.

Private pending reservations MUST NOT reduce the canonical balance.
## 10.4 Generation

Invitation capacity MUST be generated only for unsuspended inviter-eligible identities during qualifying capacity periods.

A qualifying capacity period requires a properly certified human-deliberative cycle under the Cycle Specification.

The exact generation rate above the constitutional minimum is rulebook-controlled.

For each unsuspended inviter-eligible identity in each qualifying capacity period, the generated spendable output MUST satisfy:

```text
generated_capacity >= 1
```

The rulebook MAY:

* assign more than one unit;
* delay activation until the qualifying period boundary;
* impose finite carryover and spending caps;
* reduce capacity prospectively after established abuse;
* freeze capacity during an authorized emergency or individual suspension.

The rulebook MUST NOT permanently assign zero capacity to an otherwise eligible and unsuspended class of human identities.

The rulebook MAY consider:

* verification certainty;
* identity continuity;
* maturation;
* evidence diversity;
* prior admission-abuse outcomes;
* current suspension state.

The rulebook MUST NOT generate capacity based solely on:

* wealth;
* payment;
* token ownership;
* political office;
* expert designation;
* node operation;
* popularity;
* number of Anthill connections;
* civil identity disclosure;
* institutional membership.
## 10.5 Cycle dependence

Profile v0 invitation capacity is generated through qualifying human-deliberative certified cycles, not wall-clock time.

A Dmax-only, forced, degraded, survivor, record-only, or machine-only boundary that does not separately satisfy the required human-deliberative certification rules MUST NOT:

* generate new invitation capacity;
* advance inviter maturation;
* activate new inviter eligibility;
* restore invitation suspension;
* increase carryover caps;
* create admission rewards or authority.

Such a boundary MAY:

* preserve canonical event ordering;
* preserve historical state;
* record already valid capacity debits;
* support replay and liveness bookkeeping.

No capacity accrues merely because:

* a number of days passed;
* a server remained online;
* a node emitted boundaries;
* no humans participated;
* a machine process advanced a counter.

`admission_liveness_blocked` MUST be exposed when no qualifying capacity period exists.

Dmax status alone is insufficient to create invitation authority.
## 10.6 Integer units

Invitation capacity MUST be denominated in whole admission units.

One successful Profile-v0 `identity_create` consumes exactly one unit.

Fractional invitation capacity MUST NOT authorize a partial identity creation.

A rulebook MAY use fractional internal certainty or scoring inputs when deriving integer capacity.

The spendable output must be a deterministic non-negative integer.

## 10.7 Carryover

Unused invitation capacity MAY carry forward across cycles.

Carryover MUST be bounded.

Unlimited accumulation is prohibited because it would allow:

* dormant identities to return with large admission bursts;
* compromised mature identities to release large Sybil batches;
* invitation markets to stockpile long-term admission power;
* old identity classes to accumulate disproportionate control.

The exact carryover cap is rulebook-controlled.

The rulebook SHOULD express the cap as either:

* a maximum integer balance; or
* a maximum multiple of the ordinary per-cycle generation rate.

The cap MUST be deterministic and publicly inspectable.

## 10.8 Expiration

Capacity above the permitted carryover cap MUST expire or become unavailable at the applicable boundary.

Expiration affects only unused future admission authority.

It MUST NOT:

* invalidate past identity admissions;
* reduce historical capacity records;
* alter sponsor provenance;
* alter verification state;
* create economic loss.

Expired invitation capacity MUST NOT be recoverable through private operator action.

## 10.9 Non-transferability

Invitation capacity MUST be bound to the identity for which it was generated.

An identity MUST NOT:

* transfer capacity;
* sell capacity;
* donate capacity;
* delegate capacity;
* lend capacity;
* combine capacity with another identity;
* authorize another identity to spend it.

A sponsor MAY choose which applicant to sponsor.

That choice is not a transfer of capacity.

The sponsor remains the canonical author and spender.

A future profile MAY define threshold or collective sponsorship.

Profile v0 does not permit it.

## 10.10 Non-saleability

The protocol MUST NOT provide a canonical mechanism for selling invitation capacity.

Evidence that an identity sold or systematically traded admission authorization MAY support a coordinated-admission-abuse claim under the applicable rulebook.

The protocol cannot prevent all off-protocol coercion or payment.

It MUST avoid providing canonical transfer instruments that make an invitation market easy to enforce.

## 10.11 No delegation

An inviter MUST sign its own `identity_create` candidate using an active authorized key.

A private account, delegate, application server, organization, or AI agent MUST NOT spend invitation capacity on the inviter’s behalf unless the signature is produced through a separately authorized key-control mechanism owned by that same human identity.

Operational assistance does not change canonical authorship.

## 10.12 Capacity consumption

Capacity is consumed only when a valid `identity_create` event is successfully applied.

The debit MUST occur atomically with identity creation.

A failed event MUST consume no capacity.

Failure cases include:

* invalid sponsor signature;
* inactive sponsor key;
* inviter ineligibility;
* active suspension;
* insufficient capacity;
* duplicate target identity;
* conflicting public key;
* malformed descriptor;
* invalid applicant possession proof;
* invalid admission authorization;
* unsupported admission profile.

## 10.13 Idempotent retry

An identical retry of an already accepted authored candidate MUST return the existing canonical result and MUST NOT consume another unit.

Idempotency MUST be based on the existing canonical event and signed candidate identity, not merely on similar payload contents.

A newly signed event with a different `event_id` is not an identical retry, even when it targets the same applicant.

## 10.14 Conflicting duplicate events

A new event attempting to create an already-existing identity MUST be rejected.

A new event attempting to register a public key already bound incompatibly in canonical state MUST be rejected.

A new event using an existing `event_id` with different signed bytes MUST be rejected as a conflicting duplicate.

Rejected conflicting events consume no capacity.

## 10.15 Concurrent final-unit spending

If an inviter has one remaining capacity unit and multiple valid-looking admission events compete for it, canonical order determines the result.

The first event validly applied consumes the final unit.

Later events are evaluated against the resulting zero balance and MUST be rejected for insufficient capacity.

Arrival order and relay order have no effect.

## 10.16 Admission-authorization reference

Every `identity_create` event MUST include an `admission_authorization_reference`.

The reference is the reduced handshake commitment used by the applicant and sponsor to identify the intended admission context.

It MUST cover only:

* admission profile version;
* sponsor identity;
* applicable capacity period;
* applicable rulebook reference.

It MUST NOT include `eligibility_snapshot_reference` or any other frozen sponsor-eligibility snapshot.

The reference is not a transferable capacity token, reservation, bearer credential, proof of current sponsor eligibility, proof of active sponsor key state, proof of unsuspended status, or proof that capacity remains.

Canonical replay MUST independently verify that:

* the reduced context is structurally valid and applicable;
* the sponsor exists and is human;
* the sponsor author key is active for the signed candidate;
* inviter eligibility is active at the event's canonical position;
* no invitation suspension blocks the event;
* sufficient capacity remains;
* target identity and key uniqueness rules pass;
* the applicant possession proof is valid.

Possession of a syntactically valid reference does not guarantee admission.

## 10.17 Stale authorization

An `identity_create` candidate MAY become stale before canonical application when the reduced authorization context is no longer applicable.

Staleness MAY result from:

* capacity period closure or replacement;
* admission-profile deactivation;
* referenced rulebook replacement or non-applicability under transition rules;
* invalidation of the referenced period or rulebook;
* expiry of a canonical grace period.

The following are not stale authorization by themselves and MUST use their own validation results:

* sponsor key inactivity, supersession, or revocation;
* inviter ineligibility;
* invitation suspension;
* insufficient capacity;
* target identity already existing;
* public key already being registered.

If the reduced authorization context no longer matches the required profile, period, or rulebook at application, the event MUST be rejected with a stable authorization error.

# 11. Canonical `identity_create`

## 11.1 Purpose

`identity_create` is the sole normal post-genesis Profile-v0 event that creates a new canonical human identity.

For Profile v0, the target identity kind is fixed by the active admission profile:

```text
identity_kind = human
```

This kind records that the identity is admitted through the human identity profile.

It does not establish VH, VI, human uniqueness, ordinary writer eligibility, ordinary challenge eligibility, voter eligibility, governance eligibility, Tempo eligibility, inviter eligibility, invitation capacity, or economic authority.

AI, system, tribe, organization, and other non-human identities require other event families or admission profiles.

It binds:

* an eligible human sponsor;
* a target identity;
* the target’s initial public key;
* the applicant’s proof of key possession;
* the applicable admission authorization;
* the sponsor’s invitation-capacity debit.

A valid `identity_create` establishes canonical identity existence.

It does not establish verified-human status or ordinary participation eligibility.

## 11.2 Canonical author

The canonical author of `identity_create` MUST be the sponsor.

The authored-candidate envelope MUST satisfy:

```text
author_identity_id = sponsor_identity_id
```

The sponsor identity is derived from `author_identity_id` for Profile v0.

If the payload includes a sponsor identity for compatibility or explicit binding, it MUST equal `author_identity_id`.

The target identity MUST differ from `author_identity_id`.

## 11.3 Speaker identity

For Profile-v0 `identity_create`, `speaker_identity_id` MUST be absent.

The sponsor is the event author.

The sponsor authors the admission authorization in its own canonical capacity and MUST NOT act as the applicant's speaker.

## 11.4 Required payload

The canonical Profile-v0 payload MUST contain exactly the fields authorized by Appendix A.

Profile v0 does not add a free-form `identity_kind` payload field.

The target kind is derived from the active admission profile as `identity_kind = human`.

At minimum, it MUST contain:

```text
identity_id
initial_key_descriptor
initial_public_key_ref
initial_key_possession_proof
admission_authorization_reference
verification_reference
```

Appendix A MUST define:

* field types;
* canonical ordering;
* optionality;
* null encoding;
* size limits;
* forbidden additional fields.

## 11.5 `identity_id`

`identity_id` identifies the new canonical identity.

It MUST:

* use the canonical identity identifier format;
* be distinct from the sponsor;
* not already exist in canonical state;
* not conflict with an authoritative genesis identity;
* satisfy any domain or version constraints defined by the identity specification.

The event MUST be rejected if the target identity already exists.

Non-canonical prior requests do not reserve an `identity_id`.

## 11.6 `initial_key_descriptor`

`initial_key_descriptor` MUST contain enough canonical material to reconstruct and validate the target identity’s initial Profile-v0 key.

For Profile v0 it MUST include:

```text
key_profile_version
signature_algorithm
raw_public_key_bytes
owning_identity_id
```

The descriptor MUST satisfy:

```text
key_profile_version = ed25519_v0
signature_algorithm = ed25519
length(raw_public_key_bytes) = 32 bytes
owning_identity_id = identity_id
```

Unsupported profiles or algorithms MUST be rejected.

Malformed keys MUST be rejected.

The descriptor MUST NOT contain private-key material.

## 11.7 `initial_public_key_ref`

`initial_public_key_ref` MUST equal the canonical domain-separated hash of `initial_key_descriptor` under the Profile-v0 authorship specification.

Replay MUST recompute the reference.

The event MUST be rejected if the supplied value differs from the computed value.

A public key already registered incompatibly to another identity MUST be rejected.

A future profile MAY permit defined multi-identity or organizational key relationships.

Profile v0 ordinary human identity admission does not.

## 11.8 `initial_key_possession_proof`

`initial_key_possession_proof` proves that the applicant controls the private key corresponding to `initial_key_descriptor`.

The proof MUST:

* use Ed25519 under Profile v0;
* be exactly the required signature length;
* verify against `raw_public_key_bytes`;
* bind the target identity;
* bind the final `identity_create` event;
* bind the sponsor;
* bind the initial public-key reference;
* bind the reduced admission authorization reference;
* bind the exact `verification_reference` value or the canonical no-reference state.

The proof does not make the applicant the author of `identity_create`.

It establishes key possession only.

## 11.9 `admission_authorization_reference`

`admission_authorization_reference` commits to the reduced admission authorization context:

```text
admission_profile_version
sponsor_identity_id
capacity_period_id
rulebook_reference
```

The reference MUST be validated structurally against that reduced context.

It MUST NOT be accepted as sufficient merely because its hash is well formed.

It does not freeze sponsor eligibility, sponsor key state, suspension state, remaining capacity, target uniqueness, or key uniqueness.

Those states must be replay-derived at the event's canonical application position.

## 11.10 `verification_reference`

`verification_reference` is semantically optional under Profile v0.

Appendix A MUST choose exactly one canonical no-reference representation for the signed payload.
 It may choose field omission or canonical null, but it MUST NOT treat omission and null as interchangeable encodings for the same signed semantic value.

When present, `verification_reference` MAY refer only to:

* an existing canonical verification artifact;
* a privacy-safe canonical commitment to an applicant-authorized verification package;
* another exact canonical verification commitment authorized by the Verification and Privacy Specifications.

It MUST NOT directly reference:

* a private request-stage package;
* a mutable private evidence record;
* a private submission-service identifier;
* a contact record;
* a relay-local object;
* a request-pool identifier;
* a relay identifier;
* a private account identifier;
* raw identity documents;
* private communications;
* raw private evidence.

It MUST NOT by itself:

* authorize admission;
* establish VH;
* establish VI;
* prove uniqueness;
* grant writer eligibility;
* grant inviter eligibility;
* substitute for challengeable verification artifacts;
* expose private evidence.

A valid admission MUST remain possible with the one canonical no-reference representation unless a later authorized profile explicitly requires a privacy-preserving canonical verification commitment.

The referenced object or commitment has no effect merely because it appears in `identity_create`.

Normal verification progression still occurs through separately authorized canonical verification claims, evidence, contradictions, challenges, outcomes, derived VH/VI certainty, and event-family eligibility lanes.

## 11.11 Sponsor signature binding

The sponsor's authored-candidate signature MUST bind the complete final payload after the applicant proof has been inserted, including:

* target identity;
* complete initial key descriptor;
* initial public-key reference;
* applicant possession proof;
* reduced admission authorization reference;
* exact verification reference or canonical no-reference state.

A sponsor MUST NOT be able to later claim that it authorized a different key, identity, applicant proof, authorization context, or verification reference than the signed payload contains.

Any change to the target identity, initial key descriptor, initial key reference, applicant proof, authorization reference, or verification reference invalidates the sponsor signature.

## 11.12 Validation sequence

A conforming node MUST validate `identity_create` in a deterministic order that includes at least:

1. validate event type and canonical encoding;
2. validate the authored-candidate envelope;
3. validate the sponsor signature over the complete final payload hash;
4. validate sponsor identity existence;
5. validate sponsor human classification;
6. validate sponsor author-key state at the event's canonical position;
7. reconstruct and validate the reduced admission authorization reference;
8. validate current inviter eligibility;
9. validate current invitation suspension state;
10. validate remaining spendable capacity;
11. validate target identity uniqueness;
12. validate initial key descriptor;
13. recompute `initial_public_key_ref`;
14. validate public-key uniqueness;
15. validate `verification_reference` or the canonical no-reference state;
16. validate applicant possession proof over the final applicant-bound fields;
17. apply atomic effects.

Appendix A MAY refine rejection precedence, but it MUST preserve the semantic distinctions among authorization-context validity, active sponsor key state, inviter eligibility, invitation suspension, capacity, target uniqueness, key uniqueness, verification reference, and applicant proof validity.

## 11.13 Canonical effects

A valid `identity_create` MUST atomically:

1. create the canonical identity;
2. classify the identity as event-derived;
3. record the sponsor as canonical author;
4. record the canonical creation position;
5. record the active admission profile;
6. record admission lineage;
7. register the initial key descriptor;
8. activate the initial key at the creation position;
9. establish or derive the complete protocol-defined `identity_structural_roots`, including the Anthill root;
10. debit one unit of sponsor invitation capacity;
11. establish `CanonicalAdmittedIdentity` status with ordinary participation lanes inactive;
12. enable only the restricted verification lane;
13. expose the permitted public identity and key fields.

## 11.14 Explicit non-effects

A valid `identity_create` MUST NOT automatically:

* establish VH;
* establish VI;
* declare the target a unique human;
* grant ordinary writer eligibility;
* grant ordinary challenge eligibility;
* grant voter eligibility;
* grant governance eligibility;
* grant Tempo eligibility;
* grant inviter eligibility;
* generate invitation capacity;
* mint POD, POINT, tokens, or mana;
* create a private account;
* create a session;
* create a password;
* create an email record;
* create private social state;
* create a verification attestation by the sponsor;
* assign epistemic or governance weight.

The fixed `identity_kind = human` non-effect is especially important: it classifies the admission profile and target kind, but it does not prove that the controller is a real, unique, verified, or eligible human.

## 11.15 Identity structural-root and Anthill effect

Profile v0 MUST establish or derive the complete protocol-defined `identity_structural_roots` set for the new identity.

The authoritative structural specifications MUST choose, for each required root, one of these equivalent canonical approaches:

1. derive the root deterministically from `identity_id`; or
2. materialize a deterministically identified root as an effect of `identity_create`.

The Profile-v0 required root names are fixed here as Mindgarden, Backyard of Relationships, Self Tree, and Anthill. Exact canonical identifiers, byte encodings, derivation rules, structural-role constants, containment relations, and explicit-object-versus-derivation rules remain deferred to Appendix A and structural-role reconciliation.

Anthill-specific verification and provenance semantics remain attached to the Anthill root within the broader set.

A separate discretionary event MUST NOT be required before the identity can receive verification artifacts.

No second authoritative Anthill anchor may be created for the same identity unless a later structural-role reconciliation replaces this representation explicitly.

Private interfaces MAY present additional folders, lists, or views.

They are not authoritative identity structural roots or authoritative Anthill anchors.

## 11.16 Restricted verification-lane effect

At creation, the identity MUST become eligible for only the restricted verification event families defined by Appendix A and the Verification Specification.

This may include:

* key-control events;
* ordinary verification truth claims about self-correspondence;
* ordinary verification truth claims about continuity;
* identity-specific challenge responses;
* acknowledgment of attestations;
* authorized submission of verification evidence;
* disputes concerning verification artifacts attached to the identity.

Those event families MUST create or relate ordinary truth, evidence, contradiction, and challenge objects under constrained authorization.

They MUST NOT create opaque verification-only status records.

The event MUST NOT activate general canonical writing.

## 11.17 Idempotency

An identical resubmission of the same accepted authored candidate MUST return the existing canonical result.

It MUST NOT:

* create a second identity;
* register the key again;
* create a second identity structural-root set or Anthill anchor;
* create a second lineage edge;
* debit capacity again.

Idempotency requires identical signed candidate bytes or the exact identity defined by the canonical event-id rules.

## 11.18 Conflicting duplicate identity

A distinct event attempting to create an already-existing `identity_id` MUST be rejected.

This remains true even when:

* the same sponsor submits it;
* the same applicant key is used;
* the payload is otherwise identical;
* the later event includes stronger verification material.

Later changes to identity state must use authorized lifecycle or verification events, not a second `identity_create`.

## 11.19 Conflicting public key

A Profile-v0 human identity signing key MUST NOT be reused as the initial key of a new identity if the key has already been canonically registered as active, superseded, revoked, invalid, or historically associated with another canonical human identity.

A future profile MAY define an explicit exception with its own canonical safety rule.

## 11.20 Canonical-order conflicts

When multiple valid-looking events target the same identity or compete for the same sponsor capacity, canonical order determines the result.

Nodes MUST NOT choose among conflicting admissions based on:

* local arrival time;
* relay priority;
* sponsor preference communicated after signing;
* payment;
* private queue order;
* AI ranking.

## 11.21 Stable rejection conditions

Appendix A and API error contracts MUST define stable errors for at least:

* `unsupported_admission_profile`;
* `unauthorized_sponsor`;
* `sponsor_not_human`;
* `sponsor_key_inactive`;
* `inviter_ineligible`;
* `inviter_suspended`;
* `insufficient_invitation_capacity`;
* `invalid_admission_authorization`;
* `stale_admission_authorization`;
* `self_sponsorship_forbidden`;
* `identity_already_exists`;
* `public_key_already_registered`;
* `malformed_initial_key_descriptor`;
* `initial_key_owner_mismatch`;
* `initial_public_key_ref_mismatch`;
* `invalid_initial_key_possession_proof`;
* `unsupported_initial_key_profile`;
* `conflicting_duplicate_event`;
* `invalid_verification_reference`.

Stable error identifiers MUST NOT expose private evidence or credentials.

---

# 12. Applicant key possession

## 12.1 Purpose

Applicant key-possession proof prevents a sponsor from creating an identity bound to a key the applicant does not control.

It also prevents:

* accidental key transcription errors;
* malicious sponsor substitution;
* registration of an unrelated public key;
* circular reliance on a key that is not yet canonically active.

The proof establishes possession of the proposed initial private key.

It does not establish:

* human existence;
* human uniqueness;
* civil identity;
* VH;
* VI;
* ordinary writer eligibility;
* inviter eligibility.

## 12.2 Bootstrap separation

The applicant's initial key cannot be required to have been canonically active before `identity_create`, because the event itself registers and activates that key.

Profile v0 avoids circular authorization by using two distinct signatures:

1. the applicant signs a domain-separated possession message using the proposed initial key;
2. the sponsor signs the final canonical authored candidate using an already active sponsor key.

The sponsor is the canonical event author.

The applicant is the controller of the target initial key.

The required semantic construction sequence is:

1. The applicant generates a proposed `identity_id`, initial keypair, initial key descriptor, and `initial_public_key_ref`.
2. The applicant and sponsor identify `sponsor_identity_id`, the Profile-v0 admission profile, the applicable capacity period, and the applicable rulebook reference.
3. The reduced `admission_authorization_reference` is constructed from admission profile, sponsor identity, capacity period, and rulebook reference. This does not reserve capacity.
4. The sponsor or constructing client selects the final UUIDv7 `identity_create_event_id` before the applicant creates the final possession proof. The event ID is not derived from the completed payload or signature.
5. The exact canonical `verification_reference` value is fixed as either the permitted canonical reference or commitment, or the one canonical no-reference representation.
6. Before signing, the applicant must be able to inspect and approve `identity_create_event_id`, `target_identity_id`, the complete initial key descriptor, `initial_public_key_ref`, `sponsor_identity_id`, admission profile, capacity period, rulebook reference, `admission_authorization_reference`, and the exact canonical `verification_reference` value or no-reference encoding.
7. The applicant signs the domain-separated initial-key-possession message.
8. The sponsor or constructing client assembles the complete `identity_create` payload containing `identity_id`, initial key descriptor, `initial_public_key_ref`, applicant `initial_key_possession_proof`, reduced `admission_authorization_reference`, and exact optional or no-reference `verification_reference`.
9. The sponsor constructs the ordinary Profile-v0 authored-candidate envelope and signs the final payload hash. Because the payload now includes the applicant proof, the sponsor signature binds the applicant proof.
10. Canonical ingress validates and applies the event at its actual canonical position. No transport-stage agreement, applicant proof, sponsor signature, pending candidate, or relay acknowledgment reserves capacity or guarantees acceptance.

## 12.3 Required algorithm

Under Profile v0:

* the possession-proof algorithm MUST be Ed25519;
* the proof MUST be a 64-byte Ed25519 signature;
* the verifying public key MUST be the 32-byte raw key in `initial_key_descriptor`;
* plain RFC 8032 Ed25519 verification MUST be used unless the Profile-v0 authorship specification states a narrower compatible rule.

Unsupported algorithms MUST be rejected.

## 12.4 Exact possession bytes

The Profile-v0 possession proof MUST sign a domain-separated canonical byte sequence with the following semantic fields:

```text
ascii("seed.identity.initial_key_possession.v0")
|| id(identity_create_event_id)
|| id(target_identity_id)
|| hash32(initial_public_key_ref)
|| id(sponsor_identity_id)
|| hash32(admission_authorization_reference)
|| hash32(canonical_verification_reference_or_no_reference)
```

The canonical encoding specification MUST define the exact encodings of:

* `ascii`;
* `id`;
* `hash32`;
* the canonical no-reference value;
* the verification-reference commitment used in the proof.

No separators, JSON serialization, whitespace, platform-specific text encoding, or implementation-defined concatenation may be added.

The applicant possession-proof bytes MUST NOT include:

* `initial_key_possession_proof` itself;
* the sponsor signature;
* final signed-candidate bytes;
* a payload hash that includes `initial_key_possession_proof`;
* publication-derived fields.

This avoids recursive signing.

The applicant proof binds only the applicant-relevant final fields listed in this section.

## 12.5 Event binding

The proof MUST bind `identity_create_event_id`.

A possession proof created for one event MUST NOT be valid for another event.

This prevents a proof from being copied into:

* another sponsor’s event;
* another target identity;
* another admission authorization;
* a later conflicting retry.

## 12.6 Identity binding

The proof MUST bind `target_identity_id`.

A proof for one target identity MUST NOT authorize registration of the same key under another identity.

The descriptor’s `owning_identity_id` MUST also equal `target_identity_id`.

Both constraints MUST be checked.

## 12.7 Key-reference binding

The proof MUST bind `initial_public_key_ref`.

Because `initial_public_key_ref` commits to the complete key descriptor, the proof indirectly binds:

* profile version;
* signature algorithm;
* raw public key;
* owning identity.

Replay MUST recompute the key reference before validating the possession proof.

## 12.8 Sponsor binding

The proof MUST bind `sponsor_identity_id`.

A proof prepared for one sponsor MUST NOT be reusable by a different sponsor.

If an applicant changes sponsors, the applicant MUST generate a new final possession proof for the new `identity_create` event.

A non-canonical request-stage proof MAY remain reusable for sponsor discovery, but it is not the final canonical proof.

## 12.9 Admission-authorization binding

The proof MUST bind `admission_authorization_reference`.

This prevents a proof from being moved to an event governed by a different:

* admission profile;
* sponsor;
* capacity period;
* rulebook reference.

If a new capacity period or rulebook reference is required, the authorization reference changes and the applicant MUST sign a new final possession proof.

A new applicant proof is not required solely because current sponsor capacity decreased if the candidate fields remain unchanged, but the candidate may fail at publication for insufficient capacity.

The proof MUST also bind the exact `verification_reference` value or the canonical no-reference state.

The proof MUST distinguish the exact canonical verification-reference value from the canonical no-reference state.

A sponsor MUST NOT append, remove, or replace `verification_reference` after the applicant signs.

Any change to the exact canonical reference or no-reference representation requires a new applicant possession proof.

This binding is required because `verification_reference` may reveal or commit to applicant-related verification material.

## 12.10 Descriptor validation before proof validation

A node MUST validate the initial key descriptor before verifying the possession proof.

The node MUST confirm:

* supported profile;
* supported algorithm;
* correct raw key length;
* target ownership;
* correct public-key reference;
* no forbidden descriptor fields;
* no conflicting prior key state.

A malformed descriptor MUST NOT proceed to possession-proof authorization.

## 12.11 Proof verification

A node validates the proof by:

1. reconstructing the exact possession bytes;
2. reading the raw Ed25519 public key from the validated descriptor;
3. verifying the 64-byte signature;
4. rejecting any non-canonical or malformed representation.

Verification MUST be deterministic.

A node MUST NOT use:

* private account state;
* sponsor statements;
* request-pool approval;
* AI judgment;
* network origin;
* wall-clock submission time

as substitutes for cryptographic proof verification.

## 12.12 Proof non-effects

A valid applicant possession proof does not:

* make the applicant the event author;
* create the identity;
* consume invitation capacity;
* establish VH;
* establish VI;
* prove uniqueness;
* grant eligibility;
* establish social trust;
* create an Anthill relationship;
* create a verification attestation.

The proof becomes effective only as one required component of a valid canonical `identity_create`.

## 12.13 Request-stage possession proof

A non-canonical admission request SHOULD include a separate request-stage proof of key possession.

The request-stage proof MAY sign:

```text
ascii("seed.identity.admission_request_key_possession.v0")
|| id(target_identity_id)
|| hash32(initial_public_key_ref)
|| request_nonce_or_freshness_value
```

The exact request-stage format belongs to the non-canonical request transport profile.

It MUST be domain-separated from the final canonical proof.

A valid request-stage proof MUST NOT substitute for the final event-bound proof.

## 12.14 Sponsor substitution protection

A sponsor MUST NOT replace the applicant's proposed initial key, target identity, sponsor identity, admission authorization reference, event ID, or verification reference after receiving a final possession proof unless the applicant signs a new final possession proof for the replacement fields.

An interface MUST display or otherwise allow the applicant to verify the final:

* event ID;
* target identity;
* complete initial key descriptor;
* initial public-key reference;
* sponsor identity;
* admission profile;
* capacity period;
* rulebook reference;
* admission authorization reference;
* verification reference or canonical no-reference encoding

before producing the final possession proof.

A new candidate and new applicant proof are required when any applicant-bound value changes, including event ID, sponsor identity, target identity, initial key, authorization reference, or verification reference.

A new sponsor signature is required whenever any signed envelope or payload value changes.

A new applicant proof is not required solely because current sponsor capacity decreased if the candidate fields remain unchanged.

## 12.15 Compromised pre-admission key

If the applicant believes the proposed private key was compromised before canonical admission, the applicant SHOULD abandon that local candidate or generate a new initial key.

Because no canonical identity exists yet, no canonical key-rotation event is required.

A sponsor MUST NOT submit an event using an abandoned or superseded local key proposal.

## 12.16 Lost pre-admission key

If the applicant loses the proposed private key before canonical admission, the applicant cannot validly produce the final possession proof.

The applicant must generate a new local key and update the admission request.

A sponsor’s willingness to admit the applicant cannot substitute for proof of control.

## 12.17 Post-admission key state

Upon valid identity creation:

* the initial key becomes active at the event’s canonical position;
* the possession proof remains part of admission provenance;
* later rotation or revocation follows the key-lifecycle specification;
* later key-state changes do not invalidate the original possession proof;
* the original proof does not authorize future events after the key becomes inactive.

## 12.18 Conformance vectors

The Profile-v0 conformance suite MUST include vectors for:

1. valid applicant possession proof;
2. wrong target identity;
3. wrong event ID;
4. wrong sponsor identity;
5. wrong initial public-key reference;
6. wrong admission authorization reference;
7. wrong verification reference;
8. verification reference added after applicant proof;
9. verification reference removed after applicant proof;
10. verification reference replaced after applicant proof;
11. canonical no-reference encoding accepted;
12. non-canonical alternate no-reference encoding rejected;
13. malformed public key;
14. malformed signature;
15. unsupported profile;
16. unsupported algorithm;
17. descriptor owner mismatch;
18. proof created with a different private key;
19. valid request-stage proof rejected as final proof;
20. final proof reused with another sponsor;
21. final proof reused with another event ID;
22. proof bytes excluding `initial_key_possession_proof` itself;
23. proof bytes excluding sponsor signature and recursive payload hash.

Conformance vectors MUST use test-only keys and MUST NOT include production private keys.

# 13. Admission authorization reference

## 13.1 Purpose

The `admission_authorization_reference` commits an `identity_create` event to the intended reduced admission context.

It exists to make the sponsor/applicant handshake:

* deterministic;
* replayable;
* profile-bound;
* sponsor-bound;
* capacity-period-bound;
* rulebook-bound;
* independent of private operator state.

The reference is not an invitation token, transferable credential, bearer instrument, reservation, proof of current eligibility, proof of active key state, proof of unsuspended status, or proof that capacity remains available.

Canonical replay MUST independently verify the sponsor's key, eligibility, suspension state, remaining capacity, target identity, initial key, and applicant proof at the event's canonical position.

## 13.2 Authorization context

Profile v0 defines an **admission authorization context** containing exactly these semantic fields:

```text
admission_profile_version
sponsor_identity_id
capacity_period_id
rulebook_reference
```

The context MUST NOT contain:

* `eligibility_snapshot_reference`;
* another frozen sponsor-eligibility snapshot;
* private account identifiers;
* session identifiers;
* private request-pool identifiers;
* private relay metadata;
* contact-record identifiers;
* passwords;
* unpublished verification evidence;
* raw private evidence;
* mutable node-local settings;
* opaque AI scores.

## 13.3 Reference derivation

`admission_authorization_reference` MUST be a canonical domain-separated commitment to the reduced admission authorization context.

The exact reference bytes MUST be defined by the Canonical Encoding and Hashing Specification.

The semantic construction is:

```text
admission_authorization_reference =
    canonical_domain_separated_hash(
        admission_profile_version,
        sponsor_identity_id,
        capacity_period_id,
        rulebook_reference
    )
```

The exact byte sequence, field encoding, hash function, and domain separator remain controlled by the Canonical Encoding and Hashing Specification.

An implementation MUST NOT derive the reference from JSON serialization, database row order, platform-specific text encoding, or other implementation-dependent formats.

The reference does not prove that the sponsor is currently eligible, that the sponsor key remains active, that the sponsor is unsuspended, or that capacity remains.

## 13.4 Sponsor binding

The authorization context MUST bind `sponsor_identity_id`.

The bound sponsor MUST equal the `author_identity_id` of the `identity_create` authored candidate.

An authorization reference created for one sponsor MUST NOT authorize another sponsor.

The event MUST be rejected when:

```text
authorization_context.sponsor_identity_id
!= authored_candidate.author_identity_id
```

## 13.5 Admission-profile binding

The authorization context MUST identify the active admission profile.

For this specification:

```text
admission_profile_version = sponsored_public_admission_v0
```

or another exact identifier adopted by the canonical encoding and registry specifications.

An authorization reference for another admission profile MUST NOT authorize a Profile-v0 `identity_create`.

Future profiles MUST use distinct domain separation and profile identifiers.

## 13.6 Capacity-period binding

The authorization context MUST bind the capacity period under which the sponsor’s capacity is evaluated.

The capacity period will normally correspond to:

* a certified cycle;
* a cycle-derived admission epoch;
* another deterministic period explicitly defined by the Cycle Specification.

The capacity period MUST NOT be derived solely from wall-clock time.

The event MUST be rejected if the referenced capacity period:

* does not exist;
* is not authorized to generate or spend invitation capacity;
* is not applicable at the event’s canonical position;
* was produced by an ineligible boundary type;
* conflicts with the active admission rulebook.

## 13.7 Rulebook binding

The authorization context MUST bind the rulebook governing:

* inviter eligibility;
* capacity generation;
* carryover;
* expiration;
* suspension;
* capacity consumption;
* applicable admission constraints.

A rulebook reference MUST be:

* canonical;
* versioned;
* replay-visible;
* immutable once finalized.

A private configuration file, environment variable, database flag, or node-local policy MUST NOT serve as the canonical rulebook reference.

## 13.8 Event-position replay validation

At the event's canonical application position, replay MUST independently validate:

* sponsor identity existence;
* sponsor human classification;
* sponsor author-key state;
* inviter eligibility;
* invitation suspension;
* capacity-period applicability;
* active rulebook applicability;
* remaining spendable invitation capacity;
* target uniqueness;
* key uniqueness;
* applicant possession proof.

The authorization context does not freeze any of these states.

A context may remain structurally valid while the event fails because:

* the sponsor key became inactive, superseded, or revoked;
* inviter eligibility was lost;
* invitation suspension became active;
* capacity was consumed by an earlier canonical event;
* the target identity already exists;
* the public key is already registered.

Those failures MUST use their own stable errors rather than being mislabeled as stale authorization.

## 13.9 No bearer-token semantics

The `admission_authorization_reference` is not a bearer token.

Possession of the reference does not permit its holder to:

* create an identity;
* spend another identity’s capacity;
* transfer admission authority;
* reserve capacity;
* delegate sponsorship;
* bypass sponsor signature requirements.

Only the bound sponsor, using an active authorized key, may author an event under the referenced context.

## 13.10 No capacity-unit token

Profile v0 invitation capacity is a replay-derived integer balance rather than a set of individually transferable capacity tokens.

The authorization reference therefore does not identify a transferable invitation unit.

One successful `identity_create` consumes one unit from the sponsor’s available balance.

Canonical order determines which event consumes a remaining unit when multiple events compete.

A future profile MAY define individually identified admission credentials, but Profile v0 does not imply them.

## 13.11 Reuse within one capacity period

A sponsor MAY use the same valid authorization context reference for multiple `identity_create` events during one capacity period, provided sufficient capacity remains for each event.

Reuse of the context reference does not create idempotency between distinct events.

Each event remains separately bound by:

* its `event_id`;
* its target identity;
* its initial key;
* its applicant possession proof;
* its sponsor signature.

Replay debits one unit for each distinct successfully applied event.

## 13.12 No private reservation

Creating or sharing an authorization reference does not reserve capacity.

Signing an `identity_create` candidate does not reserve capacity.

Acceptance by a relay or request pool does not reserve capacity.

Capacity is consumed only when the event is validly applied canonically.

A user interface MAY warn that multiple pending events compete for the same available balance.

Such warnings are advisory.

## 13.13 Stale authorization

An authorization reference becomes stale only when the reduced authorization context is no longer applicable for the event's canonical position.

Causes may include:

* capacity-period closure or replacement;
* admission-profile deactivation under the transition rules;
* referenced rulebook replacement or inapplicability;
* invalidation of the referenced capacity period or rulebook;
* expiration of a canonical grace period.

The following conditions are not stale authorization by themselves and MUST use their own stable validation results:

* sponsor key inactive, superseded, or revoked;
* inviter ineligible;
* inviter suspended;
* insufficient invitation capacity;
* target identity already created;
* public key already registered.

A stale reference MUST NOT authorize identity creation.

The event MUST fail without:

* creating the identity;
* activating the applicant key;
* creating admission lineage;
* creating any required identity structural root;
* consuming capacity.

## 13.14 Sponsor-key changes

The admission authorization context binds the sponsor identity, not one signing key.

The authored candidate separately binds the sponsor's `public_key_ref`.

At application, that key MUST be active.

If the sponsor rotates or revokes keys before publication:

* a candidate signed by an inactive, superseded, or revoked key MUST be rejected with the applicable key-state error;
* the rejection MUST NOT be mislabeled as `stale_admission_authorization`;
* the sponsor MAY produce a new event signed by a current active key;
* the applicant MUST produce a new possession proof when any applicant-bound value changes, including event ID, sponsor identity, authorization reference, or verification reference.

## 13.15 Rulebook changes

When a rulebook changes, the specification controlling rulebook activation MUST define whether previously prepared admission candidates remain valid.

The default Profile-v0 rule is:

* authorization is evaluated under the rulebook active at canonical application;
* a reference to a no-longer-applicable rulebook is stale;
* no private grandfathering occurs;
* a new candidate may be created under the new rulebook.

A rulebook transition MAY explicitly define a deterministic grace period.

Any grace period MUST be canonical, bounded, and replay-visible.

## 13.16 Capacity validation

Replay MUST independently determine the sponsor’s available capacity.

The calculation MUST include:

* authorized generation;
* bounded carryover;
* authorized adjustments;
* prior successful debits;
* expiration;
* suspension effects;
* canonical-order conflicts.

The event MUST be rejected when:

```text
available_invitation_capacity < 1
```

The presence of a valid authorization reference MUST NOT override an insufficient balance.

## 13.17 Atomic debit

When `identity_create` succeeds, capacity debit MUST be applied atomically with all other admission effects.

The debit MUST identify:

* sponsor identity;
* admitted identity;
* `identity_create` event;
* capacity period;
* rulebook reference;
* canonical position.

A separate mutable balance update that is not derivable from the admission event MUST NOT be the source of canonical authority.

## 13.18 Idempotency

Replaying or retrying the identical accepted authored candidate MUST resolve to the same existing debit.

It MUST NOT create a second debit.

A distinct event with different signed bytes is not an identical retry, even when it contains the same authorization reference.

## 13.19 Conflicting authorization

An event MUST be rejected when its authorization reference:

* cannot be decoded or reconstructed;
* binds another sponsor;
* binds another admission profile;
* binds an unknown or inapplicable capacity period;
* binds an unknown or inapplicable rulebook;
* conflicts with the applicable admission profile;
* conflicts with the event's canonical position under the transition rules.

Stable rejection codes MUST distinguish malformed authorization, invalid authorization, stale authorization, key-state failure, inviter ineligibility, invitation suspension, insufficient capacity, duplicate identity, and duplicate public key where this can be done without revealing private evidence.

## 13.20 Public explainability

A node MUST be able to explain the authorization result without exposing protected verification evidence.

The explanation SHOULD include:

* sponsor identity;
* admission profile;
* capacity period;
* rulebook reference;
* whether inviter eligibility was active;
* whether suspension applied;
* capacity before the event;
* debit amount;
* capacity after the event;
* validation result.

When private verification evidence contributed to eligibility, the node MAY identify only the canonical commitment, resulting predicate, or authorized private-verification outcome.

---

# 14. Initial identity authority

## 14.1 Principle

Canonical identity creation grants the minimum authority required for an identity to:

* control its identity keys;
* participate in its own verification;
* preserve continuity;
* defend itself against verification claims;
* become eligible for later participation through defined processes.

It MUST NOT grant ordinary canonical participation merely because the identity now exists.

The initial state is:

```text
canonical identity existence = active
identity_kind = human
initial key control = active
identity_structural_root_state = complete
restricted verification lane = active
ordinary participation lanes = inactive
```

## 14.2 Initial canonical state

Immediately after a valid `identity_create`, replay MUST derive that the new identity:

* exists canonically;
* is event-derived;
* has Profile-v0 target kind `identity_kind = human`;
* has one active initial Profile-v0 key;
* has canonical sponsor provenance;
* has an admission-lineage edge;
* has the required `identity_structural_roots`, including the Anthill root;
* may use the restricted verification lane;
* is not yet ordinary-writer eligible;
* is not yet inviter eligible;
* has zero invitation capacity unless a later rule explicitly grants capacity after maturation;
* has no voting, governance, Tempo, or economic authority from admission alone.

## 14.3 Public read authority

A `CanonicalAdmittedIdentity` has the same right to read public canonical state as any public observer.

Read access MUST NOT depend on:

* verification level;
* private account status;
* invitation lineage;
* political status;
* social connections.

Public read access does not imply permission to submit canonical events.

## 14.4 Identity-control authority

A `CanonicalAdmittedIdentity` MUST be able to perform the minimum authorized key-management actions needed to maintain secure control of itself.

Subject to the key-lifecycle specification, this MAY include:

* key rotation;
* revocation of an eligible non-authorizing key;
* proof of current key control;
* inspection of public key history;
* recovery-related preparation where a later profile permits it.

Identity-control authority MUST be restricted to the same identity.

It MUST NOT permit management of another identity’s keys.

Key-control events MUST NOT grant ordinary writer or inviter eligibility.

## 14.5 Verification self-claims

A `CanonicalAdmittedIdentity` MAY author verification claims concerning itself when an authorized verification schema permits self-claims.

Examples MAY include:

* “I claim to be controlled by a real human.”
* “I claim continuity with the controller at a prior canonical position.”
* “I claim this current key is under my control.”
* “I claim correspondence to the human described by this privacy-preserving reference.”
* “I dispute the claim that I am automated or duplicated.”

A self-claim is not self-verification.

It is one attributable ordinary verification truth claim and one input to the verification process.

The Verification Specification determines its evidentiary value.

A self-claim MUST NOT directly activate VH, VI, ordinary writer eligibility, inviter eligibility, voting, governance, Tempo, or economic authority.

## 14.6 Submission of verification evidence

A `CanonicalAdmittedIdentity` MAY submit verification evidence concerning itself under authorized schemas.

Such evidence MUST be represented using ordinary canonical evidence objects, evidence claims, evidence connections, contradiction relationships, challenge responses, protected commitments, or constrained profiles of those ordinary objects.

Such evidence MAY include:

* continuity proofs;
* challenge responses;
* privacy-preserving credential proofs;
* references to independently authored attestations;
* commitments to private high-risk evidence;
* key-control proofs;
* evidence opposing a duplicate-control claim.

The event family MUST be subject-bound.

The identity MUST NOT use the restricted lane to submit arbitrary evidence about unrelated truth claims.

The restricted lane MUST NOT create a parallel verification-only evidence graph.

## 14.7 Attestation acknowledgment

A `CanonicalAdmittedIdentity` MAY acknowledge receipt of or respond to an attestation concerning itself when the Verification Specification defines such an event.

Acknowledgment MUST NOT automatically mean:

* agreement with every statement in the attestation;
* establishment of VH;
* establishment of VI;
* acceptance of civil identity;
* waiver of challenge rights.

The exact meaning of acknowledgment MUST be explicit in its event schema.

An attestation remains an ordinary challengeable verification truth claim even when the subject acknowledges receipt or responds.

## 14.8 Identity-scoped challenge response

A `CanonicalAdmittedIdentity` MUST have a path to respond to admissible verification challenges concerning itself.

This right exists to prevent verification from becoming a one-sided process.

The identity MAY:

* submit a response;
* provide supporting evidence;
* dispute provenance;
* identify key compromise;
* challenge an attestation’s accuracy where authorized;
* appeal an applicable verification outcome.

This restricted defensive right is not general challenge authority.

The identity MUST NOT use it to initiate unrelated truth challenges against arbitrary ideas or identities.

Where the event family permits a challenge response or appeal, it MUST remain part of the ordinary challenge ontology or an explicitly constrained identity-verification profile of that ontology.

## 14.9 Continuity participation

A `CanonicalAdmittedIdentity` MAY author continuity-related events concerning its own control history.

Continuity participation MAY include:

* signing periodic or event-triggered continuity claims;
* acknowledging a valid key transition;
* linking an authorized offline continuity proof;
* responding to a compromise claim;
* asserting recovery continuity under a future recovery profile.

Wall-clock persistence alone MUST NOT establish continuity.

Continuity remains claim- and evidence-based.

## 14.10 Identity structural-root and Anthill authority

A `CanonicalAdmittedIdentity` MAY perform only those identity structural-root actions, including Anthill-related actions, required for its own verification and relationship management.

The permitted actions MAY include:

* acknowledging a relationship request;
* organizing public verification artifacts associated with itself;
* linking an authorized verification artifact to its Anthill anchor;
* managing visibility settings where another specification permits;
* viewing the provenance of claims and attestations concerning itself.

Identity structural-root authority MUST NOT allow the identity to create arbitrary canonical social edges without the required counterparty authorization.

An Anthill action MUST NOT automatically change VH or VI.

## 14.11 Prohibition on ordinary idea creation

A `CanonicalAdmittedIdentity` MUST NOT create ordinary canonical ideas merely because it has been admitted.

This includes:

* truth claims unrelated to its own verification;
* conceptual ideas;
* actionable ideas;
* narratives;
* general evidence objects;
* general arguments;
* ordinary public descriptions.

Verification-specific claims authorized by the restricted lane are not ordinary idea creation, even if they reuse general truth-claim infrastructure internally.

Their event family, subject, and predicate MUST be constrained.

## 14.12 Prohibition on ordinary connection creation

A `CanonicalAdmittedIdentity` MUST NOT create arbitrary canonical connections merely because it has been admitted.

It MAY create or acknowledge only identity-scoped relationships explicitly permitted by:

* the Verification Specification;
* the Anthill structural rules;
* the identity-control rules;
* Appendix A.

Restricted relationship events MUST NOT become a general method for writing into the public idea graph.

## 14.13 Prohibition on general challenges

A `CanonicalAdmittedIdentity` MUST NOT issue general truth, importance, governance, safety, or policy challenges merely because it has been admitted.

It MAY respond to or dispute verification artifacts concerning itself where the restricted verification lane permits.

The Verification Specification MUST distinguish:

* defensive identity-scoped verification participation;
* ordinary challenge creation;
* challenge voting or adjudication.

## 14.14 No voting authority

Identity creation MUST NOT grant eligibility to:

* vote on truth challenges;
* vote on verification challenges concerning others;
* participate in random voter pools;
* cast governance votes;
* certify cycles;
* select delegates;
* approve rulebooks.

Voting eligibility must be derived separately.

## 14.15 No governance authority

A `CanonicalAdmittedIdentity` MUST NOT:

* propose binding governance changes through a restricted lane;
* approve protocol amendments;
* exercise delegate authority;
* issue governance vetoes;
* alter rulebooks;
* appoint verifiers;
* grant itself eligibility.

Public commentary outside canonical authority may occur through non-canonical interfaces, but it has no binding effect.

## 14.16 No Tempo authority

Canonical identity creation MUST NOT grant authority to:

* submit ordinary Tempo claims;
* provide Tempo evidence;
* vote on Tempo predicates;
* close cycles;
* create beacons;
* certify boundaries;
* alter the authorization frontier.

Tempo eligibility remains separately derived.

## 14.17 No invitation authority

A `CanonicalAdmittedIdentity` MUST NOT:

* sponsor another identity;
* generate invitation capacity;
* transfer invitation capacity;
* participate as an inviter through another identity;
* consume genesis or operator capacity.

Invitation eligibility requires later verification and maturation.

## 14.18 No economic authority

Identity admission MUST NOT mint or allocate:

* POD;
* POINT;
* tokens;
* currency;
* transferable assets;
* governance stake;
* economic rewards;
* personal mana.

A later system MAY provide non-authoritative interface resources for onboarding.

Those resources MUST NOT be treated as canonical economic authority.

## 14.19 No attestation authority concerning others by default

A `CanonicalAdmittedIdentity` MUST NOT automatically be eligible to submit verification attestations about other identities.

Attesting that another identity is human or corresponds to a particular person is an ordinary human-authored verification truth claim that can influence derived verification and therefore requires its own eligibility rules.

The active rulebook MAY later permit limited attestation activity after sufficient verification.

Self-claims and responses concerning one’s own identity do not imply general attester eligibility.

## 14.20 Rate limits for the restricted lane

The restricted verification lane MUST be bounded.

The applicable rulebook or event-family specification MUST define protections against:

* verification-evidence flooding;
* challenge-response flooding;
* repeated duplicate self-claims;
* oversized evidence commitments;
* Anthill-edge spam;
* key-control abuse.

Rate limits MUST permit a legitimate identity to participate meaningfully in its own verification.

They MUST NOT provide an unverified identity with a substitute general-writing channel.

## 14.21 Event-family allowlist

Appendix A MUST define an explicit allowlist of event families available to a `CanonicalAdmittedIdentity` through the restricted verification lane.

Any event family not explicitly permitted MUST be rejected.

The allowlist SHOULD distinguish:

* identity control;
* identity-scoped ordinary verification truth claims;
* identity-scoped ordinary verification evidence;
* identity-scoped contradiction or dispute relationships;
* identity-scoped challenge responses;
* attestation acknowledgment;
* continuity events;
* authorized identity structural-root and Anthill structural events.

A broad wildcard such as “verification-related event” is insufficiently deterministic.

## 14.22 Subject constraint

Restricted verification events authored by a `CanonicalAdmittedIdentity` MUST normally satisfy:

```text
subject_identity_id = author_identity_id
```

Exceptions MUST be explicitly defined.

A `CanonicalAdmittedIdentity` whose ordinary authority lanes remain inactive MUST NOT use the restricted lane to make verification claims about unrelated third parties.

## 14.23 Transition to broader authority

Broader authority becomes active only when replay derives the relevant event-family eligibility under:

* raw canonical verification artifacts;
* verification certainty;
* challenge outcomes;
* maturation;
* active rulebook state;
* cycle-boundary activation.

No private service may “upgrade” the identity through a database flag.

The transition MUST be publicly explainable without exposing protected evidence.

The derivation MUST keep raw artifacts, derived VH or VI certainty, boundary activation, and each eligibility lane separate.

## 14.24 Loss of initial authority

A `CanonicalAdmittedIdentity` MAY lose some restricted capabilities when:

* its active key becomes unavailable;
* a key is compromised;
* an identity-specific safety restriction applies;
* an event family is suspended by a valid canonical rule;
* another authoritative condition is met.

Loss of restricted capability MUST NOT erase identity existence or historical state.

A path to defend against mistaken or malicious verification claims SHOULD remain available wherever safely possible.

---

# 15. Stranger, pseudonymous, and high-risk admission

## 15.1 Principle

Profile v0 MUST provide a viable admission path for a person who:

* does not personally know an existing participant;
* cannot safely reveal a civil identity;
* lives under censorship or authoritarian control;
* has intermittent connectivity;
* is geographically or socially isolated;
* cannot safely maintain a persistent public contact channel;
* belongs to a politically unpopular or marginalized group.

Sponsored admission MUST NOT be limited to private acquaintance networks.

## 15.2 Stranger sponsorship

An eligible sponsor MAY sponsor an applicant without prior personal acquaintance.

The sponsor’s authorization means only that the sponsor elects to spend one unit of invitation capacity to admit the applicant into canonical identity state.

The sponsor need not claim:

* prior friendship;
* in-person contact;
* legal-identity knowledge;
* shared political beliefs;
* shared nationality;
* shared institution;
* certainty that the applicant is human;
* certainty that the applicant is unique.

Verification occurs separately.

## 15.3 Public admission availability

The ecosystem SHOULD support multiple independent ways for applicants to reach potential sponsors.

These MAY include:

* public request pools;
* federated request pools;
* randomized sponsor matching;
* language-specific pools;
* accessibility-focused pools;
* regional relays;
* global relays;
* offline transfer;
* trusted personal transfer;
* encrypted one-way request boards.

No single pool or relay is canonical.

An applicant denied or censored by one transport MUST remain free to use another.

## 15.4 No permanent social prerequisite

The protocol MUST NOT require that an applicant already have:

* Anthill connections;
* social attestations;
* membership in a recognized organization;
* a prior canonical relationship;
* a sponsor from the applicant’s geographic community;
* an introduction by a trusted institution.

Such evidence MAY later contribute to verification.

It is not a universal prerequisite to seek sponsorship.

## 15.5 Pseudonymous applicants

An applicant MAY use a pseudonym and MAY omit civil-identity claims.

A Profile-v0 admission request MUST be valid without:

* legal name;
* government-issued identifier;
* home address;
* employer;
* citizenship;
* public biography;
* facial image;
* biometric template.

A sponsor MUST NOT be required to publicly state the applicant’s civil identity.

The target canonical identity may remain pseudonymous indefinitely where the Verification Specification permits sufficient VH without VI disclosure.

## 15.6 VH without mandatory VI

The admission system MUST preserve the possibility that an identity may establish sufficient VH without publicly establishing VI to a civil identity.

A pseudonymous person may demonstrate:

* continuing human control;
* continuity;
* challenge survival;
* independent attestations;
* privacy-preserving credentials;
* other human-presence evidence

without revealing a legal identity.

VI evidence MAY remain absent, partial, private, or scoped.

No civil-identity authority may become a universal admission gate.

## 15.7 High-risk evidence

High-risk applicants MAY need to provide sensitive verification evidence.

Such evidence MUST be handled according to the Privacy and High-Risk Submission Specification.

Sensitive evidence SHOULD remain:

* private;
* selectively disclosed;
* commitment-bound;
* challengeable through authorized procedures;
* absent from ordinary public reads.

The canonical record MAY contain:

* a commitment;
* an evidence-class identifier;
* an authorized verification outcome;
* provenance sufficient for deterministic validation.

It MUST NOT unnecessarily expose the underlying private evidence.

## 15.8 Sponsor knowledge limitation

A sponsor MAY make an admission decision based on incomplete knowledge.

The protocol MUST NOT treat sponsorship as a declaration that the sponsor has independently verified every applicant claim.

A sponsor MAY rely on:

* request validity;
* key-possession proof;
* a private interaction;
* another person’s introduction;
* a request-pool process;
* optional evidence;
* personal judgment.

The canonical meaning remains limited to capacity expenditure and admission authorization.

## 15.9 Private communication

Applicant-sponsor communication MAY remain private.

The canonical record MUST NOT require publication of:

* private conversation content;
* communication timestamps;
* network addresses;
* relay paths;
* physical meeting locations;
* identity documents;
* personal contact details.

The applicant and sponsor MAY voluntarily disclose information through authorized later claims.

Such disclosure is separate from admission validity.

## 15.10 One-way communication

Request systems SHOULD support one-way or pseudonymous communication methods where possible.

A high-risk applicant MAY need to:

* submit a request without revealing a return address;
* poll for responses through a relay;
* use delayed encrypted messages;
* rotate transport identifiers;
* receive an event candidate offline.

The transport mechanism MUST NOT become a canonical identity authority.

## 15.11 Relay-based admission

A relay MAY carry:

* the admission request;
* sponsor questions;
* applicant responses;
* the proposed `identity_create` candidate;
* the applicant’s final possession proof;
* the finalized event result.

The relay MUST NOT become:

* the applicant;
* the sponsor;
* the verifier;
* the canonical author

merely because it transported these objects.

## 15.12 Offline admission preparation

An applicant MAY prepare:

* local keys;
* an identity candidate;
* a request-stage possession proof;
* an admission request

while offline.

A sponsor MAY review the request later.

The final canonical possession proof may require updated event-specific material.

Offline delay does not invalidate the applicant’s humanity, but stale cryptographic or authorization material MUST be refreshed when required.

## 15.13 Store-and-forward behavior

Admission requests and event candidates MAY be stored and forwarded across disconnected networks.

Canonical validity is determined at publication, not at initial creation.

The event remains subject to:

* sponsor eligibility;
* active-key state;
* capacity availability;
* current rulebook;
* canonical-order conflicts.

A long-delayed event has no special priority.

## 15.14 Censorship resistance

The protocol SHOULD minimize mandatory chokepoints by supporting:

* portable signed requests;
* independent request pools;
* federated relays;
* offline transfer;
* multiple sponsors;
* standard request formats;
* public validation software;
* non-exclusive transport providers.

No sponsor is required to approve a particular request.

No applicant is guaranteed immediate admission.

Censorship resistance means that no single actor can universally prevent the request from reaching all possible sponsors.

## 15.15 Sponsor targeting risk

Canonical sponsor provenance may expose sponsors to pressure or coercion.

Profile v0 accepts public sponsor authorship as part of deterministic authorization and lineage accountability.

Implementations SHOULD minimize additional disclosure.

They MUST NOT unnecessarily publish:

* private contact history;
* relationship details;
* request-pool selection logic;
* applicant location;
* sponsor location;
* relay route;
* private evidence reviewed.

A future admission profile MAY use threshold or privacy-preserving sponsorship.

Profile v0 does not imply such a mechanism.

## 15.16 Applicant targeting risk

Canonical identity creation makes the target identity publicly visible.

Applicants MUST be informed that:

* canonical identity existence is durable;
* sponsor provenance is durable;
* public key history is durable;
* future public claims may be linked to the identity;
* pseudonymity does not guarantee immunity from correlation.

Interfaces SHOULD provide clear high-risk warnings before the applicant produces the final event-bound possession proof.

## 15.17 No ideological admission test

Admission MUST NOT require an applicant to:

* endorse the protocol;
* support a government;
* reject a religion;
* accept a political ideology;
* demonstrate preferred moral views;
* submit acceptable policy positions;
* prove intellectual sophistication.

A local sponsor may decline to sponsor any request.

The protocol-level eligibility and validity rules MUST remain politically and epistemically neutral.

## 15.18 No proof-of-reasoning requirement

An applicant MUST NOT be required to submit a persuasive argument or high-quality idea as proof of humanity.

Such a requirement would:

* favor AI systems capable of generated reasoning;
* disadvantage language minorities;
* disadvantage people with limited education;
* create viewpoint discrimination;
* confuse human existence with epistemic performance.

Ordinary idea contribution begins only after the applicable writer eligibility is established.

## 15.19 No mandatory economic cost

The canonical admission protocol MUST NOT require payment, token ownership, wealth, or computational expenditure as proof of humanity.

Transport services MAY charge fees or use deposits where allowed locally.

Alternative free or independently operated transports SHOULD remain possible.

A fee MUST NOT become canonical human-verification evidence.

## 15.20 No guaranteed anonymity

Profile v0 supports pseudonymous and privacy-conscious admission.

It does not guarantee perfect anonymity.

Threats may include:

* traffic analysis;
* sponsor coercion;
* device compromise;
* key compromise;
* correlation with later activity;
* malicious relays;
* disclosure by counterparties.

The Privacy Specification SHOULD document realistic threat models and mitigations.

No interface should promise anonymity stronger than the protocol provides.

## 15.21 Accessibility

Admission transports SHOULD be usable across:

* low-bandwidth networks;
* mobile devices;
* offline exchange;
* assistive technologies;
* multiple languages;
* limited technical literacy.

Accessibility services MAY assist in constructing requests.

They MUST NOT gain canonical control of applicant keys or admission authority unless explicitly authorized by the applicant’s key-control mechanism.

## 15.22 Admission delay

Scarce invitation capacity may cause delay.

Delay alone MUST NOT:

* reduce the applicant’s future verification certainty;
* imply suspiciousness;
* give wealthy applicants canonical priority;
* invalidate a non-expired request;
* confer authority on request-pool operators.

Rulebooks and future profiles MAY define bounded anti-gatekeeping fallback mechanisms.

Profile v0 does not activate open canonical self-registration.

## 15.23 Multiple sponsor attempts

An applicant MAY approach multiple potential sponsors.

Only one valid `identity_create` for a target identity may succeed.

Once the identity exists:

* later creation attempts are rejected;
* unused sponsor capacity in failed later attempts is not consumed;
* request pools SHOULD suppress stale duplicate requests when practical.

The applicant SHOULD avoid authorizing conflicting final events where possible.

Canonical order remains decisive.

## 15.24 Admission under network partition

During a network partition, sponsors MAY unknowingly prepare conflicting admissions or overspend apparent capacity.

No partition-local result becomes globally authoritative merely because it was locally accepted.

Upon canonical merge or publication:

* canonical order governs;
* capacity is recomputed;
* invalid overspending events are rejected;
* valid earlier admissions remain effective.

Interfaces SHOULD warn that offline capacity views may be stale.

## 15.25 Future anti-gatekeeping fallback

A future profile MAY add a globally bounded admission path for people who cannot obtain sponsorship.

Such a path MUST define:

* applicant scarcity;
* queue or lottery behavior;
* anti-flooding controls;
* canonical storage limits;
* applicant authorship;
* key bootstrap;
* initial authority;
* replay semantics;
* privacy;
* abuse handling.

No such path is active merely because Profile v0 recognizes the risk of gatekeeping.

---

# 16. Admission lineage

## 16.1 Purpose

Admission lineage records the canonical provenance relationship created when a sponsor spends invitation capacity to create another identity.

The direct lineage relationship is:

```text
sponsor_identity_id
→ admitted_identity_id
```

Admission lineage supports:

* auditability of distributed admission authority;
* detection of concentrated sponsorship;
* analysis of self-reproducing Sybil structures;
* evidence-diversity calculations;
* investigation of coordinated admission abuse;
* explanation of invitation-capacity consequences.

Admission lineage is provenance.

It is not proof of guilt, truth, verification, trustworthiness, or political affiliation.

## 16.2 Direct lineage edge

Every valid post-genesis Profile-v0 `identity_create` MUST produce one direct canonical admission-lineage edge.

The edge MUST identify:

* sponsor identity;
* admitted identity;
* `identity_create` event;
* admission profile;
* canonical creation position;
* capacity period;
* applicable rulebook reference.

The direct edge is immutable historical provenance.

Its downstream effects may change prospectively under later rulebooks or challenge outcomes.

## 16.3 Genesis identities

Genesis-admitted identities do not have ordinary sponsor lineage unless the authoritative genesis state explicitly defines it.

Their provenance MUST be classified as:

```text
genesis_admitted
```

Implementations MUST NOT fabricate sponsor edges for genesis identities.

Genesis provenance may serve as the root of one or more later event-derived lineages.

## 16.4 Legacy identities

Legacy operator-provisioned identities MAY lack complete sponsor lineage.

They MUST be classified accurately as:

```text
legacy_operator_provisioned
```

or another adopted compatibility category.

Implementations MUST NOT invent:

* sponsors;
* capacity debits;
* invitation events;
* attestation histories;
* lineage edges.

A later canonical registration or recovery process MAY provide future authority without rewriting historical provenance.

## 16.5 Derived ancestry graph

Nodes MAY derive an admission-ancestry graph by following direct sponsor edges.

Derived ancestry MAY include:

* parent sponsor;
* children admitted;
* ancestors;
* descendants;
* depth;
* branching factor;
* temporal admission clusters;
* concentrated subgraphs.

The direct canonical edge is authoritative.

Derived graph metrics are replay-derived calculations and MUST identify their algorithm and rulebook version when used canonically.

## 16.6 Lineage is not verification

A sponsor-to-admitted edge MUST NOT automatically establish:

* VH;
* VI;
* human uniqueness;
* sponsor trustworthiness;
* invitee trustworthiness;
* identity continuity;
* ordinary writer eligibility;
* inviter eligibility.

A sponsor MAY separately submit a verification attestation.

That attestation is an independent ordinary verification truth claim.

Admission lineage MAY be considered as provenance or concentration context when evaluating the independence of evidence.

It is not itself the evidence that the target is human.

## 16.7 Lineage is not social relationship

Admission lineage does not necessarily mean:

* friendship;
* family relationship;
* personal acquaintance;
* political alliance;
* organizational membership;
* continuing contact;
* mutual trust.

The sponsor may have admitted a stranger through a request pool.

Interfaces MUST NOT label admission lineage as friendship or endorsement unless separate relationship evidence exists.

## 16.8 Anthill display

The Anthill MAY display direct admission lineage and derived ancestry for provenance and verification legibility.

The Anthill MUST visually and semantically distinguish:

* admission lineage;
* social relationships;
* verification attestations;
* verification evidence;
* challenges;
* derived certainty.

A user MUST NOT be led to believe that an admission edge is automatically a human-verification attestation.

## 16.9 Permitted lineage uses

The active rulebook MAY use lineage as one input for:

* diminishing returns on tightly concentrated attestations;
* detecting rapid recursive sponsorship;
* measuring independence among evidence sources;
* identifying possible invitation rings;
* selecting cases for human review or challenge;
* increasing maturation requirements after adjudicated abuse;
* deriving forward-looking invitation-capacity restrictions.

Any canonical consequence MUST use a specified deterministic rule or challenge outcome.

## 16.10 Prohibited lineage uses

Admission lineage MUST NOT directly determine:

* truth certainty outside authorized verification predicates;
* importance rankings;
* governance vote weight;
* voter vote weight;
* Tempo influence;
* POD or POINT allocation;
* economic rewards;
* political legitimacy;
* social worth;
* automatic guilt;
* automatic identity deletion;
* retroactive event invalidation.

Lineage MUST NOT become a caste system.

## 16.11 No guilt by association

An identity MUST NOT be penalized solely because:

* its sponsor was later suspended;
* another identity in its ancestry committed abuse;
* one descendant behaved badly;
* its lineage is politically unpopular;
* its lineage originates in a censored region;
* many members share a language or community;
* its sponsor later lost verification.

Canonical consequences require evidence concerning the affected identity or a defined, proportionate lineage-risk rule.

Any lineage-risk rule MUST preserve challenge and restoration paths.

## 16.12 Sponsor accountability boundary

The sponsor is accountable for the act of spending admission capacity.

The sponsor is not automatically accountable for every later action of the admitted identity.

Lineage MAY support investigation when evidence suggests:

* intentional Sybil sponsorship;
* invitation selling;
* coordinated false attestations;
* repeated controlled identities;
* a recursive admission ring;
* deliberate capacity circumvention.

Ordinary disagreement, mistake, or isolated invitee misconduct is insufficient by itself.

## 16.13 Lineage concentration

A rulebook MAY measure concentration where:

* many identities share a small number of sponsors;
* sponsorship expands rapidly within one ancestry;
* attestations remain almost entirely within one lineage;
* reciprocal invitation and verification patterns suggest coordination;
* one lineage dominates a verification pool.

Concentration is a risk signal.

It is not conclusive proof of abuse.

Legitimate causes may include:

* a newly connected region;
* a language community;
* a refugee community;
* a large organization;
* an offline network joining in batches;
* a trusted public sponsor serving strangers.

Canonical sanctions MUST require stronger evidence or a defined adjudication process.

## 16.14 Diminishing returns

The Verification Specification MAY apply diminishing returns to attestations concentrated within one lineage or tightly related cluster.

Diminishing returns SHOULD:

* reduce the marginal effect of redundant evidence;
* encourage independent corroboration;
* avoid complete exclusion of isolated communities;
* decay or adapt when new independent evidence appears;
* remain explainable.

The system SHOULD prefer reduced evidentiary independence over automatic invalidation.

## 16.15 Cross-lineage evidence

Cross-lineage evidence MAY strengthen:

* VH certainty;
* continuity confidence;
* inviter eligibility;
* resistance to self-reproducing Sybil clusters.

Cross-lineage evidence MUST NOT be an inflexible universal requirement when the subject lacks access to independent communities.

The rulebook SHOULD support substitute evidence combinations such as:

* longer maturation;
* stronger continuity;
* independent challenge survival;
* privacy-preserving credentials;
* distributed verification procedures.

## 16.16 Lineage depth

A rulebook MAY consider lineage depth when evaluating reproduction risk.

It SHOULD place greater emphasis on:

* direct sponsorship;
* recent ancestry;
* rapid recursive expansion;
* tightly timed clusters.

Remote ancestry SHOULD have diminishing influence.

A person MUST NOT remain permanently burdened by arbitrary conduct many generations earlier.

## 16.17 Temporal behavior

Lineage analysis SHOULD consider canonical time and cycle position.

Potentially relevant features include:

* number of admissions per cycle;
* time between admission and inviter eligibility;
* recursive depth gained over a small number of cycles;
* concentration of attestations before maturation;
* synchronized activity.

Wall-clock timing alone MUST NOT determine canonical guilt.

## 16.18 Lineage taint

Where the Verification Specification defines lineage taint, taint MUST be:

* derived from explicit canonical evidence or outcomes;
* prospective;
* proportionate;
* explainable;
* challengeable;
* subject to decay, restoration, or supersession where appropriate.

Taint SHOULD represent increased uncertainty or scrutiny.

It SHOULD NOT automatically:

* erase identities;
* invalidate all descendants;
* remove all event-family eligibility;
* establish coordinated abuse without evidence;
* permanently stigmatize a lineage.

## 16.19 Forward-looking effects

A valid lineage-related outcome MAY affect:

* future invitation-capacity generation;
* maturation requirements;
* diversity requirements;
* verification certainty;
* admission-suspension state;
* selection for additional verification.

It MUST NOT retroactively invalidate a valid `identity_create` merely because later lineage risk appears.

Historical identity existence and valid earlier events remain part of replay.

## 16.20 Challengeability

An identity affected by a lineage-derived restriction MUST have access to:

* the public rule producing the effect;
* the canonical evidence or permissible commitment basis;
* an explanation of relevant graph inputs;
* a challenge or appeal procedure;
* a restoration path where the rule permits.

Private graph analysis or undisclosed AI classification MUST NOT independently impose canonical restrictions.

## 16.21 AI and anomaly detection

AI or statistical systems MAY identify patterns for further inspection.

They MAY suggest:

* possible rings;
* unusual branching;
* concentrated attestations;
* synchronized identities;
* probable duplicate control.

Such outputs are leads, not canonical verdicts.

Canonical consequences require:

* human-authored claims or authorized deterministic predicates;
* admissible evidence;
* challenge procedures;
* replay-visible outcomes.

## 16.22 Public visibility

Direct admission lineage is public canonical provenance under Profile v0.

Public read surfaces SHOULD expose:

* direct sponsor;
* admitted identity;
* creation event;
* creation position;
* admission profile.

Derived ancestry views MAY be provided.

Interfaces MUST distinguish canonical direct edges from locally computed graph interpretations.

## 16.23 Privacy limitation

Although direct sponsor provenance is public, the canonical lineage edge MUST NOT include:

* private request content;
* contact details;
* legal identity;
* relay path;
* meeting location;
* private evidence reviewed;
* sponsor-applicant conversation.

A lineage edge reveals admission authorization, not the private circumstances surrounding it.

## 16.24 No lineage deletion

A valid direct admission-lineage edge MUST NOT be deleted from canonical history.

Later events MAY:

* dispute its interpretation;
* record sponsor compromise;
* mark abuse outcomes;
* alter prospective effects.

They MUST NOT rewrite who authored the historical admission.

## 16.25 Rulebook versioning

Any rulebook using lineage-derived metrics MUST identify:

* metric definitions;
* graph scope;
* depth limit;
* temporal window;
* concentration function;
* diminishing-return function;
* affected eligibility lane;
* activation boundary.

A node MUST be able to reproduce the result from canonical inputs.

## 16.26 Civilizational objective

Admission lineage should make distributed admission accountable without creating hereditary exclusion.

The intended balance is:

```text
sponsorship is visible
abuse patterns are investigable
evidence independence is measurable
but ancestry does not become social caste
```

Every identity remains individually challengeable, individually verifiable, and capable of developing independent evidence beyond its admission lineage.

# 17. Inviter accountability

## 17.1 Purpose

Inviter accountability exists to deter deliberate abuse of scarce identity-admission capacity without discouraging legitimate sponsorship of strangers, dissidents, isolated people, or politically unpopular applicants.

The accountability model MUST distinguish among:

1. the sponsor’s act of authorizing admission;
2. the applicant’s later verification status;
3. the admitted identity’s later conduct;
4. coordinated admission abuse;
5. unrelated disagreement or misconduct.

A sponsor is responsible for the canonical act of spending invitation capacity.

A sponsor is not automatically responsible for every later action, claim, relationship, or failure of the admitted identity.

## 17.2 Narrow meaning of sponsorship

The canonical meaning of sponsorship is:

> The sponsor authorizes one unit of its invitation capacity to create the target canonical identity and bind the target’s initial key.

Sponsorship MUST NOT be interpreted as a claim that:

* the applicant is already verified as human;
* the applicant is globally unique;
* the applicant corresponds to a disclosed civil identity;
* every applicant statement is true;
* the applicant will act lawfully or responsibly;
* the sponsor agrees with the applicant’s beliefs;
* the sponsor will supervise the applicant;
* the sponsor guarantees the applicant’s future conduct.

Any verification attestation by the sponsor MUST be represented as a separate verification artifact.

## 17.3 No automatic liability for invitee conduct

A sponsor MUST NOT automatically lose invitation eligibility, invitation capacity, writer eligibility, voting eligibility, governance eligibility, Tempo eligibility, or verification certainty merely because an admitted identity later:

* makes an incorrect truth claim;
* expresses an unpopular opinion;
* disputes the sponsor;
* changes political or religious beliefs;
* fails to attain higher verification;
* becomes inactive;
* loses a key;
* commits isolated misconduct;
* receives an ordinary challenge;
* loses an unrelated challenge;
* becomes socially controversial;
* leaves the sponsor’s Anthill or social network.

The system MUST NOT create a general doctrine of inherited or associative guilt.

## 17.4 Mistaken sponsorship

A sponsor may admit an applicant who later turns out to be:

* automated;
* controlled by another existing participant;
* compromised;
* deceptive;
* unable to complete verification;
* part of an abusive cluster.

One mistaken sponsorship does not by itself establish coordinated admission abuse.

The applicable rulebook SHOULD distinguish:

* good-faith error;
* negligent repeated behavior;
* reckless sponsorship;
* knowing coordinated abuse.

The distinction MUST be supported by canonical evidence and challengeable claims where consequences differ.

## 17.5 Coordinated admission-abuse claims

Alleged coordinated admission abuse MUST be represented through explicit ordinary canonical truth claims, evidence, contradictions, challenges, and outcomes.

Examples of admissible claim predicates MAY include:

* the sponsor knowingly admitted an automated identity;
* the sponsor repeatedly admitted identities controlled by one actor;
* the sponsor sold invitation capacity;
* the sponsor transferred effective control of admission decisions;
* the sponsor participated in an invitation ring;
* the sponsor coordinated false verification attestations;
* the sponsor deliberately bypassed maturation or capacity rules;
* the sponsor used compromised keys to create unauthorized identities;
* the sponsor and applicant colluded to conceal duplicate control.

Such a claim remains a claim until evaluated under the applicable evidence, contradiction, challenge, outcome, and certainty rules.

## 17.6 Required evidence

An inviter-accountability consequence MUST NOT arise solely from:

* lineage proximity;
* an AI-generated suspicion score;
* high admission volume that was within lawful capacity;
* political similarity;
* common language;
* geographic proximity;
* membership in the same organization;
* private operator judgment;
* public unpopularity.

Evidence MAY include:

* repeated canonical sponsor patterns;
* duplicate-control evidence;
* admitted identity key-control overlap;
* transaction or communication commitments where lawfully disclosed;
* coordinated timing across canonical cycles;
* repeated false attestations;
* challenge outcomes;
* verified invitation-sale evidence;
* proof of shared automated infrastructure where properly admitted;
* another rulebook-authorized evidence class.

The evidentiary value of each class MUST be defined by the Verification, Challenge, or Safety Specifications using ordinary canonical evidence semantics or an explicitly constrained profile of them.

## 17.7 Human-authored accountability claims

Where inviter abuse is represented as a truth claim, the claim MUST be authored by an eligible human identity or another explicitly authorized human-first event family.

AI MAY:

* detect suspicious patterns;
* propose candidate claims;
* summarize evidence;
* identify potential duplicate structures;
* simulate alternative explanations.

AI MUST NOT independently create a binding inviter-abuse verdict, verification outcome, or eligibility transition.

AI-generated leads require adoption, evaluation, or challenge through the applicable human-authored canonical process.

## 17.8 Challengeability

A sponsor affected by an inviter-abuse claim MUST have access to a challengeable process.

The process SHOULD allow the sponsor to:

* inspect the public claim;
* inspect public evidence;
* access authorized summaries or commitments for protected evidence;
* dispute attribution;
* identify key compromise;
* provide counterevidence;
* distinguish good-faith error from coordination;
* challenge graph assumptions;
* appeal an applicable outcome.

A private allegation MUST NOT silently alter canonical invitation eligibility.

Protected evidence may affect the process only through an authorized public commitment, threshold result, proof, or privacy-preserving outcome.

## 17.9 Proportionality

Consequences MUST be proportionate to the established conduct and uncertainty.

Possible forward-looking consequences MAY include:

* warning status;
* increased scrutiny;
* reduced capacity generation;
* reduced carryover cap;
* longer inviter-maturation requirements;
* temporary invitation suspension;
* additional evidence-diversity requirements;
* temporary ineligibility to submit verification attestations;
* permanent loss of invitation eligibility in severe, repeatedly established cases.

Consequences SHOULD escalate only when supported by stronger or repeated evidence.

## 17.10 Separate eligibility lanes

A finding concerning admission abuse MUST affect only the eligibility lanes authorized by the applicable rule.

For example, an inviter-abuse finding MAY affect:

* invitation-capacity generation;
* invitation-capacity spending;
* attester eligibility for identity verification.

It MUST NOT automatically affect:

* truth-claim authorship;
* ordinary writer eligibility;
* voting eligibility;
* governance eligibility;
* Tempo eligibility;
* key control;
* identity existence.

Additional effects require separate predicates and authority.

## 17.11 No retroactive destruction

A later inviter-accountability outcome MUST NOT automatically:

* invalidate past identity admissions;
* erase admitted identities;
* delete sponsor lineage;
* revoke past key signatures;
* delete the invitee’s later valid contributions;
* remove historical verification artifacts.

A prior `identity_create` remains valid when it was valid at its canonical position.

Later findings MAY alter future eligibility or verification certainty.

## 17.12 Invitee independence

An admitted identity MUST be able to establish verification and authority independently of its sponsor over time.

The admitted identity MAY develop:

* independent verification attestations;
* cross-lineage evidence;
* long-term continuity;
* successful challenge history;
* new social relationships;
* identity evidence unrelated to the sponsor.

A sponsor’s later loss of eligibility MUST NOT permanently trap the invitee in an inferior status.

## 17.13 Sponsor compromise

Where evidence indicates that a sponsor key was compromised, the system MUST distinguish:

* intentional sponsor conduct;
* unauthorized conduct by a key attacker;
* ambiguous control;
* later recovery.

A compromise claim MAY justify temporary suspension while key control is resolved.

It MUST NOT automatically establish that the human sponsor intentionally participated in the resulting admissions.

Key history, event position, compromise evidence, and recovery events MUST remain separately inspectable.

## 17.14 Invitation sales

Profile v0 provides no canonical mechanism to sell invitation capacity.

A claim that a sponsor sold capacity MUST identify the alleged exchange and supporting evidence.

Payment alone does not necessarily prove sale where the payment may relate to:

* transport costs;
* legal assistance;
* device access;
* connectivity;
* unrelated services.

A canonical consequence requires evidence that the payment purchased admission authorization or effective control over the sponsor’s decision.

## 17.15 Coercion

A sponsor may be coerced by:

* a government;
* an employer;
* a criminal organization;
* a family member;
* an armed group;
* an institution;
* another participant.

The accountability system SHOULD permit coercion evidence as a mitigating or distinct factor.

A coerced sponsor MAY still require key-security or capacity restrictions.

The protocol SHOULD distinguish protective restriction from moral blame.

## 17.16 Repeated patterns

Repeated harmful patterns MAY justify stronger consequences than isolated events.

A rulebook MAY consider:

* number of established abusive admissions;
* proportion of the sponsor’s admissions linked to duplicate control;
* recurrence across cycles;
* coordination with other sponsors;
* evidence that warnings were ignored;
* attempts to evade prior suspension;
* evidence of deliberate automation.

The rulebook MUST define the relevant window, thresholds, and decay behavior.

## 17.17 Decay and restoration

Admission-abuse effects SHOULD support decay, review, or restoration where appropriate.

A rulebook MAY permit restoration after:

* a defined number of certified cycles;
* successful challenge resolution;
* strong new verification evidence;
* key rotation and security remediation;
* absence of further abusive conduct;
* restitution or corrective cooperation where relevant;
* another explicit canonical condition.

Permanent exclusion SHOULD be reserved for severe and repeatedly established abuse.

## 17.18 Stranger-sponsorship protection

Accountability rules MUST be designed so that a rational eligible participant can sponsor a stranger without accepting unlimited future liability.

The rulebook MUST NOT assume that sponsorship implies:

* close personal knowledge;
* supervision;
* shared ideology;
* long-term contact;
* identity-document verification.

Otherwise, public admission pools would become ineffective and applicants without social connections would be excluded.

## 17.19 Public explanation

A node MUST be able to explain any inviter-accountability consequence through replay-visible state.

The explanation SHOULD identify:

* affected identity;
* affected eligibility lane;
* applicable claim or outcome;
* effective canonical position;
* governing rulebook;
* consequence;
* expiration or restoration conditions;
* challenge status.

Protected evidence MAY remain private under an authorized privacy process.

The node MUST still expose the canonical commitment or authorized outcome supporting the consequence.

## 17.20 No private blacklist

A private blacklist MUST NOT determine canonical inviter eligibility.

Interfaces MAY maintain local safety lists or refuse service.

Such lists have no protocol-wide canonical effect.

A canonical restriction requires:

* authorized event or derived state;
* canonical evidence or outcome;
* deterministic replay;
* applicable rulebook authority.

## 17.21 No popularity-based accountability

An inviter MUST NOT be punished because:

* many users dislike an invitee;
* an invitee has low importance rankings;
* an invitee’s claims receive negative reactions;
* a social group campaigns against the lineage;
* the sponsor is politically isolated.

Popularity, engagement, and attention are not admission-abuse evidence.

## 17.22 Constitutional objective

Inviter accountability MUST maintain the following balance:

```text
deliberate Sybil sponsorship is costly
invitation rings are investigable
capacity abuse can be restricted
but ordinary sponsorship of strangers remains viable
```

The system must deter organized admission abuse without recreating closed social membership.

---

# 18. Sybil-resistance model

## 18.1 Purpose

Profile v0 uses layered Sybil resistance.

No single mechanism is sufficient.

The system combines:

1. permissionless local key generation;
2. non-canonical admission requests;
3. scarce sponsor capacity;
4. minimal initial canonical authority;
5. progressive verification;
6. identity continuity;
7. bounded event-family rates;
8. delayed inviter eligibility;
9. evidence-diversity requirements;
10. challengeable admission-lineage analysis.

The objective is not to guarantee that no false identity is ever admitted.

The objective is to prevent false identities from cheaply multiplying into authoritative canonical participation.

## 18.2 Key control is not humanity

A valid cryptographic key proves that an actor controls the corresponding private key.

It does not prove:

* that the controller is human;
* that the controller is one human;
* that the controller is distinct from another identity;
* that the controller corresponds to a particular civil person;
* that the controller acts independently;
* that the controller is not automated.

Sybil resistance MUST NOT treat key generation as human verification.

## 18.3 Admission scarcity

Canonical identity creation requires scarce invitation capacity.

This limits:

* canonical identity growth;
* verification demand;
* initial storage pressure;
* recursive bot registration;
* multiplication of later per-identity allowances.

Admission scarcity does not prove humanity.

It limits the rate at which uncertain identities enter the verification system.

## 18.4 Minimal initial authority

A false identity that passes admission MUST initially receive minimal authority.

A `CanonicalAdmittedIdentity` may use only:

* identity-control events;
* restricted verification events concerning itself;
* authorized continuity events;
* limited identity structural-root and Anthill structural events.

It cannot immediately:

* publish ordinary ideas;
* create arbitrary connections;
* challenge general claims;
* vote;
* govern;
* participate in Tempo;
* invite more identities.

This containment reduces the harm of individual admission errors.

## 18.5 Progressive verification

Higher authority MUST require stronger verification evidence.

The active Verification Specification SHOULD increase requirements as the system grants access to more consequential actions.

For example:

* restricted self-verification requires identity existence and active key control;
* limited ordinary writing requires sufficient VH and applicable maturation;
* broader writing or challenge rates may require stronger continuity;
* voting may require additional anti-Sybil qualification;
* invitation eligibility SHOULD require stronger VH, continuity, maturation, and evidence diversity than ordinary writing.

The exact thresholds remain rulebook-controlled.

## 18.6 Independent verification tracks

VH and VI MUST remain distinct.

Sybil resistance primarily requires confidence that:

* a real human controls the identity;
* the identity is not merely a duplicate participation surface for the same human where uniqueness matters;
* control persists over time.

Civil identity correspondence may help in some cases but MUST NOT be mandatory for all people.

A pseudonymous participant may attain meaningful VH and continuity without public civil identity.

## 18.7 Truth-claim verification model

Claims relevant to Sybil resistance MUST be represented through explicit, challengeable canonical artifacts.

Relevant predicates MAY include:

* a human controls identity A;
* identity A and identity B are controlled by one human;
* identity A is automated;
* identity A’s key was compromised;
* attester C interacted with a human controlling identity A;
* identity A maintained continuity across cycles;
* sponsor S coordinated multiple duplicate identities;
* a claimed interaction did not occur.

No claim becomes true merely through submission.

Certainty derives from evidence, contradiction, challenge, and outcomes.

## 18.8 Anthill topology

Anthill topology MAY assist in evaluating:

* attestation concentration;
* social-cluster redundancy;
* admission-lineage concentration;
* reciprocal verification patterns;
* cross-community evidence;
* potential identity farms.

Anthill topology MUST NOT independently establish:

* humanity;
* duplicate control;
* guilt;
* inviter abuse;
* writer eligibility.

Graph structure is context and provenance.

Explicit claims and evidence carry the verification content.

## 18.9 Evidence diversity

Evidence concentrated within one actor, lineage, institution, or social cluster SHOULD receive diminishing marginal weight.

A rulebook MAY consider:

* number of independent attesters;
* admission-lineage distance;
* reciprocal attestation density;
* common verification providers;
* common organizations;
* temporal independence;
* evidence-type diversity;
* challenge independence.

Diversity metrics MUST remain deterministic and explainable when they affect canonical eligibility.

## 18.10 Avoiding exclusion of isolated communities

Evidence-diversity rules MUST account for legitimate concentration.

A newly connected or isolated community may naturally exhibit:

* one regional sponsor;
* common language;
* shared institutions;
* limited external connectivity;
* high internal attestation density.

The protocol MUST NOT automatically classify such a community as a Sybil cluster.

Alternative paths MAY include:

* longer maturation;
* stronger continuity evidence;
* independent verification procedures;
* cross-cycle challenge survival;
* privacy-preserving credentials;
* gradual cross-lineage corroboration.

## 18.11 Maturation delay

Inviter eligibility MUST require a strictly positive maturation delay measured through qualifying certified human-deliberative cycles.

This prevents:

```text
one admitted Sybil
-> immediate inviter eligibility
-> many more admitted Sybils
```

The delay provides time for:

* verification claims;
* challenge;
* continuity assessment;
* duplicate-control evidence;
* lineage analysis;
* human observation.

Wall-clock waiting, Dmax-only boundaries, forced boundaries, degraded boundaries, survivor boundaries, record-only periods, and machine-only activity are insufficient for Profile-v0 inviter maturation unless the boundary separately satisfies the required human-deliberative certification rules.
## 18.12 Sybil reproduction rate

The system SHOULD be designed so that one compromised or false identity cannot reliably produce more than one mature inviter before detection, restriction, or exhaustion of capacity.

This design objective may be expressed informally as keeping the effective Sybil reproduction rate below one.

The objective is supported by:

* admission scarcity;
* low but positive invitation generation for eligible unsuspended inviters;
* bounded carryover;
* maturation across qualifying certified cycles;
* stronger inviter verification;
* cross-lineage evidence;
* lineage scrutiny;
* event-family limits.

The protocol need not define one universal numerical reproduction model in Profile v0.
## 18.13 No same-cycle recursion

A newly created identity MUST NOT:

* become inviter-eligible;
* generate invitation capacity;
* spend invitation capacity

within the same cycle in which it was created.

A rulebook SHOULD require multiple qualifying certified human-deliberative cycles before inviter eligibility.

This constraint MUST be evaluated through canonical cycle state.
## 18.14 Per-identity rate limits

Per-identity rate limits MAY restrict:

* ordinary writing;
* challenges;
* verification submissions;
* attestation creation;
* voting;
* invitation spending.

Per-identity limits are not sufficient Sybil resistance when identity creation is unconstrained.

They must operate together with admission scarcity and verification.

## 18.15 Verification-lane flood control

The restricted verification lane MUST include protections against a false identity flooding:

* self-claims;
* evidence commitments;
* duplicate challenge responses;
* Anthill structures;
* key-control attempts.

Controls MAY include:

* per-cycle event caps;
* payload-size limits;
* deduplication;
* schema-specific cooldowns;
* challenge-linked response limits;
* rejection of semantically duplicate claims.

These limits MUST still permit legitimate defense and verification progression.

## 18.16 Attestation eligibility

A newly admitted identity MUST NOT automatically be permitted to attest that other identities are human.

Attester eligibility SHOULD require:

* sufficient VH;
* continuity;
* active key control;
* maturation;
* applicable evidence or interaction authority.

This prevents unverified clusters from immediately verifying one another.

## 18.17 Reciprocal attestation

Mutual or reciprocal attestations MAY be valid evidence.

They SHOULD NOT receive full independent weight merely because two identities attest to each other.

The verification rulebook MAY discount:

* same-event reciprocity;
* tightly timed reciprocity;
* repeated closed-loop attestations;
* attestations within one unverified lineage cluster.

The actual observation described by each attestation remains separately challengeable.

## 18.18 Duplicate-control detection

Claims that one human controls multiple identities MAY contribute to Sybil resistance.

Such claims MUST be:

* explicit;
* attributable;
* evidence-supported;
* challengeable;
* scoped to a defined consequence.

Multiple identity control MUST NOT automatically imply wrongdoing in every context.

Legitimate cases may include:

* recovery;
* legacy migration;
* temporary key or identity transition;
* test identities outside production;
* an identity created but never activated.

The rulebook must distinguish duplicate canonical participation from benign historical or technical cases.

## 18.19 Uniqueness boundary

Profile v0 does not require the impossible guarantee that every human has exactly one cryptographic identity candidate.

Where uniqueness matters, the system should instead restrict simultaneous authoritative participation by multiple identities controlled by one human.

The Verification Specification MAY define:

* duplicate-control claims;
* dominant identity selection;
* eligibility suspension;
* consolidation;
* continuity-preserving transition;
* other remedies.

This admission specification does not define the full duplicate-identity lifecycle.

## 18.20 Computational costs

Proof of work, VDFs, storage proofs, or deposits MAY be used by non-canonical transports to limit spam.

They MUST NOT be treated as proof of humanity.

Wealthy or state-level actors can purchase disproportionate computation.

Computational cost MAY supplement admission-request transport but MUST NOT replace human verification or sponsor capacity.

## 18.21 No proof of reasoning

Reasoning quality MUST NOT be used as a human-identity admission test.

AI systems are capable of producing persuasive reasoning.

A reasoning requirement would also disadvantage:

* people with limited education;
* people using a second language;
* children and young participants where permitted;
* people with disabilities;
* people unfamiliar with the system’s dominant culture.

Human identity and epistemic quality are separate questions.

## 18.22 Optional credentials

Optional credentials MAY contribute evidence, including:

* institutional attestations;
* privacy-preserving proof-of-personhood credentials;
* legal identity commitments;
* professional credentials;
* community verification procedures.

No single credential provider may be universally mandatory.

Credential evidence MUST remain challengeable and provider concentration SHOULD receive diminishing trust where appropriate.

## 18.23 Nation-state resistance

A nation state may possess:

* large computational resources;
* identity-document authority;
* coercive power;
* surveillance;
* controlled institutions;
* large bot networks.

The protocol SHOULD resist nation-state capture by avoiding dependence on:

* one national ID system;
* one registry;
* one credential issuer;
* one geographic network;
* one admission relay;
* one social cluster.

Admission and verification should permit independent international evidence and pseudonymous participation.

## 18.24 Invitation-ring resistance

Invitation rings may involve multiple apparently independent sponsors.

Relevant protections include:

* maturity delays;
* low per-cycle capacity;
* bounded carryover;
* lineage analysis;
* cross-lineage evidence;
* duplicate-control claims;
* attestation-concentration discounts;
* challengeable abuse claims;
* forward-looking suspension.

No one signal is conclusive.

## 18.25 Key-compromise containment

A compromised inviter key can spend remaining capacity until the compromise is reflected in canonical key or eligibility state.

The key-lifecycle and verification systems SHOULD support:

* rapid rotation;
* revocation;
* compromise claims;
* temporary capacity freeze;
* challengeable recovery;
* clear historical attribution.

A compromise discovered later does not automatically invalidate all earlier admissions.

## 18.26 False-positive protection

Sybil defenses MUST avoid treating unusual participation as proof of automation.

False-positive risks include:

* disability-related interaction patterns;
* shared devices;
* low-bandwidth use;
* offline batch publication;
* translation tools;
* communal networks;
* high-volume legitimate community onboarding;
* privacy-preserving behavior.

Canonical consequences require defined evidence and challenge processes.

## 18.27 AI anomaly detection

AI MAY help identify suspected:

* invitation rings;
* duplicate identities;
* coordinated attestations;
* unusual timing;
* graph concentration;
* synthetic text patterns.

AI output MUST remain non-authoritative until represented through authorized claims, evidence, and human-governed procedures.

An opaque model score MUST NOT directly revoke identity or inviter eligibility.

## 18.28 Defense in depth

Conforming systems SHOULD assume that each individual defense may fail.

The system remains resilient when:

* some false identities are admitted;
* some attestations are wrong;
* some sponsors are compromised;
* some relays are censored;
* some verification providers are corrupt.

No one failure should grant immediate broad authority or unlimited reproduction.

## 18.29 Sybil-resistance explainability

When a Sybil-related rule affects eligibility, a node SHOULD explain:

* the relevant predicate;
* canonical evidence or outcome;
* applicable rulebook;
* affected lane;
* effective position;
* challenge or restoration path.

Protected evidence may remain private.

The canonical commitment and authorized result must remain auditable.

## 18.30 Residual risk

Profile v0 cannot guarantee perfect proof of unique humanity.

It provides a layered system intended to make large-scale false participation:

* slower;
* more expensive in scarce human capacity;
* more visible;
* more challengeable;
* less authoritative;
* less able to reproduce.

Future profiles MAY add stronger privacy-preserving personhood mechanisms.

---

# 19. Cycle integration

## 19.1 Purpose

Canonical cycles provide the deterministic boundaries at which invitation eligibility, capacity generation, maturation, suspension, and restoration become effective.

Invitation authority MUST NOT be generated solely by wall-clock passage.

Cycle integration ensures that admission capacity remains tied to human-authorized canonical progression rather than autonomous machine time.

## 19.2 Human-deliberation dependency

Universal cycles are driven by human deliberation under the Cycle Specification.

Machine activity alone MUST NOT create continuing invitation authority.

Where cycle progress depends on the work measure:

```text
W = V + C
```

time does not independently contribute to `W`.

Invitation-capacity generation MUST therefore depend on an eligible canonical boundary, not merely elapsed time.

## 19.3 Capacity period

Each invitation-capacity period MUST be deterministically associated with a qualifying capacity period.

A qualifying capacity period requires a properly certified human-deliberative cycle under the Cycle Specification.

The period identifier MUST be replay-visible.

A node MUST be able to determine:

* period opening position;
* whether the period qualifies for capacity generation;
* whether `admission_liveness_blocked` is active;
* governing rulebook;
* inviter-eligibility basis;
* generated capacity;
* whether maturation advanced;
* spending events;
* expiration or rollover at close.

## 19.4 Eligibility basis

The active rulebook MUST define the canonical state used to determine inviter eligibility for a capacity period.

The basis SHOULD be anchored to:

* the certified close of the preceding qualifying cycle;
* the opening of the new qualifying cycle;
* another exact qualifying canonical boundary.

This basis is not an `eligibility_snapshot_reference` inside the admission authorization context.

The specification MUST avoid ambiguous mid-cycle eligibility.

## 19.5 Capacity generation boundary

Invitation capacity MUST be generated only at a qualifying boundary authorized by the Cycle Specification.

The boundary MUST identify:

* cycle ID;
* boundary type;
* certification status;
* whether it is a qualifying capacity period;
* active rulebook;
* eligible identities;
* generated integer amounts;
* whether `admission_liveness_blocked` is active.

Generation MUST be replay-derived.

A node-local cron job or database process MUST NOT mint canonical invitation capacity.

## 19.6 Certified-cycle requirement

Profile v0 requires a properly certified human-deliberative cycle before generating new invitation capacity.

A cycle boundary that lacks required human participation, verification, certification, or challenge completion MUST NOT generate ordinary capacity.

Wall-clock passage, cron jobs, AI activity, system emitters, Dmax status, and machine-only boundary production do not create a qualifying capacity period.

This prevents machines from autonomously expanding admission authority during human absence.

## 19.7 Zero-participation and admission-liveness behavior

When there is no qualifying human participation, the cycle stalls or remains non-qualifying under the Cycle Specification.

During such a period, replay MUST expose:

```text
admission_liveness_blocked = true
```

This means:

* no new invitation capacity can be generated;
* maturation cannot advance;
* new inviter eligibility cannot activate;
* invitation restoration cannot activate;
* the admission system depends only on existing valid capacity.

Previously generated spendable capacity remains spendable during a stalled or non-qualifying period unless:

* the identity is suspended;
* the capacity expired under a rule already applicable before the stall;
* a canonical emergency rule freezes spending;
* another explicit constitutional rule applies.

A stalled cycle MUST NOT silently destroy existing capacity solely because no new qualifying boundary occurred.

`admission_liveness_blocked` is a visible protocol liveness failure, not an authorization for machines or operators to mint replacement capacity.

## 19.8 Dmin

Where Dmin acts as an anti-acceleration brake, invitation-capacity generation MUST respect it.

A cycle closed before satisfying the applicable Dmin conditions MUST NOT produce admission authority unless the Cycle Specification defines the closure as constitutionally valid for that effect.

Dmin protects against rapidly cycling the system to mint invitation capacity.

## 19.9 Dmax

Dmax is an anti-stall fallback.

Under Profile v0, a Dmax-only boundary is non-qualifying unless it separately satisfies the required human-deliberative certification rules.

A Dmax-only boundary MUST NOT:

* generate new invitation capacity;
* advance inviter maturation;
* activate new inviter eligibility;
* restore invitation suspension;
* increase carryover caps;
* create admission rewards or authority.

A Dmax-only boundary MAY:

* preserve canonical event ordering;
* preserve historical state;
* record already valid capacity debits;
* support replay and liveness bookkeeping.

Dmax status alone is insufficient to create invitation authority.

## 19.10 Forced, degraded, survivor, and machine-only boundaries

A forced boundary MUST be marked as forced.

Forced, degraded, survivor, record-only, and machine-only boundaries have the same Profile-v0 no-generation rule unless they separately satisfy the required human-deliberative certification rules.

Such a boundary MUST NOT:

* generate new invitation capacity;
* increase inviter eligibility;
* satisfy a maturation cycle;
* restore a suspended inviter;
* increase a rollover cap;
* create admission rewards or authority.

It MAY preserve deterministic ordering, historical state, already valid capacity debits, replay, and liveness bookkeeping.

Forced status remains historical. Later time certification cannot convert a forced boundary into a normal capacity-generating boundary unless the active Cycle Specification separately authorizes that conversion through human-deliberative certification.

## 19.11 Record-only cycles

A record-only cycle MAY preserve:

* event ordering;
* pending challenge state;
* capacity debits already incurred;
* public history;
* admission-liveness bookkeeping.

It MUST NOT create new admission authority, advance maturation, activate inviter eligibility, restore suspension, or increase carryover caps under Profile v0 unless it separately satisfies the required human-deliberative certification rules.

When record-only status leaves no qualifying capacity period, replay MUST expose `admission_liveness_blocked = true`.

## 19.12 Maturation counting

The inviter-maturation rule MUST define which qualifying cycles count.

A counted cycle MUST require:

* valid canonical boundary;
* required human-deliberative certification;
* applicable human participation;
* absence of disqualifying forced, degraded, survivor, record-only, Dmax-only, or machine-only status;
* finality sufficient for the intended eligibility effect.

Maturation MUST be based on cycle identities and canonical positions, not local dates.

## 19.13 Capacity generation formula

The exact generation formula above the constitutional minimum is rulebook-controlled.

For every unsuspended inviter-eligible identity in each qualifying capacity period, the formula MUST satisfy:

```text
generated_capacity >= 1
```

The rulebook MUST produce a deterministic integer output for each eligible identity.

Conceptually:

```text
generated_capacity(identity, cycle)
=
G(
    inviter_eligibility,
    verification_state,
    continuity_state,
    maturation_state,
    admission_suspension_state,
    rulebook_parameters
)
```

The formula MUST NOT depend on private inputs.

A rulebook MAY generate more than one unit, impose finite caps, reduce capacity prospectively after established abuse, or freeze spending under authorized emergency or suspension rules.

A rulebook MUST NOT permanently assign zero capacity to an otherwise eligible and unsuspended class of human identities.

## 19.14 Capacity spending within a cycle

Capacity generated for a period MAY be spent during that period once active.

Previously generated capacity MAY also be spent during stalled or non-qualifying periods unless suspension, prior expiration, authorized emergency freeze, or another explicit constitutional rule blocks spending.

Each successful `identity_create` decreases the available balance by one.

The balance change takes effect at the event's canonical position.

Multiple events in the same cycle are evaluated in canonical order.

## 19.15 Mid-cycle suspension

If inviter eligibility is suspended during a cycle:

* future capacity spending MUST stop at the suspension's effective position;
* already validly applied admissions remain valid;
* unused capacity is frozen, reduced, or expired according to the active rulebook;
* no further capacity is generated while suspension remains active.

The suspension MUST NOT be backdated unless a constitutional invalidity rule explicitly applies.

## 19.16 Mid-cycle restoration

Restoration becomes effective only through an authorized canonical restoration outcome and the qualifying activation boundary required by the active rulebook.

A forced, degraded, survivor, record-only, Dmax-only, or machine-only boundary MUST NOT restore invitation eligibility or capacity unless it separately satisfies the required human-deliberative certification rules.

If no qualifying activation boundary occurs, restoration remains pending and `admission_liveness_blocked` may remain true.

## 19.17 Rulebook transitions

A rulebook transition MUST define:

* which capacity period uses the old formula;
* which period uses the new formula;
* treatment of existing balances;
* treatment of rollover;
* treatment of pending admission candidates;
* treatment of maturation progress;
* treatment of suspension state;
* treatment of `admission_liveness_blocked`.

Rulebook transitions MUST NOT silently rewrite historical balances or debits.

## 19.18 Rollover boundary

At each applicable qualifying capacity boundary, replay MUST compute:

1. unused prior capacity;
2. permitted carryover;
3. expired excess;
4. newly generated capacity;
5. authorized adjustments;
6. new spendable balance.

A non-qualifying boundary MUST NOT increase carryover caps or generate rollover rewards.

The computation order MUST be specified and deterministic.

A recommended order is:

```text
bounded_carryover
=
min(unused_prior_capacity, carryover_cap)

new_balance
=
bounded_carryover + newly_generated_capacity
```

Alternative rules require exact specification.

## 19.19 Pending events at boundary

An authored candidate prepared before a cycle boundary but applied after it MUST be evaluated under the canonical state applicable at its application position.

It does not retain the old balance merely because it was signed earlier.

The candidate may fail due to:

* stale authorization;
* changed rulebook;
* changed eligibility;
* capacity expiration;
* key change.

## 19.20 Network partition

During a partition, different nodes may observe different apparent:

* cycle progress;
* balances;
* pending admissions;
* sponsor eligibility.

Partition-local state is not final canonical authority.

Upon merge or final canonical ordering:

* eligibility is recomputed;
* capacity is recomputed;
* overspending is rejected;
* only valid events remain effective.

Interfaces SHOULD warn that offline capacity views may be stale.

## 19.21 Cycle correction

If a cycle boundary or certification is later found invalid under an authorized correction process, the specification governing correction MUST define effects on:

* generated invitation capacity;
* admissions that spent that capacity;
* maturation;
* eligibility activation.

Because identity creation is durable and high-impact, correction semantics MUST NOT be improvised.

Profile v0 MUST NOT allow provisional capacity to create identities before the underlying boundary is sufficiently final for the required admission effect.

## 19.22 Capacity and Tempo separation

Invitation capacity and Tempo contribution authority are separate.

Tempo claims, evidence, or cycle participation MUST NOT automatically increase invitation capacity unless the rulebook explicitly uses a permitted verification or maturation output.

Verification MUST NOT weight Tempo truth influence.

Cycle participation may establish maturation only under the defined qualifying cycle rules.

## 19.23 Public cycle audit

A node SHOULD expose, for each capacity period:

* cycle ID;
* boundary type;
* certification status;
* whether the period qualifies for capacity generation;
* whether `admission_liveness_blocked` is active;
* governing rulebook;
* identities that became inviter-eligible;
* whether maturation advanced;
* generated capacity;
* whether existing capacity remains spendable;
* carryover;
* expirations;
* suspensions;
* total successful admissions.

Protected verification inputs may remain private.

The derived eligibility, liveness, and capacity results must remain publicly auditable.

## 19.24 Constitutional anti-automation rule

Machines MAY calculate cycle state and capacity.

Machines MUST NOT create the human participation needed to authorize universal invitation-capacity expansion.

AI-authored or system-emitted records MUST NOT substitute for the required human cycle work, verification, or certification.

Profile v0 defines no operator, AI, system-emitter, wall-clock, or machine-only emergency capacity-minting path.
---

# 20. Deterministic replay and conflict resolution

## 20.1 Purpose

Deterministic replay MUST reconstruct the same identity-admission state from the same canonical event history on every conforming node.

Replay MUST determine:

* identity existence;
* admission provenance;
* key state;
* `identity_structural_roots`, including Anthill anchor state;
* verification-lane access;
* inviter eligibility;
* invitation capacity;
* capacity debits;
* suspension;
* restoration;
* admission lineage.

No canonical result may depend on node-local discretion.

## 20.2 Replay inputs

Admission replay may use only authorized inputs, including:

* canonical genesis state;
* finalized canonical events;
* canonical event order;
* active rulebooks;
* certified cycle boundaries;
* raw canonical verification artifacts;
* canonical verification outcomes;
* canonical key state;
* authorized privacy-preserving outcome commitments;
* protocol version and profile state.

Replay MUST NOT use:

* private account state;
* session state;
* mutable environment variables;
* local wall-clock time;
* relay queue order;
* private reputation;
* private Anthill notes;
* opaque AI outputs;
* local moderation decisions.

## 20.3 Replay state

A conforming node MUST be able to derive at least:

```text
identities
identity_provenance
identity_initial_keys
identity_key_history
key_control_state
identity_structural_roots
identity_structural_root_state
anthill_anchors
admission_lineage
identity_kind
raw_verification_artifacts
verification_claims
verification_evidence_relationships
verification_challenges
verification_outcomes
verification_state
vh_certainty
vi_certainty
ordinary_writer_eligibility
ordinary_challenge_eligibility
voter_eligibility
governance_eligibility
tempo_eligibility
inviter_eligibility
qualifying_capacity_periods
admission_liveness_blocked
invitation_capacity_periods
invitation_capacity_generation
invitation_capacity_debits
invitation_capacity_expiration
invitation_capacity_balance
invitation_capacity_spendability
invitation_suspension
invitation_suspensions
invitation_restorations
restricted_verification_lane_eligibility
identity_dormancy_or_recovery_state
```

Materialized tables MAY be used for performance.

They MUST equal replay-derived state.
## 20.4 Event validation position

Each event MUST be validated against canonical state immediately before its own canonical position.

The event’s effects become available only after successful application.

An event MUST NOT validate against state produced by itself unless the event schema explicitly defines atomic internal effects.

## 20.5 `identity_create` application order

For each `identity_create`, replay MUST perform the deterministic validation sequence defined by Appendix A and Section 11.

At minimum, replay must validate:

* event schema;
* sponsor signature;
* sponsor active key;
* sponsor human classification;
* inviter eligibility;
* invitation suspension;
* capacity;
* authorization reference;
* target identity uniqueness;
* key descriptor;
* key reference;
* applicant possession proof;
* conflict state.

Only then may effects be applied.

## 20.6 Atomic application

A successful `identity_create` MUST atomically produce:

* identity;
* event-derived provenance;
* active initial key;
* sponsor lineage;
* `identity_structural_roots`, including Anthill anchor;
* capacity debit;
* ordinary participation lanes initialized inactive;
* restricted verification-lane status.

A replay implementation MUST NOT expose a partial intermediate canonical state.

## 20.7 Identical retries

An identical retry of an already accepted signed candidate MUST resolve to the existing canonical event and effects.

It MUST NOT:

* append another event;
* create another identity;
* activate another key;
* debit another capacity unit;
* create another lineage edge;
* create another identity structural-root set or Anthill anchor.

The exact definition of identical candidate bytes is controlled by the Profile-v0 authorship specification.

## 20.8 Conflicting event ID

If an incoming candidate uses an existing `event_id` but has different signed bytes, replay or ingress MUST reject it as a conflicting duplicate.

The prior canonical event remains authoritative.

No capacity debit occurs for the rejected conflict.

## 20.9 Duplicate identity ID

The first valid canonical event creating an `identity_id` succeeds.

A later distinct event targeting the same `identity_id` MUST be rejected.

The later event MUST NOT:

* update the key;
* replace the sponsor;
* create new verification state;
* debit capacity.

Later identity changes require authorized lifecycle events.

## 20.10 Duplicate initial public key

A Profile-v0 human identity signing key MUST be rejected as a new initial key if it has already been canonically registered as:

* active;
* superseded;
* revoked;
* invalid;
* historically associated with another canonical human identity.

A future profile MAY define an explicit exception with its own canonical safety rule.

## 20.11 Sponsor capacity race

When multiple sponsor events compete for a limited balance, canonical order decides.

Example:

```text
capacity before events = 1

event A applied first → succeeds
capacity becomes 0

event B applied later → insufficient capacity
```

Relay order, local receipt time, and signing time do not change this result.

## 20.12 Multiple sponsors targeting one identity

If different sponsors submit events targeting the same `identity_id`, the first valid canonical event succeeds.

Later events are rejected because the identity exists.

Only the successful sponsor incurs a capacity debit.

The rejected sponsors retain capacity unless another successful event consumes it.

## 20.13 Same applicant key, different target IDs

If the same initial public key appears in events for different target identities, canonical order and key-uniqueness rules apply.

The first valid registration may succeed.

Later conflicting registrations MUST be rejected.

The applicant’s possession proof does not authorize multiple identity bindings.

## 20.14 Same applicant, multiple local candidates

Canonical replay does not know whether multiple local candidates belong to the same person unless canonical claims or evidence establish that fact.

Each canonical event is evaluated under the applicable identity and key rules.

Later duplicate-control claims may affect eligibility prospectively.

Replay MUST NOT infer duplicate humanity from private or non-canonical data.

## 20.15 Sponsor eligibility race

A sponsor may sign an admission before:

* losing eligibility;
* being suspended;
* rotating keys;
* consuming remaining capacity.

The event is evaluated at canonical application.

If the required state is no longer valid, the event fails.

Signing time does not preserve eligibility.

## 20.16 Key-rotation race

If a sponsor rotates its key before an admission event signed by the old key is applied, the admission event MUST be evaluated against key state at its canonical position.

If the old key is superseded or revoked before the event position, the event fails.

If the admission is ordered before the rotation, it may succeed.

## 20.17 Suspension race

If an invitation suspension event and an `identity_create` compete:

* canonical order determines which state applies;
* an admission before effective suspension may succeed;
* an admission after effective suspension fails.

Suspension MUST NOT be applied retroactively unless a separate constitutional invalidity rule explicitly authorizes retroactive treatment.

## 20.18 Rulebook-transition race

If a rulebook transition and an admission event occur near one another:

* canonical order determines the active rulebook;
* authorization references are validated accordingly;
* stale candidates fail;
* no node may choose a preferred rulebook privately.

## 20.19 Cycle-boundary race

If an admission event is canonically ordered around a cycle boundary, the Cycle Specification MUST define whether the event uses:

* closing-period capacity;
* opening-period capacity;
* another exact boundary rule.

The rule MUST be deterministic.

Replay MUST also determine whether the boundary created a qualifying capacity period.

If no qualifying period exists, replay MUST expose `admission_liveness_blocked = true`, generate no new capacity, advance no maturation, and activate no new inviter eligibility.

Existing capacity may remain spendable under the stall rules in Section 19.7.

Wall-clock observations of "before" or "after" are irrelevant.
## 20.20 Arrival-order independence

Two nodes receiving the same valid events in different network orders MUST produce the same final state after canonical ordering.

Local arrival order MUST NOT determine:

* accepted identity;
* sponsor;
* capacity debit;
* key registration;
* eligibility;
* lineage.

## 20.21 Replay rebuild equality

A clean rebuild from genesis and canonical events MUST equal incrementally materialized state.

Equality MUST cover at least:

* identities;
* key states;
* sponsor provenance;
* lineage edges;
* `identity_structural_roots`;
* Anthill anchors;
* capacity balances;
* capacity history;
* eligibility;
* suspension;
* restricted-lane status.

## 20.22 Snapshot compatibility

Snapshots MAY accelerate replay.

A snapshot MUST commit to enough state to reproduce admission behavior, including:

* identity set;
* key history;
* admission provenance;
* capacity periods;
* balance state;
* prior debits;
* eligibility outputs;
* suspensions;
* governing rulebook references.

A node restoring a snapshot and applying later events MUST obtain the same result as full replay.

## 20.23 Protected evidence

Private verification evidence may be represented through authorized commitments or outcomes.

Replay MUST verify the public canonical inputs required by the relevant privacy-preserving procedure.

A private node-local assertion MUST NOT substitute for the authorized canonical commitment or outcome.

Where the underlying evidence cannot be public, a conforming replay MUST rely on an authorized public commitment, threshold result, proof, or privacy-preserving outcome rather than hidden local knowledge.

## 20.24 Invalid event non-effects

A rejected event MUST leave no canonical admission effect.

It MUST NOT:

* create identity rows;
* create key rows;
* create lineage;
* create identity structural-root state;
* debit capacity;
* alter eligibility;
* create private account state as a side effect.

Implementations MUST test transactional rollback.

## 20.25 Stable rejection ordering

Where one event violates multiple rules, Appendix A or the API contract SHOULD define stable rejection precedence.

Stable precedence improves:

* interoperability;
* testing;
* user explanation;
* replay consistency.

The chosen precedence MUST NOT alter the underlying non-effects of rejection.

## 20.26 Deterministic graph derivation

Admission-lineage graphs and Anthill-derived metrics used canonically MUST specify:

* graph nodes;
* graph edges;
* edge direction;
* included event types;
* depth;
* time or cycle window;
* weighting;
* cluster algorithm;
* rulebook version.

A non-deterministic machine-learning cluster label MUST NOT directly determine canonical eligibility.

## 20.27 Derived certainty and boundary activation

Verification certainty affecting inviter eligibility MUST be activated according to the Verification and Cycle Specifications.

Replay MUST distinguish:

* raw canonical verification artifacts;
* ordinary verification truth claims;
* ordinary verification evidence relationships;
* contradiction relationships;
* challenge outcomes;
* derived VH or VI certainty;
* boundary activation;
* event-family-specific eligibility output;

An event MUST NOT gain authority from a certainty change that has not yet become active.

## 20.28 Legacy compatibility

Replay MUST preserve legacy identity classifications.

A legacy identity lacking event-derived sponsor provenance MUST NOT be assigned fabricated lineage.

The applicable migration or compatibility profile MUST state whether the identity may:

* read public state;
* manage keys;
* write ordinary events;
* become inviter-eligible;
* register future event-derived authority.

## 20.29 Genesis state

Genesis-admitted identities and initial capacity MUST be explicit in authoritative genesis state.

Replay MUST distinguish genesis admission from post-genesis `identity_create`.

Genesis authority MUST NOT be reconstructed from operator database rows unavailable to other nodes.

## 20.30 Error explainability

For each rejected admission, a node SHOULD provide a stable public-safe explanation.

The explanation MAY identify:

* event ID;
* target identity;
* sponsor identity;
* rejection code;
* applicable rulebook;
* canonical position;
* retry possibility.

It MUST NOT expose:

* private keys;
* private evidence;
* credentials;
* session data;
* protected relay metadata.

## 20.31 No hidden repair

A node MUST NOT silently repair an invalid admission by:

* replacing the sponsor;
* changing the target ID;
* substituting a key;
* removing invalid fields;
* charging another capacity period;
* creating a private operator exception.

The applicant and sponsor must submit a new valid candidate where correction is possible.

## 20.32 Canonical finality

Once a valid admission is finalized under the applicable protocol rules, later replay MUST preserve:

* identity existence;
* original sponsor provenance;
* original key-registration history;
* original capacity debit;
* original canonical position.

Later events may alter future key, verification, eligibility, or suspension state.

They do not rewrite the historical admission.

# 21. Genesis and bootstrap boundary

## 21.1 Purpose

A decentralized identity-admission system requires an explicit starting state.

Normal Profile-v0 admission depends on:

* an existing canonical sponsor;
* an active sponsor key;
* inviter eligibility;
* available invitation capacity;
* a governing rulebook;
* a canonical cycle state.

Those conditions cannot all be produced through ordinary post-genesis `identity_create` events because the first canonical identities do not yet have existing sponsors.

Profile v0 therefore distinguishes:

1. authoritative genesis admission;
2. normal post-genesis event-derived admission;
3. legacy operator-provisioned state.

Genesis admission is a narrowly bounded initialization mechanism.

It MUST NOT become a continuing administrative path for creating identities after genesis.

## 21.2 Authoritative genesis state

The initial canonical state MUST be defined by an authoritative, versioned, publicly inspectable genesis artifact.

The genesis artifact MUST commit to all initial admission-relevant state required for deterministic replay, including where applicable:

* genesis identity IDs;
* identity classifications;
* initial key descriptors;
* initial public-key references;
* key-profile versions;
* initial key status;
* identity provenance classification;
* initial `identity_structural_roots` or their deterministic derivation rule, including Anthill anchors;
* initial verification-state admissions;
* initial writer-eligibility state;
* initial inviter-eligibility state;
* initial invitation capacity;
* initial cycle state;
* initial rulebook references;
* initial system and constrained-emitter identities;
* protocol and profile versions.

A conforming node MUST be able to reconstruct the same initial state from the same genesis artifact.

## 21.3 Genesis commitment

The genesis artifact MUST have a canonical commitment or identifier.

The commitment MUST bind:

* complete genesis contents;
* ordering;
* protocol version;
* applicable profiles;
* key descriptors;
* initial rulebooks;
* initial admission-capacity state.

A node MUST reject a replay whose genesis commitment differs from the network or deployment profile it claims to follow.

Genesis MUST NOT depend on:

* an undisclosed database dump;
* private operator configuration;
* local environment variables;
* mutable account records;
* undocumented initialization scripts;
* secrets unavailable to other conforming nodes.

## 21.4 Genesis-admitted identities

A genesis-admitted identity is not created by a normal post-genesis `identity_create` event.

Its provenance MUST be recorded as:

```text
genesis_admitted
```

Replay MUST distinguish this provenance from:

```text
event_derived
```

and:

```text
legacy_operator_provisioned
```

A genesis-admitted identity MAY have:

* an active initial key;
* an initial verification state;
* writer eligibility;
* inviter eligibility;
* initial invitation capacity

only where those states are explicitly admitted by the authoritative genesis artifact or deterministically derived from its rulebooks.

## 21.5 No fabricated genesis authorship

A genesis-admitted identity MUST NOT be assigned a fabricated sponsor.

A node MUST NOT create:

* a fictional `identity_create` event;
* a fictional applicant possession proof;
* a fictional invitation-capacity debit;
* a fictional admission-lineage edge;
* a fictional verification attestation

to make genesis admission appear equivalent to ordinary post-genesis admission.

Genesis provenance is valid because it is explicitly admitted by the protocol’s starting state, not because it imitates later admission history.

## 21.6 Genesis key admission

Every genesis identity expected to sign Profile-v0 authored candidates MUST have sufficient canonical key material to reconstruct and validate its initial authorized key state.

The genesis artifact MUST include or canonically reference:

```text
key_profile_version
signature_algorithm
raw_public_key_bytes
owning_identity_id
public_key_ref
initial_key_status
```

For an active Profile-v0 human signing key:

```text
key_profile_version = ed25519_v0
signature_algorithm = ed25519
owning_identity_id = genesis identity_id
```

The public-key reference MUST equal the canonical hash of the key descriptor.

A missing, malformed, or unreconstructable genesis key MUST NOT authorize new Profile-v0 writes.

## 21.7 Genesis possession proof

A normal applicant possession proof is designed for sponsor-authored post-genesis admission.

Genesis identities do not have ordinary sponsor-authored creation events.

The genesis profile MUST state whether control of each initial private key was established through:

* a genesis-specific proof-of-possession artifact;
* a pre-genesis signing ceremony;
* a genesis manifest signature;
* another publicly verifiable bootstrap procedure.

The procedure MUST be:

* versioned;
* auditable;
* domain-separated from ordinary `identity_create`;
* sufficient to prevent accidental registration of unusable keys.

A node MUST NOT infer possession merely from the presence of a public key where the genesis profile requires proof.

## 21.8 Initial human classification

The genesis artifact MUST distinguish among:

* human identities;
* AI identities;
* system identities;
* constrained emitters;
* organizations or other non-human identities.

A non-human genesis identity MUST NOT become an ordinary sponsor merely because it exists in genesis.

Human classification at genesis does not automatically establish indefinite verified-human status.

The Verification Specification MUST define whether initial human-verification state is:

* constitutionally admitted;
* time-bounded;
* challengeable;
* subject to later re-evaluation;
* replaced by ordinary event-derived verification after a transition period.

## 21.9 Initial verification state

Where genesis identities begin with verification state, the genesis artifact MUST identify:

* the admitted verification profile;
* the initial VH state;
* any admitted VI state;
* the scope of the state;
* the rulebook governing later updates;
* whether the state is challengeable;
* the boundary at which ordinary verification rules take over.

Genesis verification MUST NOT be described as event-derived unless supporting canonical verification artifacts actually exist.

If `identity_verification_update` or a successor compatibility record is used during genesis or import, it MUST be classified as a genesis/import compatibility admission of state, not as ordinary post-genesis proof that the verification predicate is true.

Such a compatibility record MUST identify its provenance class, scope, challengeability or transition rule, and retirement condition.

## 21.10 Initial writer eligibility

Genesis identities MAY begin with ordinary writer eligibility only where explicitly admitted by the genesis profile.

Initial writer eligibility MUST be:

* public;
* deterministic;
* scoped by event family;
* separate from key control;
* separate from inviter eligibility.

A node operator MUST NOT privately grant additional genesis-era writer eligibility after the network has begun.

## 21.11 Initial inviter eligibility

Some initial human identities may need inviter eligibility so normal post-genesis admission can begin.

The genesis artifact MUST explicitly identify:

* which identities are initially inviter-eligible;
* which rule authorizes that eligibility;
* whether maturity requirements are initially waived;
* when ordinary eligibility derivation replaces the bootstrap state;
* any limits on the bootstrap exception.

Genesis status alone MUST NOT create permanent inviter eligibility.

After the bootstrap transition, genesis identities MUST satisfy the same ordinary inviter rules as other identities unless a constitutional rule explicitly states otherwise.

## 21.12 Initial invitation capacity

The genesis artifact or initial rulebook MUST define the exact initial invitation capacity of each initially eligible inviter.

Initial capacity MUST be:

* finite;
* non-negative;
* publicly inspectable;
* replay-compatible;
* identity-bound;
* non-transferable;
* non-saleable.

Unlimited genesis capacity is prohibited.

The initial allocation SHOULD be only large enough to establish a viable decentralized admission population without giving a small founding group indefinite control over expansion.

After bootstrap transition, ordinary Profile-v0 capacity generation is subject to the qualifying-period and positive-capacity rules in Sections 10 and 19.
## 21.13 Distribution of initial capacity

Initial invitation capacity SHOULD be distributed across multiple independent human identities where feasible.

The genesis design SHOULD avoid concentrating all initial capacity in:

* one person;
* one organization;
* one jurisdiction;
* one political group;
* one technical operator;
* one institution.

This distribution is a security and anti-capture objective.

The exact initial distribution remains part of the adopted genesis profile.

## 21.14 Bootstrap maturation exception

Ordinary inviter eligibility requires maturation across qualifying certified human-deliberative cycles.

Initial genesis inviters cannot satisfy post-genesis maturation before the system begins.

The genesis profile MAY define a bounded bootstrap exception.

The exception MUST specify:

* eligible identities;
* scope;
* capacity limit;
* expiration condition;
* transition to ordinary eligibility;
* whether additional verification is required after launch.

The exception MUST NOT silently become permanent.

Forced, degraded, survivor, record-only, Dmax-only, or machine-only boundaries MUST NOT extend a bootstrap exception or advance ordinary maturation unless they separately satisfy the required human-deliberative certification rules.
## 21.15 Bootstrap transition

The genesis profile MUST define the condition at which bootstrap inviter authorization ends and normal rulebook-derived inviter eligibility becomes authoritative.

The transition MAY occur after:

* a specified number of certified cycles;
* sufficient growth in independently verified humans;
* a canonical protocol milestone;
* another deterministic condition.

The transition MUST be replay-visible.

A private declaration by an operator is insufficient.

## 21.16 No continuing operator admission

After genesis, an operator MUST NOT create a canonical human identity through:

* direct database insertion;
* migration side effects;
* private administrative commands;
* account creation;
* undisclosed bootstrap scripts;
* privileged API routes;
* manual key-table updates.

Post-genesis human identity creation MUST use an authorized canonical admission profile.

Profile v0 uses `identity_create`.

## 21.17 No operator-created inviter grants

An operator MUST NOT grant post-genesis inviter eligibility or invitation capacity through private state.

Any post-genesis inviter-eligibility change MUST derive from:

* verification state;
* maturation;
* cycle state;
* active rulebook;
* canonical suspension or restoration outcomes;
* another explicitly authorized canonical event.

## 21.18 Genesis identity structural roots

Every genesis identity MUST have the complete protocol-defined `identity_structural_roots` set required by the current profile.

The genesis artifact MUST either:

1. include the required roots explicitly; or
2. invoke the same deterministic root derivation rules used for event-derived identities.

Genesis and event-derived identities MUST use compatible identity structural-root semantics.

Genesis status MUST NOT provide a structurally privileged root class.

## 21.19 Genesis admission lineage

Genesis identities ordinarily have no sponsor lineage.

Their provenance root is the genesis artifact.

Interfaces MAY display genesis identities as roots of later admission lineages.

They MUST NOT imply:

* self-sponsorship;
* sponsorship by the operator;
* sponsorship by another genesis identity

unless the authoritative genesis profile explicitly records such provenance.

## 21.20 Genesis challenges

The protocol SHOULD permit later challenge of claims associated with genesis identities, including where applicable:

* current human control;
* identity continuity;
* key compromise;
* duplicate control;
* ongoing eligibility.

A challenge MUST NOT erase the fact that the identity was present in genesis.

It MAY alter future verification or eligibility.

## 21.21 Genesis key compromise

If a genesis key is compromised after launch, ordinary key-lifecycle rules SHOULD apply.

The identity SHOULD be able to:

* rotate keys;
* revoke eligible keys;
* submit compromise evidence;
* use an authorized recovery path where defined.

Genesis status MUST NOT permit secret operator replacement of canonical keys.

## 21.22 Genesis correction boundary

Errors in genesis are especially consequential.

The authoritative protocol MUST define whether a genesis artifact may ever be corrected and, if so:

* who authorizes the correction;
* how a new genesis commitment is selected;
* whether the correction creates a distinct network or protocol history;
* how prior events are treated;
* how clients identify the change.

A node MUST NOT silently modify genesis state during migration or startup.

## 21.23 Multiple deployments

Different deployments MAY use different authoritative genesis artifacts.

A node MUST identify which genesis commitment it follows.

Identities admitted in one deployment are not automatically canonical in another deployment unless an interoperability profile explicitly defines recognition.

## 21.24 Civilizational transition objective

The genesis mechanism exists only to begin the decentralized system.

The intended progression is:

```text
explicit bounded genesis authority
→ normal event-derived verification
→ broadly distributed inviter eligibility
→ no continuing dependence on founders or operators
```

The protocol SHOULD minimize the period during which bootstrap exceptions are necessary.

---

# 22. Legacy compatibility

## 22.1 Purpose

Profile v0 may be introduced after identities, keys, writer permissions, accounts, or identity-related structures already exist in earlier implementations.

Compatibility rules must preserve valid historical state without falsely claiming that legacy state was produced through Profile-v0 admission.

Migration MUST be additive and provenance-preserving.

## 22.2 Legacy classifications

Every identity present at the Profile-v0 transition MUST be classifiable as one of:

```text
genesis_admitted
legacy_operator_provisioned
event_derived
```

A later compatibility profile MAY add more precise categories.

Each identity MUST have exactly one primary admission-provenance classification for a given canonical history.

## 22.3 Event-derived identity

An identity may be classified as `event_derived` only when replay can reconstruct a valid canonical identity-creation event and its required effects.

For Profile v0 this includes:

* canonical sponsor authorship;
* applicant initial-key possession proof;
* invitation authorization;
* capacity debit;
* canonical creation position;
* admission lineage;
* initial key state;
* initially inactive ordinary-authority eligibility lanes.

An implementation MUST NOT use `event_derived` merely because a database row resembles the expected result.

## 22.4 Legacy operator-provisioned identity

A `legacy_operator_provisioned` identity is an identity introduced through an earlier implementation-specific or administrative process lacking complete canonical admission provenance.

The identity MAY remain readable and historically usable according to the compatibility profile.

Its provenance MUST remain explicit.

It MUST NOT be presented as:

* sponsor-authored;
* invitation-capacity-backed;
* applicant-proof-backed;
* event-derived

unless those facts are actually supported by canonical history.

## 22.5 No fabricated migration history

A migration MUST NOT fabricate:

* `identity_create` events;
* sponsor identities;
* invitation-capacity debits;
* admission-authorization references;
* initial-key possession proofs;
* verification attestations;
* admission-lineage edges;
* event timestamps;
* canonical positions;
* challenge outcomes.

Missing history must remain classified as missing or legacy.

Synthetic history would undermine deterministic replay and public auditability.

## 22.6 Legacy identity readability

A legacy identity SHOULD remain publicly readable where its earlier state was validly admitted by the prior deployment.

Public reads SHOULD indicate:

* identity ID;
* legacy provenance class;
* known key state;
* known historical authorship;
* current eligibility;
* whether Profile-v0 admission provenance is absent;
* any compatibility restrictions.

A user MUST NOT need a private account merely to inspect legacy identity state.

## 22.7 Historical event validity

Events historically accepted under an earlier authorized profile MUST be evaluated according to the profile and key state applicable at their canonical positions.

Profile-v0 adoption MUST NOT automatically invalidate all earlier authored events merely because they lack Profile-v0 admission provenance.

The replay specification MUST identify:

* which historical signature profiles remain recognized;
* which legacy events remain readable;
* which old identities may authorize new events;
* where compatibility ends.

## 22.8 New Profile-v0 writes

A legacy identity MUST NOT submit new Profile-v0 authored events unless replay can establish all required current authorization state.

This may require:

* a valid active Profile-v0 key;
* current human classification;
* applicable verification state;
* event-family eligibility;
* inviter eligibility where admission is attempted;
* another compatibility prerequisite.

Legacy existence alone does not grant new Profile-v0 authority.

## 22.9 Legacy key descriptors

A legacy identity with sufficient canonical key material MAY have that material classified under a supported compatibility key profile.

A key MUST NOT be treated as Profile-v0 active unless replay can reconstruct:

* raw public-key bytes;
* algorithm;
* profile version;
* owner identity;
* public-key reference;
* activation authority;
* current status.

Where this information is missing, the identity remains readable but the key cannot authorize new Profile-v0 writes.

## 22.10 Key registration transition

A later authoritative specification MAY define a canonical transition by which a legacy identity registers a Profile-v0 key.

Such a transition must solve:

* authorization by the legacy identity;
* proof of continuity;
* key possession;
* duplicate-control risk;
* current verification;
* historical provenance.

This admission specification does not invent that path.

Until it exists, an insufficiently described legacy identity may remain read-only.

## 22.11 Legacy writer eligibility

Legacy writer grants MAY remain recognized temporarily under a documented compatibility profile.

The profile MUST state:

* source of the grant;
* event families affected;
* activation position;
* expiration or transition condition;
* whether the grant is canonical, genesis-admitted, or implementation transitional.

A private mutable `canonical_writer_level` row, or any similarly named transitional materialized compatibility field, MUST NOT be described as permanent protocol authority, final writer-eligibility authority, or a substitute for replay-derived event-family eligibility. It remains subject to explicit migration and retirement rules.

## 22.12 Legacy verification updates

`identity_verification_update` MUST NOT be used as an ordinary post-genesis public event that directly declares a person verified or directly enables ordinary authorship.

For normal post-genesis operation, verification state MUST derive from ordinary canonical verification truth claims, evidence, contradiction relationships, challenges, outcomes, and rulebooks.

If retained at all, `identity_verification_update` is limited to explicit genesis/import/legacy compatibility treatment.

A compatibility use:

* does not create objective truth by declaration;
* must identify its provenance class;
* must not masquerade as event-derived verification evidence;
* must not remain a continuing operator-controlled status-setting path;
* must have a defined transition or retirement rule.

## 22.13 Legacy inviter eligibility

A legacy identity MUST NOT become inviter-eligible merely because it had ordinary writer eligibility.

Invitation authority requires its own explicit compatibility rule.

A temporary legacy inviter allowance, if unavoidable, MUST be:

* public;
* bounded;
* replay-visible;
* finite;
* transitional;
* replaced by ordinary rulebook-derived eligibility.

Private operator selection of legacy inviters is prohibited as a continuing admission mechanism.

## 22.14 Legacy invitation capacity

Any invitation capacity admitted during migration MUST be explicitly committed through:

* a versioned migration manifest;
* an authoritative transition event;
* a genesis-like profile transition;
* another replay-visible mechanism.

A migration MUST NOT infer capacity from:

* account age;
* social popularity;
* database role;
* administrator status;
* number of prior ideas;
* node ownership.

## 22.15 Legacy admission lineage

Where no authentic sponsor provenance exists, lineage MUST remain absent or marked unknown.

An implementation MUST NOT use account creator, database owner, inviter email, or operator identity as a substitute sponsor unless that relationship was itself canonical under the earlier protocol.

Unknown lineage is preferable to fabricated lineage.

## 22.16 Identity structural-root compatibility

Every legacy identity must ultimately have the required `identity_structural_roots` if the current profile requires them.

Where roots, including Anthill anchors, are deterministically derived from `identity_id`, legacy identities MAY receive those roots through that derivation without fabricating a historical event.

Where roots require explicit canonical objects, the compatibility profile MUST define a deterministic migration effect and classify its provenance accurately.

The migration MUST NOT invent historical structural roots, Anthill relationships, or verification artifacts.

## 22.17 Legacy social data

Private or implementation-specific social relationships MUST NOT automatically become canonical Anthill edges.

Migration MAY preserve private user state in the product layer.

Canonical conversion requires:

* explicit authorized event types;
* required signatures;
* counterparty acknowledgment where applicable;
* deterministic provenance.

## 22.18 Legacy verification state

Legacy verification flags MAY be preserved only under an explicit compatibility classification.

The system MUST distinguish among:

* raw canonical verification artifacts;
* canonically derived VH or VI certainty;
* genesis-admitted verification;
* legacy operator-attested verification;
* unknown or absent verification.

A legacy boolean such as `verified = true` or a legacy `identity_verification_update` row MUST NOT be silently reclassified as challengeable Profile-v0 VH or VI certainty.

## 22.19 Verification transition

A later verification-transition rule MAY require legacy identities to accumulate ordinary Profile-v0 verification truth claims, evidence, contradictions, challenges, and outcomes over time.

The rule MAY:

* preserve temporary existing eligibility;
* reduce eligibility gradually;
* require re-verification before inviter eligibility;
* impose a transition deadline;
* permit challenge of old status.

The transition must be deterministic and publicly documented.

## 22.20 Private accounts

Legacy private accounts MAY remain linked to identities for product functionality.

They MUST NOT determine:

* canonical identity existence;
* active key state;
* verification certainty;
* writer eligibility;
* inviter eligibility;
* invitation capacity.

Loss or deletion of a private account MUST NOT erase canonical identity history.

## 22.21 Account-created identity rows

An identity row created as a side effect of legacy account registration MUST be classified according to its actual provenance.

It MUST NOT automatically qualify as event-derived.

If the row lacks sufficient canonical identity authority, the compatibility profile must state its restrictions.

## 22.22 Legacy system and AI identities

Legacy non-human identities MUST be classified explicitly.

They MUST NOT gain human sponsor eligibility through migration.

A migration MUST preserve distinctions among:

* human identities;
* AI identities;
* system identities;
* constrained emitters;
* organizations;
* unknown legacy identity types.

Unknown identity type MUST NOT default to human for inviter eligibility.

## 22.23 Additive schema migration

Database migration for Profile v0 MUST be additive where practical.

It SHOULD preserve:

* identity IDs;
* key rows;
* historical events;
* authorship references;
* account links;
* existing public reads;
* legacy classification.

It MUST NOT destructively rewrite history merely to fit the new schema.

## 22.24 Materialized compatibility fields

Implementations MAY add materialized fields such as:

* `admission_provenance_class`;
* `profile_v0_key_ready`;
* `event_derived_identity`;
* `anthill_anchor_ref`;
* `legacy_write_compatibility`;
* `legacy_inviter_compatibility`.

These fields are caches or classifications.

Their values MUST be reproducible from authoritative migration inputs and canonical state.

## 22.25 Migration manifest

Where a migration admits compatibility state not derivable from ordinary events, the migration SHOULD use a versioned manifest.

The manifest SHOULD commit to:

* affected identity IDs;
* provenance classification;
* admitted key state;
* admitted eligibility state;
* temporary compatibility rules;
* transition deadlines;
* manifest hash.

The manifest MUST be publicly inspectable for open-core canonical state.

## 22.26 No hidden migration exceptions

A migration MUST NOT grant special authority based on an undisclosed allowlist.

Any identity-specific compatibility exception MUST be:

* explicit;
* canonical or manifest-admitted;
* publicly explainable;
* narrowly scoped;
* subject to expiration where appropriate.

## 22.27 Replay from pre-Profile-v0 state

A conforming migration test MUST prove that a database or snapshot from before Profile v0 can be upgraded without:

* losing historical events;
* changing valid identity IDs;
* fabricating admission history;
* granting unintended eligibility;
* converting private state into canonical authority;
* breaking public reads.

## 22.28 Export compatibility

The generated open-core export MUST include:

* legacy provenance classifications;
* additive migrations;
* public compatibility reads;
* replay rules;
* boundary protections.

It MUST NOT require private account code to interpret canonical legacy identities.

## 22.29 Eventual retirement of compatibility paths

Transitional compatibility paths SHOULD have explicit retirement conditions.

For example:

* temporary writer grants may expire after re-verification;
* temporary inviter allowances may end after ordinary eligibility becomes available;
* unsupported key profiles may become read-only;
* operator-provisioned identity creation routes may be removed.

Retirement MUST NOT delete historical identity state.

## 22.30 No forced historical homogenization

Civilizational-scale history may contain multiple legitimate protocol eras.

The system SHOULD preserve those distinctions rather than pretending every identity entered through one modern mechanism.

The public model may therefore include:

```text
genesis-admitted identities
legacy identities
event-derived Profile-v0 identities
future-profile identities
```

Deterministic provenance is more important than superficial uniformity.

---

# 23. Stable validation and rejection rules

## 23.1 Purpose

Stable validation and rejection rules ensure that conforming nodes:

* accept the same identity-admission events;
* reject the same invalid events;
* report interoperable errors;
* apply no partial effects;
* avoid leaking protected evidence.

Appendix A and the API contract MUST define the final canonical error registry.

This section defines the required semantic categories and minimum stable identifiers.

## 23.2 Validation outcome classes

An admission submission MUST result in one of:

```text
accepted
idempotent_existing
rejected
temporarily_unavailable
```

`temporarily_unavailable` is an ingress or operational result, not a canonical validation result.

A canonical invalid event is `rejected`, not temporarily unavailable.

## 23.3 Accepted event

An event is `accepted` only when:

* all structural validation passes;
* all cryptographic validation passes;
* all sponsor authorization passes;
* all capacity validation passes;
* all target and key validation passes;
* the event is atomically appended and materialized.

Acceptance MUST produce the complete canonical effects defined in Section 11.

## 23.4 Idempotent existing event

An identical retry of an already accepted signed candidate MUST return:

```text
idempotent_existing
```

or an equivalent stable success result.

It MUST reference the existing canonical event.

It MUST NOT debit capacity again.

A similar payload with different signed bytes is not idempotent.

## 23.5 Rejected event

A rejected event MUST produce no canonical effects.

It MUST NOT:

* append an accepted canonical event;
* create an identity;
* register a key;
* create identity structural-root state;
* create lineage;
* debit capacity;
* create verification state;
* create private product state as an admission side effect.

## 23.6 Temporarily unavailable

Ingress MAY return a temporary operational failure when the node cannot safely determine or commit the canonical result because of:

* storage outage;
* unavailable canonical state;
* unresolved database transaction failure;
* node synchronization state;
* another operational condition.

A temporary failure MUST NOT be reported as canonical rejection.

The client MAY retry the same signed candidate.

## 23.7 Primary rejection code

A rejected event MUST have one stable primary rejection code.

An implementation MAY include secondary diagnostics.

Secondary diagnostics MUST NOT alter the primary deterministic result.

The rejection-precedence rules MUST ensure that conforming nodes select the same primary code where one event violates multiple rules.

## 23.8 Recommended rejection precedence

Unless Appendix A adopts another exact order, nodes SHOULD determine the primary rejection in this sequence:

1. malformed envelope or encoding;
2. unsupported event or profile;
3. event-ID conflict or exact idempotency;
4. invalid sponsor signature;
5. invalid sponsor identity or key;
6. invalid sponsor event-family authorization;
7. invalid admission authorization or capacity;
8. invalid target identity;
9. invalid key descriptor or key reference;
10. invalid applicant possession proof;
11. invalid verification reference;
12. canonical-order state conflict;
13. transactional application failure.

The final order MUST be normative before interoperability testing.

## 23.9 Structural and encoding errors

The stable registry MUST include errors equivalent to:

```text
malformed_canonical_event
forbidden_additional_field
missing_required_field
invalid_field_encoding
invalid_field_length
non_canonical_encoding
unsupported_event_type
unsupported_admission_profile
unsupported_signature_profile
```

A node MUST reject non-canonical alternate encodings even where they represent similar semantic data.

## 23.10 Event identity errors

The stable registry MUST include:

```text
conflicting_duplicate_event
invalid_event_id
event_id_payload_mismatch
```

An exact retry is not an error.

A reused `event_id` with different signed bytes MUST return `conflicting_duplicate_event`.

## 23.11 Sponsor signature errors

Required sponsor-signature errors include:

```text
missing_signature
malformed_signature
invalid_signature
unknown_author_identity
author_identity_mismatch
unknown_author_key
author_key_wrong_owner
author_key_inactive
author_key_superseded
author_key_revoked
```

Errors SHOULD distinguish inactive, superseded, and revoked key states where public disclosure creates no security problem.

## 23.12 Sponsor classification errors

Required sponsor errors include:

```text
sponsor_not_human
ai_sponsor_forbidden
system_sponsor_forbidden
self_sponsorship_forbidden
```

Unknown identity type MUST NOT default to human.

## 23.13 Inviter-eligibility errors

Required inviter-state errors include:

```text
inviter_ineligible
inviter_not_mature
inviter_verification_insufficient
inviter_continuity_insufficient
inviter_suspended
inviter_eligibility_not_active
```

A public response MUST NOT reveal private verification evidence.

It MAY report only the failed public predicate or an authorized summary.

## 23.14 Capacity errors

Required capacity errors include:

```text
insufficient_invitation_capacity
invitation_capacity_not_active
invitation_capacity_frozen
invitation_capacity_period_invalid
invitation_capacity_period_closed
invitation_capacity_double_spend
```

Where canonical ordering simply leaves a zero balance, `insufficient_invitation_capacity` SHOULD be the ordinary result.

`admission_liveness_blocked` is a replay status, not a replacement for a capacity error. A rejection caused by no current spendable capacity MUST still report the applicable capacity predicate.

`invitation_capacity_double_spend` may be reserved for an explicit reuse of a uniquely identified authorization where such identification exists.
## 23.15 Admission-authorization errors

Required errors include:

```text
missing_admission_authorization
malformed_admission_authorization
invalid_admission_authorization
admission_authorization_sponsor_mismatch
admission_authorization_profile_mismatch
admission_authorization_period_unknown
admission_authorization_rulebook_unknown
stale_admission_authorization
```

The API MUST NOT expose secret eligibility evidence through these errors.

Sponsor key failure, inviter ineligibility, invitation suspension, insufficient capacity, duplicate target identity, and duplicate public key MUST use their own stable errors rather than being collapsed into admission-authorization errors.

## 23.16 Target-identity errors

Required target errors include:

```text
invalid_identity_id
identity_already_exists
identity_reserved_by_genesis
target_identity_equals_sponsor
identity_type_invalid
```

A non-canonical request does not reserve an identity ID.

## 23.17 Initial-key descriptor errors

Required key errors include:

```text
missing_initial_key_descriptor
malformed_initial_key_descriptor
unsupported_initial_key_profile
unsupported_initial_signature_algorithm
initial_key_length_invalid
initial_key_owner_mismatch
initial_public_key_ref_mismatch
public_key_already_registered
public_key_reuse_forbidden
```

Nodes MUST recompute `initial_public_key_ref`.

## 23.18 Applicant possession-proof errors

Required errors include:

```text
missing_initial_key_possession_proof
malformed_initial_key_possession_proof
invalid_initial_key_possession_proof
possession_proof_event_mismatch
possession_proof_identity_mismatch
possession_proof_sponsor_mismatch
possession_proof_key_mismatch
possession_proof_authorization_mismatch
possession_proof_verification_reference_mismatch
```

A request-stage possession proof used as the final proof SHOULD resolve to `invalid_initial_key_possession_proof` or a more specific profile error.

## 23.19 Verification-reference errors

Required errors include:

```text
invalid_verification_reference
unsupported_verification_reference_type
verification_reference_unresolvable
verification_reference_privacy_violation
verification_reference_private_pointer_forbidden
verification_reference_no_reference_encoding_invalid
```

The canonical no-reference state MUST be accepted where the active admission profile permits admission without a verification reference.

A non-canonical alternate no-reference encoding MUST be rejected.

A valid reference MUST NOT be treated as verified-human proof by the admission validator.

## 23.20 Anthill-anchor errors

Where the anchor is deterministic, conflicting anchor state MUST produce an error equivalent to:

```text
anthill_anchor_conflict
```

Where the anchor is materialized, creation MUST fail if an incompatible authoritative anchor already exists.

A duplicate materialization from an exact idempotent retry is not an error.

## 23.21 Restricted-lane initialization errors

If required restricted verification-lane state cannot be initialized atomically, admission MUST fail.

The stable error may be:

```text
restricted_verification_lane_initialization_failed
```

An identity MUST NOT be created in a state where it cannot participate in its own required verification process.

## 23.22 Canonical conflict errors

Required state-conflict errors include:

```text
canonical_state_conflict
target_created_by_prior_event
sponsor_state_changed
rulebook_state_changed
cycle_state_changed
```

Implementations SHOULD prefer the most specific stable error where deterministic.

## 23.23 Legacy compatibility errors

Where a legacy identity attempts unsupported Profile-v0 activity, errors may include:

```text
legacy_identity_profile_v0_key_required
legacy_identity_verification_transition_required
legacy_identity_inviter_transition_required
legacy_identity_write_profile_unsupported
```

These errors SHOULD identify the required transition without fabricating authority.

## 23.24 Error privacy

Error responses MUST NOT expose:

* private keys;
* passwords;
* sessions;
* full private evidence;
* hidden identity documents;
* private relay routes;
* private contact details;
* secret anti-abuse thresholds where disclosure is prohibited by an authorized safety rule.

The public result must still be sufficiently stable for interoperability and challenge.

## 23.25 Error explainability

Where safe, a rejection SHOULD include:

* stable code;
* event ID;
* sponsor identity;
* target identity;
* canonical position or evaluation context;
* retryability;
* governing profile;
* governing rulebook reference.

An explanation MUST distinguish:

* malformed request;
* canonical invalidity;
* temporary node failure.

## 23.26 Retryability

Each error SHOULD be classified as:

```text
retry_same_candidate
retry_with_new_candidate
not_retryable
```

Examples:

* temporary storage outage: `retry_same_candidate`;
* stale authorization: `retry_with_new_candidate`;
* invalid signature: `retry_with_new_candidate`;
* identity already exists: normally `not_retryable`;
* conflicting duplicate event: `not_retryable`.

## 23.27 No hidden normalization

A node MUST NOT silently correct:

* malformed key bytes;
* identity IDs;
* sponsor IDs;
* key references;
* authorization references;
* possession proofs;
* forbidden fields.

The signed candidate must validate as submitted.

## 23.28 Conformance requirements

Conformance fixtures MUST verify:

* exact primary rejection codes;
* stable precedence;
* no side effects;
* privacy-safe errors;
* retryability classification;
* identical results across implementations.

Fixtures SHOULD include events violating multiple rules to test precedence.

## 23.29 API mapping

HTTP or transport status codes MAY differ from canonical error identifiers.

The API contract MUST define a stable mapping.

For example:

* accepted: success;
* idempotent existing: success;
* malformed candidate: client error;
* canonical authorization failure: client or authorization error;
* temporary node failure: service error.

Canonical meaning must not depend on HTTP status alone.

---

# 24. Security and abuse analysis

## 24.1 Purpose

This section is primarily informative security analysis.

It identifies major threats to Profile-v0 identity admission and explains consequences of normative rules defined in Sections 2, 8, 9, 10, 14, 19, 20, and 25.

If this section conflicts with an earlier core normative section, the earlier core section controls.

No admission architecture can guarantee perfect uniqueness, perfect availability, or perfect protection from coercion.

Profile v0 aims to ensure that failures remain:

* bounded;
* visible;
* challengeable;
* non-recursive;
* non-authoritative until sufficient verification develops.

## 24.2 Security objectives

The admission system SHOULD provide:

1. permissionless local identity preparation;
2. no single mandatory admission operator;
3. bounded canonical identity growth;
4. resistance to mass automated registration;
5. minimal authority for newly admitted identities;
6. delayed and verification-dependent reproduction;
7. deterministic sponsor accountability;
8. viable stranger and pseudonymous admission;
9. protection against permanent inherited gatekeeping;
10. durable replay and provenance.

## 24.3 Threat actors

Relevant threat actors include:

* individual bot operators;
* large AI-agent farms;
* nation states;
* corporations;
* criminal organizations;
* captured governments;
* compromised sponsors;
* colluding inviters;
* malicious relays;
* corrupt verification providers;
* ideological cartels;
* wealthy actors purchasing off-protocol influence;
* attackers controlling stolen keys;
* abusive governance majorities;
* insiders with infrastructure access.

The design MUST assume that some participants, relays, and providers will be malicious.

## 24.4 Mass local identity generation

An attacker may create arbitrarily many local keypairs and admission requests.

Profile v0 does not attempt to stop local key generation.

The defense is that local candidates:

* are non-canonical;
* receive no authority;
* consume no global identity state;
* require scarce sponsor capacity before canonicalization.

Transport providers remain responsible for local request-spam defenses.

## 24.5 Admission-request flooding

An attacker may flood request pools and sponsors with non-canonical requests.

Mitigations MAY include:

* relay-local rate limits;
* proof of work;
* queue limits;
* request expiration;
* duplicate suppression;
* local moderation;
* language or region sharding;
* client-selected request pools;
* private direct submission.

These measures MUST remain transport-level.

They MUST NOT become universal proof-of-humanity requirements.

## 24.6 Canonical-storage flooding

Unrestricted canonical self-registration would allow attackers to consume permanent identity state.

Profile v0 prevents this by requiring:

* an eligible sponsor;
* available invitation capacity;
* a valid event;
* one capacity debit per identity.

Failed and pending requests remain non-canonical.

## 24.7 Per-identity allowance multiplication

Small per-identity writing or challenge allowances are unsafe when identities are cheap to create.

Profile v0 therefore grants no ordinary allowance merely from identity creation.

Meaningful event-family authority requires later verification and eligibility.

Admission scarcity and verification must operate together.

## 24.8 Sponsor compromise

An attacker controlling a sponsor’s private key may spend that sponsor’s remaining invitation capacity.

Mitigations include:

* secure key custody;
* rapid key rotation;
* key revocation;
* compromise claims;
* temporary inviter suspension;
* low per-cycle capacity;
* bounded rollover;
* public admission provenance.

The protocol cannot prevent all damage before compromise becomes canonical state.

It limits the maximum available capacity.

## 24.9 Compromised applicant key

A sponsor or attacker may attempt to bind an identity to a key not controlled by the applicant.

The applicant possession proof prevents this where the applicant’s genuine private key remains secure.

Interfaces SHOULD allow the applicant to verify the final event-bound fields before signing.

## 24.10 Malicious sponsor substitution

A sponsor may alter:

* target identity;
* initial key;
* admission authorization;
* event ID.

The final applicant possession proof binds those values.

A changed event requires a new applicant proof.

## 24.11 Invitation rings

Multiple inviters may coordinate to admit identities they collectively control.

Defenses include:

* invitation-capacity limits;
* maturation delays;
* stronger verification before inviter eligibility;
* lineage visibility;
* cross-lineage evidence;
* attestation diminishing returns;
* duplicate-control claims;
* challengeable abuse outcomes.

Lineage concentration alone is not proof.

Legitimate communities may have similar graph shapes.

## 24.12 Self-reproducing Sybil lineages

An admitted false identity may attempt to mature and invite more false identities.

Profile v0 limits reproduction through:

* no immediate inviter eligibility;
* qualifying certified-cycle maturation;
* strong VH requirements;
* continuity requirements;
* evidence-diversity rules;
* low but positive invitation generation for legitimate eligible inviters;
* bounded carryover;
* lineage scrutiny.

The intended security objective is that a false identity cannot cheaply produce a sustained expanding inviter population.
## 24.13 Synthetic verification farms

A cluster of false identities may attest to one another.

Defenses include:

* no attester eligibility at initial admission;
* attester maturation;
* diminishing returns within clusters;
* cross-lineage evidence;
* reciprocal-attestation discounts;
* challengeable human-interaction claims;
* continuity over multiple cycles;
* provider-concentration limits.

No graph metric alone is sufficient.

## 24.14 AI-generated verification artifacts

AI may generate plausible claims, evidence summaries, or responses.

Human-first authorship requires an eligible human identity to adopt authored verification artifacts where the event family requires human authorship.

AI provenance SHOULD remain visible where relevant.

AI-generated text does not establish that the subject or attester is human.

## 24.15 Nation-state identity farms

A state may control:

* large populations of coerced participants;
* official documents;
* biometric systems;
* institutions;
* compute;
* relays;
* verification providers.

No single credential provider or government record may be universally sufficient.

Verification diversity SHOULD reduce dependence on one state-controlled source.

The system cannot prevent all coordinated behavior by real humans.

Its goal is to preserve individual human identity boundaries and prevent one institution from becoming the sole admission authority.

## 24.16 Central-provider capture

A proof-of-personhood provider may become widely used and then captured.

Optional credentials MUST remain only one evidence class.

The protocol MUST preserve alternate evidence paths.

Provider concentration SHOULD reduce marginal verification value where appropriate.

## 24.17 Social gatekeeping

Existing participants may refuse to sponsor outsiders.

Mitigations include:

* universal eventual inviter eligibility;
* the constitutional positive-capacity guarantee for unsuspended inviter-eligible identities;
* stranger sponsorship;
* multiple request pools;
* relay diversity;
* low sponsor liability;
* future alternate admission profiles;
* public visibility of aggregate admission availability and `admission_liveness_blocked`.

Profile v0 cannot guarantee that every applicant receives immediate sponsorship.

It prevents one permanent group from having exclusive protocol authority and prevents rulebooks from making broad inviter eligibility practically empty through indefinite zero capacity.
## 24.18 Ideological cartels

Sponsors or request pools may discriminate based on political beliefs.

A local sponsor remains free not to use its scarce capacity.

The protocol-level admission validator MUST remain viewpoint-neutral.

Applicants must be able to reach multiple sponsors and transports.

Governance MUST NOT establish ideological conformity as inviter or applicant eligibility.

## 24.19 Invitation markets

Participants may attempt to sell invitations off protocol.

Profile v0 reduces enforceability by making capacity:

* non-transferable;
* non-delegable;
* identity-bound;
* non-tokenized.

Evidence of actual capacity sale may support an abuse claim.

Transport fees or legitimate assistance payments must not automatically be classified as invitation sale.

## 24.20 Coercion

Governments, employers, families, or criminal organizations may coerce inviters.

Mitigations include:

* limited available capacity;
* key-security tools;
* pseudonymous applicants;
* multiple sponsors;
* coercion evidence;
* temporary protective suspension;
* restoration procedures.

The protocol cannot eliminate physical coercion.

It should avoid treating coerced action as indistinguishable from voluntary coordination where evidence permits distinction.

## 24.21 Relay censorship

A relay may suppress admission requests.

Because requests are portable and non-canonical, applicants may use other relays.

No relay denial becomes globally authoritative.

Standardized request formats and offline transport improve resilience.

## 24.22 Relay surveillance

A relay may collect applicant metadata.

Implementations SHOULD minimize:

* IP retention;
* device fingerprinting;
* location collection;
* long-term contact metadata;
* request graph storage.

Applicants SHOULD be able to use privacy-preserving or indirect transports.

Profile v0 does not guarantee traffic-analysis resistance.

## 24.23 Sponsor targeting

Public sponsor provenance may expose inviters to pressure.

Profile v0 preserves sponsor identity because it is the canonical author and capacity spender.

The protocol MUST avoid exposing additional unnecessary metadata.

Future profiles may define threshold or privacy-preserving sponsorship.

## 24.24 Applicant deanonymization

A pseudonymous identity may be correlated through:

* network traffic;
* writing style;
* verification interactions;
* sponsor relationships;
* reused credentials;
* device compromise.

Interfaces MUST not promise absolute anonymity.

High-risk users should receive accurate warnings and privacy-preserving options.

## 24.25 Key loss

An identity may lose all active keys.

Profile v0 direct admission does not itself solve later key recovery.

The key-lifecycle specification must prevent unsafe last-key revocation and define the boundary for future recovery.

Identity history remains canonical even when no usable key remains.

## 24.26 Identity duplication

One human may control multiple canonical identities.

The system must not assume this never occurs.

Duplicate-control claims and verification rules may:

* reduce simultaneous eligibility;
* identify a dominant continuing identity;
* suspend duplicate participation;
* support continuity-preserving consolidation.

The exact duplicate-identity remedy belongs to the Verification or Identity Lifecycle Specification.

## 24.27 Shared devices and networks

Multiple legitimate humans may share:

* devices;
* IP addresses;
* households;
* networks;
* institutions.

Shared infrastructure MUST NOT alone establish duplicate identity control.

It may be evidence only when combined with an authorized claim and additional context.

## 24.28 Offline batch behavior

Offline users may publish events in batches after reconnection.

Batch timing may resemble automation.

Canonical systems MUST distinguish:

* delayed legitimate publication;
* prohibited rate-limit bypass;
* coordinated automated behavior.

Offline publication remains subject to canonical pacing and verification rules.

## 24.29 High-volume community onboarding

A legitimate new community may enter through concentrated sponsors and attestations.

Security systems MUST avoid automatically treating rapid regional or linguistic onboarding as abuse.

Risk signals SHOULD lead to:

* additional scrutiny;
* evidence diversification;
* challenge;
* slower maturation

rather than immediate collective exclusion.

## 24.30 False accusations

Attackers may submit false claims that a sponsor or identity is part of a Sybil ring.

Consequences MUST depend on evidence and challenge outcomes, not allegation volume.

Challenge-spam controls are necessary to prevent harassment from becoming de facto suspension.

## 24.31 Opaque anomaly detection

Statistical or AI systems may identify suspicious graphs.

They MUST NOT directly create canonical guilt or suspension.

Canonical consequences require:

* public deterministic predicates; or
* human-authored challengeable claims with admissible evidence.

Opaque model outputs may guide investigation only.

## 24.32 Governance capture

A captured governance system may attempt to:

* reserve inviter eligibility for allies;
* impose mandatory identity providers;
* exclude pseudonymous users;
* grant unlimited capacity;
* weaken human verification;
* punish political opponents through admission rules.

Constitutional invariants must prohibit:

* permanent privileged admission classes;
* private operator grants;
* verification-weighted voting;
* viewpoint-based eligibility;
* unrestricted capacity creation;
* mandatory single-provider identity.

## 24.33 Rulebook abuse

A formally valid rulebook may still create harmful parameter choices.

Examples include:

* zero maturation;
* zero capacity indefinitely for otherwise eligible and unsuspended humans;
* unlimited carryover;
* excessive capacity generation;
* impossible diversity requirements;
* permanent suspension without appeal;
* civil-ID-only verification.

Constitutional constraints MUST bound rulebook discretion.

A rulebook that makes inviter eligibility nominal by assigning zero capacity indefinitely is invalid under Profile v0.
## 24.34 Denial of service against sponsors

Attackers may overwhelm sponsors with requests or malicious final proofs.

Sponsors and interfaces MAY:

* limit request intake;
* use local queues;
* use relay filtering;
* batch validation;
* block local senders;
* require request freshness.

These local defenses do not create canonical admission criteria.

## 24.35 Canonical processing denial of service

Malformed admission candidates may consume node resources.

Nodes SHOULD perform inexpensive checks before expensive cryptographic or database work where deterministic outcome is preserved.

Possible order:

* size limits;
* canonical encoding;
* event-type validation;
* key-length validation;
* signature verification;
* stateful authorization.

Nodes MUST still report the stable canonical error precedence adopted by the protocol.

## 24.36 Capacity exhaustion attack

An attacker may trick or coerce a sponsor into spending all capacity on low-quality applicants.

Capacity is intentionally scarce and sponsor-controlled.

Interfaces SHOULD clearly display:

* remaining capacity;
* target identity;
* key reference;
* irreversible canonical consequences.

The protocol cannot reverse a valid admission simply because the sponsor later regrets the choice.

## 24.37 Stale-candidate attacks

An attacker may replay an old signed candidate after:

* sponsor suspension;
* key rotation;
* rulebook change;
* capacity exhaustion.

Canonical validation at application position prevents stale authority from succeeding.

Event-bound applicant proofs and authorization references further restrict reuse.

## 24.38 Historical replay attacks

An attacker may reuse old possession proofs or signatures in new events.

Domain separation and binding to:

* event ID;
* target identity;
* sponsor;
* key reference;
* authorization reference

prevent cross-event reuse.

## 24.39 Database isolation

Tests and operational tooling MUST NOT use protected development or production databases as disposable admission-test targets.

DB-backed tests MUST:

* require explicit disposable targets;
* reject protected database names;
* fail closed;
* create isolated state;
* verify cleanup;
* prove no protected-state changes.

This is an implementation-security requirement supporting trustworthy conformance.

## 24.40 Private-state contamination

Canonical admission code MUST NOT depend on private account, AI, or document tables.

Open-core boundary tests SHOULD prove that the canonical identity-admission substrate operates independently of private-product state.

## 24.41 Identity-history deletion

Attackers or operators may attempt to delete embarrassing identity or lineage history.

Canonical admission provenance, direct lineage, key history, and finalized events MUST remain replayable.

Later claims may dispute meaning or establish compromise.

They must not silently erase history.

## 24.42 Long-term cryptographic risk

Ed25519 may eventually become unsuitable.

Future signature profiles must provide:

* profile versioning;
* migration;
* continuity proof;
* historical verification;
* non-retroactive key transition;
* replay compatibility.

A future algorithm transition must not rewrite Profile-v0 history.

## 24.43 Protocol ossification

Overly rigid constitutional rules may prevent adoption of better privacy or personhood technologies.

Profile v0 therefore fixes structural boundaries while reserving future profiles for:

* threshold sponsorship;
* anonymous credentials;
* distributed ceremonies;
* bounded open admission;
* stronger cryptography.

Future mechanisms must satisfy equivalent anti-capture and replay requirements.

## 24.44 Residual risks

Profile v0 does not fully solve:

* perfect proof of unique humanity;
* physical coercion;
* all off-protocol invitation markets;
* traffic analysis;
* collusion among many real humans;
* captured governance;
* malicious optional credential providers;
* all false positives;
* universal timely admission.

These risks must be documented rather than hidden.

## 24.45 Security balance

The design intentionally accepts that some invalid or duplicate identities may enter canonical state.

Security depends on ensuring that such identities:

* enter slowly;
* begin with little authority;
* cannot immediately reproduce;
* face progressive verification;
* remain challengeable;
* produce visible provenance;
* cannot cheaply multiply per-identity authority.

## 24.46 Civilizational security objective

The admission layer should preserve this balance:

```text
no central actor owns identity creation
no keypair automatically receives canonical authority
real humans can seek entry through decentralized paths
false identities cannot cheaply flood or reproduce
historical provenance remains inspectable
verification remains claim-based and challengeable
```

Profile v0 is a bounded starting architecture, not a claim of perfect personhood determination.

# 25. Constitutional rules versus rulebook parameters

## 25.1 Purpose

This section separates protocol-fixed admission guarantees from parameters that may change through canonical rulebooks.

The distinction exists to prevent two opposite failures:

1. making the protocol so rigid that it cannot adapt to new threats, evidence methods, or civilizational conditions; and
2. giving ordinary governance enough discretion to remove anti-capture, human-first, privacy, or anti-Sybil protections.

Rulebooks MAY tune operation within the boundaries of this specification.

Rulebooks MUST NOT redefine the constitutional meaning of identity admission.

## 25.2 Authority hierarchy

Admission behavior MUST follow the applicable authority hierarchy.

At minimum:

1. Protocol v5 Section 0 and constitutional invariants;
2. this specification’s protocol-fixed admission rules;
3. the canonical event catalog and encoding rules;
4. verification, cycle, replay, privacy, safety, and key-lifecycle specifications;
5. active canonical rulebooks;
6. implementation behavior.

An implementation, rulebook, governance vote, private service, or operator action MUST NOT override a higher-authority rule.

Where a rulebook conflicts with a constitutional requirement, the conflicting rulebook provision is invalid for canonical application.

## 25.3 Protocol-fixed admission model

Under Profile v0, the following are protocol-fixed:

1. local identity preparation is permissionless;
2. local identity candidates are non-canonical;
3. normal post-genesis identity creation requires canonical authorization;
4. Profile-v0 canonical identity creation is sponsor-authored;
5. the sponsor must be an eligible human identity;
6. the target identity kind is fixed by Profile v0 as `identity_kind = human`;
7. canonical admission consumes scarce invitation capacity;
8. the applicant must prove control of the initial key;
9. canonical admission creates or derives the complete required `identity_structural_roots`;
10. sponsorship is not verification;
11. identity creation does not grant ordinary canonical participation;
12. a newly admitted identity receives only the restricted verification and identity-control lane;
13. invitation authority must be generally attainable by sufficiently qualified humans;
14. invitation capacity is identity-bound, non-transferable, and non-saleable;
15. canonical admission is deterministic and replayable;
16. private accounts and operator state do not determine admission validity;
17. AI and system identities cannot sponsor normal human admission;
18. unrestricted canonical self-registration is inactive under Profile v0.

A rulebook MUST NOT change any of these rules without a protocol-profile amendment.

## 25.4 Permissionless local identity preparation

The right to generate local keys and identity candidates is protocol-fixed.

A rulebook MUST NOT require permission, payment, credential ownership, institutional approval, or existing social membership before a person may locally create:

* a keypair;
* an identity candidate;
* a key descriptor;
* a possession proof;
* an admission request.

A rulebook MAY regulate canonical admission and event submission.

It MUST NOT claim authority over private key generation itself.

## 25.5 Non-canonical request boundary

The non-canonical status of admission requests is protocol-fixed for Profile v0.

A rulebook MUST NOT cause:

* pending requests;
* rejected requests;
* abandoned requests;
* request-pool entries;
* relay observations

to become canonical identities automatically.

A future protocol profile MAY introduce a canonical queue or bounded open-admission mechanism.

An ordinary rulebook cannot activate such a path without the required profile specification.

## 25.6 Human sponsor requirement

The requirement that a normal Profile-v0 sponsor be a human identity is protocol-fixed.

A rulebook MUST NOT authorize sponsorship by:

* AI identities;
* system identities;
* organizations acting without a human author;
* operators acting only through infrastructure privilege;
* private product accounts;
* anonymous unsigned services.

A future threshold or organizational admission profile MAY distribute authorization among multiple humans.

It must still preserve human-first authorship and must be separately standardized.

## 25.7 No permanent inviter class

The prohibition on a permanent privileged inviter class is protocol-fixed.

A rulebook MUST NOT reserve invitation eligibility indefinitely for:

* founders;
* genesis identities;
* node operators;
* delegates;
* experts;
* institutions;
* governments;
* token holders;
* wealthy participants;
* a named allowlist;
* descendants of particular admission lineages.

A rulebook MAY impose general eligibility thresholds.

Those thresholds must be objectively applicable to all human identities.

## 25.8 Equal-rule principle

The same inviter-eligibility rule MUST apply to all similarly situated human identities.

A rulebook MAY distinguish identities based on replay-visible states such as:

* verification certainty;
* continuity;
* maturation;
* unresolved challenges;
* active suspension;
* admission-abuse outcomes;
* active key control.

It MUST NOT distinguish based on:

* identity popularity;
* political viewpoint;
* religion;
* nationality;
* ethnicity;
* wealth;
* criticism of the system;
* social prestige;
* operator preference.

## 25.9 Separation of admission and verification

The distinction among the following is protocol-fixed:

```text
identity existence
!= identity_kind
!= identity_structural_root_state
!= key control
!= raw verification artifacts
!= VH
!= VI
!= ordinary writer eligibility
!= challenge eligibility
!= voter eligibility
!= governance eligibility
!= Tempo eligibility
!= inviter eligibility
```

A rulebook MAY define transitions and thresholds.

It MUST NOT collapse these states into one undifferentiated score or status.

## 25.10 Claim-based verification

The requirement that canonical human verification derive from explicit, attributable, challengeable ordinary truth claims, evidence, contradictions, challenges, and outcomes is protocol-fixed.

A rulebook MAY define:

* authorized predicates;
* evidence classes;
* certainty functions;
* challenge thresholds;
* maturation effects.

It MUST NOT replace claim-based verification with:

* a private administrator flag;
* an opaque AI score;
* unchallengeable government certification;
* social popularity;
* Anthill degree;
* sponsor declaration alone.

It also MUST NOT replace claim-based verification with an ordinary post-genesis `identity_verification_update` status-setting event.

## 25.11 Identity structural-root and Anthill boundary

The required `identity_structural_roots` role as structural topology and provenance, rather than automatic verification authority, is protocol-fixed.

The Anthill remains one specialized root within that topology.

A rulebook MAY use authorized Anthill-derived metrics to measure:

* evidence concentration;
* lineage concentration;
* graph diversity;
* reciprocal attestation patterns.

A rulebook MUST NOT state that:

* being connected to a verified identity makes another identity verified;
* having many Anthill edges creates authority;
* social centrality grants higher truth or vote weight;
* admission lineage establishes guilt.

## 25.12 Initial authority boundary

The minimal authority of a newly admitted identity is protocol-fixed.

A rulebook MUST NOT grant ordinary canonical writing, general challenges, voting, governance, Tempo, invitation capacity, or economic authority solely because `identity_create` succeeded.

A rulebook MAY define the restricted identity-verification events available immediately after creation.

Those events must remain subject-bound and narrowly scoped.

## 25.13 Invitation-capacity properties

The following properties are protocol-fixed under Profile v0:

* capacity is identity-bound;
* capacity is non-transferable;
* capacity is non-saleable;
* capacity is non-delegable except through a later profile;
* successful admission consumes capacity;
* identical retries do not consume capacity twice;
* capacity cannot become negative;
* capacity generation depends on qualifying human-certified canonical cycles;
* every unsuspended inviter-eligible identity receives at least one spendable unit in each qualifying capacity period;
* existing valid capacity remains spendable during stalls unless separately suspended, expired, frozen, or constitutionally blocked;
* unlimited carryover is prohibited;
* machines alone cannot generate recurring admission authority.

A rulebook MAY set numerical parameters within these constitutional bounds.
## 25.14 Rulebook-controlled capacity parameters

The active rulebook MAY define:

* base capacity generated per qualifying eligible cycle;
* eligibility-specific generation rates above the constitutional minimum;
* carryover cap;
* expiration behavior;
* temporary freeze behavior;
* restoration behavior;
* transitional reductions after abuse;
* capacity-generation rounding;
* maximum spend per cycle;
* minimum interval between admissions where applicable.

Every parameter MUST be:

* canonical;
* versioned;
* deterministic;
* publicly inspectable;
* bounded by the constitutional rules.

A rulebook MUST satisfy `generated_capacity >= 1` for each unsuspended inviter-eligible identity in each qualifying capacity period.
## 25.15 Rulebook-controlled inviter thresholds

The active rulebook MAY define:

* minimum VH certainty;
* VI requirements for scoped circumstances;
* continuity threshold;
* minimum certified-cycle maturation;
* evidence-diversity threshold;
* unresolved-challenge effects;
* good-standing predicates;
* temporary probation;
* suspension and restoration conditions.

A rulebook MUST NOT set inviter maturation to zero under normal post-genesis operation.

A rulebook MUST NOT make a single mandatory provider the only viable source of qualifying evidence.

## 25.16 Rulebook-controlled verification formulas

The Verification Specification and active rulebooks MAY define:

* certainty aggregation;
* support and contradiction effects;
* attestation independence;
* same-lineage diminishing returns;
* challenge survival effects;
* evidence aging;
* provider concentration effects;
* duplicate-control consequences.

These formulas MUST be deterministic and explainable.

They MUST NOT assign greater truth weight, vote weight, importance weight, or Tempo influence merely because an identity has higher verification.

They MUST operate on ordinary canonical verification artifacts and privacy-preserving commitments or outcomes, not private node-local assertions.

## 25.17 Rulebook-controlled restricted-lane limits

The active rulebook MAY define rate and payload limits for:

* self-verification claims;
* continuity claims;
* challenge responses;
* attestation acknowledgments;
* verification-evidence submissions;
* identity-scoped disputes;
* Anthill structural maintenance.

These limits MUST permit meaningful verification participation.

They MUST NOT create a general-purpose unverified writing channel.

## 25.18 Rulebook-controlled abuse consequences

A rulebook MAY define prospective consequences for established coordinated admission abuse, including:

* reduced generation;
* reduced carryover;
* increased maturation;
* temporary invitation suspension;
* additional evidence requirements;
* attester-eligibility restrictions;
* permanent inviter disqualification in severe repeated cases.

Consequences MUST:

* be based on canonical claims, evidence, contradiction relationships, challenges, predicates, or outcomes;
* be proportionate;
* identify the affected eligibility lane;
* provide challenge or review where applicable;
* avoid retroactive destruction of valid identities and events.

## 25.19 Prohibited rulebook changes

A rulebook MUST NOT:

* enable unrestricted Profile-v0 self-registration;
* create unlimited invitation capacity;
* assign zero capacity indefinitely to otherwise eligible and unsuspended inviters;
* eliminate sponsor humanity requirements;
* make capacity transferable or saleable;
* grant ordinary authority automatically at identity creation;
* reserve admission authority for a permanent class;
* use political viewpoint as an eligibility condition;
* require one universal government or corporate identity provider;
* allow opaque AI output to directly establish verification or suspension;
* allow wall-clock passage alone to mint capacity;
* make forced, degraded, survivor, record-only, Dmax-only, or machine-only boundaries equivalent to certified human-deliberative cycles;
* retroactively erase valid admissions;
* increase governance vote weight based on verification.

Any such rule is outside valid Profile-v0 rulebook discretion.
## 25.20 Parameter safety bounds

The governing specifications SHOULD establish explicit safe ranges where unconstrained parameters could defeat constitutional protections.

Examples include:

* minimum maturation greater than zero;
* minimum generated capacity of one unit for each unsuspended inviter-eligible identity in each qualifying period;
* finite maximum capacity generation;
* finite carryover cap;
* finite event payload sizes;
* non-negative capacity;
* bounded suspension durations unless severe permanent disqualification is adjudicated;
* maximum number of admissions per identity per cycle.

A rulebook value outside a protocol-defined safe range MUST be rejected.
## 25.21 Rulebook activation

A new admission-related rulebook MUST activate at a deterministic canonical boundary.

Its activation record MUST identify:

* rulebook ID;
* prior rulebook;
* effective boundary;
* affected eligibility lanes;
* capacity transition behavior;
* pending-candidate behavior;
* carryover behavior;
* maturation behavior.

A rulebook MUST NOT become active merely because a software deployment changed configuration.

## 25.22 Rulebook transition compatibility

A transition MUST define treatment of:

* existing invitation balances;
* pending admission candidates;
* active suspensions;
* maturation progress;
* verification certainty;
* open challenges;
* provisional restoration.

Historical admissions and debits MUST remain evaluated under the rules applicable at their canonical positions.

## 25.23 Emergency safety restrictions

A Safety Rulebook MAY impose temporary restrictions when there is strong evidence of large-scale admission abuse.

Emergency restrictions MAY include:

* temporary global admission-rate reduction;
* temporary freezing of selected capacity;
* increased confirmation or challenge requirements;
* suspension of a compromised admission profile.

Emergency action MUST NOT:

* create permanent centralized admission authority;
* eliminate all future admission indefinitely;
* secretly target political opponents;
* bypass public canonical recording;
* retroactively erase identities;
* grant operators private discretionary power;
* mint replacement capacity through operators, AI, system emitters, wall-clock timers, or machine-only boundaries.

Profile v0 defines no emergency capacity-minting path.

Emergency actions require defined expiration, review, and challenge.
## 25.24 Governance limitation

Governance MAY amend rulebooks within constitutional bounds.

Governance MUST NOT use a normal rulebook vote to remove the structural admission protections in this specification.

A change to protocol-fixed rules requires the amendment procedure applicable to authoritative protocol specifications.

## 25.25 Constitutional amendment boundary

A future protocol amendment MAY change the admission model.

Such an amendment must explicitly address:

* anti-gatekeeping effects;
* Sybil resistance;
* human-first authorship;
* privacy;
* replay;
* storage;
* initial authority;
* migration;
* interaction with existing identity provenance.

A rulebook cannot silently function as a protocol amendment.

## 25.26 Public parameter explainability

A node MUST be able to expose the active admission-related rulebook parameters required to reproduce:

* inviter eligibility;
* capacity generation;
* carryover;
* suspension;
* restoration;
* restricted-lane limits.

Protected evidence need not be disclosed.

The formula, thresholds, rulebook references, and resulting public states must remain auditable.

## 25.27 Civilizational stability principle

The fixed-versus-adjustable boundary should preserve:

```text
stable rights and anti-capture protections
+
adaptable thresholds and operational safeguards
```

The system must be capable of adapting to changing attacks without allowing temporary majorities or operators to redefine who counts as a participant through ordinary configuration.

---

# 26. Future admission profiles

## 26.1 Purpose

Profile v0 defines sponsored public admission as the active canonical admission mechanism.

The protocol intentionally preserves room for future independent admission profiles.

Future profiles may improve:

* anti-gatekeeping access;
* privacy;
* censorship resistance;
* geographic inclusion;
* resilience against sponsor capture;
* verification diversity;
* scalability.

Recognition of future profiles does not activate them.

No future admission path may be inferred from this specification without a separately adopted authoritative profile.

## 26.2 Profile requirements

Every future canonical admission profile MUST define:

1. who authors or authorizes identity creation;
2. how authorization is proven;
3. what scarce resource or constraint limits creation;
4. how applicant key possession is proven;
5. how canonical storage growth is bounded;
6. how Sybil flooding is resisted;
7. what authority a new identity initially receives;
8. how verification begins;
9. how replay validates the event;
10. how conflicts resolve;
11. how privacy is protected;
12. how abuse is challenged;
13. how legacy and Profile-v0 identities interoperate;
14. how the profile is activated and deactivated.

A profile missing any of these elements MUST NOT become canonical authority.

## 26.3 Common invariants for future profiles

Unless a constitutional amendment explicitly changes them, future profiles MUST preserve:

* permissionless local key generation;
* human-first canonical authority;
* deterministic replay;
* applicant key-possession proof;
* separation of identity existence from verification;
* minimal initial authority;
* no private account authority;
* no mandatory single provider;
* no verification-weighted truth or votes;
* durable identity provenance;
* challengeable abuse handling.

## 26.4 Profile identifiers

Each admission profile MUST have a unique canonical identifier.

The identifier MUST be bound into:

* admission events;
* authorization proofs;
* applicant possession proofs where applicable;
* replay state;
* public reads;
* conformance fixtures.

An event authorized under one profile MUST NOT be accepted as an event under another profile.

## 26.5 Profile activation

A future profile becomes active only through an authorized canonical protocol transition.

Activation MUST identify:

* profile ID;
* effective canonical position;
* applicable rulebook;
* eligible actor classes;
* interaction with existing profiles;
* migration requirements;
* deactivation or emergency-suspension rules.

Software support alone does not activate a profile.

## 26.6 Parallel profiles

Multiple admission profiles MAY operate simultaneously where the protocol explicitly permits them.

Parallel operation MUST define:

* whether profiles share capacity;
* whether they use separate global quotas;
* how identity collisions resolve;
* whether profile provenance affects later verification;
* how aggregate canonical growth is bounded;
* whether one profile may be temporarily suspended independently.

No profile may be treated as epistemically superior solely because of its admission method.

## 26.7 Profile-neutral identity status

Once canonically admitted, an identity’s admission profile remains part of provenance.

The profile MUST NOT automatically create:

* greater truth weight;
* greater vote weight;
* greater importance weight;
* permanent social class;
* permanent verification advantage.

Verification may consider evidence genuinely produced by an admission mechanism.

Admission-profile prestige alone is not evidence.

## 26.8 Bounded open admission

A future bounded open-admission profile MAY allow an applicant to authorize its own canonical creation without an existing sponsor.

Such a profile MUST define a deterministic scarcity mechanism.

Possible mechanisms include:

* global capacity quota;
* epoch admission slots;
* threshold-selected open slots;
* a queue with bounded admission;
* an authorized lottery;
* privacy-preserving uniqueness credentials;
* another specified scarce resource.

"Anyone may self-register" without a canonical bound is insufficient.

Profile v0 does not contain this fallback and defines no emergency self-registration or machine-minted admission capacity during admission-liveness failure.
## 26.9 Open-admission author

A bounded self-registration profile must solve the bootstrap signature problem.

It MUST define:

* whether the applicant is the canonical author;
* how the not-yet-active initial key signs the creation request;
* how proof of possession differs from canonical authorship;
* how the profile’s admission authorization is verified;
* how the event avoids circular key validation.

Profile-v0 sponsor rules MUST NOT be reused ambiguously.

## 26.10 Global quota design

A global-quota profile MUST define:

* quota generation;
* quota period;
* quota authority;
* ordering;
* conflict resolution;
* unused quota;
* anti-flooding behavior;
* partition handling;
* forced-boundary behavior;
* admission-liveness behavior.

A quota MUST NOT be created solely by wall-clock time or autonomous machine cycles unless constitutionally authorized by that future profile.
## 26.11 Lottery design

A future lottery profile MUST specify:

* eligible request set;
* request commitment deadline;
* randomness source;
* resistance to applicant flooding;
* resistance to random-source manipulation;
* canonical selection proof;
* privacy;
* retry behavior;
* unclaimed slot behavior.

A lottery without applicant scarcity may simply reward the actor capable of submitting the most requests.

## 26.12 Queue design

A canonical or semi-canonical queue profile MUST specify:

* queue entry authority;
* queue ordering;
* anti-spam constraint;
* request expiration;
* priority rules;
* censorship resistance;
* privacy;
* canonical storage burden;
* ability to submit through multiple relays.

Payment MUST NOT create mandatory priority unless a constitutional amendment explicitly authorizes an economic admission model.

## 26.13 Distributed ceremony admission

A future ceremony profile MAY use coordinated human-presence procedures.

It MUST define:

* ceremony organizers;
* participant eligibility;
* quorum;
* attendance evidence;
* duplicate attendance prevention;
* coercion and accessibility concerns;
* offline support;
* geographic inclusion;
* privacy;
* challengeability;
* ceremony capture resistance.

One organization MUST NOT become the universal ceremony provider.

## 26.14 Threshold sponsorship

A threshold-sponsorship profile MAY require multiple human sponsors instead of one.

It MUST define:

* sponsor threshold;
* sponsor independence;
* capacity consumption from each sponsor;
* applicant proof binding;
* partial-signature expiration;
* conflict resolution;
* sponsor withdrawal;
* privacy;
* lineage representation.

Threshold sponsorship may improve resistance to one compromised inviter but may increase gatekeeping.

## 26.15 Anonymous sponsorship credentials

A future profile MAY use privacy-preserving credentials to prove that an eligible sponsor or sponsor quorum authorized admission without publicly identifying each sponsor.

Such a profile MUST preserve:

* deterministic authorization;
* capacity consumption;
* double-spend prevention;
* abuse challengeability;
* auditability;
* resistance to credential transfer;
* sponsor accountability compatible with the privacy model.

Profile v0 public sponsor provenance MUST not be described as anonymous.

## 26.16 Anonymous proof-of-personhood credentials

A future profile MAY accept decentralized anonymous credentials as admission or verification inputs.

The credential system MUST define:

* issuer set;
* issuer diversity;
* revocation;
* uniqueness scope;
* credential transfer resistance;
* coercion;
* privacy;
* provider capture;
* challengeability;
* fallback when providers fail.

No one issuer may become universally mandatory.

## 26.17 Institutional admission

Institutions MAY contribute optional admission or verification evidence in a future profile.

Institutional authority MUST be bounded.

An institution MUST NOT gain unilateral power to create unlimited canonical human identities.

Institutional admission MAY require:

* threshold approval;
* capacity limits;
* cross-provider diversity;
* challengeability;
* public provenance;
* periodic re-evaluation.

## 26.18 Community admission profiles

A future profile MAY allow recognized decentralized communities to allocate bounded admission capacity.

It MUST define:

* how communities become eligible;
* how capacity is allocated;
* how internal decisions are human-authored;
* how community capture is resisted;
* how applicants outside communities retain access;
* how lineage and accountability are recorded.

Community admission MUST NOT replace individual admission paths entirely.

## 26.19 Recovery-based identity return

A future recovery profile may restore authority to a previously canonical identity rather than create a new identity.

Recovery MUST be distinguished from admission.

It must preserve:

* historical identity ID;
* key history;
* continuity;
* prior admissions;
* verification state where still valid.

Recovery MUST NOT consume ordinary invitation capacity unless the adopted profile explicitly requires it.

## 26.20 Legacy-identity registration profile

A future transition profile MAY permit a legacy identity to register a Profile-v0 key and enter event-derived lifecycle state.

It MUST define:

* proof of legacy control;
* identity continuity;
* key possession;
* duplicate-control handling;
* verification transition;
* provenance classification.

It MUST NOT fabricate a historical sponsor or `identity_create`.

## 26.21 Proof-of-work boundary

A future profile MAY use proof of work to bound transport or queue participation.

Proof of work MUST NOT be treated as proof of humanity.

A profile relying materially on computation must analyze:

* wealth inequality;
* nation-state advantage;
* energy use;
* accessibility;
* specialized hardware;
* bot-farm economics.

Proof of work alone is not an adequate civilizational identity-admission mechanism.

## 26.22 Verifiable delay boundary

A verifiable delay function MAY help pace requests or allocate time-bound slots.

It does not establish human uniqueness.

A VDF-based profile must still solve:

* request multiplication;
* parallel hardware;
* Sybil submission;
* global ordering;
* accessibility;
* canonical storage.

## 26.23 Proof-of-reasoning prohibition

No future admission profile SHOULD require intellectual or rhetorical performance as proof of humanity.

Any proposal using reasoning quality must undergo constitutional review for:

* AI advantage;
* disability discrimination;
* language bias;
* educational bias;
* ideological screening;
* confusion of human existence with epistemic performance.

Profile v0 prohibits proof-of-reasoning admission.

## 26.24 Economic admission risks

A future profile proposing fees, bonds, deposits, or staking MUST demonstrate that it does not:

* exclude poor participants;
* privilege wealthy attackers;
* create tradable identity rights;
* make personhood purchasable;
* undermine pseudonymous high-risk access;
* create economic vote weighting.

Economic friction may reduce spam but is not proof of humanity.

## 26.25 Profile interaction with verification

Every future profile MUST state whether its admission method produces any verification evidence.

Admission MUST still not automatically establish more than the evidence justifies.

For example:

* a human-presence ceremony may create VH evidence;
* a legal credential may create scoped VI evidence;
* a sponsor event alone creates no VH;
* a lottery win creates no verification evidence.

Profile provenance and verification artifacts must remain distinct.

## 26.26 Profile interaction with Anthill

Every future profile MUST define:

* creation or derivation of the complete identity structural-root set;
* the Anthill root representation where the profile includes Anthill semantics;
* admission provenance display;
* whether sponsor or ceremony relationships appear;
* which verification artifacts may be attached;
* privacy implications.

Anthill topology must remain distinct from verification certainty.

## 26.27 Initial authority

A future profile MUST specify the initial event-family permissions of newly admitted identities.

The default constitutional expectation is minimal authority.

A profile granting broader initial authority must justify:

* Sybil risk;
* rate limits;
* storage;
* challenge capacity;
* flood resistance;
* verification assumptions.

## 26.28 Profile abuse handling

Every future profile MUST define abuse predicates relevant to its mechanism.

Examples include:

* credential duplication;
* ceremony collusion;
* lottery manipulation;
* quota capture;
* threshold-sponsor collusion;
* issuer corruption;
* queue flooding.

Consequences must remain challengeable and replay-derived.

## 26.29 Profile suspension

A profile MAY be temporarily suspended when a severe systemic vulnerability is established.

Suspension MUST be:

* canonical;
* scoped;
* publicly justified;
* time-bounded or reviewable;
* non-retroactive toward already valid identities unless constitutional invalidity is proven.

Other active admission profiles may continue where independently safe.

## 26.30 Migration between profiles

An identity’s admission provenance remains historical.

An identity does not need to be recreated merely because another profile becomes preferred.

Where a profile produces stronger verification evidence, existing identities MAY acquire equivalent evidence through later verification procedures.

Admission profile migration MUST NOT rewrite original provenance.

## 26.31 Profile deprecation

A deprecated profile may stop accepting new admissions.

Existing identities created under it remain canonical.

Deprecation MUST define:

* final admissible event position;
* pending-event handling;
* verification implications;
* capacity expiration;
* public-read behavior;
* replay support.

Historical profile validation must remain available.

## 26.32 Constitutional future-path principle

The mature system SHOULD support multiple decentralized routes to participation.

However, path diversity must not come at the cost of underspecified consensus behavior.

The principle is:

```text
preserve extension points now
standardize each path before activation
never infer authority from an unfinished design
```

---

# 27. Conformance requirements

## 27.1 Purpose

Conformance testing proves that independent implementations interpret Profile-v0 identity admission identically.

The conformance suite MUST cover:

* canonical encoding;
* signatures;
* possession proofs;
* sponsor eligibility;
* invitation capacity;
* identity creation;
* identity structural-root creation or derivation, including Anthill anchoring;
* restricted verification access;
* replay;
* conflict handling;
* privacy boundaries;
* legacy compatibility;
* cross-document invariants.

A build passing compilation without these behavioral proofs is not sufficient.

## 27.2 Conformance artifact classes

The Profile-v0 suite SHOULD include:

1. canonical encoding vectors;
2. cryptographic signature vectors;
3. applicant possession-proof vectors;
4. authored-event fixtures;
5. replay-sequence fixtures;
6. capacity-cycle fixtures;
7. conflict-order fixtures;
8. migration fixtures;
9. API/DTO fixtures;
10. privacy and boundary fixtures;
11. generated-export tests.

Fixtures MUST be versioned.

## 27.3 Test-key safety

Conformance vectors MUST use test-only keys.

The suite MUST NOT include:

* production private keys;
* real user recovery material;
* real private evidence;
* secret operational credentials.

Private test keys MAY be published as fixtures when clearly marked unusable for production.

## 27.4 Canonical descriptor vectors

The suite MUST include vectors proving deterministic encoding and hashing of:

```text
key_profile_version
signature_algorithm
raw_public_key_bytes
owning_identity_id
```

Vectors MUST cover:

* valid Ed25519 descriptor;
* wrong raw-key length;
* unsupported algorithm;
* unsupported profile;
* wrong owner;
* forbidden additional field;
* non-canonical encoding;
* expected `public_key_ref`.

## 27.5 Applicant possession-proof vectors

Required vectors include:

1. valid final possession proof;
2. wrong event ID;
3. wrong target identity;
4. wrong sponsor;
5. wrong public-key reference;
6. wrong admission-authorization reference;
7. wrong verification reference;
8. verification reference added after applicant proof;
9. verification reference removed after applicant proof;
10. verification reference replaced after applicant proof;
11. canonical no-reference encoding accepted;
12. non-canonical alternate no-reference encoding rejected;
13. malformed signature;
14. wrong signing key;
15. wrong domain separator;
16. altered descriptor;
17. request-stage proof used as final proof;
18. signature with correct semantics but non-canonical bytes;
19. proof bytes excluding `initial_key_possession_proof` itself;
20. proof bytes excluding sponsor signature;
21. proof bytes excluding any recursive payload hash.

Each vector MUST identify:

* signed bytes;
* public key;
* signature;
* exact verification-reference or no-reference value;
* expected result;
* stable rejection code where invalid.

## 27.6 Sponsor-authored candidate vectors

The suite MUST prove:

* sponsor signature binds the complete final payload after applicant proof insertion;
* sponsor signature binds the applicant proof;
* sponsor signature binds the reduced admission-authorization reference;
* sponsor signature binds the exact verification reference or canonical no-reference state;
* target cannot alter its own key after sponsor signing;
* sponsor cannot substitute the key after applicant proof;
* payload field changes invalidate the sponsor signature;
* author identity must equal sponsor identity;
* self-sponsorship is rejected;
* a separate speaker identity is rejected where forbidden.

## 27.7 Valid identity-creation fixture

A complete valid fixture MUST demonstrate:

1. canonical human sponsor exists;
2. sponsor active key is valid;
3. inviter eligibility is active;
4. sponsor has available capacity;
5. target identity does not exist;
6. initial key descriptor is valid;
7. key reference matches;
8. applicant proof is valid;
9. reduced admission authorization is valid;
10. exact verification reference or canonical no-reference state is valid;
11. event applies atomically.

Expected effects MUST include:

* identity creation;
* `identity_kind = human`;
* event-derived provenance;
* initial key activation;
* sponsor lineage;
* `identity_structural_roots`, including Anthill anchor;
* capacity debit;
* `CanonicalAdmittedIdentity` status with ordinary participation lanes inactive;
* restricted verification-lane activation;
* no ordinary writer or inviter eligibility.
## 27.8 Stranger-sponsorship fixture

The suite MUST include a valid admission where:

* sponsor and applicant have no prior social relationship;
* no prior Anthill edge exists;
* no civil-identity reference is supplied;
* the applicant remains pseudonymous;
* all cryptographic and capacity requirements pass.

The event MUST be accepted.

This proves that prior acquaintance and civil identity are not hidden requirements.

## 27.9 Sponsor ineligibility fixtures

Required rejection fixtures include:

* sponsor identity unknown;
* sponsor not human;
* AI sponsor;
* system sponsor;
* sponsor key unknown;
* sponsor key wrong owner;
* sponsor key superseded;
* sponsor key revoked;
* inviter threshold not met;
* maturation not met;
* continuity insufficient;
* active invitation suspension;
* eligibility not yet activated at boundary.

Each fixture MUST produce no canonical side effects.

## 27.10 Capacity fixtures

The suite MUST cover:

* positive balance admission;
* zero balance rejection;
* balance generation at a qualifying human-certified cycle;
* `generated_capacity >= 1` for each unsuspended inviter-eligible identity in a qualifying capacity period;
* invalid rulebook that assigns zero capacity indefinitely;
* no generation during an ineligible or non-qualifying cycle;
* existing valid capacity remains spendable during a stall unless separately suspended, expired, frozen, or constitutionally blocked;
* bounded carryover;
* expiration of excess carryover;
* frozen capacity during suspension;
* restoration behavior;
* no forced or record-only restoration;
* no negative balance;
* one-unit debit;
* no economic issuance;
* no capacity transfer.
## 27.11 Idempotency fixture

The identical accepted candidate MUST be submitted at least twice.

The second submission MUST:

* resolve to the existing event;
* return idempotent success;
* create no second identity;
* create no second key;
* create no second lineage edge;
* create no second identity structural-root set or Anthill anchor;
* perform no second debit.

## 27.12 Conflicting duplicate-event fixture

The suite MUST submit:

* one accepted event;
* a newly signed candidate using the same `event_id` but different bytes.

The second candidate MUST be rejected as a conflicting duplicate.

Payload similarity alone is insufficient; the fixture must prove the signed bytes differ.

## 27.13 Duplicate-identity fixtures

Required cases include:

* same target identity, same sponsor, different event ID;
* same target identity, different sponsor;
* same target identity, different initial key;
* same target identity, stronger verification reference.

Only the first valid canonical creation succeeds.

Later creations consume no capacity.

## 27.14 Duplicate-key fixtures

Required cases include:

* same key proposed for two target identities;
* key already active elsewhere;
* key historically superseded;
* key historically revoked;
* descriptor with same raw key but different owner.

Expected behavior must follow the adopted key-reuse policy.

## 27.15 Final-unit race fixture

A sponsor with one remaining capacity unit submits two otherwise valid admissions.

The fixture MUST define canonical order.

Expected result:

* first event succeeds;
* second event fails for insufficient capacity;
* second event does not incorrectly return `stale_admission_authorization`;
* final balance is zero;
* only one identity is created.

The same fixture replayed with reversed arrival order but identical canonical order MUST produce the same result.

## 27.16 Sponsor-key rotation race

The suite MUST test:

1. admission ordered before sponsor rotation;
2. admission ordered after sponsor rotation;
3. admission signed by superseded key;
4. re-signed admission using new active key.

Historical canonical order must determine validity.

A candidate signed by a superseded or revoked key MUST fail with the applicable key-state error, not `stale_admission_authorization`.

## 27.17 Suspension race

The suite MUST test:

* admission before suspension effective position;
* admission after suspension effective position;
* candidate signed before suspension but applied after;
* restoration at a later boundary.

No node-local signing time may override canonical state.

## 27.18 Rulebook-transition fixtures

Fixtures MUST cover:

* old authorization before new rulebook activation;
* stale old authorization after activation;
* valid new authorization;
* carryover treatment;
* pending candidate behavior;
* stale capacity period requiring a new authorization reference and applicant proof;
* insufficient remaining capacity not incorrectly returning stale authorization;
* sponsor key revocation not incorrectly returning stale authorization;
* maturation transition;
* transition preserves existing capacity during stalls unless the prior applicable rule explicitly expires, suspends, freezes, or constitutionally blocks it;
* transition rejects a rulebook that makes inviter eligibility nominal through indefinite zero capacity.

The rulebook reference and activation boundary must be explicit.
## 27.19 Cycle fixtures

Required cycle cases include:

* certified human-deliberative cycle generating capacity;
* qualifying period generates at least one unit for each unsuspended inviter-eligible identity;
* zero-participation stalled cycle exposes `admission_liveness_blocked = true`;
* record-only cycle generates no capacity and advances no maturation;
* forced boundary generates no capacity and restores no suspension;
* degraded boundary generates no capacity;
* survivor boundary generates no capacity;
* Dmax fallback alone generates no capacity;
* Dmin-unsatisfied closure;
* normal maturation;
* existing valid capacity remains spendable during a stall where not separately blocked;
* no same-cycle inviter activation;
* no same-cycle recursive capacity;
* no machine-only emergency minting.

Expected authority effects must match the Cycle Specification and Section 19.
## 27.20 Initial-authority fixtures

After valid identity creation, tests MUST prove the target:

May:

* read public state;
* use allowed key-control events;
* submit a permitted self-authored ordinary verification truth claim;
* submit permitted subject-bound ordinary verification evidence;
* respond to an identity-scoped verification challenge using the ordinary challenge ontology or its constrained identity-verification profile;
* access its `identity_structural_roots`, including the Anthill anchor.

May not:

* create ordinary idea;
* create ordinary connection;
* issue general truth challenge;
* vote;
* govern;
* submit Tempo claim;
* invite;
* receive invitation capacity;
* exercise economic authority.

## 27.21 Verification-claim fixtures

The suite MUST prove:

* sponsorship creates no VH;
* sponsorship creates no VI;
* Anthill edge creates no VH;
* mutual social connection creates no VH;
* explicit attestation creates an ordinary challengeable verification truth claim but not automatic certainty;
* self-claim remains a claim and does not self-verify;
* raw artifacts remain distinct from derived VH and VI certainty;
* challenge outcome affects derived state only under the active rulebook;
* higher verification does not increase truth, vote, importance, governance, Tempo, or economic weight;
* `identity_verification_update` is rejected as ordinary post-genesis status-setting authority.

## 27.22 Restricted-lane scope fixtures

Required cases include:

* valid self-correspondence claim;
* valid continuity claim;
* valid ordinary verification evidence relationship;
* valid contradiction relationship concerning a verification claim;
* valid identity-scoped challenge response;
* invalid verification claim about an unrelated identity;
* invalid ordinary idea disguised as verification;
* invalid opaque verification-only record disconnected from ordinary truth/evidence/challenge semantics;
* invalid general challenge;
* rate-limit enforcement;
* payload-size enforcement;
* unauthorized event type.

Subject constraints MUST be tested explicitly.

## 27.23 Anthill fixtures

The suite MUST cover:

* one deterministic anchor per identity;
* anchor derived or created atomically;
* exact retry creates no duplicate;
* conflicting second anchor rejected;
* social edge distinct from attestation;
* lineage edge distinct from social relationship;
* verification artifacts discoverable through the anchor;
* Anthill degree has no direct eligibility effect.

## 27.24 Lineage fixtures

Required cases include:

* direct sponsor edge creation;
* genesis identity with no fabricated sponsor;
* legacy identity with unknown lineage;
* ancestry derivation;
* no automatic VH from lineage;
* no automatic guilt from sponsor suspension;
* prospective lineage-related restriction;
* historical admission remains valid.

## 27.25 Inviter-accountability fixtures

The suite MUST distinguish:

* ordinary invitee disagreement;
* isolated invitee misconduct;
* failed verification;
* sponsor key compromise;
* repeated duplicate-control sponsorship;
* established invitation sale;
* coordinated invitation ring;
* false accusation.

Only authorized abuse outcomes may alter future invitation authority.

## 27.26 Privacy fixtures

Tests MUST prove canonical admission does not require:

* legal name;
* government ID;
* IP address;
* location;
* private conversation;
* relay path;
* private account;
* email;
* biometric record.

Where a private verification reference is used, public reads must expose only authorized commitments or outcomes.

Private evidence must never pass conformance solely through a node-local assertion.

## 27.27 Error-code fixtures

Every stable rejection code adopted under Section 23 MUST have at least one fixture.

Multi-failure fixtures MUST prove stable rejection precedence.

Errors MUST avoid secret leakage.

Retryability classification SHOULD also be tested.

## 27.28 Transactional atomicity fixtures

For each failure point after stateful validation begins, tests MUST prove no partial state remains.

At minimum, simulate failure during:

* identity insertion;
* key insertion;
* lineage insertion;
* Anthill-anchor creation;
* capacity debit;
* restricted-lane initialization;
* event append.

The transaction must roll back completely.

## 27.29 Replay rebuild fixture

The suite MUST:

1. apply a sequence incrementally;
2. capture materialized state;
3. rebuild from genesis and canonical events;
4. compare results.

Equality MUST include:

* identities;
* key history;
* provenance;
* identity structural roots;
* Anthill anchors;
* lineage;
* raw verification artifacts;
* derived VH or VI certainty;
* eligibility;
* capacity generation;
* debits;
* remaining balances;
* suspension;
* restricted-lane state.

## 27.30 Arrival-order independence fixture

The same event set MUST be delivered to two implementations in different arrival orders.

After applying the same canonical order, both must produce identical state.

## 27.31 Snapshot fixture

A snapshot created after a sequence of admission events MUST restore enough state to validate later:

* capacity spends;
* key transitions;
* suspensions;
* new admissions;
* eligibility changes.

Snapshot-plus-tail replay must equal full replay.

## 27.32 Genesis fixtures

Required cases include:

* valid genesis identity;
* valid genesis key descriptor;
* explicit genesis provenance;
* initial finite capacity;
* bootstrap inviter exception;
* bootstrap exception expiry;
* no fabricated sponsor;
* no permanent genesis advantage.

## 27.33 Legacy migration fixtures

A pre-Profile-v0 database or snapshot must migrate while preserving:

* identities;
* historical events;
* key data;
* public reads;
* legacy provenance.

Tests MUST prove the migration does not fabricate:

* sponsors;
* possession proofs;
* capacity debits;
* lineage;
* verification artifacts.

## 27.34 Open-core boundary fixtures

The conformance suite MUST prove canonical identity admission does not depend on:

* account/session crates;
* private AI tables;
* private document tables;
* product-only authentication;
* private email state;
* private organizer ideas.

The default open-core build must retain canonical admission behavior.

## 27.35 API and DTO fixtures

Public DTO tests MUST verify safe representation of:

* identity ID;
* admission provenance;
* creation position;
* initial public-key reference;
* current key state;
* `identity_structural_roots`;
* Anthill anchor;
* writer eligibility;
* inviter eligibility;
* capacity summary where public;
* suspension;
* legacy classification.

Private fields must remain absent.

## 27.36 Generated-export fixtures

Generated open-core export validation MUST prove:

* specification-relevant code is included;
* private dependencies are excluded;
* migrations are included;
* boundary tests pass;
* generated source builds;
* conformance fixtures execute against the export where supported.

## 27.37 Database-isolation fixtures

DB-backed tests MUST use approved disposable databases.

The test guard MUST reject:

* protected development databases;
* production-like names;
* maintenance databases as write targets;
* absent database names;
* ordinary application URLs.

The protected database must remain unchanged before and after test execution.

## 27.38 No-skip acceptance requirement

A test that prints `SKIP` because infrastructure is unavailable does not count as executed acceptance proof.

Release-readiness reports MUST distinguish:

* passed;
* failed;
* skipped;
* not run.

Database-backed acceptance requires actual execution against disposable state.

## 27.39 Cross-implementation vectors

Where multiple implementations exist, they MUST execute the same normative vectors and produce identical:

* hashes;
* references;
* signatures;
* state transitions;
* rejection codes;
* capacity balances;
* public DTO semantics.

## 27.40 Conformance versioning

Every fixture set MUST identify:

* protocol version;
* admission profile;
* encoding profile;
* signature profile;
* rulebook version where relevant;
* expected result.

A later fixture revision MUST NOT silently change the expected result of an existing finalized profile.

## 27.41 Minimum release gate

Profile-v0 admission implementation MUST NOT be declared complete until the following are proven:

* valid sponsor-authored identity creation;
* applicant proof of possession;
* sponsor and capacity authorization;
* atomicity;
* idempotency;
* conflict handling;
* replay equality;
* cycle integration;
* minimal initial authority;
* restricted verification lane;
* identity structural-root and Anthill anchoring;
* legacy preservation;
* public DTO safety;
* open-core independence;
* protected database isolation.

## 27.42 Conformance report

Each implementation release SHOULD produce a conformance report containing:

* commands executed;
* fixture counts;
* pass/fail/skip totals;
* disposable database names without credentials;
* protected database before/after checks;
* generated-export result;
* known exclusions;
* protocol/profile versions;
* remaining blockers.

A report MUST NOT claim a proof that was skipped or inferred rather than executed.

# 28. Required canonical and public read surfaces

## 28.1 Purpose

Public read surfaces make identity admission, key provenance, invitation authority, and derived eligibility independently inspectable without exposing private identity evidence or product-layer data.

A conforming implementation MUST expose enough canonical and replay-derived information for an observer to determine:

* whether an identity exists canonically;
* how the identity entered canonical state;
* which key currently controls the identity;
* which sponsor authorized an event-derived admission;
* whether the identity is eligible to write or invite;
* how invitation capacity was generated and consumed;
* whether an admission-related suspension applies;
* which rulebooks and canonical boundaries produced those states.

Public reads are explanatory views over canonical or replay-derived state.

They MUST NOT become independent sources of authority.

## 28.2 General read requirements

Canonical identity-admission reads MUST be:

* publicly accessible under the open-core read model;
* deterministic;
* replay-consistent;
* profile-versioned;
* independent of private product accounts;
* privacy-bounded;
* stable enough for independent client implementations.

A public read response MUST distinguish among:

* canonical stored fields;
* replay-derived fields;
* optional interface summaries;
* unavailable or private information.

An implementation MUST NOT present a private database field as canonical authority merely because it appears in a public response.

## 28.3 Identity summary

A public identity summary SHOULD expose at least:

```text
identity_id
identity_kind
admission_provenance_class
creation_position
admission_profile
current_key_control_status
identity_structural_roots
ordinary_writer_eligibility
inviter_eligibility
invitation_suspension_status
anthill_anchor
```

Where a value is derived, the response SHOULD indicate that it is derived.

Where a legacy value is unknown, the response MUST use an explicit unknown or legacy classification rather than fabricating modern provenance.

## 28.4 Identity detail

A public identity-detail read SHOULD expose:

* canonical identity ID;
* identity kind;
* admission provenance;
* creation event reference where event-derived;
* creation canonical position;
* admission profile;
* sponsor identity where Profile-v0 event-derived;
* admission-lineage parent;
* initial public-key reference;
* current active public-key reference;
* key-history summary;
* identity structural-root summary;
* Anthill anchor;
* current verification-state summary;
* ordinary writer eligibility;
* ordinary challenge eligibility;
* restricted verification-lane status;
* voter eligibility;
* governance eligibility;
* Tempo eligibility;
* inviter eligibility;
* invitation-capacity summary where public;
* identity dormancy or recovery status;
* active admission-related suspensions;
* applicable rulebook references;
* legacy compatibility classification.

The detail read MUST NOT imply that sponsor provenance is a verification attestation.

## 28.5 Admission provenance

Admission provenance MUST be represented through an explicit classification.

At minimum:

```text
genesis_admitted
legacy_operator_provisioned
event_derived
future_profile_derived
```

For an `event_derived` Profile-v0 identity, the public read MUST expose:

* `identity_create` event ID;
* sponsor identity ID;
* canonical creation position;
* admission profile ID;
* capacity-period reference;
* applicable rulebook reference;
* direct admission-lineage edge.

For a genesis identity, the public read MUST expose the genesis commitment or profile reference instead of a fabricated sponsor.

For a legacy identity, missing fields MUST remain absent or explicitly unknown.

## 28.6 Sponsor read surface

A public sponsor-related read MAY expose:

* sponsor identity ID;
* identity created;
* creation event ID;
* canonical position;
* admission profile;
* capacity period;
* applicable rulebook;
* resulting lineage edge.

It MUST NOT expose:

* applicant contact details;
* request-pool membership;
* private messages;
* relay route;
* legal identity;
* private evidence;
* physical meeting details;
* network metadata.

## 28.7 Admission-lineage reads

A public direct-lineage read MUST permit clients to determine:

```text
sponsor_identity_id
admitted_identity_id
identity_create_event_id
canonical_position
admission_profile
```

Derived ancestry reads MAY expose:

* parent;
* children;
* ancestors;
* descendants;
* lineage depth;
* branching history;
* cycle-based admission counts.

Derived graph views MUST identify:

* derivation algorithm;
* scope;
* depth;
* rulebook version where canonically relevant.

A derived ancestry view MUST NOT label a lineage as abusive unless a separate canonical claim, outcome, or deterministic rule supports that conclusion.

## 28.8 Key-control reads

Public key-control reads MUST expose enough information to validate canonical signatures.

For each registered public key, reads SHOULD expose:

```text
public_key_ref
owning_identity_id
key_profile_version
signature_algorithm
raw_public_key_bytes
registration_provenance
activation_position
supersession_position
revocation_position
current_status
```

Private-key material MUST never be exposed.

The key-history read MUST distinguish:

* active;
* superseded;
* revoked;
* invalid;
* unknown;
* legacy unsupported.

## 28.9 Initial possession-proof visibility

The canonical initial-key possession proof MAY be exposed as:

* raw canonical signature bytes;
* a canonical proof reference;
* a verified-proof status;
* a public conformance representation.

The public model MUST allow an independent implementation to validate the proof where the underlying canonical event is public.

No private key or non-canonical applicant request needs to be exposed.

## 28.10 Identity structural-root and Anthill-anchor reads

Every identity-detail response MUST expose the authoritative `identity_structural_roots` set or its deterministic derivation.

It MUST also expose the Anthill anchor or its deterministic derivation where the active structural profile includes Anthill semantics.

A public Anthill read MAY expose:

* public social relationships;
* admission lineage;
* verification artifacts;
* attestations;
* contradictions;
* challenges;
* continuity claims;
* derived verification explanations.

It MUST distinguish among those object types.

An Anthill read MUST NOT present:

* social connection;
* sponsor relationship;
* attestation;
* verification result

as interchangeable concepts.

## 28.11 Verification summary

Public reads SHOULD expose a privacy-safe verification summary.

The summary MAY include:

* current VH band or eligibility result;
* current VI band or scoped correspondence status;
* raw verification-artifact categories and counts;
* effective canonical boundary;
* governing verification rulebook;
* unresolved public challenges;
* public verification-artifact counts by type;
* protected-evidence commitment references;
* current ordinary writer eligibility;
* current inviter eligibility.

The summary MUST NOT expose protected evidence merely to explain a derived result.

It MUST distinguish raw artifacts from derived VH or VI certainty and from event-family eligibility outputs.

## 28.12 Verification-artifact reads

Public verification artifacts MUST expose, where not privacy-restricted:

* artifact ID;
* artifact type;
* ordinary object class or relationship type;
* subject identity;
* author or attester identity;
* verification predicate;
* canonical position;
* supporting references;
* contradicting references;
* challenge references;
* current effectiveness;
* applicable schema.

A verification artifact MUST remain visible historically even if it later becomes ineffective.

A read surface MUST NOT present a verification artifact as an administrative certificate or direct eligibility update.

## 28.13 Restricted verification-lane status

A public identity read SHOULD expose whether the identity may currently use the restricted verification lane.

The read MAY identify:

* permitted event-family groups;
* current rate-limit period;
* remaining event allowance where publicly safe;
* active lane-specific suspension;
* governing rulebook.

The response MUST NOT imply that restricted-lane access is ordinary writer eligibility.

## 28.14 Writer-eligibility read

The public writer-eligibility view SHOULD expose:

* whether ordinary writing is currently active;
* event families covered;
* effective boundary;
* rulebook reference;
* public predicate summary;
* active suspension state where applicable.

A single boolean MAY be provided for convenience.

The canonical model SHOULD retain event-family-specific detail.

The read MUST NOT imply that raw verification artifacts or VH/VI certainty are themselves writer eligibility.

## 28.15 Inviter-eligibility read

The public inviter-eligibility view MUST expose enough information to determine whether the identity may currently sponsor admission.

It SHOULD include:

```text
inviter_eligible
effective_boundary
maturation_status
qualifying_capacity_period_status
active_key_requirement_satisfied
verification_requirement_satisfied
continuity_requirement_satisfied
active_suspension
admission_liveness_blocked
governing_rulebook
```

Where a predicate depends on protected evidence, the read MAY expose only the derived result and canonical commitment.

The read MUST NOT imply that raw verification artifacts or VH/VI certainty are themselves inviter eligibility.
## 28.16 Invitation-capacity summary

Under Profile v0, exact `invitation_capacity_balance` is publicly derivable from public canonical history, rulebooks, certified cycle boundaries, and successful capacity debits.

A public capacity summary SHOULD expose:

* capacity period;
* whether the current period qualifies for capacity generation;
* whether `admission_liveness_blocked` is active;
* whether maturation advanced;
* generated capacity;
* bounded carryover;
* expired capacity;
* successful debits;
* whether existing capacity remains spendable;
* current spendable balance;
* frozen or suspended balance;
* governing rulebook;
* last effective boundary;
* boundary type.

A public interface MAY omit, bucket, or simplify the displayed balance for coercion or targeting concerns.

That is presentation minimization only. DTO omission, delayed display, bucketing, or UI hiding is not cryptographic privacy: the exact remaining capacity remains publicly derivable by a conforming node.

A genuinely private capacity model requires a future cryptographic admission profile.
## 28.17 Capacity-history read

A capacity-history read SHOULD expose entries for:

* generation;
* carryover;
* expiration;
* debit;
* freeze;
* restoration;
* rulebook adjustment.

Each entry SHOULD identify:

```text
identity_id
capacity_period
effect_type
amount
canonical_position
source_event_or_boundary
rulebook_reference
```

Capacity history is an authorization ledger, not an economic account.

## 28.18 Suspension read

A public invitation-suspension read SHOULD expose:

* affected identity;
* affected eligibility lane;
* effective canonical position;
* suspension source;
* governing rulebook;
* public reason code;
* challenge status;
* expiration or review condition;
* restoration status.

Private evidence may remain protected.

A private operator note MUST NOT appear as canonical suspension authority.

## 28.19 Rulebook reads

Nodes MUST expose the rulebook references necessary to reproduce:

* inviter eligibility;
* invitation-capacity generation;
* rollover;
* expiration;
* suspension;
* restoration;
* restricted-lane limits.

The active rulebook content or canonical commitment MUST be publicly retrievable under the applicable rulebook specification.

## 28.20 Cycle-related admission reads

Public cycle reads SHOULD include admission-relevant outputs such as:

* capacity-period ID;
* cycle boundary type;
* certification status;
* whether the period qualifies for capacity generation;
* whether `admission_liveness_blocked` is active;
* whether the cycle generated capacity;
* whether maturation advanced;
* whether new inviter eligibility activated;
* whether restoration activated;
* whether existing capacity remains spendable;
* total capacity generated;
* total successful admissions;
* forced, degraded, survivor, Dmax-only, or record-only status;
* governing rulebook.

This view MUST distinguish machine-observed passage from human-authorized cycle authority.
## 28.21 Event read

The canonical event-log read for `identity_create` MUST expose the public authored candidate and publication wrapper fields required by the authorship profile.

At minimum:

* event ID;
* event type;
* author identity;
* public-key reference;
* payload hash;
* payload or payload reference;
* signature;
* canonical position;
* admission profile.

The exact initial-key descriptor and possession proof must remain independently verifiable.

## 28.22 Rejection reads

Rejected non-canonical candidates need not become permanent canonical records.

Where an ingress API returns a rejection, it SHOULD expose:

* stable error code;
* event ID where valid;
* retryability;
* governing profile;
* public-safe explanation.

A rejection response MUST NOT expose private evidence or credentials.

## 28.23 Legacy identity reads

A legacy identity read MUST clearly state:

* provenance class;
* whether a Profile-v0 key is active;
* whether new Profile-v0 writes are permitted;
* whether ordinary writer eligibility is transitional;
* whether inviter eligibility exists;
* whether admission lineage is unknown;
* whether `identity_structural_roots` were deterministically migrated;
* whether an Anthill anchor was deterministically migrated.

The interface MUST NOT imply full Profile-v0 admission compliance when it is absent.

## 28.24 Genesis identity reads

A genesis identity read MUST expose:

* genesis provenance;
* genesis commitment;
* initial key state;
* initial eligibility admissions;
* bootstrap exception status;
* transition to ordinary eligibility where applicable.

No fictional sponsor should appear.

## 28.25 Private fields excluded from public DTOs

Public identity-admission DTOs MUST exclude:

* private keys;
* recovery secrets;
* account passwords;
* sessions;
* cookies;
* email addresses;
* private contact information;
* applicant IP addresses;
* relay routing metadata;
* private request contents;
* undisclosed legal identity;
* raw protected evidence;
* private product account IDs;
* private AI classifications.

## 28.26 Private product boundary

A product layer MAY maintain:

* private onboarding state;
* private request drafts;
* contact preferences;
* encrypted evidence;
* local notifications;
* device key backups;
* private social organization.

Those fields MUST NOT alter canonical reads unless transformed into an authorized canonical artifact.

The open-core read model must remain usable without the private product layer.

## 28.27 DTO versioning

Public identity and admission DTOs MUST be versioned.

A DTO revision MUST distinguish:

* additive optional fields;
* changed semantics;
* deprecated fields;
* profile-specific fields.

A field named `verified`, `writer`, or `inviter` MUST have a precise defined meaning and effective boundary.

## 28.28 Explanation endpoints

A node SHOULD provide explanation surfaces for:

* why an identity exists;
* why a key is active;
* why an identity is writer-eligible;
* why an identity is inviter-eligible;
* how invitation capacity was calculated;
* why a suspension applies;
* why an admission was rejected.

Explanations MUST derive from canonical state and active rulebooks.

They MUST NOT reveal protected evidence beyond authorized summaries.

## 28.29 Read consistency

All public reads taken at the same finalized canonical position MUST be mutually consistent.

For example:

* identity detail;
* key state;
* capacity balance;
* lineage;
* eligibility;
* event log

must not report incompatible state.

Implementations SHOULD support a canonical-position parameter or snapshot reference for consistent historical reads.

## 28.30 Historical reads

Nodes SHOULD support historical admission-state reads at a specified canonical position.

Historical reads MAY expose:

* key active at that position;
* sponsor eligibility at that position;
* capacity before and after an admission;
* writer or inviter eligibility at that position;
* applicable rulebook.

Historical reads are essential for validating non-retroactive signatures and admissions.

---

# 29. Implementation sequencing

## 29.1 Purpose

Identity admission depends on key lifecycle, verification, cycle state, invitation eligibility, and capacity derivation.

Implementation MUST proceed in an order that avoids:

* temporary private admission authority;
* circular verification dependencies;
* manually assigned permanent inviters;
* canonical identity creation without replay-derived capacity;
* incomplete key security.

The sequence in this section is non-normative implementation planning guidance except where it cross-references an independently normative release or safety requirement defined elsewhere.

## 29.2 Phase 0 — Specification reconciliation

Before runtime implementation, authoritative documents MUST be reconciled to define:

* Profile-v0 sponsor-authored admission;
* exact `identity_create` payload;
* initial-key possession-proof bytes;
* key-rotation and key-revocation semantics;
* restricted verification-lane event families;
* invitation eligibility;
* invitation-capacity cycle behavior;
* Anthill-anchor semantics;
* replay and conflict rules;
* stable rejection codes.

Conformance fixtures SHOULD be designed before runtime code is accepted.

## 29.3 Phase 1 — Existing-identity key lifecycle

The first runtime phase SHOULD implement secure key lifecycle for already-existing canonical identities.

Scope:

* `identity_key_rotate`;
* `identity_key_revoke`;
* active, superseded, and revoked key states;
* historical signature validity;
* replay-derived key history;
* public key-history reads;
* additive legacy compatibility.

This phase SHOULD exclude `identity_create`.

## 29.4 Key-lifecycle prerequisites

Before key lifecycle is implemented, authority MUST settle:

* whether rotation replaces the active key;
* whether Profile v0 permits one active direct key;
* new-key possession proof;
* authorizing-key requirement;
* self-revocation prohibition;
* last-active-key restriction;
* recovery boundary;
* key-management eligibility independent of ordinary writing.

## 29.5 Phase 2 — Verification claim substrate

The next phase SHOULD implement the canonical substrate for identity verification.

Scope MAY include:

* ordinary verification truth claims;
* ordinary verification evidence;
* verification attestations as ordinary truth claims;
* ordinary contradiction relationships;
* identity-scoped ordinary challenges or constrained challenge profiles;
* challenge responses;
* VH and VI predicate schemas;
* identity structural-root and Anthill anchoring;
* public verification-artifact reads;
* deterministic certainty inputs.

This phase need not complete every future verification method.

It must provide the minimum substrate required to derive writer and inviter eligibility.

It MUST NOT implement a parallel verification-only object model.

## 29.6 Restricted verification lane

The verification-substrate phase MUST implement the restricted lane for `CanonicalAdmittedIdentity` identities.

The implementation MUST enforce:

* event-family allowlist;
* subject identity constraint;
* ordinary truth/evidence/challenge object semantics;
* payload limits;
* rate limits;
* no ordinary idea creation;
* no general challenge authority;
* no voting or governance authority.

This prevents circularity between admission and verification.

## 29.7 Phase 3 — Writer eligibility

Writer eligibility SHOULD be implemented before public identity admission is opened.

The phase SHOULD derive:

* event-family-specific writer eligibility;
* boundary activation;
* suspension and restoration;
* public explanations;
* legacy transition behavior.

The current transitional writer gate may coexist temporarily but must not remain the final protocol authority.

## 29.8 Phase 4 — Inviter eligibility

Inviter eligibility SHOULD then be implemented as a separate derived lane.

Scope:

* VH requirement;
* continuity requirement;
* maturation;
* evidence diversity;
* challenge effects;
* active-key requirement;
* suspension;
* restoration;
* public explanation.

Inviter eligibility MUST NOT be inferred from ordinary writer eligibility alone.

## 29.9 Phase 5 — Invitation-capacity derivation

After inviter eligibility exists, implement:

* cycle-based capacity generation;
* carryover;
* expiration;
* spending balance;
* suspension effects;
* restoration effects;
* public capacity history;
* deterministic replay;
* canonical ordering.

No identity admission should depend on a manually edited balance.

## 29.10 Phase 6 — Genesis and transition state

Before opening normal admission, implement the authoritative starting state for:

* genesis identities;
* initial keys;
* initial verification admissions or compatibility records;
* initial inviter eligibility;
* finite initial invitation capacity;
* bootstrap exception expiry;
* legacy classifications.

The transition must be replay-visible and independent of private operator rows.

## 29.11 Phase 7 — Canonical `identity_create`

Only after the minimum inviter and capacity substrate is operational should the runtime implement:

* sponsor-authored `identity_create`;
* applicant key possession proof;
* admission-authorization reference;
* atomic identity creation;
* initial key activation;
* sponsor provenance;
* admission lineage;
* `identity_structural_roots`, including Anthill anchor;
* capacity debit;
* `CanonicalAdmittedIdentity` status with ordinary participation lanes inactive;
* restricted-lane activation.

## 29.12 Phase 8 — Admission ingress and public reads

Add support to the centralized canonical ingress route.

The implementation SHOULD use:

```text
POST /api/v1/canonical/events
```

or the current canonical successor route.

Do not create:

* unsigned identity-creation routes;
* private operator admission routes;
* account-registration shortcuts;
* product-only canonical admission handlers.

Public read surfaces from Section 28 must be implemented and tested.

## 29.13 Phase 9 — Non-canonical admission transport

After canonical admission is secure, implement non-canonical tools such as:

* admission-request schema;
* direct request exchange;
* basic request pools;
* relays;
* offline transfer;
* sponsor review interfaces.

These tools MUST remain outside canonical authority.

They MAY be implemented in private or federated products without changing consensus.

## 29.14 Phase 10 — Inviter accountability

Implement inviter-abuse claim schemas and consequences only after:

* verification claims;
* challenges;
* evidence handling;
* lineage;
* rulebook effects;
* suspension;
* restoration

are sufficiently specified.

Do not deploy opaque automated suspension as a temporary shortcut.

## 29.15 Phase 11 — Advanced Sybil analysis

Later phases MAY add:

* lineage-concentration analysis;
* attestation-cluster diminishing returns;
* duplicate-control detection;
* AI-assisted anomaly discovery;
* community-onboarding safeguards.

Canonical effects must remain deterministic or challenge-mediated.

## 29.16 Future admission profiles

Do not implement:

* global lotteries;
* distributed ceremonies;
* anonymous sponsor credentials;
* bounded self-registration;
* proof-of-personhood admission

inside the Profile-v0 task.

Each requires its own specification, conformance fixtures, and activation process.

## 29.17 Migration order

Where multiple repositories or deployments depend on open-core authority, the recommended order is:

```text
open-core specifications
→ open-core schemas and migrations
→ open-core replay and conformance
→ generated export
→ private product integration
→ admission-request interfaces
```

Private implementation MUST NOT become de facto authority ahead of open-core specifications.

## 29.18 Database migration strategy

Admission-related migrations SHOULD be additive.

They SHOULD preserve:

* identities;
* events;
* keys;
* accounts where private;
* writer grants;
* public reads;
* historical provenance.

Migrations MUST NOT fabricate Profile-v0 admission history.

## 29.19 Test-isolation requirement

Every DB-backed implementation phase MUST use approved disposable databases.

Tests MUST fail closed against:

* protected development databases;
* production databases;
* maintenance databases used as write targets;
* ordinary application URLs.

Protected-state before-and-after checks SHOULD be included in acceptance reports.

## 29.20 Export sequencing

After each substantive open-core phase:

1. build the default open-core workspace;
2. run package and integration tests;
3. run boundary checks;
4. run DTO drift checks;
5. run conformance;
6. generate the open-core export;
7. verify the generated export;
8. run the export smoke build;
9. confirm private-state independence.

## 29.21 Release gates

Normal public identity admission MUST NOT be enabled until all of the following are proven:

* key lifecycle is secure;
* inviter eligibility is replay-derived;
* invitation capacity is replay-derived;
* `identity_create` is atomic;
* applicant possession proof is verified;
* restricted verification lane exists;
* writer eligibility remains inactive after creation;
* replay rebuild equality passes;
* protected database isolation passes;
* generated export passes;
* public DTOs do not leak private state.

## 29.22 Temporary bootstrap restraint

A deployment MAY require bounded bootstrap identities and capacity before normal admission begins.

It MUST NOT rely on an indefinite temporary operator-managed inviter table.

Every temporary bootstrap path must have:

* a versioned source;
* public bounds;
* replay visibility;
* transition condition;
* removal or expiry plan.

## 29.23 No shortcut through private accounts

Product account registration MUST NOT create canonical identity state before Phase 7.

A product MAY prepare:

* local candidate;
* keypair;
* request;
* private onboarding state.

Canonical identity begins only through authorized admission.

## 29.24 Implementation-status reporting

The implementation-status document MUST distinguish among:

* specified;
* planned;
* partially implemented;
* source-present but disabled;
* bootstrap-only;
* tested with skips;
* live-tested;
* export-proven;
* production-ready.

No phase should be marked complete based solely on source presence.

## 29.25 Recommended successor task structure

A practical task split is:

```text
TEMPO-005D-K1
Existing-identity key lifecycle

TEMPO-005E
Verification claims, writer eligibility, inviter eligibility,
and invitation-capacity derivation

TEMPO-005D-I1
Sponsor-authored identity_create and canonical admission

TEMPO-005D-T1
Non-canonical request transport and sponsor interfaces
```

Task identifiers may vary, but the dependency order SHOULD remain.

---

# 30. Non-goals

## 30.1 Purpose

This section prevents Profile v0 from being interpreted as specifying adjacent systems that remain incomplete or intentionally deferred.

A non-goal is not necessarily permanently prohibited.

It is outside the active Profile-v0 admission mechanism unless separately stated.

## 30.2 No unrestricted canonical self-registration

Profile v0 does not allow anyone to create a canonical identity merely by generating a key and submitting a self-authored event.

Open canonical self-registration requires a later profile.

## 30.3 No baseline ordinary authority for admission

Profile v0 does not grant newly admitted identities:

* ordinary ideas;
* ordinary connections;
* general challenges;
* voting;
* governance;
* Tempo;
* invitation capacity;
* economic authority.

## 30.4 No proof of perfect human uniqueness

Profile v0 does not guarantee that every canonical identity corresponds to exactly one distinct human.

It provides mechanisms to:

* limit entry;
* verify human presence;
* detect duplicate control;
* limit simultaneous authority;
* challenge abuse.

## 30.5 No mandatory civil identity

Profile v0 does not require:

* legal name;
* government ID;
* citizenship;
* institutional registration;
* public VI;
* biometric enrollment.

## 30.6 No universal biometric system

Profile v0 does not establish a universal face, fingerprint, iris, voice, DNA, or other biometric database.

Biometric evidence, where ever allowed, requires separate privacy and verification authority.

## 30.7 No mandatory proof-of-personhood provider

No one proof-of-personhood provider is required or universally sufficient.

Optional providers may become evidence sources under later rules.

## 30.8 No proof-of-work humanity

Proof of work, VDFs, deposits, or computation are not proof that a requester is human.

They may be transport-level anti-spam tools only.

## 30.9 No proof-of-reasoning admission

Argument quality, writing skill, ideological conformity, education, or rhetorical performance do not determine canonical identity existence.

## 30.10 No verification-weighted truth

Higher VH, VI, or verification level does not make a person’s factual claims intrinsically more true.

Verification may affect eligibility and rate limits only where authorized.

## 30.11 No verification-weighted votes

Higher verification does not create heavier governance, challenge, importance, or Tempo votes.

Verification may qualify an identity for a voter pool.

It does not increase vote weight.

## 30.12 No Anthill popularity authority

Anthill degree, social centrality, number of relationships, or popularity do not directly create:

* VH;
* VI;
* writer eligibility;
* inviter eligibility;
* vote weight;
* truth weight.

## 30.13 No automatic verification through sponsorship

A sponsor-authored `identity_create` is not a verification attestation.

Sponsorship establishes admission provenance and consumes capacity.

## 30.14 No automatic verification through social edges

Mutual relationships and social acknowledgment are not verification evidence unless accompanied by explicit authorized claims.

## 30.15 No private operator admission

Profile v0 does not permit operators to create post-genesis canonical identities through:

* SQL;
* private APIs;
* allowlists;
* account records;
* configuration files;
* undocumented scripts.

## 30.16 No hidden inviter grants

Profile v0 does not permit private assignment of invitation eligibility or capacity.

## 30.17 No AI canonical authority

AI does not independently:

* admit identities;
* verify humans;
* suspend inviters;
* determine challenge outcomes;
* create binding Sybil verdicts.

## 30.18 No full verification algorithm in this document

This specification does not fully define:

* VH aggregation;
* VI aggregation;
* certainty bands;
* evidence weighting;
* challenge resolution;
* verification-level formulas.

Those belong to the Verification Specification and rulebooks.

## 30.19 No full key-recovery system

This specification does not define:

* social recovery;
* succession;
* peer recovery;
* emergency key replacement;
* post-loss identity reclamation.

Those require a separate identity-recovery profile.

## 30.20 No transferable invitation assets

Invitation capacity is not tokenized, transferable, saleable, lendable, or delegable under Profile v0.

## 30.21 No threshold sponsorship

Profile v0 uses one canonical human sponsor.

Threshold sponsorship remains a future profile.

## 30.22 No anonymous canonical sponsor

Profile v0 preserves public sponsor authorship.

Anonymous or hidden sponsor authorization requires a future privacy-preserving profile.

## 30.23 No global admission lottery

Profile v0 does not define a lottery, open queue, or globally bounded self-registration lane.

## 30.24 No distributed ceremony profile

Profile v0 does not define in-person or remote human-presence ceremonies as a canonical admission method.

## 30.25 No canonical admission-request queue

Admission requests remain non-canonical.

Request pools are transport services.

## 30.26 No canonical storage of failed requests

Profile v0 does not require rejected, expired, or unsponsored requests to enter permanent canonical state.

## 30.27 No identity deletion

Profile v0 does not delete canonical identity history.

Identities may become:

* inactive;
* dormant;
* suspended;
* keyless;
* legacy read-only.

Historical existence remains.

## 30.28 No lineage caste

Admission ancestry does not create hereditary rights, guilt, verification, or social rank.

## 30.29 No automatic punishment of descendants

An inviter’s later suspension does not automatically invalidate or punish identities it previously admitted.

## 30.30 No automatic punishment for invitee speech

An inviter is not responsible for an invitee’s opinions, lawful disagreement, ordinary error, or political unpopularity.

## 30.31 No opaque canonical graph scoring

AI or non-deterministic clustering may produce investigative leads.

They do not directly determine canonical eligibility or guilt.

## 30.32 No private-account dependency

Canonical admission, keys, verification, capacity, and lineage do not require:

* product accounts;
* sessions;
* emails;
* passwords;
* private organizer state.

## 30.33 No wallet or custody UX

This specification does not define user-facing private-key storage, backups, hardware wallets, device synchronization, or recovery interfaces.

## 30.34 No complete relay protocol

This specification permits independent relays but does not fully define:

* relay discovery;
* federation;
* encrypted response channels;
* anti-spam economics;
* request ranking;
* moderation.

## 30.35 No implementation-specific schema authority

Database tables, Rust types, TypeScript DTOs, and private product flows do not override this specification.

## 30.36 No assumption of immediate global scale

Profile v0 provides a bounded starting mechanism.

Capacity and verification parameters may begin conservatively and evolve through rulebooks.

---

# 31. Open questions and deferred parameters

## 31.1 Purpose

This section records only matters intentionally deferred to subordinate specifications, rulebooks, implementation planning, or future protocol profiles.

An open parameter does not authorize implementations to choose incompatible behavior privately.

Before a deferred matter affects canonical results, the responsible authority MUST settle it deterministically.

Resolved architecture decisions from R2A through R2E are marked closed here so they are not mistaken for runtime blockers.

## 31.2 Open-question classes

Open matters are classified as:

```text
SPECIFICATION BLOCKER
RULEBOOK PARAMETER
IMPLEMENTATION CHOICE
FUTURE PROFILE
CLOSED
```

* **SPECIFICATION BLOCKER:** must be resolved before the affected runtime feature is implemented.
* **RULEBOOK PARAMETER:** structure is settled; exact value may be selected canonically.
* **IMPLEMENTATION CHOICE:** may vary without changing canonical results.
* **FUTURE PROFILE:** not active under Profile v0.
* **CLOSED:** settled by this specification; later documents may need reconciliation but may not reopen the semantic decision without an explicit protocol change.

## 31.3 Identity structural-root identifiers and derivation mechanics

**Classification:** SPECIFICATION BLOCKER for exact Appendix A and structural-role reconciliation; root names are CLOSED.

The Profile-v0 required root names are fixed here as:

1. Mindgarden;
2. Backyard of Relationships;
3. Self Tree;
4. Anthill.

The structural specifications must still choose for each root:

* exact canonical identifier;
* byte encoding;
* deterministic derivation or explicit materialization;
* containment relation;
* replay effect;
* legacy migration behavior;
* public read shape;
* collision behavior.

## 31.4 Exact admission-profile identifier

**Classification:** SPECIFICATION BLOCKER.

The event registry and encoding specification must adopt the exact identifier for sponsored public admission v0.

Example shape:

```text
sponsored_public_admission_v0
```

The final identifier must be stable and domain-separated.

## 31.5 Exact admission-authorization encoding

**Classification:** SPECIFICATION BLOCKER for byte-level encoding; reduced context semantics are CLOSED.

The reduced authorization-context fields are settled as:

```text
admission_profile_version
sponsor_identity_id
capacity_period_id
rulebook_reference
```

`eligibility_snapshot_reference` is not part of admission authorization.

The Canonical Encoding and Hashing Specification must still define:

* domain separator;
* field ordering;
* field encodings;
* hash algorithm;
* admission-profile version encoding;
* capacity-period reference encoding;
* rulebook-reference encoding;
* normative vectors.

## 31.6 Event-position replay authority

**Classification:** CLOSED.

Replay at the event position is authoritative for current sponsor key state, inviter eligibility, invitation suspension, remaining capacity, target uniqueness, key uniqueness, and applicant proof validity.

A structurally valid `admission_authorization_reference` does not freeze those states, reserve capacity, or prove current eligibility.

## 31.7 Exact possession-proof bytes

**Classification:** SPECIFICATION BLOCKER for byte-level encoding; construction order and applicant-bound fields are CLOSED.

Section 12 settles the semantic construction order and applicant-bound fields.

The Signature Profile and Encoding Specification must adopt the exact canonical bytes, including:

* exact domain separator;
* exact ID and hash encodings;
* exact reduced authorization-reference encoding;
* exact verification-reference or no-reference commitment encoding;
* normative test vectors.

## 31.8 Exact `identity_create` payload syntax

**Classification:** SPECIFICATION BLOCKER for Appendix A syntax; semantic field decisions are CLOSED.

Appendix A must finalize exact syntax and optionality for:

```text
identity_id
initial_key_descriptor
initial_public_key_ref
initial_key_possession_proof
admission_authorization_reference
verification_reference
```

The semantic decision is closed that:

* Profile-v0 target identity kind is fixed as `identity_kind = human` by profile, not a free-form payload field;
* `speaker_identity_id` MUST be absent;
* `verification_reference` is optional and narrowly constrained;
* `verification_reference` must not point directly to private request material;
* `verification_reference` has no admission, verification, or eligibility effect merely by appearing;
* Appendix A must choose one canonical no-reference representation.

## 31.9 Restricted verification event catalog

**Classification:** SPECIFICATION BLOCKER before public admission.

Appendix A and the Verification Specification must explicitly enumerate the event families available to `CanonicalAdmittedIdentity` identities.

The catalog must state which restricted events create or relate ordinary truth claims, ordinary evidence objects or relationships, contradiction relationships, ordinary challenge objects, challenge responses, protected commitments, and authorized outcomes.

A broad category such as verification events is insufficient.

## 31.10 Verification claim schemas

**Classification:** SPECIFICATION BLOCKER for full verification progression; common ontology is CLOSED.

Verification claims, evidence, contradictions, challenges, attestations, and outcomes use the ordinary canonical epistemic ontology.

The Verification Specification must still define initial constrained schemas for:

* human-control claim;
* self-correspondence claim;
* continuity claim;
* duplicate-control claim;
* automation claim;
* compromise claim;
* interaction attestation;
* challenge response.

Each schema must remain a constrained verification profile of the ordinary canonical truth, evidence, contradiction, and challenge ontology.

## 31.11 `identity_verification_update` reconciliation

**Classification:** SPECIFICATION BLOCKER for Appendix A reconciliation; post-genesis direct status authority is CLOSED.

Appendix A currently names `identity_verification_update`.

It must be reconciled so it is not an ordinary post-genesis public event that directly sets verification status or enables ordinary authorship.

If retained, it must be limited to explicit genesis/import/legacy compatibility treatment with provenance class, scope, transition or retirement rule, and no claim to objective truth by declaration.

## 31.12 Verification thresholds and formulas

**Classification:** RULEBOOK PARAMETER after formula authority exists.

The Verification Specification and active rulebook must define thresholds, formulas, and boundary activation for:

* VH required for ordinary writing;
* VH required for challenge participation;
* VH or VI required for inviter eligibility;
* attester eligibility;
* evidence diversity;
* continuity;
* lineage-taint or concentration decay.

Civil-identity disclosure must not be universally required.

## 31.13 Minimum inviter maturation

**Classification:** RULEBOOK PARAMETER with constitutional lower bound.

The value must be greater than zero certified qualifying capacity periods.

The recommended initial value requires modeling and testing.

## 31.14 Invitation-capacity generation rate

**Classification:** RULEBOOK PARAMETER with constitutional lower bound.

The initial rulebook must define the integer amount generated per eligible qualifying period.

The value should be conservative enough to limit recursive growth while permitting broad access.

The constitutional minimum is closed in this specification: each unsuspended inviter-eligible identity receives at least one spendable unit in each qualifying capacity period.

## 31.15 Carryover cap and spend-frequency cap

**Classification:** RULEBOOK PARAMETER.

The rulebook must define:

* maximum stored units;
* cap as absolute number or rate multiple;
* expiration order;
* any additional per-cycle or per-period spend-frequency cap.

Unlimited carryover is prohibited.

## 31.16 Existing-capacity behavior during suspension and stalls

**Classification:** CLOSED for stalls; RULEBOOK PARAMETER for suspension details.

During stalled or non-qualifying periods, existing valid capacity remains spendable unless it is suspended, expired under a rule already applicable before the stall, frozen by an authorized emergency rule, or blocked by another explicit constitutional rule.

During suspension, the rulebook must choose among freeze, reduction, expiration, or partial retention.

The suspension behavior must be explicit and prospective.

## 31.17 Restoration activation boundary

**Classification:** CLOSED for Profile-v0 boundary class; RULEBOOK PARAMETER for restoration predicate details.

Restoration requires an authorized canonical restoration outcome and the qualifying activation boundary required by the active rulebook.

A forced, degraded, survivor, record-only, Dmax-only, or machine-only boundary MUST NOT restore invitation eligibility or capacity unless it separately satisfies the required human-deliberative certification rules.

## 31.18 Public visibility of exact remaining capacity

**Classification:** CLOSED under Profile v0.

Exact remaining invitation capacity is publicly derivable from public canonical history, rulebooks, certified cycle boundaries, and successful debits.

A public interface may omit, bucket, delay, or simplify the displayed value for coercion or targeting concerns, but that is presentation minimization only.

DTO omission or UI hiding is not cryptographic privacy.

A genuinely private capacity model requires a future cryptographic admission profile.

## 31.19 Private verification commitments

**Classification:** SPECIFICATION work.

The Privacy and Verification Specifications must define:

* commitment format;
* authorized verifier roles;
* public outcome;
* challenge process;
* evidence-retention rules;
* deletion or expiration behavior.

## 31.20 Genesis profile

**Classification:** SPECIFICATION BLOCKER before deployment.

The authoritative genesis profile must specify:

* initial identities;
* keys;
* human classifications;
* inviter eligibility;
* finite capacity;
* bootstrap maturation exception;
* transition condition;
* genesis commitment.

## 31.21 Legacy transition

**Classification:** SPECIFICATION BLOCKER for migration.

The implementation must settle:

* which legacy identities remain writable;
* how Profile-v0 keys are registered;
* temporary writer grants;
* inviter transition;
* provenance labels;
* migration manifest.

## 31.22 Key lifecycle, recovery, and duplicate-human remedies

**Classification:** SPECIFICATION BLOCKER for direct key-lifecycle runtime where applicable; FUTURE PROFILE for recovery and consolidation paths not required by admission v0.

Remaining work includes:

* exact key-rotation payloads;
* exact key-revocation payloads;
* last-key and recovery behavior;
* duplicate-human consolidation or participation remedy;
* non-retroactive historical signature validity vectors.

## 31.23 Public request-pool protocol

**Classification:** IMPLEMENTATION CHOICE initially; possible later transport standard.

Request pools may vary locally.

A common portable request format is desirable but non-canonical unless a future transport profile adopts it.

## 31.24 Request-stage possession-proof format

**Classification:** transport-profile specification.

The final event-bound canonical proof is mandatory.

The reusable request-stage proof format remains subordinate and non-canonical.

## 31.25 Anti-spam transport mechanisms

**Classification:** IMPLEMENTATION CHOICE.

Relays may choose rate limits, proof of work, CAPTCHAs, deposits, moderation, and queue limits.

No mechanism becomes universal proof of humanity.

## 31.26 High-risk sponsor privacy and anonymous credentials

**Classification:** FUTURE verification evidence and possible admission profile.

Profile v0 exposes sponsor authorship.

Threshold sponsorship, anonymous sponsorship, and optional anonymous credentials require later profiles.

No provider is mandatory.

## 31.27 Bounded open-admission fallback and distributed ceremonies

**Classification:** FUTURE PROFILE.

A globally bounded non-sponsor lane remains desirable as a possible long-term anti-gatekeeping fallback.

Distributed ceremony authority, privacy, geographic access, and collusion resistance remain unresolved.

Neither path is specified or active under Profile v0.

## 31.28 Rulebook safe ranges

**Classification:** protocol-specification follow-up.

The governing documents must define hard safe bounds for:

* minimum maturation;
* minimum generated capacity, already fixed here as at least one unit per unsuspended inviter-eligible identity in each qualifying period;
* maximum generation;
* maximum carryover;
* maximum restricted-lane payload size;
* maximum admission rate;
* suspension review.

## 31.29 Forced-cycle admission effects

**Classification:** CLOSED for Profile v0 in this specification; Cycle Specification reconciliation still required.

Under Profile v0, Dmax-only, forced, degraded, survivor, record-only, and machine-only boundaries:

* generate no capacity;
* advance no maturation;
* activate no new inviter eligibility;
* restore no invitation suspension;
* increase no carryover caps;
* permit only existing-capacity spending where not separately suspended, expired, frozen, or constitutionally blocked.

The Cycle Specification must be reconciled to expose the boundary status and human-deliberative certification data needed to replay this rule.

## 31.30 Open-question closure rule

No unresolved item in this section may be implemented through private discretion where it affects canonical validity.

Before activation, it must be resolved through:

* authoritative specification;
* rulebook decision within constitutional bounds;
* conformance vectors;
* implementation-status update.
# 32. Cross-document reconciliation checklist

## 32.1 Purpose

Adoption of this specification requires coordinated updates across the authoritative document set.

This section is a non-normative reconciliation checklist. It does not introduce independent protocol rules.

A document MUST NOT be marked reconciled until its relevant statements are consistent with this specification.

## 32.2 `protocol v5.md`

Reconcile Protocol v5 to:

* replace stale language implying that a new user simply registers and emits identity ideas;
* recognize sponsor-authored canonical identity admission;
* distinguish local identity preparation from canonical identity creation;
* preserve human-first authorship;
* state that admission does not grant ordinary authority;
* prohibit permanent inviter classes;
* preserve pseudonymous participation;
* recognize verification through ordinary truth claims, evidence, contradictions, challenges, and outcomes;
* distinguish Anthill topology from verification;
* adopt Mindgarden, Backyard of Relationships, Self Tree, and Anthill as the Profile-v0 `identity_structural_roots` names;
* reserve future admission profiles.

## 32.3 `protocol v5-appendix-a.md`

Appendix A must define exact event schemas and effects for:

* `identity_create`;
* `identity_key_rotate`;
* `identity_key_revoke`;
* required restricted verification-lane events;
* verification claims and attestations needed for initial progression;
* admission-related suspension and restoration where event-based.

Appendix A must also reconcile `identity_verification_update` so it cannot operate as an ordinary post-genesis direct verification-status or authorship-enabling event.

If retained, it must be compatibility-only for explicit genesis/import/legacy state.

For `identity_create`, Appendix A must specify:

* canonical author;
* target constraints;
* payload fields;
* key descriptor;
* possession proof;
* reduced authorization reference;
* optional and narrowly constrained verification reference;
* applicant proof and sponsor-signature construction order;
* capacity debit;
* `identity_structural_roots` and Anthill-anchor effect;
* initial inactive state;
* stable rejection rules;
* idempotency;
* conflict behavior.

## 32.4 `canonical-event-authorship-and-signature-profile-v0.md`

Update to define:

* sponsor-authored `identity_create`;
* applicant initial-key possession bytes, including verification-reference or no-reference binding;
* key-descriptor hashing;
* event and sponsor binding;
* request-stage proof boundary;
* key rotation;
* key revocation;
* active/superseded/revoked states;
* key-management authorization independent of ordinary writer eligibility;
* non-retroactive historical signature validity.

## 32.5 `canonical-encoding-and-hashing-spec.md`

Add exact encodings for:

* initial key descriptor;
* `initial_public_key_ref`;
* applicant possession-proof bytes;
* reduced admission-authorization context;
* admission-authorization reference;
* canonical no-reference encoding for `verification_reference`;
* admission-profile identifiers;
* Anthill-anchor derivation if deterministic;
* admission-lineage edge identifiers;
* capacity-period references.

Provide normative vectors.

## 32.6 `deterministic-replay-and-merge-spec.md`

Update replay rules for:

* sponsor-authored identity creation;
* applicant possession proof;
* event-position validation of sponsor key state, inviter eligibility, suspension, capacity, target uniqueness, key uniqueness, and applicant proof;
* sponsor eligibility;
* capacity generation and debit;
* `CanonicalAdmittedIdentity` status with ordinary participation lanes inactive;
* restricted verification lane;
* `identity_structural_roots`, including Anthill anchor;
* admission lineage;
* cycle-boundary activation;
* conflict ordering;
* idempotency;
* legacy provenance;
* no hidden private inputs.

Reconcile any blanket rule requiring the newly created identity to pre-exist as a verified author.

## 32.7 `verification-spec.md`

Update to:

* preserve invite-based admission intent;
* define sponsorship as distinct from verification;
* define verification claims as ordinary canonical truth claims;
* define verification evidence through the ordinary evidence ontology;
* define initial authorized verification artifacts;
* specify restricted-lane author constraints;
* deprecate direct post-genesis `identity_verification_update` status setting;
* define attester eligibility;
* define writer eligibility;
* define inviter eligibility;
* define lineage-taint boundaries;
* define evidence diversity;
* prevent verification-weighted truth and votes;
* preserve pseudonymous VH without mandatory civil VI;
* define challenge and restoration behavior.

## 32.8 `cycle-spec.md`

Update to define or expose:

* invitation-capacity period;
* qualifying human-deliberative capacity period;
* inviter-eligibility basis;
* generation boundary;
* `admission_liveness_blocked` inputs;
* maturity counting;
* carryover boundary;
* forced, degraded, survivor, record-only, and Dmax-only boundary behavior;
* zero-participation behavior;
* Dmin and Dmax effects;
* suspension and restoration activation;
* existing-capacity spendability during stalls;
* pending-candidate behavior at boundaries;
* rulebook transitions.
## 32.9 `tempo-spec.md`

Confirm that:

* Tempo eligibility remains separate from identity existence;
* admission does not grant Tempo authority;
* verification does not weight Tempo truth influence;
* cycle participation does not automatically generate invitation authority outside the cycle rulebook.

No broad Tempo redesign should be required.

## 32.10 `privacy-and-high-risk-submission-spec.md`

Update to define:

* pseudonymous admission requests;
* privacy-safe verification commitments and explicit prohibition on direct private request pointers;
* commitment-based high-risk evidence;
* sponsor and applicant privacy boundaries;
* relay metadata exclusion;
* canonical sponsor provenance;
* no mandatory legal identity;
* no disclosure of private request content.

## 32.11 `offline-and-mindseed-spec.md`

Update to define:

* offline local identity preparation;
* offline admission requests;
* store-and-forward admission transport;
* stale-candidate behavior;
* offline applicant possession proof;
* offline capacity-view warnings;
* canonical publication requirements;
* no offline bypass of admission scarcity.

## 32.12 `node-and-conformance-spec.md`

Update node requirements for:

* admission-event validation;
* key-proof verification;
* capacity derivation;
* restricted-lane enforcement;
* stable errors;
* replay equality;
* public reads;
* conformance fixture execution;
* database isolation;
* generated-export proof.

## 32.13 `protocol-event-registry.v1.md`

Register or reconcile:

* `identity_create`;
* `identity_key_rotate`;
* `identity_key_revoke`;
* verification claim events;
* attestation events;
* identity-scoped challenge events;
* suspension/restoration events;
* derived capacity effects.

Mark implementation status accurately.

## 32.14 `cross-doc-invariants.md`

Add invariants stating:

* local identity preparation is permissionless;
* canonical admission is sponsor-authored under Profile v0;
* sponsorship is not verification;
* admission does not grant ordinary authority;
* verification uses ordinary truth, evidence, contradiction, and challenge ontology;
* raw verification artifacts, VH/VI certainty, and eligibility lanes are distinct;
* `identity_verification_update` is not ordinary post-genesis verification-status authority;
* Anthill edges do not automatically verify;
* invitation capacity is replay-derived and non-transferable;
* private account state does not authorize admission;
* AI cannot sponsor;
* `CanonicalAdmittedIdentity` identities have a restricted verification lane;
* forced machine-only cycles do not mint ordinary invitation authority.

## 32.15 `authoritative-index.md`

Add this specification with an explicit authority description.

Define its precedence for:

* admission architecture;
* sponsorship;
* invitation capacity;
* admission lineage;
* initial identity authority.

Preserve higher authority for Protocol v5 Section 0 and exact encoding/event schema documents in their domains.

## 32.16 `authoritative-stage-map.md`

Map this specification to the appropriate stage for:

* canonical identity lifecycle;
* verification;
* writer eligibility;
* inviter eligibility;
* admission;
* later Tempo prerequisites.

## 32.17 `api-contract-read-only.md`

Add or update public reads for:

* identity detail;
* admission provenance;
* sponsor lineage;
* key history;
* `identity_structural_roots`;
* Anthill anchor;
* verification summary;
* writer eligibility;
* inviter eligibility;
* capacity summary;
* suspension;
* legacy classification;
* historical reads.

Document safe presentation minimization. It MUST NOT claim that exact remaining capacity is secret under Profile v0, because conforming nodes can derive it from public canonical state.

## 32.18 Canonical write API contract

The canonical write contract must register support for:

* `identity_create`;
* key lifecycle events;
* restricted verification events.

It must define:

* signed-candidate requirements;
* stable errors;
* idempotency;
* retryability;
* no private account authentication dependency.

## 32.19 `open-core-implementation-status.md`

Track separately:

* specification status;
* key-lifecycle status;
* verification substrate;
* writer eligibility;
* inviter eligibility;
* capacity derivation;
* identity admission;
* request transport;
* conformance;
* export proof;
* private integration.

Do not mark transport UI as canonical admission authority.

## 32.20 Safety specifications

Review:

* `safety-spec.md`;
* `safety-rulebook-interface-mechanics-spec.md`.

Ensure emergency admission restrictions:

* are canonical;
* are scoped;
* expire or undergo review;
* do not create permanent operator authority;
* do not target viewpoint;
* do not erase valid identities.

## 32.21 Governance specification

Ensure governance:

* may tune rulebook parameters only within constitutional bounds;
* cannot create a permanent inviter class;
* cannot require ideological admission;
* cannot privately appoint inviters;
* cannot weight votes by verification;
* must use the appropriate amendment path for protocol-fixed changes.

## 32.22 AI boundaries specification

Ensure AI may:

* assist;
* summarize;
* detect candidate anomalies;
* draft claims.

AI may not:

* admit identities;
* establish VH or VI;
* suspend inviters;
* decide canonical abuse outcomes;
* substitute for human cycle participation.

## 32.23 Snapshot specification

Ensure snapshots commit to:

* identity provenance;
* keys;
* identity structural roots;
* Anthill anchors;
* lineage;
* eligibility;
* capacity periods;
* balances;
* suspensions;
* rulebook references;
* restricted-lane state.

Snapshot-plus-tail replay must equal full replay.

## 32.24 DTO and schema checks

Update DTO drift and schema validation to ensure:

* public/private boundaries;
* exact event fields;
* no private account IDs;
* profile versioning;
* stable eligibility meanings;
* legacy provenance.

## 32.25 Conformance fixtures

Add the fixture classes from Section 27.

The fixture set should include:

* valid paths;
* negative paths;
* race conditions;
* cycle behavior;
* legacy migration;
* export behavior;
* private-state independence.

## 32.26 Generated export

Ensure generated open-core export includes:

* authoritative specs;
* event DTOs;
* migrations;
* replay logic;
* public reads;
* test guard;
* conformance fixtures;
* no private product dependencies.

## 32.27 Reconciliation completion criteria

Cross-document reconciliation is complete only when:

* no authoritative document still implies unqualified self-registration;
* no document treats sponsorship as verification;
* no document makes Anthill relationships automatic evidence;
* no document grants writing through identity creation;
* no document relies on private inviter state;
* cycle and capacity behavior are deterministic;
* exact event and proof encodings are settled;
* conformance checks pass.

---

# 33. Summary of Profile-v0 guarantees

Section 33 is an informative summary of rules defined earlier in this specification.

The normative rule remains in the cited earlier sections; if this summary conflicts with an earlier normative section, the earlier section controls.

## 33.1 Permissionless preparation

Every person may locally generate:

* identity candidates;
* keypairs;
* possession proofs;
* admission requests.

No canonical permission is required.

## 33.2 Non-canonical pending admission

Local candidates and admission requests remain outside canonical shared state until valid admission.

Failed, expired, abandoned, or unsponsored requests do not automatically consume permanent canonical identity storage.

## 33.3 Sponsored canonical admission

Normal post-genesis Profile-v0 identity creation requires:

* one eligible canonical human sponsor;
* one active sponsor key;
* one unit of available invitation capacity;
* one valid applicant initial-key possession proof;
* one valid canonical `identity_create`.

## 33.4 No permanent inviter class

Every canonical human identity that satisfies the same deterministic:

* verification;
* continuity;
* maturation;
* key-control;
* good-standing

requirements may become inviter-eligible.

For inviter eligibility to have practical effect, every unsuspended inviter-eligible identity receives at least one spendable capacity unit in each qualifying capacity period.

Founders, operators, delegates, experts, institutions, and genesis identities do not possess permanent exclusive admission authority.
## 33.5 Stranger sponsorship

A sponsor may admit a stranger.

Prior acquaintance, shared organization, shared politics, physical meeting, legal identity, or social-network membership is not a protocol requirement.

## 33.6 Pseudonymous access

An applicant may seek admission without universally disclosing:

* legal name;
* government ID;
* precise location;
* political beliefs;
* private communications;
* persistent network address.

Civil identity is not a universal prerequisite for human verification or canonical participation.

## 33.7 Scarce admission capacity

Canonical identity creation consumes replay-derived invitation capacity.

Capacity is:

* identity-bound;
* non-transferable;
* non-saleable;
* non-delegable under Profile v0;
* generated only by qualifying human-certified periods;
* carryover-bounded;
* deterministically debited.

A rulebook MUST NOT make inviter eligibility nominal by assigning zero capacity indefinitely to eligible unsuspended humans.

## 33.8 Human-controlled cycles

Wall-clock passage and machine activity alone do not mint recurring invitation authority.

Capacity generation and maturation MUST depend on qualifying human-certified cycle boundaries.

Dmax-only, forced, degraded, survivor, record-only, and machine-only boundaries MUST NOT generate capacity, advance maturation, activate inviter eligibility, restore invitation authority, or increase carryover caps unless they separately satisfy the required human-deliberative certification rules.

When no qualifying period exists, replay MUST expose `admission_liveness_blocked = true`.

Existing valid capacity remains spendable during stalls unless separately suspended, expired, frozen, or constitutionally blocked.
## 33.9 Minimal initial authority

A newly admitted identity receives:

* canonical existence;
* active initial key control;
* sponsor provenance;
* admission lineage;
* `identity_kind = human` as the Profile-v0 target kind;
* `identity_structural_roots`, including the Anthill root;
* restricted identity-verification participation.

It does not automatically receive:

* ordinary writing;
* general challenges;
* voting;
* governance;
* Tempo;
* inviting;
* economic authority.

## 33.10 Claim-based verification

Human verification is represented through explicit, signed, attributable, challengeable ordinary truth claims, evidence, contradictions, challenges, and outcomes.

A verification artifact is not true merely because it exists.

VH and VI are derived through:

* claims;
* evidence;
* contradictions;
* attestations;
* challenges;
* outcomes;
* active rulebooks.

No ordinary post-genesis status event may directly declare VH, VI, writer eligibility, or inviter eligibility.

## 33.11 Sponsorship is not verification

A sponsor spends admission capacity.

The sponsor does not automatically attest that the applicant:

* is human;
* is unique;
* corresponds to a known civil person;
* is trustworthy;
* will behave properly.

A sponsor may submit a separate challengeable verification attestation.

That attestation is an ordinary verification truth claim, not administrative certification.

## 33.12 Identity structural roots and Anthill are topology, not authority

Each Profile-v0 identity has the complete protocol-defined `identity_structural_roots` set: Mindgarden, Backyard of Relationships, Self Tree, and Anthill.

The Anthill is one specialized root within that set.

The Anthill organizes:

* relationships;
* verification artifacts;
* challenges;
* lineage;
* provenance.

Identity structural-root existence, Anthill membership, relationship count, social centrality, and admission ancestry do not automatically establish verification or participation authority.

## 33.13 Progressive eligibility

Identity existence, key control, raw verification artifacts, VH, VI, writer eligibility, challenge eligibility, voter eligibility, governance eligibility, Tempo eligibility, and inviter eligibility remain distinct.

Each authority activates only through its specified predicates and canonical boundaries.

## 33.14 Stronger inviter threshold

Invitation eligibility may require stronger:

* VH;
* continuity;
* maturation;
* evidence diversity;
* challenge survival

than limited ordinary writing.

A newly admitted identity cannot immediately reproduce through further invitations.

## 33.15 No verification-weighted truth or voting

Higher verification may affect:

* eligibility;
* event-family access;
* rate limits;
* invitation capacity.

It does not increase:

* truth weight;
* importance weight;
* governance vote weight;
* challenge vote weight;
* Tempo influence;
* economic entitlement.

## 33.16 Accountable but non-hereditary lineage

Every event-derived admission records sponsor provenance.

Lineage may support:

* audit;
* diversity calculations;
* abuse investigation;
* Sybil analysis.

Lineage does not create:

* guilt by association;
* inherited exclusion;
* social caste;
* automatic verification;
* automatic invalidation.

## 33.17 Narrow inviter accountability

Sponsors may face forward-looking consequences for established coordinated admission abuse.

They are not automatically responsible for:

* invitee opinions;
* ordinary errors;
* later disagreement;
* isolated misconduct;
* political unpopularity;
* failure to attain higher verification.

## 33.18 Deterministic replay

Every conforming node MUST reconstruct the same:

* identities;
* keys;
* sponsor provenance;
* reduced admission-authorization references;
* applicant proof bindings;
* identity structural roots;
* Anthill anchors;
* lineage;
* eligibility;
* capacity;
* debits;
* suspensions;
* restoration state

from the same genesis and canonical event history.

## 33.19 No private canonical authority

Canonical admission and verification do not depend on:

* accounts;
* sessions;
* passwords;
* email;
* private allowlists;
* operator database rows;
* private social scores;
* opaque AI judgments.

## 33.20 No AI admission authority

AI may assist humans.

AI cannot independently:

* sponsor;
* verify;
* admit;
* suspend;
* adjudicate;
* mint capacity;
* replace human cycle participation.

## 33.21 Durable historical provenance

Valid identity creation, sponsor provenance, key history, and admission lineage remain part of canonical history.

Later changes may alter future authority.

They do not silently rewrite the past.

## 33.22 Future extensibility

Profile v0 activates sponsored public admission only.

Future profiles may add:

* bounded open admission;
* threshold sponsorship;
* distributed ceremonies;
* anonymous credentials;
* privacy-preserving authorization.

Each future path must be fully specified before activation.

## 33.23 Profile-v0 constitutional balance

Profile v0 is designed to preserve the following civilizational balance:

```text
any person may prepare an identity and seek admission

no central actor or inherited group permanently controls entry

canonical identity creation remains scarce and replayable

inviter eligibility has practical capacity effect in qualifying periods

new identities begin with little authority

verification develops through challengeable truth claims and evidence

real humans can progressively gain writing and invitation rights

bots and Sybil identities cannot cheaply flood or reproduce

pseudonymous and high-risk humans retain a viable path to participation
```
## 33.24 Completion statement

A Profile-v0 implementation conforms to this specification only when it proves:

* sponsor-authored canonical admission;
* applicant initial-key possession;
* reduced admission-authorization context;
* exact verification-reference or canonical no-reference binding;
* replay-derived sponsor eligibility;
* replay-derived invitation capacity;
* positive capacity for unsuspended inviter-eligible identities in qualifying periods;
* `admission_liveness_blocked` when no qualifying period exists;
* no capacity generation, maturation, activation, restoration, carryover-cap increase, rewards, or authority from non-qualifying boundaries;
* existing-capacity spendability during stalls where not separately blocked;
* atomic creation and capacity debit;
* minimal initial authority;
* restricted verification participation;
* Anthill separation;
* claim-based VH and VI boundaries;
* deterministic replay;
* legacy provenance preservation;
* public-read safety;
* open-core independence;
* conformance and database-isolation acceptance.

Source code presence without those executed proofs is not sufficient.
