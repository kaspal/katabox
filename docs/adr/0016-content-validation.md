# 16. Validating problem content

Date: 2026-11-11 · Status: proposed

## Context
Until now `kb-pack validate --run` only judged each reference solution against its
problem's tests. It did not check:
- that the tests themselves respect the constraints;
- that they force the complexity the editorial promises;
- that the references are correct beyond the hand-written tests.

KataBox will accept community-contributed problems, so these checks must be automatic.
Any script a problem ships must be safe to run.

## Decision

### Problem scripts run in the sandbox
A problem may ship four Python scripts (standard library only):
- `gen.py`: `generate(seed, size) -> input`, deterministic through `random.Random(seed)`;
- `validate.py`: `validate(input) -> None | broken constraint`;
- `solutions/brute.py`: obviously correct and slow;
- `solutions/naive.py`: correct, with the complexity the editorial improves on.

None of them ever runs on the host.
- **gen.py and validate.py:** `kb_judge::tools` runs them through kb-runner exactly like
  user code: a fresh work folder under the sandbox root, only the Python toolchain
  readable, CPU/wall/memory/output limits. The author's script is imported by a fixed
  driver that reads a JSON request on stdin and prints one JSON reply. Each batch
  (500 inputs, or all of a problem's tests) is one sandboxed run.
- **brute.py and naive.py** are ordinary solutions: they go through `kb_judge::judge`.

A test shows that a `gen.py` reading or writing outside the sandbox gets `PermissionError`.

### `kb-pack validate --run`
| Check | Rule |
| --- | --- |
| validator | `validate.py` accepts every committed test |
| reference (×4 languages) | Accepted on every test, total test time ≤ 50% of `time_limit_ms` |
| stress | 500 inputs from seeds 1-500 (`size = "small"`), each accepted by `validate.py`; `brute.py`'s answers are the expected outputs; every reference must agree (problem checker). The first disagreement is reported with seed, input, both answers |
| weak-test guard | `naive.py` is correct on every test it reaches and gets Time Limit Exceeded on at least one |

- **Speed rule:** the 50% compares the summed test time of the run with the problem's
  limit, because the judge applies the limit as one CPU budget per run (ADR 0008). It is
  a margin for slower machines.
- **naive.py:** required when the editorial promises a better complexity. That is a review
  rule (`docs/problem-authoring.md`): the check runs whenever `naive.py` exists.
- Stress and brute runs use a relaxed time limit (60 s per batch); they check
  correctness, not speed.
- Findings are structured (`kb_pack::check_problem` returns `Finding`s), and Rust tests
  cover each rule on altered copies of two-sum.

### Generated tests are committed
`kb-pack generate <problem> --size max --seeds 1,2` writes hidden tests:
- the inputs come from `gen.py`, checked by `validate.py`;
- the outputs come from `ref.py`, all in the sandbox.

The files are committed. Building content.db (kb-pack build, and the desktop app's build
script) never runs scripts, so it needs no toolchains and stays reproducible. Max sizes are
kept so a problem's tests stay around 1 MB.

### The validator in the app
- **content.db:** schema version 2 adds `problems.validator` (the `validate.py` source).
  The app ships its content.db, so there is no user migration.
- **Custom tests:** when a problem has a validator and the Python runtime is installed
  (ADR 0015 receipts), the app runs it on the custom inputs in the sandbox before the
  reference solution. A rejection is `invalid_custom_input` with the message
  `custom input N: <constraint>`, shown directly in the Testcases tab.
- **Without Python:** the validator is skipped (the maintainer's choice). The reference
  solution still rejects inputs it cannot handle. The tab says that constraints are only
  checked with Python, and offers to install it.
- Validator messages are constraints in math notation, which reads the same in both UI
  languages.

## Consequences
- Every problem in `content/problems` (10 now) passes these checks before merge. Authors
  get a precise failure (seed and input) instead of a wrong problem in production.
- The weak-test guard proves the complexity for Python. Compiled languages share the same
  time limit, so an O(n²) Rust solution may still pass n = 5·10⁴; per-language limits are
  a follow-up.
- `validate --run` takes a few seconds per problem (each check is one sandboxed run).

## Follow-ups
- Per-language time limits (or calibration), so the guard also applies to compiled
  languages.
- Custom checkers (`checker = "custom"`) through the same sandboxed script mechanism.
- Running `validate --run` in CI on Windows once Actions is available.
