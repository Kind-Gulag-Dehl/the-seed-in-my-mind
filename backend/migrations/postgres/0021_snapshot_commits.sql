BEGIN;

CREATE TABLE IF NOT EXISTS snapshot_commits (
  block_height bigint PRIMARY KEY REFERENCES blocks(block_height) ON DELETE CASCADE,
  snapshot_hash text NOT NULL,
  state_root_hash text NOT NULL,
  title_sentence_payload_root text NOT NULL,
  shared_map_commitment text NOT NULL,
  last_event_id uuid NOT NULL REFERENCES events(event_id) ON DELETE CASCADE,
  event_count bigint NOT NULL,
  active_rulebook_set_hash text NOT NULL,
  created_event_id uuid NOT NULL UNIQUE REFERENCES events(event_id) ON DELETE CASCADE,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (block_height >= 0),
  CHECK (event_count >= 0),
  CHECK (snapshot_hash ~ '^[0-9a-f]+$' AND char_length(snapshot_hash) = 64),
  CHECK (state_root_hash ~ '^[0-9a-f]+$' AND char_length(state_root_hash) = 64),
  CHECK (
    title_sentence_payload_root ~ '^[0-9a-f]+$'
    AND char_length(title_sentence_payload_root) = 64
  ),
  CHECK (
    shared_map_commitment ~ '^[0-9a-f]+$'
    AND char_length(shared_map_commitment) = 64
  ),
  CHECK (
    active_rulebook_set_hash ~ '^[0-9a-f]+$'
    AND char_length(active_rulebook_set_hash) = 64
  )
);

DROP TRIGGER IF EXISTS seed_snapshot_commits_append_only ON snapshot_commits;
CREATE TRIGGER seed_snapshot_commits_append_only
BEFORE UPDATE OR DELETE ON snapshot_commits
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

COMMIT;
