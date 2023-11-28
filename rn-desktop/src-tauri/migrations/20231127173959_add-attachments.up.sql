-- Add up migration script here
CREATE TABLE IF NOT EXISTS Media (
    id TEXT PRIMARY KEY NOT NULL,
    mime_type TEXT NOT NULL,
    blob_hash TEXT NOT NULL
) STRICT;
CREATE TABLE IF NOT EXISTS Blob (
    `hash` TEXT PRIMARY KEY NOT NULL,
    `blob` BLOB NOT NULL
) STRICT;
