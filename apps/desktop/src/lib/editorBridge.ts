// The workspace editor, as seen from outside the workspace (expansion M4): the command
// palette inserts snippets through it. The workspace registers its view and language
// while it is open; nothing else holds the view.
import type { EditorView } from "@codemirror/view";
import type { Language } from "@/bindings";

type Active = { view: EditorView; language: Language };
let active: Active | null = null;
const listeners = new Set<() => void>();

export function setActiveEditor(next: Active | null) {
  active = next;
  for (const l of listeners) l();
}

export function activeEditor(): Active | null {
  return active;
}

export function onActiveEditor(listener: () => void): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

/** Insert text at the cursor (replacing the selection) and focus the editor. */
export function insertIntoEditor(text: string): boolean {
  const a = active;
  if (!a || a.view.state.readOnly) return false;
  const { from, to } = a.view.state.selection.main;
  a.view.dispatch({
    changes: { from, to, insert: text },
    selection: { anchor: from + text.length },
    scrollIntoView: true,
  });
  a.view.focus();
  return true;
}
