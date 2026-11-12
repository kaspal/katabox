# Release checklist

Releases are built by `.github/workflows/release.yml` from a `v*` tag into a **draft**
GitHub release (ADR 0021). Before pushing the tag:

1. **Version:** bump it in three places, all the same:
   - `Cargo.toml` (`[workspace.package] version`);
   - `apps/desktop/src-tauri/tauri.conf.json` (`version`);
   - `apps/desktop/package.json`.

   Then `cargo check -p katabox-desktop` to refresh `Cargo.lock`.
2. **CHANGELOG.md:** a section for the version (features, fixes, content added).
3. **Checks on `main`:**
   - CI is green;
   - locally on Windows: `cargo run --release -p kb-pack -- validate --run` and
     `cargo test`.
4. **Pins:** if `crates/kb-toolchain/manifest.toml` changed since the last release,
   `KB_REAL_PINS=1 cargo test -p kb-toolchain --release --test real_pins`.
5. **Notices:** `pnpm notices` succeeds (it fails on a license with no text) and lists
   no unexpected license. `pnpm licenses:check` and `cargo deny check licenses` pass.
6. **Local installer check (Windows, clean profile):**
   1. back up and move aside `%APPDATA%\dev.katabox.app` and
      `%LOCALAPPDATA%\dev.katabox.app`;
   2. `pnpm release:build -- --bundles nsis,msi`;
   3. install the NSIS setup and start KataBox from the Start menu;
   4. check the first-run screen, install Python, and solve Two Sum;
   5. uninstall: the app is gone, the data is kept, and the `KataBox.Sandbox`
      AppContainer profile is removed;
   6. reinstall, then uninstall with "Delete the application data": everything is gone;
   7. restore your folders.
7. **Tag:** `git tag -a v<version> -m "KataBox <version>" && git push origin
   v<version>`.
8. **Draft release:**
   - check the assets (NSIS, MSI en-US/es-ES, deb, rpm, AppImage);
   - install the downloaded NSIS setup once;
   - edit the notes from the CHANGELOG;
   - publish.
9. **After signing is set up:** submit the winget manifest.
