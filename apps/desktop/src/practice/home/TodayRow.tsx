// Home → the Today row (ADR 0041): today's session on the left; this week and the solved
// rings on the right. Colours come from the design tokens only.
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowRight, Check } from "lucide-react";
import type { Difficulty } from "@/bindings";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel, usePatternLabel } from "@/components/Semantic";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { useKataOverview } from "@/lib/kata";
import { usePracticePrefs, useStreak } from "@/lib/practice";
import { dayDate } from "@/lib/progress";
import { useContinue, useProblems } from "@/lib/queries";
import { relativeTime } from "@/lib/time";
import { cn } from "@/lib/utils";
import type { Session, SessionItem } from "@/practice/home/session";
import { useWeekActivity } from "@/practice/home/week";

export const CARD = "rounded-2xl border bg-card text-card-foreground";

// --- Today's session -------------------------------------------------------------------

export function SessionCard({
  session,
  isPending,
  error,
  onRetry,
}: {
  session: Session | undefined;
  isPending: boolean;
  error: unknown;
  onRetry: () => void;
}) {
  const { t } = useTranslation();
  return (
    <section
      aria-labelledby="home-session-title"
      className="flex flex-col gap-4 rounded-2xl border border-primary/30 bg-gradient-to-b from-primary/10 to-card to-70% p-6 text-card-foreground"
      data-testid="session-card"
    >
      <div className="flex items-center justify-between gap-3">
        <h2 id="home-session-title" className="text-xs font-semibold uppercase tracking-[0.08em] text-primary">
          {session && session.items.length > 0
            ? t("home.session.title", { minutes: session.totalMinutes })
            : t("home.session.label")}
        </h2>
        {session && session.items.length > 0 && (
          <span className="font-mono text-xs text-muted-foreground" data-testid="session-progress">
            {t("home.session.progress", { done: session.done, total: session.items.length })}
          </span>
        )}
      </div>
      {isPending ? (
        <div className="flex flex-col gap-2.5">
          {[0, 1, 2].map((i) => (
            <Skeleton key={i} className="h-14 w-full rounded-xl" />
          ))}
        </div>
      ) : error || !session ? (
        <ErrorNotice error={error} onRetry={onRetry} />
      ) : session.items.length === 0 ? (
        <p className="text-sm text-muted-foreground">{t("home.session.empty")}</p>
      ) : (
        <ol className="flex flex-col gap-2.5" data-testid="session-items">
          {session.items.map((item, i) => (
            <SessionRow key={`${item.kind}-${item.problemId ?? ""}`} item={item} n={i + 1} />
          ))}
        </ol>
      )}
      <div className="flex flex-wrap items-center gap-2.5">
        {session?.firstOpen ? (
          <Button asChild size="lg" className="h-11 rounded-xl px-5 text-[15px] font-bold">
            <Link {...session.firstOpen.route} data-testid="session-start">
              {t("home.session.start")}
              <ArrowRight aria-hidden />
            </Link>
          </Button>
        ) : (
          session &&
          session.items.length > 0 && <p className="text-sm font-medium text-verdict-accepted">{t("home.session.allDone")}</p>
        )}
        <Button asChild variant="outline" size="lg" className="h-11 rounded-xl bg-transparent">
          <Link to="/settings" search={{ section: "practice" }}>
            {t("home.session.customise")}
          </Link>
        </Button>
      </div>
    </section>
  );
}

function SessionRow({ item, n }: { item: SessionItem; n: number }) {
  const { t, i18n } = useTranslation();
  const patternLabel = usePatternLabel();
  const cont = useContinue();
  const title =
    item.kind === "kata" ? t("home.session.kind.kata") : t(`home.session.kind.${item.kind}`, { title: item.title ?? "" });
  let sub: React.ReactNode;
  if (item.kind === "continue") {
    const c = cont.data;
    sub =
      c && c.problem_id === item.problemId
        ? t("home.continueEdited", { language: t(`languages.${c.language}`), when: relativeTime(c.updated_at, i18n.language) })
        : t("home.session.sub.continue");
  } else if (item.kind === "kata") {
    sub = t("home.session.sub.kata");
  } else if (item.kind === "watch" || item.kind === "startWatch") {
    sub = t(`home.session.sub.${item.kind}`);
  } else {
    sub = (
      <>
        {item.difficulty && <DifficultyLabel difficulty={item.difficulty} className="text-[13px]" />}
        {item.pattern && <span> · {patternLabel(item.pattern)}</span>}
        <span> · {t(`home.session.sub.${item.kind}`)}</span>
      </>
    );
  }
  return (
    <li data-testid={`session-item-${item.kind}`} data-done={item.doneToday || undefined}>
      <Link
        {...item.route}
        className="flex items-center gap-3.5 rounded-xl border bg-background px-4 py-3 transition-colors hover:border-primary/50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      >
        {item.doneToday ? (
          <span className="flex size-7 flex-none items-center justify-center rounded-full bg-verdict-accepted text-background">
            <Check aria-hidden className="size-4" strokeWidth={3} />
            <span className="sr-only">{t("home.session.doneLabel")}</span>
          </span>
        ) : (
          <span
            aria-hidden
            className="flex size-7 flex-none items-center justify-center rounded-full bg-primary/15 text-[13px] font-semibold text-primary"
          >
            {n}
          </span>
        )}
        <span className="min-w-0 flex-1">
          <span className={cn("block truncate font-semibold", item.doneToday && "text-muted-foreground line-through")}>
            {title}
          </span>
          <span className="block truncate text-[13px] text-muted-foreground">{sub}</span>
        </span>
        {item.onPath && (
          <span
            className="flex-none rounded-full bg-primary/15 px-2.5 py-0.5 text-[11px] font-semibold text-primary"
            data-testid="on-path-badge"
          >
            {t("home.session.onPath")}
          </span>
        )}
        <span className="flex-none font-mono text-xs text-muted-foreground">
          {t("home.session.minutes", { minutes: item.minutes })}
        </span>
      </Link>
    </li>
  );
}

// --- This week -------------------------------------------------------------------------

export function WeekCard() {
  const { t, i18n } = useTranslation();
  const week = useWeekActivity();
  const { daily, rusty } = usePracticePrefs();
  const streak = useStreak();
  const kata = useKataOverview(rusty);
  const narrow = new Intl.DateTimeFormat(i18n.language, { weekday: "narrow" });
  const long = new Intl.DateTimeFormat(i18n.language, { weekday: "long" });
  return (
    <section aria-labelledby="home-week-title" className={cn(CARD, "p-5")} data-testid="week-card">
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <h2 id="home-week-title" className="text-[13px] font-semibold">
          {t("home.week.title")}
        </h2>
        <p className="text-xs text-muted-foreground">
          {daily && (
            <>
              {t("home.week.daily")}{" "}
              <b className="text-difficulty-medium tabular-nums" data-testid="week-daily-streak">
                {streak.data ?? 0}
              </b>
            </>
          )}
          {daily && rusty && " · "}
          {rusty && (
            <>
              {t("home.week.kata")}{" "}
              <b className="text-foreground tabular-nums" data-testid="week-kata-streak">
                {kata.data?.streak ?? 0}
              </b>
            </>
          )}
        </p>
      </div>
      {week.isPending ? (
        <Skeleton className="mt-3.5 h-12 w-full" />
      ) : week.error || !week.days ? (
        <div className="mt-3.5">
          <ErrorNotice error={week.error} onRetry={week.refetch} />
        </div>
      ) : (
        <ol className="mt-3.5 grid grid-cols-7 gap-1.5 text-center text-[11px] text-muted-foreground" data-testid="week-days">
          {week.days.map((d) => {
            const date = dayDate(d.day);
            const label = t(d.submissions > 0 ? "home.week.dayActive" : "home.week.dayIdle", {
              day: long.format(date),
              count: d.submissions,
            });
            return (
              <li key={d.day} className="flex flex-col items-center gap-1.5">
                <span
                  role="img"
                  aria-label={d.today ? `${label} (${t("home.week.today")})` : label}
                  data-active={d.submissions > 0 || undefined}
                  data-today={d.today || undefined}
                  className={cn(
                    "size-[26px] rounded-lg",
                    d.submissions > 0 ? "bg-primary" : "bg-accent",
                    d.today && "ring-2 ring-difficulty-medium ring-offset-2 ring-offset-card",
                  )}
                />
                <span aria-hidden>{narrow.format(date)}</span>
              </li>
            );
          })}
        </ol>
      )}
    </section>
  );
}

// --- Solved rings ----------------------------------------------------------------------

/** A ring with any solved problem shows at least a 6° arc, not a dot. */
export const MIN_ARC = 6 / 360;

const RINGS: { d: Difficulty; r: number; w: number; stroke: string }[] = [
  { d: "easy", r: 50, w: 10, stroke: "stroke-difficulty-easy" },
  { d: "medium", r: 36, w: 8, stroke: "stroke-difficulty-medium" },
  { d: "hard", r: 24, w: 6, stroke: "stroke-difficulty-hard" },
];

export function SolvedCard() {
  const { t } = useTranslation();
  const problems = useProblems();
  const counts = RINGS.map((ring) => {
    const all = (problems.data ?? []).filter((p) => p.difficulty === ring.d);
    return { ...ring, total: all.length, solved: all.filter((p) => p.status === "solved").length };
  });
  const solved = counts.reduce((s, c) => s + c.solved, 0);
  const total = counts.reduce((s, c) => s + c.total, 0);
  return (
    <section
      aria-labelledby="home-solved-title"
      className={cn(CARD, "flex flex-1 flex-col p-5")}
      data-testid="solved-card"
    >
      <h2 id="home-solved-title" className="text-[13px] font-semibold">
        {t("home.solved")}
      </h2>
      <div className="flex flex-1 items-center gap-5 pt-3">
        {problems.isPending ? (
          <Skeleton className="h-[104px] w-full" />
        ) : problems.error ? (
          <ErrorNotice error={problems.error} onRetry={() => void problems.refetch()} />
        ) : (
          <>
            <svg width="104" height="104" viewBox="0 0 120 120" role="img" aria-label={t("home.rings.aria", { solved, total })}>
              {counts.map((c) => {
                const len = 2 * Math.PI * c.r;
                const frac = c.total > 0 ? c.solved / c.total : 0;
                return (
                  <g key={c.d}>
                    <circle cx="60" cy="60" r={c.r} fill="none" className="stroke-accent" strokeWidth={c.w} />
                    {frac > 0 && (
                      <circle
                        cx="60"
                        cy="60"
                        r={c.r}
                        fill="none"
                        className={c.stroke}
                        strokeWidth={c.w}
                        strokeLinecap="round"
                        strokeDasharray={`${Math.max(frac * len, MIN_ARC * len)} ${len}`}
                        transform="rotate(-90 60 60)"
                      />
                    )}
                  </g>
                );
              })}
              <text x="60" y="58" textAnchor="middle" className="fill-foreground text-[22px] font-bold">
                {solved}
              </text>
              <text x="60" y="74" textAnchor="middle" className="fill-muted-foreground text-[10px]">
                {t("home.rings.of", { total })}
              </text>
            </svg>
            <div className="flex flex-1 flex-col gap-2 text-[13px]">
              {counts.map((c) => (
                <div key={c.d} className="flex items-center justify-between gap-2">
                  <DifficultyLabel difficulty={c.d} />
                  <span className="tabular-nums" data-testid={`solved-${c.d}`}>
                    {t("home.solvedOf", { solved: c.solved, total: c.total })}
                  </span>
                </div>
              ))}
              <Link to="/progress" className="text-xs text-primary hover:underline">
                {t("home.rings.open")} →
              </Link>
            </div>
          </>
        )}
      </div>
    </section>
  );
}
