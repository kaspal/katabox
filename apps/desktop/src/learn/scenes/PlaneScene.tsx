// The plane (ADR 0029, Phase 10): convex hull, segment intersection. A square grid with
// axes from 0 to `max`; points are labelled dots, segments and the hull polygon are lines
// with roles (dashed while tested, solid once kept); a turn being tested is drawn as a
// small triangle o-a-b. Labels carry the point numbers and coordinates, so nothing relies
// on colour alone.
import { useTranslation } from "react-i18next";
import type { Role } from "../engine/types";
import { SceneFallback, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

export function PlaneScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const p = step.plane;
  const max = Math.max(1, p?.max ?? 10);
  const side = Math.max(200, Math.min(width - 40, 420));
  const left = Math.max(28, (width - side) / 2);
  const top = 10;
  const height = side + 40;
  const X = (x: number) => left + (x / max) * side;
  const Y = (y: number) => top + side - (y / max) * side;
  const ticks = Array.from({ length: max + 1 }, (_, k) => k).filter((k) => max <= 12 || k % 5 === 0);
  const stroke = (role: Role | undefined) => (role && role !== "idle" ? roleColor(role) : "var(--viz-edge)");

  return (
    <div ref={ref} className="relative w-full" data-testid="plane-scene">
      <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        {ticks.map((k) => (
          <g key={`g${k}`}>
            <line x1={X(k)} x2={X(k)} y1={Y(0)} y2={Y(max)} stroke="var(--viz-out)" strokeWidth={0.6} />
            <line x1={X(0)} x2={X(max)} y1={Y(k)} y2={Y(k)} stroke="var(--viz-out)" strokeWidth={0.6} />
            <text x={X(k)} y={Y(0) + 14} textAnchor="middle" className="fill-muted-foreground font-mono text-[10px]">
              {k}
            </text>
            <text x={X(0) - 6} y={Y(k) + 3} textAnchor="end" className="fill-muted-foreground font-mono text-[10px]">
              {k}
            </text>
          </g>
        ))}
        {p?.polygon && p.polygon.length > 1 && (
          <polyline
            data-testid="plane-polygon"
            points={p.polygon.map(([x, y]) => `${X(x)},${Y(y)}`).join(" ")}
            fill={p.closed ? `color-mix(in srgb, ${roleColor("sorted")} 14%, transparent)` : "none"}
            stroke={roleColor("sorted")}
            strokeWidth={2.5}
            strokeLinejoin="round"
          />
        )}
        {p?.closed && p.polygon && p.polygon.length > 2 && (
          <line x1={X(p.polygon[p.polygon.length - 1]![0])} y1={Y(p.polygon[p.polygon.length - 1]![1])} x2={X(p.polygon[0]![0])} y2={Y(p.polygon[0]![1])} stroke={roleColor("sorted")} strokeWidth={2.5} />
        )}
        {p?.segments.map((s, k) => (
          <line
            key={`s${k}`}
            data-testid="plane-segment"
            data-role={s.role ?? "idle"}
            x1={X(s.a[0])}
            y1={Y(s.a[1])}
            x2={X(s.b[0])}
            y2={Y(s.b[1])}
            style={{ stroke: stroke(s.role), transition: tr.colors }}
            strokeWidth={s.role && s.role !== "idle" ? 3 : 2}
            strokeDasharray={s.role === "compare" ? "6 4" : undefined}
          />
        ))}
        {p?.turn && (
          <polygon
            data-testid="plane-turn"
            points={p.turn.map(([x, y]) => `${X(x)},${Y(y)}`).join(" ")}
            fill={`color-mix(in srgb, ${roleColor(p.turnRole ?? "compare")} 18%, transparent)`}
            stroke={roleColor(p.turnRole ?? "compare")}
            strokeDasharray="4 3"
          />
        )}
        {p?.points.map((pt) => {
          const role = pt.role ?? "idle";
          return (
            <g key={pt.id} transform={`translate(${X(pt.x)},${Y(pt.y)})`} data-point={pt.id} data-role={role}>
              <circle r={role === "idle" ? 4.5 : 6.5} style={{ fill: role === "idle" ? "var(--viz-idle)" : roleColor(role), transition: tr.colors }} stroke="var(--card)" strokeWidth={1.5} />
              <text x={8} y={-7} className="fill-foreground font-mono text-[10px]" style={{ paintOrder: "stroke", stroke: "var(--card)", strokeWidth: 3 }}>
                {pt.label ?? `${pt.id}`}
              </text>
            </g>
          );
        })}
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
