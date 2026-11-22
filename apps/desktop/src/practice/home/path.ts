// Home → "Your path" (ADR 0041): progress along one Learn path, one step per topic.
// The rules are pure (path.test.ts); `useHomePath` only gathers the data.
import { useMemo } from "react";
import type { LearnPath, LearnTopicSummary, ProblemSummary } from "@/bindings";
import { useViewed } from "@/learn/engine/progress";
import { usePracticePrefs } from "@/lib/practice";
import { useLearnCatalog, useLearnPaths, useProblems } from "@/lib/queries";

export type StepState = "done" | "current" | "todo";

export type PathStep = {
  topic: LearnTopicSummary;
  viewed: boolean;
  /** Linked problems solved, out of `total` linked problems in the catalogue. */
  solved: number;
  total: number;
  state: StepState;
};

export type PathProgress = {
  path: LearnPath;
  steps: PathStep[];
  /** Index of the current step; equal to `steps.length` when every step is done. */
  current: number;
  done: number;
  /** The visible window: 2 steps before current, current, the next 4. */
  window: { steps: PathStep[]; start: number; moreAfter: number };
  /** The first unsolved linked problem of the current step. */
  nextProblem: ProblemSummary | null;
  /** The current step's topic, when it is not viewed yet. */
  nextTopic: LearnTopicSummary | null;
  /** The first unsolved linked problem from the current step on (a step may have none):
   * the session's path item and a new user's "Start here". */
  upcomingProblem: ProblemSummary | null;
};

export const WINDOW_BEFORE = 2;
export const WINDOW_AFTER = 4;
export const DEFAULT_PATH = "start-here";

/** Done: viewed and at least one linked problem solved (viewed is enough without links). */
export function stepDone(viewed: boolean, solved: number, total: number): boolean {
  return viewed && (total === 0 || solved > 0);
}

export function buildPathProgress(
  path: LearnPath,
  topics: Map<string, LearnTopicSummary>,
  problems: Map<string, ProblemSummary>,
  viewed: ReadonlySet<string>,
): PathProgress {
  const base = path.topics.flatMap((id) => {
    const topic = topics.get(id);
    if (!topic) return [];
    const linked = topic.problems.flatMap((p) => {
      const s = problems.get(p);
      return s ? [s] : [];
    });
    const solved = linked.filter((p) => p.status === "solved").length;
    const seen = viewed.has(id);
    return [{ topic, viewed: seen, solved, total: linked.length, linked, done: stepDone(seen, solved, linked.length) }];
  });
  const firstOpen = base.findIndex((s) => !s.done);
  const current = firstOpen === -1 ? base.length : firstOpen;
  const steps: PathStep[] = base.map((s, i) => ({
    topic: s.topic,
    viewed: s.viewed,
    solved: s.solved,
    total: s.total,
    state: s.done ? "done" : i === current ? "current" : "todo",
  }));
  const anchor = Math.min(current, Math.max(0, steps.length - 1));
  const start = Math.max(0, anchor - WINDOW_BEFORE);
  const end = Math.min(steps.length, anchor + WINDOW_AFTER + 1);
  const cur = base[current];
  return {
    path,
    steps,
    current,
    done: steps.filter((s) => s.state === "done").length,
    window: { steps: steps.slice(start, end), start, moreAfter: steps.length - end },
    nextProblem: cur?.linked.find((p) => p.status !== "solved") ?? null,
    nextTopic: cur && !cur.viewed ? cur.topic : null,
    upcomingProblem:
      base
        .slice(current)
        .flatMap((s) => s.linked)
        .find((p) => p.status !== "solved") ?? null,
  };
}

/** The pref when it names a path; else the first path with any progress; else start-here. */
export function choosePath(
  paths: LearnPath[],
  pref: string | null,
  topics: Map<string, LearnTopicSummary>,
  problems: Map<string, ProblemSummary>,
  viewed: ReadonlySet<string>,
): LearnPath | null {
  const byPref = pref ? paths.find((p) => p.id === pref) : undefined;
  if (byPref) return byPref;
  const started = paths.find((p) => buildPathProgress(p, topics, problems, viewed).done > 0);
  return started ?? paths.find((p) => p.id === DEFAULT_PATH) ?? paths[0] ?? null;
}

export function useHomePath() {
  const paths = useLearnPaths();
  const catalog = useLearnCatalog();
  const problems = useProblems();
  const { viewed } = useViewed();
  const { homePath, setHomePath } = usePracticePrefs();
  const progress = useMemo(() => {
    if (!paths.data || !catalog.data || !problems.data) return undefined;
    const topics = new Map(catalog.data.flatMap((s) => s.topics).map((t) => [t.id, t]));
    const byId = new Map(problems.data.map((p) => [p.id, p]));
    const seen = new Set(viewed);
    const path = choosePath(paths.data, homePath, topics, byId, seen);
    return path ? buildPathProgress(path, topics, byId, seen) : null;
  }, [paths.data, catalog.data, problems.data, viewed, homePath]);
  const error = paths.error ?? catalog.error ?? problems.error;
  return {
    progress,
    paths: paths.data ?? [],
    isPending: progress === undefined && !error,
    error,
    refetch: () => {
      void paths.refetch();
      void catalog.refetch();
      void problems.refetch();
    },
    setHomePath,
  };
}
