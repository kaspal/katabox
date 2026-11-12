-- user.db migration 3 (ADR 0019): spaced-repetition reviews, daily problems, personal
-- lists. Days are calendar dates as 'YYYY-MM-DD' text (the user's local date when they
-- were set); other times are INTEGER milliseconds since the Unix epoch, UTC.

CREATE TABLE reviews (
    problem_id    TEXT PRIMARY KEY NOT NULL,
    due_on        TEXT NOT NULL,
    interval_days INTEGER NOT NULL,
    passes        INTEGER NOT NULL DEFAULT 0,
    lapsed        INTEGER NOT NULL DEFAULT 0, -- failed this round: the next pass resets to 3 days
    created_at    INTEGER NOT NULL
) WITHOUT ROWID;
CREATE INDEX reviews_by_due ON reviews (due_on);

-- One row per local day on which a daily problem was picked; the pick stays all day.
CREATE TABLE daily_picks (
    day        TEXT PRIMARY KEY NOT NULL,
    problem_id TEXT NOT NULL,
    solved_at  INTEGER
) WITHOUT ROWID;

CREATE TABLE lists (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

-- problem_id may name a problem no longer in content.db: shown as unavailable.
CREATE TABLE list_items (
    list_id    INTEGER NOT NULL REFERENCES lists (id) ON DELETE CASCADE,
    problem_id TEXT NOT NULL,
    position   INTEGER NOT NULL,
    added_at   INTEGER NOT NULL,
    PRIMARY KEY (list_id, problem_id)
) WITHOUT ROWID;
