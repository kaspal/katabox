# 38. Problem packs and their threat model

Date: 2026-11-22 · Status: proposed (expansion M11)

## Context
Teachers, clubs and friends want to share problem sets beyond the bundled catalogue. A
shared file is untrusted input: whoever made it (or changed it on the way) may try to
escape the sandbox, overwrite files, exhaust the disk, run script in the webview, or pass
a pack off as built-in content.

## Decision: the format
- A `.kbpack` is a zip holding `manifest.toml` and `problems/<id>/` folders in the
  ordinary package format (follow-ups, fix-the-bug and program problems included).
- `manifest.toml`: `format = "katabox-pack/v1"`, `id`, `name`, `author`, `version`
  (semver), `licence` (SPDX), `min_app` (semver) and `[files]`, a map of every other entry
  to its SHA-256.
- `kb-pack pack DIR... --manifest pack.toml --out x.kbpack` validates each folder like
  built-in content and writes a deterministic zip (sorted entries, fixed timestamps).
- Pack problems are stored as `<pack>--<id>`; pack ids are `[a-z][a-z0-9]{2,19}` (no
  `-`, so no collision with built-in ids or follow-ups).
- **Not yet:** Learn topics in packs (they need a tracer mapping and their own trust
  rendering) and the optional "Check in sandbox" of a whole pack. Both are on the report's
  Deferred list; v1 packs hold problems only and anything else is refused.

## Threat model
**Adversary:** a malicious pack author, or someone who changed a pack in transit.
**Assets:** the user's files and processes, user.db, the bundled content, the webview
(which can call Tauri commands) and the sandbox.

| # | Threat | Defence | Test |
| --- | --- | --- | --- |
| T1 | Zip slip and confusing names | New extractor in kb-pack (`kbpack/archive.rs`); kb-toolchain's rules mirrored, not shared. `[A-Za-z0-9._-]` segments, no `.`/`..`, absolute paths, drive letters, backslashes, streams or device names, ≤ 8 levels, ≤ 180 characters, no case-folding duplicates; files created with `create_new` in a fresh staging folder | `pack_hostile::t1_*` |
| T2 | Links and special files | Symlinks refused; mode bits must be a regular file or folder | `t2_*` (a hand-made device entry) |
| T3 | Bombs and huge files | ≤ 64 MB file, ≤ 5000 entries; real decompressed size through a capped reader: 4 MB test JSON, 256 KB other text, 128 MB in all; stored or deflate only; ≤ 200:1 above 64 KB | `t3_*` (5 MB of zeros, 3 MB at > 200:1, 5001 entries) |
| T4 | Unknown file types | Extensions `.toml .md .json .py .ts .php .rs .sql` only | `t4_*` |
| T5 | Integrity | Every entry listed with a matching SHA-256; unlisted or missing entries refused | `t5_*` (flipped byte, extra, missing) |
| T6 | Bad metadata | Manifest fields checked (format, id, plain-text name and author ≤ 80, SPDX-like licence, semver, `min_app` ≤ this app); every problem through the built-in structural validation and the 512 KB test budget | `t6_t10_*` |
| T7 | Markup in text | kb-pack refuses `<script`, `<iframe`, `<object`, `<embed`, `javascript:`, `vbscript:`, `data:` and event attributes in pack Markdown. The app renders pack text with `trust="pack"`: no raw HTML (as always), links as plain text, images as their alt text, no URL kept, KaTeX untrusted with size and expansion limits | `t7_*`; Vitest: no element with `href`, `src` or event attributes |
| T8 | Code from the pack | Runs only through the ordinary judge and tools paths, in the sandbox, with the sandbox lock and normal limits. No kb-runner change | `packs_hostile_windows`: a pack reference that tries to list the profile, reach the network, write to TEMP and the registry is contained |
| T9 | Template injection through names | Signature names go through the existing parser: ASCII identifiers only | `t9_*` |
| T10 | Spoofing built-in content | Namespaced ids; a pack id may not equal a built-in problem id or `core`; a pack badge on every pack problem, and a Library source filter | `t6_t10_*`; Vitest |
| T11 | Language servers | Pack code opens like any code; servers never execute code (ADR 0014, unchanged) | — |
| T12 | Uninstall tampering | Uninstall deletes only `<local data>/packs/<id>/`; history stays in user.db | kb-app `packs_install_update_and_uninstall` |
| T13 | Honesty | The import preview says "Unsigned pack. Author as stated: …" and shows the SHA-256, licence, version and counts | Vitest |

## Decision: in the app
- **Import:** a native Open dialog (Rust, `.kbpack` filter) → extract into
  `packs/.staging-*/src`, run T1-T7, T9, T10, build the pack's own content.db → preview.
  Apply renames the staging folder to `packs/<id>/` (whole-folder promotion) and records
  the pack (user.db 0012 `packs`). A refused pack leaves nothing behind.
- **Updates (review A3):** the same file is "already installed"; the same id with
  different content installs only when its version is newer; otherwise it is refused.
- **Content:** instead of routing every query between several databases, the app opens a
  merged copy: `packs/merged.db` is the core content.db with each pack's problem rows
  added (content.db 13 adds `problems.pack_id`), rebuilt only when a fingerprint (core
  file, packs and their hashes, schema) changes. Every existing query works unchanged.
  A pack database from an older schema is rebuilt from its kept, already-checked sources.
  Installing, updating or removing a pack takes effect on restart ("Restart now").
- **Practice (A3):** pack problems are in the Library and searchable, but the daily
  problem, next problem, placement and study plans leave them out unless
  `practice.include_packs` is on.
- **Dependencies:** kb-pack now uses `zip` 7.3.0 and `flate2` 1.1.11, the versions
  already locked for kb-toolchain (no new crate); kb-app depends on the kb-pack library
  (a workspace crate) for the same checks as the CLI.
