import { Link, getRouteApi } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowLeft, Construction } from "lucide-react";
import { Page } from "@/components/Page";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel, usePatternLabel } from "@/components/Semantic";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { useProblem } from "@/lib/queries";

const route = getRouteApi("/problems/$problemId");

/** Placeholder until part 2 (editor, results, hints, history). */
export function Workspace() {
  const { t } = useTranslation();
  const { problemId } = route.useParams();
  const problem = useProblem(problemId);
  const patternLabel = usePatternLabel();

  return (
    <Page
      title={problem.data?.title ?? t("workspace.title")}
      actions={
        <Button asChild variant="ghost" size="sm">
          <Link to="/library">
            <ArrowLeft aria-hidden />
            {t("workspace.back")}
          </Link>
        </Button>
      }
    >
      {problem.isPending ? (
        <Skeleton className="h-24 w-full max-w-3xl" />
      ) : problem.error ? (
        <ErrorNotice error={problem.error} onRetry={() => void problem.refetch()} />
      ) : (
        <div className="max-w-3xl space-y-4">
          <div className="flex flex-wrap items-center gap-3 text-sm">
            <DifficultyLabel difficulty={problem.data.difficulty} />
            <span className="text-muted-foreground">{patternLabel(problem.data.pattern)}</span>
            <span className="font-mono text-xs text-muted-foreground">
              {problem.data.tags.join(", ")}
            </span>
          </div>
          <div className="flex items-start gap-3 rounded-lg border border-dashed bg-card p-5">
            <Construction aria-hidden className="mt-0.5 size-5 text-muted-foreground" />
            <div className="space-y-2">
              <p>{t("workspace.comingSoon")}</p>
              {import.meta.env.DEV && (
                <Link to="/dev/smoke" className="text-sm text-primary underline-offset-4 hover:underline">
                  {t("workspace.smokeLink")}
                </Link>
              )}
            </div>
          </div>
        </div>
      )}
    </Page>
  );
}
