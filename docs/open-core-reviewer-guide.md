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

The longer path adds an isolated reference-frontend build and the guarded 19-case migration-0025 semantic matrix. Both paths create only exact task-prefixed databases and print cleanup evidence.

## 4. Isolated export verification

Choose a new temporary output directory:

    npm run extract:open-core:dir -- C:\Temp\seed-open-core-export

The generator verifies the explicit export root, builds/tests with temporary Cargo/frontend outputs, and runs the exported guarded demo. Do not use the zip option unless release packaging is separately authorized.

## 5. Scope boundary

This package does not claim a live multi-node network, complete governance/token runtime, or the proprietary product/game experience.