# Packaging notes

## Linux

- **Runtime dependency: `libseccomp2`** (Debian/Ubuntu `libseccomp2`, Fedora/openSUSE
  `libseccomp`). kb-runner links libseccomp through hakoniwa's `seccomp` feature
  (ADR 0007). Building needs `libseccomp-dev` (Fedora: `libseccomp-devel`). The `.deb` and
  `.rpm` bundles must declare it; the AppImage relies on the host's copy (present on all
  major desktop distributions).
- **Sidecar `kb-sandbox`**: the sandbox helper must ship next to the app executable;
  `kb_runner` looks for it there (or set it with `kb_runner::set_helper_path`). It is the
  same binary as the manual test command.
- **Never build with `unscoped-fallback-for-tests`.** It is for the attack suite only;
  release builds refuse to compile with it.
- Kernel requirements: Landlock (any ABI) + seccomp for namespace mode, and unprivileged
  user namespaces. Where those are blocked (e.g. Ubuntu's AppArmor restriction), fallback
  mode needs Landlock ABI 6 (Linux 6.12+); otherwise the app must explain that code cannot
  run. `kb-sandbox probe` prints what a machine supports.
- License: hakoniwa is LGPL-3.0-only with the LGPL-3.0 linking exception (approved
  exception, `deny.toml`); include its license text in the third-party notices
  ([third-party-notices.md](third-party-notices.md)).

## Language servers (ADR 0014)

- Shipped or downloaded into `<sandbox root>/tools`: basedpyright, typescript-language-server
  and TypeScript (Node packages from `scripts/language-servers/package-lock.json`), and
  rust-analyzer. rust-src goes into the bundled Rust toolchain. The product download
  manager must verify them like the toolchains.
- They run outside the sandbox; their license texts must ship with them (see
  [third-party-notices.md](third-party-notices.md)).
- PHP has no language server on Windows (phpactor needs the posix extension).
