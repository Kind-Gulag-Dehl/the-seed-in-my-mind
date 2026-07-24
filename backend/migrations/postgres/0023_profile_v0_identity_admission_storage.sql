BEGIN;

-- A Profile-v0 identity_create atomically materializes four root ideas and three
-- membership connections from one source event. Earlier Stage 0 uniqueness rules
-- allowed only one idea and one connection per source event.
ALTER TABLE ideas
  DROP CONSTRAINT IF EXISTS ideas_created_event_id_key,
  DROP CONSTRAINT IF EXISTS ideas_created_order_key;

ALTER TABLE connections
  DROP CONSTRAINT IF EXISTS connections_created_by_event_id_key,
  DROP CONSTRAINT IF EXISTS connections_created_order_key;

CREATE INDEX IF NOT EXISTS ideas_created_event_idx
  ON ideas (created_event_id);

CREATE INDEX IF NOT EXISTS ideas_created_order_idx
  ON ideas (created_block_height, created_event_index);

CREATE INDEX IF NOT EXISTS connections_created_event_idx
  ON connections (created_by_event_id);

CREATE INDEX IF NOT EXISTS connections_created_order_idx
  ON connections (created_block_height, created_event_index);

CREATE TABLE IF NOT EXISTS canonical_identity_provenance_v0 (
  identity_id uuid PRIMARY KEY
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  identity_kind text NOT NULL,
  provenance_class text NOT NULL,
  source_event_id uuid NULL UNIQUE
    REFERENCES events(event_id) ON DELETE RESTRICT,
  source_block_height bigint NULL,
  source_event_index int NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (identity_kind = 'human'),
  CHECK (provenance_class IN (
    'genesis_admitted',
    'legacy_operator_provisioned',
    'event_derived',
    'future_profile_derived'
  )),
  CHECK (
    (source_event_id IS NULL AND source_block_height IS NULL AND source_event_index IS NULL)
    OR (source_event_id IS NOT NULL AND source_block_height >= 0 AND source_event_index >= 0)
  )
);

CREATE TABLE IF NOT EXISTS profile_v0_identity_admissions (
  identity_id uuid PRIMARY KEY
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  admission_event_id uuid NOT NULL UNIQUE
    REFERENCES events(event_id) ON DELETE RESTRICT,
  sponsor_identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  admission_profile_version text NOT NULL,
  capacity_period_id uuid NOT NULL,
  rulebook_id uuid NOT NULL,
  rulebook_version text NOT NULL,
  rulebook_hash bytea NOT NULL,
  verification_reference bytea NULL,
  created_block_height bigint NOT NULL,
  created_event_index int NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (admission_profile_version = 'sponsored_public_admission_v0'),
  CHECK (octet_length(rulebook_hash) = 32),
  CHECK (verification_reference IS NULL OR octet_length(verification_reference) = 32),
  CHECK (created_block_height >= 0),
  CHECK (created_event_index >= 0)
);

CREATE INDEX IF NOT EXISTS profile_v0_identity_admissions_sponsor_idx
  ON profile_v0_identity_admissions (sponsor_identity_id, created_block_height, created_event_index);

CREATE TABLE IF NOT EXISTS canonical_identity_structural_roots_v0 (
  identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  root_role smallint NOT NULL,
  root_idea_id uuid NOT NULL UNIQUE
    REFERENCES ideas(idea_id) ON DELETE RESTRICT,
  canonical_title text NOT NULL,
  source_event_id uuid NOT NULL
    REFERENCES events(event_id) ON DELETE RESTRICT,
  source_block_height bigint NOT NULL,
  source_event_index int NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (identity_id, root_role),
  CHECK (root_role IN (1, 2, 3, 4)),
  CHECK (canonical_title IN (
    'Mindgarden',
    'Backyard of Relationships',
    'Self Tree',
    'Anthill'
  )),
  CHECK (source_block_height >= 0),
  CHECK (source_event_index >= 0)
);

CREATE INDEX IF NOT EXISTS canonical_identity_structural_roots_v0_source_idx
  ON canonical_identity_structural_roots_v0 (source_event_id, root_role);

CREATE TABLE IF NOT EXISTS canonical_identity_admission_lineage_v0 (
  admitted_identity_id uuid PRIMARY KEY
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  sponsor_identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  admission_event_id uuid NOT NULL UNIQUE
    REFERENCES events(event_id) ON DELETE RESTRICT,
  source_block_height bigint NOT NULL,
  source_event_index int NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (admitted_identity_id <> sponsor_identity_id),
  CHECK (source_block_height >= 0),
  CHECK (source_event_index >= 0)
);

CREATE INDEX IF NOT EXISTS canonical_identity_admission_lineage_v0_sponsor_idx
  ON canonical_identity_admission_lineage_v0 (sponsor_identity_id, source_block_height, source_event_index);

CREATE TABLE IF NOT EXISTS canonical_profile_v0_direct_key_history (
  public_key_ref bytea PRIMARY KEY,
  raw_public_key bytea NOT NULL UNIQUE,
  identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  source_event_id uuid NOT NULL UNIQUE
    REFERENCES events(event_id) ON DELETE RESTRICT,
  source_block_height bigint NOT NULL,
  source_event_index int NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (octet_length(public_key_ref) = 32),
  CHECK (octet_length(raw_public_key) = 32),
  CHECK (source_block_height >= 0),
  CHECK (source_event_index >= 0)
);

CREATE INDEX IF NOT EXISTS canonical_profile_v0_direct_key_history_identity_idx
  ON canonical_profile_v0_direct_key_history (identity_id, source_block_height, source_event_index);

CREATE TABLE IF NOT EXISTS canonical_profile_v0_direct_key_state_history (
  state_record_id uuid PRIMARY KEY,
  public_key_ref bytea NOT NULL
    REFERENCES canonical_profile_v0_direct_key_history(public_key_ref) ON DELETE RESTRICT,
  key_state text NOT NULL,
  source_event_id uuid NOT NULL
    REFERENCES events(event_id) ON DELETE RESTRICT,
  source_block_height bigint NOT NULL,
  source_event_index int NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (key_state IN ('active', 'superseded', 'revoked', 'invalid')),
  CHECK (source_block_height >= 0),
  CHECK (source_event_index >= 0)
);

CREATE INDEX IF NOT EXISTS canonical_profile_v0_direct_key_state_history_lookup_idx
  ON canonical_profile_v0_direct_key_state_history (
    public_key_ref,
    source_block_height DESC,
    source_event_index DESC
  );

-- This append-only materialization is a temporary compatibility bridge until P4
-- derives admission state from replay and R2C rulebook/cycle inputs. No public or
-- account-driven mutation path is introduced for it.
CREATE TABLE IF NOT EXISTS profile_v0_admission_state_materializations (
  materialization_id uuid PRIMARY KEY,
  identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  identity_kind text NOT NULL,
  provenance_class text NOT NULL,
  admission_profile_version text NOT NULL,
  capacity_period_id uuid NOT NULL,
  rulebook_id uuid NOT NULL,
  rulebook_version text NOT NULL,
  rulebook_hash bytea NOT NULL,
  inviter_eligible boolean NOT NULL,
  invitation_suspended boolean NOT NULL,
  spendable_capacity bigint NOT NULL,
  source_event_id uuid NOT NULL UNIQUE
    REFERENCES events(event_id) ON DELETE RESTRICT,
  source_block_height bigint NOT NULL,
  source_event_index int NOT NULL,
  materialization_class text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (identity_kind = 'human'),
  CHECK (provenance_class IN (
    'genesis_admitted',
    'legacy_operator_provisioned',
    'event_derived',
    'future_profile_derived'
  )),
  CHECK (admission_profile_version = 'sponsored_public_admission_v0'),
  CHECK (octet_length(rulebook_hash) = 32),
  CHECK (spendable_capacity >= 0),
  CHECK (source_block_height >= 0),
  CHECK (source_event_index >= 0),
  CHECK (materialization_class = 'compatibility_replay_bridge')
);

CREATE INDEX IF NOT EXISTS profile_v0_admission_state_materializations_identity_order_idx
  ON profile_v0_admission_state_materializations (
    identity_id,
    source_block_height DESC,
    source_event_index DESC
  );

CREATE TABLE IF NOT EXISTS profile_v0_invitation_capacity_debits (
  admission_event_id uuid PRIMARY KEY
    REFERENCES events(event_id) ON DELETE RESTRICT,
  sponsor_identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  capacity_period_id uuid NOT NULL,
  rulebook_id uuid NOT NULL,
  rulebook_version text NOT NULL,
  rulebook_hash bytea NOT NULL,
  debit_units bigint NOT NULL,
  source_block_height bigint NOT NULL,
  source_event_index int NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (octet_length(rulebook_hash) = 32),
  CHECK (debit_units = 1),
  CHECK (source_block_height >= 0),
  CHECK (source_event_index >= 0)
);

CREATE INDEX IF NOT EXISTS profile_v0_invitation_capacity_debits_sponsor_period_idx
  ON profile_v0_invitation_capacity_debits (
    sponsor_identity_id,
    capacity_period_id,
    rulebook_id,
    rulebook_version,
    rulebook_hash
  );

DROP TRIGGER IF EXISTS seed_canonical_identity_provenance_v0_append_only ON canonical_identity_provenance_v0;
CREATE TRIGGER seed_canonical_identity_provenance_v0_append_only
BEFORE UPDATE OR DELETE ON canonical_identity_provenance_v0
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_profile_v0_identity_admissions_append_only ON profile_v0_identity_admissions;
CREATE TRIGGER seed_profile_v0_identity_admissions_append_only
BEFORE UPDATE OR DELETE ON profile_v0_identity_admissions
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_canonical_identity_structural_roots_v0_append_only ON canonical_identity_structural_roots_v0;
CREATE TRIGGER seed_canonical_identity_structural_roots_v0_append_only
BEFORE UPDATE OR DELETE ON canonical_identity_structural_roots_v0
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_canonical_identity_admission_lineage_v0_append_only ON canonical_identity_admission_lineage_v0;
CREATE TRIGGER seed_canonical_identity_admission_lineage_v0_append_only
BEFORE UPDATE OR DELETE ON canonical_identity_admission_lineage_v0
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_canonical_profile_v0_direct_key_history_append_only ON canonical_profile_v0_direct_key_history;
CREATE TRIGGER seed_canonical_profile_v0_direct_key_history_append_only
BEFORE UPDATE OR DELETE ON canonical_profile_v0_direct_key_history
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_canonical_profile_v0_direct_key_state_history_append_only ON canonical_profile_v0_direct_key_state_history;
CREATE TRIGGER seed_canonical_profile_v0_direct_key_state_history_append_only
BEFORE UPDATE OR DELETE ON canonical_profile_v0_direct_key_state_history
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_profile_v0_admission_state_materializations_append_only ON profile_v0_admission_state_materializations;
CREATE TRIGGER seed_profile_v0_admission_state_materializations_append_only
BEFORE UPDATE OR DELETE ON profile_v0_admission_state_materializations
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_profile_v0_invitation_capacity_debits_append_only ON profile_v0_invitation_capacity_debits;
CREATE TRIGGER seed_profile_v0_invitation_capacity_debits_append_only
BEFORE UPDATE OR DELETE ON profile_v0_invitation_capacity_debits
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

COMMIT;
