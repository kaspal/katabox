// Longest common subsequence. Mirrors content/learn/dynamic-programming/lcs/sample.{py,ts}:
// best[i][j] is the LCS length of the first i letters of a and the first j of b; equal
// letters extend the diagonal by one, otherwise carry the better of up and left. The
// traceback walks from the corner: diagonal on a match, else up when up is at least as
// good, else left. Output: the length, then the letters (0-25).
import { decodeText, encodeText, textCodec } from "../engine/codecs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"lcs">;
const KEYS = ["lcs.ready", "lcs.match", "lcs.carry", "lcs.trace", "lcs.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams) => ({ key, params: params as NarrationParams<number> });

export const MARKERS = ["match", "carry", "trace"] as const;

function trace(input: readonly number[]): Step[] {
  const [a = "", b = ""] = decodeText(input);
  const m = a.length;
  const n = b.length;
  const r = new Recorder([]);
  const best: (number | null)[][] = Array.from({ length: m + 1 }, (_, i) => Array.from({ length: n + 1 }, (_, j) => (i === 0 || j === 0 ? 0 : null)));
  const path: [number, number][] = [];
  const out: string[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: [number, number] | null = null, deps: [number, number][] = []) =>
    r.snap({
      narration,
      marker,
      dp: { label: "lcs_length", colLabels: ["–", ...b], rowLabels: ["–", ...a], cells: best.map((row) => [...row]), hot, deps, path: [...path] },
      vars: { i: hot ? hot[0] : null, j: hot ? hot[1] : null, found: out.length },
    });

  snap(say("lcs.ready", { a, b }), null);
  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      r.comparisons++;
      if (a[i - 1] === b[j - 1]) {
        best[i]![j] = best[i - 1]![j - 1]! + 1;
        snap(say("lcs.match", { letter: a[i - 1]!, value: best[i]![j]! }), "match", [i, j], [[i - 1, j - 1]]);
      } else {
        best[i]![j] = Math.max(best[i - 1]![j]!, best[i]![j - 1]!);
        snap(say("lcs.carry", { x: a[i - 1]!, y: b[j - 1]!, value: best[i]![j]! }), "carry", [i, j], [
          [i - 1, j],
          [i, j - 1],
        ]);
      }
    }
  }
  let i = m;
  let j = n;
  path.push([i, j]);
  while (i > 0 && j > 0) {
    if (a[i - 1] === b[j - 1]) {
      out.push(a[i - 1]!);
      snap(say("lcs.trace", { letter: a[i - 1]! }), "trace", [i, j]);
      i--;
      j--;
    } else if (best[i - 1]![j]! >= best[i]![j - 1]!) i--;
    else j--;
    path.push([i, j]);
  }
  out.reverse();
  snap(say("lcs.done", { length: out.length, word: out.join("") || "—" }), null);
  r.steps[r.steps.length - 1]!.result = [out.length, ...out.map((c) => c.charCodeAt(0) - 97)];
  return r.steps;
}

export const lcs: TracerDef = {
  id: "lcs",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 3, max_len: 17, min: -1, max: 25 },
  input: { kind: "codec", codec: textCodec(["first", "second"], { minLen: 1, maxLen: 8 }) },
  presets: {
    "shared-letters": () => encodeText(["planet", "anklet"]),
    random: (next) => {
      const word = () => Array.from({ length: 3 + Math.floor(next() * 5) }, () => "abcde"[Math.floor(next() * 5)]!).join("");
      return encodeText([word(), word()]);
    },
    "nothing-shared": () => encodeText(["moon", "sky"]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "comparisons", label: "comparisons" }],
  legend: ["key", "compare", "sorted", "swap"],
  roleLabels: { key: "current", compare: "read", sorted: "filled", swap: "taken" },
  vars: ["i", "j", "found"],
  trace,
};
