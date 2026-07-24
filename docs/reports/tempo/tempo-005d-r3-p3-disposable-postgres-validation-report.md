# TEMPO-005D-R3-P3 Disposable PostgreSQL Validation Report

## 1. Scope

This task was limited to the previously skipped P3 PostgreSQL migration and
atomic-transaction validation matrix. It was not authorized to change runtime
semantics, storage tests, migrations, replay, APIs, DTOs, or private-product
code.

Codex could not access the user's process-local administrative configuration.
The user subsequently ran the guarded matrix from that authenticated PowerShell
session. The run used isolated task-prefixed databases and cleaned them up, but
two P3 assertions failed; the validation gate therefore remains open.

## 2. Controlling Hash Confirmation

The controlling identity-admission specification matched the required hash:

```text
DD28615FB10D80D9D38BC2FB989973788627784A56AEA911472B32E8D42F73B1
```

## 3. Safe Setup Method

The existing P3 storage tests use `IsolatedAdmissionDb::create` in
`backend/crates/storage/src/profile_v0_admission.rs`. The helper requires an
explicit `SEED_TEST_DATABASE_ADMIN_URL` that targets the PostgreSQL `postgres`
maintenance database, creates an isolated database, applies the repository
migration catalog, calls
`common::test_db_guard::require_disposable_database_url` before the application
pool is opened, and drops its temporary database in cleanup.

The shared guard rejects `seed_dev`, `seed_open_core`, `postgres`, and other
non-approved names before a guarded test connection is opened.

## 4. Configuration and Naming Status

No safe administrative configuration is available to the Codex execution
process:

- `SEED_TEST_DATABASE_ADMIN_URL`: not set.
- `DATABASE_URL`: not set.
- `PGHOST`, `PGPORT`, `PGUSER`, `PGDATABASE`, and `PGPASSWORD`: not set.
- A local PostgreSQL 18 service is running and accepts TCP connections at the
  redacted maintenance endpoint `127.0.0.1:5432/postgres`.
- A passwordless, read-only maintenance probe from Codex was rejected before
  authentication (`fe_sendauth: no password supplied`); its `SELECT 1` did not
  execute. No authenticated database session or SQL operation occurred from
  Codex.
- The user subsequently verified an authenticated maintenance probe in their
  own PowerShell process. That process-local administrative URL is deliberately
  not visible to Codex and has not been printed or stored in the repository.

The P3 helper now creates names beginning exactly
`seed_admission_p3_test_`. The shared guard accepts that prefix, and cleanup
refuses a name that does not match it. These are test-only changes; they do not
alter Profile-v0 runtime semantics, storage behavior, or migrations.

## 5. Ownership Handoff

The remaining database execution must be run by the user from the PowerShell
process that already contains `SEED_TEST_DATABASE_ADMIN_URL`. Do not print,
persist, or paste that URL or its password.

Run:

```powershell
Set-Location A:\the-seed-in-my-mind-open-core\backend
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

cargo fmt --all -- --check
cargo check -p storage --all-targets
cargo test -p common --lib
cargo test -p event-log profile_v0_admission --lib
cargo test -p storage profile_v0_ --lib -- --nocapture
```

The final command is the guarded database matrix. It must log a database named
`seed_admission_p3_test_*`, must include a successful
`ISOLATED_DB_CLEANUP: ... dropped=true` line for every created test database,
and must not mention or contact `seed_dev`.

The required administrator configuration remains:

1. It targets only the `postgres` maintenance database on a known disposable
   local/test server, not `seed_dev`, a production database, or an unknown
   application database.
2. Its role can create and drop a newly generated
   `seed_admission_p3_test_<unique-suffix>` database.
3. It may terminate connections only to that task-created database for cleanup.
4. The credentials must be supplied through the environment and must not be
   printed in command output, reports, or logs.

The helper already creates only the required prefix, preserves the shared guard
call, and refuses cleanup of every non-task-created database name.

## 6. Guard Verification

Source inspection confirms the P3 helper constructs a test URL, calls
`require_disposable_database_url`, and only then opens its application pool.
The shared guard accepts `seed_admission_p3_test_`, and the helper generates
only that required prefix. Its cleanup code refuses an unmatched name before
it can issue a drop operation.

No guarded application connection was made in this run. The only PostgreSQL
probe targeted `postgres`, was read-only, and was rejected before
authentication; it did not connect to `seed_dev` or execute SQL.

## 7. Migration and Test Results

Migration 0023 was inspected and the P3 test matrix was identified, including
atomic accepted admission, rollback, retry/no-double-debit, duplicate identity,
historical key reuse, root collision, sponsor-key state, stale authorization,
eligibility, suspension, capacity, compatibility rejection, and account-path
quarantine cases.

User-owned guarded execution results:

- `cargo fmt --all -- --check`: passed.
- `cargo check -p storage --all-targets`: passed.
- `cargo test -p common --lib`: 4 passed, 0 failed.
- `cargo test -p event-log profile_v0_admission --lib`: 8 passed, 0 failed.
- `cargo test -p storage profile_v0_ --lib -- --nocapture`: 8 passed, 2 failed,
  7 filtered out.
- Every created database used the required `seed_admission_p3_test_` prefix and
  reported `differs_from_seed_dev=true`.
- Every visible cleanup line ended with `dropped=true`. Fourteen databases were
  created, but only twelve matching cleanup lines are visible in the pasted
  concurrent-test output. The two missing cleanup confirmations must be
  resolved or independently checked before cleanup can be considered proven for
  every created database.
- No `SKIP: SEED_TEST_DATABASE_ADMIN_URL is missing` output appeared.

The two failures were:

1. `profile_v0_identity_create_is_atomic_and_idempotent_in_an_isolated_database`
   expected `identities_s0` to contain 2 rows but observed 3. The test should
   establish its baseline before application and assert a one-row admission
   increment, rather than assume an absolute migration fixture count.
2. `profile_v0_invalid_applicant_proof_makes_no_admission_writes` expected
   `invalid_applicant_possession_proof` but received
   `applicant_proof_binding_mismatch` after replacing only the proof bytes. A
   corrupted proof without any changed applicant-bound field must be classified
   as an invalid proof; a binding mismatch is reserved for changed bound fields.

The first issue is a test-fixture expectation defect. The second requires a
focused review and correction of the pure validation error mapping or its
storage-layer integration before the matrix can pass.

## 8. Protected-Database Exclusion and Cleanup

`seed_dev` was not used, contacted, migrated, inspected through SQL, or named
as an administrative target. Every created database was visibly task-prefixed
and reported `differs_from_seed_dev=true`.

Every visible temporary-database cleanup ended with `dropped=true`; no cleanup
failure was reported. However, the pasted output lacks matching cleanup lines
for two created task-prefixed databases, so universal cleanup confirmation is
still incomplete.

## 9. Follow-Up and Final P3 Validation Status

P3 source-level validation remains complete, but P3 is **not integration-
validated**. The guarded PostgreSQL migration and atomic-transaction matrix
ran and failed two assertions, so it remains an outstanding release gate.

The smallest safe follow-up is a narrowly scoped P3 corrective task that fixes
the baseline-relative transaction assertion and validates the applicant-proof
error classification against the R2B precedence rules. It must also establish
the cleanup status of the two database names lacking visible cleanup evidence,
then rerun the same guarded matrix from an explicitly disposable PostgreSQL
session. No DEC-043 authority or runtime-file claim may be released before that
rerun has zero failures and a `dropped=true` confirmation for every created
database.

## 10. Focused Corrective Update

The two failed assertions were corrected without changing Profile-v0 admission
semantics, migrations, or state effects:

1. The atomic/idempotent admission test now captures the `identities_s0`
   fixture baseline after sponsor setup and asserts exactly one additional
   identity after acceptance. This verifies the atomic admission effect without
   assuming that unrelated migration fixtures leave an empty table.
2. The corrupted-proof test now expects
   `applicant_proof_binding_mismatch` for an otherwise well-formed 64-byte
   proof that fails against the reconstructed applicant-bound message. This is
   the established P2 exact-byte boundary. Malformed proof encoding or length
   remains `invalid_applicant_possession_proof` at payload parsing.
3. Every assertion inside an isolated database test body now returns a
   fallible test result. The outer test calls `db.cleanup().await` before its
   final `expect`, so a failed assertion-equivalent result cannot bypass the
   required `ISOLATED_DB_CLEANUP` line.

Local database-free validation passed:

- `cargo fmt --all`;
- `cargo test -p common --lib`: 4 passed;
- `cargo test -p event-log profile_v0_admission --lib`: 8 passed;
- `cargo test -p storage profile_v0_ --lib -- --nocapture`: 10 passed in the
  local no-admin environment, with the seven guarded PostgreSQL cases skipped;
- `cargo check -p storage --all-targets`: passed.

The authenticated guarded rerun remains required. Its acceptance evidence is
zero storage-test failures, no missing-admin skip, the exact task prefix for
each created database, and a matching `dropped=true` cleanup line for every
one.

## 11. Final Authenticated Guarded Rerun

The user ran the corrected guarded storage matrix from the PowerShell process
that held the explicit administrative configuration. The redacted evidence
shows:

- `cargo test -p storage profile_v0_ --lib -- --nocapture`: 10 passed, 0
  failed, 7 filtered out;
- fourteen `ISOLATED_DB` lines, each using the exact
  `seed_admission_p3_test_` prefix and `differs_from_seed_dev=true`;
- fourteen matching `ISOLATED_DB_CLEANUP` occurrences, each with `dropped=true`.
  Two messages were interleaved with concurrent test-status output, so the
  final check matched each exact task database name rather than relying on
  line formatting;
- no `SKIP: SEED_TEST_DATABASE_ADMIN_URL is missing` line;
- no observed `seed_dev` operation or protected-database target.

The P3 PostgreSQL migration and atomic-storage-transition matrix is therefore
integration-validated. This closes TEMPO-005D-R3-P3-DBV and releases its narrow
DEC-043 test-matrix and runtime-file claim. It does not claim completion of
the broader Profile-v0 runtime, replay, API/DTO, cycle, or capacity work.
