# Release checklist

These are the steps used for 1.0.0-beta.1, in order, so the next release repeats them.
Releases are built by `.github/workflows/release.yml` from a `v*` tag into a **draft,
pre-release** GitHub release (ADR 0021, Windows only for now). Everything up to step 8
happens locally on Windows, on a `chore/release-<version>` branch from an up-to-date
`main`.

## 1. Version
1. Set the version in the three places (`docs/packaging.md`, "Versions"):
   - `Cargo.toml` (`[workspace.package] version`);
   - `apps/desktop/package.json`;
   - `apps/desktop/src-tauri/tauri.conf.json` (`version`).
2. Set `bundle.windows.wix.version` in `tauri.conf.json` from the MSI rule (for example
   `1.0.0-beta.2` → `0.99.2`, `1.0.0` → `1.0.0`).
3. Refresh `Cargo.lock`: `cargo check -p katabox-desktop`.
4. Run `pnpm versions:check`. It must print `versions ok`.
5. If the content changed, bump `content/pack.toml` (`version` and `date`).

## 2. Documents
- **`CHANGELOG.md`:** a section for the version (features, fixes, content added).
- **Release notes:** `release/<version>.md` (highlights, install, upgrading, known
  issues, a checksums placeholder).
- **`docs/known-issues.md` and `.es.md`:** add what's new, remove what's fixed.
- **README and user guide:** update the numbers if the catalogue changed. Retake the
  `docs/images/` screenshots when the UI changed visibly (1280 × 800, light theme).
- **"What's new":** `whatsNew.items` in both locales holds the highlights for the
  dialog shown after an upgrade (`apps/desktop/src/app/WhatsNew.tsx`).

## 3. Pins and notices
- If `crates/kb-toolchain/manifest.toml` changed since the last release, run
  `KB_REAL_PINS=1 cargo test -p kb-toolchain --release --test real_pins`.
- `pnpm notices` must succeed (it fails on a license with no text) and list no
  unexpected license. `pnpm licenses:check` and `cargo deny check licenses` must pass.

## 4. Checks (scoped)
- `cargo fmt --all -- --check`.
- clippy, three ways:
  - the workspace;
  - kb-runner with `unscoped-fallback-for-tests`;
  - the desktop app, with and without `updater`.
- **Bindings:** `cargo run -p katabox-desktop --features bindings --bin export-bindings`,
  then `git diff --exit-code apps/desktop/src/bindings.ts`.
- `cargo test` for the crates the release changed, run with `--test-threads=1`.
- The kb-runner attack suite, once, after the release build:
  `cargo test -p kb-runner --test attacks_windows`.
- `pnpm test`, `pnpm build`, `pnpm licenses:check`.
- `cargo run --release -p kb-pack -- validate` (structure only). Run the whole-catalogue
  `--run` only when the content or the judge changed since the last full run.

## 5. Build
1. Verify the toolchains: `scripts/fetch-toolchains.ps1 -NoLanguageServers`.
2. Fill the profile caches: `cargo run --release -p kb-pack -- profile --inputs`. It must
   end with exit code 0.
3. Build `pnpm release:build -- --bundles "nsis,msi"`. Quote the list in PowerShell, or
   it becomes two arguments.
   - Output: `target/release/bundle/nsis/KataBox_<version>_x64-setup.exe` and
     `target/release/bundle/msi/KataBox_<version>_x64_{en-US,es-ES}.msi`.
4. Check the MSI's ProductVersion (it must match the rule), from PowerShell:

   ```powershell
   $wi = New-Object -ComObject WindowsInstaller.Installer
   $db = $wi.GetType().InvokeMember('OpenDatabase','InvokeMethod',$null,$wi,@((Resolve-Path target\release\bundle\msi\KataBox_<version>_x64_en-US.msi).Path,0))
   $v = $db.GetType().InvokeMember('OpenView','InvokeMethod',$null,$db,@("SELECT Value FROM Property WHERE Property='ProductVersion'"))
   $v.GetType().InvokeMember('Execute','InvokeMethod',$null,$v,$null)
   $r = $v.GetType().InvokeMember('Fetch','InvokeMethod',$null,$v,$null)
   $r.GetType().InvokeMember('StringData','GetProperty',$null,$r,@(1))
   ```
5. Write `target/release/bundle/SHA256SUMS.txt` (`<sha256>  <file name>` per
   installer). Copy the release notes next to it, with the checksums filled in.

## 6. Install checks (Windows, NSIS)
**Your real data first:**
- Record the SHA-256 of every file in `%APPDATA%\dev.katabox.app` (user.db, its `-wal`
  and `-shm`, backups), and the file count and size of `%LOCALAPPDATA%\dev.katabox.app`.
- Rename both folders to `dev.katabox.app.real-backup`, in the same parent folders.
- Never delete them.

1. **Upgrade:**
   1. Build the previous version's NSIS setup from the commit before the bump.
   2. Install it (`setup.exe /S` works), start it, finish the welcome, install a
      language, and create state:
      - a solved problem (this schedules a review);
      - a draft;
      - a setting (the theme);
      - a list;
      - a Learn topic played to its end.
   3. Close it, then install the new setup over it.
   4. Check:
      - one entry in Settings → Apps, with the new version;
      - "What's new" shows once, and the welcome doesn't;
      - the solved status, draft, list, theme, Learn progress and review are all there;
      - About shows the new version and the content pack.
2. **Smoke test the new build:**
   - one problem each: function (Python), program (TypeScript), SQL;
   - Learn playback;
   - the debugger (a breakpoint, then Debug this case);
   - "Find a smaller failing input";
   - Export and then Import. The file dialogs are native; type the path and press
     Enter.
3. **Uninstall:** run `%LOCALAPPDATA%\KataBox\uninstall.exe` (or Settings → Apps).
   Then check:
   - the app folder and the uninstall entry are gone;
   - `%LOCALAPPDATA%\Packages\katabox.sandbox` and the AppContainer mapping in
     `HKCU\Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Mappings`
     are gone;
   - `icacls` on a folder in `sandbox\toolchains` no longer lists an `S-1-15-2-…` grant;
   - both data folders are still there.
4. **Clean first start:**
   1. Move the test data aside, then install the new setup.
   2. Check the welcome.
   3. Install one language (a real download).
   4. Run and Submit a problem.
5. **Restore your data:**
   1. Move the test data aside (keep it as `dev.katabox.app.release-*-test`).
   2. Rename `.real-backup` back.
   3. Re-hash. Every hash, the file count and the size must match.
   4. Don't start the app before re-hashing.

Screenshots of each step go in `.handoff/screenshots/release/`. The CDP driver
`drive-release.mjs` there automates the app: start it with
`WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9224`.

**MSI:** a per-machine install needs administrator rights. Follow the maintainer's MSI
steps: install, use, uninstall, check that the cleanup ran and the data is kept.

## 7. Pull request
Commit on the release branch, push, open the PR. CI must be green. The maintainer
merges.

## 8. Tag and draft release (maintainer)
1. On `main`: `git tag -a v<version> -m "KataBox <version>"` and
   `git push origin v<version>`.
2. In the draft release:
   - check the assets (NSIS, MSI en-US and es-ES);
   - paste `release/<version>.md`, with the checksums of the files the workflow built;
   - install the downloaded NSIS setup once;
   - publish.
3. **After signing is set up:** sign the installers (SignPath), then submit the winget
   manifest.
