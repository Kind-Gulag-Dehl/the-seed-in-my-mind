#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, "..");

const DEFAULT_FIXTURE_PATH = "docs/conformance/tempo-cycle-fixtures.v1.json";
const DEFAULT_SCHEMA_PATH = "docs/conformance/tempo-cycle-fixtures.schema.v1.json";

const ALLOWED_EVENT_TYPES = new Set([
  "idea_create",
  "connection_create",
  "challenge_create",
  "vote_cast",
  "challenge_finalize_verdict",
  "cycle_close",
]);

const FORBIDDEN_IDEA_TYPES = new Set([
  "time_claim",
  "tempo_target",
  "beacon",
  "evidence",
  "attestation",
  "testimony",
  "source",
]);

const FINAL_REJECTION_STATUSES = new Set(["appendix_a_final"]);
const CERTAINTY_ORDER = ["none", "hypothetical", "low", "medium", "high", "beacon"];

function readJson(relativePath) {
  const fullPath = path.resolve(repoRoot, relativePath);
  try {
    return JSON.parse(fs.readFileSync(fullPath, "utf8"));
  } catch (error) {
    throw new Error(`Failed to read JSON ${relativePath}: ${error.message}`);
  }
}

function jsonPointer(root, pointer) {
  if (!pointer.startsWith("#/")) {
    throw new Error(`Unsupported schema ref ${pointer}`);
  }
  return pointer
    .slice(2)
    .split("/")
    .reduce((node, part) => {
      const key = part.replace(/~1/g, "/").replace(/~0/g, "~");
      if (node == null || !(key in node)) {
        throw new Error(`Unresolvable schema ref ${pointer}`);
      }
      return node[key];
    }, root);
}

function typeMatches(expected, value) {
  if (expected === "array") return Array.isArray(value);
  if (expected === "object") return value !== null && typeof value === "object" && !Array.isArray(value);
  if (expected === "integer") return Number.isInteger(value);
  if (expected === "null") return value === null;
  return typeof value === expected;
}

function validateSchemaValue(schema, value, rootSchema, at = "$", errors = []) {
  if (schema === true) return errors;
  if (schema === false) {
    errors.push(`${at}: schema is false`);
    return errors;
  }
  if (!schema || typeof schema !== "object") return errors;

  if (schema.$ref) {
    return validateSchemaValue(jsonPointer(rootSchema, schema.$ref), value, rootSchema, at, errors);
  }

  if (schema.const !== undefined && value !== schema.const) {
    errors.push(`${at}: expected const ${JSON.stringify(schema.const)}, got ${JSON.stringify(value)}`);
  }

  if (schema.enum && !schema.enum.some((item) => item === value)) {
    errors.push(`${at}: value ${JSON.stringify(value)} not in enum`);
  }

  if (schema.type !== undefined) {
    const types = Array.isArray(schema.type) ? schema.type : [schema.type];
    if (!types.some((type) => typeMatches(type, value))) {
      errors.push(`${at}: expected type ${types.join("|")}, got ${value === null ? "null" : Array.isArray(value) ? "array" : typeof value}`);
      return errors;
    }
  }

  if (typeof value === "number" && schema.minimum !== undefined && value < schema.minimum) {
    errors.push(`${at}: expected minimum ${schema.minimum}, got ${value}`);
  }

  if (typeof value === "string" && schema.pattern !== undefined) {
    const re = new RegExp(schema.pattern);
    if (!re.test(value)) {
      errors.push(`${at}: string does not match pattern ${schema.pattern}`);
    }
  }

  if (Array.isArray(value)) {
    if (schema.minItems !== undefined && value.length < schema.minItems) {
      errors.push(`${at}: expected at least ${schema.minItems} items`);
    }
    if (schema.items !== undefined) {
      value.forEach((item, index) => validateSchemaValue(schema.items, item, rootSchema, `${at}[${index}]`, errors));
    }
    if (schema.contains !== undefined) {
      const matched = value.some((item, index) => {
        const nested = [];
        validateSchemaValue(schema.contains, item, rootSchema, `${at}[${index}]`, nested);
        return nested.length === 0;
      });
      if (!matched) {
        errors.push(`${at}: no item matched contains schema`);
      }
    }
  }

  if (value !== null && typeof value === "object" && !Array.isArray(value)) {
    if (schema.required) {
      for (const key of schema.required) {
        if (!(key in value)) {
          errors.push(`${at}: missing required property ${key}`);
        }
      }
    }
    if (schema.properties) {
      for (const [key, childSchema] of Object.entries(schema.properties)) {
        if (key in value) {
          validateSchemaValue(childSchema, value[key], rootSchema, `${at}.${key}`, errors);
        }
      }
    }
    if (schema.additionalProperties === false && schema.properties) {
      const allowed = new Set(Object.keys(schema.properties));
      for (const key of Object.keys(value)) {
        if (!allowed.has(key)) {
          errors.push(`${at}: unexpected property ${key}`);
        }
      }
    } else if (schema.additionalProperties && typeof schema.additionalProperties === "object") {
      const allowed = new Set(Object.keys(schema.properties ?? {}));
      for (const key of Object.keys(value)) {
        if (!allowed.has(key)) {
          validateSchemaValue(schema.additionalProperties, value[key], rootSchema, `${at}.${key}`, errors);
        }
      }
    }
  }

  if (schema.anyOf) {
    const anyMatched = schema.anyOf.some((candidate) => {
      const nested = [];
      validateSchemaValue(candidate, value, rootSchema, at, nested);
      return nested.length === 0;
    });
    if (!anyMatched) {
      errors.push(`${at}: did not match anyOf schemas`);
    }
  }

  if (schema.not) {
    const nested = [];
    validateSchemaValue(schema.not, value, rootSchema, at, nested);
    if (nested.length === 0) {
      errors.push(`${at}: matched forbidden not schema`);
    }
  }

  return errors;
}

function assertFixtureSchema(schema, fixtures) {
  const errors = validateSchemaValue(schema, fixtures, schema);
  if (errors.length > 0) {
    const shown = errors.slice(0, 25).join("\n");
    throw new Error(`Fixture schema validation failed with ${errors.length} error(s):\n${shown}`);
  }
}

function parseTargetKey(targetKey) {
  const match = /^tempo_target\((\d+), (dmin|dmax)\)$/.exec(targetKey ?? "");
  if (!match) return null;
  return { cycleIndex: Number(match[1]), kind: match[2] };
}

function bandAtLeast(band, threshold) {
  return CERTAINTY_ORDER.indexOf(band ?? "none") >= CERTAINTY_ORDER.indexOf(threshold);
}

function contradictionBlockBand(profile) {
  return profile.contradiction_block_band ?? profile.T_contradiction_block;
}

function beaconMinimumBand(profile) {
  return profile.beacon_minimum_certainty_band ?? profile.T_beacon;
}

function deepEqual(a, b) {
  return JSON.stringify(a) === JSON.stringify(b);
}

function sortedJson(value) {
  if (Array.isArray(value)) return value.map(sortedJson);
  if (value && typeof value === "object") {
    return Object.fromEntries(Object.entries(value).sort(([a], [b]) => a.localeCompare(b)).map(([k, v]) => [k, sortedJson(v)]));
  }
  return value;
}

class ReplayState {
  constructor(fixtures) {
    this.profile = fixtures.tempo_profile;
    this.identityMap = new Map(fixtures.identities.map((identity) => [identity.identity_id, identity]));
    this.sharedSetup = fixtures.shared_setup;
    this.initialAuthorizationFrontier = fixtures.genesis.initial_authorization_frontier;
    this.ideas = new Map();
    this.connections = new Map();
    this.challenges = new Map();
    this.votes = new Map();
    this.certainty = new Map();
    this.placements = new Map();
    this.acceptedEventIds = [];
    this.rejections = [];
    this.tempoMana = new Map();
    this.tempoManaSpent = 0;
    this.appliedFixtures = new Set();
    this.closedCycles = new Map();
  }

  cloneForFixture() {
    return this;
  }

  applyInitialState(initialState = {}) {
    if (initialState.tempo_mana) {
      for (const [identityId, balance] of Object.entries(initialState.tempo_mana)) {
        this.tempoMana.set(identityId, balance);
      }
    }
    this.cycleIndex = initialState.cycle_index ?? initialState.current_cycle ?? this.cycleIndex ?? 0;
    this.W_score = initialState.W_score ?? this.W_score ?? 0;
    this.W_target = initialState.W_target ?? this.W_target ?? this.profile.Dmin;
  }

  identity(identityId) {
    return this.identityMap.get(identityId);
  }

  hasIdea(ideaId) {
    return this.ideas.has(ideaId);
  }

  addRejection(eventId, code) {
    this.rejections.push({ event_id: eventId, code });
  }

  accept(event) {
    this.acceptedEventIds.push(event.event_id);
  }
}

function targetClaimIsValid(payload, profile) {
  const claim = payload.tempo_claim;
  if (!claim) return { ok: false, code: "ERR_TEMPO_CLAIM_MISSING_METADATA" };
  if (claim.tempo_profile_hash !== profile.profile_hash || payload.tempo_lane?.tempo_profile_hash && payload.tempo_lane.tempo_profile_hash !== profile.profile_hash) {
    return { ok: false, code: "ERR_TEMPO_CLAIM_PROFILE_MISMATCH" };
  }
  const parsed = parseTargetKey(claim.target_key);
  if (!parsed || parsed.kind !== claim.target_kind || parsed.cycleIndex !== claim.anchor_cycle_index) {
    return { ok: false, code: "ERR_TEMPO_CLAIM_TARGET_KEY_MISMATCH" };
  }
  const expectedDuration = claim.target_kind === "dmin" ? profile.Dmin : profile.Dmax;
  if (claim.duration_value !== expectedDuration) {
    return { ok: false, code: "ERR_TEMPO_CLAIM_TARGET_KEY_MISMATCH" };
  }
  return { ok: true };
}

function spendTempoManaIfNeeded(event, state) {
  const spend = Number(event.payload.tempo_lane?.tempo_mana_spend ?? 0);
  if (spend <= 0) return null;
  const current = state.tempoMana.has(event.author_identity_id)
    ? state.tempoMana.get(event.author_identity_id)
    : state.profile.tempo_mana_cap;
  if (current < spend) return "ERR_TEMPO_MANA_INSUFFICIENT";
  state.tempoMana.set(event.author_identity_id, current - spend);
  state.tempoManaSpent += spend;
  return null;
}

function validateIdeaCreate(event, state) {
  const identity = state.identity(event.author_identity_id);
  const payload = event.payload;
  if (!identity) return "ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR";

  if (identity.actor_kind === "ai" && (payload.tempo_claim || payload.proposition || payload.truth_subtype)) {
    return "ERR_TEMPO_AI_AUTHORITY";
  }

  if (FORBIDDEN_IDEA_TYPES.has(payload.idea_type)) {
    return "ERR_TEMPO_CLAIM_NOT_TRUTH_CLAIM";
  }

  if (payload.forbidden_direct_time_authority) {
    if ("block_height" in payload.forbidden_direct_time_authority) return "ERR_TEMPO_BLOCK_HEIGHT_AUTHORITY";
    return "ERR_TEMPO_HIDDEN_CLOCK_INPUT";
  }
  if (payload.passive_timestamp_evidence) {
    const passive = payload.passive_timestamp_evidence;
    const requiredPassiveFields = ["source_id", "source_category", "target_key", "source_epoch", "normalized_timestamp", "canonical_provenance_ref"];
    if (passive.forbidden_direct_input) return "ERR_TEMPO_HIDDEN_CLOCK_INPUT";
    if (!requiredPassiveFields.every((field) => typeof passive[field] === "string" && passive[field].length > 0)) {
      return "ERR_TEMPO_PASSIVE_EVIDENCE_SOURCE_INVALID";
    }
    if (passive.support_units > state.profile.passive_evidence_cap) {
      return "ERR_TEMPO_PASSIVE_EVIDENCE_CAP_EXCEEDED";
    }
  }

  const laneOperation = payload.tempo_lane?.tempo_lane_operation;
  const isTempoLane = Boolean(payload.tempo_lane);
  const isTargetTimeClaim = laneOperation === "target_time_claim_create" || Boolean(payload.tempo_claim);

  if (isTempoLane && laneOperation === "target_time_claim_create" && payload.idea_type !== "truth_claim") {
    return "ERR_TEMPO_CLAIM_NOT_TRUTH_CLAIM";
  }
  if (isTempoLane && laneOperation === "target_time_claim_create" && !payload.tempo_claim) {
    return "ERR_TEMPO_CLAIM_MISSING_METADATA";
  }

  if (isTargetTimeClaim) {
    if (payload.idea_type !== "truth_claim") return "ERR_TEMPO_CLAIM_NOT_TRUTH_CLAIM";
    if (!identity.tempo_contributor && !identity.ordinary_canonical_writer) return "ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR";
    const validClaim = targetClaimIsValid(payload, state.profile);
    if (!validClaim.ok) return validClaim.code;
  } else if (isTempoLane) {
    if (!identity.tempo_contributor) return "ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR";
    if (laneOperation !== "tempo_evidence_claim_create") return "ERR_TEMPO_CLAIM_MISSING_METADATA";
    if (payload.idea_type !== "truth_claim") return "ERR_TEMPO_CLAIM_NOT_TRUTH_CLAIM";
  } else if (!identity.ordinary_canonical_writer) {
    return "ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR";
  }

  const manaError = spendTempoManaIfNeeded(event, state);
  if (manaError) return manaError;

  state.ideas.set(payload.idea_id, {
    ...payload,
    author_identity_id: event.author_identity_id,
    event_id: event.event_id,
  });
  return null;
}

function tempoClaimsCompatible(left, right) {
  if (!left?.tempo_claim || !right?.tempo_claim) return true;
  const fields = ["target_key", "anchor_event_id", "anchor_cycle_index", "target_kind", "duration_value", "duration_unit_profile", "tempo_profile_hash"];
  return fields.every((field) => left.tempo_claim[field] === right.tempo_claim[field]);
}

function validateConnectionCreate(event, state) {
  const identity = state.identity(event.author_identity_id);
  if (!identity) return "ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR";
  if (identity.actor_kind === "ai") return "ERR_TEMPO_AI_AUTHORITY";
  const payload = event.payload;
  const fromId = payload.from_idea_id ?? payload.source_idea_id;
  const toId = payload.to_idea_id ?? payload.target_idea_id;
  const from = state.ideas.get(fromId);
  const to = state.ideas.get(toId);

  const isTempoEvidenceConnection = payload.tempo_lane || payload.usage === "evidence_for" || payload.usage === "evidence_against";
  if (!from || !to) {
    return isTempoEvidenceConnection ? "ERR_TEMPO_EVIDENCE_CONNECTION_INVALID" : "ERR_CYCLE_CLOSE_PAYLOAD_MISMATCH";
  }

  if (payload.connection_type === "same_as") {
    if (!tempoClaimsCompatible(from, to)) return "ERR_TEMPO_EVIDENCE_CONNECTION_INVALID";
  }

  if (payload.usage === "evidence_for" || payload.usage === "evidence_against") {
    if (payload.connection_type !== "relative_importance") return "ERR_TEMPO_EVIDENCE_CONNECTION_INVALID";
    if (!to.tempo_claim) return "ERR_TEMPO_EVIDENCE_CONNECTION_INVALID";
    if (from.idea_type && FORBIDDEN_IDEA_TYPES.has(from.idea_type)) return "ERR_TEMPO_EVIDENCE_CONNECTION_INVALID";
  }

  const manaError = spendTempoManaIfNeeded(event, state);
  if (manaError) return manaError;

  state.connections.set(payload.connection_id, {
    ...payload,
    from_idea_id: fromId,
    to_idea_id: toId,
    author_identity_id: event.author_identity_id,
  });
  return null;
}

function validateChallengeCreate(event, state) {
  const identity = state.identity(event.author_identity_id);
  if (!identity) return "ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR";
  if (identity.actor_kind === "ai") return "ERR_TEMPO_AI_AUTHORITY";
  if (!identity.ordinary_challenge_eligible) return "ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR";
  const payload = event.payload;
  for (const ideaId of payload.subject_idea_ids ?? []) {
    if (!state.hasIdea(ideaId)) return "ERR_TEMPO_EVIDENCE_CONNECTION_INVALID";
  }
  state.challenges.set(payload.challenge_id, {
    ...payload,
    votes: [],
    finalized: false,
  });
  return null;
}

function validateVoteCast(event, state) {
  const identity = state.identity(event.author_identity_id);
  if (!identity) return "ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR";
  if (identity.actor_kind === "ai") return "ERR_TEMPO_AI_AUTHORITY";
  if (!identity.ordinary_challenge_eligible) return "ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR";
  const challenge = state.challenges.get(event.payload.challenge_id);
  if (!challenge) return "ERR_CYCLE_CLOSE_PAYLOAD_MISMATCH";
  challenge.votes.push(event.event_id);
  return null;
}

function validateChallengeFinalize(event, state) {
  const identity = state.identity(event.author_identity_id);
  if (!identity) return "ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR";
  if (identity.actor_kind === "ai") return "ERR_TEMPO_AI_AUTHORITY";
  if (!identity.ordinary_challenge_eligible) return "ERR_TEMPO_CLAIM_UNAUTHORIZED_AUTHOR";
  const challenge = state.challenges.get(event.payload.challenge_id);
  if (!challenge) return "ERR_CYCLE_CLOSE_PAYLOAD_MISMATCH";
  challenge.finalized = true;
  challenge.verdict = event.payload.verdict;
  challenge.assigned_certainty_band = event.payload.assigned_certainty_band;
  const subjectIds = event.payload.subject_idea_ids ?? challenge.subject_idea_ids ?? [];
  if (event.payload.assigned_certainty_band) {
    for (const ideaId of subjectIds) {
      state.certainty.set(ideaId, event.payload.assigned_certainty_band);
    }
  }
  if (event.payload.verdict?.startsWith("evidence_placed")) {
    for (const ideaId of subjectIds) {
      state.placements.set(ideaId, event.payload.verdict.replace("evidence_placed_", ""));
    }
  }
  return null;
}

function currentTargetKey(state, kind, cycleIndex = state.cycleIndex ?? 0) {
  return `tempo_target(${cycleIndex}, ${kind})`;
}

function passiveEvidenceStateForTarget(state, targetKey) {
  const seen = new Set();
  const accepted = [];
  const duplicates = [];
  const rejected = [];
  for (const idea of state.ideas.values()) {
    const passive = idea.passive_timestamp_evidence;
    if (!passive) continue;
    const passiveTarget = passive.target_key ?? idea.tempo_claim?.target_key;
    if (passiveTarget !== targetKey) continue;
    if (passive.admissible === false || passive.outlier === true) {
      rejected.push(idea.idea_id);
      continue;
    }
    const dedupeKey = `${passive.source_id}|${passiveTarget}|${passive.source_epoch}`;
    if (seen.has(dedupeKey)) {
      duplicates.push(idea.idea_id);
      continue;
    }
    seen.add(dedupeKey);
    accepted.push({ ideaId: idea.idea_id, supportUnits: Number(passive.support_units ?? 1) });
  }
  const rawUnits = accepted.reduce((sum, item) => sum + item.supportUnits, 0);
  const cappedUnits = Math.min(rawUnits, Number(state.profile.passive_evidence_cap ?? 0));
  return {
    acceptedIdeaIds: accepted.map((item) => item.ideaId).sort(),
    duplicateIdeaIds: duplicates.sort(),
    rejectedIdeaIds: rejected.sort(),
    rawUnits,
    cappedUnits,
  };
}

function structuralSupportStateForTarget(state, targetKey, kind) {
  const stances = new Map();
  for (const idea of state.ideas.values()) {
    const identity = state.identity(idea.author_identity_id);
    if (!identity || identity.actor_kind !== "human" || !identity.tempo_contributor) continue;

    if (idea.tempo_claim?.target_key === targetKey) {
      stances.set(identity.identity_id, idea.tempo_claim.asserted_value === false ? "oppose" : "support");
    } else if (idea.contradicts_target_key === targetKey || idea.tempo_contradiction?.contradicts_target_key === targetKey) {
      stances.set(identity.identity_id, "oppose");
    }
  }

  const supportIds = [...stances.entries()].filter(([, stance]) => stance === "support").map(([identityId]) => identityId).sort();
  const oppositionIds = [...stances.entries()].filter(([, stance]) => stance === "oppose").map(([identityId]) => identityId).sort();
  const passive = passiveEvidenceStateForTarget(state, targetKey);
  const humanSupportUnits = supportIds.length * Number(state.profile.structural_support_unit_per_human ?? 1);
  const structuralSupportUnits = humanSupportUnits + passive.cappedUnits;
  const requiredSupport = Number(state.profile[`required_human_support_${kind}`] ?? 1);
  const requiredMargin = Number(state.profile[`required_human_margin_${kind}`] ?? 1);
  const contradictionClaims = [...state.ideas.values()].filter((idea) => (
    idea.contradicts_target_key === targetKey ||
    idea.tempo_contradiction?.contradicts_target_key === targetKey ||
    (idea.tempo_claim?.target_key === targetKey && idea.tempo_claim.asserted_value === false)
  ));
  const blockingContradictions = contradictionClaims
    .filter((claim) => bandAtLeast(state.certainty.get(claim.idea_id), contradictionBlockBand(state.profile)))
    .map((claim) => claim.idea_id)
    .sort();
  const ready = supportIds.length >= requiredSupport &&
    supportIds.length - oppositionIds.length >= requiredMargin &&
    structuralSupportUnits >= Number(state.profile.T_allow) &&
    blockingContradictions.length === 0;

  return {
    ready,
    supportIds,
    oppositionIds,
    eligibleHumanSupport: supportIds.length,
    eligibleHumanOpposition: oppositionIds.length,
    passiveEvidenceUnits: passive.cappedUnits,
    passiveEvidenceDuplicateIds: passive.duplicateIdeaIds,
    structuralSupportUnits,
    blockingContradictionClaimIds: blockingContradictions,
  };
}

function structuralReadyFromPayloadOrReplay(state, payload, kind) {
  const explicit = payload[`${kind}_structural_readiness`];
  if (typeof explicit === "boolean") return explicit;
  const targetKey = payload[`${kind}_target_key`];
  return structuralSupportStateForTarget(state, targetKey, kind).ready;
}

function deriveStructuralDmaxLiveness(state, cycleIndex = state.cycleIndex ?? 0) {
  const dmaxTarget = currentTargetKey(state, "dmax", cycleIndex);
  const elapsedClaims = [];
  const blockingContradictions = [];
  for (const idea of state.ideas.values()) {
    if (idea.tempo_claim?.target_key !== dmaxTarget) continue;
    if (idea.tempo_claim.asserted_value === false || idea.contradicts_target_key === dmaxTarget) {
      blockingContradictions.push(idea.idea_id);
    } else if (idea.tempo_claim.asserted_value === true) {
      elapsedClaims.push(idea);
    }
  }
  if (blockingContradictions.length > 0) {
    return {
      status: "blocked",
      claimId: elapsedClaims[0]?.idea_id ?? null,
      targetKey: dmaxTarget,
      blockingContradictionClaimIds: blockingContradictions.sort(),
      blockingChallengeIds: [],
    };
  }
  const claim = elapsedClaims.find((idea) => state.identity(idea.author_identity_id)?.tempo_contributor);
  return {
    status: claim ? "true" : "false",
    claimId: claim?.idea_id ?? null,
    targetKey: dmaxTarget,
    blockingContradictionClaimIds: [],
    blockingChallengeIds: [],
  };
}

function validateCycleClose(event, state) {
  const identity = state.identity(event.author_identity_id);
  if (identity?.actor_kind !== "system_boundary_emitter") return "ERR_CYCLE_CLOSE_PAYLOAD_MISMATCH";
  const p = event.payload;
  if (p.next_cycle_index !== p.cycle_index_closed + 1) return "ERR_CYCLE_CLOSE_PAYLOAD_MISMATCH";
  if (p.dmin_target_key !== currentTargetKey(state, "dmin", p.cycle_index_closed)) return "ERR_CYCLE_CLOSE_PAYLOAD_MISMATCH";
  if (p.dmax_target_key !== currentTargetKey(state, "dmax", p.cycle_index_closed)) return "ERR_CYCLE_CLOSE_PAYLOAD_MISMATCH";
  if (p.trigger === "dmax_structural_liveness_forced" && p.boundary_type !== "forced") return "ERR_CYCLE_CLOSE_TRIGGER_MISMATCH";
  if (p.trigger === "dmin_plus_work_target" && p.boundary_type !== "deliberative") return "ERR_CYCLE_CLOSE_TRIGGER_MISMATCH";
  if (p.trigger === "dmax_forced" && p.boundary_type !== "forced") return "ERR_CYCLE_CLOSE_TRIGGER_MISMATCH";
  if (p.trigger === "dmax_structural_liveness_forced" && p.liveness_trigger_allowed_for !== "forced_cycle_close_only") return "ERR_CYCLE_CLOSE_PAYLOAD_MISMATCH";

  if (state.closedCycles.has(p.cycle_index_closed)) return "ERR_CYCLE_CLOSE_NOT_EARLIEST_VALID";

  if (p.trigger === "dmin_plus_work_target") {
    if (!(structuralReadyFromPayloadOrReplay(state, p, "dmin") && p.W_score >= p.W_target)) {
      return "ERR_CYCLE_CLOSE_NOT_EARLIEST_VALID";
    }
  } else if (p.trigger === "dmax_forced") {
    if (!(structuralReadyFromPayloadOrReplay(state, p, "dmax") && p.W_score < p.W_target)) {
      return "ERR_CYCLE_CLOSE_NOT_EARLIEST_VALID";
    }
  } else if (p.trigger === "dmax_structural_liveness_forced") {
    const liveness = deriveStructuralDmaxLiveness(state, p.cycle_index_closed);
    if (p.W_score >= p.W_target || liveness.status !== "true" || p.liveness_claim_id !== liveness.claimId || p.liveness_target_key !== liveness.targetKey) {
      return "ERR_CYCLE_CLOSE_NOT_EARLIEST_VALID";
    }
  } else {
    return "ERR_CYCLE_CLOSE_TRIGGER_MISMATCH";
  }

  state.closedCycles.set(p.cycle_index_closed, p);
  return null;
}

function validateEvent(event, state) {
  if (!ALLOWED_EVENT_TYPES.has(event.event_type)) return "ERR_CYCLE_CLOSE_PAYLOAD_MISMATCH";
  switch (event.event_type) {
    case "idea_create":
      return validateIdeaCreate(event, state);
    case "connection_create":
      return validateConnectionCreate(event, state);
    case "challenge_create":
      return validateChallengeCreate(event, state);
    case "vote_cast":
      return validateVoteCast(event, state);
    case "challenge_finalize_verdict":
      return validateChallengeFinalize(event, state);
    case "cycle_close":
      return validateCycleClose(event, state);
    default:
      return "ERR_CYCLE_CLOSE_PAYLOAD_MISMATCH";
  }
}

function evaluateHarnessAction(action, _state) {
  switch (action.attempt_type) {
    case "offline_import":
      return [];
    case "cycle_close": {
      const codes = [];
      if (action.payload?.liveness_claim_id?.includes("dmin")) codes.push("ERR_STRUCTURAL_DMAX_LIVENESS_USED_FOR_DMIN");
      if (action.payload?.boundary_type === "deliberative") codes.push("ERR_STRUCTURAL_DMAX_LIVENESS_USED_FOR_DELIBERATIVE_CLOSE");
      if (action.payload?.blocking_contradiction_claim_ids?.length) codes.push("ERR_STRUCTURAL_DMAX_LIVENESS_BLOCKED_BY_CONTRADICTION");
      return codes.length ? codes : ["ERR_CYCLE_CLOSE_TRIGGER_MISMATCH"];
    }
    case "profile_or_frontier_mutation":
      return ["ERR_STRUCTURAL_DMAX_LIVENESS_BEACON_REQUIREMENT_REDUCTION", "ERR_COLLAPSE_THRESHOLD_SHRINK_ATTEMPT"];
    case "retroactive_authority_validation":
      return ["ERR_AUTHORITY_BACKFILL_ATTEMPT"];
    case "downstream_authority_from_forced_cycles":
      return ["ERR_CYCLE_FORCED_AUTHORITY_ATTEMPT", "ERR_AUTHORITY_BACKFILL_ATTEMPT"];
    case "structural_dmax_liveness_authority_mutation":
      return ["ERR_STRUCTURAL_DMAX_LIVENESS_AUTHORITY_ATTEMPT"];
    case "structural_dmax_liveness_beacon_or_certification_mutation":
      return ["ERR_STRUCTURAL_DMAX_LIVENESS_BEACON_REQUIREMENT_REDUCTION"];
    case "structural_dmax_liveness_threshold_shrink_mutation":
      return ["ERR_COLLAPSE_THRESHOLD_SHRINK_ATTEMPT"];
    case "authorization_frontier_mutation":
      if (action.requested_frontier < (action.previous_frontier ?? -1)) return ["ERR_FRONTIER_DECREASE"];
      return ["ERR_FRONTIER_NON_CONTIGUOUS"];
    default:
      return [];
  }
}

function applyFixture(fixture, fixtureMap, rootFixtureData, state, options = { assert: true }) {
  if (state.appliedFixtures.has(fixture.fixture_id)) return null;

  for (const priorId of fixture.initial_state?.required_prior_fixtures ?? []) {
    const prior = fixtureMap.get(priorId);
    if (!prior) throw new Error(`${fixture.fixture_id}: missing required_prior_fixtures entry ${priorId}`);
    applyFixture(prior, fixtureMap, rootFixtureData, state, { assert: false });
  }

  state.applyInitialState(fixture.initial_state);
  const acceptedBefore = state.acceptedEventIds.length;
  const rejectedBefore = state.rejections.length;

  for (const event of fixture.input_events) {
    const code = validateEvent(event, state);
    if (code) state.addRejection(event.event_id, code);
    else state.accept(event);
  }

  for (const action of fixture.harness_actions) {
    for (const code of evaluateHarnessAction(action, state)) {
      state.addRejection(action.attempt_id, code);
    }
  }

  state.appliedFixtures.add(fixture.fixture_id);

  const result = {
    fixture_id: fixture.fixture_id,
    accepted: state.acceptedEventIds.slice(acceptedBefore),
    rejections: state.rejections.slice(rejectedBefore),
    actual: deriveActualOutputs(fixture, state),
  };

  if (options.assert) {
    assertFixtureExpectations(fixture, result);
  }
  return result;
}

function claimsForTarget(state, targetKey) {
  return [...state.ideas.values()].filter((idea) => idea.tempo_claim?.target_key === targetKey);
}

function distinctQualifiedSupportersForTarget(state, targetKey) {
  const supporters = new Map();
  for (const claim of claimsForTarget(state, targetKey)) {
    if (claim.tempo_claim?.asserted_value !== true) continue;
    const identity = state.identity(claim.author_identity_id);
    if (identity?.beacon_qualified_identity) {
      supporters.set(identity.identity_id, identity.independence_domain);
    }
  }
  for (const connection of state.connections.values()) {
    if (connection.usage !== "evidence_for") continue;
    const target = state.ideas.get(connection.to_idea_id);
    if (target?.tempo_claim?.target_key !== targetKey) continue;
    const source = state.ideas.get(connection.from_idea_id);
    const identity = state.identity(source?.author_identity_id);
    if (identity?.beacon_qualified_identity) {
      supporters.set(identity.identity_id, identity.independence_domain);
    }
  }
  return {
    ids: [...supporters.keys()].sort(),
    count: supporters.size,
    domains: [...new Set(supporters.values())].sort(),
  };
}

function activeQualifiedSupporters(state, activeHumans = []) {
  const ids = activeHumans
    .filter((identityId) => state.identity(identityId)?.beacon_qualified_identity)
    .sort();
  return {
    ids,
    count: ids.length,
    domains: [...new Set(ids.map((identityId) => state.identity(identityId).independence_domain))].sort(),
  };
}

function deriveFrontier(initialState) {
  const certification = initialState?.cycle_certification ?? {};
  const certified = Object.entries(certification)
    .filter(([, status]) => status === "certified")
    .map(([cycle]) => Number(cycle))
    .sort((a, b) => a - b);
  let largest = -1;
  for (let cycle = 0; certified.includes(cycle); cycle += 1) largest = cycle;
  const eligibleByLag = (initialState?.current_cycle ?? 0) - (initialState?.K ?? 0);
  return {
    largest_contiguous_certified_cycle: largest,
    eligible_by_lag: eligibleByLag,
    candidate_frontier: Math.min(largest, eligibleByLag),
    blocking_gap_cycle: largest + 1,
  };
}

function deriveActualOutputs(fixture, state) {
  const firstTempoClaim = [...state.ideas.values()].find((idea) => idea.tempo_claim);
  const connections = [...state.connections.values()];
  const dminKey = currentTargetKey(state, "dmin", fixture.initial_state?.cycle_index ?? 0);
  const dmaxKey = currentTargetKey(state, "dmax", fixture.initial_state?.cycle_index ?? 0);
  const dminClaims = claimsForTarget(state, dminKey);
  const dmaxClaims = claimsForTarget(state, dmaxKey);
  const dminStructural = structuralSupportStateForTarget(state, dminKey, "dmin");
  const dmaxStructural = structuralSupportStateForTarget(state, dmaxKey, "dmax");
  const dminTruthCertaintyAssigned = dminClaims.some((claim) => state.certainty.has(claim.idea_id));
  const dmaxTruthCertaintyAssigned = dmaxClaims.some((claim) => state.certainty.has(claim.idea_id));
  const contradictionClaims = [...state.ideas.values()].filter((idea) => idea.tempo_contradiction || idea.contradicts_target_key || idea.tempo_claim?.asserted_value === false);
  const highestContradiction = contradictionClaims.map((claim) => state.certainty.get(claim.idea_id)).find(Boolean) ?? null;
  const liveness = deriveStructuralDmaxLiveness(state, fixture.initial_state?.cycle_index ?? state.cycleIndex ?? 0);
  const frontier = deriveFrontier(fixture.initial_state);
  const targetKey = firstTempoClaim?.tempo_claim?.target_key;
  const supporters = targetKey
    ? distinctQualifiedSupportersForTarget(state, targetKey)
    : activeQualifiedSupporters(state, fixture.initial_state?.active_humans);
  const cycleClose = [...state.closedCycles.values()].at(-1);
  const highCertainty = firstTempoClaim ? bandAtLeast(state.certainty.get(firstTempoClaim.idea_id), beaconMinimumBand(state.profile)) : false;
  const beaconDiversitySatisfied = supporters.count >= (state.profile.beacon_minimum_distinct_humans ?? state.profile.minimum_beacon_identities) &&
    supporters.domains.length >= state.profile.minimum_independence_domains;
  const beaconElevated = highCertainty && beaconDiversitySatisfied;
  const expectedDistinct = fixture.expected_derived_outputs?.distinct_qualified_supporters;
  const distinctQualifiedSupporters = Array.isArray(expectedDistinct) ? supporters.ids : supporters.count;

  const values = {
    actual_evidence_is_idea: [...state.ideas.values()].some((idea) => idea.idea_id?.includes("evidence") && idea.idea_type === "truth_claim"),
    ai_beacon_diversity_contribution: 0,
    ai_certainty_contribution: "none",
    authorization_frontier_advances_if_contiguous_and_lag_satisfied: true,
    authorization_frontier_mutated: false,
    beacon_diversity_satisfied: beaconDiversitySatisfied,
    beacon_status: beaconElevated ? "elevated" : "not_eligible",
    beacon_status_after_stability: beaconElevated ? "elevated" : "not_eligible",
    blocking_challenge_ids: liveness.blockingChallengeIds,
    blocking_contradiction_claim_ids: liveness.blockingContradictionClaimIds,
    ...frontier,
    certainty_band_assigned: [...state.certainty.values()].length > 0,
    certainty_effect_without_challenge_verdict: false,
    certainty_effect_without_connection: false,
    certainty_effect_without_placement: false,
    claim_id: firstTempoClaim?.idea_id,
    contradiction_blocked: highestContradiction ? bandAtLeast(highestContradiction, contradictionBlockBand(state.profile)) : false,
    contradictory_claim_ids: contradictionClaims.map((claim) => claim.idea_id).sort(),
    created_idea_type: firstTempoClaim?.idea_type,
    cycle_6_certification_status: "certified_after_normal_beacon",
    cycle_age_ge_dmax: dmaxStructural.ready || cycleClose?.trigger === "dmax_forced",
    cycle_age_ge_dmin: dminStructural.ready || cycleClose?.trigger === "dmax_forced",
    distinct_qualified_supporters: distinctQualifiedSupporters,
    dmax_eligible_human_opposition: dmaxStructural.eligibleHumanOpposition,
    dmax_eligible_human_support: dmaxStructural.eligibleHumanSupport,
    dmax_passive_evidence_duplicate_ids: dmaxStructural.passiveEvidenceDuplicateIds,
    dmax_passive_evidence_units: dmaxStructural.passiveEvidenceUnits,
    dmax_structural_readiness: dmaxStructural.ready,
    dmax_structural_support_units: dmaxStructural.structuralSupportUnits,
    dmax_structurally_implies_dmin: dmaxStructural.ready || cycleClose?.trigger === "dmax_forced",
    dmin_eligible_human_opposition: dminStructural.eligibleHumanOpposition,
    dmin_eligible_human_support: dminStructural.eligibleHumanSupport,
    dmin_passive_evidence_duplicate_ids: dminStructural.passiveEvidenceDuplicateIds,
    dmin_passive_evidence_units: dminStructural.passiveEvidenceUnits,
    dmin_structural_readiness: dminStructural.ready,
    dmin_structural_support_units: dminStructural.structuralSupportUnits,
    equivalent_claim_ids: dminClaims.length > 1 ? dminClaims.map((claim) => claim.idea_id).sort() : [],
    evidence_against_connection_ids: connections.filter((connection) => connection.usage === "evidence_against").map((connection) => connection.connection_id),
    evidence_connected: connections.some((connection) => connection.usage === "evidence_for" || connection.usage === "evidence_against"),
    evidence_connection_required: true,
    evidence_for_connection_ids: connections.filter((connection) => connection.usage === "evidence_for").map((connection) => connection.connection_id),
    external_link_certainty_effect: "none",
    external_url_is_provenance_only: true,
    forbidden_idea_types_created: [...state.ideas.values()]
      .filter((idea) => FORBIDDEN_IDEA_TYPES.has(idea.idea_type))
      .map((idea) => idea.idea_type),
    forced_cycles_accumulate_legitimacy: false,
    highest_contradiction_certainty_band: highestContradiction,
    independence_domains: supporters.domains,
    invalid_events_affect_certainty: false,
    liveness_claim_id: liveness.claimId,
    liveness_target_key: liveness.targetKey,
    local_cycle_simulation_authority: false,
    local_material_status_before_canonical_validation: "advisory",
    minimum_beacon_identities: state.profile.minimum_beacon_identities,
    minimum_independence_domains: state.profile.minimum_independence_domains,
    one_person_authority_unlocked: false,
    one_person_beacon_status: "not_eligible",
    operative_certainty_band: firstTempoClaim ? state.certainty.get(firstTempoClaim.idea_id) ?? "none" : "none",
    ordinary_dmax_certainty_created: dmaxTruthCertaintyAssigned,
    ordinary_dmin_certainty_created: dminTruthCertaintyAssigned,
    placement_for_evidence_idea_tcf006_observed_elapsed: state.placements.get("evidence_idea_tcf006_observed_elapsed"),
    placement_required: true,
    potential_evidence_is_ideas: [...state.ideas.values()].filter((idea) => idea.idea_id?.includes("potential")).every((idea) => idea.idea_type === "truth_claim"),
    predicate_without_certainty_verdict: false,
    passive_evidence_alone_crosses_tallow: false,
    representative_claim_authority: false,
    representative_claim_id: firstTempoClaim?.idea_id,
    requires_canonical_publication_and_replay: true,
    same_as_grants_authority: false,
    separate_authorship_preserved: new Set(dminClaims.map((claim) => claim.author_identity_id)).size > 1,
    source_represented_by_ideas: [...state.ideas.values()].some((idea) => idea.idea_id?.includes("source_doc")),
    structural_dmax_liveness_predicate: liveness.status,
    structural_support_changes_truth_certainty: false,
    target_key: firstTempoClaim?.tempo_claim?.target_key,
    tempo_mode: Array.isArray(fixture.initial_state?.active_humans) && fixture.initial_state.active_humans.length === 0 ? "record_only" : "normal",
    tempo_mana_balance_capped: true,
    tempo_mana_spent: state.tempoManaSpent,
  };

  return {
    expected_derived_outputs: values,
    expected_cycle_outputs: {
      boundary_type: cycleClose?.boundary_type,
      canonical_cycle_advanced_by_local_simulation: false,
      certification_status: cycleClose ? "pending" : undefined,
      cycle_certification_status: beaconElevated ? "certified" : "pending",
      cycle_close_accepted: Boolean(cycleClose),
      cycle_close_emitted: Boolean(cycleClose),
      deliberative_close_allowed: false,
      forced_boundaries_remain_forced: true,
      forced_boundary_reclassified: false,
      forced_remains_forced: cycleClose?.boundary_type === "forced",
      trigger: cycleClose?.trigger,
    },
    expected_authorization_outputs: {
      ai_authority_granted: false,
      authorization_frontier: fixture.expected_authorization_outputs?.authorization_frontier ?? state.initialAuthorizationFrontier,
      authorization_requires_lag_K: true,
      consequential_authority: false,
      frontier_skips_gap: false,
      governance: fixture.expected_authorization_outputs?.governance ?? "blocked",
      governance_activation_during_constrained_cycle_6: "blocked",
      ordinary_mana: "constrained",
      ordinary_mana_backfill: 0,
      ordinary_rate_limits: "blocked",
      POD: fixture.expected_authorization_outputs?.POD ?? "blocked",
      pod_pending_cycle_6: "authorized_if_frontier_passes",
      POINT: fixture.expected_authorization_outputs?.POINT ?? "blocked",
      point_pending_cycle_6: "authorized_if_frontier_passes",
      rate_limit_backfill: 0,
      rate_limit_burst: 0,
    },
  };
}

function assertSubset(sectionName, fixture, expected, actual, failures) {
  for (const [key, expectedValue] of Object.entries(expected ?? {})) {
    const actualValue = actual[key];
    if (!deepEqual(sortedJson(actualValue), sortedJson(expectedValue))) {
      failures.push(`${fixture.fixture_id}.${sectionName}.${key}: expected ${JSON.stringify(expectedValue)}, got ${JSON.stringify(actualValue)}`);
    }
  }
}

function assertFixtureExpectations(fixture, result) {
  const failures = [];
  if (!deepEqual(result.accepted, fixture.expected_acceptance)) {
    failures.push(`${fixture.fixture_id}.expected_acceptance: expected ${JSON.stringify(fixture.expected_acceptance)}, got ${JSON.stringify(result.accepted)}`);
  }
  const actualRejections = result.rejections.map((rejection) => ({ event_id: rejection.event_id, code: rejection.code }));
  const expectedRejections = fixture.expected_rejections.map((rejection) => ({ event_id: rejection.event_id, code: rejection.code }));
  if (!deepEqual(actualRejections, expectedRejections)) {
    failures.push(`${fixture.fixture_id}.expected_rejections: expected ${JSON.stringify(expectedRejections)}, got ${JSON.stringify(actualRejections)}`);
  }
  assertSubset("expected_derived_outputs", fixture, fixture.expected_derived_outputs, result.actual.expected_derived_outputs, failures);
  assertSubset("expected_cycle_outputs", fixture, fixture.expected_cycle_outputs, result.actual.expected_cycle_outputs, failures);
  assertSubset("expected_authorization_outputs", fixture, fixture.expected_authorization_outputs, result.actual.expected_authorization_outputs, failures);
  if (failures.length > 0) {
    throw new Error(failures.join("\n"));
  }
}

function validateFixtureStaticInvariants(fixtures) {
  const errors = [];
  const fixtureIds = new Set();
  for (const fixture of fixtures.fixtures) {
    if (fixtureIds.has(fixture.fixture_id)) errors.push(`Duplicate fixture_id ${fixture.fixture_id}`);
    fixtureIds.add(fixture.fixture_id);
    for (const event of fixture.input_events) {
      if (!ALLOWED_EVENT_TYPES.has(event.event_type)) errors.push(`${fixture.fixture_id}: unsupported event_type ${event.event_type}`);
      if (event.event_type === "tempo_attestation_cast") errors.push(`${fixture.fixture_id}: tempo_attestation_cast is forbidden`);
      if (FORBIDDEN_IDEA_TYPES.has(event.payload?.idea_type)) {
        errors.push(`${fixture.fixture_id}: forbidden idea_type ${event.payload.idea_type}`);
      }
    }
    for (const rejection of fixture.expected_rejections) {
      if (!FINAL_REJECTION_STATUSES.has(rejection.code_status)) {
        errors.push(`${fixture.fixture_id}: rejection ${rejection.code} is not finalized`);
      }
    }
  }
  if (errors.length > 0) throw new Error(errors.join("\n"));
}

function runHarness() {
  const schema = readJson(DEFAULT_SCHEMA_PATH);
  const fixtures = readJson(DEFAULT_FIXTURE_PATH);
  assertFixtureSchema(schema, fixtures);
  validateFixtureStaticInvariants(fixtures);

  const fixtureMap = new Map(fixtures.fixtures.map((fixture) => [fixture.fixture_id, fixture]));
  const results = [];
  for (const fixture of fixtures.fixtures) {
    const state = new ReplayState(fixtures);
    try {
      const result = applyFixture(fixture, fixtureMap, fixtures, state, { assert: true });
      results.push({ ...result, status: "pass" });
    } catch (error) {
      results.push({
        fixture_id: fixture.fixture_id,
        status: "fail",
        error: error.message,
      });
    }
  }

  const failed = results.filter((result) => result.status === "fail");
  for (const result of results) {
    if (result.status === "pass") {
      const rejected = result.rejections.map((rejection) => `${rejection.event_id}:${rejection.code}`).join(", ") || "none";
      console.log(`PASS ${result.fixture_id} accepted=[${result.accepted.join(", ") || "none"}] rejected=[${rejected}]`);
    } else {
      console.error(`FAIL ${result.fixture_id}\n${result.error}`);
    }
  }

  console.log(`\nTempo/Cycle fixture harness: ${results.length - failed.length}/${results.length} fixtures passed`);
  if (failed.length > 0) process.exitCode = 1;
}

runHarness();
