# TEMPO-005D-A1 Identity Admission Documentation Audit

Task ID: TEMPO-005D-A1  
Track: Canonical Tempo and Stage 1  
Repository: `A:\the-seed-in-my-mind-open-core`  
Audit date: 2026-07-11  
Status: read-only protocol/documentation audit, with this report as the only substantive artifact.

## 1. Executive findings

The current authoritative documents clearly separate identity, key control, human verification, and canonical-writer eligibility in several places, but they do not yet define a deterministic end-to-end admission path for creating a new canonical identity.

Clear current authority:

- `identity_create` exists as the Appendix A event that creates a canonical identity, registers an initial Profile-v0 key descriptor when the reference validates, and leaves the identity inactive for canonical authorship until verified. Classification: NORMATIVE. Evidence: `docs/protocol v5-appendix-a.md:951-966`.
- Profile-v0 key descriptors, `public_key_ref`, key activation, revocation, non-retroactive historical validity, and private-account independence are defined. Classification: NORMATIVE. Evidence: `docs/canonical-event-authorship-and-signature-profile-v0.md:247-353`.
- Ordinary canonical writes require Profile-v0 authorship and event-family eligibility; current ordinary writes are verification-gated through the ordinary canonical-writer gate. Classification: NORMATIVE. Evidence: `docs/protocol v5.md:197-210`, `docs/verification-spec.md:132-146`.
- Verification levels may gate ordinary writer eligibility, Tempo contributor eligibility, beacon diversity eligibility, invite rate, personal mana maximum, voter-pool eligibility, and onboarding probation relief, but may not weight truth, governance votes, rankings, Tempo influence, or payouts. Classification: NORMATIVE. Evidence: `docs/verification-spec.md:820-845`.
- Pseudonymous and anonymous-but-verified participation are first-class; civil identity and VI are not required for canonical eligibility when VH is sufficient. Classification: NORMATIVE. Evidence: `docs/privacy-and-high-risk-submission-spec.md:346-407`, `docs/privacy-and-high-risk-submission-spec.md:700-719`.

Unsettled or contradictory current authority:

- The current documents do not settle who authors or authorizes `identity_create`. Appendix A defines payload and effects but not author/authorization; Verification says an identity may exist only after invitation; older Protocol v5 prose says a new user registers and emits a USER identity idea. Classification: CONTRADICTORY.
- Invite-only status is not unambiguous. Verification states invite-only onboarding, while Appendix A does not include an inviter, sponsor, invitation reference, invite capacity, or rulebook admission fields beyond `verification_reference`; older Protocol v5 registration prose reads as a self-registration precursor. Classification: CONTRADICTORY.
- The documents do not define a public non-canonical admission-request path, stranger sponsorship queue, relay-assisted admission request, lottery, quota, or open self-registration path. Classification: UNSPECIFIED.
- Invitation capacity is recognized as a gated output (`invite_rate`) and lineage taint can reduce future invite rates, but per-cycle allowance, accumulation, transferability, saleability, per-inviter caps, global caps, and formulas are not defined. Classification: PARTLY NORMATIVE, mostly UNSPECIFIED.
- The current privacy/offline/high-risk documents support pseudonymous participation, relays, store-and-forward delivery, and anonymous outer-layer content, but do not define how an unconnected high-risk applicant obtains canonical identity creation without an existing sponsor. Classification: UNSPECIFIED.

Runtime conclusion for parent task: TEMPO-005D remains blocked for identity creation. Key lifecycle details are partly implementable for already-existing identities, but canonical identity admission cannot be safely implemented until author, authorization, invitation/reference, and bootstrap-signature semantics are specified.

## 2. Authority map

| Document | Authority level for this audit | Relevant sections and evidence |
| --- | --- | --- |
| `docs/authoritative-index.md` | Authority index and precedence | Identifies authoritative specs and gives authorship/signature profile precedence for authored candidates, signed bytes, `public_key_ref`, and replay-derived key state. |
| `docs/protocol v5.md` | Constitutional and semantic root | Section 0 requires human-first authorship and verification-gated ordinary writes (`docs/protocol v5.md:190-210`). Section 8.4 contains older identity-registration prose (`docs/protocol v5.md:3185-3191`). Section 8.6 and Section 13.4/13.5 defer signed candidates and key rules to Profile-v0 (`docs/protocol v5.md:3201-3207`, `docs/protocol v5.md:4665-4748`). |
| `docs/protocol v5-appendix-a.md` | Authoritative canonical event catalog and payload schemas | Appendix A states no canonical state transition may occur except through listed events and aliases are non-authoritative (`docs/protocol v5-appendix-a.md:930-947`). It defines `identity_create`, `identity_verification_update`, visibility, rotation, and revocation (`docs/protocol v5-appendix-a.md:951-1019`). |
| `docs/canonical-event-authorship-and-signature-profile-v0.md` | Authoritative for signed candidate, Profile-v0 key descriptor, key refs, and key-state validation | Defines `public_key_ref`, descriptor bytes, private-account exclusion, initial registration data, rotation, revocation, active-key state, and authorship validity (`docs/canonical-event-authorship-and-signature-profile-v0.md:247-353`). |
| `docs/verification-spec.md` | Authoritative for verification and eligibility boundaries | Defines writer gate, Sybil friction, invite-only onboarding, lineage taint, VL-gated outputs including `invite_rate`, and privacy/offline verification behavior (`docs/verification-spec.md:132-146`, `docs/verification-spec.md:177-187`, `docs/verification-spec.md:552-590`, `docs/verification-spec.md:820-845`). |
| `docs/privacy-and-high-risk-submission-spec.md` | Authoritative for pseudonymity, high-risk submission, and outer-layer/canonical boundary | Defines pseudonymous, anonymous-but-verified, outer-layer non-canonical content, relays, delayed publication, and equivalence of pseudonymous verified identities (`docs/privacy-and-high-risk-submission-spec.md:346-407`, `docs/privacy-and-high-risk-submission-spec.md:410-428`, `docs/privacy-and-high-risk-submission-spec.md:520-563`, `docs/privacy-and-high-risk-submission-spec.md:700-719`). |
| `docs/offline-and-mindseed-spec.md` | Authoritative for offline publication and reintegration constraints | Defines delayed publication, local drafts, offline identity continuity claims, pseudonymous offline authorship, canonical publication requirements, and per-identity pacing on reintegration (`docs/offline-and-mindseed-spec.md:50-108`, `docs/offline-and-mindseed-spec.md:420-523`, `docs/offline-and-mindseed-spec.md:1112-1131`, `docs/offline-and-mindseed-spec.md:1411-1423`). |
| `docs/deterministic-replay-and-merge-spec.md` | Authoritative for replay validation | Requires deterministic validation, Profile-v0 signature verification, author identity existence, active rulebook verification requirements, and human-first authorship invariants (`docs/deterministic-replay-and-merge-spec.md:361-385`, `docs/deterministic-replay-and-merge-spec.md:410-425`). |
| `docs/protocol-event-registry.v1.md` | Derived registry, useful for drift and implementation status | Classifies identity events and notes runtime support is not implemented for identity/key lifecycle; classifies writer eligibility as derived/bootstrap, not a Protocol v5 public event (`docs/protocol-event-registry.v1.md:140-156`, `docs/protocol-event-registry.v1.md:218-222`, `docs/protocol-event-registry.v1.md:278-288`). |
| `docs/api-contract-read-only.md` | API/status contract | Summarizes signed candidate route and says current identity keys/writer eligibility are bootstrap/operator/test provisioned until public identity/key lifecycle and writer lifecycle are implemented (`docs/api-contract-read-only.md:56-58`, `docs/api-contract-read-only.md:842-853`). |
| `docs/cross-doc-invariants.md` | Cross-document invariant index | Repeats public-read and canonical-writer gate invariant for ordinary writes. Evidence found at `docs/cross-doc-invariants.md:251`. |

## 3. Current identity model

The current documents distinguish these concepts:

- Local keypair or local identity material: local/offline capability, not canonical authority by itself. Offline identity vaults may store secrets and continuity/recovery claims, but those records are claims and do not confer legitimacy. Classification: NORMATIVE. Evidence: `docs/offline-and-mindseed-spec.md:445-480`.
- Authored candidate: signed local/pre-publication event bytes; not canonical merely because the signature is valid. Classification: NORMATIVE. Evidence: `docs/offline-and-mindseed-spec.md:1123-1131`.
- Canonical identity object: created by `identity_create`. Classification: NORMATIVE. Evidence: `docs/protocol v5-appendix-a.md:953-966`.
- Active controlling keyset: replay-derived signing authority for the identity; keys are not the identity. Classification: NORMATIVE. Evidence: `docs/verification-spec.md:1157-1169`.
- Verified-human identity: derived verification state, distinct from key control. Classification: NORMATIVE. Evidence: `docs/verification-spec.md:132-146`, `docs/privacy-and-high-risk-submission-spec.md:700-719`.
- Canonical-writer eligibility: event-family gate derived from verification state and current deployment profile; distinct from read access and key control. Classification: NORMATIVE. Evidence: `docs/protocol v5.md:197-210`, `docs/verification-spec.md:132-146`.
- Challenge/vote/governance eligibility: gated separately and not granted by Tempo contributor status or key possession. Classification: NORMATIVE. Evidence: `docs/protocol v5.md:207-210`, `docs/verification-spec.md:828-845`.
- Tempo participation: separate `tempo_contributor` lane that does not grant arbitrary writing, challenge, voting, governance, POD, POINT, or token authority. Classification: NORMATIVE. Evidence: `docs/verification-spec.md:132-146`.
- AI/system identities: AI provenance may be metadata but cannot satisfy human authorship, verified-human eligibility, signature authority, or writer eligibility; `system_boundary_emitter` is a separate constrained exception. Classification: NORMATIVE. Evidence: `docs/canonical-event-authorship-and-signature-profile-v0.md:351-353`, `docs/protocol v5.md:190-193`.

Conflations or drift:

- Protocol v5 Section 8.4 says identity creation begins when a new user registers and emits a USER identity idea plus structural-role ideas. Appendix A now says `identity_create` creates the identity object and older aliases are non-authoritative. Classification: CONTRADICTORY. Evidence: `docs/protocol v5.md:3185-3191`, `docs/protocol v5-appendix-a.md:930-966`.
- Verification Section 7.1 says an identity may exist only after invitation, but Appendix A does not define inviter, sponsor, invite reference, or invite capacity fields. Classification: CONTRADICTORY/UNSPECIFIED. Evidence: `docs/verification-spec.md:552-557`, `docs/protocol v5-appendix-a.md:953-966`.

## 4. Canonical identity creation

Current documented mechanism:

- Appendix A defines `identity_create` as the canonical identity creation event. It requires `identity_id`, `initial_public_key_ref`, `initial_public_key_descriptor`, and `verification_reference`. It creates a new identity object, registers initial key state when the reference equals the descriptor hash, and leaves the identity inactive for canonical authorship until verified. Classification: NORMATIVE. Evidence: `docs/protocol v5-appendix-a.md:951-966`.
- Appendix A also defines `identity_verification_update`, whose effect is to update verification and enable or disable canonical authorship eligibility. Classification: NORMATIVE. Evidence: `docs/protocol v5-appendix-a.md:970-982`.
- The Profile-v0 spec requires enough canonical data to reconstruct the initial key descriptor and states that absent or unreconstructable key material cannot authorize new Profile-v0 writes. Classification: NORMATIVE. Evidence: `docs/canonical-event-authorship-and-signature-profile-v0.md:291-303`.

Current gaps:

- Author: no authoritative passage states whether `identity_create` is authored by the target identity, an inviter, a verifier, a bootstrap authority, a governance/rulebook actor, or a node. Classification: UNSPECIFIED.
- Authorization: no authoritative passage states what makes an `identity_create` authorized beyond the payload/effect and general event-family eligibility rules. Classification: UNSPECIFIED.
- Invitation reference: the `identity_create` payload has `verification_reference`, not a clearly defined `invitation_id`, `sponsor_identity_id`, inviter signature, invite capacity debit, or public admission request reference. Classification: UNSPECIFIED.
- Bootstrap signature: if the target self-authors, the initial key cannot already be active before the event; if another identity authors, the payload must bind the target key. The current docs do not choose the model. Classification: UNSPECIFIED.
- Private accounts: current authority excludes private account/session tables from canonical signature validation. Classification: NORMATIVE. Evidence: `docs/canonical-event-authorship-and-signature-profile-v0.md:285-290`.

## 5. Invitations

Strongest invite-only evidence:

- Verification Section 7.1 states: "Invites are scarce and create accountability chains. An identity may exist only after being invited by another agent willing to say so, under the applicable rulebooks." Classification: NORMATIVE in Verification. Evidence: `docs/verification-spec.md:552-557`.
- Verification Section 2.3 says Sybil resistance relies on social/economic friction including limited invites and taint propagation. Classification: NORMATIVE. Evidence: `docs/verification-spec.md:177-187`.
- Verification levels may gate `invite_rate`, and lineage taint can reduce future invite rates. Classification: NORMATIVE for the existence of an invite-rate output, not for formulas. Evidence: `docs/verification-spec.md:820-845`, `docs/verification-spec.md:1087-1094`, `docs/verification-spec.md:1143-1147`.

Strongest evidence against a complete invite-only implementation:

- Appendix A `identity_create` does not include inviter identity, invitation token, sponsor reference, invite capacity debit, cycle index, or admission-request reference. Classification: UNSPECIFIED. Evidence: `docs/protocol v5-appendix-a.md:953-966`.
- The registry describes `identity_create` allowed actor kind as "Human identity under bootstrap/rulebook rules", not a settled inviter class. Classification: DERIVED registry status. Evidence: `docs/protocol-event-registry.v1.md:140-146`.
- Older Protocol v5 identity lifecycle prose still says a new user registers and emits a USER identity idea. Classification: CONTRADICTORY/stale. Evidence: `docs/protocol v5.md:3185-3191`.

Who may invite:

- The current docs imply another "agent" under applicable rulebooks, and verification levels may gate `invite_rate`. They do not define whether every sufficiently verified human eventually receives invite capacity, what tier grants it, whether governance can suspend it, or whether invite capacity is cycle-limited. Classification: UNSPECIFIED beyond the existence of invite scarcity and `invite_rate`.

## 6. Self-registration

No current authoritative document defines a complete self-registration path.

Evidence possibly pointing toward self-registration:

- Protocol v5 Section 8.4 says identity creation begins when a new user registers and emits identity/structural-role ideas. Classification: CONTRADICTORY older prose. Evidence: `docs/protocol v5.md:3185-3191`.
- Offline and privacy specs allow local drafts, local pre-publication events, anonymous/pseudonymous outer-layer content, and delayed publication. Classification: NORMATIVE for local/non-canonical material. Evidence: `docs/offline-and-mindseed-spec.md:50-108`, `docs/offline-and-mindseed-spec.md:499-523`, `docs/privacy-and-high-risk-submission-spec.md:410-428`.

Evidence limiting self-registration:

- Offline/local events become canonical only if published and valid under canonical rules. Classification: NORMATIVE. Evidence: `docs/protocol-event-registry.v1.md:278-288`, `docs/offline-and-mindseed-spec.md:1112-1131`.
- Ordinary human-authored canonical candidates need a valid Profile-v0 signature from an eligible verified-human identity at canonical publication, and anonymous material remains outer layer until adopted by a verified-human identity. Classification: NORMATIVE. Evidence: `docs/offline-and-mindseed-spec.md:499-504`.
- Verification Section 7.1 says identity existence requires invitation. Classification: NORMATIVE but not fully connected to Appendix A. Evidence: `docs/verification-spec.md:552-557`.

Classification of possible self-registration paths:

- Self-authored `identity_create`: contradicted or unspecified. There is stale older Protocol prose, but no Profile-v0 bootstrap exception or Appendix A author rule.
- Open canonical registration without inviter: absent in current authority.
- Low-authority provisional canonical identity: absent in current authority.
- Non-canonical admission request: absent as a defined protocol object; outer-layer content can exist but no admission queue semantics are defined.
- Relay-assisted or offline registration: local/offline transport is defined, but canonical admission semantics are not.

## 7. Verification and participation rates

Current verification effects:

- Verification gates ordinary canonical writer eligibility, Tempo contributor eligibility, beacon diversity/qualification eligibility, invite rate, personal mana maximum, voter-pool eligibility, and onboarding probation relief. Classification: NORMATIVE. Evidence: `docs/verification-spec.md:820-845`.
- Verification must not weight importance, governance votes, Tempo claim/evidence influence, truth outside verification, POD, or POINT. Classification: NORMATIVE. Evidence: `docs/verification-spec.md:838-845`, `docs/verification-spec.md:1007-1012`.
- Current deployment profile uses `canonical_writer_level` issued through the Seed verifier role for ordinary writer eligibility. Classification: NORMATIVE current deployment profile, transitional implementation status. Evidence: `docs/verification-spec.md:142-146`.
- Current API documentation states key and writer rows are bootstrap/operator/test-provisioned until public identity/key and writer lifecycle are implemented. Classification: DOCUMENTED TRANSITIONAL STATUS. Evidence: `docs/api-contract-read-only.md:842-853`.

Unspecified verification/rate details:

- Exact thresholds for invite authority.
- Exact per-cycle invite allowance.
- Accumulation, rollover, maximum stored capacity, transferability, delegation, and saleability.
- Challenge-rate increases and write-rate formulas by verification level.
- Whether invite authority is available to every sufficiently verified human.
- Whether low-authority self-registered identities get any rate-limited rights before verification.

## 8. Sybil and bot resistance

Current protections and limits:

| Mechanism | Attack addressed | Current status | Limitation |
| --- | --- | --- | --- |
| Human-first authorship and rejection of AI/system authors for ordinary writes | AI or system identities authoring canonical human events | NORMATIVE. `docs/protocol v5.md:190-193`; `docs/canonical-event-authorship-and-signature-profile-v0.md:351-353` | Does not by itself limit creation of many human-claimed identities. |
| Profile-v0 signatures and replay-derived key state | forged authorship and key substitution | NORMATIVE. `docs/canonical-event-authorship-and-signature-profile-v0.md:247-353` | Key control is not proof of humanity. |
| Ordinary canonical-writer verification gate | unverified identities writing ordinary canonical content | NORMATIVE. `docs/protocol v5.md:197-210`; `docs/verification-spec.md:132-146` | Does not define pre-identity admission. |
| Limited invites | mass canonical identity creation | NORMATIVE phrase in Verification. `docs/verification-spec.md:177-187`, `docs/verification-spec.md:552-557` | No payload, formula, cycle cap, inviter class, or enforcement mechanics. |
| Diverse human counterparties and diminishing returns | collusive or single-cluster verification | NORMATIVE concept. `docs/verification-spec.md:177-187`; `docs/verification-spec.md:1105-1111` | Exact scoring and thresholds remain unspecified in current passages. |
| Lineage taint and quarantine | repeated invitation of fraudulent identities | NORMATIVE concept. `docs/verification-spec.md:558-574`, `docs/verification-spec.md:1087-1094`, `docs/verification-spec.md:1143-1147` | Forward-looking derived effects only; does not state invitee misconduct liability boundaries in detail. |
| Per-identity pacing on offline reintegration | offline stockpiling and rate-limit bypass | NORMATIVE. `docs/offline-and-mindseed-spec.md:420-441`, `docs/offline-and-mindseed-spec.md:1411-1423` | Per-identity pacing can be multiplied if identity creation is unconstrained. |
| Pseudonymous but verified authorship | legal identity exposure as a participation barrier | NORMATIVE. `docs/privacy-and-high-risk-submission-spec.md:346-407`, `docs/privacy-and-high-risk-submission-spec.md:700-719` | Pseudonymity protects admitted participants; it does not define admission for unconnected people. |

The documents have substantial Sybil-resistance intent after identity creation and verification, but they do not yet define deterministic Sybil resistance before canonical identity creation.

## 9. Access without trusted social connections

Supported:

- Civil identity is not required for canonical participation; default mode is pseudonymous. Classification: NORMATIVE. Evidence: `docs/privacy-and-high-risk-submission-spec.md:352-366`.
- Anonymous-but-verified identities are fully legitimate canonical authors and must not be downgraded due to absence of VI. Classification: NORMATIVE. Evidence: `docs/privacy-and-high-risk-submission-spec.md:393-407`.
- Indirect relays, delayed/store-and-forward delivery, and offline publication are supported. Classification: NORMATIVE. Evidence: `docs/privacy-and-high-risk-submission-spec.md:520-563`, `docs/offline-and-mindseed-spec.md:1112-1131`.
- Offline users may create local pre-publication events anonymously or pseudonymously, but canonical publication requires adoption by a verified-human identity. Classification: NORMATIVE. Evidence: `docs/offline-and-mindseed-spec.md:499-523`.

Not currently defined:

- A stranger sponsorship pool or public request queue.
- A privacy-preserving admission request that can be considered by existing participants without revealing civil identity.
- A relay-assisted identity admission path for people with no social connection.
- A globally bounded self-registration mechanism.
- A non-canonical request transport with canonical eventual sponsorship.
- Anti-gatekeeping guarantees that every sufficiently verified human eventually receives invitation capacity.

Assessment: current documents protect pseudonymous participation after admission and support indirect transport, but do not provide a realistic documented admission path for a person with no personal connection to an existing participant, no legal-identity documentation, and a need for pseudonymity.

## 10. Canonical-storage implications

Current rules:

- Local drafts and outer-layer content are not canonical records. Classification: NORMATIVE. Evidence: `docs/offline-and-mindseed-spec.md:98-108`, `docs/privacy-and-high-risk-submission-spec.md:410-428`.
- A valid signature alone does not make a candidate canonical; canonical status occurs only after publication/finality. Classification: NORMATIVE. Evidence: `docs/offline-and-mindseed-spec.md:1123-1131`.
- Rejected or invalid events must not affect canonical replay. Classification: DERIVED from deterministic validation. Evidence: `docs/deterministic-replay-and-merge-spec.md:361-385`.

Unspecified:

- Whether failed registration attempts are stored anywhere.
- Whether pending identity requests are canonical, non-canonical, private, relay-carried, or absent.
- Whether provisional identities can occupy canonical storage without writer authority.
- Global identity-creation storage quotas.
- Canonical handling of abandoned identity candidates.

Storage risk: an open canonical self-registration model would create a permanent canonical-storage pressure point unless a future spec defines bounded admission, non-canonical request handling, or another deterministic scarcity mechanism.

## 11. Cycle and rulebook integration

Current cycle/rulebook touchpoints:

- Verification certainty and eligibility updates activate only at defined boundaries and must be deterministic. Classification: NORMATIVE. Evidence: `docs/verification-spec.md:791-800`, `docs/verification-spec.md:1000-1005`.
- Offline reintegration enforces canonical cycles, per-identity pacing, and rate limits when offline events are published. Classification: NORMATIVE. Evidence: `docs/offline-and-mindseed-spec.md:420-441`, `docs/offline-and-mindseed-spec.md:1411-1423`.
- Identity rulebooks govern identity creation, longevity, key rotation, delegation limits, death semantics, and successor identities, and must stay compatible with human-first authorship. Classification: NORMATIVE direction but incomplete mechanics. Evidence: `docs/protocol v5.md:3623`.

Not specified:

- Per-cycle identity creation quota.
- Per-cycle invitation allowance formula.
- Carryover/rollover/max invite capacity.
- Whether Dmin/Dmax or cycle closure affects identity creation capacity.
- Eligibility snapshots for invitation authority.
- How active rulebook changes bind an already-issued invitation.

## 12. Contradictions

1. Identity creation mechanism:
   - Rule A: Appendix A says `identity_create` creates a new identity object and registers initial key state. Evidence: `docs/protocol v5-appendix-a.md:953-966`.
   - Rule B: Protocol v5 Section 8.4 says identity creation begins when a new user registers and emits a USER identity idea plus structural-role ideas atomically. Evidence: `docs/protocol v5.md:3185-3191`.
   - Current authority resolution: Appendix A is the canonical event catalog, and older aliases are non-authoritative, but Section 8.4 has not been harmonized. Owner/spec reconciliation required.

2. Invite-only identity existence versus Appendix A schema:
   - Rule A: Verification says an identity may exist only after invitation under rulebooks. Evidence: `docs/verification-spec.md:552-557`.
   - Rule B: Appendix A `identity_create` lacks inviter, sponsor, invitation, capacity, and admission-request fields. Evidence: `docs/protocol v5-appendix-a.md:953-966`.
   - Current authority resolution: not resolved. Appendix A cannot deterministically enforce invite-only semantics as written.

3. Verified-human authorship invariant versus identity bootstrap:
   - Rule A: Replay invariant says all canonical events must be authored by exactly one verified human identity, and author identity must exist and meet verification requirements at application. Evidence: `docs/deterministic-replay-and-merge-spec.md:371-380`, `docs/deterministic-replay-and-merge-spec.md:410-418`.
   - Rule B: `identity_create` creates an identity that is inactive until verified; initial key registration is bound to identity creation. Evidence: `docs/protocol v5-appendix-a.md:953-966`, `docs/canonical-event-authorship-and-signature-profile-v0.md:291-303`.
   - Current authority resolution: not resolved. A self-authored identity-create bootstrap exception, inviter-authored model, or bootstrap publisher model is needed.

4. Pseudonymous/high-risk access versus no admission path:
   - Rule A: pseudonymous and anonymous-but-verified participation must be legitimate. Evidence: `docs/privacy-and-high-risk-submission-spec.md:346-407`, `docs/privacy-and-high-risk-submission-spec.md:700-719`.
   - Rule B: invite-only onboarding is stated, but public/stranger/privacy-preserving admission requests are not defined. Evidence: `docs/verification-spec.md:552-557`.
   - Current authority resolution: not resolved. Pseudonymous participation is specified after admission, not admission itself.

5. Per-identity pacing versus unconstrained identity creation:
   - Rule A: offline and canonical pacing restrict per-identity throughput. Evidence: `docs/offline-and-mindseed-spec.md:420-441`.
   - Rule B: identity creation limits are only described as limited invites, with no deterministic quota/formula. Evidence: `docs/verification-spec.md:177-187`, `docs/verification-spec.md:552-557`.
   - Current authority resolution: not resolved. Per-identity rate limits do not alone prevent multiplication through many identities.

## 13. Evidence matrix

| Question | Current answer | Classification | Highest authority | Supporting citations | Contradictions | Gap |
| --- | --- | --- | --- | --- | --- | --- |
| Who authors `identity_create`? | Not settled. | UNSPECIFIED | Appendix A/Authorship profile | `docs/protocol v5-appendix-a.md:953-966`; `docs/canonical-event-authorship-and-signature-profile-v0.md:339-349` | Verified-human authorship invariant conflicts with identity bootstrap if target self-authors. | Author and authorization model. |
| Is canonical identity creation invite-only? | Verification says yes, but event schema does not encode it. | CONTRADICTORY | Verification plus Appendix A | `docs/verification-spec.md:552-557`; `docs/protocol v5-appendix-a.md:953-966` | Older Protocol registration prose and missing Appendix fields. | Invitation mechanics. |
| Can the target self-register? | No complete current path. | CONTRADICTORY/UNSPECIFIED | Protocol/Appendix/Authorship | `docs/protocol v5.md:3185-3191`; `docs/protocol v5-appendix-a.md:953-966` | Verification invite-only language. | Bootstrap signature exception or prohibition. |
| Can anyone create a local keypair? | Local key material and identity vaults are permitted locally; they are not canonical authority. | NORMATIVE | Offline/Authorship | `docs/offline-and-mindseed-spec.md:445-480`; `docs/canonical-event-authorship-and-signature-profile-v0.md:291-303` | None material. | None for local-only key generation. |
| Is there a non-canonical admission request? | Not defined. | UNSPECIFIED | Privacy/Offline | `docs/privacy-and-high-risk-submission-spec.md:410-428`; `docs/offline-and-mindseed-spec.md:98-108` | Outer-layer content exists but admission request semantics do not. | Request format, routing, sponsorship. |
| Who can invite? | "Another agent" under rulebooks; exact class not defined. | UNSPECIFIED | Verification | `docs/verification-spec.md:552-557`; `docs/verification-spec.md:820-845` | None decisive. | Inviter threshold and good-standing rule. |
| Can every sufficiently verified human become an inviter? | Not documented. | UNSPECIFIED | Verification | `docs/verification-spec.md:820-845` | Owner concern not answered. | Path from VL to invite capacity. |
| Is invite capacity cycle-limited? | Not defined. | UNSPECIFIED | Verification/Offline | `docs/verification-spec.md:820-845`; `docs/offline-and-mindseed-spec.md:420-441` | Per-identity pacing exists, but not invite-specific. | Formula and cycle binding. |
| Does invite capacity accumulate? | Not defined. | UNSPECIFIED | Verification | `docs/verification-spec.md:820-845` | None. | Rollover/max capacity. |
| Is invite capacity transferable? | Not defined. | UNSPECIFIED | Verification | `docs/verification-spec.md:820-845` | None. | Transfer/delegation/sale rules. |
| Does identity creation grant writing authority? | No; Appendix A says identity inactive until verified. | NORMATIVE | Appendix A | `docs/protocol v5-appendix-a.md:963-966` | Older Protocol wording permits unverified identity but not universal challenges/voting; not ordinary writer clarity. | Exact initial read/write status across event families. |
| Does verification increase write limits? | Verification gates writer eligibility; exact rate increases not defined. | PARTIAL | Verification | `docs/verification-spec.md:132-146`; `docs/verification-spec.md:820-845` | None. | Rate formulas. |
| Does verification increase challenge limits? | Challenge eligibility is separate and verification-gated; exact challenge-rate increases not defined. | PARTIAL | Protocol/Verification | `docs/protocol v5.md:207-210`; `docs/verification-spec.md:820-845` | None. | Challenge thresholds/rates. |
| Does verification grant invitation authority? | Verification levels may gate `invite_rate`; exact threshold not defined. | PARTIAL | Verification | `docs/verification-spec.md:828-836` | None. | Invite threshold and capacity. |
| Is self-registration defined? | No complete current path. | UNSPECIFIED/CONTRADICTORY | Appendix A/Protocol | `docs/protocol v5.md:3185-3191`; `docs/protocol v5-appendix-a.md:953-966` | Invite-only language. | Decide allowed/prohibited and mechanics. |
| Can provisional identities exist canonically? | Not defined. `identity_create` creates inactive identities, but not a provisional admission class. | UNSPECIFIED | Appendix A | `docs/protocol v5-appendix-a.md:953-966` | None. | Provisional status, effects, storage. |
| Are pending requests canonical? | Not defined; outer-layer/drafts are non-canonical. | UNSPECIFIED | Privacy/Offline | `docs/privacy-and-high-risk-submission-spec.md:410-428`; `docs/offline-and-mindseed-spec.md:98-108` | None. | Request lifecycle. |
| What prevents mass bot registration? | Intended limited invites, verification friction, key signatures, and lineage taint; deterministic pre-admission mechanics missing. | PARTIAL | Verification | `docs/verification-spec.md:177-187`; `docs/verification-spec.md:552-574` | Missing Appendix enforcement. | Admission scarcity. |
| What prevents per-identity limits from multiplying through Sybils? | Limited invites and verification are intended; exact enforcement missing. | PARTIAL | Verification/Offline | `docs/verification-spec.md:177-187`; `docs/offline-and-mindseed-spec.md:420-441` | Identity creation limits missing. | Pre-identity quotas. |
| How can an unconnected person gain admission? | Not documented. | UNSPECIFIED | Verification/Privacy | `docs/verification-spec.md:552-557`; `docs/privacy-and-high-risk-submission-spec.md:520-563` | Pseudonymous transport does not define sponsorship. | Stranger admission path. |
| How can a high-risk pseudonymous person gain admission? | Pseudonymous/relay participation after admission is supported; admission path itself is not. | PARTIAL | Privacy/Offline | `docs/privacy-and-high-risk-submission-spec.md:346-407`; `docs/privacy-and-high-risk-submission-spec.md:520-563` | Invite-only social dependency unresolved. | Privacy-preserving sponsorship/request. |
| Can AI/system identities invite or register humans? | AI cannot satisfy human authorship/eligibility; system emitter is separate. | NORMATIVE rejection for ordinary paths | Protocol/Authorship | `docs/protocol v5.md:190-193`; `docs/canonical-event-authorship-and-signature-profile-v0.md:351-353` | Identity-create allowed actor not fully settled. | Explicit admission-event actor exclusion. |
| Are inviters liable for invitee behavior? | Lineage taint affects future invite rates/eligibility pools/probation/quarantine forward only; details limited. | PARTIAL | Verification | `docs/verification-spec.md:558-574`; `docs/verification-spec.md:1087-1094`; `docs/verification-spec.md:1143-1147` | No sale/transfer/coercion detail. | Liability boundaries. |
| Are identity creation attempts rate-limited globally? | Not documented. | UNSPECIFIED | Verification | `docs/verification-spec.md:177-187` | None. | Global cap/quota/queue. |
| Are failed registration attempts canonical? | Not defined; invalid candidates generally do not affect replay. | UNSPECIFIED/DERIVED | Replay/Privacy | `docs/deterministic-replay-and-merge-spec.md:361-385`; `docs/privacy-and-high-risk-submission-spec.md:410-428` | None. | Rejected/pending attempt retention. |

## 14. Candidate-model comparison

This section compares possible models against current documents without choosing one.

### Model A - Strict private invitation

An applicant must personally obtain an invitation from an existing eligible participant.

- Support in current documents: strongest support from invite-only onboarding and scarce invites. Evidence: `docs/verification-spec.md:552-557`.
- Contradictions: Appendix A lacks inviter/invitation fields; older Protocol v5 registration prose does not say private invitation. Evidence: `docs/protocol v5-appendix-a.md:953-966`, `docs/protocol v5.md:3185-3191`.
- Missing rules: inviter eligibility threshold, per-cycle capacity, invite artifact, privacy protections, liability, recovery from social gatekeeping.
- Sybil risks: strong if invite scarcity is implemented; currently undefined.
- Access/gatekeeping risks: high for unconnected or high-risk users if no stranger sponsorship exists.
- Canonical-storage implications: low if only invited `identity_create` events become canonical.
- Compatibility with Profile-v0 lifecycle: possible if inviter authors `identity_create` and binds target key descriptor; not currently specified.

### Model B - Sponsored public admission

Anyone may create a local identity request; an eligible existing participant sponsors canonical identity creation, including strangers through a public or privacy-preserving request mechanism.

- Support in current documents: compatible with invite-only onboarding, relays, pseudonymous/outer-layer content, and offline local drafts. Evidence: `docs/verification-spec.md:552-557`; `docs/privacy-and-high-risk-submission-spec.md:410-428`, `docs/privacy-and-high-risk-submission-spec.md:520-563`.
- Contradictions: no explicit public admission request object or sponsor semantics exist.
- Missing rules: request transport, sponsor selection, privacy, spam protection, sponsor capacity debit, canonical/non-canonical request boundary.
- Sybil risks: depends on sponsor capacity and request flood controls; not defined.
- Access/gatekeeping risks: lower than strict private invitation if stranger sponsorship is available, but still undefined.
- Canonical-storage implications: manageable if requests are non-canonical until sponsored; undefined if requests are canonical.
- Compatibility with Profile-v0 lifecycle: likely compatible if sponsor authors `identity_create`; not specified.

### Model C - Open canonical self-registration with zero ordinary authority

Anyone may self-create a canonical identity, but receives no ordinary participation eligibility until verified.

- Support in current documents: Appendix A says `identity_create` creates inactive identity; older Protocol registration prose implies user registration. Evidence: `docs/protocol v5-appendix-a.md:953-966`; `docs/protocol v5.md:3185-3191`.
- Contradictions: Verification says identity may exist only after invitation. Evidence: `docs/verification-spec.md:552-557`.
- Missing rules: self-sign bootstrap exception, canonical storage protection, duplicate/mass registration handling, initial status, verification upgrade.
- Sybil risks: high at storage and identity-count level unless globally bounded.
- Access/gatekeeping risks: low for initial existence, but verification still needed.
- Canonical-storage implications: high unless bounded.
- Compatibility with Profile-v0 lifecycle: requires explicit self-signed identity-create bootstrap rule; not specified.

### Model D - Open canonical self-registration with low initial rate limits

Anyone may self-create a canonical identity and receives small writing or challenge allowances.

- Support in current documents: weak. Current authority says ordinary writes require verification and `identity_create` leaves identity inactive for authorship. Evidence: `docs/protocol v5.md:197-210`; `docs/protocol v5-appendix-a.md:963-966`.
- Contradictions: verification-gated writes, invite-only onboarding, inactive identity until verified.
- Missing rules: all Model C rules plus low-rate formulas, abuse caps, challenge/writer event-family boundaries.
- Sybil risks: very high unless identity creation is scarce, because low per-identity rates multiply.
- Access/gatekeeping risks: low for initial expression, but canonical spam risk high.
- Canonical-storage implications: high.
- Compatibility with Profile-v0 lifecycle: blocked by current verification-gated write policy unless specs change.

### Model E - Globally bounded self-registration

Self-registration is allowed only through a global cycle quota, lottery, queue, proof, or other scarce admission mechanism.

- Support in current documents: compatible with cycle/pacing concepts and desire for deterministic limits, but not currently defined for identity creation. Evidence: `docs/offline-and-mindseed-spec.md:420-441`; `docs/verification-spec.md:177-187`.
- Contradictions: invite-only statement if self-registration has no inviter.
- Missing rules: global quota, queue ordering, lottery source, proof type, privacy, storage of requests, anti-flooding, cycle integration.
- Sybil risks: potentially manageable if scarcity is deterministic; currently absent.
- Access/gatekeeping risks: potentially lower than private invitation; depends on queue/proof design.
- Canonical-storage implications: depends on whether requests are canonical.
- Compatibility with Profile-v0 lifecycle: possible only after defining bootstrap/self-authorship and quota semantics.

## 15. Blocking gaps

### Required before TEMPO-005D identity/key implementation

- `identity_create` author: target self, inviter/sponsor, verifier, bootstrap publisher, or governance/rulebook actor.
- `identity_create` authorization: what state/reference/capacity makes the event valid.
- Initial-key bootstrap signature model: target self-sign exception, inviter-authored payload binding, or prior key-registration process.
- Required admission/reference semantics: whether `verification_reference` is enough, whether an invitation/sponsor/admission reference is required, and how it is validated.
- Initial eligibility state by event family: identity existence, key active state, writer eligibility, verification, challenge, vote, Tempo, governance, invite authority.
- Whether identity creation is invite-only, self-registration, or a combination.

### Required before verification/writer lifecycle implementation

- Verification threshold for ordinary writer eligibility.
- Verification threshold for challenge creation and voting.
- Verification threshold for invitation authority.
- `invite_rate` formula, cycle binding, cap, rollover, decay, suspension, restoration.
- Inviter good-standing and taint effects.
- Event-family-specific eligibility for identity/key management, ordinary writes, challenges, voting, Tempo, governance, and recovery.
- Deterministic handling of centralized lanes as optional complements only.

### Required before open self-registration could be adopted

- Canonical admission class or explicit non-canonical request class.
- Initial authority level.
- Global/per-origin/per-cycle limits.
- Sybil-resistance mechanism before canonical identity creation.
- Storage impact and rejected/pending attempt retention rules.
- Relay/offline request behavior.
- Abuse handling and upgrade path from provisional to ordinary participation.

### Required for access under repression

- Privacy-preserving admission-request transport.
- Stranger sponsorship or non-social admission path.
- Relay support for admission requests, not only signed event submission.
- Anti-gatekeeping guarantees or rulebook mechanisms.
- Privacy leakage rules for sponsors, request queues, and verification artifacts.
- Documentation requirements that avoid civil identity disclosure.
- Censorship/offline behavior for applicants not yet canonical identities.

### Can safely remain future work

- Wallet UX and private key custody.
- Full challenge-based account/key recovery.
- Exact UI for public admission requests.
- Private product account linkage.
- Detailed front-end onboarding workflows.
- Private identity-vault implementation details.

## 16. Questions requiring owner decisions

1. Is canonical `identity_create` strictly invite-authored/sponsor-authored, self-authored under a bootstrap exception, bootstrap-authority authored, or a combination?
2. If invite-based, what canonical fields or references prove the invitation, who may spend invitation capacity, and when is capacity debited?
3. Does every sufficiently verified human eventually receive invitation capacity, and if yes, at which verification threshold and under what good-standing limits?
4. Is invitation capacity per-cycle, accumulated, capped, transferable, delegable, saleable, or strictly personal and nontransferable?
5. Is there a non-canonical public or privacy-preserving admission-request path for strangers and high-risk users?
6. Is open self-registration allowed at all, and if yes, is it canonical with zero authority, canonical with low authority, globally bounded, or non-canonical until sponsored?
7. How are failed, pending, or abandoned admission attempts stored or excluded from canonical storage?
8. What are the deterministic consequences for abusive inviters, invitation rings, sold invitations, coercive verification, and mistaken sponsorship?
9. Does key-management event authorization require only active key control for the identity, or also ordinary canonical-writer eligibility?
10. Does `identity_key_rotate` add an active key, supersede prior keys, or follow a profile-defined keyset policy?

## 17. No-change declaration

No normative specifications were changed.  
No runtime code was changed.  
No migrations were changed.  
No databases were read or modified.  
No generated exports were created.  
No private-repository files were changed.  
No conformance fixtures, event registries, API contracts, or implementation-status files were changed by this audit.

The only substantive repository artifact created by this task is this audit report.

## 18. Readiness

Current identity-create authority fully documented: no  
Invite-only status unambiguous: no  
Path for all sufficiently verified humans to invite documented: no  
Invitation rate limits documented: no  
Self-registration path documented: no  
Non-canonical admission request path documented: no  
Sybil resistance before canonical identity creation documented: no  
Access path for unconnected high-risk users documented: no  
Owner decisions still required: yes  
Safe to begin specification reconciliation: yes  
Safe to resume TEMPO-005D runtime implementation: no

The "no" answers are because current authoritative text defines pieces of identity, key, verification, and privacy behavior but not a deterministic admission state machine. Specification reconciliation is safe as the next step because the contradictions and missing decisions are now isolated; runtime implementation remains blocked until those decisions are made and propagated.
