// A CodeMirror 6 editor for React (ADR 0013). One EditorState is kept per `docKey`
// while mounted, so switching language and back keeps undo history and selection.
import { useEffect, useRef } from "react";
import { Compartment, EditorState, Prec, type Extension } from "@codemirror/state";
import {
  EditorView,
  drawSelection,
  highlightActiveLine,
  highlightActiveLineGutter,
  keymap,
  lineNumbers,
} from "@codemirror/view";
import { bracketMatching, indentOnInput } from "@codemirror/language";
import { forEachDiagnostic } from "@codemirror/lint";
import type { KeymapPref } from "@/lib/settings";
import { baseExtensions, keymapExtension, themeExtension } from "@/workspace/editor/setup";
import { loadLanguageExtension, type EditorLang } from "@/workspace/editor/languages";

/** Shown in the status line under the editor. */
export type EditorStatus = { line: number; col: number; errors: number; warnings: number };

function statusOf(state: EditorState): EditorStatus {
  const head = state.selection.main.head;
  const line = state.doc.lineAt(head);
  let errors = 0;
  let warnings = 0;
  forEachDiagnostic(state, (d) => {
    if (d.severity === "error") errors++;
    else if (d.severity === "warning") warnings++;
  });
  return { line: line.number, col: head - line.from + 1, errors, warnings };
}

export type CodeEditorProps = {
  /** Identity of the document (e.g. `two-sum:python:0`). A new key starts a new state. */
  docKey: string;
  /** Initial text for a key seen for the first time. */
  value: string;
  lang: EditorLang;
  keymap?: KeymapPref;
  fontSize?: number;
  readOnly?: boolean;
  lineNumbers?: boolean;
  ariaLabel: string;
  onChange?: (value: string) => void;
  /** Ctrl+' inside the editor. */
  onRun?: () => void;
  /** Ctrl+Enter inside the editor. */
  onSubmit?: () => void;
  /** Language-server support (completion, hover, diagnostics, …); none when null. */
  lspExtension?: Extension | null;
  /** The debugger's breakpoint gutter and paused line (ADR 0022); none when null. */
  debugExtension?: Extension | null;
  /** Font ligatures (off by default). */
  ligatures?: boolean;
  /** Cursor position and diagnostics counts, after every change. */
  onStatus?: (status: EditorStatus) => void;
  /** The view, for commands such as Format. */
  onView?: (view: EditorView | null) => void;
  className?: string;
};

export function CodeEditor(props: CodeEditorProps) {
  const host = useRef<HTMLDivElement>(null);
  const view = useRef<EditorView | null>(null);
  const states = useRef(new Map<string, EditorState>());
  const currentKey = useRef<string | null>(null);
  const latest = useRef(props);
  latest.current = props;
  const compartments = useRef({
    lang: new Compartment(),
    keymap: new Compartment(),
    theme: new Compartment(),
    readOnly: new Compartment(),
    lsp: new Compartment(),
    debug: new Compartment(),
  });

  const makeState = (doc: string): EditorState => {
    const p = latest.current;
    const c = compartments.current;
    const shortcuts = Prec.highest(
      keymap.of([
        { key: "Mod-Enter", run: () => (latest.current.onSubmit?.(), true) },
        { key: "Mod-'", run: () => (latest.current.onRun?.(), true) },
      ]),
    );
    const extensions: Extension[] = [
      shortcuts,
      c.keymap.of(keymapExtension(p.keymap ?? "default")),
      p.lineNumbers === false ? [] : [lineNumbers(), highlightActiveLineGutter()],
      drawSelection(),
      highlightActiveLine(),
      indentOnInput(),
      bracketMatching(),
      baseExtensions,
      // The grammar loads on demand; see the language effect below.
      c.lang.of([]),
      c.theme.of(themeExtension(p.fontSize ?? 14, p.ligatures)),
      c.lsp.of(p.lspExtension ?? []),
      c.debug.of(p.debugExtension ?? []),
      c.readOnly.of([EditorState.readOnly.of(!!p.readOnly), EditorView.editable.of(!p.readOnly)]),
      EditorView.contentAttributes.of({ "aria-label": p.ariaLabel }),
      EditorView.updateListener.of((u) => {
        if (u.docChanged) latest.current.onChange?.(u.state.doc.toString());
        latest.current.onStatus?.(statusOf(u.state));
      }),
    ];
    return EditorState.create({ doc, extensions });
  };

  // Mount once.
  useEffect(() => {
    if (!host.current) return;
    const v = new EditorView({ parent: host.current, state: makeState(props.value) });
    view.current = v;
    currentKey.current = props.docKey;
    latest.current.onView?.(v);
    latest.current.onStatus?.(statusOf(v.state));
    return () => {
      latest.current.onView?.(null);
      v.destroy();
      view.current = null;
    };
  }, []);

  // Switch documents: keep the old state, restore or create the new one.
  useEffect(() => {
    const v = view.current;
    if (!v || currentKey.current === props.docKey) return;
    if (currentKey.current) states.current.set(currentKey.current, v.state);
    const next = states.current.get(props.docKey) ?? makeState(props.value);
    v.setState(next);
    currentKey.current = props.docKey;
    // Settings may have changed while this state was parked.
    reconfigure();
  }, [props.docKey]);

  const reconfigure = () => {
    const v = view.current;
    if (!v) return;
    const p = latest.current;
    const c = compartments.current;
    v.dispatch({
      effects: [
        c.keymap.reconfigure(keymapExtension(p.keymap ?? "default")),
        c.theme.reconfigure(themeExtension(p.fontSize ?? 14, p.ligatures)),
        c.lsp.reconfigure(p.lspExtension ?? []),
        c.debug.reconfigure(p.debugExtension ?? []),
        c.readOnly.reconfigure([
          EditorState.readOnly.of(!!p.readOnly),
          EditorView.editable.of(!p.readOnly),
        ]),
      ],
    });
  };

  useEffect(reconfigure, [
    props.keymap,
    props.fontSize,
    props.readOnly,
    props.ligatures,
    props.lspExtension,
    props.debugExtension,
  ]);

  // Load the grammar for the current language (and after switching documents).
  useEffect(() => {
    let live = true;
    void loadLanguageExtension(props.lang).then((ext) => {
      if (live && view.current) {
        view.current.dispatch({ effects: compartments.current.lang.reconfigure(ext) });
      }
    });
    return () => {
      live = false;
    };
  }, [props.lang, props.docKey]);

  return (
    <div
      ref={host}
      className={props.className ?? "h-full min-h-0 overflow-hidden"}
      data-testid="code-editor"
      data-dockey={props.docKey}
    />
  );
}
