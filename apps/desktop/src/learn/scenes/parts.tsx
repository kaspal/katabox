// Scene parts shared by the recursion, Hanoi and stack scenes (ADR 0029): a stack column
// (call frames or stack entries, newest on top) and a tree drawn from `placeTrees`
// positions. Roles keep their outline cues; text always says what colour shows.
import { AnimatePresence, motion } from "motion/react";
import type { StackEntry, TreeNode } from "../engine/types";
import { outline, roleColor, type useTransitions } from "./common";

type Tr = ReturnType<typeof useTransitions>;

const FRAME_H = 30;
const FRAME_GAP = 6;

/** A vertical stack, bottom-anchored at `bottom`; the newest entry is on top. */
export function StackColumn({
  entries,
  x,
  width,
  bottom,
  title,
  empty,
  tr,
}: {
  entries: readonly StackEntry[];
  x: number;
  width: number;
  bottom: number;
  title: string;
  empty: string;
  tr: Tr;
}) {
  const instant = tr.colors === "none";
  return (
    <g data-testid="stack">
      <text x={x} y={14} className="fill-muted-foreground text-xs font-semibold uppercase tracking-wide">
        {title}
      </text>
      <line x1={x} x2={x + width} y1={bottom + 4} y2={bottom + 4} stroke="var(--viz-edge)" strokeWidth={2} />
      {entries.length === 0 && (
        <text x={x + width / 2} y={bottom - 10} textAnchor="middle" className="fill-muted-foreground text-xs italic">
          {empty}
        </text>
      )}
      <AnimatePresence initial={false}>
        {entries.map((e, k) => {
          const role = e.role ?? "idle";
          const top = k === entries.length - 1;
          return (
            <motion.g
              key={e.id}
              initial={instant ? false : { opacity: 0, y: bottom - (k + 1) * (FRAME_H + FRAME_GAP) - 24 }}
              animate={{ opacity: 1, y: bottom - (k + 1) * (FRAME_H + FRAME_GAP) + FRAME_GAP }}
              exit={instant ? { opacity: 0, transition: { duration: 0 } } : { opacity: 0, y: bottom - (k + 1) * (FRAME_H + FRAME_GAP) - 24 }}
              transition={tr.lift}
              data-frame={e.text}
              data-top={top || undefined}
            >
              <rect
                x={x}
                width={width}
                height={FRAME_H}
                rx={6}
                style={{
                  fill: `color-mix(in srgb, ${roleColor(role)} ${top ? 24 : 12}%, var(--card))`,
                  stroke: roleColor(role),
                  transition: tr.colors,
                }}
                strokeWidth={top ? 2.5 : 1.5}
                strokeDasharray={outline(role).strokeDasharray}
              />
              <text
                x={x + 10}
                y={FRAME_H / 2 + 4}
                className="fill-foreground font-mono text-[12px]"
                style={{ fontWeight: top ? 700 : 500 }}
              >
                {e.text}
              </text>
            </motion.g>
          );
        })}
      </AnimatePresence>
    </g>
  );
}

/** How tall a stack of `n` entries is drawn, with its title. */
export const stackHeight = (n: number) => 28 + Math.max(1, n) * (FRAME_H + FRAME_GAP) + 10;

/** A tree whose nodes carry `x` (column) and `depth`; nodes fade in where they will stay. */
export function TreeView({
  nodes,
  columns,
  left,
  width,
  top,
  level,
  tr,
}: {
  nodes: readonly TreeNode[];
  columns: number;
  left: number;
  width: number;
  top: number;
  level: number;
  tr: Tr;
}) {
  const col = width / Math.max(1, columns);
  const nodeW = Math.max(30, Math.min(92, col - 6));
  const nodeH = 34;
  const cx = (n: TreeNode) => left + ((n.x ?? 0) + 0.5) * col;
  const cy = (n: TreeNode) => top + (n.depth ?? 0) * level + nodeH / 2;
  const byId = new Map(nodes.map((n) => [n.id, n]));
  const instant = tr.colors === "none";
  return (
    <g data-testid="tree">
      {nodes.map((n) => {
        const p = n.parent === null ? undefined : byId.get(n.parent);
        if (!p) return null;
        return (
          <line
            key={`e${n.id}`}
            x1={cx(p)}
            y1={cy(p) + nodeH / 2}
            x2={cx(n)}
            y2={cy(n) - nodeH / 2}
            stroke="var(--viz-edge)"
            strokeWidth={1.5}
          />
        );
      })}
      {nodes.map((n) => (
        <motion.g
          key={n.id}
          initial={instant ? false : { opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={tr.lift}
          style={{ originX: `${cx(n)}px`, originY: `${cy(n)}px` }}
          data-node={n.text}
          data-role={n.role}
        >
          <rect
            x={cx(n) - nodeW / 2}
            y={cy(n) - nodeH / 2}
            width={nodeW}
            height={nodeH}
            rx={8}
            style={{
              fill: `color-mix(in srgb, ${roleColor(n.role)} 22%, var(--card))`,
              stroke: roleColor(n.role),
              transition: tr.colors,
            }}
            strokeWidth={n.role === "idle" ? 1.5 : 2.5}
            strokeDasharray={outline(n.role).strokeDasharray}
          />
          <text
            x={cx(n)}
            y={cy(n) + (n.result === undefined ? 4 : -2)}
            textAnchor="middle"
            className="fill-foreground font-mono"
            style={{ fontSize: nodeW < 50 ? 10 : 12 }}
          >
            {n.text}
          </text>
          {n.result !== undefined && (
            <text
              x={cx(n)}
              y={cy(n) + 12}
              textAnchor="middle"
              className="fill-foreground font-mono font-bold"
              style={{ fontSize: nodeW < 50 ? 10 : 12 }}
            >
              {n.result}
            </text>
          )}
        </motion.g>
      ))}
    </g>
  );
}
