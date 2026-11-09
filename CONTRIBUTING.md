# Contributing to KataBox

Thanks for helping! The easiest ways to contribute, from small to large:

1. **Report a problem with a problem.** Wrong test, unclear statement, weak hidden tests: open an issue with the problem id and the input.
2. **Translate.** UI strings live in i18n files (English and Spanish first). Problem statements have a `statement.es.md` next to `statement.md`.
3. **Add a problem.** Copy `content/problems/two-sum/`, rename the folder (it must equal `id`), and fill in every file. Run `cargo run -p kb-pack -- validate` before opening a pull request. Problems must be original; do not copy from other judge sites. Content is licensed CC BY-SA 4.0.
4. **Add a language.** Add `languages/<id>/language.toml` and a harness template, plus golden tests. Open an issue first to discuss.

## Development

See [README.md](README.md) for setup and [CLAUDE.md](CLAUDE.md) for architecture rules that apply to every contributor, human or AI.

- Keep pull requests small and focused, with tests.
- `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test` and `pnpm build` must pass.
- Changes to the sandbox (`crates/kb-runner`) need a maintainer's explicit approval.

By contributing you agree that your code is licensed under MIT OR Apache-2.0 and your content under CC BY-SA 4.0.
