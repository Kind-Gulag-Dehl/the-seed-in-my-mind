BEGIN;

ALTER TABLE snapshots
  ADD COLUMN artifact_path text NULL,
  ADD COLUMN artifact_sha256 text NULL;

ALTER TABLE snapshots
  ADD CONSTRAINT snapshots_artifact_sha256_format CHECK (
    artifact_sha256 IS NULL OR (
      artifact_sha256 ~ '^[0-9a-f]+$' AND char_length(artifact_sha256) = 64
    )
  );

COMMIT;
