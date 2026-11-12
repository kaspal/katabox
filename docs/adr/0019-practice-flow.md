# 19. What to practise next: reviews, daily problem, pattern map, lists

Date: 2026-11-11 · Status: proposed

## Context
KataBox judged problems well, but left users to choose what to do next. Build plan
enhancements 3-6 add four practice aids:
- **3:** a pattern map, and a "next problem" after Accepted;
- **4:** spaced-repetition reviews;
- **5:** an offline daily problem with a streak;
- **6:** personal lists.

The guardrails stay the same: no server, no new runtime, no new database file; new data
through a numbered user.db migration; EN and ES for every string. Everything must work
offline, and survive restarts and time-zone changes.

## Decision

### Dates: the UI's local date, stored as calendar dates
- **The local date comes from the UI.** Every practice command that needs "today" takes the
  user's local calendar date from the UI as `YYYY-MM-DD` (`lib/today.ts`).
- **Storage:** dates are plain calendar dates, as ISO text in user.db (`kb_core::Day`:
  days since 1970-01-01, with civil-date conversion and no new crate).
- **Why time zones don't matter:** "due on 2026-11-17" is due on that local date wherever
  the user is.
- **Midnight:** the UI updates its date at local midnight.
- **Why the UI decides:** Rust has no portable local time zone without a new dependency,
  and passing the date in makes every rule deterministic in tests.
- **Timestamps** (created/solved) stay UTC milliseconds.

### Spaced-repetition reviews (`reviews` table)
- **First Accepted** (the problem was not solved before this Submit): due today + 3 days,
  interval 3.
- **Review mode** (`/problems/$id?review=true`, from Home):
  - every language starts from its stub;
  - drafts are neither shown nor saved;
  - History and Submissions are locked ("hidden until you pass").
  - Only review-mode Submits move the schedule:
    - **Accepted:** the interval goes 3 → 7 → 21, and the problem is due again after it.
      **Passing the 21-day review graduates the problem**: it leaves the queue. "Review in
      3 days" in the workspace adds it back.
    - **After a lapse,** the next pass schedules 3 days, not the next interval.
    - **Not Accepted:** the interval resets to 3, the row is marked `lapsed`, and it stays
      due today so the user can try again.
  - After a pass, the tabs unlock and the banner says when the next review is.
- **Snooze** moves the due date to tomorrow; **Remove** deletes the row.
- **Toggle off:** reviews are still recorded, but the Home card is hidden.

### Daily problem and streak (`daily_picks` table)
- **The pick** is deterministic, from SHA-256(`katabox-daily|day|install id`).
  - **Candidates:** the unsolved problems, or every problem once all are solved.
  - **Weights:** candidates in the weakest pattern weigh 3 and the others 1. The weakest
    pattern is the lowest solved share among patterns that still have unsolved problems;
    ties go to track order.
  - **Install id:** a random 128-bit id in settings (`app.install_id`), created on first
    use.
- **Stable all day:** the first request of a local day stores the pick, and later requests
  that day return it, even after other problems are solved. A new local date picks
  again. If the picked problem leaves the pack, it is replaced.
- **Never a solved problem while unsolved ones remain:** candidates exclude solved ones.
- **Streak:** an Accepted Submit of the day's pick on that day marks the day solved. The
  streak is the number of consecutive solved days ending today, or yesterday when today's
  pick is still open.
- **Toggle off:** the Home card is hidden and nothing is picked.

### Pattern map and next problem
- **Track order:** `PATTERN_TRACK` (kb-app) is the canonical pattern order, the order of
  `pattern.*` in the locale files. Unknown patterns come last.
- **Patterns view:** the Progress page shows each pattern with solved/total, its
  lowest-ranked unsolved problem, and a link to the Library filtered by pattern. Ranks
  are levels 1-10 with ties (see `docs/problem-authoring.md`).
- **Next problem** after an Accepted Submit:
  1. the lowest-ranked unsolved problem of the same pattern with rank ≥ the current one
     (then by id);
  2. otherwise the lowest-ranked unsolved problem of the following patterns in track order,
     wrapping around (which reaches lower ranks of the same pattern last);
  3. otherwise "You have solved every problem".
  - It never suggests a solved problem or the current one.

### Personal lists (`lists`, `list_items`)
- **Operations:** create, rename (1-80 characters, trimmed) and delete (items cascade).
  "Add to list" is in each Library row and the workspace header.
- **Ordering:** a `position` per item. Reordering rewrites the positions in one
  transaction and must name exactly the list's problems.
- **Where:** the Library's Lists tab shows progress per list (available problems only).
- **Reorder in the UI:** native drag-and-drop (no new dependency), Alt+↑ / Alt+↓, or the
  row menu.
- **Unavailable problems:** an item whose problem is no longer in content.db shows as
  "Unavailable (removed from the problem pack)" with Remove. Nothing fails.
- **Export:** lists are not exported yet; there is no export feature. That is a
  follow-up.

### Toggles
Settings → Practice has two switches, both on by default: `practice.daily` and
`practice.review`. They control the Home cards, the features that push content at the
user. Patterns and Lists are views the user opens, so they have no toggle.

### Layers
- **kb-store:** owns the queries (`practice.rs`).
- **kb-app:** owns the rules (`practice.rs`: pure functions over the problem list, with
  unit tests).
- **UI:** only renders and calls typed commands.
- **Submit:** takes `SubmitOptions { today, review, lang }` and returns a
  `PracticeOutcome`: the review result, first solve, daily solved, streak and next
  problem.

## Consequences
- user.db schema 3 (migration `0003_practice.sql`, with a backup before migrating as
  usual).
- Review mode never touches drafts, so a review cannot destroy the user's earlier
  solution.
- Picking "today" in the UI means a wrong system clock gives wrong days. That is
  acceptable for an offline app, and the same as any calendar.
- Weighting and intervals are constants in kb-app (`WEAKEST_WEIGHT`, `REVIEW_INTERVALS`),
  easy to tune later.

## Follow-ups
- Export and import of user data, lists included.
- Review statistics on Progress (passes, graduated problems).
- A per-problem "Not today" for the daily problem.
