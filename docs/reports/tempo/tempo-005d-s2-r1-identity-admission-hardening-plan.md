# TEMPO-005D-S2-R1 Identity Admission Hardening Plan

Task ID: TEMPO-005D-S2-R1  
Track: Canonical Tempo and Stage 1  
Repository: `A:\the-seed-in-my-mind-open-core`  
Target document: `docs/identity-admission-and-invitation-capacity-spec-v0.md`  
Audit date: 2026-07-12  
Status: read-only internal consistency and revision-planning audit.

## 1. Executive assessment

The target document is a strong first reconciliation draft: it establishes sponsor-authored Profile-v0 admission, permissionless local preparation, non-canonical admission requests, scarce invitation capacity, restricted initial authority, pseudonymous access, sponsor accountability limits, and separation from private account state.

The document is not yet ready for cross-document reconciliation or runtime implementation. The main hardening needs are internal consistency, exact dependency boundaries, and removal of over-specific mechanisms that duplicate replay validation.

Blocking issues for the later edit pass:

1. The document preserves only the Anthill anchor in many normative effects, but current Protocol v5 text requires a fuller identity-root structure. Protocol v5 itself uses conflicting root names, so the target should refer to all required identity structural roots and defer the exact root registry/schema to the protocol structural-role authority until reconciled.
2. Profile-v0 `identity_create` should have a fixed target kind of `human` under the admission profile, not an unconstrained payload field. This intended human kind must remain separate from VH.
3. Verification claims must remain ordinary truth claims and evidence under one epistemic system. Any restricted verification event should create or reference ordinary truth-claim/evidence objects, not a parallel verification-only object model.
4. `verification_reference` should be retained only as optional/nullable canonical commitment metadata, not as admission authorization, verification state, or a required privacy-leaking pointer.
5. `admission_authorization_reference` and `eligibility_snapshot_reference` currently duplicate replay validation. The minimum deterministic design is a reduced admission-context commitment binding profile, sponsor, capacity period, and rulebook, while replay validates actual eligibility/capacity at event position.
6. The two-signature construction order must be specified to avoid circular signatures: the applicant proof must bind the event ID and non-proof admission fields; the sponsor signature then binds the complete payload including the applicant proof.
7. The state-machine names are partly lifecycle-like and risk implying mutual exclusion. They should become orthogonal status lanes.
8. Admission liveness needs constitutional guardrails: inviter eligibility must have practical effect over bounded certified-cycle periods, without allowing machine-only capacity generation.
9. Dmax, forced-boundary, and record-only behavior uses a mix of `SHOULD`, `RECOMMENDS`, and `MUST`. Profile-v0 should state one exact rule.
10. Capacity privacy language must distinguish DTO omission from actual secrecy: if capacity is deterministically derivable from public canonical state, it is not cryptographically hidden.

## 2. Document metrics and baseline hash

Target exists: yes  
Path: `docs/identity-admission-and-invitation-capacity-spec-v0.md`  
File size: 399,875 bytes  
Line count: 13,491 line objects by `Get-Content`; `Measure-Object -Line` reported 9,147 because it counted line content differently for this file.  
SHA-256: `72687b46bd9f5e2abc5dfb828a5bb3cca20f40a834f3a64185efd9f22431c0a3`

Baseline evidence:

- Section headings run from Section 0 through Section 33.
- The target heavily references Anthill anchors but has no hits for `Backyard`, `Relationship Garden`, or `Self Tree`.
- Protocol v5 currently requires identity-root structural roles including BACKYARD, RELATIONSHIP_GARDEN, SELF_TREE, and ANTHILL at `docs/protocol v5.md:3102-3104`, `docs/protocol v5.md:3185-3191`, and structural-role creation rules at `docs/protocol v5.md:4844-4895`.

## 3. Blocking semantic issues

| Issue | Why it blocks controlled editing/runtime | Primary target sections |
| --- | --- | --- |
| Anthill-only identity roots | `identity_create` effects would fail to preserve current Protocol v5 structural-root requirements. | 1.4, 2.4, 3.5, 4.16-4.17, 5.7, 5.11, 6.6, 6.14, 11.13-11.15, 14.10, 20.3, 21.18, 22.15, 27.23, 28.10, 33.9, 33.12 |
| Target identity kind not fixed in event semantics | Public reads mention `identity_kind`, but `identity_create` payload does not say whether kind is fixed, encoded, or derived. | 11.1, 11.4, 14.2, 21.8, 28.3-28.4, 31.8 |
| Verification ontology incomplete | The target says verification claims are truth claims, but restricted-lane wording could be read as a parallel event/object model. | 1.3, 2.4-2.5, 4.9-4.13, 14.5-14.8, 18.7, 27.21, 31.9-31.10 |
| `verification_reference` privacy and authority ambiguity | Current text lets it refer to private evidence packages, but the field is in canonical `identity_create`. | 11.4, 11.10, 11.11-11.12, 23.19, 31.8 |
| Admission authorization over-commitment | Full authorization context with `eligibility_snapshot_reference` duplicates replay validation and risks stale/frozen authority confusion. | 10.16-10.17, 11.9, 12.9, 13.1-13.20, 31.5-31.6 |
| Two-signature circularity | Applicant proof currently binds final event fields, including authorization reference; construction order must avoid binding a payload hash that includes the proof itself. | 11.8, 11.11, 11.12, 12.4-12.14, 13.3 |
| Eligibility modeled as states | Names imply lifecycle transitions despite text saying lanes are independent. | 4.8, 6.2, 6.6-6.12, 14.2, 28.3-28.15 |
| Admission liveness under stalls | Certified-cycle maturation plus no forced-boundary capacity can freeze admission indefinitely. | 8.7-8.8, 9.3-9.8, 10.4-10.5, 19.3-19.13, 25.7-25.20 |
| Positive inviter capacity not guaranteed | Rulebooks can satisfy "inviter-eligible" while generating zero indefinitely unless bounded. | 8.7, 9.3, 10.4, 19.13, 25.7-25.8, 25.14-25.20 |
| Dmax/forced-boundary inconsistency | Current wording alternates between `MUST NOT`, `SHOULD NOT`, and `RECOMMENDS`. | 19.6-19.12, 27.19, 31.38, 32.8, 33.8 |
| Capacity privacy overclaim | Public derivability and DTO omission are not the same as cryptographic privacy. | 10.22, 13.20, 28.16-28.17, 31.19 |

## 4. Identity structural-root correction

Current issue:

- Target Sections 1.4, 5.11, 6.6, 11.13-11.15, 14.10, 20.3, 21.18, 22.15, 27.23, 28.10, and 33.9/33.12 define or test only an Anthill anchor.
- Protocol v5 says each identity has identity-root structures. One passage lists BACKYARD, RELATIONSHIP_GARDEN, SELF_TREE, and ANTHILL (`docs/protocol v5.md:3102-3104` and `docs/protocol v5.md:3185-3191`). Another passage lists MINDGARDEN, BACKYARD_OF_RELATIONSHIPS, SELF_TREE, and ANTHILL (`docs/protocol v5.md:3155-3181`). Structural-role rules require identity-root structural roles to be created atomically and replay identically (`docs/protocol v5.md:4844-4895`).

Required semantic change:

- Replace Anthill-only identity-creation effects with an `identity_structural_roots` concept.
- State that valid `identity_create` must create or deterministically derive all currently required identity structural roots atomically.
- Keep Anthill-specific verification/provenance semantics as a subsection of the broader root set.
- Do not choose final names if authority remains conflicted. Use "the current protocol-defined identity-root set" until the structural-role authority is reconciled.

Blocker:

- Current Protocol v5 root terminology is not fully settled: BACKYARD/RELATIONSHIP_GARDEN versus MINDGARDEN/BACKYARD_OF_RELATIONSHIPS, and whether Mindgarden is required as a distinct root.

## 5. Target identity-kind correction

Current issue:

- The target calls `identity_create` the event that creates a "new canonical human identity" in Section 11.1.
- Public reads mention `identity_kind` in Sections 28.3 and 28.4.
- The required payload in Section 11.4 does not include `identity_kind`.

Recommended exact resolution:

- Profile-v0 `identity_create` should have fixed target kind `human` by admission profile rule.
- Do not add a free-form `identity_kind` payload field unless Appendix A explicitly adopts it for all identity events.
- Public DTOs may expose `identity_kind = human` as a derived/profile-fixed classification for event-derived Profile-v0 identities.
- State explicitly that `identity_kind = human` is an intended identity class and does not establish VH, VI, uniqueness, writer eligibility, voter eligibility, governance eligibility, Tempo eligibility, or invitation eligibility.

Affected sections:

- 4 Terminology, 11.1, 11.4, 14.2, 21.8, 22.2-22.4, 25.3, 28.3-28.4, 31.8, 33.3, 33.9.

## 6. Verification-claim ontology correction

Current issue:

- Sections 1.3, 2.4, 4.10, and 18.7 correctly state that verification claims are truth claims and derive certainty through evidence, contradiction, challenge, and outcomes.
- Sections 14.5-14.8 and 29.5-29.6 refer to restricted verification events but do not settle whether those events create ordinary truth-claim/evidence objects or a parallel verification object model.
- The target does not reference `identity_verification_update`, while Appendix A currently defines it as updating verification status and enabling/disabling canonical authorship eligibility.

Recommended exact resolution:

- Preserve one common epistemic system.
- Define a verification claim as an ordinary canonical truth-claim object with a verification predicate and subject identity.
- If a future restricted event such as `verification_claim_create` is adopted, it must create or reference an ordinary `truth_claim` idea under restricted authorization. It must not create a parallel verification-only truth object.
- Verification evidence, contradiction, and challenges should likewise reuse ordinary evidence/connection/challenge ontology with restricted event-family authorization where needed for inactive identities.
- `identity_verification_update` should not remain as a normal public event that directly sets truth or writer eligibility. It should be either:
  - deprecated for post-genesis public use and replaced by derived activation from verification claims and rulebooks; or
  - limited to explicit genesis/import/compatibility activation records whose authority and non-truth status are documented.

Preferred decision:

- Deprecate `identity_verification_update` as ordinary post-genesis verification authority. Retain only a compatibility/genesis/import role if needed, and require normal verification to derive from truth claims/evidence/challenges.

Affected sections:

- 1.3, 2.4-2.5, 4.9-4.13, 14.5-14.8, 18.7, 23.19, 27.21, 29.5-29.6, 31.9-31.10, 32.3, 32.7, 32.13, 33.10.

## 7. `verification_reference` decision

Current issue:

- Section 11.4 lists `verification_reference` as an at-minimum field.
- Section 11.10 makes it optional/nullable and allows references to verification claims, evidence commitments, private evidence submission references, request-stage verification packages, or other authorized verification objects.
- This risks collapsing admission and verification, leaking private material, and increasing canonical payload complexity.

Recommended exact decision:

- Retain `verification_reference` only as an optional/nullable canonical commitment field.
- It must not be required for admission.
- It must not establish VH, VI, uniqueness, writer eligibility, inviter eligibility, or verification outcome.
- It must not reference non-canonical private request-pool identifiers, relay identifiers, contact details, raw evidence, or private submission systems directly.
- If used, it may point only to a privacy-safe canonical commitment, existing canonical verification artifact, or canonical digest of an applicant-authorized verification package.
- Revise Appendix A later so the field is optional/nullable rather than required for Profile-v0 admission.

Affected sections:

- 0.2 identifiers, 7.8-7.10, 11.4, 11.10-11.12, 12.4 if proof binds it, 23.19, 28.5, 31.8, 31.25, 32.3, 32.10.

## 8. Admission-authorization simplification

Current issue:

- Sections 10.16, 11.9, and 13 define `admission_authorization_reference` as a commitment to profile, sponsor, capacity period, eligibility snapshot, rulebook, and optional fields.
- Sections 11.9, 13.1, 13.7, and 13.16 also correctly say replay must independently verify eligibility and capacity at canonical application position.
- `eligibility_snapshot_reference` adds conceptual weight without serving as authority, and can imply eligibility is frozen even though the event must be validated against actual canonical state.

Comparison:

1. Full context including eligibility snapshot: too heavy; duplicates replay validation; creates stale/frozen-authority confusion.
2. Reduced context containing profile, sponsor, capacity period, and rulebook: sufficient to bind the handshake to the intended admission lane while preserving event-position replay as authority.
3. No explicit context beyond capacity period and event-position replay: simplest for replay, but weaker for applicant/sponsor UX because the applicant proof may not bind the intended rulebook/profile context.

Recommended minimum deterministic design:

- Keep a reduced admission context/reference, but remove `eligibility_snapshot_reference`.
- The reduced context should bind:
  - admission profile/version;
  - sponsor identity;
  - capacity period;
  - rulebook reference or rulebook set active for that capacity period.
- Actual sponsor eligibility, suspension state, key state, and spendable capacity must always be recomputed at event application position.
- The context/reference is a signed handshake commitment, not a bearer token, reservation, eligibility proof, or capacity proof.

Affected sections:

- 0.2, 10.16-10.17, 11.9, 12.9, 13.1-13.20, 19.3-19.5, 20.15, 23.15, 31.5-31.6, 32.5, 32.8.

## 9. Two-signature construction sequence

Current issue:

- Sections 11.8, 11.11, and 12.4-12.14 require both applicant possession proof and sponsor signature.
- The document must prevent circular construction where the applicant proof signs a payload hash that includes the proof itself, or where the sponsor signs before the applicant proof exists.

Required normative handshake:

1. Applicant locally creates a target identity ID and initial key descriptor.
2. Sponsor identity is selected.
3. Sponsor and applicant identify the admission profile, capacity period, and rulebook reference for the reduced admission context.
4. Event ID is selected before final applicant possession proof.
5. Applicant verifies the final non-proof admission fields:
   - event ID;
   - target identity ID;
   - initial key descriptor;
   - initial public key ref;
   - sponsor identity ID;
   - reduced admission context/reference;
   - optional `verification_reference` commitment or null.
6. Applicant signs domain-separated possession bytes over those fields. The possession bytes must not include the possession proof itself, the sponsor signature, or the final authored-candidate signature.
7. Sponsor assembles the final `identity_create` payload including the applicant proof.
8. Sponsor constructs Profile-v0 signed candidate bytes, including payload hash of the complete payload.
9. Sponsor signs the authored candidate.
10. Publication assigns canonical position; replay validates sponsor signature, applicant proof, context, capacity, target uniqueness, structural roots, and atomic effects.

Sections needing a normative handshake description:

- 11.4, 11.8, 11.11, 11.12, 12.4-12.14, 13.3, 20.5, 23.11, 23.18, 27.5-27.7, 31.7-31.8.

## 10. Orthogonal eligibility-state model

Current issue:

- Section 6.2 presents a conceptual progression from `LocalCandidate` to `CanonicalInactiveIdentity` to independently derived eligibility states.
- Sections 6.6-6.10 define `CanonicalInactiveIdentity`, `WriterEligibleIdentity`, `InviterEligibleIdentity`, `SuspendedInviter`, and `DormantIdentity`.
- Although Section 6.11 says the protocol must not assume a linear hierarchy, the names and transition diagrams still risk a mutually exclusive lifecycle interpretation.

Recommended replacement model:

- Rename `CanonicalInactiveIdentity` to `CanonicalAdmittedIdentity` or `CanonicalIdentityWithoutOrdinaryAuthority`.
- Define orthogonal lanes:
  - canonical existence;
  - key-control state;
  - identity structural-root state;
  - verification state;
  - restricted verification-lane eligibility;
  - ordinary writer eligibility;
  - challenge eligibility;
  - voter eligibility;
  - governance eligibility;
  - Tempo eligibility;
  - inviter eligibility;
  - invitation capacity balance;
  - invitation suspension;
  - dormancy/recovery state.
- Replace transition language with lane predicates and activation boundaries.
- Keep "inactive" as a derived convenience label only when no ordinary participation lanes are active.

Affected sections:

- 4.8, 6.2, 6.6-6.12, 14.1-14.24, 17.10, 20.3, 25.9, 28.3-28.15, 30.27, 33.9, 33.13.

## 11. Admission-liveness guarantees

Current issue:

- Capacity generation depends on certified cycles and human-deliberative boundaries.
- Forced, degraded, record-only, or machine-only boundaries do not generate normal authority.
- Inviter maturation may require certified cycles.
- If cycles stall or certified cycles are unavailable, admission can freeze indefinitely even for eligible humans.

Recommended constitutional liveness guarantees:

- Existing valid spendable capacity should remain spendable during stalls unless a deterministic suspension, expiration, or rulebook transition already applies.
- A rulebook must not make inviter eligibility merely nominal. If an identity is inviter-eligible and the system continues to produce qualifying certified human-deliberative cycles, that identity must receive positive spendable capacity within a bounded number of qualifying capacity periods, unless suspended by a canonical rule.
- Rulebooks must define the maximum number of qualifying periods before positive capacity is generated.
- When no qualifying human-deliberative/certified cycles occur, replay should derive an explicit `admission_liveness_blocked` or equivalent status rather than silently pretending capacity can appear.
- The document may name a future bounded fallback profile, but Profile v0 should not mint capacity from machine-only cycles.

Affected sections:

- 1.6.4-1.6.8, 8.7-8.8, 9.3, 9.8, 10.4-10.5, 19.3-19.13, 25.7-25.20, 26.8-26.12, 31.34, 33.4, 33.8.

## 12. Positive-capacity guarantee

Current issue:

- Section 8.7 and 9.3 state that inviter eligibility must be generally attainable.
- Section 10.4 says the exact generation rate is rulebook-controlled.
- Without a safe-range rule, a rulebook could make everyone inviter-eligible while assigning zero capacity indefinitely.

Recommended exact constitutional language:

- "For any identity that remains inviter-eligible and unsuspended across the rulebook-defined qualifying period, the active rulebook MUST generate at least one spendable invitation-capacity unit within the protocol-defined maximum number of qualifying certified capacity periods."
- "A rulebook value or formula that causes all or a class of unsuspended inviter-eligible identities to receive zero capacity indefinitely is invalid unless the system is in an explicit admission-liveness failure state or a constitutionally authorized emergency suspension."
- "Capacity may remain low, delayed, capped, or diversity-conditioned, but inviter eligibility must have practical effect."

Affected sections:

- 8.7, 9.3, 10.4, 19.13, 25.7-25.8, 25.14-25.20, 31.15, 31.37.

## 13. Dmax and forced-boundary rule

Current issue:

- Section 19.6 says Profile v0 "SHOULD require" certified human-deliberative cycles for capacity.
- Section 19.9 says Profile v0 "RECOMMENDS" forced boundaries generate no capacity/maturation unless human certification requirements are met.
- Section 19.10 says forced boundaries "SHOULD NOT" generate capacity, increase inviter eligibility, satisfy maturation, restore suspended inviter, or increase rollover cap.
- Other sections use stronger language: forced, degraded, record-only, or machine-only boundaries "MUST NOT" generate invitation maturity or authority unless separately authorized.

Recommended exact Profile-v0 rule:

- Ordinary Profile-v0 invitation capacity generation requires a qualifying certified human-deliberative capacity period.
- Dmax, forced, survivor, degraded, record-only, or machine-only boundaries do not generate new invitation capacity.
- They do not count for inviter maturation.
- They do not activate new inviter eligibility.
- They do not restore invitation suspension.
- They do not increase rollover caps.
- They may preserve canonical ordering, preserve already valid spendable balances, and allow spending of previously generated capacity only if the active rulebook explicitly allows spending during that mode and no suspension applies.
- Later certification of a forced boundary may permit future rulebook-defined repair, but does not retroactively turn the forced boundary into an ordinary capacity-generating cycle.

Affected sections:

- 19.6-19.12, 19.18, 19.21, 23 capacity errors if relevant, 27.19, 31.38, 32.8, 33.8.

## 14. Capacity-publicity correction

Current issue:

- Section 10.22 says public reads should expose remaining capacity but privacy-preserving implementations may limit real-time disclosure.
- Section 28.16 says capacity summaries may expose generated/consumed/spendable/carryover/expiration values, and may expose a reduced real-time view for coercion risk.
- If capacity is replay-derived from public canonical events and rulebooks, exact capacity is publicly derivable by conforming full nodes even if a DTO omits it.

Recommended precise language:

- DTOs may omit exact remaining capacity or expose a delayed, bucketed, or reduced convenience view to reduce targeting/coercion in common clients.
- Omission from a DTO is not cryptographic privacy.
- Conforming nodes with canonical history and rulebook data can derive exact capacity unless a future cryptographic/private-state admission profile explicitly changes the model.
- Public API documentation must not claim exact remaining capacity is hidden; it can only say it is not necessarily presented in every public response.

Affected sections:

- 10.22, 13.20, 28.16-28.17, 31.19, 32.17.

## 15. Normative-content split

Recommended future split, without moving content in this task:

| Material | Current sections | Recommended classification |
| --- | --- | --- |
| Protocol core for admission architecture | 0-22, 25, 30, 33 summary where duplicated | Normative protocol core, but Section 33 should become informative summary/cross-reference. |
| Validation and stable errors | 23 | Normative conformance annex or validation annex. |
| Security analysis | 24 | Informative security analysis, with any normative rules moved/cross-referenced to core sections. |
| Constitutional/rulebook boundary | 25 | Normative protocol core. |
| Future admission profiles | 26 | Normative future-profile requirements annex plus informative examples. |
| Conformance requirements | 27 | Normative conformance annex. |
| Public read surfaces | 28 | Normative API/read-surface annex; exact DTOs may belong to API contract. |
| Implementation sequencing | 29 | Implementation plan, non-normative except safety gates that should be cross-referenced from core/conformance. |
| Non-goals | 30 | Normative scope/prohibition annex; remove repetition by cross-reference. |
| Open questions | 31 | Deferred-parameter and owner-decision register, non-normative until closed. |
| Reconciliation checklist | 32 | Repository workflow/reconciliation checklist, non-normative. |
| Summary guarantees | 33 | Informative summary; must not introduce new normative language. |

## 16. Duplication-reduction plan

| Repeated rule | Authoritative target section | Sections to cross-reference instead of repeating |
| --- | --- | --- |
| Sponsorship is not verification | 2.3/2.4 and 8.3 | 11.14, 16.6, 18.4, 25.3, 30.13, 33.11 |
| No permanent inviter class | 2.7 and 25.7 | 1.6.1, 8.7, 9.3, 33.4 |
| Minimal initial authority | 14 | 18.4, 25.12, 30.3, 33.9 |
| Anthill/topology is not evidence or authority | 2.4 and corrected structural-root section | 1.4, 3.5, 4.16-4.17, 5.11, 8.10, 18.8, 25.11, 30.12, 33.12 |
| AI has no canonical admission authority | 2.11 and 25.6 | 18.27, 24.14, 30.17, 32.22, 33.20 |
| No private account/operator authority | 2.10 and 25.2/25.6 | 5.9, 13.9, 22.19-22.20, 28.25-28.26, 29.23, 30.15-30.16, 33.19 |
| No verification-weighted votes/truth | 25.16 and 30.10-30.11 | 9.6, 18.6, 32.7, 33.15 |
| Forced boundaries generate no ordinary admission authority | 19.9-19.11 after hardening | 27.19, 31.38, 32.8, 33.8 |

## 17. Terminology normalization

Recommended normalizations:

- `CanonicalInactiveIdentity`: rename to `CanonicalAdmittedIdentity` or define as a display label for the combination "canonical existence true; ordinary authority lanes false".
- `WriterEligibleIdentity`: replace with `ordinary_writer_eligibility` lane.
- `InviterEligibleIdentity`: replace with `inviter_eligibility` lane.
- `SuspendedInviter`: replace with `invitation_suspension` lane.
- `DormantIdentity`: replace with `identity_dormancy` or `key/control availability` lane.
- `identity_id` versus `target_identity_id`: use `identity_id` for the payload field created by Appendix A; use `target_identity_id` only in explanatory/proof bytes where disambiguating sponsor versus target, and state they are the same value.
- `verified-human status` versus `VH`: use VH for the human-existence certainty/eligibility track; reserve "verified human" for derived eligibility status at a specified boundary.
- `verification level`: use only as derived gate/tier, not as truth weight or invitation authority by itself.
- `inviter level`: avoid unless there is a defined tier; prefer `inviter_eligibility` and `generated_capacity`.
- `capacity period`: define as a replay-visible period derived from qualifying certified cycle state; do not use as wall-clock period.
- `eligibility snapshot`: remove from authorization context or redefine as derived read/explanation only.
- `verification_reference`: rename in prose to `optional_verification_commitment` if Appendix A permits; otherwise state `verification_reference` is optional/nullable commitment metadata.
- `speaker_identity_id`: for Profile-v0 sponsor-authored `identity_create`, keep absent unless Appendix A explicitly requires it; sponsor is author, not speaker for applicant.
- Historical key reuse: clarify that key uniqueness for Profile-v0 human identities rejects incompatible reuse; legacy compatibility does not fabricate modern key history.
- Forced boundary, record-only, Dmax: use exact Cycle/Tempo reconciled terms and one Profile-v0 capacity rule.
- Genesis exception: use only for bounded genesis/import state; do not let it become ongoing operator admission.

`SHOULD` to `MUST` candidates:

- Section 19.6 certified-cycle requirement.
- Section 19.9/19.10 forced-boundary no-capacity/no-maturation rule.
- Section 10.22/28.16 public derivability caveat.
- Section 8.8 maturation if the approved architecture requires a hard anti-Sybil delay, subject to liveness guarantees.

## 18. Section-by-section edit map

| Section | Current issue | Required semantic change | Move/rewrite/delete | Depends on owner decision? |
| --- | --- | --- | --- | --- |
| Related specifications, Change control, 0 | Authority boundaries are useful; terms include `admission_authorization_reference` and `target_identity_id` before hardening. | Update terminology after reduced context and target identity naming decisions. | Rewrite small parts. | No. |
| 1 | Purpose is strong but Anthill-only and verification ontology need tightening. | Introduce all required identity structural roots; state verification uses ordinary truth ontology. | Focused rewrite. | Structural-root exact registry yes. |
| 2 | Invariants repeat many rules and Anthill-only framing. | Add structural-root invariant; cross-reference instead of repeating. | Focused rewrite. | Structural-root exact registry yes. |
| 3 | Scope/boundaries mostly sound. | Clarify structural-root boundary and restricted verification ontology boundary. | Small clarification. | No. |
| 4 | Terminology includes lifecycle-like states and Anthill-only terms. | Replace with orthogonal lane terms; add fixed Profile-v0 human target kind. | Focused rewrite. | No. |
| 5 | Admission architecture mostly sound; atomicity mentions Anthill only. | Require all identity structural roots atomically; reduce hidden reservation language if context simplified. | Focused rewrite. | Structural-root exact registry yes. |
| 6 | State machine risks mutually exclusive lifecycle interpretation. | Convert to layered/lane model; rename `CanonicalInactiveIdentity`. | Focused semantic rewrite. | Owner may approve exact name. |
| 7 | Non-canonical request model is sound. | Ensure verification material cannot leak through `verification_reference`; request-stage proof separated from final proof. | Small clarification. | No. |
| 8 | Sponsor eligibility sound but liveness and maturation need guardrails. | Add positive capacity and bounded qualifying-period principle by reference. | Small rewrite. | Owner may set bound later. |
| 9 | Invitation eligibility sound but can remain nominal. | Add practical-effect guarantee; lane terminology. | Focused rewrite. | Bound value yes, principle no. |
| 10 | Capacity section lacks positive capacity guarantee and overstates privacy. | Add safe-range/positive-capacity rule; correct public-capacity language. | Focused rewrite. | Bound value yes. |
| 11 | `identity_create` payload includes contentious fields and lacks target-kind/root clarity. | Fixed human target kind; optional `verification_reference`; all structural roots; reduced context. | Focused semantic rewrite. | Structural-root exact registry yes. |
| 12 | Possession-proof construction needs exact non-circular order. | Add construction sequence and define proof bytes exclude proof/sponsor signature. | Focused rewrite. | No. |
| 13 | Authorization reference overcommits with eligibility snapshot. | Reduce context; remove `eligibility_snapshot_reference`; replay remains authority. | Focused rewrite. | No if accepted. |
| 14 | Initial authority is useful but should use orthogonal lanes and ordinary truth ontology. | Replace inactive-state language; restricted lane creates ordinary truth/evidence objects. | Focused rewrite. | No. |
| 15 | Stranger/high-risk admission mostly sound. | Cross-reference non-canonical request and optional verification commitment privacy. | Small clarification. | No. |
| 16 | Lineage section sound but repeated "not verification". | Keep lineage-specific rule; cross-reference verification/admission separation. | Minor rewrite. | No. |
| 17 | Accountability sound. | Ensure consequences bind only invitation lane unless separately authorized. | Minor rewrite. | No. |
| 18 | Sybil model sound; repeated common rules. | Cross-reference core rules; keep security-specific residual-risk discussion. | Minor rewrite. | No. |
| 19 | Cycle integration has inconsistent Dmax/forced wording and liveness gap. | One exact Profile-v0 forced/Dmax rule; admission-liveness failure state; positive capacity over qualifying periods. | Focused semantic rewrite. | Bound/fallback details yes. |
| 20 | Replay section mostly sound but must include all structural roots and reduced context. | Update replay state/application order. | Focused rewrite. | Structural-root exact registry yes. |
| 21 | Genesis boundary mostly sound but Anthill-only and capacity/liveness transition needs care. | Include all roots; keep genesis bounded; no continuing operator exception. | Focused rewrite. | Structural-root exact registry yes. |
| 22 | Legacy compatibility sound. | Include structural roots, legacy unknown identity kind, and no fabricated verification. | Minor rewrite. | No. |
| 23 | Stable validation is normative/conformance material. | Keep as validation annex; update errors for reduced context, optional verification reference, roots. | Focused rewrite. | No. |
| 24 | Security analysis contains normative duplicates. | Mark informative; move/cross-reference normative rules. | Split later. | No. |
| 25 | Strong core section. | Add positive capacity, structural roots, reduced context, and exact forced-boundary rules. | Focused rewrite. | Root registry/bounds yes. |
| 26 | Future profile annex. | Keep as future-profile requirements; distinguish informative examples. | Split later. | No. |
| 27 | Conformance annex. | Add fixtures for all roots, reduced authorization, proof handshake, liveness blocked, capacity publicity. | Focused rewrite later. | No. |
| 28 | Public reads. | Add all structural roots; clarify exact capacity public derivability versus DTO omission; fixed human kind. | Focused rewrite. | Structural-root exact registry yes. |
| 29 | Implementation sequencing is non-normative plan with safety gates. | Mark implementation-plan status; update sequencing for roots and verification ontology. | Split later. | No. |
| 30 | Non-goals mostly normative scope. | Cross-reference core sections; avoid duplicate rules drifting. | Minor rewrite. | No. |
| 31 | Open questions. | Close questions settled by hardening; add only real remaining owner decisions. | Rewrite. | Yes. |
| 32 | Reconciliation checklist is workflow. | Keep non-normative; update checklist after hardening. | Minor rewrite. | No. |
| 33 | Summary repeats normative rules. | Mark informative summary; ensure it cross-references, not defines. | Rewrite. | No. |

## 19. Recommended editing batches

1. Structural-root and identity-kind pass:
   - Sections 1, 2, 3.5, 4, 5.7/5.11, 6, 11, 14, 20-22, 27-28, 33.
   - Goal: no Anthill-only creation/effect remains where all roots are required.

2. Verification ontology pass:
   - Sections 1.3, 2.4-2.5, 4.9-4.13, 14.5-14.8, 18.7, 27.21, 29.5-29.6, 31.9-31.10, 32.3/32.7/32.13, 33.10.
   - Goal: one ordinary truth/evidence/challenge system.

3. Admission payload and handshake pass:
   - Sections 7, 10.16-10.17, 11, 12, 13, 20, 23, 27.
   - Goal: optional `verification_reference`, reduced authorization context, non-circular two-signature sequence.

4. Orthogonal state and eligibility pass:
   - Sections 4, 6, 8-10, 14, 17, 25, 28, 30, 33.
   - Goal: lane model replaces lifecycle-state ambiguity.

5. Cycle/liveness/capacity pass:
   - Sections 8-10, 19, 25, 27, 28, 31, 32, 33.
   - Goal: positive-capacity guarantee and exact forced-boundary rule.

6. Document split and duplication pass:
   - Sections 23-33.
   - Goal: normative core, conformance annex, informative security analysis, implementation plan, and reconciliation checklist are clearly separated.

## 20. Owner decisions still required

Only these require owner/spec authority beyond the hardening plan:

1. Exact identity structural-root registry and naming: BACKYARD/RELATIONSHIP_GARDEN/SELF_TREE/ANTHILL versus MINDGARDEN/BACKYARD_OF_RELATIONSHIPS/SELF_TREE/ANTHILL, and whether Mindgarden is distinct from Backyard.
2. Exact name replacing `CanonicalInactiveIdentity`, if a new term is desired.
3. Maximum number of qualifying certified capacity periods before an unsuspended inviter-eligible identity must receive positive capacity.
4. Whether any Profile-v0 emergency/admission-liveness fallback exists beyond preserving existing capacity and deriving `admission_liveness_blocked`.
5. Whether `verification_reference` should be renamed in Appendix A or kept with optional/nullable semantics for compatibility.
6. Whether `identity_verification_update` remains as genesis/import compatibility only or is fully deprecated from Profile-v0 post-genesis operation.

## 21. No-change declaration

Target document edited: no  
Other normative specifications edited: no  
Runtime code edited: no  
Migrations edited: no  
Fixtures edited: no  
DTOs/APIs edited: no  
Exports generated: no  
Databases used: no  
Private repository changed: no

This report is a planning artifact only.

## 22. Readiness

- `Target document edited: no`
- `Hardening plan complete: yes`
- `Core semantic blockers isolated: yes`
- `Ready for controlled in-place edits: yes`
- `Ready for cross-document reconciliation: no`
- `Ready for runtime implementation: no`

Cross-document reconciliation is not ready until the target document is hardened in place and the remaining owner decisions above are resolved. Runtime implementation is not ready because exact event payloads, structural-root effects, verification-event ontology, capacity/liveness semantics, and conformance obligations still need controlled specification edits.
