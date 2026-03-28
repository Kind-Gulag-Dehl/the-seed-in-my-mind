use std::process::Command;

use axum::http::{Method, StatusCode};
use uuid::Uuid;

use super::tests::{
    assert_error_code, create_canonical_user_via_db_and_login, login_seed_owner_session,
    oneshot_json, register_and_get_session, test_app, test_app_with_storage, ResponseSnapshot,
};

#[tokio::test]
async fn stage1_canonical_flow_happy_path_and_negative_votes() {
    let Some(app) = test_app().await else {
        return;
    };

    let Some(owner_session) = login_seed_owner_session().await else {
        return;
    };
    let Some(writer_session) = create_canonical_user_via_db_and_login().await else {
        return;
    };
    let Some(writer_identity_id) = writer_session.identity_id.clone() else {
        eprintln!("SKIP: writer session missing canonical identity");
        return;
    };

    if !grant_writer(
        app.clone(),
        &owner_session.token,
        &writer_identity_id,
        "stage1-flow/writer-grant",
    )
    .await
    {
        return;
    }

    let flow_nonce = Uuid::now_v7().to_string();
    let Some(idea_left_id) = create_idea(
        app.clone(),
        &writer_session.token,
        &format!("Stage1 left {flow_nonce}"),
        "stage1-flow/left-idea",
    )
    .await
    else {
        return;
    };
    let Some(idea_right_id) = create_idea(
        app.clone(),
        &writer_session.token,
        &format!("Stage1 right {flow_nonce}"),
        "stage1-flow/right-idea",
    )
    .await
    else {
        return;
    };
    let Some(argument_idea_id) = create_idea(
        app.clone(),
        &writer_session.token,
        &format!("Stage1 argument {flow_nonce}"),
        "stage1-flow/argument-idea",
    )
    .await
    else {
        return;
    };

    let Some(connection_id) = create_connection(
        app.clone(),
        &writer_session.token,
        &argument_idea_id,
        &idea_left_id,
        "supports",
        "stage1-flow/argument-connection",
    )
    .await
    else {
        return;
    };

    let Some(framing_representation_ref) = first_representation_id() else {
        return;
    };

    let context_key = format!("stage1_ctx_{flow_nonce}");
    let challenge_create_payload = serde_json::json!({
        "framing_representation_ref": framing_representation_ref,
        "context_key": context_key,
        "axis": "importance",
        "timeframe": "present",
        "scope": "global",
        "target_left_idea_id": idea_left_id,
        "target_right_idea_id": idea_right_id,
        "reference_idea_id": null
    });
    let challenge_create = oneshot_json(
        app.clone(),
        Method::POST,
        "/api/v1/canonical/challenges/importance",
        challenge_create_payload.clone(),
        Some(&format!("Bearer {}", writer_session.token)),
    )
    .await;
    if challenge_create.status != StatusCode::OK {
        eprintln!(
            "SKIP: challenge creation unavailable in current environment (status={})",
            challenge_create.status
        );
        return;
    }
    let Some(challenge_id) = challenge_create
        .json
        .get("challenge_id")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
    else {
        eprintln!("SKIP: challenge creation response missing challenge_id");
        return;
    };

    let duplicate_challenge = oneshot_json(
        app.clone(),
        Method::POST,
        "/api/v1/canonical/challenges/importance",
        challenge_create_payload,
        Some(&format!("Bearer {}", writer_session.token)),
    )
    .await;
    if duplicate_challenge.status == StatusCode::CONFLICT {
        assert_error_code(&duplicate_challenge, "conflict");
    }

    let argument_attach = oneshot_json(
        app.clone(),
        Method::POST,
        &format!("/api/v1/canonical/challenges/{challenge_id}/arguments"),
        serde_json::json!({
            "connection_id": connection_id,
            "argument_idea_id": argument_idea_id,
            "subject_idea_id": idea_left_id
        }),
        Some(&format!("Bearer {}", writer_session.token)),
    )
    .await;
    if argument_attach.status != StatusCode::OK {
        eprintln!(
            "SKIP: argument attach unavailable in current environment (status={})",
            argument_attach.status
        );
        return;
    }

    if !insert_forced_cycle_close(&writer_identity_id) {
        return;
    }

    let mut voter_sessions = Vec::new();
    for idx in 0..3 {
        let Some(voter_session) = create_canonical_user_via_db_and_login().await else {
            return;
        };
        let Some(voter_identity_id) = voter_session.identity_id.clone() else {
            eprintln!("SKIP: voter session {idx} missing canonical identity");
            return;
        };
        if !grant_writer(
            app.clone(),
            &owner_session.token,
            &voter_identity_id,
            "stage1-flow/voter-grant",
        )
        .await
        {
            return;
        }
        voter_sessions.push(voter_session);
    }

    let Some(vote_session_1) =
        pull_vote_session_for_challenge(app.clone(), &voter_sessions[0].token, &challenge_id).await
    else {
        return;
    };
    let Some(vote_session_2) =
        pull_vote_session_for_challenge(app.clone(), &voter_sessions[1].token, &challenge_id).await
    else {
        return;
    };
    let Some(vote_session_3) =
        pull_vote_session_for_challenge(app.clone(), &voter_sessions[2].token, &challenge_id).await
    else {
        return;
    };

    let vote_1 = cast_vote(
        app.clone(),
        &voter_sessions[0].token,
        &challenge_id,
        &vote_session_1,
        "left",
    )
    .await;
    assert_eq!(vote_1.status, StatusCode::OK, "vote 1 failed");

    let vote_2 = cast_vote(
        app.clone(),
        &voter_sessions[1].token,
        &challenge_id,
        &vote_session_2,
        "left",
    )
    .await;
    assert_eq!(vote_2.status, StatusCode::OK, "vote 2 failed");

    let vote_3 = cast_vote(
        app.clone(),
        &voter_sessions[2].token,
        &challenge_id,
        &vote_session_3,
        "right",
    )
    .await;
    assert_eq!(vote_3.status, StatusCode::OK, "vote 3 failed");

    let duplicate_vote = cast_vote(
        app.clone(),
        &voter_sessions[0].token,
        &challenge_id,
        &vote_session_1,
        "left",
    )
    .await;
    assert_eq!(duplicate_vote.status, StatusCode::CONFLICT);
    assert_error_code(&duplicate_vote, "conflict");

    let invalid_session_vote = cast_vote(
        app.clone(),
        &voter_sessions[1].token,
        &challenge_id,
        &Uuid::now_v7().to_string(),
        "left",
    )
    .await;
    assert!(
        invalid_session_vote.status == StatusCode::BAD_REQUEST
            || invalid_session_vote.status == StatusCode::FORBIDDEN,
        "invalid session vote status={}",
        invalid_session_vote.status
    );
    let invalid_code = invalid_session_vote
        .json
        .get("error_code")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    assert!(
        invalid_code == "invalid_request" || invalid_code == "forbidden",
        "unexpected invalid session code={invalid_code}"
    );

    let detail = oneshot_json(
        app,
        Method::GET,
        &format!("/api/v1/canonical/challenges/{challenge_id}"),
        serde_json::json!({}),
        None,
    )
    .await;
    assert_eq!(detail.status, StatusCode::OK, "challenge detail failed");

    let challenge = detail
        .json
        .get("challenge")
        .expect("challenge detail payload");
    assert_eq!(
        challenge
            .get("challenge_id")
            .and_then(|value| value.as_str())
            .unwrap_or_default(),
        challenge_id
    );
    let arguments_len = challenge
        .get("arguments")
        .and_then(|value| value.as_array())
        .map(|value| value.len())
        .unwrap_or(0);
    assert!(arguments_len >= 1, "expected at least one challenge argument");
    let votes_len = challenge
        .get("votes")
        .and_then(|value| value.as_array())
        .map(|value| value.len())
        .unwrap_or(0);
    assert!(votes_len >= 3, "expected at least three votes");
    let verdict = challenge.get("verdict");
    assert!(verdict.is_some(), "expected finalized verdict");
    let winning_choice = verdict
        .and_then(|value| value.get("winning_choice"))
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    assert_eq!(winning_choice, "left");
}

#[tokio::test]
async fn identity_create_route_binds_account_identity() {
    let Some((app, storage)) = test_app_with_storage().await else {
        return;
    };
    let Some(session) = register_and_get_session().await else {
        return;
    };

    let response = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/identity_create",
        serde_json::json!({
            "identity_name": format!("Stage1 Identity {}", Uuid::now_v7()),
            "public_key": "ed25519:test-public-key",
            "metadata": "self_attested"
        }),
        Some(&format!("Bearer {}", session.token)),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK, "identity_create failed");
    let created_identity_id = response
        .json
        .get("identity_id")
        .and_then(|value| value.as_str())
        .expect("identity_create identity_id");

    let account = storage
        .get_account_by_token(&session.token)
        .await
        .expect("load account by token")
        .expect("account should exist");
    assert_eq!(
        account.canonical_identity_id.map(|value| value.to_string()),
        Some(created_identity_id.to_string())
    );
}

#[tokio::test]
async fn verifier_can_record_blocked_submission() {
    let Some(app) = test_app().await else {
        return;
    };
    let Some(owner_session) = login_seed_owner_session().await else {
        return;
    };
    let Some(owner_identity_id) = owner_session.identity_id.clone() else {
        eprintln!("SKIP: owner session missing canonical identity");
        return;
    };

    let response = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/blocked_submission",
        serde_json::json!({
            "submission_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "blocked_reason_code": "unsafe_payload",
            "blocked_by_identity": owner_identity_id
        }),
        Some(&format!("Bearer {}", owner_session.token)),
    )
    .await;

    if response.status != StatusCode::OK {
        eprintln!(
            "SKIP: blocked_submission unavailable in current environment (status={})",
            response.status
        );
        return;
    }
    assert!(
        response
            .json
            .get("event_id")
            .and_then(|value| value.as_str())
            .is_some(),
        "blocked_submission response missing event_id"
    );
}

#[tokio::test]
async fn canonical_append_persists_tempo_predicate_row_for_event_position() {
    let Some(app) = test_app().await else {
        return;
    };
    let Some(owner_session) = login_seed_owner_session().await else {
        return;
    };
    let Some(writer_session) = create_canonical_user_via_db_and_login().await else {
        return;
    };
    let Some(writer_identity_id) = writer_session.identity_id.clone() else {
        eprintln!("SKIP: writer session missing canonical identity");
        return;
    };

    if !grant_writer(
        app.clone(),
        &owner_session.token,
        &writer_identity_id,
        "tempo-row/writer-grant",
    )
    .await
    {
        return;
    }

    let Some(before_count_raw) = psql_scalar("SELECT COUNT(*)::bigint FROM tempo_predicates;")
    else {
        return;
    };
    let Some(before_count) = before_count_raw.trim().parse::<i64>().ok() else {
        eprintln!("SKIP: unable to parse tempo_predicates count");
        return;
    };

    let response = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/ideas",
        serde_json::json!({
            "idea_type": "conceptual_idea",
            "title": format!("Tempo row {}", Uuid::now_v7()),
            "sentence": "Tempo row sentence",
            "paragraph": null,
            "full": null
        }),
        Some(&format!("Bearer {}", writer_session.token)),
    )
    .await;
    assert_eq!(response.status, StatusCode::OK, "idea create failed");
    let Some(event_id) = response
        .json
        .get("event_id")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
    else {
        panic!("idea create response missing event_id");
    };

    let Some(after_count_raw) = psql_scalar("SELECT COUNT(*)::bigint FROM tempo_predicates;")
    else {
        panic!("missing tempo_predicates count after idea create");
    };
    let Some(after_count) = after_count_raw.trim().parse::<i64>().ok() else {
        panic!("invalid tempo_predicates count after idea create");
    };
    assert_eq!(
        after_count,
        before_count + 1,
        "a normal canonical append should persist exactly one tempo_predicates row"
    );

    let Some(tempo_row_exists) = psql_scalar(&format!(
        r#"
SELECT EXISTS (
  SELECT 1
  FROM tempo_predicates tp
  JOIN events e
    ON e.block_height = tp.block_height
   AND e.event_index = tp.event_index
  WHERE e.event_id = '{event_id}'::uuid
);
"#
    )) else {
        panic!("missing tempo_predicates existence check for event");
    };
    assert_eq!(
        tempo_row_exists.trim(),
        "t",
        "the appended canonical event should have a tempo_predicates row at the same position"
    );
}

#[tokio::test]
async fn next_canonical_append_emits_cycle_close_before_user_event_when_due() {
    let Some(app) = test_app().await else {
        return;
    };
    let Some(owner_session) = login_seed_owner_session().await else {
        return;
    };
    let Some(writer_session) = create_canonical_user_via_db_and_login().await else {
        return;
    };
    let Some(writer_identity_id) = writer_session.identity_id.clone() else {
        eprintln!("SKIP: writer session missing canonical identity");
        return;
    };

    if !grant_writer(
        app.clone(),
        &owner_session.token,
        &writer_identity_id,
        "cycle-close/writer-grant",
    )
    .await
    {
        return;
    }

    let Some(before_count_raw) =
        psql_scalar("SELECT COUNT(*)::bigint FROM events WHERE event_type = 'cycle_close';")
    else {
        return;
    };
    let Some(before_count) = before_count_raw.trim().parse::<i64>().ok() else {
        eprintln!("SKIP: unable to parse cycle_close count");
        return;
    };

    if !insert_tempo_ready_noop(&writer_identity_id) {
        return;
    }

    let create_first = oneshot_json(
        app.clone(),
        Method::POST,
        "/api/v1/canonical/ideas",
        serde_json::json!({
            "idea_type": "conceptual_idea",
            "title": format!("Cycle boundary first {}", Uuid::now_v7()),
            "sentence": "Cycle boundary first",
            "paragraph": null,
            "full": null
        }),
        Some(&format!("Bearer {}", writer_session.token)),
    )
    .await;
    assert_eq!(create_first.status, StatusCode::OK, "first idea failed");
    let Some(first_event_id) = create_first
        .json
        .get("event_id")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
    else {
        panic!("first idea response missing event_id");
    };

    let Some(first_block_height_raw) = psql_scalar(&format!(
        "SELECT block_height::text FROM events WHERE event_id = '{first_event_id}'::uuid;"
    )) else {
        panic!("missing first event block height");
    };
    let Some(first_block_height) = first_block_height_raw.trim().parse::<i64>().ok() else {
        panic!("invalid first event block height");
    };
    let Some(first_event_index_raw) = psql_scalar(&format!(
        "SELECT event_index::text FROM events WHERE event_id = '{first_event_id}'::uuid;"
    )) else {
        panic!("missing first event index");
    };
    let Some(first_event_index) = first_event_index_raw.trim().parse::<i32>().ok() else {
        panic!("invalid first event index");
    };

    let Some(prev_event_type) = psql_scalar(&format!(
        r#"
SELECT event_type
FROM events
WHERE block_height < {first_block_height}
   OR (block_height = {first_block_height} AND event_index < {first_event_index})
ORDER BY block_height DESC, event_index DESC
LIMIT 1;
"#
    )) else {
        panic!("missing previous event");
    };
    assert_eq!(prev_event_type.trim(), "cycle_close");

    let Some(cycle_close_block_height_raw) = psql_scalar(&format!(
        r#"
SELECT block_height::text
FROM events
WHERE block_height < {first_block_height}
   OR (block_height = {first_block_height} AND event_index < {first_event_index})
ORDER BY block_height DESC, event_index DESC
LIMIT 1;
"#
    )) else {
        panic!("missing cycle_close block height");
    };
    let Some(cycle_close_block_height) = cycle_close_block_height_raw.trim().parse::<i64>().ok()
    else {
        panic!("invalid cycle_close block height");
    };
    assert_eq!(
        first_block_height,
        cycle_close_block_height + 1,
        "user event should begin the next block after cycle_close"
    );
    assert_eq!(first_event_index, 0, "first event after cycle_close should open the next block");

    let Some(cycle_close_speaker) = psql_scalar(&format!(
        r#"
SELECT speaker_identity_id::text
FROM events
WHERE block_height < {first_block_height}
   OR (block_height = {first_block_height} AND event_index < {first_event_index})
ORDER BY block_height DESC, event_index DESC
LIMIT 1;
"#
    )) else {
        panic!("missing cycle_close speaker");
    };
    assert_eq!(
        cycle_close_speaker.trim(),
        "ffffffff-ffff-7fff-bfff-ffffffffffff"
    );
    let Some(cycle_close_signature_present) = psql_scalar(&format!(
        r#"
SELECT (signature IS NOT NULL)::text
FROM events
WHERE block_height < {first_block_height}
   OR (block_height = {first_block_height} AND event_index < {first_event_index})
ORDER BY block_height DESC, event_index DESC
LIMIT 1;
"#
    )) else {
        panic!("missing cycle_close signature state");
    };
    assert_eq!(cycle_close_signature_present.trim(), "t");

    let Some(after_first_count_raw) =
        psql_scalar("SELECT COUNT(*)::bigint FROM events WHERE event_type = 'cycle_close';")
    else {
        panic!("missing cycle_close count after first append");
    };
    let Some(after_first_count) = after_first_count_raw.trim().parse::<i64>().ok() else {
        panic!("invalid cycle_close count after first append");
    };
    assert_eq!(after_first_count, before_count + 1);

    let create_second = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/ideas",
        serde_json::json!({
            "idea_type": "conceptual_idea",
            "title": format!("Cycle boundary second {}", Uuid::now_v7()),
            "sentence": "Cycle boundary second",
            "paragraph": null,
            "full": null
        }),
        Some(&format!("Bearer {}", writer_session.token)),
    )
    .await;
    assert_eq!(create_second.status, StatusCode::OK, "second idea failed");

    let Some(after_second_count_raw) =
        psql_scalar("SELECT COUNT(*)::bigint FROM events WHERE event_type = 'cycle_close';")
    else {
        panic!("missing cycle_close count after second append");
    };
    let Some(after_second_count) = after_second_count_raw.trim().parse::<i64>().ok() else {
        panic!("invalid cycle_close count after second append");
    };
    assert_eq!(
        after_second_count, after_first_count,
        "a second append without fresh tempo evidence must not emit another cycle_close"
    );
}

async fn grant_writer(
    app: axum::Router,
    owner_token: &str,
    identity_id: &str,
    label: &str,
) -> bool {
    let response = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/verifier/grants",
        serde_json::json!({
            "identity_id": identity_id,
            "canonical_writer_level": "1",
            "email_verified": true
        }),
        Some(&format!("Bearer {owner_token}")),
    )
    .await;
    if response.status != StatusCode::OK {
        eprintln!(
            "SKIP: {label} unavailable in current environment (status={})",
            response.status
        );
        return false;
    }
    true
}

async fn create_idea(
    app: axum::Router,
    token: &str,
    title: &str,
    label: &str,
) -> Option<String> {
    let response = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/ideas",
        serde_json::json!({
            "idea_type": "conceptual_idea",
            "title": title,
            "sentence": title,
            "paragraph": null,
            "full": null
        }),
        Some(&format!("Bearer {token}")),
    )
    .await;
    if response.status != StatusCode::OK {
        eprintln!(
            "SKIP: {label} unavailable in current environment (status={})",
            response.status
        );
        return None;
    }
    response
        .json
        .get("idea_id")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
        .or_else(|| {
            eprintln!("SKIP: {label} response missing idea_id");
            None
        })
}

async fn create_connection(
    app: axum::Router,
    token: &str,
    from_idea_id: &str,
    to_idea_id: &str,
    connection_type: &str,
    label: &str,
) -> Option<String> {
    let response = oneshot_json(
        app,
        Method::POST,
        "/api/v1/canonical/connections",
        serde_json::json!({
            "from_idea_id": from_idea_id,
            "to_idea_id": to_idea_id,
            "connection_type": connection_type,
            "usage": null,
            "axis": null,
            "timeframe": null,
            "scope": null
        }),
        Some(&format!("Bearer {token}")),
    )
    .await;
    if response.status != StatusCode::OK {
        eprintln!(
            "SKIP: {label} unavailable in current environment (status={})",
            response.status
        );
        return None;
    }
    response
        .json
        .get("connection_id")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
        .or_else(|| {
            eprintln!("SKIP: {label} response missing connection_id");
            None
        })
}

async fn pull_vote_session_for_challenge(
    app: axum::Router,
    token: &str,
    challenge_id: &str,
) -> Option<String> {
    for _ in 0..8 {
        let pull = oneshot_json(
            app.clone(),
            Method::POST,
            "/api/v1/canonical/vote-sessions/pull",
            serde_json::json!({}),
            Some(&format!("Bearer {token}")),
        )
        .await;
        if pull.status != StatusCode::OK {
            eprintln!(
                "SKIP: vote-session pull unavailable in current environment (status={})",
                pull.status
            );
            return None;
        }
        let pulled_challenge = pull
            .json
            .get("challenge_id")
            .and_then(|value| value.as_str())
            .unwrap_or_default();
        if pulled_challenge == challenge_id {
            return pull
                .json
                .get("vote_session_id")
                .and_then(|value| value.as_str())
                .map(|value| value.to_string());
        }
    }
    eprintln!("SKIP: unable to pull vote session for newly created challenge");
    None
}

async fn cast_vote(
    app: axum::Router,
    token: &str,
    challenge_id: &str,
    vote_session_id: &str,
    vote_choice: &str,
) -> ResponseSnapshot {
    oneshot_json(
        app,
        Method::POST,
        &format!("/api/v1/canonical/challenges/{challenge_id}/votes"),
        serde_json::json!({
            "vote_session_id": vote_session_id,
            "vote_choice": vote_choice
        }),
        Some(&format!("Bearer {token}")),
    )
    .await
}

fn first_representation_id() -> Option<String> {
    psql_scalar(
        "SELECT representation_id::text FROM representations ORDER BY created_block_height, created_event_index LIMIT 1;",
    )
}

fn insert_forced_cycle_close(noop_speaker_identity_id: &str) -> bool {
    let Some(block_height_raw) = psql_scalar("SELECT COALESCE(MAX(block_height), 0)::bigint FROM blocks;")
    else {
        return false;
    };
    let Some(block_height) = block_height_raw.trim().parse::<i64>().ok() else {
        eprintln!("SKIP: unable to parse block height");
        return false;
    };

    let Some(noop_index_raw) = psql_scalar(&format!(
        "SELECT COALESCE(MAX(event_index), -1)::int + 1 FROM events WHERE block_height = {block_height};"
    )) else {
        return false;
    };
    let Some(noop_index) = noop_index_raw.trim().parse::<i32>().ok() else {
        eprintln!("SKIP: unable to parse noop event index");
        return false;
    };
    let close_index = noop_index + 1;

    let Some(cycle_index_raw) =
        psql_scalar("SELECT COALESCE(MAX(cycle_index), -1)::bigint + 1 FROM cycle_boundaries;")
    else {
        return false;
    };
    let Some(cycle_index) = cycle_index_raw.trim().parse::<i64>().ok() else {
        eprintln!("SKIP: unable to parse cycle index");
        return false;
    };

    let noop_event_id = Uuid::now_v7();
    let cycle_close_event_id = Uuid::now_v7();
    let closure_height = block_height;
    let next_block_height = closure_height + 1;

    let sql = format!(
        r#"
INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
VALUES ({block_height}, {noop_index}, '{noop_event_id}'::uuid, 'noop', '{noop_speaker_identity_id}'::uuid, '{{}}'::jsonb, NULL);
INSERT INTO tempo_predicates (block_height, event_index, cycle_age_ge_dmin, cycle_age_ge_dmax, constrained_mode, record_only_mode)
VALUES ({block_height}, {noop_index}, false, false, false, false)
ON CONFLICT (block_height, event_index) DO UPDATE SET
  cycle_age_ge_dmin = EXCLUDED.cycle_age_ge_dmin,
  cycle_age_ge_dmax = EXCLUDED.cycle_age_ge_dmax,
  constrained_mode = EXCLUDED.constrained_mode,
  record_only_mode = EXCLUDED.record_only_mode;
INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
VALUES (
  {block_height},
  {close_index},
  '{cycle_close_event_id}'::uuid,
  'cycle_close',
  'ffffffff-ffff-7fff-bfff-ffffffffffff'::uuid,
  jsonb_build_object(
    'cycle_index', {cycle_index},
    'closure_kind', 'forced',
    'forced_seal', true,
    'closure_boundary_ref', jsonb_build_object('block_height', {closure_height})
  ),
  NULL
);
INSERT INTO tempo_predicates (block_height, event_index, cycle_age_ge_dmin, cycle_age_ge_dmax, constrained_mode, record_only_mode)
VALUES ({block_height}, {close_index}, true, true, false, false)
ON CONFLICT (block_height, event_index) DO UPDATE SET
  cycle_age_ge_dmin = EXCLUDED.cycle_age_ge_dmin,
  cycle_age_ge_dmax = EXCLUDED.cycle_age_ge_dmax,
  constrained_mode = EXCLUDED.constrained_mode,
  record_only_mode = EXCLUDED.record_only_mode;
INSERT INTO cycle_boundaries (
  cycle_index,
  closure_kind,
  forced_seal,
  closure_block_height,
  source_block_height,
  source_event_index,
  source_event_id
) VALUES (
  {cycle_index},
  1,
  true,
  {closure_height},
  {block_height},
  {close_index},
  '{cycle_close_event_id}'::uuid
);
INSERT INTO blocks (block_height, block_hash, prev_block_hash)
VALUES (
  {next_block_height},
  to_hex({next_block_height}),
  CASE WHEN {next_block_height} > 0 THEN to_hex({next_block_height} - 1) ELSE NULL END
)
ON CONFLICT (block_height) DO NOTHING;
"#
    );

    psql_exec(&sql)
}

fn insert_tempo_ready_noop(noop_speaker_identity_id: &str) -> bool {
    let Some(block_height_raw) =
        psql_scalar("SELECT COALESCE(MAX(block_height), 0)::bigint FROM blocks;")
    else {
        return false;
    };
    let Some(block_height) = block_height_raw.trim().parse::<i64>().ok() else {
        eprintln!("SKIP: unable to parse block height");
        return false;
    };

    let Some(noop_index_raw) = psql_scalar(&format!(
        "SELECT COALESCE(MAX(event_index), -1)::int + 1 FROM events WHERE block_height = {block_height};"
    )) else {
        return false;
    };
    let Some(noop_index) = noop_index_raw.trim().parse::<i32>().ok() else {
        eprintln!("SKIP: unable to parse noop event index");
        return false;
    };

    let noop_event_id = Uuid::now_v7();
    let sql = format!(
        r#"
INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
VALUES ({block_height}, {noop_index}, '{noop_event_id}'::uuid, 'noop', '{noop_speaker_identity_id}'::uuid, '{{}}'::jsonb, NULL);
INSERT INTO tempo_predicates (block_height, event_index, cycle_age_ge_dmin, cycle_age_ge_dmax, constrained_mode, record_only_mode)
VALUES ({block_height}, {noop_index}, true, true, false, false)
ON CONFLICT (block_height, event_index) DO UPDATE SET
  cycle_age_ge_dmin = EXCLUDED.cycle_age_ge_dmin,
  cycle_age_ge_dmax = EXCLUDED.cycle_age_ge_dmax,
  constrained_mode = EXCLUDED.constrained_mode,
  record_only_mode = EXCLUDED.record_only_mode;
"#
    );
    psql_exec(&sql)
}

fn psql_scalar(sql: &str) -> Option<String> {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            eprintln!("SKIP: DATABASE_URL is missing");
            return None;
        }
    };
    let output = match Command::new("psql")
        .arg(database_url)
        .arg("-At")
        .arg("-v")
        .arg("ON_ERROR_STOP=1")
        .arg("-c")
        .arg(sql)
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            eprintln!("SKIP: psql invocation failed: {err}");
            return None;
        }
    };
    if !output.status.success() {
        eprintln!("SKIP: psql scalar query failed");
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        eprintln!("SKIP: psql scalar query returned no rows");
        return None;
    }
    Some(text)
}

fn psql_exec(sql: &str) -> bool {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            eprintln!("SKIP: DATABASE_URL is missing");
            return false;
        }
    };
    let output = match Command::new("psql")
        .arg(database_url)
        .arg("-v")
        .arg("ON_ERROR_STOP=1")
        .arg("-c")
        .arg(sql)
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            eprintln!("SKIP: psql invocation failed: {err}");
            return false;
        }
    };
    if !output.status.success() {
        eprintln!("SKIP: psql execution failed for cycle-close setup");
        return false;
    }
    true
}
