import { useTranslation } from "react-i18next";
import { Link } from "@tanstack/react-router";
import type { ProblemView } from "@/bindings";
import { BookOpen, FileText, History, Lightbulb, ListChecks, Lock, NotebookPen, Table2 } from "lucide-react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { usePatternLabel, useTagLabel } from "@/components/Semantic";
import { Markdown } from "@/workspace/problem/Markdown";
import { HintsTab } from "@/workspace/problem/HintsTab";
import { EditorialTab } from "@/workspace/problem/EditorialTab";
import { SubmissionsTab } from "@/workspace/problem/SubmissionsTab";
import { NotesTab } from "@/workspace/problem/NotesTab";
import { HistoryTab } from "@/workspace/problem/HistoryTab";
import { SchemaPanel } from "@/workspace/sql/SchemaPanel";
import type { Language } from "@/bindings";

export const PROBLEM_TABS = ["description", "hints", "editorial", "submissions", "notes", "history"] as const;
/** `schema`: the Tables tab of an SQL problem (ADR 0031), right after the description. */
export type ProblemTab = (typeof PROBLEM_TABS)[number] | "schema";

const TAB_ICON: Record<ProblemTab, typeof FileText> = {
  description: FileText,
  schema: Table2,
  hints: Lightbulb,
  editorial: BookOpen,
  submissions: ListChecks,
  notes: NotebookPen,
  history: History,
};

/** The time limit for the editor's language (ADR 0017), adjusted to this machine. */
function TimeLimit({ problem, language }: { problem: ProblemView; language: Language | null }) {
  const { t } = useTranslation();
  const limit = problem.time_limits.find((l) => l.language === language);
  const scaled = Math.abs(problem.time_scale - 1) >= 0.005;
  return (
    <>
      <dt>
        {limit && language
          ? t("workspace.limits.timeFor", { language: t(`languages.${language}`) })
          : t("workspace.limits.time")}
      </dt>
      <dd className="tabular-nums" data-testid="time-limit">
        {limit ? limit.time_limit_ms : problem.time_limit_ms} ms
        {scaled && (
          <Link to="/settings" search={{ section: "languages" }} className="ml-2 underline-offset-2 hover:underline">
            {t("workspace.limits.scaled", { factor: problem.time_scale.toFixed(2) })}
          </Link>
        )}
      </dd>
    </>
  );
}

/** Tabs hidden in review mode (ADR 0019) or during a mock interview (ADR 0020). */
export type PaneLocks = { tabs: ProblemTab[]; reason: "review" | "interview" | "kata" };

function LockedNote({ reason }: { reason: PaneLocks["reason"] }) {
  const { t } = useTranslation();
  return (
    <p
      className="flex items-center gap-2 text-sm text-muted-foreground"
      data-testid={`${reason}-locked`}
    >
      <Lock aria-hidden className="size-4" />
      {reason === "review"
        ? t("practice.review.locked")
        : reason === "kata"
          ? t("kata.locked")
          : t("practice.interview.locked")}
    </p>
  );
}

export function ProblemPane({
  problem,
  currentCode,
  currentLanguage,
  onRestored,
  locks = null,
}: {
  problem: ProblemView;
  currentCode: string | null;
  currentLanguage: Language | null;
  onRestored: (language: Language) => void;
  /** Tabs that show a "locked" note instead of their content. */
  locks?: PaneLocks | null;
}) {
  const { t } = useTranslation();
  const locked = (tab: ProblemTab) => locks?.tabs.includes(tab) === true;
  const note = locks ? <LockedNote reason={locks.reason} /> : null;
  const patternLabel = usePatternLabel();
  const tagLabel = useTagLabel();
  const tabs: readonly ProblemTab[] =
    problem.mode === "sql" ? ["description", "schema", ...PROBLEM_TABS.slice(1)] : PROBLEM_TABS;
  return (
    <Tabs defaultValue="description" className="@container flex h-full min-h-0 flex-col gap-0">
      {/* Narrow panes show icons only (with a tooltip and an accessible name); never overflow. */}
      <TabsList
        aria-label={t("workspace.tabs.label")}
        className="mx-2 mt-2 h-8 w-auto max-w-full justify-start"
        data-testid="problem-tabs"
      >
        {tabs.map((tab) => {
          const Icon = TAB_ICON[tab];
          const label = t(`workspace.tabs.${tab}`);
          return (
            <Tooltip key={tab}>
              <TooltipTrigger asChild>
                <TabsTrigger
                  value={tab}
                  aria-label={label}
                  className="group min-w-0 gap-1 px-2 text-xs"
                  data-tour={tab === "hints" ? "hints" : undefined}
                >
                  <Icon aria-hidden className="size-3.5 shrink-0" />
                  {/* Narrow panes show icons only, except the active tab's name (aria-selected: the tooltip owns data-state). */}
                  <span
                    className="hidden truncate group-aria-selected:inline @[34rem]:inline"
                    data-tab-label
                  >
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
          <Markdown text={problem.statement} trust={problem.pack ? "pack" : "app"} />
          <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 border-t pt-3 text-xs text-muted-foreground">
            <TimeLimit problem={problem} language={currentLanguage} />
            <dt>{t("workspace.limits.memory")}</dt>
            <dd className="tabular-nums">{problem.memory_mb} MB</dd>
            <dt>{t("pattern.label")}</dt>
            <dd>{patternLabel(problem.pattern)}</dd>
            <dt>{t("library.columns.tags")}</dt>
            <dd>{problem.tags.map(tagLabel).join(", ")}</dd>
          </dl>
        </TabsContent>
        {problem.mode === "sql" && (
          <TabsContent value="schema">
            <SchemaPanel problem={problem} />
          </TabsContent>
        )}
        <TabsContent value="hints">
          {locked("hints") ? note : <HintsTab problem={problem} />}
        </TabsContent>
        <TabsContent value="editorial">
          {locked("editorial") ? note : <EditorialTab problemId={problem.id} pack={problem.pack !== null} />}
        </TabsContent>
        <TabsContent value="submissions">
          {locked("submissions") ? note : <SubmissionsTab problemId={problem.id} />}
        </TabsContent>
        <TabsContent value="notes" className="h-full">
          {locked("notes") ? note : <NotesTab problemId={problem.id} />}
        </TabsContent>
        <TabsContent value="history">
          {locked("history") ? (
            note
          ) : (
          <HistoryTab
            problemId={problem.id}
            currentCode={currentCode}
            currentLanguage={currentLanguage}
            onRestored={onRestored}
          />
          )}
        </TabsContent>
      </div>
    </Tabs>
  );
}
