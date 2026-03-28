BEGIN;

CREATE TABLE IF NOT EXISTS challenge_vote_sessions (
  vote_session_id uuid PRIMARY KEY,
  challenge_id uuid NOT NULL
    REFERENCES challenges(challenge_id) ON DELETE CASCADE,
  voter_identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  session_index bigint NOT NULL,
  selection_cycle_index bigint NOT NULL,
  selection_boundary_event_id uuid NOT NULL
    REFERENCES events(event_id) ON DELETE RESTRICT,
  created_block_height bigint NOT NULL,
  created_event_index int NOT NULL,
  created_event_id uuid NOT NULL UNIQUE
    REFERENCES events(event_id) ON DELETE CASCADE,
  CHECK (session_index >= 0),
  CHECK (selection_cycle_index >= 0),
  CHECK (created_block_height >= 0),
  CHECK (created_event_index >= 0),
  FOREIGN KEY (created_block_height, created_event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS challenge_vote_sessions_voter_session_idx
  ON challenge_vote_sessions (voter_identity_id, session_index);

CREATE UNIQUE INDEX IF NOT EXISTS challenge_vote_sessions_voter_challenge_idx
  ON challenge_vote_sessions (voter_identity_id, challenge_id);

CREATE INDEX IF NOT EXISTS challenge_vote_sessions_challenge_order_idx
  ON challenge_vote_sessions (challenge_id, created_block_height, created_event_index);

CREATE TABLE IF NOT EXISTS challenge_votes (
  cast_event_id uuid PRIMARY KEY
    REFERENCES events(event_id) ON DELETE CASCADE,
  challenge_id uuid NOT NULL
    REFERENCES challenges(challenge_id) ON DELETE CASCADE,
  voter_identity_id uuid NOT NULL
    REFERENCES identities_s0(identity_id) ON DELETE RESTRICT,
  vote_session_id uuid NOT NULL UNIQUE
    REFERENCES challenge_vote_sessions(vote_session_id) ON DELETE RESTRICT,
  vote_choice text NOT NULL,
  cast_block_height bigint NOT NULL,
  cast_event_index int NOT NULL,
  CHECK (length(trim(vote_choice)) > 0),
  CHECK (cast_block_height >= 0),
  CHECK (cast_event_index >= 0),
  FOREIGN KEY (cast_block_height, cast_event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS challenge_votes_challenge_voter_idx
  ON challenge_votes (challenge_id, voter_identity_id);

CREATE INDEX IF NOT EXISTS challenge_votes_challenge_order_idx
  ON challenge_votes (challenge_id, cast_block_height, cast_event_index);

CREATE TABLE IF NOT EXISTS challenge_verdicts (
  verdict_id uuid PRIMARY KEY,
  challenge_id uuid NOT NULL UNIQUE
    REFERENCES challenges(challenge_id) ON DELETE CASCADE,
  verdict_event_id uuid NOT NULL UNIQUE
    REFERENCES events(event_id) ON DELETE CASCADE,
  winning_choice text NOT NULL,
  winning_target_idea_id uuid NULL
    REFERENCES ideas(idea_id) ON DELETE RESTRICT,
  left_votes smallint NOT NULL,
  right_votes smallint NOT NULL,
  total_votes smallint NOT NULL,
  resolved_block_height bigint NOT NULL,
  resolved_event_index int NOT NULL,
  CHECK (winning_choice IN ('left', 'right', 'no_change')),
  CHECK (left_votes >= 0),
  CHECK (right_votes >= 0),
  CHECK (total_votes >= 0),
  CHECK (total_votes = left_votes + right_votes),
  CHECK (
    (winning_choice = 'left' AND winning_target_idea_id IS NOT NULL)
    OR (winning_choice = 'right' AND winning_target_idea_id IS NOT NULL)
    OR (winning_choice = 'no_change' AND winning_target_idea_id IS NULL)
  ),
  CHECK (resolved_block_height >= 0),
  CHECK (resolved_event_index >= 0),
  FOREIGN KEY (resolved_block_height, resolved_event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS challenge_verdicts_resolved_order_idx
  ON challenge_verdicts (resolved_block_height, resolved_event_index);

DROP TRIGGER IF EXISTS seed_challenge_vote_sessions_append_only ON challenge_vote_sessions;
CREATE TRIGGER seed_challenge_vote_sessions_append_only
BEFORE UPDATE OR DELETE ON challenge_vote_sessions
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_challenge_votes_append_only ON challenge_votes;
CREATE TRIGGER seed_challenge_votes_append_only
BEFORE UPDATE OR DELETE ON challenge_votes
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_challenge_verdicts_append_only ON challenge_verdicts;
CREATE TRIGGER seed_challenge_verdicts_append_only
BEFORE UPDATE OR DELETE ON challenge_verdicts
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

COMMIT;
