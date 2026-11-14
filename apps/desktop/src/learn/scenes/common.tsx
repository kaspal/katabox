// Shared scene parts (ADR 0029): container width, role colours, the index row, pointers,
// the accessible summary and the text fallback. Scenes are SVG; items are keyed by their
// stable id, so a value that moves slides there (equal values included).
import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { motion, type Transition } from "motion/react";
import { useReducedMotion } from "../engine/media";
import type { TFunction } from "i18next";
import { GROWTH_CLASSES } from "../engine/growth";
import type { NoteKey, Pointer, Role, Step, TracerDef } from "../engine/types";

export type SceneProps = {
  step: Step;
  /** Jump to the new state with no animation (timeline drag, reduced motion). */
  instant: boolean;
  /** The topic: legend names, counters and input shape. */
  def: TracerDef;
  /** Upper bound for bar heights (the topic's max value). */
  maxValue: number;
};

const FALLBACK_WIDTH = 640;

/** The element's width, following resizes (a fixed fallback where layout is unknown). */
export function useWidth<T extends HTMLElement>() {
  const ref = useRef<T | null>(null);
  const [width, setWidth] = useState(FALLBACK_WIDTH);
  useLayoutEffect(() => {
    const w = ref.current?.getBoundingClientRect().width;
    if (w) setWidth(w);
  }, []);
  useEffect(() => {
    const el = ref.current;
    if (!el || typeof ResizeObserver === "undefined") return;
    const ro = new ResizeObserver((entries) => {
      const w = entries[0]?.contentRect.width;
      if (w) setWidth(w);
    });
    ro.observe(el);
    return () => ro.disconnect();
  }, []);
  return [ref, width] as const;
}

export const roleAt = (step: Step, pos: number): Role => step.roles[pos] ?? "idle";
export const roleColor = (role: Role) => `var(--viz-${role})`;

/** Spring for moves, a short ease for lift and colour; nothing when `instant` or reduced motion. */
export function useTransitions(instant: boolean) {
  const reduce = useReducedMotion();
  const none: Transition = { duration: 0 };
  if (instant || reduce) return { move: none, lift: none, colors: "none" };
  return {
    move: { type: "spring", stiffness: 320, damping: 32, mass: 0.9 } as Transition,
    lift: { duration: 0.22, ease: "easeOut" } as Transition,
    /** CSS transition for fills and strokes (motion cannot tween `var(--…)` colours). */
    colors: "fill 220ms ease, stroke 220ms ease",
  };
}

/** Outline that tells roles apart without colour: dashed while compared, solid while moving. */
export function outline(role: Role): { stroke?: string; strokeWidth?: number; strokeDasharray?: string } {
  if (role === "compare") return { stroke: "var(--foreground)", strokeWidth: 2, strokeDasharray: "5 3" };
  if (role === "swap") return { stroke: "var(--foreground)", strokeWidth: 2 };
  return {};
}

export function roleName(t: TFunction, role: Role, def: Pick<TracerDef, "roleLabels">): string {
  return t(`learn.roles.${def.roleLabels?.[role] ?? role}`);
}

/** Position numbers under the scene. */
export function IndexRow({ n, slot, y }: { n: number; slot: number; y: number }) {
  return (
    <g aria-hidden className="fill-muted-foreground font-mono text-[11px]">
      {Array.from({ length: n }, (_, k) => (
        <text key={k} x={(k + 0.5) * slot} y={y} textAnchor="middle">
          {k}
        </text>
      ))}
    </g>
  );
}

/** Pointer labels (pivot, i, j…) that glide between positions; shared positions fan out. */
export function Pointers({
  pointers,
  slot,
  y,
  transition,
}: {
  pointers: Pointer[];
  slot: number;
  y: number;
  transition: Transition;
}) {
  // Labels sharing a position fan out side by side, each as wide as its text (one-letter
  // labels take 26 px, as before), centred on the position.
  const width = (p: Pointer) => Math.max(18, p.label.length * 7.5) + 8;
  const offset = new Map<Pointer, number>();
  const groups = new Map<number, Pointer[]>();
  for (const p of pointers) groups.set(p.at, [...(groups.get(p.at) ?? []), p]);
  for (const group of groups.values()) {
    let x = -group.reduce((s, p) => s + width(p), 0) / 2;
    for (const p of group) {
      offset.set(p, x + width(p) / 2);
      x += width(p);
    }
  }
  return (
    <g aria-hidden>
      {pointers.map((p) => {
        const x = (p.at + 0.5) * slot + offset.get(p)!;
        return (
          <motion.g
            key={p.label}
            initial={false}
            animate={{ x }}
            transition={transition}
            style={{ color: `var(--viz-ptr-${p.tone})` }}
            data-pointer={p.label}
          >
            <path d={`M0 ${y} l5 8 h-10 z`} fill="currentColor" />
            <text y={y + 22} textAnchor="middle" fill="currentColor" className="font-mono text-xs font-semibold">
              {p.label}
            </text>
          </motion.g>
        );
      })}
    </g>
  );
}

/** One sentence for the scene's `aria-label`. */
export function sceneSummary(t: TFunction, step: Step): string {
  const hidden = new Set(step.hidden ?? []);
  const values = step.items.map((it) => (hidden.has(it.id) ? "_" : it.value)).join(", ");
  const parts: string[] = [];
  if (step.items.length > 0) parts.push(t("learn.stage.summary", { values }));
  if (step.capacity !== undefined) {
    parts.push(t("learn.stage.summary_capacity", { capacity: step.capacity, size: step.items.length }));
  }
  if (step.pegs) {
    const pegs = step.pegs.map((d, k) => `${k + 1}: ${d.join(" ") || "—"}`).join("; ");
    parts.push(t("learn.stage.summary_pegs", { pegs }));
  }
  if (step.bits) {
    parts.push(t("learn.stage.summary_bits", { rows: step.bits.map((b) => `${b.label} = ${b.value}`).join("; ") }));
  }
  if (step.chart) parts.push(t("learn.stage.summary_growth", { count: step.chart.shown, n: step.chart.n }));
  if (step.list) {
    parts.push(t("learn.stage.summary_list", { values: step.list.nodes.map((n) => n.value).join(" → ") || "—" }));
  }
  if (step.slots) {
    const values = step.slots.cells.map((c) => (c ? c.value : "_")).join(", ") || "—";
    parts.push(t("learn.stage.fallback_row", { label: t(`learn.stage.rows.${step.slots.label}`), values }) + ".");
  }
  if (step.forest?.length) {
    const nodes = step.forest
      .map((n) => {
        const parent = n.parent === null ? null : step.forest!.find((p) => p.id === n.parent);
        return parent ? `${n.text} (${parent.text})` : n.text;
      })
      .join(", ");
    parts.push(t("learn.stage.summary_tree", { nodes }));
  }
  if (step.hash) {
    const buckets = step.hash.buckets.map((b, k) => `${k}: ${b.map((it) => (it.value === -2 ? "✕" : it.value)).join(" ") || "—"}`).join("; ");
    parts.push(t("learn.stage.summary_hash", { buckets }));
  }
  if (step.graph) {
    const g = step.graph;
    const nodes = g.nodes.map((v) => (v.note ? `${v.id} (${v.note.startsWith("@") ? t(`learn.stage.notes.${v.note.slice(1) as NoteKey}`) : v.note})` : String(v.id))).join(", ");
    const edges = g.edges.map((e) => `${e.u}${g.directed ? "→" : "–"}${e.v}${g.weighted ? ` (${e.label ?? e.w})` : ""}`).join(", ") || "—";
    parts.push(t("learn.stage.summary_graph", { n: g.n, nodes, edges }));
  }
  if (step.grid) {
    const g = step.grid;
    const map = Array.from({ length: g.rows }, (_, r) =>
      Array.from({ length: g.cols }, (_, c) => {
        const k = r * g.cols + c;
        const v = g.cells[k]!;
        if (g.start && g.start[0] === r && g.start[1] === c) return "S";
        if (g.goal && g.goal[0] === r && g.goal[1] === c) return "G";
        return v === 1 ? "#" : g.path.includes(k) ? "*" : v >= 2 ? String(v) : ".";
      }).join(""),
    ).join(" / ");
    parts.push(t("learn.stage.summary_grid", { rows: g.rows, cols: g.cols, map }));
  }
  if (step.dp) {
    const d = step.dp;
    const rows = d.cells
      .map((row, r) => `${d.rowLabels?.[r] ?? ""}${d.rowLabels ? ": " : ""}${row.map((v) => (v === null ? "_" : v >= 1e6 ? "∞" : v)).join(" ")}`)
      .join("; ");
    parts.push(t("learn.stage.fallback_row", { label: t(`learn.stage.rows.${d.label}`), values: rows }) + ".");
  }
  if (step.timeline) {
    const bars = step.timeline.bars.map((b) => `${b.start}–${b.end}`).join(", ") || "—";
    const output = (step.timeline.output ?? []).map((b) => `${b.start}–${b.end}`).join(", ") || "—";
    parts.push(t("learn.stage.summary_timeline", { bars, output }));
  }
  if (step.board) {
    const b = step.board;
    const cells = Array.from({ length: b.rows }, (_, r) => b.cells.slice(r * b.cols, (r + 1) * b.cols).map((c) => c || ".").join("")).join(" / ");
    parts.push(t("learn.stage.summary_board", { rows: b.rows, cols: b.cols, cells }));
  }
  if (step.strings) {
    const rows = step.strings.rows.map((row) => `${t(`learn.stage.rows.${row.label}`)}: ${row.cells.map((c) => (c === null ? "_" : c)).join(row.kind === "letters" ? "" : " ")}`).join("; ");
    parts.push(t("learn.stage.summary_strings", { rows }));
  }
  if (step.decision) {
    const line = step.decision.current == null ? null : step.decision.lines[step.decision.current];
    if (line) parts.push(t("learn.stage.fallback_row", { label: t("learn.stage.rows.decisions"), values: line.text }) + ".");
  }
  if (step.plane) {
    const points = step.plane.points.map((p) => `${p.label ?? p.id} (${p.x}, ${p.y})`).join(", ");
    parts.push(t("learn.stage.summary_plane", { points }));
  }
  if (step.bitArray) parts.push(t("learn.stage.summary_bits_array", { bits: step.bitArray.bits.join("") }));
  if (step.table) {
    const rows = step.table.rows.map((r) => `${r.key}: ${r.value}`).join(", ") || "—";
    parts.push(t("learn.stage.fallback_row", { label: t(`learn.stage.rows.${step.table.label}`), values: rows }) + ".");
  }
  if (step.stack?.length) parts.push(t("learn.stage.summary_stack", { top: step.stack[step.stack.length - 1]!.text }));
  const at = (role: Role) =>
    Object.entries(step.roles)
      .filter(([, r]) => r === role)
      .map(([p]) => Number(p));
  const compared = at("compare");
  if (compared.length) parts.push(t("learn.stage.summary_compare", { positions: compared.join(", ") }));
  const moving = at("swap");
  if (moving.length) parts.push(t("learn.stage.summary_move", { positions: moving.join(", ") }));
  const pivot = at("pivot")[0];
  if (pivot !== undefined) {
    parts.push(t("learn.stage.summary_pivot", { value: step.items[pivot]!.value, at: pivot }));
  }
  if (step.lifted !== null) {
    parts.push(t("learn.stage.summary_key", { value: step.items[step.lifted]!.value, at: step.lifted }));
  }
  if (step.range) parts.push(t("learn.stage.summary_range", { lo: step.range[0], hi: step.range[1] }));
  if (parts.length === 0) parts.push(t("learn.stage.summary_empty"));
  return parts.join(" ");
}

/** The scene as text, for screen readers (visually hidden). */
export function SceneFallback({
  t,
  step,
  def,
}: {
  t: TFunction;
  step: Step;
  def: Pick<TracerDef, "roleLabels">;
}) {
  const list = (xs: readonly { value: number }[]) => xs.map((x) => x.value).join(", ");
  return (
    <div className="sr-only" data-testid="scene-fallback">
      {step.items.length > 0 && (
        <>
          <p>{t("learn.stage.fallback")}</p>
          <ol>
            {step.items.map((it, p) => (
              <li key={it.id}>
                {t("learn.stage.fallback_item", {
                  at: p,
                  value: it.value,
                  role: roleName(t, roleAt(step, p), def),
                })}
              </li>
            ))}
          </ol>
        </>
      )}
      {step.aux && (
        <p>
          {t("learn.stage.fallback_row", {
            label: t(`learn.stage.rows.${step.aux.label}`),
            values: step.aux.cells.map((c) => (c ? c.value : "_")).join(", "),
          })}
        </p>
      )}
      {step.buckets && (
        <ul>
          {step.buckets.map((b) => (
            <li key={b.label}>{t("learn.stage.fallback_bucket", { label: b.label, values: list(b.items) || "—" })}</li>
          ))}
        </ul>
      )}
      {step.pegs && (
        <ul>
          {step.pegs.map((disks, k) => (
            <li key={k}>{t("learn.stage.fallback_peg", { peg: k + 1, disks: disks.join(", ") || "—" })}</li>
          ))}
        </ul>
      )}
      {step.bits && (
        <ul>
          {step.bits.map((b, k) => (
            <li key={k}>
              {t("learn.stage.fallback_bits", { label: b.label, value: b.value, bits: b.value.toString(2).padStart(8, "0") })}
            </li>
          ))}
        </ul>
      )}
      {step.chart && (
        <ul>
          {GROWTH_CLASSES.slice(0, step.chart.shown).map((g) => (
            <li key={g.label}>
              {t("learn.stage.fallback_growth", { label: g.label, count: g.f(step.chart!.n), n: step.chart!.n })}
            </li>
          ))}
        </ul>
      )}
      {step.stack && (
        <p>
          {t("learn.stage.fallback_stack", {
            entries: [...step.stack].reverse().map((e) => e.text).join("; ") || "—",
          })}
        </p>
      )}
    </div>
  );
}
