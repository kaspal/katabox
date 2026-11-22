// `/snip` in the editor (expansion M4): type `/snip` and pick one of your snippets for
// this language; the `/snip…` text is replaced by the snippet's body.
import { autocompletion, type CompletionContext, type CompletionResult } from "@codemirror/autocomplete";
import { EditorState, type Extension } from "@codemirror/state";
import type { SnippetRow } from "@/bindings";

/** The completion source on its own (tested without an editor). */
export function snippetSource(snippets: readonly SnippetRow[]) {
  return (ctx: CompletionContext): CompletionResult | null => {
    const m = ctx.matchBefore(/\/snip(?:\s[\w\- ]*)?$/);
    if (!m || snippets.length === 0) return null;
    return {
      from: m.from,
      options: snippets.map((s) => ({
        label: `/snip ${s.name}`,
        detail: s.tags.join(", "),
        type: "text",
        apply: s.body,
      })),
      validFor: /^\/snip(?:\s[\w\- ]*)?$/,
    };
  };
}

/** The extension for one language's snippets (none: no extension). */
export function snippetCompletion(snippets: readonly SnippetRow[]): Extension {
  if (snippets.length === 0) return [];
  return [EditorState.languageData.of(() => [{ autocomplete: snippetSource(snippets) }]), autocompletion()];
}
