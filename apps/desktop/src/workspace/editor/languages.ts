// Editor languages load on demand (ADR 0013): each grammar is its own chunk, so opening
// a Python problem never parses the PHP grammar (which embeds HTML, CSS and JS).
import { EditorState, type Extension } from "@codemirror/state";
import { EditorView } from "@codemirror/view";
import { indentUnit, type Language as CmLanguage } from "@codemirror/language";
import type { Language } from "@/bindings";

export type EditorLang = Language | "markdown";

type Loaded = { support: Extension; language: CmLanguage };

const loaders: Record<EditorLang, () => Promise<Loaded>> = {
  python: () =>
    import("@codemirror/lang-python").then((m) => ({ support: m.python(), language: m.pythonLanguage })),
  typescript: () =>
    import("@codemirror/lang-javascript").then((m) => ({
      support: m.javascript({ typescript: true }),
      language: m.typescriptLanguage,
    })),
  javascript: () =>
    import("@codemirror/lang-javascript").then((m) => ({
      support: m.javascript(),
      language: m.javascriptLanguage,
    })),
  php: () => import("@codemirror/lang-php").then((m) => ({ support: m.php(), language: m.phpLanguage })),
  rust: () => import("@codemirror/lang-rust").then((m) => ({ support: m.rust(), language: m.rustLanguage })),
  markdown: () =>
    import("@codemirror/lang-markdown").then((m) => ({
      support: m.markdown(),
      language: m.markdownLanguage,
    })),
};

const INDENT: Record<EditorLang, string> = {
  python: "    ",
  typescript: "  ",
  javascript: "  ",
  php: "    ",
  rust: "    ",
  markdown: "  ",
};

const cache = new Map<EditorLang, Promise<Loaded>>();

function load(lang: EditorLang): Promise<Loaded> {
  let p = cache.get(lang);
  if (!p) {
    p = loaders[lang]();
    cache.set(lang, p);
  }
  return p;
}

/** The editor extension for `lang` (grammar, wrapping for Markdown). */
export async function loadLanguageExtension(lang: EditorLang): Promise<Extension> {
  const { support } = await load(lang);
  return [support, lang === "markdown" ? EditorView.lineWrapping : []];
}

/** Indentation for `lang`: the user's tab width (`editor.tab_width`), or 0 for the
 * language's own. */
export function indentExtension(lang: EditorLang, tabWidth = 0): Extension {
  const unit = tabWidth > 0 ? " ".repeat(tabWidth) : INDENT[lang];
  return [indentUnit.of(unit), EditorState.tabSize.of(unit.length)];
}

/** Map a Markdown code-fence language to an editor language. */
export function fenceLanguage(fence: string | undefined): EditorLang | null {
  switch (fence?.toLowerCase()) {
    case "python":
    case "py":
      return "python";
    case "typescript":
    case "ts":
      return "typescript";
    case "javascript":
    case "js":
      return "javascript";
    case "php":
      return "php";
    case "rust":
    case "rs":
      return "rust";
    default:
      return null;
  }
}

/** The Lezer parser for a fence language (loaded on demand), or null if unknown. */
export async function loadFenceParser(fence: string | undefined) {
  const lang = fenceLanguage(fence);
  return lang ? (await load(lang)).language.parser : null;
}
