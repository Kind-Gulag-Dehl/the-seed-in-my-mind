import { promises as fs } from "node:fs";
import path from "node:path";

const root = process.cwd();
const artifactPath = path.join(root, "tools", "open-core", "public-api-contract.v1.json");
const rustDtoPath = path.join(root, "backend", "crates", "api-types-canonical", "src", "lib.rs");
const tsDtoPath = path.join(root, "frontend", "src", "shared", "types", "canonical.ts");
const routerPath = path.join(root, "backend", "bins", "api-server", "src", "server", "router.rs");
const queryTypesPath = path.join(root, "backend", "bins", "api-server", "src", "server", "types.rs");
const publicHandlersPath = path.join(root, "backend", "bins", "api-server", "src", "server", "handlers", "public.rs");
const migrationsPath = path.join(root, "backend", "migrations", "postgres");

const fail = (message) => {
  throw new Error(`[verify-public-api-contract] ${message}`);
};

const [
  artifactRaw,
  rustDtos,
  tsDtos,
  router,
  queryTypes,
  publicHandlers,
  migrationFiles
] = await Promise.all([
  fs.readFile(artifactPath, "utf8"),
  fs.readFile(rustDtoPath, "utf8"),
  fs.readFile(tsDtoPath, "utf8"),
  fs.readFile(routerPath, "utf8"),
  fs.readFile(queryTypesPath, "utf8"),
  fs.readFile(publicHandlersPath, "utf8"),
  fs.readdir(migrationsPath)
]);

const artifact = JSON.parse(artifactRaw);
if (artifact.schema_version !== 1 || artifact.api_contract_version !== "1.0.0") {
  fail("unexpected artifact or API contract version");
}
if (artifact.owner !== "repo:open_core") {
  fail("contract owner must remain repo:open_core");
}
if (artifact.scope !== "public_product_read_contract") {
  fail("unexpected public contract scope");
}
if (artifact.snapshot_pin?.query_parameter !== "snapshot_height") {
  fail("snapshot pin must remain snapshot_height");
}
if (!queryTypes.includes("snapshot_height: Option<String>")) {
  fail("Rust query DTOs do not expose snapshot_height");
}

const sqlMigrations = migrationFiles
  .filter((name) => /^\d{4}_.+\.sql$/.test(name))
  .sort((left, right) => left.localeCompare(right));
const migrationHead = sqlMigrations.at(-1)?.replace(/\.sql$/, "");
if (migrationHead !== artifact.expected_migration_head) {
  fail(`migration head drift: artifact=${artifact.expected_migration_head} filesystem=${migrationHead}`);
}

for (const literal of [
  `API_CONTRACT_VERSION: &str = "${artifact.api_contract_version}"`,
  `SNAPSHOT_FORMAT_VERSION: &str = "${artifact.snapshot_format_version}"`,
  `EXPECTED_MIGRATION_HEAD: &str = "${artifact.expected_migration_head}"`
]) {
  if (!rustDtos.includes(literal)) {
    fail(`missing Rust contract constant: ${literal}`);
  }
}

const requiredBasisFields = new Set(artifact.snapshot_basis_fields ?? []);
const basisMatch = rustDtos.match(/pub struct SnapshotBasis\s*\{([\s\S]*?)\n\}/);
if (!basisMatch) {
  fail("missing Rust SnapshotBasis DTO");
}
for (const field of requiredBasisFields) {
  if (!new RegExp(`pub\\s+${field}\\s*:`).test(basisMatch[1])) {
    fail(`SnapshotBasis missing field ${field}`);
  }
  if (!new RegExp(`\\b${field}\\s*:`).test(tsDtos)) {
    fail(`TypeScript DTO surface missing basis field ${field}`);
  }
}

const seenPaths = new Set();
for (const endpoint of artifact.endpoints ?? []) {
  if (endpoint.method !== "GET") {
    fail(`non-read endpoint in public contract: ${endpoint.method} ${endpoint.path}`);
  }
  if (seenPaths.has(endpoint.path)) {
    fail(`duplicate endpoint path ${endpoint.path}`);
  }
  seenPaths.add(endpoint.path);
  if (!router.includes(`"${endpoint.path}"`)) {
    fail(`router is missing ${endpoint.path}`);
  }
  if (endpoint.bounded !== true) {
    fail(`endpoint is not declared bounded: ${endpoint.path}`);
  }
  if (endpoint.response_dto) {
    const rustStruct = new RegExp(`pub struct ${endpoint.response_dto}\\s*\\{`);
    if (!rustStruct.test(rustDtos)) {
      fail(`missing Rust response DTO ${endpoint.response_dto}`);
    }
  }
}

for (const endpoint of artifact.endpoints.filter((entry) => entry.snapshot_pinned)) {
  if (endpoint.path === "/api/v0/snapshot/:height") {
    continue;
  }
  if (!publicHandlers.includes("resolve_snapshot")) {
    fail(`snapshot resolver missing for ${endpoint.path}`);
  }
}

const expectedKinds = artifact.supported_canonical_signed_write_kinds ?? [];
for (const kind of expectedKinds) {
  if (!publicHandlers.includes(`"${kind}".to_string()`)) {
    fail(`capabilities handler missing supported signed kind ${kind}`);
  }
}
if (expectedKinds.some((kind) => !["idea_create", "connection_create"].includes(kind))) {
  fail("contract widened canonical signed write kinds");
}

for (const [name, expected] of Object.entries({
  page_max: 200,
  batch_idea_ids_max: 200,
  exact_match_max: 200,
  coordinates_max: 200,
  neighborhood_depth_max: 2,
  neighborhood_per_hop_max: 200
})) {
  if (artifact.limits?.[name] !== expected) {
    fail(`unexpected bound ${name}`);
  }
}

console.log(
  `[verify-public-api-contract] ok (${artifact.endpoints.length} endpoints, ${requiredBasisFields.size} basis fields, migration ${migrationHead})`
);
