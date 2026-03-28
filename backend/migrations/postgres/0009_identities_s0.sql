BEGIN;

CREATE TABLE identities_s0 (
  identity_id uuid PRIMARY KEY,
  title text NOT NULL,
  created_event_id uuid NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX identities_s0_title_ci ON identities_s0 (lower(title));

ALTER TABLE ideas
  ADD COLUMN is_identity_idea boolean NOT NULL DEFAULT false,
  ADD COLUMN underlying_identity_id uuid NULL;

CREATE INDEX ideas_identity_idea_idx
  ON ideas (underlying_identity_id)
  WHERE is_identity_idea = true;

ALTER TABLE accounts
  ADD COLUMN canonical_identity_id uuid NULL;

CREATE INDEX accounts_canonical_identity_idx
  ON accounts (canonical_identity_id);

COMMIT;
