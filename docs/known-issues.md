# Known issues

Known issues and limits in KataBox 1.0.0-beta.1. Each item says what happens and, where
there is one, what to do about it.

[Español](known-issues.es.md)

## Installing and updating

### Windows SmartScreen warns about the installer
**What happens:** the installers are not code-signed yet. The first time you run one,
Windows may show "Windows protected your PC".
**Workaround:** click **More info**, then **Run anyway**. Check the file against the
SHA-256 checksums on the release page if you want to be sure it is the original.

### No automatic updates
**What happens:** KataBox does not check for or download new versions.
**Workaround:** download the new installer from the releases page and install it over the
current version. Your progress, drafts and settings are kept.

### Windows only
**What happens:** there is no Linux build in this release. Linux support is planned.
**Workaround:** none for now.

### First use of a language needs a download
**What happens:** no runtime is bundled. The first time you use a language, KataBox
downloads its runtime and language server. Approximate sizes on Windows (download / on
disk):

| Language | Download | On disk |
| --- | --- | --- |
| Python | 98 MB | 297 MB |
| JavaScript/TypeScript | 46 MB | 140 MB |
| PHP | 87 MB | 103 MB |
| Rust | 244 MB | 959 MB |

Python and JavaScript/TypeScript share Node.js, so installing both costs less than the
sum. SQL problems use the Python runtime, so they need Python installed.
**Workaround:** install languages while you are online; after that, KataBox works
offline. Behind a firewall or proxy allow-list, allow `github.com` and its release-asset
hosts, `nodejs.org`, `windows.php.net`, `downloads.php.net`, `static.rust-lang.org` and
`registry.npmjs.org`. KataBox uses the system proxy; PAC/WPAD scripts are not supported.

## Editor

### No language server for PHP on Windows
**What happens:** PHP code gets syntax highlighting but no diagnostics or completion.
The PHP language server (phpactor) needs a PHP extension that Windows builds of PHP do
not have. The editor's status line says so.
**Workaround:** none. Run and Submit work normally.

### No language server for SQL
**What happens:** SQL has no diagnostics from a server. The editor still completes the
problem's table and column names and SQL keywords.
**Workaround:** use Run to check the query; a syntax error or unknown name is reported as
a compile error.

### Some Node.js modules are underlined in TypeScript program problems
**What happens:** in program problems, the editor's type checker knows only a small part
of Node.js: `fs` / `node:fs` (`readFileSync`, `readSync`, `writeSync`), `process`,
`Buffer` and `require`. Importing another built-in module, such as `node:readline`, may be
underlined as an error even though the program runs.
**Workaround:** read input with `readFileSync(0)` as the starter code does, or ignore the
warning; judging is not affected.

### Custom tests are not saved
**What happens:** the custom tests you add on a problem are cleared when you open another
problem or close the app.
**Workaround:** keep inputs you want to reuse in the problem's notes, which are saved.

## Debugger

### No debugger for PHP and Rust
**What happens:** the replay debugger and the visualizer work in Python and
JavaScript/TypeScript only. PHP and Rust have no debugger.
**Workaround:** debug a PHP or Rust solution with prints, or try the failing case in
Python or TypeScript.

### Limits of the TypeScript debugger on program problems
**What happens:** a TypeScript program is replayed as one function, so some programs
cannot be debugged:
- imports of anything other than Node's built-in modules, and top-level `await`: the
  debugger refuses them, with a reason;
- programs that read standard input through `process.stdin` events: these can't be
  replayed.
**Workaround:** read all input at once with `readFileSync(0)` (as the starter does), and
keep the code synchronous. Judging is not affected; only debugging is.

### Programs that depend on time or randomness can stop replaying
**What happens:** each debugger step replays your code from the start. If the code
depends on the clock, `crypto` or timers, a replay can take a different path and the
debugger stops with a divergence message. `Math.random` (TypeScript) and `random`
(Python) are seeded, so they are fine.
**Workaround:** remove the time-dependent code while debugging.

### Long runs are cut short in the TypeScript debugger
**What happens:** a TypeScript trace records at most 15,000 steps (Python: 200,000). On a
large input the trace is truncated, and each step near the end can take a few seconds.
**Workaround:** debug on a small input: a public test, a custom test, or the result of
"Find a smaller failing input".

## Judging and profiling

### The complexity profiler is an estimate
**What happens:** the profiler always says "Estimated". It times your code at a few input
sizes on your machine and fits a curve, so a noisy run can produce the wrong class, for
example O(n log n) for an O(n) solution. 95 of the 209 problems have a profile.
**Workaround:** read it as a hint, not a verdict. Compare your line with the reference's
line on the same chart.

### The profiler can't tell fast program solutions apart
**What happens:** in program (stdin/stdout) problems, each run includes the language's
start-up time, which is often longer than the work itself at the profile's input sizes.
The profiler then says the result is unreliable, or that the solution is "too fast to tell
apart here". It shows slow solutions (for example O(n²)) clearly, but cannot separate
O(n) from O(n log n).
**Workaround:** none; a "too fast" result means the solution is fast enough for the
problem.

### "Find a smaller failing input" is for function problems only
**What happens:** the shrinker is not available on program or SQL problems; the app says
why. It works only on failures whose input you can already see (public tests, custom
tests and the first failing hidden test), stops after 60 runs or 30 seconds, and uses the
problem's input checks only when Python is installed.
**Workaround:** on a program problem, cut the failing input down by hand and add it as a
custom test.

### SQL problems have fewer tools
**What happens:** SQL problems have no custom tests, no debugger, no complexity profiler,
no shrinker and no language server. Mock interviews never pick SQL problems.
**Workaround:** use Run on the public tests and compare the result tables.

## Learn

### Input size limits in Learn topics
**What happens:** each topic limits the input you can enter so the animation stays
readable. Grid pathfinding topics accept grids up to 12 × 16, the bitmask travelling
salesman topic up to 5 cities, and an animation is capped at 2,000 steps. A larger input
shows an error instead of running.
**Workaround:** use a smaller input or one of the presets.

### Learn samples are in Python and TypeScript only
**What happens:** the sample code next to each animation is in Python and TypeScript; there
are no PHP or Rust samples.
**Workaround:** none.
