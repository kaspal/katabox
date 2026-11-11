-- user.db migration 2 (ADR 0013): the verdict of the Run or Submit that produced a
-- snapshot, for the History tab. NULL for snapshots saved by a restore and for rows
-- written before this migration.
ALTER TABLE snapshots ADD COLUMN verdict TEXT;
