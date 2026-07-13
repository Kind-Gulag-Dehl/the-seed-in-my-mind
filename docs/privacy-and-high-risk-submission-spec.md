---
doc_id: privacy_and_high_risk_submission_spec
title: Privacy and High-Risk Submission Specification
status: authoritative
version: v0
last_reviewed: 2026-06-22

scope:
  - Defines privacy invariants, high-risk submission compatibility, and operator metadata minimization requirements.

authoritative_for:
  - Canonical-layer privacy constraints and high-risk submission requirements.
  - Operator metadata minimization and conformance-level privacy declarations.

not_authoritative_for:
  - Verification mechanics beyond privacy-related constraints.
  - Authored-candidate signature profiles, signed bytes, or public-key-reference construction.

depends_on:
  - protocol v5.md
  - canonical-event-authorship-and-signature-profile-v0.md
  - node-and-conformance-spec.md
  - verification-spec.md
  - offline-and-mindseed-spec.md
  - tempo-spec.md

conflicts:
  - none known

change_rules:
  - Any change here requires review of protocol v5.md, node-and-conformance-spec.md, and verification-spec.md.

reader_path:
  - prereq: verification-spec.md
  - next: tribe-spec.md

keywords:
  - privacy
  - high-risk
  - submission
  - metadata
  - anonymity
---

> **Status note:** This document is part of the intended open-core architecture. It is published in this public repo for transparency and architectural understanding. Current implementation status is limited, partial, or not yet implemented. [open-core-implementation-status.md](open-core-implementation-status.md) remains the authoritative current-state implementation reference.

# Privacy and High-Risk Submission Specification

## 1. Purpose and Scope

### 1.1 Objective

This specification defines the privacy model of the system, with particular emphasis on users operating in high-risk or authoritarian environments.

The purpose of this document is to:

- Clarify what privacy properties the protocol guarantees.
- Define what privacy properties the protocol does not guarantee.
- Establish normative constraints on canonical artifacts to prevent civil-identity leakage.
- Define submission compatibility requirements for high-risk and dissident use.
- Establish operator-level metadata minimization expectations.
- Harmonize terminology across specifications regarding anonymous, pseudonymous, and verified participation.

This document does not replace the Protocol v5, Verification Spec, Node and Conformance Spec, Governance Spec, or Offline and Mindseed Spec. It supplements them by consolidating and tightening privacy-related requirements.

Where conflicts arise, Protocol v5 constitutional invariants remain supreme.

---

### 1.2 Scope

This specification governs:

1. Canonical-layer privacy invariants.
2. Identity visibility modes.
3. Anonymous outer-layer semantics as they relate to privacy.
4. High-risk submission compatibility requirements.
5. Operator metadata handling expectations.
6. Conformance-level privacy declarations.

This specification applies to:

- Conformant nodes.
- Client implementations.
- Verification providers.
- Operators offering public node services.
- Implementations supporting offline or delayed publication workflows.

This specification does not mandate specific cryptographic primitives, specific networks, or specific hosting infrastructures unless explicitly stated elsewhere in the protocol suite.

---

### 1.3 Non-Goals

The system does not guarantee:

- Protection against physical coercion.
- Protection against device compromise.
- Protection against endpoint surveillance.
- Immunity from correlation using external datasets.
- Absolute network-layer anonymity.
- Legal immunity for participants.

The protocol is designed to minimize unnecessary identity exposure and reduce structural risks. It cannot eliminate all risks inherent in hostile environments.

Implementers and users MUST understand that privacy is a layered property dependent on:

- Device security.
- Operational security practices.
- Transport choices.
- Jurisdictional conditions.

The protocol guarantees structural minimization of civil-identity exposure at the canonical layer. It does not guarantee operational anonymity in all real-world contexts.

---

## 2. Threat Model

### 2.1 Adversary Classes

This specification assumes the following adversary types:

**A. Local Network Observer**  
An actor capable of monitoring traffic at ISP, enterprise, or state level.

**B. Node Operator Under Compulsion**  
A node operator subject to subpoena, coercion, or legal demand.

**C. State-Level Surveillance Actor**  
An adversary capable of:
- Traffic analysis
- Network correlation
- Legal compulsion
- Platform suppression
- Criminalization of participation

**D. Malicious Participant**  
A verified or unverified identity attempting to:
- Doxx participants
- Correlate identities
- Exploit metadata
- Coerce identity exposure through system mechanisms

The protocol MUST be resilient to these adversaries at the canonical design level.

---

### 2.2 High-Risk Environment Assumption

The protocol MUST assume that some participants:

- Operate under heavy surveillance.
- Face criminal penalties for dissent.
- Cannot safely disclose civil identity.
- Cannot safely connect directly to foreign-hosted services.
- May require delayed or indirect publication pathways.

Privacy features MUST be compatible with:

- Traffic monitoring environments.
- Censorship attempts.
- Intermittent connectivity.
- Confiscation risk.

Privacy protections MUST NOT depend on benevolent operators.

---

### 2.3 Survivability Requirement

The system SHALL remain usable for:

- Whistleblowing.
- Dissent reporting.
- Evidence preservation.
- Delayed publication under risk.

The protocol SHALL NOT structurally require:

- Real-name disclosure.
- Centralized submission endpoints.
- Mandatory identity revelation for canonical eligibility.

Privacy protections SHALL focus on:

- Minimizing canonical exposure.
- Preventing identity leakage through protocol artifacts.
- Supporting indirect or delayed submission.
- Reducing metadata retention at operator level.

The system is designed to survive in hostile conditions. It is not designed to guarantee invisibility.

---

## 3. Definitions and Terminology

### 3.1 Identity Types

For the purposes of this specification:

**Canonical Identity**  
A canonical idea representing a real agent in the graph.

**Verified-Human Identity (VH)**  
An identity that has satisfied the human-presence verification requirements defined in the Verification Spec and is eligible for canonical authorship.

**Civil-Identity Linkage (VI)**  
A truth-claim asserting correspondence between a canonical identity and a real-world civil identity. This linkage MAY exist but MUST NOT be required for canonical participation.

**Anonymous Identity**  
An identity whose civil identity is unknown and not claimed within the canonical layer.

**Pseudonymous Identity**  
An identity operating under a stable canonical identifier without public civil identity disclosure.

**Anonymous-but-Verified Identity**  
A verified-human identity that satisfies VH requirements but does not publish or expose civil-identity linkage.

---

### 3.2 System Layers

**Canonical Universe**  
The deterministic, append-only event log and derived state governed by Protocol v5 and subordinate specifications.

**Anonymous Outer Layer**  
A non-canonical publication layer intended for whistleblowing, dissent, experimentation, and pre-canonical content. Content in this layer has no canonical ranking or token effect until adopted by a verified-human identity.

**Offline Pre-Publication State**  
Content created locally and not yet published to a canonical node.

---

### 3.3 Privacy Modes

**Default Pseudonymous Mode**  
The standard operating mode in which canonical authorship is pseudonymous and civil identity is not required.

**Public Real-Name Mode**  
An explicitly opt-in visibility mode in which a participant chooses to associate civil identity claims with their canonical identity.

**High-Risk / Dissident Mode**  
An operational mode in which:
- Civil identity is not disclosed.
- Submission pathways are compatible with indirect or delayed publication.
- Operator metadata exposure is minimized.

High-risk mode affects operational posture. It does not alter canonical invariants.

---

## 4. Canonical Privacy Invariants

### 4.1 Prohibition of Raw Civil Identity Data

Canonical artifacts MUST NOT contain raw civil identity data.

The following data types are explicitly prohibited from inclusion in canonical events, verification artifacts, or canonical payloads:

- Government-issued identification numbers.

- Exact residential addresses.
- Exact geographic coordinates tied to a person.
- Biometric templates or raw biometric captures.
- Device serial numbers or hardware fingerprints.
- Full legal names unless explicitly asserted through a separate VI truth claim.

Where civil identity correspondence exists, it MUST be represented as:

- A truth claim (VI).
- Attestation statements.
- Commitments or non-reversible proofs.
- Abstracted references to evidence.

Raw documents, scans, photos, and direct identifying artifacts MUST remain off-canonical and encrypted at rest if stored at all.

---

### 4.2 Prohibition of Network Identifiers in Canonical Artifacts

Canonical artifacts MUST NOT contain network-layer identifiers.

The following MUST NOT appear in canonical events:

- IP addresses.
- MAC addresses.
- Transport-layer headers.
- Routing paths.
- Submission timestamps at sub-minute precision if they could enable correlation.
- Device identifiers transmitted during submission.

Canonical correctness SHALL depend only on:

- Event content.
- Deterministic validation rules.
- Cryptographic signatures.
- Canonical ordering rules.

Transport-layer details SHALL have no canonical relevance.

---

### 4.3 Coarse Temporal Constraints

Where verification artifacts include time information, timestamps MUST use coarse ranges when finer precision would materially increase identity correlation risk.

Verification attestations SHOULD use:

- Time ranges rather than exact timestamps.
- Broad locality descriptors rather than precise location data.
- Abstracted evidence references rather than event-specific coordinates.

Deterministic replay correctness MUST NOT depend on fine-grained personal timestamps.

Submission timestamps, receipt timestamps, server timestamps, node-local observations, and transport metadata MUST NOT be admitted as Tempo certainty inputs unless a verified human separately represents the material as canonical ideas and connections with inspectable provenance and deterministic replay inputs.

Tempo beacon independence and diversity requirements MUST be satisfiable without exposing civil identity, precise location, demographic category, nationality, institution, jurisdiction, or other identifying metadata as public canonical data.

Tempo independence and diversity checks MUST be derived from privacy-preserving verification or anti-Sybil evidence, or from replayable attestation-graph independence, without exposing civil identity. They act only as eligibility and diversity gates. They MUST NOT become demographic, national, institutional, jurisdictional, wealth, reputation, or civil-identity weighting, and public proofs SHOULD expose only the minimum data needed for replay and conformance.

---

### 4.4 Verified-Human Eligibility Without Civil Disclosure

Verification of human presence (VH) MUST NOT require publication of civil identity.

Canonical eligibility SHALL require:

- Verified-human status.
- Valid Profile-v0 signature binding to a canonical identity under `canonical-event-authorship-and-signature-profile-v0.md`.
- Conformance with verification gate requirements.

Canonical eligibility SHALL NOT require:

- Publication of legal name.
- Publication of civil-identity documents.
- Public VI claims.

Civil-identity linkage (VI) MAY exist as a truth claim. It MUST remain:

- Optional.
- Explicitly asserted.
- Perpetually challengeable.
- Structurally separable from VH.

The system SHALL allow anonymous-but-verified participation as a first-class mode.

---

## 5. Identity Visibility Model

### 5.1 Default Pseudonymous Mode

The default operating mode of the system is pseudonymous.

In this mode:

- Canonical authorship is attributed to a canonical identity.
- Civil identity is not required.
- VH verification is sufficient for canonical eligibility.
- No implicit disclosure of real-world identity occurs.

Client interfaces MUST NOT default to publishing civil-identity linkage.

Node operators MUST NOT require civil identity for canonical participation.

---

### 5.2 Public Real-Name Mode (Opt-In)

Public real-name association is explicitly opt-in.

When a participant chooses to assert civil-identity linkage:

- The linkage MUST be represented as a VI truth claim.
- Evidence MUST follow canonical PII minimization rules.
- The system MUST treat the claim as challengeable.

Real-name mode MUST:

- Be reversible by retracting or challenging the VI claim.
- Not implicitly reveal historical hidden identity.
- Not auto-propagate to all contexts without explicit scope.

User interfaces MUST require explicit consent before:

- Publishing civil-identity linkage.
- Displaying civil-identity claims prominently.
- Associating legal identity with prior pseudonymous content.

---

### 5.3 Anonymous-but-Verified Mode

Anonymous-but-verified mode is a valid and supported participation state.

In this mode:

- The participant satisfies VH requirements.
- No civil-identity linkage is published.
- Canonical authorship remains pseudonymous.
- Governance eligibility remains intact.

The system SHALL treat anonymous-but-verified participants as fully legitimate canonical authors.

No client or node MAY downgrade, restrict, or de-prioritize participation solely due to absence of VI.

---

## 6. Anonymous Outer-Layer Semantics

### 6.1 Non-Canonical Status

Content created in the anonymous outer layer:

- Is not part of the canonical universe.
- Has no POD, POINT, ranking, or governance effect.
- Is not considered canonical authorship.

Outer-layer content MAY be:

- Anonymous.
- Pseudonymous.
- AI-generated.
- Offline-originated.

Outer-layer content SHALL remain non-canonical until explicitly adopted by a verified-human identity.

---

### 6.2 Adoption into Canonical Universe

Adoption into the canonical universe requires:

- A verified-human identity.
- A canonical event explicitly adopting the content.
- A valid Profile-v0 signature from the adopting identity.

The adopting identity becomes accountable for the canonical representation of the adopted content.

Adoption SHALL NOT:

- Automatically reveal the original source.
- Embed submission metadata from the outer layer.
- Include transport-layer identifiers.

The source of outer-layer content MAY remain undisclosed.

---

### 6.3 Adoption Provenance Constraints

When outer-layer content is adopted:

- The canonical event MUST include only the adopted payload.
- The adoption event MUST bind the adopting identity to the content.
- The adoption event MUST NOT include:
  - Original IP information.
  - Submission path metadata.
  - Device fingerprints.
  - Correlating transport-layer artifacts.

If the original author wishes to be revealed, that reveal MUST occur through a separate explicit identity or VI claim.

Outer-layer anonymity SHALL be preserved by default.

---

## 7. High-Risk Submission Compatibility Profiles

### 7.1 Transport Neutrality

The protocol SHALL remain transport-neutral.

Canonical correctness MUST NOT depend on:

- A specific network.
- A specific domain name.
- A specific hosting provider.
- A centralized submission endpoint.
- A proprietary transport mechanism.

Conformant nodes MAY use any networking protocol consistent with deterministic validation and canonical replay requirements.

Transport choice SHALL NOT affect:

- Event validity.
- Canonical ordering.
- Verification semantics.
- Governance eligibility.

---

### 7.2 Confidential Transport Profile

Conformant nodes MUST support at least one submission pathway that provides confidentiality of payload contents during transit.

This profile SHALL ensure:

- Payload encryption between client and receiving node.
- Protection against passive inspection by intermediaries.
- Integrity validation prior to canonical processing.

This specification does not mandate:

- A specific cryptographic primitive.
- A specific network (e.g., Tor, I2P).
- A specific protocol stack.

However, conformant implementations MUST document:

- Which confidential transport profiles are supported.
- Whether encryption is mandatory or optional for submission.
- Whether plaintext submission is permitted.

Canonical artifacts MUST remain independent of transport-level encryption details.

---

### 7.3 Anonymity-Compatible Submission Profile

Conformant nodes MUST be compatible with indirect and privacy-preserving routing.

This means nodes SHALL:

- Accept submissions routed through intermediary relays.
- Accept delayed or store-and-forward delivery.
- Avoid embedding submission-path metadata into canonical artifacts.
- Avoid rejecting submissions solely because the transport path is indirect.

Nodes SHALL NOT require:

- Direct connection from the originating device.
- Stable source identifiers.
- Persistent client authentication tied to network identity.

Nodes MAY publish optional submission endpoints.  
Nodes SHALL NOT require exclusive use of those endpoints for conformance.

---

### 7.4 Store-and-Forward / Offline Publication Profile

Conformant nodes MUST support deterministic validation of events independent of:

- Real-time submission.
- Immediate connectivity.
- Continuous online presence.

The system SHALL allow:

- Human-carried synchronization.
- Delayed publication of locally created events.
- Publication via intermediary nodes.

Submission time SHALL NOT affect:

- Event validity.
- Canonical replay.
- Author eligibility (subject to verification state at inclusion time).
- Tempo certainty, cycle certification, authorization-frontier advancement, POD, POINT, or mana spendability.

Nodes SHALL treat offline-originated events equivalently to network-originated events, provided `canonical-event-authorship-and-signature-profile-v0.md` signature rules and all other validation rules are satisfied.

---

## 8. Operator Metadata Minimization

### 8.1 Canonical vs Operational Data Separation

Canonical data is:

- Deterministic.
- Publicly replayable.
- Subject to protocol-level invariants.

Operational data is:

- Local to the node.
- Not part of the canonical universe.
- Not required for deterministic replay.

Operators MUST maintain strict separation between canonical artifacts and operational metadata.

Operational metadata SHALL NOT influence canonical state.

---

### 8.2 Prohibited Default Logging

Conformant nodes MUST NOT, by default, retain the following metadata beyond what is required for immediate validation and rate-limiting:

- IP addresses.
- Stable device fingerprints.
- Full HTTP request headers.
- Transport-layer routing details.
- Persistent correlation identifiers.

If temporary retention is required for operational security or abuse mitigation:

- Retention windows MUST be explicitly bounded.
- Metadata MUST NOT be merged into canonical artifacts.
- Metadata MUST NOT be publicly exposed.

Default configurations SHOULD favor minimal retention.

---

### 8.3 Retention Policy Disclosure

Conformant nodes offering public services MUST publish:

- A metadata retention policy.
- Retention time windows.
- Supported privacy profiles.
### 9.2 No Mandatory Centralized Endpoint

Nodes MAY offer multiple privacy modes.  
Nodes SHALL clearly declare which mode is active.

Retention policy SHALL NOT affect canonical validity.

---

### 8.4 Compulsion Resilience

Node implementations SHOULD be designed such that:

- Compelled disclosure reveals minimal correlating metadata.
- Canonical logs alone do not expose civil identity.
- Transport metadata is ephemeral and separable from canonical state.

This specification does not guarantee resistance to legal compulsion.  
It mandates architectural minimization of unnecessary retained identity data.

---

## 9. Conformance and Privacy Declarations

### 9.1 Privacy Capability Declaration

Conformant nodes MUST declare:

- Supported transport confidentiality profiles.
- Support for indirect/anonymity-compatible routing.
- Metadata retention policy.
- Whether minimal logging mode is enabled by default.

This declaration SHALL be:

- Publicly accessible.
- Machine-readable where feasible.
- Non-authoritative with respect to canonical validation.

Privacy declarations SHALL NOT create governance authority.

---

### 9.2 No Mandatory Centralized Endpoint

Conformance MUST NOT require:

- Submission through a specific domain.
- Submission through a specific operator.
- Registration with a centralized authority.
- Mandatory routing through any designated node.

The protocol SHALL remain interoperable across multiple independently operated nodes.

Privacy SHALL NOT depend on trust in a single operator.

---

### 9.3 Accessibility Constraint

Privacy requirements SHALL NOT impose:

- Specialized hardware requirements beyond canonical validation.
- Excessive bandwidth requirements.
- Prohibitive computational overhead.
- Mandatory dependence on specific anonymity networks.

Conformance SHALL remain accessible to an average technically competent individual capable of running a node under existing node-and-conformance requirements.

Privacy enhancements MUST be compatible with decentralization and broad participation.

---

## 10. Interaction with Verification and Governance

### 10.1 Verification Without Civil Exposure

Verification SHALL remain structurally compatible with privacy-preserving participation.

The Verification Spec defines two separable tracks:

- VH (verified-human presence).
- VI (civil-identity correspondence).

This privacy specification clarifies that:

- VH MUST be sufficient for canonical authorship eligibility.
- VI MUST remain optional.
- Absence of VI MUST NOT reduce canonical rights or eligibility.

Verification artifacts MUST:

- Avoid inclusion of raw civil identity data.
- Avoid inclusion of fine-grained location data.
- Avoid inclusion of device identifiers.
- Use attestation-based structures rather than document publication.

The system SHALL treat:

- Anonymous-but-verified identities,
- Pseudonymous verified identities, and
- Public real-name verified identities

as structurally equivalent for purposes of authorship, governance participation, and challenge processes.

No governance mechanism MAY require civil-identity publication as a prerequisite for participation.

---

### 10.2 Verification Claims and Challengeability

All VH and VI claims remain perpetually challengeable under the challenge primitive.

Privacy protections SHALL NOT:

- Shield fraudulent verification claims from challenge.
- Prevent investigation of fabricated attestations.
- Prevent governance review of compromised verification lanes.

At the same time:

- Challenges MUST operate on canonical artifacts.
- Challenges MUST NOT rely on publication of raw PII.
- Challenges MUST follow existing safety and anti-doxxing constraints.

Verification certainty is determined deterministically through canonical evidence ideas and challenge outcomes.
Civil identity disclosure is not a substitute for proper challenge resolution.

---

### 10.3 Wrongful Deanonymization and Identity Leakage

Wrongful deanonymization is defined as:

- Publishing civil identity without consent.
- Publishing correlating information that materially increases re-identification risk.
- Embedding identifying metadata in canonical artifacts.

Wrongful deanonymization MAY constitute:

- A representation challenge.
- A safety violation.
- A governance-relevant event.

The system SHOULD provide:

- A formal challenge pathway for identity leakage claims.
- Safety review mechanisms where doxxing risk is alleged.
- Governance-level adjudication where appropriate.

Privacy protections SHALL NOT prevent accountability.  
Accountability SHALL NOT require civil-identity publication.

---

### 10.4 Governance Independence from Civil Identity

Governance processes SHALL operate under the invariant:

One verified human identity equals one eligible governance participant (subject to eligibility rules defined elsewhere).

Governance SHALL NOT:

- Weight votes by civil identity status.
- Grant authority based on public identity.
- Require public real-name association.

Governance legitimacy derives from:

- Verified human status.
- Deterministic replay.
- Challengeable claims.

Civil identity is orthogonal to governance authority.

---

## 11. Limitations and Explicit Non-Guarantees

### 11.1 No Absolute Anonymity Guarantee

This protocol does not guarantee:

- Network-layer anonymity.
- Protection from advanced traffic correlation.
- Protection from global adversary analysis.
- Immunity from real-world identification through external datasets.

Privacy protections operate at the canonical and architectural level.  
Operational anonymity depends on user behavior and environment.

---

### 11.2 Endpoint Security Is Out of Scope

The protocol does not protect against:

- Malware.
- Device seizure.
- Compromised operating systems.
- Physical surveillance.

Users operating in high-risk environments MUST employ appropriate operational security practices beyond the scope of this protocol.

---

### 11.3 Coercion and Legal Risk

The protocol cannot prevent:

- Legal coercion of node operators.
- Legal coercion of participants.
- State-imposed penalties for participation.

This specification reduces structural exposure.  
It does not eliminate jurisdictional risk.

---

### 11.4 Correlation Through External Signals

Even if canonical artifacts contain no PII, identity MAY still be inferred through:

- Writing style.
- Posting patterns.
- External social media correlation.
- Independent intelligence gathering.

The protocol does not guarantee resistance to such correlation.

---

### 11.5 No Mandatory Anonymity Networks

The protocol does not mandate:

- Specific anonymity networks.
- Specific privacy routing systems.
- Specific encryption stacks.

Nodes may support such systems, but conformance does not depend on them.

---

### 11.6 Architectural Guarantee Boundary

The protocol guarantees:

- No raw civil identity in canonical artifacts.
- No network identifiers in canonical events.
- No mandatory civil-identity disclosure for participation.
- Deterministic replay independent of transport.

The protocol does not guarantee:

- Invisibility.
- Immunity.
- Absolute untraceability.

Privacy in this system is structural minimization, not magic.

---
