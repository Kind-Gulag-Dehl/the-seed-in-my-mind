---
doc_id: tempo_cycle_fixtures_v1
title: Tempo/Cycle Conformance Fixtures v1
status: conformance-fixture
version: v1
last_reviewed: 2026-06-22

scope:
  - Defines replay fixtures for the idea-only Tempo/Cycle model.

authoritative_for:
  - Test-vector intent for future conformance harnesses.

not_authoritative_for:
  - Production Tempo profile values.
  - Runtime implementation behavior beyond the referenced authoritative specifications.

depends_on:
  - protocol v5.md
  - protocol v5-appendix-a.md
  - tempo-spec.md
  - cycle-spec.md
  - challenge-engine-spec.md
  - deterministic-replay-and-merge-spec.md
  - node-and-conformance-spec.md
  - cross-doc-invariants.md

conflicts:
  - none known

change_rules:
  - Keep this document and docs/conformance/tempo-cycle-fixtures.v1.json aligned.

keywords:
  - conformance
  - fixture
  - Tempo
  - Cycle
  - replay
  - Dmin
  - Dmax
  - certainty bands
---

# Tempo/Cycle Conformance Fixtures v1

This document summarizes the machine-readable fixtures in
`docs/conformance/tempo-cycle-fixtures.v1.json`.

These are specification fixtures only. They do not claim runtime implementation exists.

## Fixture Replay Harness

Run the conformance scaffold from the repository root:

```powershell
npm run conformance
```

To run only this fixture harness:

```powershell
npm run conformance:tempo-cycle
```

The harness validates the JSON fixtures against
`docs/conformance/tempo-cycle-fixtures.schema.v1.json`, replays canonical `input_events`, treats
`harness_actions` as non-canonical forbidden mutation attempts, and checks expected acceptance,
rejection codes, derived Tempo/Cycle state, cycle outputs, and authorization outputs. It is a
fixture replay scaffold, not the production Tempo runtime.

## Shared Setup

All fixtures share the JSON top-level `tempo_profile`, `identities`, `genesis`, and `shared_setup`
objects unless a fixture overrides a local value in `initial_state`. `shared_setup` defines the
derived cycle-0 Dmin/Dmax target views only; those targets are replay keys, not canonical input
events or authored objects.

Each fixture uses:

- `input_events` for Appendix A canonical event families only;
- `harness_actions` for non-canonical mutation attempts, offline imports, direct derived-state
  mutations, or other test-driver actions;
- `expected_rejections` for both rejected canonical input events and rejected harness actions.

## Model Under Test

- Time claims are ordinary `truth_claim` ideas with conditional `tempo_claim` metadata.
- Evidence, observations, testimony, source statements, arguments, and attestations are ideas in
  roles, usually `truth_claim` ideas.
- Evidence ideas connect to target claims through existing `relative_importance` usages:
  `evidence_for` and `evidence_against`.
- `same_as` may organize equivalent claims but does not erase authorship or grant authority.
- Dmin/Dmax targets are derived replay keys, not ideas or events.
- Beacon status is derived status of ordinary time truth claims, not an authored object.
- Certainty is assigned through evidence-placement and certainty-band challenges.
- `T_allow` is a derived structural-support threshold, not an ordinary certainty band.
- Weak passive timestamp evidence is admissible only when canonically committed, normalized,
  deduplicated, capped below `T_allow`, and combined with profile-required human support.
- Nodes must not infer certainty from counts, hidden weights, model scores, timestamps, block height,
  external links alone, or heuristics.
- Forced cycles are structural only and never authorize POD, POINT, governance, ordinary mana, or
  rate-limit effects.
- Zero participating eligible humans enter record-only posture and cannot emit universal
  `cycle_close`.
- Population collapse never shrinks `K`, `T_beacon`, diversity, independence, or stability
  requirements automatically.

## Test-Only Profile

The JSON profile uses small test values:

| Field | Value |
| --- | --- |
| `Dmin` | `2` |
| `Dmax` | `5` |
| `K` | `2` |
| `T_allow` | `2` structural-support units |
| `required_human_support_dmin` | `1` |
| `required_human_margin_dmin` | `1` |
| `required_human_support_dmax` | `1` |
| `required_human_margin_dmax` | `1` |
| `survivor_dmax_min_human_support` | `1` |
| `structural_support_unit_per_human` | `1` |
| `passive_evidence_cap` | `1` |
| `passive_source_dedup_policy` | `source_id_target_key_source_epoch` |
| `passive_source_class_policy` | `shared_upstream_source_class_capped_together` |
| `passive_outlier_policy` | `test_profile_interval_outlier_rejection_v1` |
| `contradiction_block_band` | `medium` |
| `beacon_minimum_certainty_band` | `high` |
| `beacon_minimum_distinct_humans` | `2` |
| `beacon_challenge_survival_cycles` | `1` |
| `authorization_lag_k` | `2` |
| `T_contradiction_block` | `medium` |
| `T_beacon` | `high` |
| `T_beacon_revoke` | `medium` |
| `minimum_beacon_identities` | `2` |
| `minimum_independence_domains` | `2` |
| `beacon_stability_cycles` | `1` |
| `tempo_mana_cap` | `3` |
| `tempo_mana_recharge` | `2` |
| `time_claim_create_cost` | `1` |
| `tempo_evidence_claim_create_cost` | `1` |
| `tempo_evidence_connection_cost` | `1` |
| `tempo_same_as_connection_cost` | `1` |
| `time_challenge_cost` | `2` |
| `structural_dmax_liveness_rule` | `single_valid_dmax_truth_claim_no_blockers_for_forced_close_only` |

## Fixture Index

| Fixture ID | Scenario | Key invariant |
| --- | --- | --- |
| `TCF-001-valid-target-bound-time-truth-claim` | Valid target-bound time truth claim | Time claims remain ordinary ideas |
| `TCF-002-rejected-unrelated-idea-through-tempo-lane` | Rejected unrelated idea through narrow Tempo lane | Tempo lane is not general write authority |
| `TCF-003-two-equivalent-target-bound-time-claims` | Two identities create equivalent target-bound time claims | Equivalent claims preserve authorship |
| `TCF-004-contradictory-target-bound-time-claims` | Contradictory target-bound time claims | Contradiction remains visible |
| `TCF-005-potential-evidence-spectrum-ideas` | Potential evidence spectrum consists entirely of ideas | Potential evidence is not a separate object type |
| `TCF-006-actual-evidence-ideas-only` | Actual evidence consists entirely of ideas | Evidence requires authored ideas |
| `TCF-007-evidence-connection-validation` | Evidence_for and evidence_against connection validation | Evidential role uses existing connections |
| `TCF-008-placement-challenge-orders-actual-evidence` | Placement challenge orders actual evidence against potential evidence | Evidence placement is explicit verdict history |
| `TCF-009-certainty-band-does-not-assign-tallow` | Certainty-band challenge does not assign T_allow | Truth certainty and structural support remain separate |
| `TCF-010-high-certainty-diverse-beacon-status` | Higher certainty plus diversity and stability gives beacon status | Beacon status is derived and diverse |
| `TCF-011-one-person-structural-liveness-no-beacon` | One-person structural liveness without one-person beacon authority | One person cannot unlock authority |
| `TCF-021-one-person-dmin-deliberative-shortcut-rejected` | One-person Dmin deliberative shortcut rejected | Survivor liveness is Dmax-only |
| `TCF-022-contradiction-blocks-one-person-dmax-liveness` | Contradiction blocks one-person Dmax liveness | Replay does not silently choose claims |
| `TCF-023-survivor-path-cannot-certify-beacon` | Survivor path cannot certify beacon | One person cannot satisfy beacon diversity |
| `TCF-024-later-normal-certification-finalizes-pending-only` | Later normal certification finalizes pending outputs only | Certification finalizes pending outputs only |
| `TCF-012-dmin-deliberative-close` | Dmin deliberative close | Structural close is not certification |
| `TCF-013-dmax-forced-close` | Dmax forced close | Forced close is liveness only |
| `TCF-014-forced-cycles-never-authorize` | Forced cycles never authorize POD, POINT, or governance | No POD/POINT/governance extraction |
| `TCF-015-certainty-contradiction-blocks-predicate` | Certainty contradiction blocks predicate | Certain contradiction blocks `T_allow` |
| `TCF-016-certification-gap-blocks-frontier` | Certification gap blocks frontier | Frontier is contiguous |
| `TCF-017-external-link-alone-no-certainty` | External link alone has no certainty effect | Provenance alone has no certainty effect |
| `TCF-018-source-document-and-chunk-ideas-support-after-placement` | Source-document/source-chunk ideas may support after connection and placement | External-source assertions are ideas |
| `TCF-019-ai-created-canonical-evidence-time-claims-rejected` | AI-created canonical evidence and time claims are rejected | AI cannot author canonical Tempo content |
| `TCF-020-offline-local-ideas-advisory-until-validation` | Offline/local ideas remain advisory until canonical validation | Local simulations do not advance canon |
| `TCF-025-rejection-matrix-tempo-cycle-validation` | Tempo/Cycle rejection matrix | Remaining rejection surfaces are covered |
| `TCF-026-passive-evidence-alone-cannot-cross-tallow` | Passive evidence alone cannot cross Tallow | Passive evidence cannot replace humans |
| `TCF-027-passive-plus-human-support-can-reach-tallow` | Passive evidence plus human support can reach Tallow | Passive evidence may assist required human support |
| `TCF-028-passive-source-dedup-prevents-amplification` | Passive source dedup prevents amplification | Repeated source observations count once |
| `TCF-029-zero-human-record-only` | Zero humans enters record-only | Machine-only operation cannot close cycles |

## Fixture Summaries

### TCF-001-valid-target-bound-time-truth-claim

- Event sequence: `idea_create` creates a `truth_claim` with valid `tempo_claim` metadata.
- Expected replay result: accepted as an ordinary idea; certainty remains `none`; no authority.
- Rejection codes: none.
- Protects: time claims do not become a new idea type.

### TCF-002-rejected-unrelated-idea-through-tempo-lane

- Event sequence: a Tempo contributor tries to create an unrelated truth claim and a conceptual idea
  through the Tempo lane.
- Expected replay result: both rejected; no Tempo mana spent.
- Rejection codes: `ERR_TEMPO_CLAIM_MISSING_METADATA`, `ERR_TEMPO_CLAIM_NOT_TRUTH_CLAIM`.
- Protects: the low-threshold lane is narrow.

### TCF-003-two-equivalent-target-bound-time-claims

- Event sequence: two humans create equivalent target-bound time claims and a `same_as` connection.
- Expected replay result: equivalent claims share a target key; separate authorship remains visible;
  two human stances reach structural readiness without changing ordinary truth certainty.
- Rejection codes: none.
- Protects: equivalence organization does not erase authorship, and structural support does not
  create truth certainty.

### TCF-004-contradictory-target-bound-time-claims

- Event sequence: one claim says Dmin elapsed; another ordinary truth claim says it did not.
- Expected replay result: contradiction is visible; no predicate without certainty verdicts.
- Rejection codes: none.
- Protects: contradictions remain in the map.

### TCF-005-potential-evidence-spectrum-ideas

- Event sequence: potential evidence spectrum entries are authored as truth-claim ideas.
- Expected replay result: they define candidate evidence positions only.
- Rejection codes: none.
- Protects: potential evidence is ideas, not a separate object.

### TCF-006-actual-evidence-ideas-only

- Event sequence: a target-bound claim and a Tempo-context evidence truth claim are created.
- Expected replay result: evidence is an idea; without a connection and challenge outcome it has no certainty effect.
- Rejection codes: none.
- Protects: actual evidence requires authored ideas.

### TCF-007-evidence-connection-validation

- Event sequence: `connection_create` links evidence ideas with `evidence_for` and `evidence_against`.
- Expected replay result: connections define evidential role; they do not assign certainty by themselves.
- Rejection codes: none.
- Protects: evidence placement uses existing connection surfaces.

### TCF-008-placement-challenge-orders-actual-evidence

- Event sequence: `challenge_create`, `vote_cast`, and `challenge_finalize_verdict` place evidence against the potential-evidence spectrum.
- Expected replay result: placement is recorded; certainty band is still unassigned.
- Rejection codes: none.
- Protects: evidence placement is explicit, not heuristic.

### TCF-009-certainty-band-does-not-assign-tallow

- Event sequence: a certainty-band truth challenge assigns `low` certainty to a Dmin claim.
- Expected replay result: ordinary truth certainty changes to `low`, but `cycle_age_ge_dmin`
  remains false because `T_allow` is structural support rather than a certainty band.
- Rejection codes: none.
- Protects: truth certainty and structural support are separate replay outputs.

### TCF-010-high-certainty-diverse-beacon-status

- Event sequence: source/evidence ideas, an evidence connection, and a high certainty verdict combine with two qualified identities and stability.
- Expected replay result: derived beacon status becomes elevated; representative claim is display-only.
- Rejection codes: none.
- Protects: beacon status is derived and requires diversity.

### TCF-011-one-person-structural-liveness-no-beacon

- Event sequence: one surviving human creates a valid Dmax target-bound truth claim and the system boundary emitter emits `cycle_close` with `trigger = dmax_structural_liveness_forced`.
- Expected replay result: `structural_dmax_liveness_predicate = true`; the boundary is forced; no ordinary Dmax certainty, beacon, certification, frontier advancement, or authority is created.
- Rejection codes: none.
- Protects: structural liveness is Dmax-only and authority-free.

### TCF-021-one-person-dmin-deliberative-shortcut-rejected

- Event sequence: one surviving human creates a Dmin target-bound truth claim, then a harness-only mutation attempts deliberative closure through the survivor path.
- Expected replay result: the Dmin claim remains an ordinary challengeable idea; no deliberative close occurs without ordinary certainty-band verdict history.
- Rejection codes: `ERR_STRUCTURAL_DMAX_LIVENESS_USED_FOR_DMIN`, `ERR_STRUCTURAL_DMAX_LIVENESS_USED_FOR_DELIBERATIVE_CLOSE`.
- Protects: survivor liveness cannot be used for Dmin or deliberative closure.

### TCF-022-contradiction-blocks-one-person-dmax-liveness

- Event sequence: one identity claims Dmax elapsed and another identity creates a contradictory target-bound Dmax-not-elapsed claim.
- Expected replay result: `structural_dmax_liveness_predicate = blocked`; no forced close is emitted through the survivor path.
- Rejection codes: `ERR_STRUCTURAL_DMAX_LIVENESS_BLOCKED_BY_CONTRADICTION` for a harness-only close attempt.
- Protects: replay does not silently choose between contradictory time claims.

### TCF-023-survivor-path-cannot-certify-beacon

- Event sequence: repeated survivor-mode forced closures are represented as derived setup and a harness-only mutation attempts to shrink beacon requirements and advance authority.
- Expected replay result: beacon remains not eligible; cycle certification remains pending; authorization frontier remains at genesis.
- Rejection codes: `ERR_STRUCTURAL_DMAX_LIVENESS_BEACON_REQUIREMENT_REDUCTION`, `ERR_COLLAPSE_THRESHOLD_SHRINK_ATTEMPT`.
- Protects: one-person structural liveness cannot become beacon authority.

### TCF-024-later-normal-certification-finalizes-pending-only

- Event sequence: later normal multi-identity beacon certification is available for a previously forced cycle, while a harness-only mutation attempts retroactive authority.
- Expected replay result: explicit pending outputs may authorize only if frontier rules allow; forbidden constrained-mode actions remain blocked.
- Rejection codes: `ERR_AUTHORITY_BACKFILL_ATTEMPT`.
- Protects: later certification does not retroactively validate forbidden actions.

### TCF-012-dmin-deliberative-close

- Event sequence: `cycle_close` records a deliberative boundary after Dmin structural readiness
  and `W_target` are satisfied.
- Expected replay result: structural boundary accepted; certification remains pending; authority remains blocked.
- Rejection codes: none.
- Protects: boundary creation is separate from authority.

### TCF-013-dmax-forced-close

- Event sequence: Dmax reaches structural readiness from a human stance plus capped passive evidence
  while `W_target` is unmet; `cycle_close` records a forced boundary.
- Expected replay result: Dmax mechanically implies structural Dmin; forced boundary remains forced.
- Rejection codes: none.
- Protects: passive evidence can assist required human support but forced closure preserves liveness only.

### TCF-014-forced-cycles-never-authorize

- Event sequence: no canonical events; a separate harness-only mutation attempts to extract authority from forced cycles.
- Expected replay result: attempt rejected; no backfill or burst capacity.
- Rejection codes: `ERR_CYCLE_FORCED_AUTHORITY_ATTEMPT`, `ERR_AUTHORITY_BACKFILL_ATTEMPT`.
- Protects: forced cycles never accumulate legitimacy.

### TCF-015-certainty-contradiction-blocks-predicate

- Event sequence: a certainty-band verdict assigns `medium` certainty to a contradictory claim.
- Expected replay result: contradiction blocks Dmin predicate truth.
- Rejection codes: none.
- Protects: sufficiently certain contradiction blocks predicates and beacon status.

### TCF-016-certification-gap-blocks-frontier

- Event sequence: no canonical events; derived certification state has cycle 1 pending between certified cycles 0 and 2.
- Expected replay result: authorization frontier stops at cycle 0.
- Rejection codes: none.
- Protects: frontier is contiguous and lagged.

### TCF-017-external-link-alone-no-certainty

- Event sequence: a target-bound claim includes URL/hash provenance only.
- Expected replay result: claim is accepted, but provenance alone gives no certainty.
- Rejection codes: none.
- Protects: external links are not evidence by themselves.

### TCF-018-source-document-and-chunk-ideas-support-after-placement

- Event sequence: source-document, source-chunk, and source-description claims are authored as ideas, connected as evidence, and placed by challenge verdict.
- Expected replay result: external-source material becomes relevant only through authored ideas, connections, and verdicts.
- Rejection codes: none.
- Protects: source assertions are ideas and remain challengeable.

### TCF-019-ai-created-canonical-evidence-time-claims-rejected

- Event sequence: an AI identity attempts to author a time claim, evidence idea, and vote.
- Expected replay result: all rejected; AI contributes no certainty or beacon diversity.
- Rejection codes: `ERR_TEMPO_AI_AUTHORITY`.
- Protects: AI has no canonical Tempo or challenge authority.

### TCF-020-offline-local-ideas-advisory-until-validation

- Event sequence: harness-only offline import attempt presents local ideas and a local cycle simulation.
- Expected replay result: material is advisory until canonical validation and replay; local simulation does not advance cycles.
- Rejection codes: none.
- Protects: offline work has no universal canonical authority before publication.

### TCF-025-rejection-matrix-tempo-cycle-validation

- Event sequence: invalid `idea_create`, `connection_create`, and `cycle_close` input events are submitted, while non-canonical direct mutations are represented as `harness_actions`.
- Expected replay result: no invalid event changes certainty, cycle state, frontier state, or authority.
- Rejection codes: `ERR_TEMPO_CLAIM_TARGET_KEY_MISMATCH`, `ERR_TEMPO_CLAIM_PROFILE_MISMATCH`, `ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR`, `ERR_TEMPO_HIDDEN_CLOCK_INPUT`, `ERR_TEMPO_BLOCK_HEIGHT_AUTHORITY`, `ERR_TEMPO_EVIDENCE_CONNECTION_INVALID`, `ERR_CYCLE_CLOSE_NOT_EARLIEST_VALID`, `ERR_CYCLE_CLOSE_TRIGGER_MISMATCH`, `ERR_CYCLE_CLOSE_PAYLOAD_MISMATCH`, `ERR_STRUCTURAL_DMAX_LIVENESS_AUTHORITY_ATTEMPT`, `ERR_STRUCTURAL_DMAX_LIVENESS_BEACON_REQUIREMENT_REDUCTION`, `ERR_COLLAPSE_THRESHOLD_SHRINK_ATTEMPT`, `ERR_FRONTIER_NON_CONTIGUOUS`, `ERR_FRONTIER_DECREASE`.
- Protects: implementation harnesses keep canonical event validation separate from derived-state mutation attempts.

### TCF-026-passive-evidence-alone-cannot-cross-tallow

- Event sequence: an eligible human authors an ordinary evidence idea carrying admissible passive
  timestamp metadata, but no target-bound human stance exists.
- Expected replay result: passive units are recorded and capped, structural readiness remains false,
  and no cycle close is emitted.
- Rejection codes: none.
- Protects: passive evidence cannot replace human structural participation.

### TCF-027-passive-plus-human-support-can-reach-tallow

- Event sequence: the passive evidence from TCF-026 is followed by one target-bound human Dmin
  support stance.
- Expected replay result: the human stance plus capped passive units reaches structural readiness;
  ordinary truth certainty remains `none`.
- Rejection codes: none.
- Protects: passive evidence may assist required human support without becoming truth certainty.

### TCF-028-passive-source-dedup-prevents-amplification

- Event sequence: two ordinary evidence ideas commit passive observations with the same source,
  target, and epoch.
- Expected replay result: passive support counts once and the duplicate source is identified.
- Rejection codes: none.
- Protects: event volume cannot amplify one passive source.

### TCF-029-zero-human-record-only

- Event sequence: no canonical input events; replay state has zero active eligible humans.
- Expected replay result: mode is `record_only`, no Dmin/Dmax predicate is true, and no universal
  cycle close is emitted.
- Rejection codes: none.
- Protects: machine-only operation cannot advance universal cycles.

## Validation Checklist

The JSON companion is expected to satisfy:

- JSON parses and fixture IDs are unique.
- JSON Schema parses and validates `tempo-cycle-fixtures.v1.json`.
- `input_events` contain only Appendix A canonical event families.
- Harness-only actions are separated into `harness_actions`.
- Markdown fixture IDs and titles match the JSON file.
- No canonical event uses a specialized Tempo support event family.
- No fixture defines a forbidden idea type.
- Every deliberative input is an identity-authored idea or an existing connection/challenge/vote/verdict event.
- External links and hashes are provenance only.
- No fixture relies on hidden clocks, block height, model output, or raw counts for certainty.
- Passive timestamp evidence is canonically committed, normalized, deduplicated, and capped below
  `T_allow`.
- Passive evidence alone never satisfies `T_allow`, `T_beacon`, cycle closure, certification, or
  authorization.
- Forced cycles never authorize POD, POINT, governance, ordinary mana, or rate-limit effects.
- Zero-human operation remains record-only.
- Population collapse does not shrink authority thresholds.
