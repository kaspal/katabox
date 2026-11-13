// The debugger's UI state (ADR 0022). Every action is a replay run in the sandbox, so
// this hook only keeps what the user sees and asks for: breakpoints (per document),
// watches, expanded variables, the selected frame and the console history.
import { useCallback, useEffect, useRef, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import type {
  DebugAction,
  DebugView,
  EvalResult,
  JsonValue,
  Language,
  StopInfo,
} from "@/bindings";
import { api, type CustomInput } from "@/lib/api";

export type ConsoleEntry = { id: number; result: EvalResult };

/** A path into the variables: `[scope, name, i, j, …]`. */
export type VarPath = JsonValue[];
const keyOf = (p: VarPath) => JSON.stringify(p);
const NO_LINES: number[] = [];
const sameNames = (a: string[] | null, b: string[] | null) => JSON.stringify(a) === JSON.stringify(b);

function stopOf(v: DebugView | null): StopInfo | null {
  return v?.state.state === "paused" ? v.state.stop : null;
}

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
  // The visualizer's variables (ADR 0027): sent with every stop while its view is open.
  const [visualize, setVisualizeState] = useState<string[] | null>(null);
  // The stop before the last step action, for "changed since the previous step".
  const [previous, setPrevious] = useState<StopInfo | null>(null);
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
        setPrevious(null);
        setLabel(testLabel);
        const v = await api.debugStart({
          problem,
          language,
          code,
          input: input as { [key in string]: JsonValue },
          breakpoints: lines,
          watches,
          visualize,
        });
        setView(v);
      }),
    [guard, language, problem, lines, watches, view, visualize],
  );

  const act = useCallback(
    (action: DebugAction) =>
      guard(async () => {
        if (session === null) return;
        setFrame(0);
        const before = stopOf(view);
        const next = await api.debugAction(session, action);
        setPrevious(before);
        setView(next);
      }),
    [guard, session, view],
  );

  const inspect = useCallback(
    (next: { frame?: number; expanded?: VarPath[]; watches?: string[]; visualize?: string[] | null }) => {
      const f = next.frame ?? frame;
      const e = next.expanded ?? expanded;
      const w = next.watches ?? watches;
      const z = next.visualize !== undefined ? next.visualize : visualize;
      setFrame(f);
      setExpanded(e);
      setWatches(w);
      setVisualizeState(z);
      if (session === null || view?.state.state !== "paused") return Promise.resolve(undefined);
      return guard(async () => {
        setView(await api.debugInspect(session, { frame: f, expand: e, watches: w, visualize: z }));
      });
    },
    [frame, expanded, watches, visualize, session, view, guard],
  );

  /** Open (names) or close (null) the visualizer: the current stop is shown again with
   * (or without) its snapshot; later steps keep it. `force` asks again with the same
   * names (a stop that came back without a snapshot). */
  const setVisualize = useCallback(
    (names: string[] | null, force = false) => {
      if (!force && sameNames(names, visualize)) return Promise.resolve(undefined);
      return inspect({ visualize: names });
    },
    [visualize, inspect],
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
    setPrevious(null);
  }, [view]);

  // The workspace page is reused when another problem opens: a session belongs to the
  // problem it was started on, so it ends there (the editor would stay read-only).
  const stopRef = useRef(stop);
  stopRef.current = stop;
  const sessionProblem = useRef(problem);
  useEffect(() => {
    if (sessionProblem.current === problem) return;
    sessionProblem.current = problem;
    void stopRef.current();
  }, [problem]);

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
    visualize,
    setVisualize,
    /** The stop before the last step action (null after start or an inspect-only change). */
    previous,
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
