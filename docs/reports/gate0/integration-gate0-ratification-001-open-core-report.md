# INTEGRATION-GATE0-RATIFICATION-001 Open Core report

## Status

The Gate 0 ratification manifest for DEC-054 through DEC-064 is reconciled into the approved Open Core documentation surfaces.

This batch changes documentation authority and boundary language only. It does not implement runtime behavior, modify canonical data, generate or import Seed data, add events or object schemas, alter migrations, or authorize genesis, signing, bundles, deployment, or launch.

## Ratified decisions

### DEC-054 — graph-native authority transition

- Before genesis, the checked-in specifications remain the ratified source.
- Genesis commits their operative meaning through ordinary protocol and rulebook ideas, descriptions, connections, and canonical bootstrap history.
- After genesis, the canonical event log and replay-derived active graph-native rulebook commitments are authoritative.
- Markdown specifications become height- and commitment-bound projections. Frozen source documents remain hash-addressed provenance archives, not a competing authority.

### DEC-055 — rulebooks use the ordinary graph

- A rulebook is an ordinary actionable idea interpreted with ordinary descriptions and connections.
- `rulebook_id` is the rulebook idea identifier.
- Version/hash and active-set records are commitments or replay-derived indexes, not a separate Rule object family.
- No rulebook-only create or activation event is introduced.
- Relative importance supplies default relevance navigation. An optional Ordering/Vine may provide an authored reading path but does not determine authority or create a folder hierarchy.

### DEC-056 — complete Genesis Seed Package

- Genesis uses a complete ratified package rather than a minimal record count or an 8,101-row quota.
- The final count is an audit result of the ratified scope.
- Graph-native protocol material is authored as complete idea-native semantic units.
- Exact source bytes and reconstruction proofs remain provenance evidence.
- The bootstrap sequence is the genesis boundary; `genesis` is not a new canonical event type.

### DEC-057 — canonical record versus external control

- An operator may control its own company, repositories, money, hosting, deployment, and other external resources.
- Roles, delegations, intended decisions, actions, completion claims, and evidence use the ordinary graph.
- Canonical decisions create no special control over external property and cannot compel implementation.

### DEC-058 — governance activation requires implementation

- A successful governance verdict makes a rule change eligible.
- Activation also requires a qualifying implementation-completion claim and required evidence.
- The rule becomes active only at its computed cycle boundary.
- Snapshot boundaries do not activate governance.

### DEC-059 — private Mindseed journal

- The private Mindseed journal may use the standard idea-compatible shapes.
- It remains user-controlled noncanonical product state outside protocol conformance and replay.
- Its owner may edit, delete, prune, compact, reorder, or replace its records.
- The append-only local publication log is a separate lane containing exact signed candidates already approved for possible publication.

### DEC-060 — publication creates canonical copies

- Publication is not a visibility toggle and does not promote private objects in place.
- It creates new canonical identifiers and events from an exact reviewed candidate.
- Private identifiers, prompts, edit history, and rejected drafts remain private unless separately approved.
- Queue order and dependency grouping are private product state.
- Each canonical event remains atomic and subject to ordinary rate limits.
- A product may later relay unchanged human-approved and signed bytes automatically; changed bytes require new approval and signature.

### DEC-061 — no parallel public human graph

- Persistent public human-authored graph content must use canonical publication.
- Nodes must not host publicly readable noncanonical human maps as a parallel public graph.
- Private and directly permissioned sharing may remain noncanonical outside the public graph.
- Tribe material remains public canonical material when published.

### DEC-062 — shared Public AI mode

- A shared publicly readable AI realm is permitted as explicitly noncanonical product state.
- Prompted generation and product-defined autonomous generation may enter the same realm with preserved provenance.
- Generated ideas, connections, Orderings, and realm-local ranks may be shared after structural and safety validation.
- Raw human prompts do not become map-authored ideas merely because they initiated generation.
- Product policy may apply an importance-over-cycles lifecycle to deprioritize, hide, burn, or prune noncanonical AI output without affecting canonical lifecycle history.

### DEC-063 — model profiles are not Protocol Identities

- Products may expose stable model or agent profiles, identifiers, provenance histories, maps, and relative lenses.
- These records create no Protocol Identity, human verification, canonical standing, voting power, sponsorship power, token rights, or governance role.

### DEC-064 — simulated AI challenges remain realm-local

- A human may request noncanonical simulated challenges, arguments, importance changes, and equal-weight model votes.
- A labeled model majority, simulated verdict, rank, or Ordering may update the noncanonical AI realm.
- None of these results is a canonical verdict, canonical rank, human decision, or authority.

## Files changed

- `docs/authoritative-index.md`
- `docs/protocol v5.md`
- `docs/protocol v5-appendix-a.md`
- `docs/governance-spec.md`
- `docs/offline-and-mindseed-spec.md`
- `docs/ai-boundaries-spec.md`
- `docs/canonical-encoding-and-hashing-spec.md`
- `docs/deterministic-replay-and-merge-spec.md`
- `docs/open-core-split-and-data-boundary-spec.md`
- this report

## Verification

- Tempo/Cycle conformance: 29/29 passed.
- Native Ordering conformance: 10/10 passed.
- Open Core frontend boundary check: passed.
- Open Core backend boundary check: passed.
- Focused contradiction scans found no remaining special Rule object, snapshot-boundary activation, optional rule implementation, or public-readable noncanonical human-map wording in the changed authority surfaces.
- Markdown fence parity and control-character scans passed.
- Scoped `git diff --check` passed after removing newly introduced trailing whitespace.

## Preserved and deferred

- No runtime, migration, API, DTO, frontend, event-registry, snapshot, conformance-fixture, Seed candidate, identity-admission, Tempo, coordination, staging, commit, push, or PR surface was changed.
- Existing V4 pilot, Tempo, identity-admission, importer, and unrelated dirty work remains untouched.
- Detailed product behavior for private journals, publication queues, Public AI lifecycle/retention, model routing, and simulation UX remains private-product owned.
- Runtime implementation and Seed regeneration against these ratified semantics require separately registered work.
