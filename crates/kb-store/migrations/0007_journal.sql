-- user.db migration 7 (expansion M3, ADR 0034): the mistake journal and reflections.
-- A mistake is keyed by its submission's natural key (problem, language, time), so an
-- import merges it with the same submission. Times are INTEGER milliseconds, UTC.

CREATE TABLE mistakes (
    problem_id   TEXT NOT NULL,
    language     TEXT NOT NULL,
    submitted_at INTEGER NOT NULL,
    auto_tags    TEXT NOT NULL,             -- JSON array, set by KataBox
    tags         TEXT NOT NULL,             -- JSON array, the user's (starts as auto_tags)
    note         TEXT NOT NULL DEFAULT '',
    updated_at   INTEGER NOT NULL,
    PRIMARY KEY (problem_id, language, submitted_at)
) WITHOUT ROWID;

-- "Why it works" after Accepted, with the user's complexity picks.
CREATE TABLE reflections (
    problem_id  TEXT PRIMARY KEY NOT NULL,
    why         TEXT NOT NULL,
    time_class  TEXT,
    space_class TEXT,
    updated_at  INTEGER NOT NULL
) WITHOUT ROWID;
