// Z-function search. Mirrors content/learn/strings/z-function/sample.{py,ts}: on s =
// pattern | text, z[i] is how far s[i…] agrees with the start of s. A box [left, right)
// remembers the farthest match seen; inside it, z[i] starts from the mirrored value
// (reuse) and only grows past `right` by direct comparison (extend). Wherever z equals the
// pattern's length, the pattern occurs. Output: how many, then their positions.
import { decodeText, encodeText, textCodec } from "../engine/codecs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"z-function">;
const KEYS = ["z-function.ready", "z-function.reuse", "z-function.extend", "z-function.box", "z-function.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params: params as Record<string, number> });

export const MARKERS = ["reuse", "extend"] as const;

function trace(input: readonly number[]): Step[] {
  const [text = "", pattern = ""] = decodeText(input);
  const s = `${pattern}|${text}`;
  const n = s.length;
  const m = pattern.length;
  const r = new Recorder([]);
  const z: (number | null)[] = new Array(n).fill(null);
  let left = 0;
  let right = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, i: number | null = null) => {
    const roles: Record<number, Role> = {};
    for (let k = left; k < right; k++) roles[k] = "pivot";
    if (i !== null) {
      for (let k = 0; k < (z[i] ?? 0); k++) {
        roles[k] = "compare";
        roles[i + k] = "compare";
      }
      roles[i] = "key";
    }
    const zRoles: Record<number, Role> = {};
    z.forEach((v, k) => {
      if (v === m && k > m) zRoles[k] = "sorted";
    });
    if (i !== null) zRoles[i] = "key";
    r.snap({
      narration,
      marker,
      strings: {
        rows: [
          { label: "combined", kind: "letters", offset: 0, cells: [...s], roles },
          { label: "z", kind: "numbers", offset: 0, cells: [...z], roles: zRoles },
        ],
      },
      vars: { i, left, right },
    });
  };

  snap(say("z-function.ready", { pattern, text }), null);
  for (let i = 1; i < n; i++) {
    z[i] = 0;
    if (i < right) {
      z[i] = Math.min(right - i, z[i - left]!);
      snap(say("z-function.reuse", { i, from: i - left, value: z[i]! }), "reuse", i);
    }
    while (i + z[i]! < n && s[z[i]!] === s[i + z[i]!]) {
      r.comparisons++;
      z[i]!++;
      snap(say("z-function.extend", { i, value: z[i]! }), "extend", i);
    }
    if (i + z[i]! < n) r.comparisons++;
    if (i + z[i]! > right) {
      left = i;
      right = i + z[i]!;
      r.writes++;
      snap(say("z-function.box", { left, right }), null, i);
    }
  }
  const hits: number[] = [];
  for (let i = m + 1; i < n; i++) if (z[i] === m) hits.push(i - m - 1);
  snap(say("z-function.done", { count: hits.length }), null);
  r.steps[r.steps.length - 1]!.result = [hits.length, ...hits];
  return r.steps;
}

export const zFunction: TracerDef = {
  id: "z-function",
  subject: "strings",
  scene: "string",
  limits: { min_len: 3, max_len: 25, min: -1, max: 25 },
  input: { kind: "codec", codec: textCodec(["text", "pattern"], { minLen: 1, maxLen: 12 }) },
  presets: {
    repeats: () => encodeText(["aabaabaab", "aab"]),
    random: (next) => {
      const word = (n: number) => Array.from({ length: n }, () => "ab"[Math.floor(next() * 2)]!).join("");
      return encodeText([word(7 + Math.floor(next() * 5)), word(2 + Math.floor(next() * 2))]);
    },
    "one-letter": () => encodeText(["aaaaaa", "aa"]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "box_moves" },
  ],
  legend: ["key", "compare", "pivot", "sorted"],
  roleLabels: { key: "current", compare: "matched_so_far", pivot: "z_box", sorted: "match" },
  vars: ["i", "left", "right"],
  trace,
};
