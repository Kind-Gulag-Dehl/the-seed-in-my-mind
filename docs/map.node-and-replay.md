---
doc_id: map_node_and_replay
title: Map: Node and Replay
status: derived
version: v0
last_reviewed: 2026-01-27

scope:
  - Navigation map of section headings and anchors for the listed sources.

authoritative_for:
  - Navigation only.

not_authoritative_for:
  - Any protocol semantics, encodings, or conformance rules.

depends_on:
  - node-and-conformance-spec.md
  - deterministic-replay-and-merge-spec.md

conflicts:
  - none known

change_rules:
  - If source headings or anchors change, regenerate this map.

reader_path:
  - prereq: authoritative-index.md
  - next: none

keywords:
  - map
  - node
  - replay
  - navigation
---

Purpose: quick navigation of node conformance and deterministic replay sections.

## source documents
- node-and-conformance-spec.md
- deterministic-replay-and-merge-spec.md

## quick jump table
### node-and-conformance-spec.md
| § | heading | anchor | description |
| --- | --- | --- | --- |
| n/a | 0. Scope, Purpose, Definitions | 0_scope_purpose_definitions | See node-and-conformance-spec.md (0_scope_purpose_definitions). Covers 0. Scope, Purpose, Definitions. |
| 0.1 | Scope | scope | See node-and-conformance-spec.md §0.1 (scope). Covers Scope. |
| 0.2 | Purpose | purpose | See node-and-conformance-spec.md §0.2 (purpose). Covers Purpose. |
| 0.3 | Key Definitions | key_definitions | See node-and-conformance-spec.md §0.3 (key_definitions). Covers Key Definitions. |
| n/a | 1. Node Roles and Modes | 1_node_roles_and_modes | See node-and-conformance-spec.md (1_node_roles_and_modes). Covers 1. Node Roles and Modes. |
| 1.1 | Full Canonical Node (Normative) | full_canonical_node_normative | See node-and-conformance-spec.md §1.1 (full_canonical_node_normative). Covers Full Canonical Node (Normative). |
| 1.2 | Partial Nodes (Informative Only) | partial_nodes_informative_only | See node-and-conformance-spec.md §1.2 (partial_nodes_informative_only). Covers Partial Nodes (Informative Only). |
| 1.3 | Offline Node Mode | offline_node_mode | See node-and-conformance-spec.md §1.3 (offline_node_mode). Covers Offline Node Mode. |
| n/a | 2. Canonical Data Model Requirements | 2_canonical_data_model_requirements | See node-and-conformance-spec.md (2_canonical_data_model_requirements). Covers 2. Canonical Data Model Requirements. |
| 2.1 | Mandatory Data Structures | mandatory_data_structures | See node-and-conformance-spec.md §2.1 (mandatory_data_structures). Covers Mandatory Data Structures. |
| 2.2 | Canonical Record Ordering | canonical_record_ordering | See node-and-conformance-spec.md §2.2 (canonical_record_ordering). Covers Canonical Record Ordering. |
| 2.3 | Event Envelope and Schema Requirements | event_envelope_and_schema_requirements | See node-and-conformance-spec.md §2.3 (event_envelope_and_schema_requirements). Covers Event Envelope and Schema Requirements. |
| n/a | 3. Event Validation Pipeline | 3_event_validation_pipeline | See node-and-conformance-spec.md (3_event_validation_pipeline). Covers 3. Event Validation Pipeline. |
| 3.1 | Submission Intake | submission_intake | See node-and-conformance-spec.md §3.1 (submission_intake). Covers Submission Intake. |
| 3.2 | Safety Classification Enforcement | safety_classification_enforcement | See node-and-conformance-spec.md §3.2 (safety_classification_enforcement). Covers Safety Classification Enforcement. |
| 3.3 | Governance and Rulebook Checks | governance_and_rulebook_checks | See node-and-conformance-spec.md §3.3 (governance_and_rulebook_checks). Covers Governance and Rulebook Checks. |
| 3.4 | AI Source Rules | ai_source_rules | See node-and-conformance-spec.md §3.4 (ai_source_rules). Covers AI Source Rules. |
| n/a | 4. Snapshot Requirements | 4_snapshot_requirements | See node-and-conformance-spec.md (4_snapshot_requirements). Covers 4. Snapshot Requirements. |
| 4.1 | Snapshot Generation | snapshot_generation | See node-and-conformance-spec.md §4.1 (snapshot_generation). Covers Snapshot Generation. |
| 4.2 | Snapshot Anchoring | snapshot_anchoring | See node-and-conformance-spec.md §4.2 (snapshot_anchoring). Covers Snapshot Anchoring. |
| 4.3 | Snapshot Structure | snapshot_structure | See node-and-conformance-spec.md §4.3 (snapshot_structure). Covers Snapshot Structure. |
| 4.4 | Snapshot Import | snapshot_import | See node-and-conformance-spec.md §4.4 (snapshot_import). Covers Snapshot Import. |
| 4.4 | Snapshot Import | snapshot_import_2 | See node-and-conformance-spec.md §4.4 (snapshot_import_2). Covers Snapshot Import. |
| n/a | 5. Deterministic Replay System | 5_deterministic_replay_system | See node-and-conformance-spec.md (5_deterministic_replay_system). Covers 5. Deterministic Replay System. |
| 5.1 | Replay Goals | replay_goals | See node-and-conformance-spec.md §5.1 (replay_goals). Covers Replay Goals. |
| 5.2 | Replay Algorithm Requirements | replay_algorithm_requirements | See node-and-conformance-spec.md §5.2 (replay_algorithm_requirements). Covers Replay Algorithm Requirements. |
| 5.3 | AI Activities Ignored | ai_activities_ignored | See node-and-conformance-spec.md §5.3 (ai_activities_ignored). Covers AI Activities Ignored. |
| 5.4 | Rulebook Versioning During Replay | rulebook_versioning_during_replay | See node-and-conformance-spec.md §5.4 (rulebook_versioning_during_replay). Covers Rulebook Versioning During Replay. |
| n/a | 6. Canonical Publication Profiles | 6_proof_of_deliberation_chain_integration | See node-and-conformance-spec.md (6_proof_of_deliberation_chain_integration). Covers 6. Canonical Publication Profiles. |
| 6.1 | Profile-aware publication | pod_block_formation | See node-and-conformance-spec.md §6.1 (pod_block_formation). Covers Profile-aware publication. |
| 6.2 | Publication validation rules | block_validation_rules | See node-and-conformance-spec.md §6.2 (block_validation_rules). Covers Publication validation rules. |
| 6.3 | No AI in Publication Roles | no_ai_in_pod_roles | See node-and-conformance-spec.md §6.3 (no_ai_in_pod_roles). Covers No AI in Publication Roles. |
| 6.4 | Conflicting Finality and Stall Behavior | failed_forks | See node-and-conformance-spec.md §6.4 (failed_forks). Covers Conflicting Finality and Stall Behavior. |
| n/a | 7. Identity and Authorization Requirements | 7_identity_and_authorization_requirements | See node-and-conformance-spec.md (7_identity_and_authorization_requirements). Covers 7. Identity and Authorization Requirements. |
| 7.1 | Human Identity Anchoring | human_identity_anchoring | See node-and-conformance-spec.md §7.1 (human_identity_anchoring). Covers Human Identity Anchoring. |
| 7.2 | Autopilot Transparency | autopilot_transparency | See node-and-conformance-spec.md §7.2 (autopilot_transparency). Covers Autopilot Transparency. |
| 7.3 | Identity Continuity | identity_continuity | See node-and-conformance-spec.md §7.3 (identity_continuity). Covers Identity Continuity. |
| n/a | 8. Safety Rules Enforcement | 8_safety_rules_enforcement | See node-and-conformance-spec.md (8_safety_rules_enforcement). Covers 8. Safety Rules Enforcement. |
| 8.1 | Payload Classes | payload_classes | See node-and-conformance-spec.md §8.1 (payload_classes). Covers Payload Classes. |
| 8.2 | Blocked Submission Events | blocked_submission_events | See node-and-conformance-spec.md §8.2 (blocked_submission_events). Covers Blocked Submission Events. |
| 8.3 | "Why Am I Seeing This?" Support | why_am_i_seeing_this_support | See node-and-conformance-spec.md Section 8.3 (why_am_i_seeing_this_support). Covers "Why Am I Seeing This?" Support. |
| 8.4 | Hard Floor Enforcement | hard_floor_enforcement | See node-and-conformance-spec.md §8.4 (hard_floor_enforcement). Covers Hard Floor Enforcement. |
| n/a | **8A. Custody Manifest Canonical Encoding and Hashing** | 8a_custody_manifest_canonical_encoding_and_hashing | See node-and-conformance-spec.md (8a_custody_manifest_canonical_encoding_and_hashing). Covers **8A. Custody Manifest Canonical Encoding and Hashing**. |
| n/a | **8A.1 Purpose** | 8a_1_purpose | See node-and-conformance-spec.md (8a_1_purpose). Covers **8A.1 Purpose**. |
| n/a | **8A.2 Canonical Encoding Requirements** | 8a_2_canonical_encoding_requirements | See node-and-conformance-spec.md (8a_2_canonical_encoding_requirements). Covers **8A.2 Canonical Encoding Requirements**. |
| n/a | **8A.3 Hashing Rules** | 8a_3_hashing_rules | See node-and-conformance-spec.md (8a_3_hashing_rules). Covers **8A.3 Hashing Rules**. |
| n/a | **8A.4 Conformance Fixtures** | 8a_4_conformance_fixtures | See node-and-conformance-spec.md (8a_4_conformance_fixtures). Covers **8A.4 Conformance Fixtures**. |
| n/a | **8B. Reconciliation Transcript Determinism** | 8b_reconciliation_transcript_determinism | See node-and-conformance-spec.md (8b_reconciliation_transcript_determinism). Covers **8B. Reconciliation Transcript Determinism**. |
| n/a | **8B.1 Purpose** | 8b_1_purpose | See node-and-conformance-spec.md (8b_1_purpose). Covers **8B.1 Purpose**. |
| n/a | **8B.2 Transcript Definition** | 8b_2_transcript_definition | See node-and-conformance-spec.md (8b_2_transcript_definition). Covers **8B.2 Transcript Definition**. |
| n/a | **8B.3 Determinism Requirements** | 8b_3_determinism_requirements | See node-and-conformance-spec.md (8b_3_determinism_requirements). Covers **8B.3 Determinism Requirements**. |
| n/a | **8B.4 Conformance Fixtures** | 8b_4_conformance_fixtures | See node-and-conformance-spec.md (8b_4_conformance_fixtures). Covers **8B.4 Conformance Fixtures**. |
| n/a | **8C. State Witness Receipt Verification** | 8c_state_witness_receipt_verification | See node-and-conformance-spec.md (8c_state_witness_receipt_verification). Covers **8C. State Witness Receipt Verification**. |
| n/a | **8C.1 Purpose** | 8c_1_purpose | See node-and-conformance-spec.md (8c_1_purpose). Covers **8C.1 Purpose**. |
| n/a | **8C.2 Verification Rules** | 8c_2_verification_rules | See node-and-conformance-spec.md (8c_2_verification_rules). Covers **8C.2 Verification Rules**. |
| n/a | **8C.3 Replay Stability** | 8c_3_replay_stability | See node-and-conformance-spec.md (8c_3_replay_stability). Covers **8C.3 Replay Stability**. |
| n/a | **8C.4 Conformance Fixtures** | 8c_4_conformance_fixtures | See node-and-conformance-spec.md (8c_4_conformance_fixtures). Covers **8C.4 Conformance Fixtures**. |
| n/a | **8D. Partitioned Merge Behavior** | 8d_partitioned_merge_behavior | See node-and-conformance-spec.md (8d_partitioned_merge_behavior). Covers **8D. Partitioned Merge Behavior**. |
| n/a | **8D.1 Purpose** | 8d_1_purpose | See node-and-conformance-spec.md (8d_1_purpose). Covers **8D.1 Purpose**. |
| n/a | **8D.2 Required Merge Behavior** | 8d_2_required_merge_behavior | See node-and-conformance-spec.md (8d_2_required_merge_behavior). Covers **8D.2 Required Merge Behavior**. |
| n/a | **8D.3 Determinism Requirements** | 8d_3_determinism_requirements | See node-and-conformance-spec.md (8d_3_determinism_requirements). Covers **8D.3 Determinism Requirements**. |
| n/a | **8D.4 Conformance Fixtures** | 8d_4_conformance_fixtures | See node-and-conformance-spec.md (8d_4_conformance_fixtures). Covers **8D.4 Conformance Fixtures**. |
| n/a | 9. Token Accounting on Nodes | 9_token_accounting_on_nodes | See node-and-conformance-spec.md (9_token_accounting_on_nodes). Covers 9. Token Accounting on Nodes. |
| 9.1 | POD Calculations | pod_calculations | See node-and-conformance-spec.md §9.1 (pod_calculations). Covers POD Calculations. |
| 9.2 | POINT Emission and Melt | point_emission_and_melt | See node-and-conformance-spec.md §9.2 (point_emission_and_melt). Covers POINT Emission and Melt. |
| 9.3 | Identity-Level Storage | identity_level_storage | See node-and-conformance-spec.md §9.3 (identity_level_storage). Covers Identity-Level Storage. |
| n/a | 10. Rulebook Enforcement | 10_rulebook_enforcement | See node-and-conformance-spec.md (10_rulebook_enforcement). Covers 10. Rulebook Enforcement. |
| 10.1 | Rulebook Versions | rulebook_versions | See node-and-conformance-spec.md §10.1 (rulebook_versions). Covers Rulebook Versions. |
| 10.2 | Protocol Invariants | protocol_invariants | See node-and-conformance-spec.md §10.2 (protocol_invariants). Covers Protocol Invariants. |
| 10.3 | Rulebook Activation Events | rulebook_activation_events | See node-and-conformance-spec.md §10.3 (rulebook_activation_events). Covers Rulebook Activation Events. |
| n/a | 11. Sandbox Handling & AI Boundaries | 11_sandbox_handling_ai_boundaries | See node-and-conformance-spec.md (11_sandbox_handling_ai_boundaries). Covers 11. Sandbox Handling & AI Boundaries. |
| 11.1 | Sandbox Universe | sandbox_universe | See node-and-conformance-spec.md §11.1 (sandbox_universe). Covers Sandbox Universe. |
| 11.2 | Helper Drafts | helper_drafts | See node-and-conformance-spec.md §11.2 (helper_drafts). Covers Helper Drafts. |
| 11.3 | Ent Information | ent_information | See node-and-conformance-spec.md §11.3 (ent_information). Covers Ent Information. |
| n/a | 12. Networking & Replication | 12_networking_replication | See node-and-conformance-spec.md (12_networking_replication). Covers 12. Networking & Replication. |
| 12.1 | Gossip / Receive Rules | gossip_receive_rules | See node-and-conformance-spec.md §12.1 (gossip_receive_rules). Covers Gossip / Receive Rules. |
| 12.2 | Bandwidth Optimization | bandwidth_optimization | See node-and-conformance-spec.md §12.2 (bandwidth_optimization). Covers Bandwidth Optimization. |
| 12.3 | Tribe Replication | tribe_replication | See node-and-conformance-spec.md §12.3 (tribe_replication). Covers Tribe Replication. |
| n/a | 13. Conformance Testing & Compliance Levels | 13_conformance_testing_compliance_levels | See node-and-conformance-spec.md (13_conformance_testing_compliance_levels). Covers 13. Conformance Testing & Compliance Levels. |
| 13.1 | Compliance Tiers | compliance_tiers | See node-and-conformance-spec.md §13.1 (compliance_tiers). Covers Compliance Tiers. |
| 13.2 | Mandatory Test Vectors | mandatory_test_vectors | See node-and-conformance-spec.md §13.2 (mandatory_test_vectors). Covers Mandatory Test Vectors. |
| 13.3 | Tooling | tooling | See node-and-conformance-spec.md §13.3 (tooling). Covers Tooling. |
| n/a | 14. Security Requirements | 14_security_requirements | See node-and-conformance-spec.md (14_security_requirements). Covers 14. Security Requirements. |
| 14.1 | Attack Mitigation | attack_mitigation | See node-and-conformance-spec.md §14.1 (attack_mitigation). Covers Attack Mitigation. |
| 14.2 | Chain Integrity | chain_integrity | See node-and-conformance-spec.md §14.2 (chain_integrity). Covers Chain Integrity. |
| 14.3 | Compromise Response | compromise_response | See node-and-conformance-spec.md §14.3 (compromise_response). Covers Compromise Response. |
| n/a | 15. Informative Appendices | 15_informative_appendices | See node-and-conformance-spec.md (15_informative_appendices). Covers 15. Informative Appendices. |
| 15.1 | Node Storage Diagrams | node_storage_diagrams | See node-and-conformance-spec.md §15.1 (node_storage_diagrams). Covers Node Storage Diagrams. |
| 15.2 | Replay Walkthrough Examples | replay_walkthrough_examples | See node-and-conformance-spec.md §15.2 (replay_walkthrough_examples). Covers Replay Walkthrough Examples. |
| 15.3 | Rate Limit Implementation Guide | rate_limit_implementation_guide | See node-and-conformance-spec.md §15.3 (rate_limit_implementation_guide). Covers Rate Limit Implementation Guide. |
| 15.4 | Safety Enforcement Flowcharts | safety_enforcement_flowcharts | See node-and-conformance-spec.md §15.4 (safety_enforcement_flowcharts). Covers Safety Enforcement Flowcharts. |
| 15.5 | Chain Anchor Examples | chain_anchor_examples | See node-and-conformance-spec.md §15.5 (chain_anchor_examples). Covers Chain Anchor Examples. |
| 15.6 | Tribe Node Deployment Patterns | tribe_node_deployment_patterns | See node-and-conformance-spec.md §15.6 (tribe_node_deployment_patterns). Covers Tribe Node Deployment Patterns. |
| 15.7 | AI Sandbox Integration Examples | ai_sandbox_integration_examples | See node-and-conformance-spec.md §15.7 (ai_sandbox_integration_examples). Covers AI Sandbox Integration Examples. |

### deterministic-replay-and-merge-spec.md
| § | heading | anchor | description |
| --- | --- | --- | --- |
| n/a | Deterministic Replay & Merge Specification | deterministic_replay_merge_specification | See deterministic-replay-and-merge-spec.md (deterministic_replay_merge_specification). Covers Deterministic Replay & Merge Specification. |
| n/a | 0. Purpose, scope, and authority | 0_purpose_scope_and_authority | See deterministic-replay-and-merge-spec.md (0_purpose_scope_and_authority). Covers 0. Purpose, scope, and authority. |
| 0.1 | Purpose | purpose | See deterministic-replay-and-merge-spec.md §0.1 (purpose). Covers Purpose. |
| 0.2 | Scope | scope | See deterministic-replay-and-merge-spec.md §0.2 (scope). Covers Scope. |
| 0.3 | Authority | authority | See deterministic-replay-and-merge-spec.md §0.3 (authority). Covers Authority. |
| n/a | 1. Definitions and primitives | 1_definitions_and_primitives | See deterministic-replay-and-merge-spec.md (1_definitions_and_primitives). Covers 1. Definitions and primitives. |
| 1.1 | Core terms | core_terms | See deterministic-replay-and-merge-spec.md §1.1 (core_terms). Covers Core terms. |
| 1.2 | Inputs and outputs of replay | inputs_and_outputs_of_replay | See deterministic-replay-and-merge-spec.md §1.2 (inputs_and_outputs_of_replay). Covers Inputs and outputs of replay. |
| 1.3 | Deterministic serialization and hashing dependencies | deterministic_serialization_and_hashing_dependencies | See deterministic-replay-and-merge-spec.md §1.3 (deterministic_serialization_and_hashing_dependencies). Covers Deterministic serialization and hashing dependencies. |
| n/a | 2. Canonical event ordering | 2_canonical_event_ordering | See deterministic-replay-and-merge-spec.md (2_canonical_event_ordering). Covers 2. Canonical event ordering. |
| 2.1 | Ordering invariants | ordering_invariants | See deterministic-replay-and-merge-spec.md §2.1 (ordering_invariants). Covers Ordering invariants. |
| 2.2 | Ordering source of truth | ordering_source_of_truth | See deterministic-replay-and-merge-spec.md §2.2 (ordering_source_of_truth). Covers Ordering source of truth. |
| 2.3 | Forbidden ordering inputs | forbidden_ordering_inputs | See deterministic-replay-and-merge-spec.md §2.3 (forbidden_ordering_inputs). Covers Forbidden ordering inputs. |
| n/a | 3. Replay state model | 3_replay_state_model | See deterministic-replay-and-merge-spec.md (3_replay_state_model). Covers 3. Replay state model. |
| 3.1 | Canonical state partitions | canonical_state_partitions | See deterministic-replay-and-merge-spec.md §3.1 (canonical_state_partitions). Covers Canonical state partitions. |
| 3.2 | Stored vs derived state | stored_vs_derived_state | See deterministic-replay-and-merge-spec.md §3.2 (stored_vs_derived_state). Covers Stored vs derived state. |
| 3.3 | Deterministic state hash | deterministic_state_hash | See deterministic-replay-and-merge-spec.md §3.3 (deterministic_state_hash). Covers Deterministic state hash. |
| n/a | 4. Event validation pipeline (deterministic) | 4_event_validation_pipeline_deterministic | See deterministic-replay-and-merge-spec.md (4_event_validation_pipeline_deterministic). Covers 4. Event validation pipeline (deterministic). |
| 4.1 | Envelope validation | envelope_validation | See deterministic-replay-and-merge-spec.md §4.1 (envelope_validation). Covers Envelope validation. |
| 4.2 | Payload validation | payload_validation | See deterministic-replay-and-merge-spec.md §4.2 (payload_validation). Covers Payload validation. |
| 4.3 | Invariant validation (Protocol v5 Section 0) | invariant_validation_protocol_v5_0 | See deterministic-replay-and-merge-spec.md §4.3 (invariant_validation_protocol_v5_0). Covers Invariant validation (Protocol v5 Section 0). |
| 4.4 | Rulebook validation | rulebook_validation | See deterministic-replay-and-merge-spec.md §4.4 (rulebook_validation). Covers Rulebook validation. |
| 4.5 | Deterministic failure semantics | deterministic_failure_semantics | See deterministic-replay-and-merge-spec.md §4.5 (deterministic_failure_semantics). Covers Deterministic failure semantics. |
| n/a | 5. Event application semantics (state transition rules) | 5_event_application_semantics_state_transition_rules | See deterministic-replay-and-merge-spec.md (5_event_application_semantics_state_transition_rules). Covers 5. Event application semantics (state transition rules). |
| 5.1 | General application rules | general_application_rules | See deterministic-replay-and-merge-spec.md §5.1 (general_application_rules). Covers General application rules. |
| 5.2 | Idempotency and replay safety | idempotency_and_replay_safety | See deterministic-replay-and-merge-spec.md §5.2 (idempotency_and_replay_safety). Covers Idempotency and replay safety. |
| 5.3 | Tombstones and reversals | tombstones_and_reversals | See deterministic-replay-and-merge-spec.md §5.3 (tombstones_and_reversals). Covers Tombstones and reversals. |
| n/a | 6. Challenge-driven state transitions | 6_challenge_driven_state_transitions | See deterministic-replay-and-merge-spec.md (6_challenge_driven_state_transitions). Covers 6. Challenge-driven state transitions. |
| 6.1 | Challenge lifecycle state machine (algorithmic view) | challenge_lifecycle_state_machine_algorithmic_view | See deterministic-replay-and-merge-spec.md §6.1 (challenge_lifecycle_state_machine_algorithmic_view). Covers Challenge lifecycle state machine (algorithmic view). |
| 6.2 | Voter eligibility computation | voter_eligibility_computation | See deterministic-replay-and-merge-spec.md §6.2 (voter_eligibility_computation). Covers Voter eligibility computation. |
| 6.3 | Vote aggregation and tally rules | vote_aggregation_and_tally_rules | See deterministic-replay-and-merge-spec.md §6.3 (vote_aggregation_and_tally_rules). Covers Vote aggregation and tally rules. |
| 6.4 | Verdict finalization | verdict_finalization | See deterministic-replay-and-merge-spec.md §6.4 (verdict_finalization). Covers Verdict finalization. |
| 6.5 | Transformation mapping | transformation_mapping | See deterministic-replay-and-merge-spec.md §6.5 (transformation_mapping). Covers Transformation mapping. |
| n/a | 7. Importance ranking derivation (deterministic) | 7_importance_ranking_derivation_deterministic | See deterministic-replay-and-merge-spec.md (7_importance_ranking_derivation_deterministic). Covers 7. Importance ranking derivation (deterministic). |
| 7.1 | Ranking input surfaces | ranking_input_surfaces | See deterministic-replay-and-merge-spec.md §7.1 (ranking_input_surfaces). Covers Ranking input surfaces. |
| 7.2 | Deterministic ranking algorithm | deterministic_ranking_algorithm | See deterministic-replay-and-merge-spec.md §7.2 (deterministic_ranking_algorithm). Covers Deterministic ranking algorithm. |
| 7.3 | Snapshot-anchored rankings | snapshot_anchored_rankings | See deterministic-replay-and-merge-spec.md §7.3 (snapshot_anchored_rankings). Covers Snapshot-anchored rankings. |
| n/a | 8. Snapshot derivation and verification | 8_snapshot_derivation_and_verification | See deterministic-replay-and-merge-spec.md (8_snapshot_derivation_and_verification). Covers 8. Snapshot derivation and verification. |
| 8.1 | Snapshot intervals and triggers | snapshot_intervals_and_triggers | See deterministic-replay-and-merge-spec.md §8.1 (snapshot_intervals_and_triggers). Covers Snapshot intervals and triggers. |
| 8.2 | Snapshot content requirements | snapshot_content_requirements | See deterministic-replay-and-merge-spec.md §8.2 (snapshot_content_requirements). Covers Snapshot content requirements. |
| 8.3 | Snapshot verification procedure | snapshot_verification_procedure | See deterministic-replay-and-merge-spec.md §8.3 (snapshot_verification_procedure). Covers Snapshot verification procedure. |
| 8.4 | Snapshot acceleration rules | snapshot_acceleration_rules | See deterministic-replay-and-merge-spec.md §8.4 (snapshot_acceleration_rules). Covers Snapshot acceleration rules. |
| n/a | 9. Offline logs, Mindseed packages, and reintegration | 9_offline_logs_mindseed_packages_and_reintegration | See deterministic-replay-and-merge-spec.md (9_offline_logs_mindseed_packages_and_reintegration). Covers 9. Offline logs, Mindseed packages, and reintegration. |
| 9.1 | Offline log structure | offline_log_structure | See deterministic-replay-and-merge-spec.md §9.1 (offline_log_structure). Covers Offline log structure. |
| 9.2 | Publication pack composition | publication_pack_composition | See deterministic-replay-and-merge-spec.md §9.2 (publication_pack_composition). Covers Publication pack composition. |
| 9.3 | Reintegration pipeline | reintegration_pipeline | See deterministic-replay-and-merge-spec.md §9.3 (reintegration_pipeline). Covers Reintegration pipeline. |
| 9.4 | Offline merge constraints | offline_merge_constraints | See deterministic-replay-and-merge-spec.md §9.4 (offline_merge_constraints). Covers Offline merge constraints. |
| n/a | 10. Conflicts and convergence across disconnected histories | 10_conflicts_and_convergence_across_disconnected_histories | See deterministic-replay-and-merge-spec.md (10_conflicts_and_convergence_across_disconnected_histories). Covers 10. Conflicts and convergence across disconnected histories. |
| 10.1 | Conflict types | conflict_types | See deterministic-replay-and-merge-spec.md §10.1 (conflict_types). Covers Conflict types. |
| 10.2 | Protocol-consistent conflict handling | protocol_consistent_conflict_handling | See deterministic-replay-and-merge-spec.md §10.2 (protocol_consistent_conflict_handling). Covers Protocol-consistent conflict handling. |
| 10.3 | Deterministic duplicate handling | deterministic_duplicate_handling | See deterministic-replay-and-merge-spec.md §10.3 (deterministic_duplicate_handling). Covers Deterministic duplicate handling. |
| n/a | 11. Lineages, constitutional forks, and continuity | 11_lineages_constitutional_forks_and_continuity | See deterministic-replay-and-merge-spec.md (11_lineages_constitutional_forks_and_continuity). Covers 11. Lineages, constitutional forks, and continuity. |
| 11.1 | Lineage tracking (descriptive) | lineage_tracking_descriptive | See deterministic-replay-and-merge-spec.md §11.1 (lineage_tracking_descriptive). Covers Lineage tracking (descriptive). |
| 11.2 | Breach detection rules | breach_detection_rules | See deterministic-replay-and-merge-spec.md §11.2 (breach_detection_rules). Covers Breach detection rules. |
| 11.3 | Continuity rules under breach | continuity_rules_under_breach | See deterministic-replay-and-merge-spec.md §11.3 (continuity_rules_under_breach). Covers Continuity rules under breach. |
| 11.4 | Recording other lineages and external systems | recording_other_lineages_and_external_systems | See deterministic-replay-and-merge-spec.md §11.4 (recording_other_lineages_and_external_systems). Covers Recording other lineages and external systems. |
| n/a | 12. Conformance requirements | 12_conformance_requirements | See deterministic-replay-and-merge-spec.md (12_conformance_requirements). Covers 12. Conformance requirements. |
| 12.1 | Node conformance | node_conformance | See deterministic-replay-and-merge-spec.md §12.1 (node_conformance). Covers Node conformance. |
| 12.2 | Client conformance | client_conformance | See deterministic-replay-and-merge-spec.md §12.2 (client_conformance). Covers Client conformance. |
| 12.3 | Interoperability test requirements | interoperability_test_requirements | See deterministic-replay-and-merge-spec.md §12.3 (interoperability_test_requirements). Covers Interoperability test requirements. |
| n/a | 13. Test vectors and reference procedures | 13_test_vectors_and_reference_procedures | See deterministic-replay-and-merge-spec.md (13_test_vectors_and_reference_procedures). Covers 13. Test vectors and reference procedures. |
| 13.1 | Replay vectors | replay_vectors | See deterministic-replay-and-merge-spec.md §13.1 (replay_vectors). Covers Replay vectors. |
| 13.2 | Snapshot vectors | snapshot_vectors | See deterministic-replay-and-merge-spec.md §13.2 (snapshot_vectors). Covers Snapshot vectors. |
| 13.3 | Offline reintegration vectors | offline_reintegration_vectors | See deterministic-replay-and-merge-spec.md §13.3 (offline_reintegration_vectors). Covers Offline reintegration vectors. |
| 13.4 | Breach and fork vectors | breach_and_fork_vectors | See deterministic-replay-and-merge-spec.md §13.4 (breach_and_fork_vectors). Covers Breach and fork vectors. |
| n/a | A. Genesis to Single-Idea Replay | a_genesis_to_single_idea_replay | See deterministic-replay-and-merge-spec.md (a_genesis_to_single_idea_replay). Covers A. Genesis to Single-Idea Replay. |
| n/a | B. Basic Offline Publication Pack | b_basic_offline_publication_pack | See deterministic-replay-and-merge-spec.md (b_basic_offline_publication_pack). Covers B. Basic Offline Publication Pack. |
| n/a | C. Duplicate Event Rejection | c_duplicate_event_rejection | See deterministic-replay-and-merge-spec.md (c_duplicate_event_rejection). Covers C. Duplicate Event Rejection. |
| n/a | D. Invalid Signature Rejection | d_invalid_signature_rejection | See deterministic-replay-and-merge-spec.md (d_invalid_signature_rejection). Covers D. Invalid Signature Rejection. |
