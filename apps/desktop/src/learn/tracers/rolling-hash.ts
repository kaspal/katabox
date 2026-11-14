// Rolling hash. Mirrors content/learn/hashing/rolling-hash/sample.{py,ts}: the polynomial
// hash of every window of `length` letters (base 26, mod 101). The first window is built
// letter by letter; each next window takes the leaving letter out (times 26^(length−1)),
// shifts, and adds the entering letter: O(1) per window instead of O(length).
import { decodeText, encodeText } from "../engine/codecs";
import { int, tokens } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"rolling-hash">;
const KEYS = [
  "rolling-hash.ready",
  "rolling-hash.add",
  "rolling-hash.remove",
  "rolling-hash.shift_in",
  "rolling-hash.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams) => ({ key, params });

export const MARKERS = ["add", "remove", "shift-in"] as const;
const BASE = 26;
const MOD = 101;

function trace(input: readonly number[]): Step[] {
  const length = input[0]!;
  const text = decodeText(input.slice(1))[0]!;
  const r = new Recorder(text.split("").map((c) => c.charCodeAt(0) - 97));
  r.items.forEach((it, k) => (it.text = text[k]!));
  let power = 1;
  for (let k = 0; k < length - 1; k++) power = (power * BASE) % MOD;
  const out: Item[] = [];
  let h = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, lo: number, hi: number, roles: Record<number, "swap" | "out"> = {}) =>
    r.snap({
      narration,
      marker,
      roles,
      window: hi >= lo ? [lo, hi] : null,
      aux: { label: "hashes", cells: out.map((c) => ({ ...c })), roles: {} },
      vars: { h, power, window: hi >= lo ? text.slice(lo, hi + 1) : null },
    });
  snap(say("rolling-hash.ready", { length, n: text.length, base: BASE, mod: MOD }), null, 0, -1);
  for (let k = 0; k < length; k++) {
    const c = text.charCodeAt(k) - 97;
    h = (h * BASE + c) % MOD;
    r.writes++;
    snap(say("rolling-hash.add", { letter: text[k]!, code: c, h }), "add", 0, k, { [k]: "swap" });
  }
  out.push({ id: 0, value: h });
  for (let i = length; i < text.length; i++) {
    const gone = text.charCodeAt(i - length) - 97;
    h = (((h - gone * power) % MOD) + MOD) % MOD;
    r.moves++;
    snap(say("rolling-hash.remove", { letter: text[i - length]!, code: gone, power, h }), "remove", i - length + 1, i - 1, { [i - length]: "out" });
    const c = text.charCodeAt(i) - 97;
    h = (h * BASE + c) % MOD;
    r.writes++;
    out.push({ id: out.length, value: h });
    snap(say("rolling-hash.shift_in", { letter: text[i]!, code: c, h }), "shift-in", i - length + 1, i, { [i]: "swap" });
  }
  snap(say("rolling-hash.done", { windows: out.length, length }), null, 0, -1);
  r.steps[r.steps.length - 1]!.result = out.map((c) => c.value);
  return r.steps;
}

const WORDS = ["banana", "abracadabra", "mississippi", "cabcabcab", "hellohello", "panama"];

export const rollingHash: TracerDef = {
  id: "rolling-hash",
  subject: "hashing",
  scene: "array",
  limits: { min_len: 2, max_len: 13, min: -1, max: 25 },
  input: {
    kind: "codec",
    codec: {
      fields: ["text", "length"],
      format: (input) => [decodeText(input.slice(1))[0] ?? "", String(input[0] ?? "")],
      parse: (texts) => {
        const word = (texts[0] ?? "").trim().toLowerCase();
        if (!/^[a-z]+$/.test(word)) return { error: { code: "custom", key: "letters_only", params: { field: "text" } } };
        if (word.length > 12) return { error: { code: "custom", key: "word_length", params: { field: "text", min: 1, max: 12 } } };
        const r = tokens(texts[1] ?? "");
        if ("error" in r) return r;
        if (r.values.length !== 1) return { error: { code: "custom", key: "one_number", params: { field: "length" } } };
        return { values: [r.values[0]!, ...encodeText([word])] };
      },
      check: (input) => {
        const text = decodeText(input.slice(1));
        if (text.length !== 1 || text[0]!.length === 0) return { code: "custom", key: "size" };
        const len = input[0]!;
        return len >= 1 && len <= text[0]!.length ? null : { code: "custom", key: "window", params: { max: text[0]!.length } };
      },
    },
  },
  presets: {
    random: (next) => {
      const n = int(next, 4, 12);
      const word = Array.from({ length: n }, () => String.fromCharCode(97 + int(next, 0, 4))).join("");
      return [int(next, 2, Math.min(4, n)), ...encodeText([word])];
    },
    repeat: () => [3, ...encodeText(["abcabcab"])],
    words: (next) => {
      const w = WORDS[int(next, 0, WORDS.length - 1)]!;
      return [3, ...encodeText([w])];
    },
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "letters_added" },
    { slot: "moves", label: "letters_removed" },
  ],
  legend: ["swap", "out"],
  roleLabels: { swap: "entering", out: "leaving" },
  vars: ["h", "power", "window"],
  trace,
};
