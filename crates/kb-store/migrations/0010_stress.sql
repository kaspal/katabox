-- user.db migration 10 (expansion M8, ADR 0036): the stress lab's brute-force solution
-- per problem and language, kept between sessions. Times are INTEGER milliseconds since
-- the Unix epoch, UTC.

CREATE TABLE stress_brutes (
    problem_id TEXT NOT NULL,
    language   TEXT NOT NULL,
    code       TEXT NOT NULL,
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (problem_id, language)
) WITHOUT ROWID;
