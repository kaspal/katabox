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
numeric order. `rank` (1-10) orders the library list.

## Signature types

`[signature]` declares the function every language implements. Types: `int`, `float`,
`string`, `bool`, and any of these followed by `[]` (nestable, e.g. `int[][]`).

| problem.toml | Python | TypeScript | PHP | Rust |
| --- | --- | --- | --- | --- |
| `int` | `int` | `number` | `int` | `i64` |
| `float` | `float` | `number` | `float` | `f64` |
| `string` | `str` | `string` | `string` | `String` |
| `bool` | `bool` | `boolean` | `bool` | `bool` |
| `T[]` | `list[T]` | `T[]` | `array` | `Vec<T>` |

Names: the function name is camelCase in `problem.toml` (`twoSum`). Python and Rust use
snake_case (`two_sum`); TypeScript and PHP keep camelCase. A name that is a keyword gets a
trailing `_`.

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
  the max tests also stop the slow approach in the fastest language. If it passes, raise
  the constraint (larger n in the statements, `validate.py` and `gen.py`'s `max`) and
  regenerate the max tests.

`gen.py` and `brute.py` come as a pair.

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
| naive.py, naive.rs | each is Accepted on every test it reaches and gets **Time Limit Exceeded** under its own language's limit |

## Generating tests

```sh
cargo run -p kb-pack -- generate content/problems/<id> --size small --seeds 101,102,103
cargo run -p kb-pack -- generate content/problems/<id> --size max --seeds 1,2
```

This runs `gen.py` in the sandbox, checks each input with `validate.py`, computes the
expected output with `solutions/ref.py` (also in the sandbox), and writes the next
numbered files in `tests/hidden/` (`--from N` to choose the first number).

**Generated tests are committed.** Building `content.db` never runs scripts, so the app
and CI build without toolchains. Keep max sizes so a problem's tests stay around 1 MB in
total (for example n ≤ 10⁵ with values ≤ 10⁹ is about 1 MB); one max test is enough when
it already makes both naive solutions run out of time. The CPU cap covers the whole run,
so several max tests add up. To replace max tests, delete their files and run `generate`
with `--from` set to the first freed number.

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
