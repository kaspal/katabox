// Rabin-Karp. Mirrors content/learn/strings/rabin-karp/sample.{py,ts}: hash the pattern and
// the first window (base 26, mod 101); slide the window one letter at a time, updating the
// hash in O(1) (drop the first letter, add the next); compare letters only when the hashes
// agree. Equal hashes over different letters are collisions, counted separately.
// Output: matches, collisions, then the match positions.
import { decodeText, encodeText, textCodec } from "../engine/codecs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"rabin-karp">;
const KEYS = ["rabin-karp.ready", "rabin-karp.window", "rabin-karp.match", "rabin-karp.collision", "rabin-karp.roll", "rabin-karp.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams) => ({ key, params: params as NarrationParams<number> });

export const MARKERS = ["match", "collision", "roll"] as const;
export const BASE = 26;
export const MOD = 101;

function trace(input: readonly number[]): Step[] {
  const [text = "", pattern = ""] = decodeText(input);
  const n = text.length;
  const m = pattern.length;
  const r = new Recorder([]);
  const code = (ch: string) => ch.charCodeAt(0) - 97;
  const hits: number[] = [];
  let collisions = 0;
  let at = 0;
  let target = 0;
  let window = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, role: Role = "pivot") => {
    const roles: Record<number, Role> = {};
    for (const h of hits) for (let j = 0; j < m; j++) roles[h + j] = "sorted";
    if (m <= n) for (let j = 0; j < m; j++) roles[at + j] = role;
    r.snap({
      narration,
      marker,
      strings: {
        rows: [
          { label: "text", kind: "letters", offset: 0, cells: [...text], roles },
          { label: "pattern", kind: "letters", offset: at, cells: [...pattern], roles: {} },
        ],
      },
      vars: { target, window, at },
    });
  };
  if (m > n) {
    snap(say("rabin-karp.done", { num: 0, collisions: 0 }), null);
    r.steps[r.steps.length - 1]!.result = [0, 0];
    return r.steps;
  }
  let high = 1;
  for (let k = 0; k < m - 1; k++) high = (high * BASE) % MOD;
  for (let k = 0; k < m; k++) {
    target = (target * BASE + code(pattern[k]!)) % MOD;
    window = (window * BASE + code(text[k]!)) % MOD;
  }
  snap(say("rabin-karp.ready", { target, window, m }), null);
  for (let i = 0; i <= n - m; i++) {
    at = i;
    r.comparisons++;
    if (window === target) {
      if (text.slice(i, i + m) === pattern) {
        hits.push(i);
        snap(say("rabin-karp.match", { at: i, hash: window }), "match", "sorted");
      } else {
        collisions++;
        snap(say("rabin-karp.collision", { at: i, hash: window }), "collision", "swap");
      }
    } else {
      snap(say("rabin-karp.window", { at: i, hash: window, target }), null, "pivot");
    }
    if (i + m < n) {
      const out = text[i]!;
      const into = text[i + m]!;
      window = ((((window - code(out) * high) % MOD) + MOD) * BASE + code(into)) % MOD;
      r.writes++;
      snap(say("rabin-karp.roll", { out, into, hash: window }), "roll", "pivot");
    }
  }
  snap(say("rabin-karp.done", { num: hits.length, collisions }), null);
  r.steps[r.steps.length - 1]!.result = [hits.length, collisions, ...hits];
  return r.steps;
}

export const rabinKarp: TracerDef = {
  id: "rabin-karp",
  subject: "strings",
  scene: "string",
  limits: { min_len: 3, max_len: 33, min: -1, max: 25 },
  input: { kind: "codec", codec: textCodec(["text", "pattern"], { minLen: 1, maxLen: 16 }) },
  presets: {
    "with-a-collision": () => encodeText(["bcaxaavbcab", "bca"]),
    random: (next) => {
      const word = (n: number) => Array.from({ length: n }, () => "abcd"[Math.floor(next() * 4)]!).join("");
      return encodeText([word(10 + Math.floor(next() * 6)), word(2 + Math.floor(next() * 2))]);
    },
    repeats: () => encodeText(["abcabcabc", "abc"]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "windows" },
    { slot: "writes", label: "rolls" },
  ],
  legend: ["pivot", "sorted", "swap"],
  roleLabels: { pivot: "window", sorted: "match", swap: "collision" },
  vars: ["target", "window", "at"],
  trace,
};
