import { promises as fs } from "node:fs";
import path from "node:path";

const parseArgs = () => {
  const args = process.argv.slice(2);
  const options = {
    repoRoot: process.cwd(),
    exportRoot: null
  };

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    const next = args[index + 1];

    if (arg === "--repo-root" && next) {
      options.repoRoot = path.resolve(next);
      index += 1;
      continue;
    }

    if (arg === "--export-root" && next) {
      options.exportRoot = path.resolve(next);
      index += 1;
    }
  }

  return options;
};

const options = parseArgs();
const repoRoot = options.repoRoot;
const manifestPath = path.join(repoRoot, "tools", "open-core", "export-manifest.json");

const escapeRegexChar = (char) => /[|\\{}()[\]^$+?.]/.test(char) ? `\\${char}` : char;

const globToRegExp = (glob) => {
  let expression = "^";
  for (let index = 0; index < glob.length; index += 1) {
    const char = glob[index];
    const next = glob[index + 1];

    if (char === "*") {
      if (next === "*") {
        index += 1;
        if (glob[index + 1] === "/") {
          index += 1;
          expression += "(?:.*/)?";
        } else {
          expression += ".*";
        }
      } else {
        expression += "[^/]*";
      }
      continue;
    }

    if (char === "?") {
      expression += "[^/]";
      continue;
    }

    if (char === "{") {
      const closeIndex = glob.indexOf("}", index);
      if (closeIndex === -1) {
        expression += "\\{";
        continue;
      }
      const inner = glob.slice(index + 1, closeIndex);
      const variants = inner
        .split(",")
        .map((value) => value.trim())
        .filter(Boolean);
      expression += `(?:${variants.map((value) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("|")})`;
      index = closeIndex;
      continue;
    }

    expression += escapeRegexChar(char);
  }
  expression += "$";
  return new RegExp(expression);
};

const toPosix = (absolutePath, basePath) =>
  path.relative(basePath, absolutePath).split(path.sep).join("/");

const walkFiles = async (target) => {
  const stat = await fs.stat(target);
  if (stat.isFile()) {
    return [target];
  }
  const entries = await fs.readdir(target, { withFileTypes: true });
  const nested = await Promise.all(
    entries.map(async (entry) => {
      const fullPath = path.join(target, entry.name);
      if (entry.isDirectory()) {
        return walkFiles(fullPath);
      }
      return [fullPath];
    })
  );
  return nested.flat();
};

const exists = async (targetPath) => fs.stat(targetPath).then(() => true).catch(() => false);

const loadManifest = async () => JSON.parse(await fs.readFile(manifestPath, "utf8"));

const resolveExportRoot = async (manifest) => {
  if (options.exportRoot) {
    return options.exportRoot;
  }
  // Current behavior is intentionally dual-use:
  // - if the repo root already carries EXPORT_INFO.txt, verify the repo tree itself
  // - otherwise, verify the generated export tree at manifest.export_root
  if (await exists(path.join(repoRoot, "EXPORT_INFO.txt"))) {
    return repoRoot;
  }
  return path.join(repoRoot, ...manifest.export_root.split("/"));
};

const verifyExportInfo = async (exportRoot) => {
  const infoPath = path.join(exportRoot, "EXPORT_INFO.txt");
  if (!await exists(infoPath)) {
    throw new Error("[verify-open-core-export] missing EXPORT_INFO.txt");
  }
  const content = await fs.readFile(infoPath, "utf8");
  const requiredKeys = [
    "export_name=",
    "export_scope=",
    "export_timestamp=",
    "git_commit=",
    "manifest_version=",
    "manifest_sha256=",
    "extract_command=",
    "verify_commands="
  ];

  const missing = requiredKeys.filter((key) => !content.includes(key));
  if (missing.length > 0) {
    throw new Error(
      `[verify-open-core-export] EXPORT_INFO.txt missing required entries: ${missing.join(", ")}`
    );
  }
};

const verifyRequiredRootFiles = async (manifest, exportRoot) => {
  const missing = [];
  for (const relativePath of manifest.required_root_files ?? []) {
    if (!await exists(path.join(exportRoot, ...relativePath.split("/")))) {
      missing.push(relativePath);
    }
  }

  if (missing.length > 0) {
    throw new Error(
      `[verify-open-core-export] missing required files: ${missing.join(", ")}`
    );
  }
};

const verifyRequiredAbsentPaths = async (manifest, exportRoot) => {
  const present = [];
  for (const relativePath of manifest.required_absent_paths ?? []) {
    if (await exists(path.join(exportRoot, ...relativePath.split("/")))) {
      present.push(relativePath);
    }
  }

  if (present.length > 0) {
    throw new Error(
      `[verify-open-core-export] forbidden paths are present: ${present.join(", ")}`
    );
  }
};

const verifyForbiddenPaths = async (manifest, exportRoot, exportFiles) => {
  const patterns = (manifest.forbidden_path_globs ?? []).map((glob) => ({
    glob,
    pattern: globToRegExp(glob)
  }));
  const hits = [];

  for (const relativePath of exportFiles) {
    for (const { glob, pattern } of patterns) {
      if (pattern.test(relativePath)) {
        hits.push(`${relativePath} matches ${glob}`);
      }
    }
  }

  if (hits.length > 0) {
    throw new Error(`[verify-open-core-export] forbidden export paths detected:\n- ${hits.join("\n- ")}`);
  }
};

const verifyMustNotAppear = async (manifest, exportRoot, exportFiles) => {
  const violations = [];

  for (const rule of manifest.must_not_appear ?? []) {
    const includePattern = globToRegExp(rule.glob);
    const excludePatterns = (rule.exclude_globs ?? []).map(globToRegExp);
    const textPattern = new RegExp(rule.pattern);

    for (const relativePath of exportFiles) {
      if (!includePattern.test(relativePath)) {
        continue;
      }
      if (excludePatterns.some((pattern) => pattern.test(relativePath))) {
        continue;
      }

      const content = await fs.readFile(path.join(exportRoot, ...relativePath.split("/")), "utf8");
      const lines = content.split(/\r?\n/);
      lines.forEach((line, index) => {
        if (textPattern.test(line)) {
          violations.push(`${relativePath}:${index + 1} -> ${line.trim()}`);
        }
      });
    }
  }

  if (violations.length > 0) {
    throw new Error(`[verify-open-core-export] forbidden references detected:\n- ${violations.join("\n- ")}`);
  }
};

const main = async () => {
  const manifest = await loadManifest();
  const exportRoot = await resolveExportRoot(manifest);
  if (!await exists(exportRoot)) {
    throw new Error(`[verify-open-core-export] export root not found: ${exportRoot}`);
  }

  const exportFiles = (await walkFiles(exportRoot))
    .map((absolutePath) => toPosix(absolutePath, exportRoot))
    .sort((left, right) => left.localeCompare(right));

  await verifyExportInfo(exportRoot);
  await verifyRequiredRootFiles(manifest, exportRoot);
  await verifyRequiredAbsentPaths(manifest, exportRoot);
  await verifyForbiddenPaths(manifest, exportRoot, exportFiles);
  await verifyMustNotAppear(manifest, exportRoot, exportFiles);

  console.log("[verify-open-core-export] ok");
  console.log("- mode: validates the resolved export root only; forbidden-path failures usually mean the target tree contains build/runtime artifacts that must not ship");
  console.log(`- export_root: ${exportRoot}`);
  console.log(`- files_scanned: ${exportFiles.length}`);
};

await main();
