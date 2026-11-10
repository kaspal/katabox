# 9. PHP and Rust in the judge

Date: 2026-11-10 · Status: proposed

## Context
ADR 0008 defined the judge protocol for Python and TypeScript. PHP and Rust need runtimes
that users never install themselves (no VC++ redistributable, no Visual Studio), and
Rust adds a compile step that must also run in the sandbox (ADR 0002, 0006). Windows
first.

## Decision

### PHP
- **Runtime:** PHP 8.5.12 NTS x64 (VS17 build) from windows.php.net, pinned with SHA-256
  in `scripts/fetch-toolchains.ps1` → `toolchains/php-8.5`.
- **DLLs:** `php.exe` and `php8.dll` import Windows system DLLs (KERNEL32, ADVAPI32,
  WS2_32, SHELL32, bcrypt, CRYPT32, DNSAPI, IPHLPAPI, ole32, the UCRT
  `api-ms-win-crt-*` set) plus **`VCRUNTIME140.dll`**. They import neither
  `vcruntime140_1.dll` nor `msvcp140.dll`; `php_mbstring.dll` adds nothing new. The
  third-party DLLs in the zip (ICU, OpenSSL, libsodium, …) belong to extensions in `ext/`
  that are never loaded.
- **VC++ runtime, app-local:** `vcruntime140.dll` 14.44.35211 (Microsoft-signed) is
  copied next to `php.exe` from the pinned python-build-standalone build, verified by
  SHA-256 and a minimum version of 14.44 (PHP is linked with the 14.44 toolset). The
  redistributable installer is not used: it cannot be unpacked without extra tools and its
  download link is not version-pinned.
- **Run line:** `php -n -d extension_dir=<php>/ext -d extension=mbstring
  -d memory_limit=<limit>M -d display_errors=stderr main.php`. `-n` ignores any
  `php.ini`. Available extensions: bcmath, calendar, Core, ctype, date, dom, filter,
  hash, iconv, json, lexbor, libxml, **mbstring**, mysqlnd, pcre, PDO, Phar, random,
  readline, Reflection, session, SimpleXML, SPL, standard, tokenizer, uri, xml,
  xmlreader, xmlwriter, Zend OPcache, zlib (`docs/problem-authoring.md`).
- **Harness:** `include 'solution.php'` works inside the AppContainer, so the planned
  `eval` fallback was not needed. A `ParseError` gives Compilation Error
  (`syntax_error`). A fatal compile error that PHP cannot throw (e.g. `Cannot redeclare`)
  is caught by a shutdown function before the tests start (`compile_error`). Records are
  written with `fwrite(STDOUT)`, which bypasses output buffering. Each test's output is
  captured with `ob_start` (bounded chunks, capped), and warnings are echoed into it by
  an error handler. Lists in the return value are re-indexed (`array_values`) to the
  depth of the declared return type, so `array_filter` results still encode as JSON
  arrays. Memory exhaustion is fatal and goes to stderr ("Allowed memory size"), which
  the judge reports as Memory Limit Exceeded.

### Rust
- **Toolchain:** Rust 1.99.1 for the `x86_64-pc-windows-gnu` host. The `rustc`,
  `rust-std` and `rust-mingw` component tarballs come from static.rust-lang.org, each
  pinned with its SHA-256 from the channel manifest, and are merged into
  `toolchains/rust-1.99`. `rust-mingw` brings the self-contained linker
  (`x86_64-w64-mingw32-gcc.exe`, `ld.exe`), so no Visual Studio. This replaces ADR 0002's
  "rustup minimal": pinned archives match how Python, Node and PHP are fetched, and there
  are no rustup proxies to sandbox. Compiled programs import only system DLLs
  (`msvcrt`, KERNEL32, ntdll, USERENV, WS2_32, bcryptprimitives).
- **Compile step, in the sandbox:** `rustc -O --edition 2024 --color never -A warnings
  -o solution.exe main.rs`. `main.rs` is the generated harness and `include!`s
  `solution.rs` at crate root, so solutions need no `pub` and can `use` freely. It runs
  through `kb_runner::run` with the toolchain read-only, a fresh work folder, and its own
  limits from `[compile_limits]` in `languages/rust/language.toml`. A non-zero exit
  gives Compilation Error with rustc's output (`compile_error`, or `function_not_found`
  when rustc cannot find the function). Hitting a limit gives `compile_limit`.
- **Measured** (Windows 11, 24 cores, in the sandbox, 3 runs each; CPU = user + kernel
  across all threads):

  | Compile | CPU | Wall | Peak memory | Processes |
  | --- | --- | --- | --- | --- |
  | two-sum reference + harness | 1.1–1.3 s | 0.86–0.90 s | 185 MB | 4 (rustc, gcc, ld, conhost) |
  | heavy synthetic (40 generic functions × 2 types, collections, closures) | 19.7–20.3 s | 9.0–9.4 s | 245 MB | 4 |

  CPU exceeds wall because rustc compiles on several threads and the job limit counts
  them all. **Limits chosen:** 30 s CPU, 60 s wall, 1024 MB, 16 processes. That is
  about 25× a typical solution's CPU and 1.5× the heavy case; the wall limit leaves room
  for 4-core CI runners; memory is 4–5× the peak.
- **Binary cache:** the key is the SHA-256 of the compiler path, size and mtime, the
  compile arguments, the generated harness and the source. Binaries live in
  `<sandbox root>/cache/rust/<key>.exe`. The run step grants only that folder,
  read-only, so the toolchain is not visible to solutions. New binaries are *copied*
  into the folder (so they inherit its sandbox grant; a moved file keeps its old ACL)
  under a temporary name, then renamed. The newest 64 are kept, by last use. Measured: a
  judge run with a compile takes 1.3 s; a cache hit takes 0.12 s.
- **Prints:** for each test, the harness points stdout and stderr at a fresh pipe with
  `SetStdHandle`; std looks up the handle on every write. A reader thread keeps the
  first 64 KB. After the test it flushes, restores the handles and closes the pipe.
  Records go through the original stdout handle. Verified reliable in the e2e tests
  (four full runs), so the batch-capture fallback was not needed. Panics are caught per
  test with the message and `solution.rs:line:col` (`exception`). Tests run on a thread
  with a 64 MB stack.

## Consequences
- Users install nothing: PHP carries its VC++ DLL, Rust carries its linker.
- The Rust toolchain is large (781 MB on disk after merging); the app will download it
  only when the user picks Rust.
- Re-running unchanged Rust code costs no compile; any edit costs about a second.
- A solution whose compile exceeds 30 s CPU gets Compilation Error (`compile_limit`).
- Linux is not covered yet: there are no PHP or Rust downloads in `fetch-toolchains.sh`,
  and the Rust harness only captures prints on Windows (elsewhere they are ignored as
  stray output).
- PHP warnings show in the per-test output. Rust warnings are discarded.
