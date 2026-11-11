// One syntax highlighter for the editor and for static code blocks (ADR 0013): it emits
// `tok-*` classes whose colors come from the design tokens (styles/syntax.css).
import { highlightCode, tagHighlighter, tags as t } from "@lezer/highlight";
import type { Parser } from "@lezer/common";

export const highlighter = tagHighlighter([
  { tag: t.link, class: "tok-link" },
  { tag: t.heading, class: "tok-heading" },
  { tag: t.emphasis, class: "tok-emphasis" },
  { tag: t.strong, class: "tok-strong" },
  { tag: t.keyword, class: "tok-keyword" },
  { tag: [t.atom, t.bool, t.null, t.self], class: "tok-atom" },
  { tag: t.url, class: "tok-url" },
  { tag: t.labelName, class: "tok-labelName" },
  { tag: t.literal, class: "tok-literal" },
  { tag: [t.string, t.character, t.docString], class: "tok-string" },
  { tag: t.number, class: "tok-number" },
  { tag: [t.regexp, t.escape, t.special(t.string)], class: "tok-string2" },
  { tag: t.variableName, class: "tok-variableName" },
  { tag: [t.function(t.variableName), t.function(t.propertyName)], class: "tok-function" },
  { tag: t.propertyName, class: "tok-propertyName" },
  { tag: [t.typeName, t.standard(t.typeName)], class: "tok-typeName" },
  { tag: t.namespace, class: "tok-namespace" },
  { tag: t.className, class: "tok-className" },
  { tag: t.macroName, class: "tok-macroName" },
  { tag: [t.operator, t.derefOperator, t.arithmeticOperator, t.compareOperator], class: "tok-operator" },
  { tag: [t.comment, t.lineComment, t.blockComment], class: "tok-comment" },
  { tag: t.meta, class: "tok-meta" },
  { tag: t.invalid, class: "tok-invalid" },
  { tag: t.punctuation, class: "tok-punctuation" },
]);

export type Highlighted = { text: string; className: string }[];

/** Split `code` into classed runs with `parser`. */
export function highlightWith(parser: Parser, code: string): Highlighted {
  const out: Highlighted = [];
  highlightCode(
    code,
    parser.parse(code),
    highlighter,
    (text, className) => out.push({ text, className }),
    () => out.push({ text: "\n", className: "" }),
  );
  return out;
}
