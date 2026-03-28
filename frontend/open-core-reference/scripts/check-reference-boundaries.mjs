import { promises as fs } from "node:fs";
import path from "node:path";

const rootDir = process.cwd();
const sourceFileRegex = /\.(tsx?|jsx?)$/i;
const targetRoot = path.join(rootDir, "src");
const forbiddenPatterns = [
  /from\s+["'][^"']*domains\/private\/[^"']*["']/,
  /from\s+["'][^"']*domains\/private-overlay\/[^"']*["']/,
  /from\s+["'][^"']*api\/private[^"']*["']/,
  /from\s+["'][^"']*app\/workspaceshell[^"']*["']/,
  /from\s+["'][^"']*app\/tabstate[^"']*["']/,
  /from\s+["'][^"']*components\/viewers\/builderview[^"']*["']/
];

const walk = async (target) => {
  const stat = await fs.stat(target);
  if (stat.isFile()) {
    return sourceFileRegex.test(path.basename(target)) ? [target] : [];
  }
  const entries = await fs.readdir(target, { withFileTypes: true });
  const nested = await Promise.all(
    entries.map(async (entry) => {
      const fullPath = path.join(target, entry.name);
      if (entry.isDirectory()) {
        return walk(fullPath);
      }
      return sourceFileRegex.test(entry.name) ? [fullPath] : [];
    })
  );
  return nested.flat();
};

const toPosixRelative = (absolutePath) =>
  path.relative(rootDir, absolutePath).split(path.sep).join("/");

const main = async () => {
  const files = await walk(targetRoot);
  const violations = [];

  for (const filePath of files) {
    const content = await fs.readFile(filePath, "utf8");
    const lines = content.split(/\r?\n/);
    lines.forEach((line, index) => {
      if (forbiddenPatterns.some((pattern) => pattern.test(line))) {
        violations.push({
          file: toPosixRelative(filePath),
          line: index + 1,
          text: line.trim()
        });
      }
    });
  }

  if (violations.length > 0) {
    console.error("[open-core-reference] forbidden imports found:");
    violations.forEach((violation) => {
      console.error(`- ${violation.file}:${violation.line} -> ${violation.text}`);
    });
    process.exit(1);
  }

  console.log(`[open-core-reference] boundary check ok (scanned ${files.length} files)`);
};

await main();
