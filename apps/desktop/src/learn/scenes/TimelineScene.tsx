// Intervals on a time line (ADR 0029, Phase 8): activity selection, merging intervals.
// Each interval is a bar on its own lane, in the order the algorithm looks at them; the
// bar's role says what happened to it (taken, clashing, merged). A dashed vertical line
// marks a moment (the time the room is free again), and an output lane shows the result
// (merged intervals). Start and end numbers sit on every bar, so nothing is told by
// colour alone.
import { useTranslation } from "react-i18next";
import type { Role } from "../engine/types";
import { SceneFallback, outline, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

const LANE = 26;

export function TimelineScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const tl = step.timeline;
  const lo = tl?.min ?? 0;
  const hi = Math.max(lo + 1, tl?.max ?? 1);
  const left = 24;
  const span = Math.max(100, width - left - 16);
  const x = (v: number) => left + ((v - lo) / (hi - lo)) * span;
  const bars = tl?.bars ?? [];
  const out = tl?.output ?? [];
  const axisY = 14 + bars.length * LANE + 8;
  const outY = axisY + 34;
  const height = outY + (tl?.output ? 18 + LANE : 0) + 6;
  const ticks = Array.from({ length: hi - lo + 1 }, (_, k) => lo + k).filter((v, _k, all) => all.length <= 25 || (v - lo) % 2 === 0);

  const bar = (key: string, start: number, end: number, y: number, role: Role, label?: string) => (
    <g key={key} data-role={role}>
      <rect
        x={x(start)}
        y={y}
        width={Math.max(4, x(end) - x(start))}
        height={LANE - 8}
        rx={5}
        style={{ fill: `color-mix(in srgb, ${roleColor(role)} ${role === "idle" ? 14 : 30}%, var(--card))`, stroke: roleColor(role), transition: tr.colors }}
        strokeWidth={role === "idle" ? 1 : 2}
        strokeDasharray={outline(role).strokeDasharray}
      />
      <text x={x(start) + 4} y={y + LANE / 2 + 1} className="fill-foreground font-mono text-[10px]">
        {label ?? `${start}–${end}`}
      </text>
    </g>
  );

  return (
    <div ref={ref} className="relative w-full" data-testid="timeline-scene">
      <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        {bars.map((b, k) => (
          <g key={b.id} data-bar={b.id}>
            <text x={2} y={14 + k * LANE + LANE / 2 + 1} className="fill-muted-foreground font-mono text-[10px]">
              {b.id}
            </text>
            {bar(`b${b.id}`, b.start, b.end, 14 + k * LANE + 2, b.role ?? "idle")}
          </g>
        ))}
        <line x1={left} x2={left + span} y1={axisY} y2={axisY} stroke="var(--viz-edge)" strokeWidth={1} />
        {ticks.map((v) => (
          <g key={`t${v}`}>
            <line x1={x(v)} x2={x(v)} y1={axisY} y2={axisY + 4} stroke="var(--viz-edge)" />
            <text x={x(v)} y={axisY + 16} textAnchor="middle" className="fill-muted-foreground font-mono text-[10px]">
              {v}
            </text>
          </g>
        ))}
        {tl?.mark != null && (
          <line data-testid="timeline-mark" x1={x(tl.mark)} x2={x(tl.mark)} y1={6} y2={axisY} stroke={roleColor("key")} strokeWidth={2} strokeDasharray="5 4" />
        )}
        {tl?.output && (
          <g data-testid="row-output">
            <text x={0} y={outY + 8} className="fill-muted-foreground text-xs font-medium">
              {t(`learn.stage.rows.${tl.outputLabel ?? "output"}`)}
            </text>
            {out.map((o, k) => bar(`o${k}`, o.start, o.end, outY + 14, o.role ?? "sorted"))}
          </g>
        )}
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
