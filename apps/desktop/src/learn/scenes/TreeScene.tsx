// Trees and forests (ADR 0029, Phase 2): BSTs, traversals, tries, union-find forests,
// tree diameter. Tidy layout from d3-hierarchy (engine/layout.ts); nodes keyed by id, so
// an insert, a delete or a re-link animates to the new layout. Edge labels (a trie's
// letters), notes under nodes (a distance, a count) and a second ring (end of a word, a
// root). Underneath, optionally: a queue row (`slots`) and an output row (`aux`).
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import { layoutForest } from "../engine/layout";
import { Cell } from "./ArrayScene";
import { SceneFallback, outline, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

const R = 18;
const LEVEL = 60;
const TOP = 24;

export function TreeScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const nodes = step.forest ?? [];
  const { at, width: cols, depth } = layoutForest(nodes);
  const col = Math.max(34, Math.min(70, (width - 2 * R - 8) / Math.max(1, cols - 1 || 1)));
  const span = (cols - 1) * col;
  const left = Math.max(R + 4, (width - span) / 2);
  const pos = (id: number) => {
    const p = at.get(id)!;
    return { x: left + p.x * col, y: TOP + R + p.depth * LEVEL };
  };
  const treeH = nodes.length ? TOP + 2 * R + depth * LEVEL + 22 : 40;
  const rows: { key: string; label: string; cells: { id: number; value: number }[]; roles: Record<number, string> }[] = [];
  if (step.slots) rows.push({ key: "slots", label: t(`learn.stage.rows.${step.slots.label}`), cells: step.slots.cells.filter((c): c is { id: number; value: number } => c !== null), roles: step.slots.roles ?? {} });
  if (step.aux) rows.push({ key: "aux", label: t(`learn.stage.rows.${step.aux.label}`), cells: step.aux.cells.filter((c): c is { id: number; value: number } => c !== null), roles: step.aux.roles ?? {} });
  const cell = 34;
  const rowH = cell + 30;
  const height = treeH + rows.length * rowH + 6;
  const byId = new Map(nodes.map((n) => [n.id, n]));

  return (
    <div ref={ref} className="relative w-full" data-testid="tree-scene">
      <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        {nodes.map((n) => {
          const p = n.parent === null ? undefined : byId.get(n.parent);
          if (!p || !at.has(p.id) || !at.has(n.id)) return null;
          const a = pos(p.id);
          const b = pos(n.id);
          return (
            <g key={`e${n.id}`}>
              <motion.line
                initial={false}
                animate={{ x1: a.x, y1: a.y, x2: b.x, y2: b.y }}
                transition={tr.move}
                stroke="var(--viz-edge)"
                strokeWidth={1.6}
              />
              {n.edge && (
                <motion.text
                  initial={false}
                  animate={{ x: (a.x + b.x) / 2 + (b.x >= a.x ? 7 : -7), y: (a.y + b.y) / 2 }}
                  transition={tr.move}
                  textAnchor="middle"
                  className="fill-foreground font-mono text-xs font-bold"
                  style={{ paintOrder: "stroke", stroke: "var(--card)", strokeWidth: 4, strokeLinejoin: "round" }}
                >
                  {n.edge}
                </motion.text>
              )}
            </g>
          );
        })}
        {nodes.map((n) => {
          if (!at.has(n.id)) return null;
          const { x, y } = pos(n.id);
          const role = n.role ?? "idle";
          return (
            <motion.g
              key={n.id}
              initial={instant ? false : { opacity: 0, x, y: y - 12 }}
              animate={{ opacity: 1, x, y }}
              transition={{ x: tr.move, y: tr.move, opacity: tr.lift }}
              data-node={n.id}
              data-text={n.text}
              data-role={role}
            >
              {n.mark && <circle r={R + 4} fill="none" stroke={roleColor(role === "idle" ? "sorted" : role)} strokeWidth={1.5} />}
              {n.text.length > 3 ? (
                // Several keys (a B-tree node): a box as wide as its text.
                <rect
                  x={-(n.text.length * 4.6 + 10)}
                  y={-R}
                  width={n.text.length * 9.2 + 20}
                  height={2 * R}
                  rx={8}
                  style={{ fill: `color-mix(in srgb, ${roleColor(role)} 22%, var(--card))`, stroke: roleColor(role), transition: tr.colors }}
                  strokeWidth={role === "idle" ? 1.5 : 2.5}
                  strokeDasharray={outline(role).strokeDasharray}
                />
              ) : (
                <circle
                  r={R}
                  style={{ fill: `color-mix(in srgb, ${roleColor(role)} 22%, var(--card))`, stroke: roleColor(role), transition: tr.colors }}
                  strokeWidth={role === "idle" ? 1.5 : 2.5}
                  strokeDasharray={outline(role).strokeDasharray}
                />
              )}
              <text y={5} textAnchor="middle" className="fill-foreground font-mono font-semibold" style={{ fontSize: n.text.length === 3 ? 11 : 14 }}>
                {n.text}
              </text>
              {n.note && (
                <text y={R + 14} textAnchor="middle" className="fill-muted-foreground font-mono text-[11px]" style={{ paintOrder: "stroke", stroke: "var(--card)", strokeWidth: 4, strokeLinejoin: "round" }}>
                  {n.note}
                </text>
              )}
            </motion.g>
          );
        })}
        {rows.map((row, k) => {
          const y = treeH + k * rowH;
          const slot = Math.min(44, (width - 8) / Math.max(row.cells.length, 8));
          return (
            <g key={row.key} data-testid={`row-${row.key}`}>
              <text x={0} y={y + 10} className="fill-muted-foreground text-xs font-medium">
                {row.label}
              </text>
              {row.cells.map((c, j) => (
                <g key={`${row.key}${c.id}`}>
                  <Cell item={c} role={(row.roles[j] as never) ?? "idle"} cell={Math.min(cell, slot - 6)} x={j * slot} y={y + 18} tr={tr} keyPrefix={`${row.key}-`} />
                </g>
              ))}
            </g>
          );
        })}
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
