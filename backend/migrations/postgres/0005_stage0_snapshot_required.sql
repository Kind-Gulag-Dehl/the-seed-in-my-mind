BEGIN;

DELETE FROM snapshots
WHERE state_root_hash IS NULL
   OR title_sentence_payload_root IS NULL
   OR shared_map_commitment IS NULL
   OR event_count IS NULL
   OR approximate_timestamp IS NULL;

ALTER TABLE snapshots
  ALTER COLUMN state_root_hash SET NOT NULL,
  ALTER COLUMN title_sentence_payload_root SET NOT NULL,
  ALTER COLUMN shared_map_commitment SET NOT NULL,
  ALTER COLUMN event_count SET NOT NULL,
  ALTER COLUMN approximate_timestamp SET NOT NULL;

COMMIT;
