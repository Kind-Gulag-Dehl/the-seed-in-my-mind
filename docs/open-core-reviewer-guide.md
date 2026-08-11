---
doc_id: open_core_reviewer_guide
title: Open-Core Reviewer Guide
status: derived
version: v0
last_reviewed: 2026-08-10
scope:
  - Gives reviewers a guarded evaluation path for the public package.
authoritative_for:
  - Reviewer workflow only.
not_authoritative_for:
  - Protocol semantics
depends_on:
  - open-core-demo-flow.md
  - open-core-implementation-status.md
---

# Open-Core Reviewer Guide

## 1. What to evaluate

Evaluate deterministic current-profile ingestion, replay, representation-bearing snapshot verification, read-only serving, conformance/DTO boundaries, and isolated export reproducibility. Use the implementation-status page to distinguish implemented, partial, and specification-only behavior.

## 2. Prerequisites

Windows PowerShell, Rust stable, Node.js 20+, Postgres with psql, and a process-local SEED_TEST_DATABASE_ADMIN_URL targeting the postgres maintenance database on a disposable server. Ordinary DATABASE_URL targets are rejected by reviewer verification.

## 3. Reviewer paths

15-minute path:

    npm run review:quickstart

30-minute path:

    powershell -ExecutionPolicy Bypass -File scripts/grant-reviewer-quickstart.ps1 -Profile 30

The longer path adds an isolated reference-frontend build, the guarded 19-case migration-0025 semantic matrix, and the canonical-history two-database round trip. Both paths create only exact task-prefixed databases and print cleanup evidence.

## 4. Canonical-history transfer review

The package format and excluded data are documented in `docs/canonical-history-transfer.md`. Run the non-database fixture checks with:

This transfer directory is a narrow canonical-history artifact, not a `.seedpkg`, closed SeedPackage profile, or Full Recovery Bundle. Recovery-profile conformance remains specification/static-vector scope.

    npm run verify:canonical-history

With the guarded administrator URL configured, run the full isolated round trip with:

    npm run verify:canonical-history-db

The database verifier proves byte-identical repeat export, validate-only with zero database writes, fresh-target import, exact retry idempotency, source/target package equality, replay/snapshot commitments and counts, fixed-snapshot API equality, and exact cleanup. It preserves the two pre-existing `seed_admission_p3_test_32944_*` databases.

## 5. Isolated export verification

Choose a new temporary output directory:

    npm run extract:open-core:dir -- C:\Temp\seed-open-core-export

The generator verifies the explicit export root, builds/tests with temporary Cargo/frontend outputs, and runs the exported guarded demo. Do not use the zip option unless release packaging is separately authorized.

## 6. Scope boundary

This package does not claim a live multi-node network, complete governance/token runtime, or the proprietary product/game experience.