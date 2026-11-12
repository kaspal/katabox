// The Debug tab (ADR 0022): toolbar, where the program is, Variables, Watch, Call Stack
// and the Debug Console. Every action replays the test in the sandbox.
import { useState, type ReactNode } from "react";
import { useTranslation } from "react-i18next";
import {
  ArrowDownToLine,
  ArrowLeft,
  ArrowRight,
  ArrowUpFromLine,
  ChevronDown,
  ChevronRight,
  Loader2,
  Play,
  RotateCcw,
  Square,
  X,
} from "lucide-react";
import type { DebugAction, StopInfo, VarInfo } from "@/bindings";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { ErrorNotice } from "@/components/ErrorNotice";
import { cn } from "@/lib/utils";
import type { Debugger, VarPath } from "@/workspace/debug/useDebugger";

export const DEBUG_KEYS: Record<DebugAction | "stop", string> = {
  continue: "F5",
  step_over: "F10",
  step_into: "F11",
  step_out: "Shift+F11",
  step_back: "Shift+F10",
  restart: "Ctrl+Shift+F5",
  stop: "Shift+F5",
};

function ToolButton({
  label,
  keys,
  onClick,
  disabled,
  children,
}: {
  label: string;
  keys: string;
  onClick: () => void;
  disabled?: boolean;
  children: ReactNode;
}) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button
          size="icon"
          variant="ghost"
          className="size-7"
          aria-label={label}
          aria-keyshortcuts={keys}
          onClick={onClick}
          disabled={disabled}
        >
          {children}
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        {label} ({keys})
      </TooltipContent>
    </Tooltip>
  );
}

export function DebugToolbar({ dbg }: { dbg: Debugger }) {
  const { t } = useTranslation();
  const paused = dbg.paused;
  const can = (a: DebugAction) => {
    if (dbg.busy || !dbg.active) return false;
    if (a === "restart" || a === "step_back") return true;
    return paused;
  };
  const tool = (a: DebugAction, icon: ReactNode) => (
    <ToolButton label={t(`debug.actions.${a}`)} keys={DEBUG_KEYS[a]} onClick={() => void dbg.act(a)} disabled={!can(a)}>
      {icon}
    </ToolButton>
  );
  return (
    <div className="flex items-center gap-0.5" role="toolbar" aria-label={t("debug.toolbar")} data-testid="debug-toolbar">
      {tool("continue", <Play aria-hidden />)}
      {tool("step_over", <ArrowRight aria-hidden />)}
      {tool("step_into", <ArrowDownToLine aria-hidden />)}
      {tool("step_out", <ArrowUpFromLine aria-hidden />)}
      {tool("step_back", <ArrowLeft aria-hidden />)}
      {tool("restart", <RotateCcw aria-hidden />)}
      <ToolButton label={t("debug.actions.stop")} keys={DEBUG_KEYS.stop} onClick={() => void dbg.stop()}>
        <Square aria-hidden />
      </ToolButton>
      {dbg.busy && <Loader2 aria-label={t("debug.replaying")} className="ml-1 size-4 animate-spin text-muted-foreground" />}
    </div>
  );
}

function Section({ title, children, testId }: { title: string; children: ReactNode; testId: string }) {
  return (
    <section className="min-w-0 space-y-1" aria-label={title} data-testid={testId}>
      <h4 className="text-xs font-semibold text-muted-foreground uppercase">{title}</h4>
      {children}
    </section>
  );
}

function VarRow({
  v,
  path,
  depth,
  stop,
  dbg,
}: {
  v: VarInfo;
  path: VarPath;
  depth: number;
  stop: StopInfo;
  dbg: Debugger;
}) {
  const { t } = useTranslation();
  const open = dbg.isExpanded(path);
  const kids = stop.expanded.find((e) => JSON.stringify(e.path) === JSON.stringify(path));
  const name = v.name || t("debug.item");
  return (
    <li role="treeitem" aria-expanded={v.children > 0 ? open : undefined} aria-level={depth + 1}>
      <div className="flex min-w-0 items-baseline gap-1 font-mono text-xs" style={{ paddingLeft: depth * 12 }}>
        {v.children > 0 ? (
          <button
            type="button"
            className="shrink-0 rounded text-muted-foreground hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
            aria-label={t(open ? "debug.collapse" : "debug.expand", { name })}
            onClick={() => void dbg.toggleExpand(path)}
            disabled={dbg.busy}
          >
            {open ? <ChevronDown aria-hidden className="size-3" /> : <ChevronRight aria-hidden className="size-3" />}
          </button>
        ) : (
          <span className="inline-block w-3 shrink-0" />
        )}
        <span className="shrink-0 font-medium text-foreground">{name}</span>
        <span className="truncate" title={v.value}>
          {v.value}
        </span>
        <span className="shrink-0 text-muted-foreground">{v.type}</span>
      </div>
      {open && kids && (
        <ul role="group">
          {kids.children.map((c, i) => (
            <VarRow key={i} v={c} path={[...path, i]} depth={depth + 1} stop={stop} dbg={dbg} />
          ))}
          {kids.more > 0 && (
            <li className="font-mono text-xs text-muted-foreground" style={{ paddingLeft: (depth + 1) * 12 + 16 }}>
              {t("debug.more", { count: kids.more })}
            </li>
          )}
        </ul>
      )}
    </li>
  );
}

function Variables({ stop, dbg }: { stop: StopInfo; dbg: Debugger }) {
  const { t } = useTranslation();
  const scope = (name: "locals" | "globals", vars: VarInfo[]) =>
    vars.length > 0 && (
      <div>
        <p className="text-xs text-muted-foreground">{t(`debug.scopes.${name}`)}</p>
        <ul role="tree" aria-label={t(`debug.scopes.${name}`)}>
          {vars.map((v) => (
            <VarRow key={v.name} v={v} path={[name, v.name]} depth={0} stop={stop} dbg={dbg} />
          ))}
        </ul>
      </div>
    );
  return (
    <Section title={t("debug.variables")} testId="debug-variables">
      {stop.locals.length + stop.globals.length === 0 ? (
        <p className="text-xs text-muted-foreground">{t("debug.noVariables")}</p>
      ) : (
        <>
          {scope("locals", stop.locals)}
          {scope("globals", stop.globals)}
        </>
      )}
    </Section>
  );
}

function Watch({ stop, dbg }: { stop: StopInfo | null; dbg: Debugger }) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState("");
  return (
    <Section title={t("debug.watch")} testId="debug-watch">
      <ul className="space-y-0.5">
        {dbg.watches.map((w, i) => {
          const r = stop?.evals[i];
          return (
            <li key={`${i}-${w}`} className="flex items-baseline gap-1 font-mono text-xs">
              <span className="shrink-0 font-medium text-foreground">{w}</span>
              <span className={cn("truncate", r && !r.ok && "text-destructive")}>
                {r ? (r.ok ? (r.value?.value ?? "") : r.error) : ""}
              </span>
              <button
                type="button"
                className="ml-auto shrink-0 text-muted-foreground hover:text-foreground"
                aria-label={t("debug.removeWatch", { expr: w })}
                onClick={() => void dbg.removeWatch(i)}
                disabled={dbg.busy}
              >
                <X aria-hidden className="size-3" />
              </button>
            </li>
          );
        })}
      </ul>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          if (draft.trim()) void dbg.addWatch(draft.trim());
          setDraft("");
        }}
      >
        <Input
          value={draft}
          onChange={(e) => setDraft(e.target.value)}
          placeholder={t("debug.addWatch")}
          aria-label={t("debug.addWatch")}
          className="h-7 font-mono text-xs"
          spellCheck={false}
        />
      </form>
    </Section>
  );
}

function CallStack({ stop, dbg }: { stop: StopInfo; dbg: Debugger }) {
  const { t } = useTranslation();
  return (
    <Section title={t("debug.callStack")} testId="debug-stack">
      <ol className="space-y-0.5">
        {stop.stack.map((f, i) => (
          <li key={i}>
            <button
              type="button"
              className={cn(
                "w-full rounded px-1 text-left font-mono text-xs hover:bg-accent",
                i === stop.frame && "bg-accent font-semibold",
              )}
              aria-current={i === stop.frame ? "true" : undefined}
              onClick={() => void dbg.selectFrame(i)}
              disabled={dbg.busy}
            >
              {f.function} <span className="text-muted-foreground">· {t("debug.line", { line: f.line })}</span>
            </button>
          </li>
        ))}
      </ol>
    </Section>
  );
}

function DebugConsole({ dbg }: { dbg: Debugger }) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState("");
  const prints = (dbg.view?.prints ?? []).map((p) => p.text).join("");
  return (
    <Section title={t("debug.console")} testId="debug-console">
      <div className="max-h-48 overflow-auto rounded-md border bg-muted/50 p-2 font-mono text-xs" aria-live="polite">
        {prints && <pre className="whitespace-pre-wrap" data-testid="debug-prints">{prints}</pre>}
        {dbg.consoleLog.map((e) => (
          <div key={e.id} className="border-t pt-1 first:border-t-0">
            <p className="text-muted-foreground">› {e.result.expr}</p>
            {e.result.stdout && <pre className="whitespace-pre-wrap">{e.result.stdout}</pre>}
            <p className={cn(!e.result.ok && "text-destructive")}>
              {e.result.ok ? (e.result.value?.value ?? "") : e.result.error}
            </p>
          </div>
        ))}
      </div>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          if (draft.trim()) void dbg.evaluate(draft.trim());
          setDraft("");
        }}
      >
        <Input
          value={draft}
          onChange={(e) => setDraft(e.target.value)}
          placeholder={t("debug.evaluate")}
          aria-label={t("debug.evaluate")}
          aria-describedby="debug-console-help"
          className="h-7 font-mono text-xs"
          spellCheck={false}
          disabled={!dbg.paused}
        />
      </form>
      <p id="debug-console-help" className="text-xs text-muted-foreground">
        {t("debug.consoleHelp")}
      </p>
    </Section>
  );
}

function Status({ dbg }: { dbg: Debugger }) {
  const { t } = useTranslation();
  const v = dbg.view;
  if (!v) return null;
  const s = v.state;
  let text: ReactNode;
  if (s.state === "paused") {
    text = t("debug.pausedAt", { line: s.line, step: s.stop.step + 1, total: v.steps });
  } else if (s.state === "finished") {
    const o = s.outcome;
    if (s.truncated) text = t("debug.truncated", { count: v.steps });
    else if (o?.status === "ok") text = t("debug.returned", { value: JSON.stringify(o.value) });
    else if (o?.status === "error") text = t("debug.raised", { line: o.line ?? "?" });
    else text = t("debug.memory");
  } else {
    text = t(`debug.failed.${s.reason}` as "debug.failed.load_error");
  }
  return (
    <div className="space-y-1" data-testid="debug-status">
      <p className="text-sm" role="status">
        <span className="font-medium">{dbg.label}</span> · {text}
      </p>
      {s.state === "finished" && s.outcome?.status === "error" && s.outcome.error && (
        <pre className="max-h-32 overflow-auto rounded-md bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap">
          {s.outcome.error}
        </pre>
      )}
      {s.state === "failed" && s.message && (
        <pre className="max-h-32 overflow-auto rounded-md bg-muted/50 p-2 font-mono text-xs whitespace-pre-wrap">
          {s.message}
        </pre>
      )}
    </div>
  );
}

export function DebugPanel({ dbg }: { dbg: Debugger }) {
  const { t } = useTranslation();
  const stop = dbg.view?.state.state === "paused" ? dbg.view.state.stop : null;
  if (!dbg.view && !dbg.busy && !dbg.error) {
    return <p className="text-sm text-muted-foreground">{t("debug.empty")}</p>;
  }
  return (
    <div className="space-y-3" data-testid="debug-panel">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <Status dbg={dbg} />
        <DebugToolbar dbg={dbg} />
      </div>
      {dbg.error !== null && <ErrorNotice error={dbg.error} />}
      <div className="grid gap-4 md:grid-cols-2">
        <div className="space-y-3">
          {stop && <Variables stop={stop} dbg={dbg} />}
          <Watch stop={stop} dbg={dbg} />
        </div>
        <div className="space-y-3">
          {stop && <CallStack stop={stop} dbg={dbg} />}
          <DebugConsole dbg={dbg} />
        </div>
      </div>
    </div>
  );
}

/** "Debug this case" for a test card. */
export function DebugCaseButton({
  onDebug,
  disabledReason,
}: {
  onDebug: () => void;
  disabledReason: string | null;
}) {
  const { t } = useTranslation();
  const button = (
    <Button
      size="sm"
      variant="ghost"
      className="h-6 px-2 text-xs"
      onClick={onDebug}
      disabled={disabledReason !== null}
      aria-keyshortcuts="F5"
    >
      {t("debug.debugCase")}
    </Button>
  );
  if (disabledReason === null) return button;
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <span tabIndex={0}>{button}</span>
      </TooltipTrigger>
      <TooltipContent>{disabledReason}</TooltipContent>
    </Tooltip>
  );
}
