---
doc_id: protocol_event_registry_v1
title: Protocol Event Registry v1
status: derived-index
version: v1
last_reviewed: 2026-06-22

scope:
  - Indexes canonical, derived, non-canonical, fixture-only, and runtime event-like surfaces.

authoritative_for:
  - Implementation-facing event-name navigation.
  - Drift detection between specs, fixtures, and open-core validators.

not_authoritative_for:
  - Canonical event schema ownership.
  - Protocol semantics.
  - New event-type creation.

depends_on:
  - authoritative-index.md
  - protocol v5.md
  - protocol v5-appendix-a.md
  - canonical-event-authorship-and-signature-profile-v0.md
  - challenge-engine-spec.md
  - cycle-spec.md
  - tempo-spec.md
  - governance-spec.md
  - token-spec.md
  - deterministic-replay-and-merge-spec.md
  - node-and-conformance-spec.md
  - snapshot-format-v0.md
  - api-contract-read-only.md
  - offline-and-mindseed-spec.md
  - ai-boundaries-spec.md
  - privacy-and-high-risk-submission-spec.md
  - cross-doc-invariants.md
  - conformance/tempo-cycle-fixtures.v1.md
  - conformance/tempo-cycle-fixtures.v1.json

conflicts:
  - See "Stale Aliases, Naming Drift, And Open Conflicts".

change_rules:
  - Do not add canonical event types here. Patch Appendix A first.
  - Keep this registry aligned with Appendix A and conformance fixtures.

keywords:
  - protocol
  - event registry
  - Appendix A
  - canonical events
  - derived state
  - conformance
---

# Protocol Event Registry v1

## Purpose

This registry is a derived implementation-facing index. It does not replace Protocol v5 or Appendix
A. Its purpose is to prevent drift between:

- spec event names;
- Appendix A schemas;
- conformance fixtures;
- implementation validators;
- private, AI, and offline realm planning;
- fixture-only mutation attempts;
- UI, API, and runtime actions.

Authority model:

- Protocol v5 owns root semantics and constitutional invariants.
- Appendix A owns canonical event schemas and canonical payload validation.
- Subsystem specs own scoped semantics where they do not conflict with Protocol v5.
- This registry indexes, normalizes, and cross-references the event surface.
- If this registry conflicts with an authoritative spec, the authoritative spec wins and the conflict
  must be listed here.

Implementation files inspected for current open-core event names:

- `backend/crates/event-log/src/validation.rs`
- `backend/crates/replay/src/replay/apply.rs`
- `backend/crates/storage/src/canonical.rs`
- `backend/bins/seed-importer/src/main.rs`
- `scripts/tempo-cycle-fixture-harness.mjs`
- selected API/frontend types that expose event records.

## Classification Model

| Category | Meaning | Acceptance rule |
| --- | --- | --- |
| A. Canonical event | May enter the canonical event log, must be Appendix A-defined, may mutate canonical replay state. | Rejected if schema, Profile-v0 authorship signature, authority, publication, or replay validation fails. |
| B. Canonical system-boundary event | Canonical event emitted only by a permitted mechanical system boundary role. | Must be Appendix A-defined and authored by `system_boundary_emitter`; cannot become general system-authored idea creation. |
| C. Derived state / derived view | Replay output, materialized view, snapshot view, or API projection; not an event. | Cannot be authored or accepted as a canonical event. |
| D. Non-canonical realm event | Private map, AI map, local simulation, draft realm, model lens, or offline workspace event. | Must be realm-separated and cannot mutate canonical replay. |
| E. Fixture harness action | Test-driver action used to attempt forbidden mutations or expected failures. | Never valid canonical input. |
| F. Runtime/UI/local event | Frontend/backend operational action, click, route call, cache update, provider callback, or local process step. | Not canonical unless it produces a valid Appendix A canonical event. |

## Naming Policy

Canonical event names use Appendix A spelling. New implementations should use stable snake_case
names such as `idea_create`, not older mixed forms such as `idea_created`.

If an event-like name appears in specs but is not Appendix A-owned, it must be classified as one of:

- stale alias;
- derived state;
- non-canonical realm event;
- fixture harness action;
- runtime/UI/local action;
- unresolved schema drift.

Do not invent canonical event names from UI labels, API route names, fixture harness actions, or
derived-state field names.

## Minimal Idea-Only Deliberative-Content Rule

All canonical deliberative content is expressed as identity-authored ideas using existing base idea
types.

Terms such as evidence, argument, attestation, observation, testimony, source statement, potential
evidence, actual evidence, source-document claim, and source-chunk claim describe the role or use of
an idea, not additional canonical content-object types.

Canonical relationships are expressed through existing connection types and usages. Canonical state
changes are expressed through the unified idea, connection, challenge, vote, verdict, cycle, and
replay processes.

## Complete Canonical Event Registry

This table indexes the Appendix A catalog as currently written. It includes optional/interface-level
event types when Appendix A names them as event types, and marks aliases as aliases. It does not
make optional events mandatory.

Count in this table: 39 Appendix A catalog event names, counting split aliases such as
`identity_key_rotate` and `identity_key_revoke` separately.

| event_type | category | schema owner | semantic owner | allowed actor kind | creates/changes | canonical replay effect | required major payload fields | major rejection codes | fixtures/test coverage | implementation status | notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `identity_create` | A | Appendix A A4.1.1 | Protocol/Verification/Authorship profile | Human identity under bootstrap/rulebook rules | Identity object and initial key descriptor | Creates inactive identity until verified; registers initial replay-derived key state when descriptor validates | `identity_id`, `initial_public_key_ref`, `initial_public_key_descriptor`, `verification_reference` | A7, `missing_field`, `invalid_field`, `invalid_id`, signature-profile vector failures | Backend event-log tests; Profile-v0 conformance vectors required | Open-core validator supports older/current payload drift (`title` required in implementation); runtime Profile-v0 support not implemented | `initial_public_key_ref` must be hash32 of the descriptor defined by `canonical-event-authorship-and-signature-profile-v0.md`. Implementation still requires `speaker_identity_id == identity_id`; Appendix A no longer treats older signature-field names as canonical. |
| `identity_verification_update` | A | Appendix A A4.1.2 | Verification | Authorized verifier/governance rulebook | Verification status | Enables/disables canonical authorship eligibility | `identity_id`, `verification_status`, `verification_reference` | A7 | Not found | Appendix A only | No open-core validator branch found. |
| `identity_visibility_update` | A | Appendix A A4.1.3 | Verification/Privacy | Verified human or authorized privacy rulebook | Public visibility flags | Presentation only; no validation/governance weight | `identity_id`, `public_visibility_flags` | A7 | Not found | Appendix A only | Must not affect eligibility or ranking. |
| `identity_key_rotate` | A | Appendix A A4.1.4 | Protocol/Verification/Authorship profile | Identity owner under replay-derived key rules | Key validity | New key active at finalized canonical position | `identity_id`, `new_public_key_ref`, `new_public_key_descriptor`, `authorization_public_key_ref` | A7; Profile-v0 unknown/malformed/wrong-owner key failures | Profile-v0 rotation vector required | Appendix A only; runtime support not implemented | Rotation must be authorized by an active prior key or canonical recovery process. Does not retroactively authorize earlier events. |
| `identity_key_revoke` | A | Appendix A A4.1.4 | Protocol/Verification/Authorship profile | Identity owner under replay-derived key rules | Key validity | Target key inactive for future authored candidates at finalized canonical position | `identity_id`, `revoked_public_key_ref`, `authorization_public_key_ref`, conditional `recovery_process_ref` | A7; Profile-v0 revoked-key vector failures | Profile-v0 revocation vectors required | Appendix A only; runtime support not implemented | Revocation is non-retroactive and does not invalidate past finalized events. |
| `idea_create` | A | Appendix A A4.2.1 | Protocol v5 | Verified human canonical writer; narrow human `tempo_contributor` lane for valid Tempo context only | Idea | Creates idea object | `idea_id`, `idea_type`, `speaker_identity_id`, conditional `truth_subtype`, optional `initial_representation_refs`, conditional `tempo_claim`/`tempo_lane` | A7; Tempo codes such as `ERR_TEMPO_CLAIM_MISSING_METADATA` | Tempo fixtures; backend event-log/replay tests | Open-core validator and storage support | Time claims and evidence are ordinary ideas, usually `truth_claim`. |
| `idea_update_metadata` | A | Appendix A A4.2.2 | Protocol v5 | Authorized human/rulebook | Non-substantive metadata | Metadata-only update | `idea_id`, `metadata_patch` | A7 | Not found | Appendix A only | Must not change meaning, truth status, or importance. |
| `idea_update_representation` | A alias | Appendix A A4.2.3 | Protocol v5 | Verified human canonical writer | Candidate representation for idea | Same as `representation_create` with `target_kind=idea` | `representation_id`, `target_kind`, `target_object_id`, `tier_length`, `tier_complexity`, `payload_hash`, `author_identity_id` | A7 | Not found | Not supported as distinct branch in current validator | Deprecated compatibility alias; prefer `representation_create`. |
| `idea_deprecate` | A | Appendix A A4.2.4 | Protocol v5 | Authorized human/rulebook | Idea visibility/reason state | Marks idea deprecated without deletion | `idea_id`, `reason_representation_ref` | A7 | Not found | Appendix A only | History remains. |
| `idea_retract` | A | Appendix A A4.2.4 | Protocol v5 | Authorized human/rulebook | Idea visibility/reason state | Marks idea retracted without deletion | `idea_id`, `reason_representation_ref` | A7 | Not found | Appendix A only | Retraction remains challengeable. |
| `representation_create` | A | Appendix A A4.2.5 | Protocol v5 | Verified human canonical writer | Candidate representation object | Adds candidate representation; does not select it | `representation_id`, `target_kind`, `target_object_id`, `tier_length`, `tier_complexity`, `payload_hash`, `author_identity_id` | A7, `invalid_payload_hash`, `invalid_field` | Backend event-log tests | Open-core validator supports | Canonical pointer selection happens through `challenge_finalize_verdict`. |
| `rail_create` | A | Appendix A A4.2B.1 | Protocol v5 | Verified human canonical writer | Rail/vine object | Creates ordered rail | `rail_id`, `rail_kind`, `speaker_identity_id`, `item_idea_ids`, conditional `vine_type` | A7 | Backend event-log validation | Open-core validator supports | Does not create graph connections. |
| `rail_fork` | A | Appendix A A4.2B.2 | Protocol v5 | Verified human canonical writer | New rail from base rail | Creates full ordered replacement rail | `base_rail_id`, `rail_id`, `speaker_identity_id`, `item_idea_ids` | A7 | Backend event-log validation | Open-core validator supports | Derived rail history preserved. |
| `rail_update_representation` | A alias | Appendix A A4.2B.3 | Protocol v5 | Verified human canonical writer | Candidate representation for rail | Same as `representation_create` with `target_kind=rail` | `representation_id`, `target_kind`, `target_object_id`, `tier_length`, `tier_complexity`, `payload_hash`, `author_identity_id` | A7 | Backend event-log validation | Open-core validator supports as alias | Deprecated compatibility alias; prefer `representation_create`. |
| `connection_create` | A | Appendix A A4.3.1 | Protocol v5 | Verified human canonical writer; narrow Tempo lane for allowed Tempo-context connections | Connection/edge | Creates edge | `connection_id`, endpoints, `connection_type`; for `relative_importance`: `usage`, `axis`, `timeframe`, `scope` | A7; `ERR_TEMPO_EVIDENCE_CONNECTION_INVALID` | Tempo fixtures; backend tests | Open-core validator/storage support | Evidence uses `relative_importance` with `evidence_for` or `evidence_against`; `same_as` is a connection use, not an event. |
| `connection_update` | A | Appendix A A4.3.2 | Protocol v5 | Authorized human/rulebook | Connection metadata | Updates allowed metadata only | `connection_id`, update patch | A7 | Not found | Appendix A only | Must not silently alter past claims. |
| `connection_remove` | A | Appendix A A4.3.3 | Protocol v5 | Authorized human/rulebook | Connection tombstone | Marks edge removed/detached without deleting history | `connection_id`, reason metadata | A7 | Not found | Appendix A only | Used for withdrawal/detach semantics. |
| `same_as_resolution` | A | Appendix A A4.3.4 | Protocol v5 | Authorized human/rulebook | Equivalence representative | Establishes representative; does not delete merged ideas | Cluster/reference fields | A7 | Not found | Appendix A only | `same_as` itself is normally a `connection_create` connection. |
| `challenge_create` | A | Appendix A A4.4.1 | Challenge Engine | Ordinary challenge-eligible verified human | Challenge object | Creates draft challenge | `challenge_id`, `challenge_domain`, `framing_representation_ref`, conditional `subject_idea_ids`/`subject_rail_ids`, optional future `tempo_lane` | A7, duplicate challenge rules | Tempo fixtures; backend tests | Open-core validator/storage support | Evidence-placement and certainty-band challenges are `challenge_create` with scoped subtype/semantics, not separate event types. |
| `challenge_open_arguments` | A | Appendix A A4.4.2 | Challenge Engine | Authorized challenge process actor | Challenge lifecycle | Opens argument phase | `challenge_id` | A7 | Backend validation branch | Open-core validator supports | Replay semantics controlled by Challenge Engine. |
| `challenge_close_arguments` | A | Appendix A A4.4.3 | Challenge Engine | Authorized challenge process actor | Challenge lifecycle | Closes argument phase | `challenge_id` | A7 | Backend validation branch | Open-core validator supports |  |
| `challenge_open_voting` | A | Appendix A A4.4.4 | Challenge Engine | Authorized challenge process actor | Challenge lifecycle | Opens voting | `challenge_id`, `eligibility_pool_ref` | A7 | Backend validation branch, with implementation payload drift | Open-core validator currently validates only `challenge_id` | Appendix A and implementation payload requirements differ. |
| `challenge_close_voting` | A | Appendix A A4.4.5 | Challenge Engine | Authorized challenge process actor | Challenge lifecycle | Closes voting | `challenge_id` | A7 | Backend validation branch | Open-core validator supports |  |
| `challenge_finalize_verdict` | A | Appendix A A4.4.6 | Challenge Engine | Authorized finalizer under challenge rules | Verdict/final challenge state | Locks verdict; applies downstream deterministic effects | `challenge_id`, `verdict_id`; if governance: `decision_cycle_index`, `change_class`, `delay_policy_version`, `activation_cycle_index`; representation selection fields when applicable | A7, duplicate finalization/tally mismatch | Tempo fixtures; backend tests | Open-core validator/storage support | Certainty-band outcomes and representation pointer updates are applied here. |
| `challenge_cancel` | A | Appendix A A4.4.7 | Challenge Engine | Authorized challenge process actor | Challenge lifecycle | Ends challenge without verdict | `challenge_id`, `reason_representation_ref` | A7 | Backend validation branch | Open-core validator supports |  |
| `challenge_supersede` | A | Appendix A A4.4.7 | Challenge Engine | Authorized challenge process actor | Challenge lifecycle | Replaces challenge | `challenge_id`, `reason_representation_ref` | A7 | Backend validation branch | Open-core validator supports |  |
| `vote_cast` | A | Appendix A A10.1 | Challenge Engine | Eligible verified human voter | Vote record | Counts if accepted by replay voting rules | `challenge_id`, vote choice fields; current implementation also requires `vote_session_id` and `vote_choice` | A11, invalid vote rules | Tempo fixtures; backend tests | Open-core validator/storage support | Attestations are not votes. |
| `vote_commit` | A conditional | Appendix A A10.2 | Challenge Engine | Eligible verified human voter | Vote commitment | Enables later reveal; not a vote by itself | `challenge_id`, `commit_hash`, commit metadata | A11 | Challenge spec references; no implementation branch found | Appendix A only | Only for rulebooks using commit-reveal. |
| `vote_reveal` | A conditional | Appendix A A10.3 | Challenge Engine | Eligible verified human voter | Revealed vote | Counts if matching accepted commit | `challenge_id`, `vote_choice`, `nonce` | A11 | Challenge spec references; no implementation branch found | Appendix A only | Only for rulebooks using commit-reveal. |
| `pod_mint` | A optional/interface | Appendix A A4.7.1 | Token/POD | Authorized rulebook mechanism | POD accounting event if explicit | Creates explicit POD mint if profile uses event form | `identity_id`, `amount`, `source_event_id` | A7; frontier/backfill constraints | Not found | Appendix A interface only | Current Tempo/Cycle model treats POD outputs as provisional/pending/authorized/blocked derived outputs unless explicit event path is adopted. |
| `point_mint` | A optional/interface | Appendix A A4.7.2 | Token/POINT | Authorized rulebook mechanism | POINT accounting event if explicit | Creates explicit POINT mint if profile uses event form | `identity_id`, `amount`, `epoch_id` | A7; frontier/backfill constraints | Not found | Appendix A interface only | Must obey lagged authorization frontier. |
| `point_distribute` | A optional/interface | Appendix A A4.7.2 | Token/POINT | Authorized rulebook mechanism | POINT distribution if explicit | Distributes POINT if profile uses event form | `identity_id`, `amount`, `epoch_id` | A7; frontier/backfill constraints | Not found | Appendix A interface only | Must not backfill burst authority from forced/constrained cycles. |
| `payout_epoch_finalize` | A optional/interface | Appendix A A4.7.3 | Token | Authorized rulebook mechanism | Epoch accounting | Finalizes issuance for epoch | `epoch_id`, `snapshot_id` | A7 | Not found | Appendix A interface only | Must not use block height or publication volume as time authority. |
| `safety_classify` | A optional/interface | Appendix A A4.8.1 | Safety/Privacy | Authorized safety/governance actor | Safety classification | Records classification if explicit event path is enabled | `classification_id`, `target_id`, `rulebook_id`, `jurisdiction_lens`, `explanation_ref` | A7 | Not found | Appendix A interface only | Separate from canonical `blocked_submission` accountability records. |
| `safety_appeal` | A optional/interface | Appendix A A4.8.2 | Safety/Privacy | Eligible verified human/authorized actor | Appeal | Records appeal if explicit event path is enabled | `classification_id`, `appeal_representation_ref` | A7 | Not found | Appendix A interface only |  |
| `safety_override` | A optional/interface | Appendix A A4.8.3 | Safety/Governance | Governance-authorized actor | Safety override | Records override if explicit event path is enabled | `classification_id`, `governance_authorization_ref` | A7 | Not found | Appendix A interface only | Must not bypass governance invariants. |
| `blocked_submission` | A | Appendix A A4.8.4 | Safety/Node conformance | Verified human or authorized verifier identity under active safety rulebook | Safe accountability record for rejected payload | Records block metadata; no semantic contribution to truth, importance, authority, Tempo/Cycle, POD, or POINT | `submission_hash`, `blocked_reason_code`, `blocked_by_identity`, `safe_summary_ref`, `classifier_profile_ref`, `rulebook_ref`; optional `reference_event_id`, `wrongful_block_challenge_ref` | A7; safety rulebook rejection codes | Backend event-log/storage tests cover Appendix A metadata fields | Public validator/storage/API require Appendix A metadata fields | Resolved canonical. |
| `snapshot_commit` | B | Appendix A A4.9.1 | Snapshot/Replay | `system_boundary_emitter` only | Snapshot index/attestation | Replay no-op for semantic state; indexes derived snapshot artifact | `block_height`, `snapshot_hash`, `state_root_hash`, `title_sentence_payload_root`, `shared_map_commitment`, `last_event_id`, `event_count`, `active_rulebook_set_hash` | A7, invalid snapshot hash/rulebook hash | Backend validation; snapshot tooling references | Open-core validator supports | Snapshot artifact itself is derived, not an event. |
| `cycle_close` | B | Appendix A A4.9.3 | Cycle Specification | `system_boundary_emitter` only | Structural cycle boundary | Closes cycle structurally; does not certify authority | `cycle_index_closed`, `next_cycle_index`, `boundary_type`, `trigger`, `W_score`, `W_target`, Dmin/Dmax keys/bands, liveness fields, `tempo_profile_hash`, `authorization_frontier_before`, `derived_state_commitment`, `closure_boundary_ref`; legacy fields only in named compatibility paths | `ERR_CYCLE_CLOSE_*`, structural liveness rejection codes | Tempo fixtures; backend cycle tests | Public validator requires current payload; Stage 0/internal validation supports legacy payload | `dmax_structural_liveness_forced` is forced-only and structural-only. |

## Resolved Implementation-Local And Remaining Schema Surfaces

These names appear in specs or implementation as event-like surfaces, but they are not Protocol v5
canonical event types unless Appendix A explicitly lists them.

| name | status | where found | decision | remaining action |
| --- | --- | --- | --- | --- |
| `vote_session_open` | Resolved implementation-local / derived challenge state | Open-core event-log/replay/storage tests | Protocol v5 does not define this as canonical. Vote candidate/session state is derived from challenge lifecycle, eligibility, capacity, and rulebook timing unless a future Appendix A schema adopts this event. | Align runtime validators or add a future Appendix A schema deliberately. |
| `canonical_writer_grant` | Resolved Stage 0 implementation/bootstrap event | Open-core event-log/replay/storage tests | Not a Protocol v5 canonical public event. Protocol-level write eligibility is derived from verification/governance state and the narrow Tempo lane. | Keep documented as Stage 0 local compatibility or replace with Appendix A verification/governance events. |
| `canonical_writer_revoke` | Resolved Stage 0 implementation/bootstrap event | Open-core event-log/replay/storage tests | Same as `canonical_writer_grant`. | Same as grant. |
| `genesis` | Resolved immutable genesis data / implementation bootstrap marker | Open-core validator accepts; many specs reference replay from genesis | Genesis is a bootstrap data boundary, not an Appendix A canonical event type. | Runtime validators should not treat `genesis` as a general public canonical event unless Appendix A later defines a bootstrap schema. |
| `noop` | Resolved fixture/test or implementation-local marker | Open-core validator/replay tests | Not a Protocol v5 canonical event. Conformant canonical replay should not accept `noop` as a public canonical event. | Keep out of fixtures and public schemas except implementation tests. |
| `censorship_alert` | Resolved derived/API alert surface | Protocol v5 Section 9.5 | Not a canonical event. It is derived from censorship-pressure metrics and exposed as a read/API/navigation surface. | If later made canonical, add Appendix A schema explicitly. |
| identity status examples (`identity_suspend`, `identity_reinstate`, `identity_flag_possible_fraud`) | Interface-level examples | Appendix A A12 | A12 says rulebooks may include such events, but A4.1 does not enumerate exact schemas. | Add exact event schemas only if adopted. |

## Derived-State Registry

Derived surfaces cannot be authored and cannot enter the canonical event log.

| derived surface | derived from | semantic owner | can be authored? | can enter canonical event log? | snapshot/API exposure | conformance coverage | notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Tempo Dmin/Dmax target keys | Cycle index, anchor event, active Tempo profile | Tempo/Cycle | No | No | Future read API may expose active keys | Tempo fixtures | `tempo_target(cycle_index, dmin/dmax)` is a key/view, not an object. |
| `structural_dmax_liveness_predicate` | Valid Dmax target-bound truth claim plus blockers check | Tempo/Cycle | No | No | Future read API may expose status | Tempo fixtures | Dmax-only, forced-only, structural-only. |
| `cycle_age_ge_dmin` / `cycle_age_ge_dmax` predicates | Target-bound truth claims, current eligible-human stances, capped passive evidence, blockers, profile thresholds | Tempo/Cycle | No | No | Future read API may expose predicate status | Tempo fixtures | Structural readiness does not assign truth certainty or authorize consequential effects. |
| Tempo target certainty-band state | Ideas, `evidence_for`/`evidence_against`, placement challenges, certainty-band verdicts | Tempo/Challenge | No | No | Snapshot/API future derived state | Tempo fixtures | No stance-count truth certainty. |
| Tempo structural-support state | Current eligible-human support/oppose/none stances, profile-required human support/margins, capped passive evidence, contradiction blockers | Tempo/Cycle | No | No | Snapshot/API future derived state | Tempo fixtures | `T_allow` support is not truth certainty, beacon status, certification, or authority. |
| Tempo passive-evidence state | Profile-admitted canonically committed timestamp evidence, deterministic normalization/dedup/outlier/cap rules | Tempo/Cycle | No | No | Snapshot/API future derived state | Tempo fixtures | Passive evidence alone cannot cross `T_allow`, `T_beacon`, Dmin, Dmax, survivor Dmax, certification, or frontier. |
| Derived beacon state | High-certainty target state, diversity, independence, stability, contradiction checks | Tempo/Cycle | No | No | Future read API may expose summaries | Tempo fixtures | Beacon is not an idea or event. |
| Cycle certification state | Cycle boundary type plus required target beacon/certification status | Cycle/Tempo | No | No | Snapshot/API future derived state | Tempo fixtures | Forced boundary remains forced even if later certified. |
| Authorization frontier | Contiguous certified cycles, lag `K`, previous frontier | Cycle/Token/Governance | No | No | Snapshot/API future derived state | Tempo fixtures | Initial frontier is `-1`; monotonic and contiguous. |
| Tempo operating mode | Frontier coverage, constrained allowlist, repair availability, human participation, profile | Tempo/Cycle | No | No | Future read API may expose mode | Tempo fixtures | `normal`, `constrained`, `record_only`; `time_repair_priority` is a constrained substate/alias. |
| Provisional/pending/authorized/blocked output status | Source cycle, frontier, token/governance/lifecycle rulebooks | Token/Governance/Cycle | No | No | Future API/snapshot derived state | Tempo fixtures | Includes POD, POINT, governance activation, lifecycle, rank, ordinary mana, rate limits. |
| Universal/relative rank outputs | Canonical graph, challenge outcomes, replay profile | Protocol/Replay | No | No | Snapshot derived outputs; API read surfaces | Existing replay/snapshot docs | Rank outputs are excluded from `state_root_hash` where specified. |
| Replay snapshots | Canonical log prefix, active rulebooks, snapshot format | Snapshot/Replay | No, artifact is derived | Only `snapshot_commit` enters log | Snapshot artifacts and API surfaces | Snapshot tooling | Snapshot artifact is not a canonical event. |
| Derived publication blocks / block height | Canonical event ordering and packaging profile | Publication/Replay | No | No | API/snapshot address surface | Backend snapshot/replay | Block height is packaging/address only, not time or authority. |
| Shared map commitment | Snapshot state roots and payload roots | Snapshot/Shared map | No | No | Snapshot header/API | Snapshot tooling | Commitment verifies sameness; not an event. |
| Source-document/source-section/source-chunk rendered views | Identity-authored ideas and payload/provenance refs | Protocol/Privacy | No as derived view; source claims are ideas | No as view | UI/API may render | Tempo external-source fixtures | The source itself is not evidence unless represented by identity-authored ideas and connections. |
| Model lens outputs | AI/private realm state, non-canonical maps, prompts/runs | AI Boundaries | No canonical authorship | No | Realm-local only | Not canonical | Provider/model/run IDs are provenance, not canonical authorship. |
| Vote candidate/session derivation | Challenge state, eligibility, capacity, randomness/profile | Challenge Engine | No under Protocol v5 | No | Implementation currently logs `vote_session_open` as Stage 0 compatibility | Backend replay tests | Resolved as derived/protocol-noncanonical unless Appendix A later adopts `vote_session_open`. |
| Active rulebook set | Governance verdicts, completion claims, activation cycle rules | Governance/Replay | No direct event | No, except indexed in `snapshot_commit` | Snapshot/API derived state | Docs | No standalone rulebook activation event. |
| Canonical writer eligibility | Verification state, governance/open-core writer grant state | Verification/Node | No as Protocol v5 public event | No, except Stage 0 implementation compatibility | API/internal validation | Backend replay tests | `canonical_writer_grant/revoke` are Stage 0 implementation/bootstrap events, not Appendix A canonical events. |
| Censorship-pressure profile / `censorship_alert` surface | Blocked submissions, sanitization/encapsulation counts, jurisdictional filter impacts, classifier tension | Safety/Node conformance | No | No | API/read-only or snapshot metadata surface | Not yet fixture-covered | Resolved derived/API alert surface. |

## Forbidden / Removed / Non-Event Registry

| name | why it is not canonical | replacement / correct representation | where it appeared or why it is tempting | status |
| --- | --- | --- | --- | --- |
| `time_claim` as idea type | Would create a separate time-claim ontology. | `idea_create` with `idea_type=truth_claim` and conditional `tempo_claim` metadata. | Tempo/Cycle planning risk. | Forbidden. |
| `tempo_target` as idea type/event | Dmin/Dmax targets are derived replay keys. | `tempo_target(cycle_index, dmin/dmax)` referenced by truth claims. | UI target cards/questions may look object-like. | Forbidden. |
| `beacon` as idea type/event | Beacon is derived target-level status. | Derived beacon state from ordinary truth claims and challenge verdicts. | Certification UI may display beacon cards. | Forbidden. |
| `tempo_attestation_cast` | Removed specialized Tempo event; attestations are ideas in roles. | Ordinary ideas plus `connection_create` and challenge/verdict rails. | Prior planning/spec drafts and harness guard. | Removed; harness rejects if seen. |
| `human_stance_unit_score` | Reintroduced stance-count certainty. | Certainty-band challenge verdicts over ordinary evidence rails. | Old Tempo aggregation model. | Forbidden. |
| `latest_effective_stance` | Reintroduced non-idea attestation channel. | New identity-authored ideas and normal challenge history. | Old Tempo aggregation model; schema guard forbids. | Forbidden. |
| `support_score_by_identity` | Reintroduced automatic support-count truth certainty. | Explicit evidence placement and certainty-band verdicts for truth certainty; structural support uses one current stance per eligible human and cannot create certainty. | Old Tempo aggregation model; schema guard forbids. | Forbidden as truth certainty. |
| `profile_weight` | Would weight humans by profile/status. | Eligibility/diversity gates only; no human influence weighting. | Old aggregation drafts. | Forbidden. |
| AI canonical event | AI cannot author canonical events. | Verified human authors any adoption event and is accountable for it. | AI map/agent planning. | Forbidden. |
| External link as evidence event | Links/provenance do not speak canonically. | Identity-authored source/document/source-claim ideas plus connections and challenges. | External-source UX. | Forbidden. |
| Server/client/local timestamp as time authority | Trusted clocks are forbidden. | Identity-authored truth claims plus explicit evidence/challenge history. | Runtime metadata and API receipt times. | Forbidden. |
| Block height as time authority | Block height is packaging/address only. | Tempo claims and Dmin/Dmax predicates. | Snapshot/publication address surface. | Forbidden. |
| Fixture `harness_actions` | Test-driver mutation attempts are not canonical events. | Keep in fixture `harness_actions` only. | Tempo conformance fixtures. | Fixture-only. |
| Derived certification update as authored event | Certification is derived from beacon/target state. | Replay derives `cycle_certification_state.v1`. | Runtime optimization temptation. | Forbidden unless Appendix A later adds explicit schema, which would be a semantic change. |
| Authorization frontier mutation as authored event | Frontier is derived, contiguous, lagged, monotonic. | Replay derives `authorization_frontier_state.v1`. | Admin/debug tools. | Forbidden. |
| UI click/action as canonical event | UI interactions are local runtime actions. | Only valid Appendix A events can enter canonical log. | Frontend event handlers/routes. | Non-canonical. |
| `idea_created` | Stale alias, not Appendix A canonical. | `idea_create`. | Older Protocol v5 prose. | Resolved stale alias. |
| `connection_created` | Stale alias, not Appendix A canonical. | `connection_create`. | Older Protocol v5 prose. | Resolved stale alias. |
| `challenge_opened` | Stale alias, not Appendix A canonical. | `challenge_create` or `challenge_open_*` lifecycle events, depending meaning. | Older Protocol v5 prose. | Resolved stale alias. |
| `verdict_reached` | Stale alias, not Appendix A canonical. | `challenge_finalize_verdict`. | Older Protocol v5 prose. | Resolved stale alias. |
| `importance_update` | Not Appendix A-defined as event. | Importance changes through `connection_create`/`connection_update` and challenge verdicts. | Older Protocol v5 prose. | Resolved non-event. |
| `identity_created` | Stale alias, not Appendix A canonical. | `identity_create`. | Older Protocol v5 prose. | Resolved stale alias. |
| `identity_verified` | Stale alias, not Appendix A canonical. | `identity_verification_update`. | Older Protocol v5 prose. | Resolved stale alias. |
| `global_ulid` | Deprecated envelope field name. | `event_id`. | Older node/conformance wording. | Resolved deprecated alias. |
| `human_confirmation_proof` | Deprecated envelope field name; must not imply request/session auth. | `signature`. | Older node/conformance wording. | Resolved deprecated alias. |
| `signer_key_id` | Deprecated implementation-local key wording. | `public_key_ref`. | Older node/conformance wording. | Resolved deprecated alias. |
| `human_author_id` | Deprecated envelope field name. | `author_identity_id`. | Older node/conformance wording. | Resolved deprecated alias. |
| `completion_truth_claim` | Sounds event-like but is an idea role. | `idea_create` with `idea_type=truth_claim`, linked to governance/action context. | Governance completion flow. | Resolved non-event. |
| `snapshot_created` / `snapshot_create` | Deprecated legacy/debug alias. | `snapshot_commit` for canonical index; snapshot artifact is derived. | Protocol v5 and Appendix A. | Resolved deprecated/non-canonical for new implementations. |
| `snapshot_adopt` / `checkpoint` | Local/interface accelerator marker. | Verified snapshot artifact use; `snapshot_commit` for canonical index. | Appendix A A4.9.2. | Non-canonical/local. |
| `overlay_connection_create` / `overlay_connection_update` | Private/non-canonical overlay operation. | Realm-separated private/AI map event; canonical adoption requires human Appendix A event. | Deterministic replay/merge spec. | Non-canonical realm event. |
| `forced_seal` | Deprecated implementation compatibility payload field, not event. | `boundary_type=forced` plus `trigger` on `cycle_close`; current structural liveness trigger is `dmax_structural_liveness_forced`. | Current implementation and Appendix A compatibility field. | Resolved legacy field; new docs should use `boundary_type` and `trigger`. |

## Tempo-Specific Registry Notes

- Time claims are ordinary `truth_claim` ideas with `tempo_claim` metadata.
- Tempo evidence is ordinary identity-authored ideas connected with `connection_create` using
  `evidence_for`, `evidence_against`, or `same_as` where valid.
- Dmin/Dmax targets are derived keys, not ideas, objects, or events.
- Beacons are derived target-level certainty states, not authored objects.
- `structural_dmax_liveness_predicate` is a derived predicate.
- The only canonical event consumed for structural liveness is `cycle_close` with
  `trigger = dmax_structural_liveness_forced`.
- One-person Dmax structural liveness does not create ordinary truth certainty, beacon status,
  cycle certification, authorization-frontier movement, POD, POINT, governance, lifecycle, final
  rank, ordinary mana, ordinary rate-limit, or token authority.

## Non-Canonical Realm Event Notes

Private, AI, local, offline, and shared-map realms may use analogous operation names for local
planning, simulation, or UI workflows. They must be realm-separated.

Rules:

- AI maps may have local votes, verdicts, cycles, or model-lens outputs, but those do not enter
  canonical replay.
- Offline/local events become canonical only when included in finalized canonical publication order
  and only if each event is a valid Appendix A event.
- Canonical adoption of AI/private/offline material requires a new verified-human canonical event.
- Provider, model, run, route, cache, and payload IDs are provenance, not canonical authorship.
- Non-canonical realm operations such as `overlay_connection_create` and `overlay_connection_update`
  must not be confused with Appendix A `connection_create` / `connection_update`.

## Fixture Harness Action Registry

The Tempo/Cycle fixture harness separates canonical `input_events` from non-canonical
`harness_actions`.

| harness action | why not canonical | expected use |
| --- | --- | --- |
| `cycle_close` attempt in `harness_actions` | Test-driver attempt, not event-log input. | Verify Dmin shortcut and contradiction-block rejection. |
| `profile_or_frontier_mutation` | Direct profile/frontier mutation is forbidden. | Verify anti-collapse and no threshold shrink. |
| `retroactive_authority_validation` | Direct retroactive validation is forbidden. | Verify no forbidden-action backfill. |
| `downstream_authority_from_forced_cycles` | Forced cycles cannot authorize effects. | Verify POD/POINT/governance blocking. |
| `structural_dmax_liveness_authority_mutation` | Structural liveness cannot create authority. | Verify structural-only liveness. |
| `structural_dmax_liveness_beacon_or_certification_mutation` | Liveness cannot create beacon/certification. | Verify one-person non-authority. |
| `structural_dmax_liveness_threshold_shrink_mutation` | Collapse cannot shrink thresholds. | Verify anti-collapse. |
| `authorization_frontier_mutation` | Frontier is derived. | Verify contiguous/monotonic frontier. |
| `offline_import` | Local import action is advisory until canonical validation/publication. | Verify offline realm separation. |

## Validation And Drift Checks

Future validation should check:

- every Appendix A event appears in this registry;
- every registry entry classified as canonical points to Appendix A;
- every conformance fixture `input_events[*].event_type` is Appendix A canonical or explicitly
  marked unresolved for the fixture harness;
- every fixture `harness_actions[*].attempt_type` stays fixture-only;
- no forbidden event/object names reappear as accepted input;
- no spec uses stale event aliases without aliasing them to Appendix A names;
- no derived state is described as authored;
- no AI/private/offline event can be mistaken for canonical;
- all rejection codes referenced by fixtures are owned by Appendix A or marked TODO;
- implementation validators do not accept event names that Appendix A has not adopted, except in
  explicitly documented Stage 0 compatibility paths.

## Stale Aliases, Naming Drift, And Open Conflicts

Known drift:

- Protocol v5 Section 13.4 has been normalized to Appendix A names. Any remaining mentions of
  stale aliases in this registry are historical drift records, not accepted event names.
- `blocked_submission` is Appendix A-owned as a canonical safety accountability event. Public
  open-core validation requires the Appendix A metadata fields and replay treats it as metadata-only.
- `censorship_alert` is resolved as a derived/API alert surface, not a canonical event.
- Open-core public validation rejects `vote_session_open`, `canonical_writer_grant`, and
  `canonical_writer_revoke`; Stage 0/internal validation still accepts them as documented
  implementation-local compatibility surfaces.
- Open-core public validation rejects legacy `cycle_close` payload fields such as `cycle_index`,
  `closure_kind`, and `forced_seal`. Stage 0/internal replay and storage still use those legacy
  fields through a named compatibility path. New docs and public canonical inputs should use
  `cycle_index_closed`, `boundary_type`, and `trigger`.
- Current open-core `challenge_create` validator recognizes implementation challenge domains such
  as `truth_challenge`, while Tempo fixtures use `challenge_domain = truth` with
  `challenge_subtype`. That is fixture-scoped today and needs a schema convergence pass before
  production runtime use.

## Open Questions

1. Should identity status examples in Appendix A A12 become exact event schemas?
2. Which Appendix A optional/interface events (`pod_mint`, `point_mint`, `point_distribute`,
   `payout_epoch_finalize`, safety events) are intended for the current public runtime, if any?
3. Should Stage 0 implementation-local events (`vote_session_open`, `canonical_writer_grant`,
   `canonical_writer_revoke`, `genesis`, `noop`) be removed from public validators, kept only in
   internal/bootstrap paths, or adopted through future Appendix A patches?
4. Should this registry later be generated automatically from Appendix A schemas plus implementation
   validator introspection?

## Registry Maintenance Checklist

When adding or changing an event-like surface:

1. Patch Protocol v5 or the owning subsystem spec for semantics if needed.
2. Patch Appendix A for any canonical event schema.
3. Update this registry.
4. Add or update conformance fixtures.
5. Update implementation validators only after schema ownership is clear.
6. Run registry drift searches for stale names and forbidden object/event types.
