// Heap sort (ADR 0029, Phase 1): the heap part of the array (positions 0..heap-1) drawn as
// a tree above the array, by the usual index rule (children of i are 2i + 1 and 2i + 2).
// The layout is the complete tree of the whole array, fixed for the trace, so a swap moves
// two values between fixed spots in both views at once. Values that leave the heap stay
// in the array only, in their final place.
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import { Cell } from "./ArrayScene";
import { IndexRow, Pointers, SceneFallback, outline, roleAt, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

const LEVEL = 54;
const NODE = 34;

/** In-order column of each position in a complete binary tree of `n` nodes. */
export function heapColumns(n: number): number[] {
  const col = new Array<number>(n).fill(0);
  let next = 0;
  const walk = (i: number) => {
    if (i >= n) return;
    walk(2 * i + 1);
    col[i] = next++;
    walk(2 * i + 2);
  };
  walk(0);
  return col;
}

export function HeapScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const n = step.items.length;
  const heap = step.heap ?? n;
  const cols = heapColumns(n);
  const levels = n > 0 ? Math.floor(Math.log2(n)) + 1 : 1;
  const colW = width / Math.max(1, n);
  const node = Math.max(22, Math.min(NODE, colW - 4));
  const treeH = levels * LEVEL;
  const slot = width / Math.max(1, n);
  const cell = Math.max(18, Math.min(44, slot - 8));
  const rowY = treeH + 20;
  const indexY = rowY + cell + 16;
  const pointerY = indexY + 8;
  const height = pointerY + 32;
  const cx = (p: number) => (cols[p]! + 0.5) * colW;
  const cy = (p: number) => Math.floor(Math.log2(p + 1)) * LEVEL + NODE / 2 + 4;

  return (
    <div ref={ref} className="relative w-full" data-testid="heap-scene">
      <svg
        role="img"
        aria-label={sceneSummary(t, step)}
        width={width}
        height={height}
        viewBox={`0 0 ${width} ${height}`}
        className="block overflow-visible"
      >
        {Array.from({ length: heap }, (_, p) =>
          [2 * p + 1, 2 * p + 2]
            .filter((c) => c < heap)
            .map((c) => (
              <line key={`${p}-${c}`} x1={cx(p)} y1={cy(p)} x2={cx(c)} y2={cy(c)} stroke="var(--viz-edge)" strokeWidth={1.5} />
            )),
        )}
        {step.items.map((it, p) => {
          if (p >= heap) return null;
          const role = roleAt(step, p);
          return (
            <motion.g
              key={`t${it.id}`}
              initial={false}
              animate={{ x: cx(p), y: cy(p) }}
              transition={tr.move}
              data-tree-id={it.id}
              data-role={role}
            >
              <circle
                r={node / 2}
                style={{
                  fill: `color-mix(in srgb, ${roleColor(role)} 22%, var(--card))`,
                  stroke: roleColor(role),
                  transition: tr.colors,
                }}
                strokeWidth={role === "idle" ? 1.5 : 2.5}
                strokeDasharray={outline(role).strokeDasharray}
              />
              <text y={4} textAnchor="middle" className="fill-foreground font-mono text-xs font-semibold">
                {it.value}
              </text>
            </motion.g>
          );
        })}
        <line x1={0} x2={width} y1={rowY - 12} y2={rowY - 12} stroke="var(--border)" />
        {heap < n && (
          <rect
            x={heap * slot + 2}
            y={rowY - 6}
            width={(n - heap) * slot - 4}
            height={cell + 12}
            rx={10}
            fill="var(--viz-range)"
            data-testid="sorted-tail"
          />
        )}
        {step.items.map((it, p) => (
          <g key={`a${it.id}`} data-pos={p}>
            <Cell item={it} role={roleAt(step, p)} cell={cell} x={p * slot + (slot - cell) / 2} y={rowY} tr={tr} keyPrefix="a" />
          </g>
        ))}
        <IndexRow n={n} slot={slot} y={indexY} />
        <Pointers pointers={step.pointers} slot={slot} y={pointerY} transition={tr.move} />
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
