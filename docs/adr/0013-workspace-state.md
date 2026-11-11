# 13. Workspace state and persistence

Date: 2026-11-11 · Status: proposed

## Context
The workspace is where users spend their time:
- the problem pane on the left;
- the editor top right;
- tests and results bottom right.

It must never lose work, must feel instant, and must keep the library route light. This
records where each piece of workspace state lives, when it is saved, and the rules
behind the editorial lock, history and shortcuts. It builds on ADR 0010 (stores),
0011 (commands) and 0012 (UI architecture).

## Decision

### Where state lives
| State | Where | Saved |
| --- | --- | --- |
| Code per problem and language | user.db `drafts` | Autosave 600 ms after the last edit; flushed at once before Run/Submit, before switching language, and when leaving the problem |
| Undo history, selection | In memory: one CodeMirror `EditorState` per document key while the workspace is open | Not persisted |
| Snapshots | user.db `snapshots` (ADR 0010), now with the run's **verdict** (migration 0002) | On every Run and Submit; restores keep the replaced draft as a snapshot |
| Notes | user.db `notes`, one Markdown body per problem | Autosave 500 ms after the last edit; flushed when leaving |
| Hint unlocks | user.db `hint_unlocks` | Immediately, one tier at a time and in order |
| Pane sizes | Setting `workspace.layout` (`{main, right}`, percentages by panel id) | 400 ms after a resize; localStorage copy for the first paint |
| Editor prefs | Settings `editor.keymap` (default \| vscode \| vim), `editor.font_size`, `editor.language` (last used) | On change; localStorage copy for the first paint |
| Custom tests | React state, per open problem | Not persisted (part 2b may add it) |

user.db is always the source of truth. localStorage copies exist only so the first paint
has the right sizes, theme and editor setup.

### Opening a problem
- **Starting language:** the language of the problem's most recently edited draft
  (`last_language`), else the last language used, else Python.
- **Loading:** the draft, else the starter stub.
- **Language switching:** the editor keeps showing the current document until the next
  language's text has loaded. The UI never pairs one language's key with another's text,
  and it never unmounts the editor (that would lose undo history).
- **Runtimes:** languages whose runtime is missing are listed but disabled, with the
  reason.

### Judging
- **Run** judges the public tests plus custom tests.
- **Custom tests** are validated as you type against the signature types (`int`,
  `float`, `string`, `bool`, nested `[]`). Run is blocked while one is invalid. kb-app
  still checks them and computes their expected outputs with the reference solution
  (ADR 0011).
- **Run and Submit are disabled while a run is active,** and the shortcuts are ignored,
  so `judge_busy` does not appear in normal use.
- **Phases** shown from `JudgeEvent`: "Compiling…", "Running N tests…", "Checked k/N".
- **Accepted:** a brief success treatment (about 1 s, no modal), turned off under
  `prefers-reduced-motion`.

### Editorial lock (kb-app)
- `App::editorial` unlocks once the problem is solved, or after **3** non-Accepted
  submissions (`EDITORIAL_FAILURES`). It returns the reason and the count, so the UI
  explains it ("1/3").
- `get_problem` no longer carries the editorial, so the rule lives in one place.
- Hidden tests are public by design (ADR 0005). The lock is a learning aid, not a
  security boundary.

### History
- Snapshots show a verdict badge (migration 0002 adds `snapshots.verdict`; restores and
  older rows have none).
- **Comparing:**
  - two selected snapshots are compared with `@codemirror/merge`, older on the left;
  - one selected snapshot is compared with the editor's current code.
- **Restore** replaces the draft and reloads the editor. The replaced draft becomes a
  snapshot (ADR 0010).

### Editor
- CodeMirror 6 with no wrapper library (`workspace/editor/CodeEditor.tsx`).
  - Compartments swap the language, keymap, font size and read-only state.
  - Run and Submit keys are bound at `Prec.highest`, so Ctrl+Enter submits instead of
    inserting a line.
- **One highlighter** (`workspace/editor/syntax.ts`) emits `tok-*` classes:
  - the editor uses them;
  - the static highlighting of Markdown code fences uses them;
  - their colors are the `--syntax-*` design tokens, checked by the WCAG AA contrast
    test in both themes.
- The editor theme uses CSS variables, so light and dark need no rebuild.
- **Keymaps:** default, VS Code (`@replit/codemirror-vscode-keymap`) and Vim
  (`@replit/codemirror-vim`).

### Shortcuts and focus mode
- Ctrl+' runs, Ctrl+Enter submits, Ctrl+Shift+F toggles focus mode, and Esc leaves it.
  - Esc only works outside the editor, because Vim uses Escape.
- Focus mode hides the nav rail, the shell top bar and the problem pane.
- Leaving the workspace leaves focus mode.

### Loading cost
- The workspace route is lazy (`lazyRouteComponent`). CodeMirror, KaTeX and the Markdown
  stack are in its chunk, not the main one.
- Each grammar (Python, TypeScript/JavaScript, PHP, Rust, Markdown) is a further
  on-demand chunk. PHP's grammar embeds HTML, CSS and JS and is the largest.
- Main chunk is about 640 kB; the workspace chunk is about 770 kB.

### Panes
- react-resizable-panels v4 through shadcn `resizable`.
- The problem and results panes are collapsible, from toolbar buttons or by dragging.
- In jsdom every element is a 0×0 box, so the library takes every pointer-down as a
  separator grab. Component tests therefore replace the panels with plain containers
  (`src/test/resizableStub.tsx`). The real panels are checked by hand.

## Consequences
- **No lost work:** drafts and notes are saved within a second of the last keystroke and
  at every transition. A crash loses at most that window.
- **Language servers (part 2b)** attach to the same `CodeEditor` through another
  compartment.
- **Custom tests are not persisted yet.** Adding them means another user.db migration.
