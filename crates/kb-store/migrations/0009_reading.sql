-- user.db migration 9 (expansion M7, ADR 0035): answers to reading exercises, one row per
-- item (`<set>/<n>`). Times are INTEGER milliseconds since the Unix epoch, UTC.

CREATE TABLE reading_answers (
    item_id    TEXT PRIMARY KEY NOT NULL,
    attempts   INTEGER NOT NULL,
    correct_at INTEGER,          -- the first correct answer; NULL until then
    updated_at INTEGER NOT NULL
) WITHOUT ROWID;
