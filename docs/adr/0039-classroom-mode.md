# 39. Classroom mode

Date: 2026-11-22 · Status: proposed (expansion M12)

## Context
Teachers want to hand out a set of problems with rules (a due date, the languages to use,
a time limit, hints or not) and collect what each student did. KataBox is offline and has
no accounts or server, so the hand-out and the hand-in are files, and nothing can stop a
determined student from editing their own data.

## Decision: the assignment
- An assignment is an `assignment.toml` at the root of a pack (ADR 0038): `title`, `due`
  (`YYYY-MM-DD`), `problems` (the pack's own problem folders, stored as `<pack>--<id>`, or
  built-in ids), `languages` (judge languages), `time_limit_min` (optional, 5-600) and
  `hints` (default true). `deny_unknown_fields`; at most 50 problems, no repeats. A pack
  that is only an assignment of built-in problems needs no `problems/` folder.
- `kb-pack pack … --assignment assignment.toml` adds it; the import checks it with the
  rest of the pack (it is listed with its hash like every file) and the preview names it.
- One assignment per pack, keyed by the pack id: user.db migration 0013 `assignments`
  (`id`, `pack_sha256`, `meta` JSON, `imported_at`, `started_at`, `submitted_at`). A newer
  pack version replaces the text and keeps the start and finish times. Assignments of
  uninstalled packs are hidden, not deleted.

## Decision: the student side
- **Start** records `started_at`; the rules apply from then until the student finishes
  or the time limit passes (then `submitted_at` = the deadline, like the interview
  timer, on the wall clock). One assignment runs at a time; it can't start during a mock
  interview.
- **Locks in kb-app**, never only in the UI:
  - `ensure_interview_language` also calls `ensure_assignment_language`: judging,
    debugging, profiling, shrinking and the stress lab on an assignment problem accept
    only its languages (SQL problems always take SQL);
  - with `hints = false`, `ensure_unlocked` refuses hints, the editorial, earlier
    submissions, History and accepted solutions (`LockedInAssignment`), and
    `get_problem` leaves the hint texts out, the same set as a mock interview.
- **Results**: `katabox-results/v1`, saved through the Rust Save dialog. It holds the
  student's name (typed by them), the assignment id, title and pack SHA-256, the start
  and end times, and per problem: the best verdict (Accepted if any, else the last), the
  number of submits, seconds to the first Accepted, the language, and the code only when
  the student ticks "Include my code". Only submissions made while it ran count. The
  dialog lists exactly what the file holds before it is saved.
- **Hash chain**: `h0 = sha256("katabox-results/v1|pack_sha256|student|id|title|
  started_at|ended_at|include_code")`, `h_i = sha256(h_{i-1} ‖ canonical_json(entry_i))`
  (hex; the entry's fields in their declared order); the file stores every `h_i` and the
  last as `final_hash`. There is no key: anyone can recompute it. It detects casual
  edits (a changed verdict, name, time, order or code), not a determined forger.

## Decision: the teacher side
- **Import** any number of results files (a multi-file Open dialog in Rust): each is
  read with an 8 MB cap and checked strictly (`deny_unknown_fields`, exact format, plain
  text name and title, a valid pack id, hex hashes, unique problem ids in the id
  alphabet, code ≤ 64 KB and only with `include_code`, sane times). Refused files are
  listed with the reason. A file whose chain does not match is kept and shown as
  "Edited after export". The same bytes import once (`file_sha256 UNIQUE` in
  `class_results`, migration 0013).
- **Summary**: per assignment, one row per student (their newest file), one column per
  problem: the verdict with its icon and label, the submits.
- **CSV** (RFC 4180, CRLF): `student,file_unchanged,problem,language,verdict,submits,
  first_accepted_s`, one row per student and problem. A cell starting with `=`, `+`,
  `-`, `@`, tab or carriage return gets a leading `'`, so spreadsheets never read it as
  a formula; cells with commas, quotes or line breaks are quoted.

## Not exported
Assignments belong to installed packs and class results can be imported again from
their files, so neither is part of the progress export (like packs, ADR 0038).

## Consequences
- The Classroom page (`/classroom`) holds both sides; Settings → Packs installs
  assignment packs like any pack (they apply after a restart).
- Not cheat-proof, and the docs say so plainly (EN/ES/pt-BR: `docs/classroom*.md`).
- Tests: `kb-pack/tests/pack_hostile.rs` (assignments resolve, bad ones refused), kb-app
  unit tests (chain, parsing, CSV escaping, summary) and
  `classroom_assignment_locks_results_and_teacher_import` (install, locks, time limit,
  results, import with a duplicate, an edited and a hostile file, CSV), kb-store tests
  and the migration seed, Vitest for both sides.
