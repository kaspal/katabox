// A read-only side-by-side diff (CodeMirror merge view): History snapshots (ADR 0013)
// and, since expansion M4, any two submissions.
import { useEffect, useRef } from "react";
import { EditorState } from "@codemirror/state";
import { EditorView } from "@codemirror/view";
import { MergeView } from "@codemirror/merge";
import type { Language } from "@/bindings";
import { baseExtensions, themeExtension } from "@/workspace/editor/setup";
import { loadLanguageExtension } from "@/workspace/editor/languages";

export function DiffView({
  a,
  b,
  language,
  testId = "history-diff",
}: {
  a: string;
  b: string;
  language: Language;
  testId?: string;
}) {
  const host = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!host.current) return;
    let view: MergeView | null = null;
    let live = true;
    const parent = host.current;
    void loadLanguageExtension(language).then((lang) => {
      if (!live) return;
      const ext = [
        baseExtensions,
        lang,
        themeExtension(13),
        EditorView.editable.of(false),
        EditorState.readOnly.of(true),
      ];
      view = new MergeView({
        a: { doc: a, extensions: ext },
        b: { doc: b, extensions: ext },
        parent,
        collapseUnchanged: { margin: 3, minSize: 6 },
      });
    });
    return () => {
      live = false;
      view?.destroy();
    };
  }, [a, b, language]);
  return <div ref={host} className="max-h-96 overflow-auto rounded-md border" data-testid={testId} />;
}
