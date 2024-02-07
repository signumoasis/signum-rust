-- Add up migration script here
CREATE TABLE IF NOT EXISTS peers (
    peer_announced_address TEXT PRIMARY KEY NOT NULL,
    peer_ip_address TEXT UNIQUE,
    application TEXT,
    version TEXT,
    platform TEXT,
    share_address BOOLEAN,
    network TEXT,
    blacklist_until DATETIME,
    blacklist_count INTEGER NOT NULL DEFAULT 0
);
