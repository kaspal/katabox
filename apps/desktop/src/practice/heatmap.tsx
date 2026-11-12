// Progress → activity heatmap (ADR 0021): the last 12 months, one cell per local day,
// colored by Submits. Accessible: an ARIA grid (weekdays are rows, weeks are columns)
// with one focusable cell at a time; every cell has a label with its numbers, and the
// focused or hovered day is also written out under the grid. Color is never the only cue.
import { useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { type DayActivity, dayDate, heatLevel } from "@/lib/progress";
import { cn } from "@/lib/utils";

const LEVEL_CLASS = ["bg-heat-0", "bg-heat-1", "bg-heat-2", "bg-heat-3", "bg-heat-4"] as const;
const NONE: DayActivity = { submissions: 0, solved: 0 };

/** The text of one day, for its cell label and the detail line. */
export function useDayLabel() {
  const { t, i18n } = useTranslation();
  const fmt = useMemo(
    () => new Intl.DateTimeFormat(i18n.language, { weekday: "long", year: "numeric", month: "short", day: "numeric" }),
    [i18n.language],
  );
  return (day: string, a: DayActivity) => {
    const date = fmt.format(dayDate(day));
    return a.submissions === 0
      ? t("progress.heatmap.empty", { date })
      : t("progress.heatmap.day", {
          date,
          submissions: t("progress.heatmap.submissions", { count: a.submissions }),
          solved: t("progress.heatmap.solved", { count: a.solved }),
        });
  };
}

export function Heatmap({ days, data }: { days: string[]; data: Map<string, DayActivity> }) {
  const { t, i18n } = useTranslation();
  const label = useDayLabel();
  const max = useMemo(() => Math.max(0, ...[...data.values()].map((d) => d.submissions)), [data]);
  const last = days.length - 1;
  const [focus, setFocus] = useState(last);
  const [hover, setHover] = useState<number | null>(null);
  const cells = useRef<(HTMLDivElement | null)[]>([]);
  const weeks = Math.ceil(days.length / 7);
  const monthFmt = useMemo(() => new Intl.DateTimeFormat(i18n.language, { month: "short" }), [i18n.language]);
  const weekdayFmt = useMemo(() => new Intl.DateTimeFormat(i18n.language, { weekday: "short" }), [i18n.language]);

  const at = Math.min(focus, last);
  const shown = hover ?? at;
  const move = (to: number) => {
    const i = Math.max(0, Math.min(last, to));
    setFocus(i);
    setHover(null);
    cells.current[i]?.focus();
  };
  const onKeyDown = (e: React.KeyboardEvent) => {
    const step: Record<string, number> = {
      ArrowRight: 7,
      ArrowLeft: -7,
      ArrowDown: 1,
      ArrowUp: -1,
      PageDown: 28,
      PageUp: -28,
    };
    if (e.key in step) move(at + step[e.key]);
    else if (e.key === "Home") move(0);
    else if (e.key === "End") move(last);
    else return;
    e.preventDefault();
  };

  // A month label above the week in which that month starts (and the first week).
  const months = Array.from({ length: weeks }, (_, w) => {
    const week = days.slice(w * 7, w * 7 + 7);
    const start = week.find((d) => d.endsWith("-01")) ?? (w === 0 ? week[0] : undefined);
    return start ? monthFmt.format(dayDate(start)) : null;
  });

  return (
    <div className="space-y-2" data-testid="heatmap">
      <div className="overflow-x-auto pb-1">
        <div
          role="grid"
          aria-label={t("progress.heatmap.label")}
          aria-describedby="heatmap-help"
          className="inline-grid gap-[3px] text-[10px] text-muted-foreground"
          style={{ gridTemplateColumns: `auto repeat(${weeks}, 11px)`, gridTemplateRows: "auto repeat(7, 11px)" }}
          onKeyDown={onKeyDown}
          onMouseLeave={() => setHover(null)}
        >
          {months.map(
            (m, w) =>
              m && (
                <span key={`m${w}`} aria-hidden className="whitespace-nowrap" style={{ gridColumn: w + 2, gridRow: 1 }}>
                  {m}
                </span>
              ),
          )}
          {Array.from({ length: 7 }, (_, r) => (
            <div role="row" key={r} className="contents">
              <span aria-hidden className="pr-1 leading-[11px]" style={{ gridColumn: 1, gridRow: r + 2 }}>
                {r === 0 || r === 2 || r === 4 ? weekdayFmt.format(dayDate(days[r] ?? days[0])) : ""}
              </span>
              {Array.from({ length: weeks }, (_, w) => {
                const i = w * 7 + r;
                const day = days[i];
                const pos = { gridColumn: w + 2, gridRow: r + 2 };
                if (day === undefined) return <span key={w} role="presentation" style={pos} />;
                const a = data.get(day) ?? NONE;
                return (
                  <div
                    key={w}
                    ref={(el) => {
                      cells.current[i] = el;
                    }}
                    role="gridcell"
                    tabIndex={i === at ? 0 : -1}
                    aria-label={label(day, a)}
                    aria-selected={i === at}
                    data-day={day}
                    data-level={heatLevel(a.submissions, max)}
                    className={cn(
                      "rounded-[2px] outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-1",
                      LEVEL_CLASS[heatLevel(a.submissions, max)],
                    )}
                    style={pos}
                    onFocus={() => setFocus(i)}
                    onMouseEnter={() => setHover(i)}
                    onClick={() => move(i)}
                  />
                );
              })}
            </div>
          ))}
        </div>
      </div>
      <div className="flex flex-wrap items-center justify-between gap-2 text-xs text-muted-foreground">
        <p className="min-h-4 text-sm text-foreground" data-testid="heatmap-detail">
          {label(days[shown], data.get(days[shown]) ?? NONE)}
        </p>
        <span className="inline-flex items-center gap-1" aria-hidden>
          {t("progress.heatmap.less")}
          {LEVEL_CLASS.map((c) => (
            <span key={c} className={cn("inline-block size-[11px] rounded-[2px]", c)} />
          ))}
          {t("progress.heatmap.more")}
        </span>
      </div>
      <p id="heatmap-help" className="sr-only">
        {t("progress.heatmap.help")}
      </p>
    </div>
  );
}
