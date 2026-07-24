use crate::db::{
    advisory_lock_key_for_uuid, allocate_canonical_event_position, canonical_storage_error,
    map_canonical_write_sqlx_error, EventInsertPosition,
};
use crate::{
    CanonicalWriteError, ProfileV0IdentityAdmissionResult, SignedCanonicalCandidateInput, Storage,
};
use encoding::payload::to_hex;
use event_log::profile_v0_admission::{
    parse_profile_v0_identity_create_payload, reject_ordinary_identity_verification_update,
    validate_profile_v0_identity_create_candidate, ProfileV0AdmissionPureState,
    ProfileV0AdmissionValidationError, ProfileV0IdentityCreatePayload,
};
use sqlx::{FromRow, Postgres, Transaction};
use uuid::Uuid;
use verification::admission::IdentityStructuralRootRoleV0;
use verification::signatures::{
    authored_candidate_hash_v0, decode_signature64, public_key_ref_v0, signed_candidate_bytes_v0,
    AuthoredEventCandidate, PAYLOAD_BINDING_EMBEDDED,
};

const PROFILE_V0_PUBLICATION_PROFILE: &str = "sponsored_public_admission_v0";
const STRUCTURAL_ROOT_SCOPE: &str = "identity_structural_roots_v0";
const CAPACITY_DEBIT_UNITS: i64 = 1;

#[derive(Debug, Clone, FromRow)]
struct ExistingAdmissionEventRow {
    block_height: i64,
    event_index: i32,
    event_type: String,
    authored_candidate_hash_v0: Option<String>,
    signature: Option<String>,
    payload_json: serde_json::Value,
}

#[derive(Debug, Clone, FromRow)]
struct SponsorKeyRow {
    public_key_ref: String,
    public_key_bytes: Vec<u8>,
    is_active: bool,
}

#[derive(Debug, Clone, FromRow)]
struct SponsorMaterializationRow {
    identity_kind: String,
    admission_profile_version: String,
    capacity_period_id: Uuid,
    rulebook_id: Uuid,
    rulebook_version: String,
    rulebook_hash: Vec<u8>,
    inviter_eligible: bool,
    invitation_suspended: bool,
    spendable_capacity: i64,
}

fn admission_error(error: ProfileV0AdmissionValidationError) -> CanonicalWriteError {
    CanonicalWriteError::new(error.code, error.message)
}

fn authored_candidate(input: &SignedCanonicalCandidateInput) -> AuthoredEventCandidate {
    AuthoredEventCandidate {
        signature_profile: input.signature_profile.clone(),
        event_id: input.event_id,
        event_type: input.event_type.clone(),
        author_identity_id: input.author_identity_id,
        speaker_identity_id: input.speaker_identity_id,
        public_key_ref: input.public_key_ref.clone(),
        payload_hash: input.payload_hash.clone(),
        payload_binding_mode: input.payload_binding_mode.clone(),
        payload_ref: input.payload_ref.clone(),
        author_observed_at: input.author_observed_at.clone(),
        signature: input.signature.clone(),
    }
}

fn candidate_hash(
    input: &SignedCanonicalCandidateInput,
) -> Result<(AuthoredEventCandidate, Vec<u8>, String), CanonicalWriteError> {
    let candidate = authored_candidate(input);
    let signed_bytes = signed_candidate_bytes_v0(&candidate)
        .map_err(|error| CanonicalWriteError::new(error.code, error.message))?;
    let signature = decode_signature64(&candidate.signature)
        .map_err(|error| CanonicalWriteError::new(error.code, error.message))?;
    let hash = authored_candidate_hash_v0(&signed_bytes, &signature)
        .map_err(|error| CanonicalWriteError::new(error.code, error.message))?;
    Ok((candidate, signed_bytes, hash))
}

fn root_title(role: IdentityStructuralRootRoleV0) -> &'static str {
    match role {
        IdentityStructuralRootRoleV0::Mindgarden => "Mindgarden",
        IdentityStructuralRootRoleV0::BackyardOfRelationships => "Backyard of Relationships",
        IdentityStructuralRootRoleV0::SelfTree => "Self Tree",
        IdentityStructuralRootRoleV0::Anthill => "Anthill",
    }
}

fn containment_role(index: usize) -> &'static str {
    match index {
        0 => "mindgarden_contains_backyard_of_relationships",
        1 => "mindgarden_contains_self_tree",
        2 => "mindgarden_contains_anthill",
        _ => unreachable!("Profile-v0 has exactly three root memberships"),
    }
}

fn storage_identity_title(identity_id: Uuid) -> String {
    // `identities_s0.title` is a legacy non-null display field. Profile-v0 identity
    // semantics deliberately do not accept an applicant-provided title in admission.
    format!("CanonicalAdmittedIdentity {identity_id}")
}

fn hash32_bytes(value: [u8; 32]) -> Vec<u8> {
    value.to_vec()
}

fn key_ref_lock_key(value: [u8; 32]) -> i64 {
    let mut upper = [0_u8; 8];
    upper.copy_from_slice(&value[..8]);
    let mut lower = [0_u8; 8];
    lower.copy_from_slice(&value[8..16]);
    i64::from_be_bytes(upper) ^ i64::from_be_bytes(lower)
}

async fn lock_uuid(
    tx: &mut Transaction<'_, Postgres>,
    value: Uuid,
) -> Result<(), CanonicalWriteError> {
    sqlx::query("SELECT pg_advisory_xact_lock($1::bigint)")
        .bind(advisory_lock_key_for_uuid(value))
        .execute(&mut **tx)
        .await
        .map_err(canonical_storage_error)?;
    Ok(())
}

async fn lock_key_ref(
    tx: &mut Transaction<'_, Postgres>,
    value: [u8; 32],
) -> Result<(), CanonicalWriteError> {
    sqlx::query("SELECT pg_advisory_xact_lock($1::bigint)")
        .bind(key_ref_lock_key(value))
        .execute(&mut **tx)
        .await
        .map_err(canonical_storage_error)?;
    Ok(())
}

async fn load_existing_admission_event(
    tx: &mut Transaction<'_, Postgres>,
    event_id: Uuid,
) -> Result<Option<ExistingAdmissionEventRow>, CanonicalWriteError> {
    sqlx::query_as::<_, ExistingAdmissionEventRow>(
        r#"
        SELECT
          block_height,
          event_index,
          event_type,
          authored_candidate_hash_v0,
          signature,
          payload_json
        FROM events
        WHERE event_id = $1
        FOR UPDATE
        "#,
    )
    .bind(event_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(canonical_storage_error)
}

async fn load_sponsor_key_before_position(
    tx: &mut Transaction<'_, Postgres>,
    input: &SignedCanonicalCandidateInput,
    position: EventInsertPosition,
) -> Result<SponsorKeyRow, CanonicalWriteError> {
    let row = sqlx::query_as::<_, SponsorKeyRow>(
        r#"
        SELECT public_key_ref, public_key_bytes, is_active
        FROM canonical_identity_key_states
        WHERE public_key_ref = $1
          AND identity_id = $2
          AND signature_profile = $3
          AND signature_algorithm = 'ed25519'
          AND (
            source_block_height IS NULL
            OR source_block_height < $4
            OR (source_block_height = $4 AND source_event_index < $5)
          )
        ORDER BY source_block_height DESC NULLS LAST,
                 source_event_index DESC NULLS LAST,
                 created_at DESC
        LIMIT 1
        FOR UPDATE
        "#,
    )
    .bind(&input.public_key_ref)
    .bind(input.author_identity_id)
    .bind(&input.signature_profile)
    .bind(position.block_height)
    .bind(position.event_index)
    .fetch_optional(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;

    let Some(row) = row else {
        return Err(CanonicalWriteError::new(
            "author_key_inactive",
            "sponsor author key is not active at the admission application position",
        ));
    };

    let expected_ref = public_key_ref_v0(&row.public_key_bytes, input.author_identity_id)
        .map_err(|error| CanonicalWriteError::new(error.code, error.message))?;
    if row.public_key_ref != expected_ref || row.public_key_ref != input.public_key_ref {
        return Err(CanonicalWriteError::new(
            "author_key_inactive",
            "sponsor key descriptor does not match the candidate public_key_ref",
        ));
    }

    let profile_key_state: Option<String> = sqlx::query_scalar(
        r#"
        SELECT key_state
        FROM canonical_profile_v0_direct_key_state_history
        WHERE public_key_ref = decode($1, 'hex')
          AND (
            source_block_height < $2
            OR (source_block_height = $2 AND source_event_index < $3)
          )
        ORDER BY source_block_height DESC, source_event_index DESC
        LIMIT 1
        "#,
    )
    .bind(&input.public_key_ref)
    .bind(position.block_height)
    .bind(position.event_index)
    .fetch_optional(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;

    match profile_key_state.as_deref() {
        Some("revoked") => Err(CanonicalWriteError::new(
            "author_key_revoked",
            "sponsor author key is revoked at the admission application position",
        )),
        Some("superseded") => Err(CanonicalWriteError::new(
            "key_already_superseded",
            "sponsor author key is superseded at the admission application position",
        )),
        Some("invalid") => Err(CanonicalWriteError::new(
            "author_key_inactive",
            "sponsor author key is invalid at the admission application position",
        )),
        _ if !row.is_active => Err(CanonicalWriteError::new(
            "author_key_inactive",
            "sponsor author key is inactive at the admission application position",
        )),
        _ => Ok(row),
    }
}

async fn ensure_sponsor_is_human(
    tx: &mut Transaction<'_, Postgres>,
    sponsor_identity_id: Uuid,
    position: EventInsertPosition,
) -> Result<(), CanonicalWriteError> {
    let identity_exists: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM identities_s0 WHERE identity_id = $1)")
            .bind(sponsor_identity_id)
            .fetch_one(&mut **tx)
            .await
            .map_err(canonical_storage_error)?;
    if !identity_exists {
        return Err(CanonicalWriteError::new(
            "sponsor_not_human",
            "identity_create sponsor does not exist as a canonical human identity",
        ));
    }

    let kind: Option<String> = sqlx::query_scalar(
        r#"
        SELECT identity_kind
        FROM canonical_identity_provenance_v0
        WHERE identity_id = $1
          AND (
            source_block_height IS NULL
            OR source_block_height < $2
            OR (source_block_height = $2 AND source_event_index < $3)
          )
        "#,
    )
    .bind(sponsor_identity_id)
    .bind(position.block_height)
    .bind(position.event_index)
    .fetch_optional(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    if kind.as_deref() != Some("human") {
        return Err(CanonicalWriteError::new(
            "sponsor_not_human",
            "identity_create sponsor lacks the required human identity-kind classification",
        ));
    }
    Ok(())
}

async fn build_pure_state(
    tx: &mut Transaction<'_, Postgres>,
    payload: &ProfileV0IdentityCreatePayload,
) -> Result<ProfileV0AdmissionPureState, CanonicalWriteError> {
    let mut state = ProfileV0AdmissionPureState::default();
    let identity_exists: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM identities_s0 WHERE identity_id = $1)")
            .bind(payload.identity_id)
            .fetch_one(&mut **tx)
            .await
            .map_err(canonical_storage_error)?;
    if identity_exists {
        state.known_identity_ids.insert(payload.identity_id);
    }

    let key_ref_hex = to_hex(&payload.initial_public_key_ref);
    let key_registered: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
          SELECT 1
          FROM canonical_identity_key_states
          WHERE public_key_ref = $1
             OR public_key_bytes = $2
          UNION ALL
          SELECT 1
          FROM canonical_profile_v0_direct_key_history
          WHERE public_key_ref = $3
             OR raw_public_key = $2
        )
        "#,
    )
    .bind(&key_ref_hex)
    .bind(&payload.initial_key_descriptor.raw_public_key_bytes)
    .bind(hash32_bytes(payload.initial_public_key_ref))
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)?;
    if key_registered {
        state
            .historically_registered_key_refs
            .insert(payload.initial_public_key_ref);
        let mut raw = [0_u8; 32];
        raw.copy_from_slice(&payload.initial_key_descriptor.raw_public_key_bytes);
        state.historically_registered_public_keys.insert(raw);
    }

    for root in &payload.identity_structural_root_plan.roots {
        let occupied: bool =
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM ideas WHERE idea_id = $1)")
                .bind(root.idea_id)
                .fetch_one(&mut **tx)
                .await
                .map_err(canonical_storage_error)?;
        if occupied {
            state.occupied_root_idea_ids.insert(root.idea_id);
        }
    }
    for connection_id in &payload
        .identity_structural_root_plan
        .membership_connection_ids
    {
        let occupied: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM connections WHERE connection_id = $1)",
        )
        .bind(connection_id)
        .fetch_one(&mut **tx)
        .await
        .map_err(canonical_storage_error)?;
        if occupied {
            state.occupied_root_connection_ids.insert(*connection_id);
        }
    }
    Ok(state)
}

async fn load_admission_materialization(
    tx: &mut Transaction<'_, Postgres>,
    sponsor_identity_id: Uuid,
    position: EventInsertPosition,
) -> Result<SponsorMaterializationRow, CanonicalWriteError> {
    sqlx::query_as::<_, SponsorMaterializationRow>(
        r#"
        SELECT
          identity_kind,
          admission_profile_version,
          capacity_period_id,
          rulebook_id,
          rulebook_version,
          rulebook_hash,
          inviter_eligible,
          invitation_suspended,
          spendable_capacity
        FROM profile_v0_admission_state_materializations
        WHERE identity_id = $1
          AND (
            source_block_height < $2
            OR (source_block_height = $2 AND source_event_index < $3)
          )
        ORDER BY source_block_height DESC, source_event_index DESC
        LIMIT 1
        FOR UPDATE
        "#,
    )
    .bind(sponsor_identity_id)
    .bind(position.block_height)
    .bind(position.event_index)
    .fetch_optional(&mut **tx)
    .await
    .map_err(canonical_storage_error)?
    .ok_or_else(|| {
        CanonicalWriteError::new(
            "stale_admission_authorization",
            "no active replay-replaceable Profile-v0 admission context exists for the sponsor",
        )
    })
}

fn rulebook_matches(
    materialization: &SponsorMaterializationRow,
    payload: &ProfileV0IdentityCreatePayload,
) -> bool {
    materialization.admission_profile_version == payload.admission_profile_version
        && materialization.capacity_period_id == payload.capacity_period_id
        && materialization.rulebook_id == payload.rulebook_reference.rulebook_id
        && materialization.rulebook_version == payload.rulebook_reference.rulebook_version
        && materialization.rulebook_hash == payload.rulebook_reference.rulebook_hash
}

async fn spent_capacity_units(
    tx: &mut Transaction<'_, Postgres>,
    sponsor_identity_id: Uuid,
    payload: &ProfileV0IdentityCreatePayload,
) -> Result<i64, CanonicalWriteError> {
    sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(debit_units), 0)::bigint
        FROM profile_v0_invitation_capacity_debits
        WHERE sponsor_identity_id = $1
          AND capacity_period_id = $2
          AND rulebook_id = $3
          AND rulebook_version = $4
          AND rulebook_hash = $5
        "#,
    )
    .bind(sponsor_identity_id)
    .bind(payload.capacity_period_id)
    .bind(payload.rulebook_reference.rulebook_id)
    .bind(&payload.rulebook_reference.rulebook_version)
    .bind(hash32_bytes(payload.rulebook_reference.rulebook_hash))
    .fetch_one(&mut **tx)
    .await
    .map_err(canonical_storage_error)
}

async fn validate_admission_context(
    tx: &mut Transaction<'_, Postgres>,
    sponsor_identity_id: Uuid,
    payload: &ProfileV0IdentityCreatePayload,
    position: EventInsertPosition,
) -> Result<(), CanonicalWriteError> {
    let materialization = load_admission_materialization(tx, sponsor_identity_id, position).await?;
    if materialization.identity_kind != "human" || !rulebook_matches(&materialization, payload) {
        return Err(CanonicalWriteError::new(
            "stale_admission_authorization",
            "admission profile, capacity period, or rulebook is no longer applicable",
        ));
    }
    if !materialization.inviter_eligible {
        return Err(CanonicalWriteError::new(
            "inviter_ineligible",
            "sponsor is not inviter-eligible at the admission application position",
        ));
    }
    if materialization.invitation_suspended {
        return Err(CanonicalWriteError::new(
            "inviter_suspended",
            "sponsor invitation authority is suspended at the admission application position",
        ));
    }
    let spent = spent_capacity_units(tx, sponsor_identity_id, payload).await?;
    if materialization.spendable_capacity.saturating_sub(spent) < CAPACITY_DEBIT_UNITS {
        return Err(CanonicalWriteError::new(
            "insufficient_invitation_capacity",
            "sponsor has fewer than one spendable invitation-capacity unit",
        ));
    }
    Ok(())
}

async fn insert_profile_v0_event(
    tx: &mut Transaction<'_, Postgres>,
    input: &SignedCanonicalCandidateInput,
    position: EventInsertPosition,
    signed_candidate_bytes: &[u8],
    candidate_hash: &str,
) -> Result<(), CanonicalWriteError> {
    sqlx::query(
        r#"
        INSERT INTO events (
          block_height,
          event_index,
          event_id,
          event_type,
          speaker_identity_id,
          payload_json,
          signature,
          signature_profile,
          author_identity_id,
          public_key_ref,
          payload_hash,
          payload_binding_mode,
          payload_ref,
          author_observed_at,
          signed_candidate_bytes_v0,
          authored_candidate_hash_v0,
          publication_profile
        ) VALUES (
          $1, $2, $3, 'identity_create', NULL, $4, $5, $6, $7, $8, $9,
          $10, NULL, $11, $12, $13, $14
        )
        "#,
    )
    .bind(position.block_height)
    .bind(position.event_index)
    .bind(input.event_id)
    .bind(input.payload.clone())
    .bind(&input.signature)
    .bind(&input.signature_profile)
    .bind(input.author_identity_id)
    .bind(&input.public_key_ref)
    .bind(&input.payload_hash)
    .bind(&input.payload_binding_mode)
    .bind(input.author_observed_at.as_deref())
    .bind(signed_candidate_bytes)
    .bind(candidate_hash)
    .bind(PROFILE_V0_PUBLICATION_PROFILE)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;
    Ok(())
}

async fn persist_profile_v0_admission(
    tx: &mut Transaction<'_, Postgres>,
    input: &SignedCanonicalCandidateInput,
    payload: &ProfileV0IdentityCreatePayload,
    position: EventInsertPosition,
) -> Result<(), CanonicalWriteError> {
    sqlx::query(
        r#"
        INSERT INTO identities_s0 (identity_id, title, created_event_id)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(payload.identity_id)
    .bind(storage_identity_title(payload.identity_id))
    .bind(input.event_id)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;

    sqlx::query(
        r#"
        INSERT INTO canonical_identity_provenance_v0 (
          identity_id, identity_kind, provenance_class, source_event_id,
          source_block_height, source_event_index
        ) VALUES ($1, 'human', 'event_derived', $2, $3, $4)
        "#,
    )
    .bind(payload.identity_id)
    .bind(input.event_id)
    .bind(position.block_height)
    .bind(position.event_index)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;

    sqlx::query(
        r#"
        INSERT INTO profile_v0_identity_admissions (
          identity_id, admission_event_id, sponsor_identity_id, admission_profile_version,
          capacity_period_id, rulebook_id, rulebook_version, rulebook_hash,
          verification_reference, created_block_height, created_event_index
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(payload.identity_id)
    .bind(input.event_id)
    .bind(input.author_identity_id)
    .bind(&payload.admission_profile_version)
    .bind(payload.capacity_period_id)
    .bind(payload.rulebook_reference.rulebook_id)
    .bind(&payload.rulebook_reference.rulebook_version)
    .bind(hash32_bytes(payload.rulebook_reference.rulebook_hash))
    .bind(payload.verification_reference.map(hash32_bytes))
    .bind(position.block_height)
    .bind(position.event_index)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;

    let initial_key_ref_hex = to_hex(&payload.initial_public_key_ref);
    sqlx::query(
        r#"
        INSERT INTO canonical_profile_v0_direct_key_history (
          public_key_ref, raw_public_key, identity_id, source_event_id,
          source_block_height, source_event_index
        ) VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(hash32_bytes(payload.initial_public_key_ref))
    .bind(&payload.initial_key_descriptor.raw_public_key_bytes)
    .bind(payload.identity_id)
    .bind(input.event_id)
    .bind(position.block_height)
    .bind(position.event_index)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;

    sqlx::query(
        r#"
        INSERT INTO canonical_profile_v0_direct_key_state_history (
          state_record_id, public_key_ref, key_state, source_event_id,
          source_block_height, source_event_index
        ) VALUES ($1, $2, 'active', $3, $4, $5)
        "#,
    )
    .bind(Uuid::now_v7())
    .bind(hash32_bytes(payload.initial_public_key_ref))
    .bind(input.event_id)
    .bind(position.block_height)
    .bind(position.event_index)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;

    sqlx::query(
        r#"
        INSERT INTO canonical_identity_key_states (
          key_state_id, identity_id, public_key_ref, signature_profile,
          signature_algorithm, public_key_bytes, is_active, source_event_id,
          source_block_height, source_event_index, source_kind, recovery_process_ref
        ) VALUES ($1, $2, $3, 'ed25519_v0', 'ed25519', $4, true, $5, $6, $7,
          'profile_v0_identity_create', NULL)
        "#,
    )
    .bind(Uuid::now_v7())
    .bind(payload.identity_id)
    .bind(&initial_key_ref_hex)
    .bind(&payload.initial_key_descriptor.raw_public_key_bytes)
    .bind(input.event_id)
    .bind(position.block_height)
    .bind(position.event_index)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;

    for root in &payload.identity_structural_root_plan.roots {
        sqlx::query(
            r#"
            INSERT INTO ideas (
              idea_id, idea_type, speaker_identity_id, is_identity_idea,
              underlying_identity_id, is_personal_space_organizer,
              title_representation_id, sentence_representation_id,
              created_block_height, created_event_index, created_event_id
            ) VALUES ($1, 'conceptual_idea', $2, false, NULL, false, NULL, NULL,
              $3, $4, $5)
            "#,
        )
        .bind(root.idea_id)
        .bind(payload.identity_id)
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(input.event_id)
        .execute(&mut **tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;

        sqlx::query(
            r#"
            INSERT INTO canonical_identity_structural_roots_v0 (
              identity_id, root_role, root_idea_id, canonical_title, source_event_id,
              source_block_height, source_event_index
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(payload.identity_id)
        .bind(root.role as i16)
        .bind(root.idea_id)
        .bind(root_title(root.role))
        .bind(input.event_id)
        .bind(position.block_height)
        .bind(position.event_index)
        .execute(&mut **tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
    }

    let mindgarden_id = payload.identity_structural_root_plan.roots[0].idea_id;
    for (index, connection_id) in payload
        .identity_structural_root_plan
        .membership_connection_ids
        .iter()
        .enumerate()
    {
        let child_root_id = payload.identity_structural_root_plan.roots[index + 1].idea_id;
        sqlx::query(
            r#"
            INSERT INTO connections (
              connection_id, from_idea_id, to_idea_id, connection_type, usage,
              axis, timeframe, scope, created_block_height, created_event_index,
              created_by_event_id
            ) VALUES ($1, $2, $3, 'membership', $4, NULL, NULL, $5, $6, $7, $8)
            "#,
        )
        .bind(connection_id)
        .bind(mindgarden_id)
        .bind(child_root_id)
        .bind(containment_role(index))
        .bind(STRUCTURAL_ROOT_SCOPE)
        .bind(position.block_height)
        .bind(position.event_index)
        .bind(input.event_id)
        .execute(&mut **tx)
        .await
        .map_err(map_canonical_write_sqlx_error)?;
    }

    sqlx::query(
        r#"
        INSERT INTO canonical_identity_admission_lineage_v0 (
          admitted_identity_id, sponsor_identity_id, admission_event_id,
          source_block_height, source_event_index
        ) VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(payload.identity_id)
    .bind(input.author_identity_id)
    .bind(input.event_id)
    .bind(position.block_height)
    .bind(position.event_index)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;

    sqlx::query(
        r#"
        INSERT INTO profile_v0_invitation_capacity_debits (
          admission_event_id, sponsor_identity_id, capacity_period_id, rulebook_id,
          rulebook_version, rulebook_hash, debit_units, source_block_height,
          source_event_index
        ) VALUES ($1, $2, $3, $4, $5, $6, 1, $7, $8)
        "#,
    )
    .bind(input.event_id)
    .bind(input.author_identity_id)
    .bind(payload.capacity_period_id)
    .bind(payload.rulebook_reference.rulebook_id)
    .bind(&payload.rulebook_reference.rulebook_version)
    .bind(hash32_bytes(payload.rulebook_reference.rulebook_hash))
    .bind(position.block_height)
    .bind(position.event_index)
    .execute(&mut **tx)
    .await
    .map_err(map_canonical_write_sqlx_error)?;
    Ok(())
}

impl Storage {
    /// Applies one fully formed sponsor-authored Profile-v0 identity_create candidate.
    ///
    /// This storage-only entry point is deliberately separate from the legacy account
    /// identity path. It is not a public HTTP ingress route.
    pub async fn apply_profile_v0_identity_create(
        &self,
        input: SignedCanonicalCandidateInput,
    ) -> Result<ProfileV0IdentityAdmissionResult, CanonicalWriteError> {
        if input.event_type == "identity_verification_update" {
            let error = reject_ordinary_identity_verification_update()
                .expect_err("ordinary identity_verification_update must remain compatibility-only");
            return Err(admission_error(error));
        }
        if input.event_type != "identity_create" {
            return Err(CanonicalWriteError::new(
                "unsupported_admission_profile",
                "this storage transition accepts only Profile-v0 identity_create",
            ));
        }
        if input.payload_binding_mode != PAYLOAD_BINDING_EMBEDDED || input.payload_ref.is_some() {
            return Err(CanonicalWriteError::new(
                "malformed_identity_create_payload",
                "Profile-v0 identity_create requires an embedded payload without payload_ref",
            ));
        }
        let parsed_payload =
            parse_profile_v0_identity_create_payload(&input.payload).map_err(admission_error)?;
        let (candidate, signed_candidate_bytes, candidate_hash) = candidate_hash(&input)?;

        let mut tx = self.pool.begin().await.map_err(canonical_storage_error)?;
        if let Some(existing) = load_existing_admission_event(&mut tx, input.event_id).await? {
            if existing.event_type == "identity_create"
                && existing.authored_candidate_hash_v0.as_deref() == Some(candidate_hash.as_str())
                && existing.signature.as_deref() == Some(input.signature.as_str())
                && existing.payload_json == input.payload
            {
                tx.commit().await.map_err(canonical_storage_error)?;
                return Ok(ProfileV0IdentityAdmissionResult {
                    identity_id: parsed_payload.identity_id,
                    event_id: input.event_id,
                    block_height: existing.block_height,
                    event_index: existing.event_index,
                    invitation_capacity_debit_units: CAPACITY_DEBIT_UNITS,
                    idempotent: true,
                });
            }
            return Err(CanonicalWriteError::new(
                "conflicting_duplicate_event",
                "event_id already exists with different Profile-v0 admission bytes",
            ));
        }

        let position = allocate_canonical_event_position(&mut tx).await?;
        lock_uuid(&mut tx, input.author_identity_id).await?;
        lock_uuid(&mut tx, parsed_payload.identity_id).await?;
        lock_key_ref(&mut tx, parsed_payload.initial_public_key_ref).await?;
        for root in &parsed_payload.identity_structural_root_plan.roots {
            lock_uuid(&mut tx, root.idea_id).await?;
        }
        for connection_id in &parsed_payload
            .identity_structural_root_plan
            .membership_connection_ids
        {
            lock_uuid(&mut tx, *connection_id).await?;
        }

        ensure_sponsor_is_human(&mut tx, input.author_identity_id, position).await?;
        let sponsor_key = load_sponsor_key_before_position(&mut tx, &input, position).await?;
        let pure_state = build_pure_state(&mut tx, &parsed_payload).await?;
        let payload = validate_profile_v0_identity_create_candidate(
            &candidate,
            &input.payload,
            &sponsor_key.public_key_bytes,
            &pure_state,
        )
        .map_err(admission_error)?;
        validate_admission_context(&mut tx, input.author_identity_id, &payload, position).await?;

        insert_profile_v0_event(
            &mut tx,
            &input,
            position,
            &signed_candidate_bytes,
            &candidate_hash,
        )
        .await?;
        persist_profile_v0_admission(&mut tx, &input, &payload, position).await?;
        tx.commit().await.map_err(canonical_storage_error)?;

        Ok(ProfileV0IdentityAdmissionResult {
            identity_id: payload.identity_id,
            event_id: input.event_id,
            block_height: position.block_height,
            event_index: position.event_index,
            invitation_capacity_debit_units: CAPACITY_DEBIT_UNITS,
            idempotent: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_db_guard::require_disposable_database_url;
    use ed25519_dalek::{Signer, SigningKey};
    use encoding::hash::hash_bytes;
    use event_log::profile_v0_admission::canonical_identity_create_payload_bytes_v0;
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use std::path::Path;

    const TEST_DB_ADMIN_ENV: &str = "SEED_TEST_DATABASE_ADMIN_URL";
    const TEST_DB_URL_ENV: &str = "SEED_PROFILE_V0_ADMISSION_TEST_DATABASE_URL";
    const PROFILE_V0_ADMISSION_TEST_DATABASE_PREFIX: &str = "seed_admission_p3_test_";

    #[tokio::test]
    async fn rejects_protected_database_url_before_any_profile_v0_storage_test_connection() {
        let result =
            require_disposable_database_url("postgresql://seed_app@127.0.0.1:5432/seed_dev");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn profile_v0_admission_migration_is_discoverable_without_a_database() {
        let migration_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations/postgres");
        let migrator = sqlx::migrate::Migrator::new(migration_dir)
            .await
            .expect("migration catalog parses");
        assert!(migrator.iter().any(|migration| {
            migration.version == 23
                && migration
                    .description
                    .contains("profile v0 identity admission storage")
        }));
    }

    // The full guarded transaction matrix is enabled only when an explicit admin URL
    // is supplied. It creates a `seed_admission_p3_test_` database in-process, verifies it with the
    // shared guard, applies migrations, and removes it after the test.
    #[tokio::test]
    async fn profile_v0_identity_create_is_atomic_and_idempotent_in_an_isolated_database() {
        let Some(db) = IsolatedAdmissionDb::create().await else {
            return;
        };
        let result = async {
            let input = primary_fixture_input()?;
            seed_sponsor_context(&db.pool, &input, SponsorContext::accepted(2)).await?;
            let identities_before = count(&db.pool, "identities_s0").await?;

            let storage = Storage::new(&db.database_url).await?;
            let accepted = storage
                .apply_profile_v0_identity_create(input.clone())
                .await
                .map_err(anyhow::Error::from)?;
            anyhow::ensure!(
                !accepted.idempotent,
                "first admission must not be idempotent"
            );
            anyhow::ensure!(
                accepted.invitation_capacity_debit_units == 1,
                "accepted admission must debit exactly one capacity unit"
            );

            anyhow::ensure!(
                count(&db.pool, "identities_s0").await? == identities_before + 1,
                "accepted admission must add exactly one identity relative to its fixture baseline"
            );
            anyhow::ensure!(
                count(&db.pool, "canonical_profile_v0_direct_key_history").await? == 1,
                "accepted admission must register one direct key"
            );
            anyhow::ensure!(
                count(&db.pool, "canonical_identity_structural_roots_v0").await? == 4,
                "accepted admission must register four structural roots"
            );
            anyhow::ensure!(
                count(&db.pool, "canonical_identity_admission_lineage_v0").await? == 1,
                "accepted admission must register one sponsor lineage relation"
            );
            anyhow::ensure!(
                count(&db.pool, "profile_v0_invitation_capacity_debits").await? == 1,
                "accepted admission must record one capacity debit"
            );
            anyhow::ensure!(
                sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(*) FROM connections WHERE created_by_event_id = $1",
                )
                .bind(input.event_id)
                .fetch_one(&db.pool)
                .await?
                    == 3,
                "accepted admission must create three root membership connections"
            );

            let retried = storage
                .apply_profile_v0_identity_create(input)
                .await
                .map_err(anyhow::Error::from)?;
            anyhow::ensure!(retried.idempotent, "exact retry must be idempotent");
            anyhow::ensure!(
                count(&db.pool, "profile_v0_invitation_capacity_debits").await? == 1,
                "exact retry must not create a second capacity debit"
            );
            Ok::<(), anyhow::Error>(())
        }
        .await;
        db.cleanup().await;
        result.expect("Profile-v0 atomic admission transaction");
    }

    #[tokio::test]
    async fn profile_v0_stateful_rejections_are_distinct_and_make_no_admission_writes() {
        let cases = [
            (
                "stale",
                SponsorContext::stale(),
                "stale_admission_authorization",
            ),
            (
                "inactive_key",
                SponsorContext::inactive_key(),
                "author_key_inactive",
            ),
            (
                "revoked_key",
                SponsorContext::revoked_key(),
                "author_key_revoked",
            ),
            (
                "ineligible",
                SponsorContext::ineligible(),
                "inviter_ineligible",
            ),
            (
                "suspended",
                SponsorContext::suspended(),
                "inviter_suspended",
            ),
            (
                "capacity_exhausted",
                SponsorContext::accepted(0),
                "insufficient_invitation_capacity",
            ),
        ];
        for (name, context, expected_code) in cases {
            let Some(db) = IsolatedAdmissionDb::create().await else {
                return;
            };
            let result = async {
                let input = primary_fixture_input()?;
                seed_sponsor_context(&db.pool, &input, context).await?;
                let before = admission_write_counts(&db.pool).await?;
                let storage = Storage::new(&db.database_url).await?;
                let error = storage
                    .apply_profile_v0_identity_create(input)
                    .await
                    .err()
                    .ok_or_else(|| anyhow::anyhow!("{name} must reject"))?;
                anyhow::ensure!(error.code == expected_code, "{name}: wrong error code");
                anyhow::ensure!(
                    admission_write_counts(&db.pool).await? == before,
                    "{name}: rejected admission must not write state"
                );
                Ok::<(), anyhow::Error>(())
            }
            .await;
            db.cleanup().await;
            result.expect(name);
        }
    }

    #[tokio::test]
    async fn profile_v0_duplicate_identity_key_and_root_collisions_make_no_partial_writes() {
        for (name, setup, expected_code) in [
            (
                "duplicate_identity",
                CollisionSetup::Identity,
                "identity_already_exists",
            ),
            (
                "historical_key",
                CollisionSetup::HistoricalKey,
                "public_key_already_registered",
            ),
            (
                "structural_root",
                CollisionSetup::StructuralRoot,
                "structural_root_collision",
            ),
        ] {
            let Some(db) = IsolatedAdmissionDb::create().await else {
                return;
            };
            let result = async {
                let input = primary_fixture_input()?;
                seed_sponsor_context(&db.pool, &input, SponsorContext::accepted(1)).await?;
                seed_collision(&db.pool, &input, setup).await?;
                let before = admission_write_counts(&db.pool).await?;
                let storage = Storage::new(&db.database_url).await?;
                let error = storage
                    .apply_profile_v0_identity_create(input)
                    .await
                    .err()
                    .ok_or_else(|| anyhow::anyhow!("{name} must reject"))?;
                anyhow::ensure!(error.code == expected_code, "{name}: wrong error code");
                anyhow::ensure!(
                    admission_write_counts(&db.pool).await? == before,
                    "{name}: rejected admission must not write state"
                );
                Ok::<(), anyhow::Error>(())
            }
            .await;
            db.cleanup().await;
            result.expect(name);
        }
    }

    #[tokio::test]
    async fn profile_v0_speaker_presence_is_rejected_before_any_admission_write() {
        let Some(db) = IsolatedAdmissionDb::create().await else {
            return;
        };
        let result = async {
            let mut input = primary_fixture_input()?;
            seed_sponsor_context(&db.pool, &input, SponsorContext::accepted(1)).await?;
            input.speaker_identity_id = Some(input.author_identity_id);
            let before = admission_write_counts(&db.pool).await?;
            let storage = Storage::new(&db.database_url).await?;
            let error = storage
                .apply_profile_v0_identity_create(input)
                .await
                .err()
                .ok_or_else(|| anyhow::anyhow!("speaker must be absent"))?;
            anyhow::ensure!(error.code == "speaker_not_permitted", "wrong speaker error");
            anyhow::ensure!(
                admission_write_counts(&db.pool).await? == before,
                "speaker rejection must not write state"
            );
            Ok::<(), anyhow::Error>(())
        }
        .await;
        db.cleanup().await;
        result.expect("Profile-v0 absent speaker enforcement");
    }

    #[tokio::test]
    async fn profile_v0_invalid_applicant_proof_makes_no_admission_writes() {
        let Some(db) = IsolatedAdmissionDb::create().await else {
            return;
        };
        let result = async {
            let mut input = primary_fixture_input()?;
            seed_sponsor_context(&db.pool, &input, SponsorContext::accepted(1)).await?;
            input.payload["initial_key_possession_proof"] =
                serde_json::Value::String("00".repeat(64));
            resign_sponsor_candidate(&mut input)?;

            let before = admission_write_counts(&db.pool).await?;
            let storage = Storage::new(&db.database_url).await?;
            let error = storage
                .apply_profile_v0_identity_create(input)
                .await
                .err()
                .ok_or_else(|| {
                    anyhow::anyhow!("applicant proof must validate after sponsor signing")
                })?;
            // A 64-byte proof is structurally present. Without an original applicant
            // message, a failed strict verification is the established binding-mismatch
            // boundary; malformed proof representation is rejected earlier.
            anyhow::ensure!(
                error.code == "applicant_proof_binding_mismatch",
                "wrong applicant-proof error"
            );
            anyhow::ensure!(
                admission_write_counts(&db.pool).await? == before,
                "invalid applicant proof must not write state"
            );
            Ok::<(), anyhow::Error>(())
        }
        .await;
        db.cleanup().await;
        result.expect("Profile-v0 applicant proof rejection is atomic");
    }

    #[tokio::test]
    async fn private_account_creation_does_not_materialize_profile_v0_admission_state() {
        let Some(db) = IsolatedAdmissionDb::create().await else {
            return;
        };
        let result = async {
            let storage = Storage::new(&db.database_url).await?;
            let account = storage
                .create_account_private_only("profile_v0_quarantine_test", "hash")
                .await?;
            anyhow::ensure!(
                sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(*) FROM identities_s0 WHERE identity_id = $1",
                )
                .bind(account.canonical_identity_id.unwrap_or_else(Uuid::nil))
                .fetch_one(&db.pool)
                .await?
                    == 0,
                "private account must not materialize a canonical identity"
            );
            anyhow::ensure!(
                count(&db.pool, "profile_v0_identity_admissions").await? == 0,
                "private account must not create admission provenance"
            );
            anyhow::ensure!(
                count(&db.pool, "canonical_identity_admission_lineage_v0").await? == 0,
                "private account must not create sponsor lineage"
            );
            anyhow::ensure!(
                count(&db.pool, "profile_v0_invitation_capacity_debits").await? == 0,
                "private account must not create capacity debits"
            );
            Ok::<(), anyhow::Error>(())
        }
        .await;
        db.cleanup().await;
        result.expect("private account Profile-v0 quarantine");
    }

    #[tokio::test]
    async fn legacy_identity_rows_remain_readable_without_fabricated_profile_v0_history() {
        let Some(db) = IsolatedAdmissionDb::create().await else {
            return;
        };
        let result = async {
            let legacy_identity_id = Uuid::now_v7();
            let legacy_event_id = Uuid::now_v7();
            sqlx::query(
                "INSERT INTO blocks (block_height, block_hash, prev_block_hash) VALUES (0, '0', NULL)",
            )
            .execute(&db.pool)
            .await?;
            sqlx::query(
                r#"
                INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
                VALUES (0, 0, $1, 'identity_create', $2, '{}'::jsonb, NULL)
                "#,
            )
            .bind(legacy_event_id)
            .bind(legacy_identity_id)
            .execute(&db.pool)
            .await?;
            sqlx::query(
                "INSERT INTO identities_s0 (identity_id, title, created_event_id) VALUES ($1, 'Legacy Identity', $2)",
            )
            .bind(legacy_identity_id)
            .bind(legacy_event_id)
            .execute(&db.pool)
            .await?;
            let storage = Storage::new(&db.database_url).await?;
            anyhow::ensure!(
                storage.get_identity(legacy_identity_id).await?.is_some(),
                "legacy identity must remain readable"
            );
            anyhow::ensure!(
                count(&db.pool, "canonical_identity_provenance_v0").await? == 0,
                "legacy fixture must not fabricate Profile-v0 provenance"
            );
            anyhow::ensure!(
                count(&db.pool, "profile_v0_identity_admissions").await? == 0,
                "legacy fixture must not fabricate Profile-v0 admission"
            );
            anyhow::ensure!(
                count(&db.pool, "canonical_identity_structural_roots_v0").await? == 0,
                "legacy fixture must not fabricate Profile-v0 roots"
            );
            anyhow::ensure!(
                count(&db.pool, "canonical_identity_admission_lineage_v0").await? == 0,
                "legacy fixture must not fabricate sponsor lineage"
            );
            anyhow::ensure!(
                count(&db.pool, "profile_v0_invitation_capacity_debits").await? == 0,
                "legacy fixture must not fabricate capacity debits"
            );
            Ok::<(), anyhow::Error>(())
        }
        .await;
        db.cleanup().await;
        result.expect("legacy Profile-v0 compatibility preservation");
    }

    #[test]
    fn compatibility_only_identity_verification_update_is_not_an_admission_input() {
        let error = reject_ordinary_identity_verification_update().expect_err("compatibility only");
        assert_eq!(error.code, "compatibility_event_not_authorized");
    }

    struct IsolatedAdmissionDb {
        admin_pool: PgPool,
        pool: PgPool,
        database_name: String,
        database_url: String,
    }

    #[derive(Clone, Copy)]
    enum SponsorKeyState {
        Active,
        Inactive,
        Revoked,
    }

    #[derive(Clone, Copy)]
    struct SponsorContext {
        capacity: i64,
        inviter_eligible: bool,
        invitation_suspended: bool,
        key_state: SponsorKeyState,
        stale_context: bool,
    }

    impl SponsorContext {
        fn accepted(capacity: i64) -> Self {
            Self {
                capacity,
                inviter_eligible: true,
                invitation_suspended: false,
                key_state: SponsorKeyState::Active,
                stale_context: false,
            }
        }

        fn stale() -> Self {
            Self {
                stale_context: true,
                ..Self::accepted(1)
            }
        }

        fn inactive_key() -> Self {
            Self {
                key_state: SponsorKeyState::Inactive,
                ..Self::accepted(1)
            }
        }

        fn revoked_key() -> Self {
            Self {
                key_state: SponsorKeyState::Revoked,
                ..Self::accepted(1)
            }
        }

        fn ineligible() -> Self {
            Self {
                inviter_eligible: false,
                ..Self::accepted(1)
            }
        }

        fn suspended() -> Self {
            Self {
                invitation_suspended: true,
                ..Self::accepted(1)
            }
        }
    }

    #[derive(Clone, Copy)]
    enum CollisionSetup {
        Identity,
        HistoricalKey,
        StructuralRoot,
    }

    impl IsolatedAdmissionDb {
        async fn create() -> Option<Self> {
            let admin_url = match std::env::var(TEST_DB_ADMIN_ENV) {
                Ok(value) if !value.trim().is_empty() => value,
                _ => {
                    eprintln!("SKIP: {TEST_DB_ADMIN_ENV} is missing; Profile-v0 storage tests require an explicit disposable Postgres admin URL");
                    return None;
                }
            };
            if !is_safe_admin_database_url(&admin_url) {
                eprintln!(
                    "SKIP: {TEST_DB_ADMIN_ENV} must use the postgres maintenance database; protected application databases are never test-admin targets"
                );
                return None;
            }
            let admin_pool = match PgPoolOptions::new()
                .max_connections(2)
                .connect(&admin_url)
                .await
            {
                Ok(pool) => pool,
                Err(error) => {
                    eprintln!("SKIP: unable to connect to {TEST_DB_ADMIN_ENV}: {error}");
                    return None;
                }
            };
            let database_name = format!(
                "{PROFILE_V0_ADMISSION_TEST_DATABASE_PREFIX}{}_{}",
                std::process::id(),
                Uuid::now_v7().simple()
            );
            let database_url = match database_url_for(&admin_url, &database_name) {
                Some(value) => value,
                None => {
                    admin_pool.close().await;
                    return None;
                }
            };
            let guarded_name = match require_disposable_database_url(&database_url) {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("SKIP: generated database URL rejected by guard: {error}");
                    admin_pool.close().await;
                    return None;
                }
            };
            // Set the isolated URL in-process without changing the shared DATABASE_URL
            // consumed by unrelated storage tests.
            std::env::set_var(TEST_DB_URL_ENV, &database_url);
            let create_sql = format!("CREATE DATABASE {}", quote_ident(&database_name));
            if let Err(error) = sqlx::query(&create_sql).execute(&admin_pool).await {
                eprintln!("SKIP: unable to create isolated database {guarded_name}: {error}");
                admin_pool.close().await;
                return None;
            }
            let pool = match PgPoolOptions::new()
                .max_connections(4)
                .connect(&database_url)
                .await
            {
                Ok(pool) => pool,
                Err(error) => {
                    eprintln!(
                        "SKIP: unable to connect to isolated database {guarded_name}: {error}"
                    );
                    drop_database(&admin_pool, &database_name).await;
                    admin_pool.close().await;
                    return None;
                }
            };
            let migration_dir =
                Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations/postgres");
            let migrator = match sqlx::migrate::Migrator::new(migration_dir).await {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("SKIP: unable to load migrations for {guarded_name}: {error}");
                    pool.close().await;
                    drop_database(&admin_pool, &database_name).await;
                    admin_pool.close().await;
                    return None;
                }
            };
            if let Err(error) = migrator.run(&pool).await {
                eprintln!("SKIP: unable to apply migrations to {guarded_name}: {error}");
                pool.close().await;
                drop_database(&admin_pool, &database_name).await;
                admin_pool.close().await;
                return None;
            }
            eprintln!("ISOLATED_DB: {guarded_name} differs_from_seed_dev=true");
            Some(Self {
                admin_pool,
                pool,
                database_name,
                database_url,
            })
        }

        async fn cleanup(self) {
            self.pool.close().await;
            drop_database(&self.admin_pool, &self.database_name).await;
            self.admin_pool.close().await;
        }
    }

    async fn seed_sponsor_context(
        pool: &PgPool,
        input: &SignedCanonicalCandidateInput,
        context: SponsorContext,
    ) -> Result<(), anyhow::Error> {
        let payload = parse_profile_v0_identity_create_payload(&input.payload)?;
        let sponsor_event_id = Uuid::now_v7();
        sqlx::query(
            "INSERT INTO blocks (block_height, block_hash, prev_block_hash) VALUES (0, '0', NULL)",
        )
        .execute(pool)
        .await?;
        sqlx::query(
            r#"
            INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
            VALUES (0, 0, $1, 'genesis_import_manifest', NULL, '{}'::jsonb, NULL)
            "#,
        )
        .bind(sponsor_event_id)
        .execute(pool)
        .await?;
        sqlx::query(
            "INSERT INTO identities_s0 (identity_id, title, created_event_id) VALUES ($1, 'P3 Sponsor', $2)",
        )
        .bind(input.author_identity_id)
        .bind(sponsor_event_id)
        .execute(pool)
        .await?;
        sqlx::query(
            r#"
            INSERT INTO canonical_identity_provenance_v0 (
              identity_id, identity_kind, provenance_class, source_event_id,
              source_block_height, source_event_index
            ) VALUES ($1, 'human', 'genesis_admitted', $2, 0, 0)
            "#,
        )
        .bind(input.author_identity_id)
        .bind(sponsor_event_id)
        .execute(pool)
        .await?;
        let sponsor_raw_key = fixture_hex_bytes(input_fixture_field("sponsor_raw_public_key")?)?;
        sqlx::query(
            r#"
            INSERT INTO canonical_identity_key_states (
              key_state_id, identity_id, public_key_ref, signature_profile,
              signature_algorithm, public_key_bytes, is_active, source_event_id,
              source_block_height, source_event_index, source_kind, recovery_process_ref
            ) VALUES ($1, $2, $3, 'ed25519_v0', 'ed25519', $4, $5, $6, 0, 0,
              'genesis_import_compatibility', NULL)
            "#,
        )
        .bind(Uuid::now_v7())
        .bind(input.author_identity_id)
        .bind(&input.public_key_ref)
        .bind(sponsor_raw_key)
        .bind(matches!(
            context.key_state,
            SponsorKeyState::Active | SponsorKeyState::Revoked
        ))
        .bind(sponsor_event_id)
        .execute(pool)
        .await?;
        if matches!(context.key_state, SponsorKeyState::Revoked) {
            sqlx::query(
                r#"
                INSERT INTO canonical_profile_v0_direct_key_history (
                  public_key_ref, raw_public_key, identity_id, source_event_id,
                  source_block_height, source_event_index
                ) VALUES (decode($1, 'hex'), $2, $3, $4, 0, 0)
                "#,
            )
            .bind(&input.public_key_ref)
            .bind(fixture_hex_bytes(input_fixture_field(
                "sponsor_raw_public_key",
            )?)?)
            .bind(input.author_identity_id)
            .bind(sponsor_event_id)
            .execute(pool)
            .await?;
            sqlx::query(
                r#"
                INSERT INTO canonical_profile_v0_direct_key_state_history (
                  state_record_id, public_key_ref, key_state, source_event_id,
                  source_block_height, source_event_index
                ) VALUES ($1, decode($2, 'hex'), 'revoked', $3, 0, 0)
                "#,
            )
            .bind(Uuid::now_v7())
            .bind(&input.public_key_ref)
            .bind(sponsor_event_id)
            .execute(pool)
            .await?;
        }
        let capacity_period_id = if context.stale_context {
            Uuid::now_v7()
        } else {
            payload.capacity_period_id
        };
        sqlx::query(
            r#"
            INSERT INTO profile_v0_admission_state_materializations (
              materialization_id, identity_id, identity_kind, provenance_class,
              admission_profile_version, capacity_period_id, rulebook_id,
              rulebook_version, rulebook_hash, inviter_eligible, invitation_suspended,
              spendable_capacity, source_event_id, source_block_height,
              source_event_index, materialization_class
            ) VALUES ($1, $2, 'human', 'genesis_admitted', $3, $4, $5, $6, $7,
              $8, $9, $10, $11, 0, 0, 'compatibility_replay_bridge')
            "#,
        )
        .bind(Uuid::now_v7())
        .bind(input.author_identity_id)
        .bind(&payload.admission_profile_version)
        .bind(capacity_period_id)
        .bind(payload.rulebook_reference.rulebook_id)
        .bind(&payload.rulebook_reference.rulebook_version)
        .bind(payload.rulebook_reference.rulebook_hash.to_vec())
        .bind(context.inviter_eligible)
        .bind(context.invitation_suspended)
        .bind(context.capacity)
        .bind(sponsor_event_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn seed_collision(
        pool: &PgPool,
        input: &SignedCanonicalCandidateInput,
        setup: CollisionSetup,
    ) -> Result<(), anyhow::Error> {
        let payload = parse_profile_v0_identity_create_payload(&input.payload)?;
        let collision_event_id = Uuid::now_v7();
        sqlx::query(
            r#"
            INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
            VALUES (0, 1, $1, 'genesis_import_manifest', NULL, '{}'::jsonb, NULL)
            "#,
        )
        .bind(collision_event_id)
        .execute(pool)
        .await?;
        match setup {
            CollisionSetup::Identity => {
                sqlx::query(
                    "INSERT INTO identities_s0 (identity_id, title, created_event_id) VALUES ($1, 'Existing Target', $2)",
                )
                .bind(payload.identity_id)
                .bind(collision_event_id)
                .execute(pool)
                .await?;
            }
            CollisionSetup::HistoricalKey => {
                let owner_id = Uuid::now_v7();
                sqlx::query(
                    "INSERT INTO identities_s0 (identity_id, title, created_event_id) VALUES ($1, 'Existing Key Owner', $2)",
                )
                .bind(owner_id)
                .bind(collision_event_id)
                .execute(pool)
                .await?;
                sqlx::query(
                    r#"
                    INSERT INTO canonical_identity_key_states (
                      key_state_id, identity_id, public_key_ref, signature_profile,
                      signature_algorithm, public_key_bytes, is_active, source_event_id,
                      source_block_height, source_event_index, source_kind, recovery_process_ref
                    ) VALUES ($1, $2, $3, 'ed25519_v0', 'ed25519', $4, true, $5, 0, 1,
                      'genesis_import_compatibility', NULL)
                    "#,
                )
                .bind(Uuid::now_v7())
                .bind(owner_id)
                .bind(to_hex(&payload.initial_public_key_ref))
                .bind(&payload.initial_key_descriptor.raw_public_key_bytes)
                .bind(collision_event_id)
                .execute(pool)
                .await?;
            }
            CollisionSetup::StructuralRoot => {
                sqlx::query(
                    r#"
                    INSERT INTO ideas (
                      idea_id, idea_type, speaker_identity_id, is_identity_idea,
                      underlying_identity_id, is_personal_space_organizer,
                      title_representation_id, sentence_representation_id,
                      created_block_height, created_event_index, created_event_id
                    ) VALUES ($1, 'conceptual_idea', $2, false, NULL, false, NULL, NULL,
                      0, 1, $3)
                    "#,
                )
                .bind(payload.identity_structural_root_plan.roots[0].idea_id)
                .bind(input.author_identity_id)
                .bind(collision_event_id)
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }

    fn primary_fixture_input() -> Result<SignedCanonicalCandidateInput, anyhow::Error> {
        let fixture = fixture_identity_create()?;
        let candidate = fixture
            .get("candidate")
            .and_then(serde_json::Value::as_object)
            .ok_or_else(|| anyhow::anyhow!("identity_create fixture candidate missing"))?;
        Ok(SignedCanonicalCandidateInput {
            signature_profile: fixture_string(candidate, "signature_profile")?.to_string(),
            event_id: Uuid::parse_str(fixture_string(candidate, "event_id")?)?,
            event_type: fixture_string(candidate, "event_type")?.to_string(),
            author_identity_id: Uuid::parse_str(fixture_string(candidate, "author_identity_id")?)?,
            speaker_identity_id: None,
            public_key_ref: fixture_string(candidate, "public_key_ref")?.to_string(),
            payload_hash: fixture_string(candidate, "payload_hash")?.to_string(),
            payload_binding_mode: fixture_string(candidate, "payload_binding_mode")?.to_string(),
            payload_ref: None,
            author_observed_at: None,
            signature: fixture_string(candidate, "signature")?.to_string(),
            payload: fixture
                .get("payload")
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("identity_create fixture payload missing"))?,
        })
    }

    fn resign_sponsor_candidate(
        input: &mut SignedCanonicalCandidateInput,
    ) -> Result<(), anyhow::Error> {
        let payload =
            parse_profile_v0_identity_create_payload(&input.payload).map_err(anyhow::Error::msg)?;
        input.payload_hash = to_hex(&hash_bytes(&canonical_identity_create_payload_bytes_v0(
            &payload,
        )?));
        let sponsor = SigningKey::from_bytes(&[0x11; 32]);
        input.signature = to_hex(
            &sponsor
                .sign(&signed_candidate_bytes_v0(&authored_candidate(input))?)
                .to_bytes(),
        );
        Ok(())
    }

    fn fixture_identity_create() -> Result<&'static serde_json::Value, anyhow::Error> {
        static FIXTURES: std::sync::OnceLock<serde_json::Value> = std::sync::OnceLock::new();
        let fixtures = FIXTURES.get_or_init(|| {
            serde_json::from_str(include_str!(
                "../../../../docs/conformance/profile-v0-identity-admission.vectors.json"
            ))
            .expect("Profile-v0 static vector JSON parses")
        });
        fixtures
            .pointer("/pure_crypto_fixtures/identity_create_primary")
            .ok_or_else(|| anyhow::anyhow!("identity_create fixture missing"))
    }

    fn input_fixture_field(field: &str) -> Result<&'static str, anyhow::Error> {
        fixture_identity_create()?
            .get(field)
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("identity_create fixture field {field} missing"))
    }

    fn fixture_string<'a>(
        object: &'a serde_json::Map<String, serde_json::Value>,
        field: &str,
    ) -> Result<&'a str, anyhow::Error> {
        object
            .get(field)
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("fixture field {field} missing"))
    }

    fn fixture_hex_bytes(value: &str) -> Result<Vec<u8>, anyhow::Error> {
        if value.len() % 2 != 0 {
            return Err(anyhow::anyhow!("fixture hex has odd length"));
        }
        (0..value.len())
            .step_by(2)
            .map(|index| {
                u8::from_str_radix(&value[index..index + 2], 16).map_err(anyhow::Error::from)
            })
            .collect()
    }

    async fn count(pool: &PgPool, table: &str) -> Result<i64, anyhow::Error> {
        let query = format!("SELECT COUNT(*) FROM {table}");
        Ok(sqlx::query_scalar(&query).fetch_one(pool).await?)
    }

    async fn admission_write_counts(
        pool: &PgPool,
    ) -> Result<(i64, i64, i64, i64, i64, i64), anyhow::Error> {
        Ok((
            count(pool, "events").await?,
            count(pool, "identities_s0").await?,
            count(pool, "canonical_profile_v0_direct_key_history").await?,
            count(pool, "canonical_identity_structural_roots_v0").await?,
            count(pool, "canonical_identity_admission_lineage_v0").await?,
            count(pool, "profile_v0_invitation_capacity_debits").await?,
        ))
    }

    fn database_url_for(admin_url: &str, database_name: &str) -> Option<String> {
        let (prefix, _) = admin_url.rsplit_once('/')?;
        Some(format!("{prefix}/{database_name}"))
    }

    fn is_safe_admin_database_url(value: &str) -> bool {
        let without_query = value
            .trim()
            .split(['?', '#'])
            .next()
            .unwrap_or(value.trim());
        let Some((_, path)) = without_query.rsplit_once('/') else {
            return false;
        };
        path.eq_ignore_ascii_case("postgres")
    }

    fn quote_ident(value: &str) -> String {
        format!("\"{}\"", value.replace('"', "\"\""))
    }

    async fn drop_database(admin_pool: &PgPool, database_name: &str) {
        if !database_name.starts_with(PROFILE_V0_ADMISSION_TEST_DATABASE_PREFIX) {
            eprintln!(
                "REFUSE_CLEANUP: database name does not match the Profile-v0 admission test prefix"
            );
            return;
        }
        let _ = sqlx::query(
            r#"
            SELECT pg_terminate_backend(pid)
            FROM pg_stat_activity
            WHERE datname = $1
              AND pid <> pg_backend_pid()
            "#,
        )
        .bind(database_name)
        .execute(admin_pool)
        .await;
        match sqlx::query(&format!(
            "DROP DATABASE IF EXISTS {}",
            quote_ident(database_name)
        ))
        .execute(admin_pool)
        .await
        {
            Ok(_) => eprintln!("ISOLATED_DB_CLEANUP: {database_name} dropped=true"),
            Err(_) => eprintln!("ISOLATED_DB_CLEANUP: {database_name} dropped=false"),
        }
    }
}
