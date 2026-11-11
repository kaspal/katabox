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
| `solutions/naive.py` | Correct but too slow (weak-test guard), where the complexity matters |

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

Each reference solution (and `brute.py`, `naive.py`) must declare the function exactly as
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

`gen.py` and `brute.py` come as a pair.

## Checking

```sh
cargo run -p kb-pack -- validate        # structure, metadata, tests, signatures, scripts
cargo run -p kb-pack -- validate --run  # the sandbox checks below (Windows; needs the toolchains)
```

`--run` needs `pwsh scripts/fetch-toolchains.ps1` and checks, per problem:

| Check | Rule |
| --- | --- |
| validator | `validate.py` accepts every public and hidden test |
| python, typescript, php, rust | each reference is Accepted on every test, and its total test time is at most **50% of `time_limit_ms`** (the limit is one CPU budget per run) |
| stress | `gen.py` makes 500 small inputs (seeds 1-500); each passes `validate.py`; `brute.py`'s answers are the expected outputs; every reference agrees with them (the problem's checker). The first disagreement is printed with its seed and input |
| naive | `naive.py` is Accepted on every test it reaches and gets **Time Limit Exceeded** on at least one |

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
total (for example n ≤ 5·10⁴ with values ≤ 10⁶); one max test is enough when it already
makes `naive.py` run out of time.

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
