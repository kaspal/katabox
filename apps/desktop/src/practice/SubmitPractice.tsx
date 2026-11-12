// What a Submit did for practice (ADR 0019), shown under the result: the review outcome,
// the first review scheduled, the daily problem and the "Next" suggestion.
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowRight, CalendarCheck, CheckCircle2, RotateCcw, Sparkles } from "lucide-react";
import type { PracticeOutcome } from "@/bindings";
import { Button } from "@/components/ui/button";
import { DifficultyLabel, usePatternLabel } from "@/components/Semantic";
import { StreakBadge, formatDay } from "@/practice/shared";

export function SubmitPractice({ practice, accepted }: { practice: PracticeOutcome; accepted: boolean }) {
  const { t, i18n } = useTranslation();
  const patternLabel = usePatternLabel();
  const review = practice.review;
  return (
    <div className="mt-3 space-y-3" data-testid="submit-practice">
      {review && (
        <p
          role="status"
          className="flex items-center gap-2 rounded-md border bg-muted/40 px-3 py-2 text-sm"
          data-testid="review-outcome"
        >
          <RotateCcw aria-hidden className="size-4 shrink-0" />
          {review.outcome === "scheduled"
            ? t("practice.review.passed", { date: formatDay(review.due_on, i18n.language) })
            : review.outcome === "graduated"
              ? t("practice.review.graduated")
              : t("practice.review.lapsedBanner")}
        </p>
      )}
      {practice.first_solve && !review && (
        <p className="flex items-center gap-2 text-sm text-muted-foreground" data-testid="first-review">
          <CalendarCheck aria-hidden className="size-4 shrink-0" />
          {t("practice.review.scheduled", {
            date: formatDay(threeDaysFromToday(), i18n.language),
          })}
        </p>
      )}
      {practice.daily_solved && (
        <p className="flex items-center gap-3 text-sm" data-testid="daily-solved">
          <span className="inline-flex items-center gap-1.5 font-medium text-verdict-accepted">
            <CheckCircle2 aria-hidden className="size-4" />
            {t("practice.daily.solved")}
          </span>
          <StreakBadge count={practice.streak} />
        </p>
      )}
      {accepted && (
        <section className="flex items-center gap-3 rounded-lg border bg-card p-3" data-testid="next-card">
          <Sparkles aria-hidden className="size-4 shrink-0 text-muted-foreground" />
          {practice.next ? (
            <>
              <div className="min-w-0 flex-1">
                <p className="text-xs font-semibold text-muted-foreground">{t("practice.next.title")}</p>
                <p className="truncate text-sm font-medium">{practice.next.title}</p>
                <p className="flex items-center gap-2 text-xs text-muted-foreground">
                  <DifficultyLabel difficulty={practice.next.difficulty} />
                  <span>·</span>
                  <span className="truncate">{patternLabel(practice.next.pattern)}</span>
                </p>
              </div>
              <Button asChild size="sm">
                <Link to="/problems/$problemId" params={{ problemId: practice.next.id }}>
                  {t("practice.next.open")}
                  <ArrowRight aria-hidden />
                </Link>
              </Button>
            </>
          ) : (
            <p className="text-sm">{t("practice.next.allSolved")}</p>
          )}
        </section>
      )}
    </div>
  );
}

function threeDaysFromToday(): string {
  const d = new Date();
  d.setDate(d.getDate() + 3);
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}
