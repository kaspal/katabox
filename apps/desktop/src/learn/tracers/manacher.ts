// Manacher's longest palindrome. Mirrors content/learn/strings/manacher/sample.{py,ts}:
// put a bar between and around the letters (so even and odd palindromes look alike);
// radius[i] is how far the palindrome centred at i reaches. Inside the rightmost
// palindrome found so far, start from the mirrored centre's radius (mirror), then grow by
// comparing letters (grow). Output: the longest palindrome's length and its start (the
// first one, on ties).
import { decodeText, encodeText, textCodec } from "../engine/codecs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"manacher">;
const KEYS = ["manacher.ready", "manacher.mirror", "manacher.grow", "manacher.reach", "manacher.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params: params as Record<string, number> });

export const MARKERS = ["mirror", "grow"] as const;

function trace(input: readonly number[]): Step[] {
  const [text = ""] = decodeText(input);
  const s = `|${text.split("").join("|")}|`;
  const n = s.length;
  const r = new Recorder([]);
  const radius: (number | null)[] = new Array(n).fill(null);
  let center = 0;
  let right = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, i: number | null = null, mirror: number | null = null) => {
    const roles: Record<number, Role> = {};
    for (let k = 2 * center - right; k <= right; k++) if (k >= 0 && k < n && right > 0) roles[k] = "pivot";
    if (i !== null) {
      const rad = radius[i] ?? 0;
      for (let k = i - rad; k <= i + rad; k++) roles[k] = "compare";
      roles[i] = "key";
    }
    const radRoles: Record<number, Role> = {};
    if (i !== null) radRoles[i] = "key";
    if (mirror !== null) radRoles[mirror] = "compare";
    r.snap({
      narration,
      marker,
      strings: {
        rows: [
          { label: "with_bars", kind: "letters", offset: 0, cells: [...s], roles },
          { label: "radius", kind: "numbers", offset: 0, cells: [...radius], roles: radRoles },
        ],
      },
      vars: { i, center, right },
    });
  };

  snap(say("manacher.ready", { text }), null);
  for (let i = 0; i < n; i++) {
    radius[i] = 0;
    if (i < right) {
      radius[i] = Math.min(right - i, radius[2 * center - i]!);
      snap(say("manacher.mirror", { i, mirror: 2 * center - i, value: radius[i]! }), "mirror", i, 2 * center - i);
    }
    while (i - radius[i]! - 1 >= 0 && i + radius[i]! + 1 < n && s[i - radius[i]! - 1] === s[i + radius[i]! + 1]) {
      r.comparisons++;
      radius[i]!++;
      snap(say("manacher.grow", { i, value: radius[i]! }), "grow", i);
    }
    if (i + radius[i]! > right) {
      center = i;
      right = i + radius[i]!;
      r.writes++;
      snap(say("manacher.reach", { center, right }), null, i);
    }
  }
  let best = 0;
  for (let i = 0; i < n; i++) if (radius[i]! > radius[best]!) best = i;
  const length = radius[best]!;
  const start = (best - length) / 2;
  snap(say("manacher.done", { length, start, word: text.slice(start, start + length) }), null, best);
  r.steps[r.steps.length - 1]!.result = [length, start];
  return r.steps;
}

export const manacher: TracerDef = {
  id: "manacher",
  subject: "strings",
  scene: "string",
  limits: { min_len: 1, max_len: 12, min: 0, max: 25 },
  input: { kind: "codec", codec: textCodec(["text"], { minLen: 1, maxLen: 12 }) },
  presets: {
    "nested-mirrors": () => encodeText(["abacabadab"]),
    random: (next) => encodeText([Array.from({ length: 5 + Math.floor(next() * 8) }, () => "abc"[Math.floor(next() * 3)]!).join("")]),
    "even-length": () => encodeText(["xyzzyx"]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "box_moves" },
  ],
  legend: ["key", "compare", "pivot"],
  roleLabels: { key: "current", compare: "palindrome_here", pivot: "rightmost_palindrome" },
  vars: ["i", "center", "right"],
  trace,
};
