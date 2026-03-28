BEGIN;

CREATE TABLE IF NOT EXISTS tempo_predicates (
  block_height bigint NOT NULL,
  event_index int NOT NULL,
  cycle_age_ge_dmin boolean NOT NULL,
  cycle_age_ge_dmax boolean NOT NULL,
  constrained_mode boolean NOT NULL DEFAULT false,
  record_only_mode boolean NOT NULL DEFAULT false,
  PRIMARY KEY (block_height, event_index),
  FOREIGN KEY (block_height, event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE,
  CHECK (block_height >= 0),
  CHECK (event_index >= 0)
);

CREATE TABLE IF NOT EXISTS cycle_boundaries (
  cycle_index bigint PRIMARY KEY,
  closure_kind smallint NOT NULL,
  forced_seal boolean NOT NULL,
  closure_block_height bigint NOT NULL,
  source_block_height bigint NOT NULL,
  source_event_index int NOT NULL,
  source_event_id uuid NOT NULL UNIQUE,
  created_at timestamptz NOT NULL DEFAULT now(),
  FOREIGN KEY (source_block_height, source_event_index)
    REFERENCES events(block_height, event_index) ON DELETE CASCADE,
  CHECK (cycle_index >= 0),
  CHECK (closure_kind IN (0, 1)),
  CHECK (closure_block_height >= 0),
  CHECK (source_block_height >= 0),
  CHECK (source_event_index >= 0),
  CHECK (
    (closure_kind = 0 AND forced_seal = false)
    OR (closure_kind = 1 AND forced_seal = true)
  )
);

CREATE INDEX IF NOT EXISTS cycle_boundaries_source_order_idx
  ON cycle_boundaries (source_block_height, source_event_index);

CREATE INDEX IF NOT EXISTS cycle_boundaries_closure_height_idx
  ON cycle_boundaries (closure_block_height);

INSERT INTO identities_s0 (identity_id, title, created_event_id)
VALUES (
  'ffffffff-ffff-7fff-bfff-ffffffffffff',
  'system_boundary_emitter',
  'ffffffff-ffff-7fff-bfff-ffffffffffff'
)
ON CONFLICT (identity_id) DO NOTHING;

DROP TRIGGER IF EXISTS seed_tempo_predicates_append_only ON tempo_predicates;
CREATE TRIGGER seed_tempo_predicates_append_only
BEFORE UPDATE OR DELETE ON tempo_predicates
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_cycle_boundaries_append_only ON cycle_boundaries;
CREATE TRIGGER seed_cycle_boundaries_append_only
BEFORE UPDATE OR DELETE ON cycle_boundaries
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

COMMIT;
