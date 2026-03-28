BEGIN;

CREATE TABLE accounts (
  account_id uuid PRIMARY KEY,
  username text NOT NULL UNIQUE,
  password_hash text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE auth_sessions (
  session_id uuid PRIMARY KEY,
  account_id uuid NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
  token text NOT NULL UNIQUE,
  created_at timestamptz NOT NULL DEFAULT now(),
  last_seen_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX auth_sessions_token_idx ON auth_sessions(token);
CREATE INDEX auth_sessions_account_idx ON auth_sessions(account_id);

CREATE TABLE private_ideas (
  idea_id uuid PRIMARY KEY,
  owner_account_id uuid NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
  title text NOT NULL,
  sentence text NOT NULL,
  paragraph text NULL,
  "full" text NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX private_ideas_owner_updated_idx ON private_ideas(owner_account_id, updated_at DESC);

COMMIT;
