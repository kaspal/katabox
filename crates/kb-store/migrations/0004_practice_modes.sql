-- user.db migration 4 (ADR 0020): problem opens (mock interviews pick problems never
-- opened) and mock interview sessions. Times are INTEGER milliseconds since the Unix
-- epoch, UTC.

CREATE TABLE problem_opens (
    problem_id      TEXT PRIMARY KEY NOT NULL,
    first_opened_at INTEGER NOT NULL,
    last_opened_at  INTEGER NOT NULL
) WITHOUT ROWID;

CREATE TABLE interview_sessions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at  INTEGER NOT NULL,
    duration_s  INTEGER NOT NULL,
    problem_ids TEXT NOT NULL,  -- JSON array of problem ids
    languages   TEXT NOT NULL,  -- JSON array of language ids
    mix         TEXT NOT NULL,  -- e.g. "easy+medium"
    ended_at    INTEGER,        -- NULL while active
    results     TEXT            -- JSON scorecard, set when it ends
);
CREATE UNIQUE INDEX interview_sessions_by_start ON interview_sessions (started_at);
