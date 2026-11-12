# 20. Practice modes: mock interviews, export/import, cross-language practice

Date: 2026-11-12 · Status: proposed

## Context
Build plan enhancements 2, 7 and 8 add three practice modes:
- **2:** timed mock interviews with help locked;
- **7:** moving progress between machines;
- **8:** re-solving a problem in another language.

Their acceptance checks:
- an interview survives an app restart with the timer correct, and nothing locked can be
  opened;
- export → fresh data folder → import reproduces Progress and Home exactly;
- cross-language practice works for every language pair and never overwrites a draft.

## Decision

### Mock interviews (`interview_sessions`, kb-app `interview.rs`)
- **Setup:**
  - duration 30, 45 or 60 minutes;
  - a difficulty mix of two problems: `easy+easy`, `easy+medium`, `medium+medium`,
    `medium+hard` or `hard+hard`;
  - the allowed judge languages.
- **Picking:**
  - two **unsolved problems the user has never opened**, one per slot of the mix;
  - if a slot has no fresh problem of that difficulty, the nearest difficulty is used;
  - with fewer than two fresh problems: `not_enough_problems`;
  - the choice is deterministic, seeded by the start time.
- **What counts as "opened":** user.db migration 0004 adds `problem_opens`, and the
  workspace calls `mark_opened`. Any earlier trace also counts: a draft, snapshot,
  submission, status or unlocked hint. Databases from before this change therefore need
  no back-fill.
- **One session at a time** (`interview_active`).
- **Wall-clock timer:** the session stores `started_at` (UTC ms) and `duration_s`. The time
  left is always computed from the current time, so closing and reopening the app keeps
  it correct.
  - The session ends when the user clicks End, or lazily when time is up: any interview
    query, and every lock check, first ends a session whose deadline has passed, with
    `ended_at` = the deadline.
- **Scorecard:** stored as JSON in `results` when the session ends, from the submissions
  on the session's problems inside [start, end]:
  - per problem: solved, time from the start to the first Accepted, submit count, and
    first-try pass;
  - totals: solved, first-try passes, time used.
  - The Progress page lists past interviews.
- **Locks, enforced in kb-app, not only in the UI.** While a session runs, these commands
  return `locked_in_interview` for its two problems:
  - `editorial`, `unlock_hint`, `unlocked_hints`;
  - `list_submissions`, `get_submission` (checked by the submission's problem);
  - `list_snapshots`, `restore_snapshot` (checked by the snapshot's problem);
  - `accepted_solution`.
  - `get_problem` returns no hint texts for them.
  - **Allowed:** Run/Submit (the judge), Notes (scratch paper), and every other problem.
  - **Languages:** Run/Submit on a session problem are refused in a language outside the
    session's languages, and the UI limits its language list to match.
  - **The AI tutor**, when it exists, joins this list.
- **UI:**
  - an Interview page (nav) with setup, the running session (problem cards and End), and
    the scorecard;
  - a timer in the top bar and in the workspace header (it stays visible in focus mode),
    showing a warning color under 5 minutes, with its label kept;
  - locked tabs say "Locked during the mock interview".

### Export/import (`katabox-export/v1`, kb-app `transfer.rs`, kb-store `transfer.rs`)
- **One JSON file** (`.kbx.json`): `{ format, exported_at, app_version, data }`.
  - **`data`:** statuses, submissions (with their stored reports), snapshots, notes,
    drafts, hint unlocks, problem opens, reviews, daily picks (the streak derives from
    them), lists with items, interviews, and settings.
  - **Beyond the build plan's list,** drafts, hint unlocks and opens are included, so that
    Home (the Continue card) and interview picks reproduce exactly.
  - **Settings that stay on the machine:** `app.install_id`, `judge.calibration` and
    `sandbox_root`.
- **Export:** written atomically (a temp file, then rename).
- **Import is strict:**
  - the file is at most 64 MiB, UTF-8 JSON;
  - `format` must be exactly `katabox-export/v1`;
  - `deny_unknown_fields` everywhere;
  - enums are checked (languages, verdicts, statuses, snapshot sources);
  - problem ids match `[a-z0-9-]{1,64}`;
  - dates are valid `Day`s; times fall in 2000-2200; numbers are in range;
  - size caps: code 1 MiB, notes 256 KiB, reports 4 MiB, 200 000 records per kind;
  - settings must have an allow-listed prefix (`editor.`, `workspace.`, `practice.`, or
    `ui_lang`, `theme`).
  - Any failure rejects the whole file (`import_invalid`, with the path of the bad
    field), and nothing is written. Nothing in a file is executed; code is stored as
    text.
- **Preview, then apply:** `import_preview` reads and checks the file, keeps it pending
  in the App, and returns counts per kind and conflicts. `import_apply(id)` merges in one
  user.db transaction; `import_cancel(id)` drops it.
- **Merge rules (the newer record wins):**
  - **statuses:** solved never downgrades; the earliest `first_solved_at`; the latest
    `updated_at`.
  - **submissions:** union, deduplicated by problem, language, time and code.
  - **snapshots:** by their unique (problem, language, code hash); the newer one wins.
  - **notes and drafts:** the newer `updated_at` wins.
  - **hint unlocks:** union.
  - **opens:** the earliest first open and the latest last open.
  - **reviews:** more passes wins, then the later due date.
  - **daily picks:** per day; a solved pick wins.
  - **lists:** matched by name; items unioned, appended in order.
  - **interviews:** unique start time; an imported unfinished session is imported as
    ended at its deadline.
  - **settings:** the newer `updated_at` wins.
  - Importing the same file twice changes nothing.
- **File dialogs** come from `tauri-plugin-dialog`, used **only from Rust commands**
  (`export_progress`, `import_preview`) on a blocking thread.
  - The webview gets **no dialog permission**: `capabilities/default.json` stays
    `core:default`.
  - The CSP is unchanged.
  - The UI never sees a file path.

### Cross-language practice
- **On a solved problem,** the workspace offers **Solve in…** (the judge languages that
  have a solved source language).
- **Split view:** choosing one opens `?compare=<from>`. The left pane is a read-only
  editor with the latest Accepted submission in `<from>` (`accepted_solution`). The right
  pane is the normal editor in the chosen language.
- **Drafts:** the right pane opens that language's existing draft, otherwise its stub,
  and saves only after the user types, so a draft is never overwritten.
- **Progress** shows solved problems per language (`solved_by_language`: distinct
  problems with an Accepted submission).

## Consequences
- user.db schema 4 (`0004_practice_modes.sql`).
- **Dependencies:** `tauri-plugin-dialog` (MIT/Apache) joins the desktop shell; there are
  no capability or CSP changes.
- **Format changes:** new export versions need a new `format` string; v1 files stay
  readable as long as the importer for them exists.
- Interview locks are checked on every locked command. The cost is one small query per
  call.

## Follow-ups
- The AI tutor (future) joins the interview lock list.
- Export with a choice of what to include (for example without settings).
- Interview statistics over time on Progress.
