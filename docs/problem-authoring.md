# Writing a problem

A problem is a folder in `content/problems/<id>/` (CC BY-SA 4.0). Start by copying a
complete one, such as `content/problems/budget-pair/`.

**Problems must be original.** Never copy or paraphrase statements, stories, examples or
test data from LeetCode, HackerRank, Codewars or similar sites. Classic algorithmic ideas
are fine; the wording, names, examples and tests must be our own.

## Files

| File | What it holds |
| --- | --- |
| `problem.toml` | Id (equal to the folder name), title, `title_es`, difficulty, tags, pattern, rank, the typed `[signature]` and the `[judge]` settings |
| `statement.md`, `statement.es.md` | The statement in English and natural Spanish (not word for word): story, `### Example N` blocks, `### Constraints` |
| `hints.md`, `hints.es.md` | Three hint tiers, separated by lines containing only `---`, from a nudge to the approach |
| `editorial.md`, `editorial.es.md` | Approach, a short walkthrough and the complexity |
| `tests/public/*.json` | 3-5 tests shown to the user (the statement's examples first) |
| `tests/hidden/*.json` | 15-40 tests: edge cases, small random ones, and max-size ones |
| `solutions/ref.{py,ts,php,rs}` | Reference solutions, standard library only |
| `gen.py` | Random inputs for a seed (see below) |
| `validate.py` | The constraints, checked on every test and on custom inputs in the app |
| `solutions/brute.py` | Slow but obviously correct (stress test) |
| `solutions/naive.py`, `solutions/naive.rs` | Correct but too slow (weak-test guard), where the complexity matters; always both |

Test files are `{"input": {<param>: value, ...}, "output": value}`, one test per file, in
numeric order. `rank` (1-10) is a difficulty level that orders the library list (ties by
id): easy problems 1-4, medium 5-8, hard 9-10, with earlier ranks for the simpler
problems of the same pattern.

**Tags** are free text in `problem.toml`, but every tag needs a display name in both UI
languages: add it to `tag.names` in `apps/desktop/src/i18n/locales/en.json` and `es.json`
(a kb-pack test fails otherwise). Reuse existing tags where they fit.

## Signature types

`[signature]` declares the function every language implements. Types: `int`, `float`,
`string`, `bool`, `ListNode`, `TreeNode`, and any of these followed by `[]` (nestable,
e.g. `int[][]`, `ListNode[]`). kb-pack checks every test input and output against these
types.

| problem.toml | Python | TypeScript | PHP | Rust |
| --- | --- | --- | --- | --- |
| `int` | `int` | `number` | `int` | `i64` |
| `float` | `float` | `number` | `float` | `f64` |
| `string` | `str` | `string` | `string` | `String` |
| `bool` | `bool` | `boolean` | `bool` | `bool` |
| `ListNode` | `ListNode \| None` | `ListNode \| null` | `?ListNode` | `Option<Box<ListNode>>` |
| `TreeNode` | `TreeNode \| None` | `TreeNode \| null` | `?TreeNode` | `Option<Rc<RefCell<TreeNode>>>` |
| `T[]` | `list[T]` | `T[]` (`(T \| null)[]` for nodes) | `array` | `Vec<T>` |

Names: the function name is camelCase in `problem.toml` (`twoSum`). Python and Rust use
snake_case (`two_sum`); TypeScript and PHP keep camelCase. A name that is a keyword gets a
trailing `_`.

### Linked lists and binary trees (ADR 0025)

- **What the types are:** `ListNode` is a singly linked list of `int`; `TreeNode` is a
  binary tree of `int`.
- **In tests, they are JSON arrays:**
  - **a list** is its values in order: `[4, 8, 15]` is 4 → 8 → 15, and `[]` is the empty
    list;
  - **a tree** is its level order, top to bottom and left to right, with `null` for a
    missing child. The children of a missing node are not listed and trailing nulls are
    trimmed: `[1, null, 2, 3]` is 1 with a right child 2, whose left child is 3. `[]` is
    the empty tree.
- **Canonical trees only:** kb-pack refuses a tree that is not canonical (a null root, a
  trailing null, a slot no node owns).
- **The starter code defines the classes before the function,** and every reference
  must contain the same definitions. Copy them from `cargo run -p kb-judge -- stub <id>
  --lang <lang>`. kb-pack compares the function's declaration line.
- **The scripts:**
  - `validate.py` receives the raw JSON arrays;
  - `brute.py`, `naive.py` and the references receive node objects;
  - `brute.py` builds its result with the starter class.
- **Recursion depth:** keep tree height at most 1000 in the constraints (and in
  `validate.py`), so recursive solutions work in every language.
- **Statements:** explain the encoding once, with a small example (see `playlist-rotation`
  and `fullest-level`).

### Other shapes

These shapes stand in for structures that have no type:

- **General trees:** a parent array (`int[]`, the root's parent is `-1`, any numbering;
  see `family-generations`, `sales-teams`, `common-boss`) or an edge list (`int[][]`).
  `validate.py` checks for one root and no loops.
- **Design problems** (a structure driven by operations): `ops: string[]` names each
  operation and `args: int[][]` holds its numbers, one row per operation; the result is
  `int[]` with one value per operation that reports something (see `corner-shop`,
  `library-holds`). `validate.py` checks each operation's arity and any rule the
  statement promises.
- **Backtracking:** return a count or one canonical answer with an exact tie rule
  (`outfit-picker`, `postage-stamps`), so outputs stay small and checkable with `exact`.

## Program problems (ADR 0028)

A **program problem** is solved by a whole program that reads standard input and writes standard output (HackerRank style), instead of a function the harness calls. It lives in a normal pattern; the UI shows a "Program · stdin/stdout" badge.

- `problem.toml`: `mode = "program"` (top level), **no `[signature]`**, and a text checker:
  - `checker = "tokens"` (the default choice): whitespace-separated tokens must be equal, so spacing and blank lines do not matter;
  - `checker = "tokens-float:1e-6"`: the same, numeric tokens within that absolute or relative tolerance;
  - `checker = "lines"`: lines must be equal after trailing spaces and tabs are removed; trailing empty lines do not count.
  CRLF is the same as LF for every text checker. `time_limit_ms` is the base for **each test's run** (each test starts the program again).
- Tests: `tests/{public,hidden}/*.json` as `{"stdin": "...", "stdout": "..."}`. Keep 3-5 public (the examples) and 15-20 hidden: every test pays the runtime's start-up.
- Statements: `## Input format` and `## Output format` sections (Spanish: `## Formato de entrada`, `## Formato de salida`), then `### Example N` (`### Ejemplo N`) sections, each with **two code fences**: the input, then the output. kb-pack checks that example N equals public test N (up to the final newline), in both languages.
- Every integer token in stdin and stdout stays within ±(2⁵³ − 1) (kb-pack checks it): a TypeScript solution reads numbers with `Number`. Modular answers use p = 1 000 003 so products stay exact.
- Scripts: `gen.py`'s `generate(seed, size)` returns the stdin **text**; `validate.py`'s `validate(text)` gets the text and returns the broken constraint or `None`. `brute.py`, `naive.py`, `naive.rs` and the references are programs.
- References read all of stdin at once (`sys.stdin.buffer.read()`, `readFileSync(0)`, `stream_get_contents(STDIN)`, `read_to_string`) and write the output once (Rust: a `BufWriter`). Standard library only.
- `validate --run` judges speed by the **slowest test's** CPU (each test is its own run).
- The complexity profiler works for program problems too (ADR 0024, "Program problems"): gen.py's int branch returns the stdin text for size n, and samples are wall times minus the language's startup. Pick sizes where the reference's work is not dwarfed by the startup if you can; a fast reference is shown as "too fast to tell apart", which is honest but less useful. Skip the profile when the naive solution's class is not in the fit's set (constant, log, linear, n log n, quadratic, exponential), e.g. a cubic naive.
- The model package is `content/problems/ticket-tally/`.

## Fix-the-bug problems (ADR 0035)
- `kind = "fix"` in problem.toml; a function problem, never a follow-up.
- `solutions/buggy.py` and `solutions/buggy.ts` are required, `buggy.php` and `buggy.rs`
  optional. Each contains the stub's declaration line, like the references.
- One realistic bug per problem, and the generator must produce inputs that trigger it:
  `validate --run` judges every buggy file on all tests and fails when one is Accepted.
- The statement opens with a short story about the failing code and never says where the
  bug is; the hints narrow it down; the editorial names the line and shows the fix.

## Time limit (ADR 0017)

`[judge] time_limit_ms` is the **base limit**: CPU time for the whole batch of tests in
Rust, the fastest language. Each language scales it with its `[time]` table in
`languages/<lang>/language.toml` and adds a startup allowance:

| Language | Limit | CPU cap of the run |
| --- | --- | --- |
| Rust | base × 1 | + 100 ms |
| TypeScript | base × 2 | + 500 ms |
| PHP | base × 5 | + 500 ms |
| Python | base × 10 | + 1000 ms |

The app also multiplies every limit by the machine's speed factor (0.5-3.0), measured
once with a short benchmark; kb-pack always judges at 1.0. Start with `time_limit_ms =
300` (every current problem uses it). Lower it only if a naive solution gets through;
raise it only if a reference needs more than half of its language's cap.

Each reference solution (and `brute.py`, `naive.py`, `naive.rs`) must declare the function exactly as
the generated starter code does (whitespace aside); `kb-pack validate` checks this. To see
the starter code:

```sh
cargo run -p kb-judge -- stub <id> --lang <python|typescript|php|rust>
```

## Problem scripts (ADR 0016)

All scripts are Python, standard library only. They never run on the host: kb-pack and the
app run them in the same sandbox as user code.

- **`gen.py`** defines `generate(seed: int, size: str) -> dict`, returning one input (a
  dict of parameters). `size` is `"small"` (stress test: tiny inputs where `brute.py` is
  fast and edge cases are likely) or `"max"` (the largest inputs the constraints allow).
  It must be deterministic: use only `random.Random(seed)`. Running it directly
  (`python gen.py 7 small`) prints one input as JSON.
- **`validate.py`** defines `validate(inp: dict) -> str | None`: `None` for a valid input,
  otherwise the broken constraint **as the statement writes it** (`"2 ≤ prices.length ≤
  5·10⁴"`). The math notation reads the same in English and Spanish; the app shows it
  under a custom test.
- **`solutions/brute.py`** solves the problem in the most obviously correct way (every
  pair, every subset, a fixed-point loop). Speed does not matter.
- **`solutions/naive.py`** is the correct but slow approach a reader would try first.
  It is **required** when the editorial promises a better complexity than that approach;
  it proves the max-size tests really test complexity. This is a review rule: kb-pack
  cannot tell what the editorial promises.
- **`solutions/naive.rs`** is the same algorithm in Rust, required whenever `naive.py`
  exists (kb-pack checks the pair). Compiled code is 10-100× faster, so it shows whether
  the max tests also stop the slow approach in the fastest language. It must need **at
  least 1.5× Rust's CPU cap** (the weak-test margin, so a faster machine cannot let it
  through): `validate --run` measures it once under 3× the cap and reports the ratio
  (`naive.rs needs 1.76× its CPU cap (703 of 400 ms)`, or `≥ 3.0×` when it still runs
  out of time). Below 1.5×, raise the constraint (larger n in the statements,
  `validate.py` and `gen.py`'s `max`), add a max test, or make the existing one harder,
  and regenerate the max tests.

`gen.py` and `brute.py` come as a pair.

## Complexity profile (ADR 0024)

Every problem with `naive.py` and `naive.rs` is profiled: after an Accepted result, the
app measures the user's code at growing sizes and estimates its complexity class next to
the reference solution's line.

- **`problem.toml`:** `profile = true` and `profile_sizes = [300, 1000, 3000, 10000,
  30000, 100000]`: 4-8 positive, strictly increasing sizes, from small to the problem's
  max n.
  - Exponential or small-n problems use small sizes (`[5, 6, …, 12]`).
  - Add a few small sizes before the large ones when the naive approach is exponential,
    so it shows its growth before it runs out of time.
- **`gen.py`:** `generate(seed, size)` also takes an **int** size n, right after
  `rng = random.Random(seed)`:

  ```python
  def generate(seed, size):
      rng = random.Random(seed)
      if isinstance(size, int):
          # Profile (ADR 0024): n readings and n queries.
          return {...}
  ```

  - n drives the input. Say in the comment how the other dimensions follow it (q = n,
    a √n × √n grid, k = n/2).
  - Use values that make the reference do its real work (no early exit at the start).
  - Use small value ranges, to keep the data lean.
  - `__main__` accepts `--n N [--seed S]` as well as the positional form.
- **`kb-pack profile content/problems/<id>`** (sandbox, all four toolchains):
  1. generates each size with seed 1;
  2. checks it with `validate.py` and the **profile budget: 512 KB compressed** per
     problem (separate from the tests' budget);
  3. times `ref.*` in every language (3 runs per size, the median, normalized to the
     reference machine);
  4. writes `profile/reference.json`: the sizes, the seed, a SHA-256 per input, and the
     times.

  **Commit only `reference.json`.** The inputs go to `profile/inputs/`, a gitignored
  cache. Re-run it after changing `gen.py`'s int branch, the sizes, a reference
  solution, or a runtime pin.
- **`kb-pack profile --inputs [content/problems]`** regenerates the caches and verifies
  them against the hashes. `kb-pack build` packs a profile only from a matching cache;
  without one, it leaves the problem unprofiled with a warning.
- **`kb-pack validate`** checks `reference.json` and any cached inputs against the
  hashes.
- **`validate --run`** regenerates every input (refreshing the cache) and checks:
  - it matches its hash;
  - it passes `validate.py`;
  - it is within the budget;
  - the inputs never get smaller with n.

## Checking

```sh
cargo run -p kb-pack -- validate        # structure, metadata, tests, signatures, scripts
cargo run -p kb-pack -- validate --run  # the sandbox checks below (Windows; needs the toolchains)
```

`--run` needs `pwsh scripts/fetch-toolchains.ps1`. It first prints this machine's speed
factor (the limits are still judged at 1.0; a warning appears when the machine is far
from the reference one), then checks, per problem:

| Check | Rule |
| --- | --- |
| validator | `validate.py` accepts every public and hidden test |
| python, typescript, php, rust | each reference is Accepted on every test using at most **50% of its language's CPU cap** for the run (startup included) |
| stress | `gen.py` makes 500 small inputs (seeds 1-500); each passes `validate.py`; `brute.py`'s answers are the expected outputs; every reference agrees with them (the problem's checker). The first disagreement is printed with its seed and input |
| naive.py | Accepted on every test it reaches, and **Time Limit Exceeded** under Python's limit |
| naive.rs | correct, and needs **at least 1.5× Rust's CPU cap** (measured under 3× the cap; the ratio is printed) |
| profile | with `profile = true`: `gen.py` regenerates every profile input matching its hash in `reference.json`, each passes `validate.py`, they grow with n and fit the budget (and the cache is refreshed) |

## Generating tests

```sh
cargo run -p kb-pack -- generate content/problems/<id> --size small --seeds 101,102,103
cargo run -p kb-pack -- generate content/problems/<id> --size max --seeds 1,2
```

This runs `gen.py` in the sandbox, checks each input with `validate.py`, computes the
expected output with `solutions/ref.py` (also in the sandbox), and writes the next
numbered files in `tests/hidden/` (`--from N` to choose the first number).

**Generated tests are committed** (ADR 0018). Building `content.db` never runs scripts,
so the app and CI build without toolchains. To replace max tests, delete their files and
run `generate` with `--from` set to the first freed number.

### Size budget (ADR 0018)

content.db stores each test's input and output compressed with zstd. `kb-pack validate`
prints every problem's size (`tests 1016 KB → 465 KB compressed (90% of 512 KB)`) and
**fails a problem over 512 KB compressed**. Random numbers barely compress, so keep max
tests lean:

- Use the smallest value range that keeps the problem honest. The constraints may allow
  values up to 10⁹, but a max test with values below 1000 tests the same complexity in a
  third of the space.
- One max test is enough when it already stops both naive solutions. The CPU cap covers
  the whole run, so several max tests add up anyway.
- Outputs count too. An answer of one number is cheap; an array of 10⁵ numbers is not.
- Structure compresses: sorted lists, repeated values and runs cost far less than
  shuffled random data. Keep some randomness where it matters (for example, to defeat
  shortcuts), but do not shuffle without a reason.

## Review checklist

- Original wording, story, names, examples and tests (see the top of this page).
- Spanish texts read naturally; constraints and examples match in both languages.
- Hidden tests cover the minimum sizes, equal values, the boundaries of every
  constraint, and the cases the editorial calls out.
- `validate --run` passes, including stress and, where it applies, naive.

To compile every problem into the app's `content.db` (it validates first and refuses to
build if any problem fails):

```sh
cargo run -p kb-pack -- build --out target/content.db
```
