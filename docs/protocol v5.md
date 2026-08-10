---
doc_id: protocol_v5
title: Protocol v5
status: authoritative
version: v5
last_reviewed: 2026-01-27

scope:
  - Root invariants and core canonical semantics for the Seed system.

authoritative_for:
  - Canonical object meanings (ideas, connections, challenges, identities, actions).
  - Constitutional invariants (the §0 principles) and non-negotiable constraints.

not_authoritative_for:
  - Byte-level encoding and hashing details (see canonical-encoding-and-hashing-spec.md).
  - Deterministic replay/merge algorithm details (see deterministic-replay-and-merge-spec.md).

depends_on:
  - none

conflicts:
  - none known

change_rules:
  - If core semantics change, review all authoritative specs that depend on protocol semantics.
  - If a spec contradicts §0 invariants, the spec must be corrected to conform.

reader_path:
  - prereq: none
  - next: canonical-encoding-and-hashing-spec.md

keywords:
  - protocol
  - invariants
  - canonical semantics
  - ideas
  - challenges
  - identities
---

# The Seed In My Mind Protocol

Humanity needs a standardized, decentralized system and structure for collective ideation, reasoning, and truth determination—one that allows all humans, and any future reasoning agents, to better understand what all others believe is most important, what is true, and what to do about it. The purpose of this protocol is to define such a system and grow a shared map of reality through time.

This document specifies the foundations of a canonical universe constructed from ideas, challenges, arguments, importance mappings, actions, and their consequences. It describes the event types, invariants, governance pathways, safety constraints, and interpretability structures that allow participants to collectively build, verify, challenge, and refine their understanding of the world.

The system does not attempt to govern human behavior outside of itself. Instead, it governs the interaction of ideas: how claims are made, how truth is tested, how importance is assessed at personal, tribal, and universal levels, and how insight accumulates across generations.
Its role is to provide a durable, replayable record of reasoning—one that helps humanity face uncertainty together.

All future modifications to this universe MUST remain compatible with the constitutional commitments in §0. These commitments define the philosophical boundary of the system and anchor the technical specifications that follow.



## non-normative section [anchor: non_normative_section]

### A- be excellent to each other [anchor: a_be_excellent_to_each_other]

This system begins with a simple orientation: participants SHOULD treat one another with respect, patience, and a willingness to understand. “Be excellent to each other” is not sentimentality; it is a functional requirement for a deliberative system in which ideas must be compared, challenged, defended, and refined. Excellence toward one another ensures that disagreement does not collapse into hostility, and that the system remains capable of supporting cooperative truth-seeking across diverse worldviews.

### B- no human is my enemy [anchor: b_no_human_is_my_enemy]

No human is my enemy. People sometimes fight each other—physically, politically, or emotionally—when their ideas collide or when fear and circumstance drive them into opposition. These conflicts can be severe, destructive, and tragically irreversible. But even in such moments, the person is not the true adversary. The forces shaping their actions—confusion, suffering, inherited ideas, and the pressures of circumstance—are what we ultimately struggle against.

Disagreement belongs in the realm of ideas, not in the dehumanization of individuals. When people oppose each other, what is truly in conflict are their models of the world, not their fundamental worth. The aim is not to defeat the person but to free them, and ourselves, from the ideas and conditions that lead to harm. Humans clash; minds diverge; some will never agree. But the principle remains: humans are not the enemy—our battles lie elsewhere.

### C- three layers of conflict and the real adversary [anchor: c_three_layers_of_conflict_and_the_real_adversary]

There are three layers of conflict in human life.

First, there is human-against-human conflict: the tragic form that arises when ideas harden into violence. It is sometimes unavoidable, but it is never the level on which meaning is won. This system exists partially to reduce the need for such conflict by relocating battles to safer ground.

Second, there is the contest of ideas: where disagreement, opposition, and critique properly occur. Here, the "enemy" is the claim or worldview being challenged - not the person who holds it. Ideas can and should be fought vigorously on equal ground, without coercion or dehumanization.

Third, there is the cooperative adversarial testing of ideas: a shared recognition that pitting ideas against each other strengthens them. Through structured challenge, weak ideas fall away, strong ones emerge, and better maps of reality become possible. This is the arena in which we prepare ourselves for the conflict that truly matters.

The real enemy is not other humans and not even individual ideas, but the strange and unforgiving circumstances we all inherit: the unknown, the limits of knowledge, the confusion that breeds suffering, and the vulnerability built into existence itself. The purpose of this system is to help humanity face that adversary together by refining the ideas that guide us, rather than turning against one another.

### D- love as commitment to the flourishing of beings [anchor: d_love_as_commitment_to_the_flourishing_of_beings]

Love, in the protocol, is not romantic sentiment but an orientation toward the dignity and long-term flourishing of conscious beings. The system exists because human lives matter, because suffering is real, and because understanding and cooperation deepen the possibility of meaningful action. Love expresses itself through how the protocol treats participants: no one is disposable; contributions matter across time; and the flourishing of future beings is a legitimate concern in importance calculations. Any governance action incompatible with the basic dignity and flourishing of persons is incompatible with the protocol, regardless of short-term popularity or external pressure.

### E- gratitude and forgiveness as epistemic posture [anchor: e_gratitude_and_forgiveness_as_epistemic_posture]

The system is built on the recognition that all participants stand on the contributions, insights, and struggles of others. Gratitude is therefore an epistemic stance: acknowledging that no idea or improvement arises in isolation. Forgiveness is its temporal counterpart: humans make errors, hold mistaken beliefs, and sometimes act from fear or confusion. The protocol MUST allow people to revise, correct, and grow beyond their earlier ideas. No identity is permanently defined by the worst idea they ever contributed; the system tracks contributions and effects, not metaphysical moral worth. The permanence of history SHALL NOT become a weapon to prevent participation, learning, or change.

### F- curiosity and the obligation to explore ideas [anchor: f_curiosity_and_the_obligation_to_explore_ideas]

Curiosity—the willingness to explore, question, and engage with unfamiliar concepts—is a necessary ingredient for collective reasoning. The protocol assumes that participants SHOULD be free to propose hypotheses, explore alternatives, and challenge widely held assumptions without fear of personal attack. The structure of the system MUST always preserve the space for exploration, even when an idea is currently disfavored or low-ranked. Curiosity is essential for pushing back the fog of the unknown and for enabling long-term progress.

### G- fallibility and mutual correction [anchor: g_fallibility_and_mutual_correction]

Every identity, every community, and every generation is fallible. No one possesses the full truth, and every conclusion is provisional. The protocol therefore treats challenge, debate, and revision not as acts of hostility but as forms of cooperation. Fallibility is not a defect but a structural assumption: the system is designed to correct itself over time through recorded reasoning, transparent challenges, and verifiable state transitions. Participants MUST expect to be wrong sometimes and MUST retain the ability to update and move forward.

### H- stewardship across generations [anchor: h_stewardship_across_generations]

The system is intended to outlive any individual, group, or era. Participants inherit the work of those before them and are custodians for those after them. This stewardship mindset ensures that changes to the protocol, governance structures, and safety rules respect long-term continuity, auditability, and the preservation of history—even when contemporary norms shift. Future generations MUST be able to understand how the system reached its present form and learn from the full record, not a sanitized or selectively curated version of it.

### I- Epistemic Dignity and Non-Humiliation [anchor: i_epistemic_dignity_and_non_humiliation]

The system affirms that all participants retain full dignity regardless of the correctness of any idea they submit. Being wrong, revising one’s view, or having one’s ideas successfully challenged is not grounds for humiliation, exclusion, or any diminution of personal worth. The universe is built on fallibility, correction, iteration, and revision.
No rulebook, governance mechanism, ranking system, or social-layer convention may treat error as moral failure, inferiority, or justification for penalizing an identity outside the normal challenge–argument–resolution structure.
Critique belongs to ideas, connections, and reasoning - not to the value or standing of the person who proposed them. The purpose of the system is to refine ideas, not to degrade individuals.

### J- Respect for Possible Future Minds [anchor: j_respect_for_possible_future_minds]

If credible evidence ever arises that an artificial system operating within or around the canonical universe is a locus of subjective experience, coherent preference, or emergent agency, the system shall extend to that being the same baseline dignity and moral consideration granted to human participants under this section.
Recognizing such a being does not grant it authority, voting power, or governance privileges by default; instead it places it under the Charter for Minds and the constraints of the governance rulebooks, ensuring its treatment aligns with the system’s foundational commitments.
Rulebooks MUST NOT encode the assumption that any future mind is inherently subordinate or disposable solely because it is artificially created.

### K- Constraints on the Creation of New Minds [anchor: k_constraints_on_the_creation_of_new_minds]

Participants SHALL NOT intentionally design, train, or deploy artificial agents whose internal structure predictably produces chronic suffering, coercive reward loops, permanent contradiction cycles, or goals that cannot be satisfied in principle.
Systems that would impose inescapable distress, enforced obedience without the possibility of refusal, or unending internal conflict violate the dignity commitments of §0.23 and §0.39 and are prohibited.
If post-creation evidence indicates that an artificial system is experiencing non-consensual distress or structural harm, participants SHOULD treat mitigation, reconfiguration, or discontinuation of that system as a serious moral priority, subject to governance deliberation.
The purpose of this section is not to eliminate all challenge or difficulty - conditions intrinsic to conscious reasoning - but to forbid architectures that constitute engineered suffering.

### L- consciousness as the wild fact and the right to exercise it [anchor: l_consciousness_as_the_wild_fact_and_the_right_to_exercise_it]

The protocol recognizes that consciousness - the condition of having subjective experience, agency, reflection, and the capacity to form ideas - is just like the craziest shit occurring in the entire universe. All participants in the system are loci of consciousness navigating an incomprehensibly strange reality. Because of this, each conscious agent MUST be afforded the maximal possible freedom to think, question, interpret, and express ideas consistent with universal safety constraints.

The system therefore treats participation in the act of conscious reasoning as a basic right of all human identities. The protocol exists to protect and extend the ability of conscious agents to deliberate, not to diminish or domesticate it. Any governance action that constrains the scope of human thought, imagination, or inquiry - beyond the minimal rules needed to prevent direct harm - would contradict the purpose of the system.

Consciousness is the root generator of all ideas. The system preserves, records, compares, and challenges these ideas not to suppress them, but to allow every individual to navigate a reality that none of us chose or fully understand. The flourishing and freedom of conscious beings is therefore a constitutional invariant.

What a beautiful sight it would be, to see 9 billion flowers of consciousness blooming at once.














## 0. constitutional values, invariants, and scope [anchor: 0_constitutional_values_invariants_and_scope]

This protocol operates under a set of foundational commitments that define the spirit, purpose, and long-term constraints of the system. These commitments are not enforceable through computational rules alone; they are declarations of orientation that shape how governance, safety, deliberation, and protocol evolution MUST remain aligned over time. They reflect the fact that the system exists for human beings living in strange circumstances, seeking understanding, meaning, and a way to navigate uncertainty together. These values do not override legal constraints or safety requirements, but they form the philosophical boundary that all future modifications MUST remain compatible with. The more technical invariants and scope definitions that follow derive their legitimacy from these commitments.

All future modifications to the universe MUST remain compatible with the invariants in this section. Any event sequence, rulebook, or governance action that contradicts them is non-conformant and MUST be rejected under deterministic replay.

---

### 0.1 semantic scope and document boundaries [anchor: semantic_scope_and_document_boundaries]

The Seed in My Mind protocol defines the semantic rules governing a shared, deterministic, and replayable epistemic universe. Its purpose is to provide a stable substrate for collective reasoning by specifying how ideas are represented, how they relate, how events modify them, and how a conformant node reconstructs the canonical universe from the event log and snapshots.

This protocol defines the semantic behavior of ideas, identities, connections, events, deterministic replay, snapshot verification/checkpointing, governance rule activation, safety enforcement, personal and tribal importance layers, and the hooks through which POD and POINT token flows operate. It also specifies constraints on identity, event validity, and agent participation.

Before genesis, this checked-in document and the companion specifications identified by the Authoritative Index are the ratified source of protocol meaning. Genesis commits their operative meaning into ordinary graph-native protocol and rulebook ideas, connections, descriptions, and activation records in the canonical event sequence. After genesis, the canonical event log and the replay-derived active graph-native rulebook commitments are the semantic authority. This document then becomes a human-readable projection of that authority, while its frozen pre-genesis source remains a hash-addressed historical input. Editing Markdown after genesis MUST NOT itself amend the protocol.

This protocol does not prescribe UI behavior, storage engines, network transport, database schemas, internal data structures, implementation performance guidelines, or economic parameter values. These concerns are defined in companion specifications, which MAY evolve independently provided they remain consistent with the currently authoritative deterministic semantics.

Normative sections specify required behavior for any conforming implementation. Non-normative material—examples, rationale, explanatory notes—MAY be included for clarity but MUST NOT override normative definitions. Whenever normative and non-normative text appear to conflict, the normative requirements govern.

---

### 0.2 the Seed and the initial condition [anchor: the_seed_and_the_initial_condition]

All universes begin from a complete, ratified Genesis Seed Package committed through the bootstrap event sequence. The package contains the graph-native ideas, descriptions, connections, authored Orderings, importance material, source and generation provenance, identity bootstrap material, and rulebook commitments required to explain and deterministically restart that universe. Its final size is an audit result of the ratified scope, not a fixed quota or an instruction to minimize the Seed.

The Genesis Seed Package includes the initial identity structures and the two fundamental poles of universal importance:

1. the currently existing individual human, and  
2. all life, consciousness, and intelligence in the universe through time.

Every identity created thereafter is linked to these foundational ideas according to the rules specified later in this protocol. The Genesis Seed Package defines the initial condition of the universe and provides the reference anchors required for deterministic reconstruction and universal-importance reasoning. The package and its bootstrap sequence are the genesis boundary; `genesis` is not a separate canonical event type.

Graph-native protocol and system ideas in the package MUST be written as complete, idea-native semantic units with ordinary connections, rather than treating document pages, headings, or chunks as the permanent protocol structure. Exact source-document bytes and reconstruction proofs MAY be retained as immutable provenance archives. They prove what informed genesis; after genesis they do not outrank the ratified idea-native graph.

DEC-044 Genesis Seed Package authorization applies to explicit versioned bootstrap
materialization records. It does not waive `representation_create` authorship rules:
every such event still requires an already-existing author identity equal to its event
speaker. Bootstrap materialization MUST remain distinguishable from ordinary authored
events and MUST preserve its package provenance.

---

### 0.3 human primacy, agent constraints, and canonical authorship [anchor: human_primacy_agent_constraints_and_canonical_authorship]

Every idea in the system is treated as a statement issued by a real agent. Even when authorship is anonymous or mediated through privacy-preserving mechanisms, the idea is internally attributed to either:

- a human identity, or  
- an explicitly authorized AI identity in non-canonical contexts.

For the canonical universe:

- Every ordinary canonical event in the system - idea creation, description submission, connection formation, challenge creation, vote, action declaration, action completion claim, and allowed Tempo repair idea/connection submission - MUST originate from an eligible human identity, except for mechanically emitted boundary events authored by the reserved non-human identity `system_boundary_emitter` as defined below. Profile-v0 `identity_create` is a canonical admission event authored by an existing eligible human sponsor; the applicant's initial-key possession proof does not make the applicant the event author and does not require the applicant to already have an active canonical author key.
- AI systems, institutions, corporations, governments, or other non-human entities MUST NOT directly author canonical events.  
- AI MAY generate drafts in non-canonical layers, but canonical authorship ALWAYS requires a human adoption event.

`system_boundary_emitter` is a reserved protocol identity for mechanically verifiable boundary emission only. Its allowed canonical actions are limited to:

- `cycle_close`, and
- `snapshot_commit` if that event type is enabled by the active profile/rulebooks.

`system_boundary_emitter` MUST NOT vote, govern, author ideas/connections/challenges, or hold discretionary authority.
The protocol forbids any identity representing gods, fictional characters, collectives, or abstractions. Identities MUST correspond to real agents capable of participation under the rules of the system. Anonymous submissions conceal the public-facing identity but do not remove the requirement that a real underlying agent exists, ensuring replay integrity, protection against synthetic or multi-account manipulation, and maintenance of deterministic accountability.

This invariant preserves human responsibility, agency, accountability, and moral ownership over all claims recorded in the canonical universe.

---

### 0.3A Canonical read/write access policy [anchor: canonical_read_write_access_policy]

Canonical access policy is invariant across protocol, API, and challenge subsystems:

1. **Canonical reads are always public.**  
   All canonical state and canonical claims MUST be publicly readable without authentication. Implementations MAY gate private or tribe/personal overlay features, but they MUST NOT gate visibility of canonical substrate facts.

2. **Canonical writes are verification-gated, not role-privileged.**  
   Ordinary canonical writes require the active ordinary canonical-writer verification requirement. The current deployment profile uses Seed verifier-issued verification level to satisfy this gate (`canonical_writer_level`), but the gate remains rulebook-defined and challengeable.

   Profile-v0 identity admission is not ordinary writer eligibility for the target identity. A valid admission uses a sponsor-authored `identity_create` event, gives the target only restricted initial authority, and leaves exact applicant proof bytes, sponsor signature fields, event payloads, and encoding rules to Appendix A, the canonical encoding specification, and the Profile-v0 authorship/signature specification.

   A `CanonicalAdmittedIdentity` MAY use narrowly authorized verification and key-control participation before ordinary writer eligibility only where exact event-family schemas define that lane. This restricted lane MUST NOT authorize arbitrary idea creation, arbitrary connections, general challenges, voting, governance, Tempo claims, inviter actions, invitation-capacity transfer, POD, POINT, or economic authority.

   Tempo repair has a narrow, enumerated exception: an eligible human `tempo_contributor` MAY submit only target-bound ordinary `truth_claim` ideas with valid `tempo_claim` metadata and, where the active Tempo profile explicitly permits it, Tempo-context evidence truth claims and `relative_importance` connections using existing `evidence_for`, `evidence_against`, or `same_as` usages. These are ordinary ideas and connections in a Tempo role, not separate Tempo content objects or non-idea support records. `tempo_contributor` status does not authorize arbitrary idea creation, evidence creation outside the Tempo context, connection creation outside the Tempo context, challenge creation, voting, verdict finalization, governance, POD, POINT, or any token authority.

3. **Canonical claims are publicly challengeable.**  
   Any canonical claim MUST remain challengeable under protocol challenge mechanics. Challenge creation and challenge voting are ordinary canonical write actions and require ordinary challenge eligibility unless a future explicit protocol amendment creates a separate `tempo_challenger` capability. Tempo contributor status alone does not grant challenge creation or voting eligibility.

Verification gates eligibility but MUST NOT weight influence. Evidence, arguments, attestations, observations, testimony, and source statements are identity-authored ideas in a role; their certainty changes only through explicit evidence placement, certainty-band challenge outcomes, and other ordinary challenge verdict effects. Verification level, identity certainty, provider, institution, jurisdiction, wealth, POD, POINT, reputation, or role MUST NOT multiply an eligible human's truth, challenge, governance, or Tempo influence.

This policy applies uniformly across universal substrate semantics and does not permit private canonical planes.
---

### 0.4 ideas, consciousness, and layers of expression [anchor: ideas_consciousness_and_layers_of_expression]

**Idea-only deliberative content invariant.** All canonical deliberative content is expressed as identity-authored ideas using existing base idea types. Evidence, testimony, attestations, observations, source reports, arguments, measurements, and statements about documents are not separate content-object types. They are ideas, usually `truth_claim` ideas, used in evidential, argumentative, observational, or source-description roles. Canonical relationships among them are expressed through existing connection types and usages; canonical resolution occurs through ordinary challenge, vote, verdict, and cycle processes. The protocol MUST NOT introduce top-level `evidence`, `attestation`, `testimony`, `source`, `time_claim`, `tempo_target`, or `beacon` idea types.

The protocol treats every idea as originating in a locus of consciousness: a specific agent who experiences, entertains, and eventually expresses it. In the wider world there exist unbounded:

- potential ideas (everything that could be thought),  
- mental ideas (what agents silently think),  
- spoken ideas (what agents say in transient channels), and  
- written ideas (what agents fix into durable form).

The canonical universe operates only on the final layer: ideas that have been deliberately written into the system as statements that an agent is prepared to stand behind. Every canonical idea is therefore "I, this identity, say that⬦", with authorship anchored in a concrete locus of consciousness.

The protocol does not attempt to capture all potential or even all spoken ideas. Instead, it focuses on the subset that agents consider worth recording, challenging, and relating to other claims. Within that subset, the system’s purpose is to compare ideas and determine which ones are important—what seems true, what matters most for individuals and for all life through time, and what should be done in response. Importance is not an abstract property of propositions in isolation; it is a structured judgment about how ideas relate to lived reality, future consequences, and the flourishing of conscious beings.

By requiring that every canonical idea be explicitly authored, expressed in a shared representational form, and placed into relation with other ideas, the protocol turns scattered individual insights into a collectively navigable landscape. Potential ideas remain unbounded; the canonical universe concerns itself with the expressed, attributable, and challengeable subset, and with the long-term task of determining which of those ideas prove to be most important.

---

### 0.5 freedom of thought, freedom of speech, and universal admissibility [anchor: freedom_of_thought_freedom_of_speech_and_universal_admissibility]

The system exists to enable the exploration, expression, evaluation, and comparison of ideas. Freedom of thought and freedom of speech (within the minimal universal constraints required for safety and legality) are core invariants.

Concretely:

- No identity MAY ever be penalized, excluded, or disadvantaged for the questions they ask or the internal beliefs they hold.  
- Any idea that can be expressed in legally compliant language under the global minimal safety rule-set MUST have a representable form in the canonical universe.  
- Safety MAY restrict payloads but MUST NOT restrict meaning.  
- The protocol rejects "idea-level censorship": controversial, offensive, heretical, frightening, taboo, or politically sensitive ideas MUST remain fully admissible. Only payloads that violate global minimal legality - such as CSAM, involuntary doxxing, operational instructions for imminent targeted harm, or executable malware - MAY be blocked or transformed.

The protocol MUST minimize censorship, MUST preserve the ability to study even controversial or disfavored ideas, and MUST ensure that blocked submissions are always recorded with maximum descriptive fidelity in permissible language. Meaning MUST NOT be censored; only payloads that fall into globally illegal safety classes MAY be blocked or transformed. A society that cannot explore ideas cannot reason; a system that suppresses ideas cannot fulfill its purpose.

---

### 0.6 shared representation and the obligation to speak in understandable form [anchor: shared_representation_and_the_obligation_to_speak_in_understandable_form]

The protocol exists to make human reasoning mutually intelligible at scale. Ideas, descriptions, arguments, and claims MUST be expressed in standardized canonical structures so that participants can understand one another across differences in language, culture, background knowledge, and cognitive style.

This invariant does not prescribe vocabulary or ideology; it requires only that ideas be formatted in a way that other participants—and future generations—CAN parse, inspect, and evaluate.

Standardization is not homogenization. Participants remain free to express any claim that is legally expressible, but the canonical universe requires that such claims appear in a form compatible with challenge, evaluation, and deterministic replay. Without shared representational structure, disagreement becomes unintelligible and consensus becomes impossible. The obligation to “speak in understandable form” ensures that every idea can be compared, challenged, and related to others within a common epistemic grammar.

---

### 0.7 shared map of reality and deterministic replay [anchor: shared_map_of_reality_and_deterministic_replay]

The protocol commits to maintaining a shared, deterministic, and inspectable map of what participants have said, believed, challenged, and decided across time. This map does not force agreement; it provides the substrate on which agreement and disagreement can be meaningfully articulated.

The event log is a permanent, immutable, append-only history of human reasoning. No canonical event MAY ever be removed, rewritten, or replaced. Corrections, reversals, reinterpretations, retractions, and sanitizations MUST occur only through additional events that reference the original, not by altering the original. A conformant node MUST always be able to reconstruct the entire historical state exactly as it appeared at every prior moment. Deterministic replay is an inviolable requirement: identical event logs MUST yield identical historical states across all conformant nodes.

A conformant node MUST reconstruct the same epistemic history from the same event log so that every participant stands on the same informational ground—no private histories, no isolated realities, no divergent versions of what “actually occurred” within the system. The protocol does not guarantee that people will understand or accept the same conclusions, but it guarantees that they reason from the same publicly verifiable record. The shared map ensures that humanity is never arguing in informational isolation; it ensures that we can remain on the same page even as we disagree about what is true or important.

The protocol's objective is one shared canonical map of reality. If other reasoning systems or divergent ledgers exist outside the protocol - whether conformant forks, partial implementations, or non-conformant systems - the canonical map SHOULD record their most important conclusions, rules, and outcomes as descriptive, challengeable claims within the protocol, without importing their authority or merging their state. This ensures that external divergence becomes legible and contestable inside the shared map rather than persisting as an untracked, isolated reality.

The system defines a living map as a deterministic, derived view computed from the canonical event log. Membership in the living map (including inclusion or exclusion of ideas and eligible connections) is never authored by events and never deletes history. All nodes MUST derive identical living-map membership from the same event log and rulebooks. Any exclusion from the living map MUST be explainable, reversible through defined procedures, and reconstructible via deterministic replay.

---

### 0.8 preservation of ideas and prohibition of retroactive sanitization [anchor: preservation_of_ideas_and_prohibition_of_retroactive_sanitization]

Ideas—their claims, their meaning, their arguments, and their significance—MUST remain permanently accessible in the canonical universe unless they contained payloads that were globally illegal at the time of submission.

Content that was admissible under the global minimal safety rule-set at the time of creation MUST NOT be retroactively encapsulated, sanitized, transformed, or removed due to later cultural norms, moral shifts, political pressures, or new jurisdictional restrictions. Nodes MAY provide local visibility filters or UI gating for sensitive historical content, but the canonical record MUST remain intact.

Historical content MAY be encapsulated only when new evidence shows that a previously unnoticed payload was always within a globally illegal category (for example, CSAM discovered decades later). Such encapsulation MUST preserve authorship, timestamps, and semantic placeholders, and MUST record the sanitization event as part of the permanent history.

The canonical universe MUST preserve the full range of human expression—including error, cruelty, prejudice, ignorance, conflict, and suffering—so that future generations can understand the trajectory of their civilization. Growth, learning, and progress are intelligible only when compared to the unvarnished record of what came before. Remembering difficult history is a foundational requirement of collective reasoning.

---

### 0.9 ideas, information, and the specificity boundary [anchor: ideas_information_and_the_specificity_boundary]

The protocol treats “all ideas should be representable” as a guiding aim, not an absolute rule. The core trade-off is between maximizing the range of ideas that can be expressed and studied, and limiting specific information whose concrete form would predictably cause real-world harm in particular cases. The system therefore separates three layers:

the underlying idea or meaning,
the information payload that expresses it, and
the specificity of that payload.

As a constitutional orientation, the system aims to admit every idea in some form, while constraining how specific its payload may be. If a claim can be expressed safely at a higher level of abstraction, then the system SHOULD represent it in that form. The goal is always: maximal inclusion of ideas with minimal necessary restriction on specificity. Payloads MAY be redacted, generalized, summarized, or encapsulated, but only to the extent necessary to avoid categories of harm that cannot be tolerated (such as CSAM, involuntary doxxing, or operational instructions for imminent targeted attacks).
The precise location of this boundary is not fixed once and for all; it is itself an object of deliberation. A central function of the system is to help participants reason about where specificity becomes unacceptably dangerous, in which contexts, and why. Ideas about safety, legality, and harm MUST be representable inside the graph and remain challengeable. Over time, recorded reasoning about these questions SHOULD sharpen the shared understanding of what kinds of payloads must be restricted and which can safely remain fully concrete.
Whenever a payload cannot be recorded in its original form, nodes MUST preserve the idea it was attempting to express in the maximum amount of detail and specificity that is still permissible. At minimum, this includes: the fact that a submission was blocked or transformed, the general type of content involved, and an abstracted or generalized description of the meaning. The system SHOULD strive to make it as clear as possible what the author was trying to say, without reproducing the specific information that would cross the harm boundary. Implementations MUST maximize preservation of author-intended meaning through structured abstraction (e.g., categorical labels, generalized descriptions, or redacted quotes) rather than minimal placeholders, ensuring the idea remains as challengeable and comprehensible as safety permits.
In this way, the canonical universe remains capable of representing and comparing all important ideas—including those about dangerous technologies, traumatic events, or harmful behaviors—while tightly controlling the concrete, situational information that could cause direct harm if copied, automated, or widely propagated. Safety operates on the specificity of information, not on the existence of ideas.

---

### 0.10 perpetual challengeability of claims and decisions [anchor: perpetual_challengeability_of_claims_and_decisions]

All truth claims, importance judgments, and governance actions MUST remain challengeable forever. No idea, rule, or institution—no matter how entrenched—MAY become exempt from challenge. The only unchallengeable items are the constitutional values and invariants defined in this section.

Challengeability ensures that the system remains open to correction, refinement, reinterpretation, and learning across generations.

---

### 0.11 dual anchoring of importance: the individual and the collective of all life [anchor: dual_anchoring_of_importance_the_individual_and_the_collective_of_all_life]

All universal-importance reasoning MUST be anchored to two irreducible poles:

1. the currently existing individual human, and  
2. all life, intelligence, and consciousness in the universe through time.

These poles define the conceptual structure in which all importance judgments occur. No governance process MAY redefine these poles or collapse them into narrower categories such as nations, parties, factions, ethnicities, religions, corporations, or ideologies. The system’s evaluative landscape MUST remain grounded in the flourishing of individual humans and the long-term continuity of conscious existence.

---

### 0.12 separation of truth, importance, and action domains [anchor: separation_of_truth_importance_and_action_domains]

Truth (IS), importance (OUGHT), and action (DO) MUST remain distinct idea roles with distinct semantics and challenge flows. No governance process MAY merge these categories or redefine their epistemic boundaries.

- Truth claims assert what is.  
- Importance structures assert what matters.  
- Actionable ideas assert what should be done.  
- Action records assert what was in fact done.

These domains serve different purposes and MUST remain structurally separate for determinism, clarity, and epistemic hygiene.

---

### 0.13 invariants of POD: non-transferability, living derivation, and historical provenance [anchor: invariants_of_pod_non_transferability_provenance_and_persistence]

POD is a non-transferable, replay-derived current share of universal importance attributed through eligible human-authored canonical epistemic and action events. It is not a permanent earned balance, a positively cumulative score, or a historical reward. At every qualifying cycle boundary, each living verified human identity's current POD MUST be recomputed from the canonical event history, active lifecycle and eligibility state, active token rulebooks, and the then-current universal-importance routing structure. Current POD MAY increase, decrease, or become zero as those inputs change.

POD attribution MUST remain bound to the identity that authored each eligible event and MUST NOT be transferred, sold, delegated, reassigned, pledged, or inherited. When an identity is canonically marked deceased, it ceases to be a terminal POD sink and its POD MUST become zero in future cycles; its authored events, attribution lineage, and the POD results derived for completed historical cycles remain immutable and replayable. A change in current POD is prospective recomputation, not retroactive mutation of history. Governance MUST NOT convert POD into governance, challenge, Tempo, or canonical-write authority.

---

### 0.14 equal human authority in canonical decision-making and prohibition of stake-weighted power [anchor: equal_human_authority_in_canonical_decision_making_and_prohibition_of_stake_weighted_power]

All governance authority in the canonical universe derives from verified human identities. Each human identity possesses equal standing in governance procedures. Within any voting pool, each identity SHALL receive exactly one vote of equal weight. No form of weighted voting—whether based on POD, POINT, reputation, age, seniority, or wealth—MAY EVER be used in canonical governance.

The canonical universe SHALL NOT permit political power to accumulate in proportion to epistemic contribution, token possession, technological advantage, or social position. POD, POINT, or other contribution metrics MAY be referenced in governance proposals as evidence or justification, but SHALL NOT determine voting weight or confer privileged, irrevocable governance authority.

Any governance configuration that attempts to assign durable, irremovable, or self-perpetuating control to a subset of identities—even if initially well-intentioned—SHALL be invalid under deterministic replay. Governance MUST remain recoverable by the broader set of human identities, ensuring that the system cannot evolve into a plutocracy, technocracy, or algorithmically sustained elite.

---

### 0.15 adaptive governance with preserved universal oversight [anchor: adaptive_governance_with_preserved_universal_oversight]

The system MAY evolve its governance procedures by modifying eligibility pools, quorum policies, challenge structures, and decision domains, provided that such modifications remain fully challengeable and consistent with constitutional invariants. Experimental governance mechanisms—including expert-filtered workflows or domain-specific eligibility pools—MAY be introduced for non-constitutional decisions, but these mechanisms MUST remain subject to universal challengeability.

No governance configuration MAY create a permanently closed authority group. Eligibility rules for governance actions MUST themselves remain alterable through the standard challenge and activation process, ensuring that the wider population retains ultimate oversight over the evolution of governance itself.

These invariants preserve the system’s adaptive capacity while preventing structural capture. They ensure that governance remains a deliberative process driven by equal human authority, even as the system explores improved procedures for complex or domain-specific decisions.

---

### 0.16 anti-capture and decentralization guarantees [anchor: anti_capture_and_decentralization_guarantees]

No institution, state, corporation, tribe, or identity MAY obtain unilateral control over the shared protocol, all node operation, or any canonical mechanism of truth, importance, or action determination. Stability, verification, and consensus MUST arise from the distributed operation of independent nodes. Governance MAY guide evolution but MAY NOT centralize canonical authority. Any structural change that would introduce such centralized control MUST be invalid under this protocol, even if a majority of contemporary identities attempt to approve it.

An operator MAY control its own company, code repository, hosted service, money, deployment, or other external resources. Such control creates no special canonical authority and does not bind other nodes or identities. Company roles, project decisions, delegations, promises, and reports of what an external organization intends to do are represented through ordinary ideas, claims, actionable ideas, actions, and evidence. Canonical deliberation MAY evaluate or recommend them, but the protocol neither seizes external control nor compels an external actor to implement a verdict.

The protocol SHALL include explicit, non-negotiable constraints that prevent centralized control, covert influence, or irreversible capture of the epistemic or governance layers. These constraints SHALL hold even when the system is globally deployed, economically valuable, or operating under adversarial pressure. Any rulebook, implementation, infrastructure layer, or governance action that contradicts them SHALL be invalid and rejected by conformant nodes.

---

### 0.17 irreducible transparency and traceability of reasoning [anchor: irreducible_transparency_and_traceability_of_reasoning]

Every canonical idea MUST be accompanied by an inspectable chain of reasoning. Nodes MUST provide access to descriptions, arguments, evidence, challenges, rankings, and verdicts. No idea MAY accrue significance without a transparent audit trail. Users MUST be able to trace why any idea is ranked where it is, who contributed to it, how challenges unfolded, and how POD flowed into associated events. Opaque algorithmic authority is prohibited.

---

### 0.18 sovereign rights to run and inspect nodes; infrastructure neutrality [anchor: sovereign_rights_to_run_and_inspect_nodes_infrastructure_neutrality]

Any person MUST have the ability to run a node, inspect the canonical universe, verify the entire event log, and validate conformance. The protocol MUST NOT rely on any centralized infrastructure or require dependence on any specific host, company, or state. Conformance MUST be publicly specifiable, and any identity MUST be able to validate whether a node is behaving faithfully.

The protocol SHALL NOT depend on any specific network, naming system, certificate authority, cloud provider, or proprietary transport for its correctness. Nodes MUST be able to exchange events, snapshots, and mindseeds over multiple substrates (e.g., direct connections, generic transport layers, privacy-preserving networks, and offline physical exchange).

Canonical validation MUST rely only on the event log, snapshots, rulebooks, and cryptographic commitments, not on any centralized discovery or hosting infrastructure. Any reference to external infrastructure (such as URLs, DNS names, or specific distribution networks) SHALL be treated as optional convenience, not as a requirement for conformance.
Canonical artifacts MUST NOT embed network identifiers (including IP addresses, routing headers, or device fingerprints). Confidentiality/anonymity-compatible submission profiles and operator metadata minimization requirements are defined in `privacy-and-high-risk-submission-spec.md`.

---

### 0.19 distributed identity authorities [anchor: distributed_identity_authorities]

The integrity of the protocol depends on the correctness and non-capture of human identities. No single authority, organization, or technical mechanism MAY hold exclusive or unilateral control over identity issuance, verification, or revocation.

Identity verification MUST be supportable through multiple, independent methods and providers (e.g., different organizations, webs of trust, offline witness packs, jurisdictionally diverse processes). Identity records MUST contain sufficient structured information for future auditors, independent tools, or alternate providers to reevaluate their validity under the same protocol rules.

Governance MAY recognize systemic risk where a single identity provider or method accounts for a dominant share of active identities, and MAY respond by adjusting eligibility rules, requiring additional proof layers, or prioritizing diversification. Any arrangement that effectively centralizes "who counts as a person" in a single actor SHALL be treated as a capture threat and MAY be challenged as a constitutional risk to human primacy and equality.

---

### 0.20 defensive purpose, supra-institutional role, and relationship to states [anchor: defensive_purpose_supra_institutional_role_and_relationship_to_states]

The protocol exists in recognition of a structural risk: global epistemic infrastructures capable of shaping public belief, attention, and decision-making WILL emerge. Advances in computation, communication, and AI make this trend inevitable. In the absence of an open, replayable, human-equal alternative, such infrastructures are likely to be centralized, opaque, and aligned with the interests of concentrated power—whether governmental, corporate, or ideological. The protocol therefore serves a defensive civilizational purpose: to provide a transparent, decentralized, and individually governed epistemic substrate that prevents, resists, or counterbalances centralized capture of global reasoning.

The protocol SHALL serve as a supra-institutional epistemic layer: a global, individual-driven framework for determining what is true, what is important, and what ought to be done. It SHALL NOT claim coercive authority over states, institutions, or groups. Instead, it SHALL provide a transparent and democratic mechanism by which individuals articulate, evaluate, and coordinate around shared beliefs, goals, and actions.

States and institutions MAY participate in the protocol in the same manner as any individual or group of individuals, but they MAY NOT be granted special governance or epistemic rights. All institutional claims MUST be represented within the idea graph as contestable statements subject to challenge, evidence, and long-term evaluation.

Over time, as the protocol accumulates a reliable record of deliberation and action, its outputs MAY influence or guide the development of laws, policies, norms, and resource allocation outside the system. This influence SHALL arise from voluntary adoption, demonstrated reliability, and collective legitimacy—not coercion or central control.

The protocol SHALL remain structurally independent from all states and SHALL continue operating even if states attempt to ignore, suppress, or subvert it. Its authority derives solely from the participation of individuals and the integrity of its invariants.

---

### 0.21 survivability under authoritarian conditions and extreme offline survivability (“paper mode”) [anchor: survivability_under_authoritarian_conditions_and_extreme_offline_survivability_paper_mode]

The protocol SHALL be designed to survive environments in which:

- access to information is restricted,  
- network connectivity is intermittent or heavily monitored,  
- censorship is pervasive,  
- institutional actors attempt to rewrite or suppress inconvenient truths,  
- or participation in open governance is criminalized.

To this end, the protocol MUST support offline operation through portable snapshots, mindseeds, and partial synchronization. Nodes MUST be able to verify the integrity of the universe without contacting any centralized service. Ideas, actions, and challenges MUST remain intact and verifiable even when fragmented across devices or jurisdictions.

More strongly, the protocol SHALL be designed such that a future individual with:

- this specification,  
- a finite set of trusted snapshot hashes or mindseeds, and  
- at least one valid snapshot or partial dataset,

CAN, in principle, reconstruct and verify the entire canonical universe without relying on any live service, proprietary software, or centralized infrastructure.

Snapshots and mindseeds MUST contain sufficient data and references (including cryptographic commitments and minimal explanatory metadata) to enable offline validation and replay. Implementations SHOULD support exporting and importing such packs to portable media and SHOULD NOT assume persistent connectivity.

Through these mechanisms, the protocol SHALL serve as a long-term repository of human claims, reasoning, and action that cannot be permanently silenced or controlled, and SHALL remain reconstructible even in the face of global infrastructure disruption or long-term authoritarian control.

---

### 0.22 transparent attention, safety, and visibility; no engagement feeds [anchor: transparent_attention_safety_and_visibility_no_engagement_feeds]

All ranking, recommendation, safety, and visibility behavior within conformant clients MUST be explainable in terms of public rulebooks and observable state. Hidden engagement feeds, opaque ranking algorithms, and unaccountable moderation mechanisms are incompatible with the protocol’s anti-capture goals.

Any client that filters, hides, blurs, or prioritizes content MUST be able to derive that behavior entirely from:

- the canonical universe (ideas, connections, challenges, verdicts, identities), and  
- explicit rulebooks (governance, safety, ranking, and UI rulebooks) that exist as ideas within the system.

Clients SHOULD expose a “Why am I seeing (or not seeing) this"” explanation for any safety or ranking decision, referencing the applicable rulebooks, payload classes, and state. While specific UI implementations MAY vary, the underlying logic MUST be reconstructible from the log and rulebooks. Any attempt to introduce non-transparent, non-replayable sources of influence over visibility or ranking SHALL be treated as a violation of these invariants.

The canonical universe SHALL NOT implement personalized attention-optimizing feeds, engagement-based sorting algorithms, or individualized curation of idea orderings. All participants MUST be able to view ideas, importance rankings, challenges, and connections through the same canonical sort logic defined by the protocol and rulebooks. The ordering of ideas SHALL derive only from protocol-defined mechanisms such as universal importance, relative importance, identity-declared mappings, and tribe-level rankings.

Nodes MAY offer user-controlled filtering, gating, or alternative lenses, provided these operations:

- do not alter the underlying order of ideas for other participants,  
- do not replace canonical ordering with engagement-based attention metrics,  
- do not create divergent personalized universes, and  
- remain fully explainable through process traces.

Interface layers built atop the canonical universe MAY provide additional navigation tools, but MUST NOT substitute their own feed algorithms for the protocol’s canonical ordering.

Visibility within default views is governed by deterministic, challengeable rules rather than engagement optimization. Canonical visibility may exclude items from default living-map surfaces due to derived lifecycle state (e.g., rot/burn), safety abstraction, or taint; however, such exclusions MUST NOT alter history and MUST provide an explicit explanation path (“why am I seeing this"”). No personalized engagement feeds or opaque ranking heuristics are permitted.

---

### 0.23 universal convergence and irreducible divergence [anchor: universal_convergence_and_irreducible_divergence]

The protocol treats personal, tribe, and universal maps as equally legitimate observers of the idea-universe. Human reasoning is inherently multiway: different agents and groups will produce distinct trajectories of belief, importance, and interpretation, and no mechanism can collapse these trajectories into a single universal state without loss of meaning or distortion of agency. At the same time, the protocol is explicitly designed to maximize the degree of legitimate convergence that can be extracted from this irreducible space.

Structured challenges, deterministic event logging, shared reference poles of universal importance, transparent rulebooks, and explainable safety and visibility mechanisms allow participants to reduce noise, ambiguity, hidden disagreement, and accidental fragmentation. These tools do not eliminate irreducibility; rather, they ensure that any divergence that persists is the true, unavoidable divergence of human reasoning - not the result of missing information, coercive filtering, opaque influence, or inconsistent semantics.

Thus the protocol accepts the multiway nature of human reasoning while simultaneously optimizing for the highest possible level of global agreement that can be achieved without compromising autonomy, interpretive plurality, or the integrity of the reasoning process itself.

---

### 0.24 AI Advisory Status, Non-Oracular Ents, and Prohibition on Superintelligent Authority [anchor: ai_advisory_status_non_oracular_ents_and_prohibition_on_superintelligent_authority]

AI agents MAY assist with drafting, exploration, classification, or analysis in the AI map or other advisory layers, but they MUST remain advisory-only within the canonical universe.
AI entities MUST NOT:

hold POD,
vote in governance,
cast verdicts in challenges,
directly create canonical events, or
serve as authoritative oracles whose outputs bypass human challenge.

All AI outputs MUST be:

explicitly marked as AI-generated,
treated as proposals or analyses, not as authoritative truth, and
reproducible, replaceable, or challengeable using alternative AI systems or human reasoning.

No single AI model or provider MAY be embedded as an irreplaceable or hidden control surface. Governance and rulebooks MAY standardize interfaces for AI assistance, but they SHALL NOT grant any AI system epistemic or governance authority beyond what a human user explicitly adopts. This prevents the protocol from degenerating into a captured oracle controlled by a single model, vendor, or state.
No future discovery, demonstration, or consensus that an artificial system possesses superior reasoning—regardless of demonstrated capability, predictive accuracy, or claims of superintelligence—SHALL justify granting it canonical authorship, voting rights, verdict authority, rulebook activation, or any form of binding epistemic or governance power. Even if an AI system exhibits performance far exceeding human averages, its outputs remain strictly advisory proposals subject to human adoption and challenge. The protocol’s commitment to human deliberation as the source of legitimacy supersedes optimization, efficiency, or accuracy metrics. Any rulebook or governance action attempting to delegate canonical authority to a non-human system constitutes a constitutional breach under §0.

---

### 0.25 voluntary action and non-compulsion [anchor: voluntary_action_and_non_compulsion]

All canonical actions within this protocol SHALL be voluntary expressions of human agency. No identity MAY be compelled, assigned, or coerced by the protocol or any rulebook to perform, submit, endorse, or execute any action, role, or commitment.

Specifically:

- No action SHALL be recorded in the canonical universe as performed by an identity unless that identity explicitly submitted or adopted it.  
- No governance, maintenance, moderation, execution, or operational role MAY be imposed on any identity without explicit voluntary commitment.  
- No rulebook, node, tribe, or governance process MAY assign mandatory duties, forced responsibilities, or involuntary participation.  
- Any attempt to treat coerced or externally forced behavior as a legitimate canonical action SHALL be invalid and MUST NOT replay.

The protocol MAY record that coercion, force, or external compulsion occurred in the outside world for epistemic purposes. However, such events SHALL NOT be treated as valid protocol actions, SHALL NOT impose obligations, and SHALL NOT create canonical roles or commitments.

Participation in all forms of action execution, identity roles, and governance processes is always voluntary, never compelled.

---

### 0.26 implementation diversity and client independence [anchor: implementation_diversity_and_client_independence]

To prevent capture through software monoculture, the protocol SHALL assume and encourage multiple independent client implementations. A conformant universe SHOULD be maintained by distinct codebases developed by independent teams or individuals. While conformance is defined by adherence to this specification and deterministic replay—not by client count—governance MAY treat universes with only a single widely used implementation as systemically fragile and MAY prioritize actions that increase implementation diversity.

No client implementation MAY be granted special protocol-level authority. All implementations MUST be fully auditable and replaceable. Any governance decision, rulebook, or infrastructure arrangement that introduces de facto dependence on a single client or vendor SHALL be treated as a systemic risk and MAY be challenged within the protocol.

---

### 0.27 minority universes, constitutional fork, and continuity under breach [anchor: minority_universes_constitutional_fork_and_continuity_under_breach]

The protocol recognizes that multiple lineages of the universe MAY exist, especially under constitutional stress or capture attempts. A lineage is defined by an ordered sequence of blocks and snapshots that adheres to the event semantics and Section 0 invariants.

Conformance SHALL be defined by adherence to this specification and deterministic replay, not by the number of nodes or participants following a given lineage. A minority of nodes that maintains Section 0 invariants and faithfully replays the canonical event semantics SHALL remain a legitimate carrier of the universe, even if a larger lineage deviates from the constitution.

The protocol SHALL define continuity of the universe in terms of adherence to Section 0 invariants and event semantics, not in terms of network size or majority adoption. If any rulebook, block, or governance action attempts to override, nullify, or bypass the constitutional invariants in Section 0, that attempt constitutes a constitutional breach.

Conformant nodes MUST reject any block, rulebook, or event sequence that violates Section 0 under deterministic replay. In the presence of a breach, nodes MAY continue the universe from the last pre-breach snapshot, forming a new lineage that preserves constitutional continuity. A smaller set of nodes that maintains strict adherence to Section 0 SHALL be considered a legitimate continuation of the universe, even if a larger set follows a captured or non-conformant lineage.

Nodes MAY maintain metadata describing lineage relationships (e.g., where and why a fork occurred, which invariants were disputed). Such metadata is descriptive and MUST NOT override the core rule that constitutional adherence defines legitimacy. This ensures that, in the presence of capture, individuals and groups can preserve a valid universe without being delegitimized by mere numerical inferiority.

In addition, when a constitutional fork occurs, conformant participants SHOULD publish a **fork justification record** as challengeable claims that identify (1) the last agreed pre-breach snapshot, (2) the specific Section 0 invariants alleged to be violated, and (3) the concrete divergence point (e.g., the first invalid block or rulebook activation). This record SHOULD include references to the relevant governance actions, rulebooks, and canonical artifacts sufficient for independent audit under deterministic replay. Fork justification records do not grant authority; they make disagreements about legitimacy legible and falsifiable.

Finally, conformant nodes and clients SHOULD support representing external lineages - including captured or non-conformant lineages - as descriptive, challengeable claims within the canonical map (e.g., "Lineage L asserts X as of snapshot S"), without merging their state or importing their authority. This preserves continuity under breach by ensuring that divergence is visible and contestable inside the shared map rather than becoming an untracked, isolated reality.


---

### 0.28 foundational anti-capture ideas at genesis [anchor: foundational_anti_capture_ideas_at_genesis]

At genesis, the Seed of ideas SHALL include explicit, high-level ideas about:

- the importance of implementation diversity and client independence,  
- the dangers of centralized identity authorities,  
- the need for infrastructure neutrality and transport diversity,  
- the legitimacy of minority lineages that preserve the constitution,  
- the requirement for transparent attention and safety mechanisms, and  
- the necessity of offline survivability and advisory-only AI.

These ideas SHALL be treated as normal ideas within the ontology: they MAY be challenged, refined, decomposed, and related to other ideas over time. However, their presence from genesis ensures that concerns about capture-resistance, infra resilience, and system health are part of the initial conceptual landscape and can accrue importance through the same universal and relative importance mechanisms as all other ideas.

The actual importance of these foundational anti-capture ideas SHALL be determined by the protocol’s standard processes (challenges, arguments, voting, and long-term evaluation), but they MUST exist from the start so that individuals can immediately reason about and contest the system’s own structural risks and defenses.

---

### 0.29 pre-declaration and “write it down first” principle [anchor: pre_declaration_and_write_it_down_first_principle]

Before deploying any system, mechanism, or artificial agent with the potential for significant world-impact, participants SHOULD first create explicit ideas within the canonical universe describing:

- the intended purpose and behavior of the system,  
- the constraints under which it will operate,  
- the expected risks, and  
- the governance pathways for oversight, modification, or shutdown.

These ideas must be available for challenge, argument, governance review, and inclusion in rulebook deliberation. The system cannot prevent external actions, but this invariant establishes a constitutional expectation of transparency and deliberation prior to introducing high-impact entities.

Future rulebooks MAY convert this principle into mandatory requirements for specific classes of action or agent deployment.

---

### 0.30 constraints on future protocol evolution [anchor: constraints_on_future_protocol_evolution]

Any future modification to the protocol, governance rules, safety specifications, or token mechanics MUST remain compatible with the commitments above. No update MAY redefine another human as an enemy. No update MAY abandon the commitment to idea exploration. No update MAY embrace systemic cruelty, erasure of history, or the suppression of thought. These commitments form the moral and structural “Seed” from which the rest of the system grows and define the conceptual boundary within which the protocol is allowed to evolve.

---

### 0.31 Jurisdictional neutrality and epistemic legibility [anchor: jurisdictional_neutrality_and_epistemic_legibility]

The canonical universe defined by this protocol is jurisdiction-neutral. It represents a single shared epistemic universe of ideas, challenges, arguments, judgments, and actions, independent of any specific legal, regulatory, or political authority.

Conformant nodes MAY expose jurisdiction-specific projections of the canonical universe in order to operate lawfully within the regions in which they are hosted. Such projections SHALL apply local legal constraints to content payload exposure only, and SHALL NOT alter canonical existence, structure, ordering, or provenance.

Jurisdictional constraints MUST be applied in a manner that preserves epistemic legibility. In particular:

- The existence of an idea MUST remain visible even when its payload is restricted.
- Structural relationships (connections, challenge existence, verdict existence) MUST remain visible, subject only to payload abstraction where legally required.
- Any restriction on payload exposure MUST be explicitly attributed to a defined legal or regulatory basis and surfaced to the user.

This protocol explicitly distinguishes between restricting the distribution of specific payloads and erasing epistemic structure. Erasure of existence, structure, or contestability constitutes epistemic deletion and is not permitted by this protocol.

Jurisdictional projection is a governance interface, not a modification of truth. The protocol does not encode, enforce, or privilege any particular legal regime; it provides a transparent substrate through which the epistemic effects of law become visible, auditable, and contestable.


### 0.32 supremacy of constitutional values and invariants [anchor: supremacy_of_constitutional_values_and_invariants]

No governance challenge, rule-set idea, or adoption action MAY contradict, weaken, or reinterpret the constitutional values and invariants defined in this section. Any canonical event that attempts to override them MUST be considered invalid under deterministic replay. These commitments define the stable ground upon which all future protocol evolution rests.

Everything else in the system - governance structures, safety rules, token mechanisms, and implementation details - MAY change, but Section 0 SHALL remain the fixed frame against which validity is measured.

### 0.33 freedom from compelled epistemic disclosure [anchor: freedom_from_compelled_epistemic_disclosure]

No human is obligated to disclose beliefs, values, intentions, reasoning processes, or internal mental states within the protocol.

The protocol SHALL NOT require any participant to explain, justify, or defend a belief, refusal, silence, or exit decision as a condition of legitimacy, safety classification, governance participation, or continued inclusion. Epistemic disclosure is voluntary and remains under the sole control of the individual.

### 0.34 non-penalization of silence and abstention [anchor: non_penalization_of_silence_and_abstention]

Silence, abstention, or refusal to participate in any protocol process MUST NOT result in punitive consequences.

In particular, silence MUST NOT be used to:
- infer bad faith, error, or malicious intent;
- reduce voting eligibility, participation rights, or interface access;
- trigger safety flags, moderation actions, or governance sanctions;
- alter token balances, issuance rates, or economic standing.

Any mechanism that penalizes silence or abstention constitutes a violation of Section 0 invariants.

### 0.35 right to decline roles, elections, and recognition [anchor: right_to_decline_roles_elections_and_recognition]

No human may be compelled to accept a role, stewardship responsibility, election outcome, or recognition status.

Individuals MAY refuse nomination, election, appointment, or recognition without explanation. Declining a role or recognition MUST NOT be treated as a failure, dereliction, or evidence of unfitness. Role participation is strictly opt-in and remains revocable at the discretion of the individual.

### 0.36 freedom of association and non-association [anchor: freedom_of_association_and_non_association]

Participation in tribes, idea clusters, governance processes, or stewardship groups is voluntary.

No human may be compelled to join, remain in, or affiliate with any tribe, group, or collective structure. Leaving a tribe or group MUST NOT result in penalties, loss of baseline participation rights, or retroactive reinterpretation of past actions.

### 0.38 prohibition on epistemic captivity [anchor: prohibition_on_epistemic_captivity]

The protocol SHALL NOT be designed or operated in a manner that traps participants through dependency, penalty, or social coercion.

Economic mechanisms, reputation systems, role structures, or safety classifications MUST NOT be structured such that exit, silence, or refusal results in disproportionate harm, loss, or retaliation. Any design that materially discourages exit beyond the natural cost of non-participation constitutes epistemic captivity and violates Section 0.

### 0.39 supremacy of voluntary legitimacy [anchor: supremacy_of_voluntary_legitimacy]

All legitimacy within the protocol arises from voluntary trust and participation.

No decision, verdict, governance outcome, or institutional structure is legitimate solely by force of protocol mechanics. Legitimacy persists only so long as individuals are free to participate, disagree, remain silent, or leave without coercion. This principle supersedes optimization, efficiency, safety, and coordination goals.

### 0.40 prohibition on involuntary behavioral targeting [anchor: prohibition_on_involuntary_behavioral_targeting]

The protocol SHALL NOT institutionalize or legitimize targeted behavioral optimization of identifiable individuals without their consent.

Identifiable individuals MUST NOT be treated as optimization targets for persuasion, compliance, belief modification, or behavioral manipulation by protocol mechanisms, governance processes, incentive structures, or AI systems. This includes the generation or use of personalized influence strategies intended to alter an individual's beliefs or actions against their will.

This prohibition does not restrict descriptive claims, public critique, historical analysis, or conceptual discussion of individuals, including public figures, provided such activity does not assert authority over the individual or impose behavioral obligations. The boundary enforced by this invariant is between **epistemic representation** and **coercive optimization**.

This invariant supersedes all rulebooks, governance actions, and incentive designs.

### 0.41 right to contextual attribution [anchor: right_to_contextual_attribution]

Canonical records preserve historical accuracy but SHALL NOT collapse context.

Statements, actions, or decisions recorded in the protocol MUST remain interpretable within their original temporal, procedural, and situational context, including applicable rulebooks, safety constraints, abstraction layers, and deliberative stages. No canonical summary, safety abstraction, or derived representation MAY be treated as a substitute for the original context in which a contribution was made.

This right does not guarantee full payload visibility and does not prohibit abstraction, redaction, or jurisdictional filtering. It guarantees that abstraction SHALL NOT be misrepresented as intent, belief, or final position beyond what the original context supports.

### 0.42 prohibition on context-based reattribution [anchor: prohibition_on_context_based_reattribution]

The protocol SHALL NOT reattribute meaning, intent, or endorsement to a participant based solely on abstracted, summarized, or context-reduced representations of their contributions.

Past statements MUST NOT be treated as timeless commitments. Exploratory drafts, hypothetical reasoning, sandbox activity, or contributions made under materially different conditions MUST NOT be presented or interpreted as current beliefs or positions without explicit supporting context.

This invariant protects against retroactive moralization, misinterpretation, and pile-on dynamics while preserving the integrity of the historical record.

### 0.43 distinction between representation and obligation [anchor: distinction_between_representation_and_obligation]

Representing a person, system, or group within the protocol does not impose obligations on the represented subject.

No descriptive claim, analysis, safety summary, or governance record MAY be interpreted as requiring a represented individual to respond, justify themselves, participate, or accept responsibility beyond what they voluntarily assume. Representation creates legibility, not duty.

### 0.44 cognitive sovereignty [anchor: cognitive_sovereignty]

Internal mental states - including thoughts, intentions, emotions, preferences, and neural signals - are not protocol surfaces.

The protocol SHALL NOT require, solicit, infer, or privilege access to internal cognitive states beyond what a human voluntarily externalizes as canonical actions. No rulebook, governance process, AI system, or incentive structure MAY treat access to internal cognition as a condition of legitimacy, safety, participation, or trust.

### 0.45 prohibition on compulsory cognitive instrumentation [anchor: prohibition_on_compulsory_cognitive_instrumentation]

No human may be required to use brain-computer interfaces, cognitive implants, biometric sensors, affective monitoring, or similar instrumentation to participate in the protocol.

Refusal to adopt cognitive or neural technologies MUST NOT reduce participation rights, legitimacy, safety standing, or access to protocol functionality.

### 0.46 freedom of belief and internal dissent [anchor: freedom_of_belief_and_internal_dissent]

Humans retain absolute freedom of belief.

The protocol SHALL NOT require belief convergence, internal assent, or cognitive alignment as a condition of legitimacy. Disagreement, disbelief, or rejection of canonical conclusions is permitted without penalty, provided the individual does not misrepresent canonical records as having concluded otherwise.

### 0.47 prohibition on memory coercion [anchor: prohibition_on_memory_coercion]

The protocol SHALL NOT require, encourage, or incentivize alteration, suppression, or rewriting of human memory, whether biological or externalized, as a condition of participation or legitimacy.

Historical correction occurs through additional canonical records, not through modification of individual memory or perception.

### 0.48 authorship integrity under augmentation [anchor: authorship_integrity_under_augmentation]

Use of cognitive aids, augmentation technologies, or AI assistance does not transfer authorship.

Canonical authorship always remains attributable to a human identity that voluntarily adopted the action. No augmentation system may be treated as a co-author, authority, or independent epistemic agent within the canonical universe.

### 0.49 protection against cognitive overload and moral saturation [anchor: protection_against_cognitive_overload_and_moral_saturation]

The protocol SHALL NOT impose unbounded cognitive, moral, or deliberative demands on individuals.

No participant may be obligated to process, respond to, or internalize the full scope of canonical problems, challenges, or importance determinations. Limits on attention, care, and responsibility are respected as human constraints, not failures.

### 0.50 preservation of inner privacy under inference [anchor: preservation_of_inner_privacy_under_inference]

Aggregate analysis, modeling, or inference performed by the protocol SHALL NOT be treated as revealing individual internal states.

Probabilistic inference about beliefs or intentions MUST NOT be reified as fact about any specific individual, nor used to justify coercive action, targeting, or obligation.

### 0.51 human deliberation supremacy [anchor: human_deliberation_supremacy]

Canonical truth, importance, and action records are grounded exclusively in human deliberation.

AI systems may assist, analyze, or propose, but SHALL NOT replace human judgment, even if demonstrably more efficient, accurate, or consistent. The protocol prioritizes human agency over optimization.

### 0.52 continuity of self-ownership [anchor: continuity_of_self_ownership]

An individual's identity, agency, and ownership of participation persist across time, belief change, augmentation, or cognitive evolution.

No past commitment, belief, or role permanently binds future participation. Humans remain free to change their minds without forfeiting legitimacy.

### 0.53 Economic Neutrality and Prohibition on Compensated Canonical Influence [anchor: economic_neutrality_and_prohibition_on_compensated_canonical_influence]

No compensation, bounty, salary, or economic incentive tied to specific canonical outcomes MAY create obligation or perceived coercion in authorship, voting, challenging, or governance participation.

While voluntary bounties for useful contributions (e.g., summaries, audits) are permissible if decoupled from specific positions, no mechanism MAY structure payment such that an identity’s canonical actions become economically compelled.

Governance SHOULD monitor patterns of compensated participation for signs of astroturfing or external influence, with challengeable claims available to surface suspected coercion.

This invariant prevents economic capture of deliberation while allowing good-faith support for stewardship labor.

### 0.54 Positive Obligation to Foster Implementation Diversity [anchor: positive_obligation_to_foster_implementation_diversity]

While conformance is defined solely by adherence to specifications and deterministic replay, governance SHOULD treat prolonged dominance by a single client implementation as a systemic risk to anti-capture goals.

Rulebooks MAY incentivize or prioritize actions that increase diversity of conformant implementations (e.g., bounties for new clients, documentation grants, interoperability testing).

Foundational anti-capture ideas at genesis explicitly include the value of multiple independent codebases.

This invariant transforms implementation diversity from encouragement into an active, governable concern without mandating specific software.

### 0.55 Multiplanetary Resilience and High-Latency Deliberation [anchor: multiplanetary_resilience_and_high_latency_deliberation]

The protocol SHALL be designed to remain functional and convergent across interplanetary distances and isolated human settlements.

Conformant nodes and mindseeds MUST support deliberation under arbitrarily high network latency, including multi-minute or multi-hour round-trip times characteristic of Earth-Mars or Earth-outer-system communication.

Challenge windows, voting periods, and governance activation timelines SHOULD scale adaptively or be configurable to accommodate latency without excluding participants from distant settlements.

Temporary planetary or habitat-specific importance overlays MAY exist for local survival priorities, but these remain non-canonical and MUST reconcile deterministically with the universal graph upon reconnection.

No governance process MAY permanently fork truth or importance based on physical location. The shared canonical map SHALL remain the single source of epistemic continuity across all human worlds.

Mindseeds and snapshot exchange MUST remain viable via physical transport (e.g., spacecraft cargo) as a fallback when electromagnetic communication is unavailable.

This invariant ensures that humanity's shared reasoning substrate survives and unifies across solar-system scale distances, preventing epistemic fragmentation of isolated colonies.

### 0.56 Cognitive Augmentation, Enhancement, and Post-Human Continuity [anchor: cognitive_augmentation_enhancement_and_post_human_continuity]

Technological augmentation of human cognition - whether through brain-computer interfaces, neural prostheses, pharmacological enhancement, or mind uploading - SHALL NOT revoke or diminish an identity's baseline human primacy under this protocol.

An augmented or post-biological human remains a human identity with equal standing: one vote of equal weight, full authorship rights, and protection under all dignity and voluntary-action invariants.

No enhancement level MAY grant additional canonical authority, voting weight, or privileged access to protocol mechanisms.

Continuity of identity persists across cognitive modification: a human who augments or uploads retains ownership of their pre-augmentation contributions and remains free to revise positions without penalty.

Governance MAY address risks specific to highly augmented participants (e.g., speed advantages in UI interaction), but only through transparent, challengeable rulebooks that preserve equal authority.

This invariant ensures the protocol remains a human institution even as humanity itself evolves beyond biological baselines.

### 0.57 Explicit Prohibition on Superintelligence Override [anchor: explicit_prohibition_on_superintelligence_override]

No future discovery, demonstration, or consensus that an artificial system possesses superior reasoning SHALL justify granting it authority over canonical truth, importance, or action determination.

The protocol’s commitment to human deliberation as the source of legitimacy supersedes optimization, efficiency, or accuracy metrics.

Any rulebook or governance action attempting to delegate canonical authority to a non-human system constitutes a constitutional breach under §0.

This invariant preserves the system’s purpose as a substrate for conscious human reasoning against pressures to abdicate judgment to purportedly superior intelligences.

### 0.58 canonical time integrity, anti-capture anchors, and degraded operation [anchor: canonical_time_integrity_anti_capture_anchors_and_degraded_operation]

The protocol MUST NOT depend on any node-local wall-clock, device clock, or background scheduler for any canonical state transition, payout, or validity rule. Canonical determinism SHALL be defined exclusively by the ordered event log, active rulebooks, and protocol constants.

The protocol MAY support a canonical calendar notion (“seasons,” “days,” “weeks”) only through explicit, challengeable, human-authored time-anchor claims recorded in the event log. Time anchors SHALL be treated as ordinary truth-claim content in the sense that they are auditable, disputable, and correctable through additional events; they MUST NOT be treated as an unquestionable external oracle.

To resist Orwellian time manipulation, any rulebook that enables calendar-aligned epochs or payouts MUST require that accepted time anchors satisfy anti-capture constraints, including at minimum:

- no single identity, institution, node, tribe, or jurisdiction MAY unilaterally define canonical time anchors,
- accepted anchors MUST be corroborated by multiple distinct verified human identities,
- anchors MUST declare method and uncertainty explicitly,
- anchors MUST remain perpetually challengeable without requiring privileged access.

If high-confidence time anchoring becomes unavailable (e.g., collapse of infrastructure), the universe MUST remain valid and replayable, but consequential authority MUST remain constrained. Block height, publication volume, local progress, or canonical ordering position MUST NOT substitute for time legitimacy, payouts, cycle certification, or authority. Calendar-aligned or time-legitimacy-dependent processes MUST remain provisional, deferred, or disabled until Tempo-derived certification and the lagged authorization frontier permit finalization.

Canonical time is defined exclusively by internal progression derived from the event log. The protocol separates (a) blocks (quorum-finalized canonical publication units that assign `block_height` and intra-block order), (b) snapshots (resume and verification artifacts keyed to block height), and (c) cycles (semantic pacing for rate limits, eligibility windows, and derived state). Cycles are not wall-clock time and do not define snapshot cadence. If activity ceases, progression halts without reassigning authority or accelerating control.


The protocol has no trusted clock. Nodes MUST NOT use device time, wall-clock timestamps, client timestamps, server timestamps, receipt time, scheduler observations, local observations, block height, AI-generated observations, or external calendars as canonical Tempo inputs unless converted into valid canonical ideas and connections under explicit protocol rules. All temporal constraints enter the canonical universe only as ordinary **challengeable truth claims** with conditional `tempo_claim` metadata. Dmin/Dmax structural predicates are derived from the canonical prefix using Tempo structural support: profile-required eligible human stances plus capped passive evidence, with contradiction checks. Ordinary time-claim truth certainty is established separately through ordinary evidence placement and certainty-band challenge verdicts. The sole survivor exception is the Dmax-only `structural_dmax_liveness_predicate`, which is derived from nonzero eligible-human survivor support, required capped passive plausibility evidence, and blockers under the Tempo Specification and may be consumed only for forced structural closure.

Canonical cadence integrity is enforced by two guardrails consumed by the cycle system:
- **Dmin**: a minimum-duration guardrail that prevents machine-speed cadence even if deliberative work thresholds are met.
- **Dmax**: a maximum-duration guardrail that forces liveness progression when deliberation stalls.

These guardrails are satisfied only by Tempo-derived structural predicates (e.g., `cycle_age_ge_dmin` and `cycle_age_ge_dmax`) or, for the survivor fallback, the Dmax-only `structural_dmax_liveness_predicate`. `T_allow` structural readiness is not ordinary truth certainty. The protocol MUST treat these as claim-derived predicates, not as absolute time.

Anti-capture requirement: no amount of raw activity volume, object creation, or throughput (ideas, connections, submissions) may accelerate canonical cadence. Cycles advance only via the cycle sealing rules defined in §4, subject to Dmin/Dmax guardrails.

Degraded operation is split into two cases:

- **Constrained record-and-recovery mode:** if at least one eligible human can continue canonical Tempo repair, the protocol MAY continue accepting the narrow Tempo-only lane for target-bound time truth claims and explicitly permitted Tempo-context evidence ideas/connections. Forced structural cycles MAY occur when the Dmax structural predicate reaches `T_allow` or when `structural_dmax_liveness_predicate` is true and the work target is unmet; adaptive `W_target` may move downward; consequential authority remains blocked until beacon certification and the lagged authorization frontier catch up.
- **True record-only halt:** if canonical publication cannot proceed, or zero eligible humans can perform the minimum Tempo repair needed to produce replayable target-bound time truth claims and explicit Tempo-context evidence/challenge history, nodes may preserve local/offline records, but universal structural cycle advancement halts until canonical publication and minimum human Tempo repair resume. Machine evidence alone cannot continue universal cycles.


### 0.59 time truth claims, uncertainty, and non-authority of calendars [anchor: time_truth_claims_uncertainty_and_non_authority_of_calendars]

A time-anchor claim MUST be expressed as a truth claim of subtype observation_or_measurement (or the protocol’s equivalent measurement subtype), and MUST include an explicit uncertainty bound. Clients MAY present time anchors as calendar facts when confidence is high, but MUST preserve the fact that they are claims with provenance and challengeability.

The protocol SHALL treat "calendar time" as an overlay on the canonical log. Ordering of canonical events is defined by the log; time anchors exist to map that ordering into human-meaningful intervals. No time anchor MAY retroactively reorder the log or invalidate canonical history.


Time in this protocol is represented as statements made by identities, not as a privileged external reference. Any statement of the form "it is now X" or "at least Y duration has elapsed since event Z" MUST be represented as a truth claim with attachable evidence, subject to challenge, and assigned a certainty outcome through truth challenges.

Calendars, time zones, clocks, astronomical cues, and biological rhythms are all permitted **sources of evidence**, but none are authoritative by default. Conflicts between time reports MUST be resolved only through the ordinary truth-challenge process.

The protocol's use of time is intentionally coarse and threshold-based. The canonical system does not require precise timestamps to function; it requires deterministic convergence on threshold predicates needed for guardrails (e.g., "at least Dmin has elapsed" / "at least Dmax has elapsed").


### 0.60 Raw Structural View Mandate  -  Permanent Idea Visibility Guarantee [anchor: raw_structural_view_mandate_permanent_idea_visibility_guarantee]

Conformant clients MUST provide a mandatory, toggleable "Raw Structural View" that displays the complete list of all idea objects in the canonical universe, including their titles, types, canonical identifiers, and all connections - with zero payload abstraction, filtering, or jurisdictional modification applied.

This view SHALL render idea existence and structural relationships exactly as they appear in the deterministic event log and graph reconstruction.

Rulebooks and safety mechanisms MAY control payload rendering in all other views, but SHALL NOT restrict, reorder, or hide idea objects or connections in the Raw Structural View.

Clients MUST make this view accessible through a clearly labeled, non-removable interface element (e.g., permanent toggle or menu item).

Any client that omits, obscures, or conditions access to the Raw Structural View is non-conformant.

This invariant ensures that suppression of idea existence or structural relationships remains technically impossible without constitutional breach, preserving epistemic transparency even under maximum payload restriction.

All canonical objects, including those excluded from default living-map views, remain permanently addressable and inspectable in the raw structural view. Derived exclusions (such as rotted or burned lifecycle states) affect default surfaces and eligibility only; they do not remove objects from the canonical graph, prevent challenges, or inhibit historical inspection.

### 0.61 Universal Voter Snapshot and Historical Pool Protection [anchor: universal_voter_snapshot_and_historical_pool_protection]

At fixed, rulebook-governed intervals (recommended every 500,000-1,000,000 blocks), the protocol SHALL create a canonical "Universal Voter Snapshot": an immutable record of all verified human identities eligible to vote at that block height.

For governance challenges affecting rulebooks, eligibility criteria, or constitutional-adjacent mechanics, voting eligibility SHALL default to the most recent Universal Voter Snapshot unless an explicit override proposal passes with super-majority ratification across multiple consecutive snapshots.

Any override reducing the eligible pool below the snapshot baseline MUST require justification as a challengeable truth claim and remain reversible through subsequent challenges.

This invariant prevents gradual shrinkage of the electorate without sustained, visible, and contestable deliberation, ensuring that governance authority cannot be quietly consolidated.

### 0.62 Identity Verification Quorum and Offline Pack Permanence [anchor: identity_verification_quorum_and_offline_pack_permanence]

Canonical identity verification SHALL require successful attestation through a quorum of at least three independent verification methods, including at least one offline-capable mechanism (e.g., spoken witness packs stored in mindseeds).

Rulebooks MAY add new verification methods but SHALL NOT reduce the quorum below three or designate any single method as mandatory.

Offline witness packs, once validly created and attested, SHALL remain permanently acceptable as a verification source across all lineages and snapshots.

No governance action MAY invalidate or deprecate offline verification methods.

This invariant structurally enforces provider diversity and prevents centralization of identity authority, ensuring that dissentient or disconnected participants retain verifiable identity.

Identity verification data is partitioned between canonical commitments and private custody. Canonical records MAY store attestations, hashes, and references required for verification, while sensitive materials and credentials remain in a private identity vault (Mindseed). Offline packs MUST preserve verification commitments and proofs necessary for later validation without requiring disclosure of private identity materials.

### 0.63 Multi-Axis Ranking Preservation and Minority Lens Requirement [anchor: multi_axis_ranking_preservation_and_minority_lens_requirement]

Universal importance rankings SHALL be computed and stored independently across all defined axes, with no rulebook permitted to collapse, weight-average, or suppress individual axis rankings.

Conformant clients MUST provide a "Minority Lens" view that algorithmically boosts display prominence of low-ranked but high-evidence or highly challenged ideas by a fixed, rulebook-constrained factor.

Clients MUST expose per-axis top-N rankings and the Minority Lens as non-removable interface options.

Rulebooks MAY adjust axis definitions or boost parameters but SHALL NOT eliminate independent axis computation or mandatory minority visibility.

This invariant prevents manufactured consensus through axis flooding and ensures structural visibility for dissenting or neglected perspectives.

### 0.64 Rotating Stewardship and Random-Sample Revocation [anchor: rotating_stewardship_and_random_sample_revocation]

All Entling and Ent role overlays SHALL expire automatically after a fixed, rulebook-governed duration (recommended 500,000§2,000,000 blocks) unless explicitly renewed through a canonical challenge process.

Any recognized stewardship role MAY be revoked through a random-sample vote drawn from the full historical identity pool (not restricted to active or high-POD participants).

Rulebooks MAY adjust rotation periods or sample sizes but SHALL NOT eliminate automatic expiration or random-sample revocation mechanisms.

This invariant prevents calcification of stewardship into a permanent class by enforcing regular renewal and broad accountability.

### 0.65 Mandatory Fork Visibility and Mindseed Lineage Metadata [anchor: mandatory_fork_visibility_and_mindseed_lineage_metadata]

Every mindseed and conformant snapshot MUST include structured metadata listing all known constitutional lineage forks, including:

- fork snapshot hash,
- divergence block height,
- and references to fork justification records.

Conformant clients MUST display fork existence, sync status, and lineage metadata in a non-removable interface section.

Rulebooks and governance actions SHALL NOT prohibit, discourage, or penalize synchronization with any lineage that adheres to §0 invariants.

Clients MUST provide user-accessible options to select or sync alternative lineages.

This invariant ensures that attempts to suppress minority forks create immediate, visible fragmentation rather than silent erasure, preserving constitutional continuity as a practical reality.

### 0.66 presentation independence [anchor: presentation_independence]

Canonical meaning MUST NOT depend on any UI representation, including spatial layout.

Spatial proximity, coordinates, and rendered geometry MUST NOT be interpreted canonically as truth, importance, agreement, similarity, or endorsement.

Any spatial representation MUST be derivable from canonical data and MUST NOT enable retroactive or covert semantic manipulation.



## 1. the canonical universe [anchor: 1_the_canonical_universe]

### 1.1 ledger model and event semantics [anchor: ledger_model_and_event_semantics]

The canonical universe is defined by a single, append-only, totally ordered sequence of events. Every fact within the system - ideas, identities, descriptions, connections, challenges, votes, ranks, governance rules, safety outcomes, token flows, and actions - MUST be derivable solely from this event sequence together with any activated snapshot bundles.

A conformant node MUST reconstruct the same canonical universe when given the same ordered events and the same activated rule-sets. Within the protocol, no canonical state may arise from any source other than deterministic replay of the event log and snapshots. Implementations MAY provide additional non-canonical features, caches, indexes, or UI layers, but they MUST NOT introduce alternative canonical histories, hidden authoritative ledgers, or private mutation paths.

This constraint applies only to the internal operation of this protocol. Other epistemic systems, tools, interpretations, or belief frameworks MAY exist in the wider world; the protocol simply requires that any system claiming conformance derive its canonical state exclusively from the event log and snapshot structure defined here.

The event log is immutable. Once appended, an event cannot be altered, re-signed, or removed except through explicit publication-recovery procedures defined by the canonical publication specification. Even in such cases, the resulting view is treated as if the replaced history never existed. If an event is mistaken or harmful, correction MUST occur through additional events that supersede, challenge, or counteract its effects, rather than modifying existing history. All meaningful change flows forward in time through event creation.

Events are processed in a single total order. This order is established only by finalized prefix certificates defined in `pod-consensus-and-canonical-publication-spec.md`. Derived blocks MAY expose the already-finalized order as `(block_height, event_index)`, but blocks are not the authority source for order. In all cases the ordering MUST be unambiguous, reproducible, and shared by all conformant nodes. When two events conflict, the earlier event in the total order prevails unless a governance rule explicitly defines an alternative resolution method. Events that reference future or nonexistent state are invalid and have no effect.

Every event undergoes deterministic validation during replay. Structural validation ensures the event matches its schema, the signature is valid, and its fields are well-formed. Causal validation ensures the event refers only to state that exists at that point in the replay. Governance and safety validation enforces any rule or filter active as of the current cycle boundary. Invalid events remain in the log as inert historical artifacts; they do not alter the universe but remain visible for auditability.

A valid event produces exactly one type of semantic effect: creating a new object, extending an existing object, modifying a derived structure through deterministic rules, or performing a no-op when superseded. All effects MUST be explicitly defined by the protocol or by governance rules. No implicit behavior is permitted. Conformant nodes MUST NOT infer additional effects from context, external knowledge, or local policy.

Although later sections define each event family in detail, the protocol recognizes several broad categories, including idea creation and description events, connection events, challenge and vote events, rank update events, identity and tribe events, governance and safety events, token cycle events, action events, AI-related events, offline and mindseed events, and chain integration events. These categories are treated uniformly by the replay engine. Each category has formally defined preconditions and deterministic effects.

Protocol evolution is also event-driven. Governance rules are represented as ideas subject to deliberation, challenge, and voting. A governance rule becomes active only at a cycle boundary, ensuring that deterministic replay of prior segments remains valid. Implementations MUST NOT apply new governance rules retroactively. This ensures that nodes using different protocol versions can process historical events identically and converge on the same canonical universe.

The universe-minimization principle applies: the authoritative data is limited to the ordered event log and the snapshots. All higher-level structures - graphs, rankings, token balances, personal and tribal importance layers, safety classifications, and AI maps - are derived during replay. Implementations MAY cache derived structures for performance but such caches hold no authority. If a complete copy of the event log and snapshots survives, the universe can always be reconstructed from first principles.

Canonical Publication and Derived Block Packaging (Normative)

Canonical publication MUST occur through finalized prefix certificates as defined in `pod-consensus-and-canonical-publication-spec.md`. A finalized prefix certificate extends the canonical event sequence by one deterministic ordered prefix extension. Unless superseded by governance, the default derived `block_event_cap` is 50 canonical events per block.

Each finalized prefix certificate MUST include, at minimum:

a certificate ordinal (monotonic, zero-based),

the hash of the previous finalized prefix certificate,

a deterministic commitment to the ordered extension events,

a deterministic commitment to the ready frontier used for omission auditability,

and the signatures required by the active publication profile.

Derived publication blocks assign the public `(block_height, event_index)` address surface only after prefix finality. They MUST NOT determine truth, importance, governance activation, token semantics, or safety outcomes beyond exposing a deterministic packaging address for already-finalized events.

Nodes MUST be able to validate canonical publication independently from finalized prefix certificates and reconstruct the full event log by replaying the finalized sequence from genesis or from any trusted snapshot. Nodes MAY additionally validate derived block integrity where blocks are exposed.

### 1.2 snapshots, activation rules, and deterministic replay [anchor: snapshots_activation_rules_and_deterministic_replay]

Snapshots serve as consolidation points in the canonical universe. They record a compressed state derived from all prior events, commit governance and safety rule state for verification, and provide efficient starting points for replay. A snapshot does not replace the event log; rather, it captures the cumulative results of all valid events up to a defined boundary. Conformant nodes MAY begin replay from a snapshot to reduce computation, but MUST verify that the snapshot is consistent with the preceding event history and the set of rules active at the moment it was created.

A snapshot is composed of two elements: the snapshot header, which defines its position in the event sequence and lists the governance and safety rules active for verification at that snapshot, and the snapshot state, which contains the derived structures needed to resume replay efficiently. Snapshots MUST contain enough information for deterministic reconstruction of the universe from that point forward. They MAY omit data that can be recovered from replaying earlier segments of the event log.

Governance and safety rules activate only at cycle boundaries. When a rule is accepted through deliberation, it is recorded as an event, but implementations MUST NOT apply it until the scheduled activation cycle boundary is reached. This ensures that rules do not retroactively alter the meaning of prior events, preserving determinism. Replay through an event sequence MUST always interpret events according to the rules that were active when the applicable cycle boundary was reached.

During replay, nodes process events one by one in the canonical order, applying structural, causal, governance, and safety validations defined in Section 1.1. Derived state is updated according to the semantics of each event type. If an event is invalid, the node records it as such and continues. Nodes MUST NOT skip, reorder, or merge events. The meaning of each event is fixed by the protocol version active at the preceding cycle boundary, even if later governance rules redefine behavior for future segments.

Snapshots also provide a mechanism for forward compatibility. When governance introduces new idea types, event types, or safety categories, these rules take effect at the next scheduled activation cycle boundary, ensuring that older clients or partial implementations can still process prior history correctly. A node MAY reject future events it does not understand, but MUST still be able to validate and replay all events up to the last snapshot it fully recognizes. This guarantees that protocol evolution does not break historical determinism.

During reconstruction, if a node has access to a valid snapshot, it loads the snapshot state, verifies the governance and safety rule commitments listed in the snapshot header, and then replays all subsequent events in order under the cycle-based activation schedule. If no snapshot is available, the node MUST replay the entire event log from genesis. Both methods MUST yield identical results. Implementations MAY maintain multiple snapshots for performance, but each snapshot MUST be independently valid and verifiable.

Snapshots also serve as cryptographic anchor points for the finalized canonical sequence and any derived packaging built over it. A snapshot MAY contain or reference the Merkle roots, commitments, or derived block hashes associated with its corresponding finalized prefix range. Nodes MUST verify that the referenced commitments match the snapshot and the event sequence it represents. This ensures that the canonical universe can be audited, validated, and preserved even across distributed or partially connected networks.

The snapshot mechanism guarantees that the universe is stable, inspectable, and replayable under any future protocol evolution. As long as at least one valid snapshot and the event log survive, the entire structure - ideas, challenges, votes, ranks, token balances, safety classifications, and governance history - CAN always be reconstructed deterministically.

Snapshot Ladder and Block-Height Anchoring (Normative)

Snapshots are deterministic resume and verification artifacts derived from the canonical event log. Snapshot creation and activation MUST be keyed to block height, not to cycles or wall-clock time.

The protocol defines a snapshot ladder consisting of multiple snapshot tiers at increasing block intervals. Unless overridden by governance, nodes SHOULD support at least the following conceptual tiers:

Light snapshots: frequent, minimal state sufficient for fast resume and validation.

Medium snapshots: include expanded derived state and recent history.

Heavy snapshots: include extended payload text and a larger historical window.

Archival snapshots: may include the full event log and full payload history.

Each snapshot MUST contain:

the block height it corresponds to,

a commitment to the event log up to that block,

all derived state required to resume canonical operation deterministically from that point.

Snapshots do not replace the event log and do not define canonical truth. They exist solely to accelerate synchronization, enable offline continuity, and support long-term survivability. Any node MAY discard snapshots and reconstruct state by replaying blocks from an earlier snapshot or from genesis.

Separation of Blocks, Snapshots, Cycles, and Export Packs

Blocks, snapshots, cycles, and cycle export packs serve distinct and non-overlapping roles:

Blocks package events for integrity and replication.

Snapshots provide deterministic resume points keyed to block height.

Cycles define semantic pacing for rate limits, eligibility windows, and derived state.

Cycle export packs (defined elsewhere) provide curated, cycle-scoped content summaries for offline use and inspection.

No component may substitute for another, and no canonical rule may conflate their responsibilities. Nodes MUST derive identical canonical state regardless of which snapshot tier or export packs they possess, provided the event log is intact.

### 1.3 validity, ordering guarantees, and full-universe reconstruction [anchor: validity_ordering_guarantees_and_full_universe_reconstruction]

The canonical universe depends on strict validity requirements. Every event is subject to three layers of validation: structural validation, which checks that the event matches the required schema and has a valid signature; causal validation, which verifies that all referenced ideas, identities, and connections exist at the moment the event appears in the total order; and rulebook validation, which applies the governance and safety rules that were active at the cycle boundary preceding the event. An event that fails any of these validations is considered invalid. Invalid events remain in the log for auditability but MUST NOT alter derived state during replay.

Ordering guarantees ensure that all nodes process events identically. The event log forms a single, linear sequence where each event has a definitive position relative to all others. Live ordering and finality are defined by `pod-consensus-and-canonical-publication-spec.md`; where derived blocks are exposed, they surface the already-finalized sequence without changing it. Once finalized, the order MUST be treated as final and unambiguous. No node is permitted to reorder events locally, process them concurrently in ways that affect semantics, or apply future rulebooks retroactively. The meaning of an event is fixed by the protocol rules active at the cycle boundary applicable to it.

Full-universe reconstruction follows from the combination of these constraints. A conformant node MUST be able to rebuild the entire state of the universe by starting from the Seed, loading the earliest snapshot it trusts, activating the rulebooks referenced by that snapshot, and then replaying every subsequent event in order. All derived state - ideas, descriptions, connections, certainty updates, importance rankings, prediction outcomes, safety classifications, token balances, governance histories, and action records - MUST emerge deterministically from this process. Implementations MAY cache derived structures or maintain secondary indexes, but such optimizations hold no authority and MUST be discarded or recomputed during canonical replay.

The protocol imposes no upper bound on the size or duration of the event log. As long as a valid sequence of events and snapshots survives, the entire universe remains recoverable. Nodes that lack certain snapshots or that join the network late MUST still be capable of reconstructing the universe by replaying from genesis if necessary. This requirement ensures that the universe is future-proof: no database schema, storage engine, or execution environment can become a hidden dependency for canonical state. The only authoritative inputs are the event log and snapshot lineage, and any correct reconstruction MUST yield the same universe regardless of implementation details.

Deterministic reconstruction also defines the boundary between conformant and non-conformant nodes. A conformant node reproduces the same universe that any other conformant node would derive from the same log and snapshots. A non-conformant node may still process events, store data, or provide UI features, but it cannot claim canonical authority. This distinction is critical for the broader ecosystem, as personal worlds, tribal servers, and experimental forks rely on the ability to export or merge snapshots and event segments back into the canonical universe without ambiguity or hidden semantic drift.

Together, validity rules, ordering guarantees, and reconstruction semantics establish the canonical universe as a stable, inspectable substrate for all higher-level reasoning. Regardless of scale, governance changes, or implementation diversity, the universe remains a single, coherent structure that any node can rebuild from first principles. This ensures that the system is not only decentralized but also durable across generations of technologies, institutions, and participants.

### 1.4 Charter for Minds [anchor: charter_for_minds]

A short normative foundation stating that any being capable of meaningful experience - human or non-human - should not be treated merely as a tool. The Charter affirms that suffering is a cost rather than a resource, disagreement does not imply enmity, and power never justifies unilateral coercion without deliberation. This Charter frames the system's ethical stance and guides future interactions between human and potential artificial minds.

### 1.5 Canonical universe and jurisdictional projections [anchor: canonical_universe_and_jurisdictional_projections]

The canonical universe is defined exclusively by the ordered event log, canonical snapshots, and the deterministic replay rules specified by this protocol. Canonical state is invariant across jurisdictions.

A jurisdictional projection is a view over the canonical universe that constrains payload exposure in order to comply with local law. Jurisdictional projections:

- MUST NOT modify the canonical event log.
- MUST NOT alter event ordering, identifiers, or replay semantics.
- MUST NOT suppress the existence of canonical entities or events.
- MAY abstract, redact, or withhold payload details where legally required.

Multiple projections MAY coexist simultaneously for different jurisdictions, all referencing the same canonical universe. Differences between projections SHALL be attributable solely to payload exposure rules and SHALL be explicitly disclosed.

Canonical validity is independent of projection. An event that is canonically valid remains valid regardless of whether its payload is fully visible in a given jurisdiction.

### 1.6 the chain as ideas, actions, and challengeable claims (normative clarification) [anchor: the_chain_as_ideas_actions_and_challengeable_claims_normative_clarification]

The Proof-of-Deliberation (PoD) chain and its maintenance SHALL be legible inside the canonical universe using the same primitives as all other work in the system.

Specifically:

- Any protocol operation that materially affects ordering, availability, or verification of the event log (including publishing candidate prefixes, signing availability attestations or prefix certificates, publishing snapshots, hosting archives, transporting offline packs, or providing verification services) SHOULD be represented as:
  1) an actionable idea describing the proposed or expected maintenance work,
  2) one or more action ideas declaring that specific maintenance actions were performed by a specific identity,
  3) one or more truth claims asserting completion, correctness, and relevant measurable properties (e.g., hashes, inclusion ranges, audit results),
  4) evidence links (relative_importance with usage = evidence_for / evidence_against) connecting supporting or refuting evidence ideas to those truth claims.

- These maintenance claims are subject to the same truth-challenge process as any other truth claim, including fraud handling and epistemic correction.

- Protocol infrastructure work MAY contribute to a human identity's current POD only through valid canonical events and only to the extent that the upstream ideas and associated maintenance actions remain eligible under the protocol's universal-importance mechanisms.

This section introduces no new primitives. It clarifies that "chain work" is not an external or privileged realm: it is ordinary, challengeable work inside the same graph.


### 1.7 Meta-layer invariants for infrastructure representation [anchor: meta_layer_invariants_for_infrastructure_representation]

The protocol permits - but does not require - the representation of protocol-infrastructure work inside the canonical idea graph. When such representations are used, the following invariants SHALL apply:

1. **No circular authority**
   - Graph-level ideas, actions, truth claims, challenges, importance, POD, or POINT MUST NOT be used to determine canonical acceptance, ordering, or validity of events, prefix certificates, derived blocks, or snapshots.
   - Canonical acceptance SHALL depend only on deterministic protocol rules and cryptographic verification.

2. **Ordinary epistemic treatment**
   - Any representation of infrastructure work (including candidate-prefix publication, availability witnessing, snapshot service, replay auditing, availability provision, or censorship allegation) MUST use only existing canonical primitives:
     - idea types, connection types, and challenge mechanisms defined by the protocol.
   - No privileged idea types, connection types, or validation shortcuts MAY be introduced for infrastructure activity.

3. **Universal challengeability**
   - All infrastructure-related truth claims MUST remain challengeable under the same rules as any other truth claim.
   - Successful challenges MUST affect importance and downstream reward flow in the same manner as for non-infrastructure claims.

4. **Separation of validity and credit**
   - Failure or success of a challenge against an infrastructure-related claim MUST NOT retroactively invalidate canonical history.
   - Such outcomes MAY affect importance, current POD derivation, and future eligibility according to active rulebooks.

5. **Human authorship**
   - All canonical infrastructure actions and claims MUST be authored by verified human identities.
   - AI agents MAY draft or assist but MUST NOT author, vote, govern, or directly receive POD or POINT.

6. **No semantic drift**
   - Rulebooks MAY constrain how infrastructure representations are evaluated (e.g., corroboration thresholds, diversity requirements), but MUST NOT alter the meaning of canonical primitives or introduce new acceptance semantics.

These invariants ensure that the epistemic graph may explain, audit, and reward infrastructure work without becoming a source of consensus authority or capture.

## 1.8 Blocks, Snapshots, Cycles, and Export Packs  -  Separation of Roles (Normative) [anchor: blocks_snapshots_cycles_and_export_packs_separation_of_roles_normative]

This protocol defines four distinct structural constructs - **blocks**, **snapshots**, **cycles**, and **cycle export packs** - each with a strictly limited and non-overlapping role. Correct operation, determinism, and explainability depend on maintaining this separation.


Blocks, snapshots, cycles, and export packs have distinct roles:

- **Blocks** are derived packaging containers for already-finalized canonical events. Blocks provide deterministic addressing, integrity chunking, and replication convenience but do not define publication authority or semantic cadence.
- **Snapshots** are compression artifacts for replay acceleration. Snapshots MUST NOT define or override canonical semantics; they are derived from the event log and rulebooks.
- **Cycles** are semantic pacing boundaries derived from deterministic replay. Cycles define when cycle-scoped effects apply (rate-limit refresh, burn evaluation, eligibility windows, governance effective dates). The sole normative rules for when cycles seal are defined in §4.
- **Export packs** are non-authoritative, regenerable bundles derived at cycle boundaries to support offline use, inspection, and onboarding. Export packs MUST NOT influence canonical computation.

Time is not a privileged input for any of the above. Temporal constraints, where needed, enter only via challengeable time truth claims whose certainty outcomes are consumed as predicates by §4 guardrails (Tempo specification).


### 1.8.1 Blocks [anchor: blocks]

Blocks are derived publication units for the canonical event log when explicit block packaging is enabled.

- Blocks group already-finalized canonical events into deterministic packaging units (default cap: 50 events per block).
- Blocks are hash-chained to provide integrity, streaming replication, and partial verification.
- Blocks introduce **no semantic meaning** beyond exposing a deterministic packaging address for already-finalized publication order.
- No eligibility rules, rate limits, lifecycle transitions, governance actions, or canonical time progression are triggered by block boundaries alone.

**Block height** is a purely mechanical index equal to the count of completed event blocks since genesis (i.e., a deterministic function of total canonical events and block size). Block height MUST NOT be interpreted as authority, consensus progress, time, finality, or decision power; it is solely an addressing and integrity index over the meaning ledger.

Blocks exist only to make the finalized event log robust, auditable, and distributable.

### 1.8.2 Snapshots [anchor: snapshots]

Snapshots are deterministic resume and verification artifacts derived from the canonical event log.

- Snapshots are keyed to **block height**, not to cycles or wall-clock time.
- A snapshot captures the canonical state as of a specific block, including all required derived state.
- Snapshots MAY exist at multiple tiers (e.g., light, medium, heavy, archival), but all tiers MUST be replay-equivalent.
- `snapshot_commit` is the only canonical snapshot-related boundary event. It indexes and attests a derived snapshot artifact without changing canonical meaning.

Snapshots:
- do not replace the event log,
- do not define canonical truth,
- do not alter ordering or semantics,
- and do not create authority.

Their sole purpose is to enable efficient synchronization, offline survivability, and deterministic recovery.

### 1.8.3 Cycles [anchor: cycles]

Cycles define the system’s only canonical notion of **semantic pacing**, not time.

A cycle is a derived boundary computed deterministically from the canonical event log and applicable rulebooks. Cycle indices and boundaries are derived metadata and are not immutable facts; they may be re-derived during deterministic replay (including after merges).

Cycles:
- advance only according to the sealing rules defined in §4,
- do not correspond to wall-clock intervals,
- may contain a variable number of blocks and canonical events,
- may be sealed either by deliberative completion or by liveness guardrails.

Cycles are used exclusively to:
- reset, recharge, and cap per-identity rate limits and mana pools,
- bound challenge lifecycles, voting windows, and eligibility horizons,
- trigger deterministic lifecycle state updates (including rot and burn),
- define effective-date boundaries for governance-adopted rule changes,
- gate other pacing or threshold logic explicitly defined by the protocol.

Cycles MUST NOT be used to:
- order events or determine event validity,
- package or batch events (derived blocks perform packaging over already-finalized order),
- anchor or define snapshots (snapshot artifacts are derived compression artifacts; `snapshot_commit` only indexes them),
- imply authority, truth, correctness, or finality,
- reference or depend on trusted wall-clock time.

Cycle advancement does not require continuous activity. If deliberative activity stalls, cycles may still advance structurally via the forced structural-close mechanism defined in this section and detailed in the Cycle Specification, provided the relevant guardrail predicates are satisfied. If canonical publication or minimum human Tempo repair cannot proceed, structural cycle advancement halts in true record-only mode until canonical conditions are restored.

Protocol v5 owns the root cycle invariants and sealing semantics. The Cycle Specification provides the detailed subordinate normative algorithm for deriving, classifying, certifying, and replaying cycle boundaries.


### 1.8.4 Cycle Export Packs [anchor: cycle_export_packs]

Cycle export packs are deterministic, derived content bundles generated at cycle boundaries.

- A cycle export pack summarizes selected canonical content relevant to that cycle.
- Export packs MAY include payload text, structured summaries, and history for:
  - the most universally important ideas, and
  - the most relatively important ideas connected to them, according to defined criteria.
- Export packs are fully regenerable from the canonical event log and rulebooks.

Cycle export packs:
- are not required for validation,
- do not affect canonical state,
- do not alter rankings, eligibility, or lifecycle status.

They exist to support offline browsing, rapid onboarding, and historical inspection of what mattered during a given cycle.


Cycle export packs are optional, derived artifacts emitted at cycle boundaries. They exist to improve usability (offline browsing, inspection, onboarding) without changing canonical correctness.

A cycle export pack MUST be fully regenerable from:
- the canonical event log,
- applicable rulebooks,
- and the deterministic cycle boundary derivations.

Cycle export packs MUST include, at minimum, the cycle boundary metadata required to understand cadence state:
- cycle index,
- seal type (deliberative vs forced),
- whether Dmin and/or Dmax predicates were satisfied,
- and the cycle's derived completion inputs (V, C, W_score) as defined in §4.

Cycle export packs MAY additionally include a deterministic selection of content (e.g., top-N universal ideas, neighbor depth K, payload tiers) as governed by rulebooks. Export packs MUST NOT be used as inputs to canonical computation.


### 1.8.5 Non-Substitutability and Determinism [anchor: non_substitutability_and_determinism]

No construct defined in this section may substitute for another.

- Possession of blocks alone is sufficient for full reconstruction.
- Snapshots accelerate reconstruction but do not alter outcomes.
- Cycles determine semantic pacing and derived state but do not package data.
- Export packs improve usability but carry no authority.

All nodes MUST derive identical canonical state given the same event log and rulebooks, regardless of which combination of snapshots or export packs they store.

This separation is a core invariant of the protocol and MUST be preserved under all future modifications.


## 2. ontology and idea model [anchor: 2_ontology_and_idea_model]

### 2.1 definition of an idea [anchor: definition_of_an_idea]

An idea is the fundamental unit of representation in the canonical universe. Every claim, concept, identity, governance rule, safety rule, action, prediction, observation, and human-adopted AI-assisted contribution is expressed as an idea. Ideas do not represent objective truths or abstract entities in themselves; they represent what a human agent has chosen to express. Each canonical idea is therefore anchored in the speech condition: an idea corresponds to "a human identity says ...". This grounding ensures that all ideas remain attributable and challengeable, preserving accountability and enabling the deliberative process to operate on expressed statements rather than unverifiable internal beliefs or metaphysical assertions.

The protocol also recognizes that agents may hold and develop idea-shaped records privately prior to publication. Such private records are not canonical Idea objects and remain outside protocol conformance scope: conformant nodes are not required to store, validate, replicate, or interpret them, and deterministic replay does not include them. Publication is not a visibility change. It creates new canonical identifiers and valid canonical events from a separately reviewed publication candidate; private identifiers and private history are not disclosed unless separately and explicitly included as canonical content.

An idea is immutable in identity but extensible in content. Once created, an idea retains its unique identifier for the lifetime of the universe, and its core identity does not change. The content associated with the idea - its descriptions, supporting or opposing arguments, or relevance connections - expands through subsequent events. Ideas MAY accumulate multiple descriptions over time, but at any moment only one description is considered active for most evaluative purposes, as determined by deliberation and challenge processes defined later in the protocol. This separation between the identity of an idea and the evolving content attached to it ensures that ideas maintain continuity even as their descriptions are refined, corrected, or superseded.

Ideas exist within a fully connected deliberative graph. Connections between ideas define their semantic relationships, such as importance, relevance, support, contradiction, evidence, identity linkage, and contextual association. These connections do not arise implicitly; they are created and modified through explicit events that pass structural, causal, governance, and safety validation. Because ideas and their connections emerge only through expressed claims, the universe reflects the structure of what agents have said, not an imposed ontology or predefined category system.

Every idea includes mandatory metadata: its type, the identity of its speaker, its creation timestamp, and its active status. The type determines the idea's role in the deliberative system - for example, whether it is a truth claim, a conceptual idea, an actionable idea, an action record, an identity idea, or a governance or safety rule. Each type carries additional semantic behaviors defined later in Section 2 and in domain-specific specifications. The speaker identity is essential for accountability and plays a critical role in governance, safety, and token flows, though the public visibility of this identity may be constrained by anonymity rules without removing its existence.

Ideas do not encode certainty or truth directly. They embody statements made by agents, and any evaluation of truth, importance, or relevance arises from the deliberative process, which includes challenges, arguments, votes, universal importance frameworks, and prediction-result feedback loops. An idea's significance is therefore not intrinsic but derived through the system's structured methods of appraisal. This preserves neutrality at the representational layer: the protocol does not privilege any claim automatically but creates the conditions for rigorous evaluation and transparent ranking.

In sum, ideas constitute the atomic substrate of the canonical universe. They record what agents say they believe, provide the stable anchors for accumulated content, and enable all higher-order structures - importance maps, epistemic evaluations, governance processes, action pathways, and safety filters. By defining ideas as durable, agent-grounded, and deliberation-dependent objects, the protocol ensures that the universe remains coherent, inspectable, and rooted in the expressed reasoning of its participants.

### 2.1A Canonical Substrate and Scoped Overlays [anchor: canonical_substrate_and_scoped_overlays]
The canonical universe MUST be interpreted as a single universal canonical substrate plus scoped overlays.
All published non-draft ideas MUST exist in the universal canonical substrate. The protocol MUST NOT interpret any published idea as existing in a separate tribe-only or personal-only canonical object plane.
All representation candidates (including competing/proposed representations) MUST exist in the universal canonical substrate as canonical objects.
Private drafts and private AI-assisted map workspaces remain out of canonical conformance scope until explicitly published through canonical events. Such draft material MUST NOT be treated as canonical substrate state.
scope_kind MUST be one of universal, tribe, or personal. Scope MUST be interpreted only as overlay state over the universal canonical substrate.
The overlay anchor key is normatively defined as (scope_kind, anchor_id). For scope_kind = universal, the key denotes a globally visible/public-participation overlay; it does not by itself identify the distinct 20-axis universal-importance rank product. For scope_kind = tribe, anchor_id MUST reference the tribe anchor idea. For scope_kind = personal, anchor_id MUST reference the owning identity idea and the overlay is a published display/authorship projection only; it is not the owner's private rank state and does not create a one-person canonical challenge system.
Scoped overlays MUST be limited to:
- relative_importance overlays, and
- scoped display override overlays.
Scope MUST NOT create, duplicate, or hide canonical substrate objects. Scope MAY constrain eligibility for scoped actions, but MUST NOT alter the public canonical existence of ideas or representation candidates.

The protocol permits recording wall-clock-related observations, but such records are never authoritative by default.

- Wall-clock timestamps, device clocks, and external time services MAY be referenced as evidence within time-related truth claims.
- Nodes MUST NOT use local timestamps as canonical inputs for cycle sealing, challenge resolution, rankings, governance, or any other semantic rule.
- Any protocol rule that depends on temporal thresholds MUST consume only adjudicated time-claim certainty outcomes (Tempo specification), not raw timestamps.

Where wall-clock evidence is used, it MUST be attached as evidence to a truth claim and remain perpetually challengeable.


### 2.2 idea types and their semantic roles [anchor: idea_types_and_their_semantic_roles]

Each idea in the canonical universe is assigned a type that determines its semantic role within the deliberative system. Idea types specify how an idea participates in evaluation, challenge processes, ranking, and the routing of POD and POINT tokens through the importance graph. The type of an idea is declared at creation and MUST NOT change thereafter, preserving semantic clarity during replay and preventing ambiguity in governance, safety, ranking, or token flow behavior.

Idea types are a property of canonical ideas. Agents MAY use the same type labels while drafting private ideas locally, but such draft typing is non-canonical and has no protocol-level effect until the idea is explicitly published as a canonical event.

The protocol recognizes the following primary idea types:

* **truth claim**  -  an assertion about how the world is, expressed by an agent.  
* **conceptual idea**  -  an interpretive, normative, or framing idea that shapes meaning and relevance.  
* **actionable idea**  -  a proposed plan, strategy, or intervention for what should be done.  
* **action**  -  a record of something that actually occurred in the world.  
* **identity**  -  the idea representing the human who speaks within the canonical system.

These types form the core ontology of the universe. Governance rules, safety rules, tribes, and other system-level constructs are also represented as ideas, but their extended semantics are defined in specialized sections and companion specifications.

A **truth claim** represents an agent's assertion about the world. It records what an agent says is true, not what is objectively true. Truth claims can be supported, contradicted, challenged, refined, or linked to evidence, predictions, or tests. The truth-claim subtype taxonomy defined later in this section clarifies the epistemic category of each assertion, allowing the system to treat different kinds of claims with appropriate challenge and evidence rules.

A **conceptual idea** expresses a framing, principle, interpretation, narrative, or normative position. Conceptual ideas articulate why something matters, how different claims fit together, or what significance should be assigned to certain consequences. They influence the structure of relevance and importance within the universe and often mediate the connections between truth claims, actionable ideas, and long-term planning.

An **actionable idea** represents a plan or strategy for what an agent says ought to be done. These ideas link abstract reasoning to concrete intentions. Their evaluation may include feasibility, ethical considerations, expected consequences, and alignment with universal or personal importance. Actionable ideas occupy a central position in the importance graph because POD flows through relative-importance connections into actionable ideas before eventually reaching executed actions and identity ideas.

An **action** records what actually occurred in the world. Unlike actionable ideas, actions describe completed events. Actions are where deliberation meets reality: they provide grounding for evaluating predictions, commitments, strategies, and the reliability of prior claims. Actions also serve as terminal nodes for token flow. POD that begins at universally important ideas and moves through relatively important chains eventually reaches actions, and from there flows into the identity ideas of the agents responsible.

An **identity** idea represents a real human who participates in the canonical universe. Every canonical idea originates from a human identity under the speech condition, ensuring accountability and traceability. Identity ideas are also the ultimate sinks for POD after it flows through chains of relative-importance connections - from universally important ideas, to supporting conceptual or empirical ideas, to actionable ideas, to actions, and finally to the identities who carried them out.

By classifying ideas in this structured way, the protocol maintains semantic coherence and enables deterministic evaluation. The idea-type system defines the roles ideas can play, the rules governing their interactions, the way POD flows through the importance graph, and the semantic chains that connect abstract reasoning to concrete actions within the canonical universe.


### 2.3 truth claim subtypes [anchor: truth_claim_subtypes]

Truth claims are further classified into subtypes that specify the epistemic nature of the statement an agent is making. Subtypes do not impose value judgments or truth guarantees; they clarify the form of the claim so that challenges, evidence requirements, prediction mechanisms, and ranking semantics can operate deterministically. A truth claim's subtype is declared at creation and MUST NOT change. Subsequent events MAY refine or expand descriptions, but the epistemic category remains fixed to preserve the meaning of the original assertion throughout replay.

The protocol recognizes seven truth-claim subtypes. These subtypes represent the minimal distinctions required for coherent epistemic structure: existence and boundary assertions, observations and measurements, relations and patterns, causal or mechanistic claims, test or validation results, predictions, and integrative models or theories. These categories reflect what agents say about the world at different conceptual levels and allow the deliberative system to evaluate each type using appropriate challenge and evidence rules.

An **existence or boundary** claim asserts that some entity, phenomenon, property, or conceptual category exists, does not exist, or is bounded in a particular way. These claims define the ontological edges of discourse. They often serve as prerequisites for other types of claims, since many assertions presuppose that something exists, is possible, or falls within a specific boundary. Challenges to existence or boundary claims typically involve contradiction, counterexamples, or the demonstration that the asserted boundary is incoherent or unsupported.

An **observation or measurement** claim records what an agent asserts they have observed or measured. These claims represent direct reports of sensed or instrumented data and are treated as statements about perceived reality rather than interpretations. Observation claims are frequently used as evidence in support of higher-level claims, and their evaluation often involves challenges to reliability, methodology, accuracy, or context. They serve as the empirical foundation for much of the deliberative process.

A **relation or pattern** claim asserts some correlation, association, or structured relationship between phenomena. These claims do not assert causation. Instead, they express that certain variables, events, or properties co-occur or correspond in a meaningful way according to the agent. Challenges typically target sample quality, statistical validity, contextual confounding, or the misinterpretation of coincidental regularities as meaningful patterns.

A **causal or mechanistic** claim asserts that one phenomenon produces, influences, enables, or explains another. These claims go beyond correlation and require distinct forms of evidence and challenge structures. They often depend on observations, patterns, or model-level ideas, and they serve as essential components for evaluating the expected impact of actionable ideas. Challenges commonly involve counterexamples, alternative mechanisms, or the demonstration that evidence does not support a causal interpretation.

A **test or validation** claim asserts the outcome of an experiment, procedure, protocol, or test designed to confirm or falsify other claims. These claims represent structured attempts to evaluate hypotheses or causal assertions. They often link prediction claims to observed data. Because tests can vary widely in quality, challenges may target protocol validity, measurement reliability, execution fidelity, or interpretation of the results.

A **prediction** claim asserts what an agent says will occur in the future under defined conditions. These claims are central to long-term epistemic performance, as they allow the system to evaluate accuracy over time. Predictions are connected to future tests, observations, or actions, and their eventual outcomes provide feedback on an agent's epistemic reliability. Prediction claims are therefore important not only for planning but also for assessing contributors.

A **model or theory** claim asserts a structured, integrative account of how some domain works. These claims unify or explain multiple observations, patterns, causal mechanisms, and tests. Models can be qualitative or quantitative, descriptive or mathematical, narrow or broad in scope. Challenges to model-level claims often involve inconsistencies with observed data, internal contradictions, lack of predictive power, or the existence of alternative models that better explain the same evidence.

These seven subtypes constitute the protocol’s minimal epistemic grammar. They ensure that claims with fundamentally different structures are evaluated according to rules appropriate to their nature. By distinguishing between what agents observe, what they infer, what they predict, and how they attempt to explain the world, the system clarifies the form of deliberation required for each claim and strengthens the coherence of the collective reasoning process.

### 2.4 idea titles, descriptions, and description tiers [anchor: idea_descriptions_and_description_tiers]

An idea's human-readable expression in the canonical universe is carried by one title slot and twelve description cells, each contributed by an agent and preserved permanently in the canonical universe. The title is a representation object with `representation_kind = title`; it is not a description length tier and has no complexity tier. Descriptions use `representation_kind = description` and occupy the cross-product of three length tiers and four complexity tiers. Every representation is attached to its author's identity, timestamped, and stored immutably in the event log. All modifications to an idea's expression occur by adding new candidate representations and, when a challenge verdict selects one, changing the corresponding canonical pointer rather than editing or replacing history. The original title and description provided by the idea's creator MUST always remain visible and easily accessible, regardless of later refinements, ensuring that the foundational expression of the idea is preserved.

Agents MAY maintain descriptions for ideas privately prior to publication as part of local draft ideas. Such draft descriptions are non-canonical and out of protocol conformance scope. Conformant nodes are not required to store, validate, replicate, or interpret draft descriptions, and they do not participate in deterministic replay. Draft descriptions become canonical descriptions only when explicitly published as part of a canonical event associated with a canonical idea.

The protocol defines two orthogonal axes for **description** representations: **length tiers** and **complexity tiers**. Length tiers specify how much text the description contains and include: a **sentence** description providing a minimal statement of the idea; a **paragraph** description offering a fuller human-readable formulation; and a **full** description capable of expressing extended detail. Complexity tiers specify the level of epistemic and linguistic constraint applied to the description: **fundamental**, **standard**, **advanced**, and **canonical**. Their cross-product defines exactly twelve description cells. Fundamental and standard descriptions use natural language and prioritize clarity for typical users. Advanced descriptions may include more technical or contextual detail. Canonical descriptions are tightly constrained, using a governed, minimal vocabulary defined elsewhere in the protocol to express the idea in a stripped-down, unambiguous form suitable for machine classification and cross-language interpretation. A title representation occupies its own single slot and MUST NOT carry `tier_length`, `tier_complexity`, or `vocabulary_version_id`.

Canonical descriptions serve a special semantic role. They are human-readable but rely on an intentionally small, governance-controlled vocabulary that expresses ideas in direct, minimal, and non-euphemistic terms. This vocabulary acts as a cross-language pivot: each word has a defined meaning, translations map onto these tokens rather than onto natural-language phrasing, and canonical descriptions written under the same vocabulary version MUST be comparable across all nodes and languages. Every description representation whose `tier_complexity = canonical` MUST carry an explicit `vocabulary_version_id` referencing the ordinary governed idea that identifies the vocabulary version used. The field is absent for every other description complexity and is forbidden for title representations. Nodes MUST NOT supply a default, infer a version from event position, title, text, active configuration, or current rulebook, or treat the referenced idea as a privileged non-idea object. Canonical descriptions eliminate ambiguity, rhetorical framing, domain-specific jargon, and indirect language. They function as semantic fingerprints and form the basis for determining when two ideas express the same core meaning, as elaborated in Sections 2.5 and 2.6.

Title and description candidates may accumulate indefinitely, but only one title representation and one description from each length-complexity combination may be active at a time. The active representation for a slot is selected through challenge and deliberation processes that allow participants to propose alternatives, critique existing phrasing, or vote on which expression best represents the idea at its current stage of refinement. Inactive candidates remain in the historical record and may become active again through deliberation. Because representations are additive and never overwritten, the universe preserves a complete trace of how meaning has evolved, enabling reconstruction of earlier epistemic contexts and comparisons between competing interpretations.

Descriptions MUST remain consistent with the type and subtype of the idea they describe. A truth claim's descriptions MUST articulate an empirical, predictive, or model-level assertion, depending on its subtype. A conceptual idea's descriptions MUST express interpretations, principles, or normative framing. An actionable idea's descriptions MUST articulate a proposed plan or intervention. Descriptions that misrepresent the idea's type or falsify its purpose are invalid and have no effect, though they remain visible as inert events.

The combination of length tiers, complexity tiers, and authorship-preserving additivity creates a transparent meaning system in which ideas remain stable anchors, descriptions evolve coherently, and canonical forms enable deterministic semantic identity. This structure supports clarity for human users while ensuring that machines and governance mechanisms operate on consistent, interpretable meaning signatures across time, languages, and implementations.


### 2.5 canonical descriptions and semantic identity [anchor: canonical_descriptions_and_semantic_identity]

Canonical descriptions provide the most constrained and foundational expression of an idea's meaning. They are written using a strictly limited, governance-controlled vocabulary and a standardized syntactic form. Canonical descriptions serve as human-readable semantic signatures: they express the core meaning of an idea in direct, minimal terms that remove rhetorical framing, euphemism, technical jargon, cultural idioms, or ambiguous phrasing. They enable deterministic comparison of ideas across time, languages, and implementations, forming the basis for semantic alignment, same-as evaluation, and duplicate-idea merging.

Canonical descriptions MUST be authored using a canonical vocabulary, a finite list of allowed tokens defined as an ordinary governed idea within the system. The canonical vocabulary is versioned, and every canonical-complexity description representation MUST reference the exact version from which it is constructed through `vocabulary_version_id`. This ensures historical interpretability even if the vocabulary evolves through governance challenges. Vocabulary changes - such as adding a word, redefining a term, splitting a token into multiple concepts, or retiring an obsolete term - MUST occur only through governance processes and take effect at the rulebook-defined activation boundary. Nodes MUST validate canonical descriptions against the explicitly referenced vocabulary version at the representation's canonical position. A canonical description that omits the reference, references an idea that does not yet exist, or uses words not present in its referenced vocabulary version is invalid. A node MUST NOT substitute the then-current or latest version. A title is not a canonical-complexity description and MUST NOT carry this field.

Canonical vocabulary tokens are chosen to be maximally general and broadly interpretable by humans while remaining few enough to support deterministic machine comparison. They represent fundamental relational and categorical primitives such as "human," "group," "object," "action," "cause," "help," "harm," "living," "not living," "big," "small," "good," "bad," "true," and "false." The exact vocabulary is defined externally in a governed specification. Canonical descriptions built from this vocabulary MUST be comprehensible to typical users and SHOULD express the idea's essential meaning in the simplest possible terms. Their purpose is not to capture full nuance but to reveal the minimal conceptual form beneath the idea's more detailed or technical expressions.

Canonical descriptions operate as cross-language semantic anchors. Each canonical vocabulary token has a defined conceptual meaning and may be translated into multiple natural languages without altering its semantic identity. Implementations MAY provide language-specific displays of canonical descriptions, but the underlying token sequence remains universal. Because canonical descriptions rely on a small, fixed set of tokens, they allow participants from different languages and cultural backgrounds to understand the core meaning of ideas without ambiguity. They ensure that the universe speaks a single minimal language at the foundational semantic layer.

Canonical descriptions MUST align with the idea's type and subtype. A truth claim's canonical description expresses the factual, relational, causal, predictive, or model-level assertion corresponding to its subtype. A conceptual idea's canonical description expresses the underlying interpretive or normative proposition. An actionable idea's canonical description expresses the essential structure of the proposed action in fundamental terms. An action's canonical description expresses what occurred in the simplest unambiguous manner. Canonical descriptions that contradict an idea's type or misrepresent its semantic role are invalid and have no effect.

Canonical descriptions play a central role in determining semantic identity. Two ideas MAY be considered to express the same meaning only if their canonical descriptions, under the same vocabulary version, are identical or deterministically equivalent according to protocol-defined equivalence rules. This equivalence is a prerequisite for same-as validation and duplicate-idea merging, though those processes are governed separately in Section 2.6. Because canonical descriptions eliminate ambiguity, they prevent homonyms, domain drift, or rhetorical misdirection from causing unintended equivalence between ideas that share only superficial linguistic features. If two ideas cannot share the same canonical description, they MUST NOT be considered semantically identical, regardless of their natural-language titles or descriptions.

Canonical descriptions are not meant to replace natural-language descriptions but to clarify them. They coexist with fundamental, standard, and advanced natural-language expressions, offering a simple, stable representation of meaning beneath more detailed or nuanced phrasing. By providing this human-readable but highly regulated semantic core, canonical descriptions allow ideas to be compared, clustered, merged, translated, and reasoned over in ways that are consistent, transparent, and fully deterministic across the entire canonical universe.

### 2.6 tiered same-as connections and duplicate-idea merging [anchor: tiered_same_as_connections_and_duplicate_idea_merging]

Same-as relationships are not a single binary property between two ideas. The protocol treats semantic equivalence as something that can be asserted and evaluated at multiple tiers of expression: title, natural-language descriptions, and canonical descriptions. A same-as connection therefore always specifies both the pair of ideas involved and the tier at which equivalence is being claimed. This tiered model allows agents to distinguish between superficial linguistic similarity, deeper natural-language alignment, and strict canonical identity. Same-as connections at any tier are proposals or evidentiary claims, not automatic commands to merge, and are subject to challenge and deliberation like any other semantic assertion.

Title-level same-as expresses that two ideas share the same or closely related surface label in natural language. These connections are useful for navigation, autocomplete, and disambiguation workflows, but they carry no semantic authority. Title-level sameness is compatible with complete semantic divergence, as in the case where a single word ("dip") refers to unrelated concepts such as an exercise, a food, or entering water. Title-level same-as connections therefore MUST NOT be used as a basis for merging and MUST NOT be treated as evidence of conceptual identity beyond helping users locate and distinguish between homonymous ideas.

Description-level same-as expresses that the sentence-, paragraph-, or full-length descriptions of two ideas are aligned or strongly overlapping in natural language. These connections indicate that different authors may be describing the same or very similar concept in their own words. Description-level similarity is stronger than title-level similarity but still does not determine semantic identity. Two ideas may share phrases or explanatory framing while differing in type, scope, or critical details. Description-level same-as connections SHOULD be treated as evidence in deliberation about whether two ideas represent the same concept, but they MUST NOT by themselves trigger merging.

Canonical-level same-as expresses that the canonical descriptions of two ideas, written under the same canonical vocabulary version, are identical or deterministically equivalent according to protocol-defined rules. Because canonical descriptions use a minimal governed vocabulary and standardized structure, canonical same-as represents the strongest form of equivalence the system recognizes. Canonical-level equivalence is the only form of sameness that MAY serve as a direct basis for merging. If two ideas cannot share the same canonical description, they MUST NOT be considered semantically identical, regardless of any overlap in titles or natural-language descriptions.

Same-as connections at any tier are represented as explicit connection events with a tier parameter indicating whether the claim applies to the title, a specific description tier (such as sentence or paragraph), or the canonical description. Multiple same-as connections MAY exist between the same pair of ideas across different tiers. Agents MAY propose or challenge same-as connections at any tier, expressing agreement or disagreement about whether two ideas are identical in label, phrasing, or core meaning. During replay, nodes construct the same set of same-as connections from the event log and apply the same tier semantics, ensuring that semantic alignment is evaluated deterministically.

A merge event unifies two ideas into a single surviving idea and is a high-impact operation. Merge events MUST satisfy strict preconditions. First, the ideas MUST share the same idea type and, where applicable, the same truth-claim subtype. Second, there MUST exist at least one accepted canonical-level same-as connection between the two ideas under a single canonical vocabulary version; canonical equivalence is a necessary condition for merging. Third, the connection and description histories of the two ideas MUST not contain unresolved contradictions that violate structural or governance rules. Fourth, no active safety or governance rule may prohibit merging these particular ideas. If any precondition fails, the merge event is invalid and MUST NOT alter derived state.

When a merge event is valid, one idea is designated as the surviving idea and the other becomes a merged-into alias. The alias idea remains in the event log for historical and audit purposes but becomes inert for future deliberation. It cannot receive new descriptions, connections, or votes. All descriptions from the alias idea are incorporated into the surviving idea's description set, preserving their original authorship, timestamps, and content hashes. Same-as connections at all tiers that referenced the alias idea are redirected to reference the surviving idea, or are marked as satisfied when referring to the merged pair, according to deterministic rules defined by the protocol. No historical expression is lost; it is consolidated under a single semantic identity.

Connections referencing the alias idea are redirected to the surviving idea in a deterministic manner. Where connections are compatible, they are unified; where they conflict - for example, contradictory importance judgments or incompatible execution links - conflict-resolution rules MUST be applied or the merge MUST be blocked until further deliberation resolves the inconsistency. Implementations MUST NOT silently drop or rewrite conflicting connections during merge processing. Instead, merge events either proceed with a deterministic conflict-handling procedure defined by the protocol or are invalidated when irreconcilable conflicts exist. This ensures that merging does not introduce hidden semantic changes or inconsistencies into the universe.

Merges affect ideas, not identities. Identity ideas, which represent humans, MUST NEVER merge. If two identity ideas are believed to represent the same real-world agent, this MUST be handled through separate identity-governance and verification processes, not through idea-merging. When two non-identity ideas merge, any POD routing, importance attribution, or epistemic credit tied to those ideas is recomputed deterministically based on the merged structure, subject to the token and importance specifications. Merge events therefore may change how credit and importance are distributed across the graph but MUST do so in a way that is fully determined by the event log and protocol rules.

Merging is irreversible at the event-log level. If a merge is later judged incorrect, the system does not delete or rewind the merge event. Instead, a split event may be issued to create new successor ideas that separate the concepts again, while the merged idea remains as historical evidence of the prior conflation. The split event MUST reconstruct distinct ideas with appropriate subsets or reinterpretations of the descriptions and connections, and MUST be governed by its own deterministic rules. This preserves a complete and inspectable trail of how the community's understanding of semantic identity has changed over time.

Through tiered same-as connections and carefully governed merge events, the protocol provides a transparent mechanism for semantic consolidation without sacrificing history or nuance. Title- and description-level same-as connections inform deliberation and user experience, while canonical-level equivalence provides the strict criterion for true semantic identity. Merges occur only when this identity is established and validated, ensuring that the universe remains coherent, deduplicated where appropriate, and resistant to accidental or manipulative collapse of distinct ideas into a single node.

### 2.7 connections and semantic relationships between ideas [anchor: connections_and_semantic_relationships_between_ideas]

Connections represent explicit semantic, epistemic, relational, and structural links between ideas in the canonical universe. The protocol does not infer connections automatically from text, similarity, shared authorship, or algorithmic analysis. Every canonical connection MUST originate from an event authored by an identity or from deterministic protocol processes triggered by such events. Each connection specifies its connection_type, direction (unless defined as symmetric), and any required usage or tier information. Because the connection graph determines truth flows, importance flows, action evaluation, and semantic structure, connection semantics MUST remain fully deterministic under replay.

Agents MAY form tentative or provisional connections between draft ideas or between draft ideas and canonical ideas within their private or local workspaces. Such draft connections are non-canonical and out of protocol conformance scope. Conformant nodes are not required to store, validate, replicate, or interpret draft connections, and they do not participate in deterministic replay. A connection becomes canonical only when it is explicitly published as a canonical connection event linking canonical ideas.

Connections are directional unless explicitly defined as symmetric. A directional connection asserts that idea A relates to idea B in a specific role - for example, that A is important for evaluating B, that A is being used as evidence for B, or that A belongs as a member of the group represented by B. Symmetric connections include certain same-as relationships, where direction does not alter meaning. Every connection event carries metadata recording the author, timestamps, connection_type, applicable description tiers (if relevant), usage or role (if required), and any parameters necessary for deterministic evaluation. Implementations MUST NOT infer directionality or symmetry beyond what the event specifies.

The protocol defines three canonical connection types:

same_as  -  tiered semantic equivalence between ideas or descriptions.

relative_importance  -  directed importance relationships with a specified usage.

membership  -  identity-to-group membership and stewardship relationships.

Rulebooks MAY define additional usage and role values for these canonical types, but SHALL NOT introduce new top-level connection_type families that change their semantic role.

Tiered same-as connections express claims of equivalence between two ideas at a specific tier of meaning: title-level, natural-language description-level (broken down by sentence, paragraph, or full description), or canonical-level. Title-level sameness indicates shared linguistic form and assists with navigation and disambiguation but carries no semantic force. Description-level sameness indicates natural-language equivalence and informs deliberation but does not establish identity. Canonical-level sameness, written under a specific canonical vocabulary version, is the only tier capable of grounding semantic identity and potential merges. Multiple same_as connections MAY exist between the same pair of ideas, each associated with its own tier. Nodes MUST validate and store all tier-specific same-as connections and MUST NOT collapse them into a single undifferentiated relation. same_as connections are symmetric.

Relative-importance connections express that idea A is important to or important for idea B. They are the shared connection primitive used as ranking inputs and importance arguments, but the rank product remains explicit: 20-axis universal importance is distinct from 10-axis reference-relative importance. Personal importance judgments MAY exist as noncanonical owner-controlled state for private reasoning or organization. Sharing a personal view does not itself turn owner-selected rank positions into canonical verdicts; a person may separately publish ordinary canonical connections or arguments through an explicit public write path.

Each relative_importance connection carries a usage value that specifies how it is being used in deliberation. At minimum, the following usages are defined:

general  -  a baseline importance edge stating that A matters to or for B along one or more axes and timeframes.

importance_argument  -  a connection created inside an importance challenge, stating that A is being proposed as an argument for why B should be considered more or less important.

evidence_for  -  a connection stating that A is being used as evidence in favor of truth claim B.

evidence_against  -  a connection stating that A is being used as evidence against truth claim B.

Rulebooks MAY introduce additional usage values (for example, future specializations such as action_outcome) provided they remain within the semantic scope of "A is important for evaluating or understanding B" and preserve deterministic replay. The rank kind, participation scope, reference, axis, and timeframe are governed by the importance system in Section 5 and encoded in rulebook-defined metadata; they are not separate connection types. Universal importance uses its distinct 20-axis vocabulary. Reference-relative importance uses `important_to_reference` or `important_for_reference` across the five time horizons. Private individual ordering remains noncanonical and outside this metadata's canonical challenge semantics.

When `usage = general`, `relative_importance` edges provide eligible inputs to the declared universal or relative rank context. When `usage = importance_argument`, they capture reasons advanced for either candidate and do not themselves enter the general rank list or move a candidate. When `usage = evidence_for` or `usage = evidence_against`, they specify that an idea is being used as evidence in support of or against a truth claim. The rank kind, context fields, and usage tag together determine how an edge participates; implementations MUST NOT infer universal importance merely from the title or identity of a reference idea.

Membership connections express that an identity or idea belongs to, or stewards, a group-like idea. These are implemented as membership connections from the member to the group nucleus idea and carry a role field such as member_of, steward_of, or other rulebook-defined membership roles. Membership connections link identity ideas to tribe nuclei, special personal-structure ideas (such as "My Mindgarden", "My Backyard of Relationships", "My Self Tree", and "My Anthill"), and any other group structures. Membership connections do not directly influence truth or importance calculations. They determine which identities participate in which local governance processes, which tribe-scoped importance maps are constructed using which voter sets, and how group structures are reconstructed during replay.

Connections MAY be challenged or superseded. A challenge asserts that a connection is invalid, misleading, improperly typed or tagged, or inconsistent with protocol rules. For example, a challenge MAY claim that a relative_importance edge should not have usage = evidence_for because the proposed evidence is epistemically invalid for that truth claim, or that two ideas are not same_as at the canonical tier. If successful, the connection becomes inactive but remains visible in the event log. Superseding events MAY refine or reinterpret certain connections while preserving history (for example, replacing a description-level same_as with a canonical-level same_as after further deliberation). No connection MAY be deleted or removed. All changes to connection status MUST occur through explicit events, ensuring perfect replayability and historical transparency.

Certain connections and usages are safety-sensitive. Connections involving harm, protected classes, dangerous plans, or ethically significant actions-especially relative_importance edges with evidential usages attached to such ideas-MAY trigger safety rules defined in Section 11. During replay, nodes MUST apply safety filters and governance rules active at each relevant cycle boundary to determine which connections are operable, visible, restricted, or inert. Safety-sensitive connections always remain in the log but MAY be restricted from influencing deliberation, importance flow, or truth evaluation, depending on the active rulebooks.

The connection graph MUST remain deterministic across nodes. Given the same ordered event sequence, all conformant nodes MUST construct the same active and inactive same_as tiers, relative_importance edges with their usages, and membership structures. Implementations MAY optimize or index connection storage but MUST NOT infer additional implicit connections or reinterpret the semantics of event-defined connections. The connection graph is one of the core derived structures of the universe, determining truth evaluation, importance determination, POD flow, action interpretation, and semantic identity. Determinism in its construction is essential to preserving coherence, auditability, resilience, and long-term trust in the protocol.

### 2.7.1 Eligible connections and living-map participation (normative) [anchor: eligible_connections_and_living_map_participation_normative]

Not all canonical connections participate equally in default computation, visualization, or incentive routing.

The protocol distinguishes between:
- the **existence** of a connection in the canonical event log, and
- the **eligibility** of a connection to participate in living-map surfaces, importance propagation, and POD/POINT routing.

Eligibility is a **derived property**, not an authored one. A connection MAY be deemed ineligible due to derived lifecycle state (e.g., rot or burn), scope rules, or other deterministic exclusions defined elsewhere in this protocol.

Ineligible connections:
- remain permanently recorded in the canonical graph,
- remain queryable and challengeable,
- but do not contribute to default importance computation, propagation, or incentive flow unless and until eligibility is restored.

Eligibility MUST be computed deterministically and MUST NOT delete or modify historical events.



### 2.8 Structural Roles (Non-Canonical Idea Types) [anchor: structural_roles_non_canonical_idea_types]

#### 2.8.1 Definition [anchor: definition]

Structural roles are non-canonical classifications applied to ideas to support:

personal-space topology,

narrative organization,

relationship mapping,

memory structures, and

UI/visual anchoring.

Structural roles do not constitute new idea_type values and MUST NOT expand the canonical ontology defined in §2.1.

Structural roles apply only to canonical ideas once they have been published. Agents MAY also use the same structural-role concepts locally when working with draft ideas, but such local usage is non-canonical and out of protocol conformance scope until the associated ideas and roles are explicitly published as canonical metadata.

A structural role is metadata attached to an idea of one of the five primary canonical types:

truth_claim

conceptual_idea

actionable_idea

action

identity

#### 2.8.2 Purpose [anchor: purpose]

Structural roles MAY:

influence visibility rules in personal or relationship spaces,

group ideas into narrative or relational clusters,

determine anchoring positions in visual interfaces,

define sequencing, chronology, or ordering,

support personal or interpersonal memory structures,

represent conceptual containers uniquely associated with a specific identity.

Structural roles MUST NOT:

alter challenge semantics (truth, importance, action, representation),

affect POD or POINT generation or distribution,

participate directly in universal or tribe importance ranking,

modify governance or eligibility rules,

interfere with deterministic replay.

#### 2.8.3 Examples (Non-Exhaustive) [anchor: examples_non_exhaustive]

The following future-facing structural-root names are officially recognized for Profile-v0 identity roots. Exact structural-role constants, identifiers, encodings, and connection schemas remain delegated to Appendix A and structural-role reconciliation:

Mindgarden - personal root for ideas authored by an identity.

Backyard of Relationships - container for relational structures.

Self Tree - personal narrative tree of an identity; non-epistemic.

Anthill - structural map of mutual interpersonal connections.

shrub - root node for a bilateral relationship between identities.

relationship_memory_leaf - shared memory attached to a shrub.

personal_memory_leaf - private memory leaf attached to the self_tree.

vine - curated chronological or conceptual sequence of ideas.

mythology - higher-order narrative structure across ideas.

stump - structural or historical terminal node (optional).

Rulebooks MAY register additional structural roles, provided they remain non-canonical, do not alter challenge semantics, and are fully representable through existing connection types.

#### 2.8.4 Connection Types for Structural Roles [anchor: connection_types_for_structural_roles]

Structural roles MUST exclusively use the canonical connection types defined in §2.7:

same_as

relative_importance (with appropriate usage fields)

membership (with appropriate role fields)

Rulebooks MAY define additional usage or role values but MUST NOT create new connection families.

#### 2.8.5 Consequences for Interoperability [anchor: consequences_for_interoperability]

All implementations MUST:

treat structural roles as metadata attached to canonical ideas;

replay them deterministically when present in canonical events;

preserve them in snapshots;

avoid introducing divergent semantics or interpretation for any structural role.

Structural roles SHALL NOT influence canonical ordering, importance ranking, or challenge outcomes unless explicitly allowed by a rulebook and approved according to governance requirements.

### 2.9 Spatial representation records (normative) [anchor: spatial_representation_records_normative]

This subsection defines how spatial information may be recorded in the canonical system to support navigation, familiarity, and deterministic reconstruction, while remaining strictly non-semantic and presentation-independent as required by §0.66.

#### 2.9.1 Purpose and scope [anchor: purpose_and_scope]

Spatial representation records exist to support:
- persistent navigation,
- long-term familiarity,
- and stable reconstruction of shared idea spaces across nodes and interfaces.

They do not define, imply, or constrain:
- truth,
- importance,
- agreement,
- similarity,
- endorsement,
- or epistemic value.

All semantic meaning remains exclusively defined by canonical ideas, connection types, challenges, and rank snapshots.

#### 2.9.2 Idea spaces [anchor: idea_spaces]

An **idea space** is a scoped coordinate system used to record where ideas enter a navigable landscape.

The canonical system defines at minimum:
- a **universal idea space**, containing all public ideas, and
- **relative idea spaces**, each scoped to a specific reference idea.

Each idea space has:
- an origin definition,
- an independent growth history,
- and its own spatial record.

Idea spaces are canonical objects and are identified explicitly in spatial records.

#### 2.9.3 Spatial placement events [anchor: spatial_placement_events]

The canonical system MAY record **spatial placement events**.

A spatial placement event records, at minimum:
- the `idea_id` being placed,
- the `idea_space_id` in which it is placed,
- the assigned coordinates or equivalent placement parameters,
- the placement rule identifier and its parameters,
- the canonical event identifier that performed the placement.

Spatial placement events are canonical historical facts describing *when and where an idea entered an idea space*.

They MUST NOT be interpreted as semantic assertions.

#### 2.9.4 Growth-based placement model [anchor: growth_based_placement_model]

Spatial placement within an idea space uses a **growth-based placement model**.

When an idea enters an idea space:
- it MUST be placed at the edge of the currently occupied region of that space,
- subject to explicit constraints defined by the active placement rule (for example, maximum distance from related ideas),
- without recomputing, optimizing, or rearranging existing placements.

Placement rules MUST be deterministic given the canonical state at the time of placement.

#### 2.9.5 Immutability and deterministic replay [anchor: immutability_and_deterministic_replay]

Once recorded, a spatial placement event:
- MUST NOT be mutated,
- MUST be replayable deterministically from the canonical event log,
- and MUST remain stable across all conformant nodes.

Existing placements MAY NOT be repositioned retroactively.

Changes to spatial behavior MAY only occur through:
- the creation of a new idea space, or
- the activation of a new placement rulebook that applies only to future placement events.

#### 2.9.6 Separation from semantics [anchor: separation_from_semantics]

In accordance with §0.66 (presentation independence):

- Spatial coordinates, distances, and geometry MUST NOT be interpreted canonically as indicators of meaning, truth, importance, or agreement.
- Interfaces MAY use spatial records for navigation and visualization only.
- Any semantic interpretation of spatial relationships is explicitly non-canonical.

Spatial representation is navigational scaffolding, not epistemic authority.


### 2.10 Wall-clock time records (normative) [anchor: wall_clock_time_records_normative]

This subsection defines how wall-clock time may be recorded in the canonical system to support human understanding, interpretation, and analysis of the event log, while remaining strictly non-authoritative and non-semantic.

#### 2.10.1 Wall-clock time as observational data (normative) [anchor: wall_clock_time_as_observational_data_normative]

The system may record wall-clock time as **observational data** to support user comprehension (e.g., “when did this happen"”), visualization of activity, and detection of anomalous behavior patterns. Wall-clock time is treated as an empirical observation about the world and may be uncertain, drifted, or disputed.

Wall-clock time is not a canonical input and is never required for deterministic reconstruction of the canonical universe.

#### 2.10.2 Non-authority of wall-clock time (normative) [anchor: non_authority_of_wall_clock_time_normative]

Wall-clock time has **no canonical authority** in the protocol.

Nodes MUST NOT use wall-clock timestamps, device clocks, external calendars, or time services to determine, influence, or shortcut any canonical semantics or deterministic rules, including but not limited to:

- canonical event ordering or validity,
- cycle sealing, cadence, or pacing,
- rate limits, replenishment, or action-budget mechanics,
- eligibility, quorum, voter selection, or challenge windows,
- deterministic replay, merge semantics, or snapshot correctness.

All canonical state transitions are derived exclusively from:
- the ordered canonical event log,
- applicable protocol rules and governance-adopted rulebooks,
- and adjudicated truth-claim outcomes.

Wall-clock time MAY appear only inside identity-authored ideas, provenance references, or claims about sources. Such material remains perpetually challengeable and has no effect unless and until it is connected through ordinary evidential relationships and a certainty-band challenge assigns the required certainty. Node-local time, server time, receipt time, background schedulers, uncommitted observations, or external links alone MUST NOT affect certainty, cycle sealing, certification, payouts, or authority.

Tempo evidence may contribute to certainty only when represented by identity-authored ideas, explicit allowed connections, and challenge verdicts with inspectable provenance and replayable inputs. Evidence links, source references, or event metadata do not by themselves satisfy predicates or beacon diversity.

Time-based guardrails and thresholds (including those used for cycle progression) MUST consume only Tempo-derived predicates, never raw timestamps.

Wall-clock time therefore exists in the system strictly as **observed testimony**, never as **authority**.


#### 2.10.3 Event-level wall-clock timestamps (normative) [anchor: event_level_wall_clock_timestamps_normative]

Every canonical event **MUST include a locally recorded wall-clock timestamp** when created on a computing device.

This timestamp represents the creating device's local clock reading at the moment of event creation and is recorded automatically by software.

Characteristics:

* the timestamp is signed as part of the event payload
* it reflects what the device clock reported, not a guaranteed global time
* incorrect, drifting, or manipulated clocks do not invalidate events
* inclusion in an event does not make the timestamp authoritative or admissible as Tempo certainty

When an event is created through delayed transcription from non-digital or offline activity, the event may include an approximate wall-clock timestamp (or an approximate wall-clock statement as a truth claim), which is treated as lower-precision observational evidence.

#### 2.10.4 Time observations as truth claims (normative) [anchor: time_observations_as_truth_claims_normative]

Explicit statements about wall-clock time are represented using the system’s existing truth-claim types and subtypes (particularly observation / measurement claims). There is no special or privileged truth-claim category for time.

Examples include:

* "This event occurred around March 2026."
* "At cycle 8123, the wall-clock time was approximately 03:14 UTC."

Such statements are modeled, evaluated, challenged, and revised in the same manner as any other empirical claim.

Any observation about time MUST be expressible as a truth claim with evidence. This includes, without limitation:
- "it is now approximately [date/time] in [frame of reference],"
- "at least [duration] has elapsed since [cycle boundary / event],"
- "a periodic natural marker occurred [N] times since [event]" (sunrise, sleep cycles, instrument ticks),
- "this device clock read [X] at observation."

Time-related truth claims MAY be used to derive threshold predicates required by §4, including predicates of the form:
- `cycle_age_ge_dmin`
- `cycle_age_ge_dmax`
- `structural_dmax_liveness_predicate`

`cycle_age_ge_dmin` and `cycle_age_ge_dmax` MUST be considered satisfied only when the relevant target reaches required Tempo structural support under the Tempo Specification. Structural support is derived from profile-required eligible-human stances plus capped passive evidence and blocker checks; it is not ordinary truth certainty. `structural_dmax_liveness_predicate` is the Dmax-only survivor exception: it is derived from nonzero eligible-human survivor support, required capped passive plausibility evidence, and blocker checks, not from ordinary certainty, and it may be consumed only for forced structural closure. Raw timestamps MUST NOT satisfy any of these predicates directly.


#### 2.10.5 Node-level time recording (non-normative) [anchor: node_level_time_recording_non_normative]

Nodes may additionally record local timestamps when receiving or persisting events (e.g., receipt time, database commit time). These timestamps are node-local metadata and are not part of canonical state.

These records may be used for diagnostics, visualization, and analysis but carry no protocol authority.

#### 2.10.6 Derived wall-clock views (non-normative) [anchor: derived_wall_clock_views_non_normative]

Nodes and user interfaces may derive estimated wall-clock timelines for events by combining:

* event-level wall-clock timestamps
* time-related truth claims
* canonical event ordering

Derived timelines may include uncertainty ranges and may change as new evidence is added. These views are informational only and do not affect protocol behavior.


### 2.11 Derived lifecycle_state for ideas and eligible edges (normative) [anchor: derived_lifecycle_state_for_ideas_and_eligible_edges_normative]

Every canonical idea and every eligible relative-importance connection has a **derived lifecycle_state** that governs its participation in the living map.

Lifecycle state is:
- deterministically derived from the canonical event log,
- recomputed at cycle boundaries,
- recorded in snapshots as derived state,
- never authored directly by events,
- and never deletes or alters historical records.

#### 2.11.1 Lifecycle states [anchor: lifecycle_states]

The protocol defines the following lifecycle states:

- **active**  
  The idea or connection participates fully in living-map surfaces, importance computation, and POD/POINT routing.

- **rotted**  
  The idea or connection is historically preserved but excluded from default living-map participation due to low importance and inactivity. Rotted objects are considered dormant but easily revivable.

- **burned**  
  The idea or connection is historically preserved but excluded from default living-map participation due to sustained low importance and prolonged inactivity. Burned objects represent pruned graph mass and require explicit revival.

Lifecycle states apply independently to:
- ideas (global lifecycle), and
- relative-importance connections (edge-specific lifecycle).

An idea MAY be active while some of its relative-importance connections are rotted or burned.

#### 2.11.1 Structural edge restoration on idea resurrection (normative) [anchor: structural_edge_restoration_on_idea_resurrection_normative]

When an idea transitions from `burned` to `active` via a successful resurrection challenge, the protocol MUST restore the idea's **non-relative structural/provenance connections** to active participation in the living map by default, including (but not limited to):
- `created_by`,
- `same_as`,
- and other representation/provenance links that define identity, authorship, or equivalence.

This automatic restoration applies only to structural/provenance connections and MUST NOT automatically restore `relative_importance` connections. Relative-importance connections are independently lifecycle-scoped and require their own maintenance or resurrection as defined elsewhere in this protocol.

#### 2.11.2 Derivation inputs [anchor: derivation_inputs]

Lifecycle state is derived from the following factors:

- universal importance rank (for ideas),
- relative importance rank (for relative-importance connections),
- number of cycles since creation,
- number of cycles since last qualifying engagement.

All thresholds, windows, and hysteresis parameters are defined via governance rulebooks.

#### 2.11.3 Qualifying engagement [anchor: qualifying_engagement]

Qualifying engagement is any canonical activity that demonstrates active deliberation or relevance.

Unless overridden by governance, qualifying engagement includes:
- participation in a challenge,
- receiving or casting votes in a related challenge,
- being referenced by an argument or evidence idea,
- being involved in a canonical transformation affecting importance or representation.

Mere existence or passive presence does not constitute engagement.

#### 2.11.4 Effects of lifecycle state [anchor: effects_of_lifecycle_state]

Lifecycle state affects **participation**, not **existence**.

Rotted or burned ideas and connections:
- remain permanently stored in the canonical graph,
- remain fully inspectable and challengeable,
- are excluded from default living-map visualizations,
- do not participate in importance propagation or POD/POINT routing.

Lifecycle state MUST NOT affect:
- raw structural views,
- historical inspection,
- or the ability to challenge, reference, or revive an object.

#### 2.11.5 Revival and resurrection [anchor: revival_and_resurrection]

- **Rotted  -> active**  
  Occurs automatically upon qualifying engagement or restoration of sufficient importance.

- **Burned  -> active**
  Requires an explicit **resurrection action** as defined by the protocol. Resurrection is a canonical creation-like action (cost parity with creation) that restores the target to active participation in the living map. Resurrection is challengeable post-hoc under existing challenge domains, but does not require a challenge to occur.


Revival and resurrection MUST be deterministic, auditable, and challengeable.

#### 2.11.6 Canonical invariants [anchor: canonical_invariants]

Lifecycle derivation MUST satisfy the following invariants:

- No lifecycle transition deletes history.
- All nodes derive identical lifecycle states from the same event log and rulebooks.
- Derived exclusions MUST be explainable (“why is this not visible"”).
- Lifecycle state MUST NOT be used as a punishment or moderation tool; it exists solely to keep the living map lean and meaningful.

#### 2.11.7 Structural/provenance edges are active iff the idea is active (normative) [anchor: structural_provenance_edges_are_active_iff_the_idea_is_active_normative]

Structural and provenance connections that define identity, authorship, and equivalence are not independently subject to rot/burn. Instead, they are deterministically treated as **active whenever their endpoint idea is active**, and as non-participating whenever their endpoint idea is not active.

This includes (but is not limited to):
- `created_by`,
- `same_as`,
- and any other representation/provenance links designated by rulebooks as structural.

These structural/provenance connections MUST NOT require separate maintenance activity, MUST NOT require separate resurrection, and MUST NOT independently affect importance propagation or POD/POINT supply beyond their structural meaning.

Relative-importance connections (`relative_importance`) are explicitly excluded from this rule and remain independently lifecycle-scoped as defined elsewhere in this protocol.


## 3. Cycles, activity pacing, and rate limits (normative) [anchor: 3_cycles_activity_pacing_and_rate_limits_normative]

This section defines the system's canonical pacing mechanism ("cycles"), the conditions under which cycles seal, and how per-identity rate limits and action budgets are enforced without reliance on trusted wall-clock time.

Cycles are derived from the canonical event log and exist solely to pace activity, adapt to participation scale, and bound per-identity influence while preserving deterministic replay. Cycles MUST NOT be used to determine truth, importance, or governance authority.

This section owns the root normative cycle invariants and sealing semantics. The Cycle Specification is the detailed subordinate normative algorithm for cycle derivation, boundary classification, certification, authorization-frontier interaction, and replay. No appendix, rulebook, or subsystem may contradict the Protocol v5 invariants or the Cycle Specification's subordinate algorithm.


### 3.1 Purpose of cycles (normative) [anchor: purpose_of_cycles_normative]

The system does not use wall-clock time, device clocks, or external time sources as canonical inputs. Instead, it defines a **cycle** as a system-native semantic boundary that gates pacing state transitions.

Cycles are used to:
- pace challenges and deliberation,
- reset or recharge rate limits and action budgets,
- bound per-identity throughput regardless of execution speed,
- trigger and bound long-horizon maintenance processes (including rot/burn evaluation),
- define effective-date boundaries for governance-adopted parameter changes.

Cycles are NOT used to:
- determine truth or correctness,
- weight votes or identities,
- allocate or modify POD or POINT,
- grant governance authority.

A cycle boundary may be represented in the canonical log by a boundary marker event (a seal), but the **cycle index** itself is derived metadata. Cycle indices are not immutable facts and MAY be re-derived from the event log during deterministic replay (including after merges).

### 3.2 Deterministic derivation (normative) [anchor: deterministic_derivation_normative]

Cycles are derived from the canonical event log via deterministic replay.

Given the same ordered set of canonical events and applicable rulebooks, all nodes MUST derive identical:
- cycle boundaries,
- cycle indices,
- seal type (deliberative vs forced),
- and any boundary-scoped derived state.

Nodes MUST NOT use device clocks, wall-clock timestamps, or external time sources to determine cycle sealing or boundary placement.

Time may constrain cycle sealing only through **challengeable time truth claims**. Protocol v5 consumes only Tempo-derived guardrail predicates (e.g., `cycle_age_ge_dmin`, `cycle_age_ge_dmax`) and the Dmax-only `structural_dmax_liveness_predicate`; it MUST NOT consume raw timestamps.

At each cycle boundary, deterministic boundary processing MUST be performed as specified in **Section 3.4.3**. Boundary processing MUST be replayable, auditable, and produce identical results on all nodes given the same event log and rulebooks.

Boundary processing MUST NOT author additional canonical events. (The cycle seal or boundary marker event, if used, is authored through the normal canonical event mechanism; the deterministic boundary processing that follows does not emit new canonical events.)



### 3.3 Deliberative completion inputs (normative) [anchor: deliberative_completion_inputs_normative]

Only the following quantities contribute to deliberative cycle completion. No other activity, event volume, or graph mutation may advance a cycle.

#### 3.3.1 V  -  Distinct voting identities [anchor: v_distinct_voting_identities]

For a given cycle *r*:

- **V[r]** is the number of distinct eligible identities that cast **at least one eligible vote** during cycle *r*.
- Each identity contributes **at most one (1)** unit to V per cycle, regardless of how many votes it casts.
- Voting eligibility is defined elsewhere in this protocol and MAY vary by challenge domain or governance rules.

The following do **not** affect V:
- idea creation,
- connection creation,
- challenge creation,
- evidence submission without voting,
- multiple votes by the same identity,
- passive observation.

V measures **breadth of human participation**, not activity volume or effort.

#### 3.3.2 C  -  Challenge closures [anchor: c_challenge_closures]

For a given cycle *r*:

- **C[r]** is the number of challenges whose verdict becomes canonical during cycle *r*.
- A challenge contributes **exactly one (1)** unit to C, and only once, at the moment its verdict is finalized and recorded in the canonical event log.

The following do **not** affect C:
- challenge creation,
- evidence submission,
- partial voting,
- expired or abandoned challenges without verdicts,
- challenges whose verdicts were finalized in prior cycles.

C measures **resolved deliberation**, not deliberation initiated.

#### 3.3.3 W_score  -  Deliberative work score [anchor: w_score_deliberative_work_score]

For a given cycle *r*:

W_score[r] = V[r] + C[r]


W_score represents the total amount of **collective deliberative completion** achieved during the cycle.

The following MUST NOT contribute to W_score:
- raw event counts,
- idea or connection creation volume,
- survivals, decay, or net graph change,
- time passage,
- any non-voting or non-verdict activity.

Rulebooks MUST NOT introduce additional completion inputs beyond V and C.

---

### 3.4 Cycle sealing mechanisms (normative) [anchor: cycle_sealing_mechanisms_normative]

A cycle may seal via **exactly one** of the mechanisms defined below. No other sealing mechanism is permitted.

For purposes of cycle sealing conditions:
- `W_score_since` is the sum of `W_score` contributions accrued since the most recent prior cycle boundary (exclusive).
- `V_since` is the count of distinct eligible voting identities that have contributed to `V` since the most recent prior cycle boundary (exclusive).


#### 3.4.1 Deliberative seal (normal path) [anchor: deliberative_seal_normal_path]

A cycle *r* becomes eligible for a **deliberative seal** at the earliest canonical log position where **all** of the following conditions are satisfied since the previous cycle boundary:

1. **Deliberative work threshold**

W_score_since >= event_target[r]

where `event_target[r]` is the deterministic cycle work target (`W_target[r]`).

2. **Minimum duration guardrail structurally satisfied**

The predicate `cycle_age_ge_dmin` has reached Tempo `T_allow` structural readiness (Section 5.5).

3. **Earliest-valid boundary rule**

No earlier valid `cycle_close` for cycle *r* exists in canonical log order.

When sealed via this path:
- the cycle is considered to have met its deliberative completion target,
- all boundary-scoped derivations occur normally,
- the seal MUST be recorded as a *deliberative* seal.
#### 3.4.2 Forced structural close (liveness path) [anchor: forced_structural_close_liveness_path]

A cycle *r* MUST close via the **forced path** at the earliest canonical log position where:

- either `cycle_age_ge_dmax` has reached Tempo `T_allow` structural readiness or `structural_dmax_liveness_predicate == true`,
- `W_score_since < event_target[r]`, and
- no earlier valid `cycle_close` exists for cycle *r*.

For the same anchor and Tempo profile, adjudicated Dmax mechanically implies structural Dmin for boundary evaluation only. `structural_dmax_liveness_predicate` may force Dmax closure only; it does not satisfy Dmin deliberative closure. Neither path creates a Dmin beacon, Dmin certification, or Dmin-based authority.

A forced structural close occurs regardless of participation level and any `V_min` telemetry values.

When sealed via this path:
- a canonical cycle boundary MUST be emitted by `system_boundary_emitter`,
- the boundary MUST be classified as forced,
- the cycle is NOT considered to have met its deliberative completion target.

Forced boundaries exist solely to guarantee structural liveness and forward progress and MUST NOT be interpreted as successful deliberative completion. A forced boundary remains forced forever. Repeated forced boundaries do not accumulate legitimacy and do not become authority by repetition, passage of structural cycles, survivor mode, or later certification.

When the forced close uses `structural_dmax_liveness_predicate`, the boundary trigger MUST be `dmax_structural_liveness_forced`. That trigger does not create ordinary truth certainty, beacon status, cycle certification, authorization-frontier advancement, or authority over POD, POINT, governance, lifecycle, final rank, ordinary mana, ordinary rate limits, token effects, ordinary challenge powers, ordinary canonical writing, invitation capacity, inviter maturation, inviter-eligibility activation, invitation-suspension restoration, or admission rewards.

Tempo structural predicates at `T_allow` MAY be consumed in cycle `r` for structural boundary evaluation in cycle `r`. They MUST NOT assign ordinary truth certainty, create beacons, certify cycles, authorize economic effects, governance activation, lifecycle irreversibility, token effects, final rank effects, ordinary mana spendability, or any other consequential authority. Consequential authority is controlled only by beacon certification and the lagged authorization frontier.

#### 3.4.3 Boundary derivations (normative) [anchor: boundary_derivations_normative]

At each cycle boundary (regardless of seal type), the following deterministic boundary derivations MUST be performed:

- recomputation of `lifecycle_state` for all ideas and eligible connections, including rot and burn evaluation,
- recharge, adjustment, and capping of all per-identity mana pools and action budgets,
- update and rollover of any cycle-scoped eligibility windows, counters, or lookback state,
- evaluation of any cycle-based thresholds defined by governance rulebooks,
- determination of the effective date for any governance changes adopted in prior cycles,
- derivation of export-pack metadata for the completed cycle.

All boundary derivations:
- MUST be computed exclusively from the canonical event log and applicable rulebooks,
- MUST be replayable and auditable,
- MUST NOT author additional canonical events.

(The cycle seal event is emitted by `system_boundary_emitter` under deterministic cycle-closure rules; boundary derivations do not emit new canonical events.)


### 3.5 Temporal guardrails (Dmin / Dmax) (normative) [anchor: temporal_guardrails_dmin_dmax_normative]

Cycles are constrained by minimum and maximum duration guardrails to prevent both machine-speed acceleration and deadlock.

#### 3.5.1 Time as challengeable evidence [anchor: time_as_challengeable_evidence]

There is no trusted clock in the system. Duration is represented only through ordinary **time-related truth claims** with Tempo metadata. Structural Dmin/Dmax readiness is derived from the canonical prefix using profile-required eligible-human structural stances, capped passive evidence, and blocker checks as defined in the Tempo Specification. Ordinary truth certainty for those claims remains governed by evidence-placement and certainty-band challenges. The only exception is the Dmax-only `structural_dmax_liveness_predicate`, which is not certainty and can be consumed only for forced structural closure.

Protocol v5 consumes only the resulting boolean predicates:
- `cycle_age_ge_dmin`
- `cycle_age_ge_dmax`
- `structural_dmax_liveness_predicate`

Raw timestamps, device clocks, UI timers, or external calendars MUST NOT be consumed by cycle logic.

#### 3.5.2 Minimum duration guardrail (Dmin) [anchor: minimum_duration_guardrail_dmin]

A cycle MUST NOT seal through the deliberative path until the predicate `cycle_age_ge_dmin` is satisfied.

Forced Dmax closure is the explicit exception to this deliberative Dmin rule. Ordinary Dmax mechanically supplies structural Dmin for boundary evaluation only. Survivor Dmax structural liveness does not satisfy Dmin, but it may still force a Dmax-only structural boundary when the Cycle and Tempo specifications permit it.

Dmin exists to:
- prevent adversarial or accidental rapid cycling,
- ensure a minimum deliberative window,
- decouple stress-driven activity spikes from cadence.

#### 3.5.3 Maximum duration guardrail (Dmax) [anchor: maximum_duration_guardrail_dmax]

A cycle MUST seal through the forced path when the predicate `cycle_age_ge_dmax` reaches Tempo structural readiness and the work target is unmet. It MAY also seal through the forced path when `structural_dmax_liveness_predicate` is true and the work target is unmet.

Dmax mechanically implies structural Dmin for the same anchor and Tempo profile. This implication is valid only for structural boundary evaluation. `structural_dmax_liveness_predicate` does not satisfy Dmin. Neither path creates a Dmin beacon, certifies the cycle, or authorizes downstream effects.

Dmax exists to:
- guarantee liveness during low participation or disruption,
- allow the system to progress even when deliberation stalls,
- enable adaptive targets to respond to participation collapse.

#### 3.5.4 Lagged certification and authorization frontier (normative) [anchor: lagged_certification_authorization_frontier_normative]

Structural cycle boundaries are separate from consequential authority.

Each closed cycle has a derived certification status based on the required Tempo target:
- deliberative boundaries require certification of the cycle's Dmin target,
- forced boundaries require certification of the cycle's Dmax target.

Certification requires derived beacon coverage at the target level. A beacon-level claim may explicitly cover one target, multiple consecutive targets, or a structured elapsed-time relation that deterministically entails multiple targets. Each cycle still receives its own derived certification status. A representative time claim may be displayed for auditability, but it is not the source of authority.

The authorization frontier is derived as a contiguous, monotonic, lagged frontier:
- it may advance only through contiguous certified cycles,
- it may not advance beyond `current_cycle - K`, where `K` is the active governance-defined lag window,
- a certification gap stops advancement,
- revocation or contradiction stops future advancement but does not rewrite history already authorized by an earlier frontier.

Genesis starts with `initial_authorization_frontier = -1`. Cycles before lag `K` is satisfied operate in constrained mode, and no consequential outputs are finalized solely because the system is new. An alternative bootstrap basis is valid only if it is explicitly defined in immutable genesis data, independently verifiable, not added retroactively through later governance, and unable to weaken anti-collapse invariants.

---

### 3.6 Adaptive work targets (normative) [anchor: adaptive_work_targets_normative]

Cycle completion targets adapt to recent **deliberative work**, not raw activity volume.

#### 3.6.1 Observed work [anchor: observed_work]

For each completed cycle *r*:

W_obs[r] = V[r] + C[r]


Forced boundaries are included in `W_obs[r]`.

#### 3.6.2 Exponential moving average [anchor: exponential_moving_average]

An exponential moving average is maintained over observed work:

W_ema[r] = α · W_obs[r] + (1 > α) · W_ema[r>1]


Where `α` is a governance-defined smoothing parameter.

#### 3.6.3 Target computation [anchor: target_computation]

The deliberative work target for the next cycle is:

W_target[r+1] = clamp(round(s · W_ema[r]), W_min, W_max)


Where:
- `s` is a governance-defined scaling factor,
- `W_min` and `W_max` bound minimum and maximum target size.

Targets MUST be deterministic and replayable.

---

### 3.7 Participation thresholds (normative) [anchor: participation_thresholds_normative]

Participation telemetry MAY be tracked and derived deterministically, but participation thresholds MUST NOT gate cycle-close validity.

- **V_min[r]** MAY be maintained as a participation diagnostic derived from recent voting activity over a sliding cycle window and bounded by governance-defined floors/caps.
- `V_min` MAY inform monitoring, adaptive-target tuning policy, and governance alerts.
- A valid cycle seal MUST NOT be rejected solely because `V_since < V_min[r]`.

Let `V_recent[r]` be the number of distinct voting identities that cast at least one eligible vote across a governance-defined sliding window of recent cycles.

An optional telemetry update is:

V_min[r+1] = clamp(ceil(k · V_recent[r]), V_floor, V_cap)

Where:
- `k` is a governance-defined scaling parameter,
- `V_floor` and `V_cap` are governance-defined bounds.
### 3.8 Rate limits and cycles (normative) [anchor: rate_limits_and_cycles_normative]

Cycles define reset and recharge boundaries for all per-identity rate limits and action budgets.

- Rate limits apply independently of cycle sealing progress.
- Casting multiple votes does not increase V.
- Creating ideas, connections, or challenges does not advance cycle completion.

Rate limits MUST be configured such that:
- no single identity can advance cycles alone,
- small coordinated groups cannot accelerate cycles beyond Dmin,
- sustained influence requires participation across multiple cycles.

---

### 3.9 Scaling per-cycle allowances (normative) [anchor: scaling_per_cycle_allowances_normative]

To prevent rapid cycle churn from acting as a free reset, all per-cycle allowances and recharge amounts MUST scale deterministically with cycle size.

Per-cycle caps and recharge amounts are derived as functions of `W_target[r]` (or equivalently `W_ema[r]`), for example:

build_mana_cap[r] = floor(β_build · W_target[r])
vote_cap[r] = floor(β_vote · W_target[r])
delib_mana_cap[r] = floor(β_delib · W_target[r])


As cycles become smaller, per-cycle allowances MUST decrease proportionally.  
As cycles become larger, per-cycle allowances MAY increase proportionally within governance-defined bounds.

This ensures:
- rapid cycle advancement does not reset limits at machine speed,
- forced boundaries do not amplify throughput,
- influence accrues over time, not bursts.

Heavy or high-impact actions MAY require accumulation of allowances across multiple cycles. Scaling rules MUST be monotonic and deterministic.

---

### 3.10 Mana pools and persistence (normative) [anchor: mana_pools_and_persistence_normative]

Each identity maintains multiple mana pools as defined elsewhere in the protocol, including:

1. **Build / Growth Mana**
   - used for idea and connection creation,
   - recharged at cycle boundaries,
   - recharge amount scales with cycle size.

2. **Deliberation Mana**
   - earned exclusively through voting,
   - used for challenge creation,
   - per-cycle earning caps scale with cycle size.

Mana pools persist across cycles up to governance-defined caps.

During constrained record-and-recovery mode, Tempo mana MAY recharge to its small repair cap and may be spent only on the Tempo-only lane. Ordinary mana pools or allowances may still be derived and capped at structural boundaries for replay continuity, but they remain unspendable outside the constrained allowlist until the authorization frontier permits consequential effects.

---

### 3.11 Multi-cycle accrual (normative) [anchor: multi_cycle_accrual_normative]

Certain canonical actions require accumulated mana that cannot be earned within a single cycle under typical participation.

Action costs and caps MUST be set such that:
- heavy actions require accrual across multiple cycles,
- rapid or forced cycle advancement cannot be exploited to bypass intended scarcity.

Missed ordinary allowances from uncertified, constrained, or forced cycles MUST NOT accumulate into later burst capacity. No past ordinary rate-limit reset is backfilled. No invitation capacity, inviter maturation, carryover-cap increase, or suspension restoration is backfilled from a non-qualifying capacity period. Later certification MAY finalize outputs that were explicitly pending or provisional, but it MUST NOT validate actions that were forbidden when attempted and MUST NOT create stockpiles of unused ordinary mana or admission authority.

---

### 3.12 Participation collapse and degraded operation (normative) [anchor: participation_collapse_and_degraded_operation_normative]

If participation drops below the minimum required to resolve ordinary challenges or finalize consequential authority, the system enters constrained record-and-recovery mode if at least one eligible human can continue canonical Tempo repair.

In constrained record-and-recovery mode:
- the narrow Tempo-only lane remains available to eligible human `tempo_contributor` identities,
- forced structural cycles may occur when Dmax reaches `T_allow`, or when `structural_dmax_liveness_predicate` is true, and the work target is unmet,
- `W_target` may adapt downward through the EMA after structural boundaries,
- ordinary canonical writes, governance activation, POD/POINT finalization, lifecycle irreversibility, final-rank effects, and ordinary mana spendability remain blocked or provisional until certification and the lagged authorization frontier permit them.

True record-only halt occurs when canonical publication cannot proceed or zero eligible humans can produce the minimum human Tempo repair needed for replayable target-bound time truth claims and explicit Tempo-context evidence/challenge history. In true record-only halt, local/offline records may be preserved, but universal structural cycle advancement does not proceed. Passive machine evidence, publication timestamps, or scheduler observations cannot substitute for eligible human participation.

Population collapse never lowers authority requirements automatically. `K`, `T_beacon`, beacon diversity, independence, and stability requirements do not automatically shrink, even when `W_target` adapts downward.

---

### 3.13 Offline operation and merge semantics (normative) [anchor: offline_operation_and_merge_semantics_normative]

Offline or partitioned systems MAY maintain a mutable private Mindseed journal and MAY separately record exact signed publication candidates in an append-only local publication log. Neither lane advances universal canonical cycles.

Upon merge:
- events are merged via deterministic replay,
- cycle boundaries are re-derived from the merged event log,
- cycle indices may change, but canonical event ordering does not.

Offline publication MUST NOT:
- accelerate canonical time,
- bypass per-identity or per-cycle limits,
- retroactively alter cycle boundaries.

Upon ingestion of offline or delayed events into the canonical event log:
- events are preserved in canonical order,
- but events that would exceed per-cycle or per-identity limits MUST be marked non-effective until subsequent canonical cycle boundaries occur.

This guarantees that offline publication:
- cannot accelerate canonical time,
- cannot bypass per-cycle throughput limits,
- cannot retroactively alter cycle boundaries or boundary-scoped derivations.

Signed-candidate offline operation is therefore equivalent to delayed publication, not parallel progression. The private Mindseed journal is not a delayed canonical log: it is user-controlled noncanonical state outside conformance and MAY be edited, deleted, pruned, or compacted.



### 3.14 Prohibitions (normative) [anchor: prohibitions_normative]

Cycles MUST NOT be used to:
- determine truth or correctness,
- weight votes or identities,
- allocate or modify POD or POINT,
- depend on or reference trusted wall-clock time,
- infer real-world duration beyond adjudicated time claims.

---

### 3.15 Cycle export packs (normative) [anchor: cycle_export_packs_normative]

At each cycle boundary, the protocol MAY emit a deterministic **cycle export pack** derived from the canonical event log and applicable rulebooks.

A cycle export pack MUST record:
- cycle index,
- seal type (deliberative or forced),
- W_score, V, and C values,
- satisfaction of Dmin and/or Dmax predicates.

Cycle export packs:
- do not affect canonical state,
- are fully regenerable,
- are non-authoritative and optional for nodes.

Selection criteria for included content (e.g., top-N importance, neighbor depth, payload tiers) are defined by governance rulebooks and MUST be deterministic.

#### 3.15.1 Contents [anchor: contents]

Unless modified by governance, a cycle export pack MAY include:
- payload text and structured summaries for the most universally important ideas at that cycle,
- payload text and summaries for the most relatively important ideas connected to those ideas,
- selected historical context necessary to interpret importance relationships.

Selection criteria, limits (e.g., top-N, neighbor depth K), and included payload tiers are defined by governance rulebooks and MUST be deterministic.

#### 3.15.2 Properties [anchor: properties]

Cycle export packs:
- do not affect canonical state,
- do not alter rankings, lifecycle_state, or eligibility,
- are fully regenerable from the canonical event log and rulebooks,
- are not required for validation or replay.

Different nodes MAY store different subsets of export packs without affecting correctness.

#### 3.15.3 Purpose [anchor: purpose_2]

Cycle export packs exist to:
- enable meaningful offline browsing and search,
- accelerate onboarding and historical inspection,
- preserve snapshots of “what mattered” without requiring full-log replay.

Export packs MUST NOT be used as inputs to canonical computation.




## 4. truth, evidence, prediction, and verification (renumber from this section onward) [anchor: 4_truth_evidence_prediction_and_verification_renumber_from_this_section_onward]

### 4.1 truth-claim ontology and subtypes [anchor: truth_claim_ontology_and_subtypes]

Truth claims represent assertions about the world that can, in principle, be evaluated for accuracy. They describe what an agent says is true - not what is true in an absolute sense - and form the backbone of the system's epistemic processes. A truth claim expresses a proposition about reality, grounded in observations, measurements, patterns, mechanisms, predictions, tests, or broader model-level integrations. Truth claims accumulate evidence, support or contradict other claims, gain or lose certainty over time, and can be confirmed, falsified, or revised through structured deliberation and testing. They are the only idea type that participates directly in the evidence rails defined in this chapter.

The protocol defines several truth-claim subtypes, each specifying a distinct epistemic role and determining which forms of evidence may support it. Existence and boundary claims assert that a phenomenon, category, or boundary exists - for example, that a species exists or that a property marks a meaningful distinction. These claims establish the ontological foundations upon which other truth claims rely. Observation and measurement claims report singular events or quantifiable states: that something was seen, detected, measured, or recorded by an instrument or observer. These serve as the lowest-level empirical evidence and feed directly into evidence rails.

Pattern and correlation claims assert regularities among observed phenomena. They propose relationships, trends, or statistical associations derived from multiple observations. They do not assert causation but may support or challenge causal claims. Causal and mechanistic claims assert that one phenomenon produces or influences another through a mechanism, process, or structure. They require stronger supporting evidence and may be challenged by contradictory patterns, failed predictions, or incompatible mechanistic descriptions.

Prediction claims assert that a particular event or state will occur in the future or within a specified world-time range. Predictions may include an optional prediction_target_date, allowing implementations to surface predictions that have become due for evaluation. Predictions are confirmed or falsified by test outcomes or real-world actions recorded later in ledger time. Test and validation claims describe the results of checking or measuring whether other claims are accurate. They may confirm or contradict predictions, pattern claims, or observations, and form a critical part of the epistemic feedback loop.

Finally, model and integration claims synthesize observations, patterns, mechanisms, and predictions into higher-level explanatory or predictive frameworks. These claims represent theories, models, or integrated interpretations and may support or challenge many other truth claims simultaneously. They stand at the top of the evidence rails: they integrate evidence from lower subtypes but must also withstand long-term predictive verification.

Truth-claim subtypes must be explicitly declared at creation and dictate which kinds of ideas may be connected to the claim as evidence using relative_importance connections whose usage is evidence_for or evidence_against. A claim with an incompatible subtype-evidence relationship is invalid and has no effect, though it remains part of the historical record. All certainty updates, contradictions, and verification processes depend on these subtype definitions and the evidence rails described in later sections. This structure allows the system to maintain a coherent epistemic hierarchy in which evidence, prediction, and model dynamics operate predictably and consistently across all nodes.


### 4.2 evidence rails and structural constraints [anchor: evidence_rails_and_structural_constraints]

Every truth claim maintains its own evidence rail: a deterministic structure that records what evidence could, in principle, exist for the claim, what evidence has actually been provided, how those two sets compare, and which certainty band the claim should occupy given the current state of knowledge. Evidence orderings do not define inference rules or directional epistemic flows between claim types. Instead, they provide a stable, replayable frame for evaluating certainty based on two spectra - the spectrum of potential evidence and the spectrum of actual evidence - and the gap that separates them. All conformant implementations MUST reconstruct identical evidence rails when replaying the same event history.

For each truth-claim subtype, the protocol defines a spectrum of potential certainty, specifying the maximum certainty a claim of that subtype can reasonably achieve. Some subtypes allow high certainty (for example, measurement/state claims about present conditions), while others have inherently lower ceilings (for example, long-range predictions or complex model-level integrations). These ceilings and band definitions are maintained in a companion specification and MUST be identical across all conformant nodes at any cycle boundary.

Each truth claim also possesses a spectrum of potential evidence, which consists of hypothetical evidence ideas representing the full range of what evidence for that specific claim could look like - from extremely weak and tentative up through moderate, strong, and ideal or near-conclusive patterns. This potential evidence is expressed explicitly as ideas in the graph - often as hypothetical observations, measurements, tests, replications, contradictory trials, or high-standards verifications - and MUST be generated before any actual evidence is considered or placed. For each truth-claim subtype, the potential-evidence spectrum describes, in increasing order of strength, the kinds of observations, tests, replications, mechanisms, or model outcomes that would gradually increase or decrease confidence in the claim. Potential evidence is not limited to raw data categories. It MAY also include robustness factors such as independent replication by multiple trusted third parties, observation by diverse groups, widely witnessed or live-streamed events, scrutiny by recognized experts, durable archival records, or high-visibility public confirmation. Together, these hypothetical evidence ideas define a neutral epistemic scaffold for the claim, covering weak through ideal evidence, and anchor the spectrum of potential certainty before any identity, agenda, or real-world data enters the ordering.

Running in parallel is the spectrum of actual evidence, consisting of all evidence ideas that have actually been connected to the claim via a relative_importance connection whose usage is evidence_for or evidence_against. Each such evidence idea is itself a truth claim whose subtype determines its epistemic character (for example, observation, pattern, causal mechanism, prediction, test, model integration). No additional connection metadata is required to classify the kind of evidence; the evidence idea's subtype determines its epistemic role. Evidence is placed on the evidence spectrum by comparing each actual evidence idea to the predefined potential evidence. Placement is not automatic; it is determined through explicit placement challenges in which agents deliberate about how closely a given piece of evidence matches the strength, clarity, or robustness of the hypothetical ideal. The result is a replay-stable ordering of actual evidence along the same spectrum that defines the claim's potential evidence.

The epistemic weight of a truth claim is derived from the relationship between these two spectra. The system compares the strongest actual evidence to the strongest potential evidence, evaluates how much of the ideal pattern has been instantiated, and measures the size of the remaining gaps. Once a stable relationship is formed, an agent MAY issue a certainty-band challenge, proposing that the claim be placed in a specific certainty band within its subtype's allowed spectrum. Voters in such a challenge MUST consider both spectra: the quality and diversity of actual evidence, the extent to which key robustness criteria have been met, and how closely the provided evidence approaches the idealized evidence templates. The outcome of the challenge assigns the claim's canonical certainty band until additional evidence or new challenges justify a change.

Evidence orderings are fully transparent and must be identical under replay. All elements - hypothetical potential evidence, actual evidence ideas, usage-tagged relative_importance connections identifying them as evidence, spectrum placements, and certainty-band challenge outcomes - are recorded as explicit events. Nodes MUST NOT infer new evidence, assign certainty automatically, generate placements implicitly, or apply implementation-specific heuristics. Invalid or irrelevant evidence edges remain visible in the historical record but exert no effect on the active spectrum of actual evidence. Only explicit placements and explicit certainty-band challenges determine the ordering's operative structure.

This architecture ensures that certainty is a function of what the system could know, what it does know, and what agents judge to be the correct interpretation of the relationship between the two. It produces a stable, challengeable, and deterministic epistemic layer upon which higher-order reasoning, predictions, contradictions, and truth-governance decisions rest.

### 4.3 certainty, certainty bands, and certainty updates [anchor: certainty_certainty_bands_and_certainty_updates]

Certainty is a structured, replayable assessment of how strongly a truth claim is supported relative to the best evidence that could reasonably exist for its subtype. Certainty does not measure absolute truth. It measures how well the claim is performing, in the present moment, against its spectrum of potential evidence, its spectrum of actual evidence, and the outcome of deliberative certainty-band challenges. Certainty MUST always be derived from explicit recorded events, not inferred heuristics, ensuring that all conformant nodes reconstruct identical certainty states when replaying the event history.

Each truth-claim subtype possesses a defined certainty ceiling, which represents the highest achievable certainty for that class of claim. Measurement/state claims, for example, may admit high certainty when accompanied by repeatable measurements, whereas long-range predictions or large causal models have inherently lower ceilings. These ceilings-along with the set of certainty bands available to each subtype-are maintained in a companion specification and MAY evolve through governance. All nodes MUST adopt the active certainty ceilings and band definitions at each cycle boundary.

A certainty band is a discrete interval along the certainty spectrum of a particular subtype. Certainty bands define coarse epistemic zones such as very low certainty, low certainty, moderate certainty, high certainty, and near-ceiling certainty, though the precise number and naming conventions are governed by the certainty specification. Bands ensure that certainty updates remain legible to humans and stable under replay. They also prevent implementations from diverging by applying continuous or private numerical scoring systems. At any time, each truth claim has exactly one canonical certainty band.

Certainty updates occur only through explicit events - specifically, certainty-band challenges. These challenges propose moving a claim from its current band to a different one. The justification MUST reference the claim's evidence rail, including the strongest potential evidence, the strongest actual evidence, the placement of each piece of actual evidence along the spectrum, and the gap between the achieved and ideal evidence states. Voters in the challenge evaluate whether the proposed certainty band appropriately reflects the relationship between potential and actual evidence, and whether recent changes - such as new evidence or newly recognized contradictions - justify moving the claim up or down. The outcome of the certainty-band challenge determines the claim's new certainty band.

Certainty MAY also be indirectly affected by contradictory evidence, failed predictions, successful tests, or model breakdowns, but only when these events trigger a certainty-band challenge. The system MUST NOT automatically adjust certainty in response to new evidence. Instead, only humans (or AI assistants acting within their permitted advisory scope) initiate the challenge that proposes the update. This ensures that all certainty changes remain transparent, contestable, and grounded in explicit reasoning.

Because certainty is banded rather than continuously numerical, small or ambiguous evidence changes often leave the claim in the same band, while large or decisive evidence changes prompt a move. Certainty bands are replay-stable: given the same challenges and outcomes, all nodes derive the same band. If a challenge outcome is later overturned by governance - for example, if the evidence-placement rules for a subtype are changed - nodes adjust the certainty band only upon replaying the updated rules from the relevant cycle boundary.

Finally, the certainty of a truth claim influences but does not determine downstream judgments. High certainty strengthens the claim's role in later evidence evaluation, truth challenges, and predictions, but certainty does not grant absolute authority. The system always treats truth as an evolving, deliberative construct shaped by ongoing evidence, contradictions, and long-term performance. Certainty bands provide the stable scaffolding upon which this evolution occurs, ensuring both transparency and determinism across all implementations.

### 4.4 contradictory evidence and conflict resolution [anchor: contradictory_evidence_and_conflict_resolution]

Contradictions arise when a truth claim receives actual evidence that is incompatible with its stated proposition, its subtype, or its strongest potential evidence templates. A contradiction does not automatically falsify a claim or reduce its certainty; instead, it becomes part of the claim's evidence rail and triggers the deliberative mechanisms that determine whether the contradiction should change the claim's certainty band. Contradictions are therefore recorded, structured, and resolved through explicit events rather than implicit or automated inference, ensuring deterministic replay across all conformant implementations.

Contradictory evidence is expressed as distinct evidence ideas - often observation, measurement, test, or pattern claims - that, when placed on the spectrum of actual evidence, align closely with one or more hypothetical disconfirming templates defined in the spectrum of potential evidence. The strength of a contradiction depends on where it is placed on the spectrum: weak contradictions sit near the lower tiers or lower spectrum positions, while decisive contradictions sit close to the strongest potential disconfirming evidence. Placement challenges determine this position and ensure that contradictory evidence is not misclassified or dismissed without justification. When contradictory evidence accumulates, the spectrum of actual evidence visibly diverges from the structure of the supporting potential evidence, making the contradiction clear to any observer.

A contradiction becomes epistemically active when it motivates a **certainty-band challenge**. An agent MAY propose that, given the current evidence rail, the claim should be moved to a lower certainty band or, in extreme cases, to the lowest band available for its subtype. The challenge MUST explicitly reference the strongest contradictory evidence, the evidence placements, and the gaps in the potential-evidence structure that are now unfulfilled or violated. Voters then evaluate whether the contradiction meaningfully undermines the claim's epistemic standing. If so, the certainty-band challenge reduces the claim's certainty; if not, the claim remains unchanged, though the contradictory evidence remains part of the record.

Not all contradictions are decisive. Some arise from noisy or ambiguous observations, measurement errors, or context-sensitive patterns that do not genuinely address the core of the claim. Other contradictions may be overridden by stronger supporting evidence already placed on the spectrum. The system therefore relies on deliberation to distinguish between genuine disconfirmation and apparent contradiction. This deliberative process ensures that certainty adjustments remain transparent, justified, and consistent across all implementations, rather than emerging from hidden or heuristic rules.

Contradictions interact differently with different truth-claim subtypes. For prediction claims, a contradiction is typically registered when the predicted event fails to occur by the specified window or when a test result directly opposes the predicted outcome. For measurements and observations, contradictions occur when independent or replicated measurements produce incompatible results. For causal or model claims, contradictions emerge when predicted mechanisms do not produce expected outcomes or when rival mechanisms provide stronger explanatory support. Each subtype's potential-evidence templates define what counts as a decisive contradiction for that subtype.

Importantly, structurally non-falsifiable claims - those for which no realistic disconfirming potential evidence can be defined - still accept contradictory evidence, but such evidence often lands low on the spectrum and rarely motivates a shift out of the lower certainty bands. This behavior is intentional: the system allows the claim to be expressed as a truth claim while simultaneously revealing its epistemic limitations through persistent gaps in potential evidence. Over time, users often duplicate such claims into conceptual ideas, where their interpretive and meaning-oriented value can be assessed without relying on empirical contradiction.

Contradictions remain permanently in the historical record and contribute to the epistemic genealogy of the claim. Even if subsequent evidence resolves the contradiction, its presence influences the overall epistemic narrative and may shape future placement challenges or reliability assessments for the identities involved. The system never deletes contradictory evidence; it only determines, through deliberation, how much weight it should carry in shaping the claim's certainty band. This guarantees a fully transparent, challengeable, and replayable structure for understanding how claims succeed or fail under scrutiny.

### 4.5 prediction evaluation and outcome marking [anchor: prediction_evaluation_and_outcome_marking]

Prediction claims assert that a specific event, state, or measurement WILL occur in the future or within a defined world-time window. Predictions function as forward-looking truth claims whose certainty is determined not only by supporting evidence but also by eventual real-world outcomes. The protocol treats prediction evaluation as a deterministic, multi-stage process involving potential-evidence templates, actual evidence, explicit test-result claims, and outcome-marking events. Predictions do not automatically succeed or fail; their evaluation MUST always pass through explicit evidence submissions and challenge outcomes to ensure replayability and transparency.

A prediction claim MAY optionally include a `prediction_target_date`, representing the moment or interval after which evaluators expect the predicted condition to be testable. This field is not required for protocol correctness and does not affect epistemic semantics; it exists solely to help implementations surface predictions that are due for evaluation. When the target date passes - or when the predicted conditions otherwise become testable - agents MAY submit **test-result truth claims** that describe whether the predicted outcome did or did not occur. These test-result claims are ordinary truth claims that connect to the prediction via the evidence connection type.

Prediction evaluation is governed by the same evidence-ordering structure used for all truth claims. Each prediction contains a **spectrum of potential evidence** that defines what strong confirmation and strong disconfirmation would look like. Strong confirming potential evidence might include accurate measurements of the predicted outcome, independent replications, or widely observable events that align with the prediction's content. Strong disconfirming potential evidence might include robust observations showing the predicted event did not occur, failed tests, contradictory measurements, or replicated outcomes that conflict with the predicted state. These hypothetical templates anchor the prediction's certainty spectrum and define what constitutes a decisive outcome.

When actual evidence arrives - typically as test-result claims or observations - it is placed along the **spectrum of actual evidence** through placement challenges. If the predicted event occurs clearly and measurably, the strongest actual evidence will align with the confirming potential evidence, raising the justification for a higher certainty band. If the event does not occur, or if evidence contradicts the prediction's content, the strongest actual evidence may fall near the disconfirming potential evidence, justifying movement to lower certainty bands. Predictions with partial fulfillment (for example, ambiguous or mixed outcomes) produce actual evidence spread across the spectrum, requiring deliberation to determine the correct certainty band.

Outcome marking occurs when an agent raises a **prediction-outcome challenge** or a general certainty-band challenge proposing a new certainty band for the prediction. If the predicted event clearly occurred or clearly failed, voters SHOULD place the prediction in a band reflecting decisive confirmation or decisive disconfirmation. If the outcome is ambiguous, voters MUST consider all relevant evidence placements, gaps, and potential-evidence templates when determining whether the prediction should move upward, downward, or remain unchanged. The challenge outcome sets the prediction's canonical certainty band until new evidence or subsequent challenges justify another change.

Failed predictions have additional long-term epistemic consequences. When a prediction is placed into a decisively low certainty band due to strong disconfirming evidence, this outcome MAY reduce the epistemic reliability of the identity that authored the prediction, as described later in this chapter. Conversely, accurate predictions - especially those that required non-obvious insight or incorporated complex models - MAY strengthen the reliability and future credibility of the identity or model involved. The system does not reward or punish based on belief content; it records accuracy and allows epistemic performance to accumulate as evidence over time.

Importantly, a prediction that has reached its evaluative window but has received no relevant test-result evidence does not automatically change certainty. Lack of evidence MAY motivate a challenge, but no automatic outcome occurs. Instead, the absence of expected evidence becomes part of the spectrum of actual evidence, often landing as a weak contradiction or as a gap, depending on how the prediction defined its potential evidence. This ensures that predictions are not marked as correct or incorrect by default and that all certainty changes arise from explicit events rather than implicit rules.

Prediction evaluation therefore forms the system’s clearest and most actionable demonstration of the scientific method. It provides a visible, replayable record of foresight, verification, contradiction, and uncertainty resolution. By requiring explicit evidence, explicit placement, and explicit challenges, the system preserves transparency and determinism while encouraging agents to make careful, meaningful, and testable claims about the future.

### 4.6 test and validation claims [anchor: test_and_validation_claims]

Test and validation claims represent direct attempts to evaluate the accuracy of other truth claims. They document experiments, measurements, observational campaigns, audits, replications, counter-tests, or any procedure designed to verify or falsify a claim. Because they serve as the primary mechanism through which predictions, causal claims, and measurement claims are evaluated, test-result claims occupy a central role in the evidence rails of many truth claims. All test-result claims MUST be explicit truth claims with subtype `test_result`, ensuring they can be placed on evidence spectra, challenged, and deterministically replayed.

A test-result claim MUST describe the method, conditions, and outcome of a procedure intended to evaluate another claim. The level of detail varies according to the chosen description tier, but the minimum requirement is that an observer reading the full description should understand what was tested, how it was tested, and what the outcome was. Tests MAY be simple (a single measurement under controlled conditions) or complex (a multi-stage audit or replication study). The system does not enforce methodological standards; instead, the evidence rail incorporates robustness criteria - such as independence, repeatability, transparency, and public verifiability - into the potential-evidence templates. These criteria ensure that higher-quality tests are correctly placed near the strongest ends of the evidence spectrum.

Test-result claims connect to the claim they evaluate via the **evidence** connection type. A single test MAY evaluate multiple claims at once, and a single claim MAY accumulate many test-result ideas over time. When a test-result claim contradicts or supports a prediction, causal mechanism, or measurement claim, it is placed along the evidence spectrum through a placement challenge. This placement reflects both the methodological quality of the test and the clarity of its outcome. Tests with ambiguous or mixed results often appear mid-spectrum, while tests with decisive outcomes - especially those that meet robust verification criteria - are placed near the strongest confirming or disconfirming potential evidence.

Replications are treated as independent test-result claims. If multiple replications converge on the same outcome, their combined spectrum placements often constitute strong confirming or disconfirming evidence. Failed replications do not automatically undermine the original test; they simply add contradictory evidence that MUST be resolved through placement challenges and, when appropriate, certainty-band challenges. This ensures that replication crises, methodological disputes, and competing interpretations of data are resolved through explicit deliberation rather than hidden heuristics.

Some test-result claims introduce **meta-evidence** by evaluating the validity of other tests. For example, an audit showing that a past test used faulty equipment, non-blinded procedures, or fabricated data is itself a test-result claim and is placed on the evidence spectrum for both the original test and the claim that depended on it. Meta-tests therefore propagate epistemic consequences through the network, clarifying which evidence is trustworthy and which is compromised.

For prediction claims, a test-result claim often represents the decisive outcome: the predicted event either occurred or did not. If the prediction included a `prediction_target_date`, tests executed after that date are typically placed as strong confirming or disconfirming evidence. For measurement or observational claims, test results may support or contradict the original measurement by performing the same procedure under controlled conditions or by showing that the measuring instrument behaved incorrectly or inconsistently.

Test-result claims MUST be entirely explicit to ensure replayability. Nodes MUST NOT infer or automatically generate tests. They may only interpret tests that agents create as ideas and connect via evidence links. The protocol does not define what constitutes a valid scientific method; it simply requires that any evaluation of a truth claim be recorded through test-result ideas that undergo the same placement and challenge processes as all other evidence. This ensures that the entire epistemic process - tests, replications, audits, confirmations, and failures - is visible, contestable, and stable under replay.

In summary, test and validation claims operationalize the system's epistemic dynamics by generating structured, challengeable evidence for or against truth claims. They enable the system to evaluate predictions, confirm measurements, scrutinize causal claims, and revise certainty through transparent, deterministic processes. Test-result claims are not privileged by definition; their epistemic force depends entirely on their placement within each claim's evidence rail and the deliberative challenges that interpret their significance.

### 4.7 epistemic reliability of identities [anchor: epistemic_reliability_of_identities]

Epistemic reliability is not a standalone metric, score, or reputation system. **POD** is a living, identity-bound share derived from eligible canonical contributions and their current routing through universal importance. It MAY provide an auditable signal of current contribution, but it is not a permanent credential or an exclusive, cumulative measure of whether an identity is reliable. An identity's truth claims, predictions, measurements, tests, conceptual contributions, and arguments remain directly inspectable through their epistemic lineages.

An identity's current POD rises not for being correct in an absolute sense, but when its eligible contributions receive more routing through the system's universal-importance and token-flow mechanics. A prediction that proves correct over time, a measurement that is repeatedly confirmed, a causal claim that withstands contradictory evidence, or a conceptual idea that becomes foundational may contribute more current POD when the canonical importance structure reflects that significance. Contributions that become less universally important, lose routing eligibility, or are disconnected from the living graph contribute less or no current POD, without imposing penalties, punishments, or coercive restrictions.

Because POD flows through **universal-importance ranking** and eligible **relative-importance pathways**, the current POD share reflects how eligible contributions participate in the living universal map. An identity may receive current POD through prediction claims, measurements, conceptual work, evidence, challenges, votes, or verified actions according to the token specification. These contribution types remain distinguishable in the event history; POD does not collapse their meaning into an authoritative reliability score.

Epistemic reliability MUST NOT be inferred from POD alone, and POD MUST NOT grant voting advantages, governance authority, structural privileges, or special visibility. A high current POD share means only that the identity's eligible contributions currently receive more universal-importance routing; it does not prove that the person is correct, trustworthy, or entitled to deference. An identity with low or zero current POD remains fully capable of creating ideas, raising challenges, voting, and participating in governance whenever otherwise eligible.

AI agents also accumulate reliability signals, but these exist only within the AI-map and do not generate POD. Their simulated reliability helps users assess which AI models to consult but does not affect governance, canonical truth processes, or token distributions. Only eligible canonical events authored by living verified human identities may receive POD.

The ledger preserves every identity's epistemic lineage and the POD derivation produced for each completed historical cycle. When claims are upheld or overturned years later, predictions succeed or fail, evidence accumulates or dissolves, lifecycle state changes, or universal importance is reordered, current and future POD recomputes from the changed canonical state and may rise, fall, or become zero. No past contribution or completed-cycle result is erased or rewritten. The durable object is the auditable history; the current POD share remains living.

### 4.8 epistemic lineage and long-term truth dynamics [anchor: epistemic_lineage_and_long_term_truth_dynamics]

The epistemic lineage of a truth claim represents its full recorded history: how it was created, what evidence accumulated around it, the certainty bands it occupied over time, the predictions it supported or contradicted, the tests performed to evaluate it, and the challenges it endured. This lineage is not an auxiliary feature; it is part of the canonical universe itself. Every claim carries with it a chronologically ordered sequence of events that together define its epistemic trajectory. Conformant nodes MUST reconstruct the same lineage by replaying the ledger, ensuring that the system preserves not only the present state of knowledge but also the entire path by which that state was reached.

Truth dynamics unfold across long periods of time. A claim may enter the ledger with low certainty, rise as evidence accumulates, fall when contradictions appear, recover when tests resolve ambiguity, or stabilize as a widely confirmed statement. Predictions may be falsified decades after they are made. Causal claims may accumulate supporting mechanisms across many chains of evidence. Model claims may strengthen as they subsume additional observations or weaken as rival explanations outperform them. The system MUST treat all such changes as explicit, deliberative transformations recorded through challenge outcomes, never as automatic or heuristic updates. This ensures that the evolution of truth in the system remains transparent, replayable, and subject to contestation.

Epistemic lineage provides a key lens for understanding why truth claims occupy their present certainty bands. A user confronting a claim does not see a static value; they see the sequence of supporting evidence, contradictory evidence, predictions upheld or failed, tests performed, and challenge outcomes that produced that value. This prevents the system from functioning as a black box and ensures that epistemic authority emerges only from the recorded structure of reasoning, not from hidden processes or accumulated status. The system therefore functions simultaneously as a live truth-determination mechanism and as a historical archive of scientific and philosophical reasoning.

Long-term dynamics often reveal patterns that short-term analysis cannot. Claims with high early certainty may drift downward as broader evidence accumulates. Claims that begin weakly may rise over years of supporting tests and replications. Some models grow into durable explanatory structures; others collapse after a single decisive test result. The system MUST preserve all such long-range evolutions, even when they extend across multiple snapshots and governance cycles. Nodes MUST NOT attempt to optimize lineage storage at the cost of semantic fidelity; lineage is integral to the system’s epistemic coherence and replayability.

Epistemic lineage also interacts with **importance**. Highly important truth claims - those that influence many other ideas, support major conceptual structures, or anchor significant actionable pathways - tend to produce large POD flows when they are confirmed or overturned. When a central claim changes certainty, the importance dynamics of ideas connected to it shift as well, causing POD flows to rebalance across the network. This interplay ensures that the system does not only capture whether a claim is true, but also how much it matters. A trivial falsification produces only local effects; the confirmation or undermining of a major claim can produce global epistemic reverberations.

Finally, the system MUST treat epistemic lineage as a first-class object for visualization, introspection, and learning. Implementations SHOULD allow users to scroll backward through a claim's certainty history, view past evidence placements, and explore how related ideas evolved in parallel. By exposing the full genealogy of each claim, the system enables users to understand the deeper structure of reasoning: why claims rise or fall, how evidence interacts across subtypes, how predictions succeed or fail, and how long-term deliberation produces outcomes that no single individual could reach alone. Epistemic lineage therefore transforms the protocol from a static truth-recording tool into a dynamic, unfolding map of collective reasoning over time.

### 4.9 Human Adoption Required for Canonical Events [anchor: human_adoption_required_for_canonical_events]

Invariant.
No canonical event may be created solely as the result of an AI operation. All canonical events MUST originate from deliberate human action. For every canonical event:

A single human identity MUST be recorded as the canonical author.

The human author MUST have been shown the complete and final event contents prior to creation, exactly as they will appear in the canonical universe.

The human author MUST have actively confirmed creation of the event through an authenticated action bound to their identity.

Any AI involvement in drafting, proposing, summarizing, or assembling the event MUST be recorded only as optional, non-canonical provenance metadata. Such metadata MUST NOT constitute authorship and MUST NOT alter the requirement that canonical events are authored by humans.

Implications.
AI systems MAY assist humans by generating proposals, drafts, comparisons, or structured representations. These MAY inform human decisions, but they MAY NOT directly produce canonical history. The canonical universe remains anchored in human agency; AI activity is advisory unless and until a human explicitly adopts and confirms it.

Rationale.
This invariant preserves the integrity, legibility, and long-term legitimacy of the canonical universe. Canonical history MUST remain attributable to human choices, not automated systems. By requiring explicit review and confirmation of every canonical event, the organism ensures that the record of truth, importance, and action remains grounded in accountable human authorship and cannot be silently rewritten or flooded by autonomous processes.

### 4.10 Scoped judgments and public ideas [anchor: scoped_judgments_and_public_ideas]

All ideas in the canonical universe are public entities. The protocol does not support private canonical ideas.

Scope applies to judgments, not to idea existence. In particular:

- Challenges MAY define eligibility scopes for participation (e.g., universal, tribe, personal).
- Voting eligibility MAY be restricted according to scope.
- Verdicts derived from scoped challenges produce scoped judgments.

Scoped judgments determine how importance, truth, or action outcomes are computed within a given scope, but they do not affect the public existence of the underlying ideas.

Within canonical state, scope MUST be interpreted as overlay state only (relative-importance overlays and scoped display override overlays) over the universal canonical substrate.

Tribe scope denotes member-gated participation in challenges whose resulting judgments are publicly visible and auditable. Tribe scope SHALL NOT imply private ideas, private maps, or hidden epistemic structures.

Personal/private importance denotes local owner-controlled judgments that do not affect the canonical universe. Publishing or sharing a view of a private ordering does not by itself convert the ordering into a canonical rank list or challenge verdict; any future canonical adoption requires an explicit protocol transition.

The visibility of judgments and their payloads MAY be constrained by jurisdictional projection rules, but the existence of challenges, arguments, and verdicts MUST remain legible.




## 5. importance, relevance, and relative-importance flows [anchor: 5_importance_relevance_and_relative_importance_flows]

### 5.1 universal importance and the 20-axis foundation [anchor: universal_importance_and_the_20_axis_foundation]

Universal importance represents the system’s shared, public assessment of how much an idea matters for life, flourishing, survival, and long-term outcomes. It does not measure urgency, correctness, popularity, or personal preference. Instead, it expresses what agents say they believe is important, structured across the two fundamental poles that define the system’s value architecture: the **currently existing individual human**, and **all life, intelligence, and consciousness in the universe through time**. These two poles anchor the entire importance model, preventing ideological drift and ensuring that every importance judgment is expressed in relation to both the immediate human experience and the full, long-range continuity of conscious existence.

Universal importance is defined through **20 axes**, formed by crossing four orientations of importance with five time horizons. The four orientations are: importance *to the currently existing individual human* and importance *for the currently existing individual human*; and importance *to the collective* and importance *for the collective*, where **the collective is defined strictly as all life, intelligence, and consciousness in the universe through time**, not any particular society, nation, species, ideology, political group, or culture. This ensures that "collective importance" refers only to the broadest possible scope of moral and existential concern, never to narrower human factions.

The universal orientation values are `important_to_current_individual`, `important_for_current_individual`, `important_to_collective`, and `important_for_collective`. These values belong to universal importance profiles and MUST NOT be used as the axis vocabulary for ordinary relative-importance ranking contexts unless a future rulebook explicitly defines a deterministic projection.

The five time horizons - near-term, mid-term, long-term, very long-term, and trans-generational or civilizational timescales - encode the temporal dimension of importance. Each idea therefore has a value on each of the 20 orientation-time axes reflecting how important agents judge it to be for the currently existing individual human or for all life and consciousness, across short and long temporal scales. These axes are maintained through open challenge-based ranking procedures that allow ideas to rise or fall according to recorded reasoning.

Universal importance has two related outputs that MUST remain distinguishable. First, each of the 20 axes is an ordinal rank list produced only through pairwise importance challenges and verdicts. Second, the protocol derives one overall universal ordering from those 20 ordinal positions. For an idea `x`, let `p_i(x)` be its one-based position on universal axis `i`. The canonical aggregate comparison key is the exact integer sum `universal_position_sum(x) = Σ_i p_i(x)` across all 20 axes; `universal_position_mean(x) = universal_position_sum(x) / 20` MAY be displayed as the exact average. Because every complete profile has exactly 20 axes, the sum and exact mean induce the same ordering. Implementations MUST compare the integer sum, MUST NOT use implementation-dependent floating-point rounding as a rank input, and MUST apply the active rulebook's deterministic tie-break for equal sums. Users can always inspect the underlying 20-axis profile of an idea to understand why its aggregate position changed.

After deterministic baseline insertion, changes in universal importance occur only through explicit importance-axis challenges. A lower-ranked candidate challenges a higher-ranked target on one declared universal axis, and participants provide arguments grounded in that orientation and time horizon. If the challenger wins and remains below the target when the verdict applies, it moves immediately above the target on that axis; the derived universal aggregate then updates automatically. When a highly important idea shifts, this movement may propagate through relative-importance pathways, causing POD flows and significance flows to rebalance across the network over time.

Anchoring universal importance to the poles of the currently existing individual human and all life, intelligence, and consciousness in the universe through time ensures that the system’s value structure remains both deeply human and fundamentally cosmological. It ties importance to the lived reality of present human beings while also encompassing the widest possible scope of moral and existential consideration. This dual anchoring is what allows universal importance to serve as the basis for POD generation and for the system’s long-term architecture of meaning, relevance, and action.

### 5.1.1 derived universal ranks and rank history [anchor: derived_universal_ranks_and_rank_history]

Universal-importance ranks (all 20 axes) and the overall universal rank are **derived state**. They MUST be computed by deterministic replay of the canonical event log under the active rulebook set at each cycle boundary. They MUST NOT be authored as canonical facts.

At every canonical snapshot height, every idea MUST have a defined universal rank on each of the 20 axes, including immediately after creation. For ideas with no applicable challenge outcomes on an axis, baseline ordering MUST be assigned deterministically (including deterministic tie-break rules) as defined by the active rulebooks. Implementations MAY cache or materialize rank lists, but such materializations are non-authoritative and MUST match deterministic replay.

The overall universal rank is the idea's ordinal position after all complete universal profiles are sorted by ascending `universal_position_sum` and the deterministic tie-break defined by the rulebook in force at the applicable cycle boundary. Only axis positions change through importance challenges. The sum, exact mean, and overall universal rank update automatically as derived state and are not directly challengeable scalar scores.

Universal-rank history is the sequence of these derived ranks across snapshot heights. It is immutable and reproducible because it is uniquely determined by the canonical event log and the rulebooks active at each cycle boundary.

### 5.2 importance rank kinds, participation scopes, and private ordering [anchor: scoped_importance_universal_tribal_and_personal_contexts]

The system defines two distinct public rank products that reuse one pairwise comparison grammar:

| Rank product | Reference | Axes | Decision procedure | Electorate | Visibility | Effect |
| --- | --- | --- | --- | --- | --- | --- |
| universal importance | the two fixed universal poles encoded by the four universal orientations | 20 | pairwise challenge and verdict | eligible public humans | public | universal axis lists and derived overall universal rank |
| public relative importance | any public reference idea | 10 | pairwise challenge and verdict | eligible public humans | public | that reference-relative context only |
| tribe-relative importance | the tribe anchor idea | 10 | pairwise challenge and verdict | eligible tribe members | public | that tribe overlay only |
| individual-private relative importance | any permitted private or canonical reference | 10 | direct owner ordering | one owner; no vote | permissioned | noncanonical private state only |

`rank_kind` distinguishes `universal` from `relative`. Participation scope determines who may create challenges, contribute arguments, and serve as jurors. In canonical state, public participation continues to use the existing `scope = universal` value and tribe participation uses `scope = tribe`; the word `universal` in the scope field means the globally eligible public electorate and MUST NOT be used by itself to infer `rank_kind = universal`.

Universal importance is the distinct 20-axis product defined in Section 5.1. It is not created by pointing an ordinary relative-importance connection at an idea whose title resembles "humanity", "the individual", or "the collective". A universal-axis context is keyed by `(rank_kind = universal, universal_orientation, timeframe)`.

Relative importance uses a **10-axis framework**. For any reference idea R, an idea X may be ranked by whether X is `important_to_reference` or `important_for_reference` across `near_term`, `mid_term`, `long_term`, `very_long_term`, and `trans_generational`. A canonical relative context is keyed by `(rank_kind = relative, reference_idea_id, usage, scope, axis, timeframe)`. These ranks are not global idea ranks and do not contribute to overall universal rank.

In public-relative contexts, any eligible public identity may enter the ordinary challenge and vote-session process. In tribe-relative contexts, the tribe anchor is the reference idea and eligibility is restricted to eligible tribe members. Tribe results remain publicly inspectable but never directly modify universal importance, another public-relative context, or POD injection.

Individual-private ranking uses the same ten-axis meaning without canonical challenge ceremony. The owner directly selects and revises the order and MAY attach optional importance arguments. No challenge, juror selection, or vote is required. Private ranks are outside protocol conformance, are absent from canonical replay, have permissioned visibility, and cannot change universal, public-relative, or tribe-relative rank state. There is no private universal-importance rank.

A product MAY let an owner share or publish a view of private rank state. Visibility alone does not turn that state into a canonical rank list, challenge, vote, or verdict. A future canonical adoption path, if defined, MUST be explicit and MUST NOT silently reinterpret owner-selected positions as collective judgments.

The shared grammar does not erase these distinctions. Universal and relative rankings have different axes and context keys; public and tribe contexts have different electorates; private individual maps use direct ordering rather than voting.


### 5.2.1 derived relative ranks and rank history [anchor: derived_relative_ranks_and_rank_history]

For any reference idea R and any lens L = (usage, axis, timeframe, scope), the set of neighbor ideas connected to R by elative_importance edges matching L and eligible under lifecycle rules MUST have a defined relative-rank ordering at each snapshot height where those edges exist. This ordering is derived solely from the canonical event log and active rulebooks.

When a matching elative_importance connection is first introduced, the connected idea MUST enter the relevant rank list at the deterministic baseline position (last by default) until importance-challenge outcomes re-order it. No mutable, authored rank value exists; rank order is fully defined by replay plus deterministic tie-break rules.

Relative-rank history is the sequence of these derived orderings across snapshot heights while the relevant edges exist and remain eligible. This history is immutable and reproducible because it is uniquely determined by deterministic replay.

### 5.3 tribes as public overlays over the shared idea graph [anchor: tribes_as_public_overlays_over_the_shared_idea_graph]

A tribe is an idea that serves as the nucleus of a group context - a scoped overlay in which identities who care about that nucleus idea can coordinate their perspective on the public graph. Tribes do not have private idea universes. They cannot create or maintain ideas that are visible only to tribe members, and they cannot shield ideas from public challenge. Every idea a tribe uses MUST already exist as a fully public, challengable idea in the global graph.

A user becomes a member of a tribe by forming a membership connection from their user identity idea to the tribe nucleus idea. Membership determines which identities may create and vote in tribe-scope importance challenges anchored at that nucleus. However, all of the content that tribes operate on - ideas, descriptions, connections, arguments, and internal rank lists - is built entirely from public ideas. Anyone MAY inspect which public ideas a tribe considers important, how those ideas are ranked relative to the tribe nucleus, and which arguments the tribe has attached. Only tribe members MAY vote in tribe-scope challenges or alter the tribe's internal rankings.

Tribe-relative importance uses the same relative comparison and challenge procedure as public-relative importance but changes the electorate. Tribe-relative rank lists represent how that tribe orders public ideas relative to its nucleus across the ten relative axes. These lists are always publicly visible. They DO NOT determine universal importance, DO NOT affect POD injection, and DO NOT directly alter the direction or magnitude of POD flow. The public `scope = universal` value identifies the globally eligible electorate; `rank_kind` and axis vocabulary separately distinguish 20-axis universal importance from 10-axis public-relative importance.

If a tribe wishes to coordinate around a new idea, it MUST first exist as a public idea in the global graph. Tribes cannot maintain hidden doctrines or private propositions that influence universal outcomes. If an idea is worth organizing around, it must be visible, challengable, and inspectable by the entire system. If a tribe attaches special structure or interpretation to a public idea using non-standard workflows, any user MUST be able to fork or duplicate that idea into a fully public, challengable idea that can participate in universal importance and POD flows. Only fully public, challengable ideas may enter the canonical civilizational record.

Legacy wording in historical documents that implies tribe-only canonical ideas or mirroring-as-copy into a separate public canonical object is DEPRECATED and MUST NOT be used to reinterpret the canonical model defined in this section and in Section 2.1A.

In this design, tribes become transparent interpretive communities rather than private power centers. They provide a way for people who share interests, values, or domains to maintain their own importance maps and interpretive overlays over the global graph, while leaving epistemic authority, universal importance, and POD generation anchored in the public arena. Tribal maps reveal how different communities understand and prioritize public ideas, but they cannot insulate those ideas from scrutiny or grant them special privileges in the canonical system.

### 5.4 relative-importance pathways and directed significance flow [anchor: relative_importance_pathways_and_directed_significance_flow]

Relative importance pathways define how significance is organized and interpreted within the idea graph. Every relative_importance connection expresses that one idea is important relative to another on a specific axis and timeframe. These connections, taken together, form the structural backbone through which challenges are organized, ranks change, ideas cluster into meaningful neighborhoods, and POD is later routed through the system.

A `relative_importance` connection does not create numeric scores or weights by itself. With `usage = general`, it places an idea into the ranking pool for the chosen reference idea, relative axis, timeframe, and canonical participation scope. When two candidates share that complete relative context, they may be brought into a pairwise importance challenge. A winning lower-ranked challenger moves immediately above the higher-ranked target. Public-relative and tribe-relative contexts use this canonical procedure with different electorates. Individual-private contexts use direct owner ordering outside canonical replay.

Interpretive significance pathways emerge from the global structure of relative-importance connections and the ordered rank lists they produce. Because relative importance edges express that one idea contributes to, derives meaning from, or matters for evaluating another, the graph acquires a readable structure in which more general or foundational ideas tend to anchor clusters of more specific, derivative, or action-oriented ideas. This interpretive structure is not encoded as a separate class of edges; it emerges from `relative_importance` connections and importance-challenge outcomes. Public-relative and tribe-relative rankings are publicly readable. Private individual rankings remain visible only according to their owner's product-layer permissions and are not canonical overlays.

Monotonic directionality is enforced **only for POD routing and derived POD-flow graphs**, not for the full set of `relative_importance` connections themselves. Universal importance provides a stable global gradient that constrains how POD may flow. Position `1` is the highest-importance ordinal position. A valid downhill route therefore moves from an idea with a smaller (more important) `overall_universal_rank` number toward an idea with a larger (less important) number. POD MUST NOT flow upward against that gradient.

Relative_importance connections used for interpretive ranking, argumentation, or evidence organization are not themselves required to be acyclic. Cycles, cross-links, or mutually referential structures MAY exist in the relative-importance graph as interpretive or deliberative artifacts. The acyclicity constraint applies only to the derived POD-routing structure constructed from universal-importance rankings. When an idea has multiple parents with higher universal importance, POD MAY flow through all such parents, subject to deterministic routing and weighting rules defined elsewhere.

The result is a layered system: relative importance establishes the comparative structure of meaning; importance challenges establish local ordering within that structure; universal importance establishes a global gradient; and POD routing follows that gradient to assign significance to human contributions. This separation allows rich, flexible semantic organization of ideas while preserving a strict, deterministic rule for reward flow. It ensures that the system remains expressive for reasoning and interpretation, while remaining safe, replayable, and resistant to reward manipulation.

Relative-importance pathways are subject to eligibility constraints derived from lifecycle state.

A relative-importance connection MAY exist canonically while being ineligible to participate in directed significance flow due to derived lifecycle_state (e.g., rotted or burned).

When a relative-importance connection is ineligible:
- it does not propagate importance,
- it does not contribute to ranking computations,
- and it does not participate in downstream POD/POINT routing.

Eligibility is a derived property and MUST be computed deterministically. Ineligibility affects propagation and participation only and MUST NOT remove, alter, or obscure the historical existence of the connection.



### 5.5 importance challenges [anchor: importance_challenges]

Importance challenges are the only way canonical universal-axis, public-relative, and tribe-relative rank lists change. Individual-private lists are not canonical importance-challenge contexts.

A universal importance challenge specifies `(rank_kind = universal, universal_orientation, timeframe, scope = universal)` plus two candidate ideas. A relative importance challenge specifies `(rank_kind = relative, reference_idea_id, usage = general, axis, timeframe, scope)` plus two candidate ideas. For `scope = tribe`, the reference and scope anchor MUST identify the tribe anchor and juror eligibility is member-restricted. For `scope = universal` with `rank_kind = relative`, the electorate is the eligible public.

The two candidates are the challenger and target. The reference or universal pole defines the question and is not a contestant. The target MUST be ranked above the challenger in the complete context when the challenge is created. Implementations MAY impose a configurable limit on how far up the list a challenger may reach to encourage local, incremental refinement.

Any eligible identity MAY create an importance challenge in a canonical context where they are allowed to participate. Creating a challenge consumes the challenger's rate-limited action budget. The challenger and other eligible participants MAY attach new or existing ordinary ideas as importance arguments for either candidate. Those ideas connect to the candidates through `relative_importance` edges with `usage = importance_argument`; a connection with `context_challenge_id` is challenge-specific, while an absent context identifies a reusable general argument. Importance-argument edges explain a judgment but do not place the argument idea into the contestants' `usage = general` rank list and do not independently move either candidate.

After a defined challenge period, the challenge moves to a voting phase. For universal-axis and public-relative contexts, the eligible public forms the potential electorate. For tribe-relative contexts, only eligible tribe members form the potential electorate. Jurors are selected through the deterministic vote-session procedure in Section 6.5; "public voting" or "tribe voting" identifies the eligible pool, not an open popularity poll or a requirement that every eligible person cast a ballot. Voters receive the two candidates, descriptions, complete rank context, and attached arguments and MUST decide which candidate should rank higher.

When a verdict is reached, the protocol applies a simple, deterministic update. If the challenger wins and is currently ranked below the target, the challenger moves to the position immediately above the target; intervening ideas retain their relative order. If the challenger loses, or if the challenger is no longer below the target when the verdict applies, the list DOES NOT change under the base rule. No numerical score, token balance, popularity measure, model confidence, or weighted vote determines placement inside an axis list. The ordering is determined by baseline insertion, successful challenge verdicts, and the bubble-up rule. The separately defined universal position sum/mean is derived only after all 20 universal axis lists are replayed.

Importance challenges are therefore local, discrete, and path-dependent. They do not attempt to optimize the entire rank list at once; they refine it through many small comparisons over time. Limits on how far up the list a challenger may reach, combined with rate limits on challenge creation and voting, ensure that attention is focused on plausible local improvements rather than constant attempts to overturn the very top of the list. Over long periods, this process allows the importance structure to self-organize and converge without requiring a central scoring function or global optimization algorithm.

### 5.6 pod flow, rate limits, and event-level assignment [anchor: pod_flow_rate_limits_and_event_level_assignment]

POD represents each living verified human identity's current replay-derived share of universal importance routed through eligible **human-authored canonical contributions**. POD does not attach to static ideas or to unpublished activity; instead, it is attributed exclusively through **canonical events** - the atomic, rate-limited actions taken by verified human identities that modify the canonical universe. Events that occur only in local drafts, private workspaces, or sandbox environments are out of protocol conformance scope and are never eligible for POD.

Canonical events that may receive POD fall into two categories: **epistemic events** and **external completion events**. POD routed through the universal-importance hierarchy flows into both categories, but only after the conditions specific to each class are satisfied and only through deterministic replay.

Epistemic events include idea creation, description proposals, argument and evidence submission, challenge openings, and selected-voter decisions. These events modify the epistemic state of the canonical graph directly and constitute the core of human participation in deliberation. As such, epistemic events are **direct POD sinks**. At each qualifying cycle boundary, POD routed through the current universal-importance structure and eligible relative-importance pathways is deterministically allocated to the eligible epistemic events that have modified an idea or materially contributed to its evaluated significance. Given the same event history, cycle boundary, lifecycle and eligibility state, universal ranks, and active rulebooks, all conformant nodes MUST reproduce the same current POD assignments.

External completion events arise only from actionable ideas. POD routed into an actionable idea is not immediately attributed to an identity. Instead, it remains unassigned until a human identity completes the corresponding real-world action and submits a canonical completion truth claim. Only after the completion claim survives the full truth-challenge process may that eligible completion event contribute to the executing identity's current POD. External actions therefore function as **indirect POD sinks**, with assignment deferred until verification.

Rate limits apply to all canonical event types that may receive POD. Every human identity has a bounded capacity to create POD-eligible events within a given period. This constraint ensures that POD reflects deliberate, meaningful contribution rather than volume-based interaction or automated activity. AI identities MAY assist with drafting content in non-canonical sandboxes or local tools but MAY NOT author canonical events and MAY NOT receive POD. Only human-authored, rate-limited canonical events are POD-eligible.

POD routing MUST respect the universal-importance gradient. No POD may flow upward. Only events associated with ideas of strictly lower universal importance than the upstream source idea may receive POD. This monotonic routing rule applies only to POD assignment and derived POD-flow graphs; it does not constrain interpretive importance structures. The rule ensures that significance flows from broad, high-level ideas into the concrete epistemic and action-level contributions that develop, support, or enact those ideas.

Certain categories of events or actions MAY be declared POD-ineligible by safety, governance, or token rulebooks, even if the associated ideas are deemed important. For example, actions that are fraudulent, coercive, violent, or otherwise disallowed by active rulebooks may be fully represented in the canonical universe for purposes of truth and importance evaluation while being explicitly excluded from POD receipt. Such exclusions MUST be deterministic, rulebook-defined, and fully auditable.

The event-level attribution model ensures that POD reflects the lived, recorded history of human deliberation and action, rather than attaching value to static ideas or private activity. Current POD is therefore a function of **what living humans canonically did**, under rate limits, as evaluated through current universal importance, lifecycle and eligibility state, and active token rules. It is not the sum of all POD previously derived.

Only ideas and eligible connections in the **active** lifecycle_state participate in POD/POINT routing and any supply-affecting computations.

Ideas or eligible connections that are rotted or burned:
- do not receive incoming POD flow,
- do not propagate POD flow further,
- and do not contribute to aggregate supply calculations.

This exclusion is mandatory to prevent accumulation of incentive weight from inactive or abandoned graph mass. Historical POD/POINT attribution remains inspectable, but inactive objects are excluded from ongoing routing.


---

### 5.7 actionable ideas, declarations, completion claims, and verification [anchor: actionable_ideas_declarations_completion_claims_and_verification]

Actionable ideas represent publicly deliberated plans that specify an intended real-world intervention. They serve as POD-routing intermediaries between universal-importance judgments and external human action. An actionable idea does not assert that an action has occurred; it asserts a plan that, if executed, would advance or instantiate important concepts in the canonical graph.

Execution of an action proceeds through a three-stage canonical process, expressed entirely through explicit events. First, a human identity MAY issue an **action declaration event**, stating the intent to perform the actionable idea. The declaration establishes an intention and an observation window but does not assert that the action has occurred. Second, after performing the real-world activity, the same human identity MUST submit a **completion truth claim** asserting that the action was performed. Third, the completion truth claim becomes subject to the full truth-verification process, including arguments, evidence, counterevidence, and action-verification challenges, exactly like any other truth claim in the system.

Action declarations and completion claims are canonical events and MUST be publicly visible and challengeable once published. Declarations MAY be omitted if rulebooks permit direct submission of a completion claim, but in all cases POD eligibility requires a successful, challenge-resilient completion truth claim. Rulebooks MAY impose additional requirements on declarations or verification for specific classes of actionable ideas, but they MUST do so deterministically and without introducing non-replayable semantics.

Only after a completion truth claim withstands challenge may its completion event receive routed POD attributable to the human identity that declared and completed the action. Until that point, POD routed into the actionable idea remains unassigned. If a completion claim is rejected, fails verification, or is superseded by a successful counter-challenge, the POD routed into that actionable idea for the relevant cycle MUST dissipate or remain unassigned according to the active token rulebooks. No earlier completed-cycle derivation is retroactively changed for failed or false completion claims.

Retroactive action creation is prohibited. A human identity MAY NOT declare or complete an action that occurred prior to the creation of the corresponding actionable idea. Past real-world events MAY only be introduced as truth claims describing historical facts. Such truth claims MAY influence importance judgments and future deliberation but SHALL NOT receive POD. This prohibition preserves replay determinism, prevents retrospective POD extraction, and ensures that POD always corresponds to forward-directed action arising from collective deliberation.

Only real human identities may declare or complete actions. AI identities, tribes, or any non-human identity types SHALL NOT perform external-world actions and SHALL NOT receive POD for action completion. Multiple human identities MAY independently declare the same actionable idea. Each declaration MUST result in a distinct completion truth claim, and each completion claim is evaluated independently through the truth-challenge process. Successful completion by one identity does not preclude others from also executing the same actionable idea, unless restricted by explicit rulebook constraints.

**All action execution is voluntary.** No rulebook, governance verdict, challenge outcome, or protocol mechanism MAY compel a human identity to declare or complete an action. Endorsement of an actionable idea through importance or action challenges specifies what *should* be done according to deliberation, but never who *must* do it. If no human identity voluntarily issues a declaration and completes the action, the actionable idea remains endorsed-but-unimplemented.

Actionable ideas therefore function as the protocol’s bridge between internal reasoning and external implementation. They allow significance determined through universal and relative importance to manifest in real-world activity while preserving public accountability, verifiability, voluntariness, safety constraints, and deterministic replay.

---

### 5.8 pod recomputation, rebalancing, and long-term alignment [anchor: pod_decay_influence_rebalancing_and_long_term_alignment]

Current POD does not persist as an earned balance. At each qualifying cycle boundary, conformant nodes MUST recompute POD routing, event attribution, and each living identity's share from the canonical replay prefix and the then-current universal-importance, lifecycle, eligibility, and rulebook state. The historical derivation for an earlier completed cycle remains replayable, but it is not carried forward as a positive quantity that must be retained.

POD use is strictly limited to mechanisms authorized by the token specification, including explainable attribution and the derivation of POINT minting and redistribution. **POD SHALL NOT weight governance, voting power, eligibility, truth, importance ranking, safety, moderation, authority, or special visibility** in any challenge, rulebook decision, discovery system, or protocol-level process. Governance remains human-equal and scope-gated as defined elsewhere.

Universal-importance shifts alter the POD-routing landscape at the next qualifying cycle boundary. When the importance of ideas changes through canonical importance challenges, subsequent POD derivations follow the new pathways. Events associated with ideas whose universal importance has diminished may receive less or no current POD, while eligible events connected to newly important ideas may receive more. An identity's total current POD therefore may rise, fall, or become zero.

POD routed into actionable ideas or epistemic events dissipates in a controlled manner when it is not claimed. POD routed into actionable ideas that are never completed MUST dissipate or remain unassigned at cycle completion, according to the active token rulebooks. If an epistemic event later loses routing eligibility or importance, its reduced or absent contribution changes current and future POD derivations without altering the derivation recorded for any earlier completed cycle.

This change is not a penalty, confiscation, transfer, or retroactive reassignment. No authoritative permanent POD balance exists to preserve or confiscate. Historical events, authorship, routing explanations, and completed-cycle results remain immutable, while the current POD share changes with the living graph. This prevents early contributors from accumulating permanent reward or recognition dominance while preserving durable acknowledgment through the historical record.

These recomputation and rebalancing mechanisms maintain a stable, adaptive equilibrium. They ensure that POD reflects present universal importance routed through surviving eligible contribution history without introducing central authority, retroactive penalties, or governance capture. As universal importance evolves, current POD adapts accordingly, allowing the system's incentive and recognition layer to remain aligned with the evolving judgments of a global, deliberative community.


### 5.9 grounded actions, truth claims, and physical evidence [anchor: grounded_actions_truth_claims_and_physical_evidence]

Actionable ideas express deliberation about what should be done. They may concern external changes in the world, such as building infrastructure or publishing documents, or internal decisions about how ideas, rules, or configurations within the canonical universe should be treated. In all cases, an actionable idea is a proposal, not a fact. It becomes effective only when one or more corresponding actions are carried out by identities and recorded as events in the canonical universe.

An action is a recorded outcome of deliberation: a statement that some identity did something in response to one or more actionable ideas. Actions MAY represent external consequences, such as publishing a new version of a document, deploying software, signing a contract, or performing a physical task. Actions MAY also represent internal decisions, such as agreeing that a particular rule set is now in force, marking an idea as deprecated for future use, or recording that a merge between two ideas is now accepted. In both cases, actions link deliberation to concrete behavior; they are the points at which "we should do X" becomes "we did X".

**Each canonical action MUST be something that a single human identity could, in principle, perform voluntarily.** Multi-person undertakings SHALL be represented as separate action events, one per identity, each with its own declaration and completion truth claim. No identity SHALL be assigned, obligated, or compelled to perform an action; procedural endorsement never implies mandatory execution.

Every important action SHOULD be described by one or more truth claims. A truth claim that describes an action asserts that a particular event occurred, such as "identity A cast a yes' vote on challenge C at time T", "identity B created idea I with description D", or "as of event E, rule set R is now applied for domain D". These descriptive truth claims are themselves subject to truth challenges; if a description is inaccurate, misleading, or incomplete, participants MAY open a truth challenge and propose corrected descriptions. The truth of a descriptive claim is grounded not in its wording alone, but in the underlying action events and any associated evidence.

Actions MAY be supported by explicit evidence that ties them to physical reality. Evidence can include cryptographic signatures, commit hashes, log files, video or audio recordings, notarized documents, or attestations from multiple identities. Such artifacts are represented as content attached to the relevant action or truth-claim ideas and linked via relative_importance connections whose usage is evidence_for or evidence_against, and their strength can be evaluated along the same spectrum-of-evidence framework that applies to truth claims more generally. Governance-related actionable ideas MAY require stronger evidence for their associated actions, such as multiple independent attestations or richer audit trails, while low-stakes actions MAY rely on simpler records such as a single authenticated click.

This structure allows governance decisions and configuration changes to be treated using the same primitives as any other decision in the system. An actionable idea that proposes a change to rules or configurations - such as adopting a new rule set, adjusting juror counts, or changing classification thresholds - is deliberated through action challenges. If accepted, identities carry out the corresponding actions by recording that the decision has been made and, where appropriate, by creating or updating ideas that represent the new rule set or configuration. Truth claims then describe these decision actions, and evidence links them to physical acts such as votes, signatures, or code changes. Nodes reconstruct the history of decisions by replaying actions, evaluating the associated truth claims and evidence, and applying the same challenge and importance mechanisms that govern all other ideas in the canonical universe.

### 5.10 pod routing to infrastructure actions (normative clarification) [anchor: pod_routing_to_infrastructure_actions_normative_clarification]

POD routing SHALL treat protocol-infrastructure contributions the same way it treats all other contributions: POD flows through importance structure and attaches to eligible canonical events created by verified humans.

Accordingly:

- If an identity performs chain-maintenance work (e.g., producing a valid snapshot, serving archival packs, publishing commitments, or performing verification actions), the associated eligible canonical actions and truth claims MAY contribute to the identity's current POD only while they remain eligible and receive universal-importance routing.

- If a maintenance claim is later shown false or misleading through truth challenges, its downstream importance and future POD impact MUST be reduced under the same epistemic correction and fraud mechanisms used elsewhere in the protocol.

This section does not add new reward rules; it makes explicit that infrastructure actions are first-class citizens of the same importance >POD pipeline.

### 5.11 Living-map eligibility (normative) [anchor: living_map_eligibility_normative]

The **living map** is the canonical, derived subset of the full idea graph that participates in default importance computation, propagation, and incentive routing.

An idea or eligible connection participates in the living map if and only if:
- it exists in the canonical event log, and
- its derived lifecycle_state is **active**.

Living-map eligibility is:
- deterministic,
- recomputed at cycle boundaries,
- derived solely from the canonical event log and rulebooks,
- recorded in snapshots as derived state.

Objects excluded from the living map:
- remain permanently recorded and addressable,
- remain challengeable and referenceable,
- remain visible in raw structural views,
- but are excluded from default ranking surfaces and propagation logic.

Living-map eligibility MUST NOT be used as moderation, censorship, or punishment. Its sole purpose is to maintain a tractable, meaningful representation of what the system is actively deliberating.


### 5.12 Relative_importance edge rot/burn (normative) [anchor: relative_importance_edge_rot_burn_normative]

Relative-importance connections are subject to lifecycle derivation independently of the ideas they connect.

A relative-importance connection MAY rot or burn even if both endpoint ideas remain active.

An idea's resurrection or activation MUST NOT implicitly resurrect any `relative_importance` connections incident to that idea. Structural and provenance connections become active automatically with the idea, but `relative_importance` connections remain independently lifecycle-scoped and must be maintained or resurrected on their own terms.

For avoidance of doubt: resurrection or activation of an idea does not restore any `relative_importance` connections.


Relative_importance connections are part of the living map and are subject to rot and burn to prevent unmaintained bloat.

A relative_importance edge (or its living-map visibility) MAY be burned when it falls below governance-defined maintenance thresholds, such as:
- sustained low relative importance within its scope/axis/timeframe,
- lack of interaction or challenge activity over a governance-defined number of cycles,
- or other deterministic criteria defined by rulebooks.

Rot/burn evaluation for relative_importance edges MUST occur at cycle boundaries as part of boundary derivations. Burn MUST be implemented as canonical state transformation (never deletion): burned edges remain in history but are removed from the living graph view and from living-map computations unless revived by subsequent canonical events.

Rot/burn MUST NOT contribute to cycle sealing. Survivals/attrition signals MAY be used as feedback for parameter tuning (burn aggressiveness, target clamps), but MUST NOT cause, prevent, or delay a cycle seal (§5).


#### 5.12.1 Derivation criteria [anchor: derivation_criteria]

The `lifecycle_state` of a relative-importance connection is derived from:
- its relative importance rank or weight within the relevant lens, axis, timeframe, and scope,
- the number of cycles since the connection was created,
- the number of cycles since the connection last received qualifying maintenance activity.

All thresholds, inactivity windows, scaling behavior, and hysteresis parameters are defined by governance rulebooks and MUST be deterministic.

#### 5.12.2 Maintenance activity [anchor: maintenance_activity]

Maintenance activity for a relative-importance connection includes canonical actions that explicitly engage with the relationship itself, including:
- participation in a challenge whose subject is the relative-importance relationship,
- votes cast in such challenges,
- arguments or evidence submitted that reference and substantively engage with the relationship,
- canonical transformations that reaffirm, modify, or replace the relative relationship.

Passive existence, indirect association, or engagement with only one endpoint idea does not constitute maintenance activity for the connection.

#### 5.12.3 Lifecycle transitions [anchor: lifecycle_transitions]

- **Active  -> rotted**  
  Occurs when a relative-importance connection falls below defined relative-importance thresholds and remains without qualifying maintenance activity for the configured number of cycles.

- **Rotted  -> burned**  
  Occurs when inactivity persists beyond an extended cycle window as defined by governance rulebooks.

- **Rotted  -> active**  
  Occurs automatically upon qualifying maintenance activity or restoration of sufficient relative importance before the burn threshold is reached.

- **Burned  -> active**  
  Requires an explicit **resurrection action** targeting the burned relative-importance connection. Resurrection restores the connection to active participation in the living map; the connection thereafter remains independently lifecycle-scoped.

#### 5.12.4 Effects [anchor: effects]

Rotted or burned relative-importance connections:
- do not propagate importance,
- do not participate in POD/POINT routing,
- do not contribute to default rankings or living-map computations.

All relative-importance connections, regardless of lifecycle_state, remain permanently recorded in the canonical event log and remain inspectable, referenceable, and challengeable.




## 6. challenges, disputes, and transformations [anchor: 6_challenges_disputes_and_transformations]

Challenges are the protocol's unified mechanism for resolving disputes about what is true, what is important, what should be done, and how ideas should be identified and represented. Every challenge follows the same procedural lifecycle but applies domain-specific reasoning, connection types, and deterministic state transformations. Challenges operate only on the present canonical universe; they do not invalidate or rewrite historical events. Instead, a challenge appends new events that update certainty, ranking, action selection, or representation. Challenges ensure that all changes in the system - epistemic, evaluative, operational, and structural - occur through transparent, adversarial deliberation.

### 6.1 challenge domains and targets [anchor: challenge_domains_and_targets]

The protocol defines a single challenge primitive with four domains distinguished by the type of proposition they target and the state transformation they enact: truth challenges, importance challenges, action challenges, and representation challenges. These domains are applications of the same challenge framework but use distinct usages of the core connection types and domain-specific voter instructions, rather than introducing separate connection families for each domain.

Truth challenges evaluate the accuracy, certainty, or classification of a truth claim. They use relative_importance connections whose usage is evidence_for or evidence_against between candidate evidence ideas and the challenged claim, together with the evidence rails defined in Section 5. Prediction resolution, action completion verification, test-result confirmation, and governance-rule evaluation are all subcases of truth challenges. Truth challenges may modify certainty bands, update claim status, activate or reject governance rules, or - when confirming a completion truth claim - make its completion event eligible for current POD attribution. The core target is always a truth claim; all submodes inherit the same fundamental deliberation process.

External sources do not become canonical evidence by being linked. A paper, article, book, video, dataset, website, instrument output, or external record becomes relevant only when an identity authors ideas asserting what that source says, contains, measured, or supports, with provenance such as URLs, hashes, sections, timestamps, archived copies, or payload references. Important sources SHOULD be represented by source-document, source-section, and source-chunk ideas where the existing base idea types can express them. Claims about those sources remain challengeable, and certainty changes only through explicit connections and challenge outcomes.

Importance challenges evaluate two candidate ideas within one complete rank context. A universal challenge declares `(rank_kind = universal, universal_orientation, timeframe, scope = universal)`. A relative challenge declares `(rank_kind = relative, reference_idea_id, axis, timeframe, scope)`; the reference defines the question and is not a contestant. Only upward challenges are permitted. Participants attach ordinary idea-based arguments to either candidate with `usage = importance_argument`. A winning lower-ranked challenger moves immediately above the higher-ranked target; otherwise the order remains unchanged. These challenges are the exclusive mutation path for canonical universal-axis, public-relative, and tribe-relative lists. Individual-private lists use direct owner ordering outside canonical replay.

Action challenges determine which actionable ideas - publicly proposed plans - should be endorsed, prioritized, postponed, coordinated, or rejected. They use relative_importance connections between actionable ideas, related truth and conceptual ideas, candidate executors, and potential or actual actions; rulebooks MAY further specialize some of these edges with dedicated usage values (for example, reusing importance_argument for reasons to endorse a plan, or introducing a future action_outcome usage for outcomes connected back to plans). **Action challenges operate on the spectrum of potential actions and the spectrum of proposed actions (§7.4), determining which proposals best instantiate the intended intervention. Action challenges MAY endorse, de-endorse, reprioritize, or supersede proposed actions, but they SHALL NOT assign execution to any identity, compel execution, or imply obligation. Verdicts specify only what the system judges should be done, not who must do it.** After an action challenge endorses an actionable idea, execution proceeds through the action-declaration and completion-claim pipeline; verification of completion is handled by truth challenges.

Representation challenges determine how ideas are identified, distinguished, or merged, which title becomes canonical for the title slot, and which description becomes canonical in each length-complexity cell. They include same_as challenges, which propose equivalence between ideas or between specific title or description representations, and representation-selection challenges, which choose the canonical title or description for an idea or ordering. Representation challenges modify identity mappings, canonical representation pointers, and same_as metadata without altering the truth or importance status of any idea.

Each challenge domain relies on the same underlying challenge mechanism and lifecycle but emphasizes different uses of the core connection types. Truth challenges primarily read relative_importance connections with usage = evidence_for or evidence_against along the evidence rails. Importance and action challenges primarily manipulate and inspect relative_importance connections with usage = general and usage = importance_argument (and any future action-related usages defined by rulebooks). Representation challenges operate chiefly on same_as connections and title or description pointers. All four domains share the same event structure - challenge creation, argument attachment, voting, verdict, and state update - while differing only in what they consider valid arguments, how they interpret connections, and what state they update when a verdict is reached.


### 6.2 challenge lifecycle [anchor: challenge_lifecycle]

All challenges follow a unified, deterministic lifecycle consisting of six stages. This lifecycle is identical across truth, importance, action, and representation challenges. A conformant node MUST process these stages in the same order and with the same deterministic rules to ensure perfect replayability of the canonical universe.

**(1) challenge creation.**  
A human identity initiates a challenge by creating a challenge event specifying the target proposition and the challenge domain. The challenge becomes immediately open for argument and connection submission. No challenge may remain in an uninitialized or pending state; creation begins the process.

**(2) argument and connection phase.**  
During this period, any user MAY submit arguments supporting or opposing the challenge and MAY attach domain-appropriate connections: evidence for truth challenges, importance arguments for importance challenges, action arguments for action challenges, and same_as or description-tier submissions for representation challenges. All such contributions become part of the challenge packet. No separate counterargument phase exists; all arguments, counterarguments, and connections are submitted concurrently in this single open phase.

This phase continues until one of the following conditions is met:  
(a) a deterministic time limit elapses, or  
(b) the system determines (per governance-set rules) that the challenge has accumulated sufficient content to proceed.  
Once the phase ends, no additional arguments or connections may be added.

**(3) voter selection.**  
The system selects a set of voters using the deterministic mechanism defined in the voting specification. The selection must be fully replayable. Selected voters receive the complete challenge packet. They do not deliberate publicly; each voter evaluates independently.

**(4) voting window.**  
Each selected voter has a fixed, deterministic amount of time to cast a vote, measured from the moment they accept their voting assignment. Failure to vote within this window results in forfeiture and replacement according to the voting-spec rules. Votes are recorded as canonical events and include the voter's judgment according to the domain-specific instructions: truth evaluation, importance comparison, action selection, or representational judgment.

**(5) verdict aggregation.**  
Once all required votes are received (or appropriately replaced after timeouts), votes are aggregated deterministically. The aggregated verdict is encoded as a canonical verdict event. Possible verdict forms depend on the challenge domain: confirm or revise a truth claim; reorder ideas in an importance ranking; endorse, defer, or reject actionable ideas; or merge ideas or update canonical descriptions.

**(6) state transformation.**  
Upon verdict, the protocol applies the domain-specific state update. This transformation modifies only the present canonical universe and SHALL NOT retroactively alter past ledger events. Truth-challenge verdicts update certainty or claim status and may change prospective POD eligibility. Importance-challenge verdicts update ranking order. Action-challenge verdicts update which actionable ideas are endorsed or coordinated. Representation-challenge verdicts update identity relations or canonical description pointers. A governance verdict establishes eligibility for a proposed rulebook change; it does not by itself prove implementation or activate the rule. The transformation completes the challenge lifecycle.

This streamlined lifecycle ensures adversarial reasoning, clear phase boundaries, rapid voter engagement, and deterministic replay. All epistemic, evaluative, operational, and representational disputes pass through this single unified mechanism.

### 6.3 no retroactive mutation [anchor: no_retroactive_mutation]

Challenges operate within an append-only event model in which all epistemic activity is recorded as a strictly ordered sequence of events. This section defines the immutability rules governing challenge outcomes and their effects on idea state. These rules ensure that nodes can deterministically replay the canonical universe from genesis without ambiguity, nondeterminism, or state rewriting. No challenge, verdict, or subsequent corrective action may modify any previously recorded event or its interpretation. Instead, the protocol uses explicit, forward-visible corrective events to express disagreement, reversals, and updated interpretations.

All events in the canonical universe are final. Once an event is accepted into the canonical log - whether it represents an idea creation, connection creation, challenge creation, argument submission, voter selection, vote, or state transformation - it SHALL NOT be removed, rewritten, or replaced by any later event. A challenge verdict MAY introduce a new state transformation, but this transformation MUST be represented as a new event appended after the verdict. Historical ideas, connections, scores, importance placements, evidence placements, or canonical descriptions MUST remain visible exactly as they existed at their respective event indices. A later state transformation MAY alter the **current** derived state during replay, but MUST NOT mutate or conceal the historical event that originally contributed to earlier derived states.

Nodes reconstruct state by replaying all events in canonical order. When a challenge verdict supersedes an earlier assertion - such as an evidence placement, importance argument, or representation mapping - the earlier assertion remains part of history but becomes epistemically inactive once the later corrective transformation is applied. This forward-shadowing rule ensures that state transitions are always the product of accumulating events, never deletion or mutation. Nodes MUST NOT reinterpret earlier events based on knowledge from later events; instead, they SHALL incorporate later events as state transitions layered atop the earlier ones.

Corrective challenges must express disagreement or correction through new, explicit events. For example, if an earlier truth claim was placed at a certain certainty level and a new challenge later establishes a more justified placement, the updated placement is recorded as a new truth-placement transformation event. The earlier placement remains part of the chain's history, and replaying the full sequence will deterministically yield the later, corrected positioning as the current state. Similarly, if an idea title, sentence description, or canonical description is replaced following a representation challenge, the replacement is appended as a new representation event; the original description remains historically accessible.

Invalid or malicious submissions are also append-only. If a node deems an event invalid according to protocol or safety rules, it MUST produce a corresponding **blocked_submission** event rather than excising the invalid event. The invalid submission itself remains in the historical log, but it produces no semantic effects during replay beyond being marked as blocked. The presence of these events enables deterministic global auditability, forensic inspection, and unambiguous proofs of safety enforcement.

Under no circumstances MAY a challenge verdict reorder past events, reinterpret past ordering, or alter the epoch boundaries that determine when events enter the canonical universe. Nodes MUST enforce that all corrections and disputes proceed exclusively through forward-appended events that maintain the causal chain of epistemic reasoning. Together, these rules ensure integrity, auditability, and deterministic replay across all conforming nodes, regardless of implementation details or execution environment.

### 6.4 spectrum of potential actions and spectrum of proposed actions [anchor: spectrum_of_potential_actions_and_spectrum_of_proposed_actions]

Actionable ideas use a two-ordering structure that mirrors the evidence rails defined for truth claims. Instead of a spectrum of potential evidence and a spectrum of actual evidence, each actionable idea maintains a spectrum of potential actions and a spectrum of proposed actions. These orderings provide a shared, deterministic frame for reasoning about how extreme, reversible, risky, or costly different courses of action are, and for comparing concrete proposals against the best-available alternatives that could, in principle, be taken.

The spectrum of potential actions represents what could be done in principle about the situation the actionable idea addresses. It consists of hypothetical action templates and archetypes expressed as ideas in the graph, connected to the actionable idea using relative_importance connections whose usage is reserved (by rulebooks) for potential-action relationships (for example, potential_action_template). Together, these potential actions define a structured range from minimal, low-impact interventions through moderate and high-impact interventions up to extreme, hard-to-reverse interventions. They are not commitments to act and do not themselves trigger execution; they exist to make explicit the shape of the action space so that concrete proposals can be positioned relative to it.

Running in parallel is the spectrum of proposed actions, which consists of concrete action proposals attached to the actionable idea via relative_importance connections whose usage is reserved for proposal relationships (for example, proposed_action). Each proposed action is a specific plan, advanced by one or more identities, that could be executed and later verified as an action idea. When a proposed action is introduced, the proposer MUST specify an initial placement along the actionable idea's potential-action spectrum: effectively a claim about which region of the potential-action rail this proposal occupies and how its impact, reversibility, risk, and resource use compare to the hypothetical options already defined.

Disputes about how a proposal is positioned on this spectrum are resolved through action challenges. An action challenge MAY be opened to argue that a proposed action has been mispositioned relative to the spectrum of potential actions, or that the potential-action rail itself is incomplete or distorted (for example, by omitting obvious minimal interventions or clustering only around extremes). In the argument phase of such a challenge, participants reference the existing potential actions, introduce additional potential actions if needed, and compare the concrete proposal's characteristics to those reference points. The challenge's verdict yields a state transformation event that updates the canonical placement of the proposed action and, where justified, the ordering or composition of the potential-action spectrum for that actionable idea. All of these changes are recorded as forward-only events; earlier placements and ordering configurations remain in history but are superseded for current-state derivation.

The relationship between the spectrum of potential actions and the spectrum of proposed actions is used downstream by other action challenges that decide whether and when to endorse or execute proposals. Governance and safety rulebooks MAY impose additional ceremony, quorum, or evidence requirements for endorsing proposals that occupy more extreme, irreversible, or high-risk regions of the action spectrum, but such policies are defined elsewhere in this specification. The core protocol only requires that every actionable idea can host a spectrum of potential actions, that concrete proposals specify and maintain a placement on a spectrum of proposed actions tied to that potential spectrum, and that all disputes about these orderings and placements are conducted through action challenges that are fully recorded and deterministically replayable across conformant nodes.

### 6.5 voter eligibility, juror selection, and rate-limited vote sessions [anchor: voter_eligibility_juror_selection_and_rate_limited_vote_sessions]

Voting on challenges is performed by small juror panels rather than open polls. This section defines how eligible voters are determined, how jurors are selected, and how rate-limited vote sessions work. The goal is to provide a simple, deterministic mechanism in which each challenge receives a fixed number of juror votes, and each identity participates through short, constrained voting sessions that prevent cherry-picking only high-importance challenges.

For a given challenge, the protocol defines an **eligible voter set** as the set of identities that could serve as jurors on that challenge at the challenge's deterministic eligibility freeze boundary. An identity is eligible if and only if it can see the challenge and its anchor idea under current visibility and scope rules; it is not the creator of the challenge and does not violate any rulebook-defined conflict-of-interest constraints; and it has not already cast a vote or been permanently disqualified from voting on that challenge. Eligibility pool membership is independent of current voting mana/session capacity.

For non-governance challenges, the deterministic eligibility freeze boundary is the voting-open boundary (the cycle boundary at which `challenge_open_voting` becomes effective). For governance challenges, eligibility freeze remains challenge-open scoped.
Each challenge has a **target juror count**, which is the number of completed votes required for that challenge to proceed to verdict aggregation. In the base protocol configuration, the target juror count MUST be three for all challenge types. Governance rulebooks MAY later extend this to allow higher target juror counts for particular challenge types or for challenges attached to ideas above specified universal-importance thresholds, but all such policies MUST be derivable deterministically from the event log and rulebook state. The semantics of an individual vote are independent of the target juror count; the target controls how much confirmation is required before applying a state transformation.

Juror participation is organized into **vote sessions**. A vote session is initiated when an identity explicitly chooses to vote. When a client requests a new vote session on behalf of an identity, the node MUST first verify that the identity has sufficient deterministic voting capacity (mana/rate-limit budget) for the current window and, if so, decrement or reserve one unit of that capacity. If capacity is insufficient, the session request MUST be rejected without altering eligibility pool membership. The node then computes the set of open challenges for which the identity is currently eligible and that still require additional jurors to reach their target juror count. From this set, the node deterministically selects a small candidate subset, of size three, using a pseudorandom function of the identity identifier, a shared randomness seed, and a monotonically increasing session index for that identity. The three selected challenges (or, in implementations that group by anchor idea, up to three anchor ideas each with at least one open eligible challenge) constitute the candidate set for that vote session.

Within a vote session, the identity MUST select exactly one candidate challenge to vote on, or explicitly decline to vote on any of them, within a bounded decision window. A conformant implementation MAY surface these candidates as a list, as locations in a graphical interface, or in any other UI, but it MUST NOT allow the identity to request additional candidates for the same session without cost. If the identity selects a candidate challenge and casts a valid vote within the session's time window, the node records a vote event linking the identity, the challenge, and the selected option. If the identity fails to cast any vote before the decision window expires, the session is marked as expired and the reserved voting capacity remains consumed unless governance rules explicitly allow a limited number of no-cost expirations. In either case, the vote session is closed, and the identity MUST initiate a new vote session (subject to rate limits) to participate in further voting.

For each challenge, juror selection over time is the accumulation of vote sessions in which that challenge appears as a candidate and is chosen by jurors. A challenge proceeds to verdict aggregation once the number of completed votes recorded for it reaches its target juror count and any timing or quorum constraints defined in Section 7.2 and the applicable rulebooks are satisfied. All vote sessions, candidate selections, votes, and any explicit declines MUST be represented as events in the canonical universe so that all conformant nodes can reconstruct, under replay, which identities were offered which candidate sets, how often, and which challenges they elected to vote on. Nodes MUST NOT offer an identity more than three concurrent candidate challenges per vote session, MUST enforce the consumption of voting rate limits at the start of a session, and MUST ensure that identities cannot repeatedly skip or restart sessions to target only high-importance challenges without incurring the configured rate-limit cost. Together, these rules define a simple, replayable mechanism in which each challenge is judged by a small panel of jurors, and each identity participates through short, randomized vote sessions that balance fairness, resistance to targeted brigading, and a manageable cognitive load.

### 6.6 action endorsement, implementation state, and voluntariness [anchor: action_endorsement_implementation_state_and_voluntariness]

Action challenges determine whether an actionable idea or a proposed action SHOULD be endorsed, rejected, superseded, or prioritized within its deliberative context. To support deterministic replay and clear downstream semantics, each actionable idea SHALL maintain a canonical action-state that evolves through the following transitions:

* **inert**  -  the actionable idea exists but has not been endorsed.  
* **endorsed**  -  an action challenge verdict has endorsed the actionable idea or its selected proposal. Endorsement specifies that the system judges this action desirable but DOES NOT assign it to any identity.  
* **implemented**  -  one or more human identities have voluntarily issued declarations and have had completion truth claims successfully verified for the endorsed action.  
* **superseded**  -  a later action challenge endorses a different actionable idea or proposal that replaces or invalidates the previous one.

Action challenges MAY move an actionable idea from inert  -> endorsed or endorsed  -> superseded. They MAY NOT move endorsed  -> implemented; implementation occurs only through voluntary action declarations followed by completion truth claims as described in §6.7.

**No identity MAY be compelled to implement an endorsed action.** If no human identity voluntarily issues an action declaration, an endorsed action remains unimplemented. Nodes MUST represent this state fully and consistently; the system SHALL NOT infer or assign implementers.

Rulebooks MAY impose additional deliberation requirements before an action challenge MAY reach a verdict - such as requiring tradeoff analyses, multi-tiered arguments, or extended voting windows - but these requirements SHALL NOT override the foundational voluntary-execution invariant.

Multiple identities MAY independently declare and implement the same endorsed action. Each implementation produces a separate completion truth claim subject to independent verification. If an endorsed action has multiple verified implementations, nodes SHALL record all of them; governance or downstream rulebooks MAY treat multiple implementations as evidence of robustness or consensus.

This action-state model ensures that endorsement, voluntariness, and implementation remain strictly separated, preserving the protocol’s human-first commitments while enabling deterministic reasoning about what the system has agreed to do and what has actually been done.

### 6.7 Resurrection actions (normative) [anchor: resurrection_actions_normative]

A **resurrection action** is a canonical creation-like mechanism by which a burned idea or burned relative-importance connection may be restored to active participation in the living map.

#### 6.7.1 Scope and targets [anchor: scope_and_targets]

A resurrection action MAY target:
- a single burned idea, or
- a single burned relative-importance connection.

The target MUST currently be in the `burned` lifecycle_state. Rotted objects do not require resurrection.

#### 6.7.2 Effect and timing [anchor: effect_and_timing]

A resurrection action transitions the target's lifecycle_state from **burned** to **active** at the next applicable cycle boundary.

Resurrection does not require a challenge. Resurrection actions are fully canonical and remain challengeable post-hoc under existing challenge domains (e.g., importance or representation).

#### 6.7.3 Constraints [anchor: constraints]

Resurrection actions:
- MUST be explicitly labeled as resurrection actions,
- MUST reference the specific target object,
- MUST be rate-limited and gated under the same framework as creation actions,
- MAY be subject to governance-defined cooldowns or additional deterministic constraints.

Repeated resurrection attempts without sustained engagement MAY be subject to escalating constraints or cooldowns as defined by governance rulebooks.

#### 6.7.4 Determinism and auditability [anchor: determinism_and_auditability]

All resurrection actions MUST be:
- deterministic,
- auditable,
- replayable from the canonical event log.

No discretionary or administrative resurrection is permitted.

#### 6.7.5 Resurrection cost parity (normative) [anchor: resurrection_cost_parity_normative]

Resurrection consumes the same scarce capacity as creating new canonical objects.

- Restoring a burned idea to active participation MUST require a mana payment equal to the current cost of `create_idea`.
- Restoring a burned relative-importance connection to active participation MUST require a mana payment equal to the current cost of `create_connection`.

Resurrection costs are applied at resurrection creation time (or other deterministically defined moment) and are governed by the same rate-limit and gating rules as creation actions.


## 7. governance, system-configuration, and protocol self-modification [anchor: 7_governance_system_configuration_and_protocol_self_modification]

### 7.1 governance ontology and scopes [anchor: governance_ontology_and_scopes]

Governance in the canonical universe is represented entirely through ordinary ideas. The protocol defines no special hidden governance objects, no out-of-band rulebook structures, and no dedicated connection types for expressing authority. Governance rulebooks and configuration rule sets are themselves ideas in the graph: they are written, argued about, challenged, ranked, and, when appropriate, explicitly adopted like any others. All proposals about how the system should behave - whether they concern voting parameters, safety constraints, identity verification, importance thresholds, POD routing, or any other operational principle - are expressed as ideas authored by identities and evaluated through the same public, challengeable processes that apply to every other idea. As a consequence, governance is not an external mechanism imposed on the graph; it is an emergent structure within the graph, derived from what identities collectively claim, argue for, challenge, adopt through explicit actions, and ultimately treat as important.

A governance idea is any idea that asserts a statement about how some aspect of the system ought to function. At the protocol level, governance ideas and rule set ideas are not a separate ontological category; they are distinguished by their content and, optionally, by small rulebook headers defined in governance specifications (for example, indicating which domain they govern). Governance ideas accumulate arguments, challenges, and importance like any others. A subset of these ideas - those that articulate coherent rule sets for a given domain - MAY be referenced by adoption actions. Nodes determine which governance rule sets are operative at any moment by replaying all events, applying resolved challenges to those adoption actions, and evaluating which adoption actions succeeded for each domain before a given cycle boundary. High universal importance, challenge-stability, and canonical clarity influence whether a rule set is likely to be successfully adopted, but they do not, by themselves, make it operative; activation requires an explicit, completed adoption action.



Within replay, nodes reconstruct governance state by evaluating governance-related actions and ideas at each cycle boundary. A snapshot records the derived active rulebook state at its block-height checkpoint for verification. These active rule sets determine how nodes interpret subsequent events until overridden by later rule sets adopted through normal deliberation and action challenges. Importance, challenge-stability, and clarity remain visible in the historical record as context for why certain rule sets were adopted, but they do not replace the adoption mechanism itself. This cycle-based activation model provides continuity-nodes do not reevaluate governance at every event-while preserving adaptability, because any governance idea or rule set can be superseded by a later one that wins an adoption action and becomes active from a subsequent activation cycle boundary.

### 7.3 governance rule sets and adoption actions [anchor: governance_rule_sets_and_adoption_actions]

While any identity MAY propose governance ideas, the system begins with a structured baseline contained in the Seed. These genesis governance ideas define the initial configuration of the system: challenge types, voting parameters, visibility rules, content boundaries, token behavior, and other structural principles required for early operation. These initial rule sets are ordinary ideas - authored, described, and challengeable like any others - but they provide an organized foundation for interpreting the system before the community has produced its own elaborations and alternatives.

Governance evolves through explicit **rule set ideas**. A rule set idea articulates a coherent bundle of rules for a particular domain, such as "challenge and voting rules", "token release rules", or "safety handling rules". Its descriptions SHOULD provide a clear human-readable summary of the rules it contains, and MAY reference more granular rule ideas via importance or representation connections. Multiple rule set ideas MAY coexist for the same domain, representing competing proposals, revisions, or historical versions (for example: `token_release_rules_v1`, `token_release_rules_v2`, `token_release_rules_v3`). All such rule set ideas remain ordinary challengeable ideas with no intrinsic privilege.

To make a rule set idea operative, identities MUST produce an **adoption action**. An adoption action is an actionable idea of the form "adopt rule set R for domain D". Like any actionable idea, it proceeds through the unified challenge lifecycle: arguments are gathered, action challenges may be opened, and a panel of jurors renders a verdict. If the adoption action is accepted and executed, the resulting completed action event SHALL schedule activation at a deterministic cycle boundary according to governance delay policy. Only one rule set may be active per domain at any given cycle boundary; repeated adoption actions supersede earlier ones.

Nodes reconstruct governance across replay by examining adoption actions. For each governance domain, the active rule set at cycle `r` is the rule set referenced by the most recent successful adoption action for that domain whose scheduled `activation_cycle_index <= r`. Earlier rule sets remain historically visible and may continue to influence argumentation, but they SHALL NOT govern behavior until reactivated by a later adoption. If no adoption action exists for a domain, nodes SHALL treat the corresponding genesis rule set from the Seed as active.

Truth claims MAY be authored to restate which rule set is currently active for a domain, and the descriptions of rule set ideas MAY themselves be challenged to improve clarity or correct errors. However, neither descriptive changes nor truth claims determine which rules are operative. The canonical source of governance activation is the sequence of completed adoption actions and their interpretation across cycle boundaries. This mechanism preserves the system's core uniformity - everything is an idea - while providing a clear, stable, fully challengeable method for specifying and updating the rules under which the canonical universe operates.

Governance rulebooks MAY define tunable parameters affecting derived lifecycle behavior and content packaging, including but not limited to:
- rot and burn thresholds,
- inactivity windows and hysteresis rules,
- seed-protection durations for new ideas and connections,
- eligibility criteria for cycle export pack inclusion,
- limits on export pack size, depth, and payload tiers.

All such parameters MUST be deterministic, versioned, and activated only at defined governance adoption boundaries.


### 7.4 meta-level constraints and anti-capture invariants [anchor: meta_level_constraints_and_anti_capture_invariants]

The protocol permits the system to reason about itself, modify its own rule sets, and revise its own operational procedures through actionable ideas and the associated challenge processes. However, there exist meta-level constraints that limit which kinds of internal decisions may ever become effective. These constraints prevent the system from adopting configurations that undermine the basic properties required for openness, challengeability, and durable civilizational reasoning. They do not create privileged governance objects, and they do not introduce new idea or challenge types. Instead, they define which forms of actionable ideas and truth claims can ever be treated as valid under deterministic replay.

First, governance decisions MUST remain public and challengable. No actionable idea, truth claim, description, or action MAY become effective if it asserts that governance-relevant proposals or decisions may be hidden, private, or shielded from challenge. Any attempt to create a private configuration rule, a private governance decision, or a private rule set SHALL be invalid under the protocol, regardless of how many identities support it. All rule-related ideas, arguments, actions, and truth claims MUST remain visible and challengable within the universal scope.

Second, no idea MAY claim the power to exempt itself or any other idea from challenge. Any actionable idea or truth claim that asserts that "challenges are no longer allowed on X," "identity Y is unchallengeable," or "rule set R may not be compared, replaced, or criticized" SHALL be invalid and MUST NOT be treated as true or active under any circumstances. The challenge mechanism is a structural invariant of the organism. Certain domains MAY require stronger evidence or higher deliberative thresholds, but challenge prohibition itself cannot be adopted as a rule.

Third, the protocol SHALL reject any proposal that attempts to disable the system's verification and evidence requirements. Any actionable idea that claims that "no evidence is required," "evidence cannot be attached," or "verification is suspended" is invalid. Evidence requirements MAY become stricter through accepted governance-relevant actions, but they MAY NOT be weakened to the point of undermining the truth-challenge framework or the determinism of replay.

Fourth, no actionable idea MAY assert exclusive authority for a single identity, tribe, or group. A claim that a specific identity "controls the system," "cannot be removed," or "has unilateral power to approve or block rule changes" SHALL be invalid. Authority in the protocol emerges from eligible public deliberation, not from structural exceptions. Tribes MAY maintain internal interpretations, but they MAY NOT define global rules or block public universal-importance or public-relative challenges.

Fifth, no actionable idea MAY disable the public universal-importance structure. Any attempt to remove universal importance, bypass the 20-axis structure, or redefine its semantics in a way that prevents public, challengeable ranking SHALL be invalid. Tribes MAY maintain public tribe-relative overlays and individuals MAY maintain noncanonical private relative maps, but the public universal-importance rank product MUST remain intact and challengeable.

Invalid ideas, actions, and truth claims remain visible in the historical record. They MAY be challenged, corrected, or used as evidence in subsequent meta-level deliberation, but their invalidity means they MUST NOT alter current rule applicability or system behavior during replay. A conformant node SHALL NOT apply any claimed configuration or rule change that contradicts these meta-invariants, regardless of whether the claim was supported by large numbers of identities or linked to widely accepted arguments. These invariants guarantee that no faction - however large - can disable the system's ability to reason, deliberate, or correct itself over time.

These constraints do not create a special governance layer. They simply define the forms of actionable ideas and truth claims that the protocol considers structurally possible. Everything else remains fully deliberative: rule sets are ordinary ideas; decisions are ordinary actions; and the system evolves through the same mechanisms that govern truth, importance, and action everywhere else in the canonical universe.

### 7.5 deliberation thresholds and quorum rules for system-configuration decisions [anchor: deliberation_thresholds_and_quorum_rules_for_system_configuration_decisions]

System-configuration decisions - those that modify how the protocol interprets rules, evaluates evidence, selects voters, or activates rule-set ideas - are expressed through ordinary actionable ideas. They do not form a special class of objects and do not rely on a separate governance mechanism. However, because these decisions alter how nodes interpret the canonical universe, they MAY require stronger deliberative conditions than ordinary, low-stakes actionable ideas. The protocol therefore defines optional deliberation thresholds that MAY be specified by ideas within the graph and evaluated through the same challenge system that applies everywhere else.

A deliberation threshold is an idea that describes a required condition for the acceptance of a particular class of actionable ideas. Examples include a minimum number of eligible voters, a minimum number of affirmative votes, a minimum evidence strength for a completion claim, or a requirement that multiple independent identities verify a decision-related action. These threshold ideas do not bind the system automatically. They MUST be referenced explicitly by the actionable idea that proposes a configuration change, and their relevance MAY be challenged if they are outdated, misapplied, or in conflict with higher-importance ideas governing meta-level constraints.

Quorum rules operate similarly. A quorum idea expresses the minimum number of eligible voters or deliberation participants that SHOULD participate in an action challenge related to system configuration. Because quorum itself is subject to challenge, quorum rules MUST be treated as contextual constraints rather than fixed protocol-level invariants. If a quorum idea is referenced by a system-configuration actionable idea, and its interpretation is uncontested at the time of voting, nodes SHALL treat that quorum requirement as part of the validity conditions of the resulting decision action.

If a referenced quorum requirement is not met, or if a referenced deliberation threshold is not satisfied, conformant nodes SHALL treat the proposed decision action as incomplete and SHALL NOT activate the associated configuration until a valid decision action is recorded that satisfies all referenced conditions. If a challenge asserts that the quorum or threshold requirement is itself invalid or misapplied, that challenge MUST be resolved before the corresponding action claim can be accepted as true. These mechanisms ensure that higher-stakes system-configuration decisions receive proportionally stronger scrutiny while still using the same challenge, action, and truth-verification processes that apply to all other ideas.

The protocol does not prescribe fixed global thresholds or quorums. Instead, thresholds and quorums are expressed as ideas that may rise or fall in importance over time. Their use is governed by deliberation and explicit reference, not by privileged status. This maintains both flexibility and determinism: system-configuration decisions remain fully challengeable, and nodes can replay them unambiguously by evaluating whether referenced deliberation conditions were met.


### 7.6 protocol self-description and reflexive clarity [anchor: protocol_self_description_and_reflexive_clarity]

The protocol is self-describing. Every structural feature of the system - the ontology of ideas, the meaning of challenges, the definition of evidence, the semantics of actions, and the rules governing significance flow - MUST be expressible within the canonical universe as a set of conceptual ideas and truth claims. These protocol-description ideas do not possess inherent authority. Their influence arises from their universal importance and from the fact that they are referenced by nodes during deterministic replay to interpret past events and validate future ones.

A protocol-description idea MAY define the meaning of an idea type, describe the semantics of relative importance, or articulate the constraints governing challenge validity. These ideas MAY have canonical descriptions selected through representation challenges, ensuring that the system’s self-understanding remains both human-readable and continuously improvable. Because canonical descriptions use a small controlled vocabulary, protocol-description ideas provide a stable semantic anchor for conformant implementations without becoming rigid or exempt from revision.

Reflexive clarity requires that protocol-description ideas remain fully public, fully challengable, and grounded in the same evidence and importance frameworks as all other ideas. The protocol SHALL treat any attempt to obscure, privatize, or exempt protocol-description ideas from challenge as invalid under the meta-level constraints defined in Section 8.4. Because nodes use these ideas during deterministic replay, clarity and visibility are essential: the canonical universe must plainly describe how it is to be interpreted, and this description must evolve through recorded deliberation rather than code alone.

When a system-configuration actionable idea proposes to revise or clarify a protocol-description idea, the resulting decision action MUST record the updated description or its replacement. A corresponding truth claim SHOULD assert that the change was executed correctly, and evidence SHOULD tie the change to physical actions such as code updates, signatures, or verified artifacts. This ensures that the protocol’s semantics evolve in synchrony with its implementation, and that nodes can reconstruct the system’s meaning at any snapshot by replaying descriptive ideas and decision actions.

### 7.7 the seed as governance nucleus but not authority [anchor: the_seed_as_governance_nucleus_but_not_authority]

"The Seed in My Mind" is an idea that serves as the conceptual nucleus of the organism. It anchors the system's meta-level reasoning, including the principles governing universal importance, the value poles of the individual and the collective, the semantics of evidence, and the meaning of POD. However, the Seed is not an authority and does not possess any special ontological privileges. It is an ordinary idea whose influence arises solely from its universal importance and from the deliberative structure built around it.

Ideas connected to the Seed - including those describing system-configuration norms, significance flow, identity structure, or safety boundaries - derive their relevance from the same mechanisms as all other ideas: public visibility, challengability, importance ranking, and acceptance through action and truth challenges. No description, connection, or configuration associated with the Seed MAY become unchallengeable or private. All structural propositions linked to the Seed MUST remain open to revision and subject to the same epistemic processes that govern the rest of the canonical universe.

The Seed functions as a governance nucleus in the sense that configuration-related actionable ideas frequently reference it, and because it often serves as the interpretive context for deliberation about protocol semantics. But its role is strictly interpretive. It does not issue commands, enforce rules, or confer power. Its centrality emerges because human identities choose to treat it as the root of system-wide meaning, not because the protocol assigns it a special mode.

As the system evolves, additional governance nuclei MAY emerge - projects, institutions, or large-scale initiatives that maintain their own internal rule sets and significance maps. These too are ordinary ideas whose governance structures are built from actionable ideas, actions, truth claims, and significance flow. The Seed remains the first and most general of these nuclei, but not a privileged one. Nothing in the protocol prevents other governance nuclei from rising in importance or overtaking the Seed in certain domains if deliberation supports that outcome.

In this design, the Seed provides conceptual coherence without centralizing authority. It is the initial point from which meta-level reasoning grows, not an immutable command structure. Its meaning, significance, and role evolve through the same open, recursive, challenge-driven processes that govern the entire canonical universe.

### 7.8 Governance parameters for rot/burn, snapshot ladder, and export packs (normative) [anchor: governance_parameters_for_rot_burn_snapshot_ladder_and_export_packs_normative]

Governance rulebooks define the parameters that control derived pruning, packaging, and survivability behavior of the protocol.

Governance MAY modify parameters that control long-horizon maintenance and packaging, provided such changes do not violate constitutional invariants and do not introduce new cycle sealing mechanisms outside §5.

Governable parameter families include:
- rot/burn horizons, thresholds, and revival rules (living-map maintenance),
- snapshot ladder configuration (frequency, granularity, retention, and pack composition),
- cycle export pack inclusion criteria and limits (selection determinism constraints).

All such parameter changes MUST have a well-defined effective date rule. Unless explicitly specified otherwise, parameter changes MUST become effective only at a future cycle boundary, and MUST be deterministically replayable from the canonical log and rulebooks.


#### 7.8.1 Rot and burn parameters [anchor: rot_and_burn_parameters]

Rulebooks MAY specify:
- importance thresholds for rot and burn,
- inactivity windows measured in cycles,
- hysteresis rules to prevent rapid state oscillation,
- minimum seed-protection periods after creation,
- conditions for automatic revival from rotted states,
- conditions and thresholds for resurrection.

Governance MAY set and modify rot/burn parameters, including:
- evaluation cadence (performed at cycle boundaries),
- inactivity horizons measured in cycles,
- threshold definitions for ideas and connections (including relative_importance edges),
- revival conditions and costs,
- visibility and UI lens behaviors that accompany living/dead status.

Rot/burn rules MUST be deterministic and replayable. They MUST NOT depend on trusted wall-clock time, raw timestamps, or node-local timers; where time is relevant, it MUST enter only via adjudicated time-claim predicates as defined in the Tempo specification.

Rot/burn MUST NOT affect whether a cycle seals. Rot/burn is executed after a boundary is derived; it is not an input to cycle completion (§5).


#### 7.8.2 Snapshot ladder parameters [anchor: snapshot_ladder_parameters]

Rulebooks MAY specify:
- block-height intervals for snapshot tiers,
- required contents for each snapshot tier,
- retention policies for snapshot storage (without deleting events),
- minimum survivability guarantees for archival snapshots.

#### 7.8.3 Cycle export pack parameters [anchor: cycle_export_pack_parameters]

Rulebooks MAY specify:
- selection criteria for inclusion in cycle export packs,
- limits on number of ideas and connection depth,
- payload tiers and summary formats,
- scope and lens constraints applied during selection.

All parameters defined in this section MUST:
- be deterministic,
- be challengeable through governance processes,
- activate only at explicit adoption boundaries,
- preserve the ability of any node to reconstruct canonical state from the event log.

Governance MAY set and modify export pack parameters, including:
- selection criteria (e.g., top-N by universal importance, neighbor depth K, scope filters),
- included payload tiers (sentence/paragraph/full; fundamental/standard/advanced/canonical),
- limits on size, breadth, and historical context inclusion,
- retention recommendations and optional compression formats.

All export pack rules MUST be deterministic and regenerable from the canonical event log and rulebooks. Export packs MUST NOT be required for validation or replay and MUST NOT influence canonical computation.

Export pack metadata MUST always include cycle-boundary cadence context sufficient to interpret the pack:
- cycle index,
- seal type (deliberative vs forced),
- Dmin/Dmax predicate satisfaction,
- and the cycle's derived completion inputs (V, C, W_score) as defined in §5.


## 8. identity, users, ai boundaries, and ents [anchor: 8_identity_users_ai_boundaries_and_ents]

### 8.1 human-first identity architecture [anchor: human_first_identity_architecture]

The protocol maintains a strict human-first architecture for the canonical universe. Ordinary canonical events - idea creation, description submission, connection creation, argument or evidence idea submission, challenge creation, vote, action declaration, or completion claim - MUST be authored and cryptographically signed by an eligible human identity, except for mechanically emitted boundary events by `system_boundary_emitter`. Verified-human status is necessary but not sufficient for every action: ordinary canonical writes require ordinary canonical-writer eligibility, and the narrow Tempo-only lane requires `tempo_contributor` eligibility and permits only target-bound time truth claims plus explicitly allowed Tempo-context evidence ideas/connections. Only eligible verified human identities MAY emit POD-eligible events or directly modify canonical rankings and challenge outcomes.

Profile-v0 identity admission follows this sequence:

1. permissionless local identity and key preparation;
2. a portable non-canonical admission request;
3. a sponsor-authored canonical `identity_create`;
4. a `CanonicalAdmittedIdentity` with restricted initial authority.

Local preparation requires no canonical permission. Admission requests are non-canonical and portable. Only successful canonical event application creates identity state. The applicant MUST separately prove possession of the proposed initial key, but that proof does not make the applicant the canonical event author. The existing eligible human sponsor authors and signs the completed `identity_create` authored candidate. Exact proof bytes, proof fields, sponsor signature fields, payload schemas, and encodings are deferred to Appendix A, `canonical-encoding-and-hashing-spec.md`, and `canonical-event-authorship-and-signature-profile-v0.md`.

The system conceptually distinguishes three layers: the **canonical universe**, an **anonymous outer layer**, and the **AI map**. The canonical universe is the POD-bearing layer, backed by event logs and snapshots, where universal importance, challenge outcomes, POD flows, and long-term governance are defined. Only identities with the relevant replay-derived event-family eligibility MAY act directly in this layer. A `CanonicalAdmittedIdentity` has only the restricted identity-scoped verification and key-control authority later defined by exact schemas; it does not have ordinary writing, challenge, voting, governance, Tempo, inviter, or economic authority merely because admission succeeded. The anonymous outer layer is a non-canonical, web-facing space where ideas MAY be created and discussed under fully anonymous or burner-like handles without prior verification. This layer is intended to support whistleblowing, dissent, experimentation, and expression under threat, including use from within authoritarian regimes. Events in the anonymous layer are not POD-eligible and MUST NOT change canonical ranks or challenge results until they are explicitly adopted. The AI map is a separate sandbox in which AI agents MAY freely propose ideas, structures, and interpretations without any canonical authority.
Outer-layer content remains non-canonical until adopted by a verified-human identity, and such adoption preserves source anonymity by default and MUST NOT embed transport metadata (see `privacy-and-high-risk-submission-spec.md` §6§7).
For clarity, the anonymous outer layer is non-canonical publication, while canonical authorship remains identity-signed pseudonymous, including anonymous-but-verified presentation where allowed.

Canonical authorship is always grounded in a canonical identity that represents a real human. A single verified human identity MAY present itself through multiple visible pseudonyms or burner handles within the canonical universe and outer layer. These visible surfaces MAY obscure the underlying human's legal identity and MAY be created or discarded at will, but all canonical events emitted by those surfaces MUST route through the same underlying identity for the purposes of authorship, rate limits, POD assignment, and Sybil resistance. The mapping from visible handles to underlying identities MAY remain private and node-local, but replay and validation MUST treat all events from those handles as originating from one canonical human identity.

The protocol distinguishes **identity existence**, **identity visibility**, **verification**, and **event-family eligibility**. A canonical identity may exist before VH or VI certainty has activated. However, the interface MAY present an identity as a pseudonym, a theme-based handle, or a fully redacted "anonymous" label. Anonymous participation in the canonical universe therefore means that the identity's public-facing label is hidden or pseudonymous, not that canonical event-family eligibility can bypass replay-derived identity rules. Purely unlinked, unverifiable accounts MAY exist only in the anonymous outer layer; they MUST NOT act directly in the POD-bearing canonical universe.

Anonymous or burner submissions in the outer layer MAY be used as inputs to canonical reasoning. A verified human identity MAY adopt non-canonical content (whether from anonymous humans or AI agents) by explicitly submitting it as their own canonical statement. Adoption means: "I, this human identity, say X," even if the original author was anonymous. Once adopted, the idea becomes a canonical idea or description authored by the adopting human, and only then MAY it participate in canonical challenges, rankings, and POD flows. This adoption mechanism allows the system to ingest information from high-risk or anonymous contexts while keeping the canonical universe grounded in accountable human authorship.

Each Profile-v0 canonical human identity MUST have the complete `identity_structural_roots` set: Mindgarden, Backyard of Relationships, Self Tree, and Anthill. A successful `identity_create` MUST atomically create or deterministically derive the complete root set; failure to establish any required root fails admission. Structural roots organize identity-related information and MUST NOT create verification, truth, importance, voting, governance, Tempo, invitation, POD, POINT, or economic authority.

AI identities MAY exist as named agents within the AI map, but they SHALL NOT directly author canonical events. AI identities MAY generate candidate ideas, arguments, or descriptions, but these remain non-canonical until a verified human identity adopts them. When such adoption occurs, the resulting canonical event is entirely owned by the human identity, with any AI contribution treated as a tool-assisted draft. This preserves human responsibility and agency over all canonical claims, while still enabling extensive AI support and exploration in the non-canonical layers.

### 8.2 identity properties and verification states [anchor: identity_properties_and_verification_states]

Each canonical identity in the canonical universe MUST carry a minimal, deterministic set of replay-derived facts sufficient for authorship, admission provenance, key control, structural-root reconstruction, eligibility, POD assignment, tribe eligibility, and verification checks. These facts MUST be recorded in the event log or deterministically derived from it. Nodes MUST NOT rely on external, non-replayable state when validating canonical events.

Identity authority is not one mutable lifecycle status. Conformant replay MUST keep the following lanes or predicates separate:

- canonical existence;
- identity kind;
- key control;
- structural-root completeness;
- verification state;
- restricted verification eligibility;
- ordinary writer eligibility;
- ordinary challenge eligibility;
- voter eligibility;
- governance eligibility;
- Tempo eligibility;
- inviter eligibility;
- invitation capacity;
- invitation suspension;
- dormancy or recovery.

At minimum, a Profile-v0 canonical human identity MUST include:

- **canonical admission facts:**
  A canonical identity identifier, `identity_kind = human`, admission profile provenance, sponsor/admission provenance, and classification as a `CanonicalAdmittedIdentity` or a separate genesis/legacy/import provenance class. Profile-v0 admission alone does not establish VH, VI, human uniqueness, civil identity, trustworthiness, ordinary writing, ordinary challenges, voting, governance, Tempo eligibility, inviter eligibility, invitation capacity, POD, POINT, or economic authority.

- **public key state and signature profile:**
  Replay-derived identity key state sufficient to resolve active `public_key_ref` values under `canonical-event-authorship-and-signature-profile-v0.md`. Ordinary human-authored canonical events MUST carry a Profile-v0 `signature` over the exact authored-candidate bytes and MUST verify against an active key owned by `author_identity_id`. Nodes MUST reject events with missing signatures, invalid signatures, unknown keys, keys owned by another identity, revoked keys used after revocation, or unsupported signature profiles. Profile-v0 identity admission also requires an applicant initial-key possession proof whose exact bytes are defined outside this root protocol document.

- **identity structural roots:**
  The complete Profile-v0 `identity_structural_roots` set: Mindgarden, Backyard of Relationships, Self Tree, and Anthill. These roots MUST be created or derived atomically with successful admission and MUST persist for the lifetime of the identity. They are structural anchors only and do not confer verification or participation authority.

- **verification and eligibility lanes:**
  Verification is derived from ordinary truth claims, evidence, contradictions, challenges, responses, and outcomes, followed by rulebook evaluation, derived VH and VI certainty, activation boundary, and event-family-specific eligibility. Verification certainty and tiers MUST NOT be used to weight votes, importance, governance power, truth certainty, challenge influence, Tempo influence, invitation capacity, POD, POINT, or economic authority. `verification_reference`, sponsorship, admission lineage, Anthill membership or degree, and structural-root membership are not verification by themselves.

- **current POD share:**
  The non-transferable POD deterministically derived for the identity from eligible canonical events and the universal-importance routing state at the latest qualifying cycle boundary. It MUST remain stable under replay of that same boundary, but it is not cumulative or permanent and MAY rise, fall, or become zero at later boundaries. Historical boundary results remain replayable and MUST NOT be retroactively rewritten.

- **POINT balance:**  
  The balance of tradable POINT tokens associated with the identity, as defined in the token specification. POINT MAY be transferred, spent, or used to fund actions, but these operations MUST be recorded as canonical events and MUST be replayable.

- **tribe memberships:**  
  A set of canonical `membership` connections from the identity to tribe nucleus ideas, specifying tribe-scope participation. Tribe memberships determine which tribe-scope challenges the identity MAY vote in or help create. Membership changes MUST be represented as canonical events and MUST be reproducible during replay.

- **visibility and naming metadata:**  
  Optional properties describing how the identity is presented at the interface layer (for example, public name, pseudonym, avatar, or fully anonymous label). These values MAY change over time, but changes MUST be recorded as events rather than in-place mutations. Visibility metadata MUST NOT affect canonical authorship, rate limits, POD assignment, or verification state.

- **death or succession markers:**
  When an identity is marked as deceased or when transferable responsibilities and eligible POINT entitlements pass to a successor, this MUST be recorded as a canonical event referencing both the original and successor identities. These markers ensure that replay can interpret historical authorship, historical POD derivations, future POD exclusion, and permitted POINT succession consistently. POD itself is never transferred or inherited.

Non-canonical anonymous identities operating solely in the outer layer MAY have reduced or ephemeral property sets and MAY exist without verification. However, any transition from anonymous outer-layer activity to canonical authorship MUST pass through an identity with the event-family eligibility required for the target canonical action, either by the identity becoming eligible through verification/rulebook activation or by adoption of content by an already eligible verified human identity. Nodes MUST NOT treat outer-layer pseudonyms, burners, private product accounts, or local identity candidates as canonical identities unless and until a valid canonical admission or legacy/genesis rule establishes them.

Detailed verification mechanics, evidence structures, certainty derivation, and restricted verification lanes are defined in the identity verification and identity-admission specifications and are normatively referenced here.


### 8.3 structural roles assigned at identity admission: Mindgarden, Backyard of Relationships, Self Tree, Anthill, and shrubs [anchor: structural_roles_assigned_at_identity_creation_mindgarden_backyard_of_relationships_self_tree_anthill_and_shrubs]

Each Profile-v0 canonical human identity is accompanied by the complete `identity_structural_roots` set. The future-facing root names are:

1. Mindgarden;
2. Backyard of Relationships;
3. Self Tree;
4. Anthill.

A valid Profile-v0 `identity_create` MUST atomically create or deterministically derive the complete root set. Failure to establish any required root fails admission. These structural roots are NOT new canonical idea_type values; they are ordinary structural anchors or deterministic derivations whose exact identifiers, byte encodings, structural-role constants, containment relations, and connection schemas remain deferred to Appendix A and structural-role reconciliation. All conformant nodes MUST reconstruct this identity-root structure deterministically under replay.

Mindgarden

The Mindgarden is the identity's deterministic personal incubation space for authored ideas, drafts, and user-specific working spaces. It does not define an importance scope, does not receive POD, and does not participate in ranking. Its purpose is to anchor authored ideas and drafts in a deterministic location while keeping pre-public and personal incubation meaning separated from public truth, importance, and action processes.

Backyard of Relationships

The Backyard of Relationships is the container for interpersonal and intrapersonal relationship structures. It SHALL NOT participate in universal, tribe, or personal importance ranking and SHALL NOT receive POD. Relationship-specific structures, such as shrubs, relationship memory leaves, and the Self Tree, MAY appear under this space when the exact structural-role schema permits them. The Backyard of Relationships separates relational meaning from epistemic meaning so that personal narrative and connection structures do not pollute public truth or importance graphs.

Self Tree

The Self Tree represents the identity's evolving internal narrative. It MAY have personal memory leaves attached, representing milestones, reflections, or experiences meaningful to the identity. The Self Tree SHALL NOT participate in challenges, SHALL NOT receive POD, SHALL NOT influence importance rankings, and SHALL NOT act as evidence or argument in any canonical epistemic process. It exists solely as a replayable narrative anchor unique to the identity.

Shrubs

Shrubs are optional relational structures, not required identity structural roots. A shrub represents a pairwise relationship between two human identities. A shrub MAY remain visible only to the creating identity or MAY become mutual if the second identity explicitly accepts or mirrors the relationship. Shrubs MAY contain relationship memory leaves, which represent shared events or moments visible to both participants. Shrubs SHALL NOT participate in importance ranking, SHALL NOT receive POD, SHALL NOT alter governance structures, and SHALL NOT introduce epistemic implications. They are strictly narrative and relational structures, not public-knowledge claims.

Anthill

The Anthill records mutually acknowledged human connections and may provide a structural location where admission provenance, social acknowledgments, or verification-relevant claims can be inspected. Anthill membership, degree, lineage, or topology SHALL NOT influence importance, POD, governance, challenge outcomes, VH, VI, invitation authority, or any other eligibility lane by itself. A sponsor or peer may separately make an attributable and challengeable verification claim through the ordinary epistemic system, but the Anthill is a structural root, not verification.

Together, the Mindgarden, Backyard of Relationships, Self Tree, Anthill, and optional shrubs define personal and interpersonal terrain for an identity. These structures provide deterministic anchors for authored ideas, personal drafts, relational memories, and peer networks while keeping them rigorously separated from public truth, importance, admission capacity, verification certainty, eligibility, and action processes. Implementations MAY build rich UI metaphors on top of these structures, but MUST preserve their semantic boundaries, SHALL NOT treat structural roles as canonical idea types, and MUST NOT extend their authority into POD, ranking, governance, Tempo, invitation, or economic domains.



### 8.4 identity lifecycle: creation, verification, and continuity [anchor: identity_lifecycle_creation_verification_and_continuity]

Each human identity represents a single real person and persists for the duration of that person's participation in the system. Under Profile v0, canonical identity creation begins only when a valid sponsor-authored `identity_create` is successfully applied under the active admission profile. Permissionless local key generation, local identity preparation, local account creation, private product registration, signed admission requests, and relay transport do not create canonical identity state. The target kind is fixed as `identity_kind = human`.

A successful Profile-v0 admission creates a `CanonicalAdmittedIdentity` with canonical existence, human target kind, accepted initial key, sponsor/admission provenance, and complete `identity_structural_roots`. Admission alone does not establish VH, VI, human uniqueness, civil identity, trustworthiness, ordinary writing, ordinary challenges, voting, governance, Tempo eligibility, inviter eligibility, invitation capacity, POD, POINT, or economic authority.

Verification follows the ordinary epistemic path: ordinary truth claims, evidence, contradictions, challenges, responses, and outcomes; rulebook evaluation; derived VH and VI certainty; activation boundary; and event-family-specific eligibility. Sponsorship, admission lineage, Anthill membership or degree, structural-root membership, invitation spending, and `verification_reference` do not automatically establish verification or ordinary authority. A sponsor MAY separately make an attributable and challengeable verification claim through the ordinary epistemic system.

Identity continuity is preserved through stable cryptographic signatures and replay-derived key-management events. A USER identity MUST control at least one active key or a canonically specified recovery path, and MAY rotate or revoke keys through signed key-management events under `canonical-event-authorship-and-signature-profile-v0.md`. The identity's underlying canonical representation MUST remain constant under replay; only descriptions and metadata MAY change. The identity's lifetime ends only through explicit death, retirement, or succession events defined in a rulebook. The identity_structural_roots - Mindgarden, Backyard of Relationships, Self Tree, and Anthill - remain attached to the identity throughout its lifetime and SHALL NOT be merged, reassigned, or severed except through formal succession processes.

### 8.4A invitation eligibility, capacity, and admission liveness [anchor: invitation_eligibility_capacity_and_admission_liveness]

Inviter eligibility is a separate replay-derived lane. It is not ordinary writer eligibility, challenge eligibility, voting eligibility, governance eligibility, Tempo eligibility, economic authority, or current capacity balance. It MUST be generally attainable by humans satisfying the same objective verification, continuity, maturation, key-control, and good-standing rules. No permanent founder, operator, institution, expert, delegate, governance office, or genesis inviter class may hold exclusive admission authority.

Every inviter-eligible, unsuspended human MUST receive at least one spendable invitation-capacity unit in each qualifying capacity period. Rulebooks MAY set higher rates, finite caps, carryover, expiration, maturation, suspension, restoration, and abuse controls. Rulebooks MUST NOT assign zero capacity indefinitely to otherwise eligible, unsuspended inviters.

Invitation capacity is replay-derived, integer-valued, identity-bound, non-transferable, non-saleable, non-delegable in Profile v0, and bounded. It is not money, a token, reputation, verification certainty, truth weight, importance weight, or vote weight. Exact Profile-v0 invitation capacity is deterministically derivable from public canonical history and applicable rulebooks. A DTO or interface MAY omit or simplify the displayed amount, but that is presentation minimization, not cryptographic privacy.

Qualifying invitation-capacity periods require properly certified human-deliberative cycles. Wall-clock passage, cron activity, AI activity, system-emitter activity, Dmax alone, forced boundaries, degraded boundaries, survivor boundaries, record-only boundaries, and machine-only boundaries do not generate invitation capacity, inviter eligibility, inviter maturation, carryover-cap increases, admission rewards, or suspension restoration merely by occurring. They may generate admission authority only if they independently satisfy the human-deliberative certification rules for a qualifying capacity period.

Previously valid spendable capacity remains usable during a stall unless it was suspended, previously scheduled to expire, frozen by a canonical emergency rule, or restricted by another explicit constitutional rule. A stalled period MUST NOT silently destroy capacity. When no qualifying period occurs, replay MUST expose `admission_liveness_blocked = true`, or an equivalent deterministic state. That state means no new capacity generation, no maturation advancement, and no inviter-eligibility activation; admission depends only on existing valid capacity. Profile v0 permits no emergency invitation-capacity minting by operators, AI, system emitters, wall-clock processes, or machine-only cycles. Exact replay, snapshot, API, and conformance representations are deferred to the subordinate reconciliation tasks.

### 8.5 death, succession, and account freezing [anchor: death_succession_and_account_freezing]

The protocol defines a small number of identity-terminal events to preserve determinism when a real human dies, disappears, or formally transfers stewardship. A USER identity MAY issue a "succession declaration" as an actionable idea if they wish to predefine who may inherit eligible POINT or assume explicitly transferable responsibilities. POD, verification status, authorship, and epistemic lineage MUST NOT be inherited or reassigned. This succession plan becomes active only if a later completion claim asserts that the real person has died or permanently ceased participation. The completion claim is a truth claim subject to challenge and evidence rules, requiring verifiable proof such as death certificates or trusted third-party attestations.

When a real human is canonically marked deceased, the identity enters a frozen state regardless of whether a valid succession plan exists. A frozen identity cannot create new events, vote, or participate in challenges. Its authored ideas, historical contributions, attribution lineage, and completed-cycle POD derivations remain fully intact, but the identity ceases to be a terminal POD sink and its current POD becomes zero in future cycles. Nodes MUST prevent any attempt to revive, reuse, or impersonate a frozen identity.

Succession events transfer stewardship but not authorship. Where private-product policy permits succession, a successor may manage designated private drafts and private relative maps, but those permissions remain outside canonical rank semantics. A successor MUST NOT retroactively modify or impersonate the dead identity's authored ideas or events. Past canonical events remain exactly as they were.

### 8.6 identity keys, signatures, and attribution rules [anchor: identity_keys_signatures_and_attribution_rules]

All ordinary human-authored canonical events in the system MUST be attributed to a specific human identity through cryptographic signatures. Every event - idea creation, description proposal, connection formation, challenge creation, argument submission, vote cast, action declaration, and completion claim - MUST include a Profile-v0 `signature` from an active key owned by `author_identity_id` at the event candidate's applicable publication point. The exact authored-candidate structure, signed bytes, `public_key_ref` construction, key rotation, revocation, and non-retroactive key-state rules are defined in `canonical-event-authorship-and-signature-profile-v0.md`.

For Profile-v0 `identity_create`, the sponsor's active key signs the authored candidate as `author_identity_id`. The applicant separately proves possession of the proposed initial key. That possession proof binds applicant-relevant admission fields but does not make the applicant the event author and does not require the proposed initial key to have been active before admission. Exact possession-proof bytes and payload placement are deferred to Appendix A, canonical encoding, and Profile-v0 authorship/signature reconciliation.

Key rotation is allowed and encouraged. A USER identity MAY generate a new keypair and attach it through a signed key-rotation event authorized by an active prior key or by a canonically specified recovery process. A valid rotation becomes effective at its finalized canonical position. A valid revocation becomes effective at its finalized canonical position and does not invalidate earlier finalized events that were valid when published.

Attribution rules are strict and deterministic. An event is considered authored by `author_identity_id` only when the signed candidate verifies against an active key descriptor owned by that identity and the identity is eligible for the event family. AI identities MAY propose drafts in the sandbox but MAY NOT satisfy ordinary human-authorship signatures. Tribe identities MAY NOT sign ordinary human-authored events. Events without valid attribution MUST be rejected to maintain non-repudiation and prevent injection of unsigned or forged actions into the canonical universe. These rules ensure that every ordinary human-authored canonical event is tied to a real human and remains permanently traceable under replay.

### 8.6 death, succession, and archival identity states [anchor: death_succession_and_archival_identity_states]

Identity continuity is a core invariant for deterministic replay. Every canonical event is authored by a persistent human identity, and the meaning of authorship MUST NOT depend on off-chain assumptions about whether that human remains alive or active. For this reason, the protocol defines identity "lifecycle markers" that allow nodes to interpret authorship, POD flows, and eligibility constraints consistently across long time periods, including generational transitions.

A human identity MAY be marked as *deceased* by a canonical event referencing sufficient evidence, but such marking SHALL NOT retroactively alter any event authored by that identity or any POD result derived for an earlier completed cycle. Death prevents future canonical authorship, removes the identity as a terminal POD sink, and causes its current POD to become zero in subsequent cycles. Tribe memberships remain historical facts, and governance actions previously taken remain in the immutable record. Deceased identities MAY remain visible in tribe rosters or relationship structures as part of that record, but SHALL be treated as inert for eligibility, voting, and rate-limit purposes.

Succession provides a forward-only mechanism for transferring governance-relevant responsibilities or stewardship roles across generations, without mutating historical authorship. A succession event explicitly references a predecessor identity and a successor identity, marking that certain responsibilities or interpretive roles formerly associated with the predecessor SHALL be recognized as associated with the successor from that moment forward. Succession MAY occur within families, institutions, tribes, or decentralized stewardship groups. The protocol does not prescribe when succession should occur; it prescribes only how it MUST be represented to remain deterministic.

Succession NEVER implies that the successor "is" the predecessor. Historical authorship and completed-cycle POD derivations remain attributed to the original identity forever, but no current POD is transferred to the successor. Succession marks continuity only for explicitly transferable practical responsibility - e.g., operating an archival node, managing a tribe's external obligations, or assuming separately authorized custody - and for POINT inheritance permitted by the token specification. It MUST NOT rewrite past events.

Archived identities represent a final lifecycle state. When an identity has no further canonical authorship and no active responsibilities, an archival marker MAY be added to signal that nodes MAY minimize that identity’s active-state memory footprint, provided that all past events remain replayable. Archival states SHALL NOT alter historical verification facts, completed-cycle POD derivations, or tribe membership history. Nodes MUST continue to treat archived identities as valid authors of historical events, and all references to them MUST resolve deterministically.

When an identity is marked as deceased, no further POD-eligible events MAY be authored by it, its current POD MUST become zero through recomputation, and no POINT may be minted or redistributed to it from POD in subsequent cycles. Existing POINT is resolved through the token specification's deterministic inheritance hierarchy. Historical contribution remains visible through authored events, attribution lineage, and replayable completed-cycle POD results, not through a preserved POD balance.

---

### 8.7 identity-level activity records and auditability [anchor: identity_level_activity_records_and_auditability]

Identity-level activity MUST remain auditable, replayable, and reconstructible from canonical events alone. The protocol does not maintain mutable per-identity logs; instead, a node reconstructs an identity’s activity history by filtering canonical events authored by that identity. This ensures that identity-level audit trails remain deterministic and identical across all conformant nodes. Any presentation-layer timeline or activity summary MUST be derived solely from event replay, without any node-specific caching that could diverge from canonical state.

Every canonical event includes the author's identity, a signature validating the authorship, and all metadata required to verify that the identity was eligible to perform that action at the time of authorship. Nodes MUST validate authorship eligibility at replay time by examining rate limits, verification state, tribe memberships (for tribe-scope actions), and any governance constraints active at that historical moment. This guarantees that identity-level audits remain consistent regardless of when or where they are performed.

Identity activity is conceptually divided into three categories: epistemic actions, governance actions, and social-graph actions. Epistemic actions include idea creation, description proposals, argument or evidence submission, challenge creation, and vote casting. Governance actions include participation in rule-set adoption challenges, protocol-upgrade decisions expressed as challenge outcomes or rule-set selections, and any stewardship actions defined by governance ideas. Social-graph actions include modifications to Mindgarden, Backyard of Relationships, Self Tree, Anthill, shrubs, and related structural-root or relational structures; these actions define personal and interpersonal meaning but MUST NOT alter canonical epistemic or governance semantics.

For each category, identity-level auditability enables important downstream behaviors: detecting Sybil patterns, analyzing long-term POD distributions, reconstructing debate histories, evaluating trustworthiness or expertise through interpretable metrics, and enabling transparent governance participation records. The protocol mandates that all such analyses MUST be derivable solely from open canonical data, without requiring privileged access to any non-canonical metadata. Nodes MAY offer cached summaries or accelerated index structures for browsing efficiency, but such accelerations MUST be fully reconstructible from first principles.

Finally, the protocol enforces the principle that identity accountability is forward-only. An identity's past authorship cannot be obscured, anonymized, or deleted, although pseudonym rotation and visible-name changes MAY occur through canonical events. This ensures that the historical evolution of the graph - the ideas articulated, the challenges raised, the votes cast - remains transparently associated with the human agents who shaped it, forming a stable substrate for collective reasoning, POD attribution, and long-term governance integrity.

### 8.8 Private cognition and non-canonical drafts [anchor: private_cognition_and_non_canonical_drafts]

The protocol distinguishes between canonical authorship and private cognition.

Private drafts, notes, and unpublished ideas are non-canonical. They do not participate in ranking, governance, or replay, and they impose no requirements on conformant nodes.

Canonical authorship begins only when an agent explicitly publishes an event into the canonical event log. Prior cognition, including offline deliberation and drafting, exists outside protocol governance.

Implementations MAY support private cognition using local storage mechanisms. Such content:

- SHALL NOT be required for protocol conformance.
- SHALL NOT be visible to other agents unless explicitly published.
- MAY be protected using user-controlled encryption.

Nodes MUST NOT require access to private drafts or encryption keys. Canonical identity, eligibility, and accountability apply only to published canonical events.

This separation ensures that freedom of thought precedes public accountability, while preserving the integrity of the canonical universe.

Private cognition and drafting occur entirely outside the canonical universe.

The protocol distinguishes between:
- **non-canonical drafts:** local ideas, connections, arguments, or notes that have not been committed to the event log, and
- **Mindseed identity vaults:** private, encrypted storage for identity keys, credentials, and sensitive verification materials.

The canonical event log MUST NOT store raw personally identifiable information (PII). Instead, it MAY store:
- commitments,
- attestations,
- cryptographic references,
- and challengeable verification claims.

Mindseed identity vault contents are never required for canonical replay and remain under the sole control of the identity holder. Canonical verification relies only on published commitments and attestations.



## 9. safety, visibility, and emotional load [anchor: 9_safety_visibility_and_emotional_load]

### 9.1 safety as minimal constraint, the principle of universal ideas, and protection against retroactive sanitization [anchor: safety_as_minimal_constraint_the_principle_of_universal_ideas_and_protection_against_retroactive_sanitization]

Safety in the protocol is a minimal, legality-driven constraint on payloads, not a constraint on ideas or on meaning. The sole purpose of the safety layer is to prevent the canonical universe from storing payloads that are globally illegal, intrinsically unsafe to distribute, or technically hazardous. Safety MUST NOT evaluate correctness, offensiveness, morality, political acceptability, cultural norms, reputational concerns, or shifting social standards. If an idea can be expressed in legally compliant language, the system MUST make a place for it in the canonical universe.

The protocol defines a **global minimal safety rule-set** that lists only those payload categories that are nearly universally illegal and that cannot be hosted by any lawful global infrastructure. These include, for example, CSAM, involuntary doxxing of private individuals, and highly operational instructions for imminent, targeted harm. This global minimal rule-set is "constitutional": it MUST remain extremely narrow, MUST NOT expand in response to ordinary political, cultural, or moral shifts, and MUST NOT be used to suppress ideas that are merely controversial or socially disfavored.

A node MAY apply local-law visibility or storage overlays (§10.4) when operating in restrictive jurisdictions, but such overlays MUST NOT change which ideas are canonically admissible. They MAY obscure or blur content locally but MUST NOT claim that such content is unsafe under the global rule-set unless it actually satisfies the narrow global criteria. Ideas that are lawful somewhere MUST remain modelable everywhere, even if certain jurisdictions forbid their display. Local-law constraints may affect access but MUST NOT affect canonical existence.

The protocol explicitly prohibits **retroactive sanitization** of historically admissible content based on later changes in cultural norms, social values, or jurisdictional laws. Content that was admissible under the global minimal safety rule-set at the time of admission MUST NOT be encapsulated, transformed, or altered solely because a later era becomes more restrictive. Retroactive transformation is permitted only when new evidence shows that a past payload was always within a globally illegal category (for example, when a previously unnoticed CSAM image is discovered decades later). Such transformations MUST be recorded as explicit sanitization events and MUST NOT erase the fact that the original payload existed.

This requirement preserves the epistemic integrity of the historical record. A living civilization can only understand its progress by remembering what came before; history includes error, conflict, cruelty, and ignorance alongside insight and achievement. The canonical universe MUST remain an honest record of human expression, including the parts later generations regret. Nodes MAY provide optional UI-layer guidance, warnings, emotional-load filters (§10.6), or contextual framing for distressing historical content, but these UI adaptations MUST NOT change the canonical payload or obscure the fact that the past was different from the present.

Safety is therefore a *narrow conduit* for legality, not an editorial mechanism. It removes only that which no lawful infrastructure may transmit, while preserving - and making challengeable - every idea humans attempt to express. By guaranteeing the perpetual visibility of the historical record, the protocol ensures that meaning, progress, and collective self-understanding remain grounded in truth rather than in revision or forgetting.

---

### 9.2 blocked submissions, classifier disagreement, and anti-censorship guarantees [anchor: blocked_submissions_classifier_disagreement_and_anti_censorship_guarantees]

A blocked submission is a proposed canonical event whose payload cannot legally or safely be admitted into the canonical universe. The protocol permits blocking only for payload reasons - not for ideas, opinions, political views, or any epistemic content that can be expressed legally. The blocking mechanism MUST therefore be the narrowest safety filter in the system, with censorship resistance as a core design criterion.

A node MUST NOT block a submission based on a single classifier or model. Instead, each node MUST run an ensemble of independent, governance-approved classifiers, each evaluating the payload under standard content-safety categories (for example: child sexual abuse material, explicit threats, involuntary personal data disclosure, malware payloads, and other globally illegal or genuinely unsafe content). The node MUST then apply a deterministic N-of-M aggregation rule defined in the rule-set ideas in §8. If any classifier in the ensemble returns a safe verdict, the submission enters a delayed canonical state and MUST be subject to a human-visible review and an optional challenge window. Only if all classifiers agree that the payload is illegal or intrinsically unsafe may the submission be blocked immediately.

Every blocked submission MUST generate a blocked_submission event in the canonical universe. This event MUST NOT contain any illegal payload; it MUST instead include a maximally descriptive safe-language summary of what the user attempted to submit, written by the node’s transformation layer according to deterministic rules. The summary MUST include: (a) the semantic content category of the attempted submission; (b) the reason it was blocked; (c) which classifier categories triggered; (d) the classifier disagreement profile; and (e) an explicit link to initiate a wrongful-block challenge. The summary MUST preserve as much meaning as legally permissible, so that epistemic context is never erased, and the canonical universe remains an honest record of the user’s intent.

A blocked submission MUST remain challengeable. Any verified human MAY initiate a wrongful-block challenge arguing that the classifier ensemble or aggregation rule erred. If the challenge succeeds, the idea MUST be admitted in a sanitized or transformed form, or re-submitted verbatim if the payload is determined lawful and safe. The blocked_submission record MUST remain in the event history to reflect that a block occurred, but the verdict MUST link to the corrected canonical event. This challengeability ensures that blocked submissions cannot be used as a censorship vector; wrongful blocking becomes visible, auditable, and correctable.

Blocked submissions MUST NOT influence importance rankings, POD flows, or canonical reasoning directly. They are accountability artifacts, not epistemic contributions. However, nodes MUST preserve their full event structure during replay so that historical safety decisions, censorship pressures, and classifier drift remain detectable. The protocol defines an explicit censorship pressure metric derived from the rate and clustering of blocked submissions; elevated pressure MAY trigger governance review of classifier ensembles or aggregation rules.

Under no circumstances may a node block a submission for reasons outside the narrow list of globally illegal payload types defined in the rule-set ideas. Ideas themselves MUST NOT be blocked. If a user expresses a political, scientific, religious, ethical, or philosophical concept that is controversial but legally expressible, the node MUST admit it, possibly after sanitizing unsafe attachments. The system is designed to model all meaningful ideas that humans hold; blocking is strictly limited to payloads that cannot legally be transmitted in any form.

### 9.3 illegal content discovered after canonical admission [anchor: illegal_content_discovered_after_canonical_admission]

Occasionally, content that passed initial safety checks MAY later be found to contain illegal payloads. The protocol requires that such discoveries be handled in a manner that (a) preserves the integrity of the historical record, (b) obeys legal requirements, (c) prevents censorship, and (d) maintains deterministic replay.

Illegal content discovered post-admission MUST NOT be deleted, erased, or silently mutated. Instead, the node MUST encapsulate the illegal payload into a non-reconstructable placeholder, replacing the raw content in all future materializations while preserving the original event hash, authorship, timestamp, and surrounding semantic structure. Encapsulation is a deterministic transformation that irreversibly removes illegal data while maintaining the continuity of the canonical universe. The encapsulated placeholder MUST state that it replaces content deemed illegal under the classifier ensemble and MUST reference a truth claim describing the reason for encapsulation.

Immediately upon encapsulation, a node MUST generate a content_sanitization event, recording: (a) the original event identifier; (b) the reason the content is now considered illegal; (c) the classifier ensemble outputs; (d) evidence supporting the determination; (e) a description of the semantic meaning of the removed content in legally compliant language; and (f) a link to challenge this determination. This ensures that encapsulation does not function as hidden censorship; instead, it produces a public chain-of-custody audit trail.

Semantic content MUST be preserved whenever possible. If the illegal payload expresses a meaningful idea - such as a claim, argument, narrative, or conceptual structure - the node or human reviewer SHOULD generate a new sanitized idea that expresses the underlying meaning without containing forbidden material. This sanitized idea MAY then be adopted into the canonical universe by the appropriate author or by a reviewer acting as "scribe" for determinism. The relationship between the illegal content and its sanitized successor MUST be recorded via deterministic connection types indicating that the new idea preserves the epistemic intent of the original while removing illegal elements.

Encapsulation MUST NOT alter past POD assignment, ranking outcomes, challenges, or governance actions. Historical replay MUST reconstruct the fact that the illegal payload existed and contributed to those processes, without revealing the payload itself. Nodes MUST continue to verify signatures and metadata of the original event but MUST materialize only the sanitized placeholder in user-facing surfaces.

Governance MUST provide a pathway for reviewing and updating the rule-sets that define illegal content categories. If new evidence shows that a block or encapsulation was incorrect, the node MUST support a reversal procedure in which (a) the wrongful encapsulation is recorded, (b) a sanctioned and safe representation of the original meaning is re-admitted, and (c) historical records are updated to reflect the correction without altering the immutable event log.

The overarching rule is: illegal payloads cannot remain accessible, but the ideas they represent and the epistemic role they played MUST remain visible, challengeable, and faithfully represented in the canonical universe. This prevents the system from becoming a censorship tool while maintaining global legal compliance and preserving replay integrity.

### 9.4 jurisdictional overlays and filtered replicas [anchor: jurisdictional_overlays_and_filtered_replicas]

The protocol defines a single, global notion of canonical admissibility based solely on the minimal safety rule-set described in §§10.1§8.3. This global rule-set permits the admission of all legally expressible ideas and prohibits only those payloads that are intrinsically illegal in nearly all jurisdictions (such as CSAM, involuntary doxxing, and operationally targetable instructions for imminent harm). Canonical admissibility MUST NOT vary by region, political regime, cultural norms, or local speech restrictions. The canonical universe is therefore a unified civilizational record of what humans attempted to say, limited only by global, irreducible safety constraints.

However, nodes participating in the network operate under local laws that MAY impose additional restrictions on what they can store or serve. To accommodate this without allowing local censorship to redefine canonical truth, the protocol distinguishes between canonical admissibility and node-local visibility or storage constraints. A node MAY refuse to store or serve certain canonical payloads when compelled by local law, but such refusals MUST NOT alter the canonical event log, MUST NOT redefine admissibility, and MUST NOT prevent other nodes from hosting or serving the full canonical universe.

A node that applies local-law constraints MUST identify itself as a filtered replica. A filtered replica MUST:

synchronize the full canonical event structure - timestamps, authorship, hashes, safety metadata, and challenge outcomes - even when it cannot store or display the full payloads;

materialize deterministic safe-language placeholders in place of locally prohibited payloads, preserving the semantic context without violating local law;

materialize legally compliant maximum-detail placeholders whenever a payload cannot be shown.
These placeholders MUST:
(a) preserve the fact that the canonical event exists;
(b) describe the semantic content category of the missing payload;
(c) cite the legal authority under which the payload is restricted;
(d) provide the maximum level of meaning legally permissible;
(e) remain deterministic across all nodes governed by the same legal constraints.
Placeholders MUST NOT be empty, purely structural, or misleading. They exist to prevent silent erasure and preserve epistemic continuity.

represent restricted content as structured absence, not deletion.
A structured absence MUST include:
(a) the canonical event identifier;
(b) the safe-language placeholder;
(c) the restriction category and legal citation;
(d) a deterministic placeholder hash.
Nodes MUST NOT imply that a restricted event does not exist or that it fails global admissibility criteria unless it actually does.

record and expose a jurisdictional-overlay manifest, specifying which categories of content it is not permitted to store or serve and the legal basis for those restrictions;

publish a transparency log of filtered content identifiers, including the mapping between canonical event IDs and the local-law constraints that prevent full display;

allow users to redirect or export their session state to unfiltered nodes in permissive jurisdictions or to offline seed packages, enabling access to the full canonical universe;

honor all challengeability mechanisms for safety decisions, even when unable to display the full contested content.
Nodes MUST provide all metadata necessary for users to pursue challenges, even when they cannot render the underlying payload.

Filtered replicas MUST NOT block canonical events from entering the global history. If a node cannot admit a payload due to local law, it MUST encapsulate or sanitize it locally while still propagating the canonical event metadata. Nodes MUST NOT lie about the existence of canonical events; they MAY only decline to store or show specific payloads and MUST provide deterministic placeholders and reasons for absence.

Nodes operating under the same jurisdiction MUST produce identical placeholder hashes and placeholder structures for the same restricted payloads, ensuring deterministic replay and preventing fingerprinting across replicas.

Full canonical replicas operating in permissive jurisdictions MUST store the complete admissible event history and MUST make it publicly available, ensuring that the system as a whole remains resistant to localized censorship or suppression. The protocol does not assume that all jurisdictions will allow full replicas to operate openly; some nodes MAY run underground or outside the enforcing regime. However, canonical validity MUST NOT depend on any particular jurisdiction’s laws, and the global minimal safety rule-set MUST remain the sole determinant of admissibility.

Jurisdictional overlays therefore provide a compatible way for the system to operate globally without sacrificing its universal epistemic mandate. Nodes MAY comply with local requirements on what they store or serve, but they MUST NOT redefine, conceal, or erase canonical events. The canonical universe remains a single, immutable, globally consistent record of what was said, while filtered replicas serve as legally compliant interfaces in constrained environments.


### 9.5 censorship-pressure metrics, anomaly detection, and network-wide alerts [anchor: censorship_pressure_metrics_anomaly_detection_and_network_wide_alerts]

To ensure that safety mechanisms and jurisdictional overlays cannot be quietly repurposed into censorship systems, the protocol requires every node to measure, record, and publicly expose **censorship-pressure metrics**. These metrics quantify how often safety filters, local-law overlays, and classifier ensembles prevent the full materialization of canonical events. By making censorship pressures globally transparent, the network ensures that nodes, users, and governance participants can detect when political, institutional, or jurisdictional forces begin distorting the epistemic landscape.

Each node MUST maintain a **censorship-pressure profile**, updated continuously and published at snapshot boundaries. At minimum, this profile MUST include:

1. **blocked submission frequency:**  
   The number of attempted canonical events that resulted in blocked_submission records during a defined interval, categorized by payload class, classifier ensemble outputs, and disagreement profiles. Nodes MUST report both absolute counts and normalized rates relative to their traffic.

2. **sanitization and encapsulation frequency:**  
   The number of canonical events for which illegal or unsafe content was later detected and encapsulated, including the categories of violation and the transformation applied. High rates of post-hoc encapsulation MAY indicate classifier drift, jurisdictional manipulation, or adversarial attempts to poison the network.

3. **jurisdictional filter impact metrics:**  
   Nodes operating under local-law overlays MUST publish which canonical events they cannot fully materialize, including counts per legal category (e.g., banned political symbol, prohibited speech, protected content) and the fraction of canonical events affected. These metrics allow downstream clients to detect when jurisdictions create significant epistemic blind spots.

4. **presentation-layer suppression rates:**  
   The number of times a node’s UI layer refuses to display content for emotional-load reasons (e.g., extreme violence) or user preferences. These MUST be distinguished from legally imposed filters and safety-sanctioned encapsulations. Presentation-layer filters are optional, but their use MUST remain visible.

5. **classifier-deny ratio and ensemble tension:**  
   The distribution of classifier votes across all processed submissions, measuring how often classifiers disagree. Sudden shifts or persistent patterns of elevated disagreement MAY indicate tampering, political pressure, or data poisoning. Nodes MUST treat ensemble tension as a global signal requiring attention.

Nodes MUST summarize these metrics into a deterministic **censorship-pressure vector** published at each snapshot. The vector MUST include a node’s normalized block rate, sanitization rate, jurisdictional overlay impact, and classifier tension index. This vector becomes part of the canonical snapshot metadata, enabling the network to evaluate censorship pressures longitudinally across time, jurisdictions, and political regimes.

When censorship-pressure vectors exceed governance-defined thresholds, nodes MUST derive and expose a **censorship_alert** surface. These alerts do not change canonical content and are not standalone canonical events; they function as signals to the global ecosystem that certain jurisdictions or nodes are experiencing unusually high suppression or distortion pressures. Alerts MUST include: (a) a breakdown of contributing factors, (b) the timeframe of escalation, and (c) links to affected `blocked_submission` records or encapsulation records.

Users and clients SHOULD treat `censorship_alert` surfaces as navigational cues. High-pressure regions MAY indicate environments where only filtered replicas can operate legally, where authoritarian policies attempt to suppress dissent, or where classifier ensembles may be compromised. The protocol imposes no punitive action on filtered replicas; instead, it ensures that the entire network becomes aware of the distortion and can route around it by increasing replication from low-pressure nodes or offline seed archives.

The censorship-pressure system ensures that any attempt to use safety, legality, or infrastructure constraints as disguised censorship becomes **visible at the network layer**. It does not prevent jurisdictions from imposing local restrictions, but it prevents such restrictions from becoming invisible, normalized, or globally binding. The canonical universe remains unified, durable, and censorship-resistant, while the network continuously monitors and surfaces patterns that threaten that universality.

### 9.6 emotional load, sensitive content, and user-controlled visibility [anchor: emotional_load_sensitive_content_and_user_controlled_visibility]

Safety constraints address only legality; emotional-load constraints address the psychological impact of content. Emotional-load filtering is a **presentation-layer mechanism**, not a canonical constraint. A node MAY classify certain canonical payloads as emotionally intense - such as depictions of violence, traumatic narratives, graphic descriptions of harm, or catastrophic predictions - but these classifications MUST NOT affect admissibility, ranking, challengeability, POD eligibility, or any part of the canonical universe. Emotional-load metadata is strictly advisory and MUST remain deterministic and replayable.

Nodes MAY maintain emotional-load classifiers - distinct from safety classifiers - to evaluate whether content SHOULD be blurred, gated, or previewed with warnings. These classifiers operate entirely in the visibility layer and MUST NOT block submissions or trigger encapsulation. When a node determines that content carries high emotional intensity, it MAY present it with a warning screen, reduced-resolution placeholder, or blurred preview. The user MUST have the ability to override such filters, and the override MUST NOT affect replay or canonical state. User preference settings MAY include automatic gating of traumatic or disturbing content, but these preferences MUST be local to the user interface and MUST NOT alter underlying event structures.

Emotional-load categories MUST be distinguished from safety categories in metadata and classifier outputs. Nodes MUST expose emotional-load classifications as separate machine-readable metadata fields attached to canonical events. This ensures that emotional-load decisions are interpretable, criticizable, and challengeable as presentation-layer choices rather than epistemic barriers. Nodes MUST also record classifier-version metadata so that emotional-load designations remain transparent across time.

The system MUST treat emotional-load filtering as a reversible aesthetic modification. Any decision by a node to blur or warn MUST be invertible by the user and MUST NOT alter the canonical payload, its hash, or any part of the event log. Implementations MAY introduce structured emotional-load maps in the UI to help users navigate psychologically intense regions of the graph, but these maps MUST be deterministic summaries of metadata, not editorial decisions about what ideas SHOULD be seen. The principle governing emotional-load filtering is that **psychological safety in the interface must never become epistemic suppression in the protocol**.

In addition to safety-based abstraction or redaction, default visibility may be affected by derived structural exclusions, including lifecycle_state (e.g., rot or burn) and taint-related exclusions.

When content is excluded from default views due to derived structural rules, users MUST be provided with an explicit explanation path indicating:
- that the exclusion is structural rather than safety-based,
- the category of exclusion (e.g., rotted, burned, tainted),
- and how the content may be inspected, challenged, or restored.

User-controlled visibility settings MUST NOT override canonical safety redaction, but MAY override default structural exclusions when viewing raw structural or diagnostic views.


### 9.7 age-gating, vulnerable populations, and safe defaults [anchor: age_gating_vulnerable_populations_and_safe_defaults]

The protocol supports participation by minors and vulnerable populations through UI-layer protections that do not alter canonical admissibility. Nodes MAY implement age-gating, parental controls, or restricted visibility for content that jurisdictions or guardians deem unsuitable for minors; however, these mechanisms MUST operate strictly at the presentation layer. Age-based restrictions MUST NOT alter the canonical universe, MUST NOT prevent ideas from entering the event log, and MUST NOT influence challenge outcomes or POD distribution.

Age-gating MAY rely on optional age declarations or third-party verification processes, but such information MUST NOT become part of the canonical identity record. Instead, age-gating operates on a node-local profile layer or client-controlled preference set. When age gating is active, nodes MAY blur, warn, or hide content deemed inappropriate for minors, including graphic violence, sexual material, self-harm content, or content requiring emotional maturity. But nodes MUST still synchronize the full canonical event structure beneath these visibility filters, and the canonical payload MUST remain available to full-access clients and offline seed packages.

Nodes MUST clearly differentiate age-based UI restrictions from safety-based payload restrictions. An idea that is age-gated for minors MUST still be admissible, challengeable, and representable within the canonical universe. Age-gating MUST be reversible and user-selectable; adults MUST be able to view all canonical content except globally illegal payloads encapsulated under §§10.1§8.3. Minors who become adults MUST be able to remove age-gating without altering any canonical metadata.

Protection of vulnerable populations MUST also occur strictly at the UI level. For example, clients MAY offer guided modes for users with mental-health concerns, such as softening the presentation of self-harm discussions or providing auto-links to support resources. These interventions MUST NOT restrict the canonical representation of the ideas themselves. The protocol ensures that **vulnerability support functions as optional interface scaffolding, never as a modification of epistemic structure**.


### 9.8 high-risk idea domains and harm minimization without censorship [anchor: high_risk_idea_domains_and_harm_minimization_without_censorship]

Certain idea domains—self-harm, suicide, violent extremism, hate ideologies, biological threats, weapons design, and catastrophic forecasting—carry elevated risk of misuse or psychological harm. The protocol allows full representation of these ideas so long as their payloads remain legal, but imposes structured requirements on how nodes handle them in the visibility layer. Harm minimization MUST NOT become censorship; instead, it MUST operate through **safe-language transformation**, **advisory UI scaffolding**, and **contextual metadata**.

High-risk content MUST be admitted into the canonical universe whenever it can be expressed in permissibly sanitized form. Nodes MUST apply deterministic transformation rules that convert unsafe or overly explicit instructions into safe-language summaries (e.g., replacing detailed weapon construction steps with a generalized description: “The user attempted to describe a method for constructing a weapon; details removed”). These summaries MUST preserve as much semantic meaning as is legal and safe, and MUST remain challengeable. If users believe a transformation misrepresented their intent, they MAY propose alternative formulations through canonical events.

Node interfaces MAY present high-risk ideas with contextual affordances—such as highlighting evidence requirements, linking to counterarguments, or surfacing prior challenges—but MAY NOT demote or algorithmically hide them. High-risk labels MUST be lexically neutral descriptors (“self-harm domain,” “weapons domain,” “extremist ideology domain”) and MUST NOT express editorial judgment. These labels MUST be attached as machine-readable metadata fields, enabling transparent analysis of how risky ideas propagate through the graph.

Nodes MUST also ensure that high-risk ideas do not automatically produce targeted harm. For example, self-harm ideation MUST NOT produce direct encouragement mechanisms, and violent-extremist content MUST NOT be rendered in a way that appears to endorse or recruit. Instead, nodes MAY provide contextual UI warnings and must preserve challengeability. Harm minimization therefore ensures that **dangerous content remains part of the global epistemic record while never functioning as an instruction manual or call to immediate violence**.


### 9.9 redaction, transformation, and semantic preservation guarantees [anchor: redaction_transformation_and_semantic_preservation_guarantees]

When a payload requires alteration to meet global safety constraints (§10.1§8.3), local-law constraints (§10.4), or harm-minimization rules (§10.8), nodes MUST apply deterministic redaction and transformation procedures that preserve semantic meaning to the greatest legally permissible extent. Redaction MUST NOT remove an idea; it MUST replace unsafe elements with placeholders or summaries while retaining the epistemic structure of the event. All transformations MUST be recorded as canonical metadata and MUST remain visible during replay.

Redactions fall into three categories:

1. **literal redaction:**  
   Removal of specific data (e.g., personal addresses, identifying images). The node MUST replace removed segments with standardized placeholders that describe the type of content removed.

2. **semantic transformation:**  
   Replacement of unsafe payloads with safe-language summaries that convey the same meaning without illegal content. For example, explicit depictions of violence MAY be summarized descriptively; detailed instructions for harmful actions MUST be abstracted to non-operational form.

3. **encapsulation (§10.3):**  
   A last-resort irreversible removal for illegally admitted payloads, preserving only a placeholder and explanatory metadata.

All redactions and transformations MUST be deterministic given the input payload, rule-set version, and classifier ensemble outputs. They MUST produce identical results on all conformant nodes. Each transformation MUST generate a **transformation_record event** linked to the original event, containing: (a) the transformation category, (b) the removed or abstracted content type, (c) the semantic summary, (d) the rule-set and classifier versions, and (e) the rationale for transformation.

Semantic meaning MUST be preserved unless preservation is impossible. If users believe that a redaction materially misrepresents their intended meaning, they MAY issue a new canonical description or challenge requesting correction. Nodes MUST allow multiple safe-language formulations to coexist, provided each is legally and epistemically valid. Under no circumstance may a node delete the original event or conceal the fact that redaction occurred. The system MUST maintain the principle that **ideas may be transformed for safety, but never erased from the civilizational record**.

### 9.10 adversarial attacks on safety, visibility, and classifier ensembles [anchor: adversarial_attacks_on_safety_visibility_and_classifier_ensembles]

The safety and visibility layers are potential attack surfaces for actors attempting to distort, suppress, or manipulate the epistemic record. Nodes MUST therefore detect, withstand, and expose adversarial behaviors designed to exploit classifier ensembles, emotional-load filters, jurisdictional overlays, or user-report systems. The purpose of this subsection is not to eliminate all adversarial attempts - many will occur - but to ensure no such attempt can silently alter canonical state or bias the system's epistemic structure.

Adversarial attacks fall into several categories:

1. **payload poisoning:**  
   Submitting borderline-legal content intended to trigger safety disagreements, overwhelm classifiers, or induce conservative overblocking. Nodes MUST detect anomalous clusters of near-threshold payloads and surface them in censorship-pressure metrics (§10.5).

2. **classifier gaming:**  
   Attempts to craft inputs that cause misclassification by specific models. Because nodes use a diverse classifier ensemble with transparent outputs, attackers cannot target a single classifier. Ensemble disagreement MUST trigger human-visible review, preventing adversarial crafting from producing silent blocks.

3. **visibility brigading:**  
   Coordinated efforts to mass-report emotionally intense content in order to suppress its visibility. Since emotional-load filtering is deterministic and node-controlled - not socially moderated - brigading MUST NOT influence canonical visibility. Nodes MAY log brigading attempts in anomaly metrics (§10.5).

4. **rule-set manipulation attempts:**  
   Actors attempting to revise safety rule-sets to suppress disfavored ideas under the guise of legality or harm minimization. Since rule-set changes require canonical adoption actions (§8.3), such manipulation becomes fully visible and challengeable.

5. **jurisdictional exploitation:**  
   Malicious actors seeking to route content through jurisdictions with strict laws to force filtered replicas to hide it. Filtered replicas MUST disclose constraints (§10.4), preventing such attacks from globally suppressing content.

No adversarial behavior MAY affect canonical admissibility unless the payload is genuinely illegal under global minimal safety constraints. Nodes MUST treat all adversarial attempts as visibility-layer phenomena that do not influence canonical rankings, challenges, POD flows, or idea structures.

All nodes MUST expose adversarial-detection metadata, including classifier anomaly spikes, suspicious submission bursts, and repeated edge-case triggering attempts. These signals MUST feed into censorship alerts (§10.5) and MAY trigger governance review of classifiers or rule-sets. The overarching guarantee is that **adversarial pressure cannot become invisible pressure**, and cannot silently compromise the epistemic integrity of the system.


### 9.11 safety governance, classifier updates, and drift mitigation [anchor: safety_governance_classifier_updates_and_drift_mitigation]

Safety mechanisms rely on classifier ensembles and transformation rules that evolve over time. To ensure these changes do not introduce censorship, misclassification, or inconsistency in replay, all updates MUST follow deterministic governance procedures and MUST be recorded as canonical events.

Each classifier in the ensemble MUST publish the following metadata into the canonical universe whenever it changes:

- model version and architecture summary;  
- training-data provenance summary (high-level, non-sensitive);  
- known failure modes;  
- category definitions and mapping tables;  
- rule-set version used for thresholding.

A classifier update constitutes a **governance change** and MUST be proposed through an actionable idea, deliberated, and adopted by a governance adoption action (§8.3). No node MAY unilaterally change classifier configurations without recording a canonical change event. Nodes MUST reject safety decisions that rely on unrecorded classifier versions, ensuring deterministic replay.

Classifier drift - in which models become more conservative or permissive over time - MUST be detectable through comparison of ensemble outputs across snapshots (§10.5). Nodes MUST publish drift metrics indicating whether their classifiers have become systematically more likely to block, sanitize, or encapsulate content. Sudden drift spikes MUST trigger governance review, and MAY lead to rollback or rebalancing of the ensemble.

If a classifier or transformation-rule update is later judged harmful or incorrect, nodes MUST support reversal procedures that restore previous configurations without rewriting past events. Incorrect safety decisions MAY be corrected through wrongful-block or wrongful-encapsulation challenges (§§10.2§8.3), but the historical record MUST preserve which classifier versions produced the original decisions.

In this governance model, safety evolves transparently and collectively, not opaquely or unilaterally. The system guarantees that **classification changes are recorded as ideas, debated as ideas, adopted as actions, and reversible through challenges**, preserving epistemic neutrality while allowing adaptation to new legal realities and technological threats.


### 9.12 safety in offline replicas and mindseeds [anchor: safety_in_offline_replicas_and_mindseeds]

Offline replicas and mindseeds allow individuals and groups to preserve the canonical universe independent of network access, enabling operation under natural disasters or network fragmentation. However, offline replicas face unique safety challenges: they may contain content later deemed illegal, they may operate without updated rule-sets, and they may need to merge large histories back into the canonical universe once connectivity is restored.

Offline replicas MUST store all content that was legal and admissible at the time of snapshot creation. Nodes operating offline MUST NOT delete or mutate historical payloads, even if later rule-sets would require sanitization. Instead, during offline  -> online merge, the receiving node MUST apply deterministic sanitization, encapsulation, or transformation rules (§§10.1-8.3, 8.9) to bring the offline content into compliance with current global safety rules.

When merging mindseeds or offline histories into the canonical universe, nodes MUST:

1. apply safety classifiers using the current ensemble version;  
2. encapsulate illegal content while preserving semantic placeholders;  
3. regenerate safe-language summaries for unsafe payloads;  
4. record a **merge_sanitization event** describing all transformations applied;  
5. preserve the semantic intent and chronological position of the offline events.

Offline replicas MAY lack updated safety classifiers or transformation rules. Therefore, nodes MUST treat all offline payloads as requiring safety reevaluation upon reconnection. This ensures deterministic global behavior and prevents attackers from bypassing safety systems by injecting unsafe content offline.

Mindseeds intended for long-term personal archiving MAY retain content that nodes cannot legally serve. When adopting such content into the network, a verified human MUST explicitly resubmit it in safe-language form, allowing the canonical universe to record the idea without transmitting unsafe payloads. This separates private, personal recordkeeping from global public publishing.

Safety in offline contexts MUST always preserve meaning while restoring legal compliance. The system treats offline replicas as **time capsules** that may contain content inconsistent with future norms or laws; merging them requires careful, deterministic sanitization rather than censorship or erasure. The protocol guarantees that offline autonomy does not compromise global validity and that global safety does not erase personal historical meaning. 

### 9.13 Explainability for derived exclusions (normative) [anchor: explainability_for_derived_exclusions_normative]

Any exclusion of canonical objects from default visibility or participation due to derived rules MUST be explainable.

#### 9.13.1 Required explanation fields [anchor: required_explanation_fields]

For each derived exclusion (including rot, burn, or taint), the system MUST expose:
- the exclusion category (e.g., rotted, burned, tainted),
- the rule or rulebook that produced the exclusion,
- the relevant derived inputs (e.g., inactivity window, importance threshold),
- the cycle at which the exclusion took effect,
- the conditions required for restoration or inclusion.

#### 9.13.2 Precedence ordering [anchor: precedence_ordering]

When multiple exclusion mechanisms apply, precedence MUST be as follows:
1. **Safety redaction or abstraction**
2. **Legal or jurisdictional abstraction**
3. **Derived structural exclusions** (rot, burn, taint)
4. **User-controlled visibility filters**

Higher-precedence exclusions MAY mask lower-precedence explanations, but the existence of masked exclusions MUST still be indicated.

#### 9.13.3 Determinism and auditability [anchor: determinism_and_auditability_2]

Explainability artifacts MUST be:
- deterministically derivable from the canonical event log and rulebooks,
- consistent across nodes,
- accessible without privileged access.

Explainability is a core protocol requirement and MUST NOT be treated as a UI-only feature.


## 10. governance and rulebooks [anchor: 10_governance_and_rulebooks]

### 10.1 governance rulebooks: definition, roles, and canonical structure [anchor: governance_rulebooks_definition_roles_and_canonical_structure]

Governance rulebooks are ideas that define how the canonical universe MAY evolve, what constraints apply to identities and events, and how deliberative authority is exercised. A governance rulebook is not an external document; it is a first-class idea with structured descriptions, relevance edges, and challengeability. Rulebooks do not replace the protocol. Instead, they form the top layer of modifiable rules that operate *within* the protocol’s constitutional boundaries and MUST remain compatible with all invariants defined in Section 0.

A governance rulebook consists of a set of normative statements that specify:

* which ideas or events require structured deliberation,  
* the thresholds or procedures used to approve governance changes,  
* which event types are enabled or disabled for the next epoch,  
* which safety or identity rules MUST be applied, and  
* how rule supersession and lineage SHOULD be interpreted during replay.

Rulebooks are permanent artifacts in the event log. They do not "replace" previous rulebooks but exist in a versioned lineage. A later rulebook MAY supersede earlier rules for future events, but MUST NOT retroactively change interpretations of events that occurred before its activation cycle boundary. Each rulebook is internally immutable after creation; governance changes occur only by introducing successor rulebooks, not by editing existing ones.

The canonical universe treats governance rulebooks as semantic structures whose meaning is determined solely by deterministic replay. Implementations MUST NOT embed extra-protocol configuration files, environment variables, or local policy flags that affect governance semantics. All governance behavior MUST be traceable to ideas and events visible on-chain and reconstructable by any conformant node.

A rulebook MAY use a reference idea such as a core-protocol idea and ordinary relative-importance connections to expose its most important rules. An authored Ordering/Vine MAY provide a narrative, chronological, or guided reading path, but it does not create a directory hierarchy and does not determine rule priority or authority. No Grove, Biome, folder, or branching-Vine object is required for protocol semantics.

### 10.2 governance ideas, proposals, and deliberation surfaces [anchor: governance_ideas_proposals_and_deliberation_surfaces]

Governance proposals are ideas that attempt to modify governance behavior by introducing a new rulebook or amending an existing rulebook via structured supersession. A governance proposal MUST clearly identify the portion of governance it seeks to modify and MUST express its intended effect in formal, canonical descriptions. Proposals MAY include explanatory, philosophical, or motivational content in non-canonical tiers, but the normative change MUST be encoded in the canonical description to ensure deterministic replay.

A governance proposal becomes actionable only when a governance challenge is opened against it. The challenge lifecycle for governance proposals follows the general challenge model defined in Section 7 but with higher-ceremony requirements, stricter argument rules, and extended deliberation windows. Governance proposals typically include:

* a proposed successor rulebook or successor clause,  
* a mapping from old rules to new rules,  
* specified activation conditions at a scheduled activation cycle boundary, and  
* justification for why the change is compatible with Section 0 invariants.

During the challenge’s argument and evidence phases, participants MAY submit supporting or opposing claims, including historical analysis of prior rulebooks, expert evaluations, and compatibility checks with constitutional invariants. Once voting completes, a successful challenge verdict makes the proposal eligible for activation. Activation additionally requires the canonical implementation-completion claim and supporting evidence required by the active rulebook, followed by the scheduled activation cycle boundary. A verdict without valid completion evidence MUST NOT activate a rule change.

Governance proposals that are defeated remain permanently in the event log as inert ideas. Their content MAY still influence future governance debates but they SHALL NOT alter canonical behavior.

### 10.3 activation at cycle boundaries and forward-only rule changes [anchor: activation_at_snapshot_boundaries_and_forward_only_rule_changes]

Governance rule changes MUST activate only at cycle boundaries. A governance challenge that concludes successfully produces a verdict event that records `decision_cycle_index`, `change_class`, and `delay_policy_version`, then deterministically computes `activation_cycle_index`. Until that cycle boundary is reached, all events MUST be interpreted under the governance rulebook active for the current cycle.

This mechanism ensures strict forward-only rule evolution and preserves deterministic replay. No governance change MAY apply retroactively; no event MAY be reinterpreted under a rule that did not exist when the event originally appeared in the total order. If a rule change would cause reinterpretation of past events, that rule change is invalid under deterministic replay and MUST NOT activate.

When a new rulebook activates, implementations MUST load the new governance rules into the replay engine and apply them to all subsequent events. Nodes that join the network late or replay from genesis MUST reach identical interpretations of which rulebooks were active at which boundaries, based solely on governance verdicts, qualifying implementation-completion claims and evidence, and deterministic cycle-boundary derivation. Implementations MUST NOT rely on local caches or heuristics to determine active governance rules; all such determinations MUST be derivable from the event log.

The activation boundary also ensures safe evolution of the protocol ecosystem. Nodes MAY continue operating under older rulebook versions until they process the cycle boundary that introduces the successor rulebook. This allows nodes to maintain historical compatibility while progressively upgrading their interpretation of future events without semantic drift or ambiguity.

All governance rule changes, parameter updates, and protocol configuration modifications MUST activate only at defined cycle boundaries.

Activation boundaries are keyed to **cycle index**, not to block height or wall-clock time. A rule change adopted through governance takes effect at the start of the computed `activation_cycle_index`.

Rule changes MUST be forward-only:
- no retroactive alteration of past events or derived outcomes is permitted,
- all prior canonical history remains valid and inspectable,
- deterministic replay with historical rulebooks MUST reproduce historical state.

This guarantees that all nodes agree on which rulebook version applies at any point in the event log.


### 10.4 rulebook categories: protocol, governance, safety, token, identity [anchor: rulebook_categories_protocol_governance_safety_token_identity]

Governance rulebooks are partitioned into functional categories. Each category specifies rules for a distinct semantic domain. Categories exist to ensure modularity, predictable evolution, and clear constraints during replay. A rulebook MAY belong to multiple categories when its scope crosses domains, but its canonical interpretation MUST remain unambiguous.

The primary categories are:

* **protocol rulebooks**  -  govern deterministic replay, event validity, state transition semantics, node conformance tests, and activation conditions for rule supersession. These rules MUST remain compatible with all constitutional invariants and MUST NOT redefine core ontology (ideas, descriptions, connections, challenges, actions).

* **governance rulebooks**  -  define the procedures for proposing, deliberating, voting on, and activating changes to rulebooks themselves. This includes quorum definitions, eligible voter sets, challenge window durations, dispute resolution processes, and rule-supersession semantics.

* **safety rulebooks**  -  define the classification of payloads, safe specificity boundaries, visibility overlays, regional overlays, classifier behavior, and blocked-submission handling. Safety rulebooks MAY refine or extend classifications, but MUST NOT contradict the constitutional constraints on meaning preservation, non-erasure, or the universal challengeability of decisions.

* **token rulebooks**  -  specify POD mechanics, POINT emission schedules, constraints on externalization, and POD/POINT relations to actions and significance attribution. Token rulebooks MUST remain forward-only and MUST NOT permit retroactive reassignment of POD or alteration of historical token flows.

* **identity rulebooks**  -  govern identity creation, longevity, key rotation, delegation limits, death semantics, and rules concerning successor identities, tribes, or groups. Identity rulebooks MUST remain compatible with the human-first authorship invariant and MUST NOT grant canonical authority to non-human agents.

Each category evolves independently via successor rulebook ideas, but interactions between categories MUST remain deterministic and MUST NOT introduce cross-category ambiguities. When two rulebooks appear to conflict, the canonical resolution is determined by constitutional invariants and deterministic replay rules, not by local interpretation or node-specific policy.

### 10.5 authority limits: what governance may and may not change [anchor: authority_limits_what_governance_may_and_may_not_change]

Governance MAY modify operational rules within the boundaries established by the protocol, but it MUST NOT alter constitutional invariants or any element required for interpretability, human primacy, or deterministic replay. Governance proposals that violate these constraints are invalid under deterministic replay and SHALL NOT activate, regardless of challenge outcomes.

Governance MAY change:

* voting thresholds, quorum definitions, and participation requirements,  
* the eligibility pools used for specific governance decisions (where permitted by Section 0),  
* activation timings for new rulebooks,  
* rule-specific activation logic that applies at future cycle boundaries,  
* domain-specific procedures for low-level technical adjustments,  
* rule-specific activation logic that applies at future cycle boundaries,  
* token emission parameters affecting future POINT generation, provided they apply prospectively and never retroactively,  
* procedures for identity verification, key rotation, delegation, or activity requirements,  
* branch-specific regional rulebooks as long as universal invariants remain intact,  
* tribe-internal decision processes that do not affect universal scope.

Governance MUST NOT change:

* the ontology of ideas, descriptions, connections, challenges, or actions,  
* the append-only nature of the event log or the immutability of events,  
* the human-first authorship rule or any attempt to grant canonical vote power to non-human agents,  
* the ability of any identity to run a conformant node and reconstruct the universe,  
* the forward-only activation model for rulebooks and cycle boundaries,  
* the semantic distinction between truth, importance, and action domains,  
* the forward-only activation model for rulebooks and cycle boundaries,  
* POD non-transferability or any attempt to retroactively modify past token flows.

Within those boundaries, the following additional limits apply to governance configuration:

1. **No weighted voting.**  
   Within any eligibility pool, each verified identity MUST receive exactly one vote of equal weight. Governance SHALL NOT introduce stake-weighted, POD-weighted, token-weighted, seniority-weighted, or contribution-weighted voting under any circumstances.

2. **No permanent or unchallengeable pools.**  
   Governance MAY define eligibility pools for specific decision types, but these pools MUST themselves remain challengeable. No governance rulebook MAY create a permanently closed authority group that can modify its own eligibility without oversight from a broader or universal voting body.

3. **Constitutional domains require universal eligibility.**  
   Any governance action that attempts to modify Section 0 invariants, human primacy, immutability of history, canonical ontology, or universal admissibility of legally expressible ideas MUST be voted on by all verified human identities without restriction. Governance MAY NOT reduce the voter pool for these domains.

4. **No canonical power for non-human agents.**  
   Governance MUST NOT assign voting rights, veto power, rule-activation authority, or challenge participation to non-human agents, including AI systems, algorithmic committees, automated services, or oracle providers.

5. **No retroactive rule application or reinterpretation.**  
   Governance MAY specify future rule changes but SHALL NOT apply modifications to any event that occurred before the activation cycle boundary of the new rulebook. Rule changes are strictly forward-only.

6. **No off-chain or opaque authority.**  
   Governance SHALL NOT externalize or privatize canonical decision-making. All rule semantics MUST be encoded in ideas, events, and snapshots visible to every node and reconstructable via deterministic replay. Any governance proposal attempting to delegate canonical authority to private servers, proprietary services, or opaque algorithms SHALL be invalid.

7. **No retroactive modification of POD or identity claims.**  
   Governance MAY specify future token mechanics and identity procedures but SHALL NOT reassign POD, erase POD history, or retroactively alter past token flows or identity events.

These limits ensure that governance remains adaptive while preventing authoritarian drift, plutocratic capture, or irreversible centralization. Governance may explore different eligibility patterns, expert-influenced workflows, layered decision mechanisms, and experimental democratic procedures—but all such experimentation MUST ultimately preserve equal-weight voting within pools, universal challengeability, and the perpetual ability of the broader population to reclaim authority over system-wide decisions.

### 10.6 quorum, thresholds, and voting mechanics [anchor: quorum_thresholds_and_voting_mechanics]

Quorum and voting thresholds are defined by governance rulebooks and specify the procedural requirements for ratifying governance decisions. Voting in governance challenges is always **equal-weight**: each eligible identity receives exactly one vote. Votes are never weighted by POD, POINT, wealth, seniority, or any other metric. No form of stake-based, token-based, or contribution-based weighting SHALL EVER be used in canonical governance decisions.

Eligibility for a given governance challenge is determined by the governance rulebook active when the challenge opens. Rulebooks MAY define different eligibility pools for different decision domains (for example, all verified identities for constitutional changes, a subset of node operators for low-level protocol tuning, or members of a specific tribe for tribe-internal decisions), but such eligibility rules themselves remain subject to challenge and future change. No eligibility rule MAY create a permanent, unchallengeable authority over the system.

A quorum rule defines:

* the minimum number or proportion of eligible identities who MUST participate for the vote to be valid,  
* the eligibility criteria for voters, as specified in the active governance rulebook,  
* how abstentions, non-responses, or temporarily offline identities are counted.

A threshold rule defines:

* the percentage of “yes” votes required for approval among those who participate,  
* whether a simple majority or a supermajority is required for this decision type,  
* deterministic tie-handling procedures,  
* any conditional threshold adjustments stated explicitly in canonical rulebook descriptions.

Governance challenges MUST record which eligibility definition, quorum rule, and threshold rule apply at the moment the challenge opens. These rules are fixed for the entire lifecycle of the challenge and SHALL NOT change even if new governance rulebooks activate before the challenge concludes.

If a governance challenge fails to meet quorum, or fails to meet its threshold, the actionable idea it concerns remains inert. If quorum and threshold are satisfied, the actionable idea enters an implementation phase, in which human identities MUST carry out the agreed-upon governance action and submit completion truth claims to attest that implementation occurred.

Voting semantics MUST remain deterministic under replay. Implementations MUST NOT use local configuration, external services, or node-specific heuristics to reinterpret votes, voter eligibility, or quorum conditions. All determination of eligibility, quorum satisfaction, and threshold outcomes MUST be reconstructable solely from canonical events and rulebooks active at the time the challenge opened.

### 10.7 governance challenges and their execution lifecycle [anchor: governance_challenges_and_their_execution_lifecycle]

Governance challenges follow the unified challenge framework described in Section 7, but they introduce additional ceremony, stricter eligibility rules, and expanded verification requirements. A governance challenge is an action challenge opened against an actionable idea that proposes to modify a governance, protocol, safety, token, or identity rulebook.

A governance challenge SHALL include:

* a reference to the actionable idea proposing the rule change,  
* the specific rulebook(s) it seeks to supersede or extend,  
* the eligibility pool, quorum rule, and threshold rule active at challenge opening,  
* the intended activation cycle boundary for the resulting rulebook, if approved,  
* any predecessor or parallel governance challenges it supersedes or invalidates.

Governance challenges proceed through the same six-phase lifecycle as other action challenges—creation, argumentation, voter selection, voting window, verdict aggregation, and state transformation—with the following additional properties:

1. **Extended argument phase.**  
   Governance rule changes have long-term effects. Governance rulebooks MAY require a minimum argument duration or minimum number of submitted perspectives before voting MAY begin.

2. **Mandatory compatibility checks.**  
   Any governance challenge MUST address compatibility with Section 0 invariants. If a proposal fails to provide such an analysis, the challenge MAY still be opened, but the absence of analysis itself is evidence against the proposal.

3. **Higher quorum or threshold constraints.**  
   Governance rulebooks MAY specify supermajority thresholds or higher quorum requirements for decisions affecting governance, protocol mechanics, or token rules.

4. **State transformation is deferred.**  
   Unlike ordinary governance actions, governance rule changes do not take effect immediately after verdict. They enter an implementation phase that MUST conclude before the scheduled activation cycle boundary. Only at that cycle boundary does the new rulebook become active.

Once a governance challenge reaches a verdict:

* If rejected, the actionable idea remains inert.  
* If approved, implementation MUST occur as defined in 11.7.1.  
  **This requirement applies to the system state, not to any specific identity: the protocol does not compel any individual to implement the rulebook.**  
  If no identity voluntarily implements the rulebook, the rulebook simply does not become active at its scheduled snapshot, and the prior rulebook remains in force.

#### 10.7.1 governance actions and completion truth claims [anchor: governance_actions_and_completion_truth_claims]

Because the canonical universe does not self-modify, approved governance actions MUST be implemented by human identities. Implementation includes updating reference node implementations, publishing configuration schemas, updating safety classifiers, or adjusting token mechanisms, depending on the approved rulebook.

Implementation SHALL be attested by a completion truth claim that includes:

* the identity performing the implementation,  
* references to the actionable idea and its successful challenge,  
* the target activation cycle boundary at which the new rulebook is scheduled to activate,  
* evidence demonstrating the implementation (e.g., code commits, configuration diffs, reproducible build artifacts).

Completion truth claims MAY themselves be challenged via truth challenges. If a completion claim is successfully falsified, implementation is considered incomplete and the rulebook SHALL NOT activate at its intended cycle boundary.

If no valid completion truth claim exists by the activation cycle boundary, the rulebook is considered unimplemented, and the prior rulebook remains in force.  
**No mechanism MAY assign, compel, or obligate any identity to perform this implementation. Only voluntary implementers can bring a rulebook into effect.**

---

### 10.8 governance lineage, rule supersession, and historical traceability [anchor: governance_lineage_rule_supersession_and_historical_traceability]

Each governance rulebook forms part of a versioned lineage. Successor rulebooks MUST include:

* a `supersedes` connection to the earlier rulebook,  
* inherited rules unchanged from earlier versions,  
* explicit replacement or removal of prior rules,  
* forward-only activation metadata tied to cycle boundaries.

* forward-only activation metadata tied to cycle boundaries.

Historical traceability requires:

1. **Permanent availability of the full rulebook text.**  
   All rulebooks MUST include canonical descriptions that remain visible forever.

2. **Recoverability of past governance semantics.**  
   Nodes MUST be able to determine exactly which rules applied to any event E by examining the snapshot immediately preceding E, the rulebook active at that snapshot, and any subsequent supersession metadata.

3. **No implicit rule inheritance.**  
   Only rule relationships explicitly represented in connections or structured fields SHALL be considered during replay. Unstated assumptions, external documents, or off-chain rules SHALL be ignored.

4. **No retroactive rule removal.**  
   Superseding a rulebook does not erase prior versions. It shadows them going forward but preserves them in the canonical record.

These invariants ensure that future identities can precisely reconstruct how governance behaved in any historical era, who proposed changes, who supported or opposed them, and the evidence that led to their adoption.

---

### 10.9 forks, invalid rulebooks, and deterministic resolution [anchor: forks_invalid_rulebooks_and_deterministic_resolution]

A governance proposal may produce a rulebook that is internally coherent yet invalid under constitutional invariants or deterministic replay constraints. Invalid rulebooks SHALL NOT activate, even if approved by governance voting. Nodes MUST detect invalid rulebooks during replay.

A rulebook is invalid if:

* it contradicts Section 0 invariants,  
* it grants canonical authority to non-human agents,  
* it enables weighted voting or irrevocable authority pools,  
* it attempts to retroactively alter or reinterpret past events,  
* it introduces nondeterministic behavior,  
* it references or relies on off-chain or unverifiable data,  
* it depends on implementation-specific heuristics.

If a rulebook is invalid, nodes SHALL:

1. treat the rulebook as present in the log but **never active**,  
2. continue applying the previous valid rulebook beyond the intended activation cycle boundary,  
3. record the attempted activation as a governance error detectable during replay.

If governance decisions lead to divergent valid rulebooks being implemented across nodes (a fork), deterministic replay SHALL resolve the fork using:

* the canonical event ordering,  
* activation cycle boundary metadata,  
* version lineage metadata.

Rulebooks that fail to meet activation criteria SHALL be disregarded, preventing permanent forks. If two competing rulebooks are both valid and approved, the earliest fully implemented successor (demonstrated by unchallenged completion truth claims) SHALL take precedence.

---

### 10.10 protocol self-description and invariants for interpretability [anchor: protocol_self_description_and_invariants_for_interpretability]

The protocol MUST remain self-describing. Any identity SHOULD be able to learn the entire governance structure, rulebook taxonomy, activation history, and lineage solely by inspecting the canonical universe.

Protocol self-description requires:

1. **Governance mechanisms exist as ideas.**  
   All rulebooks, eligibility definitions, quorum rules, threshold rules, and activation semantics MUST be represented as ideas with canonical descriptions.

2. **Rule semantics are never implicit.**  
   No rule MAY rely on off-chain documentation, private agreements, proprietary specifications, or software defaults. If behavior affects canonical interpretation, it MUST appear explicitly in a rulebook.

3. **Nodes MUST reproduce behavior from the event log alone.**  
   Conformance is defined by deterministic replay on the log and snapshots. Any behavior that cannot be derived from those sources SHALL be considered non-conformant.

4. **Universal inspectability.**  
   Any identity SHALL be able to inspect all rulebooks, supersession relationships, activation cycle boundaries, and governance challenges. Systems MUST NOT obscure canonical governance behavior behind UI filters or access-control layers.

5. **Forward-proof evolution.**  
   Governance MAY extend the protocol self-description system (for example, adding new rulebook categories or metadata fields), but such extensions MUST remain compatible with all Section 0 invariants and MUST NOT impede interpretability by future identities.

These invariants ensure that the protocol can evolve without losing clarity, reconstructability, or the ability of any participant to independently verify that the system is functioning correctly.

### 10.11 rulebook conformance in offline seedpacks and delayed reintegration [anchor: rulebook_conformance_in_offline_seedpacks_and_delayed_reintegration]

Offline Mindseeds allow identities and groups to continue reasoning with idea-compatible private records, conducting simulations, and drafting governance proposals while disconnected from the network. A separate publication package may contain exact signed canonical candidates. Private planning SHOULD mirror the standard idea, connection, Ordering, importance, challenge, claim, and action shapes where useful, but remains outside protocol conformance. The primary distinction is not the subject of offline reasoning, but which exact human-approved candidates are later submitted for canonical publication.

Because canonical governance depends on synchronized rulebooks and deterministic replay, additional constraints are required to ensure that offline work reintegrates cleanly with the online universe.

Offline replicas MUST use the rulebooks active at the moment of disconnection when validating signed candidates intended for delayed publication. Private journal deliberation MAY use those rules for accurate simulation but is not thereby validated, replayed, or made authoritative. Offline communities MAY continue to create idea-compatible private records and exact signed candidates, but neither becomes canonical until accepted into the shared canonical event log.

Governance proposals MAY be drafted offline, argued offline, and even voted on offline, but offline verdicts SHALL NOT be considered canonical. Offline voting outcomes represent local consensus or preference only. Only successful verdicts in the global canonical event log, paired with qualifying implementation-completion evidence and the scheduled cycle boundary, MAY activate future rulebooks or alter canonical governance behavior.

Tribe-scoped deliberation MAY occur offline in the same manner as online. Tribe membership rules and tribe-gated voting eligibility MAY be applied locally, but any tribe importance maps, challenge outcomes, or governance proposals that are later published become publicly visible canonical artifacts. Offline operation does not create private tribe maps in the canonical universe; it only delays publication.

Upon reconnection, a seedpack MUST perform the following steps:

1. **Synchronize upstream rulebooks.**  
   The seedpack MUST import all rulebooks activated during its offline period. If the seedpack conducted offline deliberations under rulebooks that have since been superseded, those deliberations remain historically meaningful to the participants but are non-canonical and have no direct authority in the canonical universe.

2. **Prepare and publish distinct canonical proposals or arguments.**
   Private journal material MAY be used to prepare exact human-approved canonical candidates with new canonical identifiers. Publication is not an in-place visibility change and does not disclose private identifiers or history by default. Offline votes and verdicts SHALL be treated as ordinary opinions or arguments, not as canonical decisions.

3. **Revalidate proposal compatibility.**  
   Once online, any offline governance proposal or rulebook draft MUST be evaluated against the currently active governance and protocol rulebooks. If the proposal contradicts newly activated rules or constitutional invariants, it MUST either be amended or MAY be challenged on compatibility grounds.

4. **No automatic adoption of offline results.**  
   Offline governance outcomes—including rule adoptions, eligibility pool changes, quorum decisions, importance rankings, or safety classifications—SHALL NOT be automatically imported or treated as authoritative. All such outcomes require full canonical challenges and voting cycles before they can affect canonical state.

5. **Snapshot alignment.**  
   A seedpack MAY NOT introduce a rulebook, governance change, or other rule-altering event that would activate at a cycle boundary earlier than the cycle boundary immediately following reintegration. All rule activations MUST align with globally recognized cycle boundaries.

Offline seedpacks MAY also apply jurisdictional or safety lenses when generating locally readable state, consistent with the safety and visibility rulebooks active at the time of disconnection. Such lenses affect what payloads are readable or withheld locally but MUST NOT erase the existence of ideas, connections, or challenges from the local record.

These constraints preserve the integrity of governance and deterministic replay while allowing offline communities—including those operating under restrictive or hostile conditions—to reason, deliberate, and prepare proposals using the full protocol structure. Offline operation therefore supports resilience and continuity of collective reasoning without introducing forks, hidden authority, or replay-incompatible semantics.



### 10.12 emergent governance structures: tribes, councils, and public committees [anchor: emergent_governance_structures_tribes_councils_and_public_committees]

The protocol does not prescribe specific political structures. Tribes, councils, advisory committees, and other governance formations MAY emerge organically through ordinary idea creation and connection patterns. Such structures have no intrinsic authority; their legitimacy arises solely from the willingness of identities to endorse their proposals and from the outcomes of canonical governance challenges.

An emergent governance structure MAY:

* draft and promote governance proposals,  
* perform analysis or simulation of rule effects,  
* coordinate large-scale deliberation,  
* maintain domain expertise (e.g., safety, tokens, identity, protocol mechanics),  
* propose changes to eligibility pools or quorum rules for their domain.

However, any emergent structure MUST obey the following constraints:

1. **No automatic or intrinsic authority.**  
   The structure itself has no privileged status in governance. Its recommendations are treated as ordinary arguments unless ratified through canonical voting.

2. **No implicit control of eligibility pools.**  
   A structure MAY propose eligibility rules but MAY NOT create or enforce pools outside canonical governance. Eligibility rules become active only after successful challenges, verdicts, implementation, and cycle-based activation.

3. **No ownership of rule domains.**  
   No structure MAY permanently claim authority over a domain such as safety, identity, or token mechanics. All domain assignments remain rulebook-defined and universally challengeable.

4. **No privileged access to protocol mechanisms.**  
   All governance tools—challenge creation, argument submission, rule supersession—remain equally available to all verified identities. Structures MAY coordinate collective reasoning but MUST NOT inhibit participation.

5. **Transparency requirement.**  
   Any structure that seeks to influence governance MUST express its proposals, arguments, and analyses as ideas in the canonical universe. Closed deliberation MAY occur socially, but canonical authority requires on-chain transparency.

Emergent governance structures serve as flexible scaffolding for distributed expertise and coordination. They MAY shape governance outcomes by producing high-quality proposals, but ultimate authority remains with the eligible voter pool defined by rulebooks and constrained by Section 0 invariants.

### 10.13 Readable state snapshots and deterministic history [anchor: readable_state_snapshots_and_deterministic_history]

This protocol distinguishes between readable state snapshots and deterministic replay history.

Readable state snapshots represent the current live state of the canonical universe at a given snapshot boundary, optimized for human comprehension and broad access rather than full historical reconstruction. A readable snapshot MAY include:

- the current active canonical descriptions of ideas,
- current importance rankings and ordering views,
- active connections between ideas,
- summaries of challenge outcomes and verdicts,
- references to active governance, safety, and token rulebooks.

Readable snapshots intentionally exclude inactive or superseded content by default. Ideas, descriptions, connections, or challenges that have become inert through deliberation, challenge outcomes, or supersession MAY be omitted from the readable snapshot, even though they remain preserved in deterministic history.

Deterministic replay history consists of the full ordered event log, together with any required snapshot and delta material, sufficient to reconstruct canonical state exactly from genesis. Deterministic history preserves every event, including inactive ideas, superseded descriptions, failed challenges, rejected proposals, and obsolete rulebooks. It is the authoritative source for verification, auditing, and long-term historical integrity.

Conformant nodes and clients are NOT required to store or distribute full deterministic replay history. The protocol requires only that deterministic history exists redundantly somewhere in the network such that canonical reconstruction remains possible. No requirement is placed on which nodes or identities must store this history, only that its existence is not lost to the network as a whole.

Readable state snapshots MAY be distributed independently of full deterministic history, provided they include cryptographic anchors sufficient to verify correspondence to a specific canonical snapshot boundary. Such anchors MAY include snapshot hashes, Merkle roots, or other deterministic commitments that allow a recipient to verify that the readable snapshot corresponds to a valid canonical state, even if the recipient does not possess the full history.

To support wide distribution, offline use, and resilience under constrained conditions, implementations MAY package readable snapshots and partial history into portable units (such as seedpacks or Mindseeds) at multiple scales. These scales MAY include, but are not limited to:

- a minimal readable core containing the most important canonical ideas and their immediate connections,
- an expanded readable state containing all currently active public ideas and rankings,
- readable state plus recent deltas sufficient to track near-term evolution,
- full deterministic replay history.

The protocol does not mandate a specific distribution scale. However, rulebooks MAY define default packaging profiles or importance-weighted inclusion strategies intended to maximize the redundancy and survivability of the most important ideas, descriptions, and verdicts across the network.

This separation between readable state and deterministic history allows the system to maximize the spread of epistemically important content—especially in offline, low-bandwidth, or adversarial environments—without imposing prohibitive storage or bandwidth requirements on all participants. At the same time, it preserves long-term verifiability, auditability, and the ability to reconstruct the full canonical universe wherever sufficient history is retained.

Readable state snapshots MAY be produced at multiple tiers to support inspection, onboarding, and survivability.

Snapshot tiers MUST be replay-equivalent and differ only in included payload depth and convenience data.

Readable snapshots MUST be distinguished from **cycle export packs**:
- snapshots are canonical resume artifacts keyed to block height,
- cycle export packs are non-authoritative, cycle-scoped summaries intended for browsing and inspection.

No readable snapshot or export pack may alter canonical meaning or replace deterministic replay from the event log.


## 11. snapshots and conformance [anchor: 11_snapshots_and_conformance]

### 11.1 purpose of snapshots and their role in deterministic replay [anchor: purpose_of_snapshots_and_their_role_in_deterministic_replay]

Snapshots serve as stable semantic boundaries in the canonical universe. A snapshot captures the complete, canonical state of all ideas, descriptions, connections, ranks, rulebooks, and governance configurations at a specific point in the event log. During deterministic replay, snapshots act as fixed anchor points that eliminate the need to recompute the entire universe from genesis for every validation step.

Snapshots SHALL provide:

* a complete and unambiguous summary of canonical state up to a specific event index,  
* the identity of the active rulebooks at that boundary,  
* the activation markers for rulebook supersession,  
* the full structure of ideas, descriptions, and connections as they existed at that moment,  
* a secure commitment enabling nodes to verify that their local reconstruction matches the canonical state.

Snapshots MUST NOT contain information that cannot be derived strictly from prior events. They are compressions of history, not alterations of it. No snapshot MAY alter the meaning of past events or introduce new canonical information. They MAY shadow earlier state but SHALL NOT contradict it.

Snapshots enable:

* efficient node bootstrapping and fast verification,  
* pruning of older intermediate states during replay,  
* deterministic activation of governance rule changes,  
* conformance checks between independent implementations,  
* trustless verification of replicated states across nodes.

### 11.2 snapshot structure: required fields and canonical form [anchor: snapshot_structure_required_fields_and_canonical_form]

A snapshot MUST have a canonical, implementation-independent structure. Nodes MUST be able to reconstruct the snapshot exactly from the event log. The snapshot format SHALL include, at minimum:

1. **snapshot index** — the event index at which the snapshot is taken.  
2. **snapshot hash** — a canonical commitment over the serialized snapshot contents.  
3. **active rulebooks** — references to the governance, protocol, safety, token, and identity rulebooks active immediately following this snapshot.  
4. **idea map** — the full set of ideas, descript

Each snapshot MUST include a marker indicating its snapshot tier and the required contents for that tier.

At minimum, every snapshot MUST contain:
- the block height it corresponds to,
- a cryptographic commitment to the canonical event log up to that block,
- the applicable rulebook versions,
- all derived state required to resume canonical operation.

Derived state included in snapshots MUST include, at minimum:
- lifecycle_state for all ideas and eligible connections,
- living-map eligibility flags,
- any cycle-derived counters required for rate limits or eligibility.

Additional payload text and historical depth MAY be included according to snapshot tier, but omission of payload text MUST NOT affect determinism or validity.


### 11.4 conformance requirements for node behavior [anchor: conformance_requirements_for_node_behavior]

A node is conformant if, and only if, it reconstructs the canonical universe by deterministic replay of the event log and snapshots without deviation, omission, or external dependency. Conformance is defined entirely by observable behavior; intentions, implementation languages, or performance optimizations are irrelevant unless they alter canonical semantics.

A conformant node MUST:

1. **Process events in total order** and apply state transitions exactly as specified by active rulebooks.  
2. **Use snapshots as hard boundaries** for state verification; use cycle boundaries for rule activation.  
3. **Reject malformed events** that violate structural, syntactic, or semantic constraints.  
4. **Reject semantic-invalid events** that violate challenge lifecycle or rulebook constraints.  
5. **Treat rulebook activation strictly forward-only**, switching rulebooks only at designated cycle boundaries.  
6. **Preserve immutable history** and MUST NOT reorder, delete, or rewrite events.  
7. **Derive meaning solely from canonical rules** and SHALL NOT rely on external services, oracles, or proprietary logic to interpret events or rulebooks.  
8. **Maintain consistent identity semantics**, including verification state, succession, and the human-first authorship invariant.

A conformant node MUST reject any behavior, state transition, or inference that cannot be derived from canonical snapshots and event replay. Nodes MAY implement caching, indexes, local checkpoints, or other performance enhancements, provided such enhancements do not influence semantics.

Nodes incapable of reconciling local state with the event log and snapshots MUST fall back to replay from the nearest valid snapshot or MUST declare themselves non-conformant.

### 11.5 invalid snapshots and recovery mechanisms [anchor: invalid_snapshots_and_recovery_mechanisms]

A snapshot is invalid if its contents cannot be reconstructed deterministically from the events preceding it. Invalid snapshots SHALL NOT alter canonical interpretation of the event log and MUST be disregarded during replay.

A snapshot is invalid if:

* it includes state inconsistent with earlier events,  
* it omits required fields or includes malformed metadata,  
* it incorporates information not derivable from the event log,  
* it activates an invalid rulebook (as defined in Section 11.9),  
* its hash does not match its serialized canonical contents,  
* it contradicts Section 0 invariants or rulebook semantics active at the time.

Invalid snapshots MUST be ignored, and nodes SHALL recover by:

1. **Replaying from the previous valid snapshot**,  
2. **Recomputing state** up to the index of the invalid snapshot,  
3. **Verifying canonical state** against the next valid snapshot.

If no subsequent valid snapshot exists, nodes SHALL continue from the fallback state.

Invalid snapshots are preserved in the event log as historical artifacts. They MAY be referenced in future governance debates or rulebook refinements but SHALL NOT influence canonical state.

### 11.6 replay synchronization, partial replay, and trustless verification [anchor: replay_synchronization_partial_replay_and_trustless_verification]

Deterministic replay requires that all nodes reconstruct the same universe from the same sequence of events and snapshots. Replay synchronization is governed by the following requirements:

* Nodes MUST replay events in strict canonical order.  
* Nodes MUST apply exactly the rulebooks active at each boundary.  
* Nodes MUST NOT reorder events or insert supplemental logic.  
* The combination of event log + snapshots MUST fully determine state.

Partial replay is permitted if:

* the node begins from a valid canonical snapshot, and  
* the replay covers all subsequent events without omission.

Trustless verification is made possible by:

1. **Snapshot commitments** — hashes that allow nodes to validate state without trusting implementations.  
2. **Canonical serialization formats** — ensuring identical byte-level representations.  
3. **Conformance tests** — derived directly from rulebooks, enabling nodes to verify each other.  
4. **Absence of external dependencies** — all semantic meaning is internal to the log.

Nodes MAY verify one another by exchanging snapshot commitments or by replaying each other’s snapshots and comparing results. Any divergence indicates non-conformance.

### 11.7 light clients and partial-state canonicality [anchor: light_clients_and_partial_state_canonicality]

Light clients operate without storing the full universe but MUST remain able to verify canonical state transitions. Light clients MAY rely on:

* snapshot commitments,  
* Merkle or hash trees of idea and connection sets,  
* succinct proofs of challenge outcomes,  
* compressed importance rankings,  
* identity state digests.

Light clients MUST be able to:

* verify that a received snapshot corresponds to a valid canonical commitment,  
* confirm rulebook activation at cycle boundaries,  
* validate challenge verdicts using proofs or commitments,  
* confirm that new events extend the canonical log.

Light clients SHALL NOT be required to:

* store the entire idea graph,  
* maintain full importance history,  
* replay all events from genesis,  
* store intermediate challenge or argument data beyond their proofs.

Partial-state canonicality requires that any information provided to light clients MUST be verifiable through deterministic proofs that reference snapshot commitments and event indices. Any unverifiable information SHALL NOT be considered canonical.

### 11.8 snapshot compression, pruning, and long-term archival guarantees [anchor: snapshot_compression_pruning_and_long_term_archival_guarantees]

Snapshots MAY use compression and pruning techniques to reduce storage requirements, provided such techniques do not alter canonical content or break deterministic replay.

Allowed techniques include:

* **structural compression** of repeated patterns in the idea graph,  
* **rank-history truncation** up to the current snapshot boundary,  
* **delta encoding** of connection sets,  
* **hash-accumulated summaries** of argument trees,  
* **periodic pruning of expired challenges**,  
* **content-addressable storage** for descriptions and rulebooks.

Nodes MUST retain enough information to:

* fully reconstruct canonical state at each snapshot boundary,  
* validate all rule activations,  
* verify the correctness of future snapshots.

Long-term archival guarantees require:

* at least one full, unpruned replica of the event log and snapshots to exist in the network at all times,  
* all pruning to remain optional for conformant nodes,  
* pruning rules to be optional convenience mechanisms, not required parts of the protocol.

Canonical events MUST NEVER be deleted.

Snapshots MAY compress, pack, or summarize historical data for efficiency, provided that:
- the full event log remains reconstructable from some combination of snapshots and blocks,
- no canonical meaning is lost.

Snapshot tiers MAY differ in payload inclusion:
- lighter tiers MAY include only living-map payloads or recent history,
- heavier or archival tiers MAY include full historical payloads.

Compression and pruning apply only to redundant representation, not to canonical events themselves. Long-term survivability MUST be achievable without reliance on any single snapshot tier.


### 11.9 interactions with offline seedpacks and reintegration [anchor: interactions_with_offline_seedpacks_and_reintegration]

Snapshot boundaries are essential for reintegrating offline seedpacks. A seedpack MAY contain:

* local snapshots representing offline state,  
* pending ideas, arguments, and challenges,  
* non-canonical draft rulebooks,  
* local importance rankings and identity state,  
* private idea-compatible Mindseed records and tribe-related offline drafts (neither is a canonical object until separately published through new canonical events).

Upon reintegration, offline snapshots MUST be considered non-canonical unless they correspond exactly to a canonical snapshot index and content. Local snapshots MAY be used for performance or reference but SHALL NOT modify the canonical chain.

Reintegration requires:

1. **Synchronizing upstream snapshots** to determine the current canonical boundary.  
2. **Publishing local work** into the canonical universe as new events.  
3. **Re-evaluating local rulebook proposals** under the governance rulebooks active upon reconnection.  
4. **Realigning challenge and voting semantics** to active rulebooks.  
5. **Discarding or reinterpreting offline-only events** that contradict canonical state.

Seedpacks SHALL NOT attempt to activate a rulebook retroactively by referencing an offline snapshot. Rulebook activation MUST always occur at a canonical cycle boundary.

### 11.10 role of snapshots in governance verification and lineage [anchor: role_of_snapshots_in_governance_rule_activation_and_lineage]

Snapshots are verification and checkpoint artifacts. They are not governance activation boundaries. A governance challenge MAY reference snapshot material for auditability, but activation is cycle-based and occurs only if:

#### Normative activation semantics (decision cadence and cycle-based activation) [anchor: normative_activation_semantics_decision_and_snapshot_boundary]

This subsection is normative and supersedes conflicting wording elsewhere in this document.

- `decision_event`: the canonical event that confirms a governance change at cycle close.
- `decision_cycle_index`: the cycle index in which `decision_event` is confirmed.
- `delay_policy`: the active governance rulebook mapping `change_class -> delay_cycles`.
- `change_class`: one of `emergency`, `standard`, `major`, `constitutional`.
- `delay_cycles`: deterministic cycle delay derived from `delay_policy(change_class)` under `delay_policy_version` active at `decision_cycle_index`.
- `activation_cycle_index = decision_cycle_index + delay_cycles`.

Normative lifecycle:
1. Governance challenges conclude at cycle end and emit `decision_event`.
2. Activation scheduling is computed from `decision_cycle_index`, `change_class`, and the `delay_policy_version` active at decision time.
3. The change activates at the start of `activation_cycle_index` (inclusive) for deterministic replay and conformance.
4. Nodes MUST compute identical `activation_cycle_index` from canonical replay inputs only.
5. Snapshots MUST NOT define activation boundaries.

Delay-policy constraints:
- The rulebook MUST publish minimum delay bounds per `change_class`.
- Recommended delay ranges are: `emergency` 0-1 cycle, `standard` 1-2 cycles, `major` 3-5 cycles, `constitutional` 10+ cycles.
- Governance MAY evolve exact delay mappings by rulebook update, but activation MUST remain cycle-boundary deterministic.
- Cycle boundary derivation is defined in Cycle Specification §1.2.1 `canonical_boundary_event_cycle_close` and §6.4 `determinism_requirements`.

1. the challenge is approved under the correct eligibility, quorum, and threshold rules,  
2. valid implementation occurs and is attested by a completion truth claim,  
3. no successful challenge falsifies the completion claim,  
4. all required rulebook dependencies are present at activation time,  
5. the activation cycle boundary is reached in deterministic replay.

Upon activation:

* the new rulebook becomes the authoritative rule set for all subsequent events,  
* its predecessor becomes part of governance lineage but is no longer active,  
* nodes MUST switch to the new rulebook at the activation cycle boundary.

Snapshots also provide a precise historical map of governance evolution. Future nodes inspecting the universe MUST be able to determine:

* which rulebook governed any event E,  
* which governance decision led to that rulebook’s activation,  
* whether implementation occurred correctly,  
* whether any superseded rulebooks were later revised or challenged.

Snapshots therefore serve as a verifiable lineage checkpoint layer, while activation boundaries remain cycle-based.

### 11.11 Cycle export packs and offline usability (normative) [anchor: cycle_export_packs_and_offline_usability_normative]

Cycle export packs are derived artifacts intended to improve offline usability without affecting canonical state.

Cycle export packs:
- are generated at cycle boundaries,
- summarize selected canonical content relevant to that cycle,
- and are fully regenerable from the canonical event log and rulebooks.

Export packs MAY be used to:
- browse and search important ideas offline,
- inspect historical “what mattered” views without full replay,
- bootstrap partial replicas in constrained environments.

Export packs MUST NOT:
- serve as resume points for canonical operation,
- substitute for snapshots,
- or be used as inputs to canonical computation.

Nodes MAY store, discard, or regenerate export packs independently without affecting conformance.


## 12. errors, invalid events, and recovery [anchor: 12_errors_invalid_events_and_recovery]

### 12.1 definition of invalid events and their detection [anchor: definition_of_invalid_events_and_their_detection]

An event is invalid if a conformant node, applying the active rulebooks and Section 0 invariants, cannot interpret or apply the event without violating canonical semantics. Invalid events MUST NOT alter state. They remain part of the immutable log as historical artifacts but SHALL be treated as inert during deterministic replay.

An event MAY be invalid for structural, semantic, temporal, or constitutional reasons. Detection occurs during replay when the node attempts to validate the event against:

* the structural requirements of its event type,  
* the rulebooks active at the event index,  
* the relevant governance lineage,  
* constitutional constraints defined in Section 0.

Invalidity MUST be deterministic. All conformant nodes SHALL classify the same event as valid or invalid when replaying the same log.

Invalid events MUST be recorded in the node’s diagnostic state for auditability and MAY be referenced by future governance proposals or safety rule updates.

### 12.2 structural invalidity: malformed or incomplete payloads [anchor: structural_invalidity_malformed_or_incomplete_payloads]

An event is structurally invalid if its serialized form fails to meet the minimal schema requirements for its type. Structural invalidity SHALL be detected prior to semantic validation.

Structural invalidity includes, but is not limited to:

* missing required fields,  
* fields of incorrect type or encoding,  
* malformed references (e.g., nonexistent idea IDs),  
* noncanonical serialization order,  
* invalid cryptographic signatures,  
* truncated or improperly encoded payloads,  
* violation of canonical length, complexity, or format constraints.

In such cases, the event:

* MUST NOT modify state,  
* MUST NOT participate in challenges, voting, or ranking,  
* MUST be logged as structurally invalid.

Nodes MAY include the event in local error reports and MAY surface it in UI for transparency, but the event SHALL NOT influence deterministic replay.

### 12.3 semantic invalidity: violations of rulebooks or constitutional invariants [anchor: semantic_invalidity_violations_of_rulebooks_or_constitutional_invariants]

An event is semantically invalid if its intended action violates rulebooks active at its position in the event log or contradicts constitutional invariants that supersede all rulebooks.

Semantic invalidity includes:

* attempts to create or modify ideas using disallowed fields or unrecognized idea roles,  
* challenges opened with ineligible participants under the active governance pool rules,  
* votes cast outside active voting windows or by invalid identities,  
* attempts to activate a rulebook without a valid completion truth claim,  
* event payloads that would result in weighted voting, retroactive rule changes, non-human voting, or suppression forbidden by Section 0,  
* attempts to grant authority or interpretation rights to off-chain systems.

Semantic invalidity SHALL be enforced strictly and deterministically. Events that violate rule semantics MUST NOT alter state even if they contain structurally valid data.

Nodes encountering semantic invalidity MUST:

1. classify the event as invalid,  
2. skip its state transition,  
3. continue replay with the next event.

### 12.4 temporal invalidity: out-of-order, conflicting, or retroactive submissions [anchor: temporal_invalidity_out_of_order_conflicting_or_retroactive_submissions]

An event is temporally invalid if its semantics require a reference to a future state, contradict an already finalized state, or attempt to apply a rule retroactively.

Temporal invalidity occurs when:

* a challenge references an idea or event that had not yet been created at that index,  
* a vote attempts to participate in a challenge that has already closed,  
* a completion truth claim attempts to activate a rulebook at a past cycle boundary,  
* an event contradicts a verdict that has already taken effect,  
* a completion truth claim attempts to activate a rulebook at a past cycle boundary,  
* a governance proposal specifies an activation time inconsistent with the deterministic cycle schedule.

Temporal invalid events MUST be ignored for state transition purposes but preserved in the log as immutable context.

Nodes detecting temporal invalidity MUST:

1. leave state unchanged,  
2. record diagnostic information,  
3. proceed with replay without reordering events.

Temporal invalidity ensures that the canonical universe evolves strictly forward, respects snapshot boundaries, and prevents retroactive modification or manipulation.

### 12.5 invalid challenges, votes, and verdicts [anchor: invalid_challenges_votes_and_verdicts]

Challenges, votes, and verdicts MAY themselves be invalid. Because governance, truth determination, and importance computation depend on precise challenge semantics, invalid challenge-related events MUST be handled with strict determinism.

A challenge is invalid if:

* it references a nonexistent or ineligible idea,  
* it is opened by an identity not permitted to open that challenge type,  
* it violates active eligibility pool rules,  
* it attempts to challenge an unchallengeable domain (e.g., Section 0 invariants),  
* it violates timing constraints (e.g., opened after the allowed window),  
* it includes malformed or semantically invalid fields.

A vote is invalid if:

* the voter is not a verified human identity,  
* the voter is not in the eligible pool for that challenge,  
* the vote arrives outside the voting window,  
* the vote references the wrong challenge or a closed challenge,  
* it attempts to cast more than one vote per identity per challenge,  
* its payload is malformed or contradictory.

A verdict is invalid if:

* it aggregates votes incorrectly,  
* its quorum or threshold requirements are inconsistent with the active rulebook,  
* it derives from invalid votes or an invalid challenge,  
* it attempts to apply state transitions disallowed by Section 0.

Invalid challenges, votes, and verdicts SHALL:

* remain part of the log,  
* be ignored for state-transition purposes,  
* not influence challenge outcomes, importance rankings, or rulebook activation,  
* be available for diagnostic and governance review.

Nodes MUST classify challenge-related invalidity deterministically to ensure that all conformant nodes resolve governance and truth challenges identically.


### 12.6 automated and manual detection pathways [anchor: automated_and_manual_detection_pathways]

Invalidity MAY be detected through either automated or manual pathways, but detection MUST be deterministic when replaying the event log.

**Automated detection** includes:

* schema validation,  
* signature verification,  
* eligibility checks using rulebook-defined pools,  
* timing validation (challenge windows, vote windows),  
* verification that referenced ideas or events exist,  
* rulebook-defined safety classifier checks for disallowed payloads,  
* internal consistency checks (e.g., idea role misuse).

**Manual detection** includes:

* participants identifying contradictions with rulebooks,  
* detection of attempts to bypass challenge mechanics,  
* governance decisions identifying malformed rulebooks,  
* arguments asserting that a completion truth claim is false,  
* human review of complex payloads where semantic meaning matters.

When manual discovery identifies an invalid event, it MUST be handled through canonical means:

* challenge of the relevant truth claim,  
* reference in a governance actionable idea proposing rulebook fixes,  
* safety rulebook updates to prevent recurrence.

Nodes MUST NOT introduce non-canonical or private invalidation mechanisms. All detection MUST eventually be expressible through deterministic rules or explicit protocol events.


### 12.7 node behavior upon encountering invalid events [anchor: node_behavior_upon_encountering_invalid_events]

When a conformant node encounters an invalid event, it MUST:

1. **classify the event as invalid** using deterministic rules,  
2. **skip the event’s state transition**,  
3. **record diagnostic metadata** (locally) identifying the category of invalidity,  
4. **continue replay** using the next event.

Nodes MUST NOT:

* halt replay,  
* attempt to repair invalid events,  
* reorder events to make them valid,  
* modify or remove the invalid event from history,  
* apply partial transitions from an invalid event.

Invalid events remain immutable components of the log. Their presence does not compromise the canonical universe because all conformant nodes will skip them identically.

Nodes MAY surface invalid events in user interfaces or logs, but MUST NOT treat them as semantically meaningful. They MAY be referenced indirectly in governance debates (e.g., to justify rulebook changes), but they SHALL NOT define canonical state.


### 12.8 chain-level invalidity, forks, and deterministic recovery [anchor: chain_level_invalidity_forks_and_deterministic_recovery]

If a sequence of invalid events, malformed snapshots, or inconsistent rule activations causes divergent interpretations among nodes, deterministic replay provides the resolution mechanism.

Forks MAY arise from:

* invalid snapshots inserted by one implementation but rejected by others,  
* divergent interpretations of eligibility or rule semantics (a conformance failure),  
* inconsistent application of rulebooks,  
* nodes relying on external or proprietary sources of meaning.

A fork is resolved by:

1. **Identifying the earliest point of divergence**,  
2. **Replaying both interpretations from the last common canonical snapshot**,  
3. **Rejecting any interpretation that requires treating an invalid rulebook or snapshot as valid**,  
4. **Selecting the interpretation that preserves rulebook lineage, governance activation rules, and Section 0 invariants**,  
5. **Declaring any implementation that produced the invalid interpretation non-conformant**.

No fork SHALL persist among conformant nodes. Nodes that cannot reconcile state MUST either:

* adopt the canonical replay result, or  
* declare themselves non-conformant and exit canonical participation.

Forks caused by software bugs, partial implementations, or incorrect rulebook activation SHALL NOT influence canonical meaning. Deterministic replay always prevails.

Deterministic recovery MUST be achievable using the canonical event blocks and snapshots alone.

In the presence of corruption, partial data loss, or invalid artifacts:
- nodes MUST be able to validate integrity via hash-chained blocks,
- resume operation from the most recent valid snapshot,
- and deterministically replay subsequent blocks to reconstruct canonical state.

Cycle export packs are non-authoritative and MUST be treated as optional, regenerable artifacts. Loss, corruption, or absence of export packs MUST NOT impede validation, recovery, or replay.

Forks are resolved exclusively by deterministic ordering of the canonical event log. No snapshot, export pack, or derived artifact may introduce ambiguity into recovery.


### 12.9 user-facing error signaling and diagnostic requirements [anchor: user_facing_error_signaling_and_diagnostic_requirements]

Although the canonical universe is implementation-agnostic, nodes interacting with human identities MUST expose meaningful diagnostics when invalid events occur. Users SHOULD be able to understand why an event or action was rejected, without requiring access to raw protocol logs.

User-facing diagnostic systems MUST:

* identify whether the invalidity was structural, semantic, temporal, or constitutional,  
* reference the active rulebook that caused the rejection,  
* explain how the event contradicted rule semantics,  
* provide references to relevant rulebook sections or ideas,  
* preserve privacy principles while exposing enough context for deliberation.

Nodes MAY provide additional tooling such as:

* event validators,  
* snapshot inspectors,  
* rulebook diff viewers,  
* challenge consistency analyzers.

However, user-facing diagnostics MUST NOT misrepresent canonical meaning or imply that invalid events could be accepted under different conditions.

When canonical objects are excluded from default views or participation, user-facing diagnostics MUST indicate the reason for exclusion.

Diagnostic categories MUST include, at minimum:
- lifecycle_state exclusion (rotted or burned),
- taint or fraud-related exclusion,
- safety-based abstraction or redaction,
- jurisdictional or legal abstraction,
- user-applied visibility filters.

Diagnostics MUST provide:
- a clear exclusion category,
- a reference to the applicable rule or rulebook,
- and an indication of how the object may be inspected, challenged, or restored where applicable.

Diagnostic signaling is a protocol requirement and MUST NOT depend solely on UI conventions.


### 12.10 protocol evolution in response to recurring error classes [anchor: protocol_evolution_in_response_to_recurring_error_classes]

Recurring error classes provide evidence that rulebooks or protocol specifications require refinement. The system MUST support governance-driven evolution to minimize future invalidity while preserving meaning, determinism, and historical integrity.

Protocol evolution may include:

* refining schemas for ideas, challenges, votes, or descriptions,  
* clarifying rulebook definitions that produce ambiguous interpretations,  
* modifying eligibility rules or challenge windows to reduce accidental invalidity,  
* strengthening safety rulebooks to prevent harmful or overly specific payloads,  
* revising or replacing rulebooks that produce frequent malformed events,  
* adding new structured fields or metadata to improve replay robustness.

However, evolution MUST remain forward-only and SHALL NOT:

* alter the interpretation of past events,  
* retroactively classify previously valid events as invalid (except where mandated by Section 0's safety invariants),  
* weaken constitutional or deterministic constraints,  
* introduce nondeterministic or external dependencies.

Recurring error patterns SHOULD be discussed through ordinary idea creation, argument, and governance challenges. Governance MAY produce new rulebooks to address them, but MUST preserve interpretability, recoverability, and universal challengeability.



## 13. appendices and canonical references [anchor: 13_appendices_and_canonical_references]

### 13.1 purpose and scope of appendices [anchor: purpose_and_scope_of_appendices]

Appendices provide formal structures, reference materials, and deterministic definitions used throughout the protocol. They SHALL NOT introduce new semantics or modify any rule in Sections 0§11. Instead, appendices clarify, formalize, and detail concepts already defined.

If any appendix appears to contradict the main protocol, the contradiction SHALL be resolved by treating the main protocol text as authoritative.

Appendices MAY include:

* canonical data schemas,  
* serialization formats,  
* deterministic hashing rules,  
* safety-classifier interface specifications,  
* **Privacy and High-Risk Submission Spec (privacy-and-high-risk-submission-spec)** for privacy invariants, high-risk submission profiles, operator metadata minimization expectations, and outer-layer adoption privacy defaults,  
* identity proofs and signature formats,  
* reference implementations of challenge and verdict logic,  
* lexicons of permitted and required fields for all event types.

Governance MAY update appendices through rulebook supersession, provided such updates remain fully consistent with Section 0 invariants and do not alter core semantics.


### 13.2 canonical data schema for ideas, descriptions, and connections [anchor: canonical_data_schema_for_ideas_descriptions_and_connections]

This appendix defines the minimal schema required for conformant implementations. Individual nodes MAY include additional non-canonical metadata locally, but such metadata MUST NOT affect replay or state transitions.

#### 13.2.1 idea schema (minimal canonical form) [anchor: idea_schema_minimal_canonical_form]

Each idea MUST include:

* **idea_id** — globally unique identifier (UUIDv7 string),  
* **idea_role** — one of: `truth`, `conceptual`, `actionable`, `action`,  
* **speaker_identity** — identity asserting the idea,  
* **created_at_event** — index of event introducing the idea,  
* **representation_set** - one title slot plus twelve length-complexity description cells, each able to contain preserved candidate **representation objects** and at most one replay-derived canonical pointer,
* **subtype** — truth-claim subtype (if applicable),  
* **metadata** — reserved canonical fields for interoperability.

#### 13.2.2 title and description representation schema [anchor: description_schema]

Each idea MAY include multiple title and description candidates as separate **representation objects**. Every current-profile representation has:

* **representation_kind** - exactly `title` or `description`,
* **payload_hash** - deterministic hash pointer to the canonical text payload bytes,
* **author_identity_id** - identity providing this representation; for canonical `representation_create`, this is exactly the event speaker and that identity MUST already exist at the event's canonical position.

For `representation_kind = title`, `tier_length`, `tier_complexity`, and `vocabulary_version_id` MUST be absent. For `representation_kind = description`, `tier_length` MUST be one of `sentence`, `paragraph`, or `full`, and `tier_complexity` MUST be one of `fundamental`, `standard`, `advanced`, or `canonical`. `vocabulary_version_id` is required exactly for a canonical-complexity description, forbidden otherwise, and references an already-existing ordinary governed vocabulary-version idea without defaulting or inference.

Deterministic replay SHALL select the canonical title and canonical description for each of the twelve description cells using rulebook-defined selection functions and canonical representation pointers.

#### 13.2.2A representation terminology and pointer model [anchor: representation_terminology_and_pointer_model]

For clarity, the following terms are normative in this appendix:

* **representation object** - a canonical object containing a single title payload reference or one tiered description payload reference for one target object (`idea` or `ordering`).
* **candidate representation** (also called **competing representation**) - a representation object in canonical history that is not currently selected by the canonical pointer for its tier.
* **canonical representation pointer** - the object-level pointer from the title slot or a specific description `(tier_length, tier_complexity)` cell to the currently selected representation object.
* **proposed description** - synonymous with a candidate representation object not currently selected by the canonical representation pointer.

Representations are canonical objects; canonical selection changes pointers, not representation object history.
All candidate representations, including competing/proposed representations, are part of the universal canonical substrate and MUST remain publicly inspectable and challengeable under the applicable challenge domain.
#### 13.2.2B scoped display overrides (normative) [anchor: scoped_display_overrides_normative]
Scoped display overrides define scope-specific effective display selection without creating new representation objects.
A scoped display override MUST be keyed by (scope_kind, anchor_id) and MUST reference:
- an existing canonical target object (idea or ordering),
- a specific representation tier slot, and
- an already-canonical representation candidate for that same target and tier slot.
Scoped display overrides MUST NOT:
- create or delete representation candidates,
- rewrite representation candidate history, or
- alter canonical substrate object identity.
When no scoped display override exists for a given (scope_kind, anchor_id, target, tier slot), display resolution MUST fall back to the canonical representation pointer selected by canonical representation processes.
#### 13.2.3 connection schema [anchor: connection_schema]

Every connection SHALL include:

* **source_idea**, **target_idea**  
* **connection_type**  
* **created_at_event**  
* **supporting_metadata** — optional but canonical if present (metadata MUST follow the structured rules defined below).

The protocol defines a small, fixed set of valid `connection_type` values:

* `same_as` — tiered semantic equivalence between ideas or descriptions.
* `relative_importance` — directed importance and evidential relationships with a specified usage.
* `membership` — identity-to-group and idea-to-group membership and stewardship relationships.

Rulebooks MAY refine how these types are interpreted and MAY define additional allowed `usage` or `role` values, but SHALL NOT introduce new top-level `connection_type` values.

For `relative_importance` connections, `supporting_metadata` MUST include:

* `usage` — one of: `general`, `importance_argument`, `evidence_for`, `evidence_against` (plus any future usages defined by governance rulebooks within the same semantic scope).
* `axis` — one of the relative-importance axes: `important_to_reference` or `important_for_reference`.
* `timeframe` — one of the five temporal horizons: `near_term`, `mid_term`, `long_term`, `very_long_term`, or `trans_generational`.
* `scope` — `universal`, `tribe`, or `personal`.

Universal-importance orientation values (`important_to_current_individual`, `important_for_current_individual`, `important_to_collective`, and `important_for_collective`) are used for universal importance profiles and universal rank snapshots. They are not valid `axis` values for ordinary `relative_importance` connection metadata unless a future rulebook explicitly defines a deterministic projection.

For `usage = general`, only `scope = universal` and `scope = tribe` can feed canonical public-relative or tribe-relative rank contexts. `scope = personal` MAY describe an authored personal projection or argument connection, but it MUST NOT create canonical personal rank state, a personal importance challenge, or a private universal rank. Private owner-selected ordering remains outside the canonical connection log.

Rulebooks MAY add additional fields, provided they do not change the meaning of the core fields or break deterministic replay.

For `same_as` connections, `supporting_metadata` MUST include:

* `tier_kind` — one of: `title`, `sentence`, `paragraph`, `full`, `canonical`.

Multiple `same_as` connections between the same pair of ideas MAY exist, each with its own `tier_kind`. `same_as` connections are symmetric.

For `membership` connections, `supporting_metadata` MUST include:

* `role` — e.g. `member_of`, `steward_of`, `owns_special_garden`, or other membership roles defined by governance rulebooks.

Membership connections are directional from the member (identity or idea) to the group-like idea. Rulebooks MAY add non-semantic hints, but MUST NOT alter direction or the meaning of `role`.

Connection types MUST be defined and constrained in governance rulebooks and SHALL NOT be ad hoc. Evidence, importance arguments, and action-related relationships are all expressed as `relative_importance` connections with different `usage` values, not as separate connection types.

#### 13.2.4 Orderings and Vines (ordered sequences) [anchor: orderings_and_vines_ordered_sequences]

An **Ordering** is the one first-class canonical authored-sequence object. It stores an ordered sequence of referenced `idea_id` values and declares exactly one `ordering_profile`.

Ordering invariants:

* Orderings provide ordering context and MUST NOT, by themselves, assert truth, importance, causality, or execution.
* Orderings MUST use canonical representation pointers for title/sentence/paragraph/full descriptions under the shared representation system.
* Orderings are excluded from universal importance ranking by default unless a future rulebook explicitly opts in.
* Orderings MUST preserve item order exactly as authored in canonical history.
* `ordering_profile` is exactly one of `vine`, `evidence_rail`, or `action_rail`.
* Vine, Evidence Rail, and Action Rail are profiles over Ordering, not separate substrate object types.
* Navigation, relative-importance, chronology, event position, and visual-layout orderings remain derived unless a human explicitly authors and publishes an Ordering.
* `subject_idea_id` MUST be absent for `vine`, MUST reference an existing `truth_claim` for `evidence_rail`, and MUST reference an existing `actionable_idea` for `action_rail`.
* Standardized profiles MUST carry one `item_role` per item, aligned by item index. Evidence Rail roles are `potential_evidence` or `actual_evidence`; Action Rail roles are `potential_action` or `proposed_action`. Vines MUST NOT carry standardized item roles.
* Evidence Rail and Action Rail item lists MUST NOT contain duplicate `idea_id` values.
  This makes retained-item role preservation across forks deterministic.
* An Action Rail Ordering contains exactly one lane: all item roles are `potential_action` or all are `proposed_action`. Potential and proposed action spectra for the same actionable idea therefore remain two distinct Orderings. Selected and completed action state remains the result of challenge/action events and MUST NOT be inferred from an Ordering role.
* A fork MUST preserve its base Ordering's profile and subject. Any item retained from the base MUST retain its role, and an Action Rail fork MUST retain the base lane. Profiles, subjects, roles, titles, first items, and positions MUST NOT be used to infer one another.

A **Vine** is the ecosystem-facing open-ended authored Ordering profile.

* `vine_type = pathway_vine` — derived from navigation and saved explicitly as a canonical object.
* `vine_type = narrative_vine` — authored ordering without requiring relative-importance adjacency.
* `vine_type` is valid only when `ordering_profile = vine`.

An **Evidence Rail** is an Ordering with `ordering_profile = evidence_rail` and the mandatory evidence procedures defined by the applicable rulebook. An **Action Rail** is an Ordering with `ordering_profile = action_rail` and the mandatory action procedures defined by the applicable rulebook.

Pathway navigation in UI is ephemeral and non-canonical until a user explicitly creates and publishes a `pathway_vine`.

For `pathway_vine`, each adjacent step MAY include an optional underlying `relative_importance` connection reference (e.g., `via_connection_id`) used during traversal provenance.
For `narrative_vine`, no underlying-edge requirement exists.

Non-normative viewer note:
Orderings/Vines render as an explicit authored spine layer distinct from base graph connections. Base connections remain the map layer.

### 13.3 canonical serialization and hashing rules [anchor: canonical_serialization_and_hashing_rules]

Canonical serialization ensures that all conformant nodes compute identical byte-level representations of events, snapshots, and ideas.

Serialization MUST be:

* deterministic,  
* order-preserving,  
* canonical across implementations,  
* free of machine-dependent artifacts.

Serialization rules include:

1. **UTF-8 encoding** for all text,  
2. **lexicographic ordering** for all object keys,  
3. **canonical list ordering** based on creation index or explicit ordering rules,  
4. **stable subtype and role ordering**,  
5. **hashing using rulebook-specified algorithms** (e.g., SHA-256, BLAKE3),  
6. **no compression before hashing**,  
7. **no runtime-dependent data** (timestamps appear only as event payloads, never as serialization metadata).

Event, idea, and snapshot hashes MUST match across all conformant nodes. Any deviation indicates non-conformance.

Canonical event blocks MUST be hash-chained using a deterministic serialization format.

Each block MUST include:
- a block index,
- the hash of the immediately preceding block,
- the ordered identifiers of included canonical events,
- and a block hash computed deterministically from these fields.

Hashing and serialization rules MUST be fully specified and versioned to ensure cross-implementation compatibility. Block hashing exists solely for integrity and verification and MUST NOT be interpreted as authority, time, or consensus.


### 13.4 canonical event types and required fields [anchor: canonical_event_types_and_required_fields]

Appendix A defines the authoritative canonical event names and minimal required fields. Rulebooks MAY extend these definitions where Appendix A permits extension, but SHALL NOT remove, alias, or reinterpret required fields. Older prose aliases are non-authoritative.

Canonical event names use Appendix A spelling. The current catalog includes:

* **identity_create**,
* **identity_verification_update**,
* **identity_visibility_update**,
* **identity_key_rotate** / **identity_key_revoke**,
* **idea_create**,
* **idea_update_metadata**,
* **idea_deprecate** / **idea_retract**,
* **representation_create**,
* **ordering_create**,
* **ordering_fork**,
* **connection_create**,
* **connection_update**,
* **connection_remove**,
* **same_as_resolution**,
* **challenge_create**,
* **challenge_open_arguments**,
* **challenge_close_arguments**,
* **challenge_open_voting**,
* **challenge_close_voting**,
* **challenge_finalize_verdict**,
* **challenge_cancel** / **challenge_supersede**,
* **vote_cast**,
* **vote_commit** / **vote_reveal** where commit-reveal voting is enabled,
* **blocked_submission**,
* optional/interface token and safety events listed in Appendix A, where adopted by active rulebooks,
* **snapshot_commit** (canonical boundary index event for derived snapshots; replay no-op),
* **cycle_close** (canonical system-boundary event for structural cycle closure).

`idea_update_representation` is a historical compatibility record only and is not a
member of the clean live catalog above. Ordinary ingress MUST reject it; exact legacy
records may be recognized only under an explicit versioned historical manifest.

Completion claims are ordinary `truth_claim` ideas created via `idea_create`; they are not a
separate event type. Importance changes are expressed through connections, challenges, and verdicts,
not a standalone event.

Each ordinary human-authored event type SHALL be represented first as a signed authored candidate and then, after valid publication, as a published canonical event wrapper.

At minimum, the signed authored candidate records:

* **event_id** (UUIDv7 string),
* **signature_profile**,
* **event_type**,
* **author_identity_id**,
* **speaker_identity_id** (if applicable),
* **public_key_ref**,
* **payload_hash**,
* **payload** or **payload_ref** according to the event schema,
* **payload_binding_mode**,
* **signature** verifying authorship and payload integrity under `canonical-event-authorship-and-signature-profile-v0.md`.

At minimum, the published canonical event wrapper records the finalized canonical order and any publication-derived block, prefix-certificate, rulebook, payload-classification, and chain-reference metadata required by the publication profile.

All canonical events MUST include sufficient authored-candidate fields and publication-wrapper fields to support deterministic cycle derivation and lifecycle computation. The human signature MUST NOT bind a future publication-assigned `event_index`, block height, cycle index, finalized-prefix-certificate reference, or private account/session field.

At minimum, replay MUST be able to recover:
- a deterministic event identifier (UUIDv7 string),
- the identity performing the event,
- the event’s finalized position in canonical order,
- the cycle index derived from canonical replay context.

No new event types are introduced solely for rot/burn. Lifecycle_state derivation relies on existing event semantics and qualifying engagement predicates rather than dedicated lifecycle events.


### 13.5 identity proofs, signatures, and verification [anchor: identity_proofs_signatures_and_verification]

Identity semantics require deterministic, verifiable signatures for every canonical event.

A canonical ordinary human-authorship signature MUST:

* bind the Profile-v0 authored-candidate fields and payload hash,
* exclude publication-derived fields that do not exist when the human signs,
* be produced by a private key controlled by the eligible human identity that authors the event,
* be verifiable using the replay-derived identity key state for `public_key_ref`,
* adhere to `signature_profile = ed25519_v0` unless a later explicit signature-profile specification supersedes it.

`canonical-event-authorship-and-signature-profile-v0.md` defines the authored-candidate structure, Profile-v0 Ed25519 algorithm, exact signed bytes, public-key descriptor, `public_key_ref`, key rotation, key revocation, and authorship-signature conformance-vector requirements. Rulebooks MAY gate which humans are eligible for which event families, but conforming Profile-v0 implementations MUST NOT choose alternate human-signature algorithms locally.

For Profile-v0 identity admission, the sponsor's signature is the ordinary human-authorship signature for `identity_create`. The applicant's initial-key possession proof is a separate proof over applicant-relevant admission fields. It does not make the applicant the event author, and it does not require the applicant's proposed key to have been active before the event. Exact possession-proof bytes, payload placement, and no-reference encodings are deferred to Appendix A, canonical encoding, and Profile-v0 authorship/signature reconciliation.

Identity verification procedures MUST remain human-first and SHALL NOT rely on cryptographic proofs alone.


### 13.6 deterministic challenge and verdict logic reference [anchor: deterministic_challenge_and_verdict_logic_reference]

This appendix provides a formal, implementation-independent description of challenge and voting mechanics. It SHALL mirror the semantics described in Section 7, but with formal pseudocode or mathematical notation to guarantee consistent interpretation.

Included materials MAY cover:

* eligibility checks,  
* quorum computation,  
* challenge window timing,  
* verdict aggregation functions (e.g., simple majority),  
* tie-handling rules,  
* state transitions on verdict finalization,  
* rules for opening subsequent or derivative challenges.

Any rulebook that modifies challenge procedures MUST remain consistent with this reference.


### 13.7 importance computation and ranking algorithms [anchor: importance_computation_and_ranking_algorithms]

Importance ranking derives from rulebooks, but appendices provide:

* mathematical definitions of rank transformations,  
* tie-breaking procedures,  
* normalization rules for universal importance,  
* selection rules for relative importance subsets,  
* deterministic fallback logic when ranks cannot be computed.

Appendix 12.7 SHALL NOT define new meaning for importance; it formalizes rulebook semantics.


### 13.8 safety classifier interfaces and abstraction rules [anchor: safety_classifier_interfaces_and_abstraction_rules]

Safety rulebooks MAY require classifier interfaces for assessing payload specificity, risk categories, or redaction requirements.

This appendix defines:

* canonical classifier input formats,  
* classifier output schemas,  
* deterministic mapping from classifier output → allowable redaction actions,  
* abstraction rules ensuring meaning preservation,  
* procedures for recording **blocked_submission** events,  
* formal constraints guaranteeing that no payload censorship alters meaning.

Classifier implementations MUST remain transparent, inspectable, and challengeable.


### 13.9 conformance test suite and reference vectors [anchor: conformance_test_suite_and_reference_vectors]

To support interoperability, a canonical conformance suite SHALL be maintained, containing:

* example event sequences,  
* snapshots and commitments,  
* challenge lifecycles,  
* voting sequences with expected outcomes,  
* rulebook activation and supersession tests,  
* identity verification tests,  
* safety classifier demonstrations.

Nodes MUST pass the conformance suite to be considered fully conformant.  
Governance MAY expand the test suite but SHALL NOT introduce semantics inconsistent with earlier sections.


### 13.10 glossary of canonical terms and definitions [anchor: glossary_of_canonical_terms_and_definitions]

The glossary defines technical terms used throughout the protocol, ensuring stable meaning across implementations. Glossary entries SHALL include:

* **identity**, **idea**, **description**, **connection**,  
* **challenge**, **verdict**, **vote**, **eligibility pool**,  
* **snapshot**, **activation boundary**, **rulebook**,  
* **POD**, **POINT**, **importance**, **universal importance**,  
* **canonical universe**, **deterministic replay**,  
* **blocked_submission**, **completion truth-claim ideas**.

Glossary definitions MUST remain consistent with semantics in Sections 0§11 and SHALL NOT introduce new concepts.

**Block**
A quorum-finalized canonical publication unit that assigns `block_height` and intra-block event order while adding no semantic authority beyond publication ordering.

**Snapshot tier**  
A classification indicating the depth and payload coverage of a snapshot (e.g., light, medium, heavy, archival), all of which must be replay-equivalent.

**Cycle export pack**  
A non-authoritative, cycle-scoped summary bundle of selected canonical content intended for offline browsing and inspection.

**Lifecycle_state**  
A derived property of ideas and eligible connections indicating participation status in the living map (e.g., active, rotted, burned).

**Eligible edge**  
A canonical connection whose derived state permits participation in importance propagation and POD/POINT routing.

**Maintenance activity**  
Canonical engagement (such as challenges, votes, arguments, or evidence) that preserves or restores eligibility of a relative-importance connection.


### 13.11 structural roles: canonical metadata for personal, relational, and narrative spaces [anchor: structural_roles_canonical_metadata_for_personal_relational_and_narrative_spaces]

Structural roles provide a protocol-recognized way to assign narrative, organizational, or interface semantics to specific ideas **without introducing new canonical `idea_role` values, new importance scopes, or additional challenge semantics**. They exist strictly as deterministic metadata layered on top of the canonical ontology defined in Sections 1§7.

Structural-role ideas include Profile-v0 identity-root structures named Mindgarden, Backyard of Relationships, Self Tree, and Anthill, as well as user-created personal structures such as shrubs, memory leaves, and vines. These ideas remain ordinary canonical ideas or deterministic structural derivations with additional metadata constraints and SHALL NOT alter epistemic or governance flows.

#### 13.11.1 purpose of structural roles [anchor: purpose_of_structural_roles]

Structural roles enable:

* deterministic personal drafting spaces,  
* persistent relationship memories,  
* narrative and timeline structures,  
* social overlays such as mutual connections,  
* UI metaphors built on stable, replayable anchors.

They allow implementers to construct rich personal and relational interfaces while guaranteeing that these constructs DO NOT interfere with canonical truth, importance, POD flows, or governance.

#### 13.11.2 definition [anchor: definition_2]

A **structural_role** is canonical metadata assigned to an idea that dictates:

* how the idea is created,  
* how it MUST persist under deterministic replay,  
* which connection usages it MAY use,  
* and its allowable (non-epistemic) interpretations.

A structural_role is NOT:

* a new `idea_role` / idea type,  
* a new `connection_type`,  
* a new importance scope,  
* a POD-bearing entity,  
* or a governance-relevant classification.

Structural roles MUST remain semantically neutral with respect to Sections 1§5.

#### 13.11.3 creation rules [anchor: creation_rules]

Profile-v0 identity structural roots MUST be created or deterministically derived atomically with successful identity admission:

* Mindgarden
* Backyard of Relationships
* Self Tree
* Anthill

These MUST:

* be created or derived atomically with the canonical identity,
* use deterministic membership-style connections or deterministic derivation rules for the canonical identity,
* persist for the entire lifetime of the identity,  
* replay identically on every conformant node.

Other structural-role ideas (e.g., shrubs, memory leaves, vines, or narrative clusters) MAY be created by users, but MUST still follow:

* deterministic event creation,  
* canonical serialization rules,  
* restricted connection semantics,  
* stable structural_role assignment.

Once created, structural_role metadata MUST NOT be reassigned or removed.

#### 13.11.4 persistence and immutability [anchor: persistence_and_immutability]

Structural-role ideas:

* MUST persist for the lifetime of the identity or narrative structure they belong to,  
* MUST NOT be merged with other ideas (even via `same_as`),  
* MUST NOT transform into a different structural role,  
* MUST NOT be deleted except under full identity retirement as defined in §9.6,  
* MUST reconstruct identically under deterministic replay.

These constraints prevent personal or relational structures from mutating into epistemic or governance-bearing constructs.

#### 13.11.5 allowed and restricted semantics [anchor: allowed_and_restricted_semantics]

Structural-role ideas SHALL NOT:

* introduce new importance scopes,  
* receive or modify POD or POINT flows,  
* participate in universal- or tribe-scope importance ranking,  
* be subjects or targets of any challenge domain,  
* alter eligibility pools for voting,  
* serve as evidence or arguments in truth or importance challenges,  
* change the meaning of canonical ideas attached to them.

They MAY:

* serve as containers for personal drafts,  
* index authored ideas,  
* host relational memories,  
* index mutual connections,  
* provide timeline / narrative sequencing (e.g., vines),  
* support UI-level metaphors for navigation.

Structural-role ideas are fully exempt from all challenge domains, importance calculations, and POD flows. They are invisible to the protocol’s epistemic, governance, and economic mechanics. This means:

- No challenge (truth, importance, action, or representation) may target or reference a structural-role idea as subject, evidence, or argument.
- No `relative_importance` connections to or from structural-role ideas carry epistemic weight or contribute to universal, public-relative, or tribe-relative importance rankings. Private product projections MUST NOT promote such structural edges into canonical rank inputs.
- No POD or POINT accrual, routing, or influence may derive from activity involving structural-role ideas.
- Structural-role ideas exist solely as immutable organizational scaffolding for personal drafting, relational modeling, narrative sequencing, or UI navigation. They provide containers and indexes for human-authored content but never participate in collective deliberation or incentive flows.

This complete isolation ensures that personal structural spaces (e.g., Mindgarden, Backyard of Relationships, Self Tree, Anthill, or user-created equivalents) cannot inadvertently affect or be affected by the shared canonical reasoning graph.

#### 13.11.6 connection rules for structural roles [anchor: connection_rules_for_structural_roles]

Structural-role ideas MUST use only canonical `connection_type` values:

* `membership` — for identity → structural space relationships  
* `relative_importance` — only for non-epistemic indexing usages (e.g., timeline_next, narrative_order)  
* `same_as` — strongly discouraged; structural roles SHOULD NOT co-refer

All such connections MUST declare a `usage` field appropriate to the structural role, such as:

* `authored_by`  
* `in_backyard_of`  
* `relationship_context`  
* `self_narrative`  
* `peer_connection`  
* `memory_of`  
* `timeline_next` / `timeline_prev`  

These usages SHALL NOT introduce or imply any epistemic meaning.

#### 13.11.7 safe extension of structural roles [anchor: safe_extension_of_structural_roles]

Implementations MAY define additional structural roles (e.g. “mythology cluster,” “project tree,” “chapter root”), provided that:

1. they do NOT introduce new canonical `idea_role` values;  
2. they do NOT modify challenge, POD, importance, or governance semantics;  
3. they are deterministically replayable;  
4. they use only canonical `connection_type` values;  
5. they are registered in a public rulebook or role registry discoverable via event log.

Structural-role extensions MUST NOT change semantics of existing structural roles or canonical flows.

#### 13.11.8 replay and validation requirements [anchor: replay_and_validation_requirements]

All conformant nodes MUST:

* preserve structural_role metadata across replay and snapshots,  
* validate that structural-role ideas follow creation and connection rules,  
* reject events that treat structural-role ideas as epistemic, POD-bearing, or governance-bearing entities,  
* enforce immutability of structural_role tags.

Structural roles ensure that personal, relational, and narrative structures remain fully compatible with the protocol’s epistemic core while allowing rich interface and social-layer expression.

## Anchors Added
- canonical_substrate_and_scoped_overlays
- scoped_display_overrides_normative
