// The debugger's UI state (ADR 0022). Every action is a replay run in the sandbox, so
// this hook only keeps what the user sees and asks for: breakpoints (per document),
// watches, expanded variables, the selected frame and the console history.
import { useCallback, useRef, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import type {
  DebugAction,
  DebugView,
  EvalResult,
  JsonValue,
  Language,
} from "@/bindings";
import { api, type CustomInput } from "@/lib/api";

export type ConsoleEntry = { id: number; result: EvalResult };

/** A path into the variables: `[scope, name, i, j, …]`. */
export type VarPath = JsonValue[];
const keyOf = (p: VarPath) => JSON.stringify(p);
const NO_LINES: number[] = [];

export function useDebugLanguages() {
  return useQuery({
    queryKey: ["debug-languages"],
    queryFn: () => api.debugLanguages(),
    staleTime: Infinity,
  });
}

export function useDebugger(problem: string, language: Language | null, docKey: string) {
  const [view, setView] = useState<DebugView | null>(null);
  const [label, setLabel] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>(null);
  const [breakpoints, setBreakpoints] = useState<Record<string, number[]>>({});
  const [watches, setWatches] = useState<string[]>([]);
  const [expanded, setExpanded] = useState<VarPath[]>([]);
  const [frame, setFrame] = useState(0);
  const [consoleLog, setConsoleLog] = useState<ConsoleEntry[]>([]);
  const running = useRef(false);
  const nextId = useRef(1);

  // A stable empty list: a new [] every render would rebuild the editor's extension.
  const lines = breakpoints[docKey] ?? NO_LINES;
  const session = view && view.state.state !== "failed" ? view.session : null;

  /** Run one backend call at a time; errors are kept for the panel. */
  const guard = useCallback(async <T,>(job: () => Promise<T>): Promise<T | undefined> => {
    if (running.current) return undefined;
    running.current = true;
    setBusy(true);
    setError(null);
    try {
      return await job();
    } catch (e) {
      setError(e);
      return undefined;
    } finally {
      running.current = false;
      setBusy(false);
    }
  }, []);

  const start = useCallback(
    (code: string, input: CustomInput, testLabel: string) =>
      guard(async () => {
        if (!language) return;
        if (view && view.state.state !== "failed") await api.debugEnd(view.session).catch(() => null);
        setExpanded([]);
        setFrame(0);
        setConsoleLog([]);
        setLabel(testLabel);
        const v = await api.debugStart({
          problem,
          language,
          code,
          input: input as { [key in string]: JsonValue },
          breakpoints: lines,
          watches,
        });
        setView(v);
      }),
    [guard, language, problem, lines, watches, view],
  );

  const act = useCallback(
    (action: DebugAction) =>
      guard(async () => {
        if (session === null) return;
        setFrame(0);
        setView(await api.debugAction(session, action));
      }),
    [guard, session],
  );

  const inspect = useCallback(
    (next: { frame?: number; expanded?: VarPath[]; watches?: string[] }) => {
      const f = next.frame ?? frame;
      const e = next.expanded ?? expanded;
      const w = next.watches ?? watches;
      setFrame(f);
      setExpanded(e);
      setWatches(w);
      if (session === null || view?.state.state !== "paused") return Promise.resolve(undefined);
      return guard(async () => {
        setView(await api.debugInspect(session, { frame: f, expand: e, watches: w }));
      });
    },
    [frame, expanded, watches, session, view, guard],
  );

  const toggleBreakpoint = useCallback(
    (line: number) => {
      const set = new Set(lines);
      if (set.has(line)) set.delete(line);
      else set.add(line);
      const next = [...set].sort((a, b) => a - b);
      setBreakpoints((b) => ({ ...b, [docKey]: next }));
      if (session !== null) void api.debugSetBreakpoints(session, next).catch(setError);
    },
    [lines, docKey, session],
  );

  const toggleExpand = useCallback(
    (path: VarPath) => {
      const k = keyOf(path);
      const open = expanded.some((p) => keyOf(p) === k);
      // Closing a value also closes everything inside it.
      const inside = k.slice(0, -1) + ",";
      const next = open
        ? expanded.filter((p) => keyOf(p) !== k && !keyOf(p).startsWith(inside))
        : [...expanded, path];
      return inspect({ expanded: next });
    },
    [expanded, inspect],
  );

  const evaluate = useCallback(
    (expr: string) =>
      guard(async () => {
        if (session === null) return;
        const result = await api.debugEvaluate(session, frame, expr);
        setConsoleLog((log) => [...log, { id: nextId.current++, result }]);
      }),
    [guard, session, frame],
  );

  const stop = useCallback(async () => {
    if (view && view.state.state !== "failed") await api.debugEnd(view.session).catch(() => null);
    setView(null);
    setError(null);
    setExpanded([]);
    setConsoleLog([]);
  }, [view]);

  return {
    view,
    label,
    busy,
    error,
    /** A session the backend still holds (paused or finished). */
    active: session !== null,
    paused: view?.state.state === "paused",
    breakpoints: lines,
    watches,
    expanded,
    isExpanded: (path: VarPath) => expanded.some((p) => keyOf(p) === keyOf(path)),
    frame,
    consoleLog,
    start,
    act,
    toggleBreakpoint,
    toggleExpand,
    selectFrame: (f: number) => inspect({ frame: f, expanded: [] }),
    addWatch: (expr: string) => inspect({ watches: [...watches, expr] }),
    removeWatch: (i: number) => inspect({ watches: watches.filter((_, j) => j !== i) }),
    evaluate,
    stop,
  };
}

export type Debugger = ReturnType<typeof useDebugger>;
