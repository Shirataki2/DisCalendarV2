-- Add migration script here
CREATE TABLE guild_config (
    guild_id TEXT PRIMARY KEY NOT NULL,
    restricted BOOLEAN NOT NULL DEFAULT FALSE
);
