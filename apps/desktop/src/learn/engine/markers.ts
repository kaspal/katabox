// `@step` markers (ADR 0029): samples carry `# @step <id>` / `// @step <id>` as a trailing
// comment on the line they mark. The UI shows the code without them and highlights the
// line behind the current step. Lines are never renumbered.

const MARKER = /\s*(?:#|\/\/)\s*@step\s+([a-z][a-z0-9-]*)\s*$/;

export type SampleView = {
  /** Display lines, marker comments removed (trailing spaces trimmed). */
  lines: string[];
  /** Marker id → 1-based line number. */
  lineOf: Record<string, number>;
};

export function parseSample(source: string): SampleView {
  const lines: string[] = [];
  const lineOf: Record<string, number> = {};
  const raw = source.replace(/\r\n/g, "\n").replace(/\n$/, "").split("\n");
  raw.forEach((line, k) => {
    const m = MARKER.exec(line);
    if (m) {
      lineOf[m[1]!] = k + 1;
      lines.push(line.slice(0, m.index).replace(/\s+$/, ""));
    } else {
      lines.push(line);
    }
  });
  return { lines, lineOf };
}
