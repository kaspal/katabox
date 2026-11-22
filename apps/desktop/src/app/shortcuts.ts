// The keyboard-shortcuts dialog (release 1.0.0-beta.1): the Shell owns it; F1, the
// command palette, the top bar and Settings → About open it through this event.
const EVENT = "katabox:shortcuts";

export function openShortcuts() {
  window.dispatchEvent(new Event(EVENT));
}

export function onOpenShortcuts(handler: () => void): () => void {
  window.addEventListener(EVENT, handler);
  return () => window.removeEventListener(EVENT, handler);
}

/** The command palette (Ctrl K): the Shell owns it; the Home's search button opens it. */
const PALETTE_EVENT = "katabox:palette";

export function openPalette() {
  window.dispatchEvent(new Event(PALETTE_EVENT));
}

export function onOpenPalette(handler: () => void): () => void {
  window.addEventListener(PALETTE_EVENT, handler);
  return () => window.removeEventListener(PALETTE_EVENT, handler);
}

/** A key name that is a word is translated (`shortcuts.key.<name>`); others show as is. */
export const NAMED_KEYS = ["Space", "Home", "End", "Enter", "PageUp", "PageDown", "Esc"] as const;

type Combo = readonly string[];
export type ShortcutRow = { id: string; combos: readonly Combo[] };
export type ShortcutGroup = { id: string; rows: readonly ShortcutRow[] };

/** Every shortcut the app binds (kept in step with the handlers; a test checks the ids
 * have labels in both languages). */
export const SHORTCUTS: readonly ShortcutGroup[] = [
  {
    id: "general",
    rows: [
      { id: "palette", combos: [["Ctrl", "K"]] },
      { id: "searchAll", combos: [["Ctrl", "Shift", "P"]] },
      { id: "shortcuts", combos: [["F1"]] },
    ],
  },
  {
    id: "library",
    rows: [
      { id: "search", combos: [["/"]] },
      { id: "move", combos: [["↑"], ["↓"], ["PageUp"], ["PageDown"], ["Home"], ["End"]] },
      { id: "open", combos: [["Enter"]] },
      { id: "listOrder", combos: [["Alt", "↑"], ["Alt", "↓"]] },
    ],
  },
  {
    id: "workspace",
    rows: [
      { id: "run", combos: [["Ctrl", "'"]] },
      { id: "submit", combos: [["Ctrl", "Enter"]] },
      { id: "focus", combos: [["Ctrl", "Shift", "F"]] },
      { id: "leaveFocus", combos: [["Esc"]] },
    ],
  },
  {
    id: "editor",
    rows: [
      { id: "find", combos: [["Ctrl", "F"]] },
      { id: "undoRedo", combos: [["Ctrl", "Z"], ["Ctrl", "Y"]] },
      { id: "format", combos: [["Shift", "Alt", "F"]] },
      { id: "definition", combos: [["F12"]] },
    ],
  },
  {
    id: "debugger",
    rows: [
      { id: "breakpoint", combos: [["F9"]] },
      { id: "continue", combos: [["F5"]] },
      { id: "stepOver", combos: [["F10"]] },
      { id: "stepBack", combos: [["Shift", "F10"]] },
      { id: "stepInto", combos: [["F11"]] },
      { id: "stepOut", combos: [["Shift", "F11"]] },
      { id: "restart", combos: [["Ctrl", "Shift", "F5"]] },
      { id: "stop", combos: [["Shift", "F5"]] },
    ],
  },
  {
    id: "learn",
    rows: [
      { id: "playPause", combos: [["Space"]] },
      { id: "step", combos: [["←"], ["→"]] },
      { id: "restart", combos: [["Home"]] },
    ],
  },
];
