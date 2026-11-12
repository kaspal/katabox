# 17. Fair time limits per language

Date: 2026-11-11 · Status: accepted

## Context
Until now every language shared a problem's `time_limit_ms`, plus a fixed 1 s startup
allowance. Python references run 10-100× slower than Rust ones on the same work, so the
limit had to suit Python. Two problems followed:
- An O(n²) solution in Rust (or TypeScript) passed tests built to stop it.
- The weak-test guard (ADR 0016: `naive.py` must get TLE) proved the complexity for Python
  only.

Machines differ too. A limit tuned on a fast desktop can fail a correct solution on a slow
laptop.

## Decision

### Base × language factor
- **Base limit:** `problem.toml`'s `[judge] time_limit_ms` keeps its name. It now means the
  **base limit**, set for Rust, the fastest language.
- **Per-language settings:** each language declares how its limit derives from the base, in
  `languages/<lang>/language.toml`:

  ```toml
  [time]
  factor = 10        # × the base limit
  startup_ms = 1000  # CPU for starting the runtime and the harness
  ```

- **The limits:**
  - limit(lang) = base × factor;
  - CPU cap of the run = (limit + startup_ms) × scale.
- **Code:** `kb_judge::limits` computes them; the judge sets kb-runner's per-run CPU cap from
  them. kb-runner is unchanged, because limits were already per run.
- **The measure** stays CPU time for the whole batch of tests (ADR 0008).
- **TLE:** the message names the effective limit (limit × scale).
- **Reports:** a new `Report.cpu_us` (serde default) records the run's CPU time.
- **Overrides:** per-problem overrides of a factor can be added later; none is needed now.

Tuned on the 10 problems (reference machine; refs' CPU includes runtime startup):

| Language | factor | startup_ms | Limit at base 300 ms | Slowest reference (CPU of the whole run) |
| --- | --- | --- | --- | --- |
| Rust | 1 | 100 | 300 ms (cap 400) | 125 ms (rainfall: parsing 2 MB of input) |
| TypeScript | 2 | 500 | 600 ms (cap 1100) | 187 ms |
| PHP | 5 | 500 | 1500 ms (cap 2000) | 156 ms |
| Python | 10 | 1000 | 3000 ms (cap 4000) | 359 ms |

Every problem uses base 300 ms. The O(n²) naive solutions measured on the max tests with a
relaxed limit:
- **naive.rs:** 1.0-1.2 s (two-sum, rainfall), 1.3-3 s (budget, stretch, booking, rising), up to
  15-20 s (nested-brackets, print-shop).
- **naive.py:** 15 s and up.

So the Rust cap of 400 ms stops every naive.rs with a margin of at least 2.5×.

### Calibration: the machine's time scale
- **The benchmark** (`kb_judge::calibration`) is short, fixed and CPU-bound: integer
  arithmetic, hash-map updates, string building and sorting.
  - It returns a checksum, so a run that skipped work is caught.
  - It runs in the sandbox like user code (`tools::run_script`), in Python, or in
    JavaScript on Node when Python isn't installed.
  - Each version has its own reference time, measured on the reference machine (the
    machine the limits were tuned on). Python is 216 ms and Node 198 ms for the fastest
    of 5 rounds.
  - Rounds are timed in-process with a high-resolution clock. Windows CPU-time counters
    tick every 15.6 ms, too coarse for a 200 ms round. The fastest round counts, which
    filters out other programs.
- **The factor:** factor = measured ÷ reference. The judge applies it clamped to
  **[0.5, 3.0]** (`JudgeEnv::time_scale`), so a fast machine gets less time and a slow one
  more, within bounds.
- **When it is measured:** once per machine.
  - The first Run or Submit measures first when Python or Node is installed, and so does
    the end of a language install. That costs about 1.5 s, once.
  - The result is stored in user.db as the setting `judge.calibration` (factor, applied,
    runtime, times, date), so no migration is needed.
  - Without a benchmark runtime, the scale stays 1.0 and the measurement is retried later.
- **In the app:**
  - Settings → Judge speed shows the factor (and the measured value when clamped), the
    runtime and the date, with **Recalibrate**. It fails with `calibration_unavailable`
    when neither runtime is installed.
  - The workspace shows the time limit for the editor's language at this machine's scale.
    A note "adjusted ×1.30 for this computer" links to Settings.
  - `ProblemView` gains `time_limits` (per language) and `time_scale`.

### kb-pack rules (`validate --run`)
- **Judging scale:** limits are judged at scale 1.0 (the reference machine).
- **Machine factor:** `validate --run` first prints this machine's factor, with a warning
  outside 0.8-1.25, because speed results may differ on the reference machine.
- **Reference speed:** each reference must be Accepted using **at most 50% of its own
  language's CPU cap** (startup included), measured as `Report.cpu_us`. ADR 0016 compared
  the summed wall time of the tests with the single shared limit.
- **Weak-test guard:** `solutions/naive.rs` is required whenever `solutions/naive.py`
  exists (a static check). Both must be correct on every test they reach, then get TLE
  under their own language's limit.
- **Relaxed runs:** stress, brute and `generate` use a relaxed base of 6 s (60 s for
  Python).

### Content changes
These raised constraints let naive.rs run out of time:
- **two-sum:** n ≤ 10⁴ → **n ≤ 10⁵**, with one max test (1.0 MB) instead of two at 10⁴.
- **rainfall-totals:** n, q ≤ 5·10⁴ → **n, q ≤ 10⁵**, with one max test of nearly full ranges
  (2.1 MB with its 10⁵ outputs; it was 1.4 MB).

## Consequences
- The weak-test guard now proves the intended complexity in Python and Rust. TypeScript and
  PHP have factors between them and are covered by the same tests. An O(n²) TypeScript
  two-sum now gets TLE.
- **Authors:** a new problem sets one base limit and adds `naive.rs` with `naive.py`.
  `validate --run` reports each language's CPU against its own budget.
- **User's view:** on a slow machine limits grow up to 3×; on a very fast one they shrink
  to 0.5×. Users see the adjusted number.
- **Wall time** stays 3× the CPU cap. A machine much slower than 3× the reference may
  still see TLE on correct but heavy solutions.
- **Benchmark reference times** must be re-measured if the workload, or the pinned Python
  or Node version, changes:
  `cargo test -p kb-judge --test calibration_windows -- --nocapture`.
- **Content size:** rainfall-totals' tests exceed the "about 1 MB" guideline (2.1 MB).

## Follow-ups
- Per-problem factor overrides, if a problem needs them (for example heavy string work in
  PHP).
- Running the guard for TypeScript and PHP too (`naive.ts`, `naive.php`) if a problem
  shows a gap.
- Recording the time scale with each submission, so old verdicts can be explained.
