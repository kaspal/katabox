# CLAUDE.md

Guidance for Claude (and any AI pair) working in this repo. Read it at the start of every session.

## What KataBox is

A free, open-source, offline desktop app for coding practice (LeetCode/HackerRank style) on Windows and Linux. Users solve problems in Python, JavaScript/TypeScript, PHP and Rust; code runs locally in an OS sandbox and is judged against public and hidden tests. The full plan lives in the "KataBox — Desktop Coding Practice Platform: Build Plan" doc; decisions are summarized in `docs/adr/`.

## Layout

| Path | What it is |
| --- | --- |
| `apps/desktop/` | Tauri 2 app. `src/` = React 19 + TypeScript UI (ADR 0012: `app/` shell, router, palette; `pages/`; `components/ui/` shadcn; `lib/api.ts` is the only door to the generated `bindings.ts`; `i18n/locales/`; `styles/tokens.css`; `smoke/` is a temporary dev-only screen), `src-tauri/` = thin Rust shell (commands and wiring only; ADR 0011) |
| `crates/kb-app` | App service behind the Tauri commands: state, sandbox root, Run/Submit, typed `AppError` (ADR 0011) |
| `crates/kb-core` | Domain types (Language, Difficulty, Verdict, Limits, TestResult, the judge Report). No I/O |
| `crates/kb-judge` | Checkers, harness/stub codegen (minijinja), and the judge: Python, TypeScript, PHP, Rust (ADR 0008, 0009) |
| `crates/kb-runner` | Sandboxed execution. **Security-critical** |
| `crates/kb-store` | SQLite via sqlx (runtime queries only): `content.db` (read-only, FTS5) + `user.db` (WAL, migrations in `migrations/`; ADR 0010) |
| `crates/kb-toolchain` | Download/verify/locate runtimes |
| `crates/kb-bridge` | LSP + DAP stdio relay |
| `crates/kb-pack` | `kb-pack validate [--run]` and `kb-pack build` (content.db) CLI for problem packages |
| `content/problems/` | Problem packages (CC BY-SA 4.0) |
| `languages/` | Per-language runtime definitions and harness templates |

## Commands

```sh
scripts/fetch-toolchains.sh # Linux: pinned Python + Node for the sandbox and judge tests (.ps1 on Windows adds PHP + Rust)
cargo test                       # library crates (default-members); Linux needs libseccomp-dev
cargo deny check licenses        # license check (hakoniwa is an approved LGPL exception)
kb-sandbox probe                 # Linux: what this kernel supports (cargo run -p kb-runner --bin kb-sandbox -- probe)
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cargo run -p kb-pack -- validate # validate every problem (--run: judge every reference solution; Windows)
cargo run -p kb-pack -- build    # validate, then compile content/problems into target/content.db (--out FILE)
cargo run -p kb-judge -- run two-sum --lang <python|typescript|php|rust> <file>  # judge a file (Windows; --public, --json)
pnpm install && pnpm build       # UI build (tsc + vite)
pnpm test                        # UI tests (Vitest + Testing Library; includes token contrast and en/es key parity)
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
- Every user-facing string goes through i18n (English and Spanish): add the key to both `apps/desktop/src/i18n/locales/{en,es}.json` (keys are typed; a parity test checks both). Show errors and judge messages by their stable codes (`errors.<code>`, `judge.verdict.<v>`, `judge.message.<code>`). Colors for verdicts, difficulty and status always come with an icon or label, and design tokens must keep the WCAG AA contrast test passing (ADR 0012).
- New data in `user.db` only via a numbered migration: a new `crates/kb-store/migrations/NNNN_*.sql` added to `MIGRATIONS`. Never edit a released migration (ADR 0010).
- Problems must be original. Never copy statements or tests from LeetCode, HackerRank or similar sites.
- Solutions use each language's standard library only.
- Record significant decisions as a new ADR in `docs/adr/`.

## Problem package format

See `docs/problem-authoring.md` and `content/problems/two-sum/` for the reference example: `problem.toml` (optional `title_es`), `statement.md`, `statement.es.md`, `hints.md` (tiers separated by `---`), `editorial.md`, `tests/public/*.json`, `tests/hidden/*.json` (`{"input": {...}, "output": ...}`), `solutions/ref.{py,ts,php,rs}`, optional `hints.es.md`, `editorial.es.md`, `gen.py`, `validate.py`, `solutions/brute.py`.
