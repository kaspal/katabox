// Big-O growth (ADR 0029, Phase 1): the step counts of six loops for n = 1…16, one curve
// per growth class, revealed one at a time by the steps. Each curve has its own dash
// pattern and a label at its end, so colour is never the only cue. The n slider below is
// the scene's own (it does not change the input): it ranks the shown curves at that n.
import { useId, useState } from "react";
import { useTranslation } from "react-i18next";
import { GROWTH_CLASSES, GROWTH_MAX_N } from "../engine/growth";
import { SceneFallback, sceneSummary, useWidth, type SceneProps } from "./common";

const Y_MAX = 260;

/** Colour and dash per growth class (in GROWTH_CLASSES order); labels tell them apart too. */
const STYLE = [
  { color: "var(--viz-sorted)", dash: undefined },
  { color: "var(--viz-ptr-i)", dash: "2 4" },
  { color: "var(--viz-pivot)", dash: "8 4" },
  { color: "var(--viz-key)", dash: "12 4 2 4" },
  { color: "var(--viz-compare)", dash: "4 2" },
  { color: "var(--viz-swap)", dash: "1 3" },
] as const;
const GROWTH = GROWTH_CLASSES.map((g, k) => ({ ...g, ...STYLE[k]! }));

export function GrowthScene({ step, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const n = step.chart?.n ?? 1;
  const shown = step.chart?.shown ?? 0;
  const [probe, setProbe] = useState(n);
  const id = useId();
  const pad = { l: 44, r: 86, t: 26, b: 34 };
  const height = 300;
  const w = Math.max(160, width - pad.l - pad.r);
  const h = height - pad.t - pad.b;
  const X = (x: number) => pad.l + ((x - 1) / (GROWTH_MAX_N - 1)) * w;
  const Y = (y: number) => pad.t + h - (Math.min(y, Y_MAX) / Y_MAX) * h;
  const ns = Array.from({ length: GROWTH_MAX_N }, (_, k) => k + 1);
  const visible = GROWTH.slice(0, shown);
  // Each label sits where its curve leaves the plot (top edge or n = 16), pushed apart so
  // labels that end close together (O(1), O(log n), O(n)) never overlap.
  const labels = visible
    .map((g) => {
      const last = ns.filter((x) => g.f(x) <= Y_MAX).at(-1) ?? 1;
      return { label: g.label, color: g.color, x: Math.min(X(last) + 6, pad.l + w + 6), y: Math.max(Y(g.f(last)) + 4, pad.t + 10) };
    })
    .sort((a, b) => b.y - a.y);
  for (let k = 1; k < labels.length; k++) {
    const prev = labels[k - 1]!;
    const cur = labels[k]!;
    if (Math.abs(cur.x - prev.x) < 70 && prev.y - cur.y < 13) cur.y = prev.y - 13;
  }
  const ranked = visible
    .map((g) => ({ label: g.label, count: g.f(probe) }))
    .sort((a, b) => a.count - b.count);

  return (
    <div ref={ref} className="relative flex w-full flex-col gap-2" data-testid="growth-scene">
      <svg
        role="img"
        aria-label={sceneSummary(t, step)}
        width={width}
        height={height}
        viewBox={`0 0 ${width} ${height}`}
        className="block overflow-visible"
      >
        <defs>
          <clipPath id={`${id}-plot`}>
            <rect x={pad.l} y={pad.t - 2} width={w + 2} height={h + 4} />
          </clipPath>
        </defs>
        {[0, 64, 128, 192, 256].map((v) => (
          <g key={v} aria-hidden>
            <line x1={pad.l} x2={pad.l + w} y1={Y(v)} y2={Y(v)} stroke="var(--border)" />
            <text x={pad.l - 8} y={Y(v) + 4} textAnchor="end" className="fill-muted-foreground font-mono text-[10px]">
              {v}
            </text>
          </g>
        ))}
        {[1, 4, 8, 12, 16].map((v) => (
          <text key={v} x={X(v)} y={pad.t + h + 18} textAnchor="middle" aria-hidden className="fill-muted-foreground font-mono text-[10px]">
            {v}
          </text>
        ))}
        <text x={pad.l + w / 2} y={height - 2} textAnchor="middle" className="fill-muted-foreground text-xs">
          {t("learn.stage.growth_x")}
        </text>
        <text x={pad.l - 8} y={12} textAnchor="end" className="fill-muted-foreground text-xs">
          {t("learn.stage.growth_y")}
        </text>
        <line x1={X(n)} x2={X(n)} y1={pad.t} y2={pad.t + h} stroke="var(--foreground)" strokeDasharray="3 3" />
        <text x={X(n) + 4} y={pad.t + 12} className="fill-foreground font-mono text-xs font-semibold">
          n = {n}
        </text>
        <g clipPath={`url(#${id}-plot)`}>
          {visible.map((g, k) => (
            <polyline
              key={g.label}
              points={ns.map((x) => `${X(x)},${Y(g.f(x))}`).join(" ")}
              fill="none"
              stroke={g.color}
              strokeWidth={k === shown - 1 ? 3.5 : 2}
              strokeDasharray={g.dash}
              data-curve={g.label}
            />
          ))}
        </g>
        {labels.map((l) => (
          <text key={l.label} x={l.x} y={l.y} className="font-mono text-xs font-bold" style={{ fill: l.color }}>
            {l.label}
          </text>
        ))}
      </svg>
      <div className="flex flex-wrap items-center gap-3 text-sm">
        <label className="flex items-center gap-2 text-muted-foreground" htmlFor={`${id}-probe`}>
          {t("learn.stage.growth_probe")}
          <input
            id={`${id}-probe`}
            type="range"
            min={1}
            max={GROWTH_MAX_N}
            value={probe}
            onChange={(e) => setProbe(Number(e.target.value))}
            className="w-36 accent-primary"
          />
          <span className="w-14 whitespace-nowrap font-mono text-foreground tabular-nums">n = {probe}</span>
        </label>
        {ranked.length > 0 && (
          <p className="text-sm" data-testid="growth-readout">
            {t("learn.stage.growth_readout", { n: probe })}{" "}
            <span className="font-mono">{ranked.map((r) => `${r.label} ${r.count}`).join(" ≤ ")}</span>
          </p>
        )}
      </div>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
