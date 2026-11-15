import { useTranslation } from "react-i18next";
import type { TFunction } from "i18next";
import { ChartSpline, Loader2, RotateCcw } from "lucide-react";
import type { ComplexityClass, Estimate, ProfileStop, ProfileView } from "@/bindings";
import { Button } from "@/components/ui/button";
import { ErrorNotice } from "@/components/ErrorNotice";

/** The complexity profiler's state for the last Accepted result (ADR 0024). */
export type ComplexityState =
  | { status: "idle" }
  | { status: "running" }
  | { status: "done"; view: ProfileView }
  | { status: "error"; error: unknown };

const NOTATION: Record<ComplexityClass, string> = {
  constant: "O(1)",
  logarithmic: "O(log n)",
  linear: "O(n)",
  linearithmic: "O(n log n)",
  quadratic: "O(n²)",
  exponential: "O(2ⁿ)",
};

export const notation = (c: ComplexityClass) => NOTATION[c];

/** "Estimated: O(n)" or "Estimated: O(n) or O(n log n)": never without "estimated". */
export function estimateLabel(t: TFunction, e: Estimate): string {
  return e.close_alternative
    ? t("complexity.estimatedOr", { class: notation(e.class), other: notation(e.close_alternative) })
    : t("complexity.estimated", { class: notation(e.class) });
}

function confidenceText(t: TFunction, e: Estimate): string {
  if (e.close_alternative || e.confidence < 0.3) return t("complexity.confidence.low");
  return e.confidence >= 0.6 ? t("complexity.confidence.high") : t("complexity.confidence.medium");
}

function stopText(t: TFunction, stop: ProfileStop, lang: string): string {
  switch (stop.reason) {
    case "budget":
      return t("complexity.stopped.budget", { n: stop.n.toLocaleString(lang) });
    case "time_limit":
      return t("complexity.stopped.time_limit", { n: stop.n.toLocaleString(lang) });
    case "failed":
      return t("complexity.stopped.failed", {
        n: stop.n.toLocaleString(lang),
        verdict: t(`judge.verdict.${stop.verdict}`),
      });
    case "compile":
      return t("complexity.stopped.compile");
  }
}

/** Milliseconds with sensible precision (0.004, 0.25, 12.3, 1520). */
export function msText(us: number): string {
  const ms = us / 1000;
  if (ms < 0.1) return ms.toFixed(3);
  if (ms < 10) return ms.toFixed(2);
  if (ms < 100) return ms.toFixed(1);
  return ms.toFixed(0);
}

function sizeTick(n: number): string {
  if (n >= 1e6) return `${n / 1e6}M`;
  if (n >= 1e3) return `${n / 1e3}k`;
  return String(n);
}

function timeTick(us: number): string {
  if (us >= 1e6) return `${us / 1e6} s`;
  if (us >= 1e3) return `${us / 1e3} ms`;
  return `${us} µs`;
}

type Pt = { n: number; us: number };

const W = 400;
const H = 220;
const M = { left: 52, right: 14, top: 12, bottom: 40 };

/** The decades covering [min, max] (at least one decade wide). */
function decades(min: number, max: number): [number, number] {
  const lo = Math.floor(Math.log10(Math.max(1, min)));
  const hi = Math.max(lo + 1, Math.ceil(Math.log10(Math.max(1, max))));
  return [lo, hi];
}

/** Every decade between lo and hi, thinned to at most 6 labels. */
function ticks(lo: number, hi: number): number[] {
  const step = Math.ceil((hi - lo + 1) / 6);
  const out: number[] = [];
  for (let d = lo; d <= hi; d += step) out.push(10 ** d);
  return out;
}

function Line({ points, x, y, dash, marker, testId }: {
  points: Pt[];
  x: (n: number) => number;
  y: (us: number) => number;
  dash?: string;
  marker: "circle" | "square";
  testId: string;
}) {
  if (points.length === 0) return null;
  const d = points.map((p, i) => `${i ? "L" : "M"}${x(p.n).toFixed(1)},${y(p.us).toFixed(1)}`).join(" ");
  return (
    <g data-testid={testId}>
      <path d={d} fill="none" stroke="currentColor" strokeWidth={2} strokeDasharray={dash} />
      {points.map((p) =>
        marker === "circle" ? (
          <circle key={p.n} cx={x(p.n)} cy={y(p.us)} r={3.5} fill="currentColor" />
        ) : (
          <rect key={p.n} x={x(p.n) - 3.5} y={y(p.us) - 3.5} width={7} height={7} fill="currentColor" />
        ),
      )}
    </g>
  );
}

/** A log-log chart of time per call against n: your line (solid, circles) and the
 * reference (dashed, squares), so colour is never the only cue. */
export function ComplexityChart({ view, label }: { view: ProfileView; label: string }) {
  const { t } = useTranslation();
  const you: Pt[] = view.points.map((p) => ({ n: p.n, us: Math.max(1, p.median_us) }));
  const ref: Pt[] = view.reference.map((r) => ({ n: r.n, us: Math.max(1, r.us) }));
  const all = [...you, ...ref];
  if (all.length === 0) return null;
  const [xlo, xhi] = decades(Math.min(...all.map((p) => p.n)), Math.max(...all.map((p) => p.n)));
  const [ylo, yhi] = decades(Math.min(...all.map((p) => p.us)), Math.max(...all.map((p) => p.us)));
  const x = (n: number) => M.left + ((Math.log10(n) - xlo) / (xhi - xlo)) * (W - M.left - M.right);
  const y = (us: number) => H - M.bottom - ((Math.log10(us) - ylo) / (yhi - ylo)) * (H - M.top - M.bottom);
  return (
    <svg
      viewBox={`0 0 ${W} ${H}`}
      role="img"
      aria-label={label}
      className="h-auto w-full max-w-md text-muted-foreground"
      data-testid="complexity-chart"
    >
      {ticks(xlo, xhi).map((n) => (
        <g key={`x${n}`}>
          <line x1={x(n)} x2={x(n)} y1={M.top} y2={H - M.bottom} stroke="currentColor" strokeOpacity={0.2} />
          <text x={x(n)} y={H - M.bottom + 14} textAnchor="middle" fontSize={10} fill="currentColor">
            {sizeTick(n)}
          </text>
        </g>
      ))}
      {ticks(ylo, yhi).map((us) => (
        <g key={`y${us}`}>
          <line x1={M.left} x2={W - M.right} y1={y(us)} y2={y(us)} stroke="currentColor" strokeOpacity={0.2} />
          <text x={M.left - 6} y={y(us) + 3} textAnchor="end" fontSize={10} fill="currentColor">
            {timeTick(us)}
          </text>
        </g>
      ))}
      <text x={(M.left + W - M.right) / 2} y={H - 6} textAnchor="middle" fontSize={10} fill="currentColor">
        {t("complexity.axisN")}
      </text>
      <text
        transform={`translate(12 ${(M.top + H - M.bottom) / 2}) rotate(-90)`}
        textAnchor="middle"
        fontSize={10}
        fill="currentColor"
      >
        {t("complexity.axisTime")}
      </text>
      <g className="text-chart-ref">
        <Line points={ref} x={x} y={y} dash="6 4" marker="square" testId="line-ref" />
      </g>
      <g className="text-chart-you">
        <Line points={you} x={x} y={y} marker="circle" testId="line-you" />
      </g>
    </svg>
  );
}

function Legend() {
  const { t } = useTranslation();
  const swatch = (dash: string | undefined, square: boolean) => (
    <svg aria-hidden width={28} height={10} className="shrink-0">
      <line x1={0} x2={28} y1={5} y2={5} stroke="currentColor" strokeWidth={2} strokeDasharray={dash} />
      {square ? <rect x={10.5} y={1.5} width={7} height={7} fill="currentColor" /> : <circle cx={14} cy={5} r={3.5} fill="currentColor" />}
    </svg>
  );
  return (
    <ul className="flex flex-wrap gap-x-4 gap-y-1 text-xs">
      <li className="flex items-center gap-1.5">
        <span className="text-chart-you">{swatch(undefined, false)}</span>
        {t("complexity.legendYou")}
      </li>
      <li className="flex items-center gap-1.5">
        <span className="text-chart-ref">{swatch("6 4", true)}</span>
        {t("complexity.legendRef")}
      </li>
    </ul>
  );
}

/** The finished profile: label, chart, explanation and measurements. */
export function ComplexityResult({ view }: { view: ProfileView }) {
  const { t, i18n } = useTranslation();
  const label = view.estimate
    ? estimateLabel(t, view.estimate)
    : view.fast
      ? t("complexity.fast")
      : view.unreliable
        ? t("complexity.unreliable")
        : t("complexity.notEnough");
  const refLabel = view.reference_estimate
    ? t("complexity.reference", { class: notation(view.reference_estimate.class) })
    : view.reference_fast
      ? t("complexity.referenceFast")
      : null;
  const largest = view.points.at(-1)?.n ?? 0;
  const summary = [label, refLabel].filter(Boolean).join(". ");
  const measured = new Map(view.points.map((p) => [p.n, p.median_us]));
  const sizes = [...new Set([...view.reference.map((r) => r.n), ...view.points.map((p) => p.n)])].sort((a, b) => a - b);
  const refAt = new Map(view.reference.map((r) => [r.n, r.us]));
  return (
    <div className="space-y-3" data-testid="complexity-result">
      <div>
        <p className="text-base font-semibold" data-testid="complexity-label">
          {label}
        </p>
        {refLabel && <p className="text-sm text-muted-foreground">{refLabel}</p>}
      </div>
      {view.estimate && (
        <p className="text-sm">
          {confidenceText(t, view.estimate)}
          {view.estimate.noisy && <> {t("complexity.noisy")}</>}
        </p>
      )}
      {view.unreliable && (
        <p className="text-sm text-muted-foreground" data-testid="complexity-unreliable">
          {view.fast
            ? t("complexity.fastWhy", { n: largest.toLocaleString(i18n.language), startup: Math.round(view.startup_us / 1000) })
            : t("complexity.unreliableWhy", { startup: Math.round(view.startup_us / 1000) })}
        </p>
      )}
      {view.startup_us > 0 && !view.unreliable && (
        <p className="text-xs text-muted-foreground">{t("complexity.startup", { startup: Math.round(view.startup_us / 1000) })}</p>
      )}
      {view.stopped && (
        <p className="text-sm text-muted-foreground" data-testid="complexity-stopped">
          {stopText(t, view.stopped, i18n.language)}
        </p>
      )}
      {view.stopped?.reason === "compile" && view.stopped.output && (
        <pre className="max-h-40 overflow-auto rounded-md bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap">
          {view.stopped.output}
        </pre>
      )}
      <figure className="space-y-2">
        <ComplexityChart view={view} label={t("complexity.chartLabel", { summary })} />
        <figcaption>
          <Legend />
        </figcaption>
      </figure>
      <p className="text-xs text-muted-foreground">{t("complexity.explanation")}</p>
      <table className="w-full max-w-md text-sm" data-testid="complexity-table">
        <caption className="mb-1 text-left text-xs text-muted-foreground">{t("complexity.table.caption")}</caption>
        <thead className="text-left text-xs text-muted-foreground">
          <tr>
            <th scope="col" className="py-1 pr-3 font-medium">{t("complexity.table.n")}</th>
            <th scope="col" className="py-1 pr-3 text-right font-medium">{t("complexity.table.you")}</th>
            <th scope="col" className="py-1 text-right font-medium">{t("complexity.table.ref")}</th>
          </tr>
        </thead>
        <tbody>
          {sizes.map((n) => {
            const you = measured.get(n);
            const ref = refAt.get(n);
            return (
              <tr key={n} className="border-t">
                <th scope="row" className="py-1 pr-3 text-left font-normal tabular-nums">{n.toLocaleString(i18n.language)}</th>
                <td className="py-1 pr-3 text-right tabular-nums">
                  {you === undefined ? <span className="text-muted-foreground">{t("complexity.table.notRun")}</span> : msText(you)}
                </td>
                <td className="py-1 text-right tabular-nums">{ref === undefined ? "–" : msText(ref)}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}

/** The Complexity action under an Accepted result, and what it measured. */
export function ComplexityPanel({
  state,
  onMeasure,
  disabled = false,
}: {
  state: ComplexityState;
  onMeasure: () => void;
  disabled?: boolean;
}) {
  const { t } = useTranslation();
  return (
    <section className="space-y-3 rounded-lg border p-3" aria-labelledby="complexity-title" data-testid="complexity-panel">
      <div className="flex flex-wrap items-center gap-2">
        <h4 id="complexity-title" className="text-sm font-semibold">
          {t("complexity.title")}
        </h4>
        {state.status !== "running" && (
          <Button
            size="sm"
            variant="outline"
            className="ml-auto"
            onClick={onMeasure}
            disabled={disabled}
            title={t("complexity.buttonHint")}
          >
            {state.status === "idle" ? <ChartSpline aria-hidden /> : <RotateCcw aria-hidden />}
            {state.status === "idle" ? t("complexity.button") : t("complexity.again")}
          </Button>
        )}
      </div>
      {state.status === "idle" && <p className="text-sm text-muted-foreground">{t("complexity.buttonHint")}</p>}
      {state.status === "running" && (
        <p className="flex items-center gap-2 text-sm text-muted-foreground" aria-live="polite" data-testid="complexity-running">
          <Loader2 aria-hidden className="size-4 animate-spin" />
          {t("complexity.measuring")}
        </p>
      )}
      {state.status === "error" && <ErrorNotice error={state.error} />}
      {state.status === "done" && <ComplexityResult view={state.view} />}
    </section>
  );
}
