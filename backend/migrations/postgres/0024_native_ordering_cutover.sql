BEGIN;

-- DEC-043 is an intentional pre-launch wire/schema break. The old Rail and
-- private Vine tables are renamed in place only so existing disposable local
-- databases can be inspected before they are regenerated. No compatibility
-- views, aliases, or dual-write paths are created.

ALTER TABLE rails RENAME TO orderings;
ALTER TABLE rail_items RENAME TO ordering_items;
ALTER TABLE private_vines RENAME TO private_orderings;
ALTER TABLE private_vine_items RENAME TO private_ordering_items;

ALTER TABLE orderings RENAME COLUMN rail_id TO ordering_id;
ALTER TABLE orderings RENAME COLUMN rail_kind TO ordering_profile;
ALTER TABLE orderings RENAME COLUMN base_rail_id TO base_ordering_id;
ALTER TABLE ordering_items RENAME COLUMN rail_id TO ordering_id;
ALTER TABLE private_orderings RENAME COLUMN private_vine_id TO private_ordering_id;
ALTER TABLE private_ordering_items RENAME COLUMN private_vine_id TO private_ordering_id;

ALTER TABLE orderings DROP CONSTRAINT rails_rail_kind_check;
ALTER TABLE orderings
  ADD CONSTRAINT orderings_profile_vine_type_check
  CHECK (
    (ordering_profile = 0 AND vine_type IN (0, 1))
    OR (ordering_profile IN (1, 2) AND vine_type IS NULL)
  );

ALTER TABLE private_orderings
  ADD COLUMN ordering_profile smallint NOT NULL DEFAULT 0;
ALTER TABLE private_orderings ALTER COLUMN vine_type DROP NOT NULL;
ALTER TABLE private_orderings DROP CONSTRAINT private_vines_vine_type_check;
ALTER TABLE private_orderings
  ADD CONSTRAINT private_orderings_profile_vine_type_check
  CHECK (
    (ordering_profile = 0 AND vine_type IN (0, 1))
    OR (ordering_profile IN (1, 2) AND vine_type IS NULL)
  );

DO $$
DECLARE
  target_table text;
  constraint_row record;
  renamed text;
BEGIN
  FOREACH target_table IN ARRAY ARRAY[
    'orderings',
    'ordering_items',
    'private_orderings',
    'private_ordering_items'
  ]
  LOOP
    FOR constraint_row IN
      SELECT conname
      FROM pg_constraint
      WHERE conrelid = target_table::regclass
        AND (conname LIKE '%rail%' OR conname LIKE '%vine%')
    LOOP
      renamed := replace(
        replace(
          replace(
            replace(constraint_row.conname, 'private_vines', 'private_orderings'),
            'private_vine',
            'private_ordering'
          ),
          'rails',
          'orderings'
        ),
        'rail',
        'ordering'
      );
      IF renamed <> constraint_row.conname THEN
        EXECUTE format(
          'ALTER TABLE %I RENAME CONSTRAINT %I TO %I',
          target_table,
          constraint_row.conname,
          renamed
        );
      END IF;
    END LOOP;
  END LOOP;
END
$$;

DO $$
DECLARE
  index_row record;
  renamed text;
BEGIN
  FOR index_row IN
    SELECT schemaname, indexname
    FROM pg_indexes
    WHERE schemaname = 'public'
      AND tablename IN (
        'orderings',
        'ordering_items',
        'private_orderings',
        'private_ordering_items'
      )
      AND (indexname LIKE '%rail%' OR indexname LIKE '%vine%')
  LOOP
    renamed := replace(
      replace(
        replace(
          replace(index_row.indexname, 'private_vines', 'private_orderings'),
          'private_vine',
          'private_ordering'
        ),
        'rails',
        'orderings'
      ),
      'rail',
      'ordering'
    );
    IF renamed <> index_row.indexname THEN
      EXECUTE format(
        'ALTER INDEX %I.%I RENAME TO %I',
        index_row.schemaname,
        index_row.indexname,
        renamed
      );
    END IF;
  END LOOP;
END
$$;

ALTER TRIGGER seed_rails_append_only ON orderings
  RENAME TO seed_orderings_append_only;
ALTER TRIGGER seed_rail_items_append_only ON ordering_items
  RENAME TO seed_ordering_items_append_only;

COMMIT;
