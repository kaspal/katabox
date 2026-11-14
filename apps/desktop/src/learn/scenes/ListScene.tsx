// Linked lists (ADR 0029, Phase 2): nodes in the tracer's display order, keyed by id, so
// an insert slides the others aside and a delete closes the gap. A link to the next node
// in the row is a straight arrow; any other link (a reversal pointing back, a cycle, a
// node skipped over) is an arc below the row. Doubly linked lists draw the back links a
// little lower. Labels (head, tail, prev, curr, slow, fast) sit under their nodes. An
// optional key → value table (LRU's map) sits on the right.
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import type { ListData, Step, TableData } from "../engine/types";
import { Pointers, SceneFallback, outline, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

const NODE_H = 40;
const TOP = 16;
const TABLE_W = 150;

function arrowHead(x: number, y: number, dir: 1 | -1) {
  return `M${x} ${y} l${-7 * dir} -4.5 v9 z`;
}

function Links({ list, at, nodeW, tr }: { list: ListData; at: (id: number) => number; nodeW: number; tr: ReturnType<typeof useTransitions> }) {
  const index = new Map(list.nodes.map((n, k) => [n.id, k]));
  const mid = TOP + NODE_H / 2;
  const paths: { key: string; d: string; head: string; back?: boolean }[] = [];
  for (const n of list.nodes) {
    const k = index.get(n.id)!;
    const to = list.next[n.id];
    if (to === undefined || to === null) continue;
    const j = index.get(to);
    if (j === undefined) continue;
    const x1 = at(n.id) + nodeW;
    const x2 = at(to);
    if (j === k + 1) {
      paths.push({ key: `n${n.id}`, d: `M${x1} ${mid - (list.prev ? 6 : 0)} H${x2 - 2}`, head: arrowHead(x2, mid - (list.prev ? 6 : 0), 1) });
    } else {
      // An arc below the row, deeper for farther targets.
      const sx = at(n.id) + nodeW / 2;
      const ex = at(to) + nodeW / 2;
      const depth = 56 + Math.abs(j - k) * 8; // below the label band
      const y0 = TOP + NODE_H;
      paths.push({
        key: `n${n.id}`,
        d: `M${sx} ${y0} C${sx} ${y0 + depth}, ${ex} ${y0 + depth}, ${ex} ${y0 + 3}`,
        head: `M${ex} ${y0 + 1} l-4.5 7 h9 z`,
      });
    }
  }
  if (list.prev) {
    for (const n of list.nodes) {
      const k = index.get(n.id)!;
      const to = list.prev[n.id];
      if (to === undefined || to === null) continue;
      const j = index.get(to);
      if (j === undefined || j !== k - 1) continue;
      const x1 = at(n.id);
      const x2 = at(to) + nodeW;
      paths.push({ key: `p${n.id}`, d: `M${x1} ${mid + 6} H${x2 + 2}`, head: arrowHead(x2, mid + 6, -1), back: true });
    }
  }
  return (
    <g aria-hidden data-testid="links">
      {paths.map((p) => (
        <g key={p.key} style={{ color: p.back ? "var(--viz-ptr-j)" : "var(--viz-edge)" }}>
          <motion.path initial={false} animate={{ d: p.d }} transition={tr.move} fill="none" stroke="currentColor" strokeWidth={1.8} />
          <motion.path initial={false} animate={{ d: p.head }} transition={tr.move} fill="currentColor" />
        </g>
      ))}
      {list.nodes
        .filter((n) => list.next[n.id] === null)
        .map((n) => (
          <text key={`null${n.id}`} x={at(n.id) + nodeW + 4} y={mid + 4} className="fill-muted-foreground font-mono text-[11px]">
            ∅
          </text>
        ))}
    </g>
  );
}

function Table({ table, x, tr }: { table: TableData; x: number; tr: ReturnType<typeof useTransitions> }) {
  const { t } = useTranslation();
  return (
    <g data-testid="table">
      <text x={x} y={12} className="fill-muted-foreground text-xs font-semibold uppercase tracking-wide">
        {t(`learn.stage.rows.${table.label}`)}
      </text>
      {table.rows.map((r, k) => {
        const role = r.role ?? "idle";
        return (
          <motion.g key={r.key} initial={false} animate={{ y: 22 + k * 30 }} transition={tr.move} data-key={r.key} data-role={role}>
            <rect
              x={x}
              width={TABLE_W - 8}
              height={26}
              rx={6}
              style={{ fill: `color-mix(in srgb, ${roleColor(role)} 18%, var(--card))`, stroke: roleColor(role), transition: tr.colors }}
              strokeWidth={role === "idle" ? 1 : 2}
              strokeDasharray={outline(role).strokeDasharray}
            />
            <text x={x + 10} y={17} className="fill-foreground font-mono text-xs">
              {r.key} → {r.value}
            </text>
          </motion.g>
        );
      })}
    </g>
  );
}

export function ListScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const list = step.list ?? { nodes: [], next: {}, labels: [] };
  const tableW = step.table ? TABLE_W : 0;
  const area = Math.max(160, width - tableW - (tableW ? 16 : 0));
  const n = Math.max(1, list.nodes.length);
  const pitch = Math.min(112, area / Math.max(n, 4));
  const nodeW = Math.max(30, Math.min(58, pitch - 30));
  const index = new Map(list.nodes.map((nd, k) => [nd.id, k]));
  const at = (id: number) => (index.get(id) ?? 0) * pitch + 4;
  const pointerY = TOP + NODE_H + 8;
  const maxSpan = Math.max(0, ...list.nodes.map((nd, k) => {
    const to = list.next[nd.id];
    const j = to === null || to === undefined ? undefined : index.get(to);
    return j === undefined || j === k + 1 ? 0 : Math.abs(j - k);
  }));
  const arcs = maxSpan > 0 ? 50 + maxSpan * 8 : 0;
  const height = Math.max(TOP + NODE_H + Math.max(arcs, 44) + 12, step.table ? 30 + step.table.rows.length * 30 : 0);
  // Labels sit under node centres: Pointers places `at` k at (k + 0.5) * slot.
  const labels = list.labels
    .filter((l) => index.has(l.id))
    .map((l) => ({ label: l.label, at: index.get(l.id)!, tone: l.tone }));

  return (
    <div ref={ref} className="relative w-full" data-testid="list-scene">
      <svg role="img" aria-label={sceneSummary(t, step as Step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        <Links list={list} at={at} nodeW={nodeW} tr={tr} />
        {list.nodes.map((nd) => {
          const role = nd.role ?? "idle";
          return (
            <motion.g
              key={nd.id}
              initial={instant ? false : { opacity: 0, y: -10 }}
              animate={{ opacity: 1, x: at(nd.id), y: 0 }}
              transition={{ x: tr.move, opacity: tr.lift, y: tr.lift }}
              data-node={nd.id}
              data-value={nd.value}
              data-role={role}
            >
              <rect
                y={TOP}
                width={nodeW}
                height={NODE_H}
                rx={8}
                style={{ fill: `color-mix(in srgb, ${roleColor(role)} 22%, var(--card))`, stroke: roleColor(role), transition: tr.colors }}
                strokeWidth={role === "idle" ? 1.5 : 2.5}
                strokeDasharray={outline(role).strokeDasharray}
              />
              <text
                x={nodeW / 2}
                y={TOP + NODE_H / 2 + 5}
                textAnchor="middle"
                className="fill-foreground font-mono font-semibold"
                style={{ fontSize: nd.text && nd.text.length > 3 ? 12 : 15 }}
              >
                {nd.text ?? nd.value}
              </text>
            </motion.g>
          );
        })}
        <g transform={`translate(${4 + nodeW / 2 - pitch / 2} 0)`}>
          <Pointers pointers={labels} slot={pitch} y={pointerY} transition={tr.move} />
        </g>
        {step.table && <Table table={step.table} x={width - TABLE_W} tr={tr} />}
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
