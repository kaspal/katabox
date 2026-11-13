# 8. Judge: harness generation and protocol

Date: 2026-11-10 · Status: accepted

## Context
The Core judge phase needs to run a user's function against a problem's tests in the
sandbox (ADR 0002, 0006) and report a verdict per test. Starting with Python and
TypeScript on Windows. Constraints: one sandboxed run at a time (ADR 0006), the user's
prints must not corrupt results, and users must see the first failing input.

## Decision
- **Code generation from the typed signature.** `problem.toml`'s `[signature]` uses a
  small type grammar (`int | float | string | bool`, plus `[]`), parsed by kb-judge and
  validated by kb-pack. Each language has two minijinja templates in `languages/<id>/`:
  `stub.<ext>.j2` (starter code for the editor) and `harness.<ext>.j2`. Templates are
  embedded at build time (`include_str!`), so the app never reads them from disk.
  Naming follows each language: `twoSum` in TypeScript, `two_sum` in Python (which also
  accepts `twoSum`).
- **One sandboxed run per batch.** The work folder gets `main.<ext>` (harness),
  `solution.<ext>` (the user's code, unchanged) and, for TypeScript, a
  `package.json` with `"type": "module"`. The harness loads the solution as a module and
  calls the function once per test.
- **Protocol.** The judge sends one JSON request on stdin: a fresh per-run nonce, the
  per-test print cap, and the test inputs (expected outputs never enter the sandbox).
  The harness answers on stdout with records, each written as `\n<nonce> <json>\n`:
  `compile_error` (with a message code), `load_error`, `ready`, one `result` per test
  (status, return value or traceback with a message code, time in microseconds, captured
  prints), `done`. Lines without the nonce are ignored; the
  leading newline keeps a record intact even after a partial line written straight to
  the file descriptor.
- **The user's output is captured per test** (stdout and stderr together) by swapping
  `sys.stdout`/`sys.stderr` in Python and `process.stdout.write`/`process.stderr.write`
  in Node, capped at `Limits::max_output_kb`. Exceeding the cap is Output Limit Exceeded.
  Prints while the module loads are discarded.
- **TypeScript runs on Node's built-in type stripping** (Node 26 LTS, pinned in the
  fetch scripts). There is no type checking: Compilation Error means the source does
  not parse (`ERR_INVALID_TYPESCRIPT_SYNTAX`), uses syntax stripping cannot remove
  (`ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX`: `enum`, `namespace`, parameter properties), or
  does not export the function. A `SyntaxError` with stack frames (e.g. `JSON.parse` at
  module level) is a Runtime Error. Python `SyntaxError` at load is also Compilation Error.
- **Node runs with `--preserve-symlinks --preserve-symlinks-main`.** Without them Node
  `lstat`s every folder up to `C:\` while resolving modules, which the AppContainer
  denies. No kb-runner change was needed.
- **Time.** The problem's `time_limit_ms` is CPU time for the whole batch (kb-core
  `Limits`), plus a fixed 1 s `STARTUP_ALLOWANCE` for interpreter startup. The sandbox
  enforces it; the test that was running when the batch was stopped gets Time Limit
  Exceeded and later tests are not run. Per-test time is the wall time of the call,
  measured inside the harness and stored in **microseconds** (kb-core
  `TestResult::time_us`, `Report::time_us`): typical calls take well under a
  millisecond, so milliseconds would round almost everything to 0.
- **Verdicts.** Per test: return value checked with the problem's checker (AC/WA), an
  exception (RE), Python `MemoryError` (MLE), or too many prints (OLE). If the run ends
  before every test reported, the next test gets TLE (CPU or wall limit), OLE (sandbox
  output cap), MLE (job memory limit hit or an out-of-memory message), or RE (the
  process exited, e.g. `process.exit()`, `os._exit`, stack overflow). A harness that
  stops without reporting, or reports nonsense, is Internal Error, never blamed on the
  user. The overall verdict is the first non-accepted test (kb-core `overall_verdict`).
  Memory is the batch peak, capped at the limit (ADR 0006: the job counter includes
  refused allocations).
- **Every message has a stable code.** Checker, harness and judge messages are kb-core
  `Message { code, text }`, where `code` is a `MessageCode` serialized in snake_case
  (`output_mismatch`, `syntax_error`, `time_limit`, ...) and `text` is English (plus
  compiler output or a traceback where relevant). The UI translates by code; codes are
  never renamed, only added. The harnesses send codes for what only they can know
  (`syntax_error`, `unsupported_syntax`, `function_not_found`, `exception`,
  `return_not_serializable`); the judge assigns the rest from the sandbox outcome.

## Consequences
- One process start per submission rather than per test: fast for many small tests.
  An infinite loop in one test hides the results of the tests after it.
- The nonce is not a security boundary. User code runs in the harness process and could
  find it and forge records, but it only fools the user's own local judge. The sandbox
  remains the boundary.
- TypeScript type errors are not reported. A real type check (bundling `tsc`) can come
  later as a separate, optional step.
- The English text stays for logs, the CLI and as a fallback. Translating it is a UI
  concern: map each `MessageCode` to an i18n key and show compiler output and
  tracebacks untranslated.
- Return values must be JSON-serializable; non-finite floats and cyclic structures are
  Runtime Errors in Python, while Node turns `NaN`/`Infinity` into `null`.
- Verified on Windows only (`crates/kb-judge/tests/e2e_windows.rs`, 21 tests). Linux
  should work through the same kb-runner API but is untested, in particular Node under
  the seccomp policy and `RLIMIT_NPROC` counting Node's threads.
- The Rust harness's JSON reader lives in `languages/rust/json.rs` and reads test inputs
  straight into the argument types; the protocol above is unchanged (ADR 0026).
