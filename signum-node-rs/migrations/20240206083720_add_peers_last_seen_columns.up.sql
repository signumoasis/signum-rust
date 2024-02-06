-- Add up migration script here
ALTER TABLE peers
ADD COLUMN last_seen DATETIME;

ALTER TABLE peers
ADD COLUMN attempts_since_last_seen INTEGER NOT NULL DEFAULT 0;
