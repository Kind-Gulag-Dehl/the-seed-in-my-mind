BEGIN;

ALTER TABLE events
  ADD COLUMN IF NOT EXISTS signature_profile text NULL,
  ADD COLUMN IF NOT EXISTS author_identity_id uuid NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  ADD COLUMN IF NOT EXISTS public_key_ref text NULL,
  ADD COLUMN IF NOT EXISTS payload_hash text NULL,
  ADD COLUMN IF NOT EXISTS payload_binding_mode text NULL,
  ADD COLUMN IF NOT EXISTS payload_ref bytea NULL,
  ADD COLUMN IF NOT EXISTS author_observed_at text NULL,
  ADD COLUMN IF NOT EXISTS signed_candidate_bytes_v0 bytea NULL,
  ADD COLUMN IF NOT EXISTS authored_candidate_hash_v0 text NULL,
  ADD COLUMN IF NOT EXISTS publication_profile text NULL,
  ADD COLUMN IF NOT EXISTS finalized_prefix_certificate_ref text NULL;

CREATE UNIQUE INDEX IF NOT EXISTS events_authored_candidate_hash_v0_key
  ON events(authored_candidate_hash_v0)
  WHERE authored_candidate_hash_v0 IS NOT NULL;

CREATE INDEX IF NOT EXISTS events_author_identity_order_idx
  ON events(author_identity_id, block_height, event_index)
  WHERE author_identity_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS canonical_identity_key_states (
  key_state_id uuid PRIMARY KEY,
  identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  public_key_ref text NOT NULL,
  signature_profile text NOT NULL,
  signature_algorithm text NOT NULL,
  public_key_bytes bytea NOT NULL,
  is_active boolean NOT NULL,
  source_event_id uuid NULL UNIQUE
    REFERENCES events(event_id) ON DELETE CASCADE,
  source_block_height bigint NULL,
  source_event_index int NULL,
  source_kind text NOT NULL,
  recovery_process_ref text NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (public_key_ref ~ '^[0-9a-f]{64}$'),
  CHECK (signature_profile = 'ed25519_v0'),
  CHECK (signature_algorithm = 'ed25519'),
  CHECK (octet_length(public_key_bytes) = 32),
  CHECK (
    (source_event_id IS NULL AND source_block_height IS NULL AND source_event_index IS NULL)
    OR (source_event_id IS NOT NULL AND source_block_height >= 0 AND source_event_index >= 0)
  )
);

CREATE INDEX IF NOT EXISTS canonical_identity_key_states_lookup_idx
  ON canonical_identity_key_states (
    public_key_ref,
    identity_id,
    source_block_height DESC NULLS LAST,
    source_event_index DESC NULLS LAST,
    created_at DESC
  );

CREATE INDEX IF NOT EXISTS canonical_identity_key_states_identity_idx
  ON canonical_identity_key_states (
    identity_id,
    source_block_height DESC NULLS LAST,
    source_event_index DESC NULLS LAST
  );

DROP TRIGGER IF EXISTS seed_canonical_identity_key_states_append_only ON canonical_identity_key_states;
CREATE TRIGGER seed_canonical_identity_key_states_append_only
BEFORE UPDATE OR DELETE ON canonical_identity_key_states
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

COMMIT;
