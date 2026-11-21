import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowRight, CheckCircle2, PlayCircle } from "lucide-react";
import type { Difficulty } from "@/bindings";
import { Page } from "@/components/Page";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel } from "@/components/Semantic";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { usePracticePrefs } from "@/lib/practice";
import { useContinue, useProblems } from "@/lib/queries";
import { DailyCard, HomeCard as Card, ReviewCard, TopicReviewCard } from "@/practice/HomeCards";
import { RustyCard } from "@/practice/kata";
import { relativeTime } from "@/lib/time";

export { relativeTime } from "@/lib/time";

export function Home() {
  const { t, i18n } = useTranslation();
  const cont = useContinue();
  const problems = useProblems();
  const practice = usePracticePrefs();

  const counts = (["easy", "medium", "hard"] as Difficulty[]).map((d) => {
    const all = (problems.data ?? []).filter((p) => p.difficulty === d);
    return { d, total: all.length, solved: all.filter((p) => p.status === "solved").length };
  });

  return (
    <Page title={t("home.title")}>
      <div className="grid max-w-4xl gap-4 md:grid-cols-2">
        <Card title={t("home.continue")} icon={<PlayCircle aria-hidden className="size-4" />}>
          {cont.isPending ? (
            <Skeleton className="h-12 w-full" />
          ) : cont.error ? (
            <ErrorNotice error={cont.error} onRetry={() => void cont.refetch()} />
          ) : cont.data ? (
            <div className="flex items-center justify-between gap-3">
              <div className="min-w-0">
                <p className="truncate text-base font-medium">{cont.data.title}</p>
                <p className="text-sm text-muted-foreground">
                  {t("home.continueEdited", {
                    language: t(`languages.${cont.data.language}`),
                    when: relativeTime(cont.data.updated_at, i18n.language),
                  })}
                </p>
              </div>
              <Button asChild>
                <Link to="/problems/$problemId" params={{ problemId: cont.data.problem_id }}>
                  {t("home.continueOpen")}
                  <ArrowRight aria-hidden />
                </Link>
              </Button>
            </div>
          ) : (
            <div className="flex items-center justify-between gap-3">
              <p className="text-sm text-muted-foreground">{t("home.noDraft")}</p>
              <Button asChild variant="outline">
                <Link to="/library">{t("home.browse")}</Link>
              </Button>
            </div>
          )}
        </Card>

        <Card title={t("home.solved")} icon={<CheckCircle2 aria-hidden className="size-4" />}>
          {problems.isPending ? (
            <Skeleton className="h-12 w-full" />
          ) : problems.error ? (
            <ErrorNotice error={problems.error} />
          ) : (
            <ul className="grid grid-cols-3 gap-3">
              {counts.map(({ d, solved, total }) => (
                <li key={d} className="rounded-md border p-3">
                  <DifficultyLabel difficulty={d} className="text-xs" />
                  <p className="mt-1 text-lg font-semibold tabular-nums" data-testid={`solved-${d}`}>
                    {t("home.solvedOf", { solved, total })}
                  </p>
                </li>
              ))}
            </ul>
          )}
        </Card>

        {practice.daily && <DailyCard />}
        {practice.review && <ReviewCard />}
        {practice.learnReview && <TopicReviewCard />}
        {practice.rusty && <RustyCard />}
      </div>
    </Page>
  );
}
