// Follow-up variants (expansion M5, ADR 0035): harder siblings of a problem, shown as
// "Base · Follow-up 1 · Follow-up 2" once the base is solved, and a banner on a
// follow-up's own page that links back to its base.
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowUpRight, GitBranchPlus, Lock } from "lucide-react";
import type { ProblemView } from "@/bindings";
import { DifficultyLabel } from "@/components/Semantic";
import { ExerciseShell } from "@/components/exercise/ExerciseShell";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useProblem } from "@/lib/queries";
import { cn } from "@/lib/utils";

/** The strip of links between a base and its follow-ups (nothing when there are none). */
export function FollowupTabs({ problem }: { problem: ProblemView }) {
  const { t } = useTranslation();
  const baseId = problem.parent ?? problem.id;
  const base = useProblem(baseId, problem.parent !== null);
  const source = problem.parent ? base.data : problem;
  const followups = source?.followups ?? [];
  if (followups.length === 0) return null;
  const item = "inline-flex items-center gap-1.5 rounded-md px-2 py-1 text-xs";
  return (
    <nav aria-label={t("followup.nav")} className="flex flex-wrap items-center gap-1 border-b px-3 py-1.5" data-testid="followup-tabs">
      <Link
        to="/problems/$problemId"
        params={{ problemId: baseId }}
        aria-current={problem.id === baseId ? "page" : undefined}
        className={cn(item, problem.id === baseId ? "bg-accent font-medium" : "hover:bg-accent")}
      >
        {t("followup.base")}
      </Link>
      {followups.map((f, i) => {
        const label = t("followup.n", { n: i + 1 });
        if (!f.unlocked) {
          return (
            <Tooltip key={f.id}>
              <TooltipTrigger asChild>
                <span className={cn(item, "text-muted-foreground")} tabIndex={0} data-testid="followup-locked">
                  <Lock aria-hidden className="size-3" />
                  {label}
                  <span className="sr-only">{t("followup.locked")}</span>
                </span>
              </TooltipTrigger>
              <TooltipContent>{t("followup.locked")}</TooltipContent>
            </Tooltip>
          );
        }
        return (
          <Link
            key={f.id}
            to="/problems/$problemId"
            params={{ problemId: f.id }}
            aria-current={problem.id === f.id ? "page" : undefined}
            className={cn(item, problem.id === f.id ? "bg-accent font-medium" : "hover:bg-accent")}
            title={f.title}
          >
            {label}
            <DifficultyLabel difficulty={f.difficulty} className="text-[10px]" />
          </Link>
        );
      })}
    </nav>
  );
}

/** On a follow-up's page: what it adds, and the way back to the base statement. */
export function FollowupBanner({ problem }: { problem: ProblemView }) {
  const { t } = useTranslation();
  const base = useProblem(problem.parent ?? "", problem.parent !== null);
  if (!problem.parent) return null;
  const n = (base.data?.followups.findIndex((f) => f.id === problem.id) ?? -1) + 1;
  return (
    <div className="border-b px-3 py-2">
      <ExerciseShell
        kind="followup"
        icon={<GitBranchPlus />}
        title={problem.title}
        position={n > 0 && base.data ? { index: n, total: base.data.followups.length } : undefined}
        testId="followup-banner"
      >
        <p className="text-sm text-muted-foreground">
          {t("followup.of")}{" "}
          <Link to="/problems/$problemId" params={{ problemId: problem.parent }} className="font-medium text-foreground hover:underline">
            {base.data?.title ?? problem.parent}
            <ArrowUpRight aria-hidden className="ml-0.5 inline size-3" />
          </Link>
          . {t("followup.intro")}
        </p>
      </ExerciseShell>
    </div>
  );
}
