# Changelog

All notable changes to KataBox are recorded in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- The window reopens where it was, at the same size, and maximised if it was. If its
  screen is gone, it opens centred on the main screen. Settings → Appearance → **Reset
  window size and position**.
- **Progress** in tabs: Overview, By pattern, By topic (now with the Learn topics viewed
  per subject) and History.
- **Settings** in sections: Appearance, Editor, Practice, Languages and runtimes, Learn,
  Data and About, with **Search settings**.
  - The Learn section adds the animation speed.
  - Every tab and section can be linked to, and the last one used opens next time.
- The command palette opens each Progress tab and Settings section ("Progress: By
  pattern", "Settings: Editor").

### Fixed
- The command palette now finds a command typed exactly as its label, punctuation
  included ("Theme: Dark"), and ignores accents.

## [1.0.0-beta.1] - 2026-11-15

The first release. Everything below is new.

### Practice

- 209 original problems: 150 function problems, 35 program problems that read standard
  input and write standard output, and 24 SQL problems (SQLite). 68 easy, 90 medium and
  51 hard.
- 27 patterns, ordered as a track from fundamentals to SQL, with a pattern map on the
  Progress page.
- Run checks your code on the public tests and your own custom tests; Submit judges it
  against the public and hidden tests.
- Fair time limits per language, scaled to your machine by a short calibration
  benchmark.
- Three hint tiers per problem. The editorial unlocks once you solve the problem or after
  three unsuccessful submissions.
- Linked lists and binary trees (`ListNode`, `TreeNode`) as input and output types in
  every language.
- Program problems check output by tokens or lines, and show the first differing token
  or line next to the expected output.
- SQL problems with a Tables tab, tests shown as tables, and wrong rows shown next to the
  expected ones.
- Daily problem with a streak, spaced-repetition reviews, and a "next problem"
  suggestion after an Accepted submission.
- Personal lists, with drag-and-drop or keyboard reordering.
- Timed mock interviews (30, 45 or 60 minutes, two unseen problems) with hints, the
  editorial and past submissions locked, and a scorecard at the end.
- "Solve in…" another language, with your earlier solution beside the new one.
- Progress page: solved by difficulty, language and topic, a 12-month activity heatmap,
  the review queue and interview history.
- Complexity profiler: 95 problems can estimate your solution's time complexity from
  runs at growing input sizes, shown next to the reference. The result is always marked
  "Estimated".
- Failing-test shrinker: "Find a smaller failing input" reduces a failing visible test
  to a smaller one that still fails, then lets you debug it or keep it as a custom test.
- Welcome flow on first start: install a language, an optional three-problem placement,
  and a short tour of Run, Submit, hints and the debugger.
- Export and import of all your progress as one JSON file (`katabox-export/v1`). Import
  shows a preview and merges, keeping the newer record.

### Learn

- A Learn section with 118 animated topics in 14 subjects, from sorting and searching to
  graphs, dynamic programming, strings and databases.
- Three learning paths: "Start here", "Interview essentials" and "Graph master".
- Each topic has an animation you can play, pause and step through, its theory, sample
  code in Python and TypeScript that follows the animation line by line, and a quiz.
- Try your own input on each topic, within the topic's limits.
- Links between Learn topics and practice problems, and an optional "Review a topic"
  card on Home.

### Editor and debugging

- Code editor (CodeMirror) with default, VS Code, Vim and Emacs keymaps, font size, tab
  width and four colour themes.
- Language servers for diagnostics and completion: basedpyright (Python),
  typescript-language-server (JavaScript/TypeScript) and rust-analyzer (Rust). They
  never run your code.
- Drafts and notes saved automatically; snapshots with their verdicts, side-by-side
  comparison and restore.
- Focus mode (Ctrl+Shift+F), Run (Ctrl+') and Submit (Ctrl+Enter).
- Command palette (Ctrl+K) for pages, problems, theme and language.
- Replay debugger for Python and JavaScript/TypeScript: breakpoints, stepping forward and
  back, variables, watches and a console. Every step is an ordinary sandboxed run.
- "Debug this case" on a failing test.
- Visualize tab in the debugger: arrays with pointers, grids, linked lists, trees and
  graphs drawn from your variables.
- Program problems can be debugged in Python and TypeScript.

### Languages

- Python, JavaScript/TypeScript, PHP and Rust for function and program problems; SQL
  (SQLite) for SQL problems.
- JavaScript/TypeScript is one language: the TypeScript file can be edited as plain
  JavaScript with a per-problem toggle.
- Runtimes and language servers are downloaded on demand, from pinned URLs checked by
  size and SHA-256, and can be uninstalled per language.
- Solutions use each language's standard library only.

### Privacy and security

- Works offline after the first language download. The network is used only to download
  runtimes and language servers, from a fixed list of HTTPS hosts.
- No telemetry and no account.
- Your code runs in an OS sandbox (Windows AppContainer and Job Object) with no network,
  no access to your files, and capped CPU time, memory and output.
- SQL queries run read-only in an in-memory SQLite database with strict limits.
- "Copy diagnostics" in Settings copies the app version, OS, runtime versions and recent
  error codes, with no code and no personal data.
- Your data stays on your machine: `user.db` in `%APPDATA%\dev.katabox.app`; the sandbox,
  runtimes and caches in `%LOCALAPPDATA%\dev.katabox.app`.
- Code is MIT OR Apache-2.0; problems and Learn content are CC BY-SA 4.0.

### Accessibility

- English and Spanish interface, problems and Learn content.
- Keyboard use throughout, with a visible focus ring, a "Skip to main content" link and
  page landmarks.
- Light, dark and system themes, plus a high-contrast option. Colours meet WCAG AA, and
  verdicts, difficulty and status always come with an icon or a label.
- A "Reduce motion" setting that also applies to Learn animations.
- Screen-reader labels on the activity heatmap, the Learn scenes and the output diffs.
- A keyboard-shortcuts dialog, opened with F1, from the command palette or with the
  keyboard button in the top bar.

### Installers

- Windows 10/11 (x64) installers: an NSIS installer (per-user, no administrator rights)
  and MSI packages in English and Spanish (per-machine, for managed machines).
- The installer installs WebView2 silently if it is missing.
- Uninstalling removes the sandbox's Windows profile and folder permissions. Your data is
  kept unless you tick "Delete the application data" (NSIS).
- An About page with the app version, the content pack version and date, licences and
  third-party notices.
- A "What's new" dialog after an upgrade.

[1.0.0-beta.1]: https://github.com/kaspal/katabox/releases/tag/v1.0.0-beta.1
