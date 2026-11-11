# 14. Language servers

Date: 2026-11-11 · Status: proposed

## Context
The editor needs code intelligence:
- completion;
- hover;
- signature help;
- diagnostics;
- go to definition within the file;
- format document.

That means real language servers, one per language:
- basedpyright for Python;
- typescript-language-server for TypeScript and JavaScript;
- phpactor for PHP;
- rust-analyzer for Rust.

Unlike the judge (ADR 0002, 0006, 0007), servers are long-lived processes that answer
within milliseconds. They cannot run through kb-runner, which runs one short job at a time
in a fresh sandbox. They therefore run **outside the sandbox**. This ADR records the rules
that keep that safe, and the design of the bridge between the editor and the servers.
It builds on ADR 0003 (CodeMirror), 0011 (commands) and 0012/0013 (UI, workspace).

## Decision

### Security rules
1. **Servers never execute user code.** Each server is configured so that analysis never
   runs what the user wrote (see the table below). A setting that would run user code is
   a bug, never an option.
2. **rust-analyzer never invokes cargo.**
   - The project is a `rust-project.json`, with no build scripts, no proc-macros and no
     `checkOnSave` (so no `cargo check`).
   - `cargo.sysrootQueryMetadata: "none"` is required. Even with a rust-project.json,
     rust-analyzer otherwise tries `cargo metadata` on the sysroot (we saw it in testing).
   - The bundled Rust toolchain has no cargo, and **cargo must never be added to it**.
   - rust-analyzer may run the bundled `rustc` for target queries (`--print cfg`,
     `--print sysroot`). That is our pinned binary, not user code.
3. **No installs or fetching at runtime.**
   - Servers come pinned and checksummed from the dev fetch, and later from the product
     download manager.
   - Settings that would fetch are off: TypeScript's automatic type acquisition (npm),
     auto-import from packages, Composer.
   - `workspace/executeCommand` is blocked.
4. **Clean environment.** The environment is emptied (`env_clear`), then only these are set:
   - `SystemRoot`, `windir` and `SystemDrive` on Windows. Without `SystemRoot`, Node's
     crypto fails ("entropy source strength too weak").
   - `PATH`, set to the server's own runtime folder only.
   - `TEMP`, `TMP`, `TMPDIR`, `HOME`, `USERPROFILE`, `APPDATA`, `LOCALAPPDATA` and
     `XDG_*`, all pointing into a per-server state folder under the app's data folder.

   Nothing else is inherited: no proxy or network variables, no `RUSTUP_*`/`CARGO_*`, no
   `NODE_OPTIONS`, no `PYTHON*`.
5. **One workspace: the problem's virtual project.** Each server sees exactly one folder,
   `<app local data>/lsp/<problem>/<language>/`. It is both the working directory and the
   only root. The folder holds `solution.<ext>` and that server's config.
6. **No network settings.** No server is given a URL, a registry or a proxy.

### What each server can read
The servers are not sandboxed, so **the operating system lets them read anything the
user can read**. The table lists what they open in practice, given their configuration
and the bridge policy. This is configuration, not enforcement (see Follow-ups).

| Server | Runs as | Reads | Settings that keep user code from running |
| --- | --- | --- | --- |
| basedpyright 1.40.2 | bundled Node 26 | its install folder (bundled typeshed stubs), the project | no interpreter configured (bundled typeshed only, never starts Python); `openFilesOnly`; no auto-search paths, no library code |
| typescript-language-server 6.0.3 + TypeScript 6.0.3 | bundled Node 26 | its install folder (lib `.d.ts`), the project (`tsconfig.json`: strict, `types: []`, the solution only) | automatic type acquisition off, no plugins, no package auto-imports, tsserver log off |
| rust-analyzer 1.99.1 | itself | the Rust sysroot and std's sources (rust-src 1.99.1 in `tools/rust-src`, ADR 0015), the project (`rust-project.json`, edition 2024) | no build scripts, no proc-macros, no `checkOnSave`, no cargo, `files.watcher: client` |
| phpactor 2026.10.07.0 | bundled PHP 8.5 (`-n`, mbstring only) | its phar, the project | analysis only; no extensions such as php-cs-fixer, phpstan, psalm or composer. **Not available on Windows** (below) |

### The bridge (`crates/kb-bridge`)
- **Transport.** Content-Length framed JSON-RPC over the server's stdio.
  - Headers are limited to 8 KB and messages to 64 MB.
  - Messages reach the UI over a **Tauri channel** per connection: no ports, no
    localhost listeners, and no CSP change.
- **Sessions.**
  - One process per language, started lazily on the first connect.
  - Every `lsp_connect` starts a **fresh session with a new id** and stops the language's
    previous one.
  - `lsp_send` and `lsp_disconnect` must carry that id.
  - Why: a server is initialized once per process, so a second client (a remounted
    editor, or React's dev double effects) can never share one. Rebinding was tried and
    sent a second `initialize` to an already-initialized server. A stale client's late
    messages are refused (`language_server_not_connected`) instead of landing in the new
    session.
- **Lifecycle.**
  - **Idle stop** after 5 minutes without editor messages: `shutdown` and `exit`, then
    kill after 2 s.
  - **Crash → restart** with backoff (1 s doubling to 30 s), in the same session. The UI
    starts a new client for the new process; until that client's `initialize`, the bridge
    refuses messages (`-32002`).
  - After 5 crashes in 10 minutes the server is `unavailable`, with its last stderr line.
  - A client `exit` is a stop, not a crash.
  - Statuses: `starting`, `ready`, `restarting {attempt, delay_ms}`,
    `unavailable {reason}`, `stopped`.
  - The editor works the same without a server.
- **Policy** (`policy.rs`), applied in Rust whatever the UI sends:
  - **Initialize.** `initialize` is rewritten: root, workspace folders,
    `initializationOptions` (the bridge's settings), `processId` and `clientInfo`.
  - **Client methods.** Only these are allowed: the lifecycle, `textDocument/*` with a URI
    inside the project, `$/cancelRequest`, `$/setTrace`, and
    `workspace/didChangeConfiguration` with the bridge's settings forced. Everything else
    gets `-32601`.
  - **Server requests** are answered by the bridge, never by the UI:
    - `workspace/configuration` returns the bridge's settings;
    - `workspaceFolders` returns the project folder;
    - capability registration, progress creation and `*/refresh` return null;
    - `window/showDocument` returns `{success: false}`;
    - `workspace/applyEdit` outside the project returns `{applied: false}`;
    - `telemetry/event` is dropped.
  - **URI spelling.** Server URIs inside the project are rewritten to the exact spelling
    the bridge gave out. basedpyright and tsserver send `file:///c%3A/…` for
    `file:///C:/…`, and the editor matches files by exact URI. Without this, their
    diagnostics were silently dropped.
- **Pull diagnostics.**
  - The editor (`@codemirror/lsp-client` 6.3) takes pushed diagnostics only, and
    rust-analyzer serves its native diagnostics by pull only.
  - For such servers the bridge declares pull, refresh and `serverStatusNotification`
    support. It pulls `textDocument/diagnostic` 300 ms after an open or change, and again
    on `workspace/diagnostic/refresh` or when rust-analyzer reports `quiescent` (its
    project has loaded). The result goes to the UI as `publishDiagnostics`.
  - rust-analyzer's semantic diagnostics are "experimental" and need
    `diagnostics.experimental.enable`; without it, only syntax errors appear.
  - E0282 ("type annotations needed") is disabled, because it fires on correct code such
    as `let m: HashMap<i64, i64> = HashMap::new();`.
- **Containment.**
  - On Windows each server runs in a **Job Object** (`win32job`, safe API) with
    **kill-on-close**: stopping, crashing or killing the app ends the server's whole
    process tree, tsserver and rustc queries included (checked by killing the dev app).
  - Servers run with `CREATE_NO_WINDOW`.
  - On Linux: `kill_on_drop`, plus the server in its own process group.

### Editor
- `@codemirror/lsp-client` 6.3.0 (MIT) talks to the bridge through a `Transport` over the
  typed commands.
  - It provides completion, hover, signature help, diagnostics, go to definition (F12)
    and format (Shift+Alt+F, plus a Format button).
- Server Markdown is untrusted. It passes through an allow-list DOM sanitizer
  (`sanitize.ts`: no scripts, handlers, links or media), and the CSP blocks inline
  scripts as well.
- The status line under the editor shows the server state (with the reason when
  unavailable), the errors/warnings count with icons, and the cursor position.

### TypeScript 6, not 7
`typescript@7` is the native Go port and has no `tsserver`. typescript-language-server
needs `typescript@6`, so both are pinned at 6.0.3. TypeScript 7's built-in LSP is a later
option.

### phpactor on Windows
phpactor requires the PHP `posix` extension, which Windows builds of PHP do not provide.
- Its Box requirement checker can be bypassed, but `bin/phpactor` checks again and exits.
- So PHP's server is **unavailable on Windows**: the status line gives the reason, and the
  phar is not fetched there.
- On Linux the pin is phpactor 2026.10.07.0 (SHA-256 `b28bf616…db16b`). Wiring it up
  belongs to the Linux fetch, which has no language servers yet.

## Consequences
- Servers can read whatever the user can read; nothing *enforces* the project-only rule
  beyond configuration and the URI policy. This is the main accepted risk.
- Development and CI need the language-server fetch: `fetch-toolchains.ps1` does it by
  default (`-NoLanguageServers` skips it). The Windows CI job runs the real-server tests.
- Every connect starts a fresh server. A remount costs one server start (about 1 s for
  Python and TypeScript, a few seconds for rust-analyzer), and stale clients can't
  confuse a server.
- rust-src (std's sources) was merged into the bundled Rust toolchain folder; ADR 0015
  moves it to `tools/rust-src`, which `rust-project.json`'s `sysroot_src` points to.

## Follow-ups
- Run servers inside a **long-lived sandbox profile** (read-only toolchains and tools,
  read-write project and state, no network), so that "what each server can read" is
  enforced rather than configured.
- **Memory and process caps** for the Job Object: `win32job`'s safe API has kill-on-close
  but no caps. Adding caps needs a little `unsafe` Win32 code or an upstream change.
- phpactor on Linux in `fetch-toolchains.sh`, and a PHP server option for Windows.
- TypeScript 7's native LSP once typescript-language-server's role is covered.
