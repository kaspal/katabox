-- content.db schema (ADR 0010). Built by `kb-pack build`, read-only at runtime.
-- Changing this file means bumping CONTENT_SCHEMA_VERSION in src/content.rs.

CREATE TABLE content_meta (
    key   TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
) WITHOUT ROWID;

CREATE TABLE problems (
    id            TEXT PRIMARY KEY NOT NULL,
    difficulty    TEXT NOT NULL CHECK (difficulty IN ('easy', 'medium', 'hard')),
    pattern       TEXT NOT NULL,
    rank          INTEGER NOT NULL,
    -- {"name": ..., "params": [{"name": ..., "type": ...}], "returns": ...}
    signature     TEXT NOT NULL,
    checker       TEXT NOT NULL,
    time_limit_ms INTEGER NOT NULL,
    memory_mb     INTEGER NOT NULL,
    profile       INTEGER NOT NULL DEFAULT 0,
    -- validate.py (constraint checks for custom tests, run in the sandbox; ADR 0016)
    validator     TEXT
) WITHOUT ROWID;

CREATE TABLE problem_tags (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    tag        TEXT NOT NULL,
    PRIMARY KEY (problem_id, tag)
) WITHOUT ROWID;
CREATE INDEX problem_tags_by_tag ON problem_tags (tag);

-- One row per problem per UI language ('en' always present).
CREATE TABLE problem_texts (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    lang       TEXT NOT NULL,
    title      TEXT NOT NULL,
    statement  TEXT NOT NULL,
    editorial  TEXT,
    PRIMARY KEY (problem_id, lang)
) WITHOUT ROWID;

CREATE TABLE hints (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    lang       TEXT NOT NULL,
    tier       INTEGER NOT NULL,
    body       TEXT NOT NULL,
    PRIMARY KEY (problem_id, lang, tier)
) WITHOUT ROWID;

CREATE TABLE tests (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    kind       TEXT NOT NULL CHECK (kind IN ('public', 'hidden')),
    idx        INTEGER NOT NULL,
    -- JSON compressed with zstd, one blob each (ADR 0018; kb_store::blob)
    input      BLOB NOT NULL,
    output     BLOB NOT NULL,
    PRIMARY KEY (problem_id, kind, idx)
) WITHOUT ROWID;

CREATE TABLE reference_solutions (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    language   TEXT NOT NULL,
    code       TEXT NOT NULL,
    PRIMARY KEY (problem_id, language)
) WITHOUT ROWID;

-- Full-text search over both languages; one row per problem per language.
CREATE VIRTUAL TABLE problem_fts USING fts5 (
    problem_id UNINDEXED,
    lang UNINDEXED,
    title,
    tags,
    statement,
    tokenize = 'unicode61 remove_diacritics 2'
);
