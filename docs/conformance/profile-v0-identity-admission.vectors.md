# Profile-v0 Identity Admission Static Conformance Requirements

Status: static contract inventory. This document and its JSON companion are not a claim that the current runtime executes Profile-v0 identity admission.

`profile-v0-identity-admission.vectors.json`, validated by `profile-v0-identity-admission.schema.json`, provides the required static cases for the later conformance harness. The source-of-truth semantics remain Appendix A, the Profile-v0 signature and encoding specifications, deterministic replay, verification, cycle, and snapshot specifications.

## Coverage

The vector inventory covers sponsor authorship, absent speaker, fixed human target, optional `verification_reference` encoding, reduced authorization commitment, non-recursive applicant proof, completed-payload sponsor signature, identity and key uniqueness, four structural roots, atomic capacity debit and retry idempotence, validation-error distinction, one-active-key lifecycle, compatibility-only `identity_verification_update`, qualifying-period/liveness behavior, legacy provenance, and restricted verification scope.

`pure_crypto_fixtures` contains fixed public test keys, payload bytes, commitments, proofs, and signatures for the database-free P2 harness. It covers the byte-level and structural portions of IA-002 through IA-013 and IA-025. The fixture contains no production or personal key material. `static_contract_complete` rows are schema/contract assertions that can be reviewed and parsed now. `runtime_harness_required` rows still require canonical state application, replay, snapshot, or route execution and therefore must not be reported as executed until those runtime slices exist.

## Execution boundary

No vector may use a private account, private request, private evidence, relay-local data, operator decision, wall clock, AI output, or a database-specific row order as an input to canonical validity. The later harness must preserve Appendix A validation precedence and must prove that failed or duplicate events do not debit capacity or mutate admission state.
