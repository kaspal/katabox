// Home's practice cards (ADR 0019): the daily problem with the streak, and today's reviews.
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowRight, CalendarDays, CheckCircle2, Clock, RotateCcw, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel, usePatternLabel } from "@/components/Semantic";
import { useDailyProblem, useReviewActions, useReviewsDue } from "@/lib/practice";
import { StreakBadge } from "@/practice/shared";

export function HomeCard({
  title,
  icon,
  children,
  testId,
}: {
  title: string;
  icon: React.ReactNode;
  children: React.ReactNode;
  testId?: string;
}) {
  return (
    <section className="flex flex-col gap-3 rounded-lg border bg-card p-5 text-card-foreground" data-testid={testId}>
      <h2 className="flex items-center gap-2 text-sm font-semibold text-muted-foreground">
        {icon}
        {title}
      </h2>
      {children}
    </section>
  );
}

export function DailyCard() {
  const { t } = useTranslation();
  const patternLabel = usePatternLabel();
  const daily = useDailyProblem();
  const d = daily.data;
  return (
    <HomeCard title={t("practice.daily.title")} icon={<CalendarDays aria-hidden className="size-4" />} testId="daily-card">
      {daily.isPending ? (
        <Skeleton className="h-12 w-full" />
      ) : daily.error ? (
        <ErrorNotice error={daily.error} onRetry={() => void daily.refetch()} />
      ) : !d ? (
        <p className="text-sm text-muted-foreground">{t("practice.daily.none")}</p>
      ) : (
        <>
          <div className="flex items-center justify-between gap-3">
            <div className="min-w-0">
              <p className="truncate text-base font-medium" data-testid="daily-title">
                {d.problem.title}
              </p>
              <p className="flex items-center gap-2 text-sm text-muted-foreground">
                <DifficultyLabel difficulty={d.problem.difficulty} className="text-xs" />
                <span>·</span>
                <span className="truncate">{patternLabel(d.problem.pattern)}</span>
              </p>
            </div>
            {d.solved ? (
              <span className="inline-flex items-center gap-1.5 text-sm font-medium text-verdict-accepted">
                <CheckCircle2 aria-hidden className="size-4" />
                {t("practice.daily.solved")}
              </span>
            ) : (
              <Button asChild>
                <Link to="/problems/$problemId" params={{ problemId: d.problem.id }}>
                  {t("practice.daily.open")}
                  <ArrowRight aria-hidden />
                </Link>
              </Button>
            )}
          </div>
          <div className="flex items-center justify-between gap-3 border-t pt-3">
            <StreakBadge count={d.streak} />
            <span className="text-xs text-muted-foreground">{t("practice.daily.hint")}</span>
          </div>
        </>
      )}
    </HomeCard>
  );
}

export function ReviewCard() {
  const { t } = useTranslation();
  const reviews = useReviewsDue();
  const actions = useReviewActions();
  return (
    <HomeCard title={t("practice.review.title")} icon={<RotateCcw aria-hidden className="size-4" />} testId="review-card">
      {reviews.isPending ? (
        <Skeleton className="h-12 w-full" />
      ) : reviews.error ? (
        <ErrorNotice error={reviews.error} onRetry={() => void reviews.refetch()} />
      ) : (reviews.data ?? []).length === 0 ? (
        <p className="text-sm text-muted-foreground">{t("practice.review.empty")}</p>
      ) : (
        <ul className="divide-y">
          {(reviews.data ?? []).map(({ problem, review }) => (
            <li key={problem.id} className="flex items-center gap-2 py-2" data-testid={`review-${problem.id}`}>
              <div className="min-w-0 flex-1">
                <p className="truncate text-sm font-medium">{problem.title}</p>
                <p className="text-xs text-muted-foreground">
                  {review.lapsed ? t("practice.review.lapsed") : t("practice.review.interval", { days: review.interval_days })}
                </p>
              </div>
              <Button asChild size="sm">
                <Link to="/problems/$problemId" params={{ problemId: problem.id }} search={{ review: true }}>
                  {t("practice.review.start")}
                </Link>
              </Button>
              <Button
                variant="ghost"
                size="icon"
                className="size-8"
                aria-label={`${t("practice.review.snooze")}: ${problem.title}`}
                title={t("practice.review.snooze")}
                onClick={() => actions.snooze.mutate(problem.id)}
              >
                <Clock aria-hidden />
              </Button>
              <Button
                variant="ghost"
                size="icon"
                className="size-8"
                aria-label={`${t("practice.review.remove")}: ${problem.title}`}
                title={t("practice.review.remove")}
                onClick={() => actions.remove.mutate(problem.id)}
              >
                <Trash2 aria-hidden />
              </Button>
            </li>
          ))}
        </ul>
      )}
    </HomeCard>
  );
}
