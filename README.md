# KataBox

**Offline coding practice for your desktop.** Solve problems from easy to hard in Python, JavaScript/TypeScript, PHP and Rust, with a real editor, public and hidden tests, hints, editorials and a debugger. No account, no subscription, no internet needed after setup.

[Leer en español](README.es.md)

> **Status:** early development (pre-alpha). Windows installers come with the first release.

## Planned features

- 150 original problems across 23 patterns (fundamentals, arrays, two pointers, sliding window, trees, graphs, dynamic programming…)
- Public examples, your own custom tests (expected output computed for you) and hidden tests that show the first failing input
- Editor with autocomplete and diagnostics; step-through debugger with "Debug this case"
- Tiered hints, editorials and reference solutions in every language
- Mock interview mode, spaced-repetition review, daily problem, personal lists
- Complexity profiler and data-structure visualizer
- Optional local AI tutor through Ollama (nothing leaves your machine)
- English and Spanish interface

## Install

**Windows 10/11 (x64):** download `KataBox_<version>_x64-setup.exe` from the [Releases](https://github.com/kaspal/katabox/releases) page and run it.

- It installs for your user only; no administrator rights are needed. An `.msi` is also available for managed machines.
- Until the installers are code-signed, Windows SmartScreen may warn on first launch: choose **More info → Run anyway**.
- On first start KataBox asks which languages to install and downloads them (Python is about 100 MB). After that it works offline.
- **Uninstall** from Settings → Apps. Your progress is kept unless you tick **Delete the application data**.

Linux packages (.deb, .rpm, AppImage) are built by the same release workflow; see [docs/packaging.md](docs/packaging.md) for their requirements.

## Why desktop?

Your code runs on your machine in a sandbox (no network, capped time and memory), so everything works offline, starts instantly and stays private.

## Building from source

Requirements: Rust (stable), Node 22+, pnpm 10, and on Linux the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) (WebKitGTK 4.1).

```sh
pnpm install
pnpm dev          # run the app
cargo test        # run the Rust tests
```

## Contributing

Problems, translations and new languages are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Code: [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option. Problem content in `content/`: [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/).
