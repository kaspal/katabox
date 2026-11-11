# 11. App state layout and command conventions

Date: 2026-11-11 · Status: proposed

## Context
The desktop app has to open content.db and user.db, pick a sandbox root with the
toolchains, judge from content.db and expose all of it to the React UI, while
"business logic goes in `crates/`" (CLAUDE.md) and the UI never touches the file system
(ADR 0001). This records where state lives and the rules every command follows.

## Decision

### Layers
- **`crates/kb-app`** is the app service. It owns the stores, the `JudgeEnv` and the
  sandbox root, and it implements every command as an async method returning
  `Result<T, AppError>`. That covers Run/Submit orchestration, custom tests, redaction
  and toolchain status. It is a normal library crate, so `cargo test` covers it with no
  window.
- **`apps/desktop/src-tauri`** is wiring only:
  - it resolves the folders;
  - it opens `kb_app::App` once at startup and keeps it as managed state;
  - each `#[tauri::command]` is a short forward to kb-app.
- kb-store does not depend on kb-judge (ADR 0010). kb-judge builds a `Problem` from
  kb-store's `JudgeData` with `Problem::from_parts`, from plain parts, so neither crate
  knows the other.

### Where things live

| What | Where |
| --- | --- |
| content.db | Bundled resource (`resources/content.db` → `content.db`). The desktop `build.rs` builds it from `content/problems` with `kb_pack::build_pack`, so `tauri dev`, `tauri build` and `cargo check` all have it. It is gitignored. |
| user.db | Tauri `app_data_dir()` (`%APPDATA%\dev.katabox.app\user.db`; `~/.local/share/dev.katabox.app` on Linux) |
| Sandbox root | The first match wins: (1) env `KATABOX_SANDBOX_ROOT`; (2) the user.db setting `sandbox_root`; (3) debug builds only, `<repo>/target/katabox` if it carries the marker; (4) `app_local_data_dir()/sandbox`, created with `.katabox-sandbox-root`. A root named by env or setting must already carry the marker: the app never marks a folder the user named. |
| Toolchains | `<root>/toolchains/{python-3.14,node-26,php-8.5,rust-1.99}` (`JudgeEnv::dev`). kb-runner only grants folders inside the root, so the root and the toolchains are chosen together. Until the download manager exists, toolchains come from the fetch scripts. |

If the app cannot open (for example, content.db is missing), the error is kept in the
managed state, every command returns it, and the UI shows it. The app never panics at
startup.

### Command conventions
- **Async; `Result<T, AppError>`.** `AppError { code, message }`:
  - `code` is a snake_case `AppErrorCode` and is never renamed (the UI translates by
    code);
  - `message` is English, for logs and as a fallback, the same rule as judge `Message`s.
  - Store and judge errors map to codes in one place (`kb-app/src/error.rs`).
- **Judging** runs in `tokio::task::spawn_blocking`.
  - One Run or Submit at a time per app. A second call gets `judge_busy` instead of
    queueing; kb-runner also serializes sandbox runs.
- **Progress** goes over a Tauri `Channel<JudgeEvent>`:
  - `compiling` (compiled languages);
  - `running {total}`;
  - one `checked {done, total, …}` per test;
  - `finished {verdict}`.

  kb-runner returns output only when the batch ends, so `checked` events arrive
  together after the run. Live per-test progress would need a streaming change in
  kb-runner (not done; it needs the maintainer's sign-off).
- **Run** = public tests + custom inputs.
  - A custom input must have exactly the signature's params.
  - Its expected output comes from first running the reference solution in the same
    language on the custom inputs. If the reference can't handle an input,
    `invalid_custom_input` names that input.
  - Runs are snapshotted and never change the problem's status.
- **Submit** = every test.
  - The full Report is stored in user.db.
  - The response redacts what hidden tests returned and printed (`Report::redact_hidden`).
    Only `first_failure` reveals a hidden test, as the plan says. `get_submission`
    redacts the same way.

### TypeScript bindings
- tauri-specta generates `apps/desktop/src/bindings.ts`. The file is checked in, and
  the UI calls only `commands.*` from it, never `invoke` with strings.
- To regenerate: `cargo run -p katabox-desktop --bin export-bindings`. Debug builds of
  the app also regenerate it at startup.
- CI (Linux desktop job) regenerates it and fails on any diff; `pnpm build` typechecks it.
- Command errors come back as `{status: "error", error: AppError}` (tauri-specta's Result
  mode).
- 64-bit integers (times in µs or ms, ids) are exported as `number`. All realistic values
  are far below 2^53.
- Versions are pinned exactly, because tauri-specta 2 is still a release candidate:
  `tauri-specta =2.0.0-rc.21`, `specta =2.0.0-rc.22`, `specta-typescript =0.0.9`.
  - The newest RCs (specta rc.28 / specta-typescript 0.0.15) refuse to export 64-bit
    integers and `serde_json::Value`, which every Report contains. Supporting them
    needs a type remapper. Revisit when specta 2 is stable.
- Types that cross IPC derive `specta::Type` behind an optional `specta` feature in
  kb-core, kb-store and kb-app, so the library crates don't need specta otherwise.

### Linux
The app binary doubles as the sandbox helper: `main.rs` dispatches kb-runner's hidden
`__*` subcommands, and `set_helper_path(current_exe)` points the runner at it (ADR 0007).
This hasn't been run by hand yet; it is compile-checked in CI.

### Temporary smoke screen
`apps/desktop/src/smoke/` is a deliberately plain screen that proves the commands work.
It has a problem list, the statement as plain text, a language picker, a textarea, Run,
Submit and the verdict with per-test results. It is English only and is replaced by the
real UI; nothing should build on it.

## Consequences
- New commands follow the same pattern: a method on `kb_app::App`, a short forward in
  `src-tauri/src/lib.rs`, an entry in `specta_builder()`, then regenerate the bindings.
- Tests for command behaviour live in kb-app and run without WebView2 or WebKitGTK.
  On Windows, Tauri test binaries cannot start without the app manifest
  (STATUS_ENTRYPOINT_NOT_FOUND), which is why bindings come from a binary, not a test.
- The judge's busy rule and phase-level progress are visible to users. Both can be
  revisited without changing the command shapes.
