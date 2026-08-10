# Seed conformance-binding vectors

These fixtures define the executable pre-genesis contract for representation slots,
canonical-vocabulary binding, and representation authorship.

- `representation_kind` is explicitly `title` or `description`.
- A target has one separate title slot and twelve description cells: three lengths by
  four complexities.
- Title representations omit length, complexity, and vocabulary fields.
- Canonical-complexity descriptions carry one explicit `vocabulary_version_id`;
  every other description omits it.
- Numeric storage codes and the historical `tier_length = title` form are not accepted
  canonical event values.
- `author_identity_id` equals the event speaker and both author and vocabulary ideas
  exist before use.
- Canonical JSON/BLAKE3 vectors prove authorship and vocabulary bindings change the
  committed event payload.

Run `npm run conformance:seed-bindings`. The JavaScript harness verifies the schema,
all twelve cells, stateful pre-use rules, and canonical JSON bytes. The Rust event-log
test reads the same file and verifies the live validator plus exact BLAKE3 digests.
