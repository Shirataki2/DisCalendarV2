-- Add migration script here
CREATE TABLE guild_config IF NOT EXISTS (
    guild_id TEXT PRIMARY KEY NOT NULL,
    restricted BOOLEAN NOT NULL DEFAULT FALSE
);
