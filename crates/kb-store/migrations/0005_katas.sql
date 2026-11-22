-- user.db migration 5 (expansion M1, ADR 0033): kata drills. A kata re-solves solved
-- problems from memory against the clock; its attempts are kept apart from ordinary
-- submissions (no submissions row, status, review or draft). Times are INTEGER
-- milliseconds since the Unix epoch, UTC; `day` is the user's local date it started.

CREATE TABLE kata_sessions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at  INTEGER NOT NULL,
    day         TEXT NOT NULL,
    kind        TEXT NOT NULL CHECK (kind IN ('daily', 'pattern', 'list', 'custom')),
    source      TEXT,           -- the pattern id or list name, if any
    problem_ids TEXT NOT NULL,  -- JSON array of problem ids, in order
    duration_s  INTEGER NOT NULL,
    ended_at    INTEGER         -- NULL while running
);
CREATE UNIQUE INDEX kata_sessions_by_start ON kata_sessions (started_at);

-- One row per problem opened in a session. Keyed by natural values so imports merge.
CREATE TABLE kata_attempts (
    session_started_at INTEGER NOT NULL,
    problem_id         TEXT NOT NULL,
    started_at         INTEGER NOT NULL,
    submits            INTEGER NOT NULL DEFAULT 0,
    verdict            TEXT,     -- the last verdict, if any
    solved_at          INTEGER,  -- the first Accepted, if any
    time_ms            INTEGER,  -- solved_at - started_at
    language           TEXT,     -- the language of the first Accepted
    code               TEXT,     -- the code of the first Accepted
    PRIMARY KEY (session_started_at, problem_id)
) WITHOUT ROWID;
CREATE INDEX kata_attempts_by_problem ON kata_attempts (problem_id);
