// Home → "Today's session" (ADR 0041): 1-3 short items picked from what already exists
// (the latest draft, the daily problem, the daily kata, the path's next problem). Pure
// rules in `buildSession` (session.test.ts); `useSession` gathers the data.
import { useMemo } from "react";
import type { ContinueItem, Difficulty, ProblemSummary } from "@/bindings";
import { useKataOptions } from "@/lib/kata";
import { useDailyProblem, usePracticePrefs } from "@/lib/practice";
import { useActivity } from "@/lib/progress";
import { useContinue, useProblems } from "@/lib/queries";
import { localToday } from "@/lib/today";
import type { PathProgress } from "@/practice/home/path";

export type SessionKind = "continue" | "daily" | "kata" | "path" | "start";

export type SessionRoute = { to: "/problems/$problemId"; params: { problemId: string } } | { to: "/kata" };

export type SessionItem = {
  kind: SessionKind;
  /** The problem's title (null for the kata, whose title is a translation). */
  title: string | null;
  problemId: string | null;
  difficulty: Difficulty | null;
  pattern: string | null;
  route: SessionRoute;
  minutes: number;
  doneToday: boolean;
  /** The item is also the path's next problem ("On your path"). */
  onPath: boolean;
};

export type Session = {
  items: SessionItem[];
  /** The sum of the items' minutes, rounded to the nearest 5. */
  totalMinutes: number;
  done: number;
  /** The first item that is not done ("Start session"), or null when all are done. */
  firstOpen: SessionItem | null;
};

export type SessionInput = {
  continueItem: ContinueItem | null;
  /** The continued problem's summary (status, difficulty), when it is in the catalogue. */
  continueProblem: ProblemSummary | null;
  daily: { problem: ProblemSummary; solved: boolean } | null;
  dailyOn: boolean;
  /** null: katas are off, or no kata is available today. */
  kata: { doneToday: boolean } | null;
  pathNext: ProblemSummary | null;
  /** Problems Accepted today (local day). */
  solvedToday: ReadonlySet<string>;
  /** No problem solved yet: the session starts with the path's first problem. */
  brandNew: boolean;
};

export const MAX_ITEMS = 3;
export const MINUTES: Record<Difficulty, number> = { easy: 10, medium: 20, hard: 30 };
export const CONTINUE_MINUTES = 5;
export const KATA_MINUTES = 10;

const problemRoute = (problemId: string): SessionRoute => ({ to: "/problems/$problemId", params: { problemId } });

function problemItem(kind: SessionKind, p: ProblemSummary, doneToday: boolean, onPath: boolean): SessionItem {
  return {
    kind,
    title: p.title,
    problemId: p.id,
    difficulty: p.difficulty,
    pattern: p.pattern,
    route: problemRoute(p.id),
    minutes: MINUTES[p.difficulty],
    doneToday,
    onPath,
  };
}

export function roundToFive(minutes: number): number {
  return Math.round(minutes / 5) * 5;
}

export function buildSession(input: SessionInput): Session {
  const items: SessionItem[] = [];
  const pathId = input.pathNext?.id ?? null;
  const dailyId = input.dailyOn && input.daily ? input.daily.problem.id : null;

  if (input.brandNew && input.pathNext) {
    // A brand-new user: "Start here" is the path's first problem.
    items.push(problemItem("start", input.pathNext, input.solvedToday.has(input.pathNext.id), true));
  }

  // 1. Continue: the latest draft, unless it is solved (from before today) or is the same
  // problem as the daily or the path item (which then carries it).
  const c = input.continueItem;
  if (c && c.problem_id !== dailyId && c.problem_id !== pathId) {
    const solvedToday = input.solvedToday.has(c.problem_id);
    const solved = input.continueProblem?.status === "solved";
    if (!solved || solvedToday) {
      items.push({
        kind: "continue",
        title: c.title,
        problemId: c.problem_id,
        difficulty: input.continueProblem?.difficulty ?? null,
        pattern: input.continueProblem?.pattern ?? null,
        route: problemRoute(c.problem_id),
        minutes: CONTINUE_MINUTES,
        doneToday: solvedToday,
        onPath: false,
      });
    }
  }

  // 2. Daily (the existing pick); merged with the path's next problem when they match.
  if (input.dailyOn && input.daily && !items.some((i) => i.problemId === input.daily?.problem.id)) {
    items.push(problemItem("daily", input.daily.problem, input.daily.solved, input.daily.problem.id === pathId));
  }

  // 3. The daily kata.
  if (input.kata && items.length < MAX_ITEMS) {
    items.push({
      kind: "kata",
      title: null,
      problemId: null,
      difficulty: null,
      pattern: null,
      route: { to: "/kata" },
      minutes: KATA_MINUTES,
      doneToday: input.kata.doneToday,
      onPath: false,
    });
  }

  // 4. The path's next problem, when there is room and it is not already in.
  if (input.pathNext && items.length < MAX_ITEMS && !items.some((i) => i.problemId === pathId)) {
    items.push(problemItem("path", input.pathNext, input.solvedToday.has(input.pathNext.id), true));
  }

  const list = items.slice(0, MAX_ITEMS);
  return {
    items: list,
    totalMinutes: roundToFive(list.reduce((sum, i) => sum + i.minutes, 0)),
    done: list.filter((i) => i.doneToday).length,
    firstOpen: list.find((i) => !i.doneToday) ?? null,
  };
}

/** Problems Accepted on the local day `today`, from the activity rows. */
export function acceptedOn(rows: { at: number; problem_id: string; accepted: boolean }[], today: string): Set<string> {
  return new Set(rows.filter((r) => r.accepted && localToday(new Date(r.at)) === today).map((r) => r.problem_id));
}

export function useSession(path: PathProgress | null | undefined) {
  const prefs = usePracticePrefs();
  const cont = useContinue();
  const daily = useDailyProblem(prefs.daily);
  const kata = useKataOptions();
  const activity = useActivity();
  const problems = useProblems();
  const queries = [cont, problems, activity, ...(prefs.daily ? [daily] : []), ...(prefs.rusty ? [kata] : [])];
  const pending = queries.some((q) => q.isPending) || path === undefined;
  const error = queries.find((q) => q.error)?.error ?? null;
  const session = useMemo(() => {
    if (pending || error) return undefined;
    const today = localToday();
    const byId = new Map((problems.data ?? []).map((p) => [p.id, p]));
    const k = kata.data;
    const kataSession = k?.daily_session;
    return buildSession({
      continueItem: cont.data ?? null,
      continueProblem: cont.data ? (byId.get(cont.data.problem_id) ?? null) : null,
      daily: daily.data ? { problem: daily.data.problem, solved: daily.data.solved } : null,
      dailyOn: prefs.daily,
      kata:
        prefs.rusty && k && (k.daily.length > 0 || kataSession)
          ? {
              doneToday: !!kataSession && kataSession.ended_at !== null && kataSession.solved > 0,
            }
          : null,
      pathNext: path?.upcomingProblem ?? null,
      solvedToday: acceptedOn(activity.data ?? [], today),
      brandNew: !(problems.data ?? []).some((p) => p.status === "solved"),
    });
  }, [pending, error, problems.data, kata.data, cont.data, daily.data, prefs.daily, prefs.rusty, activity.data, path]);
  return {
    session,
    isPending: pending && !error,
    error,
    refetch: () => queries.forEach((q) => void q.refetch()),
  };
}
