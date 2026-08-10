BEGIN;

-- INTEGRATION-SEED-CONFORMANCE-BINDINGS-001 is a pre-genesis correction.
-- Existing nonconforming local data must be regenerated; no value is inferred
-- or backfilled by this migration.

ALTER TABLE representations
  ALTER COLUMN tier_complexity DROP NOT NULL,
  ADD COLUMN vocabulary_version_id uuid NULL;

ALTER TABLE representations
  ADD CONSTRAINT representations_slot_shape_check
  CHECK (
    (
      tier_enum = 0
      AND tier_complexity IS NULL
      AND vocabulary_version_id IS NULL
    )
    OR (
      tier_enum IN (1, 2, 3)
      AND tier_complexity IS NOT NULL
      AND tier_complexity IN (0, 1, 2, 3)
      AND (
        (tier_complexity = 3 AND vocabulary_version_id IS NOT NULL)
        OR (tier_complexity <> 3 AND vocabulary_version_id IS NULL)
      )
    )
  ),
  ADD CONSTRAINT representations_author_identity_fk
  FOREIGN KEY (author_identity_id)
    REFERENCES identities_s0(identity_id)
    ON DELETE RESTRICT
    DEFERRABLE INITIALLY DEFERRED,
  ADD CONSTRAINT representations_vocabulary_version_fk
  FOREIGN KEY (vocabulary_version_id)
    REFERENCES ideas(idea_id)
    ON DELETE RESTRICT
    DEFERRABLE INITIALLY DEFERRED;

CREATE INDEX representations_vocabulary_version_idx
  ON representations(vocabulary_version_id)
  WHERE vocabulary_version_id IS NOT NULL;

ALTER TABLE orderings
  ADD COLUMN subject_idea_id uuid NULL;

ALTER TABLE orderings
  DROP CONSTRAINT orderings_profile_vine_type_check,
  ADD CONSTRAINT orderings_profile_subject_check
  CHECK (
    (
      ordering_profile = 0
      AND vine_type IN (0, 1)
      AND subject_idea_id IS NULL
    )
    OR (
      ordering_profile IN (1, 2)
      AND vine_type IS NULL
      AND subject_idea_id IS NOT NULL
    )
  ),
  ADD CONSTRAINT orderings_subject_idea_fk
  FOREIGN KEY (subject_idea_id)
    REFERENCES ideas(idea_id)
    ON DELETE RESTRICT
    DEFERRABLE INITIALLY DEFERRED;

CREATE INDEX orderings_subject_idea_idx
  ON orderings(subject_idea_id)
  WHERE subject_idea_id IS NOT NULL;

ALTER TABLE ordering_items
  ADD COLUMN item_role smallint NULL,
  ADD CONSTRAINT ordering_items_role_check
  CHECK (item_role IS NULL OR item_role IN (0, 1, 2, 3));

CREATE OR REPLACE FUNCTION seed_validate_representation_conformance_bindings()
RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
  event_row record;
  author_event_row record;
  vocabulary_row record;
BEGIN
  SELECT
    e.event_id,
    e.speaker_identity_id,
    e.block_height,
    e.event_index
  INTO event_row
  FROM events e
  WHERE e.block_height = NEW.created_block_height
    AND e.event_index = NEW.created_event_index;

  IF NOT FOUND OR event_row.event_id <> NEW.created_event_id THEN
    RAISE EXCEPTION
      'representation % must reference its exact creation event',
      NEW.representation_id
      USING ERRCODE = '23514';
  END IF;

  IF event_row.speaker_identity_id IS NULL
     OR event_row.speaker_identity_id <> NEW.author_identity_id THEN
    RAISE EXCEPTION
      'representation % author must equal its event speaker',
      NEW.representation_id
      USING ERRCODE = '23514';
  END IF;

  SELECT e.block_height, e.event_index
  INTO author_event_row
  FROM identities_s0 i
  JOIN events e ON e.event_id = i.created_event_id
  WHERE i.identity_id = NEW.author_identity_id;

  IF NOT FOUND
     OR (author_event_row.block_height, author_event_row.event_index)
        >= (NEW.created_block_height, NEW.created_event_index) THEN
    RAISE EXCEPTION
      'representation % author identity must exist before use',
      NEW.representation_id
      USING ERRCODE = '23514';
  END IF;

  IF NEW.vocabulary_version_id IS NOT NULL THEN
    SELECT i.created_block_height, i.created_event_index
    INTO vocabulary_row
    FROM ideas i
    WHERE i.idea_id = NEW.vocabulary_version_id;

    IF NOT FOUND
       OR (vocabulary_row.created_block_height, vocabulary_row.created_event_index)
          >= (NEW.created_block_height, NEW.created_event_index) THEN
      RAISE EXCEPTION
        'representation % vocabulary idea must exist before use',
        NEW.representation_id
        USING ERRCODE = '23514';
    END IF;
  END IF;

  RETURN NEW;
END;
$$;

CREATE CONSTRAINT TRIGGER seed_representations_conformance_bindings
AFTER INSERT OR UPDATE ON representations
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION seed_validate_representation_conformance_bindings();

CREATE OR REPLACE FUNCTION seed_validate_ordering_conformance_bindings()
RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
  target_ordering_id uuid;
  ordering_row record;
  base_row record;
  subject_row record;
  item_count bigint;
  invalid_role_count bigint;
  distinct_role_count bigint;
  base_action_lane smallint;
  base_action_lane_max smallint;
  fork_action_lane smallint;
  fork_action_lane_max smallint;
BEGIN
  target_ordering_id := COALESCE(NEW.ordering_id, OLD.ordering_id);

  SELECT
    o.ordering_profile,
    o.vine_type,
    o.subject_idea_id,
    o.base_ordering_id,
    o.created_block_height,
    o.created_event_index
  INTO ordering_row
  FROM orderings o
  WHERE o.ordering_id = target_ordering_id;

  IF NOT FOUND THEN
    RETURN COALESCE(NEW, OLD);
  END IF;

  SELECT count(*)
  INTO item_count
  FROM ordering_items oi
  WHERE oi.ordering_id = target_ordering_id;

  IF ordering_row.ordering_profile = 0 THEN
    SELECT count(*)
    INTO invalid_role_count
    FROM ordering_items oi
    WHERE oi.ordering_id = target_ordering_id
      AND oi.item_role IS NOT NULL;

    IF invalid_role_count <> 0 THEN
      RAISE EXCEPTION
        'Vine ordering % must not carry item roles',
        target_ordering_id
        USING ERRCODE = '23514';
    END IF;
  ELSE
    SELECT i.idea_type, i.created_block_height, i.created_event_index
    INTO subject_row
    FROM ideas i
    WHERE i.idea_id = ordering_row.subject_idea_id;

    IF NOT FOUND
       OR (subject_row.created_block_height, subject_row.created_event_index)
          >= (ordering_row.created_block_height, ordering_row.created_event_index) THEN
      RAISE EXCEPTION
        'ordering % subject idea must exist before use',
        target_ordering_id
        USING ERRCODE = '23514';
    END IF;

    IF ordering_row.ordering_profile = 1
       AND subject_row.idea_type <> 'truth_claim' THEN
      RAISE EXCEPTION
        'Evidence Rail ordering % requires a truth_claim subject',
        target_ordering_id
        USING ERRCODE = '23514';
    END IF;

    IF ordering_row.ordering_profile = 2
       AND subject_row.idea_type <> 'actionable_idea' THEN
      RAISE EXCEPTION
        'Action Rail ordering % requires an actionable_idea subject',
        target_ordering_id
        USING ERRCODE = '23514';
    END IF;

    IF item_count = 0 THEN
      RAISE EXCEPTION
        'standardized ordering % requires at least one item',
        target_ordering_id
        USING ERRCODE = '23514';
    END IF;

    IF EXISTS (
      SELECT 1
      FROM ordering_items oi
      WHERE oi.ordering_id = target_ordering_id
      GROUP BY oi.idea_id
      HAVING count(*) > 1
    ) THEN
      RAISE EXCEPTION
        'standardized ordering % must not contain duplicate item IDs',
        target_ordering_id
        USING ERRCODE = '23514';
    END IF;

    IF ordering_row.ordering_profile = 1 THEN
      SELECT count(*)
      INTO invalid_role_count
      FROM ordering_items oi
      WHERE oi.ordering_id = target_ordering_id
        AND (oi.item_role IS NULL OR oi.item_role NOT IN (0, 1));
    ELSE
      SELECT count(*), count(DISTINCT oi.item_role)
      INTO invalid_role_count, distinct_role_count
      FROM ordering_items oi
      WHERE oi.ordering_id = target_ordering_id
        AND (oi.item_role IS NULL OR oi.item_role NOT IN (2, 3));

      SELECT count(DISTINCT oi.item_role)
      INTO distinct_role_count
      FROM ordering_items oi
      WHERE oi.ordering_id = target_ordering_id;
    END IF;

    IF invalid_role_count <> 0 THEN
      RAISE EXCEPTION
        'ordering % carries an invalid or missing item role',
        target_ordering_id
        USING ERRCODE = '23514';
    END IF;

    IF ordering_row.ordering_profile = 2 AND distinct_role_count <> 1 THEN
      RAISE EXCEPTION
        'Action Rail ordering % must use one homogeneous potential or proposed lane',
        target_ordering_id
        USING ERRCODE = '23514';
    END IF;

    IF ordering_row.ordering_profile = 2
       AND ordering_row.base_ordering_id IS NOT NULL THEN
      SELECT min(base_item.item_role), max(base_item.item_role)
      INTO base_action_lane, base_action_lane_max
      FROM ordering_items base_item
      WHERE base_item.ordering_id = ordering_row.base_ordering_id;

      SELECT min(fork_item.item_role), max(fork_item.item_role)
      INTO fork_action_lane, fork_action_lane_max
      FROM ordering_items fork_item
      WHERE fork_item.ordering_id = target_ordering_id;

      IF base_action_lane IS NULL
         OR base_action_lane IS DISTINCT FROM base_action_lane_max
         OR fork_action_lane IS NULL
         OR fork_action_lane IS DISTINCT FROM fork_action_lane_max
         OR fork_action_lane IS DISTINCT FROM base_action_lane THEN
        RAISE EXCEPTION
          'Action Rail fork % must preserve the base potential/proposed lane',
          target_ordering_id
          USING ERRCODE = '23514';
      END IF;
    END IF;
  END IF;

  IF ordering_row.base_ordering_id IS NOT NULL THEN
    SELECT o.ordering_profile, o.vine_type, o.subject_idea_id
    INTO base_row
    FROM orderings o
    WHERE o.ordering_id = ordering_row.base_ordering_id;

    IF NOT FOUND
       OR base_row.ordering_profile <> ordering_row.ordering_profile
       OR base_row.vine_type IS DISTINCT FROM ordering_row.vine_type
       OR base_row.subject_idea_id IS DISTINCT FROM ordering_row.subject_idea_id THEN
      RAISE EXCEPTION
        'ordering fork % must preserve profile, vine type, and subject',
        target_ordering_id
        USING ERRCODE = '23514';
    END IF;

    IF EXISTS (
      SELECT 1
      FROM ordering_items base_item
      JOIN ordering_items fork_item
        ON fork_item.ordering_id = target_ordering_id
       AND fork_item.idea_id = base_item.idea_id
      WHERE base_item.ordering_id = ordering_row.base_ordering_id
        AND fork_item.item_role IS DISTINCT FROM base_item.item_role
    ) THEN
      RAISE EXCEPTION
        'ordering fork % must preserve retained-item roles',
        target_ordering_id
        USING ERRCODE = '23514';
    END IF;
  END IF;

  RETURN COALESCE(NEW, OLD);
END;
$$;

CREATE CONSTRAINT TRIGGER seed_orderings_conformance_bindings
AFTER INSERT OR UPDATE ON orderings
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION seed_validate_ordering_conformance_bindings();

CREATE CONSTRAINT TRIGGER seed_ordering_items_conformance_bindings
AFTER INSERT OR UPDATE OR DELETE ON ordering_items
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION seed_validate_ordering_conformance_bindings();

COMMIT;
