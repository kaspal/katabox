// "Find a smaller failing input" (ADR 0030): on a failing test whose input is already
// shown, shrink it in the sandbox (kb-app) to a smaller input that still fails, with
// progress and Stop; then debug it or add it to the custom tests.
import { createContext, useContext, useState } from "react";
import { useTranslation } from "react-i18next";
import { Loader2, Minimize2, Plus, Square } from "lucide-react";
import type { Language, ShrinkEvent, ShrinkResult } from "@/bindings";
import { Button } from "@/components/ui/button";
import { ErrorNotice } from "@/components/ErrorNotice";
import { VerdictBadge } from "@/components/Verdict";
import { api, type CustomInput } from "@/lib/api";
import { DebugThisCase } from "@/workspace/debug/context";

export type ShrinkCase = {
  problemId: string;
  language: Language;
  /** The editor's code now. */
  code: () => string;
  /** Why shrinking is unavailable now (a judge run going on), or null. */
  disabledReason: string | null;
  /** Add an input to the custom tests (Testcases tab). */
  addCustom: (input: CustomInput) => void;
};

export const ShrinkContext = createContext<ShrinkCase | null>(null);

const show = (input: CustomInput) =>
  Object.entries(input)
    .map(([k, v]) => `${k} = ${JSON.stringify(v)}`)
    .join("\n");

type State =
  | { kind: "idle" }
  | { kind: "running"; progress: ShrinkEvent | null }
  | { kind: "done"; result: ShrinkResult }
  | { kind: "error"; error: unknown };

/** The button and its panel, under a failing test's details. */
export function ShrinkPanel({ input }: { input: CustomInput }) {
  const { t } = useTranslation();
  const ctx = useContext(ShrinkContext);
  const [state, setState] = useState<State>({ kind: "idle" });
  if (!ctx) return null;

  const start = async () => {
    setState({ kind: "running", progress: null });
    try {
      const result = await api.shrink(ctx.problemId, ctx.language, ctx.code(), input, (e) =>
        setState({ kind: "running", progress: e }),
      );
      setState({ kind: "done", result });
    } catch (error) {
      setState({ kind: "error", error });
    }
  };

  if (state.kind === "idle") {
    return (
      <Button
        size="sm"
        variant="outline"
        disabled={ctx.disabledReason !== null}
        title={ctx.disabledReason ?? undefined}
        onClick={() => void start()}
        data-testid="shrink-start"
      >
        <Minimize2 aria-hidden />
        {t("results.shrink.start")}
      </Button>
    );
  }
  if (state.kind === "running") {
    const p = state.progress;
    const pct = p ? Math.round((100 * p.runs) / p.max_runs) : 0;
    return (
      <div className="space-y-2 rounded-md border p-3" data-testid="shrink-running">
        <p className="flex items-center gap-2 text-sm">
          <Loader2 aria-hidden className="size-4 animate-spin" />
          {p ? t("results.shrink.progress", { size: p.size, original: p.original_size }) : t("results.shrink.starting")}
        </p>
        <div
          role="progressbar"
          aria-label={t("results.shrink.runs", { runs: p?.runs ?? 0, max: p?.max_runs ?? 60 })}
          aria-valuemin={0}
          aria-valuemax={p?.max_runs ?? 60}
          aria-valuenow={p?.runs ?? 0}
          className="h-1.5 overflow-hidden rounded-full bg-muted"
        >
          <div className="h-full rounded-full bg-primary transition-[width]" style={{ width: `${pct}%` }} />
        </div>
        <Button size="sm" variant="outline" onClick={() => void api.cancelShrink()}>
          <Square aria-hidden />
          {t("results.shrink.stop")}
        </Button>
      </div>
    );
  }
  if (state.kind === "error") {
    return <ErrorNotice error={state.error} onRetry={() => void start()} />;
  }
  const r = state.result;
  if (r.stop === "not_failing") {
    return (
      <p role="status" className="text-sm text-muted-foreground" data-testid="shrink-done">
        {t("results.shrink.notFailing")}
      </p>
    );
  }
  return (
    <section className="space-y-3 rounded-md border p-3" data-testid="shrink-done" aria-label={t("results.shrink.title")}>
      <h5 className="flex flex-wrap items-center gap-2 text-sm font-semibold">
        {t("results.shrink.title")}
        <VerdictBadge verdict={r.verdict} />
      </h5>
      <p role="status" className="text-sm text-muted-foreground">
        {t(`results.shrink.stopped.${r.stop}`, { size: r.size, original: r.original_size, runs: r.runs })}
      </p>
      <pre className="overflow-x-auto rounded-md border bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap break-all" data-testid="shrink-input">
        {show(r.input)}
      </pre>
      <div className="grid gap-2 text-xs md:grid-cols-2">
        <div>
          <p className="mb-1 font-medium text-muted-foreground">{t("results.expected")}</p>
          <pre className="rounded-md border bg-muted/50 p-2 font-mono whitespace-pre-wrap break-all">{JSON.stringify(r.expected)}</pre>
        </div>
        <div>
          <p className="mb-1 font-medium text-muted-foreground">{t("results.actual")}</p>
          <pre className="rounded-md border bg-muted/50 p-2 font-mono whitespace-pre-wrap break-all">
            {r.actual === null ? "—" : JSON.stringify(r.actual)}
          </pre>
        </div>
      </div>
      <div className="flex flex-wrap items-center gap-2">
        <DebugThisCase input={r.input} label={t("results.shrink.title")} />
        <Button size="sm" variant="outline" onClick={() => ctx.addCustom(r.input)}>
          <Plus aria-hidden />
          {t("results.shrink.addToTests")}
        </Button>
      </div>
    </section>
  );
}
