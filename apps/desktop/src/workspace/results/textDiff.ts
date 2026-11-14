// Where two program outputs first differ (ADR 0028), by the same rules as the judge's text
// checkers (crates/kb-judge/src/checker.rs): `tokens` (ASCII whitespace), `tokens-float:<eps>`
// (numbers within a tolerance) or `lines` (trailing spaces and tabs and trailing empty
// lines ignored). CRLF is LF everywhere. Pure, for the failure view's highlight.

/** A span on one line of a text: `line` is 0-based, `start`/`end` are string offsets. */
export type Span = { line: number; start: number; end: number };

export type TextDiff = {
  /** The first differing token or line in each text; `null` when that text ended first. */
  expected: Span | null;
  actual: Span | null;
};

export function normalizeText(s: string): string {
  return s.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
}

type Token = Span & { text: string };

function tokensOf(text: string): Token[] {
  const out: Token[] = [];
  normalizeText(text)
    .split("\n")
    .forEach((line, l) => {
      const re = /[^ \t\n\f\v]+/g; // ASCII whitespace separates tokens, as in Rust
      for (let m = re.exec(line); m !== null; m = re.exec(line)) {
        out.push({ line: l, start: m.index, end: m.index + m[0].length, text: m[0] });
      }
    });
  return out;
}

/** Lines without trailing spaces and tabs, without trailing empty lines. */
export function linesOf(text: string): string[] {
  const out = normalizeText(text)
    .split("\n")
    .map((l) => l.replace(/[ \t]+$/, ""));
  while (out.length > 0 && out[out.length - 1] === "") out.pop();
  return out;
}

function numberOf(tok: string): number | null {
  if (!/^[+-]?(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?$/.test(tok)) return null;
  const x = Number(tok);
  return Number.isFinite(x) ? x : null;
}

/** The tolerance of a `tokens-float:<eps>` checker, else null. */
export function tolerance(checker: string): number | null {
  const m = /^tokens-float:(.+)$/.exec(checker);
  if (!m) return null;
  const eps = Number(m[1]);
  return Number.isFinite(eps) && eps > 0 ? eps : null;
}

/** The first difference between `expected` and `actual` under `checker`, or null if the
 * checker would accept them. */
export function textDiff(expected: string, actual: string, checker: string): TextDiff | null {
  if (checker === "lines") {
    const e = linesOf(expected);
    const a = linesOf(actual);
    const n = Math.max(e.length, a.length);
    for (let i = 0; i < n; i++) {
      if (e[i] !== a[i]) {
        return {
          expected: i < e.length ? { line: i, start: 0, end: e[i].length } : null,
          actual: i < a.length ? { line: i, start: 0, end: a[i].length } : null,
        };
      }
    }
    return null;
  }
  const eps = tolerance(checker);
  const e = tokensOf(expected);
  const a = tokensOf(actual);
  const n = Math.max(e.length, a.length);
  for (let i = 0; i < n; i++) {
    const x = e[i];
    const y = a[i];
    if (x && y) {
      if (x.text === y.text) continue;
      if (eps !== null) {
        const p = numberOf(x.text);
        const q = numberOf(y.text);
        if (p !== null && q !== null) {
          const diff = Math.abs(p - q);
          if (diff <= eps || diff <= eps * Math.max(Math.abs(p), Math.abs(q))) continue;
        }
      }
    }
    const span = (t: Token | undefined): Span | null => (t ? { line: t.line, start: t.start, end: t.end } : null);
    return { expected: span(x), actual: span(y) };
  }
  return null;
}

/** The first line of a text, with "…" when there is more. */
export function firstLine(text: string): string {
  const lines = normalizeText(text).replace(/\n+$/, "").split("\n");
  return lines.length > 1 ? `${lines[0]} …` : lines[0];
}
