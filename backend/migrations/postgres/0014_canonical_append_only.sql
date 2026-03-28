BEGIN;

CREATE OR REPLACE FUNCTION seed_enforce_canonical_append_only()
RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
  allow_mutation text;
BEGIN
  allow_mutation := current_setting('seed.allow_canonical_mutation', true);
  IF allow_mutation = 'on' THEN
    IF TG_OP = 'UPDATE' THEN
      RETURN NEW;
    END IF;
    RETURN OLD;
  END IF;

  RAISE EXCEPTION 'canonical table "%" is append-only; % is blocked', TG_TABLE_NAME, TG_OP
    USING ERRCODE = '42501';
END;
$$;

DROP TRIGGER IF EXISTS seed_blocks_append_only ON blocks;
CREATE TRIGGER seed_blocks_append_only
BEFORE UPDATE OR DELETE ON blocks
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_events_append_only ON events;
CREATE TRIGGER seed_events_append_only
BEFORE UPDATE OR DELETE ON events
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_ideas_append_only ON ideas;
CREATE TRIGGER seed_ideas_append_only
BEFORE UPDATE OR DELETE ON ideas
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_connections_append_only ON connections;
CREATE TRIGGER seed_connections_append_only
BEFORE UPDATE OR DELETE ON connections
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_snapshots_append_only ON snapshots;
CREATE TRIGGER seed_snapshots_append_only
BEFORE UPDATE OR DELETE ON snapshots
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_identities_s0_append_only ON identities_s0;
CREATE TRIGGER seed_identities_s0_append_only
BEFORE UPDATE OR DELETE ON identities_s0
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_representations_append_only ON representations;
CREATE TRIGGER seed_representations_append_only
BEFORE UPDATE OR DELETE ON representations
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_rails_append_only ON rails;
CREATE TRIGGER seed_rails_append_only
BEFORE UPDATE OR DELETE ON rails
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

DROP TRIGGER IF EXISTS seed_rail_items_append_only ON rail_items;
CREATE TRIGGER seed_rail_items_append_only
BEFORE UPDATE OR DELETE ON rail_items
FOR EACH ROW
EXECUTE FUNCTION seed_enforce_canonical_append_only();

COMMIT;
