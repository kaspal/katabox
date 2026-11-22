# 37. Focus sessions and automatic backups

Date: 2026-11-22 · Status: proposed (expansion M10)

## Context
- Practice happens in sittings; there was no way to time one or see what it achieved.
- Progress lives in one local user.db. Export existed, but only when the user remembered
  to use it.

## Decision: focus sessions
- **Wall clock:** a session is `started_at` plus work and break lengths (`focus.work`,
  default 25; `focus.break`, default 5). The phase and time left are computed from the
  clock (`kb-app/src/focus.rs`, pure and tested; the UI mirrors it), so a sleeping laptop
  or a restart never drifts it.
- **Survives a restart:** the running session is the `focus.current` setting (stays on
  this machine; `focus.work` and `focus.break` travel with an export).
- **Notices:** a chip in the top bar shows the phase and time left, with Stop; a phase
  change shows an in-app notice for a few seconds. No OS notifications (no new capability
  or plugin).
- **Summary:** Stop writes a `focus_sessions` row (user.db 0011) with the work periods
  completed and what happened in the window, counted from existing tables: problems
  opened or submitted, problems with an Accepted submission, katas solved and plan items
  done. Shown on Progress → Overview; exported (a session never changes once written, so
  an import adds missing ones).
- Start and Stop are also in the command palette.

## Decision: automatic backups
- **What:** the ordinary export (`katabox-export/v2`), written atomically (a temporary
  file, then a rename) as `katabox-backup-YYYYMMDD-HHMMSS.kbx.json` (UTC) in a folder the
  user chose. Restoring is Settings → Data → Import.
- **Folder:** picked in a native dialog opened from Rust (`blocking_pick_folder`, the
  dialog plugin already in use; the webview still has no dialog permission). A synced
  folder is the user's choice; the docs say so.
- **When:** on start, in the background, when `backup.every_days` (default 7) have passed
  since `backup.last`; on exit when `backup.on_exit` is on, capped at 5 s so closing never
  hangs; and **Back up now**. A failure is kept as `backup.error` and shown in Settings →
  Data; it never stops the app.
- **Pruning:** after a backup, KataBox deletes only files whose names match
  `^katabox-backup-\d{8}-\d{6}\.kbx\.json$` exactly, oldest first, beyond `backup.keep`
  (default 5, at least 1). Everything else in the folder is left alone.
- **README (review A4):** the first backup to a folder writes
  `README-katabox-backups.txt`, which says what the files are, how to restore one, and
  that KataBox deletes only `katabox-backup-*.kbx.json` files beyond the keep count.
- **Settings:** every `backup.*` key stays on this machine (exports carry only
  allow-listed settings).

## Tests
- Pure: phases, file names, the exact-name match (near misses refused), pruning (foreign
  files kept, keep ≥ 1), the due rule.
- kb-app: three backups with keep 2 leave the newest two, the README and two foreign
  files; a backup imports like an export; due on start versus on exit; a missing folder is
  reported. Focus start, restart-safe current session, Stop with a summary.
- user.db 0011 up-migration test (M0 helper) and the export round trip; Vitest for the
  chip, the card and the Settings section.
