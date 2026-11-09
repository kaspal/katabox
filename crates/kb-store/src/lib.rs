//! Storage for KataBox.
//!
//! Two SQLite files (see the plan, "Problem database and content"):
//! - `content.db`: read-only at runtime, written only by the pack installer.
//! - `user.db`: drafts, submissions, snapshots, notes, reviews, lists, settings (WAL mode).
//!
//! Planned: sqlx with embedded numbered migrations, FTS5 search over problems,
//! automatic backup of `user.db` before each migration.
