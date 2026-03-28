BEGIN;

CREATE TABLE IF NOT EXISTS representations (
  representation_id uuid PRIMARY KEY,
  target_kind smallint NOT NULL,
  target_id uuid NOT NULL,
  tier_enum smallint NOT NULL,
  tier_complexity smallint NOT NULL,
  payload_hash text NOT NULL,
  payload_text text NULL,
  author_identity_id uuid NOT NULL,
  language_locale text NULL,
  provenance text NULL,
  created_block_height bigint NOT NULL,
  created_event_index int NOT NULL,
  created_event_id uuid NOT NULL UNIQUE,
  FOREIGN KEY (created_block_height, created_event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE,
  CHECK (target_kind IN (0, 1)),
  CHECK (tier_enum IN (0, 1, 2, 3)),
  CHECK (tier_complexity >= 0 AND tier_complexity <= 3),
  CHECK (created_block_height >= 0),
  CHECK (created_event_index >= 0),
  CHECK (payload_hash ~ '^[0-9a-f]{64}$')
);

CREATE INDEX IF NOT EXISTS representations_target_slot_idx
  ON representations (
    target_kind,
    target_id,
    tier_enum,
    tier_complexity,
    created_block_height,
    created_event_index
  );

CREATE INDEX IF NOT EXISTS representations_created_order_idx
  ON representations (created_block_height, created_event_index);

ALTER TABLE ideas
  ADD COLUMN IF NOT EXISTS title_representation_id uuid NULL,
  ADD COLUMN IF NOT EXISTS sentence_representation_id uuid NULL;

DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_constraint
    WHERE conname = 'ideas_title_representation_fk'
      AND conrelid = 'ideas'::regclass
  ) THEN
    ALTER TABLE ideas
      ADD CONSTRAINT ideas_title_representation_fk
      FOREIGN KEY (title_representation_id)
      REFERENCES representations(representation_id)
      ON DELETE SET NULL;
  END IF;
END $$;

DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_constraint
    WHERE conname = 'ideas_sentence_representation_fk'
      AND conrelid = 'ideas'::regclass
  ) THEN
    ALTER TABLE ideas
      ADD CONSTRAINT ideas_sentence_representation_fk
      FOREIGN KEY (sentence_representation_id)
      REFERENCES representations(representation_id)
      ON DELETE SET NULL;
  END IF;
END $$;

CREATE TABLE IF NOT EXISTS rails (
  rail_id uuid PRIMARY KEY,
  rail_kind smallint NOT NULL,
  vine_type smallint NULL,
  speaker_identity_id uuid NOT NULL,
  created_block_height bigint NOT NULL,
  created_event_index int NOT NULL,
  created_event_id uuid NOT NULL UNIQUE,
  base_rail_id uuid NULL,
  title_representation_id uuid NULL,
  sentence_representation_id uuid NULL,
  FOREIGN KEY (created_block_height, created_event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE,
  FOREIGN KEY (base_rail_id)
    REFERENCES rails(rail_id) ON DELETE RESTRICT,
  FOREIGN KEY (title_representation_id)
    REFERENCES representations(representation_id) ON DELETE SET NULL,
  FOREIGN KEY (sentence_representation_id)
    REFERENCES representations(representation_id) ON DELETE SET NULL,
  CHECK (rail_kind = 0),
  CHECK (vine_type IS NULL OR vine_type IN (0, 1)),
  CHECK (created_block_height >= 0),
  CHECK (created_event_index >= 0)
);

CREATE UNIQUE INDEX IF NOT EXISTS rails_created_order_key
  ON rails (created_block_height, created_event_index);

CREATE INDEX IF NOT EXISTS rails_speaker_identity_idx
  ON rails (speaker_identity_id);

CREATE INDEX IF NOT EXISTS rails_base_rail_idx
  ON rails (base_rail_id);

CREATE TABLE IF NOT EXISTS rail_items (
  rail_id uuid NOT NULL,
  idx int NOT NULL,
  idea_id uuid NOT NULL,
  via_connection_id uuid NULL,
  PRIMARY KEY (rail_id, idx),
  FOREIGN KEY (rail_id) REFERENCES rails(rail_id) ON DELETE CASCADE,
  FOREIGN KEY (idea_id) REFERENCES ideas(idea_id) ON DELETE CASCADE,
  CHECK (idx >= 0)
);

CREATE INDEX IF NOT EXISTS rail_items_idea_idx
  ON rail_items (idea_id);

CREATE INDEX IF NOT EXISTS rail_items_via_connection_idx
  ON rail_items (via_connection_id)
  WHERE via_connection_id IS NOT NULL;

COMMIT;
