BEGIN;

CREATE UNIQUE INDEX IF NOT EXISTS ideas_identity_idea_unique_idx
  ON ideas (underlying_identity_id)
  WHERE is_identity_idea = true
    AND underlying_identity_id IS NOT NULL;

COMMIT;
