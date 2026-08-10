#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), "..");
const fixture = JSON.parse(
  fs.readFileSync(path.join(repoRoot, "docs/conformance/seed-conformance-bindings.vectors.json"), "utf8"),
);
const schema = JSON.parse(
  fs.readFileSync(path.join(repoRoot, "docs/conformance/seed-conformance-bindings.schema.json"), "utf8"),
);
const uuidV7 = /^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;
const lengths = ["sentence", "paragraph", "full"];
const complexities = ["fundamental", "standard", "advanced", "canonical"];

function fail(message) {
  throw new Error(message);
}

function stableJson(value) {
  if (Array.isArray(value)) return `[${value.map(stableJson).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.keys(value)
      .sort()
      .map((key) => `${JSON.stringify(key)}:${stableJson(value[key])}`)
      .join(",")}}`;
  }
  return JSON.stringify(value);
}

function before(position, event) {
  return (
    position.block_height < event.block_height ||
    (position.block_height === event.block_height && position.event_index < event.event_index)
  );
}

function validateEvent(event) {
  if (!uuidV7.test(event.id) || !uuidV7.test(event.speaker_identity_id)) return "invalid_id";
  if (event.kind !== "representation_create") return "unsupported_event_type";
  const payload = event.payload;
  for (const field of ["representation_id", "target_object_id", "author_identity_id"]) {
    if (!uuidV7.test(payload[field] ?? "")) return payload[field] === undefined ? "missing_field" : "invalid_id";
  }
  if (!["idea", "ordering"].includes(payload.target_kind)) {
    return payload.target_kind === undefined ? "missing_field" : "invalid_field";
  }
  if (!/^[0-9a-f]{64}$/.test(payload.payload_hash ?? "")) {
    return payload.payload_hash === undefined ? "missing_field" : "invalid_field";
  }
  if (payload.author_identity_id !== event.speaker_identity_id) return "invalid_field";
  if (!["title", "description"].includes(payload.representation_kind)) {
    return payload.representation_kind === undefined ? "missing_field" : "invalid_field";
  }
  if (payload.representation_kind === "title") {
    if (
      payload.tier_length !== undefined ||
      payload.tier_complexity !== undefined ||
      payload.vocabulary_version_id !== undefined
    ) {
      return "invalid_field";
    }
  } else {
    if (!lengths.includes(payload.tier_length)) {
      return payload.tier_length === undefined ? "missing_field" : "invalid_field";
    }
    if (!complexities.includes(payload.tier_complexity)) {
      return payload.tier_complexity === undefined ? "missing_field" : "invalid_field";
    }
    if (payload.tier_complexity === "canonical") {
      if (!uuidV7.test(payload.vocabulary_version_id ?? "")) {
        return payload.vocabulary_version_id === undefined ? "missing_field" : "invalid_id";
      }
    } else if (payload.vocabulary_version_id !== undefined) {
      return "invalid_field";
    }
  }

  const authorPosition = fixture.identity_positions[payload.author_identity_id];
  if (!authorPosition) return "unknown_author";
  if (!before(authorPosition, event)) return "author_not_preexisting";
  if (payload.vocabulary_version_id !== undefined) {
    const vocabularyPosition = fixture.idea_positions[payload.vocabulary_version_id];
    if (!vocabularyPosition) return "unknown_vocabulary";
    if (!before(vocabularyPosition, event)) return "vocabulary_not_preexisting";
  }
  return null;
}

if (fixture.schema !== schema.properties.schema.const) fail("schema identifier mismatch");
if (fixture.vectors.length < schema.properties.vectors.minItems) fail("insufficient vectors");
if (new Set(fixture.vectors.map((vector) => vector.id)).size !== fixture.vectors.length) {
  fail("duplicate vector id");
}

let passed = 0;
const validCells = new Set();
for (const vector of fixture.vectors) {
  let code = null;
  for (const event of vector.events) {
    code = validateEvent(event);
    if (code) break;
    if (event.payload.representation_kind === "description") {
      validCells.add(`${event.payload.tier_length}:${event.payload.tier_complexity}`);
    }
  }
  const accepted = code === null;
  if (accepted !== vector.expected.accept || code !== vector.expected.code) {
    fail(`${vector.id}: expected ${stableJson(vector.expected)}, got ${stableJson({ accept: accepted, code })}`);
  }
  passed += 1;
}
const expectedCells = new Set(lengths.flatMap((length) => complexities.map((complexity) => `${length}:${complexity}`)));
if (stableJson([...validCells].sort()) !== stableJson([...expectedCells].sort())) {
  fail("accepted vectors do not cover exactly all twelve description cells");
}

const canonicalPayloads = new Set();
for (const vector of fixture.hash_vectors) {
  const canonical = stableJson(vector.payload);
  if (canonical !== vector.canonical_json_utf8) fail(`${vector.id}: canonical JSON bytes mismatch`);
  if (canonicalPayloads.has(canonical)) fail(`${vector.id}: duplicate hash payload`);
  canonicalPayloads.add(canonical);
  if (!/^[0-9a-f]{64}$/.test(vector.blake3)) fail(`${vector.id}: invalid BLAKE3 field`);
}

console.log(
  `seed conformance bindings: ${passed}/${fixture.vectors.length} vectors passed; ${validCells.size}/12 description cells covered`,
);
