---
doc_id: open_core_architecture_overview
title: Open-Core Architecture Overview
status: derived
version: v0
last_reviewed: 2026-03-05
scope:
  - Explains the current public open-core architecture for grant reviewers and contributors.
  - Clarifies the open-core versus proprietary boundary at a system level.
authoritative_for:
  - Reviewer-facing architecture summary only.
not_authoritative_for:
  - Protocol semantics
  - Canonical encoding rules
  - Governance, token, or consensus semantics
depends_on:
  - open-core-split-and-data-boundary-spec.md
  - open-core-boundary-manifest.md
  - api-contract-read-only.md
  - stage0-runtime-configuration.md
---

# Open-Core Architecture Overview

## 1. Purpose

This document explains the public open-core package in the simplest accurate terms:

- what exists today,
- what is published as public architecture but not yet fully implemented,
- what is intentionally public,
- what remains proprietary or downstream,
- what the current runtime proves.

## 2. Current architecture

The current public open core is a deterministic canonical-state substrate.

It is composed of:
- canonical event ingestion tooling,
- deterministic replay over the append-only event log,
- snapshot generation and verification,
- a read-only API server,
- a minimal reference frontend for viewing verified state,
- export and boundary verification tooling,
- curated protocol and conformance documentation.

Some additional protocol/spec documents are also published in this repo for architectural transparency. Those documents describe intended open-core behavior beyond the currently runnable Stage 0 runtime. For the implemented-versus-spec-only split, use `open-core-implementation-status.md`.

## 3. System flow

```text
seed/demo data
    |
    v
seed-importer
    |
    v
canonical events + materialized tables
    |
    v
deterministic replay
    |
    +----> snapshot-builder ----> snapshot artifacts / commitments
    |                                   |
    |                                   v
    |                             snapshot-verify
    |
    v
read-only api-server
    |
    v
open-core reference viewer
```

## 4. What is open core

The open-core package includes the infrastructure needed to inspect and verify shared canonical state:

- backend crates for canonical data handling,
- replay/snapshot binaries,
- the read-only public API surface,
- the reference frontend,
- boundary checks,
- canonical DTO drift checks,
- export and smoke verification scripts,
- the curated docs required to understand and run the package.

Within this public repo, that splits into:
- implemented runtime surfaces that can be run and verified now,
- public spec documents that describe broader intended open-core architecture,
- reviewer docs that explain the current package and its boundaries.

## 5. What is not in the public export

The public export intentionally excludes:

- local environment files,
- runtime logs and caches,
- build outputs,
- internal planning and scratch material,
- product/game-specific private modules,
- private DTO crates and private-overlay frontend code.

Those excluded areas belong to the downstream private/product layer, not to the public open-core reviewer package.

The export boundary is enforced by the export manifest and automated verification scripts.

## 6. What the current runtime is and is not

The current public runtime is:
- deterministic,
- replayable,
- snapshot-verifiable,
- inspectable through a public read-only API,
- suitable for reviewer evaluation and open-core publication.

It is not yet:
- a live decentralized multi-node network,
- the complete governance runtime,
- the complete token/economic runtime,
- the full game/product layer,
- a finished end-user application.

Public future/spec-only documents in this repo should be read as architectural transparency, not as evidence that the above unimplemented runtime surfaces already exist.

## 7. Why the reference frontend exists

The reference frontend is intentionally narrow.

Its job is to prove that:
- verified snapshots can be served over a stable API,
- canonical ideas can be browsed without private code,
- a downstream interface can read the same shared state as any conformant client.

It is not intended to represent the full planned product UX.
