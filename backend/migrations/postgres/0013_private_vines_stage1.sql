BEGIN;

CREATE TABLE private_vines (
  private_vine_id uuid PRIMARY KEY,
  owner_account_id uuid NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
  vine_type smallint NOT NULL,
  title text NULL,
  sentence text NULL,
  paragraph text NULL,
  "full" text NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  CHECK (vine_type IN (0, 1))
);

CREATE INDEX private_vines_owner_updated_idx
  ON private_vines(owner_account_id, updated_at DESC, private_vine_id ASC);

CREATE TABLE private_vine_items (
  private_vine_id uuid NOT NULL REFERENCES private_vines(private_vine_id) ON DELETE CASCADE,
  idx int NOT NULL,
  idea_id uuid NOT NULL,
  via_connection_id uuid NULL,
  PRIMARY KEY (private_vine_id, idx),
  CHECK (idx >= 0)
);

CREATE INDEX private_vine_items_idea_idx ON private_vine_items(idea_id);
CREATE INDEX private_vine_items_vine_idx ON private_vine_items(private_vine_id);

COMMIT;
