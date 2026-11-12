import { useEffect, useRef } from "react";
import type { PracticeOutcome } from "@/bindings";
import { SubmitPractice } from "@/practice/SubmitPractice";
import { useTranslation } from "react-i18next";
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
  validateCustom,
  valuesFrom,
  type CustomTestDraft,
  type FieldError,
  type Signature,
} from "@/workspace/results/validate";
import type { Report } from "@/bindings";

export type ResultsTab = "testcases" | "result";

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
}) {
  const { t } = useTranslation();
  const phaseText = usePhaseText()(phase);
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
    <Tabs value={tab} onValueChange={(v) => onTab(v as ResultsTab)} className="flex h-full min-h-0 flex-col gap-0">
      <div className="flex items-center gap-3 px-2 pt-2">
        <TabsList className="h-8">
          <TabsTrigger value="testcases" className="px-2 text-xs">
            {t("results.tabs.testcases")}
          </TabsTrigger>
          <TabsTrigger value="result" className="px-2 text-xs">
            {t("results.tabs.result")}
          </TabsTrigger>
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
            <section key={test.index} className="rounded-lg border p-3 text-sm">
              <h4 className="mb-1 text-xs font-semibold text-muted-foreground">
                {t("results.testKind.public")} {test.index}
              </h4>
              <pre className="font-mono text-xs whitespace-pre-wrap break-all">
                {Object.entries(test.input)
                  .map(([k, v]) => `${k} = ${JSON.stringify(v)}`)
                  .join("\n")}
              </pre>
              <p className="mt-1 font-mono text-xs text-muted-foreground">
                {t("results.expected")}: {JSON.stringify(test.output)}
              </p>
            </section>
          ))}
          {signature &&
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
          {signature && (
            <div className="flex flex-wrap gap-2">
              <Button
                size="sm"
                variant="outline"
                onClick={() =>
                  onCustoms([
                    ...customs,
                    {
                      id: nextId(),
                      values: problem.public_tests[0]
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
            <>
              <ResultView report={report} celebrate={celebrate} />
              {practice && <SubmitPractice practice={practice} accepted={report.verdict === "accepted"} />}
            </>
          ) : busy ? (
            <p className="flex items-center gap-2 text-sm text-muted-foreground">
              <Loader2 aria-hidden className="size-4 animate-spin" />
              {phaseText}
            </p>
          ) : (
            <p className="text-sm text-muted-foreground">{t("results.empty")}</p>
          )}
        </TabsContent>
      </div>
    </Tabs>
  );
}

function fieldErrorText(t: ReturnType<typeof useTranslation>["t"], e: FieldError): string {
  switch (e.kind) {
    case "empty":
      return t("results.custom.errors.empty");
    case "json":
      return t("results.custom.errors.json");
    case "type":
      return t("results.custom.errors.type", { type: e.expected });
  }
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
  const { errors } = validateCustom(signature, draft.values);
  return (
    <section className="space-y-2 rounded-lg border border-dashed p-3" data-testid={`custom-${index}`}>
      <div className="flex items-center justify-between">
        <h4 className="text-xs font-semibold text-muted-foreground">
          {t("results.testKind.custom")} {index}
        </h4>
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
