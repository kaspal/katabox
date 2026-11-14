// Buckets (ADR 0029, Phase 1): counting, radix and bucket sort. The array on top; below,
// one column per bucket with its label (a value, a digit or a range). A value keeps its
// identity: it drops from its slot into a bucket and later flies back to its new slot, in
// the order the bucket hands it out. The bucket being filled or emptied is outlined.
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import type { Item, Role } from "../engine/types";
import { Cell } from "./ArrayScene";
import { IndexRow, Pointers, SceneFallback, roleAt, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

const ROW_Y = 8;
const GAP = 64; // from the index row to the buckets' rims

type Placed = { item: Item; x: number; y: number; role: Role; pos?: number };

export function BucketsScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const n = step.items.length;
  const buckets = step.buckets ?? [];
  const slot = width / Math.max(1, n);
  const cell = Math.max(18, Math.min(48, slot - 8));
  const bw = width / Math.max(1, buckets.length);
  const depth = Math.max(1, step.bucketDepth ?? n);
  const bcell = Math.max(16, Math.min(34, bw - 10, 220 / depth - 3));
  const indexY = ROW_Y + cell + 16;
  const pointerY = indexY + 8;
  const rim = pointerY + GAP;
  const floor = rim + depth * (bcell + 3) + 6;
  const height = floor + 26;
  const hidden = new Set(step.hidden ?? []);

  const placed: Placed[] = [];
  step.items.forEach((it, p) => {
    if (!hidden.has(it.id)) {
      placed.push({ item: it, x: p * slot + (slot - cell) / 2, y: ROW_Y, role: roleAt(step, p), pos: p });
    }
  });
  buckets.forEach((b, k) =>
    b.items.forEach((it, level) =>
      placed.push({
        item: it,
        x: k * bw + (bw - bcell) / 2,
        y: floor - (level + 1) * (bcell + 3),
        role: step.bucket === k ? "compare" : "idle",
      }),
    ),
  );
  placed.sort((a, b) => a.item.id - b.item.id);

  return (
    <div ref={ref} className="relative w-full" data-testid="buckets-scene">
      <svg
        role="img"
        aria-label={sceneSummary(t, step)}
        width={width}
        height={height}
        viewBox={`0 0 ${width} ${height}`}
        className="block overflow-visible"
      >
        {Array.from({ length: n }, (_, p) => {
          const empty = hidden.has(step.items[p]!.id);
          return (
            <rect
              key={p}
              x={p * slot + (slot - cell) / 2}
              y={ROW_Y}
              width={cell}
              height={cell}
              rx={6}
              fill="none"
              stroke="var(--border)"
              strokeDasharray={empty ? "5 4" : undefined}
              data-empty={empty || undefined}
            />
          );
        })}
        <IndexRow n={n} slot={slot} y={indexY} />
        <Pointers pointers={step.pointers} slot={slot} y={pointerY} transition={tr.move} />
        {buckets.map((b, k) => {
          const on = step.bucket === k;
          return (
            <g key={k} data-bucket={b.label} data-active={on || undefined}>
              <path
                d={`M${k * bw + 4} ${rim} V${floor} H${(k + 1) * bw - 4} V${rim}`}
                fill={on ? "var(--viz-range)" : "none"}
                stroke={on ? "var(--viz-compare)" : "var(--viz-edge)"}
                strokeWidth={on ? 2.5 : 1.5}
                strokeDasharray={on ? "5 3" : undefined}
              />
              <text
                x={(k + 0.5) * bw}
                y={floor + 18}
                textAnchor="middle"
                className="fill-foreground font-mono text-xs font-semibold"
              >
                {b.label}
              </text>
            </g>
          );
        })}
        {placed.map((c) => (
          <motion.g key={c.item.id} initial={false} animate={{ opacity: 1 }} data-pos={c.pos}>
            <Cell item={c.item} role={c.role} cell={c.pos === undefined ? bcell : cell} x={c.x} y={c.y} tr={tr} />
          </motion.g>
        ))}
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
