---
doc_id: open_core_demo_flow
title: Open-Core Demo Flow
status: derived
version: v0
last_reviewed: 2026-08-10
scope:
  - Documents the guarded deterministic reviewer demo path.
authoritative_for:
  - Demo instructions only.
not_authoritative_for:
  - Canonical semantics
depends_on:
  - open-core-implementation-status.md
---

# Open-Core Demo Flow

## 1. Purpose

The demo proves deterministic current-profile Seed ingestion, replay, representation-bearing snapshot construction and independent verification, read-only API serving, and optionally an isolated reference-frontend build. It does not prove the full public specification.

## 2. Safety prerequisite

Set SEED_TEST_DATABASE_ADMIN_URL process-locally to the postgres maintenance database on a disposable PostgreSQL server with create/drop privileges. Do not store or paste the URL.

The script rejects ordinary database targets. It generates only seed_opencore_m1_reviewer_repair_001_* databases, owns only its spawned API process, writes Cargo/frontend/snapshot output below a temporary run directory, and verifies exact database cleanup. It never removes source-tree node_modules, lockfiles, build output, or unrelated processes.

## 3. Commands

From the repository root:

    npm run review:demo

Optional isolated frontend build:

    powershell -ExecutionPolicy Bypass -File scripts/open-core-demo.ps1 -BuildReferenceFrontend

The fuller reviewer path also runs the migration-0025 semantic matrix:

    powershell -ExecutionPolicy Bypass -File scripts/grant-reviewer-quickstart.ps1 -Profile 30

## 4. Evidence

Success prints snapshot height/hash/shared-map commitment, imported ideas, exact task-prefixed database creation, exact-name dropped=true cleanup, preserved admission-database counts, and PASS open-core-demo.