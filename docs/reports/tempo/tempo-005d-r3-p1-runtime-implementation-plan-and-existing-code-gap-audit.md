# TEMPO-005D-R3-P1 Profile-v0 Identity Admission Runtime Implementation Plan and Existing-Code Gap Audit

## 1. Scope and authority method

**Task:** TEMPO-005D-R3-P1
**Repository:** the-seed-in-my-mind-open-core
**Repository role:** authoritative public/open-core
**Status:** planning complete; no runtime implementation performed.

This is a read-only implementation audit. It applies the completed Profile-v0 admission
architecture without reopening it. The authority order used was: Protocol v5 constitutional
invariants; Appendix A exact schemas and errors; encoding and authorship/signature bytes;
deterministic replay; subsystem specifications; the scoped identity-admission specification;
and the planned API contract. Existing runtime code is evidence only.

The audit inspected the actual public Rust workspace, migrations, API server, static vectors,
frontend canonical types, importer, and test guards. No database was opened, no migration was
run, and no runtime, API, DTO, test, vector, or schema file was changed.

## 2. Controlling specification hash confirmation

docs/identity-admission-and-invitation-capacity-spec-v0.md SHA-256:

~~~text
DD28615FB10D80D9D38BC2FB989973788627784A56AEA911472B32E8D42F73B1
~~~

The required hash matched before and after this audit. The target remains internally complete
and is the controlling Profile-v0 admission architecture.

## 3. Files inspected

The audit inspected 58 implementation-facing files or document surfaces, grouped as follows:

- Authority and completed reconciliation: AGENTS.md, authoritative index, status, API contract,
  identity-admission specification, Appendix A, encoding, authorship, registry, replay,
  verification, cycle, Tempo, snapshot, node/conformance, and all five R3 reconciliation reports.
- Workspace and pure runtime: backend/Cargo.toml; the encoding, identity, event-log, verification,
  common, storage, replay, snapshot, and api-types-canonical crate manifests and sources named in
  this task.
- Persistence: migrations 0001, 0009, 0010, 0014, 0015, 0016, 0019, 0021, and 0022.
- Public interface: API router, canonical/public/auth handlers, server types/errors,
  signed-ingress and Stage-1 tests, canonical Rust DTOs, and frontend canonical types.
- Compatibility and tooling: seed importer, snapshot binaries, signature vectors, new
  identity-admission vector inventory/schema/guide, package scripts, and the disposable DB guard.

## 4. Current implementation map

| Surface | Current behavior | Profile-v0 classification |
| --- | --- | --- |
| event-log validation identity_create | Requires speaker and requires speaker_identity_id to equal identity_id. It accepts title, an optional bare key reference, and optional string verification reference. | Incompatible transitional validator: no sponsor, applicant proof, complete descriptor, roots, capacity, or no-speaker rule. |
| Storage::create_canonical_identity | Authenticated account operation writes self-speaker identity_create, inserts identities_s0, and sets accounts.canonical_identity_id. Metadata is required as verification reference. | Incompatible account-coupled canonical creation. Quarantine and later retirement required. |
| Storage::create_account_with_canonical_cluster | Test/private helper creates account, identity idea, identity row, five organizer ideas, and membership edges; uses Mind Garden and Saved Ideas. | Transitional private/account bootstrap helper, never Profile-v0 materializer. |
| POST /api/v1/canonical/events | Validates signed embedded-payload idea_create and connection_create only. A missing candidate speaker defaults to author. | Conformant only for the narrow existing signed-write substrate; rejects Profile-v0 identity/key events. |
| canonical_identity_key_states | Stores Profile-v0 key bytes, descriptor reference, is_active, and source position; rows are bootstrap/operator/test provisioned. | Reusable substrate but lacks event-derived registration, global non-reuse, superseded/revoked history, and rotation transitions. |
| canonical_writer_verification_states and grant/revoke | email_verified plus canonical_writer_level gates ordinary signed idea/connection writes; verifier roles include a bootstrap record. | Transitional compatibility authority, not VH/VI or final replay-derived eligibility. |
| ReplayDriver | Reads materialized Stage-0 tables. It validates identity events but treats identity_create as no-op; output is ideas, rails, connections, payloads, cycle, and Tempo only. | Incomplete: no admission, keys, lanes, capacity, liveness, or provenance. |
| Snapshot format and binaries | Stage-0 commits ideas, connections, rails; snapshot verifier supports Profile::Stage0 only. | Incomplete: needs versioned admission-aware profile/sections and verifier support. |
| Seed importer | Enforces self-speaker seed identity_create, creates/upserts roots and legacy cluster state, and writes bootstrap data. | Compatibility/import tooling only. Must not manufacture Profile-v0 history. |
| Public reads and DTOs | API v0 identity read is only ID/title; canonical DTOs expose event signature fields and legacy writer status. Frontend mirrors this narrow shape. | Incomplete planned public identity/admission projection. |
| identity_verification_update | No ordinary runtime event path is present. Existing verification writer grants are separate Stage-0 internal events. | Correctly absent from public ingress, but manifest-only compatibility ingestion/provenance remains unimplemented. |

The events table can carry absent speaker because speaker_identity_id is nullable. Current signed-ingress
storage converts a missing speaker to the author, then always writes a speaker. That must become
event-family-specific while preserving existing idea/connection behavior.

## 5. Target requirement-to-code map

| Requirement | Authority and vector IDs | Proposed owner and likely files | Storage/replay/API impact | Dependencies |
| --- | --- | --- | --- | --- |
| Sponsored identity_create, absent speaker, fixed human profile | Appendix A A4.1.1; IA-001--005, IA-008 | event-log schema.rs/validation.rs; storage types.rs/canonical.rs | Typed parse and per-event speaker rule; later candidate ingress accepts it. | Pure schema/proof foundation. |
| Authorization commitment and verification_reference | Encoding 7.5; Appendix A A4.1.1; IA-006--009 | encoding sources; verification signatures.rs | Exact bytes in payload; replay equality check. | Existing primitive codecs/BLAKE3. |
| Applicant proof and completed sponsor signature | Authorship profile 5.2; IA-010--013 | verification signatures.rs; event-log typed payload tests | Pure Ed25519 proof before storage; sponsor uses existing candidate signature. | Typed payload and fixed fixtures. |
| Initial descriptor, global non-reuse, active direct key | Appendix A A4.1.1/A4.1.4; IA-015, IA-025--028 | verification; storage canonical.rs and key queries; replay | Append-only registration/transition history; position-aware lookup. | Migration foundation and proofs. |
| Atomic sponsored admission | Appendix A A4.1.1; replay; IA-001, IA-014--019 | storage canonical.rs/types.rs, event-log | One transaction: event, identity, key, roots, lineage, one debit, or none. | Roots, capacity inputs, projections. |
| Four roots and three connections | Appendix A A4.1.1.3; IA-016--017 | storage canonical.rs; replay apply.rs; snapshot sources | Materialize Mindgarden, Backyard of Relationships, Self Tree, Anthill and containment edges. | One-to-many materialization migration. |
| Lineage and provenance class | Appendix A A4.1.1/A4.1.2; IA-029--030, IA-034 | new storage projection/query module; replay; snapshot | Preserve event-derived/genesis/legacy/future classes without invented facts. | Compatibility migration. |
| Rotation and limited revocation | Appendix A A4.1.4; IA-025--028 | event-log, verification, storage, replay, planned API types | Controlled identity author; atomic supersession; revoke only a superseded key. | Key transition state. |
| Replay-derived lanes and legacy writer gate | Replay and verification; IA-029--030, IA-035 | replay state.rs/apply.rs/replay.rs; storage reads | Add lanes; isolate canonical_writer_level as legacy input until later writer lifecycle. | Admission replay model. |
| Restricted verification lane | Verification 1.3B; IA-035 | event-log authorization, replay, later API | Self-specific verification/key/root operations only; no ordinary authority. | Exact event catalog and lane state. |
| Capacity, qualifying periods, liveness | Cycle/replay; IA-018--024, IA-031--033 | replay, storage, snapshot, current cycle seam | Position-source ledger inputs and derived public balance/explanation. | Admission transaction and certification state. |
| VH/VI/eligibility interface | Verification/replay; IA-035 | verification/replay | Typed derived interfaces only; no scoring or writer redesign. | Separate verification lifecycle owns formulas. |
| Admission-aware snapshots | Snapshot; IA-014--019, IA-031--034 | snapshot sources and snapshot binaries | Versioned profile/sections, deterministic order, committed state root. | Complete replay output. |
| Write/read API and DTOs | API contract; IA-001, IA-020--024, IA-029--035 | api-types-canonical, server router/handlers/types/errors, frontend type/client | Extend existing ingress; public identity admission reads at snapshot basis. | Storage/replay/snapshot state. |
| Static vector execution | IA-001--035 | verification/event-log/replay tests, API HTTP tests, possibly harness script | Execute bytes, transitions, migration, snapshot, and HTTP cases. | Each vertical slice. |

## 6. Incompatibility and drift register

| ID | Finding | Severity | Containment and owning slice |
| --- | --- | --- | --- |
| R3P1-01 | Self-speaker validator and account route create identities. | Blocker | Replace only with sponsor transaction; quarantine or retire old route after proof. |
| R3P1-02 | Signed ingress supports only idea/connection and defaults missing speaker to author. | Blocker | Add per-event speaker rule and typed identity payload before enabling events. |
| R3P1-03 | ideas.created_event_id is unique but one admission materializes four roots. | Blocker | Controlled constraint/replay one-to-many change. |
| R3P1-04 | connections.created_by_event_id is unique but one admission materializes three connections. | Blocker | Controlled constraint/replay one-to-many change. |
| R3P1-05 | identities_s0.title is non-null/case-insensitive unique but Profile-v0 payload has no identity title. | Blocker | Add title-free admission projection or nullable compatibility migration; never fabricate title. |
| R3P1-06 | Key rows have only is_active and unique source_event_id; rotation needs supersession plus activation at one event. | Blocker | Append-only key registration/transition history. |
| R3P1-07 | No global public-key non-reuse constraint. | High | Audit legacy duplicates, then enforce new registration uniqueness fail-closed. |
| R3P1-08 | Replay ignores admission and snapshot omits its state. | Blocker | Implement replay before snapshot/API reads. |
| R3P1-09 | canonical_writer_level and account sessions are operational authority. | High | Maintain explicit legacy quarantine; admission never consults them for target authority. |
| R3P1-10 | Importer/account cluster emits old names/five-root clusters/self-speaker history. | High | Compatibility classification only; do not rewrite history. |
| R3P1-11 | DTO/read APIs cannot explain keys, provenance, roots, lanes, capacity, or liveness. | High | Add after replay/snapshot projection. |
| R3P1-12 | Vectors are inventory only; no executable identity-admission fixture/harness exists. | High | Begin fixed-byte fixtures, then replay/DB/API groups. |

## 7. Data and migration strategy

Use forward-only projection additions and compatibility classification. Preserve existing identities,
account links, seed roots, writer rows, and key rows as readable legacy evidence.

1. **Identity projection:** retain identities_s0 for current foreign keys, but add an admission-aware
   identity projection keyed by identity_id with kind, provenance, creation position, profile, and
   compatibility manifest. Resolve title constraints without inventing a Profile-v0 title.
2. **Keys:** retain canonical_identity_key_states as bootstrap evidence. Add append-only global key
   registration and per-key transition rows for activation/supersession/revocation. This supports
   two state transitions at one rotation event without updating append-only data.
3. **Admission, roots, lineage:** add event-derived admission, root-role, and sponsor-lineage
   projections. Roots remain ordinary ideas and connections, with structural roles queryable.
4. **One-to-many event materialization:** replace current one-event/one-idea and one-event/one-
   connection uniqueness assumptions. Update database constraints and replay maps together.
5. **Capacity/liveness:** use source-positioned, append-only capacity entries or equivalent
   replay-verifiable inputs for generation, debit, expiry, freeze, suspension, restoration, and
   period explanation. Balance is derived, never client supplied.
6. **Eligibility:** new state rows are caches/materializations with source positions. Stored
   canonical_writer_level remains legacy compatibility, not final authority.

Migration controls:

- Audit existing key reference duplicates before a global unique constraint. Classify legacy
  conflicts; do not let a new constraint silently corrupt or rewrite old rows.
- Deploy code that reads old/new provenance before enabling new admission events.
- Relaxing source-event uniqueness is necessary and not a data rewrite. Replace it with correct
  indexes and full replay/materialization tests in the same controlled slice.
- An accepted canonical event cannot be rolled back after publication. Do not expose ingress until
  migration, replay, snapshot, and conformance coverage are ready.
- Never infer sponsors, proofs, capacity debits, lineage, verification claims, or eligibility history.

## 8. Proposed vertical implementation slices

### TEMPO-005D-R3-P2 - Pure Profile-v0 Admission Schema and Cryptographic Validation

- **Purpose:** typed exact payload decoding, absent-speaker rules, reduced authorization commitment,
  applicant/replacement proofs, and direct-key proof helpers without database or route support.
- **Likely files:** event-log schema.rs/validation.rs/lib.rs; verification signatures.rs; encoding
  sources; fixed public conformance fixtures and pure tests.
- **Acceptance:** IA-002--013 and IA-025 execute as pure tests; recursive proofs reject; existing
  idea/connection vectors remain unchanged.
- **Stop boundary:** no migration, storage, replay, route, DTO, or account change.

### TEMPO-005D-R3-P3 - Admission Persistence Foundation and Legacy Classification

- **Purpose:** minimal schema for canonical identities, provenance, key history, roots, lineage, and
  capacity inputs; classify old rows.
- **Likely files:** next Postgres migration; storage types.rs/canonical.rs/read.rs/lib.rs; migration tests.
- **Acceptance:** old data migrates unchanged/readable; one-to-many constraints corrected; no new
  public event type is accepted.
- **Stop boundary:** no public ingress or writer-gate redesign.

### TEMPO-005D-R3-P4 - Atomic Sponsor-Authored Identity Admission Transaction

- **Purpose:** extend signed candidates with application-position identity_create validation and
  atomic identity/key/root/lineage/capacity materialization.
- **Likely files:** storage canonical.rs/types.rs, event-log, isolated storage/API-server test helpers.
- **Acceptance:** IA-001 and IA-014--024; exact retry is idempotent; rejected candidates have no
  effect; signed idea/connection behavior stays passing.
- **Stop boundary:** no rotation/revocation, restricted verification catalog, or public read DTOs.

### TEMPO-005D-R3-P5 - Replay-Derived Admission and Direct-Key Lifecycle

- **Purpose:** replay authority for admission/provenance/roots/key history and direct rotation/revoke.
- **Likely files:** replay state.rs/apply.rs/parsing.rs/replay.rs/tests.rs, storage key queries,
  snapshot-facing replay types.
- **Acceptance:** IA-015--019 and IA-025--028; rebuild equals incremental materialization;
  historical signatures remain valid at original position.
- **Stop boundary:** no VH/VI scoring, writer-gate redesign, or external API expansion.

### TEMPO-005D-R3-P6 - Restricted Verification, Capacity Periods, and Admission Liveness

- **Purpose:** narrow self-specific lane, rulebook/replay interface, qualifying periods, positive
  capacity, suspension/restoration, and admission_liveness_blocked.
- **Likely files:** event-log, verification, replay, capacity projections, current cycle seam, tests.
- **Acceptance:** IA-031--033 and IA-035; non-qualifying boundaries never create authority; the
  restricted lane rejects ordinary writing/challenge/vote/governance/Tempo/invitation/economic acts.
- **Stop boundary:** no full VH/VI scoring, broader evidence/challenge system, or future profile.

### TEMPO-005D-R3-P7 - Admission-Aware Snapshot, Public Reads, and Signed Ingress Contract

- **Purpose:** versioned snapshot state and planned public ingress/read DTOs.
- **Likely files:** snapshot crate/binaries; api-types-canonical; API router/handlers/types/errors;
  frontend canonical types/client; DTO boundary checks.
- **Acceptance:** stable Appendix A errors, snapshot basis, root/key/provenance/lane/capacity/liveness
  reads; no private request/account/evidence exposure.
- **Stop boundary:** no private request transport or frontend key-management UI.

### TEMPO-005D-R3-P8 - Executable Conformance, Migration Compatibility, and Isolation Proof

- **Purpose:** execute IA-001--035 and prove compatibility/rebuild/HTTP behavior.
- **Likely files:** crate/API tests, static fixture/harness support, migration tests, verification docs
  only if needed.
- **Acceptance:** every vector executable; replay equality; legacy classification; boundary/DTO checks.
- **Stop boundary:** no private repository integration or product UI.

### TEMPO-005D-R3-P9 - Superseded Account-Path Containment

- **Purpose:** remove, disable, or hard-quarantine account-coupled identity creation and writer authority.
- **Likely files:** storage account methods, full API routes/handlers/tests, importer compatibility tests,
  status and migration-retirement notes.
- **Acceptance:** ordinary canonical admission has no account/session dependency; old rows readable by
  explicit provenance only.
- **Stop boundary:** no deletion of historical data or private product accounts.

## 9. Slice dependency graph

~~~text
P2 exact types/proofs
  -> P3 persistence and compatibility foundation
  -> P4 atomic identity_create
  -> P5 replay and direct keys
  -> P6 restricted lane/capacity/liveness
  -> P7 snapshots, public reads, and ingress exposure
  -> P8 conformance and migration proof
  -> P9 account-path containment
~~~

P3 and P4 share transaction/schema assumptions and must be reviewed in order. P5 precedes P7
because public reads and snapshots may only expose replay-derived state. P6 supplies the
admission-specific interfaces only; the complete VH/VI and writer-eligibility lifecycle remains
a later task.

## 10. Test and conformance strategy

| Vector group | Future executable home | Database requirement |
| --- | --- | --- |
| IA-002--005, IA-008 | event-log schema/validation unit tests | None |
| IA-006--013, IA-025 | verification exact-byte tests using fixed public fixtures and the existing signature-vector reader pattern | None |
| IA-014--019, IA-027 | replay pure state tests and storage transaction/rebuild equality tests | Replay none; storage disposable DB |
| IA-020--024, IA-026, IA-028 | storage validation and signed-ingress HTTP tests | Disposable DB |
| IA-029--030, IA-034 | migration compatibility and replay provenance tests | Disposable DB for migration; pure fixture where practical |
| IA-031--033 | replay/cycle pure fixtures and snapshot consistency tests | None initially; DB for materialized checks |
| IA-035 | event-family authorization/replay unit tests and API denial tests | Unit none; API disposable DB |

Every future DB-backed command must set its URL in the same process and call
common::test_db_guard::require_disposable_database_url before opening a pool. The guard rejects
seed_dev, seed_open_core, postgres, and template databases. New admission test database prefixes
must be added deliberately. seed_dev is never a test target. DB proof must include protected-count
before/after checks and external-writer observation, as established by TEMPO-005C-R2.

## 11. API/DTO and product-boundary strategy

- POST /api/v1/canonical/events remains the only canonical write ingress. It accepts complete
  sponsor-authored candidates, never applicant requests, account registration, client capacity,
  client eligibility, writer levels, or private evidence.
- Preserve current API v0 viewer flows. Add versioned identity reads only after replay/snapshot support.
- Implement canonical Rust DTOs, then mirror frontend shared types and client surfaces. Field labels
  distinguish replay-derived, historical, optional presentation, and excluded private data.
- Non-canonical applicant transport remains outside the canonical API and has no persistence or
  authority requirement here.
- Do not add account/session state, private evidence, relay records, or private modules to the
  canonical dependency path. Existing account-linked routes remain quarantined through P9.

## 12. Compatibility and bootstrap strategy

Treat existing root clusters, identities_s0 rows, bootstrap key rows, seed importer data, and
operator-provisioned writer states as readable compatibility evidence. Classify source-backed data
as genesis_admitted, legacy_operator_provisioned, or future_profile_derived. Only a valid new event
may be event_derived.

Do not fabricate Profile-v0 sponsors, proofs, authorization references, capacity debits, lineage,
keys, structural role IDs, verification claims, or eligibility history. Legacy roots retain historical
titles/memberships. The new materializer alone creates the exact four-root plan.

## 13. Risks and decisions

### Settled and not to reopen

- Sponsor-authored identity_create, absent speaker, fixed human target, and applicant proof binding.
- One active direct key; rotation supersedes; revocation targets superseded keys; last active revoke
  is forbidden.
- Four explicit roots and three containment connections, atomically materialized.
- Admission grants neither verification nor ordinary writing, voting, governance, Tempo, capacity,
  or economics.
- Capacity is replay-derived/public; accounts, operators, AI, clocks, and machines cannot substitute
  for canonical authority.

### Implementation risks requiring controls

- Source-event uniqueness constraints and replay maps must change together.
- Legacy key duplication can block global uniqueness; audit/classify before enforcement.
- Account and writer-state paths can become a second admission model; do not route new events through them.
- Snapshot Stage0 must become versioned committed Profile-v0 state, not a side table.
- Full verification/writer lifecycle remains later; admission code cannot invent formulas or thresholds.

### Owner decisions currently required

None before P2. P3 needs a narrow implementation review for preserving legacy identities_s0 title
reads while Profile-v0 refuses to fabricate an identity title. That is storage compatibility work,
not a protocol choice.

## 14. Recommended first runtime implementation task

**TEMPO-005D-R3-P2 - Pure Profile-v0 Admission Schema and Cryptographic Validation.**

This is the smallest safe code-change slice. It has no database, migration, account, route, or
replay side effect. It can establish typed payload parsing, optional-reference encoding,
applicant/replacement proof verification, direct-key proof helpers, no-speaker enforcement, and
fixed vectors while preserving the existing signed idea_create/connection_create substrate. It
must stop before persistence or public event enablement.

## 15. Readiness assessment

- Cross-document reconciliation: complete.
- Runtime implementation plan: complete.
- Ready for the first controlled runtime implementation slice: **yes**.
- Profile-v0 runtime implementation complete: **no**. This task implemented no Profile-v0 event,
  migration, replay state, snapshot, API projection, or executable admission vector.
