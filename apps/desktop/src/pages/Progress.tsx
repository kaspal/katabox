// Progress (ADR 0019, 0021): totals, the 12-month activity heatmap, solved by
// difficulty, the review queue, solved by language and by topic, the pattern map with
// each pattern's next problem, past mock interviews, and the daily streak.
import { useMemo } from "react";
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowRight, CheckCircle2 } from "lucide-react";
import type { ReactNode } from "react";
import { PatternLearnLinks } from "@/components/PatternLearnLinks";
import { Page } from "@/components/Page";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel, usePatternLabel, useTagLabel } from "@/components/Semantic";
import { Button } from "@/components/ui/button";
import { Progress as Bar } from "@/components/ui/progress";
import { Skeleton } from "@/components/ui/skeleton";
import { usePatterns, useStreak } from "@/lib/practice";
import { useInterviews, useSolvedByLanguage } from "@/lib/modes";
import { bucketByDay, heatmapDays, useActivity, useReviewSummary, useSolvedBreakdown } from "@/lib/progress";
import { localToday, useToday } from "@/lib/today";
import { Heatmap } from "@/practice/heatmap";
import { StreakBadge, formatDay } from "@/practice/shared";
import { Scorecard } from "@/practice/interview";

function Section({ title, children, testId }: { title: string; children: ReactNode; testId?: string }) {
  return (
    <section className="rounded-lg border bg-card p-4" data-testid={testId}>
      <h2 className="mb-3 text-sm font-semibold text-muted-foreground">{title}</h2>
      {children}
    </section>
  );
}

function Stat({ label, value, testId }: { label: string; value: ReactNode; testId: string }) {
  return (
    <div className="rounded-lg border bg-card p-3" data-testid={testId}>
      <p className="text-sm text-muted-foreground">{label}</p>
      <p className="text-2xl font-semibold tabular-nums">{value}</p>
    </div>
  );
}

function CountBar({ label, name, solved, total }: { label: ReactNode; name: string; solved: number; total: number }) {
  const { t } = useTranslation();
  const text = t("practice.patterns.solved", { solved, total });
  return (
    <div className="space-y-1">
      <div className="flex items-center justify-between gap-2 text-sm">
        <span className="min-w-0 truncate">{label}</span>
        <span className="shrink-0 tabular-nums text-muted-foreground">{text}</span>
      </div>
      <Bar value={total === 0 ? 0 : (100 * solved) / total} aria-label={`${name}: ${text}`} />
    </div>
  );
}

export function Progress() {
  const { t, i18n } = useTranslation();
  const patternLabel = usePatternLabel();
  const tagLabel = useTagLabel();
  const today = useToday();
  const patterns = usePatterns();
  const streak = useStreak();
  const byLanguage = useSolvedByLanguage();
  const interviews = useInterviews();
  const activity = useActivity();
  const breakdown = useSolvedBreakdown();
  const reviews = useReviewSummary();

  const days = useMemo(() => heatmapDays(today), [today]);
  const perDay = useMemo(() => bucketByDay(activity.data ?? []), [activity.data]);
  const totals = useMemo(() => {
    let submissions = 0;
    let active = 0;
    for (const d of days) {
      const a = perDay.get(d);
      if (a) {
        submissions += a.submissions;
        active += 1;
      }
    }
    return { submissions, active };
  }, [days, perDay]);
  const solved = (breakdown.data?.by_difficulty ?? []).reduce(
    (acc, d) => ({ solved: acc.solved + d.solved, total: acc.total + d.total }),
    { solved: 0, total: 0 },
  );
  const r = reviews.data;
  const dueNow = r ? r.overdue + r.due_today : 0;

  return (
    <Page
      title={t("progress.title")}
      actions={streak.data !== undefined ? <StreakBadge count={streak.data} /> : undefined}
    >
      <div className="max-w-5xl space-y-6">
        <div className="grid grid-cols-1 gap-3 sm:grid-cols-3">
          <Stat
            testId="stat-solved"
            label={t("progress.stats.solved")}
            value={breakdown.data ? t("practice.patterns.solved", solved) : "–"}
          />
          <Stat testId="stat-active" label={t("progress.stats.activeDays")} value={activity.data ? totals.active : "–"} />
          <Stat
            testId="stat-submissions"
            label={t("progress.stats.submissions")}
            value={activity.data ? totals.submissions : "–"}
          />
        </div>

        <Section title={t("progress.heatmap.title")} testId="activity">
          {activity.isPending ? (
            <Skeleton className="h-28 w-full" />
          ) : activity.error ? (
            <ErrorNotice error={activity.error} onRetry={() => void activity.refetch()} />
          ) : (
            <Heatmap days={days} data={perDay} />
          )}
        </Section>

        <div className="grid gap-6 md:grid-cols-2">
          <Section title={t("progress.difficulty.title")} testId="by-difficulty">
            {breakdown.error ? (
              <ErrorNotice error={breakdown.error} onRetry={() => void breakdown.refetch()} />
            ) : (
              <div className="space-y-3">
                {(breakdown.data?.by_difficulty ?? []).map((d) => (
                  <div key={d.difficulty} data-testid={`difficulty-${d.difficulty}`}>
                    <CountBar
                      label={<DifficultyLabel difficulty={d.difficulty} />}
                      name={t(`difficulty.${d.difficulty}`)}
                      solved={d.solved}
                      total={d.total}
                    />
                  </div>
                ))}
              </div>
            )}
          </Section>

          <Section title={t("progress.reviews.title")} testId="review-summary">
            {reviews.error ? (
              <ErrorNotice error={reviews.error} onRetry={() => void reviews.refetch()} />
            ) : !r ? (
              <Skeleton className="h-24 w-full" />
            ) : r.overdue + r.due_today + r.next_7_days + r.later === 0 ? (
              <p className="text-sm text-muted-foreground">{t("progress.reviews.none")}</p>
            ) : (
              <div className="space-y-3">
                <dl className="grid grid-cols-2 gap-x-6 gap-y-1 text-sm">
                  {(
                    [
                      ["overdue", r.overdue],
                      ["today", r.due_today],
                      ["week", r.next_7_days],
                      ["later", r.later],
                    ] as const
                  ).map(([k, n]) => (
                    <div key={k} className="flex justify-between gap-2" data-testid={`reviews-${k}`}>
                      <dt className="text-muted-foreground">{t(`progress.reviews.${k}`)}</dt>
                      <dd className="tabular-nums font-medium">{n}</dd>
                    </div>
                  ))}
                </dl>
                <div className="flex flex-wrap items-center justify-between gap-2 text-sm">
                  {r.next_due && (
                    <span className="text-muted-foreground">
                      {t("progress.reviews.next", { date: formatDay(r.next_due, i18n.language) })}
                    </span>
                  )}
                  {dueNow > 0 && (
                    <Button asChild size="sm" variant="outline">
                      <Link to="/">
                        {t("progress.reviews.reviewNow", { count: dueNow })}
                        <ArrowRight aria-hidden />
                      </Link>
                    </Button>
                  )}
                </div>
              </div>
            )}
          </Section>
        </div>

        <Section title={t("practice.patterns.byLanguage")}>
          <ul className="grid grid-cols-2 gap-3 sm:grid-cols-4" data-testid="solved-by-language">
            {(byLanguage.data ?? []).map((c) => (
              <li key={c.language} className="rounded-md border p-3" data-testid={`lang-${c.language}`}>
                <span className="text-sm text-muted-foreground">{t(`languages.${c.language}`)}</span>
                <p className="text-xl font-semibold tabular-nums">{c.solved}</p>
              </li>
            ))}
          </ul>
        </Section>

        <Section title={t("progress.topics.title")} testId="by-topic">
          {(breakdown.data?.by_tag ?? []).length === 0 ? (
            <p className="text-sm text-muted-foreground">{t("progress.topics.empty")}</p>
          ) : (
            <ul className="grid gap-x-8 gap-y-3 sm:grid-cols-2">
              {(breakdown.data?.by_tag ?? []).map((c) => (
                <li key={c.tag} data-testid={`topic-${c.tag}`}>
                  <CountBar
                    label={
                      <Link to="/library" search={{ tag: c.tag }} className="hover:underline">
                        {tagLabel(c.tag)}
                      </Link>
                    }
                    name={tagLabel(c.tag)}
                    solved={c.solved}
                    total={c.total}
                  />
                </li>
              ))}
            </ul>
          )}
        </Section>

        <div>
          <h2 className="mb-3 text-sm font-semibold text-muted-foreground">{t("practice.patterns.title")}</h2>
          {patterns.isPending ? (
            <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
              {Array.from({ length: 6 }, (_, i) => (
                <Skeleton key={i} className="h-32" />
              ))}
            </div>
          ) : patterns.error ? (
            <ErrorNotice error={patterns.error} onRetry={() => void patterns.refetch()} />
          ) : (patterns.data ?? []).length === 0 ? (
            <p className="text-muted-foreground">{t("practice.patterns.empty")}</p>
          ) : (
            <ul className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3" data-testid="pattern-map">
              {(patterns.data ?? []).map((p) => {
                const done = p.solved === p.total;
                return (
                  <li
                    key={p.pattern}
                    className="flex flex-col gap-3 rounded-lg border bg-card p-4"
                    data-testid={`pattern-${p.pattern}`}
                  >
                    <div className="flex items-start justify-between gap-2">
                      <h3 className="font-semibold">{patternLabel(p.pattern)}</h3>
                      <span className="shrink-0 text-sm tabular-nums text-muted-foreground">
                        {t("practice.patterns.solved", { solved: p.solved, total: p.total })}
                      </span>
                    </div>
                    <Bar
                      value={p.total === 0 ? 0 : (100 * p.solved) / p.total}
                      aria-label={`${patternLabel(p.pattern)}: ${t("practice.patterns.solved", { solved: p.solved, total: p.total })}`}
                    />
                    <PatternLearnLinks pattern={p.pattern} className="text-xs" />
                    <div className="mt-auto flex items-center justify-between gap-2 text-sm">
                      {done || !p.next ? (
                        <span className="inline-flex items-center gap-1.5 text-verdict-accepted">
                          <CheckCircle2 aria-hidden className="size-4" />
                          {t("practice.patterns.done")}
                        </span>
                      ) : (
                        <Link
                          to="/problems/$problemId"
                          params={{ problemId: p.next.id }}
                          className="inline-flex min-w-0 items-center gap-1.5 hover:underline"
                        >
                          <span className="truncate">{t("practice.patterns.next", { title: p.next.title })}</span>
                          <DifficultyLabel difficulty={p.next.difficulty} className="shrink-0 text-xs" />
                        </Link>
                      )}
                      <Button asChild variant="ghost" size="sm" className="shrink-0">
                        <Link to="/library" search={{ pattern: p.pattern }}>
                          {t("practice.patterns.browse")}
                          <ArrowRight aria-hidden />
                        </Link>
                      </Button>
                    </div>
                  </li>
                );
              })}
            </ul>
          )}
        </div>

        <div>
          <h2 className="mb-3 text-sm font-semibold text-muted-foreground">{t("practice.interview.history")}</h2>
          {(interviews.data ?? []).length === 0 ? (
            <p className="text-sm text-muted-foreground">{t("practice.interview.none")}</p>
          ) : (
            <ul className="space-y-2" data-testid="interview-history">
              {(interviews.data ?? []).map((iv) => (
                <li key={iv.id} className="rounded-lg border bg-card">
                  <details>
                    <summary className="flex cursor-pointer flex-wrap items-center gap-x-4 gap-y-1 px-4 py-2 text-sm">
                      <span className="font-medium">
                        {formatDay(localToday(new Date(iv.started_at)), i18n.language)}
                      </span>
                      <span>{t("practice.interview.minutes", { count: iv.duration_s / 60 })}</span>
                      <span>{t(`practice.interview.mixes.${iv.mix}` as "practice.interview.title")}</span>
                      <span>
                        {t("practice.interview.solved", { solved: iv.solved, total: iv.problems.length })}
                      </span>
                      <span className="text-muted-foreground">
                        {t("practice.interview.firstTry", { count: iv.first_try })}
                      </span>
                    </summary>
                    <div className="border-t p-3">
                      <Scorecard result={iv} />
                    </div>
                  </details>
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>
    </Page>
  );
}
