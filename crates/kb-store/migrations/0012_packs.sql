-- user.db migration 12 (expansion M11, ADR 0038): installed problem packs. Uninstalling
-- sets removed_at (the history of their problems stays); reinstalling clears it. Times
-- are INTEGER milliseconds since the Unix epoch, UTC.

CREATE TABLE packs (
    id           TEXT PRIMARY KEY NOT NULL,
    name         TEXT NOT NULL,
    author       TEXT NOT NULL,
    version      TEXT NOT NULL,
    licence      TEXT NOT NULL,
    sha256       TEXT NOT NULL,   -- of the .kbpack file
    problems     INTEGER NOT NULL,
    installed_at INTEGER NOT NULL,
    removed_at   INTEGER
) WITHOUT ROWID;
