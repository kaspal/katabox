// The stress lab (expansion M8, ADR 0036): write a slow but obviously correct brute force
// beside your solution, then let kb-app run both on many small inputs in the sandbox until
// they disagree. A disagreement can be shrunk, debugged or added to the custom tests.
import { useContext, useEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { FlaskConical, Loader2, Plus, Square, TriangleAlert } from "lucide-react";
import type { Language, StressEvent, StressResult } from "@/bindings";
import { Button } from "@/components/ui/button";
import { ErrorNotice } from "@/components/ErrorNotice";
import { VerdictBadge } from "@/components/Verdict";
import { api, type CustomInput } from "@/lib/api";
import { DebugThisCase } from "@/workspace/debug/context";
import { CodeEditor } from "@/workspace/editor/CodeEditor";
import { ShrinkContext, ShrinkPanel } from "@/workspace/results/Shrink";

const SAVE_MS = 800;

const show = (input: CustomInput) =>
  Object.entries(input)
    .map(([k, v]) => `${k} = ${typeof v === "string" && k === "stdin" ? `\n${v}` : JSON.stringify(v)}`)
    .join("\n");

const text = (v: unknown) => (typeof v === "string" ? v : JSON.stringify(v));

type State =
  | { kind: "idle" }
  | { kind: "running"; progress: StressEvent | null }
  | { kind: "done"; result: StressResult }
  | { kind: "error"; error: unknown };

export function StressPanel({
  problemId,
  language,
  mode,
}: {
  problemId: string;
  language: Language;
  mode: "function" | "program" | "sql";
}) {
  const { t } = useTranslation();
  const ctx = useContext(ShrinkContext);
  const [brute, setBrute] = useState<string | null>(null);
  const [state, setState] = useState<State>({ kind: "idle" });
  const timer = useRef<number | undefined>(undefined);
  const latest = useRef("");

  // The saved brute force, else the language's starter.
  useEffect(() => {
    let live = true;
    setBrute(null);
    setState({ kind: "idle" });
    if (mode === "sql") return;
    void (async () => {
      const saved = await api.stressBrute(problemId, language).catch(() => null);
      const text = saved ?? (await api.getStub(problemId, language).catch(() => ""));
      if (live) {
        latest.current = text;
        setBrute(text);
      }
    })();
    return () => {
      live = false;
      window.clearTimeout(timer.current);
    };
  }, [problemId, language, mode]);

  if (mode === "sql") {
    return (
      <p role="status" className="text-sm text-muted-foreground" data-testid="stress-unavailable">
        {t("stress.sql")}
      </p>
    );
  }
  if (!ctx) return null;

  const onBrute = (code: string) => {
    latest.current = code;
    window.clearTimeout(timer.current);
    timer.current = window.setTimeout(() => void api.saveStressBrute(problemId, language, code).catch(() => {}), SAVE_MS);
  };

  const start = async () => {
    window.clearTimeout(timer.current);
    await api.saveStressBrute(problemId, language, latest.current).catch(() => {});
    setState({ kind: "running", progress: null });
    try {
      const result = await api.stressTest(problemId, language, ctx.code(), latest.current, (e) =>
        setState({ kind: "running", progress: e }),
      );
      setState({ kind: "done", result });
    } catch (error) {
      setState({ kind: "error", error });
    }
  };

  const running = state.kind === "running";
  const p = running ? state.progress : null;
  const r = state.kind === "done" ? state.result : null;

  return (
    <div className="space-y-3" data-testid="stress-panel">
      <p className="text-sm text-muted-foreground">{t("stress.intro")}</p>
      <div className="h-48 overflow-hidden rounded-md border" data-testid="stress-brute">
        {brute !== null && (
          <CodeEditor
            docKey={`stress:${problemId}:${language}`}
            value={brute}
            lang={language}
            ariaLabel={t("stress.bruteLabel", { language: t(`languages.${language}`) })}
            onChange={onBrute}
            readOnly={running}
          />
        )}
      </div>
      <div className="flex flex-wrap items-center gap-2">
        {running ? (
          <Button size="sm" variant="outline" onClick={() => void api.cancelStress()}>
            <Square aria-hidden />
            {t("stress.stop")}
          </Button>
        ) : (
          <Button
            size="sm"
            onClick={() => void start()}
            disabled={brute === null || ctx.disabledReason !== null}
            title={ctx.disabledReason ?? undefined}
            data-testid="stress-start"
          >
            <FlaskConical aria-hidden />
            {t("stress.start")}
          </Button>
        )}
        {running && (
          <span className="flex items-center gap-1.5 text-sm" role="status" data-testid="stress-running">
            <Loader2 aria-hidden className="size-4 animate-spin" />
            {p ? t("stress.progress", { rounds: p.rounds, max: p.max_rounds, tested: p.tested }) : t("stress.starting")}
          </span>
        )}
      </div>
      {state.kind === "error" && <ErrorNotice error={state.error} onRetry={() => void start()} />}
      {r && (
        <section className="space-y-3 rounded-md border p-3" data-testid="stress-done" aria-label={t("stress.title")}>
          {r.source === "random" && (
            <p className="flex items-start gap-1.5 text-xs text-muted-foreground" data-testid="stress-random">
              <TriangleAlert aria-hidden className="mt-0.5 size-3.5 flex-none" />
              {t("stress.random")}
            </p>
          )}
          <h5 className="flex flex-wrap items-center gap-2 text-sm font-semibold">
            {t(`stress.stopped.${r.stop}`, { tested: r.tested, rounds: r.rounds })}
            {r.verdict && <VerdictBadge verdict={r.verdict} />}
          </h5>
          {r.message && <pre className="whitespace-pre-wrap rounded-md border bg-muted/50 p-2 font-mono text-xs">{r.message.text}</pre>}
          {r.input && (
            <pre className="overflow-x-auto rounded-md border bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap break-all" data-testid="stress-input">
              {show(r.input)}
            </pre>
          )}
          {r.stop === "found" && r.expected !== null && (
            <div className="grid gap-2 text-xs md:grid-cols-2">
              <div>
                <p className="mb-1 font-medium text-muted-foreground">{t("stress.bruteOutput")}</p>
                <pre className="rounded-md border bg-muted/50 p-2 font-mono whitespace-pre-wrap break-all">{text(r.expected)}</pre>
              </div>
              <div>
                <p className="mb-1 font-medium text-muted-foreground">{t("stress.yourOutput")}</p>
                <pre className="rounded-md border bg-muted/50 p-2 font-mono whitespace-pre-wrap break-all">
                  {r.actual === null ? "—" : text(r.actual)}
                </pre>
              </div>
            </div>
          )}
          {r.stop === "found" && r.input && (
            <div className="flex flex-wrap items-center gap-2">
              <DebugThisCase input={r.input} label={t("stress.title")} />
              <Button size="sm" variant="outline" onClick={() => ctx.addCustom(r.input!)}>
                <Plus aria-hidden />
                {t("results.shrink.addToTests")}
              </Button>
              {mode === "function" && <ShrinkPanel input={r.input} />}
            </div>
          )}
        </section>
      )}
    </div>
  );
}
