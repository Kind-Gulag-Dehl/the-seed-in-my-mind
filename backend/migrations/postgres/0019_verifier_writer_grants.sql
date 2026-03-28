BEGIN;

CREATE TABLE IF NOT EXISTS verifier_role_assignments (
  assignment_id uuid PRIMARY KEY,
  verifier_identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  is_active boolean NOT NULL,
  source_event_id uuid NULL UNIQUE
    REFERENCES events(event_id) ON DELETE CASCADE,
  source_block_height bigint NULL,
  source_event_index int NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  note text NULL,
  CHECK (
    (source_event_id IS NULL AND source_block_height IS NULL AND source_event_index IS NULL)
    OR (source_event_id IS NOT NULL AND source_block_height >= 0 AND source_event_index >= 0)
  )
);

CREATE INDEX IF NOT EXISTS verifier_role_assignments_identity_order_idx
  ON verifier_role_assignments (
    verifier_identity_id,
    source_block_height DESC NULLS LAST,
    source_event_index DESC NULLS LAST,
    created_at DESC
  );

INSERT INTO verifier_role_assignments (
  assignment_id,
  verifier_identity_id,
  is_active,
  source_event_id,
  source_block_height,
  source_event_index,
  note
)
SELECT
  '00000000-0000-7000-8000-00000000f901'::uuid,
  '380b7817-db3b-7b76-8cf3-87df879ddddb'::uuid,
  true,
  NULL,
  NULL,
  NULL,
  'bootstrap_seed_verifier'
WHERE EXISTS (
  SELECT 1
  FROM identities_s0
  WHERE identity_id = '380b7817-db3b-7b76-8cf3-87df879ddddb'::uuid
)
ON CONFLICT (assignment_id) DO NOTHING;

CREATE TABLE IF NOT EXISTS canonical_writer_verification_states (
  identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  email_verified boolean NOT NULL,
  canonical_writer_level smallint NOT NULL,
  granted_by_identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  source_event_id uuid NOT NULL UNIQUE
    REFERENCES events(event_id) ON DELETE CASCADE,
  source_block_height bigint NOT NULL,
  source_event_index int NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (canonical_writer_level >= 0),
  CHECK (source_block_height >= 0),
  CHECK (source_event_index >= 0),
  PRIMARY KEY (identity_id, source_block_height, source_event_index),
  FOREIGN KEY (source_block_height, source_event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS canonical_writer_verification_states_identity_order_idx
  ON canonical_writer_verification_states (
    identity_id,
    source_block_height DESC,
    source_event_index DESC
  );

CREATE INDEX IF NOT EXISTS canonical_writer_verification_states_level_idx
  ON canonical_writer_verification_states (canonical_writer_level, email_verified);

CREATE INDEX IF NOT EXISTS canonical_writer_verification_states_source_order_idx
  ON canonical_writer_verification_states (source_block_height, source_event_index);

DROP TRIGGER IF EXISTS seed_verifier_role_assignments_append_only ON verifier_role_assignments;
CREATE TRIGGER seed_verifier_role_assignments_append_only
BEFORE UPDATE OR DELETE ON verifier_role_assignments
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_canonical_writer_verification_states_append_only ON canonical_writer_verification_states;
CREATE TRIGGER seed_canonical_writer_verification_states_append_only
BEFORE UPDATE OR DELETE ON canonical_writer_verification_states
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

COMMIT;
