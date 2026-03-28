BEGIN;

DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_constraint
    WHERE conname = 'blocks_block_hash_key'
      AND conrelid = 'blocks'::regclass
  ) THEN
    ALTER TABLE blocks
      ADD CONSTRAINT blocks_block_hash_key UNIQUE (block_hash);
  END IF;
END $$;

DROP INDEX IF EXISTS events_event_id_idx;

CREATE INDEX IF NOT EXISTS snapshots_block_height_desc_idx
  ON snapshots (block_height DESC);

COMMIT;
