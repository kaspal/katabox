# 33. Kata drills, skill rating and study plans

Date: 2026-11-21 · Status: proposed (expansion M1-M2)

## Context
- Solving a problem once doesn't mean it can be solved again a month later.
- Reviews (ADR 0019) re-open a problem on a fixed ladder. They don't measure speed,
  and they don't help pick what to practise next.

## Decision: kata drills (M1)
**What a kata is:** re-solving **solved** problems from memory against the clock.
- **The editor** starts from the starter code, never the user's old code.
- **Hints, the editorial, submissions, History and notes** are locked while the kata
  runs. The lock is `ensure_unlocked`, like mock interviews (ADR 0020), plus a kata-only
  lock on notes.
- **Kept apart:** a kata Submit is judged exactly like Submit (all tests), but it writes
  only `kata_attempts`. There is no submissions row, status, review, daily or draft
  change; a kata Run leaves no History snapshot; drafts are not saved.
  - `SubmitOptions.kata` carries the session (its `started_at`).
  - `SubmitResult.kata` carries the attempt and whether it set a new best time.
- **One at a time:** one kata runs at a time, and never alongside a mock interview, in
  either order. A session nobody ends stops counting as running 2 hours after it
  started (`KATA_MAX_MS`).

**Data** (user.db migration 0005):
- `kata_sessions`: one row per session, keyed by `started_at`.
- `kata_attempts`: one row per (session, problem), keyed by natural values so imports
  merge. It keeps the first Accepted's time, language and code, and counts submits.

**Rules** (pure, in `kb-app/src/kata.rs`):
- **Rusty score** in [0, 1]:
  - `1 − exp(−days / 30)` since the last success, where a success is an Accepted
    submission or a solved kata attempt;
  - plus 0.15 for each failed kata attempt among the problem's last three (the running
    session doesn't count);
  - capped at 1.
- **Daily kata:**
  - 3 distinct solved problems;
  - a weighted choice without replacement: weight is `round((rusty + 0.05) × 1000)`,
    integers so the pick is exact everywhere;
  - seeded by SHA-256 of `katabox-kata|day|install_id|k`;
  - 15 minutes, shown as a soft clock (overtime is shown, never enforced);
  - one daily session per local day.
- **Pattern and list sets:** up to 5 solved problems, rustiest first, 10 minutes each.
- **Personal bests:** the fastest time from opening to Accepted, and the fewest submits.
- **Kata streak:** consecutive local days with a solved kata attempt, ending today or
  yesterday.

**Export:** `kata_sessions` and `kata_attempts` are v2 sections (M0).
- An imported attempt wins when it solved what the local one didn't, or when it counts
  more submits with the same outcome.
- An imported session that never ended is imported as ended at its expiry.

**UI:**
- a Kata page;
- `?kata=<started_at>` in the workspace, with an ExerciseShell bar (k of n, the clock,
  Next or Skip);
- a Home "Rusty" card (`practice.rusty`);
- a Progress → Katas tab.

## Decision: skill rating (M2)
**An estimate, not stored.** The rating is replayed from the history every time it is
needed (cached in memory, dropped on every Submit, kata Submit and import).
- The same history always gives the same numbers.
- An import needs no merge rule for it.
- The "back-fill" of solves made before this feature is the replay itself.

**Events:** the first Accepted submission of each problem, and every solved kata
attempt, in time order.

**The constants** (all in `kb-app/src/rating.rs`):

| What | Value |
| --- | --- |
| Start | 1200 |
| Problem rating | Easy 1000, Medium 1400, Hard 1800, + 20 per rank above 1 (at most 10 ranks) |
| Score | `S = clamp(1 − 0.1 × (submits − 1) − 0.2 × clamp((min − target) / target, 0, 1), 0.3, 1)` |
| Target minutes | Easy 15, Medium 30, Hard 45 |
| Expected | `E = 1 / (1 + 10^((R_problem − R) / 400))` |
| Update | `R += K (S − E)` |
| K | 40 for a scope's first 10 events, 24 until 30, then 16 |

- **Submits** count every submission of that problem up to and including the Accepted
  one.
- **Minutes** run from the first open (`problem_opens`). Older solves with no open time
  get no time penalty.
- **Scopes:** each pattern and the overall rating are updated from every event.

**Soft weight in practice:**
- **Daily problem:** a pattern's weight is multiplied by `1 + clamp((1500 − R) / 400,
  0, 1)`. The weights are integers (× 100) so the pick stays exact everywhere.
- **Next problem:** inside a pattern, Easy problems are skipped once the pattern's
  rating is 400 above theirs, while a harder unsolved one is left.
- **Unchanged:** with no rating events, both picks are exactly the old ones (unit
  tested). The start floor and the SQL rule are untouched.

**UI:**
- Progress → Overview shows the overall rating; By pattern shows each pattern's rating
  with a sparkline.
- The rating is always labelled "est.", with a tooltip saying it is a guide, not a
  grade.

## Decision: study plans (M2)
**Data** (user.db migration 0006):
- `study_plans`: one active at a time; a new plan archives the old one.
- `plan_items`: day, kind (problem, topic, review or kata), minutes, status (pending,
  done or skipped).

**Generator** (pure, `kb-app/src/plan.rs`):
- **Order:** the chosen patterns in track order. Each pattern gets its unviewed Learn
  topics first, then its unsolved problems by rank.
- **Minutes:** Easy 20, Medium 35, Hard 50, topic 15, review 10, kata 15.
- **Days:** each day is filled up to the daily minutes. An item longer than a whole day
  gets a day of its own. Each week's last day opens with a kata and the due reviews.
- **Leftovers:** what doesn't fit is counted and shown.

**Following the plan:**
- **Done automatically:** problems solved, topics viewed, and kata days with a solved
  kata.
- **By hand:** reviews are marked done by the user.
- **Reschedule:** moves the item and every pending item after it by one day.

**Calendar:** RFC 5545 (CRLF, 75-octet folding, escaped text).
- One all-day event per plan day, with UID `katabox-plan-<created>-<day>@katabox.local`.
- Saved through a Save dialog opened by Rust. The webview still has no dialog
  permission.
- The UI sends the words for the file in its language.

**Export:** v2 sections `study_plans` (insert if new; one stays active, the newest) and
`plan_items` (newer `updated_at` wins).

## Consequences
- Kata results never change the ordinary statistics, reviews or the daily problem.
- Locks are enforced in kb-app, not only in the UI.
