// Merging overlapping intervals. Mirrors content/learn/greedy/merge-intervals/sample.{py,ts}:
// sort by start (then end); an interval that starts no later than the last merged one
// ends joins it (its end grows if needed); otherwise it opens a new merged interval.
// Touching intervals (3–5 and 5–7) merge. Output: the merged intervals, flat.
import { intervalsCodec } from "../engine/shapes";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"merge-intervals">;
const KEYS = ["merge-intervals.ready", "merge-intervals.sorted", "merge-intervals.open", "merge-intervals.extend", "merge-intervals.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["extend", "open"] as const;
export const MAX_TIME = 24;

function trace(input: readonly number[]): Step[] {
  const iv = Array.from({ length: input.length / 2 }, (_, k) => ({ id: k, start: input[2 * k]!, end: input[2 * k + 1]! }));
  const order = [...iv].sort((a, b) => a.start - b.start || a.end - b.end);
  const r = new Recorder([]);
  const roles: Record<number, Role> = {};
  const merged: { start: number; end: number }[] = [];
  let sortedYet = false;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, cur: number | null = null) => {
    const bars = (sortedYet ? order : iv).map((b) => ({ ...b, role: b.id === cur ? ("key" as Role) : roles[b.id] }));
    r.snap({
      narration,
      marker,
      timeline: {
        min: 0,
        max: Math.max(...iv.map((b) => b.end)),
        bars,
        mark: merged.length ? merged[merged.length - 1]!.end : null,
        output: merged.map((m, k) => ({ ...m, role: k === merged.length - 1 && cur !== null ? ("swap" as Role) : ("sorted" as Role) })),
        outputLabel: "merged",
      },
      vars: { merged: merged.length },
    });
  };

  snap(say("merge-intervals.ready", { n: iv.length }), null);
  sortedYet = true;
  snap(say("merge-intervals.sorted"), null);
  for (const b of order) {
    r.comparisons++;
    const last = merged[merged.length - 1];
    if (last && b.start <= last.end) {
      last.end = Math.max(last.end, b.end);
      roles[b.id] = "sorted";
      r.writes++;
      snap(say("merge-intervals.extend", { start: b.start, end: b.end, to: last.end }), "extend", b.id);
    } else {
      merged.push({ start: b.start, end: b.end });
      roles[b.id] = "sorted";
      r.moves++;
      snap(say("merge-intervals.open", { start: b.start, end: b.end }), "open", b.id);
    }
  }
  snap(say("merge-intervals.done", { n: iv.length, count: merged.length }), null);
  r.steps[r.steps.length - 1]!.result = merged.flatMap((m) => [m.start, m.end]);
  return r.steps;
}

const enc = (pairs: [number, number][]) => pairs.flat();

export const mergeIntervals: TracerDef = {
  id: "merge-intervals",
  subject: "greedy",
  scene: "timeline",
  limits: { min_len: 2, max_len: 20, min: 0, max: MAX_TIME },
  input: { kind: "codec", codec: intervalsCodec({ min: 0, max: MAX_TIME, count: [1, 10] }) },
  presets: {
    bookings: () => enc([[7, 9], [1, 4], [3, 5], [14, 17], [8, 11], [5, 6]]),
    random: (next) => {
      const n = 4 + Math.floor(next() * 7);
      return enc(
        Array.from({ length: n }, () => {
          const s = Math.floor(next() * 20);
          return [s, Math.min(MAX_TIME, s + 1 + Math.floor(next() * 4))] as [number, number];
        }),
      );
    },
    "nested-and-touching": () => enc([[1, 10], [2, 3], [4, 5], [10, 12], [13, 14]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "merged_in" },
    { slot: "moves", label: "opened" },
  ],
  legend: ["key", "sorted", "swap"],
  roleLabels: { key: "current", sorted: "placed", swap: "growing" },
  vars: ["merged"],
  trace,
};
