# 28. Program problems (stdin/stdout)

Date: 2026-11-14 · Status: accepted (maintainer sign-off on the judge mode, the format and schema 5)

## Context
- **Every problem was a function:** the harness calls `f(args)` with JSON arguments and
  compares the returned JSON value (ADR 0008).
- **The maintainer asked for HackerRank-style program problems:** the user's file is the
  whole program; it reads standard input and writes standard output.
- **Decisions fixed by the maintainer:**
  - it is a **flag on a problem**, in the existing patterns, with a badge and a library
    filter;
  - about 15 new problems teach input handling;
  - checking is **token-based** by default.

## Decision
### The mode, and one plumbing for both
- **`problem.toml`:**
  - gains a top-level `mode = "program"`; the default is `"function"`;
  - a program problem has **no `[signature]`**.
- **Inside the judge,** a program problem has the synthetic signature
  `main(stdin: string) -> string` (`kb_judge::program_signature`).
- **Tests:**
  - on disk, a test is `{"stdin": "...", "stdout": "..."}`;
  - the loader turns it into `TestCase { input: {"stdin": text}, expected: "<stdout>" }`.
- **What works unchanged:** tests, `check_input`, custom tests (the expected output comes
  from the reference, as today), failures, "Debug this case", the compressed test blobs
  (ADR 0018) and the export format.
- **The mode itself** is `kb_core::ProblemMode`, shared by every crate and by the UI.

### Checkers (`kb-judge/src/checker.rs`)
- **The text checkers, for program problems only:**
  - `tokens` (the default): tokens separated by ASCII whitespace;
  - `tokens-float:<eps>`: numeric tokens within an absolute or relative tolerance (`nan`
    and `inf` are words);
  - `lines`: lines after trailing spaces and tabs are removed; trailing empty lines don't
    count, inner empty lines do.
- **Line endings:** CRLF and a lone CR count as LF everywhere.
- **Messages:** a mismatch is `OutputMismatch` with a location, for example "line 2,
  token 2: expected `7`, got `8`", "output ended early: expected 3 tokens, got 2" or
  "extra output after …".
- **Mixing is refused:** value checkers (`exact`, `unordered`, `float`) are refused on
  program problems, and text checkers on function problems.

### Judging (`kb-judge/src/program.rs`)
- **There is no harness. One check, then one run per test:**
  - Rust compiles once, through the binary cache.
  - Python, TypeScript and PHP get one short sandboxed syntax check (`compile()`, a
    `node -e` check, `php -l`), so a syntax error is a `CompilationError`, as in function
    mode.
  - The TypeScript check treats the file exactly as the run does: Node's own
    `stripTypeScriptTypes` (strip-only mode), then a parse as an ES module
    (`vm.SourceTextModule`, which never links or evaluates). Not `node --check`: it parses
    a `.ts` file as CommonJS without stripping types, so annotated programs without a
    leading `import` were false compilation errors (fixed in `fix/ts-program-check`).
  - Then each test is one sandboxed run of the user's file (`solution.<ext>`) with
    `spec.stdin` = the test's text. Progress is reported per test.
- **Limits per run:**
  - CPU: the language's usual cap, `limits::cpu_cap_ms(base, def, scale)`, which already
    includes its startup allowance. So `time_limit_ms` is a **per-test** base.
  - Memory: as configured.
  - Output: `PROGRAM_OUTPUT_BYTES` (8 MiB), stdout and stderr together.
- **Verdicts:**
  - a CPU or wall limit is TLE, and the output cap is OLE;
  - the memory limit, or the runtime's out-of-memory text, is MLE;
  - a non-zero exit is RE (`ProcessExited`, with the exit code and the stderr tail);
  - otherwise the checker compares stdout.
- **When the batch stops:** a time or memory limit ends it, as in function mode, because
  every remaining test would cost the full limit. A wrong answer or a runtime error goes
  on to the next test.
- **What the report holds:**
  - `TestOutcome.actual` is the program's stdout as a JSON string, line ends shown as LF;
  - `stdout` holds the same text, kept for compatibility;
  - the new optional **`stderr`** field on `TestOutcome` and `Failure` holds stderr
    (optional fields only; nothing renamed);
  - `time_us` is the run's CPU (user + kernel, start-up included).
- **Stored text is capped:** stdout, stderr, the failing input and the expected text are
  kept up to 64 KB (`STORED_TEXT_BYTES`), with "… (truncated)". The checker always
  compares the whole output. kb-pack sets `JudgeEnv::full_program_output` to keep it
  whole, for expected outputs and stress comparisons. A custom test whose expected output
  would be cut is refused.
- **Start-up per test:** measured with `program_windows::startup_cost_per_run`, the median
  CPU of 20 runs of a trivial program in the sandbox. Release build, this machine
  (calibration factor about 1.0):

  | Language | Start-up CPU (median) | Wall per run | Allowance (`startup_ms`) | Share |
  | --- | --- | --- | --- | --- |
  | Python | 31 ms | 66 ms | 1000 ms | 3% |
  | TypeScript | 94 ms | 98 ms | 500 ms | 19% |
  | PHP | 62 ms | 69 ms | 500 ms | 12% |
  | Rust | 16 ms | 34 ms | 100 ms | 16% |

  - **The rule:** keep a language's allowance per test if start-up uses at most 60% of
    it. Every language passes, so the existing allowances serve as the per-test
    allowance, and nothing new is configured.
  - **Hidden tests:** program problems keep 15-20 hidden tests, so a Submit stays a few
    seconds (measured in the report).
- **kb-runner is unchanged:** it already feeds stdin and caps output. The protections
  hold through program mode (`program_attacks_windows`).

### Package and storage
- **kb-pack validate, for program problems:**
  - no signature-line check;
  - a text checker is required;
  - `{"stdin", "stdout"}` tests with no NUL characters;
  - **every integer token within ±(2⁵³ − 1)**, because TypeScript reads numbers with
    `Number`;
  - `## Input format` / `## Output format` headings (ES: `## Formato de entrada` /
    `## Formato de salida`);
  - **each `### Example N` (or `### Ejemplo N`) equal to public test N:** its two code
    fences are the stdin and the stdout, compared up to the final newline, in both
    languages;
  - `profile = true` is refused.
- **kb-pack validate --run:** the usual checks (validator, reference speed, stress
  against brute.py, naive TLE and the naive.rs margin). Speed is judged by the **slowest
  test's** CPU, since each test is its own run.
- **Problem scripts:** `gen.py` returns the stdin text, and `validate.py` receives the
  text; the tool drivers wrap and unwrap `{"stdin": …}`. kb-pack `generate` writes
  program test files.
- **content.db schema 5:**
  - a `problems.mode` column (`'function' | 'program'`); the signature column holds the
    synthetic signature;
  - `ProblemSummary` and `ProblemView` carry `mode`; `ProblemView` also carries
    `checker`, which the UI's diff follows;
  - `ProblemFilter.mode` filters the library.

### Starters, editor and UI
- **Starters:** each language's program starter (`languages/<lang>/program.<ext>.j2`)
  reads all of stdin at once, with the fast idiom:
  - Python: `sys.stdin.buffer.read()`;
  - TypeScript: `readFileSync(0)`;
  - PHP: `stream_get_contents(STDIN)`;
  - Rust: `read_to_string` and a `BufWriter`.
  
  It writes the output once. Its comment says why, and that **debug prints go to
  stderr**, since stdout is the answer. The language servers and completions are
  unchanged.
- **The UI:**
  - the "Program · stdin/stdout" badge (icon and text) and a Function/Program library
    filter;
  - stdin/stdout samples, and a custom test that is one stdin text box;
  - in the failure view, the stdin, then the expected and actual output side by side,
    with the first differing token or line marked (dotted underline, bold and a
    screen-reader note, not colour alone; `textDiff.ts` mirrors the checkers);
  - stderr under its own heading;
  - an "Output" column, and a note that times include each run's start-up.

### Debugger, visualizer, profiler
- **Python: supported.** The debug harness's program branch:
  - pre-compiles the file (a syntax error is a load error), then runs it as `__main__`
    with monitoring on from its first line;
  - puts the test's stdin on fd 0 (kb-judge writes `stdin.txt` next to the harness), so
    `input()`, `sys.stdin.read()`, `sys.stdin.buffer` and `open(0)` all read it;
  - gives the capture a `.buffer` for `sys.stdout.buffer.write`;
  - treats `sys.exit()` / `sys.exit(0)` as a normal end and any other code as an error;
  - joins the program's threads;
  - shows the module frame's variables once, as locals.
  
  The visualizer follows.
- **TypeScript: not yet** (`debug::program_supported`). Module evaluation is
  asynchronous to the inspector, and stdin injection and `process.exit` would need new
  handshakes. The UI says "Program problems can be debugged in Python for now."
- **The profiler: not yet.** Process time includes start-up and parsing, so ADR 0024's
  fit and calibration do not carry over.

## Consequences
- **Program problems reuse every function-mode path** except the run itself and its
  storage caps. A future custom checker, interactive problem or multi-file program can
  start from `program.rs`.
- **Each test pays the runtime's start-up.** That is small against the allowances, but it
  limits how many hidden tests a program problem should have.
- **Stored reports of program problems** hold at most 64 KB of text per field.
- **Deferred:** TypeScript debugging for programs, and the profiler for programs.
