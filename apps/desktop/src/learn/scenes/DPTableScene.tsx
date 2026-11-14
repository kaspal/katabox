// Dynamic programming tables (ADR 0029, Phases 7-8): a 1-D or 2-D table with row and
// column headers. The cell being computed is outlined, the cells it reads are dashed and
// joined to it by arrows, and the traceback path is marked once the table is full. Empty
// cells are blank, so the fill order shows. Above the table, optionally: an input row
// (`slots`: values, items, a call stack); below it, an output row (`aux`).
import { useTranslation } from "react-i18next";
import type { Role } from "../engine/types";
import { Cell } from "./ArrayScene";
import { SceneFallback, outline, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

export function DPTableScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const dp = step.dp;
  const nRows = dp?.cells.length ?? 0;
  const nCols = dp?.cells[0]?.length ?? 0;
  const head = dp?.rowLabels ? Math.max(30, 8 * Math.max(...dp.rowLabels.map((l) => l.length)) + 10) : 0;
  const size = Math.max(20, Math.min(42, Math.floor((width - head - 8) / Math.max(1, nCols))));
  const tableW = head + nCols * size;
  const left = Math.max(0, (width - tableW) / 2);
  const rows: { key: string; label: string; cells: { id: number; value: number; text?: string }[]; roles: Record<number, string> }[] = [];
  if (step.slots) rows.push({ key: "slots", label: t(`learn.stage.rows.${step.slots.label}`), cells: step.slots.cells.filter((c): c is { id: number; value: number } => c !== null), roles: step.slots.roles ?? {} });
  const cell = 34;
  const rowH = cell + 30;
  const top = rows.length * rowH + 22;
  /** The title, then the column numbers, then the cells. */
  const GRID = 34;
  const tableH = GRID + nRows * size;
  const outY = top + tableH + 10;
  const height = outY + (step.aux ? rowH : 0) + 4;
  const at = (r: number, c: number) => ({ x: left + head + c * size + size / 2, y: top + GRID + r * size + size / 2 });
  const hot = dp?.hot ?? null;
  const has = (list: readonly [number, number][] | undefined, r: number, c: number) => !!list?.some(([a, b]) => a === r && b === c);

  const drawRow = (row: (typeof rows)[number], y: number) => {
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
  };

  return (
    <div ref={ref} className="relative w-full" data-testid="dp-scene">
      <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        <defs>
          <marker id="kb-dp-arrow" viewBox="0 0 10 10" refX={8} refY={5} markerWidth={6} markerHeight={6} orient="auto-start-reverse">
            <path d="M0,0 L10,5 L0,10 z" fill={roleColor("compare")} />
          </marker>
        </defs>
        {rows.map((row, k) => drawRow(row, k * rowH))}
        {dp && (
          <g data-testid="dp-table">
            <text x={left} y={top + 10} className="fill-muted-foreground text-xs font-medium">
              {t(`learn.stage.rows.${dp.label}`)}
            </text>
            {dp.colLabels.map((l, c) => (
              <text key={`c${c}`} x={left + head + c * size + size / 2} y={top + GRID - 5} textAnchor="middle" className="fill-muted-foreground font-mono text-[10px]">
                {l}
              </text>
            ))}
            {dp.rowLabels?.map((l, r) => (
              <text key={`r${r}`} x={left + head - 6} y={top + GRID + r * size + size / 2 + 4} textAnchor="end" className="fill-muted-foreground font-mono text-[11px]">
                {l}
              </text>
            ))}
            {dp.cells.map((row, r) =>
              row.map((v, c) => {
                const role: Role = hot && hot[0] === r && hot[1] === c ? "key" : has(dp.deps, r, c) ? "compare" : has(dp.path, r, c) ? "swap" : v === null ? "idle" : "sorted";
                return (
                  <g key={`${r}-${c}`} transform={`translate(${left + head + c * size},${top + GRID + r * size})`} data-cell={`${r}-${c}`} data-role={role}>
                    <rect
                      x={1}
                      y={1}
                      width={size - 2}
                      height={size - 2}
                      rx={4}
                      style={{
                        fill: role === "idle" ? "var(--card)" : `color-mix(in srgb, ${roleColor(role)} ${role === "sorted" ? 12 : 28}%, var(--card))`,
                        stroke: role === "idle" ? "var(--viz-out)" : roleColor(role),
                        transition: tr.colors,
                      }}
                      strokeWidth={role === "key" || role === "swap" ? 2.5 : 1}
                      strokeDasharray={outline(role).strokeDasharray}
                    />
                    {v !== null && (
                      <text x={size / 2} y={size / 2 + 4} textAnchor="middle" className="fill-foreground font-mono" style={{ fontSize: size < 26 ? 10 : 12 }}>
                        {v === Infinity || v >= 1e6 ? "∞" : v}
                      </text>
                    )}
                  </g>
                );
              }),
            )}
            {hot &&
              dp.deps?.map(([r, c], k) => {
                const a = at(r, c);
                const b = at(hot[0], hot[1]);
                const dx = b.x - a.x;
                const dy = b.y - a.y;
                const len = Math.hypot(dx, dy) || 1;
                const pad = size * 0.32;
                const bend = 0.25 * len;
                const mx = (a.x + b.x) / 2 - (dy / len) * Math.min(bend, 24);
                const my = (a.y + b.y) / 2 + (dx / len) * Math.min(bend, 24);
                return (
                  <path
                    key={`d${k}`}
                    data-testid="dp-dep"
                    d={`M${a.x + (dx / len) * pad},${a.y + (dy / len) * pad} Q${mx},${my} ${b.x - (dx / len) * pad},${b.y - (dy / len) * pad}`}
                    fill="none"
                    stroke={roleColor("compare")}
                    strokeWidth={1.6}
                    markerEnd="url(#kb-dp-arrow)"
                    opacity={0.9}
                  />
                );
              })}
          </g>
        )}
        {step.aux &&
          drawRow({ key: "aux", label: t(`learn.stage.rows.${step.aux.label}`), cells: step.aux.cells.filter((c): c is { id: number; value: number } => c !== null), roles: step.aux.roles ?? {} }, outY)}
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
