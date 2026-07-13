---
doc_id: ai_boundaries_spec
title: AI Boundaries Specification
status: authoritative
version: v0
last_reviewed: 2026-06-22

scope:
  - Defines public AI authority boundaries, sandbox separation, adoption constraints, and conformance requirements.

authoritative_for:
  - AI non-authority invariants and human adoption requirements.
  - Sandbox isolation, governance constraints, and offline compatibility requirements for AI systems.

not_authoritative_for:
  - Product UX flows, model training methods, or deployment strategy.

depends_on:
  - protocol v5.md
  - governance-spec.md
  - safety-spec.md
  - roles-and-stewardship-spec.md
  - offline-and-mindseed-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of protocol v5.md, governance-spec.md, safety-spec.md, and node-and-conformance-spec.md.

reader_path:
  - prereq: roles-and-stewardship-spec.md
  - next: none

keywords:
  - AI
  - sandbox
  - adoption
  - conformance
  - mindseed
---

> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

> **Repository authority note:** In the active two-repo model, this public file is the open-core source of truth for AI boundary rules. Private Ent/product/narrative extensions are subordinate to this document and MUST NOT redefine public AI authority, authorship, governance, token, sandbox, or adoption constraints.

# AI Boundaries Specification

## 0. Purpose and scope

### 0.1 Purpose

This specification defines the public protocol-critical, trust-critical, and conformance-critical boundaries governing artificial intelligence systems within the Protocol v5 ecosystem.

Its purpose is to permit AI-assisted reasoning, drafting, maintenance support, safety analysis, and stewardship-oriented assistance without granting AI systems canonical authority, governance power, economic power, or interpretive finality.

This document preserves the following invariants:

- human-first canonical authorship;
- deterministic replay of the canonical universe;
- perpetual challengeability of canonical claims;
- pluralism without centralized AI arbitration;
- offline and Mindseed portability without AI dependence.

### 0.2 Scope

This specification governs:

- canonical authority and non-authority boundaries for AI systems;
- separation between the canonical universe and AI sandbox environments;
- human adoption requirements for any canonical effect derived from AI output;
- public conformance requirements for nodes and clients that surface AI output;
- governance and safety controls that may constrain AI deployment or availability;
- offline and Mindseed compatibility requirements;
- trust and fairness guarantees related to AI disagreement, local metrics, and non-canonical simulation.

This specification does not define product UX flows, internal heuristics, model training pipelines, internal evaluation systems, deployment architecture, or operational detection tactics beyond the rule-level mitigations required for conformance.

Role semantics remain governed by [roles-and-stewardship-spec.md](roles-and-stewardship-spec.md). This document is role-neutral and defines only technical and procedural constraints on AI systems.

## 1. Canonical authority and non-authority invariants

### 1.1 Human-first canonical authority

Only verified human identities may author, modify, or directly affect canonical state within Protocol v5.

Accordingly, only verified human identities may author canonical events, submit arguments or vote in challenges, activate or deactivate rulebooks, initiate governance actions, mint or influence POD or POINT, or directly mutate canonical rankings, verdicts, or state.

AI systems MUST NOT perform any of these actions, directly or indirectly. This prohibition is absolute and SHALL NOT be bypassed by configuration, delegation, automation, governance action, emergency procedure, or economic incentive.

### 1.2 Advisory-only invariant

All AI output within the ecosystem is advisory.

AI systems MAY analyze canonical state, generate drafts or proposals, simulate outcomes, summarize system behavior, explain rule interactions, and identify risks or inconsistencies. AI systems MUST NOT autonomously create canonical events, vote in or adjudicate challenges, activate or enforce governance, mint or influence tokens, or silently trigger canonical effects.

Any canonical effect derived from AI output requires explicit human adoption recorded as a valid Protocol v5 event. Upon adoption, the adopting human is the sole canonical author and bears full responsibility.

### 1.3 No AI standing

AI systems never acquire standing, legitimacy, authorship, immunity, or protocol-defined rights within the canonical universe.

No model capability, longevity, adoption frequency, tribe affiliation, or governance configuration may elevate an AI system into a canonical participant, representative, or final arbiter.

## 2. Canonical universe vs AI sandbox

### 2.1 Structural separation

The ecosystem maintains two distinct spaces:

- Canonical universe: deterministic, replayable, authoritative, and human-authored.
- AI sandbox: non-canonical, non-deterministic, exploratory, and implementation-defined.

AI systems MAY operate only within the AI sandbox. AI systems MUST NOT execute inside the canonical replay path and MUST NOT introduce any dependency of canonical correctness on sandbox computation.

No node or client behavior MAY require the presence, availability, or correct operation of any AI component in order to validate, replay, or interpret canonical state.

AI systems MAY simulate Tempo, cycle-close, challenge, vote, verdict, beacon, certification, authorization-frontier, governance, POD, POINT, and lifecycle flows only in non-canonical realms. Such simulations are advisory and MUST NOT become canonical inputs by default.

AI systems MUST NOT be canonical Tempo contributors, authors of Tempo claims or Tempo-context evidence ideas, challengers, voters, verdict finalizers, beacon supporters, cycle authorities, governance actors, publication authorities, or token authorities.

AI-generated observations, timestamps, interpretations, local cycle simulations, AI-map time claims, and AI-map beacon states MUST NOT affect canonical Tempo certainty, Dmin/Dmax predicates, cycle certification, the lagged authorization frontier, or consequential authority unless a verified human explicitly turns the material into valid canonical ideas and connections under ordinary Protocol v5 rules. After adoption, the adopting human is the sole canonical author, and the AI-originated material remains only provenance.

AI-map time claims, AI-map attestations, and AI-map beacons are realm-local annotations only. They are not authored canonical ideas or objects, do not satisfy beacon diversity, and do not grant Tempo mana, POD, POINT, governance, lifecycle, rank, or ordinary write authority.

### 2.2 Sandbox artifact boundary

Sandbox artifacts MAY include drafts, analyses, simulations, summaries, safety explanations, or other advisory material.

Sandbox artifacts:

- are not protocol objects;
- have no canonical identifiers or canonical standing;
- MUST NOT be referenced by canonical hashes or canonical dependencies;
- MUST remain confined to client-local or node-local sandbox contexts unless shared as optional non-canonical payloads;
- MUST be clearly labeled as non-authoritative whenever surfaced or shared.

Nodes and clients MUST remain conformant with the sandbox entirely disabled.

### 2.3 Advisory surfaces and automation boundary

Implementations MAY provide helper models, staging surfaces, or other sandbox mechanisms for human review and possible adoption.

If AI output is surfaced for possible adoption, conformant implementations MUST:

- clearly mark it as AI-generated;
- preserve attribution to the producing model identity and relevant lineage or version state;
- require explicit, affirmative human confirmation for any adoption;
- prevent silent, default, implied, or background publication into canon.

Automation or autopilot features, if offered, remain sandbox-only. They MUST NOT create canonical events, vote in challenges, activate or deactivate rulebooks, perform governance actions, or affect POD or POINT. They MUST remain interruptible by the associated human and MUST NOT operate invisibly.

## 3. Human adoption boundary

### 3.1 Adoption mechanics

AI output MAY affect the canonical universe only when a verified human explicitly adopts that output and records the adoption as a valid Protocol v5 event that passes all canonical validation rules.

Adoption MUST be explicit, intentional, and attributable. No AI output MAY become canonical by default, implication, background process, inferred consent, or implementation convenience.

Absent explicit human adoption recorded as a canonical event, AI output has no canonical effect and MUST be ignored during canonical validation and replay.

### 3.2 Accountability

Upon adoption of AI-generated content, the adopting human becomes the sole canonical author of the resulting event and bears full responsibility for its content and consequences.

Errors or harms arising from adopted AI output are treated under the same challenge, sanction, and correction mechanisms that apply to purely human-authored content.

## 4. Pluralism and disagreement handling

### 4.1 Pluralism by design

The ecosystem explicitly supports multiple AI systems operating concurrently, including widely replicated models, node-local models, and tribe-hosted models specialized for particular domains.

No AI system is authoritative, definitive, or privileged by default. The protocol SHALL NOT designate or permit any single AI system to function as a final arbiter, canonical interpreter, or centralized epistemic authority.

### 4.2 Disagreement handling

Disagreement between AI systems is expected and informative.

AI disagreement MUST NOT be resolved through hidden arbitration, implicit weighting, majority voting among models, or automatic collapse into a single synthesized position without explicit human involvement.

AI systems MAY surface disagreements, contrast competing interpretations, and explain differing assumptions, but they MUST NOT adjudicate or finalize the resolution. Resolution occurs only through human reasoning and canonical challenge mechanisms.

## 5. Sandbox debate, simulation, and local metrics

### 5.1 Sandbox debate and simulation

The AI sandbox MAY support non-canonical debate, simulation, comparative reasoning, and exploratory metrics.

All such activity remains strictly non-canonical. Sandbox debate and simulation outputs MUST NOT be treated as deliberative outcomes, verdicts, canonical rankings, or authoritative analyses.

No sandbox metric MAY leak into canonical state, affect deterministic replay, or influence canonical eligibility pools, rankings, or token flows except through explicit human adoption of canonical content.

### 5.2 Local AI reputations and metrics

Nodes MAY maintain sandbox-local metrics evaluating AI usefulness or performance for implementation-specific purposes.

Such metrics are advisory, non-authoritative, and strictly local. They MUST NOT be transferred, aggregated, federated, synchronized, or framed as substitutes for protocol-defined measures such as POD, POINT, challenge outcomes, canonical importance, or governance legitimacy.

Conformant implementations MUST ensure that local AI metrics cannot leak into canonical state, be misinterpreted as protocol data, or be relied upon by canonical workflows.

## 6. Governance interaction and safety controls

### 6.1 Governance boundaries

Governance MAY regulate which AI models or AI-enabled integrations may be equipped by nodes or clients, where such models may operate, and which sandbox capabilities are permitted in a given context.

Governance actions apply to deployment control, availability, and risk management only. Governance MUST NOT grant AI systems authority, standing, voting rights, representative status, or economic influence.

Governance actions affecting AI deployment MUST NOT retroactively alter canonical state, invalidate prior human-authored events, or modify historical challenge outcomes.

### 6.2 Safety and trust requirements

Conformant implementations MUST apply safeguards sufficient to preserve human-first authority and prevent AI systems from silently influencing canonical state.

At minimum, public rule-level mitigations include:

- explicit labeling of AI-generated output;
- visible display of model identity and relevant lineage or version state wherever AI output is surfaced for possible adoption;
- strict isolation of sandbox data and computation from canonical replay and validation;
- explicit human confirmation for every canonical adoption of AI-generated content;
- the ability to suspend, quarantine, restrict, or revoke unsafe or non-conformant AI integrations through governance-defined processes.

Failure to implement required safeguards constitutes non-conformance with this specification.

## 7. Conformance requirements

### 7.1 Node requirements

A conformant node MUST:

- reject any canonical event not authored by a verified human identity;
- ignore sandbox data entirely during canonical validation and deterministic replay;
- enforce the human adoption boundary so that no AI output can affect canonical state without an explicit valid Protocol v5 event;
- remain fully conformant with AI functionality disabled.

### 7.2 Client requirements

A conformant client MUST:

- clearly and continuously distinguish AI-generated output from human-authored content;
- preserve model attribution wherever AI output is surfaced for possible adoption;
- prevent silent or implicit publication of AI output into canonical workflows;
- require explicit human action for any adoption;
- keep sandbox content visibly distinct from canonical content;
- expose sufficient explanation when safety, visibility, or governance decisions affect AI output or availability.

## 8. Offline and Mindseed compatibility

AI systems are optional components of the ecosystem and SHALL NOT be required for offline operation, node conformance, or canonical replay.

Offline and Mindseed operation MUST remain fully functional in the complete absence of AI systems. No canonical process, validation rule, replay mechanism, or interpretation of canonical state may depend on AI computation, availability, or output.

AI systems MAY be included in offline contexts only as optional non-canonical payloads. Such payloads MUST remain advisory, MUST NOT affect deterministic replay, and MUST NOT be required to interpret, validate, or reintegrate offline-generated canonical events.

Any AI-generated content present in offline or Mindseed environments remains subject to the same sandbox isolation, adoption boundaries, and non-authority constraints defined elsewhere in this specification.

## 9. Relationship to other specifications

This specification is subordinate to and depends on the following documents within their respective domains:

- [protocol v5.md](protocol%20v5.md)
- [governance-spec.md](governance-spec.md)
- [token-spec.md](token-spec.md)
- [safety-spec.md](safety-spec.md)
- [node-and-conformance-spec.md](node-and-conformance-spec.md)
- [offline-and-mindseed-spec.md](offline-and-mindseed-spec.md)
- [roles-and-stewardship-spec.md](roles-and-stewardship-spec.md)
- [tribe-spec.md](tribe-spec.md)

In the event of any conflict, Protocol v5 SHALL be authoritative. This specification introduces no independent authority, governance power, or economic semantics and SHALL be interpreted strictly within the broader protocol specification set.
