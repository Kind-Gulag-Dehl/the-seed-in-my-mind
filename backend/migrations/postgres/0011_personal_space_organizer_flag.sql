BEGIN;

ALTER TABLE ideas
  ADD COLUMN IF NOT EXISTS is_personal_space_organizer boolean NOT NULL DEFAULT false;

WITH organizer_targets AS (
  SELECT DISTINCT c.to_idea_id
  FROM connections c
  JOIN ideas from_idea ON from_idea.idea_id = c.from_idea_id
  WHERE c.connection_type = 'membership'
    AND c.usage = 'has_space'
    AND from_idea.is_identity_idea = true
)
UPDATE ideas i
SET is_personal_space_organizer = true
FROM organizer_targets t
WHERE i.idea_id = t.to_idea_id
  AND i.is_personal_space_organizer = false;

COMMIT;
