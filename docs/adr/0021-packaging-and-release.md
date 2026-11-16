# 21. Packaging, uninstall, notices, signing and updates

Date: 2026-11-12 · Status: proposed

## Context
KataBox needs to be installable on Windows: an icon, product metadata, installers, a
clean uninstall, the third-party notices its licenses require, and a path to code
signing and automatic updates.

Two constraints hold while the repository is private:
- no signing key exists;
- no update endpoint may be called.

The build plan's packaging table sets the targets:
- Windows: NSIS per-user plus MSI, with WebView2 "already present on supported Windows;
  `embedBootstrapper` as a fallback", signing through SignPath Foundation;
- Linux: deb, rpm and AppImage;
- updates: the Tauri updater with signed manifests on GitHub Releases.

## Decision

### Identity
- **Icon:** an original mark, an isometric cube in the UI's primary indigo, drawn in
  `apps/desktop/src-tauri/icons/katabox.svg`.
  - Every size comes from it: `pnpm tauri icon src-tauri/icons/katabox.svg -o
    src-tauri/icons`, from `apps/desktop`. The mobile folders are deleted.
  - The UI uses the same mark: `components/Logo.tsx` and the favicon.
- **Metadata:**
  - product name "KataBox", version 0.1.0;
  - publisher "KataBox contributors";
  - copyright "Copyright © 2026 Ehrlichmann Casas and KataBox contributors";
  - license `MIT OR Apache-2.0`;
  - the window title stays "KataBox".

### Installers (`tauri.conf.json` → `bundle.windows`)
- **NSIS, the main download.**
  - It installs **per-user** (`installMode: currentUser`): no administrator rights, files
    under `%LOCALAPPDATA%\KataBox`.
  - It is in English and Spanish, following the system language.
  - It has no license page: MIT/Apache need no acceptance.
- **MSI, for managed machines.**
  - It is Tauri's per-machine WiX template, in en-US and es-ES.
  - It never deletes user data.
- **WebView2:** `embedBootstrapper` (silent). Windows 10/11 already ship WebView2, and the
  bootstrapper (about 1.8 MB) only covers machines without it.
- **content.db** is built by `build.rs` from `content/problems` and bundled as a
  resource (ADR 0018). Toolchains are never bundled: the first run downloads them
  (ADR 0015).

### Data folders and uninstall
- **Where data lives:**
  - user.db in `%APPDATA%\dev.katabox.app`;
  - the sandbox root, toolchains, caches and language-server projects in
    `%LOCALAPPDATA%\dev.katabox.app`.
  - Development builds use the `-dev` folders (ADR 0012).
- **Uninstalling removes the app; the user's data stays.**
  - NSIS: Tauri's own "Delete the application data" checkbox (off by default) removes
    both folders.
  - The MSI never removes them.
- **Sandbox cleanup:** the sandbox leaves state outside those folders.
  - It leaves the `KataBox.Sandbox` AppContainer profile (registry and
    `%LOCALAPPDATA%\Packages\katabox.sandbox`), and grants on the sandbox's folders.
  - Both installers therefore run `katabox-desktop.exe --uninstall-cleanup` before removing
    the files:
    - NSIS: the `NSIS_HOOK_PREUNINSTALL` hook in `windows/hooks.nsh`;
    - MSI: a deferred custom action in `windows/cleanup.wxs`, run as the user and
      ignoring failures.
  - **What it does:** it resolves the sandbox roots (the `sandbox_root` setting, read
    without migrating user.db, and `<local data>/sandbox`). It lists the folders that
    can hold persistent grants (`toolchains/*`, `cache/*`, `runs/*`). Then it calls
    kb-runner's existing `cleanup`, which revokes those grants and deletes the profile.
  - **There is no kb-runner change.** The cleanup never deletes files.
  - **Updates:** it does not run when an update runs the old uninstaller (`/UPDATE`,
    or an MSI upgrade), so the next version keeps its sandbox.
  - **Dev builds** use the same profile name. Deleting it is harmless: the next run
    recreates it and re-grants (grants are idempotent).

### Third-party notices
- **Generator:** `pnpm notices` (`scripts/gen-notices.mjs`) writes
  `src-tauri/resources/THIRD_PARTY_NOTICES.txt`. It has three parts:
  - every Rust crate linked into the app, from `cargo metadata` for the target
    platform, following normal dependencies only, with the crate's own license and
    notice files;
  - every npm package in the UI bundle (`pnpm licenses list --prod`) with its files;
  - `docs/third-party-notices.md` (runtimes and language servers).
- **Packages without a license file** are listed with their authors and SPDX id. The
  standard text is in an appendix from `scripts/license-texts/`. The script fails on a
  license it has no text for.
- **No new tool:** it reads only what a build already has on disk (the cargo registry,
  node_modules); cargo-about is not needed.
- **In the app:** the file is bundled. Settings → About → Third-party notices shows it,
  read by a Rust command, so the UI never reads files.
- **Release guard:** development builds get a placeholder, and a release build
  (`PROFILE=release`) refuses to compile with it.

### Code signing: prepared, off
- **Off for now:** installers are unsigned while the repository is private.
- **When the repository is public:** apply to SignPath Foundation's open-source
  program, then add its GitHub action to the release workflow (the commented step).
  Azure Trusted Signing is the paid fallback. The steps are in `docs/packaging.md`.

### Automatic updates: prepared, off
- **The plugin is opt-in:** `tauri-plugin-updater` is an **optional** dependency behind
  the cargo feature `updater`, off by default.
  - Without the feature, the plugin is not registered, so no endpoint can be called.
  - `tauri.conf.json` has no updater section, and no capability is added. Checks will
    run from Rust.
- **The config is separate:** it lives in `src-tauri/tauri.updater.conf.json`
  (`createUpdaterArtifacts`, the public key placeholder, the GitHub Releases
  `latest.json` endpoint). It is merged only by `tauri build --features updater
  --config src-tauri/tauri.updater.conf.json`.
- **Keys:** no key is generated or stored by this change. The maintainer creates the
  key pair: the private key stays outside the repository, backed up, and goes into
  GitHub secrets; the public key goes into the config. The steps are in
  `docs/packaging.md`.

### Releases
- **Workflow:** `.github/workflows/release.yml` runs only for `v*` tags or by hand.
  - It builds the Windows (NSIS, MSI) and Linux (deb, rpm, AppImage) installers with
    `tauri-apps/tauri-action`.
  - It attaches them to a **draft** release.
  - Signing and updater steps are present as comments.
- **Checklist:** `docs/release-checklist.md` lists the steps before a tag.

## Consequences
- The CSP and the capabilities are unchanged.
- `tauri.conf.json` gains bundle metadata, the notices resource and the
  `bundle.windows` section.
- **Release builds need `pnpm notices` first**: `pnpm release:build` runs both.
- **The MSI is per-machine** (Tauri's template) while its data is per-user. Its cleanup
  runs as the uninstalling user.
- **Unsigned installers show SmartScreen warnings** until signing is set up. The README
  explains it.
- **Turning updates on later** means: the key pair, the secrets, the public key in
  `tauri.updater.conf.json`, the feature in the release build, and an update check in
  the UI. It is a separate change with its own review.

## Follow-ups
- Signing (SignPath), then winget.
- ~~Branded installer artwork~~: done in content batch 3 (`pnpm installer-art`).
- The updater UI (check from Settings, from Rust) once a public endpoint exists.
- Linux postinst for the AppArmor profile (build plan); MSIX later.

## Addendum: 1.0.0-beta.1 (2026-11-15)
- **Version:** the app is 1.0.0-beta.1. WiX can't take a pre-release, so the MSI gets a
  numeric version from a rule that keeps every later release higher
  (`1.0.0-beta.N` → `0.99.N`). `pnpm versions:check` enforces it; the details are in
  `docs/packaging.md`, "Versions".
- **The identifier stays `dev.katabox.app`,** despite Tauri's macOS warning about
  `.app`: changing it would move every user's data folders.
- **Content pack:** `content/pack.toml` gives the pack its own version and date, shown on
  Settings → About.
- **The release workflow builds Windows only** until Linux ships. The Linux matrix entry
  is kept as a comment.
- **First start after an upgrade:** a "What's new" dialog. It is driven by the
  machine-local setting `app.last_version`, which is never exported.
