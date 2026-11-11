# 10. Store schema and migration rules

Date: 2026-11-11 · Status: accepted

## Context
ADR 0004 split storage into a read-only `content.db` and a read-write `user.db`. This
records the concrete schemas (`crates/kb-store`), how each file is created and upgraded,
and the rules that keep user progress safe across app updates.

## Decision

### Access
- **sqlx 0.8** (SQLite, bundled with FTS5) with runtime queries only, no compile-time
  checked macros, so building needs no `DATABASE_URL`. The API is async (Tauri commands
  can be async).
- **Dependencies:** kb-store depends on kb-core only, never on kb-judge or kb-runner.
  The judge's result types (`Report`, `TestOutcome`, `Failure`, `CompileInfo`,
  `TestCase`) live in kb-core and kb-judge re-exports them. Their serde shape is the
  stored submission JSON: fields are never renamed or removed (a kb-core test pins it).
- **Times** in user.db are `INTEGER` milliseconds since the Unix epoch, UTC. content.db's
  build time is UTC ISO 8601 text.

### content.db
- Built only by `kb-pack build` (`kb_store::content::build`), which refuses to build
  unless every problem passes `kb-pack validate`. The file is written to `<out>.tmp` and
  renamed over `<out>` when complete, so a failed build never leaves a partial pack.
- Tables: `content_meta` (pack_id, pack_version, schema_version, built_at), `problems`
  (difficulty, pattern, rank, signature JSON, checker, limits, profile), `problem_tags`,
  `problem_texts` (title, statement, editorial per UI language), `hints` (tier per
  language), `tests` (public and hidden, `input`/`output` JSON), `reference_solutions`,
  and `problem_fts` (FTS5 over title, tags and statement, one row per problem and
  language, `unicode61 remove_diacritics 2`).
- English is required for every problem; any other UI language falls back to English
  field by field (title, statement, editorial, hint list). The Spanish title comes from
  the optional `title_es` in `problem.toml`.
- Rows are inserted in a fixed order, so rebuilding the same content gives identical
  tables (FTS shadow tables included) except `content_meta.built_at`.
- `PRAGMA user_version` holds the content schema version. The app opens content.db
  read-only (`query_only`) and refuses any other version. Changing the content schema
  means bumping the version and rebuilding packs; there are no content migrations
  (content is replaced whole, never upgraded).
- Search input is split into words; each word is quoted and prefix-matched, so FTS
  syntax typed by the user is literal and cannot cause an error.

### user.db
- WAL mode, `synchronous = NORMAL`, foreign keys on, 5 s busy timeout. One pooled
  connection: the app is the only writer, and a single connection rules out
  `SQLITE_BUSY` between our own transactions.
- **Migrations** are numbered SQL files in `crates/kb-store/migrations/` (`0001_init.sql`,
  ...), embedded with `include_str!` and listed in `MIGRATIONS`. The applied version is
  `PRAGMA user_version`. At open:
  1. A database newer than the binary is refused (`NewerSchema`), never downgraded.
  2. If migrations are pending and the file already held a schema (version > 0), it is
     first copied with `VACUUM INTO` to `user.db.bak-v<old version>-<UTC time>` in the
     same folder (a consistent copy even with a WAL file present). Then only the newest
     5 backups are kept (ordered by the UTC time in the name); the one just written is
     never deleted, and files that do not follow the naming pattern are never touched.
     Pruning is best effort and never fails the open.
  3. Each pending migration runs in its own transaction, which also sets
     `user_version`, so a failure leaves the previous version intact.
- **Rules:** a released migration is never edited; every change is a new file with the
  next number. Versions are contiguous from 1 (a unit test checks). New data only via a
  migration.
- Tables (migration 1): `settings` (JSON values), `drafts` (per problem and language),
  `submissions` (verdict, time, peak memory, full Report JSON), `problem_status`
  (attempted / solved, `first_solved_at`), `snapshots`, `notes` (one Markdown body per
  problem), `hint_unlocks`. No foreign keys to content.db (problems live in another
  file; a pack update must never break user rows).
- **Status:** only submissions change it. Accepted → `solved` (sets `first_solved_at`
  once); anything else → `attempted`, unless already solved. Runs do not change status.

### Solution history
- A snapshot is taken on every Run and Submit. Snapshots are deduplicated per problem and
  language by the SHA-256 of the code: saving code that is already in the history removes
  the old entry and inserts it again on top. Ordering is by id (monotonic), not time.
- At most 200 snapshots per problem (all languages together); the oldest go first.
- Restoring a snapshot saves the current draft as a snapshot (source `restore`) unless
  it equals the restored code, then replaces the draft, all in one transaction.
- Listing 200 snapshots must take under 100 ms (test; about 4 ms in a debug build).

## Consequences
- Pack updates replace content.db wholesale and can never damage user.db.
- An app downgrade cannot open a newer user.db; the user keeps the backup made by the
  newer version's first migration.
- Snapshot ids change when the same code is snapshotted again; the UI must reload the
  history after a Run, Submit or restore.
- Judging from content.db (instead of the package folder) will need kb-judge to build a
  `Problem` from stored parts; `JudgeData` already holds everything required.
