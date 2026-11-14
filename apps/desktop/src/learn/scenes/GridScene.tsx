// Grids (ADR 0029, Phase 6): pathfinding and maze generation. Square cells; walls are
// solid (edge colour), weights 2-9 are printed faintly, a cell's label (a distance, f = g +
// h) replaces its weight while it has one. Start and goal are labelled S and G. The path
// found is a line through cell centres, so it does not rely on colour. In compare mode
// (the race) each topic draws its own grid at half width.
import { useTranslation } from "react-i18next";
import type { Role } from "../engine/types";
import { SceneFallback, outline, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

export function GridScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const g = step.grid;
  const rows = g?.rows ?? 0;
  const cols = g?.cols ?? 0;
  const size = Math.max(14, Math.min(36, Math.floor((width - 8) / Math.max(1, cols))));
  const gridW = cols * size;
  const left = Math.max(0, (width - gridW) / 2);
  const height = rows * size + 8;
  const center = (k: number) => ({ x: left + (k % cols) * size + size / 2, y: 4 + Math.floor(k / cols) * size + size / 2 });
  const font = size >= 28 ? 12 : size >= 20 ? 10 : 8;

  return (
    <div ref={ref} className="relative w-full" data-testid="grid-scene">
      <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        {g?.cells.map((v, k) => {
          const r = Math.floor(k / cols);
          const c = k % cols;
          const role: Role = g.roles[k] ?? "idle";
          const wall = v === 1;
          const isStart = g.start && g.start[0] === r && g.start[1] === c;
          const isGoal = g.goal && g.goal[0] === r && g.goal[1] === c;
          const label = isStart ? "S" : isGoal ? "G" : (g.text[k] ?? (v >= 2 ? String(v) : ""));
          return (
            <g key={k} transform={`translate(${left + c * size},${4 + r * size})`} data-cell={`${r}-${c}`} data-role={wall ? "wall" : role}>
              <rect
                x={1}
                y={1}
                width={size - 2}
                height={size - 2}
                rx={Math.min(5, size / 6)}
                style={{
                  fill: wall ? "var(--viz-edge)" : role === "idle" ? "var(--card)" : `color-mix(in srgb, ${roleColor(role)} 30%, var(--card))`,
                  stroke: wall ? "var(--viz-edge)" : role === "idle" ? "var(--viz-out)" : roleColor(role),
                  transition: tr.colors,
                }}
                strokeWidth={role === "key" ? 2.5 : 1}
                strokeDasharray={outline(role).strokeDasharray}
              />
              {label !== "" && !wall && (
                <text
                  x={size / 2}
                  y={size / 2 + font / 2.8}
                  textAnchor="middle"
                  className={isStart || isGoal ? "fill-foreground font-mono font-bold" : g.text[k] !== undefined ? "fill-foreground font-mono" : "fill-muted-foreground font-mono"}
                  style={{ fontSize: isStart || isGoal ? font + 2 : font }}
                >
                  {label}
                </text>
              )}
            </g>
          );
        })}
        {g && g.path.length > 1 && (
          <polyline
            data-testid="grid-path"
            points={g.path.map((k) => `${center(k).x},${center(k).y}`).join(" ")}
            fill="none"
            stroke={roleColor("swap")}
            strokeWidth={Math.max(2, size / 8)}
            strokeLinecap="round"
            strokeLinejoin="round"
            opacity={0.85}
          />
        )}
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
