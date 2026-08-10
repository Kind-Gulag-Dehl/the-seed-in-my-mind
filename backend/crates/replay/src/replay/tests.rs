use super::*;
use serde_json::json;

fn v7(id: &str) -> Uuid {
    Uuid::parse_str(id).expect("uuid parse")
}

fn system_emitter() -> Uuid {
    Uuid::parse_str(SYSTEM_BOUNDARY_EMITTER_ID_STR).expect("system emitter id")
}

fn empty_maps() -> (
    HashMap<Uuid, IdeaRow>,
    HashMap<Uuid, OrderingRow>,
    HashMap<Uuid, Vec<OrderingItemRow>>,
    HashMap<Uuid, ConnectionRow>,
    HashMap<Uuid, RepresentationRow>,
    HashMap<Uuid, IdeaPayloadRow>,
) {
    (
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    )
}

fn tempo_row(
    block_height: i64,
    event_index: i32,
    cycle_age_ge_dmin: bool,
    cycle_age_ge_dmax: bool,
) -> TempoPredicateRow {
    TempoPredicateRow {
        block_height,
        event_index,
        cycle_age_ge_dmin,
        cycle_age_ge_dmax,
        constrained_mode: false,
        record_only_mode: false,
    }
}

fn identity_create_event(event_id: Uuid, identity_id: Uuid, event_index: i32) -> EventRow {
    EventRow {
        block_height: 1,
        event_index,
        event_id,
        event_type: "identity_create".to_string(),
        speaker_identity_id: Some(identity_id),
        payload_json: json!({
            "identity_id": identity_id,
            "title": "Representation author",
            "speaker_identity_id": identity_id
        }),
    }
}

fn idea_create_fixture(
    event_id: Uuid,
    idea_id: Uuid,
    speaker_identity_id: Uuid,
    event_index: i32,
    title: &str,
) -> (EventRow, IdeaRow, IdeaPayloadRow) {
    let sentence = format!("{title} sentence");
    let payload_hash =
        encoding::payload::payload_hash_hex(title, &sentence, None, None).expect("payload hash");
    (
        EventRow {
            block_height: 1,
            event_index,
            event_id,
            event_type: "idea_create".to_string(),
            speaker_identity_id: Some(speaker_identity_id),
            payload_json: json!({
                "idea_id": idea_id,
                "idea_type": "conceptual_idea",
                "speaker_identity_id": speaker_identity_id,
                "title": title,
                "sentence": sentence,
                "payload_hash": payload_hash
            }),
        },
        IdeaRow {
            idea_id,
            idea_type: "conceptual_idea".to_string(),
            speaker_identity_id,
            created_event_id: event_id,
            created_block_height: 1,
            created_event_index: event_index,
        },
        IdeaPayloadRow {
            idea_id,
            title: Some(title.to_string()),
            sentence: Some(sentence),
            paragraph: None,
            full: None,
            payload_hash: Some(payload_hash),
        },
    )
}

fn forced_cycle_close_payload(cycle_index: i64, closure_block_height: i64) -> Value {
    json!({
        "cycle_index": cycle_index,
        "closure_kind": "forced",
        "forced_seal": true,
        "closure_boundary_ref": {
            "block_height": closure_block_height
        }
    })
}

fn deliberative_cycle_close_payload(cycle_index: i64, closure_block_height: i64) -> Value {
    json!({
        "cycle_index": cycle_index,
        "closure_kind": "deliberative",
        "forced_seal": false,
        "closure_boundary_ref": {
            "block_height": closure_block_height
        }
    })
}

fn challenge_create_payload(challenge_id: Uuid, left: Uuid, right: Uuid) -> Value {
    json!({
        "challenge_id": challenge_id,
        "challenge_domain": "importance_challenge",
        "framing_representation_ref": "00000000-0000-7000-8000-00000000d900",
        "context_key": "universal:default",
        "axis": "important_to_humanity",
        "timeframe": "medium_term",
        "scope": "universal",
        "subject_idea_ids": [left, right]
    })
}

fn cycle_boundary_row(
    cycle_index: i64,
    closure_kind: i16,
    forced_seal: bool,
    closure_block_height: i64,
    source_block_height: i64,
    source_event_index: i32,
    source_event_id: Uuid,
) -> CycleBoundaryRow {
    CycleBoundaryRow {
        cycle_index,
        closure_kind,
        forced_seal,
        closure_block_height,
        source_event_id,
        source_block_height,
        source_event_index,
    }
}

fn vote_session_open_payload(
    vote_session_id: Uuid,
    challenge_id: Uuid,
    session_index: i64,
    selection_cycle_index: i64,
    selection_boundary_event_id: Uuid,
    voter_identity_id: Uuid,
) -> Value {
    json!({
        "vote_session_id": vote_session_id,
        "challenge_id": challenge_id,
        "session_index": session_index,
        "selection_cycle_index": selection_cycle_index,
        "selection_boundary_event_id": selection_boundary_event_id,
        "speaker_identity_id": voter_identity_id
    })
}

fn vote_cast_payload(
    challenge_id: Uuid,
    vote_session_id: Uuid,
    vote_choice: &str,
    voter_identity_id: Uuid,
) -> Value {
    json!({
        "challenge_id": challenge_id,
        "vote_session_id": vote_session_id,
        "vote_choice": vote_choice,
        "speaker_identity_id": voter_identity_id
    })
}

fn writer_verification_row(
    identity_id: Uuid,
    email_verified: bool,
    canonical_writer_level: i16,
    granted_by_identity_id: Uuid,
    source_event_id: Uuid,
    source_block_height: i64,
    source_event_index: i32,
) -> WriterVerificationMaterializedRow {
    WriterVerificationMaterializedRow {
        identity_id,
        email_verified,
        canonical_writer_level,
        granted_by_identity_id,
        source_event_id,
        source_block_height,
        source_event_index,
    }
}

fn verifier_role_row(
    verifier_identity_id: Uuid,
    is_active: bool,
    source_event_id: Option<Uuid>,
    source_block_height: Option<i64>,
    source_event_index: Option<i32>,
) -> VerifierRoleRow {
    VerifierRoleRow {
        verifier_identity_id,
        is_active,
        source_event_id,
        source_block_height,
        source_event_index,
    }
}

#[test]
fn replay_is_deterministic() {
    let event = EventRow {
        block_height: 1,
        event_index: 0,
        event_id: v7("00000000-0000-7000-8000-000000000101"),
        event_type: "idea_create".to_string(),
        speaker_identity_id: Some(v7("00000000-0000-7000-8000-00000000a001")),
        payload_json: json!({
            "idea_id": "00000000-0000-7000-8000-00000000b001",
            "idea_type": "truth_claim",
            "speaker_identity_id": "00000000-0000-7000-8000-00000000a001",
            "title": "title",
            "sentence": "sentence",
            "payload_hash": encoding::payload::payload_hash_hex("title", "sentence", None, None).unwrap()
        }),
    };

    let idea_row = IdeaRow {
        idea_id: v7("00000000-0000-7000-8000-00000000b001"),
        idea_type: "truth_claim".to_string(),
        speaker_identity_id: v7("00000000-0000-7000-8000-00000000a001"),
        created_event_id: v7("00000000-0000-7000-8000-000000000101"),
        created_block_height: 1,
        created_event_index: 0,
    };

    let payload_row = IdeaPayloadRow {
        idea_id: idea_row.idea_id,
        title: Some("title".to_string()),
        sentence: Some("sentence".to_string()),
        paragraph: None,
        full: None,
        payload_hash: Some(
            encoding::payload::payload_hash_hex("title", "sentence", None, None).unwrap(),
        ),
    };

    let mut idea_map = HashMap::new();
    idea_map.insert(idea_row.created_event_id, idea_row);
    let ordering_map = HashMap::new();
    let ordering_items = HashMap::new();
    let connection_map = HashMap::new();
    let representation_map = HashMap::new();
    let mut payload_map = HashMap::new();
    payload_map.insert(payload_row.idea_id, payload_row);
    let tempo_rows = vec![tempo_row(1, 0, false, false)];
    let cycle_boundary_map = HashMap::new();

    let first = apply_events(
        &[event.clone()],
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .unwrap();
    let second = apply_events(
        &[event],
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .unwrap();
    assert_eq!(first.ideas, second.ideas);
    assert_eq!(first.payloads, second.payloads);
    assert_eq!(first.cycle_status, second.cycle_status);
    assert_eq!(first.tempo_status, second.tempo_status);
}

#[test]
fn title_representation_replays_as_a_separate_slot() {
    let author_identity_id = v7("00000000-0000-7000-8000-00000000a001");
    let target_idea_id = v7("00000000-0000-7000-8000-00000000c001");
    let representation_id = v7("00000000-0000-7000-8000-00000000d001");
    let identity_event_id = v7("00000000-0000-7000-8000-00000000e000");
    let target_event_id = v7("00000000-0000-7000-8000-00000000e100");
    let representation_event_id = v7("00000000-0000-7000-8000-00000000e001");
    let title = "The Seed in My Mind";
    let payload_hash = "1111111111111111111111111111111111111111111111111111111111111111";
    let (target_event, target_row, target_payload) = idea_create_fixture(
        target_event_id,
        target_idea_id,
        author_identity_id,
        1,
        "Target idea",
    );
    let representation_event = EventRow {
        block_height: 1,
        event_index: 2,
        event_id: representation_event_id,
        event_type: "representation_create".to_string(),
        speaker_identity_id: Some(author_identity_id),
        payload_json: json!({
            "representation_id": representation_id,
            "target_kind": "idea",
            "target_object_id": target_idea_id,
            "representation_kind": "title",
            "payload_hash": payload_hash,
            "payload_text": title,
            "author_identity_id": author_identity_id
        }),
    };
    let representation_row = RepresentationRow {
        representation_id,
        target_kind: 0,
        target_id: target_idea_id,
        tier_enum: 0,
        tier_complexity: None,
        vocabulary_version_id: None,
        payload_hash: payload_hash.to_string(),
        payload_text: Some(title.to_string()),
        author_identity_id,
        language_locale: None,
        provenance: None,
        created_event_id: representation_event_id,
        created_block_height: 1,
        created_event_index: 2,
    };
    let events = vec![
        identity_create_event(identity_event_id, author_identity_id, 0),
        target_event,
        representation_event,
    ];
    let mut idea_by_event = HashMap::new();
    idea_by_event.insert(target_event_id, target_row);
    let mut representation_by_event = HashMap::new();
    representation_by_event.insert(representation_event_id, representation_row);
    let mut payload_by_idea = HashMap::new();
    payload_by_idea.insert(target_idea_id, target_payload);

    let output = apply_events(
        &events,
        &idea_by_event,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &representation_by_event,
        &payload_by_idea,
        &[
            tempo_row(1, 0, false, false),
            tempo_row(1, 1, false, false),
            tempo_row(1, 2, false, false),
        ],
        &HashMap::new(),
    )
    .expect("title representation replay");

    assert_eq!(output.representations.len(), 1);
    let replayed = &output.representations[0];
    assert_eq!(replayed.representation_id, representation_id);
    assert_eq!(replayed.target_kind, "idea");
    assert_eq!(replayed.target_object_id, target_idea_id);
    assert_eq!(replayed.representation_kind, "title");
    assert_eq!(replayed.tier_length, None);
    assert_eq!(replayed.tier_complexity, None);
    assert_eq!(replayed.vocabulary_version_id, None);
    assert_eq!(replayed.payload_hash, payload_hash);
    assert_eq!(replayed.payload_text.as_deref(), Some(title));
    assert_eq!(replayed.author_identity_id, author_identity_id);
    assert_eq!(replayed.created_event_id, representation_event_id);
}

#[test]
fn replay_rejects_a_materialized_but_later_vocabulary_idea() {
    let author_identity_id = v7("00000000-0000-7000-8000-00000000a001");
    let target_idea_id = v7("00000000-0000-7000-8000-00000000c001");
    let vocabulary_version_id = v7("00000000-0000-7000-8000-00000000b003");
    let identity_event_id = v7("00000000-0000-7000-8000-00000000e000");
    let target_event_id = v7("00000000-0000-7000-8000-00000000e100");
    let representation_event_id = v7("00000000-0000-7000-8000-00000000e045");
    let vocabulary_event_id = v7("00000000-0000-7000-8000-00000000e101");
    let (target_event, target_row, target_payload) = idea_create_fixture(
        target_event_id,
        target_idea_id,
        author_identity_id,
        1,
        "Target idea",
    );
    let (vocabulary_event, vocabulary_row, vocabulary_payload) = idea_create_fixture(
        vocabulary_event_id,
        vocabulary_version_id,
        author_identity_id,
        3,
        "Vocabulary version",
    );
    let representation_event = EventRow {
        block_height: 1,
        event_index: 2,
        event_id: representation_event_id,
        event_type: "representation_create".to_string(),
        speaker_identity_id: Some(author_identity_id),
        payload_json: json!({
            "representation_id": "00000000-0000-7000-8000-00000000d045",
            "target_kind": "idea",
            "target_object_id": target_idea_id,
            "representation_kind": "description",
            "tier_length": "full",
            "tier_complexity": "canonical",
            "vocabulary_version_id": vocabulary_version_id,
            "payload_hash": "1111111111111111111111111111111111111111111111111111111111111111",
            "author_identity_id": author_identity_id
        }),
    };
    let representation_row = RepresentationRow {
        representation_id: v7("00000000-0000-7000-8000-00000000d045"),
        target_kind: 0,
        target_id: target_idea_id,
        tier_enum: 3,
        tier_complexity: Some(3),
        vocabulary_version_id: Some(vocabulary_version_id),
        payload_hash: "1111111111111111111111111111111111111111111111111111111111111111"
            .to_string(),
        payload_text: None,
        author_identity_id,
        language_locale: None,
        provenance: None,
        created_event_id: representation_event_id,
        created_block_height: 1,
        created_event_index: 2,
    };
    let events = vec![
        identity_create_event(identity_event_id, author_identity_id, 0),
        target_event,
        representation_event,
        vocabulary_event,
    ];
    let mut idea_by_event = HashMap::new();
    idea_by_event.insert(target_event_id, target_row);
    idea_by_event.insert(vocabulary_event_id, vocabulary_row);
    let mut representation_by_event = HashMap::new();
    representation_by_event.insert(representation_event_id, representation_row);
    let mut payload_by_idea = HashMap::new();
    payload_by_idea.insert(target_idea_id, target_payload);
    payload_by_idea.insert(vocabulary_version_id, vocabulary_payload);

    let error = apply_events(
        &events,
        &idea_by_event,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &representation_by_event,
        &payload_by_idea,
        &[
            tempo_row(1, 0, false, false),
            tempo_row(1, 1, false, false),
            tempo_row(1, 2, false, false),
            tempo_row(1, 3, false, false),
        ],
        &HashMap::new(),
    )
    .expect_err("a globally materialized later idea cannot satisfy pre-use");

    assert_eq!(error.code, "missing_vocabulary");
    assert!(error.message.contains(&vocabulary_version_id.to_string()));
}

#[test]
fn native_ordering_create_and_fork_replay_deterministically() {
    let speaker = v7("00000000-0000-7000-8000-00000000a010");
    let create_event_id = v7("00000000-0000-7000-8000-000000000110");
    let fork_event_id = v7("00000000-0000-7000-8000-000000000111");
    let base_ordering_id = v7("00000000-0000-7000-8000-00000000b010");
    let fork_ordering_id = v7("00000000-0000-7000-8000-00000000b011");
    let idea_a = v7("00000000-0000-7000-8000-00000000c010");
    let idea_b = v7("00000000-0000-7000-8000-00000000c011");
    let events = vec![
        EventRow {
            block_height: 1,
            event_index: 0,
            event_id: create_event_id,
            event_type: "ordering_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "ordering_id": base_ordering_id,
                "ordering_profile": "vine",
                "vine_type": "narrative_vine",
                "speaker_identity_id": speaker,
                "item_idea_ids": [idea_a]
            }),
        },
        EventRow {
            block_height: 1,
            event_index: 1,
            event_id: fork_event_id,
            event_type: "ordering_fork".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "base_ordering_id": base_ordering_id,
                "ordering_id": fork_ordering_id,
                "ordering_profile": "vine",
                "speaker_identity_id": speaker,
                "item_idea_ids": [idea_a, idea_b]
            }),
        },
    ];
    let mut ordering_map = HashMap::new();
    ordering_map.insert(
        create_event_id,
        OrderingRow {
            ordering_id: base_ordering_id,
            ordering_profile: 0,
            vine_type: Some(1),
            subject_idea_id: None,
            speaker_identity_id: speaker,
            created_event_id: create_event_id,
            created_block_height: 1,
            created_event_index: 0,
            base_ordering_id: None,
        },
    );
    ordering_map.insert(
        fork_event_id,
        OrderingRow {
            ordering_id: fork_ordering_id,
            ordering_profile: 0,
            vine_type: Some(1),
            subject_idea_id: None,
            speaker_identity_id: speaker,
            created_event_id: fork_event_id,
            created_block_height: 1,
            created_event_index: 1,
            base_ordering_id: Some(base_ordering_id),
        },
    );
    let mut ordering_items = HashMap::new();
    ordering_items.insert(
        base_ordering_id,
        vec![OrderingItemRow {
            ordering_id: base_ordering_id,
            idx: 0,
            idea_id: idea_a,
            item_role: None,
            via_connection_id: None,
        }],
    );
    ordering_items.insert(
        fork_ordering_id,
        vec![
            OrderingItemRow {
                ordering_id: fork_ordering_id,
                idx: 0,
                idea_id: idea_a,
                item_role: None,
                via_connection_id: None,
            },
            OrderingItemRow {
                ordering_id: fork_ordering_id,
                idx: 1,
                idea_id: idea_b,
                item_role: None,
                via_connection_id: None,
            },
        ],
    );
    let tempo_rows = vec![tempo_row(1, 0, false, false), tempo_row(1, 1, false, false)];
    let run = || {
        apply_events(
            &events,
            &HashMap::new(),
            &ordering_map,
            &ordering_items,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &tempo_rows,
            &HashMap::new(),
        )
        .expect("native Ordering replay")
    };

    let first = run();
    let second = run();
    assert_eq!(first.orderings, second.orderings);
    assert_eq!(first.orderings.len(), 2);
    assert_eq!(first.orderings[1].base_ordering_id, Some(base_ordering_id));
    assert_eq!(
        first.orderings[1].vine_type.as_deref(),
        Some("narrative_vine")
    );
    assert_eq!(first.orderings[1].items.len(), 2);
}

#[test]
fn replay_applies_canonical_writer_grant_and_revoke_deterministically() {
    let (idea_map, ordering_map, ordering_items, connection_map, representation_map, payload_map) =
        empty_maps();
    let cycle_boundary_map = HashMap::new();
    let tempo_rows = vec![tempo_row(1, 0, false, false), tempo_row(1, 1, false, false)];
    let verifier = seed_bootstrap_verifier_identity_id();
    let target_identity = v7("00000000-0000-7000-8000-00000000d001");
    let grant_event_id = v7("00000000-0000-7000-8000-000000000301");
    let revoke_event_id = v7("00000000-0000-7000-8000-000000000302");

    let events = vec![
        EventRow {
            block_height: 1,
            event_index: 0,
            event_id: grant_event_id,
            event_type: "canonical_writer_grant".to_string(),
            speaker_identity_id: Some(verifier),
            payload_json: json!({
                "identity_id": target_identity,
                "canonical_writer_level": 1,
                "email_verified": true
            }),
        },
        EventRow {
            block_height: 1,
            event_index: 1,
            event_id: revoke_event_id,
            event_type: "canonical_writer_revoke".to_string(),
            speaker_identity_id: Some(verifier),
            payload_json: json!({
                "identity_id": target_identity
            }),
        },
    ];

    let mut writer_verification_by_event = HashMap::new();
    writer_verification_by_event.insert(
        grant_event_id,
        writer_verification_row(target_identity, true, 1, verifier, grant_event_id, 1, 0),
    );
    writer_verification_by_event.insert(
        revoke_event_id,
        writer_verification_row(target_identity, false, 0, verifier, revoke_event_id, 1, 1),
    );
    let verifier_roles = vec![verifier_role_row(verifier, true, None, None, None)];

    let first = apply_events_with_verification(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
        &writer_verification_by_event,
        &verifier_roles,
    )
    .expect("replay should accept grant/revoke");
    let second = apply_events_with_verification(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
        &writer_verification_by_event,
        &verifier_roles,
    )
    .expect("replay should remain deterministic");

    assert_eq!(first.cycle_status, second.cycle_status);
    assert_eq!(first.tempo_status, second.tempo_status);
}

#[test]
fn replay_rejects_canonical_writer_grant_from_non_verifier() {
    let (idea_map, ordering_map, ordering_items, connection_map, representation_map, payload_map) =
        empty_maps();
    let cycle_boundary_map = HashMap::new();
    let tempo_rows = vec![tempo_row(1, 0, false, false)];
    let non_verifier = v7("00000000-0000-7000-8000-00000000a222");
    let target_identity = v7("00000000-0000-7000-8000-00000000d002");
    let event_id = v7("00000000-0000-7000-8000-000000000303");
    let events = vec![EventRow {
        block_height: 1,
        event_index: 0,
        event_id,
        event_type: "canonical_writer_grant".to_string(),
        speaker_identity_id: Some(non_verifier),
        payload_json: json!({
            "identity_id": target_identity,
            "canonical_writer_level": 1,
            "email_verified": true
        }),
    }];

    let mut writer_verification_by_event = HashMap::new();
    writer_verification_by_event.insert(
        event_id,
        writer_verification_row(target_identity, true, 1, non_verifier, event_id, 1, 0),
    );

    let err = apply_events_with_verification(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
        &writer_verification_by_event,
        &[],
    )
    .expect_err("non-verifier grant should fail");
    assert_eq!(err.code, "forbidden_author");
}

#[test]
fn replay_is_deterministic_for_idea_and_connection_across_cycle_boundary() {
    let (idea_map, ordering_map, ordering_items, connection_map, representation_map, payload_map) =
        empty_maps();

    let speaker = v7("00000000-0000-7000-8000-00000000a001");
    let idea_a_id = v7("00000000-0000-7000-8000-00000000b010");
    let idea_b_id = v7("00000000-0000-7000-8000-00000000b011");
    let idea_a_event_id = v7("00000000-0000-7000-8000-000000000910");
    let idea_b_event_id = v7("00000000-0000-7000-8000-000000000911");
    let cycle_close_event_id = v7("00000000-0000-7000-8000-000000000912");
    let connection_event_id = v7("00000000-0000-7000-8000-000000000913");
    let connection_id = v7("00000000-0000-7000-8000-00000000c010");

    let payload_hash_a = encoding::payload::payload_hash_hex("idea a", "sentence a", None, None)
        .expect("payload hash a");
    let payload_hash_b = encoding::payload::payload_hash_hex("idea b", "sentence b", None, None)
        .expect("payload hash b");

    let events = vec![
        EventRow {
            block_height: 1,
            event_index: 0,
            event_id: idea_a_event_id,
            event_type: "idea_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "idea_id": idea_a_id,
                "idea_type": "truth_claim",
                "speaker_identity_id": speaker,
                "title": "idea a",
                "sentence": "sentence a",
                "payload_hash": payload_hash_a
            }),
        },
        EventRow {
            block_height: 1,
            event_index: 1,
            event_id: idea_b_event_id,
            event_type: "idea_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "idea_id": idea_b_id,
                "idea_type": "conceptual_idea",
                "speaker_identity_id": speaker,
                "title": "idea b",
                "sentence": "sentence b",
                "payload_hash": payload_hash_b
            }),
        },
        EventRow {
            block_height: 1,
            event_index: 2,
            event_id: cycle_close_event_id,
            event_type: "cycle_close".to_string(),
            speaker_identity_id: Some(system_emitter()),
            payload_json: forced_cycle_close_payload(0, 1),
        },
        EventRow {
            block_height: 2,
            event_index: 0,
            event_id: connection_event_id,
            event_type: "connection_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "connection_id": connection_id,
                "from_idea_id": idea_a_id,
                "to_idea_id": idea_b_id,
                "connection_type": "same_as",
                "speaker_identity_id": speaker
            }),
        },
    ];

    let mut idea_by_event = idea_map;
    idea_by_event.insert(
        idea_a_event_id,
        IdeaRow {
            idea_id: idea_a_id,
            idea_type: "truth_claim".to_string(),
            speaker_identity_id: speaker,
            created_event_id: idea_a_event_id,
            created_block_height: 1,
            created_event_index: 0,
        },
    );
    idea_by_event.insert(
        idea_b_event_id,
        IdeaRow {
            idea_id: idea_b_id,
            idea_type: "conceptual_idea".to_string(),
            speaker_identity_id: speaker,
            created_event_id: idea_b_event_id,
            created_block_height: 1,
            created_event_index: 1,
        },
    );

    let mut payload_by_idea = payload_map;
    payload_by_idea.insert(
        idea_a_id,
        IdeaPayloadRow {
            idea_id: idea_a_id,
            title: Some("idea a".to_string()),
            sentence: Some("sentence a".to_string()),
            paragraph: None,
            full: None,
            payload_hash: Some(payload_hash_a),
        },
    );
    payload_by_idea.insert(
        idea_b_id,
        IdeaPayloadRow {
            idea_id: idea_b_id,
            title: Some("idea b".to_string()),
            sentence: Some("sentence b".to_string()),
            paragraph: None,
            full: None,
            payload_hash: Some(payload_hash_b),
        },
    );

    let mut connection_by_event = connection_map;
    connection_by_event.insert(
        connection_event_id,
        ConnectionRow {
            connection_id,
            from_idea_id: idea_a_id,
            to_idea_id: idea_b_id,
            connection_type: "same_as".to_string(),
            usage: None,
            axis: None,
            timeframe: None,
            scope: None,
            created_by_event_id: connection_event_id,
            created_block_height: 2,
            created_event_index: 0,
        },
    );

    let tempo_rows = vec![
        tempo_row(1, 0, false, false),
        tempo_row(1, 1, false, false),
        tempo_row(1, 2, true, true),
        tempo_row(2, 0, false, false),
    ];

    let mut cycle_boundary_map = HashMap::new();
    cycle_boundary_map.insert(
        cycle_close_event_id,
        cycle_boundary_row(0, 1, true, 1, 1, 2, cycle_close_event_id),
    );

    let first = apply_events(
        &events,
        &idea_by_event,
        &ordering_map,
        &ordering_items,
        &connection_by_event,
        &representation_map,
        &payload_by_idea,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .expect("first replay apply");
    let second = apply_events(
        &events,
        &idea_by_event,
        &ordering_map,
        &ordering_items,
        &connection_by_event,
        &representation_map,
        &payload_by_idea,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .expect("second replay apply");

    assert_eq!(first.ideas, second.ideas);
    assert_eq!(first.connections, second.connections);
    assert_eq!(first.payloads, second.payloads);
    assert_eq!(first.cycle_status, second.cycle_status);
    assert_eq!(first.tempo_status, second.tempo_status);
}

#[test]
fn replay_rejects_invalid_event() {
    let event = EventRow {
        block_height: 1,
        event_index: 0,
        event_id: v7("00000000-0000-7000-8000-000000000101"),
        event_type: "idea_create".to_string(),
        speaker_identity_id: Some(v7("00000000-0000-7000-8000-00000000a001")),
        payload_json: json!({
            "idea_id": "00000000-0000-7000-8000-00000000b001",
            "idea_type": "truth_claim",
            "speaker_identity_id": "00000000-0000-7000-8000-00000000a001",
            "title": "title",
            "payload_hash": "deadbeef"
        }),
    };

    let idea_row = IdeaRow {
        idea_id: v7("00000000-0000-7000-8000-00000000b001"),
        idea_type: "truth_claim".to_string(),
        speaker_identity_id: v7("00000000-0000-7000-8000-00000000a001"),
        created_event_id: v7("00000000-0000-7000-8000-000000000101"),
        created_block_height: 1,
        created_event_index: 0,
    };

    let mut idea_map = HashMap::new();
    idea_map.insert(idea_row.created_event_id, idea_row);
    let ordering_map = HashMap::new();
    let ordering_items = HashMap::new();
    let connection_map = HashMap::new();
    let representation_map = HashMap::new();
    let payload_map = HashMap::new();
    let tempo_rows = Vec::new();
    let cycle_boundary_map = HashMap::new();

    let err = apply_events(
        &[event],
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .expect_err("should error");
    assert_eq!(err.code, "event_validation_failed");
}

#[test]
fn cycle_close_dmin_guardrail_blocks_early_close() {
    let (idea_map, ordering_map, ordering_items, connection_map, representation_map, payload_map) =
        empty_maps();
    let human = v7("00000000-0000-7000-8000-00000000a001");
    let close_event_id = v7("00000000-0000-7000-8000-000000000402");
    let events = vec![
        EventRow {
            block_height: 1,
            event_index: 0,
            event_id: v7("00000000-0000-7000-8000-000000000401"),
            event_type: "noop".to_string(),
            speaker_identity_id: Some(human),
            payload_json: json!({}),
        },
        EventRow {
            block_height: 1,
            event_index: 1,
            event_id: close_event_id,
            event_type: "cycle_close".to_string(),
            speaker_identity_id: Some(system_emitter()),
            payload_json: deliberative_cycle_close_payload(0, 1),
        },
    ];
    let tempo_rows = vec![tempo_row(1, 0, false, false), tempo_row(1, 1, false, true)];
    let mut cycle_boundary_map = HashMap::new();
    cycle_boundary_map.insert(
        close_event_id,
        cycle_boundary_row(0, 0, false, 1, 1, 1, close_event_id),
    );

    let err = apply_events(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .expect_err("expected dmin guardrail failure");
    assert_eq!(err.code, "cycle_close_predicate_not_satisfied");
}

#[test]
fn forced_cycle_close_is_accepted_at_earliest_valid_position() {
    let (idea_map, ordering_map, ordering_items, connection_map, representation_map, payload_map) =
        empty_maps();
    let human = v7("00000000-0000-7000-8000-00000000a001");
    let close_event_id = v7("00000000-0000-7000-8000-000000000502");
    let events = vec![
        EventRow {
            block_height: 1,
            event_index: 0,
            event_id: v7("00000000-0000-7000-8000-000000000501"),
            event_type: "noop".to_string(),
            speaker_identity_id: Some(human),
            payload_json: json!({}),
        },
        EventRow {
            block_height: 1,
            event_index: 1,
            event_id: close_event_id,
            event_type: "cycle_close".to_string(),
            speaker_identity_id: Some(system_emitter()),
            payload_json: forced_cycle_close_payload(0, 1),
        },
    ];
    let tempo_rows = vec![tempo_row(1, 0, false, false), tempo_row(1, 1, true, true)];
    let mut cycle_boundary_map = HashMap::new();
    cycle_boundary_map.insert(
        close_event_id,
        cycle_boundary_row(0, 1, true, 1, 1, 1, close_event_id),
    );

    let applied = apply_events(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .expect("forced closure should be accepted");
    assert_eq!(applied.cycle_status.cycle_index, 1);
    assert_eq!(applied.cycle_status.last_cycle_close_height, Some(1));
}

#[test]
fn cycle_close_resets_cycle_age_predicates_for_next_cycle() {
    let (idea_map, ordering_map, ordering_items, connection_map, representation_map, payload_map) =
        empty_maps();
    let human = v7("00000000-0000-7000-8000-00000000a001");
    let close_event_id = v7("00000000-0000-7000-8000-000000000552");
    let events = vec![
        EventRow {
            block_height: 1,
            event_index: 0,
            event_id: v7("00000000-0000-7000-8000-000000000551"),
            event_type: "noop".to_string(),
            speaker_identity_id: Some(human),
            payload_json: json!({}),
        },
        EventRow {
            block_height: 1,
            event_index: 1,
            event_id: close_event_id,
            event_type: "cycle_close".to_string(),
            speaker_identity_id: Some(system_emitter()),
            payload_json: forced_cycle_close_payload(0, 1),
        },
        EventRow {
            block_height: 1,
            event_index: 2,
            event_id: v7("00000000-0000-7000-8000-000000000553"),
            event_type: "noop".to_string(),
            speaker_identity_id: Some(human),
            payload_json: json!({}),
        },
    ];
    let tempo_rows = vec![tempo_row(1, 0, false, false), tempo_row(1, 1, true, true)];
    let mut cycle_boundary_map = HashMap::new();
    cycle_boundary_map.insert(
        close_event_id,
        cycle_boundary_row(0, 1, true, 1, 1, 1, close_event_id),
    );

    let applied = apply_events(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .expect("next-cycle events should not inherit prior cycle dmin/dmax flags");
    assert_eq!(applied.cycle_status.cycle_index, 1);
    assert!(!applied.cycle_status.closure_predicate_satisfied);
}

#[test]
fn replay_rejects_when_cycle_close_is_not_earliest_valid() {
    let (idea_map, ordering_map, ordering_items, connection_map, representation_map, payload_map) =
        empty_maps();
    let human = v7("00000000-0000-7000-8000-00000000a001");
    let close_event_id = v7("00000000-0000-7000-8000-000000000602");
    let events = vec![
        EventRow {
            block_height: 1,
            event_index: 0,
            event_id: v7("00000000-0000-7000-8000-000000000601"),
            event_type: "noop".to_string(),
            speaker_identity_id: Some(human),
            payload_json: json!({}),
        },
        EventRow {
            block_height: 1,
            event_index: 1,
            event_id: close_event_id,
            event_type: "cycle_close".to_string(),
            speaker_identity_id: Some(system_emitter()),
            payload_json: forced_cycle_close_payload(0, 1),
        },
    ];
    let tempo_rows = vec![tempo_row(1, 0, true, true)];
    let mut cycle_boundary_map = HashMap::new();
    cycle_boundary_map.insert(
        close_event_id,
        cycle_boundary_row(0, 1, true, 1, 1, 1, close_event_id),
    );

    let err = apply_events(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .expect_err("expected earliest-valid rejection");
    assert_eq!(err.code, "missing_cycle_close_at_earliest_valid_position");
}

#[test]
fn replay_rejects_duplicate_cycle_close_for_closed_cycle() {
    let (idea_map, ordering_map, ordering_items, connection_map, representation_map, payload_map) =
        empty_maps();
    let close_1 = v7("00000000-0000-7000-8000-000000000701");
    let close_2 = v7("00000000-0000-7000-8000-000000000702");
    let events = vec![
        EventRow {
            block_height: 1,
            event_index: 0,
            event_id: close_1,
            event_type: "cycle_close".to_string(),
            speaker_identity_id: Some(system_emitter()),
            payload_json: forced_cycle_close_payload(0, 1),
        },
        EventRow {
            block_height: 1,
            event_index: 1,
            event_id: close_2,
            event_type: "cycle_close".to_string(),
            speaker_identity_id: Some(system_emitter()),
            payload_json: forced_cycle_close_payload(0, 1),
        },
    ];
    let tempo_rows = vec![tempo_row(1, 0, true, true), tempo_row(1, 1, true, true)];
    let mut cycle_boundary_map = HashMap::new();
    cycle_boundary_map.insert(close_1, cycle_boundary_row(0, 1, true, 1, 1, 0, close_1));
    cycle_boundary_map.insert(close_2, cycle_boundary_row(0, 1, true, 1, 1, 1, close_2));

    let err = apply_events(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .expect_err("duplicate cycle_close should fail");
    assert_eq!(err.code, "cycle_close_cycle_index_mismatch");
}

#[test]
fn replay_rejects_non_system_cycle_close_and_system_non_boundary() {
    let (idea_map, ordering_map, ordering_items, connection_map, representation_map, payload_map) =
        empty_maps();
    let human = v7("00000000-0000-7000-8000-00000000a001");

    let close_by_human = EventRow {
        block_height: 1,
        event_index: 0,
        event_id: v7("00000000-0000-7000-8000-000000000801"),
        event_type: "cycle_close".to_string(),
        speaker_identity_id: Some(human),
        payload_json: forced_cycle_close_payload(0, 1),
    };
    let err = apply_events(
        &[close_by_human],
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &[tempo_row(1, 0, true, true)],
        &HashMap::new(),
    )
    .expect_err("non-system cycle_close should fail");
    assert_eq!(err.code, "event_validation_failed");

    let non_boundary_by_system = EventRow {
        block_height: 1,
        event_index: 0,
        event_id: v7("00000000-0000-7000-8000-000000000802"),
        event_type: "noop".to_string(),
        speaker_identity_id: Some(system_emitter()),
        payload_json: json!({}),
    };
    let err = apply_events(
        &[non_boundary_by_system],
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &[],
        &HashMap::new(),
    )
    .expect_err("system emitter non-boundary should fail");
    assert_eq!(err.code, "event_validation_failed");
}

#[test]
fn replay_rejects_duplicate_importance_challenge_instance() {
    let (
        mut idea_map,
        ordering_map,
        ordering_items,
        connection_map,
        representation_map,
        mut payload_map,
    ) = empty_maps();
    let speaker = v7("00000000-0000-7000-8000-00000000a110");
    let idea_a_id = v7("00000000-0000-7000-8000-00000000b110");
    let idea_b_id = v7("00000000-0000-7000-8000-00000000b111");
    let idea_a_event_id = v7("00000000-0000-7000-8000-000000000a10");
    let idea_b_event_id = v7("00000000-0000-7000-8000-000000000a11");
    let challenge_1_id = v7("00000000-0000-7000-8000-00000000c110");
    let challenge_2_id = v7("00000000-0000-7000-8000-00000000c111");
    let challenge_1_event_id = v7("00000000-0000-7000-8000-000000000a12");
    let challenge_2_event_id = v7("00000000-0000-7000-8000-000000000a13");
    let payload_hash_a =
        encoding::payload::payload_hash_hex("idea a", "sentence a", None, None).unwrap();
    let payload_hash_b =
        encoding::payload::payload_hash_hex("idea b", "sentence b", None, None).unwrap();

    let events = vec![
        EventRow {
            block_height: 1,
            event_index: 0,
            event_id: idea_a_event_id,
            event_type: "idea_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "idea_id": idea_a_id,
                "idea_type": "conceptual_idea",
                "speaker_identity_id": speaker,
                "title": "idea a",
                "sentence": "sentence a",
                "payload_hash": payload_hash_a
            }),
        },
        EventRow {
            block_height: 1,
            event_index: 1,
            event_id: idea_b_event_id,
            event_type: "idea_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "idea_id": idea_b_id,
                "idea_type": "conceptual_idea",
                "speaker_identity_id": speaker,
                "title": "idea b",
                "sentence": "sentence b",
                "payload_hash": payload_hash_b
            }),
        },
        EventRow {
            block_height: 1,
            event_index: 2,
            event_id: challenge_1_event_id,
            event_type: "challenge_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: challenge_create_payload(challenge_1_id, idea_a_id, idea_b_id),
        },
        EventRow {
            block_height: 1,
            event_index: 3,
            event_id: challenge_2_event_id,
            event_type: "challenge_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: challenge_create_payload(challenge_2_id, idea_a_id, idea_b_id),
        },
    ];

    idea_map.insert(
        idea_a_event_id,
        IdeaRow {
            idea_id: idea_a_id,
            idea_type: "conceptual_idea".to_string(),
            speaker_identity_id: speaker,
            created_event_id: idea_a_event_id,
            created_block_height: 1,
            created_event_index: 0,
        },
    );
    idea_map.insert(
        idea_b_event_id,
        IdeaRow {
            idea_id: idea_b_id,
            idea_type: "conceptual_idea".to_string(),
            speaker_identity_id: speaker,
            created_event_id: idea_b_event_id,
            created_block_height: 1,
            created_event_index: 1,
        },
    );
    payload_map.insert(
        idea_a_id,
        IdeaPayloadRow {
            idea_id: idea_a_id,
            title: Some("idea a".to_string()),
            sentence: Some("sentence a".to_string()),
            paragraph: None,
            full: None,
            payload_hash: Some(payload_hash_a),
        },
    );
    payload_map.insert(
        idea_b_id,
        IdeaPayloadRow {
            idea_id: idea_b_id,
            title: Some("idea b".to_string()),
            sentence: Some("sentence b".to_string()),
            paragraph: None,
            full: None,
            payload_hash: Some(payload_hash_b),
        },
    );

    let err = apply_events(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &[
            tempo_row(1, 0, false, false),
            tempo_row(1, 1, false, false),
            tempo_row(1, 2, false, false),
            tempo_row(1, 3, false, false),
        ],
        &HashMap::new(),
    )
    .expect_err("duplicate challenge instance should fail");
    assert_eq!(err.code, "duplicate_challenge_instance");
}

#[test]
fn replay_rejects_importance_argument_after_voting_open_boundary() {
    let (
        mut idea_map,
        ordering_map,
        ordering_items,
        mut connection_map,
        representation_map,
        mut payload_map,
    ) = empty_maps();
    let speaker = v7("00000000-0000-7000-8000-00000000a120");
    let idea_a_id = v7("00000000-0000-7000-8000-00000000b120");
    let idea_b_id = v7("00000000-0000-7000-8000-00000000b121");
    let argument_id = v7("00000000-0000-7000-8000-00000000b122");
    let challenge_id = v7("00000000-0000-7000-8000-00000000c120");
    let close_event_id = v7("00000000-0000-7000-8000-000000000b15");
    let connection_id = v7("00000000-0000-7000-8000-00000000d120");
    let connection_event_id = v7("00000000-0000-7000-8000-000000000b16");

    let payload_hash_a =
        encoding::payload::payload_hash_hex("idea a", "sentence a", None, None).unwrap();
    let payload_hash_b =
        encoding::payload::payload_hash_hex("idea b", "sentence b", None, None).unwrap();
    let payload_hash_arg =
        encoding::payload::payload_hash_hex("arg", "arg sentence", None, None).unwrap();

    let idea_event_ids = [
        v7("00000000-0000-7000-8000-000000000b10"),
        v7("00000000-0000-7000-8000-000000000b11"),
        v7("00000000-0000-7000-8000-000000000b12"),
    ];

    let events = vec![
        EventRow {
            block_height: 1,
            event_index: 0,
            event_id: idea_event_ids[0],
            event_type: "idea_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "idea_id": idea_a_id,
                "idea_type": "conceptual_idea",
                "speaker_identity_id": speaker,
                "title": "idea a",
                "sentence": "sentence a",
                "payload_hash": payload_hash_a
            }),
        },
        EventRow {
            block_height: 1,
            event_index: 1,
            event_id: idea_event_ids[1],
            event_type: "idea_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "idea_id": idea_b_id,
                "idea_type": "conceptual_idea",
                "speaker_identity_id": speaker,
                "title": "idea b",
                "sentence": "sentence b",
                "payload_hash": payload_hash_b
            }),
        },
        EventRow {
            block_height: 1,
            event_index: 2,
            event_id: idea_event_ids[2],
            event_type: "idea_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "idea_id": argument_id,
                "idea_type": "conceptual_idea",
                "speaker_identity_id": speaker,
                "title": "arg",
                "sentence": "arg sentence",
                "payload_hash": payload_hash_arg
            }),
        },
        EventRow {
            block_height: 1,
            event_index: 3,
            event_id: v7("00000000-0000-7000-8000-000000000b13"),
            event_type: "challenge_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: challenge_create_payload(challenge_id, idea_a_id, idea_b_id),
        },
        EventRow {
            block_height: 1,
            event_index: 4,
            event_id: close_event_id,
            event_type: "cycle_close".to_string(),
            speaker_identity_id: Some(system_emitter()),
            payload_json: forced_cycle_close_payload(0, 1),
        },
        EventRow {
            block_height: 2,
            event_index: 0,
            event_id: connection_event_id,
            event_type: "connection_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "connection_id": connection_id,
                "from_idea_id": argument_id,
                "to_idea_id": idea_a_id,
                "connection_type": "relative_importance",
                "usage": "importance_argument",
                "axis": "important_to_humanity",
                "timeframe": "medium_term",
                "scope": "universal",
                "context_challenge_id": challenge_id,
                "speaker_identity_id": speaker
            }),
        },
    ];

    for (idx, idea_id, title, sentence, payload_hash) in [
        (0_i32, idea_a_id, "idea a", "sentence a", payload_hash_a),
        (1_i32, idea_b_id, "idea b", "sentence b", payload_hash_b),
        (2_i32, argument_id, "arg", "arg sentence", payload_hash_arg),
    ] {
        let event_id = idea_event_ids[idx as usize];
        idea_map.insert(
            event_id,
            IdeaRow {
                idea_id,
                idea_type: "conceptual_idea".to_string(),
                speaker_identity_id: speaker,
                created_event_id: event_id,
                created_block_height: 1,
                created_event_index: idx,
            },
        );
        payload_map.insert(
            idea_id,
            IdeaPayloadRow {
                idea_id,
                title: Some(title.to_string()),
                sentence: Some(sentence.to_string()),
                paragraph: None,
                full: None,
                payload_hash: Some(payload_hash),
            },
        );
    }

    connection_map.insert(
        connection_event_id,
        ConnectionRow {
            connection_id,
            from_idea_id: argument_id,
            to_idea_id: idea_a_id,
            connection_type: "relative_importance".to_string(),
            usage: Some("importance_argument".to_string()),
            axis: Some("important_to_humanity".to_string()),
            timeframe: Some("medium_term".to_string()),
            scope: Some("universal".to_string()),
            created_by_event_id: connection_event_id,
            created_block_height: 2,
            created_event_index: 0,
        },
    );

    let mut cycle_boundary_map = HashMap::new();
    cycle_boundary_map.insert(
        close_event_id,
        cycle_boundary_row(0, 1, true, 1, 1, 4, close_event_id),
    );

    let err = apply_events(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &[
            tempo_row(1, 0, false, false),
            tempo_row(1, 1, false, false),
            tempo_row(1, 2, false, false),
            tempo_row(1, 3, false, false),
            tempo_row(1, 4, true, true),
            tempo_row(2, 0, false, false),
        ],
        &cycle_boundary_map,
    )
    .expect_err("importance argument should fail outside open-arguments phase");
    assert_eq!(err.code, "challenge_lifecycle_invalid");
}

#[test]
fn replay_applies_importance_arguments_deterministically_in_order() {
    let (
        mut idea_map,
        ordering_map,
        ordering_items,
        mut connection_map,
        representation_map,
        mut payload_map,
    ) = empty_maps();
    let speaker = v7("00000000-0000-7000-8000-00000000a130");
    let idea_a_id = v7("00000000-0000-7000-8000-00000000b130");
    let idea_b_id = v7("00000000-0000-7000-8000-00000000b131");
    let argument_1_id = v7("00000000-0000-7000-8000-00000000b132");
    let argument_2_id = v7("00000000-0000-7000-8000-00000000b133");
    let challenge_id = v7("00000000-0000-7000-8000-00000000c130");
    let argument_conn_1_id = v7("00000000-0000-7000-8000-00000000d130");
    let argument_conn_2_id = v7("00000000-0000-7000-8000-00000000d131");
    let argument_conn_1_event = v7("00000000-0000-7000-8000-000000000c15");
    let argument_conn_2_event = v7("00000000-0000-7000-8000-000000000c16");

    let idea_specs = [
        (
            v7("00000000-0000-7000-8000-000000000c10"),
            idea_a_id,
            "idea a",
            "sentence a",
        ),
        (
            v7("00000000-0000-7000-8000-000000000c11"),
            idea_b_id,
            "idea b",
            "sentence b",
        ),
        (
            v7("00000000-0000-7000-8000-000000000c12"),
            argument_1_id,
            "arg one",
            "arg one sentence",
        ),
        (
            v7("00000000-0000-7000-8000-000000000c13"),
            argument_2_id,
            "arg two",
            "arg two sentence",
        ),
    ];

    let mut events = Vec::new();
    for (idx, event_id, idea_id, title, sentence) in
        idea_specs
            .iter()
            .enumerate()
            .map(|(idx, (event_id, idea_id, title, sentence))| {
                (idx as i32, *event_id, *idea_id, *title, *sentence)
            })
    {
        let payload_hash =
            encoding::payload::payload_hash_hex(title, sentence, None, None).unwrap();
        events.push(EventRow {
            block_height: 1,
            event_index: idx,
            event_id,
            event_type: "idea_create".to_string(),
            speaker_identity_id: Some(speaker),
            payload_json: json!({
                "idea_id": idea_id,
                "idea_type": "conceptual_idea",
                "speaker_identity_id": speaker,
                "title": title,
                "sentence": sentence,
                "payload_hash": payload_hash
            }),
        });
        idea_map.insert(
            event_id,
            IdeaRow {
                idea_id,
                idea_type: "conceptual_idea".to_string(),
                speaker_identity_id: speaker,
                created_event_id: event_id,
                created_block_height: 1,
                created_event_index: idx,
            },
        );
        payload_map.insert(
            idea_id,
            IdeaPayloadRow {
                idea_id,
                title: Some(title.to_string()),
                sentence: Some(sentence.to_string()),
                paragraph: None,
                full: None,
                payload_hash: Some(payload_hash),
            },
        );
    }

    events.push(EventRow {
        block_height: 1,
        event_index: 4,
        event_id: v7("00000000-0000-7000-8000-000000000c14"),
        event_type: "challenge_create".to_string(),
        speaker_identity_id: Some(speaker),
        payload_json: challenge_create_payload(challenge_id, idea_a_id, idea_b_id),
    });
    events.push(EventRow {
        block_height: 1,
        event_index: 5,
        event_id: argument_conn_1_event,
        event_type: "connection_create".to_string(),
        speaker_identity_id: Some(speaker),
        payload_json: json!({
            "connection_id": argument_conn_1_id,
            "from_idea_id": argument_1_id,
            "to_idea_id": idea_a_id,
            "connection_type": "relative_importance",
            "usage": "importance_argument",
            "axis": "important_to_humanity",
            "timeframe": "medium_term",
            "scope": "universal",
            "context_challenge_id": challenge_id,
            "speaker_identity_id": speaker
        }),
    });
    events.push(EventRow {
        block_height: 1,
        event_index: 6,
        event_id: argument_conn_2_event,
        event_type: "connection_create".to_string(),
        speaker_identity_id: Some(speaker),
        payload_json: json!({
            "connection_id": argument_conn_2_id,
            "from_idea_id": argument_2_id,
            "to_idea_id": idea_b_id,
            "connection_type": "relative_importance",
            "usage": "importance_argument",
            "axis": "important_to_humanity",
            "timeframe": "medium_term",
            "scope": "universal",
            "context_challenge_id": challenge_id,
            "speaker_identity_id": speaker
        }),
    });

    connection_map.insert(
        argument_conn_1_event,
        ConnectionRow {
            connection_id: argument_conn_1_id,
            from_idea_id: argument_1_id,
            to_idea_id: idea_a_id,
            connection_type: "relative_importance".to_string(),
            usage: Some("importance_argument".to_string()),
            axis: Some("important_to_humanity".to_string()),
            timeframe: Some("medium_term".to_string()),
            scope: Some("universal".to_string()),
            created_by_event_id: argument_conn_1_event,
            created_block_height: 1,
            created_event_index: 5,
        },
    );
    connection_map.insert(
        argument_conn_2_event,
        ConnectionRow {
            connection_id: argument_conn_2_id,
            from_idea_id: argument_2_id,
            to_idea_id: idea_b_id,
            connection_type: "relative_importance".to_string(),
            usage: Some("importance_argument".to_string()),
            axis: Some("important_to_humanity".to_string()),
            timeframe: Some("medium_term".to_string()),
            scope: Some("universal".to_string()),
            created_by_event_id: argument_conn_2_event,
            created_block_height: 1,
            created_event_index: 6,
        },
    );

    let tempo_rows = vec![
        tempo_row(1, 0, false, false),
        tempo_row(1, 1, false, false),
        tempo_row(1, 2, false, false),
        tempo_row(1, 3, false, false),
        tempo_row(1, 4, false, false),
        tempo_row(1, 5, false, false),
        tempo_row(1, 6, false, false),
    ];

    let first = apply_events(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &HashMap::new(),
    )
    .expect("first apply");
    let second = apply_events(
        &events,
        &idea_map,
        &ordering_map,
        &ordering_items,
        &connection_map,
        &representation_map,
        &payload_map,
        &tempo_rows,
        &HashMap::new(),
    )
    .expect("second apply");

    assert_eq!(first.connections, second.connections);
    assert_eq!(first.connections.len(), 2);
    assert_eq!(first.connections[0].connection_id, argument_conn_1_id);
    assert_eq!(first.connections[1].connection_id, argument_conn_2_id);
}

fn build_importance_voting_fixture(
    verdict_choice: &str,
) -> (
    Vec<EventRow>,
    HashMap<Uuid, IdeaRow>,
    HashMap<Uuid, IdeaPayloadRow>,
    Vec<TempoPredicateRow>,
    HashMap<Uuid, CycleBoundaryRow>,
) {
    let creator = v7("00000000-0000-7000-8000-00000000a201");
    let voter_1 = v7("00000000-0000-7000-8000-00000000a202");
    let voter_2 = v7("00000000-0000-7000-8000-00000000a203");
    let voter_3 = v7("00000000-0000-7000-8000-00000000a204");
    let idea_left = v7("00000000-0000-7000-8000-00000000b201");
    let idea_right = v7("00000000-0000-7000-8000-00000000b202");
    let idea_left_event = v7("00000000-0000-7000-8000-000000000b21");
    let idea_right_event = v7("00000000-0000-7000-8000-000000000b22");
    let challenge_id = v7("00000000-0000-7000-8000-00000000c201");
    let challenge_event = v7("00000000-0000-7000-8000-000000000b23");
    let close_0_event = v7("00000000-0000-7000-8000-000000000b20");
    let close_1_event = v7("00000000-0000-7000-8000-000000000b24");
    let session_1 = v7("00000000-0000-7000-8000-00000000e201");
    let session_2 = v7("00000000-0000-7000-8000-00000000e202");
    let session_3 = v7("00000000-0000-7000-8000-00000000e203");
    let session_event_1 = v7("00000000-0000-7000-8000-000000000b31");
    let session_event_2 = v7("00000000-0000-7000-8000-000000000b33");
    let session_event_3 = v7("00000000-0000-7000-8000-000000000b35");
    let vote_event_1 = v7("00000000-0000-7000-8000-000000000b32");
    let vote_event_2 = v7("00000000-0000-7000-8000-000000000b34");
    let vote_event_3 = v7("00000000-0000-7000-8000-000000000b36");
    let verdict_event = v7("00000000-0000-7000-8000-000000000b37");
    let verdict_id = v7("00000000-0000-7000-8000-00000000f201");

    let payload_hash_left =
        encoding::payload::payload_hash_hex("left", "left sentence", None, None).unwrap();
    let payload_hash_right =
        encoding::payload::payload_hash_hex("right", "right sentence", None, None).unwrap();
    let winning_target = match verdict_choice {
        "left" => Some(idea_left),
        "right" => Some(idea_right),
        _ => None,
    };

    let events = vec![
        EventRow {
            block_height: 1,
            event_index: 0,
            event_id: close_0_event,
            event_type: "cycle_close".to_string(),
            speaker_identity_id: Some(system_emitter()),
            payload_json: forced_cycle_close_payload(0, 1),
        },
        EventRow {
            block_height: 2,
            event_index: 0,
            event_id: idea_left_event,
            event_type: "idea_create".to_string(),
            speaker_identity_id: Some(creator),
            payload_json: json!({
                "idea_id": idea_left,
                "idea_type": "conceptual_idea",
                "speaker_identity_id": creator,
                "title": "left",
                "sentence": "left sentence",
                "payload_hash": payload_hash_left
            }),
        },
        EventRow {
            block_height: 2,
            event_index: 1,
            event_id: idea_right_event,
            event_type: "idea_create".to_string(),
            speaker_identity_id: Some(creator),
            payload_json: json!({
                "idea_id": idea_right,
                "idea_type": "conceptual_idea",
                "speaker_identity_id": creator,
                "title": "right",
                "sentence": "right sentence",
                "payload_hash": payload_hash_right
            }),
        },
        EventRow {
            block_height: 2,
            event_index: 2,
            event_id: challenge_event,
            event_type: "challenge_create".to_string(),
            speaker_identity_id: Some(creator),
            payload_json: challenge_create_payload(challenge_id, idea_left, idea_right),
        },
        EventRow {
            block_height: 3,
            event_index: 0,
            event_id: close_1_event,
            event_type: "cycle_close".to_string(),
            speaker_identity_id: Some(system_emitter()),
            payload_json: forced_cycle_close_payload(1, 3),
        },
        EventRow {
            block_height: 4,
            event_index: 0,
            event_id: session_event_1,
            event_type: "vote_session_open".to_string(),
            speaker_identity_id: Some(voter_1),
            payload_json: vote_session_open_payload(
                session_1,
                challenge_id,
                0,
                2,
                close_1_event,
                voter_1,
            ),
        },
        EventRow {
            block_height: 4,
            event_index: 1,
            event_id: vote_event_1,
            event_type: "vote_cast".to_string(),
            speaker_identity_id: Some(voter_1),
            payload_json: vote_cast_payload(challenge_id, session_1, "left", voter_1),
        },
        EventRow {
            block_height: 4,
            event_index: 2,
            event_id: session_event_2,
            event_type: "vote_session_open".to_string(),
            speaker_identity_id: Some(voter_2),
            payload_json: vote_session_open_payload(
                session_2,
                challenge_id,
                0,
                2,
                close_1_event,
                voter_2,
            ),
        },
        EventRow {
            block_height: 4,
            event_index: 3,
            event_id: vote_event_2,
            event_type: "vote_cast".to_string(),
            speaker_identity_id: Some(voter_2),
            payload_json: vote_cast_payload(challenge_id, session_2, "left", voter_2),
        },
        EventRow {
            block_height: 4,
            event_index: 4,
            event_id: session_event_3,
            event_type: "vote_session_open".to_string(),
            speaker_identity_id: Some(voter_3),
            payload_json: vote_session_open_payload(
                session_3,
                challenge_id,
                0,
                2,
                close_1_event,
                voter_3,
            ),
        },
        EventRow {
            block_height: 4,
            event_index: 5,
            event_id: vote_event_3,
            event_type: "vote_cast".to_string(),
            speaker_identity_id: Some(voter_3),
            payload_json: vote_cast_payload(challenge_id, session_3, "right", voter_3),
        },
        EventRow {
            block_height: 4,
            event_index: 6,
            event_id: verdict_event,
            event_type: "challenge_finalize_verdict".to_string(),
            speaker_identity_id: Some(voter_1),
            payload_json: json!({
                "challenge_id": challenge_id,
                "verdict_id": verdict_id,
                "winning_choice": verdict_choice,
                "winning_target_idea_id": winning_target,
                "left_votes": 2,
                "right_votes": 1,
                "total_votes": 3,
                "vote_event_ids": [vote_event_1, vote_event_2, vote_event_3]
            }),
        },
    ];

    let mut idea_map = HashMap::new();
    idea_map.insert(
        idea_left_event,
        IdeaRow {
            idea_id: idea_left,
            idea_type: "conceptual_idea".to_string(),
            speaker_identity_id: creator,
            created_event_id: idea_left_event,
            created_block_height: 2,
            created_event_index: 0,
        },
    );
    idea_map.insert(
        idea_right_event,
        IdeaRow {
            idea_id: idea_right,
            idea_type: "conceptual_idea".to_string(),
            speaker_identity_id: creator,
            created_event_id: idea_right_event,
            created_block_height: 2,
            created_event_index: 1,
        },
    );

    let mut payload_map = HashMap::new();
    payload_map.insert(
        idea_left,
        IdeaPayloadRow {
            idea_id: idea_left,
            title: Some("left".to_string()),
            sentence: Some("left sentence".to_string()),
            paragraph: None,
            full: None,
            payload_hash: Some(payload_hash_left),
        },
    );
    payload_map.insert(
        idea_right,
        IdeaPayloadRow {
            idea_id: idea_right,
            title: Some("right".to_string()),
            sentence: Some("right sentence".to_string()),
            paragraph: None,
            full: None,
            payload_hash: Some(payload_hash_right),
        },
    );

    let tempo_rows = vec![
        tempo_row(1, 0, true, true),
        tempo_row(2, 0, false, false),
        tempo_row(3, 0, true, true),
        tempo_row(4, 0, false, false),
    ];

    let mut cycle_boundary_map = HashMap::new();
    cycle_boundary_map.insert(
        close_0_event,
        cycle_boundary_row(0, 1, true, 1, 1, 0, close_0_event),
    );
    cycle_boundary_map.insert(
        close_1_event,
        cycle_boundary_row(1, 1, true, 3, 3, 0, close_1_event),
    );

    (
        events,
        idea_map,
        payload_map,
        tempo_rows,
        cycle_boundary_map,
    )
}

#[test]
fn replay_accepts_importance_vote_sessions_and_majority_verdict() {
    let (events, idea_map, payload_map, tempo_rows, cycle_boundary_map) =
        build_importance_voting_fixture("left");
    let applied = apply_events(
        &events,
        &idea_map,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .expect("replay should accept deterministic voting flow");
    assert_eq!(applied.cycle_status.cycle_index, 2);
    assert_eq!(applied.cycle_status.observed_work, 4);
}

#[test]
fn replay_rejects_mismatched_importance_verdict_payload() {
    let (events, idea_map, payload_map, tempo_rows, cycle_boundary_map) =
        build_importance_voting_fixture("right");
    let err = apply_events(
        &events,
        &idea_map,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &payload_map,
        &tempo_rows,
        &cycle_boundary_map,
    )
    .expect_err("mismatched verdict payload should fail deterministically");
    assert_eq!(err.code, "verdict_mismatch");
}

#[test]
fn approximate_timestamp_uses_uuidv7_timestamp() {
    let event_id = v7("00000000-0000-7000-8000-000000000001");
    let ts = approximate_timestamp_from_event_id(event_id).expect("timestamp");
    assert_eq!(ts, DateTime::<Utc>::from_timestamp(0, 0).unwrap());
}

#[test]
fn approximate_timestamp_rejects_non_v7_ids() {
    let event_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let err = approximate_timestamp_from_event_id(event_id).expect_err("should fail");
    assert_eq!(err.code, "invalid_id");
}
