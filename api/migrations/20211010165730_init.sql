-- Add migration script here
CREATE TABLE guilds (
    id SERIAL PRIMARY KEY NOT NULL,
    guild_id TEXT NOT NULL,
    name TEXT NOT NULL,
    avatar_url TEXT,
    locale TEXT NOT NULL DEFAULT 'ja',
    UNIQUE (guild_id)
);

CREATE TABLE events (
    id SERIAL PRIMARY KEY NOT NULL,
    guild_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    notifications TEXT[] NOT NULL,
    color TEXT NOT NULL DEFAULT '#0000ff',
    is_all_day BOOLEAN NOT NULL DEFAULT FALSE,
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE event_settings (
    id SERIAL PRIMARY KEY NOT NULL,
    guild_id TEXT NOT NULL,
    channel_id TEXT NOT NULL
);
