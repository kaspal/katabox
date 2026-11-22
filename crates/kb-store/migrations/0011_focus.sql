-- user.db migration 11 (expansion M10, ADR 0037): finished focus sessions. The running
-- one lives in the `focus.current` setting. Times are INTEGER milliseconds since the Unix
-- epoch, UTC.

CREATE TABLE focus_sessions (
    started_at INTEGER PRIMARY KEY NOT NULL,
    ended_at   INTEGER NOT NULL,
    work_min   INTEGER NOT NULL,
    break_min  INTEGER NOT NULL,
    cycles     INTEGER NOT NULL,   -- work periods completed
    summary    TEXT NOT NULL       -- JSON: what happened in the window
) WITHOUT ROWID;
