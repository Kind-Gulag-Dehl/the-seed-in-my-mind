---
doc_id: api_contract_read_only
title: API Contract (Read Only)
status: derived
version: v0
last_reviewed: 2026-04-01

scope:
  - Current public read-only runtime contract snapshot for the open-core package.
  - Forward-looking read routes are included only when explicitly labeled as spec/future.

authoritative_for:
  - Implementation reference only (must track authoritative specs).

not_authoritative_for:
  - Canonical semantics, encodings, or node behaviors.

depends_on:
  - protocol v5.md
  - canonical-encoding-and-hashing-spec.md

conflicts:
  - none known

change_rules:
  - If endpoints or field definitions change in authoritative specs, update this doc to match.

reader_path:
  - prereq: protocol v5.md
  - next: none

keywords:
  - API
  - contract
  - read-only
---

# Read-Only API Contract v0

## 0. Purpose and Scope

This document defines the current public read-only HTTP API contract for the open-core runtime's canonical data access surface.

Endpoints explicitly marked as implemented below are the current open-core runtime routes exposed by `backend/bins/api-server/src/server/router.rs`. Any sections explicitly labeled as spec/future are informative only and are not part of the current open-core runtime contract or conformance surface.

Canonical read/write policy reference: `protocol v5.md` (`canonical_read_write_access_policy`).

The purpose of this API is to enable unauthenticated access to verifiable canonical state and derived views at deterministic snapshot boundaries (block heights), facilitating:

* Client-side verification of shared canonical reality through exposure of the `shared_map_commitment`.
* Efficient exploration of the idea graph via bounded neighborhoods and ranked listings.
* Bootstrapping of new participants with meaningful human-readable content (Tier 0 title and sentence payloads).

All responses MUST be derived exclusively from deterministic replay of the append-only event log, materialized at verified snapshot boundaries. Snapshot height always refers to **block height**; any cycle-related fields are advisory metadata only and MUST NOT alter snapshot identity or validity. The API MUST preserve the protocol's core invariants: single canonical truth, human-first authorship traceability, and strict separation of canonical facts from non-canonical derived views.

This contract is strictly scoped to read-only operations for the public canonical surface. It introduces no authentication mechanisms, no write paths, and no exposure of private sandbox subsystems in the canonical API. Implementations MAY additionally expose authenticated, non-canonical private drafting and AI-map helper APIs that are explicitly labeled below and remain outside the deterministic canonical log.

This scope does not change canonical write policy. Canonical writes are handled on versioned write surfaces (for example, `/api/v1/canonical/*`) and are authentication + canonical-writer-verification gated; canonical reads remain publicly readable without authentication.

Explicit non-goals (for the public canonical read-only surface):
* Real-time event streaming or subscription.
* Rate limiting or abuse mitigation details (implementation-specific).
* Authentication or access control.
* Exposure of full payload bytes beyond Tier 0 embedding.
* Mutation or draft management.

## 1. General Requirements and Invariants

Conformant implementations MUST adhere to the following for the public read-only surface:

* All endpoints are HTTP GET only.
* No authentication or authorization is required or supported.
* Every response MUST include the basis snapshot's `shared_map_commitment` in headers for client-side verification against trusted sources.
* Responses MUST be deterministic: identical requests against the same snapshot yield byte-identical JSON (excluding non-authoritative fields like server timestamps).
* Ordering in list responses MUST be stable and deterministic. Each list endpoint MUST define a primary sort key; ties MUST break by canonical log order `(block_height, event_index)` derived from the canonical event. Identifiers MUST NOT be used as canonical ordering sources or tie-breakers.
* Pagination MUST use offset/limit or cursor-based mechanisms with deterministic continuation guarantees.
* Errors MUST return appropriate HTTP status codes with a JSON body containing `error_code` and `message`.
* Caching MUST be supported via ETag derived from `snapshot_hash` and appropriate Cache-Control headers.
* All numeric identifiers and quantities MUST be transmitted as base-10 decimal strings (JSON bodies, query parameters, and headers) to avoid precision loss.
* All identifier fields in wire format MUST be UUIDv7 strings (lowercase hex with hyphens) unless explicitly labeled as a hash/commitment. Exception: `snapshot_id` is defined as `hex(snapshot_hash)` per Snapshot Format v0.
* Snapshot height always refers to block height; cycle data (if provided) is metadata only.
* Responses MUST NOT expose civil identity by default; author attribution is a pseudonymous author identity plus a non-identifying verification level (persona attachment is optional if explicitly set).

Implementations MUST reject any request implying mutation on the public read-only surface and return 405 Method Not Allowed.

## 1.1 Pagination and Ordering

List endpoints MUST define a primary sort key when ordering is exposed (e.g., derived rank). Regardless of primary sort key, the final stable tie-breaker MUST be canonical log order `(block_height, event_index)` derived from the relevant canonical event (e.g., `created_event_id`). Identifiers (idea_id, connection_id, event_id, etc.) MUST NOT be used as ordering sources or tie-breakers.

Ordering defaults:
* `/ideas/top` uses `derived_universal_rank` **ascending** by default (rank `1` appears first). Clients MAY request `order=desc` to reverse the list deterministically.

Paginated list responses MUST be evaluated against a specific basis: an explicit snapshot reference if the endpoint provides it, otherwise the server's current canonical head. Stable paging assumes a stable basis. `ETag` and `snapshot_hash` MAY be used for caching and client-side stability, but `snapshot_hash` is derived and non-canonical.

Cursor pagination (when used) MUST follow:
* Request query parameters: `limit` (int), `cursor` (opaque string, optional).
* Response fields: `items: [...]`, `next_cursor` (string|null).
* The cursor MUST encode the last-seen ordering position at minimum `(block_height, event_index)` plus any primary sort key if used; the cursor format is opaque to clients.

Given the same basis and unchanged underlying data, repeated calls with the same cursor MUST return the same page. If the basis changes mid-pagination, the server MAY reject with 409 Conflict and require the client to restart using a new basis.

## 2. Common Response Headers

All successful responses MUST include the following headers:

| Header Name                | Value Type | Description                                                                 |
|----------------------------|------------|-----------------------------------------------------------------------------|
| X-Shared-Map-Commitment    | hex string | The authoritative `shared_map_commitment` of the basis snapshot.            |
| X-Snapshot-Height          | decimal string | Block height of the basis snapshot.                                     |
| X-State-Root-Hash          | hex string | `state_root_hash` over canonical facts.                                     |
| X-Title-Sentence-Payload-Root | hex string | `title_sentence_payload_root` over embedded Tier 0 payloads (equal to `pocket_map_payload_root`). |
| ETag                       | hex string | `snapshot_hash` for conditional requests and caching.                       |
| Cache-Control              | string     | Implementation-defined; RECOMMENDED: max-age=300 for recent snapshots.      |

All numeric header values are encoded as base-10 decimal strings.

## 3. Data Types and DTOs

Responses use JSON objects with the following common schemas (fields are normative unless marked optional):

### IdeaSummary
```json
{
  "idea_id": "UUIDv7 string",
  "idea_type": "string (enum: truth_claim | conceptual_idea | actionable_idea | action | identity)",
  "speaker_identity_id": "UUIDv7 string",
  "created_event_id": "UUIDv7 string",
  "title": "string",
  "sentence": "string | null (derived, non-canonical; from canonical representation)",
  "derived_universal_rank": "decimal string | null (derived, non-canonical)",
  "author": {
    "author_identity_id": "UUIDv7 string | null (derived; equals speaker_identity_id unless proxy)",
    "verification_level": "string | null (derived, non-canonical)",
    "persona_id": "UUIDv7 string | null (optional, derived, non-canonical)"
  }
}
```
`sentence`, `derived_universal_rank`, and the `author` block are derived, non-canonical convenience fields and MAY be omitted or null.

### ConnectionSummary
```json
{
  "connection_id": "UUIDv7 string",
  "from_idea_id": "UUIDv7 string",
  "to_idea_id": "UUIDv7 string",
  "connection_type": "string (enum: same_as | relative_importance | membership)",
  "created_by_event_id": "UUIDv7 string",
  "usage": "string (required if connection_type = relative_importance)",
  "axis": "string (required if connection_type = relative_importance)",
  "timeframe": "string (required if connection_type = relative_importance)",
  "scope": "string (required if connection_type = relative_importance)",
  "value_representation": "string | null (optional)",
  "certainty_band": "string | null (optional)",
  "weight": "decimal string | null (optional, derived; for relative_importance)"
}
```

### OverlayConnectionSummary
Reserved for the spec/future overlay endpoints in section 4.11. It is not currently used by the open-core runtime router.
```json
{
  "scope_kind": "string (enum: universal | tribe | personal)",
  "anchor_id": "UUIDv7 string | string constant (rulebook-defined universal anchor)",
  "from_idea_id": "UUIDv7 string",
  "to_idea_id": "UUIDv7 string",
  "usage": "string",
  "axis": "string",
  "timeframe": "string",
  "value_representation": "string | null (optional)",
  "certainty_band": "string | null (optional)",
  "weight": "decimal string | null (optional, derived)",
  "source_event_id": "UUIDv7 string"
}
```

### ScopedDisplayOverrideSummary
Reserved for the spec/future overlay endpoints in section 4.11. It is not currently used by the open-core runtime router.
```json
{
  "scope_kind": "string (enum: universal | tribe | personal)",
  "anchor_id": "UUIDv7 string | string constant (rulebook-defined universal anchor)",
  "target_kind": "string (enum: idea | rail)",
  "target_id": "UUIDv7 string",
  "display_slot_key": "string",
  "representation_id": "UUIDv7 string",
  "source_event_id": "UUIDv7 string"
}
```

### IdeaDetail (extends IdeaSummary)
```json
{
  ...IdeaSummary fields,
  "derived_universal_axis_ranks": "{ axis_name: decimal string } | null (derived, non-canonical)",
  "payload_hash": "hex string (Tier 0)",
  "incoming_connections": "[ConnectionSummary]",
  "outgoing_connections": "[ConnectionSummary]"
}
```
When included, `derived_universal_rank` represents the overall universal rank for the basis snapshot, and `derived_universal_axis_ranks` provides the 20-axis universal rank values for the same basis. If the basis is the latest snapshot, these values represent the current universal ranks.

### NeighborhoodResponse
```json
{
  "central_idea": "IdeaDetail",
  "adjacent_ideas": "[IdeaSummary]",
  "connections": "[ConnectionSummary]",
  "depth_reached": "decimal string"
}
```

### SnapshotMetadata
All numeric fields in this object are decimal strings.
```json
{
  "snapshot_id": "hex string (lowercase; equals hex(snapshot_hash))",
  "height": "decimal string",
  "snapshot_hash": "hex string (derived, non-canonical)",
  "state_root_hash": "hex string",
  "title_sentence_payload_root": "hex string",
  "shared_map_commitment": "hex string",
  "prev_snapshot_hash": "hex string | null (derived, non-canonical)",
  "event_count": "decimal string",
  "approximate_timestamp": "decimal string (unix seconds, non-authoritative)",
  "cycle_index": "decimal string | null (metadata)",
  "cycle_close_height": "decimal string | null (metadata, block height of cycle_close)"
}
```

`title_sentence_payload_root` equals `pocket_map_payload_root` at the same height.
Cycle fields are advisory metadata only; snapshots remain keyed to block height.

All strings are UTF-8; empty fields use null where applicable.

### SnapshotCommitMetadata
All numeric fields in this object are decimal strings.
```json
{
  "block_height": "decimal string",
  "snapshot_id": "hex string",
  "snapshot_hash": "hex string",
  "state_root_hash": "hex string",
  "title_sentence_payload_root": "hex string",
  "shared_map_commitment": "hex string",
  "last_event_id": "UUIDv7 string",
  "event_count": "decimal string",
  "active_rulebook_set_hash": "hex string",
  "created_event_id": "UUIDv7 string"
}
```

## 4. Endpoints

Current implemented router namespaces:

- `/api/v0`
- `/api/v1/canonical`

Sections 4.0-4.10 describe endpoints currently implemented in the open-core runtime. Section 4.11 is spec/future only and is not implemented in the current open-core runtime.

### 4.0 GET /health

Health check for automation and local verification.

Success Response (200 OK):
```json
{ "ok": true }
```

Notes:
* `/health` is not a canonical data endpoint and does not include snapshot headers.

### 4.1 GET /snapshot/latest

Returns metadata for the most recent available snapshot.

Query Parameters:
Returns paginated top ideas by derived universal rank (default ascending; rank `1` first).

Success Response (200 OK):
```json
{
  "snapshot": SnapshotMetadata,
  "preview_ideas": "[IdeaSummary] (optional)"
}
```

### 4.2 GET /snapshot/{height}

Path Parameter:
* `height`: decimal string (block height)

Returns metadata for the specified historical snapshot.

Success Response (200 OK):
```json
{ "snapshot": SnapshotMetadata }
```

Errors:
* 404 Not Found if height unavailable.

### 4.3 GET /ideas/top

Returns paginated top ideas by derived universal rank (default ascending; rank `1` first).

Query Parameters:
* `limit`: decimal string (default 50, max 200)
* `offset`: decimal string (default 0)
* `order`: `asc|desc` (optional; default `asc` for rank 1 first)

Success Response (200 OK):
```json
{
  "ideas": "[IdeaSummary]",
  "total": "decimal string (approximate)",
  "offset": "decimal string",
  "limit": "decimal string"
}
```

### 4.4 GET /idea/{id}

Path Parameter:
* `id`: UUIDv7 string (idea_id)

Returns detailed view of a single idea with direct connections.

Success Response (200 OK):
```json
{ "idea": IdeaDetail }
```

Errors:
* 404 Not Found if idea absent in basis snapshot.

### 4.5 GET /idea/{id}/neighborhood

Path Parameter:
* `id`: UUIDv7 string (idea_id)

Query Parameters:
* `depth`: decimal string (default 1, max 2)
* `limit_per_hop`: decimal string (default 50)

Returns bounded neighborhood subgraph (depth-limited traversal).

Success Response (200 OK):
```json
NeighborhoodResponse
```

Ordering:
* `connections` are ordered deterministically by `(created_block_height, created_event_index, connection_id)`.
* `adjacent_ideas` are ordered deterministically using the first connection that introduces the idea (same ordering keys as above).

Errors:
* 404 Not Found if central idea absent.

### 4.6 GET /search/ideas

Simple keyword search over embedded Tier 0 text.

Query Parameters:
* `q`: string (required)
* `limit`: decimal string (default 50)
* `offset`: decimal string (default 0)

Success Response (200 OK):
```json
{
  "results": "[IdeaSummary]",
  "total": "decimal string (approximate)"
}
```

### 4.10 Stage-0 Extension Endpoints (implemented)

The following read endpoints are implemented in the current open-core runtime as extension endpoints. They are canonical read views derived from replayed canonical state, but are outside the minimal core set in sections 4.1-4.6.

Stability expectations for Stage 0:
* Endpoint paths are stable within their current versioned namespace.
* Response JSON fields listed below are stable for Stage 0 clients unless otherwise noted.
* Semantics remain read-only and deterministic for a fixed snapshot basis.

#### 4.10.1 GET /rail/{rail_id}

Path Parameter:
* `rail_id`: UUIDv7 string

Success Response (200 OK):
```json
{
  "rail": {
    "rail_id": "UUIDv7 string",
    "rail_kind": "string (vine | tree | sequence | grouping | pathway_vine | contrast_vine | argument_vine)",
    "vine_type": "string | null",
    "author_identity_id": "UUIDv7 string",
    "canonical_representations": {
      "title_representation_id": "UUIDv7 string | null",
      "title_payload_hash": "hex string | null",
      "sentence_representation_id": "UUIDv7 string | null",
      "sentence_payload_hash": "hex string | null"
    },
    "items": [
      {
        "idx": "decimal string",
        "idea_id": "UUIDv7 string",
        "via_connection_id": "UUIDv7 string | null"
      }
    ]
  }
}
```

Errors:
* 404 Not Found if the rail does not exist in the latest canonical snapshot.

#### 4.10.2 GET /idea/{idea_id}/rails

Path Parameter:
* `idea_id`: UUIDv7 string

Success Response (200 OK):
```json
{
  "rails": [
    {
      "rail_id": "UUIDv7 string",
      "rail_kind": "string",
      "vine_type": "string | null"
    }
  ]
}
```

Notes:
* Empty list is valid when the idea has no canonical rails.

#### 4.10.3 GET /connections/relative-importance

Query Parameters:
* `idea_ids`: comma-separated UUIDv7 list (required, max 200 distinct IDs after de-duplication)

Success Response (200 OK):
```json
{
  "connections": "[ConnectionSummary]"
}
```

Ordering:
* Results are deterministic and sorted by canonical log position `(created_block_height, created_event_index)`.
* Only `relative_importance` edges where both endpoints are in `idea_ids` are returned.

Errors:
* 400 Bad Request if `idea_ids` is missing, invalid, or exceeds 200 IDs.

#### 4.10.4 GET /identity/{identity_id}

Path Parameter:
* `identity_id`: UUIDv7 string

Success Response (200 OK):
```json
{
  "identity": {
    "identity_id": "UUIDv7 string",
    "title": "string"
  }
}
```

Errors:
* 404 Not Found if identity is absent.

#### 4.10.5 GET /snapshots/commits

Returns recent snapshot commit metadata records.

Query Parameters:
* `limit`: decimal string (default 50, max 200)

Success Response (200 OK):
```json
{
  "commits": "[SnapshotCommitMetadata]"
}
```

#### 4.10.6 GET /snapshots/commits/{height}

Path Parameter:
* `height`: decimal string (block height)

Success Response (200 OK):
```json
{
  "commit": SnapshotCommitMetadata
}
```

Errors:
* 404 Not Found if height unavailable.

#### 4.10.7 GET /coordinates

Returns the current Stage 0 coordinate projection of the canonical idea map.

Success Response (200 OK):
```json
{
  "mode": "string",
  "reference_id": "UUIDv7 string | null",
  "coords": [
    {
      "id": "UUIDv7 string",
      "x": "number",
      "y": "number",
      "title": "string",
      "sentence": "string | null",
      "idea_type": "string",
      "derived_universal_rank": "decimal string | null",
      "ri_in_count": "decimal string",
      "ri_out_count": "decimal string"
    }
  ],
  "meta": {
    "spacing": "number",
    "algo": "string",
    "relaxed": "boolean"
  }
}
```

#### 4.10.8 GET /api/v1/canonical/cycles/current

Returns current derived cycle status.

Success Response (200 OK):
```json
{
  "cycle": {
    "cycle_index": "decimal string",
    "h_start": "decimal string",
    "current_height": "decimal string",
    "w_target": "decimal string",
    "observed_work": "decimal string",
    "cycle_age_ge_dmin": "boolean",
    "cycle_age_ge_dmax": "boolean",
    "closure_predicate_satisfied": "boolean",
    "last_cycle_close_height": "decimal string | null"
  }
}
```

#### 4.10.9 GET /api/v1/canonical/event-log

Returns the current canonical event log view and its derived block/cycle bands.

Success Response (200 OK):
```json
{
  "events": [
    {
      "event_id": "UUIDv7 string",
      "global_index": "decimal string",
      "block_height": "decimal string",
      "block_event_index": "decimal string",
      "event_type": "string"
    }
  ],
  "blocks": [
    {
      "id": "string",
      "block_height": "decimal string",
      "start_global_index": "decimal string",
      "end_global_index": "decimal string",
      "label": "string"
    }
  ],
  "cycles": [
    {
      "id": "string",
      "cycle_index": "decimal string",
      "start_global_index": "decimal string",
      "end_global_index": "decimal string",
      "label": "string",
      "closure_event_id": "UUIDv7 string | null"
    }
  ]
}
```

#### 4.10.10 GET /api/v1/canonical/tempo/status

Returns current derived tempo status.

Success Response (200 OK):
```json
{
  "tempo": {
    "cycle_age_ge_dmin": "boolean",
    "cycle_age_ge_dmax": "boolean",
    "constrained_mode": "boolean",
    "record_only_mode": "boolean"
  }
}
```

#### 4.10.11 GET /api/v1/canonical/verification/{identity_id}

Path Parameter:
* `identity_id`: UUIDv7 string

Success Response (200 OK):
```json
{
  "verification": {
    "identity_id": "UUIDv7 string",
    "email_verified": "boolean",
    "canonical_writer_level": "string",
    "active_verifier": "boolean",
    "last_updated_event_id": "UUIDv7 string | null",
    "last_updated_block_height": "decimal string | null",
    "last_updated_event_index": "decimal string | null"
  }
}
```

Errors:
* 404 Not Found if the identity has no verification state.

#### 4.10.12 GET /api/v1/canonical/challenges/{challenge_id}

Path Parameter:
* `challenge_id`: UUIDv7 string

Success Response (200 OK):
```json
{
  "challenge": {
    "challenge_id": "UUIDv7 string",
    "challenge_domain": "string",
    "context_key": "string",
    "axis": "string",
    "timeframe": "string",
    "scope": "string",
    "target_left_idea_id": "UUIDv7 string",
    "target_right_idea_id": "UUIDv7 string",
    "reference_idea_id": "UUIDv7 string | null",
    "framing_representation_ref": "string",
    "created_by_identity_id": "UUIDv7 string",
    "created_event_id": "UUIDv7 string",
    "created_cycle_index": "decimal string",
    "current_cycle_index": "decimal string",
    "phase": "string",
    "arguments": "[CanonicalChallengeArgumentSummary]",
    "votes": "[CanonicalChallengeVoteSummary]",
    "verdict": "CanonicalChallengeVerdictSummary | null"
  }
}
```

Errors:
* 404 Not Found if the challenge is absent.

### 4.11 Not part of current open-core runtime (spec / future)

The following routes are part of the broader public API/spec surface but are not currently exposed by `backend/bins/api-server/src/server/router.rs` in the open-core runtime. They are informative/spec-only here and are not normative for current open-core conformance.

#### 4.11.1 GET /overlays/relative-importance

Returns scoped `relative_importance` overlay state for a declared overlay scope.

Query Parameters:
* `scope_kind`: `universal|tribe|personal` (required)
* `anchor_id`: UUIDv7 string or rulebook-defined universal anchor constant (required)
* `from_idea_id`: UUIDv7 string (optional)
* `to_idea_id`: UUIDv7 string (optional)
* `axis`: string (optional)
* `timeframe`: string (optional)
* `usage`: string (optional)
* `limit`: decimal string (default 50, max 200)
* `offset`: decimal string (default 0)

Illustrative Response Shape:
```json
{
  "scope_key": { "scope_kind": "string", "anchor_id": "string" },
  "items": "[OverlayConnectionSummary]",
  "total": "decimal string (approximate)",
  "offset": "decimal string",
  "limit": "decimal string"
}
```

#### 4.11.2 GET /overlays/display-overrides

Returns scoped display override state for a declared overlay scope.

Query Parameters:
* `scope_kind`: `universal|tribe|personal` (required)
* `anchor_id`: UUIDv7 string or rulebook-defined universal anchor constant (required)
* `target_kind`: `idea|rail` (optional)
* `target_id`: UUIDv7 string (optional)
* `display_slot_key`: string (optional)
* `limit`: decimal string (default 50, max 200)
* `offset`: decimal string (default 0)

Illustrative Response Shape:
```json
{
  "scope_key": { "scope_kind": "string", "anchor_id": "string" },
  "items": "[ScopedDisplayOverrideSummary]",
  "total": "decimal string (approximate)",
  "offset": "decimal string",
  "limit": "decimal string"
}
```

#### 4.11.3 Future: rank history queries

Nodes MAY expose rank history queries that return a series keyed by snapshot height (block height). If exposed in a future version, rank history responses MUST be deterministic for a declared basis and MUST include the snapshot height for each entry alongside derived rank values. This contract does not require specific endpoints or payload shapes in the current open-core runtime.

Future overlay endpoint backward compatibility:
* Implementations MAY support `GET /api/v0/overlays/importance` as a compatibility alias of `GET /api/v0/overlays/relative-importance` if the overlay endpoint family is introduced later.
* If alias support is provided, response shape MUST be identical to the canonical endpoint.

### 4.12 Non-Canonical Sandbox APIs (out of scope for this contract)

Implementations MAY include authenticated, non-canonical sandbox APIs for accounts, private drafts, and AI map assistance. These APIs are outside the canonical event log: they MUST NOT create canonical events, MUST NOT modify canonical-state derivations, and MUST NOT affect public ranks or snapshots. They are private to the owning account and are not part of the deterministic public API surface defined above.

Examples of out-of-scope sandbox route families:
* `/api/v0/auth/*`
* `/api/v0/private/ideas/*`
* `/api/v0/private/ai/*`

AI helper behavior is advisory-only in the canonical universe: it may parse text, autocomplete fields, propose connections, or draft descriptions, but it MUST NOT write canonical events directly.

### 4.13 Canonical Write Surfaces (out of scope for this read-only contract)

Canonical write endpoints are intentionally out of scope for this document. Implementations MAY expose versioned canonical write APIs (for example under `/api/v1/canonical/*`) with authentication and canonical-writer eligibility checks.
Normative clarification [anchor: canonical_write_activation_semantics_note]: when versioned canonical write surfaces are introduced, governance/rulebook changes are decided at cycle close and activate at deterministic cycle boundaries (`activation_cycle_index`) per Protocol v5 and Governance Specification.

Normative policy remains:
* canonical reads are public and unauthenticated for canonical substrate state;
* canonical writes require canonical-writer verification;
* canonical claims remain publicly challengeable.

See `protocol v5.md` (`canonical_read_write_access_policy`).

## 5. Versioning and Compatibility

The current public read surface uses two versioned namespaces:

- `/api/v0` for the main Stage 0 read API
- `/api/v1/canonical` for additional canonical status/detail reads already exposed by the current runtime

This contract remains fixed until a canonical-breaking change requires a new versioned surface.

Future extensions MUST:
* Preserve all existing endpoints and schemas.
* Add new optional fields or endpoints without removing old ones.
* Introduce new base paths only for incompatible changes.
* Preserve legacy behavior of existing endpoints (`/snapshot/*`, `/ideas/top`, `/idea/{id}`, `/idea/{id}/neighborhood`, `/search/ideas`).
* Preserve current Stage-0 extension behavior for the implemented endpoints in section 4.10 unless an explicit versioned change is introduced.

## 6. Conformance Requirements

Conformant servers MUST:
* Implement the endpoints marked as implemented in sections 4.0-4.10 with identical schemas and determinism.
* Serve responses only from verified snapshots (replay-validated `shared_map_commitment`).

Sections explicitly labeled as spec/future are not required for current open-core runtime conformance.

Conformant clients SHOULD:
* Verify `X-Shared-Map-Commitment` against trusted sources.
* Cache aggressively using ETag.

This contract ensures verifiable, efficient access to the shared canonical universe while preserving the protocol's invariants of durability, challengeability, and human-scale collective reasoning.

