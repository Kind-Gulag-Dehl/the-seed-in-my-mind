# Seed V4 Identity-Admission Source-Integrity Report

Task: `OPENCORE-IDENTITY-SOURCE-INTEGRITY-001`

Registered parent task: `INTEGRATION-SEED-V4-PILOT-001`

Repository role: authoritative public/open-core

Baseline: `6068072160fc032eb1ec3b7641cb917c38f08776`

## 1. Scope and authority

This run is encoding-only and meaning-preserving. It repairs
`docs/identity-admission-and-invitation-capacity-spec-v0.md` without changing protocol
semantics, document classification, headings, words, whitespace other than one
nonconforming carriage return, or the historical frozen V3 evidence.

Controlling rules:

- `docs/authoritative-index.md` classifies the target as the scoped Profile-v0
  identity-admission authority published for transparency.
- `docs/canonical-encoding-and-hashing-spec.md` requires valid UTF-8, NFC, LF-only
  text, no BOM, and no silent reinterpretation of ambiguous invalid data.
- The owner ruled that the current mojibake is to be repaired while historical V3
  bytes and hashes remain evidence.

## 2. Pre-repair byte identity and frozen V3 evidence

The exact pre-repair worktree file was captured before editing:

| Evidence | Bytes | SHA-256 | Notes |
| --- | ---: | --- | --- |
| Current worktree source | 785,726 | `dd28615fb10d80d9d38bc2fb989973788627784a56aea911472b32e8d42f73b1` | Valid UTF-8; NFC; one CR; no BOM; no replacement character |
| Worktree source with its single CR removed | 785,725 | `914effee3392c8ad3e233cc703b9aae82fcb6bd00d02babd4c62532a76a20b01` | Byte-identical to the frozen V3 blob |
| Frozen V3 pinned Git blob | 785,725 | `914effee3392c8ad3e233cc703b9aae82fcb6bd00d02babd4c62532a76a20b01` | Blob `7db3f4f75e3ffbad12259f2874e9984675ab4512` at commit `09f18fddd982651be10551c9dc4da916fbb85d54` |

The one CR was at raw byte offset 764,427, in the line ending after
`distinguish Anthill topology from verification;`. Removing it, and only it, made the
worktree bytes exactly equal the frozen V3 blob.

The private frozen V3 reconstruction remains unmodified at
`docs/planning/generated/seed-launch-v3-candidate/seed-document-reconstruction.v3.json`.
It records source ID `manifest-v3-addition:002`, 847 sections, 870 chunks,
byte-for-byte proof status `pass`, the same pinned commit/blob, and the same
`914eff...b01` raw, normalized, and reconstructed SHA-256.

Earlier Open Core evidence also explicitly recorded that the target retained
pre-existing mojibake and that the then-controlling worktree hash was
`dd28615f...f73b1`; see
`docs/reports/tempo/tempo-005d-s2-r2e-final-internal-consistency-report.md`.

## 3. Exact corruption inventory captured before repair

The file contained 168 corrupted punctuation locations. Every non-ASCII code point in
the final recovered text belongs to the following closed inventory:

| Intended code point | Count | Historical eight-pass token UTF-8 bytes | Token SHA-256 |
| --- | ---: | ---: | --- |
| `U+2019` RIGHT SINGLE QUOTATION MARK | 110 | 2,026 | `76bfcf29980ce099158dd7d92a99947b6f861daf16f213e4bb4d5f679e940927` |
| `U+2192` RIGHTWARDS ARROW | 28 | 2,156 | `ff9ff2883679f14f9f6f8edef75ed92c3b40d85230211376758ce8aff6e714cb` |
| `U+201C` LEFT DOUBLE QUOTATION MARK | 9 | 1,898 | `df3e4ac5a013b49caaf1e0f173f0ffe30457cc07df0160106e953108236c09e4` |
| `U+201D` RIGHT DOUBLE QUOTATION MARK | 9 | 1,735 | `6b2cebee0f8b6503b687737c666078e4245210db6249dd332576b307fdb016f0` |
| `U+2014` EM DASH | 12 | 2,026 | `a19d59d0b7b746b0da9795a316857caaf6761a427d6d744f9f3c7da042eeb2f5` |

Locations below are one-based `line:column` positions in the recovered text. Line
structure is unchanged by recovery.

- `U+2019`:
  `26:278`, `28:67`, `96:75`, `97:49`, `100:149`, `104:47`, `299:12`,
  `303:16`, `304:16`, `305:16`, `306:29`, `307:16`, `308:16`, `309:16`,
  `310:16`, `345:18`, `523:33`, `669:76`, `794:10`, `794:40`, `841:15`,
  `842:15`, `843:15`, `844:15`, `845:15`, `846:15`, `847:15`, `1134:14`,
  `1149:16`, `1150:16`, `1151:16`, `1152:16`, `1153:16`, `1154:16`,
  `1423:131`, `1461:126`, `1540:15`, `1662:99`, `1748:119`, `1788:12`,
  `1848:12`, `1976:41`, `2201:14`, `2328:35`, `2362:8`, `2426:34`,
  `2453:10`, `2572:80`, `2602:12`, `2618:47`, `2636:12`, `2716:12`,
  `2716:109`, `2793:12`, `2839:12`, `2876:14`, `3241:13`, `3322:13`,
  `3546:125`, `3675:13`, `3676:16`, `3678:14`, `3754:112`, `4180:18`,
  `4191:15`, `4356:10`, `4362:46`, `4501:80`, `4515:33`, `4576:25`,
  `4590:68`, `4689:48`, `4837:50`, `4911:27`, `5074:41`, `5177:12`,
  `5221:31`, `5243:63`, `5300:17`, `5347:16`, `5374:48`, `5520:124`,
  `5528:23`, `6044:15`, `6045:17`, `6046:25`, `6058:121`, `6067:40`,
  `6069:39`, `6088:21`, `6245:21`, `6265:10`, `6296:131`, `6323:28`,
  `6402:13`, `6541:13`, `6795:36`, `7442:10`, `7568:14`, `7954:78`,
  `9273:34`, `9273:71`, `9294:65`, `9844:22`, `10423:39`, `10465:18`,
  `10789:12`, `12596:60`, `12733:11`, `12737:45`.
- `U+2192`:
  `270:1`, `271:1`, `272:1`, `273:1`, `274:1`, `275:1`, `276:1`,
  `277:1`, `1580:1`, `1581:1`, `1921:1`, `1922:1`, `1923:1`, `1924:1`,
  `2132:1`, `2141:1`, `2148:1`, `5600:1`, `7542:23`, `7545:23`,
  `8261:1`, `8262:1`, `8263:1`, `12386:1`, `12387:1`, `12388:1`,
  `12389:1`, `12390:1`.
- `U+201C`:
  `2362:39`, `3221:1`, `4847:3`, `4848:3`, `4849:3`, `4850:3`,
  `4851:3`, `5110:26`, `5135:24`.
- `U+201D`:
  `2362:48`, `3221:15`, `4847:45`, `4848:73`, `4849:49`, `4850:87`,
  `4851:58`, `5110:53`, `5135:32`.
- `U+2014`:
  `12138:17`, `12155:17`, `12184:17`, `12224:17`, `12238:17`,
  `12256:17`, `12272:18`, `12286:18`, `12302:18`, `12323:18`,
  `12338:19`, `12354:19`.

## 4. Unambiguous-recovery proof

The entire pre-repair Unicode string was inverted through the Windows-1252 byte map and
strict UTF-8 decoder. Every character at each of eight passes had exactly one
Windows-1252 byte image, every resulting byte stream was valid UTF-8, and reapplying
the forward transform reproduced the input exactly.

| Inverse pass | UTF-8 bytes | SHA-256 |
| ---: | ---: | --- |
| 0 | 785,726 | `dd28615fb10d80d9d38bc2fb989973788627784a56aea911472b32e8d42f73b1` |
| 1 | 598,142 | `ebbdb907d6bb6a56ab86f13ce4132fc572ea8ec1ba7768c033ba47e426e29194` |
| 2 | 513,846 | `81d45577df2332fffb60528fa9ffa9565348c8a8ca5d6fe7b6571983c51daf8a` |
| 3 | 476,068 | `45527102da8166bc9743fe141f20ec1a76844e62d171c5cbb150a23988fff80d` |
| 4 | 459,280 | `0a424e6ec4dcacd29ef09607a5f82d837b5f81cfcb43b332c0f08e3f635f21ac` |
| 5 | 451,876 | `4af051eab657584e4d367d5c7b58f49ee30cbf558c34df7401248e7a636c2bf2` |
| 6 | 448,496 | `c72937262b4733ba256ce4aa35476c8edbadcd0c66027e8bf01e639c179858a8` |
| 7 | 446,815 | `32d9e25c974519e0906759155186f21dcccbe821d7aa369e984c5ec45e4a3e26` |
| 8, before LF normalization | 445,993 | `1c9754fbab0b9bd34a298385fbb6c6c396f5fe4b463109fc1a697e8669b3f876` |

The eighth inverse produces only the five intended punctuation code points in the
closed inventory above. A ninth inverse is impossible because `U+2192` has no
Windows-1252 byte image. This is the mechanical stop condition. There are no competing
replacement choices and no wording inference.

Normalizing the single CR to LF produces the intended 445,992-byte source with expected
SHA-256:

```text
063c7db8f73c79e50db870d7a43392188dc2a42f377211117d31f10301445a0c
```

Forward-applying the eight historical transforms to those repaired LF-only bytes must
reproduce the exact frozen V3 785,725-byte payload and
`914effee...a20b01` hash. The focused validator enforces that round trip.

## 5. Negative fixture and validator

`docs/conformance/identity-admission-source-encoding-negative.v0.json` contains one
exact historical eight-pass corrupted em-dash token as base64. It is explicitly marked
`non_authoritative_negative_fixture`, records its byte length and hash, and is not a
copy of the authority document.

`scripts/identity-source-integrity-harness.mjs` validates the repaired authority's exact
hash and Unicode inventory, strict UTF-8, NFC, LF-only form, no BOM, no replacement
character, no known mojibake patterns, byte-stable reconstruction normalization, and
the frozen V3 forward round trip. It also requires the negative fixture to fail with
`known_mojibake_pattern`.

## 6. Final validation

Completed:

- `node --check scripts/identity-source-integrity-harness.mjs` - pass.
- `node scripts/identity-source-integrity-harness.mjs` - pass:
  - authoritative source bytes: 445,992;
  - authoritative source SHA-256:
    `063c7db8f73c79e50db870d7a43392188dc2a42f377211117d31f10301445a0c`;
  - strict UTF-8: pass;
  - Unicode NFC: pass;
  - LF-only and final LF: pass;
  - no BOM: pass;
  - no `U+FFFD`: pass;
  - no known mojibake patterns: pass;
  - reconstruction-normalized bytes equal raw repaired bytes: pass;
  - eight-pass forward round trip to the frozen V3 785,725 bytes and
    `914effee...a20b01` hash: pass;
  - negative fixture rejection with `known_mojibake_pattern`: pass.
- Markdown structure check - 847 headings and 13,953 line objects, unchanged from the
  documented pre-repair source.
- Focused `git diff --check` - pass. Git emitted its existing Windows warning that LF
  may be converted to CRLF if Git later rewrites the worktree file; the current
  authoritative bytes are LF-only, and the focused validator fails closed on any such
  conversion.
- Focused word-diff review - pass; sampled changes are only the inventoried apostrophes,
  arrows, quotation marks, and em dashes.

No build, importer test, database action, source generation, canonical import, signing,
staging, commit, push, or deployment was run.

## 7. Scope and preservation result

- Protocol wording and semantics changed: no.
- Authority classification or freeze ratification changed: no.
- Historical V3 source bytes, hash records, reconstruction artifact, commit, and blob
  changed: no.
- Corrupted material retained in the current authority: no.
- Corrupted material copied into ordinary current documentation: no.
- Corrupted material retained as one explicit non-authoritative negative fixture: yes.
- `backend/bins/seed-importer/**` changed by this sub-run: no. A concurrent modification
  under that path appeared during the run and was preserved without inspection or edit.
- Shared `INTEGRATION-SEED-V4-PILOT-001` active-task entry changed by this sub-run: no;
  it remains active for the concurrent Profile-v0 work.
