BEGIN;

-- Invalidate all existing sessions that were persisted with plaintext tokens.
DELETE FROM auth_sessions;

ALTER TABLE auth_sessions
  DROP COLUMN token;

ALTER TABLE auth_sessions
  ADD COLUMN token_hash text NOT NULL UNIQUE;

CREATE INDEX auth_sessions_token_hash_idx ON auth_sessions(token_hash);

COMMIT;
