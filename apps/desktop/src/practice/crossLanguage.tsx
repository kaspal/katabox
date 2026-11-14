// Cross-language practice (ADR 0020): "Solve in…" and the read-only accepted solution
// shown beside the editor. The editor itself keeps its usual rule: the target language
// opens its existing draft, otherwise its stub, and nothing is saved until you type.
import { useTranslation } from "react-i18next";
import { Languages, X } from "lucide-react";
import type { Language } from "@/bindings";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Skeleton } from "@/components/ui/skeleton";
import { ErrorNotice } from "@/components/ErrorNotice";
import { useAcceptedSolution } from "@/lib/modes";
import { JUDGE_LANGUAGES, useEditorPrefs } from "@/lib/settings";
import { CodeEditor } from "@/workspace/editor/CodeEditor";

/** The source language for solving in `to`: the current one if it is solved, else any. */
export function sourceFor(solved: Language[], current: Language | null, to: Language): Language | null {
  if (current && current !== to && solved.includes(current)) return current;
  return solved.find((l) => l !== to) ?? null;
}

export function SolveInMenu({
  solved,
  current,
  onPick,
}: {
  solved: Language[];
  current: Language | null;
  onPick: (from: Language, to: Language) => void;
}) {
  const { t } = useTranslation();
  const targets = JUDGE_LANGUAGES.filter((l) => sourceFor(solved, current, l) !== null);
  if (targets.length === 0) return null;
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" size="sm" className="gap-1.5" aria-label={t("practice.crossLang.solveIn")}>
          <Languages aria-hidden />
          <span className="hidden xl:inline">{t("practice.crossLang.solveIn")}</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuLabel>{t("practice.crossLang.solveIn")}</DropdownMenuLabel>
        {targets.map((to) => (
          <DropdownMenuItem key={to} onSelect={() => onPick(sourceFor(solved, current, to)!, to)}>
            {t(`languages.${to}`)}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

/** The accepted solution in `language`, read-only, on the left of the editor. */
export function CompareView({
  problem,
  language,
  onClose,
}: {
  problem: string;
  language: Language;
  onClose: () => void;
}) {
  const { t } = useTranslation();
  const { fontSize, ligatures, tabWidth, theme: editorTheme } = useEditorPrefs();
  const solution = useAcceptedSolution(problem, language);
  const name = t(`languages.${language}`);
  return (
    <section
      className="flex min-h-0 w-1/2 min-w-0 flex-col border-r"
      aria-label={t("practice.crossLang.accepted", { language: name })}
      data-testid="compare-view"
    >
      <div className="flex items-center gap-2 border-b bg-muted/40 px-3 py-1.5 text-xs">
        <span className="min-w-0 flex-1 truncate font-medium">
          {t("practice.crossLang.accepted", { language: name })}
        </span>
        <Button variant="ghost" size="icon" className="size-6" aria-label={t("practice.crossLang.close")} onClick={onClose}>
          <X aria-hidden />
        </Button>
      </div>
      <div className="min-h-0 flex-1">
        {solution.isPending ? (
          <Skeleton className="m-3 h-32" />
        ) : solution.error ? (
          <div className="p-3">
            <ErrorNotice error={solution.error} />
          </div>
        ) : solution.data ? (
          <CodeEditor
            docKey={`accepted:${problem}:${language}:${solution.data.submitted_at}`}
            value={solution.data.code}
            lang={language}
            readOnly
            fontSize={fontSize}
            tabWidth={tabWidth}
            colorTheme={editorTheme}
            ligatures={ligatures}
            ariaLabel={t("practice.crossLang.accepted", { language: name })}
          />
        ) : (
          <p className="p-3 text-sm text-muted-foreground">{t("practice.crossLang.none", { language: name })}</p>
        )}
      </div>
    </section>
  );
}
