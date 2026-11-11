// Language-server Markdown is rendered to HTML by the LSP client; the HTML is untrusted
// (ADR 0014). This allow-list sanitizer keeps formatting and code, and drops everything
// else: scripts, styles, event handlers, links' targets, images, frames, forms.

const KEEP = new Set([
  "p", "br", "hr", "code", "pre", "em", "strong", "b", "i", "s", "del", "span", "div",
  "ul", "ol", "li", "blockquote", "h1", "h2", "h3", "h4", "h5", "h6",
  "table", "thead", "tbody", "tr", "th", "td",
]);
// Elements removed with their content.
const DROP = new Set([
  "script", "style", "iframe", "object", "embed", "frame", "frameset", "link", "meta",
  "template", "svg", "math", "form", "input", "button", "textarea", "select", "img", "video",
  "audio", "source", "base",
]);

function clean(node: Node, doc: Document): Node[] {
  if (node.nodeType === Node.TEXT_NODE) return [doc.createTextNode(node.textContent ?? "")];
  if (node.nodeType !== Node.ELEMENT_NODE) return [];
  const el = node as Element;
  const tag = el.tagName.toLowerCase();
  if (DROP.has(tag)) return [];
  const children = [...el.childNodes].flatMap((c) => clean(c, doc));
  if (!KEEP.has(tag)) return children; // unwrap (e.g. <a>: keep its text, drop the link)
  const out = doc.createElement(tag);
  // Only a code-highlighting class survives; no other attribute does.
  const cls = el.getAttribute("class");
  if (cls && /^[\w\s-]+$/.test(cls) && (tag === "code" || tag === "span" || tag === "pre")) {
    out.setAttribute("class", cls);
  }
  out.append(...children);
  return [out];
}

export function sanitizeHTML(html: string): string {
  const doc = new DOMParser().parseFromString(`<body>${html}</body>`, "text/html");
  const out = document.createElement("div");
  out.append(...[...doc.body.childNodes].flatMap((n) => clean(n, document)));
  return out.innerHTML;
}
