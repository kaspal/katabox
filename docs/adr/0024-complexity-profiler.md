# 24. Complexity profiler and the JavaScript editor mode

Date: 2026-11-12 · Status: accepted

## Context
Enhancement 9 of the build plan: after an Accepted result, measure the solution at
growing input sizes and estimate its complexity class.
- **Inputs:** gen.py with an `--n` size, for problems marked `profile = true`.
- **Runs:** 6 sizes (10²-10⁵, capped by the time limit), 3 repetitions each, the median.
- **The fit:** least squares against O(1), O(log n), O(n), O(n log n), O(n²) and O(2ⁿ).
- **The UI:** a log-log chart with the user's line, the reference line and the
  best-fit label.
- **Done when:**
  - a calibration set of at least 20 known solutions gets the right class at least 85%
    of the time;
  - the label always says "estimated";
  - a run takes under 15 s.

Every harness already times each call inside the process (`TestRecord.time_us`: the
call only, no process start, no parsing). Tests with `expected: null` run without
checking.

The same task adds a JavaScript editor mode: "JavaScript" in KataBox is the TypeScript
entry (ADR 0023), whose strict checking reports implicit-any on every untyped
parameter of plain JavaScript.

## Decision

### Inputs: precomputed, generated and hash-verified
The maintainer chose precomputed inputs: the app never needs Python to profile. The
inputs are packed into content.db, but **not committed**: only their hashes are.
- **`problem.toml`:** `profile = true` plus `profile_sizes = [n₁, …]`: 4-8 positive,
  strictly increasing sizes. kb-pack requires them together, and refuses a `profile/`
  folder without them.
- **`gen.py`:** `generate(seed, size)` also takes an int `size = n`. It returns an input
  whose driving dimension is n, and says in a comment how the other dimensions scale
  (for example, q = n queries, a √n × √n grid, or k = n/2). `python gen.py --n N
  [--seed S]` prints it.
- **`kb-pack profile <dir>`** (in the sandbox):
  1. generates every size with seed 1;
  2. checks each input with validate.py and the profile budget;
  3. writes the inputs into the **cache**, `profile/inputs/<n>.json` (gitignored);
  4. profiles `ref.*` in all four languages;
  5. writes `profile/reference.json` (**committed**): `{sizes, seed, sha256,
     machine_factor, times_us: {language: [µs…]}}`. `sha256` is one hash per size, of the
     input as compact JSON. The times are divided by the machine factor (ADR 0017), so
     they are for the reference machine.
- **`kb-pack profile --inputs [DIR]`** regenerates the caches of one problem or of every
  profiled problem, and verifies each input against its committed hash.
- **`kb-pack validate`** (no toolchain) checks:
  - the reference: sizes, seed, one hash per size, and times for every language at
    every size;
  - any cached input against its hash;
  - no stray files in the cache.
- **`validate --run`** regenerates every input in the sandbox (and so refreshes the
  cache). Each must:
  - match its committed hash (gen honours `--n`, deterministically);
  - pass validate.py;
  - be no smaller than the previous size, the last larger than the first.

  Together they must fit the **profile budget of 512 KB compressed** per problem
  (separate from the tests' budget).
- **`kb-pack build`** packs a problem's profile only from a complete cache that matches
  every hash:
  - a mismatching cache fails the build;
  - without a cache, the problem is packed without profile data (its Complexity action
    is hidden), with a warning.

  So builds (the desktop crate's build.rs, `pnpm dev`, CI) never need the toolchains. The
  **Windows release job** fetches the runtimes and runs `kb-pack profile --inputs` before
  the build, so its installers carry every profile.
- **How this differs from tests (ADR 0018):**
  - **Tests** stay committed as JSON: people review them, and the build never generates
    them.
  - **Profile inputs** are machine-made and never reviewed by hand. Committing them
    added 37 MB of JSON, so git keeps only the hashes (with the seed and the sizes), and
    gen.py reproduces the inputs.
  - **The cost:** a build carries profiles only where the sandbox and Python have run
    (`validate --run`, `kb-pack profile --inputs`, the Windows release).
- **content.db schema v4:**
  - `profile_inputs(problem_id, n, input)` (zstd JSON);
  - `profile_reference(problem_id, language, n, time_us)`;
  - `ProblemView.profile`.
- **Which problems:** the 53 with `naive.py` and `naive.rs` (the editorial promises a
  better complexity than the naive approach).
  - Typical sizes run from 300 to 100 000.
  - Exponential or small-n problems use small sizes:
    - delivery-round: 5-12;
    - typo-distance and seed-rows: a few small sizes, then large ones.
  - Scalar inputs use the number itself: change-machine's amount, print-shop-speed's
    largest job, shared-tile's side.

### Runs (`kb-judge/src/profile.rs`)
- **One ordinary judge run per size**, through the existing sandbox path: no kb-runner
  change. Each run holds the input 3 times (`expected: null`); the point is the median
  per-call time.
- **The budget:**
  - The whole profile has **12 s** of wall time.
  - Each size's CPU cap is the remaining budget, at most 5 s. It is turned into a base
    limit per language and machine scale (ADR 0017).
  - Before each size, the cost is predicted from the last point (n² growth plus the
    run's overhead). The profile stops early (`budget`) rather than overrun.
- **The stops:**
  - a Time Limit Exceeded stops it (`time_limit`);
  - any other failure stops it without numbers from that size (`failed`, with the
    verdict);
  - a compile error stops it (`compile`, with the output).

### The fit (`kb-judge/src/complexity.rs`)
- **The model:** for each class f, `t = c + a·f(n)` with a, c ≥ 0. `c` is the per-call
  overhead and `a` the cost per unit of work.
- **Weighted least squares** on relative errors (weights 1/t²), so small and large
  sizes count alike.
- **No overflow:** f is evaluated relative to the largest size, with 2ⁿ in the log
  domain.
- **Occam's razor:** the **simplest** class whose error is within **2.5×** the best
  error wins. When that is not the best-fitting class, the best fit is shown as the
  alternative. Otherwise the runner-up is shown when its error is within 1.5×.
  - **Why:** caches and allocation make large inputs a little slower per element. With
    a plain "smallest error" rule, that reads as an extra log factor: the first
    calibration run labelled 4 of 14 O(n) solutions O(n log n).
  - **The factor:** 2.5 and 3.0 give the same results; 4.0 starts calling O(n²)
    O(n log n).
- **Confidence:** `1 − e₁/e₂` (the gap between the two best fits).
- **Noisy:** set when most points are under 1 ms.

### Honesty rules (UI)
- **The label:** always "Estimated: O(…)", and "Estimated: O(a) or O(b)" when there is
  an alternative.
- **Confidence in words:** clear, reasonable, or "several curves fit about as well".
- **Too few points:** with fewer than 3, there is no class, only "not enough sizes ran
  in time".
- **Noise:** noisy timings are flagged, never hidden.
- **Early stops:** the stop reason is shown, and unrun sizes say "not run" in the
  table.
- **The explanation** says what was measured and that the result is an estimate.
- **The reference line:** the reference times at the user's machine scale
  (`reference × time_scale`), with the reference's own estimated class.

### App and UI
- **The command:** `profile_solution(problem, language, code)` takes the judge slot (a
  concurrent Run, Submit or profile gets `judge_busy`).
  - A problem without data gets `profile_unavailable`.
  - It is allowed in mock interviews (it reveals nothing locked), in the interview's
    languages only.
- **The Result tab:** after an Accepted Run or Submit on a profiled problem, a
  Complexity section measures the code that was judged. A new result forgets the old
  measurement.
- **The chart:**
  - an SVG log-log chart in a `<figure>` (`role="img"` with the summary as its label);
  - your line is solid with circles; the reference is dashed with squares;
  - a legend; tokens `--chart-you` and `--chart-ref` (≥ 3:1 on the background and on
    cards, in both themes; tested).
- **The table:** n, yours and the reference, in ms. It is the chart's text alternative.

### JavaScript editor mode
- **The toggle:** a per-problem **JS / TS** toggle next to the language picker while
  the language is TypeScript. It is saved as `editor.script.<problem>`, so imports
  carry it (`editor.` allow-list).
- **JS mode:**
  - the editor uses the JavaScript grammar;
  - the language server connects as `Language::JavaScript` (served by the TypeScript
    server). kb-bridge writes that project's tsconfig with `checkJs: true` and
    `strict: false`.
  - TypeScript keeps `strict: true`.
- **What does not change:** spec.rs and policy.rs. The draft, Run, Submit and the
  debugger stay TypeScript (Node runs plain JavaScript through type stripping).

## Calibration
- **The test:** `crates/kb-judge/tests/complexity_windows.rs` profiles 28 known
  solutions: ref and naive of 7 problems, in Python and Rust.

  | Problem | ref | naive |
  | --- | --- | --- |
  | two-sum | n | n² |
  | balanced-streaks | n | n² |
  | fewest-hops | n | n² |
  | thermostat-swings | n | n² |
  | queue-swaps | n log n | n² |
  | shelf-codes | n log n | n² |
  | path-tiles | n | 2ⁿ |

- **Results** (this Windows machine, release build):

  | Rule | Hit rate | Misses |
  | --- | --- | --- |
  | Smallest error | 24-25/28 (86-89%) | O(n) → O(n log n) |
  | Occam 2.5× (adopted) | 28/28 and 26/28 (93-100%) | O(n) → O(n log n): Python two-sum, thermostat-swings |

- **Speed:** the slowest run took 6.5 s. Naive solutions stop at the 5 s size cap or on
  the budget, with 3 or more points.
- **Python two-sum's reference** reads O(n log n) on most runs: the dict of 10⁵ ints
  grows measurably faster than linear. The kb-app end-to-end test therefore uses
  balanced-streaks.

## Consequences
- **Size:** content.db grows by about 7 MB with every profile (321 inputs, 7.2 MB
  compressed). The repository grows only by the 53 reference files (hashes and times),
  not by the inputs.
- **Linux installers** ship without profiles until sandboxed problem scripts run on
  Linux; dev builds without a cache likewise.
- **A pin change:** a runtime pin change or a calibration benchmark change (ADR 0017)
  means re-running `kb-pack profile` for the reference times, alongside
  `validate --run`.
- **Timings vary between runs and machines.** The calibration rate is a property of
  this machine; the UI's wording keeps it an estimate.
- **Weak profiles:** a few profiles say little:
  - shared-tile's gcd runs in about 1 µs at every size (noisy, O(1)-like);
  - feature-flags' Python reference is dominated by a fixed 2¹⁶ precomputation.

  They still separate the naive solutions.
