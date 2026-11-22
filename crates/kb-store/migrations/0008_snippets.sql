-- user.db migration 8 (expansion M4, ADR 0034): the personal snippet library. Keyed by
-- a random id so imports merge the same snippet across machines. Times are INTEGER
-- milliseconds since the Unix epoch, UTC.

CREATE TABLE snippets (
    uid        TEXT PRIMARY KEY NOT NULL,  -- 16 hex characters
    language   TEXT NOT NULL,
    name       TEXT NOT NULL,
    tags       TEXT NOT NULL,              -- JSON array
    body       TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
) WITHOUT ROWID;
CREATE INDEX snippets_by_language ON snippets (language, name);
