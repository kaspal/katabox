# CLAUDE.md

Guidance for Claude (and any AI pair) working in this repo. Read it at the start of every session.

## What KataBox is

A free, open-source, offline desktop app for coding practice (LeetCode/HackerRank style) on Windows and Linux. Users solve problems in Python, JavaScript/TypeScript, PHP and Rust; code runs locally in an OS sandbox and is judged against public and hidden tests. The full plan lives in the "KataBox — Desktop Coding Practice Platform: Build Plan" doc; decisions are summarized in `docs/adr/`.

## Layout

| Path | What it is |
| --- | --- |
| `apps/desktop/` | Tauri 2 app. `src/` = React 19 + TypeScript UI, `src-tauri/` = thin Rust shell (commands and wiring only) |
| `crates/kb-core` | Domain types (Language, Difficulty, Verdict, Limits, TestResult). No I/O |
| `crates/kb-judge` | Checkers (done) and harness codegen (next) |
| `crates/kb-runner` | Sandboxed execution. **Security-critical** |
| `crates/kb-store` | SQLite: `content.db` (read-only) + `user.db` |
| `crates/kb-toolchain` | Download/verify/locate runtimes |
| `crates/kb-bridge` | LSP + DAP stdio relay |
| `crates/kb-pack` | `kb-pack validate` CLI for problem packages |
| `content/problems/` | Problem packages (CC BY-SA 4.0) |
| `languages/` | Per-language runtime definitions and harness templates |

## Commands

```sh
cargo test                       # library crates (default-members)
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cargo run -p kb-pack -- validate # validate every problem
pnpm install && pnpm build       # UI build
pnpm dev                         # run the desktop app (needs WebView2 / WebKitGTK 4.1)
cargo check -p katabox-desktop   # desktop shell (needs WebKitGTK on Linux)
```

## Rules

- Small PRs, one concern each, with tests. CI must be green before merge; the maintainer reviews every PR.
- **Never change `crates/kb-runner` sandbox code, toolchain download verification, or the CSP in `tauri.conf.json` without the maintainer's explicit sign-off in the PR description.**
- Business logic goes in `crates/`, never in `src-tauri` or React components.
- The UI never touches the file system or spawns processes; it calls typed Tauri commands.
- No network access at runtime except toolchain, pack and update downloads.
- No telemetry, ever.
- Every user-facing string goes through i18n (English and Spanish).
- New data in `user.db` only via a numbered migration.
- Problems must be original. Never copy statements or tests from LeetCode, HackerRank or similar sites.
- Solutions use each language's standard library only.
- Record significant decisions as a new ADR in `docs/adr/`.

## Problem package format

See `content/problems/two-sum/` for the reference example: `problem.toml`, `statement.md`, `statement.es.md`, `hints.md` (tiers separated by `---`), `editorial.md`, `tests/public/*.json`, `tests/hidden/*.json` (`{"input": {...}, "output": ...}`), `solutions/ref.{py,ts,php,rs}`, optional `gen.py`, `validate.py`, `solutions/brute.py`.
