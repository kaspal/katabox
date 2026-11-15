# 23. JavaScript/TypeScript debugging through Node's in-process inspector

Date: 2026-11-12 · Status: proposed

## Context
ADR 0022 added a replay debugger for Python: every debugger action is one ordinary
sandboxed run of a debug harness, with no kb-runner change, no sockets and no live
process. Its records are language-neutral:
- `trace`: a code table, tokens, prints by step, a truncation reason;
- `outcome`, `stop`, `diverged`, `load_error`.

ADR 0022's probe also showed that Node's in-process inspector (a worker thread with
`inspector.Session.connectToMainThread()`) works inside the Windows AppContainer, with
no port.

"JavaScript" in KataBox is the TypeScript language entry: the judge runs `.ts` files
with Node's type stripping, which also runs plain JavaScript. The UI calls it
"JavaScript / TypeScript". `Language::JavaScript` stays unsupported as a judge
language.

## Decision
**The same replay model, UI, commands and types as Python.** Only the harness and its
limits are new: `languages/typescript/debug.ts.j2` (`debug` line in `language.toml`).
kb-app turns the language on by itself, because `debug::languages()` follows
`kb_judge::debug::supported`.

### The harness: one file, two threads
- **The main thread:**
  - reads the request;
  - seeds `Math.random` (mulberry32, seed 0);
  - imports the solution;
  - starts the worker on the same file and **awaits** its "ready" message (never
    `Atomics.wait`, which deadlocks the inspector);
  - runs `debugger;` right before calling the solution.
  - **Console output:** captured and tagged with the current step, which the worker
    keeps in a `SharedArrayBuffer`.
- **The worker (the recorder):**
  - **Setup:** `connectToMainThread()`, then `Debugger.enable`.
  - **The start:** at the `debugger;` pause it blackboxes every script except the
    solution and steps into it. (V8 ignores `debugger;` in blackboxed code, so the
    blackboxing waits until then.) From then on, Node and harness frames are never
    seen.
  - **Steps vs pauses:** V8 pauses at statement positions, several per line (a `for`
    header pauses at `i++` and at `i < n`). A **step** is a pause in the solution
    whose (line, depth) differs from the previous pause; the other pauses are stepped
    through silently. A loop's body therefore gives the same steps as in Python, and a
    loop written on one line is one step.
  - **Depth:** the number of solution frames on the stack.
- **The same records as Python:**
  - the token encoding, the FNV-1a prefix hash (in BigInt; written into the JSON as raw
    digits, since it is a u64);
  - prints by step, the byte budget and the code-table limit.
- **Ending at a stop or a truncation:** the worker writes the record, then evaluates
  `process.exit(0)` in the paused frame, so nothing after the step runs and no wall
  limit is needed.
- **Stops:**
  - **The stack:** the solution frames.
  - **Variables:** "Locals" are the local, block, catch and closure scopes of the
    selected frame; "Globals" are the solution module's own scope, without functions.
    `globalThis` is never shown.
  - **Values:** described in the main thread with `util.inspect` (200 characters), a
    type name (the constructor or the primitive type) and a child count (array length,
    Map/Set size, own keys).
  - **Expansions:** paged at 100, with the same paths as Python.
  - **Watches and console:** `Debugger.evaluateOnCallFrame`, with console output
    collected around each evaluation. Each stop is a fresh replay, so an evaluation
    never changes later steps.

### Determinism
- `Math.random` is seeded.
- Clocks (`Date.now`, `performance.now`), `crypto` and timers are left alone. A program
  that depends on them is refused by the divergence check (prefix hash, or the program
  ending before the step), as in Python.
- V8's object key order and Map/Set order are insertion orders, so ordinary programs
  replay exactly.

### Limits
Measured in the Windows sandbox (debug build of the tests, Node 26):
- **A pause costs about 0.1 ms.** 30 000 pauses took 4.1 s for a trace, and 4.0 s for a
  stop at the end. Python's `sys.monitoring` costs about 0.005 ms per step.
- **The JS caps** therefore keep a trace, and a stop near its end, around 3 s:

  | Cap | Value |
  | --- | --- |
  | Steps | 15 000 |
  | Pauses | 20 000 (several pauses can make one step; this cap ends one-line loops). A pause truncation is reported as `steps`. |
  | CPU | the TypeScript Run cap × 8 (8.8 s at the default base), because stepping costs CPU in both threads |

  Python keeps 200 000 steps and × 5.
- **At the caps:**

  | Run | Wall |
  | --- | --- |
  | A loop truncated at 13 334 steps (20 000 pauses) | 2.5 s |
  | A stop at its last step | 2.7 s |
  | A one-line loop (pause cap, 2 steps) | 2.4 s |
  | Two-sum's TypeScript reference: trace / stop | 0.29 s / 0.21 s |

  Startup is about 0.2 s.
- **The byte budget, graceful truncation and output cap** are unchanged from ADR 0022.

### TypeScript
- Type stripping replaces annotations with spaces, so lines and columns stay where they
  are. A test traces a solution with a type-only import, an interface, a multi-line
  generic type alias, multi-line annotations and generic calls. The steps land exactly
  on the source lines of its statements.
- Breakpoints set by line in the editor therefore apply to TS and JS alike.

### Linux (not tested)
`RLIMIT_NPROC` counts threads on Linux, and the worker and the inspector add some. The
Linux sandbox may need room for them. This is to be checked with the Linux work; this
machine is Windows.

## Consequences
- JavaScript and TypeScript debugging with no kb-runner change, no port and no network
  (the attack suite is unchanged).
- **JS traces are shorter than Python's** (15 000 steps), and every action costs up to a
  few seconds near the caps. Typical solutions on public tests run well under the caps
  (two-sum: 20 steps).
- PHP and Rust stay unavailable (ADR 0022 lists their options).

## Program problems (2026-11, features-content M4)
A TypeScript program is replayed as one function.
- **Preparing the text:**
  - its types are stripped with `module.stripTypeScriptTypes`, which keeps positions;
  - imports of Node's built-in modules become `const … = __kb_require(…)` on the same
    lines; other imports are refused with a reason;
  - the text is compiled with `vm.compileFunction` under the solution's file URL, so the
    recorder sees `solution.ts` with its own line numbers.
- **Running it:**
  - `fs.readFileSync(0 | "/dev/stdin")` and `fs.readSync(0, …)` read the test's stdin;
  - `process.exit(code)` throws a sentinel: 0 is a normal end, another code an error;
  - the recorder ends the process with a saved `exit` (`__katabox_exit`).
- **Not replayable:**
  - top-level `await`, which can't run inside a synchronous function, is refused with a
    reason;
  - programs that read stdin through `process.stdin` events.

Every catalogue TypeScript program reference replays its first public test with the
judge's output (`debug_js_windows`). No sockets and no runner change.
