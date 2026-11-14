// Graphs (ADR 0029, Phases 4-5): nodes placed once by d3-force (engine/graphLayout.ts),
// so they never move during playback; only colours, notes and edge states change. Edges
// carry arrows when directed and weights when weighted; a pair of opposite directed
// edges is drawn as two bows. Notes under nodes (a distance, a side), a second ring
// (settled, a root). Underneath, optionally: a frontier row (`slots`: queue, stack,
// priority queue) and an output row (`aux`).
import { useTranslation } from "react-i18next";
import { layoutGraph } from "../engine/graphLayout";
import type { NoteKey, Role } from "../engine/types";
import { Cell } from "./ArrayScene";
import { SceneFallback, outline, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

const R = 17;
const PAD = R + 16;

export function GraphScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const g = step.graph;
  const n = g?.n ?? 0;
  const { pos, aspect } = layoutGraph(n, g?.edges ?? []);
  const graphH = Math.max(170, Math.min(340, (width - 2 * PAD) * aspect + 2 * PAD));
  const graphW = Math.min(width, (graphH - 2 * PAD) / Math.max(aspect, 0.25) + 2 * PAD + 40);
  const left = (width - graphW) / 2;
  const at = (id: number) => ({ x: left + PAD + pos[id]!.x * (graphW - 2 * PAD), y: PAD + pos[id]!.y * (graphH - 2 * PAD) });
  /** Where the panels and rows start: below the lowest node's note. */
  const below = graphH + 18;
  const rows: { key: string; label: string; cells: { id: number; value: number }[]; roles: Record<number, string> }[] = [];
  if (step.slots) rows.push({ key: "slots", label: t(`learn.stage.rows.${step.slots.label}`), cells: step.slots.cells.filter((c): c is { id: number; value: number } => c !== null), roles: step.slots.roles ?? {} });
  if (step.aux) rows.push({ key: "aux", label: t(`learn.stage.rows.${step.aux.label}`), cells: step.aux.cells.filter((c): c is { id: number; value: number } => c !== null), roles: step.aux.roles ?? {} });
  const cell = 34;
  const rowH = cell + 30;
  // Panels under the graph: an adjacency list and/or a matrix (Floyd-Warshall, representations).
  const adj = g?.adjacency;
  const mat = g?.matrix;
  const mc = mat ? Math.max(18, Math.min(30, (width - 40) / Math.max(1, mat.cells.length) - 2)) : 0;
  const matH = mat ? (mat.cells.length + 1) * (mc + 2) + 24 : 0;
  const adjH = adj ? adj.length * 18 + 24 : 0;
  const panelH = Math.max(matH, adjH);
  const height = below + panelH + rows.length * rowH + 6;
  const noteText = (note: string) => (note.startsWith("@") ? t(`learn.stage.notes.${note.slice(1) as NoteKey}`) : note);
  const pairs = new Set((g?.edges ?? []).map((e) => `${e.u}>${e.v}`));
  const halo = { paintOrder: "stroke", stroke: "var(--card)", strokeWidth: 4, strokeLinejoin: "round" } as const;

  return (
    <div ref={ref} className="relative w-full" data-testid="graph-scene">
      <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        <defs>
          {(["idle", "compare", "key", "pivot", "sorted", "swap", "out"] as Role[]).map((r) => (
            <marker key={r} id={`kb-arrow-${r}`} viewBox="0 0 10 10" refX={9} refY={5} markerWidth={7} markerHeight={7} orient="auto-start-reverse">
              <path d="M0,0 L10,5 L0,10 z" fill={r === "idle" ? "var(--viz-edge)" : roleColor(r)} />
            </marker>
          ))}
        </defs>
        {g?.edges.map((e) => {
          if (!pos[e.u] || !pos[e.v]) return null;
          const a = at(e.u);
          const b = at(e.v);
          const dx = b.x - a.x;
          const dy = b.y - a.y;
          const len = Math.hypot(dx, dy) || 1;
          const [ux, uy] = [dx / len, dy / len];
          // Opposite directed edges bow apart; otherwise straight.
          const bow = g.directed && pairs.has(`${e.v}>${e.u}`) ? 14 : 0;
          const [nx, ny] = [-uy * bow, ux * bow];
          const s = { x: a.x + ux * R, y: a.y + uy * R };
          const end = { x: b.x - ux * (R + (g.directed ? 3 : 0)), y: b.y - uy * (R + (g.directed ? 3 : 0)) };
          const mid = { x: (s.x + end.x) / 2 + nx, y: (s.y + end.y) / 2 + ny };
          const role = e.role ?? "idle";
          const active = role !== "idle";
          const d = bow ? `M${s.x},${s.y} Q${mid.x + nx},${mid.y + ny} ${end.x},${end.y}` : `M${s.x},${s.y} L${end.x},${end.y}`;
          return (
            <g key={`e${e.id}`} data-edge={`${e.u}-${e.v}`} data-role={role}>
              <path
                d={d}
                fill="none"
                style={{ stroke: active ? roleColor(role) : "var(--viz-edge)", transition: tr.colors }}
                strokeWidth={active ? 3 : 1.6}
                strokeDasharray={role === "compare" ? "6 4" : role === "out" ? "2 4" : undefined}
                markerEnd={g.directed ? `url(#kb-arrow-${active ? role : "idle"})` : undefined}
              />
              {(g.weighted || e.label) && (
                <text x={mid.x + nx * 0.2 - uy * 9} y={mid.y + ny * 0.2 + ux * 9 + 4} textAnchor="middle" className="fill-foreground font-mono text-xs font-bold" style={halo}>
                  {e.label ?? e.w}
                </text>
              )}
            </g>
          );
        })}
        {g?.nodes.map((v) => {
          if (!pos[v.id]) return null;
          const { x, y } = at(v.id);
          const role = v.role ?? "idle";
          return (
            <g key={v.id} transform={`translate(${x},${y})`} data-node={v.id} data-role={role}>
              {v.mark && <circle r={R + 4} fill="none" stroke={roleColor(role === "idle" ? "sorted" : role)} strokeWidth={1.5} />}
              <circle
                r={R}
                style={{ fill: `color-mix(in srgb, ${roleColor(role)} 22%, var(--card))`, stroke: roleColor(role), transition: tr.colors }}
                strokeWidth={role === "idle" ? 1.5 : 2.5}
                strokeDasharray={outline(role).strokeDasharray}
              />
              <text y={5} textAnchor="middle" className="fill-foreground font-mono text-sm font-semibold">
                {v.text ?? v.id}
              </text>
              {v.note && (
                <text y={R + 14} textAnchor="middle" className="fill-muted-foreground font-mono text-[11px]" style={halo}>
                  {noteText(v.note)}
                </text>
              )}
            </g>
          );
        })}
        {adj && (
          <g data-testid="adjacency" transform={`translate(0,${below})`}>
            <text x={0} y={12} className="fill-muted-foreground text-xs font-medium">
              {t("learn.stage.rows.adjacency")}
            </text>
            {adj.map((list, u) => (
              <text key={u} x={0} y={32 + u * 18} className="fill-foreground font-mono text-xs" style={{ fontWeight: g?.hotRow === u ? 700 : 400 }}>
                {`${u}: ${list.join(" ") || "—"}`}
              </text>
            ))}
          </g>
        )}
        {mat && (
          <g data-testid="matrix" transform={`translate(${adj ? Math.max(150, width - (mat.cells.length + 1) * (mc + 2) - 4) : 0},${below})`}>
            <text x={0} y={12} className="fill-muted-foreground text-xs font-medium">
              {t(`learn.stage.rows.${mat.label}`)}
            </text>
            {mat.cells.map((_, j) => (
              <text key={`h${j}`} x={(j + 1) * (mc + 2) + mc / 2} y={36} textAnchor="middle" className="fill-muted-foreground font-mono text-[11px]">
                {j}
              </text>
            ))}
            {mat.cells.map((row, i) => (
              <g key={`r${i}`} transform={`translate(0,${24 + (i + 1) * (mc + 2)})`}>
                <text x={mc / 2} y={mc / 2 + 4} textAnchor="middle" className="fill-muted-foreground font-mono text-[11px]">
                  {i}
                </text>
                {row.map((v, j) => {
                  const hot = mat.hot?.some(([a, b]) => a === i && b === j);
                  const via = mat.via?.some(([a, b]) => a === i && b === j);
                  const role: Role = hot ? "key" : via ? "compare" : "idle";
                  return (
                    <g key={j} transform={`translate(${(j + 1) * (mc + 2)},0)`} data-cell={`${i}-${j}`} data-role={role}>
                      <rect
                        width={mc}
                        height={mc}
                        rx={4}
                        style={{ fill: `color-mix(in srgb, ${roleColor(role)} ${role === "idle" ? 8 : 26}%, var(--card))`, stroke: roleColor(role), transition: tr.colors }}
                        strokeWidth={role === "idle" ? 1 : 2}
                        strokeDasharray={outline(role).strokeDasharray}
                      />
                      <text x={mc / 2} y={mc / 2 + 4} textAnchor="middle" className="fill-foreground font-mono" style={{ fontSize: mc < 24 ? 10 : 12 }}>
                        {v === null ? "∞" : v}
                      </text>
                    </g>
                  );
                })}
              </g>
            ))}
          </g>
        )}
        {rows.map((row, k) => {
          const y = below + panelH + k * rowH;
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
