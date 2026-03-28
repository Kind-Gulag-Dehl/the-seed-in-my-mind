BEGIN;

CREATE TABLE blocks (
  block_height bigint PRIMARY KEY,
  block_hash text NOT NULL,
  prev_block_hash text NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (block_height >= 0),
  CHECK (block_hash ~ '^[0-9a-f]+$'),
  CHECK (prev_block_hash IS NULL OR prev_block_hash ~ '^[0-9a-f]+$')
);

CREATE TABLE events (
  block_height bigint NOT NULL REFERENCES blocks(block_height) ON DELETE CASCADE,
  event_index int NOT NULL,
  event_id uuid NOT NULL UNIQUE,
  event_type text NOT NULL,
  speaker_identity_id uuid NULL,
  payload_json jsonb NOT NULL,
  signature text NULL,
  ingested_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (block_height, event_index),
  CHECK (block_height >= 0),
  CHECK (event_index >= 0)
);

CREATE TABLE ideas (
  idea_id uuid PRIMARY KEY,
  idea_type text NOT NULL,
  speaker_identity_id uuid NOT NULL,
  created_block_height bigint NOT NULL,
  created_event_index int NOT NULL,
  created_event_id uuid NOT NULL UNIQUE,
  FOREIGN KEY (created_block_height, created_event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE,
  CHECK (created_block_height >= 0),
  CHECK (created_event_index >= 0)
);

CREATE TABLE connections (
  connection_id uuid PRIMARY KEY,
  from_idea_id uuid NOT NULL REFERENCES ideas(idea_id) ON DELETE CASCADE,
  to_idea_id uuid NOT NULL REFERENCES ideas(idea_id) ON DELETE CASCADE,
  connection_type text NOT NULL,
  usage text NULL,
  axis text NULL,
  timeframe text NULL,
  scope text NULL,
  created_block_height bigint NOT NULL,
  created_event_index int NOT NULL,
  created_by_event_id uuid NOT NULL UNIQUE,
  FOREIGN KEY (created_block_height, created_event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE,
  CHECK (created_block_height >= 0),
  CHECK (created_event_index >= 0)
);

CREATE TABLE snapshots (
  snapshot_id uuid PRIMARY KEY,
  block_height bigint NOT NULL UNIQUE REFERENCES blocks(block_height),
  format_version text NOT NULL,
  snapshot_hash text NOT NULL,
  prev_snapshot_hash text NULL,
  state_root_hash text NULL,
  title_sentence_payload_root text NULL,
  shared_map_commitment text NULL,
  active_rulebook_set_hash text NULL,
  last_event_id uuid NULL,
  event_count bigint NULL,
  approximate_timestamp timestamptz NULL,
  cycle_index bigint NULL,
  cycle_close_height bigint NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  CHECK (block_height >= 0),
  CHECK (snapshot_hash ~ '^[0-9a-f]+$'),
  CHECK (prev_snapshot_hash IS NULL OR prev_snapshot_hash ~ '^[0-9a-f]+$'),
  CHECK (state_root_hash IS NULL OR state_root_hash ~ '^[0-9a-f]+$'),
  CHECK (title_sentence_payload_root IS NULL OR title_sentence_payload_root ~ '^[0-9a-f]+$'),
  CHECK (shared_map_commitment IS NULL OR shared_map_commitment ~ '^[0-9a-f]+$'),
  CHECK (active_rulebook_set_hash IS NULL OR active_rulebook_set_hash ~ '^[0-9a-f]+$'),
  CHECK (event_count IS NULL OR event_count >= 0),
  CHECK (cycle_index IS NULL OR cycle_index >= 0),
  CHECK (cycle_close_height IS NULL OR cycle_close_height >= 0)
);

COMMIT;
