// Skill rating (expansion M2, ADR 0033): the rating with its "Estimated" note and a
// small sparkline. The rating is a guide, not a grade.
import { useTranslation } from "react-i18next";
import { Gauge } from "lucide-react";
import type { RatingView } from "@/bindings";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";

/** A small line of the rating's last points; the numbers are in its accessible name. */
export function Sparkline({ points, width = 80, height = 20 }: { points: number[]; width?: number; height?: number }) {
  const { t } = useTranslation();
  if (points.length < 2) return null;
  const min = Math.min(...points);
  const max = Math.max(...points);
  const span = max - min || 1;
  const xy = points.map((p, i) => [
    (i / (points.length - 1)) * (width - 2) + 1,
    height - 1 - ((p - min) / span) * (height - 2),
  ]);
  const d = xy.map(([x, y], i) => `${i === 0 ? "M" : "L"}${x.toFixed(1)},${y.toFixed(1)}`).join(" ");
  return (
    <svg
      role="img"
      aria-label={t("rating.trend", { from: points[0], to: points[points.length - 1] })}
      width={width}
      height={height}
      viewBox={`0 0 ${width} ${height}`}
      className="text-primary"
      data-testid="rating-sparkline"
    >
      <path d={d} fill="none" stroke="currentColor" strokeWidth={1.5} strokeLinejoin="round" />
    </svg>
  );
}

/** "1 234 (Estimated)" with a tooltip that says what the number is. */
export function RatingBadge({ view, compact = false }: { view: RatingView; compact?: boolean }) {
  const { t, i18n } = useTranslation();
  const n = new Intl.NumberFormat(i18n.language).format(view.rating);
  return (
    <span className="inline-flex items-center gap-2" data-testid="rating">
      <Tooltip>
        <TooltipTrigger asChild>
          <span
            tabIndex={0}
            className="inline-flex items-center gap-1 rounded-md border px-1.5 py-0.5 text-xs tabular-nums"
            aria-label={t("rating.label", { rating: n })}
          >
            <Gauge aria-hidden className="size-3.5" />
            {n}
            <span className="text-muted-foreground">{t("rating.estimated")}</span>
          </span>
        </TooltipTrigger>
        <TooltipContent className="max-w-64">{t("rating.help", { count: view.events })}</TooltipContent>
      </Tooltip>
      {!compact && <Sparkline points={view.history.map((p) => p.rating)} />}
    </span>
  );
}
