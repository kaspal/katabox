-- user.db migration 1 (ADR 0010). Never edit a released migration; add a new file.
-- Times are INTEGER milliseconds since the Unix epoch, UTC.

CREATE TABLE settings (
    key        TEXT PRIMARY KEY NOT NULL,
    value      TEXT NOT NULL, -- JSON
    updated_at INTEGER NOT NULL
) WITHOUT ROWID;

CREATE TABLE drafts (
    problem_id TEXT NOT NULL,
    language   TEXT NOT NULL,
    code       TEXT NOT NULL,
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (problem_id, language)
) WITHOUT ROWID;

CREATE TABLE submissions (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    problem_id TEXT NOT NULL,
    language   TEXT NOT NULL,
    code       TEXT NOT NULL,
    verdict    TEXT NOT NULL,
    time_us    INTEGER NOT NULL,
    memory_kb  INTEGER NOT NULL,
    report     TEXT NOT NULL, -- the full kb-judge Report as JSON
    created_at INTEGER NOT NULL
);
CREATE INDEX submissions_by_problem ON submissions (problem_id, id DESC);

CREATE TABLE problem_status (
    problem_id      TEXT PRIMARY KEY NOT NULL,
    status          TEXT NOT NULL CHECK (status IN ('attempted', 'solved')),
    first_solved_at INTEGER,
    updated_at      INTEGER NOT NULL
) WITHOUT ROWID;

CREATE TABLE snapshots (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    problem_id TEXT NOT NULL,
    language   TEXT NOT NULL,
    code       TEXT NOT NULL,
    code_hash  TEXT NOT NULL, -- SHA-256 hex of code
    source     TEXT NOT NULL CHECK (source IN ('run', 'submit', 'restore')),
    created_at INTEGER NOT NULL
);
CREATE UNIQUE INDEX snapshots_dedupe ON snapshots (problem_id, language, code_hash);
CREATE INDEX snapshots_by_problem ON snapshots (problem_id, id DESC);

CREATE TABLE notes (
    problem_id TEXT PRIMARY KEY NOT NULL,
    body       TEXT NOT NULL, -- Markdown
    updated_at INTEGER NOT NULL
) WITHOUT ROWID;

CREATE TABLE hint_unlocks (
    problem_id  TEXT NOT NULL,
    tier        INTEGER NOT NULL,
    unlocked_at INTEGER NOT NULL,
    PRIMARY KEY (problem_id, tier)
) WITHOUT ROWID;
