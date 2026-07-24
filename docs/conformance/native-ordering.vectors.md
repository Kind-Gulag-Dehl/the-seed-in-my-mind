# Native Ordering conformance vectors

These vectors are the executable DEC-043 contract for the one authored Ordering substrate.

- `vine` is profile code `0`.
- `evidence_rail` is profile code `1`.
- `action_rail` is profile code `2`.
- `ordering_create` and `ordering_fork` are the only Ordering lifecycle events.
- Canonical event payloads use the named profile strings; numeric profile codes are storage/snapshot encodings only.
- Every fork repeats the base Ordering profile.
- Vine metadata is valid only for the Vine profile.
- Superseded `rail_*` events and an Ordering-specific representation alias are rejected.

Run `npm run conformance:ordering`. The JavaScript harness checks the fixture schema and stateful fork semantics. The Rust event-log test reads the same file and checks the canonical validator plus exact canonical JSON/BLAKE3 bytes.
