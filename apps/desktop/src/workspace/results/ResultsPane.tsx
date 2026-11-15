import { useEffect, useMemo, useRef, type ReactNode } from "react";
import { DebugThisCase } from "@/workspace/debug/context";
import type { PracticeOutcome } from "@/bindings";
import { SubmitPractice } from "@/practice/SubmitPractice";
import { useTranslation } from "react-i18next";
import type { TFunction } from "i18next";
import { Loader2, Plus, Trash2 } from "lucide-react";
import type { JudgeEvent, ProblemView } from "@/bindings";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { ErrorNotice } from "@/components/ErrorNotice";
import { isAppError } from "@/lib/errors";
import { cn } from "@/lib/utils";
import type { Phase } from "@/workspace/hooks";
import { ResultView } from "@/workspace/results/ResultView";
import {
  elementType,
  validateCustom,
  valuesFrom,
  type CustomTestDraft,
  type FieldError,
  type Signature,
} from "@/workspace/results/validate";
import type { Report } from "@/bindings";
import { RowsTable } from "@/workspace/sql/RowsTable";
import { NamedInputTables } from "@/workspace/sql/SqlViews";
import { asRows, parseSqlTables } from "@/workspace/sql/tables";

export type ResultsTab = "testcases" | "result" | "debug";

function usePhaseText() {
  const { t } = useTranslation();
  return (phase: Phase): string => {
    if (phase.kind === "starting") return t("results.phase.starting");
    if (phase.kind !== "event") return "";
    const e: JudgeEvent = phase.event;
    switch (e.event) {
      case "compiling":
        return t("results.phase.compiling");
      case "running":
        return t("results.phase.running", { count: e.total });
      case "checked":
        return t("results.phase.checked", { done: e.done, total: e.total });
      case "finished":
        return "";
    }
  };
}

export function ResultsPane({
  problem,
  signature,
  tab,
  onTab,
  customs,
  onCustoms,
  busy,
  phase,
  report,
  error,
  celebrate,
  validatorNeedsPython = false,
  onInstallPython,
  practice = null,
  debugPanel = null,
  complexity = null,
}: {
  problem: ProblemView;
  signature: Signature | null;
  tab: ResultsTab;
  onTab: (tab: ResultsTab) => void;
  customs: CustomTestDraft[];
  onCustoms: (next: CustomTestDraft[]) => void;
  busy: boolean;
  phase: Phase;
  report: Report | null;
  error: unknown;
  celebrate: boolean;
  /** The problem has validate.py but Python is not installed (constraints unchecked). */
  validatorNeedsPython?: boolean;
  onInstallPython?: () => void;
  /** What the last Submit did for practice (ADR 0019); null after a Run. */
  practice?: PracticeOutcome | null;
  /** The Debug tab's content (ADR 0022); no tab when null. */
  debugPanel?: ReactNode;
  /** The complexity profiler under an Accepted result (ADR 0024). */
  complexity?: ReactNode;
}) {
  const { t } = useTranslation();
  const phaseText = usePhaseText()(phase);
  const program = problem.mode === "program";
  // SQL problems (ADR 0031): tables in, rows out; no custom tests yet.
  const sql = problem.mode === "sql";
  const sqlColumns = useMemo(
    () => Object.fromEntries(parseSqlTables(problem.sql_tables).map((t) => [t.name, t.columns.map((c) => c.name)])),
    [problem.sql_tables],
  );
  const nextId = () => Math.max(0, ...customs.map((c) => c.id)) + 1;
  const customError =
    isAppError(error) && error.code === "invalid_custom_input" ? error.message : null;
  // "custom input N: <why>" from the backend: the validator's broken constraint, a wrong
  // key, or what the reference solution could not handle.
  const customParts = customError?.match(/^custom input (\d+): ([\s\S]*)$/);
  // Bring the reason into view: it sits below the custom tests the user just edited.
  const customErrorRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (customError) customErrorRef.current?.scrollIntoView?.({ block: "nearest" });
  }, [customError]);

  return (
    <Tabs
      value={tab}
      onValueChange={(v) => onTab(v as ResultsTab)}
      className="flex h-full min-h-0 flex-col gap-0"
      data-tour="results"
    >
      <div className="flex items-center gap-3 px-2 pt-2">
        <TabsList className="h-8">
          <TabsTrigger value="testcases" className="px-2 text-xs">
            {t("results.tabs.testcases")}
          </TabsTrigger>
          <TabsTrigger value="result" className="px-2 text-xs">
            {t("results.tabs.result")}
          </TabsTrigger>
          {debugPanel && (
            <TabsTrigger value="debug" className="px-2 text-xs">
              {t("debug.tab")}
            </TabsTrigger>
          )}
        </TabsList>
        {busy && (
          <span className="flex items-center gap-1.5 text-xs text-muted-foreground" aria-live="polite" data-testid="phase">
            <Loader2 aria-hidden className="size-3.5 animate-spin" />
            {phaseText}
          </span>
        )}
      </div>
      <div className="min-h-0 flex-1 overflow-auto p-3">
        <TabsContent value="testcases" className="space-y-3">
          {problem.public_tests.map((test) => (
            <section key={test.index} className="rounded-lg border p-3 text-sm" data-testid={`public-${test.index}`}>
              <div className="mb-1 flex items-center justify-between gap-2">
                <h4 className="text-xs font-semibold text-muted-foreground">
                  {t("results.testKind.public")} {test.index}
                </h4>
                <DebugThisCase input={test.input} label={`${t("results.testKind.public")} ${test.index}`} />
              </div>
              {sql ? (
                <div className="space-y-2">
                  <NamedInputTables input={test.input} columns={sqlColumns} limit={20} />
                  {asRows(test.output) && (
                    <RowsTable
                      caption={t("sqlProblem.expected")}
                      columns={asRows(test.output)!.columns}
                      rows={asRows(test.output)!.rows}
                      limit={20}
                      testId={`public-${test.index}-expected`}
                    />
                  )}
                </div>
              ) : program ? (
                <div className="grid gap-2 md:grid-cols-2">
                  <div className="min-w-0">
                    <p className="mb-1 text-xs text-muted-foreground">{t("results.program.input")}</p>
                    <pre className="max-h-48 overflow-auto rounded-md bg-muted/50 p-2 font-mono text-xs whitespace-pre" tabIndex={0}>
                      {typeof test.input.stdin === "string" ? test.input.stdin : ""}
                    </pre>
                  </div>
                  <div className="min-w-0">
                    <p className="mb-1 text-xs text-muted-foreground">{t("results.program.expected")}</p>
                    <pre className="max-h-48 overflow-auto rounded-md bg-muted/50 p-2 font-mono text-xs whitespace-pre" tabIndex={0}>
                      {typeof test.output === "string" ? test.output : ""}
                    </pre>
                  </div>
                </div>
              ) : (
                <>
                  <pre className="font-mono text-xs whitespace-pre-wrap break-all">
                    {Object.entries(test.input)
                      .map(([k, v]) => `${k} = ${JSON.stringify(v)}`)
                      .join("\n")}
                  </pre>
                  <p className="mt-1 font-mono text-xs text-muted-foreground">
                    {t("results.expected")}: {JSON.stringify(test.output)}
                  </p>
                </>
              )}
            </section>
          ))}
          {program &&
            customs.map((c, i) => (
              <ProgramCustomCard
                key={c.id}
                index={i + 1}
                draft={c}
                onChange={(values) => onCustoms(customs.map((x) => (x.id === c.id ? { ...x, values } : x)))}
                onRemove={() => onCustoms(customs.filter((x) => x.id !== c.id))}
              />
            ))}
          {signature &&
            !program &&
            customs.map((c, i) => (
              <CustomTestCard
                key={c.id}
                index={i + 1}
                signature={signature}
                draft={c}
                onChange={(values) => onCustoms(customs.map((x) => (x.id === c.id ? { ...x, values } : x)))}
                onRemove={() => onCustoms(customs.filter((x) => x.id !== c.id))}
              />
            ))}
          {customError && (
            <div role="alert" data-testid="custom-error" className="space-y-1" ref={customErrorRef}>
              {customParts ? (
                <p className="rounded-lg border border-destructive/40 bg-card p-3 text-sm">
                  {t("results.custom.invalid", { index: customParts[1], message: customParts[2] })}
                </p>
              ) : (
                <ErrorNotice error={error} />
              )}
            </div>
          )}
          {validatorNeedsPython && customs.length > 0 && (
            <p className="flex flex-wrap items-center gap-2 text-xs text-muted-foreground" data-testid="validator-needs-python">
              {t("results.custom.needsPython")}
              {onInstallPython && (
                <Button size="sm" variant="outline" className="h-6 px-2 text-xs" onClick={onInstallPython}>
                  {t("results.custom.installPython")}
                </Button>
              )}
            </p>
          )}
          {sql && (
            <p className="text-xs text-muted-foreground" data-testid="sql-no-custom">
              {t("sqlProblem.customUnavailable")}
            </p>
          )}
          {signature && !sql && (
            <div className="flex flex-wrap gap-2">
              <Button
                size="sm"
                variant="outline"
                onClick={() =>
                  onCustoms([
                    ...customs,
                    {
                      id: nextId(),
                      values: program
                        ? { stdin: typeof problem.public_tests[0]?.input.stdin === "string" ? problem.public_tests[0].input.stdin : "" }
                        : problem.public_tests[0]
                          ? valuesFrom(signature, problem.public_tests[0].input)
                          : Object.fromEntries(signature.params.map((p) => [p.name, ""])),
                    },
                  ])
                }
              >
                <Plus aria-hidden />
                {t("results.custom.add")}
              </Button>
            </div>
          )}
        </TabsContent>
        <TabsContent value="result">
          {error && !customError ? (
            <ErrorNotice error={error} />
          ) : report ? (
            <ResultView
              report={report}
              celebrate={celebrate}
              mode={problem.mode}
              checker={problem.checker}
              sqlColumns={sqlColumns}
              afterSummary={
                (practice || complexity) && (
                  <>
                    {practice && <SubmitPractice practice={practice} accepted={report.verdict === "accepted"} />}
                    {complexity}
                  </>
                )
              }
            />
          ) : busy ? (
            <p className="flex items-center gap-2 text-sm text-muted-foreground">
              <Loader2 aria-hidden className="size-4 animate-spin" />
              {phaseText}
            </p>
          ) : (
            <p className="text-sm text-muted-foreground">{t("results.empty")}</p>
          )}
        </TabsContent>
        {debugPanel && <TabsContent value="debug">{debugPanel}</TabsContent>}
      </div>
    </Tabs>
  );
}

function fieldErrorText(t: TFunction, e: FieldError): string {
  switch (e.kind) {
    case "empty":
      return t("results.custom.errors.empty");
    case "json":
      return t("results.custom.errors.json");
    case "type": {
      // Lists and trees get a hint at their JSON form (ADR 0025).
      const element = elementType(e.expected);
      if (element === "ListNode") return t("results.custom.errors.list", { type: e.expected });
      if (element === "TreeNode") return t("results.custom.errors.tree", { type: e.expected });
      return t("results.custom.errors.type", { type: e.expected });
    }
  }
}

/** A program problem's custom test (ADR 0028): the stdin text, typed as is. The expected
 * output comes from the reference solution when it runs. */
function ProgramCustomCard({
  index,
  draft,
  onChange,
  onRemove,
}: {
  index: number;
  draft: CustomTestDraft;
  onChange: (values: Record<string, string>) => void;
  onRemove: () => void;
}) {
  const { t } = useTranslation();
  const id = `custom-${draft.id}-stdin`;
  const stdin = draft.values.stdin ?? "";
  return (
    <section className="space-y-2 rounded-lg border border-dashed p-3" data-testid={`custom-${index}`}>
      <div className="flex items-center justify-between">
        <h4 className="text-xs font-semibold text-muted-foreground">
          {t("results.testKind.custom")} {index}
        </h4>
        <DebugThisCase input={{ stdin }} label={`${t("results.testKind.custom")} ${index}`} />
        <Button size="sm" variant="ghost" onClick={onRemove} aria-label={t("results.custom.remove", { index })}>
          <Trash2 aria-hidden />
        </Button>
      </div>
      <label htmlFor={id} className="block text-xs text-muted-foreground">
        {t("results.program.input")}
      </label>
      <textarea
        id={id}
        value={stdin}
        onChange={(e) => onChange({ stdin: e.target.value })}
        spellCheck={false}
        rows={Math.min(12, Math.max(3, stdin.split("\n").length))}
        className="w-full rounded-md border bg-background p-2 font-mono text-xs focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
      />
    </section>
  );
}

function CustomTestCard({
  index,
  signature,
  draft,
  onChange,
  onRemove,
}: {
  index: number;
  signature: Signature;
  draft: CustomTestDraft;
  onChange: (values: Record<string, string>) => void;
  onRemove: () => void;
}) {
  const { t } = useTranslation();
  const { errors, input } = validateCustom(signature, draft.values);
  return (
    <section className="space-y-2 rounded-lg border border-dashed p-3" data-testid={`custom-${index}`}>
      <div className="flex items-center justify-between">
        <h4 className="text-xs font-semibold text-muted-foreground">
          {t("results.testKind.custom")} {index}
        </h4>
        <DebugThisCase input={input} label={`${t("results.testKind.custom")} ${index}`} />
        <Button size="sm" variant="ghost" onClick={onRemove} aria-label={t("results.custom.remove", { index })}>
          <Trash2 aria-hidden />
        </Button>
      </div>
      {signature.params.map((p) => {
        const err = errors[p.name];
        const id = `custom-${draft.id}-${p.name}`;
        return (
          <div key={p.name} className="grid grid-cols-[auto_1fr] items-start gap-x-2 gap-y-1">
            <label htmlFor={id} className="pt-1.5 font-mono text-xs">
              {p.name}
              <span className="text-muted-foreground">: {p.type}</span>
            </label>
            <div>
              <Input
                id={id}
                value={draft.values[p.name] ?? ""}
                onChange={(e) => onChange({ ...draft.values, [p.name]: e.target.value })}
                aria-invalid={!!err}
                aria-describedby={err ? `${id}-error` : undefined}
                spellCheck={false}
                className={cn("h-8 font-mono text-xs")}
              />
              {err && (
                <p id={`${id}-error`} className="mt-1 text-xs text-destructive">
                  {fieldErrorText(t, err)}
                </p>
              )}
            </div>
          </div>
        );
      })}
    </section>
  );
}
