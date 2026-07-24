---
doc_id: tempo_spec
title: Tempo Specification
status: authoritative
version: v0
last_reviewed: 2026-06-22

scope:
  - Defines time claims, Tempo evidence roles, Dmin/Dmax predicates, derived beacons, and constrained/time-repair mode behavior.

authoritative_for:
  - Target-bound Tempo truth-claim semantics.
  - Tempo evidence admissibility through ordinary ideas, connections, and challenges.
  - Dmin/Dmax predicate production and Tempo modes.

not_authoritative_for:
  - Canonical encoding/hashing (see canonical-encoding-and-hashing-spec.md).
  - Cycle boundary emission and authorization-frontier mechanics (see cycle-spec.md).

depends_on:
  - protocol v5.md
  - protocol v5-appendix-a.md
  - challenge-engine-spec.md
  - cycle-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of node-and-conformance-spec.md, deterministic-replay-and-merge-spec.md, and governance-spec.md.

reader_path:
  - prereq: cycle-spec.md
  - next: challenge-engine-spec.md

keywords:
  - tempo
  - time claims
  - evidence ideas
  - certainty bands
  - constrained mode
  - time repair
---

> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

# Tempo Specification

*(Time Claims, Evidence Ideas, Structural Support, Certainty Bands, and Predicate Production)*

---

## 0. Purpose, Scope, and Design Invariants [anchor: 0_purpose_scope_and_design_invariants]

Tempo represents and adjudicates time-related information inside the protocol. Tempo does not advance cycles, certify cycles, authorize power, mint economic effects, or create a separate truth system. Tempo exists so time enters the Seed only as socially contestable, identity-authored ideas.

This specification normatively owns:

- time anchors and target-bound time truth claims;
- Tempo-context evidence rules using ordinary ideas and connections;
- ordinary certainty-band interpretation for time claims;
- structural-support production of `cycle_age_ge_dmin` and `cycle_age_ge_dmax`;
- derived beacon status for time claims;
- Tempo modes used by downstream restriction and recovery logic.

This specification does not own cycle progression metrics, cycle boundary emission, authorization-frontier advancement, ordinary mana issuance, POD, POINT, governance activation, or lifecycle effects.

This specification also does not grant admission, restricted-verification, inviter, or
invitation-capacity authority. Tempo eligibility is a distinct replay-derived lane;
admission alone, sponsorship, structural-root membership, inviter eligibility, and
invitation-capacity generation do not grant Tempo participation or influence.

Core invariants:

1. Canonical time is claimed, not read from a trusted clock.
2. Time claims are ordinary `truth_claim` ideas with conditional `tempo_claim` metadata.
3. Evidence, testimony, attestations, observations, source reports, arguments, measurements, and statements about documents are ideas in roles, not separate content-object types.
4. Dmin/Dmax targets are derived replay keys/views, not authored ideas, events, or connection types.
5. Beacons are derived high-certainty statuses of ordinary time truth claims, not authored objects.
6. Canonical truth certainty changes only through explicit idea, connection, challenge, vote, and verdict history.
7. Nodes must not infer truth certainty from counts, hidden weights, model scores, links, timestamps, local/server/client clocks, block height, scheduler observations, or uncommitted AI output.
8. `T_allow` is a derived structural-support threshold, not a canonical truth-certainty band.
9. Passive machine timestamp evidence may assist structural support only when it is canonically committed, deterministically normalized, capped below `T_allow`, and combined with profile-required eligible human participation.
10. Beacon certification, lag `K`, and the contiguous authorization frontier control consequential authority.
11. Forced cycles never grant authority, and population collapse never lowers beacon diversity, lag, or authority thresholds automatically.

---

## 1. Time Anchors, Targets, and Claims [anchor: 1_time_anchors_targets_and_claims]

### 1.1 Time Anchor [anchor: time_anchor]

A time anchor is a canonical reference point against which elapsed time is evaluated. Valid anchors include the genesis anchor for cycle 0, a prior `cycle_close` event, a previously certified Tempo target, or another governance-approved canonical anchor type. Anchors must be immutable and replayable from the canonical log.

### 1.2 Derived Dmin/Dmax Targets [anchor: derived_dmin_dmax_targets]

At the start of each structural cycle, replay derives:

```text
tempo_target(cycle_index, dmin)
tempo_target(cycle_index, dmax)
```

These keys are stable replay targets. They may be shown in UI as target cards or questions, but they are not canonical authored objects, events, ideas, or connection types. No system component may create a placeholder ordinary time claim on behalf of a target.

### 1.3 Time Claims [anchor: time_claims]

A time claim is an identity-authored ordinary `truth_claim` idea asserting a bounded elapsed-time relation relative to an anchor, such as:

> At least Dmin has elapsed since cycle r began.

Target-bound Tempo claims use the existing truth subtype specified in Appendix A and conditional `tempo_claim` metadata binding the claim to a derived target key, anchor, cycle index, target kind, elapsed-time relation, duration, Tempo profile, and provenance references.

Rules:

- authorship remains attached to each claim;
- multiple identities may create equivalent or contradictory claims;
- equivalent claims may share the same derived target key;
- `same_as` connections may organize equivalent claims but do not erase separate authorship or determine authority;
- contradictory claims remain visible and challengeable;
- target-bound claims must not request cycle advancement, certification, economic effects, governance effects, token effects, lifecycle effects, or ordinary mana effects.

Tempo must reject exact wall-clock timestamp claims as Tempo authority. A claim may describe a timestamp as source content or provenance, but the timestamp does not become a trusted clock.

---

## 2. Evidence Ideas and External Sources [anchor: 2_evidence_ideas_and_external_sources]

Tempo uses the same evidence model as the rest of the protocol.

Potential evidence is represented by hypothetical evidence ideas. Actual evidence is represented by identity-authored evidence ideas, usually `truth_claim` ideas. Evidence ideas connect to a target time truth claim using existing `relative_importance` connections with `usage = evidence_for` or `usage = evidence_against`.

Tempo-context evidence connections must validate both endpoints and the Tempo context. Nodes reject the connection with `ERR_TEMPO_EVIDENCE_CONNECTION_INVALID` when it references a non-existent target claim, points to an idea that is not valid evidence under the current schema, uses `evidence_for` or `evidence_against` without a valid target-bound time truth claim when required, uses `same_as` between claims with incompatible `tempo_claim` target keys, anchors, target kinds, durations, or Tempo profile hashes, attempts to make an external URL/hash/payload count directly as evidence without an identity-authored idea describing it, attempts to treat a derived `tempo_target` or beacon as an authored idea, or attempts to create Tempo certainty outside ordinary evidence-placement and certainty-band challenge flow.

An attestation in a Tempo UI is an idea, usually a truth claim, such as:

- "I observed that Dmin elapsed."
- "The claimed anchor is incorrect."
- "This event occurred after the cycle began."
- "The evidence does not establish that Dmax elapsed."

These are not votes or separate content objects.

External sources are provenance until represented by authored ideas. A paper, article, book, video, dataset, website, instrument output, or external record is not automatically canonical evidence. An identity creates ideas asserting what that source says, contains, measured, or supports. Those ideas may reference a URL, file hash, payload, author, section, timestamp, or archived copy. Important sources should be represented as source-document, source-section, or source-chunk ideas where existing base idea types can express them. Claims about those source ideas remain challengeable. An external link alone has no certainty effect.

Node-local time, server time, client timestamps, receipt time, background scheduler observations, block height, publication volume, local observations, and AI observations must not affect Tempo truth certainty unless a verified human turns them into valid canonical ideas and connections under ordinary rules.

### 2.1 Passive Machine Timestamp Evidence [anchor: passive_machine_timestamp_evidence]

Passive machine timestamp evidence is an intentional weak evidence channel. It is not authoritative time, it does not determine canonical event ordering, and it never substitutes for eligible human participation.

Permitted passive sources must be identically available to every replaying node and committed by canonical or canonically anchored data. The minimal admissible source categories are:

- a machine timestamp included in a signed human-authored canonical event envelope and covered by that event signature;
- a machine observation committed in a canonical publication or finalization artifact where the active publication profile already defines such a committed field;
- a content-addressed external timestamp observation referenced by a human-authored canonical event and available identically to replay;
- another explicitly Tempo-profile-approved canonical timestamp field with equivalent deterministic commitment and replay availability.

Forbidden direct inputs include uncommitted database `created_at`, HTTP receipt time, current node clock during replay, scheduler execution time, cache timestamps, filesystem modification time, local-only device metadata, implementation-specific server logs, values that differ between nodes, and snapshot approximate timestamps unless the active Tempo profile explicitly admits that exact committed field for Tempo evidence.

Passive evidence normalization MUST be defined by the immutable Tempo profile for the target. It MUST specify canonical timestamp format, precision, uncertainty interval, source identifier, source class, target key, source epoch or observation interval, canonical provenance, admissibility, deduplication, outlier handling, and influence cap. Repeated events carrying the same underlying clock source MUST NOT multiply influence. At minimum, replay deduplicates by `(source_id, target_key, source_epoch)`. Multiple observations depending on the same upstream time source MUST share a source class or otherwise be capped together by the profile.

The total passive contribution MUST obey:

```text
passive_contribution <= passive_evidence_cap
passive_evidence_cap < T_allow
```

No implementation may choose its own passive cap, source-class policy, or outlier algorithm.

---

## 3. Certainty and Challenges [anchor: 3_certainty_and_challenges]

Tempo reuses canonical truth certainty bands. It does not define a second numerical certainty system.

For each time truth claim:

1. potential evidence ideas define the evidence spectrum;
2. actual evidence ideas are connected explicitly;
3. evidence-placement challenges determine where actual evidence belongs;
4. a certainty-band challenge proposes the claim's certainty band;
5. eligible humans vote under ordinary challenge rules;
6. the verdict assigns the operative certainty band.

The canonical truth-certainty band order is defined once in Appendix A and encoded deterministically for replay and snapshots. `T_contradiction_block`, `T_beacon`, and `T_beacon_revoke` refer to ordinary canonical certainty bands or deterministic integer encodings of that band order. Floating-point truth certainty is forbidden.

`T_allow` is different. `T_allow` is a Tempo structural-support threshold derived from the canonical prefix. It asks only whether enough presently recorded eligible-human stance support and capped passive evidence exist to permit provisional structural progression while ordinary truth challenges continue. Crossing `T_allow` does not assign canonical truth certainty, does not create a beacon, does not certify a cycle, does not advance the authorization frontier, and does not finalize economic, governance, lifecycle, token, ordinary-mana, or rate-limit authority.

Challenge creation, voting, and verdict finalization require ordinary challenge eligibility unless a future explicit protocol amendment creates a narrowly scoped Tempo challenge capability. `tempo_contributor` status alone does not grant challenge creation, voting, or verdict authority.

Challenge verdicts do not delete claims. They update current derived certainty-band state and may be challenged later.

---

## 4. Tempo Profile, Structural Support, and Stances [anchor: 4_tempo_profile_structural_support_and_stances]

At the start of each structural cycle, replay derives and binds an immutable Tempo profile reference for each Dmin/Dmax target. The profile and eligibility basis are frozen for that target. Later rulebook/profile changes apply only to future targets and MUST NOT alter historical target evaluation.

The active Tempo profile MUST define or reference at least:

- `required_human_support_dmin`;
- `required_human_margin_dmin`;
- `required_human_support_dmax`;
- `required_human_margin_dmax`;
- `survivor_dmax_min_human_support`;
- `T_allow`;
- `passive_evidence_cap`;
- `passive_source_dedup_policy`;
- `passive_source_class_policy`;
- `passive_outlier_policy`;
- `contradiction_block_band`;
- `beacon_minimum_certainty_band`;
- `beacon_minimum_distinct_humans`;
- `beacon_challenge_survival_cycles`;
- `authorization_lag_k`;
- any required evidence source, source-class, or interval limits.

Profile references MUST be deterministically encoded, hashable, and validated according to Appendix A and the canonical encoding specification. Profiles may define numeric structural-support units, but those units are not truth-certainty scores.

### 4.1 Human Structural Stance [anchor: human_structural_stance]

For each Tempo target, replay derives at most one current structural stance per eligible human identity:

- `support`;
- `oppose`;
- `none`.

The stance is derived from ordinary canonical evidence or support/opposition connections under the active Tempo profile. A later valid stance by the same identity may supersede the earlier stance for current structural counting only; historical statements remain permanently preserved. Submitting many equivalent evidence ideas, claims, or connections MUST NOT multiply one human's structural weight.

### 4.2 Structural Support Score [anchor: structural_support_score]

For each target:

```text
eligible_human_support = count(current eligible-human support stances)
eligible_human_opposition = count(current eligible-human oppose stances)
eligible_human_margin = eligible_human_support - eligible_human_opposition
structural_support = human_support_component + capped_passive_contribution
```

The profile defines the exact deterministic unit conversion from current human stances to `human_support_component`. Passive contribution is capped below `T_allow`; therefore passive evidence alone can never cross `T_allow`.

An unresolved open challenge alone does not automatically block structural readiness. A finalized adverse verdict, an opposing target-bound claim at or above `contradiction_block_band`, or loss of the required support/margin may block readiness.

---

## 5. Tempo Contributor Eligibility and Tempo Mana [anchor: 5_tempo_contributor_eligibility_and_tempo_mana]

`tempo_contributor` is a narrow low-threshold lane for human time repair. It may permit:

- creation of target-bound time truth claims with valid `tempo_claim` metadata;
- creation of Tempo-context evidence truth claims if the active Tempo profile permits it;
- creation of Tempo-context `evidence_for`, `evidence_against`, or `same_as` connections if the active Tempo profile permits it;
- participation in time-related placement or certainty challenges only if the actor also has ordinary challenge eligibility, or if a future explicit Tempo challenge capability is adopted.

`tempo_contributor` does not grant arbitrary canonical idea creation, evidence creation outside Tempo context, connection creation outside Tempo context, challenge creation, challenge voting, verdict finalization, governance authority, POD, POINT, token authority, ordinary mana authority, ordinary rate-limit authority, or ordinary canonical-writer eligibility.

The Profile-v0 restricted verification lane is not `tempo_contributor` eligibility and
does not authorize Tempo claims, Tempo-context evidence, Tempo mana, or structural
stances. Conversely, a qualifying capacity period or an invitation-capacity debit is not
a Tempo reward, claim, influence input, or authority source.

Tempo mana is a dedicated, capped, non-transferable capacity that rate-limits the allowed Tempo lane operations. It recharges deterministically at structural cycle boundaries, is capped by the active profile, and is spent only by valid events in canonical log order. Invalid events do not spend mana. Forced cycles and uncertified cycles cannot create an unlimited bank.

The active Tempo profile defines:

- `tempo_mana_cap`;
- `tempo_mana_recharge`;
- `time_claim_create_cost`;
- `tempo_evidence_claim_create_cost`;
- `tempo_evidence_connection_cost`;
- `tempo_same_as_connection_cost`;
- `time_challenge_cost`.

`time_challenge_cost` applies only when challenge participation is otherwise valid. It does not create challenge eligibility.

---

## 6. Predicate Production [anchor: 6_predicate_production]

Tempo evaluates ordinary target-bound time truth claims and outputs:

- `cycle_age_ge_dmin`;
- `cycle_age_ge_dmax`;
- `structural_dmax_liveness_predicate`.

A Dmin structural predicate may become true only when:

- `eligible_human_support >= profile.required_human_support_dmin`;
- `eligible_human_margin >= profile.required_human_margin_dmin`;
- `structural_support >= T_allow`;
- no opposing target-bound claim has ordinary canonical certainty at or above `profile.contradiction_block_band`.

Ordinary Dmax structural readiness uses the same structural-support system with `profile.required_human_support_dmax` and `profile.required_human_margin_dmax`.

Passive evidence alone MUST NOT satisfy Dmin, ordinary Dmax, survivor Dmax, or `T_allow`. At least the applicable profile-required eligible human participation is always required.

Predicate truth permits structural boundary evaluation only. It does not imply durability, legitimacy, certification, payout eligibility, governance activation, lifecycle finality, final rank authority, POD, POINT, ordinary mana spendability, or ordinary rate-limit authority.

For the same anchor and Tempo profile, `cycle_age_ge_dmax == true` mechanically implies structural `cycle_age_ge_dmin == true` for boundary evaluation only. It does not create a Dmin beacon, Dmin certification, or Dmin-based authority.

`structural_dmax_liveness_predicate` is a separate Dmax-only, structural-only survivor predicate. It is not ordinary truth certainty, beacon certainty, truth finality, certification, or authorization. It may become true without ordinary Dmax structural readiness only under the narrow survivor rule in Section 8, and it may be consumed only by Cycle Specification forced closure logic.

---

## 7. Derived Beacon Status and Coverage [anchor: 7_derived_beacon_status_and_coverage]

A beacon is not an object, event, idea type, selected winning claim, or authority granted to a claim author.

Beacon status is a derived status of an ordinary time truth claim that:

- reaches `T_beacon`;
- satisfies diversity requirements through identity-authored supporting claims or evidence ideas;
- satisfies independence requirements without exposing or weighting civil identity categories;
- satisfies governance-defined stability and challenge-survivability rules;
- has no contradictory claim at or above `T_beacon_revoke` or the active contradiction threshold.

Multiple beacon-status time claims may coexist. A representative claim may be selected for display by deterministic tie-breaking, but the selection is display-only and grants no authority to its author.

Beacon revocation stops future authorization-frontier advancement where certification coverage is missing or blocked by a contradiction. It does not rewrite already authorized history.

`T_beacon` is the minimum ordinary canonical truth-certainty band required for derived beacon status. It is not structural support. Beacon status also requires the profile-defined minimum distinct eligible human support, challenge-survival condition, and absence of contradiction at or above the blocking certainty band.

Beacon coverage may cover one Tempo target, multiple consecutive Tempo targets, or a structured elapsed-time relation that deterministically entails multiple targets. Coverage MUST be explicit, replay-verifiable, based on canonical structured relations, and independent of natural-language interpretation. Each cycle receives its own derived certification status even when one beacon covers several targets.

Certification states are derived replay state and include at least `pending`, `certified`, `contested`, and `revoked`. A normal cycle is certified through applicable Dmin target coverage. A forced cycle may become time-certified through applicable Dmax coverage, but it remains forced permanently and never retroactively earns missing deliberative rewards.

---

## 8. Structural Liveness and Survivor Scenarios [anchor: 8_structural_liveness_and_survivor_scenarios]

The protocol must reconcile three requirements:

- all Tempo content is identity-authored ideas;
- certainty normally comes from explicit evidence-placement and certainty-band challenge verdicts;
- nonzero survivor participation must be able to maintain structural time repair and eventually satisfy Dmax structural progression while remaining unable to satisfy beacon diversity or unlock consequential authority alone.

The selected survivor-compatible mechanism is `structural_dmax_liveness_predicate`.

A Dmax target-bound truth claim may satisfy `structural_dmax_liveness_predicate` only when:

- at least `profile.survivor_dmax_min_human_support` eligible humans currently participate;
- at least one valid target-bound Dmax claim or support stance exists;
- required capped passive plausibility evidence exists under the active profile;
- ordinary Dmax support requirements cannot be met;
- the system is in the applicable constrained/time-repair condition;
- every contributing claim is a valid ordinary `truth_claim`;
- every contributing claim has valid `tempo_claim` metadata for the current Dmax target;
- every contributing claim or stance was accepted through the narrow Tempo lane;
- required Tempo mana for target-bound time-claim creation or stance creation was paid;
- no accepted contradictory target-bound time claim currently blocks the target under the active rulebook;
- no unresolved blocking truth challenge currently blocks the target under the active rulebook;
- no existing certainty-band verdict contradicts the claim at or above `T_contradiction_block`.

If another identity creates a contradictory target-bound claim, the liveness predicate is `blocked` until ordinary challenge process resolves the contradiction, unless the active rulebook explicitly defines a deterministic non-authoritative tie behavior. Nodes MUST NOT silently choose one claim.

This predicate applies only to Dmax forced structural closure. It MUST NOT:

- create ordinary Dmax structural readiness or truth certainty;
- satisfy `cycle_age_ge_dmin`;
- create a deliberative boundary;
- create beacon status;
- certify a cycle;
- advance the authorization frontier;
- authorize POD, POINT, governance, lifecycle, final rank, ordinary mana, ordinary rate limits, token effects, ordinary challenge authority, or ordinary canonical write authority;
- reduce `K`, `T_beacon`, beacon diversity, independence, or stability requirements.

Later identity-authored ideas and ordinary truth challenges may challenge the liveness claim. Later normal beacon certification may certify the relevant target, but it may finalize only explicitly pending outputs through the lagged frontier and MUST NOT validate actions that were forbidden when attempted.

---

Zero participating eligible humans means no new human Tempo claims or stances, no Dmin structural readiness, no Dmax structural readiness, no survivor Dmax structural liveness, no universal `cycle_close`, no cycle certification, and no authorization-frontier movement. Machine evidence alone may not continue universal cycles.

The term "internal anti-stall" is explanatory terminology for this survivor Dmax structural liveness mechanism. It is not a third independent fallback.

---

## 9. Tempo Modes [anchor: 9_tempo_modes]

Tempo exposes derived mode flags consumed by downstream systems:

- `normal`;
- `constrained`;
- `record_only`.

`normal` requires sufficient recent beacon/certification coverage, stable predicates, low contradiction density, and authorization-frontier coverage for normal effects.

`constrained` applies when structural progression or Tempo repair remains possible but beacon support, certification coverage, or authorization-frontier coverage lags. Early cycles before lag `K` and certification coverage are constrained unless immutable independently verifiable genesis data defines a bootstrap basis that cannot weaken anti-collapse invariants.

`record_only` applies when canonical publication cannot proceed or zero eligible human participation prevents minimum replayable Tempo repair. In record-only posture, nodes may read, replay, preserve, and prepare local/offline drafts for possible later publication, but universal structural cycle advancement does not proceed.

`time_repair_priority` is not an independent global authority mode. It is a constrained-mode substate, reason code, or restricted action profile used when beacon coverage is absent for multiple cycles, contradictions block predicate stability, or time legitimacy is severely degraded. In this substate, target-bound time truth claims and explicitly permitted Tempo-context evidence ideas/connections should be prioritized. Challenge creation, voting, and verdict finalization remain ordinary challenge-system powers unless explicitly amended.

`time-only mode` is a deprecated historical alias for constrained time-repair behavior. Partition status is operational/offline context, not an independent global authority mode.

Mode transitions must be deterministic, replayable, and derived solely from canonical data.

---

## 10. Structural/Authority Separation [anchor: 10_structural_authority_separation]

The Cycle Specification may consume `T_allow` structural predicates derived during cycle `r` within cycle `r` only for structural boundary evaluation:

- `cycle_age_ge_dmin` with `W_score >= W_target` permits deliberative structural close;
- `cycle_age_ge_dmax` with unmet work target permits forced structural close;
- `structural_dmax_liveness_predicate` with unmet work target permits forced structural close only;
- Dmax mechanically implies structural Dmin for the same anchor and profile.

Same-cycle Tempo predicate consumption must not authorize economic, governance, lifecycle, token, final-rank, POD, POINT, ordinary-mana, ordinary-rate-limit, or other irreversible effects.

Tempo predicates, Dmax, forced/survivor structural closure, certification, and the
authorization frontier MUST NOT generate Profile-v0 invitation capacity, advance inviter
maturation, activate new inviter eligibility, restore invitation suspension, or mint
emergency admission authority unless the boundary independently qualifies under the
Cycle Specification's human-deliberative capacity rule. Tempo does not provide a
machine, operator, or AI fallback for admission liveness.

Consequential authority requires beacon-level certification and the Cycle Specification's contiguous lagged authorization frontier. Later certification may finalize explicitly pending outputs only. It must not validate actions that were forbidden when attempted or create stockpiles of unused ordinary mana allowances or rate-limit resets.

Population collapse must not automatically reduce `K`, `T_beacon`, beacon identity requirements, independence requirements, or stability-cycle requirements. One surviving human may keep structural cycles and time repair moving where the active profile permits, but does not automatically become sovereign.

---

## 11. Replay Completeness [anchor: 11_replay_completeness]

The canonical log must be sufficient to reconstruct:

- all target-bound time truth claims;
- all potential evidence ideas and actual evidence ideas used in Tempo context;
- `evidence_for`, `evidence_against`, and `same_as` connections;
- evidence-placement challenges and verdicts;
- certainty-band challenges and verdicts;
- contradiction challenge outcomes;
- immutable Tempo profile references;
- human structural stances;
- passive evidence normalization, deduplication, outlier handling, and capped contribution;
- Tempo mana balances;
- structural Dmax liveness predicate status, including blocking contradictions and blocking challenges;
- Dmin/Dmax predicates;
- derived beacon status and revocation;
- beacon target coverage;
- cycle certification inputs;
- Tempo mode transitions.

Tempo must never advance cycles, authorize economic or governance power, trust clocks or infrastructure, or create irreversibility. Tempo measures contested time and exposes it safely.
