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

## Consequences
- Kata results never change the ordinary statistics, reviews or the daily problem.
- Locks are enforced in kb-app, not only in the UI.
