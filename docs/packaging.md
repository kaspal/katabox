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

## Languages: runtimes and language servers (ADR 0014, 0015)

- Nothing is bundled: on first run the app asks which languages to install and downloads
  them (kb-toolchain) into the sandbox root (`<local app data>/sandbox` by default):
  `toolchains/` for the judge, `tools/` for language servers (basedpyright,
  typescript-language-server + TypeScript, rust-src, rust-analyzer).
- Sizes on Windows (download / on disk): Python 98 MB / 297 MB, JavaScript/TypeScript
  46 MB / 140 MB, PHP 87 MB / 103 MB, Rust 244 MB / 959 MB (Node is shared between
  Python's language server and TypeScript).
- Network: HTTPS to the manifest's hosts only (github.com and its release-asset hosts,
  nodejs.org, windows.php.net and downloads.php.net, static.rust-lang.org,
  registry.npmjs.org), the system proxy, only when the user installs. Firewall or proxy
  allow-lists need those hosts.
- Pins are compiled into the app (`crates/kb-toolchain/manifest.toml`); each app version
  installs exactly its pinned files.
- They run outside the sandbox; their license texts must ship with them (see
  [third-party-notices.md](third-party-notices.md)).
- PHP has no language server on Windows (phpactor needs the posix extension).

## Versions

- **The app version** is in three files, all the same: `Cargo.toml`
  (`[workspace.package] version`), `apps/desktop/package.json` and
  `apps/desktop/src-tauri/tauri.conf.json` (`version`). Refresh `Cargo.lock` with
  `cargo check -p katabox-desktop` after a change.
- **NSIS** takes it as is, pre-releases included (`1.0.0-beta.1`). Its upgrade check
  compares semver, so every later version installs over an earlier one.
- **MSI (WiX) needs a numeric version,** set in `bundle.windows.wix.version`. Windows
  Installer compares only `major.minor.build`, and a later release must always compare
  higher, or the MSI refuses the upgrade. The rule:

  | App version | MSI version | Example |
  | --- | --- | --- |
  | `M.m.p` (a final release) | `M.m.p` | `1.0.0` → `1.0.0`, `1.0.1` → `1.0.1` |
  | `M.0.0-beta.N` (N = 1-99) | `(M-1).99.N` | `1.0.0-beta.1` → `0.99.1`, `1.0.0-beta.2` → `0.99.2` |
  | `M.0.0-rc.N` (N = 1-99) | `(M-1).99.(100+N)` | `1.0.0-rc.1` → `0.99.101` |

  So 0.1.0 < 1.0.0-beta.1 (0.99.1) < beta.2 (0.99.2) < rc.1 (0.99.101) < 1.0.0 <
  1.0.1. Minor version 99 is reserved for those pre-releases. Other pre-releases (an
  alpha, or a beta of 1.1.0) are refused until the rule is extended, with its self-test.
- **The check:** `pnpm versions:check` (`scripts/check-versions.mjs`) runs a self-test of
  the ordering, then checks that the three files, `Cargo.lock` and the MSI version agree
  with the rule. CI and the release workflow run it.
- **The bundle identifier stays `dev.katabox.app`.** Tauri warns that an identifier
  ending in `.app` clashes with the macOS application bundle extension. That matters
  only on macOS, which KataBox doesn't target. Changing it would move every user's data
  folders (`%APPDATA%\<identifier>`, `%LOCALAPPDATA%\<identifier>`), so it stays.
- **The content pack** has its own version and date in `content/pack.toml` (id, version,
  `YYYY-MM-DD`). `kb-pack validate` checks it, the build writes it into content.db, and
  Settings → About shows it. Bump it when the content changes in a release.

## Windows installers (ADR 0021)

- **Build:** `pnpm release:build -- --bundles nsis,msi` (from the repo root). It runs
  `pnpm notices` first: a release build refuses to bundle the placeholder notices.
  Output: `target/release/bundle/nsis/KataBox_<version>_x64-setup.exe` and
  `target/release/bundle/msi/KataBox_<version>_x64_{en-US,es-ES}.msi`.
- **NSIS (the main download):**
  - installs **per-user**, no administrator rights, into `%LOCALAPPDATA%\KataBox`;
  - is in English and Spanish, following the system language;
  - adds a Start-menu shortcut;
  - shows the KataBox artwork: `windows/sidebar.bmp` (164 × 314, welcome and finish
    pages) and `windows/header.bmp` (150 × 57, the other pages, installer and
    uninstaller). Both are drawn from `icons/katabox.svg` by `pnpm installer-art`
    (`scripts/gen-installer-art.mjs`, no dependencies) and committed; re-run it after
    changing the icon.
- **MSI (managed machines):** Tauri's per-machine WiX template, in en-US and es-ES.
- **WebView2:** `embedBootstrapper` (silent). Windows 10/11 already have WebView2; the
  bundled bootstrapper (about 1.8 MB) installs it where it is missing.
- **Bundled resources:** `content.db` (built by `build.rs`) and
  `THIRD_PARTY_NOTICES.txt` (`pnpm notices`). Toolchains are never bundled.
- **Data:**
  - user.db: `%APPDATA%\dev.katabox.app`;
  - sandbox root, toolchains, caches, language-server projects:
    `%LOCALAPPDATA%\dev.katabox.app`.
- **Uninstall:**
  - **What is removed:** the app. The data stays unless the user ticks "Delete the
    application data" in the NSIS uninstaller. The MSI never deletes data.
  - **Sandbox cleanup:** both uninstallers first run `katabox-desktop.exe --uninstall-cleanup`
    (`windows/hooks.nsh`, `windows/cleanup.wxs`). It revokes the sandbox's folder grants
    and deletes the `KataBox.Sandbox` AppContainer profile through kb-runner's cleanup.
    It is skipped when an update replaces the app.
  - **By hand,** from the install folder: `katabox-desktop.exe --uninstall-cleanup`.
  - **Removing the data by hand** (after an uninstall that kept it): delete
    `%APPDATA%\dev.katabox.app` (user.db: progress, drafts, settings) and
    `%LOCALAPPDATA%\dev.katabox.app` (the sandbox, installed languages and caches).
    Export first to keep the progress. If the app was removed without its uninstaller,
    run the cleanup above first, from a copy of the install folder, or reinstall and
    uninstall once, so the AppContainer profile and the folder grants go too.

## Code signing (prepared, off)

Installers are unsigned until the repository is public, so Windows SmartScreen warns on
first launch ("More info" → "Run anyway"). Steps when the repository is public:

1. Apply to the SignPath Foundation open-source program (https://signpath.org) with the
   public GitHub repository. Their conditions include an OSI license (MIT/Apache:
   yes), releases built by CI from the public source, and a signing policy page.
2. In SignPath, create the project (`katabox`), an artifact configuration for the NSIS
   `.exe` and the `.msi`, and a release signing policy that requires manual approval.
3. Add the secret `SIGNPATH_API_TOKEN`, and the organization id and project slug as
   workflow variables.
4. In `.github/workflows/release.yml`, after the build:
   1. upload the installers as an artifact;
   2. add `signpath/github-action-submit-signing-request` (wait for completion);
   3. attach the signed files to the draft release instead of the unsigned ones.
5. Fallback: Azure Trusted Signing (paid, identity-validated) through `signCommand` in
   `bundle.windows`.

After signing works, publish to winget (a manifest PR to `microsoft/winget-pkgs` per
release).

## Automatic updates (prepared, off)

- **Plugin:** `tauri-plugin-updater` is an optional dependency behind the cargo feature
  `updater`, off by default. Without it, the plugin is not registered and no endpoint
  can be called.
- **Config:** in `apps/desktop/src-tauri/tauri.updater.conf.json` (public key, the
  GitHub Releases `latest.json` endpoint, `createUpdaterArtifacts`). It is merged only
  by:

  ```sh
  pnpm notices && pnpm tauri build --features updater --config src-tauri/tauri.updater.conf.json
  ```

- **Turning it on (the maintainer):**
  1. **Create the key pair** (run it yourself; never commit the private key):

     ```sh
     pnpm tauri signer generate -w "<a folder outside the repository>/katabox-updater.key"
     ```

     Choose a strong password when asked. This writes `katabox-updater.key` (private)
     and `katabox-updater.key.pub` (public).
  2. **Keep the private key and its password outside the repository,** backed up in two
     places: a password manager and an offline copy. Losing it means existing installs
     can no longer verify updates.
  3. **Add the GitHub secrets** `TAURI_SIGNING_PRIVATE_KEY` (the content of
     `katabox-updater.key`) and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`.
  4. **Set `plugins.updater.pubkey`** in `tauri.updater.conf.json` to the **content** of
     `katabox-updater.key.pub` (one base64 line), and check `endpoints`.
  5. **Enable it in the release workflow:**
     - uncomment the key environment variables;
     - add `--features updater --config src-tauri/tauri.updater.conf.json` to `args`;
     - set `includeUpdaterJson: true`.
  6. **Add the update check** (Rust command plus a Settings button) in its own reviewed
     change.
