import { promises as fs } from "node:fs";
import path from "node:path";

const repoRoot = process.cwd();
const rustTypesPath = path.join(repoRoot, "backend", "crates", "api-types-canonical", "src", "lib.rs");
const tsTypesPath = path.join(repoRoot, "frontend", "src", "shared", "types", "canonical.ts");
const surfacePath = path.join(repoRoot, "tools", "open-core", "canonical-dto-surface.json");

const parseRustStructFields = (source) => {
  const lines = source.split(/\r?\n/);
  const structs = new Map();
  let currentStruct = null;

  for (const line of lines) {
    const structMatch = line.match(/^pub struct ([A-Za-z0-9_]+)\s*\{/);
    if (structMatch) {
      currentStruct = structMatch[1];
      structs.set(currentStruct, new Set());
      continue;
    }
    if (currentStruct && line.trim() === "}") {
      currentStruct = null;
      continue;
    }
    if (!currentStruct) {
      continue;
    }
    const fieldMatch = line.match(/^\s*pub\s+([A-Za-z0-9_]+)\s*:/);
    if (fieldMatch) {
      structs.get(currentStruct).add(fieldMatch[1]);
    }
  }

  return structs;
};

const parseTsInterfaces = (source) => {
  const lines = source.split(/\r?\n/);
  const interfaces = new Map();
  let currentInterface = null;

  for (const line of lines) {
    const interfaceMatch = line.match(/^export interface ([A-Za-z0-9_]+)(?: extends ([A-Za-z0-9_]+))?\s*\{/);
    if (interfaceMatch) {
      currentInterface = interfaceMatch[1];
      interfaces.set(currentInterface, {
        extendsName: interfaceMatch[2] ?? null,
        ownFields: new Set()
      });
      continue;
    }
    if (currentInterface && line.trim() === "}") {
      currentInterface = null;
      continue;
    }
    if (!currentInterface) {
      continue;
    }
    const fieldMatch = line.match(/^\s*([A-Za-z0-9_]+)\??\s*:/);
    if (fieldMatch) {
      interfaces.get(currentInterface).ownFields.add(fieldMatch[1]);
    }
  }

  return interfaces;
};

const resolveTsFields = (interfaces, interfaceName, activeStack = []) => {
  const entry = interfaces.get(interfaceName);
  if (!entry) {
    return null;
  }
  if (activeStack.includes(interfaceName)) {
    throw new Error(`[verify-canonical-dto] cycle detected in TS interfaces: ${activeStack.join(" -> ")} -> ${interfaceName}`);
  }
  const resolved = new Set(entry.ownFields);
  if (entry.extendsName) {
    const baseFields = resolveTsFields(interfaces, entry.extendsName, [...activeStack, interfaceName]);
    if (!baseFields) {
      throw new Error(`[verify-canonical-dto] TS interface ${interfaceName} extends unknown interface ${entry.extendsName}`);
    }
    for (const field of baseFields) {
      resolved.add(field);
    }
  }
  return resolved;
};

const sorted = (items) => [...items].sort((a, b) => a.localeCompare(b));

const verify = async () => {
  const [rustSource, tsSource, surfaceRaw] = await Promise.all([
    fs.readFile(rustTypesPath, "utf8"),
    fs.readFile(tsTypesPath, "utf8"),
    fs.readFile(surfacePath, "utf8")
  ]);

  const surface = JSON.parse(surfaceRaw);
  const rustStructs = parseRustStructFields(rustSource);
  const tsInterfaces = parseTsInterfaces(tsSource);

  const failures = [];
  for (const contract of surface.interfaces ?? []) {
    const rustFields = rustStructs.get(contract.name);
    if (!rustFields) {
      failures.push(`${contract.name}: missing Rust struct in api-types-canonical`);
      continue;
    }

    const tsFields = resolveTsFields(tsInterfaces, contract.name);
    if (!tsFields) {
      failures.push(`${contract.name}: missing TS interface in frontend/src/shared/types/canonical.ts`);
      continue;
    }

    const allowRustOnly = new Set(contract.allow_rust_only ?? []);
    const allowTsOnly = new Set(contract.allow_ts_only ?? []);
    const rustOnly = sorted(
      [...rustFields].filter((field) => !tsFields.has(field) && !allowRustOnly.has(field))
    );
    const tsOnly = sorted(
      [...tsFields].filter((field) => !rustFields.has(field) && !allowTsOnly.has(field))
    );

    if (rustOnly.length > 0 || tsOnly.length > 0) {
      const parts = [];
      if (rustOnly.length > 0) {
        parts.push(`rust-only=[${rustOnly.join(", ")}]`);
      }
      if (tsOnly.length > 0) {
        parts.push(`ts-only=[${tsOnly.join(", ")}]`);
      }
      failures.push(`${contract.name}: ${parts.join(" ")}`);
    }
  }

  if (failures.length > 0) {
    console.error("[verify-canonical-dto] canonical dto drift detected:");
    failures.forEach((failure) => console.error(`- ${failure}`));
    process.exit(1);
  }

  console.log(`[verify-canonical-dto] ok (${surface.interfaces.length} interface contracts verified)`);
};

await verify();
