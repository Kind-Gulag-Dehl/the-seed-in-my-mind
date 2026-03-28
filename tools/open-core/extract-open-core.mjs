import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { promises as fs } from "node:fs";
import os from "node:os";
import path from "node:path";

const repoRoot = process.cwd();
const manifestPath = path.join(repoRoot, "tools", "open-core", "export-manifest.json");
const cliOptions = {
  zip: process.argv.includes("--zip")
};

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
      const options = inner.split(",").map((value) => value.trim()).filter(Boolean);
      expression += `(?:${options.map((value) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("|")})`;
      index = closeIndex;
      continue;
    }

    expression += escapeRegexChar(char);
  }
  expression += "$";
  return new RegExp(expression);
};

const toPosixRelative = (absolutePath) =>
  path.relative(repoRoot, absolutePath).split(path.sep).join("/");

const resolveInRepo = (relativePath) => path.join(repoRoot, ...relativePath.split("/"));

const removePathIfPresent = async (targetPath) => {
  await fs.rm(targetPath, { recursive: true, force: true }).catch(() => undefined);
};

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

const runCommand = (command, args, options = {}) => {
  const resolvedCommand = process.platform === "win32" && command === "npm" ? "npm.cmd" : command;
  const result = spawnSync(resolvedCommand, args, {
    cwd: options.cwd ?? repoRoot,
    stdio: "inherit",
    env: options.env ?? process.env
  });
  if (result.status !== 0) {
    throw new Error(`[open-core-extract] command failed: ${command} ${args.join(" ")}`);
  }
};

const runCommandCapture = (command, args, options = {}) => {
  const resolvedCommand = process.platform === "win32" && command === "npm" ? "npm.cmd" : command;
  const result = spawnSync(resolvedCommand, args, {
    cwd: options.cwd ?? repoRoot,
    stdio: ["ignore", "pipe", "pipe"],
    env: options.env ?? process.env,
    encoding: "utf8"
  });
  if (result.status !== 0) {
    const stderr = (result.stderr ?? "").trim();
    throw new Error(
      `[open-core-extract] command failed: ${command} ${args.join(" ")}${stderr ? ` :: ${stderr}` : ""}`
    );
  }
  return (result.stdout ?? "").trim();
};

const runPowerShellScript = (scriptPath, args = [], options = {}) => {
  if (process.platform === "win32") {
    runCommand("powershell", ["-ExecutionPolicy", "Bypass", "-File", scriptPath, ...args], options);
    return;
  }
  runCommand("pwsh", ["-NoLogo", "-NoProfile", "-File", scriptPath, ...args], options);
};

const assertPathExists = async (absolutePath, message) => {
  const exists = await fs.stat(absolutePath).then(() => true).catch(() => false);
  if (!exists) {
    throw new Error(message);
  }
};

const loadManifest = async () => {
  const raw = await fs.readFile(manifestPath, "utf8");
  return JSON.parse(raw);
};

const readEnvFile = async (filePath) => {
  const exists = await fs.stat(filePath).then(() => true).catch(() => false);
  if (!exists) {
    return {};
  }

  const raw = await fs.readFile(filePath, "utf8");
  const values = {};
  for (const line of raw.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) {
      continue;
    }
    const delimiter = trimmed.indexOf("=");
    if (delimiter === -1) {
      continue;
    }
    const key = trimmed.slice(0, delimiter).trim();
    const value = trimmed.slice(delimiter + 1).trim();
    if (!key) {
      continue;
    }
    values[key] = value;
  }
  return values;
};

const buildVerificationEnv = async () => {
  const sourceBackendEnv = await readEnvFile(path.join(repoRoot, "backend", ".env"));
  const mergedEnv = { ...process.env };

  for (const [key, value] of Object.entries(sourceBackendEnv)) {
    if (!mergedEnv[key]) {
      if (key === "PGPASSFILE" && value && !path.isAbsolute(value)) {
        mergedEnv[key] = path.join(repoRoot, "backend", value);
      } else {
        mergedEnv[key] = value;
      }
    }
  }

  if (!mergedEnv.DATABASE_URL && mergedEnv.database_url) {
    mergedEnv.DATABASE_URL = mergedEnv.database_url;
  }
  if (!mergedEnv.PGPASSFILE && mergedEnv.pgpassfile) {
    mergedEnv.PGPASSFILE = mergedEnv.pgpassfile;
  }

  return mergedEnv;
};

const extract = async (manifest) => {
  const exportRoot = resolveInRepo(manifest.export_root);
  const includePatterns = manifest.include_globs.map(globToRegExp);
  const excludePatterns = manifest.exclude_globs.map(globToRegExp);

  await fs.rm(exportRoot, { recursive: true, force: true });
  await fs.mkdir(exportRoot, { recursive: true });

  const allFiles = (await walkFiles(repoRoot))
    .map((filePath) => toPosixRelative(filePath))
    .filter((relativePath) => !relativePath.startsWith("_export/"))
    .sort((left, right) => left.localeCompare(right));

  let copiedCount = 0;
  let excludedCount = 0;
  const selectedFiles = [];

  for (const relativePath of allFiles) {
    const included = includePatterns.some((pattern) => pattern.test(relativePath));
    if (!included) {
      continue;
    }
    if (excludePatterns.some((pattern) => pattern.test(relativePath))) {
      excludedCount += 1;
      continue;
    }

    const sourcePath = resolveInRepo(relativePath);
    const destinationPath = path.join(exportRoot, ...relativePath.split("/"));
    await fs.mkdir(path.dirname(destinationPath), { recursive: true });
    await fs.copyFile(sourcePath, destinationPath);
    copiedCount += 1;
    selectedFiles.push(relativePath);
  }

  if (copiedCount === 0) {
    throw new Error("[open-core-extract] no files copied; check include_globs");
  }

  return { exportRoot, copiedCount, excludedCount, selectedFiles };
};

const verifyRequiredRootFiles = async (manifest, exportRoot) => {
  for (const relativePath of manifest.required_root_files) {
    await assertPathExists(
      path.join(exportRoot, ...relativePath.split("/")),
      `[open-core-extract] required root file missing in export: ${relativePath}`
    );
  }
};

const verifyRequiredAbsentPaths = async (manifest, exportRoot) => {
  for (const relativePath of manifest.required_absent_paths ?? []) {
    const exists = await fs
      .stat(path.join(exportRoot, ...relativePath.split("/")))
      .then(() => true)
      .catch(() => false);
    if (exists) {
      throw new Error(`[open-core-extract] path must be absent in export: ${relativePath}`);
    }
  }
};

const sanitizeBackendManifestsForOpenCore = async (exportRoot) => {
  const backendCargoTomlPath = path.join(exportRoot, "backend", "Cargo.toml");
  const apiServerCargoTomlPath = path.join(exportRoot, "backend", "bins", "api-server", "Cargo.toml");

  const backendCargoToml = await fs.readFile(backendCargoTomlPath, "utf8");
  const sanitizedBackendCargoToml = backendCargoToml
    .split(/\r?\n/)
    .filter((line) => line.trim() !== "\"crates/api-types-private\",")
    .join("\n");
  await fs.writeFile(backendCargoTomlPath, `${sanitizedBackendCargoToml}\n`, "utf8");

  let apiServerCargoToml = await fs.readFile(apiServerCargoTomlPath, "utf8");
  apiServerCargoToml = apiServerCargoToml.replace(/default = \["full"\]/g, "default = [\"open_core\"]");
  apiServerCargoToml = apiServerCargoToml.replace(/full = \["dep:api-types-private"\]/g, "full = []");
  apiServerCargoToml = apiServerCargoToml
    .split(/\r?\n/)
    .filter((line) => !line.trimStart().startsWith("api-types-private = "))
    .join("\n");
  await fs.writeFile(apiServerCargoTomlPath, `${apiServerCargoToml}\n`, "utf8");
};

const cleanupFrontendProjectArtifacts = async (projectRoot) => {
  const transientPaths = [
    "dist",
    "node_modules",
    "coverage",
    ".vite",
    "package-lock.json",
    "tsconfig.tsbuildinfo"
  ];

  await Promise.all(
    transientPaths.map((relativePath) => removePathIfPresent(path.join(projectRoot, relativePath)))
  );
};

const collectExportFiles = async (exportRoot) =>
  (await walkFiles(exportRoot))
    .map((absolutePath) => path.relative(exportRoot, absolutePath).split(path.sep).join("/"))
    .sort((left, right) => left.localeCompare(right));

const verifyMustNotAppear = async (manifest, exportRoot) => {
  const violations = [];
  const allExportFiles = await collectExportFiles(exportRoot);

  for (const rule of manifest.must_not_appear) {
    const includePattern = globToRegExp(rule.glob);
    const excludePatterns = (rule.exclude_globs ?? []).map(globToRegExp);
    const pattern = new RegExp(rule.pattern);

    for (const relativePath of allExportFiles) {
      if (!includePattern.test(relativePath)) {
        continue;
      }
      if (excludePatterns.some((excludePattern) => excludePattern.test(relativePath))) {
        continue;
      }
      const content = await fs.readFile(path.join(exportRoot, ...relativePath.split("/")), "utf8");
      const lines = content.split(/\r?\n/);
      lines.forEach((line, index) => {
        if (pattern.test(line)) {
          violations.push({
            description: rule.description,
            file: relativePath,
            line: index + 1,
            text: line.trim()
          });
        }
      });
    }
  }

  if (violations.length > 0) {
    console.error("[open-core-extract] must-not-appear violations:");
    violations.forEach((violation) => {
      console.error(
        `- ${violation.file}:${violation.line} (${violation.description}) -> ${violation.text}`
      );
    });
    throw new Error("[open-core-extract] export contains forbidden references");
  }
};

const verifyNpmProjects = async (manifest, exportRoot) => {
  for (const project of manifest.required_npm_projects) {
    const projectRoot = path.join(exportRoot, ...project.path.split("/"));
    const packageJsonPath = path.join(projectRoot, "package.json");
    await assertPathExists(
      packageJsonPath,
      `[open-core-extract] npm project missing package.json: ${project.path}`
    );

    const packageJson = JSON.parse(await fs.readFile(packageJsonPath, "utf8"));
    for (const scriptName of project.required_scripts) {
      if (!packageJson.scripts || typeof packageJson.scripts[scriptName] !== "string") {
        throw new Error(
          `[open-core-extract] npm project ${project.path} missing required script: ${scriptName}`
        );
      }
    }

    for (const command of project.commands) {
      runCommand(command[0], command.slice(1), { cwd: projectRoot });
    }

    await cleanupFrontendProjectArtifacts(projectRoot);
  }
};

const verifyRustWorkspaces = async (manifest, exportRoot, verificationEnv) => {
  for (const workspace of manifest.required_rust_workspaces) {
    const workspaceRoot = path.join(exportRoot, ...workspace.path.split("/"));
    const cargoTomlPath = path.join(workspaceRoot, "Cargo.toml");
    await assertPathExists(
      cargoTomlPath,
      `[open-core-extract] rust workspace missing Cargo.toml: ${workspace.path}`
    );
    const cargoToml = await fs.readFile(cargoTomlPath, "utf8");
    for (const member of workspace.required_workspace_members) {
      if (!cargoToml.includes(member)) {
        throw new Error(
          `[open-core-extract] rust workspace ${workspace.path} missing required member entry: ${member}`
        );
      }
    }

    const tempCargoTargetDir = await fs.mkdtemp(path.join(os.tmpdir(), "seed-open-core-export-rust-"));
    const env = {
      ...verificationEnv,
      CARGO_TARGET_DIR: tempCargoTargetDir,
      CARGO_INCREMENTAL: "0",
      CARGO_PROFILE_DEV_DEBUG: "0",
      CARGO_PROFILE_TEST_DEBUG: "0"
    };

    try {
      for (const command of workspace.commands) {
        runCommand(command[0], command.slice(1), { cwd: workspaceRoot, env });
      }
    } finally {
      await removePathIfPresent(tempCargoTargetDir);
    }
  }
};

const runBoundaryCheck = (manifest, exportRoot) => {
  const [command, ...args] = manifest.boundary_check_command;
  runCommand(command, [...args, "--repo-root", exportRoot], { cwd: exportRoot });
};

const runVerificationCommands = (manifest, exportRoot, verificationEnv) => {
  for (const command of manifest.verification_commands ?? []) {
    runCommand(command[0], command.slice(1), { cwd: exportRoot, env: verificationEnv });
  }
};

const runExportSmokeCheck = (exportRoot, verificationEnv) => {
  const smokeScriptPath = path.join(repoRoot, "tools", "open-core", "smoke-export.ps1");
  runPowerShellScript(smokeScriptPath, ["-ExportRoot", exportRoot], { cwd: repoRoot, env: verificationEnv });
};

const getExportMetadata = async (manifest) => {
  const manifestRaw = await fs.readFile(manifestPath, "utf8");
  const manifestSha256 = createHash("sha256").update(manifestRaw).digest("hex");
  const exportTimestamp = new Date().toISOString();
  let gitCommit = "unknown";
  try {
    gitCommit = runCommandCapture("git", ["rev-parse", "HEAD"], { cwd: repoRoot });
  } catch {
    gitCommit = "unknown";
  }

  return {
    exportTimestamp,
    gitCommit,
    manifestSha256,
    manifestVersion: String(manifest.version ?? "unknown")
  };
};

const writeExportInfo = async (exportRoot, metadata) => {
  const infoLines = [
    "export_name=the-seed-in-my-mind-open-core",
    "export_scope=reference node, deterministic replay and snapshot tooling, curated protocol docs, reference viewer",
    `export_timestamp=${metadata.exportTimestamp}`,
    `git_commit=${metadata.gitCommit}`,
    `manifest_version=${metadata.manifestVersion}`,
    `manifest_sha256=${metadata.manifestSha256}`,
    "extract_command=npm run extract:open-core",
    "verify_commands=npm run verify:backend | npm run verify:boundaries | npm run verify:canonical-dto | npm run extract:open-core | powershell -ExecutionPolicy Bypass -File tools/open-core/smoke-export.ps1",
    "reviewer_guide=docs/open-core-reviewer-guide.md",
    "demo_guide=docs/open-core-demo-flow.md",
    "implementation_status=docs/open-core-implementation-status.md",
    ""
  ].join("\n");

  await fs.writeFile(path.join(exportRoot, "EXPORT_INFO.txt"), infoLines, "utf8");
};

const createZipArtifact = async (manifest, exportRoot) => {
  const packageScriptPath = path.join(repoRoot, "tools", "open-core", "package-export.ps1");
  const outputZipPath = path.join(repoRoot, "tools", "open-core", "dist", "open-core-export.zip");
  const metadata = await getExportMetadata(manifest);

  runPowerShellScript(
    packageScriptPath,
    [
      "-ExportRoot",
      exportRoot,
      "-OutputZip",
      outputZipPath,
      "-ExportTimestamp",
      metadata.exportTimestamp,
      "-GitCommit",
      metadata.gitCommit,
      "-ManifestVersion",
      metadata.manifestVersion,
      "-ManifestSha256",
      metadata.manifestSha256
    ],
    { cwd: repoRoot }
  );
};

const main = async () => {
  const manifest = await loadManifest();
  const verificationEnv = await buildVerificationEnv();
  const { exportRoot, copiedCount, excludedCount } = await extract(manifest);

  await sanitizeBackendManifestsForOpenCore(exportRoot);
  await verifyRequiredRootFiles(manifest, exportRoot);
  await verifyRequiredAbsentPaths(manifest, exportRoot);
  await verifyMustNotAppear(manifest, exportRoot);
  await writeExportInfo(exportRoot, await getExportMetadata(manifest));
  runVerificationCommands(manifest, exportRoot, verificationEnv);
  runBoundaryCheck(manifest, exportRoot);
  await verifyRustWorkspaces(manifest, exportRoot, verificationEnv);
  await verifyNpmProjects(manifest, exportRoot);
  runExportSmokeCheck(exportRoot, verificationEnv);
  runCommand("node", ["scripts/verify-open-core-export.mjs"], { cwd: exportRoot, env: verificationEnv });

  console.log("[open-core-extract] report");
  console.log(`- export_root: ${toPosixRelative(exportRoot)}`);
  console.log(`- copied_files: ${copiedCount}`);
  console.log(`- excluded_files: ${excludedCount}`);
  if (cliOptions.zip) {
    await createZipArtifact(manifest, exportRoot);
    console.log("- zip_artifact: tools/open-core/dist/open-core-export.zip");
  }
  console.log("[open-core-extract] success");
};

await main();
