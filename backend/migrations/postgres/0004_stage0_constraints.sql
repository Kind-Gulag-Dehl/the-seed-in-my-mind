BEGIN;

DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_constraint
    WHERE conname = 'ideas_created_order_key'
      AND conrelid = 'ideas'::regclass
  ) THEN
    ALTER TABLE ideas
      ADD CONSTRAINT ideas_created_order_key UNIQUE (created_block_height, created_event_index);
  END IF;
END $$;

DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_constraint
    WHERE conname = 'connections_created_order_key'
      AND conrelid = 'connections'::regclass
  ) THEN
    ALTER TABLE connections
      ADD CONSTRAINT connections_created_order_key UNIQUE (created_block_height, created_event_index);
  END IF;
END $$;

COMMIT;
