BEGIN;

CREATE TABLE IF NOT EXISTS canonical_writer_verifications (
  account_id uuid PRIMARY KEY
    REFERENCES accounts(account_id) ON DELETE CASCADE,
  email_verified boolean NOT NULL DEFAULT false,
  canonical_writer_level smallint NOT NULL DEFAULT 0,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  CHECK (canonical_writer_level >= 0)
);

CREATE INDEX IF NOT EXISTS canonical_writer_verifications_level_idx
  ON canonical_writer_verifications (canonical_writer_level, email_verified);

INSERT INTO canonical_writer_verifications (account_id, email_verified, canonical_writer_level)
SELECT account_id, false, 0
FROM accounts
ON CONFLICT (account_id) DO NOTHING;

COMMIT;
