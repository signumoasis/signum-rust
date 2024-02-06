-- Add down migration script here
ALTER TABLE peers
DROP COLUMN last_seen;

ALTER TABLE peers
DROP COLUMN attempts_since_last_seen;
