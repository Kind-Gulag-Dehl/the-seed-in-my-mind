# TEMPO-005D-S3-R1 Identity Admission Cross-Document Reconciliation Audit

Task ID: TEMPO-005D-S3-R1
Repository: `A:\the-seed-in-my-mind-open-core`
Audit date: 2026-07-12
Scope: read-only reconciliation planning across authoritative and implementation-facing documents.

## 1. Executive Summary

The completed `docs/identity-admission-and-invitation-capacity-spec-v0.md` is internally complete and ready for cross-document reconciliation. It is not ready for runtime implementation because several higher-level protocol documents, exact-schema documents, replay/snapshot/API contracts, conformance fixtures, and current implementation-facing definitions still describe the older identity model.

The target Profile-v0 architecture is settled for this audit:

- Permissionless local identity preparation.
- Non-canonical admission request.
- Sponsor-authored canonical `identity_create`.
- Fixed target kind `identity_kind = human`.
- Atomic creation or deterministic derivation of `identity_structural_roots`: Mindgarden, Backyard of Relationships, Self Tree, Anthill.
- Admission creates a `CanonicalAdmittedIdentity` with restricted initial authority only.
- Sponsorship, lineage, Anthill topology, and admission are not VH, VI, uniqueness, or ordinary eligibility.
- Verification uses ordinary canonical truth claims, evidence, contradictions, challenges, outcomes, rulebooks, derived VH/VI certainty, and event-family-specific eligibility lanes.
- Admission authorization context is reduced to `admission_profile_version`, `sponsor_identity_id`, `capacity_period_id`, and `rulebook_reference`.
- `verification_reference` is optional, narrowly restricted, and has no admission or verification effect by itself.
- Invitation capacity is publicly replay-derivable in Profile v0.
- Qualifying capacity periods require certified human-deliberative cycles; Dmax, forced, degraded, survivor, record-only, or machine-only boundaries generate no invitation authority unless they independently satisfy those certification rules.
- `admission_liveness_blocked` must be exposed as deterministic replay state when no qualifying capacity period occurs.

The highest-risk unresolved areas are:

1. Protocol v5 and Appendix A still contain self-registration and verified-human-authorship wording that conflicts with sponsor-authored admission and restricted verification/key-control lanes.
2. Appendix A lacks the exact Profile-v0 `identity_create` payload, applicant proof, sponsor signature, admission authorization, structural-root, speaker, capacity-debit, and compatibility semantics.
3. `identity_verification_update` is still described as a post-genesis verification and authorship eligibility setter in Appendix A.
4. Current runtime-facing validation and storage paths still model `identity_create` as speaker/self-authored and, in one API path, account-coupled.
5. Replay, snapshot, API, DTO, and conformance definitions do not yet represent admission capacity, liveness, restricted verification lanes, identity structural roots, sponsor lineage, or exact applicant proof behavior.

Smallest safe first reconciliation batch: **TEMPO-005D-S3-R2A - Authority and Protocol Root Reconciliation**. It should update only authority/index/stage mapping, cross-document invariants, and Protocol v5 level semantics. It must not edit Appendix A exact schemas, runtime code, DTOs, migrations, fixtures, or exports.

## 2. Scope and Authority Method

The audit treats the identity-admission specification as the controlling target for Profile-v0 admission architecture unless contradicted by a higher constitutional invariant. The repository authority hierarchy was determined from `docs/authoritative-index.md`:

1. `docs/protocol v5.md` Section 0 and constitutional invariants control root protocol semantics.
2. `docs/canonical-encoding-and-hashing-spec.md` controls primitive encodings, hashing, byte formats, and commitments.
3. `docs/canonical-event-authorship-and-signature-profile-v0.md` controls authored candidates, Profile-v0 human-authorship signatures, signed bytes, `public_key_ref`, and replay-derived key state.
4. `docs/deterministic-replay-and-merge-spec.md` controls deterministic replay and merge behavior.
5. `docs/protocol v5-appendix-a.md` is subordinate schema authority for canonical events and payloads.
6. `docs/pod-consensus-and-canonical-publication-spec.md` controls canonical publication/finality mechanics.
7. `docs/node-and-conformance-spec.md` controls node behavior and conformance requirements.
8. Subsystem specs control their scoped domains where they do not contradict higher documents.
9. `docs/open-core-implementation-status.md` controls current implementation status only.
10. Current implementation files are evidence of implementation state, not protocol authority.

Findings are classified using the requested categories:

- Authority classifications: constitutional invariant, authoritative protocol semantics, exact-schema/encoding requirement, rulebook-controlled policy, compatibility/genesis/import behavior, implementation contract, derived status/documentation, informative or planning-only text, harmless historical terminology.
- Change categories: contradiction requiring normative edit, missing normative definition, stale terminology, ambiguous authority, compatibility classification required, implementation/schema drift, conformance gap, informative clarification only, no change required.
- Severity: blocker, high, medium, low, informational.

## 3. Baseline Identity-Admission Document Hash

File: `docs/identity-admission-and-invitation-capacity-spec-v0.md`

- SHA-256: `dd28615fb10d80d9d38bc2fb989973788627784a56aea911472b32e8d42f73b1`
- Size: 785,726 bytes.
- Lines: 13,953.

## 4. Files Inspected

Documentation and conformance files inspected or search-scanned:

- `AGENTS.md`
- `docs/authoritative-index.md`
- `docs/map.index.md`
- `docs/map.protocol-v5.md`
- `docs/map.node-and-replay.md`
- `docs/map.encoding-snapshots-bundles.md`
- `docs/map.offline-and-preservation.md`
- `docs/map.token-governance.md`
- `docs/cross-doc-invariants.md`
- `docs/identity-admission-and-invitation-capacity-spec-v0.md`
- `docs/reports/tempo/tempo-005d-a1-identity-admission-documentation-audit.md`
- `docs/reports/tempo/tempo-005d-s2-r1-identity-admission-hardening-plan.md`
- `docs/reports/tempo/tempo-005d-s2-r2e-final-internal-consistency-report.md`
- `docs/protocol v5.md`
- `docs/protocol v5-appendix-a.md`
- `docs/canonical-event-authorship-and-signature-profile-v0.md`
- `docs/canonical-encoding-and-hashing-spec.md`
- `docs/deterministic-replay-and-merge-spec.md`
- `docs/pod-consensus-and-canonical-publication-spec.md`
- `docs/node-and-conformance-spec.md`
- `docs/snapshot-format-v0.md`
- `docs/canonical-preservation-spec.md`
- `docs/offline-and-mindseed-spec.md`
- `docs/cycle-spec.md`
- `docs/tempo-spec.md`
- `docs/challenge-engine-spec.md`
- `docs/governance-spec.md`
- `docs/token-spec.md`
- `docs/verification-spec.md`
- `docs/privacy-and-high-risk-submission-spec.md`
- `docs/safety-spec.md`
- `docs/safety-rulebook-interface-mechanics-spec.md`
- `docs/ai-boundaries-spec.md`
- `docs/open-core-architecture-overview.md`
- `docs/open-core-boundary-manifest.md`
- `docs/open-core-split-and-data-boundary-spec.md`
- `docs/open-core-implementation-status.md`
- `docs/api-contract-read-only.md`
- `docs/protocol-event-registry.v1.md`
- `docs/conformance/canonical-event-signature-profile-v0.md`
- `docs/conformance/canonical-event-signature-profile-v0.vectors.json`
- `docs/conformance/tempo-cycle-event-fixtures.md`
- `docs/conformance/tempo-cycle-event-fixtures.v1.json`
- `docs/conformance/tempo-cycle-event-fixtures.schema.json`

Implementation-facing surfaces inspected or search-scanned:

- `backend/crates/api-types-canonical/src/lib.rs`
- `frontend/src/shared/types/canonical.ts`
- `backend/crates/event-log/src/schema.rs`
- `backend/crates/event-log/src/validation.rs`
- `backend/crates/storage/src/canonical.rs`
- `backend/crates/storage/src/types.rs`
- `backend/crates/replay/src/replay.rs`
- `backend/bins/seed-importer/src/main.rs`
- `backend/bins/api-server/src/server/handlers/canonical.rs`
- `backend/bins/api-server/src/server/types.rs`
- `backend/bins/api-server/src/server/tests_signed_ingress.rs`
- `backend/bins/api-server/src/server/tests_stage1_flow.rs`
- `backend/migrations/postgres/0016_create_canonical_writer_verification_state.sql`
- `backend/migrations/postgres/0019_add_account_canonical_writer_verification.sql`
- `backend/migrations/postgres/0022_signed_canonical_write_substrate.sql`

Total inspected/search-scanned files and surfaces: 58.

## 5. Search Vocabulary and Audit Method

Searches covered identity admission, self-registration, sponsor, invitation, admission authorization, verification references, eligibility snapshots, identity kinds, structural roots, verification ontology, writer and inviter eligibility, capacity, cycle liveness, admission errors, applicant proof, key descriptors, speaker identity, runtime validators, DTOs, and conformance vectors.

Representative search terms:

- `identity_create`, `identity creation`, `identity registration`, `self-registration`, `registration`, `invite`, `invitation`, `sponsor`, `admission request`, `admission_authorization_reference`, `verification_reference`, `eligibility_snapshot_reference`
- `CanonicalInactiveIdentity`, `CanonicalAdmittedIdentity`, `WriterEligibleIdentity`, `InviterEligibleIdentity`, `SuspendedInviter`, `DormantIdentity`
- `identity_kind`, `verified human`, `verified-human`, `VH`, `VI`, `canonical_writer_level`, `identity_verification_update`
- `Mindgarden`, `Mind Garden`, `Backyard of Relationships`, `Backyard`, `Relationship Garden`, `Garden of Relationships`, `BACKYARD`, `RELATIONSHIP_GARDEN`, `Anthill`, `identity_structural_roots`
- `verification claim`, `verification evidence`, `verification attestation`, `ordinary truth`, `truth_claim`, `challenge outcome`, `verification level`
- `inviter`, `invitation capacity`, `capacity period`, `positive capacity`, `zero capacity`, `admission_liveness_blocked`, `qualifying capacity period`, `Dmax`, `forced`, `degraded`, `survivor`, `record-only`, `machine-only`, `emergency mint`
- `stale_admission_authorization`, `author_key_inactive`, `author_key_revoked`, `inviter_ineligible`, `inviter_suspended`, `insufficient_invitation_capacity`, `identity_already_exists`, `public_key_already_registered`, `possession_proof`, `key descriptor`, `speaker_identity_id`, `payload_hash`, `public_key_ref`

The audit intentionally avoided database access, export generation, runtime test execution, and normative-document edits.

## 6. Reconciliation Matrix

| ID | Surface and source location | Current behavior or wording | Conflict or gap | Target reconciliation | Authority classification | Change category | Severity | Dependencies | Batch | Blocks runtime? |
|---|---|---|---|---|---|---|---|---|---|---|
| AD-001 | `docs/authoritative-index.md`; missing `docs/authoritative-stage-map.md`; target spec not indexed as an admission authority | The completed admission spec is not yet clearly placed in the authority index, and the requested stage map file is absent. | Reconciliation work lacks a single indexed authority relationship for Profile-v0 admission. | Add the identity-admission spec to the authority index with scope and precedence; either create or formally replace the missing stage-map reference. | Authoritative protocol semantics | Ambiguous authority | Blocker | None | R2A | Yes |
| AD-002 | `docs/protocol v5.md:3185` | "new user registers and emits a USER identity idea" with roots. | Directly implies self-authored registration instead of sponsor-authored canonical `identity_create`. | Replace with local identity preparation, non-canonical admission request, and sponsor-authored `identity_create`. | Constitutional invariant / authoritative protocol semantics | Contradiction requiring normative edit | Blocker | AD-001 | R2A | Yes |
| AD-003 | `docs/protocol v5.md:181`, `205`, `3088`, `3094-3098`; `docs/protocol v5-appendix-a.md:356`, `879` | Ordinary canonical events require verified-human authorship and active keys already owned by the author. | Needs explicit sponsor-authored admission and restricted verification/key-control lanes without treating the applicant key as previously active. | Clarify ordinary-event rule, admission exception shape, sponsor authorship, and restricted lanes. | Constitutional invariant / authoritative protocol semantics | Contradiction requiring normative edit | Blocker | AD-001, AD-002 | R2A | Yes |
| AD-004 | `docs/protocol v5-appendix-a.md:953-966` | `identity_create` payload has only `identity_id`, `initial_public_key_ref`, `initial_public_key_descriptor`, and `verification_reference`. | Missing sponsor/admission profile/capacity period/rulebook, reduced authorization reference, applicant proof, fixed human kind, structural roots, capacity debit, no-speaker rule, and no-reference encoding. | Replace Appendix A payload/effect definition with Profile-v0 sponsored admission schema and exact effects. | Exact-schema/encoding requirement | Missing exact-schema/encoding requirement | Blocker | R2A root semantics | R2B | Yes |
| AD-005 | `docs/protocol v5-appendix-a.md:970-982`; `docs/protocol v5.md:4669-4670` | `identity_verification_update` updates verification and enables/disables canonical authorship eligibility. | Target removes ordinary post-genesis direct verification-status authority; only genesis/import/legacy compatibility may remain. | Deprecate as ordinary protocol event; define compatibility provenance and retirement path where needed. | Authoritative protocol semantics | Contradiction requiring normative edit | Blocker | Verification ontology edit | R2B/R2C | Yes |
| AD-006 | `docs/protocol v5.md:3100-3102`, `3140`, `3155-3175`, `3185`, `4846`, `4881-4886`, `4944` | Stale root names include `BACKYARD`, `RELATIONSHIP_GARDEN`, `backyard`, and `relationship_garden`; root creation is tied to registration/verified human wording. | Target root names are Mindgarden, Backyard of Relationships, Self Tree, Anthill; aggregate is `identity_structural_roots`. | Normalize root names and keep final identifiers/encodings deferred to structural-role reconciliation. | Authoritative protocol semantics | Stale terminology | Blocker | AD-001 | R2A | Yes |
| AD-007 | `docs/protocol v5-appendix-a.md:798-807`, `953-966`; `backend/crates/event-log/src/validation.rs:197-224`; `backend/bins/seed-importer/src/main.rs:1572-1580` | `identity_create` is often modeled with `speaker_identity_id` or speaker equal to target identity. | Profile-v0 `identity_create` must have absent `speaker_identity_id`; sponsor is author, not applicant speaker. | Normalize schema, validation, tests, seed-import compatibility, and public DTOs. | Exact-schema/encoding requirement / implementation contract | Implementation/schema drift | Blocker | AD-004 | R2B/R2E | Yes |
| AD-008 | `docs/canonical-event-authorship-and-signature-profile-v0.md`; `docs/canonical-encoding-and-hashing-spec.md`; no exact applicant proof vector found | Applicant possession proof domain, exact bound fields, no-reference binding, and non-recursive construction are not defined outside the target spec. | Runtime cannot produce interoperable Profile-v0 identity admission proofs. | Add exact signed-byte definition, domain separator, optional/no-reference encoding dependency, and vectors. | Exact-schema/encoding requirement | Missing exact-schema/encoding requirement | Blocker | AD-004 | R2B | Yes |
| AD-009 | `docs/protocol v5-appendix-a.md:953-966`; no reduced authorization definition found in encoding/authorship docs | Appendix A has no `admission_authorization_reference`, `capacity_period_id`, `rulebook_reference`, or reduced context. | Admission validation cannot distinguish context validity from sponsor eligibility, suspension, and capacity. | Define reduced context and hash commitment under canonical encoding authority. | Exact-schema/encoding requirement | Missing exact-schema/encoding requirement | Blocker | AD-004, AD-008 | R2B | Yes |
| AD-010 | `docs/protocol v5-appendix-a.md:1005-1017`; `docs/canonical-event-authorship-and-signature-profile-v0.md` key lifecycle sections | Rotation and revocation are present but do not settle add/replacement, supersession, last-key, recovery, key-management eligibility, or admission initial-key interaction. | TEMPO-005D runtime remains blocked without exact lifecycle semantics. | Define direct-key lifecycle semantics or split unresolved parts into a scoped key-lifecycle spec before runtime. | Authoritative protocol semantics | Missing normative definition | Blocker | AD-001, AD-004 | R2B | Yes |
| AD-011 | `docs/verification-spec.md:120`, `181`, `552-562`, `648`, `707`, `820-845`, `1091`, `1145-1146` | Verification spec uses invite-only and invite-rate language but lacks sponsor-authored admission capacity, positive-capacity, or public derivability semantics. | It is unclear how verification output becomes `inviter_eligibility` and capacity under Profile v0. | Reconcile invite terminology into inviter eligibility, invitation capacity, suspension, and rulebook-controlled rates. | Rulebook-controlled policy / authoritative protocol semantics | Missing normative definition | Blocker | R2A/R2B | R2C | Yes |
| AD-012 | `docs/verification-spec.md`; `docs/protocol v5-appendix-a.md`; current API/DTO docs | No exact restricted verification lane for `CanonicalAdmittedIdentity` is defined outside the target. | New identities cannot progress toward verification without ordinary writer eligibility unless this lane exists. | Define restricted verification event-family authorization into ordinary truth/evidence/challenge objects. | Authoritative protocol semantics | Missing normative definition | Blocker | AD-005 | R2C/R2E | Yes |
| AD-013 | `docs/verification-spec.md:218`, `321`, `820-845`; `docs/protocol v5-appendix-a.md:970-982` | Anthill anchoring, operator verification, and `identity_verification_update` can be read as verification/status assignment rather than ordinary epistemic artifacts. | Target requires ordinary truth/evidence/challenge ontology -> VH/VI derivation -> eligibility lanes. | Reclassify sponsorship and Anthill topology as provenance/topology, not verification; remove direct status setting. | Authoritative protocol semantics | Contradiction requiring normative edit | Blocker | AD-005, AD-012 | R2C | Yes |
| AD-014 | `docs/protocol v5.md:205`; `docs/api-contract-read-only.md:674`, `842-853`; `backend/crates/replay/src/replay.rs:289`, `527-532`; migrations `0016`, `0019` | `canonical_writer_level` is represented as stored/mutable writer gate state. | Target treats it as legacy/materialized compatibility, not final replay-derived event-family authority. | Mark compatibility status, define retirement/migration, and avoid presenting it as final protocol authority. | Compatibility/genesis/import behavior | Compatibility classification required | High | AD-012, AD-013 | R2E | Yes |
| AD-015 | `docs/protocol-event-registry.v1.md` search terms: `identity_create`, `identity_verification_update`, `identity_key_rotate`, `identity_key_revoke` | Registry does not yet reflect the completed admission architecture and compatibility status. | Event families, payload fields, and compatibility classification drift from target. | Update registry only after Appendix A exact schemas are reconciled. | Exact-schema/encoding requirement | Stale terminology | High | AD-004, AD-005, AD-010 | R2B | Yes |
| AD-016 | `backend/crates/event-log/src/validation.rs:197-224` | Runtime validator requires `speaker_identity_id`, validates title, and treats identity event speaker as the new identity. | Directly contradicts sponsor-authored Profile-v0 `identity_create`. | Later runtime task must replace or quarantine this validator after schema reconciliation. | Implementation contract | Implementation/schema drift | Blocker | AD-004, AD-007 | R2E | Yes |
| AD-017 | `backend/crates/storage/src/canonical.rs:1209-1257`; `backend/bins/api-server/src/server/tests_stage1_flow.rs:310`; `backend/bins/api-server/src/server/handlers/canonical.rs:1542-1672` | Account/session-oriented identity and writer flows create or change canonical identity/writer state. | Public Profile-v0 admission must not depend on private accounts or operator-controlled flags. | Classify current paths as private/legacy compatibility or remove them from public canonical authority in runtime reconciliation. | Implementation contract / compatibility behavior | Implementation/schema drift | Blocker | AD-014, AD-016 | R2E | Yes |
| AD-018 | `backend/bins/seed-importer/src/main.rs:1572-1904` | Seed importer creates identity/root events with self speaker and bootstrap assumptions. | This may be valid import behavior but must not masquerade as Profile-v0 admission. | Classify as genesis/import compatibility and preserve provenance. | Compatibility/genesis/import behavior | Compatibility classification required | Medium | AD-035 | R2E | No, if classified before implementation |
| AD-019 | `docs/deterministic-replay-and-merge-spec.md`; `docs/snapshot-format-v0.md:188`, `346`, `393`; `docs/api-contract-read-only.md` | Replay/snapshot/API contracts do not define admission capacity, capacity debits, liveness, sponsor provenance, structural roots, restricted lanes, or key lifecycle history. | Clean replay and public explanation cannot reconstruct target state. | Extend replay, snapshot digests, read DTOs, and historical-read behavior. | Implementation contract / derived status | Implementation/schema drift | Blocker | R2B/R2C semantics | R2C/R2E | Yes |
| AD-020 | `docs/cross-doc-invariants.md:164-165`; `docs/protocol v5-appendix-a.md:1782-1783`; `docs/node-and-conformance-spec.md:1097`; `docs/deterministic-replay-and-merge-spec.md:1195` | Forced/non-qualifying boundary no-authority lists mention ordinary rate limits, POD, POINT, governance, or lifecycle but not admission capacity/maturation everywhere. | Target says Dmax/forced/degraded/survivor/record-only/machine-only boundaries generate no invitation capacity or maturation unless human-certified. | Add admission capacity and maturation to no-authority invariant lists and replay/node conformance. | Constitutional invariant / authoritative protocol semantics | Contradiction requiring normative edit | High | R2A | R2C | Yes |
| AD-021 | `docs/cycle-spec.md`; `docs/tempo-spec.md`; rulebook-facing docs | No exact qualifying capacity period, positive-capacity minimum, maturation, carryover, suspension restoration, or liveness integration is present outside the target. | Rulebooks could remain unable to determine capacity or liveness deterministically. | Define cycle/rulebook hooks while leaving numeric rates rulebook-controlled. | Rulebook-controlled policy | Missing normative definition | Blocker | AD-020 | R2C | Yes |
| AD-022 | `docs/api-contract-read-only.md`; `docs/privacy-and-high-risk-submission-spec.md`; DTO surfaces | Public capacity derivability is not propagated; omission could be mistaken for privacy. | Target says exact Profile-v0 capacity is publicly replay-derivable; DTO omission is presentation minimization only. | Clarify API/privacy language and future DTO expectations. | Derived status/documentation | Informative clarification only | Medium | AD-021 | R2D/R2E | No |
| AD-023 | `docs/protocol v5-appendix-a.md:953-966`; `docs/privacy-and-high-risk-submission-spec.md`; `docs/offline-and-mindseed-spec.md` | `verification_reference` is an interface-level pointer and privacy treatment is not constrained by Appendix A. | Target forbids private request packages, relay-local objects, contact records, raw private evidence, and mutable private records. | Narrow `verification_reference` to canonical artifacts or privacy-safe canonical commitments; choose one canonical no-reference encoding later. | Authoritative protocol semantics / exact-schema | Missing normative definition | High | AD-004, AD-008 | R2B/R2D | Yes |
| AD-024 | `docs/privacy-and-high-risk-submission-spec.md`; `docs/offline-and-mindseed-spec.md`; `docs/verification-spec.md` | Support for unconnected, high-risk, or pseudonymous applicants lacks a non-canonical admission request and stranger sponsorship model. | Target architecture distinguishes local prep and non-canonical request from canonical admission. | Define non-canonical request transport boundaries and privacy-safe sponsorship paths without making requests canonical. | Authoritative protocol semantics / future-profile boundary | Missing normative definition | High | AD-002, AD-023 | R2D | Yes for admission UX, not for core schema |
| AD-025 | `docs/offline-and-mindseed-spec.md:302-304`, `485`, `503` | Offline publication text requires verified-human identity for ordinary publication and optional speaker where schema permits. | Needs sponsor-authored admission and restricted verification/key-control lane exceptions; `identity_create` speaker must be absent. | Add offline admission/restricted-lane handling and speaker-specific constraints. | Authoritative protocol semantics | Contradiction requiring normative edit | High | AD-003, AD-007 | R2D | Yes |
| AD-026 | `docs/ai-boundaries-spec.md`; `docs/safety-spec.md`; `docs/governance-spec.md` | General AI/system boundary language exists, but admission-specific no-sponsor/no-minting constraints are not explicit everywhere. | Target forbids AI, operator, wall-clock, and machine-only emergency capacity minting in Profile v0. | Add cross-references where needed without redesigning AI/safety/governance. | Informative / constitutional boundary | Informative clarification only | Medium | AD-020, AD-021 | R2D | No |
| AD-027 | `docs/token-spec.md:1410`, `1804` | Token inheritance planning can route POINT to the identity that invited the deceased identity. | This gives sponsor lineage an economic consequence and may conflict with "invitation capacity is not money/reputation/economic authority." | Either remove inviter as economic fallback or classify as future profile requiring separate governance review. | Authoritative protocol semantics / future-profile policy | Contradiction requiring normative edit | Medium | Token/governance review | R2D | No for admission core, yes for final token consistency |
| AD-028 | Missing `docs/canonical-error-registry.md` or equivalent; `docs/protocol v5-appendix-a.md:2090-2130` | Error registry is Tempo-focused and lacks admission errors and precedence. | Deterministic validation cannot distinguish stale authorization, inactive key, ineligible inviter, suspension, capacity exhaustion, duplicate identity/key, malformed proof, and restricted-lane scope errors. | Add admission error definitions and precedence after schemas are settled. | Exact-schema/encoding requirement | Missing exact-schema/encoding requirement | Blocker | AD-004, AD-009, AD-021 | R2B/R2E | Yes |
| AD-029 | `docs/conformance/canonical-event-signature-profile-v0.vectors.json`; `docs/conformance/canonical-event-signature-profile-v0.md` | Existing vectors exercise Profile-v0 signatures but not sponsored identity admission, applicant proof, capacity, liveness, or structural roots. | Runtime cannot prove interoperability for the new admission protocol. | Add conformance vectors for proof bytes, sponsor signature, reduced auth context, no-reference encoding, speaker absence, key reuse, capacity debit, and liveness. | Implementation contract / exact-schema | Conformance gap | Blocker | R2B/R2C | R2E | Yes |
| AD-030 | `backend/crates/api-types-canonical/src/lib.rs`; `frontend/src/shared/types/canonical.ts`; `docs/api-contract-read-only.md` | Public DTOs expose minimal `IdentityInfo`, event-log fields, and `canonical_writer_level`, but not admission/key/capacity/liveness surfaces. | Target requires safe public read surfaces for identity roots, key history, admission provenance, eligibility lanes, capacity derivability, and liveness. | Extend DTO contracts after exact schema/replay reconciliation. | Implementation contract | Implementation/schema drift | High | AD-019, AD-021 | R2E | Yes |
| AD-031 | `docs/open-core-implementation-status.md` | Status accurately notes signed idea/connection substrate and bootstrap key rows, but does not yet capture completed admission spec, cross-doc blockers, and runtime non-readiness. | Implementation status could overstate readiness if not updated after reconciliation. | Update after normative reconciliation to state target architecture and remaining runtime blockers. | Derived status/documentation | Informative clarification only | Medium | R2A-R2D | R2E | No |
| AD-032 | `docs/open-core-boundary-manifest.md`; `docs/open-core-split-and-data-boundary-spec.md`; `docs/open-core-architecture-overview.md` | No direct Profile-v0 admission semantic contradiction found. | Future export/boundary checks must include admission code after implementation, but no immediate normative edit is required. | No change now; revisit during runtime/export implementation. | Implementation contract | No change required | Low | Runtime implementation | R2E | No |
| AD-033 | `docs/conformance/canonical-event-signature-profile-v0.vectors.json`; search term `speaker_identity_id` | Signature vectors cover ordinary authored candidates with optional speaker behavior, but not the identity-create no-speaker rule. | Identity admission implementations could incorrectly include applicant speaker. | Add exact no-speaker identity admission vectors. | Exact-schema/encoding requirement | Conformance gap | Medium | AD-007 | R2E | Yes |
| AD-034 | `docs/api-contract-read-only.md:842-853`; signed write route docs | Current public write API surface is signed idea/connection-focused and bootstrap-key-state-focused. | Identity admission route support, request/response DTOs, limits, and errors are not specified. | Add API contract after Appendix A and error registry are reconciled. | Implementation contract | Implementation/schema drift | High | AD-004, AD-028 | R2E | Yes |
| AD-035 | `docs/protocol v5-appendix-a.md`; `docs/deterministic-replay-and-merge-spec.md`; `docs/snapshot-format-v0.md`; migrations `0016`, `0019`, `0022` | Legacy/provisioned identities and writer/key rows lack a complete provenance classification such as `genesis_admitted`, `legacy_operator_provisioned`, `event_derived`, or `future_profile_derived`. | Legacy rows must not be rewritten as though Profile-v0 admission occurred. | Define compatibility provenance in schemas, replay, snapshots, and implementation status. | Compatibility/genesis/import behavior | Compatibility classification required | High | AD-004, AD-014 | R2B/R2C/R2E | Yes |
| AD-036 | Account/writer paths in `backend/crates/storage/src/canonical.rs`, `backend/bins/api-server/src/server/handlers/canonical.rs`, migrations `0016`, `0019` | Private-account-derived or operator-granted rows can influence canonical writer state. | Target separates private accounts from canonical identity and final writer eligibility. | Quarantine as transitional deployment state; require explicit migration and no public authority claim. | Compatibility/genesis/import behavior | Compatibility classification required | High | AD-014, AD-035 | R2E | Yes |

Finding totals:

- Total findings: 36.
- Severity totals: blocker 19; high 10; medium 6; low 1; informational 0.
- Change-category totals: ambiguous authority 1; contradiction requiring normative edit 7; missing exact-schema/encoding requirement 4; stale terminology 2; implementation/schema drift 6; compatibility classification required 4; missing normative definition 6; conformance gap 2; informative clarification only 3; no change required 1.

## 7. Detailed Findings Grouped by Semantic Area

### A. Admission Authorship and Lifecycle

Direct contradictions:

- `docs/protocol v5.md:3185` still describes a new user registering and emitting a user identity idea.
- `docs/protocol v5.md:181`, `205`, `3088`, and `3094-3098` define ordinary verified-human authorship in a way that needs a sponsor-authored admission and restricted-lane carveout.
- `docs/protocol v5-appendix-a.md:356` says only verified human agents may author canonical events except the system boundary emitter.
- Runtime-facing validation in `backend/crates/event-log/src/validation.rs:197-224` still validates identity creation as though the target identity is the speaker.
- Storage/API paths in `backend/crates/storage/src/canonical.rs:1209-1257` still create canonical identity material from account-coupled input.

Target reconciliation:

- Protocol root language should define permissionless local key and identity material preparation as non-canonical.
- A canonical Profile-v0 `identity_create` is sponsor-authored.
- Admission does not grant ordinary authority.
- Runtime-facing paths must later be either replaced or explicitly classified as private/legacy compatibility.

### B. Applicant Proof and Sponsor Signature

Missing exact definitions:

- Exact applicant possession-proof domain separator.
- Exact applicant proof signed bytes.
- Exact no-reference encoding for `verification_reference`.
- Reduced `admission_authorization_reference` encoding.
- Sponsor signature over the completed payload containing applicant proof.
- Rejection vectors for altered event ID, sponsor, key, authorization reference, verification reference, or recursive payload hash.

Target reconciliation:

- R2B should update Appendix A, canonical authorship, canonical encoding/hashing, event registry, and errors together.
- Runtime implementation should not begin until those exact-byte definitions and fixtures exist.

### C. Identity Kind and Initial Authority

Conflicts and gaps:

- Protocol root language still ties identity creation to registration and verified-human identity.
- Appendix A does not state fixed Profile-v0 target `identity_kind = human`.
- API/DTO surfaces do not distinguish admission, key control, VH, VI, and event-family eligibility lanes.

Target reconciliation:

- Profile-v0 `identity_create` creates a human-kind `CanonicalAdmittedIdentity`.
- Human kind does not establish VH, VI, uniqueness, writer/challenge/voter/governance/Tempo/inviter/economic authority.

### D. Structural Roots

Stale names and mechanics:

- Protocol v5 contains `BACKYARD`, `RELATIONSHIP_GARDEN`, `backyard`, and `relationship_garden`.
- Some protocol wording treats root creation as part of generic registration or verified-human creation.
- Appendix A lacks `identity_structural_roots` effect semantics.

Target reconciliation:

- Future-facing names: Mindgarden, Backyard of Relationships, Self Tree, Anthill.
- Exact root identifiers, encodings, derivation mechanics, and containment remain a structural-role reconciliation dependency.
- Identity creation must atomically create or deterministically derive all required roots.

### E. Verification Ontology

Conflicts and gaps:

- Appendix A still gives `identity_verification_update` direct verification and authorship-eligibility effects.
- Verification spec invite and Anthill language needs careful distinction from verification truth.
- `canonical_writer_level` is presented in multiple runtime/API places as stored status.

Target reconciliation:

- Verification artifacts are ordinary canonical truth/evidence/challenge objects under restricted authorization where needed.
- VH/VI certainty is replay-derived.
- Event-family eligibility lanes are replay-derived outputs.
- `identity_verification_update` is compatibility/genesis/import only, if retained at all.

### F. Restricted Verification Lane

Missing support:

- No exact event-family schema or authorization path for a `CanonicalAdmittedIdentity` to make self-verification claims, key-control claims, identity-continuity claims, self-specific evidence, or identity-scoped challenge responses before ordinary writer eligibility.
- API/DTO surfaces do not expose restricted-lane eligibility or outcomes.

Target reconciliation:

- Define restricted verification lane as constrained authorization into ordinary canonical epistemic objects.
- Explicitly reject ordinary idea creation, arbitrary connections, general challenges, voting, governance, Tempo claims, inviting, and economic actions through that lane.

### G. Invitation Eligibility and Capacity

Conflicts and gaps:

- Verification spec uses `invite_rate` and "account may issue" language.
- No exact rulebook interface currently binds `inviter_eligibility`, `invitation_suspension`, `invitation_capacity_balance`, positive capacity, and capacity debits.
- Reduced admission authorization context is absent from Appendix A.
- Public derivability of capacity is not propagated into API/privacy docs.

Target reconciliation:

- Rulebooks control rates, caps, carryover, and abuse reductions, but cannot assign zero capacity indefinitely to eligible unsuspended human inviters.
- One unit is debited atomically for each successful canonical `identity_create`.
- Authorization references are not reservations, bearer tokens, or transferable capacity units.
- Profile-v0 exact capacity is publicly replay-derivable.

### H. Cycles, Qualifying Periods, and Liveness

Conflicts and gaps:

- Cycle/replay/node docs do not consistently include invitation capacity and maturation in non-qualifying boundary no-authority rules.
- `admission_liveness_blocked` is not represented in replay, snapshot, API, DTO, or conformance definitions.
- There is no exact public read contract for whether a period qualifies, maturation advanced, capacity was generated, existing capacity remains spendable, or liveness is blocked.

Target reconciliation:

- Qualifying capacity periods require certified human-deliberative cycles.
- Dmax, forced, degraded, survivor, record-only, and machine-only boundaries generate no admission capacity or maturation unless separately certified as qualifying.
- Existing valid capacity remains spendable during stalls unless suspended, expired by pre-existing rule, frozen by emergency rule, or constitutionally restricted.

### I. Genesis and Legacy Compatibility

Compatibility gaps:

- Bootstrap/import code and migrations contain identities, keys, writer levels, and roots that were not created by Profile-v0 sponsored admission.
- There is no complete provenance classification across Appendix A, replay, snapshots, API, and implementation status.

Target reconciliation:

- Define provenance classes such as `genesis_admitted`, `legacy_operator_provisioned`, `event_derived`, and `future_profile_derived`.
- Do not fabricate sponsors, applicant proofs, capacity debits, lineage edges, or attestations for existing rows.

### J. Privacy and Public Derivability

Conflicts and gaps:

- `verification_reference` is not yet constrained in Appendix A and privacy/offline specs.
- Public capacity derivability is not consistently documented.
- Non-canonical admission request and stranger sponsorship flow is not represented in privacy/offline docs.

Target reconciliation:

- `verification_reference` may only reference canonical verification artifacts or privacy-safe canonical commitments.
- It must not point to private request packages, raw private evidence, relay-local objects, contact records, private account IDs, or mutable private records.
- DTO omission is presentation minimization, not cryptographic privacy.

### K. Errors and Deterministic Validation

Missing exact definitions:

- `stale_admission_authorization`
- `author_key_inactive`
- `author_key_revoked`
- `inviter_ineligible`
- `inviter_suspended`
- `insufficient_invitation_capacity`
- `identity_already_exists`
- `public_key_already_registered`
- invalid applicant possession proof
- malformed key descriptor
- incomplete identity structural roots
- non-human Profile-v0 target
- restricted-lane scope violation
- diagnostic liveness state where applicable

Target reconciliation:

- Add admission errors and precedence after Appendix A exact schemas are settled.
- Do not conflate stale authorization with inactive keys, ineligibility, suspension, capacity exhaustion, duplicate targets, or duplicate keys.

### L. API, DTO, Replay, Snapshot, and Conformance Drift

Drift surfaces:

- `backend/crates/api-types-canonical/src/lib.rs`
- `frontend/src/shared/types/canonical.ts`
- `docs/api-contract-read-only.md`
- `docs/snapshot-format-v0.md`
- `docs/deterministic-replay-and-merge-spec.md`
- `docs/conformance/canonical-event-signature-profile-v0.vectors.json`
- `backend/crates/event-log/src/validation.rs`
- `backend/crates/storage/src/canonical.rs`

Target reconciliation:

- Add schema/read surfaces for identity detail, structural roots, sponsor/admission provenance, active and historical key state, eligibility lanes, capacity balance, period qualification, liveness, and compatibility provenance.
- Add conformance vectors for identity admission, key lifecycle, reduced authorization, applicant proof, sponsor signature, capacity, liveness, and legacy classification.

## 8. Documents Requiring No Change

No direct Profile-v0 identity-admission semantic contradiction was found in these documents during this audit:

- `docs/open-core-boundary-manifest.md`
- `docs/open-core-split-and-data-boundary-spec.md`
- `docs/open-core-architecture-overview.md`

They should be revisited during runtime/export implementation to ensure the implemented admission code remains public open-core and private-account independent, but no immediate normative reconciliation edit is required.

## 9. Unavailable, Missing, Renamed, or Superseded Documents

Requested or expected documents not found at the searched paths:

- `docs/authoritative-stage-map.md`
- `docs/identity-key-lifecycle-spec.md`
- `docs/canonical-write-api-contract.md`
- `docs/canonical-error-registry.md`
- `docs/error-registry.md`
- `docs/rulebook-spec.md`

Current equivalents used instead:

- Authority hierarchy: `docs/authoritative-index.md`
- Event registry: `docs/protocol-event-registry.v1.md`
- API contract: `docs/api-contract-read-only.md`
- Exact authorship/signature: `docs/canonical-event-authorship-and-signature-profile-v0.md`
- Encoding/hashing: `docs/canonical-encoding-and-hashing-spec.md`
- Key lifecycle fragments: `docs/protocol v5-appendix-a.md` and the Profile-v0 authorship/signature spec
- Rulebook behavior fragments: verification, cycle, safety-rulebook, node/conformance, and protocol documents

## 10. Exact-Schema and Encoding Decision Register

Exact-schema or byte-level decisions still required before runtime:

1. Profile-v0 `identity_create` payload fields and canonical field order.
2. One canonical no-reference encoding for `verification_reference`.
3. Exact `admission_authorization_reference` byte layout and domain separator.
4. Exact applicant possession-proof byte layout and domain separator.
5. Exact sponsor signed-candidate payload hash inclusion after applicant proof insertion.
6. Exact `speaker_identity_id` absence encoding for `identity_create`.
7. Exact `identity_kind = human` schema representation without adding a free-form target-kind field.
8. Exact structural-root effect representation and whether roots are explicit objects or deterministic derivations.
9. Exact initial key descriptor and public key reference validation in identity admission.
10. Exact key rotation and revocation payloads, including add/replacement, supersession, last-key, and recovery constraints.
11. Exact admission error registry and precedence.
12. Exact legacy/genesis/import provenance fields.

## 11. Rulebook Decision Register

Rulebook-controlled policy decisions still required:

1. Numeric invitation capacity rates above the constitutional minimum of at least one unit.
2. Capacity caps and carryover limits.
3. Abuse reductions and suspension criteria.
4. Restoration procedures and activation boundaries.
5. Verification thresholds and formulas for `inviter_eligibility`.
6. Verification thresholds and formulas for ordinary writer, challenge, voter, governance, Tempo, attester, and other eligibility lanes.
7. Exact qualifying capacity period linkage to certified human-deliberative cycles.
8. Grace periods for stale authorization, if any.
9. Public read bucketing or display minimization, recognizing exact capacity remains replay-derivable.

## 12. Compatibility and Migration Decision Register

Compatibility decisions still required:

1. Provenance class names and storage for existing identities and keys.
2. How legacy operator-provisioned key rows remain readable without becoming Profile-v0 admission.
3. How `canonical_writer_level` is classified and retired.
4. How historical bootstrap roots are represented without fabricating applicant proofs or sponsors.
5. How historical `identity_verification_update` rows, if any, are classified.
6. How seed importer events are preserved as import/bootstrap records.
7. How snapshots distinguish event-derived admission from legacy/import state.

## 13. Runtime, Schema, and DTO Drift Register

Runtime/schema/DTO drift requiring later implementation work:

1. `backend/crates/event-log/src/validation.rs` self/speaker-based `identity_create` validation.
2. `backend/crates/storage/src/canonical.rs` account-coupled identity creation.
3. `backend/bins/api-server/src/server/handlers/canonical.rs` account-driven writer state handlers.
4. `backend/crates/replay/src/replay.rs` `canonical_writer_level` materialization.
5. `backend/crates/api-types-canonical/src/lib.rs` missing admission/key/capacity/liveness read DTOs.
6. `frontend/src/shared/types/canonical.ts` missing corresponding public DTO fields.
7. Migration `0022` signed substrate lacks admission-specific candidate/proof/capacity storage.
8. Migrations `0016` and `0019` writer verification tables need compatibility classification.
9. API contract lacks identity admission write/read/error definitions.
10. Snapshot format lacks admission liveness, capacity, key history, sponsor provenance, and compatibility provenance fields.

## 14. Conformance-Vector Gap Register

Required future conformance vectors:

1. Valid sponsor-authored Profile-v0 `identity_create`.
2. Applicant proof exact bytes and domain separator.
3. Applicant proof rejects altered event ID.
4. Applicant proof rejects altered target identity.
5. Applicant proof rejects altered initial key.
6. Applicant proof rejects altered sponsor identity.
7. Applicant proof rejects altered authorization reference.
8. Applicant proof rejects added, removed, or replaced `verification_reference`.
9. Canonical no-reference encoding accepted.
10. Non-canonical alternate no-reference encoding rejected.
11. Applicant proof does not include itself recursively.
12. Sponsor signature binds completed payload containing applicant proof.
13. Sponsor signature rejects payload mutation.
14. `speaker_identity_id` absent for `identity_create`.
15. Fixed Profile-v0 human target kind.
16. Non-human target rejected.
17. Complete structural roots required atomically.
18. Historical key reuse rejected.
19. Reduced authorization context exact bytes.
20. Wrong sponsor, period, or rulebook rejected.
21. Structurally valid context but sponsor currently ineligible rejected with inviter-specific error.
22. Structurally valid context but sponsor suspended rejected with suspension-specific error.
23. Structurally valid context but capacity exhausted rejected with capacity-specific error.
24. Stale period/rulebook returns stale authorization.
25. Exact retry does not debit capacity twice.
26. Successful admission debits one unit atomically.
27. Forced/Dmax/degraded/survivor/record-only/machine-only boundaries generate no capacity.
28. Qualifying capacity period generates at least one unit for unsuspended eligible inviters.
29. Existing capacity remains spendable during stalls unless explicitly frozen, suspended, or expired.
30. `admission_liveness_blocked` deterministic replay state.
31. Legacy/import identities remain readable but not reclassified as Profile-v0 admission.
32. Restricted verification lane accepts permitted self-verification artifacts and rejects ordinary idea/connection/challenge/vote/governance/Tempo/economic actions.

## 15. Dependency Graph

Dependency order:

1. Authority placement and Protocol v5 root semantics must be reconciled before exact schemas.
2. Appendix A, authorship/signature, encoding/hashing, event registry, key lifecycle, and error definitions depend on the root semantics.
3. Replay, verification, cycle/rulebook, Tempo, and snapshot definitions depend on exact event effects and derived state.
4. Privacy, offline, safety, governance, AI, and token documents depend on the semantic shape of admission, liveness, public capacity, and restricted lanes.
5. API, DTO, conformance, implementation status, boundary/export docs, and runtime implementation depend on all prior normative reconciliation.

Runtime identity-admission implementation remains blocked until at least R2A through R2C are complete and R2E has produced exact API/DTO/conformance contracts.

## 16. Controlled Edit Batches

### Batch R2A - Authority and Protocol Root Reconciliation

Scope:

- `docs/authoritative-index.md`
- missing or replacement stage-map entry
- `docs/cross-doc-invariants.md`
- `docs/protocol v5.md`
- navigation maps only where needed

Edits:

- Index the identity-admission spec and precedence.
- Replace self-registration language with local prep -> non-canonical request -> sponsor-authored `identity_create`.
- Normalize Profile-v0 human target, initial authority, and orthogonal lanes at root level.
- Normalize structural-root names at protocol root level.
- Add high-level invariants for sponsor-not-verification, no direct admission authority, positive capacity, qualifying periods, no machine-only minting, and public capacity derivability.

Stop boundary:

- Do not edit Appendix A exact schemas.
- Do not define exact byte encodings.
- Do not edit runtime, DTOs, migrations, fixtures, exports, or conformance vectors.

### Batch R2B - Exact Event, Authorship, Encoding, Key, and Error Reconciliation

Scope:

- `docs/protocol v5-appendix-a.md`
- `docs/canonical-event-authorship-and-signature-profile-v0.md`
- `docs/canonical-encoding-and-hashing-spec.md`
- `docs/protocol-event-registry.v1.md`
- key lifecycle home or Appendix A key lifecycle sections
- canonical error registry or Appendix A error section

Edits:

- Define exact Profile-v0 `identity_create` schema.
- Define applicant proof bytes and no-recursion rule.
- Define reduced admission authorization bytes.
- Define no-reference encoding dependency.
- Define speaker absence.
- Reconcile key rotation/revocation enough for runtime.
- Deprecate or classify `identity_verification_update`.
- Add admission errors and precedence.

### Batch R2C - Replay, Verification, Cycle, Rulebook, Tempo, and Snapshot Reconciliation

Scope:

- `docs/deterministic-replay-and-merge-spec.md`
- `docs/verification-spec.md`
- `docs/cycle-spec.md`
- `docs/tempo-spec.md`
- rulebook-facing specs
- `docs/snapshot-format-v0.md`

Edits:

- Add restricted verification lane.
- Reconcile ordinary truth/evidence/challenge ontology.
- Add inviter eligibility/capacity/liveness replay.
- Add qualifying capacity period and non-qualifying boundary effects.
- Add snapshot/replay state and digest extensions.
- Clarify Tempo remains disabled for admission alone.

### Batch R2D - Privacy, Offline, Safety, Governance, AI, and Token Boundary Reconciliation

Scope:

- `docs/privacy-and-high-risk-submission-spec.md`
- `docs/offline-and-mindseed-spec.md`
- `docs/safety-spec.md`
- `docs/safety-rulebook-interface-mechanics-spec.md`
- `docs/governance-spec.md`
- `docs/ai-boundaries-spec.md`
- `docs/token-spec.md`

Edits:

- Constrain `verification_reference`.
- Define non-canonical request privacy/relay boundaries.
- Clarify pseudonymous/high-risk admission limits.
- Add no AI/operator/machine minting cross-references.
- Review inviter economic/inheritance implications.

### Batch R2E - Node/API/DTO/Conformance/Status/Boundary Reconciliation

Scope:

- `docs/node-and-conformance-spec.md`
- `docs/api-contract-read-only.md`
- canonical/public DTO specs
- conformance vector docs and schemas
- `docs/open-core-implementation-status.md`
- boundary/export docs if needed

Edits:

- Add public read/write contract fields and errors.
- Add conformance vector requirements.
- Update implementation status and compatibility classifications.
- Prepare runtime task prerequisites without implementing runtime.

## 17. Recommended Next Task

Recommended task ID: **TEMPO-005D-S3-R2A**

Title: **Authority and Protocol Root Reconciliation for Profile-v0 Identity Admission**

Precise scope:

- Edit only the authority index/stage mapping, cross-doc invariants, Protocol v5, and navigation maps if needed.
- Establish the identity-admission specification as the Profile-v0 admission target.
- Replace root-level self-registration language with sponsor-authored admission.
- Normalize root-level structural names to Mindgarden, Backyard of Relationships, Self Tree, Anthill.
- Add root-level invariants for admission-not-verification, restricted initial authority, positive invitation capacity, qualifying periods, admission liveness, no machine-only capacity minting, and public derivability of capacity.
- Stop before Appendix A schemas, encoding, runtime, DTOs, conformance fixtures, databases, or exports.

## 18. Final Readiness Assessment

Direct answers:

- Which files directly contradict sponsor-authored admission? `docs/protocol v5.md`, `docs/protocol v5-appendix-a.md`, `backend/crates/event-log/src/validation.rs`, `backend/crates/storage/src/canonical.rs`, `backend/bins/api-server/src/server/tests_stage1_flow.rs`, and seed-importer/bootstrap surfaces unless classified as compatibility.
- Which files still imply direct registration or self-authored identity creation? `docs/protocol v5.md`, `docs/protocol v5-appendix-a.md`, `backend/crates/event-log/src/validation.rs`, `backend/crates/storage/src/canonical.rs`, `backend/bins/seed-importer/src/main.rs`.
- Which files treat sponsorship, lineage, or Anthill relationships as verification? `docs/verification-spec.md` and Appendix A verification-update language require reconciliation; Anthill and lineage must be downgraded to topology/provenance unless represented as ordinary claims/evidence.
- Where does `identity_verification_update` directly assign verification or authority? `docs/protocol v5-appendix-a.md:970-982`, with registry/status references requiring follow-up.
- Where is writer eligibility still represented as a mutable flag? `docs/protocol v5.md:205`, `docs/api-contract-read-only.md:674`, `backend/crates/replay/src/replay.rs`, `backend/bins/api-server/src/server/handlers/canonical.rs`, and migrations `0016`/`0019`.
- Where are structural-root names stale? `docs/protocol v5.md` contains `BACKYARD`, `RELATIONSHIP_GARDEN`, `backyard`, and `relationship_garden`; seed/import surfaces also need compatibility framing.
- Where do capacity and cycle rules conflict with positive capacity or `admission_liveness_blocked`? Cycle/replay/node docs omit admission capacity/maturation/liveness in non-qualifying-boundary rules and do not expose `admission_liveness_blocked`.
- Which exact definitions are missing? Appendix A identity schema, applicant proof bytes, reduced authorization bytes, no-reference encoding, speaker absence, exact key lifecycle, replay/snapshot/API fields, errors, and conformance vectors.
- Which changes are constitutional? Sponsor-authored admission, no self-registration as canonical publication, fixed human Profile-v0 admission, no admission-as-verification, no machine-only authority generation, public capacity derivability.
- Which changes are exact-schema? Appendix A payloads, encoding/hashing, applicant proof, authorization reference, errors, event registry, key lifecycle payloads.
- Which changes are rulebook-controlled? Numeric capacity rates, caps, carryover, abuse reductions, verification thresholds, restoration thresholds.
- Which changes are compatibility? Legacy identities, bootstrap/import keys, `canonical_writer_level`, seed-import identity/root generation, historical verification records.
- Which changes are implementation-only? Current Rust validators, storage, DTOs, tests, and migrations after normative reconciliation.
- Which changes are informative? API display minimization, architecture/status language, safety/AI cross-references where no rule changes.
- Smallest safe first reconciliation batch: R2A as defined above.
- Ready for runtime identity-admission implementation after this audit alone: no. Controlled reconciliation edits and validation must occur first.

Final readiness:

- Identity-admission target internally complete: yes.
- Ready for cross-document reconciliation: yes.
- Ready for runtime implementation: no.

Runtime implementation remains blocked by AD-001 through AD-017, AD-019, AD-021, AD-028, AD-029, AD-030, AD-034, AD-035, and AD-036.
