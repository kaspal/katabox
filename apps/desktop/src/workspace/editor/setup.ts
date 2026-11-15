// CodeMirror building blocks: languages, theme (from the design tokens), keymaps.
import type { Extension } from "@codemirror/state";
import { EditorView, keymap } from "@codemirror/view";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import { syntaxHighlighting } from "@codemirror/language";
import { closeBrackets, closeBracketsKeymap } from "@codemirror/autocomplete";
import { highlightSelectionMatches, searchKeymap } from "@codemirror/search";
import { emacs } from "@replit/codemirror-emacs";
import { vim } from "@replit/codemirror-vim";
import { vscodeKeymap } from "@replit/codemirror-vscode-keymap";
import type { KeymapPref } from "@/lib/settings";
import { highlighter } from "@/workspace/editor/syntax";

export type { EditorLang } from "@/workspace/editor/languages";

export function keymapExtension(pref: KeymapPref): Extension {
  const base = [...closeBracketsKeymap, ...historyKeymap, ...searchKeymap, indentWithTab];
  switch (pref) {
    case "vscode":
      return keymap.of([...vscodeKeymap, ...base]);
    case "vim":
      // vim() must come before other keymaps.
      return [vim(), keymap.of([...defaultKeymap, ...base])];
    case "emacs":
      // Like vim(), emacs() must come before other keymaps.
      return [emacs(), keymap.of([...defaultKeymap, ...base])];
    default:
      return keymap.of([...defaultKeymap, ...base]);
  }
}

/** Theme from the design tokens: CSS variables, so light and dark need no rebuild. */
export function themeExtension(fontSize: number, ligatures = false): Extension {
  // JetBrains Mono draws `->`, `!=`, … as single glyphs; off unless the user opts in.
  // Also for code in tooltips (server docs), which live outside the scroller.
  const ligatureStyle = {
    fontVariantLigatures: ligatures ? "normal" : "none",
    fontFeatureSettings: ligatures ? "normal" : '"liga" 0, "calt" 0',
  };
  return EditorView.theme({
    "&": {
      height: "100%",
      fontSize: `${fontSize}px`,
      backgroundColor: "var(--card)",
      color: "var(--syntax-variable)",
    },
    ".cm-scroller": {
      fontFamily: '"JetBrains Mono Variable", ui-monospace, "Cascadia Code", monospace',
      lineHeight: "1.6",
      ...ligatureStyle,
    },
    ".cm-tooltip": {
      backgroundColor: "var(--popover)",
      color: "var(--popover-foreground)",
      border: "1px solid var(--border)",
      borderRadius: "6px",
    },
    ".cm-tooltip code, .cm-tooltip pre": ligatureStyle,
    ".cm-tooltip.cm-tooltip-autocomplete > ul > li[aria-selected]": {
      backgroundColor: "var(--accent)",
      color: "var(--accent-foreground)",
    },
    // Server docs can be long (rust-analyzer's std docs): bounded, scrollable, and long
    // signatures wrap instead of running off the window.
    ".cm-lsp-documentation, .cm-lsp-hover-tooltip": {
      padding: "4px 8px",
      maxWidth: "min(36rem, 80vw)",
      maxHeight: "18rem",
      overflowY: "auto",
      overflowWrap: "anywhere",
    },
    // The signature tooltip scrolls itself; its documentation must not scroll too.
    ".cm-lsp-signature-tooltip .cm-lsp-documentation": { maxHeight: "none", overflowY: "visible" },
    ".cm-lsp-documentation pre, .cm-lsp-hover-tooltip pre": { whiteSpace: "pre-wrap", margin: "2px 0" },
    ".cm-lsp-documentation p, .cm-lsp-hover-tooltip p": { margin: "4px 0" },
    ".cm-diagnostic-error": { borderLeft: "3px solid var(--destructive)" },
    ".cm-diagnostic-warning": { borderLeft: "3px solid var(--difficulty-medium)" },
    ".cm-content": { caretColor: "var(--foreground)" },
    ".cm-cursor, .cm-dropCursor": { borderLeftColor: "var(--foreground)" },
    "&.cm-focused": { outline: "none" },
    ".cm-gutters": {
      backgroundColor: "var(--card)",
      color: "var(--muted-foreground)",
      border: "none",
    },
    ".cm-activeLine": { backgroundColor: "color-mix(in srgb, var(--accent) 55%, transparent)" },
    ".cm-activeLineGutter": { backgroundColor: "transparent", color: "var(--foreground)" },
    "&.cm-focused > .cm-scroller > .cm-selectionLayer .cm-selectionBackground, .cm-selectionBackground, ::selection":
      { backgroundColor: "color-mix(in srgb, var(--primary) 28%, transparent)" },
    ".cm-selectionMatch": { backgroundColor: "color-mix(in srgb, var(--primary) 14%, transparent)" },
    ".cm-matchingBracket": {
      backgroundColor: "color-mix(in srgb, var(--primary) 18%, transparent)",
      outline: "1px solid color-mix(in srgb, var(--primary) 50%, transparent)",
    },
    ".cm-panels": { backgroundColor: "var(--popover)", color: "var(--popover-foreground)" },
    ".cm-searchMatch": { backgroundColor: "color-mix(in srgb, var(--difficulty-medium) 30%, transparent)" },
    ".cm-fat-cursor": { backgroundColor: "color-mix(in srgb, var(--foreground) 60%, transparent)" },
  });
}

export const baseExtensions: Extension = [
  history(),
  closeBrackets(),
  highlightSelectionMatches(),
  syntaxHighlighting(highlighter),
];
