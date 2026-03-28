BEGIN;

CREATE TABLE IF NOT EXISTS challenges (
  challenge_id uuid PRIMARY KEY,
  challenge_domain text NOT NULL,
  context_key text NOT NULL,
  target_left_idea_id uuid NOT NULL
    REFERENCES ideas(idea_id) ON DELETE RESTRICT,
  target_right_idea_id uuid NOT NULL
    REFERENCES ideas(idea_id) ON DELETE RESTRICT,
  reference_idea_id uuid NULL
    REFERENCES ideas(idea_id) ON DELETE RESTRICT,
  framing_representation_ref uuid NOT NULL
    REFERENCES representations(representation_id) ON DELETE RESTRICT,
  created_by_identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  created_block_height bigint NOT NULL,
  created_event_index int NOT NULL,
  created_event_id uuid NOT NULL UNIQUE
    REFERENCES events(event_id) ON DELETE CASCADE,
  created_cycle_index bigint NOT NULL,
  lifecycle_state smallint NOT NULL DEFAULT 0,
  terminal_event_id uuid NULL
    REFERENCES events(event_id) ON DELETE SET NULL,
  CHECK (challenge_domain = 'importance_challenge'),
  CHECK (length(trim(context_key)) > 0),
  CHECK (target_left_idea_id <> target_right_idea_id),
  CHECK (created_block_height >= 0),
  CHECK (created_event_index >= 0),
  CHECK (created_cycle_index >= 0),
  CHECK (lifecycle_state IN (0, 1, 2, 3, 4)),
  FOREIGN KEY (created_block_height, created_event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS challenge_context (
  challenge_id uuid PRIMARY KEY
    REFERENCES challenges(challenge_id) ON DELETE CASCADE,
  axis text NOT NULL,
  timeframe text NOT NULL,
  scope text NOT NULL,
  CHECK (length(trim(axis)) > 0),
  CHECK (length(trim(timeframe)) > 0),
  CHECK (length(trim(scope)) > 0)
);

CREATE TABLE IF NOT EXISTS challenge_targets (
  challenge_id uuid NOT NULL
    REFERENCES challenges(challenge_id) ON DELETE CASCADE,
  target_position smallint NOT NULL,
  idea_id uuid NOT NULL
    REFERENCES ideas(idea_id) ON DELETE RESTRICT,
  PRIMARY KEY (challenge_id, target_position),
  UNIQUE (challenge_id, idea_id),
  CHECK (target_position IN (0, 1))
);

CREATE TABLE IF NOT EXISTS challenge_arguments (
  challenge_id uuid NOT NULL
    REFERENCES challenges(challenge_id) ON DELETE CASCADE,
  connection_id uuid PRIMARY KEY
    REFERENCES connections(connection_id) ON DELETE CASCADE,
  argument_idea_id uuid NOT NULL
    REFERENCES ideas(idea_id) ON DELETE RESTRICT,
  subject_idea_id uuid NOT NULL
    REFERENCES ideas(idea_id) ON DELETE RESTRICT,
  created_block_height bigint NOT NULL,
  created_event_index int NOT NULL,
  created_event_id uuid NOT NULL UNIQUE
    REFERENCES events(event_id) ON DELETE CASCADE,
  CHECK (created_block_height >= 0),
  CHECK (created_event_index >= 0),
  CHECK (argument_idea_id <> subject_idea_id),
  FOREIGN KEY (created_block_height, created_event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS challenges_created_order_idx
  ON challenges (created_block_height, created_event_index);

CREATE INDEX IF NOT EXISTS challenges_created_cycle_idx
  ON challenges (created_cycle_index, created_block_height, created_event_index);

CREATE UNIQUE INDEX IF NOT EXISTS challenges_active_importance_key_idx
  ON challenges (
    challenge_domain,
    context_key,
    target_left_idea_id,
    target_right_idea_id,
    (COALESCE(reference_idea_id, '00000000-0000-0000-0000-000000000000'::uuid))
  )
  WHERE lifecycle_state IN (0, 1);

CREATE INDEX IF NOT EXISTS challenge_arguments_challenge_order_idx
  ON challenge_arguments (challenge_id, created_block_height, created_event_index);

DROP TRIGGER IF EXISTS seed_challenges_append_only ON challenges;
CREATE TRIGGER seed_challenges_append_only
BEFORE UPDATE OR DELETE ON challenges
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_challenge_context_append_only ON challenge_context;
CREATE TRIGGER seed_challenge_context_append_only
BEFORE UPDATE OR DELETE ON challenge_context
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_challenge_targets_append_only ON challenge_targets;
CREATE TRIGGER seed_challenge_targets_append_only
BEFORE UPDATE OR DELETE ON challenge_targets
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_challenge_arguments_append_only ON challenge_arguments;
CREATE TRIGGER seed_challenge_arguments_append_only
BEFORE UPDATE OR DELETE ON challenge_arguments
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

COMMIT;
