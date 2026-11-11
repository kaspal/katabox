import { useTranslation } from "react-i18next";
import type { ProblemView } from "@/bindings";
import { BookOpen, FileText, History, Lightbulb, ListChecks, NotebookPen } from "lucide-react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { usePatternLabel } from "@/components/Semantic";
import { Markdown } from "@/workspace/problem/Markdown";
import { HintsTab } from "@/workspace/problem/HintsTab";
import { EditorialTab } from "@/workspace/problem/EditorialTab";
import { SubmissionsTab } from "@/workspace/problem/SubmissionsTab";
import { NotesTab } from "@/workspace/problem/NotesTab";
import { HistoryTab } from "@/workspace/problem/HistoryTab";
import type { Language } from "@/bindings";

export const PROBLEM_TABS = ["description", "hints", "editorial", "submissions", "notes", "history"] as const;
export type ProblemTab = (typeof PROBLEM_TABS)[number];

const TAB_ICON: Record<ProblemTab, typeof FileText> = {
  description: FileText,
  hints: Lightbulb,
  editorial: BookOpen,
  submissions: ListChecks,
  notes: NotebookPen,
  history: History,
};

export function ProblemPane({
  problem,
  currentCode,
  currentLanguage,
  onRestored,
}: {
  problem: ProblemView;
  currentCode: string | null;
  currentLanguage: Language | null;
  onRestored: (language: Language) => void;
}) {
  const { t } = useTranslation();
  const patternLabel = usePatternLabel();
  return (
    <Tabs defaultValue="description" className="@container flex h-full min-h-0 flex-col gap-0">
      {/* Narrow panes show icons only (with a tooltip and an accessible name); never overflow. */}
      <TabsList className="mx-2 mt-2 h-8 w-auto max-w-full justify-start" data-testid="problem-tabs">
        {PROBLEM_TABS.map((tab) => {
          const Icon = TAB_ICON[tab];
          const label = t(`workspace.tabs.${tab}`);
          return (
            <Tooltip key={tab}>
              <TooltipTrigger asChild>
                <TabsTrigger value={tab} aria-label={label} className="min-w-0 gap-1 px-2 text-xs">
                  <Icon aria-hidden className="size-3.5 shrink-0" />
                  <span className="hidden truncate @[34rem]:inline" data-tab-label>
                    {label}
                  </span>
                </TabsTrigger>
              </TooltipTrigger>
              <TooltipContent className="@[34rem]:hidden">{label}</TooltipContent>
            </Tooltip>
          );
        })}
      </TabsList>
      <div className="min-h-0 flex-1 overflow-auto p-4">
        <TabsContent value="description" className="space-y-4">
          <Markdown text={problem.statement} />
          <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 border-t pt-3 text-xs text-muted-foreground">
            <dt>{t("workspace.limits.time")}</dt>
            <dd className="tabular-nums">{problem.time_limit_ms} ms</dd>
            <dt>{t("workspace.limits.memory")}</dt>
            <dd className="tabular-nums">{problem.memory_mb} MB</dd>
            <dt>{t("pattern.label")}</dt>
            <dd>{patternLabel(problem.pattern)}</dd>
            <dt>{t("library.columns.tags")}</dt>
            <dd className="font-mono">{problem.tags.join(", ")}</dd>
          </dl>
        </TabsContent>
        <TabsContent value="hints">
          <HintsTab problem={problem} />
        </TabsContent>
        <TabsContent value="editorial">
          <EditorialTab problemId={problem.id} />
        </TabsContent>
        <TabsContent value="submissions">
          <SubmissionsTab problemId={problem.id} />
        </TabsContent>
        <TabsContent value="notes" className="h-full">
          <NotesTab problemId={problem.id} />
        </TabsContent>
        <TabsContent value="history">
          <HistoryTab
            problemId={problem.id}
            currentCode={currentCode}
            currentLanguage={currentLanguage}
            onRestored={onRestored}
          />
        </TabsContent>
      </div>
    </Tabs>
  );
}
