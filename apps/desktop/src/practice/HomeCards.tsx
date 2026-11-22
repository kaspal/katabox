// Home's practice cards (ADR 0019, 0041): the card frames and today's reviews.
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Clock, MoreHorizontal, RotateCcw, Trash2 } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { ErrorNotice } from "@/components/ErrorNotice";
import { useReviewActions, useReviewsDue } from "@/lib/practice";

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

/** The Home's "Keep it fresh" row (ADR 0041): an icon, a title, one line, one action. */
export function CompactCard({
  title,
  icon,
  line,
  action,
  testId,
  busy,
  error,
}: {
  title: string;
  icon: React.ReactNode;
  line: React.ReactNode;
  action?: React.ReactNode;
  testId?: string;
  busy?: boolean;
  error?: unknown;
}) {
  return (
    <section
      className="flex items-center gap-3 rounded-2xl border bg-card px-4 py-3.5 text-card-foreground"
      data-testid={testId}
      data-compact
    >
      <span className="flex-none text-muted-foreground">{icon}</span>
      <div className="min-w-0 flex-1">
        <h2 className="text-sm font-semibold">{title}</h2>
        {busy ? (
          <Skeleton className="mt-1 h-4 w-32" />
        ) : error ? (
          <ErrorNotice error={error} />
        ) : (
          <p className="truncate text-[13px] text-muted-foreground">{line}</p>
        )}
      </div>
      {!busy && !error && action}
    </section>
  );
}

export function ReviewCard({ compact = false }: { compact?: boolean }) {
  const { t } = useTranslation();
  const reviews = useReviewsDue();
  const actions = useReviewActions();
  if (compact) {
    const due = reviews.data ?? [];
    return (
      <CompactCard
        title={t("practice.review.title")}
        icon={<RotateCcw aria-hidden className="size-5" />}
        testId="review-card"
        busy={reviews.isPending}
        error={reviews.error}
        line={due.length === 0 ? t("home.fresh.reviewNone") : t("home.fresh.reviewDue", { count: due.length })}
        action={
          due.length > 0 && (
            <div className="flex flex-none items-center gap-1">
              <Button asChild size="sm" variant="outline">
                <Link to="/problems/$problemId" params={{ problemId: due[0].problem.id }} search={{ review: true }}>
                  {t("home.fresh.reviewOpen")}
                </Link>
              </Button>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" size="icon" className="size-8" aria-label={t("home.fresh.reviewMore")}>
                    <MoreHorizontal aria-hidden />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  {due.map(({ problem }) => (
                    <DropdownMenuGroup key={problem.id}>
                      <DropdownMenuLabel className="text-xs">{problem.title}</DropdownMenuLabel>
                      <DropdownMenuItem onSelect={() => actions.snooze.mutate(problem.id)}>
                        <Clock aria-hidden />
                        {`${t("practice.review.snooze")}: ${problem.title}`}
                      </DropdownMenuItem>
                      <DropdownMenuItem onSelect={() => actions.remove.mutate(problem.id)}>
                        <Trash2 aria-hidden />
                        {`${t("practice.review.remove")}: ${problem.title}`}
                      </DropdownMenuItem>
                    </DropdownMenuGroup>
                  ))}
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          )
        }
      />
    );
  }
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
