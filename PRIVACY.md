# Privacy

KataBox runs on your computer and keeps your data there. This page says what it stores, where, and when it uses the network.

## What KataBox does not do

- No telemetry or analytics.
- No account or sign-in.
- No crash reporting.
- No update checks. The auto-updater is not compiled into the app.
- No ads and no third-party services.

## What is stored, and where

Everything stays on your machine, in two folders.

**`%APPDATA%\dev.katabox.app`** holds `user.db`, a SQLite database with:

- your settings (editor, appearance, languages and similar choices);
- drafts: the code you are writing, per problem and language;
- submissions: your code, the verdict, time, memory and the judge's report;
- code snapshots from Run and Submit, with their verdicts;
- problem status (attempted, solved), which problems you opened and when, and which hints you unlocked;
- your notes;
- spaced reviews, daily problems, your personal lists and mock interview sessions;
- Learn progress (topics viewed and the last topic you opened).

**`%LOCALAPPDATA%\dev.katabox.app`** holds:

- the sandbox folder, where your code runs (each run's work folder is deleted when the run ends);
- the languages you installed (runtimes and language servers);
- caches (for example compiled Rust programs) and partial downloads;
- the project files the language servers use for autocomplete.

KataBox's sandbox also registers a Windows AppContainer profile (`KataBox.Sandbox`). The uninstaller removes it.

When you uninstall, both folders are kept unless you tick **Delete the application data**. You can also delete them yourself.

## When KataBox uses the network

Only when you install a language (on first run, from Settings → Languages, or from an Install button) and only after you click Install or Retry. Then it:

- downloads runtimes and language servers over HTTPS from these hosts only: github.com, release-assets.githubusercontent.com, objects.githubusercontent.com, nodejs.org, windows.php.net, downloads.php.net, static.rust-lang.org and registry.npmjs.org;
- downloads only the files pinned in the app (URL, size and SHA-256), and checks each file before unpacking it;
- uses your system proxy settings;
- sends a User-Agent of `KataBox/<version>` and nothing about you.

Those hosts see a normal download request, including your IP address, as with any download.

There are no other connections. Running, submitting, debugging and everything else work offline.

## Your code

Your code runs only on your machine, inside the sandbox, which has no network access. It is never uploaded.

## Export and diagnostics

- **Export** writes a file only to the place you choose. Import reads only the file you pick.
- **Copy diagnostics** (Settings) copies versions, settings flags and recent error codes to your clipboard, so you can paste them into a bug report. It contains no code, file paths or personal data, and nothing is sent.

## In the future

An optional AI tutor may be added later. If it is, it will run locally on your machine, with no cloud service.

## Questions

Open an issue on the repository. For security problems, see [SECURITY.md](SECURITY.md).
