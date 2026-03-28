import { spawnSync } from "node:child_process";
import { promises as fs } from "node:fs";
import path from "node:path";

const parseArgs = () => {
  const args = process.argv.slice(2);
  const options = {
    repoRoot: process.cwd(),
    frontendDir: "frontend",
    frontendBoundaryScript: "lint:boundaries"
  };

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    const next = args[index + 1];
    if (arg === "--repo-root" && next) {
      options.repoRoot = path.resolve(next);
      index += 1;
      continue;
    }
    if (arg === "--frontend-dir" && next) {
      options.frontendDir = next;
      index += 1;
      continue;
    }
    if (arg === "--frontend-boundary-script" && next) {
      options.frontendBoundaryScript = next;
      index += 1;
      continue;
    }
  }

  return options;
};

const options = parseArgs();
const repoRoot = options.repoRoot;
const sourceFileRegex = /\.(rs|toml)$/i;
const frontendBoundaryCommand = [
  "--prefix",
  options.frontendDir,
  "run",
  options.frontendBoundaryScript
];
const npmCmd = process.platform === "win32" ? "npm.cmd" : "npm";

const backendForbiddenPatterns = [
  /api-types-private/,
  /api_types_private/
];

const backendIgnoredPathSegments = [
  `${path.sep}target${path.sep}`,
  `${path.sep}target-codex${path.sep}`,
  `${path.sep}api-types-private${path.sep}`
];

const toPosixRelative = (absolutePath) =>
  path.relative(repoRoot, absolutePath).split(path.sep).join("/");

const walk = async (target) => {
  const stat = await fs.stat(target);
  if (stat.isFile()) {
    return sourceFileRegex.test(path.basename(target)) ? [target] : [];
  }

  const entries = await fs.readdir(target, { withFileTypes: true });
  const files = await Promise.all(
    entries.map(async (entry) => {
      const fullPath = path.join(target, entry.name);
      if (entry.isDirectory()) {
        return walk(fullPath);
      }
      return sourceFileRegex.test(entry.name) ? [fullPath] : [];
    })
  );
  return files.flat();
};

const runFrontendBoundaryCheck = () => {
  const result = spawnSync(npmCmd, frontendBoundaryCommand, {
    cwd: repoRoot,
    encoding: "utf8"
  });

  const stdout = result.stdout?.trim();
  const stderr = result.stderr?.trim();

  if (stdout) {
    console.log(stdout);
  }
  if (stderr) {
    console.error(stderr);
  }

  if (result.status !== 0) {
    throw new Error("[open-core-boundary] frontend boundary check failed");
  }
};

const runBackendBoundaryCheck = async () => {
  const backendCratesDir = path.join(repoRoot, "backend", "crates");
  const files = await walk(backendCratesDir);
  const violations = [];

  for (const filePath of files) {
    if (backendIgnoredPathSegments.some((segment) => filePath.includes(segment))) {
      continue;
    }
    const content = await fs.readFile(filePath, "utf8");
    const lines = content.split(/\r?\n/);
    lines.forEach((line, index) => {
      const hit = backendForbiddenPatterns.find((pattern) => pattern.test(line));
      if (hit) {
        violations.push({
          file: toPosixRelative(filePath),
          line: index + 1,
          text: line.trim()
        });
      }
    });
  }

  if (violations.length > 0) {
    console.error("[open-core-boundary] backend forbidden dependency references found:");
    violations.forEach((violation) => {
      console.error(`- ${violation.file}:${violation.line} -> ${violation.text}`);
    });
    throw new Error("[open-core-boundary] backend boundary check failed");
  }

  console.log(`[open-core-boundary] backend check ok (scanned ${files.length} files)`);
};

const main = async () => {
  if (!await fs
    .stat(path.join(repoRoot, options.frontendDir))
    .then((stat) => stat.isDirectory())
    .catch(() => false)) {
    throw new Error(`[open-core-boundary] frontend directory not found: ${options.frontendDir}`);
  }

  runFrontendBoundaryCheck();
  await runBackendBoundaryCheck();
  console.log("[open-core-boundary] all checks passed");
};

await main();
