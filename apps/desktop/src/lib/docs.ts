// The user guide, bundled (expansion M4): the Help page renders it and "Search
// everything" searches its sections. Loaded on demand (dynamic import), never fetched.
import en from "../../../../docs/user-guide.md?raw";
import es from "../../../../docs/user-guide.es.md?raw";

export type DocSection = { id: string; title: string; body: string };

/** A stable anchor id from a heading: lower case, no accents, dashes. */
export function slug(text: string): string {
  return text
    .normalize("NFKD")
    .replace(/[̀-ͯ]/g, "")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

/** The guide's `##` sections (the title above them is left out). */
export function docSections(lang: string): DocSection[] {
  const text = lang === "es" ? es : en;
  const out: DocSection[] = [];
  let cur: DocSection | null = null;
  for (const line of text.split(/\r?\n/)) {
    const h = /^## (.+)$/.exec(line);
    if (h) {
      cur = { id: slug(h[1]), title: h[1].trim(), body: "" };
      out.push(cur);
    } else if (cur) {
      cur.body += line + "\n";
    }
  }
  return out;
}
