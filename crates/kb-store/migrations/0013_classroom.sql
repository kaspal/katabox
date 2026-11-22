-- user.db migration 13 (expansion M12, ADR 0039): classroom mode. A student's assignments
-- come with packs (one per pack, keyed by the pack id); a teacher's imported results files
-- are kept as they were read. Times are INTEGER milliseconds since the Unix epoch, UTC.

CREATE TABLE assignments (
    id           TEXT PRIMARY KEY NOT NULL,  -- the pack id
    pack_sha256  TEXT NOT NULL,              -- of the .kbpack it came with
    meta         TEXT NOT NULL,              -- assignment.toml as JSON
    imported_at  INTEGER NOT NULL,
    started_at   INTEGER,
    submitted_at INTEGER                     -- finished (by the student or the time limit)
) WITHOUT ROWID;

CREATE TABLE class_results (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    assignment_id TEXT NOT NULL,
    student       TEXT NOT NULL,
    file_sha256   TEXT NOT NULL UNIQUE,      -- the same file is imported once
    data          TEXT NOT NULL,             -- the katabox-results/v1 file as JSON
    chain_ok      INTEGER NOT NULL,          -- 0 when the hash chain does not match
    imported_at   INTEGER NOT NULL
);
CREATE INDEX class_results_by_assignment ON class_results (assignment_id, student);
