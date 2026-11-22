-- user.db migration 6 (expansion M2, ADR 0033): study plans. One plan is active at a
-- time (archived = 0); its items are planned by day. Times are INTEGER milliseconds
-- since the Unix epoch, UTC; days are the user's local dates as 'YYYY-MM-DD'.

CREATE TABLE study_plans (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at      INTEGER NOT NULL,
    goal            TEXT NOT NULL,     -- JSON: {"patterns": [...], "left_over": n}
    weeks           INTEGER NOT NULL,
    minutes_per_day INTEGER NOT NULL,
    start_day       TEXT NOT NULL,
    archived        INTEGER NOT NULL DEFAULT 0
);
CREATE UNIQUE INDEX study_plans_by_created ON study_plans (created_at);

CREATE TABLE plan_items (
    plan_created_at INTEGER NOT NULL,
    position        INTEGER NOT NULL,
    day             TEXT NOT NULL,
    kind            TEXT NOT NULL CHECK (kind IN ('problem', 'topic', 'review', 'kata')),
    ref             TEXT,             -- the problem or topic id; NULL for reviews and katas
    minutes         INTEGER NOT NULL,
    status          TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'done', 'skipped')),
    updated_at      INTEGER NOT NULL,
    PRIMARY KEY (plan_created_at, position)
) WITHOUT ROWID;
CREATE INDEX plan_items_by_day ON plan_items (plan_created_at, day);
