# 32. Window memory, page tabs and palette matching

Date: 2026-11-16 · Status: proposed (ui-polish)

## Context
- KataBox always opened at 1280 × 800 on the default spot, wherever the user had left
  it.
- Progress and Settings had grown into long single pages.
- The command palette didn't find a command typed exactly as its label ("Theme: Dark").
  cmdk's fuzzy score needs every typed character, the colon included, in the item's
  value.

## Decision
**Window memory, without a plugin.**
- `tauri-plugin-window-state` would need a new capability entry, so the shell does it
  by hand.
- `src-tauri/src/window_state.rs` reads the window and the monitors. It saves the
  bounds in the ordinary settings store (`window.bounds`):
  - 500 ms after the last move or resize;
  - and on close.
- The rules are pure functions in `kb_app::window`, with unit tests:
  - **Restore:** use the saved bounds if at least half the window lies on a current
    monitor (moved inside if it sticks out). Otherwise centre on the primary monitor.
    The size is always clamped between the minimum and the monitor's work area.
  - **Never saved:** minimised bounds.
  - **Maximised:** keeps the last normal bounds, plus a flag.
- **Units:** physical pixels, with the scale factor of the monitor the bounds were saved
  on. On a monitor with another scale the size is rescaled, so the logical size stays.
  The default and minimum sizes are the logical ones in `tauri.conf.json`.
- **No flash:** the window is created hidden (`visible: false`), then placed and shown in
  `setup`.
- Fullscreen is not remembered.
- **Reset:** Settings → Appearance has "Reset window size and position" (the
  `reset_window` command).
- `window.bounds` is per machine and is never exported.

**Page tabs.**
- Progress has tabs (Overview, By pattern, By topic, History). Settings has sections
  (Appearance, Editor, Practice, Languages and runtimes, Learn, Data, About): vertical
  tabs in a labelled nav, stacked above the content below 1100 px.
- Both use the Radix tabs, so they get tab roles and arrow keys.
- The open tab is a search param (`/progress?tab=`, `/settings?section=`), so links,
  the palette and Back work.
- The last tab is remembered per machine (`progress.tab`, `settings.section`; not
  exported).

**Palette matching.**
- `paletteFilter` normalises both the item value and the query before cmdk scores them:
  lower case, accents removed, every run of punctuation turned into one space.
- Every command is found by its visible label, in English and Spanish, with or without
  punctuation or accents.

## Consequences
- No new dependency, plugin or capability.
- The window bounds cost one settings write per settled move.
- A window placed partly off-screen on purpose is moved fully onto its monitor at the
  next start.
