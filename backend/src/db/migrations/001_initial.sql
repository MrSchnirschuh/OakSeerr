-- Initial schema for OakSeerr
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL DEFAULT '',
    email TEXT,
    avatar_url TEXT,
    jellyfin_user_id TEXT,
    permissions INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS media (
    id TEXT PRIMARY KEY NOT NULL,
    tmdb_id INTEGER,
    tvdb_id INTEGER,
    musicbrainz_id TEXT,
    isbn TEXT,
    media_type TEXT NOT NULL CHECK(media_type IN ('movie', 'tv', 'music', 'book', 'comic')),
    title TEXT NOT NULL,
    overview TEXT,
    poster_url TEXT,
    backdrop_url TEXT,
    release_date TEXT,
    status TEXT NOT NULL DEFAULT 'unknown' CHECK(status IN ('unknown', 'available', 'requested', 'processing')),
    rating REAL,
    genres TEXT,
    season_count INTEGER,
    episode_count INTEGER,
    artist_name TEXT,
    author_name TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS media_requests (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    media_type TEXT NOT NULL CHECK(media_type IN ('movie', 'tv', 'music', 'book', 'comic')),
    media_id TEXT NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'approved', 'declined', 'fulfilled')),
    download_status TEXT NOT NULL DEFAULT 'none' CHECK(download_status IN ('none', 'queued', 'downloading', 'imported', 'failed')),
    external_service_id TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS integrations (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    integration_type TEXT NOT NULL CHECK(integration_type IN ('radarr', 'sonarr', 'lidarr', 'readarr', 'mylar3')),
    base_url TEXT NOT NULL,
    api_key TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
);

-- Default settings
INSERT OR IGNORE INTO settings (key, value) VALUES ('app_name', 'OakSeerr');
INSERT OR IGNORE INTO settings (key, value) VALUES ('app_theme', 'default');
INSERT OR IGNORE INTO settings (key, value) VALUES ('jellyfin_url', '');
INSERT OR IGNORE INTO settings (key, value) VALUES ('sso_enabled', 'false');
INSERT OR IGNORE INTO settings (key, value) VALUES ('TMDB_API_KEY', '');
INSERT OR IGNORE INTO settings (key, value) VALUES ('LASTFM_API_KEY', '');
INSERT OR IGNORE INTO settings (key, value) VALUES ('COMICVINE_API_KEY', '');
