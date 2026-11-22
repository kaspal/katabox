import { describe, expect, it } from "vitest";
import type { ProblemSummary } from "@/bindings";
import { acceptedOn, buildSession, roundToFive, type SessionInput } from "@/practice/home/session";
import { weekDays } from "@/practice/home/week";

function p(id: string, difficulty: ProblemSummary["difficulty"] = "easy", solved = false): ProblemSummary {
  return {
    id,
    title: id.toUpperCase(),
    difficulty,
    pattern: "arrays-hashing",
    rank: 1,
    mode: "function",
    tags: [],
    status: solved ? "solved" : null,
    fix: false,
    pack: null,
  };
}

const base: SessionInput = {
  continueItem: null,
  continueProblem: null,
  daily: null,
  dailyOn: true,
  kata: null,
  pathNext: null,
  solvedToday: new Set(),
  brandNew: false,
};
const draft = (id: string) => ({ problem_id: id, title: id.toUpperCase(), language: "python" as const, updated_at: 0 });

describe("buildSession", () => {
  it("orders continue, daily, kata, then the path, at most 3 items", () => {
    const s = buildSession({
      ...base,
      continueItem: draft("a"),
      continueProblem: p("a", "medium"),
      daily: { problem: p("b", "medium"), solved: false },
      kata: { doneToday: false },
      pathNext: p("c"),
    });
    expect(s.items.map((i) => i.kind)).toEqual(["continue", "daily", "kata"]);
    expect(s.items.map((i) => i.minutes)).toEqual([5, 20, 10]);
    expect(s.totalMinutes).toBe(35);
  });

  it("fills with the path's next problem when there is room", () => {
    const s = buildSession({ ...base, daily: { problem: p("b", "hard"), solved: false }, pathNext: p("c") });
    expect(s.items.map((i) => [i.kind, i.problemId, i.onPath])).toEqual([
      ["daily", "b", false],
      ["path", "c", true],
    ]);
    expect(s.totalMinutes).toBe(40);
  });

  it("merges the daily with the path's next problem and badges it", () => {
    const s = buildSession({ ...base, daily: { problem: p("c"), solved: false }, pathNext: p("c") });
    expect(s.items).toHaveLength(1);
    expect(s.items[0]).toMatchObject({ kind: "daily", problemId: "c", onPath: true });
  });

  it("skips the daily when it is off", () => {
    const s = buildSession({ ...base, dailyOn: false, daily: { problem: p("b"), solved: false }, pathNext: p("c") });
    expect(s.items.map((i) => i.kind)).toEqual(["path"]);
  });

  it("drops a draft that is already solved, keeps one solved today as done", () => {
    const old = buildSession({ ...base, continueItem: draft("a"), continueProblem: p("a", "easy", true) });
    expect(old.items).toEqual([]);
    const today = buildSession({
      ...base,
      continueItem: draft("a"),
      continueProblem: p("a", "easy", true),
      solvedToday: new Set(["a"]),
      pathNext: p("c"),
    });
    expect(today.items[0]).toMatchObject({ kind: "continue", doneToday: true });
    expect(today.firstOpen?.kind).toBe("path");
    expect(today.done).toBe(1);
  });

  it("does not repeat a draft that is the daily or the path problem", () => {
    const s = buildSession({
      ...base,
      continueItem: draft("b"),
      continueProblem: p("b"),
      daily: { problem: p("b"), solved: false },
    });
    expect(s.items.map((i) => i.kind)).toEqual(["daily"]);
  });

  it("marks the daily and the kata done today", () => {
    const s = buildSession({ ...base, daily: { problem: p("b"), solved: true }, kata: { doneToday: true } });
    expect(s.items.every((i) => i.doneToday)).toBe(true);
    expect(s.firstOpen).toBeNull();
    expect(s.done).toBe(2);
  });

  it("starts a brand-new user on the path's first problem", () => {
    const s = buildSession({ ...base, brandNew: true, daily: { problem: p("c"), solved: false }, pathNext: p("c") });
    expect(s.items).toHaveLength(1);
    expect(s.items[0]).toMatchObject({ kind: "start", problemId: "c", onPath: true });
    expect(s.firstOpen?.route).toEqual({ to: "/problems/$problemId", params: { problemId: "c" } });
  });

  it("is empty with nothing to do", () => {
    const s = buildSession(base);
    expect(s).toEqual({ items: [], totalMinutes: 0, done: 0, firstOpen: null });
  });

  it("rounds to the nearest 5", () => {
    expect([12, 13, 17, 5, 0].map(roundToFive)).toEqual([10, 15, 15, 5, 0]);
  });
});

describe("activity helpers", () => {
  const at = (d: string, h = 10) => new Date(`${d}T${String(h).padStart(2, "0")}:00:00`).getTime();

  it("finds problems accepted today", () => {
    const rows = [
      { at: at("2026-11-22"), problem_id: "a", accepted: true },
      { at: at("2026-11-22"), problem_id: "b", accepted: false },
      { at: at("2026-11-21"), problem_id: "c", accepted: true },
    ];
    expect([...acceptedOn(rows, "2026-11-22")]).toEqual(["a"]);
  });

  it("lists Monday to Sunday of this week", () => {
    // 2026-11-22 is a Sunday.
    const days = weekDays([{ at: at("2026-11-17"), problem_id: "a", accepted: true }], "2026-11-19");
    expect(days.map((d) => d.day)).toEqual([
      "2026-11-16",
      "2026-11-17",
      "2026-11-18",
      "2026-11-19",
      "2026-11-20",
      "2026-11-21",
      "2026-11-22",
    ]);
    expect(days[1]).toMatchObject({ submissions: 1, solved: 1 });
    expect(days[3].today).toBe(true);
    expect(days[4].future).toBe(true);
  });
});
