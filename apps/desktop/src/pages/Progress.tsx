// Progress (ADR 0019): the pattern map — every pattern with solved/total and its next
// problem — and the daily streak.
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowRight, CheckCircle2 } from "lucide-react";
import { Page } from "@/components/Page";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel, usePatternLabel } from "@/components/Semantic";
import { Button } from "@/components/ui/button";
import { Progress as Bar } from "@/components/ui/progress";
import { Skeleton } from "@/components/ui/skeleton";
import { usePatterns, useStreak } from "@/lib/practice";
import { useInterviews, useSolvedByLanguage } from "@/lib/modes";
import { StreakBadge, formatDay } from "@/practice/shared";
import { Scorecard } from "@/practice/interview";
import { localToday } from "@/lib/today";

export function Progress() {
  const { t } = useTranslation();
  const patternLabel = usePatternLabel();
  const patterns = usePatterns();
  const streak = useStreak();
  const byLanguage = useSolvedByLanguage();
  const interviews = useInterviews();
  const { i18n } = useTranslation();

  return (
    <Page
      title={t("progress.title")}
      actions={streak.data !== undefined ? <StreakBadge count={streak.data} /> : undefined}
    >
      <h2 className="mb-3 text-sm font-semibold text-muted-foreground">{t("practice.patterns.byLanguage")}</h2>
      <ul className="mb-6 grid max-w-5xl grid-cols-2 gap-3 sm:grid-cols-4" data-testid="solved-by-language">
        {(byLanguage.data ?? []).map((c) => (
          <li key={c.language} className="rounded-lg border bg-card p-3" data-testid={`lang-${c.language}`}>
            <span className="text-sm text-muted-foreground">{t(`languages.${c.language}`)}</span>
            <p className="text-xl font-semibold tabular-nums">{c.solved}</p>
          </li>
        ))}
      </ul>
      <h2 className="mb-3 text-sm font-semibold text-muted-foreground">{t("practice.patterns.title")}</h2>
      {patterns.isPending ? (
        <div className="grid max-w-5xl gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {Array.from({ length: 6 }, (_, i) => (
            <Skeleton key={i} className="h-32" />
          ))}
        </div>
      ) : patterns.error ? (
        <ErrorNotice error={patterns.error} onRetry={() => void patterns.refetch()} />
      ) : (patterns.data ?? []).length === 0 ? (
        <p className="text-muted-foreground">{t("practice.patterns.empty")}</p>
      ) : (
        <ul className="grid max-w-5xl gap-3 sm:grid-cols-2 lg:grid-cols-3" data-testid="pattern-map">
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

      <h2 className="mt-8 mb-3 text-sm font-semibold text-muted-foreground">{t("practice.interview.history")}</h2>
      {(interviews.data ?? []).length === 0 ? (
        <p className="text-sm text-muted-foreground">{t("practice.interview.none")}</p>
      ) : (
        <ul className="max-w-5xl space-y-2" data-testid="interview-history">
          {(interviews.data ?? []).map((r) => (
            <li key={r.id} className="rounded-lg border bg-card">
              <details>
                <summary className="flex cursor-pointer flex-wrap items-center gap-x-4 gap-y-1 px-4 py-2 text-sm">
                  <span className="font-medium">
                    {formatDay(localToday(new Date(r.started_at)), i18n.language)}
                  </span>
                  <span>{t("practice.interview.minutes", { count: r.duration_s / 60 })}</span>
                  <span>{t(`practice.interview.mixes.${r.mix}` as "practice.interview.title")}</span>
                  <span>{t("practice.interview.solved", { solved: r.solved, total: r.problems.length })}</span>
                  <span className="text-muted-foreground">{t("practice.interview.firstTry", { count: r.first_try })}</span>
                </summary>
                <div className="border-t p-3">
                  <Scorecard result={r} />
                </div>
              </details>
            </li>
          ))}
        </ul>
      )}
    </Page>
  );
}
