import crypto from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repositoryRoot = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "..",
);
const sourcePath = path.join(
  repositoryRoot,
  "docs",
  "identity-admission-and-invitation-capacity-spec-v0.md",
);
const fixturePath = path.join(
  repositoryRoot,
  "docs",
  "conformance",
  "identity-admission-source-encoding-negative.v0.json",
);

const expectedSource = {
  byteLength: 445_992,
  lineFeeds: 13_953,
  sha256: "063c7db8f73c79e50db870d7a43392188dc2a42f377211117d31f10301445a0c",
  unicodeCounts: new Map([
    [0x2014, 12],
    [0x2019, 110],
    [0x201c, 9],
    [0x201d, 9],
    [0x2192, 28],
  ]),
};

const frozenV3 = {
  byteLength: 785_725,
  sha256: "914effee3392c8ad3e233cc703b9aae82fcb6bd00d02babd4c62532a76a20b01",
  mojibakePasses: 8,
};

const knownMojibakePattern =
  /[\u0080-\u009f\u00c2\u00c3\u00e2\u00c5\u00c6\u0192]/u;
const forbiddenControlPattern =
  /[\u0000-\u0008\u000b\u000c\u000e-\u001f\u007f]/u;

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function decodeUtf8Strict(bytes, label) {
  try {
    return new TextDecoder("utf-8", {
      fatal: true,
      ignoreBOM: true,
    }).decode(bytes);
  } catch (error) {
    throw new Error(`${label}: invalid_utf8: ${error.message}`);
  }
}

function validateEncoding(bytes, { document = true, label }) {
  const errors = [];
  let text;

  try {
    text = decodeUtf8Strict(bytes, label);
  } catch {
    return { errors: ["invalid_utf8"], text: null };
  }

  if (bytes.subarray(0, 3).equals(Buffer.from([0xef, 0xbb, 0xbf]))) {
    errors.push("utf8_bom_present");
  }
  if (text.includes("\uFEFF")) {
    errors.push("unicode_bom_present");
  }
  if (text.includes("\uFFFD")) {
    errors.push("replacement_character_present");
  }
  if (text !== text.normalize("NFC")) {
    errors.push("not_nfc");
  }
  if (knownMojibakePattern.test(text)) {
    errors.push("known_mojibake_pattern");
  }
  if (forbiddenControlPattern.test(text)) {
    errors.push("forbidden_control_character");
  }
  if (!Buffer.from(text, "utf8").equals(bytes)) {
    errors.push("utf8_round_trip_mismatch");
  }

  if (document) {
    if (text.includes("\r")) {
      errors.push("carriage_return_present");
    }
    if (!text.endsWith("\n")) {
      errors.push("missing_final_lf");
    }
  }

  return { errors, text };
}

function countNonAsciiCodePoints(text) {
  const counts = new Map();
  for (const character of text) {
    const codePoint = character.codePointAt(0);
    if (codePoint > 0x7f) {
      counts.set(codePoint, (counts.get(codePoint) ?? 0) + 1);
    }
  }
  return counts;
}

function compareCodePointCounts(actual, expected) {
  const errors = [];
  for (const [codePoint, expectedCount] of expected) {
    const actualCount = actual.get(codePoint) ?? 0;
    if (actualCount !== expectedCount) {
      errors.push(
        `unicode_count_mismatch U+${codePoint
          .toString(16)
          .toUpperCase()}: expected=${expectedCount} actual=${actualCount}`,
      );
    }
  }
  for (const [codePoint, actualCount] of actual) {
    if (!expected.has(codePoint)) {
      errors.push(
        `unexpected_non_ascii_code_point U+${codePoint
          .toString(16)
          .toUpperCase()}: count=${actualCount}`,
      );
    }
  }
  return errors;
}

function recreateFrozenV3Bytes(repairedText) {
  const windows1252 = new TextDecoder("windows-1252", {
    ignoreBOM: true,
  });
  let text = repairedText;
  for (let pass = 0; pass < frozenV3.mojibakePasses; pass += 1) {
    text = windows1252.decode(Buffer.from(text, "utf8"));
  }
  return Buffer.from(text, "utf8");
}

const sourceBytes = fs.readFileSync(sourcePath);
const sourceValidation = validateEncoding(sourceBytes, {
  document: true,
  label: "identity admission authority",
});
const failures = [...sourceValidation.errors];

if (sourceBytes.length !== expectedSource.byteLength) {
  failures.push(
    `source_byte_length_mismatch expected=${expectedSource.byteLength} actual=${sourceBytes.length}`,
  );
}
if (sha256(sourceBytes) !== expectedSource.sha256) {
  failures.push(
    `source_sha256_mismatch expected=${expectedSource.sha256} actual=${sha256(sourceBytes)}`,
  );
}

if (sourceValidation.text !== null) {
  const lineFeeds = [...sourceBytes].filter((byte) => byte === 0x0a).length;
  if (lineFeeds !== expectedSource.lineFeeds) {
    failures.push(
      `source_lf_count_mismatch expected=${expectedSource.lineFeeds} actual=${lineFeeds}`,
    );
  }

  failures.push(
    ...compareCodePointCounts(
      countNonAsciiCodePoints(sourceValidation.text),
      expectedSource.unicodeCounts,
    ),
  );

  const reconstructionNormalized = Buffer.from(
    sourceValidation.text
      .normalize("NFC")
      .replace(/\r\n/g, "\n")
      .replace(/\r/g, "\n"),
    "utf8",
  );
  if (!reconstructionNormalized.equals(sourceBytes)) {
    failures.push("reconstruction_normalization_changes_repaired_source");
  }

  const recreatedFrozenBytes = recreateFrozenV3Bytes(sourceValidation.text);
  if (
    recreatedFrozenBytes.length !== frozenV3.byteLength ||
    sha256(recreatedFrozenBytes) !== frozenV3.sha256
  ) {
    failures.push(
      `frozen_v3_round_trip_mismatch expected_bytes=${frozenV3.byteLength} expected_sha256=${frozenV3.sha256} actual_bytes=${recreatedFrozenBytes.length} actual_sha256=${sha256(recreatedFrozenBytes)}`,
    );
  }
}

const fixture = JSON.parse(fs.readFileSync(fixturePath, "utf8"));
const fixtureBytes = Buffer.from(fixture.sample.utf8_base64, "base64");
if (fixture.authority_status !== "non_authoritative_negative_fixture") {
  failures.push("negative_fixture_authority_status_invalid");
}
if (fixture.expected_valid !== false) {
  failures.push("negative_fixture_expected_valid_must_be_false");
}
if (fixtureBytes.length !== fixture.sample.utf8_byte_length) {
  failures.push(
    `negative_fixture_byte_length_mismatch expected=${fixture.sample.utf8_byte_length} actual=${fixtureBytes.length}`,
  );
}
if (sha256(fixtureBytes) !== fixture.sample.sha256) {
  failures.push(
    `negative_fixture_sha256_mismatch expected=${fixture.sample.sha256} actual=${sha256(fixtureBytes)}`,
  );
}

const negativeValidation = validateEncoding(fixtureBytes, {
  document: false,
  label: "negative encoding fixture",
});
if (!negativeValidation.errors.includes(fixture.expected_error)) {
  failures.push(
    `negative_fixture_did_not_fail_as_expected expected=${fixture.expected_error} actual=${negativeValidation.errors.join(",")}`,
  );
}

const result = {
  status: failures.length === 0 ? "pass" : "fail",
  authoritative_source: {
    path: path.relative(repositoryRoot, sourcePath).replaceAll("\\", "/"),
    utf8_bytes: sourceBytes.length,
    sha256: sha256(sourceBytes),
    nfc: sourceValidation.text === sourceValidation.text?.normalize("NFC"),
    lf_only: sourceValidation.text !== null && !sourceValidation.text.includes("\r"),
    no_bom:
      !sourceBytes.subarray(0, 3).equals(Buffer.from([0xef, 0xbb, 0xbf])),
    no_replacement_character:
      sourceValidation.text !== null && !sourceValidation.text.includes("\uFFFD"),
    no_known_mojibake: !sourceValidation.errors.includes(
      "known_mojibake_pattern",
    ),
    reconstruction_normalized_sha256:
      sourceValidation.text === null
        ? null
        : sha256(
            Buffer.from(
              sourceValidation.text
                .normalize("NFC")
                .replace(/\r\n/g, "\n")
                .replace(/\r/g, "\n"),
              "utf8",
            ),
          ),
  },
  frozen_v3_round_trip: {
    mojibake_passes: frozenV3.mojibakePasses,
    utf8_bytes: frozenV3.byteLength,
    sha256: frozenV3.sha256,
  },
  negative_fixture: {
    path: path.relative(repositoryRoot, fixturePath).replaceAll("\\", "/"),
    expected_error: fixture.expected_error,
    actual_errors: negativeValidation.errors,
  },
  failures,
};

console.log(JSON.stringify(result, null, 2));
if (failures.length > 0) {
  process.exitCode = 1;
}
