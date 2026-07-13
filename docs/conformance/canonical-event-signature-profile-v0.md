# Canonical Event Signature Profile v0 Conformance Vectors

Status: authoritative conformance artifact.

The companion JSON file `canonical-event-signature-profile-v0.vectors.json` contains deterministic public test vectors for ordinary human-authored Profile-v0 canonical event candidates.

The vectors cover:

- valid Ed25519 Profile-v0 signatures;
- invalid signatures and candidate mutation after signing;
- altered payload hash, event type, author identity, and speaker identity;
- wrong key owner, unknown key, and revoked-key cases;
- historical validity before later revocation;
- malformed public-key and signature encodings;
- unsupported signature profiles;
- exact signed-candidate bytes, `public_key_ref`, and `authored_candidate_hash_v0`;
- publication wrapper mutation;
- assignment of a canonical position without changing authored bytes;
- attempted inclusion of publication-assigned `event_index` in signed bytes.

The fixture intentionally contains public deterministic test keys and signatures only. It does not contain production keys, personal keys, private account data, private-key custody behavior, or wallet UX semantics.
