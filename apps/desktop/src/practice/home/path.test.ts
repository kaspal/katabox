import { describe, expect, it } from "vitest";
import type { LearnPath, LearnTopicSummary, ProblemSummary } from "@/bindings";
import { buildPathProgress, choosePath, stepDone } from "@/practice/home/path";

const topic = (id: string, problems: string[] = []): LearnTopicSummary => ({ id, subject: "s", title: id, tags: [], problems });
const prob = (id: string, solved = false): ProblemSummary => ({
  id,
  title: id,
  difficulty: "easy",
  pattern: "x",
  rank: 1,
  mode: "function",
  tags: [],
  status: solved ? "solved" : null,
  fix: false,
  pack: null,
});

function world(n: number, links: Record<string, string[]> = {}, solved: string[] = []) {
  const ids = Array.from({ length: n }, (_, i) => `t${i}`);
  const topics = new Map(ids.map((id) => [id, topic(id, links[id] ?? [`p-${id}`])]));
  const problems = new Map<string, ProblemSummary>();
  for (const t of topics.values()) for (const p of t.problems) problems.set(p, prob(p, solved.includes(p)));
  const path: LearnPath = { id: "start-here", title: "Start here", blurb: "", topics: ids };
  return { path, topics, problems };
}

describe("path progress", () => {
  it("needs viewed and one linked problem solved; viewed alone without links", () => {
    expect(stepDone(true, 1, 3)).toBe(true);
    expect(stepDone(true, 0, 3)).toBe(false);
    expect(stepDone(false, 3, 3)).toBe(false);
    expect(stepDone(true, 0, 0)).toBe(true);
  });

  it("marks done steps, the first open one as current, and the next problem and topic", () => {
    const w = world(4, { t2: [] }, ["p-t0", "p-t1"]);
    const g = buildPathProgress(w.path, w.topics, w.problems, new Set(["t0", "t2"]));
    expect(g.steps.map((s) => s.state)).toEqual(["done", "current", "done", "todo"]);
    expect(g.current).toBe(1);
    expect(g.done).toBe(2);
    expect(g.steps[1]).toMatchObject({ viewed: false, solved: 1, total: 1 });
    // t1's only problem is solved, so it needs the topic viewed: no next problem.
    expect(g.nextProblem).toBeNull();
    expect(g.nextTopic?.id).toBe("t1");
  });

  it("gives the first unsolved linked problem of the current step", () => {
    const w = world(2, { t0: ["a", "b", "c"] }, ["a"]);
    const g = buildPathProgress(w.path, w.topics, w.problems, new Set(["t0"]));
    expect(g.current).toBe(1);
    const h = buildPathProgress(w.path, w.topics, w.problems, new Set());
    expect(h.current).toBe(0);
    expect(h.nextProblem?.id).toBe("b");
    expect(h.nextTopic?.id).toBe("t0");
  });

  it("shows a window of 7: two before, current, four after, and how many more", () => {
    const solved = Array.from({ length: 5 }, (_, i) => `p-t${i}`);
    const w = world(15, {}, solved);
    const viewed = new Set(solved.map((p) => p.slice(2)));
    const g = buildPathProgress(w.path, w.topics, w.problems, viewed);
    expect(g.current).toBe(5);
    expect(g.window.start).toBe(3);
    expect(g.window.steps.map((s) => s.topic.id)).toEqual(["t3", "t4", "t5", "t6", "t7", "t8", "t9"]);
    expect(g.window.moreAfter).toBe(5);
  });

  it("starts the window at the first step for a new user", () => {
    const w = world(10);
    const g = buildPathProgress(w.path, w.topics, w.problems, new Set());
    expect(g.window.steps.map((s) => s.topic.id)).toEqual(["t0", "t1", "t2", "t3", "t4"]);
    expect(g.window.moreAfter).toBe(5);
    expect(g.steps[0].state).toBe("current");
  });

  it("handles a finished path", () => {
    const w = world(3, { t0: [], t1: [], t2: [] });
    const g = buildPathProgress(w.path, w.topics, w.problems, new Set(["t0", "t1", "t2"]));
    expect(g.current).toBe(3);
    expect(g.done).toBe(3);
    expect(g.nextProblem).toBeNull();
    expect(g.nextTopic).toBeNull();
    expect(g.window.steps).toHaveLength(3);
  });

  it("skips topics missing from the catalogue", () => {
    const w = world(2);
    const g = buildPathProgress({ ...w.path, topics: ["t0", "gone", "t1"] }, w.topics, w.problems, new Set());
    expect(g.steps.map((s) => s.topic.id)).toEqual(["t0", "t1"]);
  });

  it("chooses the pref, then the first path with progress, then start-here", () => {
    const w = world(4, {}, ["p-t2"]);
    const paths: LearnPath[] = [
      { id: "a", title: "A", blurb: "", topics: ["t0", "t1"] },
      { id: "start-here", title: "S", blurb: "", topics: ["t0"] },
      { id: "b", title: "B", blurb: "", topics: ["t2", "t3"] },
    ];
    expect(choosePath(paths, "b", w.topics, w.problems, new Set())?.id).toBe("b");
    expect(choosePath(paths, null, w.topics, w.problems, new Set())?.id).toBe("start-here");
    expect(choosePath(paths, "unknown", w.topics, w.problems, new Set(["t2"]))?.id).toBe("b");
    expect(choosePath([], null, w.topics, w.problems, new Set())).toBeNull();
  });
});
