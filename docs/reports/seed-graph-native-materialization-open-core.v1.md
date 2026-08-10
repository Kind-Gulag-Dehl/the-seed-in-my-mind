---
title: Seed Graph-Native Open Core Materialization Authority Report
project: The Seed in My Mind
status: clerical correction verified; downstream semantic plan complete
version: v1
last_updated: 2026-07-27
task_id: INTEGRATION-SEED-GRAPH-MATERIALIZATION-001
---

# Seed Graph-Native Open Core Materialization Authority Report v1

## 1. Result

Appendix A and the current canonical event registry support a purely clerical correction:

> `Count in this table: 39 Appendix A catalog event names`

became:

> `Count in this table: 38 Appendix A catalog event names`

The registry contains 38 unique current event rows. Appendix A contains the same 38 unique current event names across A4 and A10 when split aliases are counted separately and the explicitly deprecated or interface-only `snapshot_create` and `snapshot_adopt` names are excluded. The symmetric difference is zero.

No event was added, removed, renamed, reclassified, or semantically changed. No payload, actor, replay effect, requirement level, rejection code, or implementation status changed.

## 2. Exact proof

The comparison used the current working-tree authority:

- `docs/protocol v5-appendix-a.md`, SHA-256 `2f6e9fff01dbbef69ba787f185fe767e5cd4969b0748b9d7eaa1cb777c283154`;
- corrected `docs/protocol-event-registry.v1.md`, SHA-256 `e7687b2d8663ecacbe9db60c00c3526a7992371b93512a00d92f1b922a0cc014`.

Mechanical results:

| Assertion | Result |
|---|---:|
| Registry rows | 38 |
| Registry unique names | 38 |
| Appendix current catalog names | 38 |
| Appendix unique current catalog names | 38 |
| Symmetric difference | 0 |

The three vote events are defined in Appendix A A10 rather than A4. The count therefore covers the complete Appendix event catalog, not only A4 headings.

## 3. Downstream semantic materialization

The private planning artifact
`docs/planning/generated/seed-graph-native-open-core-semantic-materialization.v1.json`
was created with SHA-256
`32f9cca6cdd36fa789c68dd63cb0688081fa1b7b762d91ccaa597e5b5f654564`.

It resolves:

- all 58 bounded Open Core clause-coverage units;
- all 38 current event rows;
- 23 deduplicated authority-bound semantic records;
- two genuinely missing semantic records without allocating a slug or UUID;
- the DEC-043 Ordering predecessors through explicit in-place supersession;
- the deprecated `idea_update_representation` alias as archival-only.

The plan preserves exact current source paths, sections, and SHA-256 values. Generated semantic wording is marked AI authority-bound and not human-reviewed.

## 4. Boundaries

This work changed only the registry count prose and this report in Open Core. It did not change Appendix A, Protocol v5 semantics, runtime code, schemas, migrations, APIs, DTOs, conformance vectors, databases, Seed candidates, stable IDs, canonical events, signing, genesis, staging, commits, or deployment.

GATE-SEED-003-04 is not advanced by this report alone. The parent integration must combine this evidence with the private-authority materialization, verify source refreshes and collisions, and record the resulting gate state.
