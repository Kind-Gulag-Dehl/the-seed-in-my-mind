# Proposed outline: `identity-admission-and-invitation-capacity-spec-v0.md`

The document should resolve the admission-layer gaps identified by the audit: identity-creation authorship, invitation authorization, invitation capacity, non-canonical requests, Sybil resistance before identity creation, and access for unconnected high-risk applicants. 

## Document header

* Title: **Identity Admission and Invitation Capacity Specification v0**
* Status: Draft / Authoritative once adopted
* Version: `0.1.0`
* Protocol profile: Profile v0
* Authority level
* Effective protocol version
* Related specifications
* Change-control requirements

---

## 0. Normative language and notation

### 0.1 Normative terms

Define:

* MUST
* MUST NOT
* SHOULD
* SHOULD NOT
* MAY

### 0.2 Identifiers and references

Define notation for:

* identity IDs
* event IDs
* public-key references
* cycle IDs
* rulebook IDs
* canonical positions
* invitation-capacity units
* admission-authorization references

### 0.3 Authority boundaries

State that this specification governs:

* admission into canonical identity state;
* sponsor authorization;
* invitation eligibility and capacity;
* the canonical effects of `identity_create`;
* admission provenance;
* the boundary between admission and later verification.

State that it does not replace:

* the authorship/signature profile;
* the verification specification;
* the cycle specification;
* Appendix A event schemas;
* deterministic replay rules;
* privacy and offline transport rules.

---

## 1. Purpose

### 1.1 Problem statement

Explain the need to balance:

* permissionless access;
* resistance to permanent gatekeeping;
* pseudonymous and high-risk participation;
* protection from bot and Sybil flooding;
* bounded canonical storage growth;
* deterministic decentralized enforcement.

### 1.2 Design objective

State the Profile-v0 model:

```text
permissionless local identity preparation
→ non-canonical admission request
→ eligible sponsor consumes invitation capacity
→ canonical identity creation
→ inactive identity with active initial key
→ separate verification progression
→ later writer and inviter eligibility
```

### 1.3 Civilizational-scale requirements

Include:

* no permanent inviter class;
* no mandatory government or corporate identity provider;
* no centralized operator discretion;
* no inherited admission monopoly;
* no unrestricted multiplication of canonical authority through key generation;
* long-term deterministic replay and auditability.

---

## 2. Core invariants

### 2.1 Permissionless local preparation

Anyone may create local keys and identity material without protocol permission.

### 2.2 Canonical admission is distinct from local identity creation

A locally generated identity is not yet canonical.

### 2.3 Admission is distinct from verification

```text
identity existence
!= key control
!= verified-human status
!= writer eligibility
!= challenge eligibility
!= voter eligibility
!= governance eligibility
!= Tempo eligibility
!= inviter eligibility
```

### 2.4 Human sponsorship

Normal post-genesis Profile-v0 identity creation requires an eligible human sponsor.

### 2.5 No permanent privileged class

All human identities satisfying the same deterministic requirements must be capable of becoming inviter-eligible.

### 2.6 Canonical scarcity

Canonical identity creation must consume replay-derived admission capacity.

### 2.7 Political and epistemic neutrality

Sponsorship must not mean endorsement of:

* beliefs;
* factual claims;
* political positions;
* future conduct;
* legal or civil identity.

### 2.8 No private authority

Private accounts, sessions, operator tables, or undisclosed databases must not determine canonical admission eligibility.

### 2.9 No AI admission authority

AI and system identities must not sponsor ordinary human identity creation.

---

## 3. Scope

### 3.1 Included

* local identity candidates;
* non-canonical admission requests;
* sponsorship;
* invitation eligibility;
* invitation capacity;
* canonical `identity_create`;
* applicant proof of key possession;
* initial identity state;
* sponsor provenance;
* capacity debit;
* replay;
* admission-related suspension;
* high-risk and stranger admission requirements.

### 3.2 Excluded

* full verification scoring;
* exact VH/VI formulas;
* ordinary writing rates;
* challenge rates;
* voting rules;
* governance participation;
* Tempo participation;
* private accounts and sessions;
* key custody;
* key recovery;
* distributed ceremonies;
* anonymous proof-of-personhood systems;
* open canonical self-registration;
* globally bounded lotteries;
* frontend admission-pool design.

---

## 4. Terminology

Define precisely:

### 4.1 Local identity candidate

A locally generated identity ID, keypair, descriptor, and related private state.

### 4.2 Admission request

A signed, portable, non-canonical request seeking sponsorship.

### 4.3 Applicant

The controller of the proposed initial key.

### 4.4 Sponsor

The canonical human author of `identity_create`.

### 4.5 Invitation eligibility

Replay-derived permission to sponsor identities.

### 4.6 Invitation capacity

Replay-derived, spendable admission allowance.

### 4.7 Admission authorization

The canonical proof that the sponsor was eligible and had available capacity.

### 4.8 Canonical inactive identity

A created identity with valid key control but no ordinary participation authority.

### 4.9 Admission lineage

The historical sponsor relationship created by canonical identity admission.

### 4.10 Coordinated admission abuse

Deliberate abuse such as Sybil sponsorship, invitation selling, or admission-ring operation.

### 4.11 Genesis-admitted identity

An identity introduced through authoritative genesis state rather than ordinary post-genesis admission.

### 4.12 Event-derived identity

An identity created by a valid canonical `identity_create` event.

---

## 5. Admission architecture

### 5.1 Layer separation

Define three layers:

1. Local/private identity preparation.
2. Non-canonical request and transport.
3. Canonical admission.

### 5.2 Trust boundary

State that request relays and interfaces may:

* transport;
* filter local spam;
* queue;
* mirror;
* match applicants and sponsors.

They may not decide canonical eligibility.

### 5.3 Canonical decision point

Canonical identity creation occurs only when:

* the sponsor’s authored event is valid;
* sponsor eligibility is valid;
* invitation capacity is available;
* applicant possession proof is valid;
* all canonical event constraints pass.

---

## 6. Admission state machine

### 6.1 Non-canonical states

```text
LocalCandidate
AdmissionRequest
SponsoredPendingPublication
```

Clarify that these do not affect canonical replay.

### 6.2 Canonical states

```text
CanonicalInactiveIdentity
WriterEligibleIdentity
InviterEligibleIdentity
SuspendedInviter
DormantIdentity
```

Only define admission-related aspects of later states; defer verification transition formulas.

### 6.3 Valid transitions

```text
LocalCandidate
→ AdmissionRequest
→ SponsoredPendingPublication
→ CanonicalInactiveIdentity
```

Later, through separate specifications:

```text
CanonicalInactiveIdentity
→ WriterEligibleIdentity
→ InviterEligibleIdentity
```

### 6.4 Invalid transitions

Reject:

* local candidate directly becoming canonical without authorization;
* applicant self-sponsorship;
* AI sponsorship;
* operator-only admission;
* invitation capacity below one;
* reuse of a consumed capacity unit;
* ordinary authorship becoming active automatically at creation.

---

## 7. Non-canonical admission requests

### 7.1 Required properties

An admission request should bind:

* proposed identity ID;
* initial public-key descriptor;
* initial public-key reference;
* applicant possession proof;
* request version;
* expiration or freshness information where applicable;
* optional privacy-preserving contact or relay information.

### 7.2 Non-effects

An admission request does not:

* create identity state;
* reserve an identity ID globally;
* grant authority;
* consume canonical capacity;
* establish verification;
* create canonical lineage.

### 7.3 Transport independence

Requests may be transported through:

* direct exchange;
* public pools;
* anonymous relays;
* offline packs;
* store-and-forward delivery;
* privacy-preserving matching systems.

### 7.4 Spam handling

Transport-level systems may apply local anti-spam measures, but these must not become canonical proof of humanity.

### 7.5 Privacy requirements

The request need not expose:

* legal name;
* precise location;
* government identity;
* persistent network address;
* political beliefs;
* proposed canonical contributions.

---

## 8. Sponsor eligibility

### 8.1 Required conditions

Immediately before `identity_create`, the sponsor must:

* exist canonically;
* be a human identity;
* control an active valid key;
* satisfy current inviter-eligibility rules;
* possess at least one available invitation-capacity unit;
* not be under active admission suspension.

### 8.2 Universal eventual eligibility

State that any human satisfying the same objective conditions may become inviter-eligible.

### 8.3 Prohibited sponsor classes

Reject sponsorship by:

* AI identities;
* system identities;
* private accounts;
* node operators acting only as operators;
* non-canonical identities;
* suspended inviters;
* identities without active keys.

### 8.4 Sponsor-target separation

The sponsor and target identity must differ.

### 8.5 Stranger sponsorship

No prior personal relationship is required.

### 8.6 Sponsor meaning

The sponsor attests only that one unit of admission capacity should be spent to admit the target into the verification system.

---

## 9. Invitation eligibility

### 9.1 Structural eligibility inputs

The later rulebook may consider:

* verified-human certainty;
* identity continuity;
* active key control;
* maturation across certified cycles;
* evidence diversity;
* current admission-abuse status;
* unresolved identity challenges.

### 9.2 Non-permitted inputs

Inviter eligibility must not depend on:

* political agreement;
* popularity;
* wealth;
* expert status alone;
* governance office alone;
* operator status;
* legal identity alone;
* payment to another participant.

### 9.3 Boundary activation

Eligibility changes must activate at defined deterministic boundaries.

### 9.4 Suspension and restoration

Suspension must be:

* canonically justified;
* challengeable;
* time-bounded or reviewable;
* replay-derived;
* restorable through defined rules.

---

## 10. Invitation capacity

### 10.1 Core properties

Invitation capacity is:

* identity-bound;
* non-transferable;
* non-saleable;
* non-delegable in Profile v0;
* replay-derived;
* cycle-generated;
* deterministically spendable.

### 10.2 Generation

Define structural behavior:

* generated at certified cycle boundaries;
* only for inviter-eligible identities;
* no generation during ineligible periods;
* no retroactive generation unless explicitly defined.

### 10.3 Accumulation

Permit bounded rollover while prohibiting unlimited hoarding.

Exact cap remains rulebook-controlled.

### 10.4 Consumption

One successful canonical identity creation consumes one unit unless a later profile specifies otherwise.

### 10.5 Debit timing

Specify whether capacity is debited:

* at canonical event application;
* atomically with identity creation;
* only once for idempotent retries.

### 10.6 Double-spend prevention

Concurrent admissions using the same remaining unit resolve through canonical order.

### 10.7 No negative balance

An admission event must fail if available capacity is insufficient.

### 10.8 No recursive same-cycle expansion

A newly created identity must not become eligible to generate invitation capacity in the same cycle.

### 10.9 Rulebook-controlled parameters

Defer:

* generation rate;
* maturation duration;
* rollover cap;
* diversity threshold;
* suspension duration;
* restoration requirements.

---

## 11. Canonical `identity_create`

### 11.1 Canonical author

The sponsor is `author_identity_id`.

### 11.2 Target identity

The payload identifies the new target identity.

### 11.3 Proposed payload

```text
identity_id
initial_key_descriptor
initial_public_key_ref
initial_key_possession_proof
admission_authorization_reference
verification_reference
```

### 11.4 Derived sponsor identity

Sponsor identity should normally derive from the authored-candidate envelope rather than being duplicated in the payload.

### 11.5 Validation sequence

1. Validate event encoding.
2. Validate sponsor signature.
3. Validate sponsor identity and active key.
4. Validate inviter eligibility.
5. Validate available invitation capacity.
6. Validate target identity uniqueness.
7. Validate initial key descriptor.
8. Validate public-key reference.
9. Validate applicant possession proof.
10. Validate admission authorization.
11. Append event and materialize effects atomically.
12. Debit invitation capacity exactly once.

### 11.6 Canonical effects

A valid event:

* creates the canonical identity;
* registers its initial key;
* activates the initial key;
* records sponsor provenance;
* records creation position;
* consumes sponsor capacity;
* assigns canonical inactive status.

### 11.7 Explicit non-effects

It does not:

* verify the target as human;
* grant writer eligibility;
* grant challenge eligibility;
* grant voting rights;
* grant governance rights;
* grant Tempo rights;
* grant invitation eligibility;
* create a private account;
* create a session;
* create economic authority.

---

## 12. Applicant key possession

### 12.1 Purpose

Prevent sponsors from assigning keys to applicants who do not control them.

### 12.2 Required descriptor fields

```text
key_profile_version
signature_algorithm
raw_public_key_bytes
owning_identity_id
```

### 12.3 Profile-v0 constraints

* Ed25519
* exactly 32 public-key bytes
* descriptor owner equals target identity
* deterministic `public_key_ref`

### 12.4 Domain-separated proof

Define or reference exact possession-proof bytes binding:

* creation event ID;
* target identity ID;
* initial public-key reference;
* sponsor identity;
* protocol/profile version.

### 12.5 Distinction from canonical authorship

Applicant proof establishes key possession only.

It does not make the applicant the canonical author of `identity_create`.

---

## 13. Admission authorization reference

### 13.1 Purpose

Bind the admission event to replay-derived sponsor capacity.

### 13.2 Minimum semantics

It must identify or deterministically derive:

* applicable cycle;
* sponsor eligibility state;
* relevant rulebook version;
* capacity source;
* capacity debit.

### 13.3 Design alternatives

The specification should choose one:

* explicit capacity-unit identifier;
* deterministic sponsor/cycle sequence number;
* replay-derived ordinal debit.

### 13.4 Idempotency

Identical replay or retry must not debit capacity more than once.

### 13.5 Conflict handling

Different events competing for the same final unit resolve by canonical order.

---

## 14. Initial identity authority

### 14.1 Permitted actions

A canonical inactive identity may:

* read public state;
* manage its own keys;
* submit permitted verification evidence;
* respond to identity-scoped verification challenges;
* maintain continuity evidence.

### 14.2 Prohibited actions

It may not:

* create ordinary ideas;
* create ordinary connections;
* issue general truth challenges;
* vote;
* govern;
* submit Tempo claims;
* invite others;
* receive participation tokens or equivalent authority.

### 14.3 Later activation

Ordinary authority requires separately derived verification and event-family eligibility.

---

## 15. Stranger, pseudonymous, and high-risk admission

### 15.1 No acquaintance requirement

Sponsors may admit unknown applicants.

### 15.2 Pseudonymity

Civil identity is not universally required.

### 15.3 Relay support

Requests may be relayed without the relay becoming sponsor or canonical author.

### 15.4 Offline support

Admission requests may be prepared offline and delivered later.

### 15.5 Canonical provenance

Profile v0 preserves canonical sponsor authorship.

### 15.6 Non-canonical privacy

The canonical record must not require:

* applicant IP address;
* physical location;
* private conversation;
* legal identity;
* relay path;
* sponsor-applicant contact details.

### 15.7 Censorship resistance

Applicants must be able to submit the same signed request through multiple independent transports.

---

## 16. Admission lineage

### 16.1 Canonical lineage edge

Canonical admission records a sponsor-to-created-identity provenance relationship.

### 16.2 Permitted uses

Lineage may be used for:

* detecting concentrated admission clusters;
* applying diminishing returns to same-lineage evidence;
* evaluating coordinated admission abuse;
* limiting self-reproducing Sybil trees.

### 16.3 Prohibited uses

Lineage must not automatically determine:

* truth;
* importance;
* vote weight;
* political legitimacy;
* guilt by association;
* retroactive invalidation.

### 16.4 Cross-lineage evidence

Cross-lineage diversity may strengthen inviter eligibility, but must not become an absolute barrier for isolated populations without alternate evidence paths.

---

## 17. Inviter accountability

### 17.1 Non-liability for ordinary conduct

An inviter is not automatically responsible for:

* an invitee’s opinions;
* factual errors;
* political views;
* isolated misconduct;
* failure to become highly verified;
* disagreement with the sponsor.

### 17.2 Coordinated abuse

Possible abuse includes:

* intentional Sybil admission;
* invitation selling;
* invitation transfer;
* organized invitation rings;
* deliberate circumvention of admission rules;
* repeated fraudulent sponsorship.

### 17.3 Evidence requirements

Consequences require:

* canonical evidence;
* specified challenge procedures;
* deterministic certainty or verdict rules;
* appeal or restoration paths.

### 17.4 Permitted consequences

Possible forward-looking effects:

* reduced future capacity;
* temporary suspension;
* additional maturity requirements;
* increased evidence diversity requirements.

### 17.5 Prohibited consequences

Do not:

* erase valid historical events;
* retroactively invalidate legitimate invitees automatically;
* use opaque AI judgment as canonical authority;
* punish sponsors solely for controversial invitees.

---

## 18. Sybil-resistance model

### 18.1 Layered defense

Explain that the system uses:

1. scarce admission capacity;
2. maturation;
3. progressive verification;
4. diverse evidence;
5. limited rate permissions;
6. delayed invitation eligibility;
7. challengeable lineage-abuse detection.

### 18.2 Admission versus verification

Admission limits quantity.

Verification limits authority.

### 18.3 Sybil reproduction target

State the design objective that a compromised identity should not easily create a self-sustaining lineage of mature inviters.

### 18.4 No computational substitution

Proof of work, VDFs, or similar mechanisms may throttle transport spam but do not establish human status.

### 18.5 No reasoning-quality admission test

Reasoning quality must not determine whether a person may exist canonically.

---

## 19. Cycle integration

### 19.1 Capacity generation boundary

Invitation capacity becomes active only at defined cycle boundaries.

### 19.2 Eligibility snapshot

State which canonical state determines eligibility for a cycle.

### 19.3 Consumption during a cycle

Define how debits affect remaining capacity.

### 19.4 Cycle close

Define whether pending admission events are evaluated under the eligibility state at:

* event application;
* cycle opening;
* another exact canonical position.

### 19.5 Stalled cycles

Explain behavior when cycles do not close.

Avoid allowing clock time alone to mint invitation capacity.

### 19.6 Forced boundaries

Clarify whether forced boundaries can generate invitation authority or merely preserve system continuity.

---

## 20. Deterministic replay and conflict resolution

### 20.1 Replay inputs

Replay must reconstruct:

* identity existence;
* sponsor provenance;
* initial key state;
* inviter eligibility;
* generated capacity;
* consumed capacity;
* remaining capacity;
* suspension state.

### 20.2 Canonical-order conflicts

Define:

* two sponsors targeting the same identity ID;
* one sponsor attempting two admissions with one remaining unit;
* identical retries;
* conflicting duplicate event IDs;
* concurrent suspension and admission;
* rulebook change near admission.

### 20.3 Arrival-order independence

Results depend only on canonical order.

### 20.4 No hidden inputs

Wall-clock time, private account state, node-local reputation, and opaque AI scores must not affect replay.

---

## 21. Genesis and bootstrap boundary

### 21.1 Genesis-admitted identities

Define how initial identities enter authoritative state.

### 21.2 Initial invitation capacity

Specify whether genesis state assigns initial capacity directly or derives it from initial rulebook state.

### 21.3 Distinction from normal admission

Genesis admission is not ordinary `identity_create`.

### 21.4 No permanent bootstrap authority

Genesis status must not provide permanent special admission powers.

### 21.5 Legacy operator-provisioned identities

Classify legacy state without fabricating canonical admission events.

---

## 22. Legacy compatibility

### 22.1 Identity classifications

```text
genesis_admitted
legacy_operator_provisioned
event_derived
```

### 22.2 Readability

Legacy identities remain readable.

### 22.3 New writes

Legacy identities may require valid Profile-v0 key and eligibility state before new writes.

### 22.4 No fabricated history

Do not manufacture:

* sponsors;
* invitation debits;
* possession proofs;
* identity-creation events.

---

## 23. Stable validation and rejection rules

Define stable errors for:

* unauthorized sponsor;
* sponsor not human;
* sponsor suspended;
* inviter ineligible;
* insufficient invitation capacity;
* duplicate identity;
* duplicate public key;
* malformed descriptor;
* invalid key reference;
* invalid possession proof;
* self-sponsorship;
* AI/system sponsorship;
* invalid admission authorization;
* capacity already consumed;
* unsupported admission profile;
* applicant already canonical.

---

## 24. Security and abuse analysis

### 24.1 Threat actors

* bot farms;
* nation states;
* wealthy coordinated actors;
* invitation markets;
* compromised sponsors;
* captured relays;
* ideological cartels;
* compromised keys.

### 24.2 Gatekeeping risks

* inherited networks;
* regional exclusion;
* language exclusion;
* chilling effects from sponsor liability;
* relay censorship.

### 24.3 Sybil risks

* invitation rings;
* cross-attestation farms;
* recursive lineages;
* synthetic activity;
* mass request spam.

### 24.4 Mitigations

Map each risk to protocol or rulebook controls.

### 24.5 Residual risks

State clearly what Profile v0 cannot solve.

---

## 25. Constitutional rules versus rulebook parameters

### 25.1 Constitutional or protocol-fixed

* permissionless local key generation;
* sponsor-authored Profile-v0 admission;
* no permanent inviter class;
* no central mandatory identity provider;
* identity creation grants no ordinary authority;
* invitation capacity is non-transferable and non-saleable;
* canonical sponsor provenance;
* deterministic replay;
* no verification-weighted truth or votes;
* no unrestricted self-registration in Profile v0.

### 25.2 Rulebook-controlled

* verification threshold for inviter eligibility;
* maturation cycles;
* capacity generation rate;
* rollover cap;
* diversity requirements;
* suspension duration;
* restoration requirements;
* challenge and verdict thresholds;
* admission-abuse penalties.

### 25.3 Future-profile matters

* bounded open admission;
* distributed ceremonies;
* threshold admission;
* anonymous personhood credentials;
* alternate sponsor-hiding authorization proofs.

---

## 26. Future admission profiles

### 26.1 Bounded open admission

Reserved but inactive.

### 26.2 Distributed ceremony admission

Reserved but inactive.

### 26.3 Anonymous credential admission

Reserved but inactive.

### 26.4 Threshold sponsorship

Reserved but inactive.

### 26.5 Compatibility requirements

Future paths must define:

* scarcity;
* authorization;
* replay;
* storage;
* privacy;
* initial authority;
* abuse handling;
* interaction with existing invitation capacity.

---

## 27. Conformance requirements

### 27.1 Valid admission cases

* valid sponsor;
* valid applicant proof;
* valid stranger sponsorship;
* valid pseudonymous applicant;
* valid capacity debit;
* replay equality.

### 27.2 Invalid sponsor cases

* ineligible sponsor;
* suspended sponsor;
* AI sponsor;
* system sponsor;
* self-sponsor;
* zero capacity.

### 27.3 Invalid applicant cases

* duplicate identity;
* malformed key;
* wrong key owner;
* invalid proof;
* duplicate key reference.

### 27.4 Capacity cases

* exact debit;
* idempotent retry;
* double spend;
* concurrent last-unit admissions;
* rollover boundary;
* no same-cycle recursive capacity.

### 27.5 Eligibility-separation cases

Prove creation does not grant:

* writing;
* challenges;
* voting;
* governance;
* Tempo;
* inviting;
* economic authority.

### 27.6 Privacy cases

Prove canonical admission does not require:

* legal identity;
* location;
* private relay history;
* product account.

---

## 28. Required canonical and public read surfaces

Define safe public visibility of:

* canonical identity ID;
* creation position;
* creation profile;
* sponsor authorship;
* initial public-key reference;
* identity state;
* writer eligibility status;
* inviter eligibility status;
* current invitation-capacity summary where public;
* suspension status;
* provenance classification.

Exclude:

* private keys;
* private contact details;
* request transport metadata;
* legal identity documents;
* private account/session IDs.

---

## 29. Implementation sequencing

### 29.1 Existing-identity key lifecycle

Implement rotation and revocation first.

### 29.2 Verification and inviter eligibility

Implement replay-derived verification, writer eligibility, inviter eligibility, and invitation capacity.

### 29.3 Identity admission

Implement `identity_create` after the minimum inviter-capacity substrate exists.

### 29.4 Future transports and interfaces

Implement public request pools and relay UI later without making them canonical authority.

---

## 30. Non-goals

Explicitly list:

* no open self-registration in Profile v0;
* no mandatory KYC;
* no mandatory biometrics;
* no proof-of-reasoning admission;
* no proof-of-work as proof of humanity;
* no AI-determined canonical suspensions;
* no verification-weighted vote power;
* no hidden operator inviter grants;
* no private account authority;
* no identity-history pruning;
* no full key recovery mechanism.

---

## 31. Open questions and deferred parameters

Keep this section small and parameter-focused.

Possible remaining questions:

* exact invitation-capacity debit representation;
* exact sponsor maturation threshold;
* exact rollover cap;
* whether capacity generation is integer-only;
* exact certification boundary used for capacity generation;
* exact public visibility of remaining capacity;
* minimum evidence classes before inviter eligibility;
* suspension and appeal event families.

Every unresolved question should identify the later specification responsible for resolving it.

---

## 32. Cross-document reconciliation checklist

List required updates to:

* Protocol v5
* Appendix A
* Signature Profile v0
* Verification Specification
* Cycle Specification
* Privacy Specification
* Offline/MindSeed Specification
* Replay and Merge Specification
* Node and Conformance Specification
* Event Registry
* API Contract
* Cross-Document Invariants
* Authoritative Index
* Implementation Status

---

## 33. Summary of Profile-v0 guarantees

End with a compact normative summary:

* anyone may prepare an identity;
* canonical admission requires distributed scarce sponsorship;
* sponsorship may be offered to strangers;
* all sufficiently verified humans can eventually become sponsors;
* admission creates identity and key control only;
* meaningful authority requires later verification;
* invitation capacity is cycle-bound and non-transferable;
* no central authority controls admission;
* no unrestricted self-registration exists in Profile v0;
* future decentralized admission paths remain possible.
