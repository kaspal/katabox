// Knuth-Morris-Pratt. Mirrors content/learn/strings/kmp/sample.{py,ts}: first the failure
// table: fail[i] is the length of the longest proper prefix of the pattern that also ends
// at i (a "border"); then the scan: on a mismatch, slide the pattern so its border lines
// up, instead of starting over, so no text letter is read twice. Output: how many
// matches, then their starting positions.
import { decodeText, encodeText, textCodec } from "../engine/codecs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"kmp">;
const KEYS = ["kmp.ready", "kmp.fall", "kmp.border", "kmp.scan", "kmp.skip", "kmp.match", "kmp.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams) => ({ key, params: params as NarrationParams<number> });

export const MARKERS = ["fall", "border", "skip", "match"] as const;

function trace(input: readonly number[]): Step[] {
  const [text = "", pattern = ""] = decodeText(input);
  const m = pattern.length;
  const r = new Recorder([]);
  const fail: (number | null)[] = new Array(m).fill(null);
  fail[0] = 0;
  const hits: number[] = [];
  let phase: "table" | "scan" = "table";
  let shift = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, o: { i?: number; k?: number; matched?: number } = {}) => {
    const textRoles: Record<number, Role> = {};
    const patRoles: Record<number, Role> = {};
    const failRoles: Record<number, Role> = {};
    for (const h of hits) for (let j = 0; j < m; j++) textRoles[h + j] = "sorted";
    if (phase === "scan" && o.i !== undefined) {
      for (let j = 0; j < (o.matched ?? 0); j++) {
        textRoles[o.i - (o.matched ?? 0) + j] = "compare";
        patRoles[j] = "compare";
      }
      textRoles[o.i] = "key";
    }
    if (phase === "table" && o.i !== undefined) {
      patRoles[o.i] = "key";
      if (o.k !== undefined) patRoles[o.k] = "compare";
      failRoles[o.i] = "key";
    }
    if (o.k !== undefined && o.k > 0) failRoles[o.k - 1] = "compare";
    r.snap({
      narration,
      marker,
      strings: {
        rows: [
          { label: "text", kind: "letters", offset: 0, cells: [...text], roles: textRoles },
          { label: "pattern", kind: "letters", offset: shift, cells: [...pattern], roles: patRoles },
          { label: "failure", kind: "numbers", offset: shift, cells: [...fail], roles: failRoles },
        ],
      },
      vars: { i: o.i ?? null, k: o.k ?? null, matches: hits.length },
    });
  };

  snap(say("kmp.ready", { text, pattern }), null);
  let k = 0;
  for (let i = 1; i < m; i++) {
    while (k > 0 && pattern[i] !== pattern[k]) {
      r.comparisons++;
      k = fail[k - 1]!;
      snap(say("kmp.fall", { i, k }), "fall", { i, k });
    }
    r.comparisons++;
    if (pattern[i] === pattern[k]) k++;
    fail[i] = k;
    r.writes++;
    snap(say("kmp.border", { i, k }), "border", { i, k });
  }
  phase = "scan";
  snap(say("kmp.scan"), null);
  k = 0;
  for (let i = 0; i < text.length; i++) {
    while (k > 0 && text[i] !== pattern[k]) {
      r.comparisons++;
      k = fail[k - 1]!;
      shift = i - k;
      snap(say("kmp.skip", { i, k, letter: text[i]! }), "skip", { i, k, matched: k });
    }
    shift = i - k;
    r.comparisons++;
    if (text[i] === pattern[k]) k++;
    if (k === m) {
      hits.push(i - m + 1);
      snap(say("kmp.match", { at: i - m + 1 }), "match", { i, k, matched: m - 1 });
      k = fail[k - 1]!;
    }
  }
  snap(say("kmp.done", { num: hits.length, reads: r.comparisons }), null);
  r.steps[r.steps.length - 1]!.result = [hits.length, ...hits];
  return r.steps;
}

export const kmp: TracerDef = {
  id: "kmp",
  subject: "strings",
  scene: "string",
  limits: { min_len: 3, max_len: 33, min: -1, max: 25 },
  input: { kind: "codec", codec: textCodec(["text", "pattern"], { minLen: 1, maxLen: 16 }) },
  presets: {
    overlaps: () => encodeText(["abababcabab", "abab"]),
    random: (next) => {
      const word = (n: number) => Array.from({ length: n }, () => "ab"[Math.floor(next() * 2)]!).join("");
      return encodeText([word(8 + Math.floor(next() * 8)), word(2 + Math.floor(next() * 3))]);
    },
    "no-border": () => encodeText(["abcxabcdabcy", "abcd"]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "matched_so_far", sorted: "match" },
  vars: ["i", "k", "matches"],
  trace,
};
