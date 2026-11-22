# 41. The Home session, the path on the Home, the new icon and the startup splash

Date: 2026-11-22 · Status: proposed (feat/home-brand)

## Context
The Home was a grid of equal cards (continue, solved, daily, reviews, plan, rusty). It
did not say what to do today, and the Learn paths (ADR 0029) were not on it at all. The
maintainer approved three mockups: a Home built around a short "today's session" and the
learning path (HomeRecommended), a new app icon ("Kata Box") and a startup splash.

## Decision: the Home
The rules are pure functions in `apps/desktop/src/practice/home/` with unit tests; the
hooks only gather data the app already has. No new storage, no new command.

- **Today's session** (`session.ts`, `buildSession`): one to three items, in this order:
  1. *Continue*: the latest draft (`continue_item`), unless its problem was solved before
     today. A draft solved today stays in the list, marked done.
  2. *Daily*: the existing daily pick (`daily_problem`, unchanged), when
     `practice.daily` is on.
  3. *Kata*: the daily kata, when katas are on (`practice.rusty`, the only kata switch)
     and one is available (`kata_options.daily` not empty, or today's session exists).
  4. *Path*, when there is room: "Watch: <topic>" (about 5 minutes, linking to the
     topic) when the current step is unwatched and has no unsolved linked problem;
     otherwise the step's first unsolved problem, else the first one further along.
  - No duplicates: a daily pick that is also the path's next problem is one item with
    the "On your path" badge; a draft that is the daily or the path problem is not
    listed separately.
  - A brand-new user (nothing solved) gets "Start here" first: "Start here: watch
    <topic>" when the first step is such a topic, else the path's first problem. So the
    session and the path never disagree about where the user is.
  - Minutes: continue 5, kata 10, problems Easy 10 / Medium 20 / Hard 30. The header
    shows the sum rounded to the nearest 5.
  - Done today: daily when solved today, kata when today's kata ended with a solve,
    continue / path when Accepted today (from the activity rows). "Start session" opens
    the first item that is not done.
- **The path** (`path.ts`): `practice.homePath` names the path (exported with the other
  `practice.` settings); unset means the first path with any progress, else
  `start-here`. One step per topic. A step is *done* when the topic was viewed
  (`learn.viewed`) and at least one linked problem is solved; a topic with no linked
  problems is done once viewed. *Current* is the first step not done. The track shows a
  window of 7 (two before current, current, four after) and a "+N more" node; below
  900 px only current and two on either side. Nodes are links to their topics with an
  `aria-label`; the current one pulses, never under reduced motion (system or
  `ui.motion`).
- **This week** is Monday to Sunday from the heatmap's activity rows; **Solved** is three
  nested rings from the problem list's counts.
- **Watch next** (three Learn topics, never one the session already offers): the
  path's next topic; a watched topic with an unsolved linked problem
  (`topic_to_review`); then, until there are three, unwatched topics for the daily
  problem's pattern (which leans toward the weakest pattern, ADR 0019), unwatched path
  topics, and any unwatched topic. The thumbnails are static SVGs per subject family
  drawn with the `--viz-*` tokens (curves for complexity, recursion and math; bars,
  graph, tree, grid and strip for the others); the Learn engine never runs on the Home. No step
  counts are shown: the catalogue summaries do not have them.
- **Keep it fresh:** Reviews, Study plan and Rusty in compact form (one line and one
  action); a card whose switch is off is not rendered and the row reflows. Review's
  snooze and remove stay reachable from a menu on the compact card.
- **Layout:** fluid; the Today row (1.7 : 1) stacks below about 1100 px and Watch next
  goes from three columns to two, then one. The page keeps a visually hidden `<h1>`
  ("Home"); the greeting is the visible heading.

## Decision: the icon
- `src-tauri/icons/katabox.svg` is the approved design, with the `{ }` braces drawn as
  stroked paths (no font). `pnpm tauri icon src-tauri/icons/katabox.svg` (from
  `apps/desktop`) makes every size.
- The chevron is drawn screen-aligned (no shear) and centred on the right face: sheared
  like the braces, its top stroke flattened and it read as a "7" (a flaw in the
  mockup). The braces stay sheared; the right one mirrors the left exactly.
- At 16-32 px the braces turn to noise, so `katabox-small.svg` (cube and a bolder
  chevron) is rasterised into `icon.ico`'s 16, 24 and 32 px layers and `32x32.png` by
  `pnpm icon-small`, which keeps the ICO's larger layers and reads the result back.
- `scripts/lib/svg-raster.mjs` is a small, dependency-free rasteriser for these files
  (rect, paths with curves flattened, linear gradients, matrix transforms) shared by
  `icon-small` and `installer-art`. The art is original, so the notices do not change.
- Brand colours are tokens (`--brand-*`), the same in both themes; the `Logo` component
  uses them.

## Decision: the startup splash
- **Window:** `splash` in `tauri.conf.json` (640 × 400, frameless, not resizable,
  centred, no taskbar button, dark background colour) with `create: false`: the shell
  builds it at setup only when the plan shows it, so in `never` mode it never paints.
- **Page:** `splash.html` + `src/splash/` is a second Vite entry: static HTML, CSS
  keyframes from the approved mockup, and a few lines of TS. No React, no i18next, no
  Tauri API, no IPC. Text in EN/ES/pt-BR from a tiny map, in the UI language the main
  window mirrors in localStorage (`katabox.ui_lang`), else the system's. Reduced motion
  (the app's `katabox.ui.motion` mirror or the system setting) uses the fade-only
  variant. The version comes from Vite's `define`.
- **Modes** (`app.splash`, machine-only, Settings → Appearance → "Startup animation"):
  `firstRunAfterUpdate` (default) plays the whole animation (about 2.6 s) on the first
  start after an install or update (the stored `app.splash_version` differs from this
  build's) and otherwise closes as soon as the app is ready; `always` plays it in full
  every time; `never` shows no splash. The pure rule is `kb_app::splash::splash_plan`.
- **Handover:** the main window starts hidden and is placed (`window_state::place`); the
  UI calls `app_ready` once, after the shell's first render and the first `app_info`
  settles. The command (ignored unless it comes from `main`) waits out the rest of the
  animation when the plan says so, shows the main window, adds `.out` to the splash
  with `eval` (a 180 ms fade), and closes it. The splash never delays start-up beyond
  its animation.
- **Failsafe:** after 15 s without `app_ready` the shell shows the main window, closes
  the splash and logs a warning, so the main window always appears. Debug builds read
  `KATABOX_DEV_APP_READY=ignore|delay:<ms>` to exercise this; it is compiled out of
  release builds.
- **Security:** the capability (`capabilities/default.json`) still lists only `main`;
  the splash window has no permissions. Tauri allows app commands from any window, so
  the splash page loads no Tauri API (a test checks the page's sources) and `app_ready`
  checks the caller's label. The CSP is unchanged: the splash uses bundled script and
  style files only.
- **Linux:** untested there. WebKitGTK honours `decorations: false`, but window
  shadows and `backgroundColor` vary by compositor; the failsafe still guarantees the
  main window. Check the splash on Linux before a Linux release.

## Consequences
- Two new rules: the splash never gets a capability; the splash adds delay only on the
  first run after an update or in `always` mode.
- `window_state::restore` became `place` + `show`; anything that showed the main window
  at setup now goes through the handover.
- Custom paths for the Home are a later idea (`.handoff/backlog.md`).
