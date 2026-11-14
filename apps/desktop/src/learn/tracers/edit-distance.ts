// Edit distance (Levenshtein). Mirrors content/learn/dynamic-programming/edit-distance/sample.{py,ts}:
// cost[i][j] is the fewest single-letter inserts, deletes and replacements that turn the
// first i letters of a into the first j of b. Row 0 and column 0 are i and j (delete or
// insert everything); equal letters cost nothing (the diagonal); otherwise 1 plus the
// cheapest of replace (diagonal), delete (up) and insert (left). Output: [the distance].
import { decodeText, encodeText, textCodec } from "../engine/codecs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"edit-distance">;
const KEYS = ["edit-distance.ready", "edit-distance.same", "edit-distance.change", "edit-distance.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params: params as Record<string, number> });

export const MARKERS = ["same", "change"] as const;

function trace(input: readonly number[]): Step[] {
  const [a = "", b = ""] = decodeText(input);
  const m = a.length;
  const n = b.length;
  const r = new Recorder([]);
  const cost: (number | null)[][] = Array.from({ length: m + 1 }, (_, i) => Array.from({ length: n + 1 }, (_, j) => (i === 0 ? j : j === 0 ? i : null)));
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: [number, number] | null = null, deps: [number, number][] = []) =>
    r.snap({
      narration,
      marker,
      dp: { label: "edits", colLabels: ["–", ...b], rowLabels: ["–", ...a], cells: cost.map((row) => [...row]), hot, deps },
      vars: { i: hot ? hot[0] : null, j: hot ? hot[1] : null },
    });

  snap(say("edit-distance.ready", { a, b }), null);
  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      r.comparisons++;
      if (a[i - 1] === b[j - 1]) {
        cost[i]![j] = cost[i - 1]![j - 1]!;
        snap(say("edit-distance.same", { letter: a[i - 1]!, value: cost[i]![j]! }), "same", [i, j], [[i - 1, j - 1]]);
      } else {
        const options = [cost[i - 1]![j - 1]!, cost[i - 1]![j]!, cost[i]![j - 1]!];
        cost[i]![j] = 1 + Math.min(...options);
        snap(say("edit-distance.change", { x: a[i - 1]!, y: b[j - 1]!, replace: options[0]!, remove: options[1]!, insert: options[2]!, value: cost[i]![j]! }), "change", [i, j], [
          [i - 1, j - 1],
          [i - 1, j],
          [i, j - 1],
        ]);
      }
    }
  }
  snap(say("edit-distance.done", { a, b, value: cost[m]![n]! }), null);
  r.steps[r.steps.length - 1]!.result = [cost[m]![n]!];
  return r.steps;
}

export const editDistance: TracerDef = {
  id: "edit-distance",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 3, max_len: 17, min: -1, max: 25 },
  input: { kind: "codec", codec: textCodec(["first", "second"], { minLen: 1, maxLen: 8 }) },
  presets: {
    "near-miss": () => encodeText(["carpet", "carrot"]),
    random: (next) => {
      const word = () => Array.from({ length: 3 + Math.floor(next() * 5) }, () => "abcde"[Math.floor(next() * 5)]!).join("");
      return encodeText([word(), word()]);
    },
    "swapped-letters": () => encodeText(["form", "from"]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "comparisons", label: "comparisons" }],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "read", sorted: "filled" },
  vars: ["i", "j"],
  trace,
};
