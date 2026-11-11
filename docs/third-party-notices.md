# Third-party notices

KataBox is MIT OR Apache-2.0. It bundles or downloads the components below. Release
packages must ship each component's license text (`LICENSE`/`COPYING` files from the
component, or the npm package's license file). Rust crates and npm packages linked into
the app are checked against the allow-lists in `deny.toml` and
`scripts/check-npm-licenses.mjs`.

## Linked into the app

| Component | License | Notes |
| --- | --- | --- |
| hakoniwa | LGPL-3.0-only WITH LGPL-3.0-linking-exception | Linux sandbox (kb-runner, ADR 0007); approved exception in `deny.toml`; include its license text |
| win32job | MIT OR Apache-2.0 | Windows Job Object for language servers (kb-bridge, ADR 0014) |
| url | MIT OR Apache-2.0 | kb-bridge |
| @codemirror/lsp-client 6.3.0 | MIT | Editor ↔ language server client (ADR 0014) |

Other Rust crates and npm packages are covered by the license checks above; their texts
go into the bundle's generated notices.

## Language servers (downloaded, run as separate processes; ADR 0014)

| Component | Version | License | Source |
| --- | --- | --- | --- |
| basedpyright | 1.40.2 | MIT (bundles pyright, MIT, and typeshed stubs, Apache-2.0 / MIT) | npm, `scripts/language-servers/package-lock.json` |
| typescript-language-server | 6.0.3 | Apache-2.0 | npm, same lockfile |
| TypeScript | 6.0.3 | Apache-2.0 | npm, same lockfile |
| rust-analyzer | 1.99.1 | MIT OR Apache-2.0 | static.rust-lang.org dist 2026-10-15 |
| rust-src (Rust standard library sources) | 1.99.1 | MIT OR Apache-2.0 | static.rust-lang.org dist 2026-10-15 |
| phpactor | 2026.10.07.0 | MIT | GitHub release (Linux only; unavailable on Windows) |

## Language runtimes (downloaded; ADR 0002)

The pinned Python, Node, PHP and Rust toolchains fetched by `scripts/fetch-toolchains.*`
keep their own license files in their install folders (for example Node's `LICENSE`,
Python's `LICENSE.txt`, PHP's `license.txt`, and Rust's `share/doc`). The download manager
must keep those files with each runtime.
