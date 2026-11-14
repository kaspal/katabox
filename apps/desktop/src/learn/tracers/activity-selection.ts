// Activity selection. Mirrors content/learn/greedy/activity-selection/sample.{py,ts}:
// sort the activities by end time (ties: start, then input order) and take each one
// that starts no earlier than the last taken one ends. Ending first leaves the most
// room for the rest. Output: how many, then their input positions in increasing order.
import { intervalsCodec } from "../engine/shapes";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"activity-selection">;
const KEYS = ["activity-selection.ready", "activity-selection.sorted", "activity-selection.take", "activity-selection.clash", "activity-selection.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["take", "clash"] as const;
export const MAX_TIME = 24;

function trace(input: readonly number[]): Step[] {
  const iv = Array.from({ length: input.length / 2 }, (_, k) => ({ id: k, start: input[2 * k]!, end: input[2 * k + 1]! }));
  const order = [...iv].sort((a, b) => a.end - b.end || a.start - b.start || a.id - b.id);
  const r = new Recorder([]);
  const roles: Record<number, Role> = {};
  let freeAt: number | null = null;
  let sortedYet = false;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, cur: number | null = null) => {
    const bars = (sortedYet ? order : iv).map((b) => ({ ...b, role: b.id === cur ? ("key" as Role) : roles[b.id] }));
    r.snap({
      narration,
      marker,
      timeline: { min: 0, max: Math.max(...iv.map((b) => b.end)), bars, mark: freeAt, output: iv.filter((b) => roles[b.id] === "sorted").map((b) => ({ start: b.start, end: b.end })), outputLabel: "chosen" },
      vars: { free_at: freeAt, taken: Object.values(roles).filter((x) => x === "sorted").length },
    });
  };

  snap(say("activity-selection.ready", { n: iv.length }), null);
  sortedYet = true;
  snap(say("activity-selection.sorted"), null);
  for (const b of order) {
    r.comparisons++;
    if (freeAt === null || b.start >= freeAt) {
      roles[b.id] = "sorted";
      freeAt = b.end;
      r.writes++;
      snap(say("activity-selection.take", { k: b.id, start: b.start, end: b.end }), "take", b.id);
    } else {
      roles[b.id] = "out";
      snap(say("activity-selection.clash", { k: b.id, start: b.start, free: freeAt }), "clash", b.id);
    }
  }
  const chosen = iv.filter((b) => roles[b.id] === "sorted").map((b) => b.id);
  snap(say("activity-selection.done", { num: chosen.length, n: iv.length }), null);
  r.steps[r.steps.length - 1]!.result = [chosen.length, ...chosen];
  return r.steps;
}

const enc = (pairs: [number, number][]) => pairs.flat();

export const activitySelection: TracerDef = {
  id: "activity-selection",
  subject: "greedy",
  scene: "timeline",
  limits: { min_len: 2, max_len: 20, min: 0, max: MAX_TIME },
  input: { kind: "codec", codec: intervalsCodec({ min: 0, max: MAX_TIME, count: [1, 10] }) },
  presets: {
    "long-meeting": () => enc([[1, 4], [3, 5], [0, 10], [5, 7], [4, 6], [6, 9], [8, 11], [9, 12]]),
    random: (next) => {
      const n = 4 + Math.floor(next() * 7);
      return enc(
        Array.from({ length: n }, () => {
          const s = Math.floor(next() * 20);
          return [s, Math.min(MAX_TIME, s + 1 + Math.floor(next() * 5))] as [number, number];
        }),
      );
    },
    "back-to-back": () => enc([[0, 2], [2, 4], [4, 6], [1, 3], [3, 5]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "taken" },
  ],
  legend: ["key", "sorted", "out"],
  roleLabels: { key: "current", sorted: "taken", out: "skipped" },
  vars: ["free_at", "taken"],
  trace,
};
