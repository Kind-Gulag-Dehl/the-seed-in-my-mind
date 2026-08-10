#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), "..");
const fixturePath = path.join(repoRoot, "docs/conformance/native-ordering.vectors.json");
const schemaPath = path.join(repoRoot, "docs/conformance/native-ordering.schema.json");

const fixture = JSON.parse(fs.readFileSync(fixturePath, "utf8"));
const schema = JSON.parse(fs.readFileSync(schemaPath, "utf8"));
const expectedCodes = { vine: 0, evidence_rail: 1, action_rail: 2 };
const uuidV7 = /^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;

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

function profile(value) {
  return typeof value === "string" && Object.hasOwn(expectedCodes, value)
    ? value
    : null;
}

function validateEvent(event, state) {
  if (!uuidV7.test(event.id) || !uuidV7.test(event.speaker_identity_id)) return "invalid_id";
  if (fixture.forbidden_legacy_event_types.includes(event.kind)) return "unsupported_event_type";
  if (!["ordering_create", "ordering_fork"].includes(event.kind)) return "unsupported_event_type";

  const payload = event.payload;
  if (!uuidV7.test(payload.ordering_id ?? "")) return payload.ordering_id ? "invalid_id" : "missing_field";
  const declaredProfile = profile(payload.ordering_profile);
  if (!declaredProfile) return payload.ordering_profile === undefined ? "missing_field" : "invalid_field";
  if (!Array.isArray(payload.item_idea_ids) || payload.item_idea_ids.some((id) => !uuidV7.test(id))) {
    return "invalid_field";
  }
  if (declaredProfile === "vine") {
    if (payload.subject_idea_id !== undefined || payload.item_roles !== undefined) return "invalid_field";
    if (event.kind === "ordering_create" && !["pathway_vine", "narrative_vine"].includes(payload.vine_type)) {
      return payload.vine_type === undefined ? "missing_field" : "invalid_field";
    }
    if (
      event.kind === "ordering_fork" &&
      payload.vine_type !== undefined &&
      !["pathway_vine", "narrative_vine"].includes(payload.vine_type)
    ) {
      return "invalid_field";
    }
  } else if (payload.vine_type !== undefined && payload.vine_type !== null) {
    return "invalid_field";
  } else {
    if (payload.item_idea_ids.length === 0) return "invalid_field";
    if (!uuidV7.test(payload.subject_idea_id ?? "")) {
      return payload.subject_idea_id === undefined ? "missing_field" : "invalid_id";
    }
    const expectedType = declaredProfile === "evidence_rail" ? "truth_claim" : "actionable_idea";
    if (fixture.idea_types[payload.subject_idea_id] !== expectedType) return "subject_type_mismatch";
    if (!Array.isArray(payload.item_roles)) {
      return payload.item_roles === undefined ? "missing_field" : "invalid_field";
    }
    if (payload.item_roles.length !== payload.item_idea_ids.length) return "invalid_field";
    if (new Set(payload.item_idea_ids).size !== payload.item_idea_ids.length) return "invalid_field";
    if (
      declaredProfile === "evidence_rail" &&
      payload.item_roles.some((role) => !["potential_evidence", "actual_evidence"].includes(role))
    ) {
      return "invalid_field";
    }
    if (declaredProfile === "action_rail") {
      const lane = payload.item_roles[0];
      if (
        !["potential_action", "proposed_action"].includes(lane) ||
        payload.item_roles.some((role) => role !== lane)
      ) {
        return "invalid_field";
      }
    }
  }

  if (event.kind === "ordering_fork") {
    if (!uuidV7.test(payload.base_ordering_id ?? "")) {
      return payload.base_ordering_id ? "invalid_id" : "missing_field";
    }
    const base = state.get(payload.base_ordering_id);
    if (!base) return "base_ordering_not_found";
    if (base.profile !== declaredProfile) return "ordering_profile_mismatch";
    if (base.subjectIdeaId !== (payload.subject_idea_id ?? null)) return "ordering_subject_mismatch";
    const forkRoles = new Map(
      payload.item_idea_ids.map((ideaId, index) => [ideaId, payload.item_roles?.[index] ?? null]),
    );
    for (const [ideaId, role] of base.itemRoles) {
      if (forkRoles.has(ideaId) && forkRoles.get(ideaId) !== role) {
        return "ordering_item_role_mismatch";
      }
    }
    if (declaredProfile === "action_rail" && base.actionLane !== payload.item_roles[0]) {
      return "action_lane_mismatch";
    }
  }
  if (state.has(payload.ordering_id)) return "duplicate_ordering";
  state.set(payload.ordering_id, {
    profile: declaredProfile,
    subjectIdeaId: payload.subject_idea_id ?? null,
    itemRoles: new Map(
      payload.item_idea_ids.map((ideaId, index) => [ideaId, payload.item_roles?.[index] ?? null]),
    ),
    actionLane: declaredProfile === "action_rail" ? payload.item_roles[0] : null,
  });
  return null;
}

if (fixture.schema !== schema.properties.schema.const) fail("schema identifier mismatch");
if (stableJson(fixture.profile_codes) !== stableJson(expectedCodes)) fail("profile code mismatch");
for (const [ideaId, ideaType] of Object.entries(fixture.idea_types)) {
  if (!uuidV7.test(ideaId)) fail(`invalid fixture idea ID ${ideaId}`);
  if (!["conceptual_idea", "truth_claim", "actionable_idea", "identity", "action"].includes(ideaType)) {
    fail(`invalid fixture idea type ${ideaType}`);
  }
}
if (fixture.vectors.length < schema.properties.vectors.minItems) fail("insufficient vectors");
if (new Set(fixture.vectors.map((vector) => vector.id)).size !== fixture.vectors.length) {
  fail("duplicate vector id");
}
for (const legacy of schema.properties.forbidden_legacy_event_types.items.enum) {
  if (!fixture.forbidden_legacy_event_types.includes(legacy)) fail(`missing forbidden event ${legacy}`);
}

let passed = 0;
for (const vector of fixture.vectors) {
  const state = new Map();
  let code = null;
  for (const event of vector.events) {
    code = validateEvent(event, state);
    if (code) break;
  }
  const accepted = code === null;
  if (accepted !== vector.expected.accept || code !== vector.expected.code) {
    fail(`${vector.id}: expected ${stableJson(vector.expected)}, got ${stableJson({ accept: accepted, code })}`);
  }
  passed += 1;
}

for (const vector of fixture.hash_vectors) {
  if (stableJson(vector.payload) !== vector.canonical_json_utf8) {
    fail(`${vector.id}: canonical JSON bytes mismatch`);
  }
}

console.log(`native-ordering conformance: ${passed}/${fixture.vectors.length} vectors passed`);
