---
doc_id: map_encoding_snapshots_bundles
title: Map: Encoding, Snapshots, Bundles
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
  - canonical-encoding-and-hashing-spec.md
  - snapshot-format-v0.md
  - shared-map-and-payload-bundles-spec.md

conflicts:
  - none known

change_rules:
  - If source headings or anchors change, regenerate this map.

reader_path:
  - prereq: authoritative-index.md
  - next: none

keywords:
  - map
  - encoding
  - snapshots
  - bundles
  - navigation
---

Purpose: quick navigation of encoding, snapshot, and bundle format sections and anchors.

## source documents
- canonical-encoding-and-hashing-spec.md
- snapshot-format-v0.md
- shared-map-and-payload-bundles-spec.md

## quick jump table
### canonical-encoding-and-hashing-spec.md
| § | heading | anchor | description |
| --- | --- | --- | --- |
| n/a | Canonical Encoding and Hashing Specification | canonical_encoding_and_hashing_specification | See canonical-encoding-and-hashing-spec.md (canonical_encoding_and_hashing_specification). Covers Canonical Encoding and Hashing Specification. |
| n/a | 0. Purpose and Scope | 0_purpose_and_scope | See canonical-encoding-and-hashing-spec.md (0_purpose_and_scope). Covers 0. Purpose and Scope. |
| n/a | 1. Canonical Byte Model | 1_canonical_byte_model | See canonical-encoding-and-hashing-spec.md (1_canonical_byte_model). Covers 1. Canonical Byte Model. |
| n/a | 2. Primitive Encodings | 2_primitive_encodings | See canonical-encoding-and-hashing-spec.md (2_primitive_encodings). Covers 2. Primitive Encodings. |
| 2.1 | Integers | integers | See canonical-encoding-and-hashing-spec.md §2.1 (integers). Covers Integers. |
| 2.2 | Identifiers (`id`) | identifiers_id | See canonical-encoding-and-hashing-spec.md §2.2 (identifiers_id). Covers Identifiers (`id`). |
| 2.3 | Enums | enums | See canonical-encoding-and-hashing-spec.md §2.3 (enums). Covers Enums. |
| n/a | 3. Text and Payload Canonicalization | 3_text_and_payload_canonicalization | See canonical-encoding-and-hashing-spec.md (3_text_and_payload_canonicalization). Covers 3. Text and Payload Canonicalization. |
| 3.1 | Character Encoding | character_encoding | See canonical-encoding-and-hashing-spec.md §3.1 (character_encoding). Covers Character Encoding. |
| 3.2 | Normalization Policy | normalization_policy | See canonical-encoding-and-hashing-spec.md §3.2 (normalization_policy). Covers Normalization Policy. |
| 3.3 | Payload Hashing | payload_hashing | See canonical-encoding-and-hashing-spec.md §3.3 (payload_hashing). Covers Payload Hashing. |
| n/a | 4. Structured Object Encoding | 4_structured_object_encoding | See canonical-encoding-and-hashing-spec.md (4_structured_object_encoding). Covers 4. Structured Object Encoding. |
| 4.1 | Field Ordering | field_ordering | See canonical-encoding-and-hashing-spec.md §4.1 (field_ordering). Covers Field Ordering. |
| 4.2 | Optional Fields | optional_fields | See canonical-encoding-and-hashing-spec.md §4.2 (optional_fields). Covers Optional Fields. |
| 4.3 | Lists and Repeated Fields | lists_and_repeated_fields | See canonical-encoding-and-hashing-spec.md §4.3 (lists_and_repeated_fields). Covers Lists and Repeated Fields. |
| 4.4 | Section Encoding | section_encoding | See canonical-encoding-and-hashing-spec.md §4.4 (section_encoding). Covers Section Encoding. |
| n/a | 5. Hashing Primitives | 5_hashing_primitives | See canonical-encoding-and-hashing-spec.md (5_hashing_primitives). Covers 5. Hashing Primitives. |
| 5.1 | Hash Algorithm | hash_algorithm | See canonical-encoding-and-hashing-spec.md §5.1 (hash_algorithm). Covers Hash Algorithm. |
| 5.2 | Domain Separation | domain_separation | See canonical-encoding-and-hashing-spec.md §5.2 (domain_separation). Covers Domain Separation. |
| n/a | 6. Merkle Construction Rules | 6_merkle_construction_rules | See canonical-encoding-and-hashing-spec.md (6_merkle_construction_rules). Covers 6. Merkle Construction Rules. |
| 6.1 | Leaf Construction | leaf_construction | See canonical-encoding-and-hashing-spec.md §6.1 (leaf_construction). Covers Leaf Construction. |
| 6.2 | Internal Node Hashing | internal_node_hashing | See canonical-encoding-and-hashing-spec.md §6.2 (internal_node_hashing). Covers Internal Node Hashing. |
| 6.3 | Root Semantics | root_semantics | See canonical-encoding-and-hashing-spec.md §6.3 (root_semantics). Covers Root Semantics. |
| n/a | 7. Canonical Commitments | 7_canonical_commitments | See canonical-encoding-and-hashing-spec.md (7_canonical_commitments). Covers 7. Canonical Commitments. |
| 7.1 | Section Hashes | section_hashes | See canonical-encoding-and-hashing-spec.md §7.1 (section_hashes). Covers Section Hashes. |
| 7.2 | `state_root_hash` | state_root_hash | See canonical-encoding-and-hashing-spec.md §7.2 (state_root_hash). Covers `state_root_hash`. |
| 7.3 | Payload Roots | payload_roots | See canonical-encoding-and-hashing-spec.md §7.3 (payload_roots). Covers Payload Roots. |
| 7.4 | Canonical publication artifacts | canonical_publication_artifacts | See canonical-encoding-and-hashing-spec.md §7.4 (canonical_publication_artifacts). Covers Canonical publication artifacts. |
| n/a | 8. Shared Map Commitment | 8_shared_map_commitment | See canonical-encoding-and-hashing-spec.md (8_shared_map_commitment). Covers 8. Shared Map Commitment. |
| n/a | 9. Ordering and Comparison Semantics | 9_ordering_and_comparison_semantics | See canonical-encoding-and-hashing-spec.md (9_ordering_and_comparison_semantics). Covers 9. Ordering and Comparison Semantics. |
| 9.1 | Identifier Ordering | identifier_ordering | See canonical-encoding-and-hashing-spec.md §9.1 (identifier_ordering). Covers Identifier Ordering. |
| 9.2 | Bytewise Ordering | bytewise_ordering | See canonical-encoding-and-hashing-spec.md §9.2 (bytewise_ordering). Covers Bytewise Ordering. |
| 9.3 | Stability Guarantees | stability_guarantees | See canonical-encoding-and-hashing-spec.md §9.3 (stability_guarantees). Covers Stability Guarantees. |
| n/a | 10. Validation and Rejection Rules | 10_validation_and_rejection_rules | See canonical-encoding-and-hashing-spec.md (10_validation_and_rejection_rules). Covers 10. Validation and Rejection Rules. |
| 10.1 | Hard Rejection Conditions | hard_rejection_conditions | See canonical-encoding-and-hashing-spec.md §10.1 (hard_rejection_conditions). Covers Hard Rejection Conditions. |
| 10.2 | Soft Rejection and Forward Compatibility | soft_rejection_and_forward_compatibility | See canonical-encoding-and-hashing-spec.md §10.2 (soft_rejection_and_forward_compatibility). Covers Soft Rejection and Forward Compatibility. |
| 10.3 | Validation Scope | validation_scope | See canonical-encoding-and-hashing-spec.md §10.3 (validation_scope). Covers Validation Scope. |
| n/a | 11. Security and Adversarial Considerations | 11_security_and_adversarial_considerations | See canonical-encoding-and-hashing-spec.md (11_security_and_adversarial_considerations). Covers 11. Security and Adversarial Considerations. |
| 11.1 | Ambiguity Attacks | ambiguity_attacks | See canonical-encoding-and-hashing-spec.md §11.1 (ambiguity_attacks). Covers Ambiguity Attacks. |
| 11.2 | Cross-Implementation Drift | cross_implementation_drift | See canonical-encoding-and-hashing-spec.md §11.2 (cross_implementation_drift). Covers Cross-Implementation Drift. |
| 11.3 | Offline Partition Risks | offline_partition_risks | See canonical-encoding-and-hashing-spec.md §11.3 (offline_partition_risks). Covers Offline Partition Risks. |
| 11.4 | Payload Withholding | payload_withholding | See canonical-encoding-and-hashing-spec.md §11.4 (payload_withholding). Covers Payload Withholding. |
| n/a | 12. Conformance Requirements | 12_conformance_requirements | See canonical-encoding-and-hashing-spec.md (12_conformance_requirements). Covers 12. Conformance Requirements. |
| 12.1 | Required Capabilities | required_capabilities | See canonical-encoding-and-hashing-spec.md §12.1 (required_capabilities). Covers Required Capabilities. |
| 12.2 | Optional Capabilities | optional_capabilities | See canonical-encoding-and-hashing-spec.md §12.2 (optional_capabilities). Covers Optional Capabilities. |
| 12.3 | Conformance Testing | conformance_testing | See canonical-encoding-and-hashing-spec.md §12.3 (conformance_testing). Covers Conformance Testing. |
| 12.4 | Evolution and Stability | evolution_and_stability | See canonical-encoding-and-hashing-spec.md §12.4 (evolution_and_stability). Covers Evolution and Stability. |
| n/a | 13. Versioning and Evolution | 13_versioning_and_evolution | See canonical-encoding-and-hashing-spec.md (13_versioning_and_evolution). Covers 13. Versioning and Evolution. |
| 13.1 | Version Identification | version_identification | See canonical-encoding-and-hashing-spec.md §13.1 (version_identification). Covers Version Identification. |
| 13.2 | Backward Compatibility | backward_compatibility | See canonical-encoding-and-hashing-spec.md §13.2 (backward_compatibility). Covers Backward Compatibility. |
| 13.3 | Forward Evolution | forward_evolution | See canonical-encoding-and-hashing-spec.md §13.3 (forward_evolution). Covers Forward Evolution. |
| 13.4 | Deprecation | deprecation | See canonical-encoding-and-hashing-spec.md §13.4 (deprecation). Covers Deprecation. |
| n/a | Appendix A. Example Encodings (Non-Normative) | appendix_a_example_encodings_non_normative | See canonical-encoding-and-hashing-spec.md (appendix_a_example_encodings_non_normative). Covers Appendix A. Example Encodings (Non-Normative). |
| n/a | A.1 Identifier Encoding Example | a_1_identifier_encoding_example | See canonical-encoding-and-hashing-spec.md (a_1_identifier_encoding_example). Covers A.1 Identifier Encoding Example. |
| n/a | A.2 Payload Canonicalization Example | a_2_payload_canonicalization_example | See canonical-encoding-and-hashing-spec.md (a_2_payload_canonicalization_example). Covers A.2 Payload Canonicalization Example. |
| n/a | Appendix B. Example Merkle Construction (Non-Normative) | appendix_b_example_merkle_construction_non_normative | See canonical-encoding-and-hashing-spec.md (appendix_b_example_merkle_construction_non_normative). Covers Appendix B. Example Merkle Construction (Non-Normative). |
| n/a | B.1 Leaf Example | b_1_leaf_example | See canonical-encoding-and-hashing-spec.md (b_1_leaf_example). Covers B.1 Leaf Example. |
| n/a | B.2 Internal Node Example | b_2_internal_node_example | See canonical-encoding-and-hashing-spec.md (b_2_internal_node_example). Covers B.2 Internal Node Example. |
| n/a | Appendix C. Known Pitfalls and Explicit Non-Goals (Non-Normative) | appendix_c_known_pitfalls_and_explicit_non_goals_non_normative | See canonical-encoding-and-hashing-spec.md (appendix_c_known_pitfalls_and_explicit_non_goals_non_normative). Covers Appendix C. Known Pitfalls and Explicit Non-Goals (Non-Normative). |
| n/a | C.1 Known Pitfalls | c_1_known_pitfalls | See canonical-encoding-and-hashing-spec.md (c_1_known_pitfalls). Covers C.1 Known Pitfalls. |
| n/a | C.2 Explicit Non-Goals | c_2_explicit_non_goals | See canonical-encoding-and-hashing-spec.md (c_2_explicit_non_goals). Covers C.2 Explicit Non-Goals. |
| n/a | Appendix D: Conformance Test Vectors (Normative) | appendix_d_conformance_test_vectors_normative | See canonical-encoding-and-hashing-spec.md (appendix_d_conformance_test_vectors_normative). Covers Appendix D: Conformance Test Vectors (Normative). |
| n/a | D.1 Primitive Encodings | d_1_primitive_encodings | See canonical-encoding-and-hashing-spec.md (d_1_primitive_encodings). Covers D.1 Primitive Encodings. |
| n/a | D.2 Domain-Separated Hashing | d_2_domain_separated_hashing | See canonical-encoding-and-hashing-spec.md (d_2_domain_separated_hashing). Covers D.2 Domain-Separated Hashing. |
| n/a | D.3 Merkle Tree Constructions | d_3_merkle_tree_constructions | See canonical-encoding-and-hashing-spec.md (d_3_merkle_tree_constructions). Covers D.3 Merkle Tree Constructions. |

### snapshot-format-v0.md
| § | heading | anchor | description |
| --- | --- | --- | --- |
| n/a | 0. scope and non-goals | none | See snapshot-format-v0.md (no_anchor). Covers 0. scope and non-goals. |
| n/a | scope | none | See snapshot-format-v0.md (no_anchor). Covers scope. |
| n/a | non-goals | none | See snapshot-format-v0.md (no_anchor). Covers non-goals. |
| n/a | 1. snapshot identity (block height, snapshot tiers) | none | See snapshot-format-v0.md (no_anchor). Covers 1. snapshot identity (block height, snapshot tiers). |
| 1.1 | canonical identity | none | See snapshot-format-v0.md §1.1 (no_anchor). Covers canonical identity. |
| 1.2 | snapshot tier | none | See snapshot-format-v0.md §1.2 (no_anchor). Covers snapshot tier. |
| 1.3 | block-height basis | none | See snapshot-format-v0.md §1.3 (no_anchor). Covers block-height basis. |
| n/a | 2. Commitment Hierarchy | none | See snapshot-format-v0.md (no_anchor). Covers 2. Commitment Hierarchy. |
| n/a | 3. header schema (fields, types, canonical order) | none | See snapshot-format-v0.md (no_anchor). Covers 3. header schema (fields, types, canonical order). |
| 3.1 | primitive types (used in header) | none | See snapshot-format-v0.md §3.1 (no_anchor). Covers primitive types (used in header). |
| 3.2 | header fields (canonical order) | none | See snapshot-format-v0.md §3.2 (no_anchor). Covers header fields (canonical order). |
| n/a | 4. body schema (sections/tables, canonical ordering) | none | See snapshot-format-v0.md (no_anchor). Covers 4. body schema (sections/tables, canonical ordering). |
| 4.1 | section IDs (v0) | none | See snapshot-format-v0.md §4.1 (no_anchor). Covers section IDs (v0). |
| 4.1.1 | derived state packs: ranks | none | See snapshot-format-v0.md §4.1.1 (no_anchor). Covers derived state packs: ranks. |
| 4.2 | canonical record ordering | none | See snapshot-format-v0.md §4.2 (no_anchor). Covers canonical record ordering. |
| 4.3 | section schemas | none | See snapshot-format-v0.md §4.3 (no_anchor). Covers section schemas. |
| n/a | 5. embedded text rules (title + sentence tier) | none | See snapshot-format-v0.md (no_anchor). Covers 5. embedded text rules (title + sentence tier). |
| n/a | 6. canonical serialization rules (byte-level) | none | See snapshot-format-v0.md (no_anchor). Covers 6. canonical serialization rules (byte-level). |
| 6.1 | integer encoding | none | See snapshot-format-v0.md §6.1 (no_anchor). Covers integer encoding. |
| 6.2 | string, id, and bytes encoding | none | See snapshot-format-v0.md §6.2 (no_anchor). Covers string, id, and bytes encoding. |
| 6.3 | boolean and enum encoding | none | See snapshot-format-v0.md §6.3 (no_anchor). Covers boolean and enum encoding. |
| 6.4 | list encoding | none | See snapshot-format-v0.md §6.4 (no_anchor). Covers list encoding. |
| 6.5 | ordering and determinism | none | See snapshot-format-v0.md §6.5 (no_anchor). Covers ordering and determinism. |
| 6.6 | compression and transport wrappers | none | See snapshot-format-v0.md §6.6 (no_anchor). Covers compression and transport wrappers. |
| n/a | 7. hashing rules (snapshot_hash, state_root_hash, payload roots) | none | See snapshot-format-v0.md (no_anchor). Covers 7. hashing rules (snapshot_hash, state_root_hash, payload roots). |
| 7.1 | section_hash | none | See snapshot-format-v0.md §7.1 (no_anchor). Covers section_hash. |
| 7.2 | state_root_hash | none | See snapshot-format-v0.md §7.2 (no_anchor). Covers state_root_hash. |
| 7.2.1 | active_rulebook_set_hash | none | See snapshot-format-v0.md §7.2.1 (no_anchor). Covers active_rulebook_set_hash. |
| 7.3 | title_sentence_payload_root (Tier 0 root) | none | See snapshot-format-v0.md §7.3 (no_anchor). Covers title_sentence_payload_root (Tier 0 root). |
| 7.4 | snapshot_hash | none | See snapshot-format-v0.md §7.4 (no_anchor). Covers snapshot_hash. |
| n/a | 8. verification procedure | none | See snapshot-format-v0.md (no_anchor). Covers 8. verification procedure. |
| n/a | 9. extension and compatibility rules | none | See snapshot-format-v0.md (no_anchor). Covers 9. extension and compatibility rules. |
| 9.1 | forward compatibility | none | See snapshot-format-v0.md §9.1 (no_anchor). Covers forward compatibility. |
| 9.2 | backward compatibility | none | See snapshot-format-v0.md §9.2 (no_anchor). Covers backward compatibility. |
| 9.3 | schema evolution | none | See snapshot-format-v0.md §9.3 (no_anchor). Covers schema evolution. |
| n/a | 10. conformance fixtures (format + required vectors; values may be TODO) | none | See snapshot-format-v0.md (no_anchor). Covers 10. conformance fixtures (format + required vectors; values may be TODO). |
| n/a | A. Minimal Genesis Snapshot (Height 0) | none | See snapshot-format-v0.md (no_anchor). Covers A. Minimal Genesis Snapshot (Height 0). |
| n/a | B. Single-Idea Snapshot | none | See snapshot-format-v0.md (no_anchor). Covers B. Single-Idea Snapshot. |
| n/a | C. Multi-Idea with Connection | none | See snapshot-format-v0.md (no_anchor). Covers C. Multi-Idea with Connection. |

### shared-map-and-payload-bundles-spec.md
| § | heading | anchor | description |
| --- | --- | --- | --- |
| n/a | Deterministic Text Availability, Distribution, and Verification | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers Deterministic Text Availability, Distribution, and Verification. |
| n/a | 0. Purpose | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 0. Purpose. |
| n/a | 1. Design principles | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 1. Design principles. |
| n/a | 2. Canonical text payload model (recap) | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 2. Canonical text payload model (recap). |
| n/a | 3. Bundle tiers (standardized distribution artifacts) | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 3. Bundle tiers (standardized distribution artifacts). |
| 3.1 | Tier 0 - "Pocket Map" | none | See shared-map-and-payload-bundles-spec.md Section 3.1 (no_anchor). Covers Tier 0 - "Pocket Map". |
| 3.2 | Tier 1 - "Citizen Map" | none | See shared-map-and-payload-bundles-spec.md Section 3.2 (no_anchor). Covers Tier 1 - "Citizen Map". |
| 3.3 | Tier 2 - "Civic Archive" | none | See shared-map-and-payload-bundles-spec.md Section 3.3 (no_anchor). Covers Tier 2 - "Civic Archive". |
| 3.4 | Tier 3 - "Full Archive" | none | See shared-map-and-payload-bundles-spec.md Section 3.4 (no_anchor). Covers Tier 3 - "Full Archive". |
| n/a | Publication and Retention Schedule | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Defines bundle publication cadence variables and retention policy, referencing snapshot and pack cadence rules. |
| n/a | 4. Deterministic bundle selection rules | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 4. Deterministic bundle selection rules. |
| n/a | 5. Shared map commitment | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 5. Shared map commitment. |
| n/a | 6. Snapshots and mandatory embedded text | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 6. Snapshots and mandatory embedded text. |
| n/a | 7. Custody, replication, and redundancy | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 7. Custody, replication, and redundancy. |
| 7.1 | Node classes and obligations | none | See shared-map-and-payload-bundles-spec.md §7.1 (no_anchor). Covers Node classes and obligations. |
| 7.2 | Custody manifests | none | See shared-map-and-payload-bundles-spec.md §7.2 (no_anchor). Covers Custody manifests. |
| 7.3 | Redundancy targets | none | See shared-map-and-payload-bundles-spec.md §7.3 (no_anchor). Covers Redundancy targets. |
| n/a | 8. Transport independence | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 8. Transport independence. |
| n/a | 9. Offline and authoritarian environments | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 9. Offline and authoritarian environments. |
| n/a | 10. Non-goals | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 10. Non-goals. |
| n/a | 11. Summary | none | See shared-map-and-payload-bundles-spec.md (no_anchor). Covers 11. Summary. |
