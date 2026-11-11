# CLAUDE.md

Guidance for Claude (and any AI pair) working in this repo. Read it at the start of every session.

## What KataBox is

A free, open-source, offline desktop app for coding practice (LeetCode/HackerRank style) on Windows and Linux. Users solve problems in Python, JavaScript/TypeScript, PHP and Rust; code runs locally in an OS sandbox and is judged against public and hidden tests. The full plan lives in the "KataBox — Desktop Coding Practice Platform: Build Plan" doc; decisions are summarized in `docs/adr/`.

## Layout

| Path | What it is |
| --- | --- |
| `apps/desktop/` | Tauri 2 app. `src/` = React 19 + TypeScript UI (ADR 0012: `app/` shell, router, palette; `pages/`; `workspace/` editor, problem and results panes (ADR 0013); `components/ui/` shadcn; `lib/api.ts` is the only door to the generated `bindings.ts`; `i18n/locales/`; `styles/tokens.css`; `smoke/` is a temporary dev-only screen), `src-tauri/` = thin Rust shell (commands and wiring only; ADR 0011) |
| `crates/kb-app` | App service behind the Tauri commands: state, sandbox root, Run/Submit, typed `AppError` (ADR 0011) |
| `crates/kb-core` | Domain types (Language, Difficulty, Verdict, Limits, TestResult, the judge Report). No I/O |
| `crates/kb-judge` | Checkers, harness/stub codegen (minijinja), and the judge: Python, TypeScript, PHP, Rust (ADR 0008, 0009); per-language time limits and the machine calibration benchmark (ADR 0017) |
| `crates/kb-runner` | Sandboxed execution. **Security-critical** |
| `crates/kb-store` | SQLite via sqlx (runtime queries only): `content.db` (read-only, FTS5) + `user.db` (WAL, migrations in `migrations/`; ADR 0010) |
| `crates/kb-toolchain` | Installs languages (runtimes + language servers) into the sandbox root: compiled-in `manifest.toml` (the only place pins live), safe downloader and extractor, receipts; `kb-toolchain` CLI (ADR 0015). **Download verification lives here** |
| `crates/kb-bridge` | Language servers over stdio, outside the sandbox: sessions, message policy, virtual project per problem (ADR 0014). DAP later |
| `crates/kb-pack` | `kb-pack validate [--run]`, `generate` and `build` (content.db) CLI for problem packages; `--run` runs every problem script in the sandbox (ADR 0016) and judges speed per language (ADR 0017) |
| `content/problems/` | Problem packages (CC BY-SA 4.0) |
| `languages/` | Per-language runtime definitions (`language.toml`, with the `[time]` factor and startup allowance of ADR 0017) and harness templates |

## Commands

```sh
scripts/fetch-toolchains.sh # Linux: Python + JS/TS into target/katabox via `kb-toolchain install` (.ps1 on Windows: every language; -NoLanguageServers for runtimes only)
cargo run -p kb-toolchain -- list            # what is installed in the dev root (install/uninstall LANG; --root DIR)
KB_REAL_PINS=1 cargo test -p kb-toolchain --release --test real_pins -- --nocapture  # download and check every pin (network; required before changing manifest.toml)
cargo test                       # library crates (default-members); Linux needs libseccomp-dev
cargo deny check licenses        # license check (hakoniwa is an approved LGPL exception)
kb-sandbox probe                 # Linux: what this kernel supports (cargo run -p kb-runner --bin kb-sandbox -- probe)
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cargo run -p kb-pack -- validate # validate every problem (--run: machine factor, then sandbox checks: validator, reference speed per language, 500-seed stress vs brute.py, naive.py and naive.rs TLE; Windows)
cargo run -p kb-pack -- generate content/problems/<id> --size max --seeds 1,2  # write hidden tests from gen.py (sandbox; committed)
cargo run -p kb-pack -- build    # validate, then compile content/problems into target/content.db (--out FILE)
cargo run -p kb-judge -- run two-sum --lang <python|typescript|php|rust> <file>  # judge a file (Windows; --public, --json)
cargo test -p kb-judge --test calibration_windows -- --nocapture  # calibration benchmark (prints this machine's rounds; re-measure the reference times when it or a runtime pin changes)
cargo test -p kb-bridge          # language servers: fake-server lifecycle; real basedpyright, typescript-language-server, rust-analyzer on Windows (needs the .ps1 fetch)
pnpm install && pnpm build       # UI build (tsc + vite)
pnpm test                        # UI tests (Vitest + Testing Library; includes token contrast and en/es key parity)
pnpm licenses:check              # npm license allow-list (scripts/check-npm-licenses.mjs; mirrors deny.toml)
pnpm dev                         # run the desktop app (needs WebView2 / WebKitGTK 4.1); debug builds use <identifier>-dev data folders ("DEV DATA" badge) and target/katabox as sandbox root, or KATABOX_SANDBOX_ROOT
cargo check -p katabox-desktop   # desktop shell (needs WebKitGTK on Linux); build.rs builds resources/content.db
cargo run -p katabox-desktop --bin export-bindings  # regenerate apps/desktop/src/bindings.ts after changing a command or IPC type
```

## Rules

- Small PRs, one concern each, with tests. CI must be green before merge; the maintainer reviews every PR.
- **Never change `crates/kb-runner` sandbox code, toolchain download verification, or the CSP in `tauri.conf.json` without the maintainer's explicit sign-off in the PR description.**
- Business logic goes in `crates/`, never in `src-tauri` or React components.
- The UI never touches the file system or spawns processes; it calls typed Tauri commands through the generated `bindings.ts` (never `invoke` with strings). Commands return `Result<T, AppError>` with a stable error code (ADR 0011).
- No network access at runtime except toolchain, pack and update downloads.
- No telemetry, ever.
- Pins (URL, size, SHA-256) change only in `crates/kb-toolchain/manifest.toml`, and only after the real-pins test downloaded and matched them. Installs never execute anything (no npm, no installers) and keep ADR 0015's rules: allow-listed HTTPS hosts and redirects, verify before extract, whole-folder promotion, receipt last.
- Language servers (ADR 0014) run outside the sandbox and **never execute user code**: no build scripts, proc-macros, `cargo check`, interpreters, type acquisition or commands. rust-analyzer never invokes cargo; never add cargo to the bundled Rust toolchain. Server settings and the message policy live in `crates/kb-bridge` (`spec.rs`, `policy.rs`); loosening either needs the maintainer's sign-off.
- Every user-facing string goes through i18n (English and Spanish): add the key to both `apps/desktop/src/i18n/locales/{en,es}.json` (keys are typed; a parity test checks both). Show errors and judge messages by their stable codes (`errors.<code>`, `judge.verdict.<v>`, `judge.message.<code>`). Colors for verdicts, difficulty and status always come with an icon or label, and design tokens must keep the WCAG AA contrast test passing (ADR 0012).
- Workspace state (drafts, notes, snapshots with verdicts, layout and editor settings `workspace.layout`, `editor.keymap`, `editor.font_size`, `editor.language`) and its save rules are in ADR 0013; keep CodeMirror and other heavy code inside the lazy workspace route.
- New data in `user.db` only via a numbered migration: a new `crates/kb-store/migrations/NNNN_*.sql` added to `MIGRATIONS`. Never edit a released migration (ADR 0010).
- Time limits (ADR 0017): `time_limit_ms` in problem.toml is the base (Rust) limit; languages scale it by their `[time]` table in `language.toml`, and the app by the machine's calibration (0.5-3.0). kb-pack judges at 1.0. Changing a factor, startup allowance or the benchmark means re-running `validate --run` on every problem (and re-measuring the benchmark's reference times).
- Problems must be original. Never copy statements or tests from LeetCode, HackerRank or similar sites.
- Solutions use each language's standard library only.
- Record significant decisions as a new ADR in `docs/adr/`.

## Problem package format

See `docs/problem-authoring.md` and `content/problems/budget-pair/` for a complete example: `problem.toml` (with `title_es`), `statement.md`, `statement.es.md`, `hints.md` + `hints.es.md` (3 tiers separated by `---`), `editorial.md` + `editorial.es.md`, `tests/public/*.json` (3-5), `tests/hidden/*.json` (15-40; `{"input": {...}, "output": ...}`), `solutions/ref.{py,ts,php,rs}`, `gen.py`, `validate.py`, `solutions/brute.py`, and `solutions/naive.py` plus `solutions/naive.rs` (always together) when the complexity matters. Problem scripts never run on the host: kb-pack and the app run them in the sandbox (ADR 0016). `validate --run` must pass before merge; generated tests are committed.
