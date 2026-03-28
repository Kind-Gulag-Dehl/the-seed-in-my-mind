BEGIN;

ALTER TABLE auth_sessions
  ADD COLUMN expires_at timestamptz NOT NULL DEFAULT (now() + interval '30 days');

CREATE INDEX auth_sessions_expires_at_idx ON auth_sessions(expires_at);

COMMIT;
