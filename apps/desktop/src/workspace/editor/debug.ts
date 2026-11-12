// The debugger's editor decorations (ADR 0022): a breakpoint gutter (click a line to
// toggle) and the paused line. Built from plain data and swapped through a compartment,
// so the React state stays the single source of truth.
import { RangeSet, type Extension } from "@codemirror/state";
import { Decoration, EditorView, GutterMarker, gutter } from "@codemirror/view";

class BreakpointMarker extends GutterMarker {
  constructor(readonly current: boolean) {
    super();
  }
  eq(other: BreakpointMarker) {
    return other.current === this.current;
  }
  toDOM() {
    const el = document.createElement("span");
    el.className = this.current ? "cm-bp cm-bp-current" : "cm-bp";
    el.setAttribute("aria-hidden", "true");
    return el;
  }
}

class CurrentMarker extends GutterMarker {
  toDOM() {
    const el = document.createElement("span");
    el.className = "cm-debug-arrow";
    el.textContent = "▶";
    el.setAttribute("aria-hidden", "true");
    return el;
  }
}

const breakpoint = new BreakpointMarker(false);
const breakpointHere = new BreakpointMarker(true);
const arrow = new CurrentMarker();
const pausedLine = Decoration.line({ class: "cm-debug-line", attributes: { "data-debug-line": "true" } });

export type DebugDecorations = {
  /** Breakpoint line numbers (1-based). */
  breakpoints: ReadonlySet<number>;
  /** The paused line, or null. */
  current: number | null;
  /** A click in the gutter: toggle the breakpoint on `line`. */
  onToggle: (line: number) => void;
  /** Accessible name of the gutter. */
  label: string;
};

export function debugExtension(d: DebugDecorations): Extension {
  return [
    gutter({
      class: "cm-breakpoint-gutter",
      markers: (view) => {
        const marks = [];
        const lines = view.state.doc.lines;
        const all = new Set<number>(d.breakpoints);
        if (d.current !== null) all.add(d.current);
        for (const n of [...all].sort((a, b) => a - b)) {
          if (n < 1 || n > lines) continue;
          const at = view.state.doc.line(n).from;
          const marker = d.breakpoints.has(n)
            ? n === d.current
              ? breakpointHere
              : breakpoint
            : arrow;
          marks.push(marker.range(at));
        }
        return RangeSet.of(marks);
      },
      initialSpacer: () => breakpoint,
      domEventHandlers: {
        mousedown(view, block) {
          d.onToggle(view.state.doc.lineAt(block.from).number);
          return true;
        },
      },
    }),
    EditorView.decorations.of((view) => {
      if (d.current === null || d.current < 1 || d.current > view.state.doc.lines) {
        return Decoration.none;
      }
      return Decoration.set([pausedLine.range(view.state.doc.line(d.current).from)]);
    }),
    EditorView.theme({
      ".cm-breakpoint-gutter": { width: "14px", cursor: "pointer" },
      ".cm-breakpoint-gutter .cm-gutterElement": { display: "flex", alignItems: "center", justifyContent: "center" },
      ".cm-bp": {
        display: "inline-block",
        width: "9px",
        height: "9px",
        borderRadius: "50%",
        backgroundColor: "var(--destructive)",
      },
      ".cm-bp-current": { boxShadow: "0 0 0 2px var(--background), 0 0 0 3px var(--primary)" },
      ".cm-debug-arrow": { color: "var(--primary)", fontSize: "9px", lineHeight: "1" },
      ".cm-debug-line": { backgroundColor: "color-mix(in srgb, var(--primary) 18%, transparent)" },
    }),
    EditorView.contentAttributes.of({ "aria-describedby": "" }),
    EditorView.editorAttributes.of({ "aria-label": d.label }),
  ];
}
