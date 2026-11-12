# 22. A replay debugger inside the unchanged sandbox

Date: 2026-11-12 · Status: proposed

## Context
The build plan promises a step-through debugger with "Debug this case". ADR 0002 assumed
socket debug adapters (debugpy, js-debug, Xdebug, CodeLLDB). They cannot work as they
are:
- **User code always runs in the sandbox** (ADRs 0002, 0006), and the sandbox blocks
  all networking, loopback included. That stays: no loopback exemption, no network
  capability, no user code outside the sandbox.
- **kb-runner is batch-only.** `run(&SandboxSpec) -> Outcome` writes stdin once and
  closes it; stdout and stderr come back after the whole tree exits. It has no handle,
  no streaming and no callback, on Windows or Linux. A debuggee that pauses and waits
  for commands needs a new kb-runner API.

## Decision
**Replay debugging, with no kb-runner change** (the maintainer's choice). Every debugger
action is one ordinary sandboxed run of a debug harness that re-executes the chosen test
deterministically up to a target step. The program is never paused in a live process.

### The harness (`languages/python/debug.py.j2`)
- The judge's `harness.py.j2` is untouched. The debug harness uses the same nonce-tagged
  records and stdin request (ADR 0008), with new record kinds: `trace`, `outcome`,
  `stop`, `diverged`, `load_error`.
- **Tracing:** `sys.monitoring` (PEP 669, Python ≥ 3.12; the pin is 3.14) with its own
  tool id.
  - LINE, PY_START, PY_RESUME, PY_RETURN, PY_YIELD, PY_UNWIND and PY_THROW events count
    only for code whose file is `solution.py`.
  - Other code gets `DISABLE`, so harness and library frames are invisible and cost
    nothing after their first event. (Unwind and throw events cannot be disabled; they
    are ignored.)
  - **A step is a LINE event in the user's file.** The trace keeps `(line, depth)` per
    step; depth is the number of user frames.
- **`trace` mode** runs the test once and returns the trace, the prints (each chunk
  tagged with its step), and the outcome (the returned value, or the exception with its
  line and step).
- **`stop` mode** replays to step N and reports:
  - the call stack (user frames only);
  - the chosen frame's locals and globals, skipping modules, functions, classes and
    dunders;
  - the expansions the UI asked for;
  - the watch and console evaluations.
  It then exits with `os._exit`, so nothing after step N runs.
- **Values:** `repr` cut to 200 characters, the type name and a child count. Children
  are listed for lists, tuples, dicts, sets and objects with `__dict__`, 100 at a time.
  Expansion is lazy: a path is a scope, a name, then keys or indices.

### Trace encoding and output budget
A step must cost about one byte, or the trace would hit the output cap.
- **A code table** of the distinct `(line, depth)` pairs. Each step is one token:
  - one character from a 91-character alphabet (ASCII 33-125 without `"` and `\`, so
    JSON needs no escapes) for codes 0-90;
  - `~` and two characters for codes 91-8371.
- **Explicit budget:** `TRACE_BUDGET_BYTES` (1 MiB) for the tokens, plus
  `RECORD_ALLOWANCE` (512 KiB) for the table, outcome and record framing, plus twice
  the per-test print cap.
  - The debug run's `SandboxSpec.max_output_bytes` is exactly that sum, and the harness
    is told the same numbers.
- **Graceful truncation:** recording stops, and the run still finishes, at whichever
  comes first:
  - `MAX_STEPS` (200 000);
  - the token budget;
  - the code-table limit.
  The trace says which (`steps`, `bytes` or `codes`), and the recorded steps stay
  navigable. The UI says "Stopped recording after N steps".
- **Output limit:** going over the cap is still Output Limit Exceeded, but only a
  program that prints too much can get there.

### Determinism
- **Interpreter flags:** the debug run starts `python -P -s -X utf8` with
  `PYTHONHASHSEED=0` (via `SandboxSpec.env`), not `-I`, since `-I` implies `-E`, which
  would ignore the seed.
  - kb-runner already gives the child a clean environment holding only what we set, so
    this is as isolated as `-I`.
  - String hashing, and with it set and dict order, is the same on every replay.
- **Randomness:** the harness calls `random.seed(0)`.
- **Divergence check:** each stop run hashes the `(line, depth)` events up to its step
  (FNV-1a 64) and kb-judge compares the hash with the trace's prefix.
  - On a mismatch (time, `os.urandom`, `id()`-dependent logic) the session ends with
    `debug_diverged`: "This program behaves differently on each run…".
  - The same happens when the program ends before the target step.

### Evaluation semantics
Each stop is a fresh replay. A watch or console expression (even `xs.append(1)`) runs
at that stop only and never changes later steps. `eval` is tried first, then `exec` for
statements; prints are captured, and exceptions are shown as text. The console help says
so. This is safer than a live debugger, and an e2e test checks it.

### Limits (per run; the sandbox is unchanged)
- **Same sandbox as judging:** each trace or stop run is a normal `kb_runner::run`:
  - the same AppContainer and Job Object;
  - the same memory and process caps and work-folder cleanup;
  - the one-run-at-a-time lock;
  - kb-app's `judge_slot` (`judge_busy`).
- **CPU:** the language's Run cap × 5, at most 60 s, because tracing slows Python. The
  wall limit stays 3× the CPU cap.
- **Runaway programs:**
  - an infinite loop that steps hits the step cap (a truncated, usable trace);
  - heavy work inside a step hits the CPU cap;
  - sleeping hits the wall limit.
  Both limits show as "The test hit the time limit while debugging".
- **No paused process exists,** so "CPU while paused" and a session wall clock are moot.
  Ending a session drops the trace; there is nothing to kill.

### Measured cost
Python 3.14 in the Windows sandbox (`debug_windows.rs`, debug build of the tests):

| Run | Wall |
| --- | --- |
| Trace of a 200 000-step loop (step cap reached) | 1.05 s |
| Stop at step 1 000 | 0.33 s |
| Stop at step 150 000 | 0.48 s |
| Stop at step 199 999 | 0.67 s |
| `time.sleep` loop ended by the wall limit (CPU 6.5 s, wall 19.5 s) | 19.55 s |

A stop near step 150 000 costs about half a second, well under the 2 s threshold set in
review. No mitigation is needed now. If one is ever needed, the first option would be
periodic checkpoints that replay from the nearest snapshot; that needs pickling user
state, so it is not planned.

### kb-app and the UI
- **kb-app `debug.rs`:**
  - One session at a time (`debug_start` ends the previous one).
  - Inputs are checked with `kb_judge::check_input` against the signature, as for
    custom tests, so the UI can debug a public, custom or first-failing test.
  - Navigation is made of pure, unit-tested functions over the trace:
    - continue: the next step on a breakpoint line, else the end;
    - step over: the next step with depth ≤ current;
    - step into: the next step;
    - step out: the next step with depth < current;
    - step back: the previous step (free with replay);
    - restart.
  - **Commands:** `debug_languages`, `debug_start`, `debug_action`,
    `debug_set_breakpoints`, `debug_inspect`, `debug_evaluate`, `debug_end`.
  - **Errors:** `debug_unsupported`, `debug_diverged`.
- **UI:**
  - **Editor:** a breakpoint gutter (click, or F9 on the cursor's line); the paused line
    is highlighted with an arrow.
  - **Read-only editor during a session,** so the trace always matches the code (this
    replaced the planned `debug_stale` error, which became unnecessary).
  - **The Debug tab:** Variables (lazy tree), Watch, Call stack and the Debug console
    (prints up to the current step, plus evaluation).
  - **"Debug this case"** on public tests, custom tests and the first failing test.
  - **Shortcuts:** F5 continue/start, F10 over, F11 into, Shift+F11 out, Shift+F10 back,
    Ctrl+Shift+F5 restart, Shift+F5 stop. F5, F10 and F11 are always `preventDefault`ed
    in the workspace, so WebView2 never reloads or goes full screen.
  - Other languages see the buttons disabled, with "Debugging is available for Python
    for now".
- **Mock interviews:** debugging is allowed. It runs the user's own code and reveals
  nothing locked; `ensure_unlocked` is not involved.

### Other languages (evaluated, not built)
- **JavaScript/TypeScript: Node's in-process inspector works in the sandbox.**
  - A probe ran Node 26 through `kb_judge::tools::run_script`, the same AppContainer
    and Job Object as judging:
    - a worker thread opens `inspector.Session` + `connectToMainThread()` (no port, no
      socket);
    - it sets `Debugger.setBreakpointByUrl` on `solution.mjs`, then the main thread
      calls the solution;
    - the worker got `Debugger.paused` twice at the breakpoint and read `seen` and `n`
      with `Debugger.evaluateOnCallFrame`;
    - it stepped over, resumed, and the result was `[2,4]`;
    - the run used 218 ms CPU, the same as outside the sandbox.
  - **Pitfalls found:**
    - the main thread must wait for the worker asynchronously (`Atomics.wait` deadlocks
      it, because it then cannot answer inspector messages);
    - both threads need a live event-loop handle while replies are pending.
  - **Mapping onto replay:** a trace run would step line by line through the inspector
    (one pause round trip per step, much slower than `sys.monitoring`, so a smaller step
    cap). A stop run sets breakpoints and counts hits to reach step N, then uses
    `evaluateOnCallFrame` and `Runtime.getProperties` for variables.
  - **Determinism:** `Math.random` and `Date` would need seeding or stubbing in the
    harness.
  - **Not checked:** Linux was not tested (this was a Windows machine). There,
    `RLIMIT_NPROC` counts threads, and the worker adds some, so the process cap may need
    room.
  - **Possible phase 2:** no kb-runner change is needed.
- **PHP:**
  - Xdebug needs a socket (DBGp over TCP), so it is out.
  - phpdbg is a separate SAPI binary we do not ship.
  - A possible path is a replay harness on `declare(ticks=1)` +
    `register_tick_function` with `debug_backtrace`: slow and coarse, but standard and
    socket-free.
- **Rust:**
  - gdb, lldb and rr need ptrace (denied by seccomp on Linux) or debug APIs that are
    unavailable to an AppContainer child.
  - The realistic path is an instrumented build: the harness inserts line probes into
    the user's source before `rustc`, then replays like Python. That is a large job.

## Consequences
- Debugging adds no sandbox exception: no loopback, no network, no live process, no
  kb-runner change. The attack suite is unchanged (20/20).
- **Costs:**
  - each action is a full replay (about 0.3-0.7 s per stop);
  - non-deterministic programs are refused rather than debugged wrongly;
  - evaluations do not persist;
  - very long runs show only their first 200 000 steps.
- **Bonus:** step back.
- ADR 0002's list of socket debug adapters, and ADR 0003's "DAP types", are superseded
  by this ADR for now. The UI uses its own small types (`kb_core::debug`).

**Follow-up:** JavaScript/TypeScript debugging, built on the inspector probe above, is ADR 0023.
