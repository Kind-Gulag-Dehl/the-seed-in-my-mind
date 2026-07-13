---
doc_id: tempo_cycle_canonical_schema_and_replay_resolution_v1
title: Tempo/Cycle Canonical Schema and Replay Resolution v1
status: planning/spec-resolution
version: v1
last_reviewed: 2026-06-22

scope:
  - Resolves Tempo/Cycle schema and replay semantics around the Seed's idea-only deliberative model.

authoritative_for:
  - Planning guidance for subsequent authoritative spec patches.

not_authoritative_for:
  - Runtime behavior until authoritative specs are patched.
  - Production Tempo profile values.

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
  - Legacy text that treats Tempo support as separate non-idea records.

change_rules:
  - Do not use this planning file as an implementation contract until the authoritative documents are patched.

keywords:
  - Tempo
  - Cycle
  - idea-only model
  - time claims
  - evidence
  - certification
  - replay
---

# Tempo/Cycle Canonical Schema and Replay Resolution v1

This is a planning/spec-resolution document. It records the intended final model for Tempo/Cycle
schema and deterministic replay, but it does not itself amend Protocol v5 until the authoritative
documents are patched.

## Core Resolution

All canonical deliberative content is expressed as identity-authored ideas using existing base idea
types. Terms such as evidence, argument, attestation, observation, testimony, source statement, and
potential evidence describe the role or use of an idea, not additional canonical content-object
types. Canonical relationships are expressed through existing connection types and usages. Canonical
state changes are expressed through the unified challenge, vote, verdict, and cycle processes.

Tempo therefore does not define a separate evidence system, attestation event family, truth system,
or certainty formula. Time claims are ordinary `truth_claim` ideas with conditional `tempo_claim`
metadata. Evidence for or against them is also expressed as identity-authored ideas, usually
`truth_claim` ideas, connected through existing `relative_importance` connections with
`usage = evidence_for` or `usage = evidence_against`.

## Rejected Alternatives

- No `time_claim` idea type.
- No `tempo_target` idea type.
- No `beacon` idea type.
- No top-level canonical `evidence`, `attestation`, `testimony`, or `source` content-object type.
- No specialized Tempo support event family.
- No non-idea support, opposition, withdrawal, supersession, or latest-claim records.
- No certainty from counts, hidden weights, model scores, links, timestamps, block height,
  or local/server/client clocks.
- No system-authored ordinary placeholder time-claim ideas.

## Time Claims

A time claim is an identity saying something as an ordinary `truth_claim`, for example:

> At least Dmin has elapsed since cycle r began.

Target-bound Tempo claims use `idea_type = truth_claim`, the existing truth subtype appropriate for
observation or measurement, and conditional `tempo_claim` metadata:

- derived `target_key`;
- anchor event;
- anchor cycle index;
- target kind (`dmin` or `dmax`);
- elapsed-time relation;
- duration value and profile;
- active Tempo profile hash;
- provenance or payload references.

Multiple identities may independently create equivalent or contradictory time claims. Equivalent
claims may share the same derived target key, and `same_as` connections may organize them, but
separate authorship remains visible and no `same_as` connection grants authority.

## Derived Targets

Replay derives Dmin/Dmax target keys:

```text
tempo_target(cycle_index, dmin)
tempo_target(cycle_index, dmax)
```

These are stable replay keys/views. They are not authored canonical objects, canonical events, ideas,
or connection types. User interfaces may render them as target cards or questions, but humans author
ordinary truth claims that reference them.

## Evidence and External Sources

Potential evidence is represented by hypothetical evidence ideas. Actual evidence is represented by
identity-authored evidence ideas, usually `truth_claim` ideas. Evidence ideas connect to a target
truth claim through existing `relative_importance` connections with `usage = evidence_for` or
`usage = evidence_against`.

A paper, article, book, video, dataset, website, instrument output, or external record is not
automatically canonical evidence. An identity creates ideas asserting what that source says,
contains, measured, or supports. Those ideas may reference a URL, file hash, payload, author,
section, timestamp, or archived copy as provenance. Provenance does not replace the identity-authored
idea, and an external link alone never changes certainty.

Important external sources should be represented by source-document, source-section, and source-chunk
ideas inside the map when the existing base idea types can express them. Claims about those source
ideas remain challengeable.

## Certainty

Tempo reuses the canonical truth certainty-band model:

1. Potential evidence ideas define the evidence spectrum.
2. Actual evidence ideas are connected explicitly.
3. Evidence-placement challenges determine where actual evidence belongs.
4. A certainty-band challenge proposes the current certainty band for a time claim.
5. Eligible humans vote under ordinary challenge rules.
6. The verdict assigns the operative certainty band.

`T_allow`, `T_contradiction_block`, `T_beacon`, and `T_beacon_revoke` refer to ordered canonical
certainty bands, or deterministic threshold encodings over that band order. Any numeric encoding is
only an encoding of canonical certainty-band order, not a second certainty system.

Nodes must not infer certainty from equivalent-claim counts, raw author counts,
hidden non-idea support records, model scores, links, timestamps, local/server/client clocks, block height,
scheduler observations, or uncommitted AI output.

## Predicates and Beacons

Tempo evaluates ordinary target-bound time truth claims.

A Dmin/Dmax predicate may become true only when:

- at least one relevant time claim has an operative certainty at or above `T_allow`;
- no contradictory time claim has operative certainty at or above `T_contradiction_block`;
- all relevant certainty came from recorded idea, connection, challenge, vote, and verdict events.

A beacon is not an object or idea type. Beacon status is a derived status of an ordinary time truth
claim that reaches `T_beacon`, satisfies diversity through identity-authored supporting claims or
evidence, survives stability and challenge checks, and has no sufficiently certain contradiction.
Multiple beacon-status time claims may coexist.

## Tempo Mana and Narrow Lane

Tempo mana remains a small, non-transferable rate limit for the narrow Tempo repair lane. It may
rate-limit:

- target-bound time truth-claim creation;
- Tempo-context evidence truth-claim creation if the active profile permits it;
- `evidence_for`, `evidence_against`, and `same_as` connections in a Tempo context if the active
  profile permits them;
- Tempo-specific placement or certainty challenge participation only if the actor also has the
  required challenge eligibility, or if a future explicit Tempo-only challenge capability is adopted.

Tempo mana is not spent on any separate attestation event. It does not grant arbitrary canonical
writing, ordinary evidence creation outside Tempo context, connection creation outside Tempo context,
challenge creation, voting, verdict finalization, governance, POD, POINT, token authority, or ordinary
mana authority.

## Structural and Authority Separation

Low-certainty adjudicated time claims may satisfy `T_allow` for structural cycle closure. Beacon-level
adjudicated time claims certify timing. Lag `K` and contiguous certification control consequential
authorization.

Forced cycles keep the system live and allow adaptive `W_target` to move downward, but forced cycles
never create authority and never accumulate legitimacy. Later certification may finalize explicitly
pending outputs only; it must not validate forbidden actions retroactively or backfill ordinary mana
or rate-limit capacity.

Population collapse must never become an authority shortcut. One surviving human may keep structural
cycles and time repair moving where the active profile permits, but cannot alone satisfy beacon
diversity or unlock economic, governance, POD, POINT, lifecycle, ordinary write, or ordinary challenge
authority.

## Structural Dmax Survivor Liveness

The selected idea-only survivor mechanism is `structural_dmax_liveness_predicate`.

This predicate is not ordinary truth certainty, beacon certainty, truth finality, certification, or
authorization. It is a structural-only replay predicate that may be used only to force a Dmax cycle
boundary when ordinary Dmax certainty cannot yet be produced because participation has collapsed.

A Dmax target-bound truth claim may satisfy `structural_dmax_liveness_predicate` only when:

- the author is an eligible human `tempo_contributor`;
- the claim is a valid ordinary `truth_claim`;
- the claim has valid `tempo_claim` metadata for the current Dmax target;
- the claim was accepted through the narrow Tempo lane;
- required Tempo mana for target-bound time-claim creation was paid;
- no accepted contradictory target-bound time claim currently blocks the target under the active
  rulebook;
- no unresolved blocking truth challenge currently blocks the target under the active rulebook;
- no existing certainty-band verdict contradicts the claim at or above the configured contradiction
  band.

If another identity creates a contradictory target-bound claim, such as "Dmax has not elapsed since
cycle r began," the `structural_dmax_liveness_predicate` is `blocked` until ordinary challenge
process resolves the contradiction, unless the active rulebook explicitly defines a deterministic
non-authoritative tie behavior. Replay must not silently choose one claim.

If `structural_dmax_liveness_predicate == true` and `W_score < W_target`, the system boundary
emitter may emit `cycle_close` with `boundary_type = forced` and
`trigger = dmax_structural_liveness_forced`. The forced boundary remains forced forever.

The same rule must not:

- create `cycle_age_ge_dmin`;
- create `cycle_age_ge_dmax` ordinary certainty;
- create a deliberative boundary;
- create beacon status;
- certify a cycle;
- advance the authorization frontier;
- authorize POD, POINT, governance, lifecycle, final rank, ordinary mana, ordinary rate limits, token
  effects, ordinary challenge authority, or ordinary canonical write authority;
- reduce `K`, `T_beacon`, minimum beacon identities, independence requirements, or stability
  requirements.

Later identity-authored ideas and ordinary truth challenges may challenge the claim. Later
high-certainty beacon certification may certify the relevant timing target under normal rules, but
it may finalize only explicitly pending outputs if frontier rules allow and must not validate actions
that were forbidden when attempted.

## Replay Inputs

Tempo/Cycle replay derives state only from:

- identity-authored time truth claims;
- identity-authored potential evidence ideas;
- identity-authored actual evidence ideas;
- explicit `evidence_for`, `evidence_against`, and `same_as` connections;
- evidence-placement challenge outcomes;
- certainty-band challenge outcomes;
- ordinary challenge votes and verdicts;
- `cycle_close` events;
- the active Tempo profile.

Replay does not derive certainty from unsubmitted external documents, external links alone, model
scores, local/server/client clocks, receipt times, block height, scheduler observations, or AI output.

## Appendix A Patch Guidance

Appendix A should:

- keep conditional `tempo_claim` metadata on ordinary `truth_claim` ideas;
- keep derived Dmin/Dmax target views as replay views only;
- remove the specialized Tempo support event family and all non-idea support/supersession/latest-claim fields;
- use ordinary `connection_create` for `evidence_for`, `evidence_against`, and `same_as`;
- use ordinary challenge/vote/verdict events for evidence placement, certainty-band assignment, and
  contradiction adjudication;
- add Tempo-lane validation and mana fields to existing idea, connection, and challenge surfaces only
  where necessary;
- add derived replay fields for `structural_dmax_liveness_predicate`, `liveness_claim_id`,
  `liveness_target_key`, `blocking_contradiction_claim_ids`, `blocking_challenge_ids`,
  `liveness_predicate_status`, and `liveness_trigger_allowed_for`;
- add `dmax_structural_liveness_forced` as a `cycle_close.trigger` value for forced boundaries only.
