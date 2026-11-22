import { describe, expect, it } from "vitest";
import type { LearnPath, LearnTopicSummary, ProblemSummary } from "@/bindings";
import { buildPathProgress } from "@/practice/home/path";
import { pickWatchNext, thumbFamily } from "@/practice/home/WatchNext";

const topic = (id: string, subject = "s", problems: string[] = []): LearnTopicSummary => ({
  id,
  subject,
  title: id,
  tags: [],
  problems,
});

describe("Watch next (ADR 0041)", () => {
  // A new user on start-here: the first steps have no problems, so the session already
  // offers "Start here: watch t0".
  const ids = ["t0", "t1", "t2", "t3", "t4"];
  const topics = new Map(ids.map((id) => [id, topic(id)]));
  const path: LearnPath = { id: "start-here", title: "Start here", blurb: "", topics: ids };
  const g = buildPathProgress(path, topics, new Map<string, ProblemSummary>(), new Set());
  const catalog = [...topics.values(), topic("x1"), topic("x2")];

  it("always fills three distinct cards, never the session's topic", () => {
    const picks = pickWatchNext({
      path: g,
      review: null,
      patternTopics: [topic("t2"), topic("p1")],
      catalog,
      viewed: new Set(),
      exclude: new Set(["t0"]),
    });
    expect(picks.map((p) => p.topic.id)).toEqual(["t2", "p1", "t1"]);
    expect(new Set(picks.map((p) => p.topic.id)).size).toBe(3);
  });

  it("starts with the path's next topic and the review pick, then fills", () => {
    const picks = pickWatchNext({
      path: g,
      review: { topic: topic("r"), problem: { id: "rp" } as ProblemSummary },
      patternTopics: [],
      catalog,
      viewed: new Set(["t1"]),
    });
    expect(picks.map((p) => [p.kind, p.topic.id])).toEqual([
      ["path", "t0"],
      ["review", "r"],
      ["weak", "t2"],
    ]);
  });

  it("shows fewer only when fewer unwatched topics exist", () => {
    const picks = pickWatchNext({
      path: null,
      review: null,
      patternTopics: [],
      catalog: [topic("a"), topic("b"), topic("c")],
      viewed: new Set(["a", "b"]),
    });
    expect(picks.map((p) => p.topic.id)).toEqual(["c"]);
  });

  it("maps subject icons to thumbnail families", () => {
    expect(thumbFamily("chart-line")).toBe("curve");
    expect(thumbFamily("sigma")).toBe("curve");
    expect(thumbFamily("chart-column")).toBe("bars");
    expect(thumbFamily("graph")).toBe("graph");
    expect(thumbFamily("tree")).toBe("tree");
    expect(thumbFamily("table")).toBe("grid");
    expect(thumbFamily("search")).toBe("strip");
  });
});
