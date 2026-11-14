// Bloom filter. Mirrors content/learn/hashing/bloom-filter/sample.{py,ts}: 16 bits and three
// hash functions ((a·key + b) mod 16). add sets the key's three bits; check answers "maybe"
// (1) only if all three are set, and "no" (0) at the first clear bit. A "maybe" can be
// wrong (other keys set those bits); a "no" never is.
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"bloom-filter">;
const KEYS = [
  "bloom-filter.ready",
  "bloom-filter.set",
  "bloom-filter.check_set",
  "bloom-filter.check_clear",
  "bloom-filter.maybe",
  "bloom-filter.no",
  "bloom-filter.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params });

export const MARKERS = ["set", "check", "answer"] as const;
export const OPS: OpSpec[] = [
  { name: "add", args: 1 },
  { name: "check", args: 1 },
];
const codec = opsCodec(OPS);
const M = 16;
const HASHES: [number, number][] = [
  [1, 0],
  [7, 3],
  [11, 5],
];
export const positions = (key: number) => HASHES.map(([a, b]) => (key * a + b) % M);
const enc = (ops: [string, number][]) => ops.flatMap(([name, v]) => [OPS.findIndex((o) => o.name === name) + 1, v]);

function trace(input: readonly number[]): Step[] {
  const ops = decodeOps(input, OPS, codec.width);
  const r = new Recorder([]);
  const bits = new Array<number>(M).fill(0);
  const out: Item[] = [];
  let nextId = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, key: number | null, hits: number[], miss: number | null = null, outRole = false) =>
    r.snap({
      narration,
      marker,
      bitArray: { bits: [...bits], hits: [...hits], miss, key },
      aux: { label: "found", cells: out.map((c) => ({ ...c })), roles: outRole ? { [out.length - 1]: "swap" } : {} },
      vars: { key, bitsSet: bits.reduce((s, b) => s + b, 0) },
    });

  snap(say("bloom-filter.ready", { m: M }), null, null, []);
  for (const op of ops) {
    const key = op.args[0]!;
    const ps = positions(key);
    if (op.name === "add") {
      const done: number[] = [];
      for (const p of ps) {
        bits[p] = 1;
        done.push(p);
        r.writes++;
        snap(say("bloom-filter.set", { key, p }), "set", key, done);
      }
    } else {
      let maybe = 1;
      const seen: number[] = [];
      for (const p of ps) {
        seen.push(p);
        r.comparisons++;
        if (bits[p] === 0) {
          maybe = 0;
          snap(say("bloom-filter.check_clear", { key, p }), "check", key, seen, p);
          break;
        }
        snap(say("bloom-filter.check_set", { key, p }), "check", key, seen);
      }
      out.push({ id: nextId++, value: maybe });
      snap(say(maybe ? "bloom-filter.maybe" : "bloom-filter.no", { key, positions: ps.join(", ") }), "answer", key, seen, maybe ? null : seen[seen.length - 1]!, true);
    }
  }
  snap(say("bloom-filter.done", { checks: out.length }), null, null, []);
  r.steps[r.steps.length - 1]!.result = out.map((c) => c.value);
  return r.steps;
}

export const bloomFilter: TracerDef = {
  id: "bloom-filter",
  subject: "hashing",
  scene: "bits-array",
  limits: { min_len: 2, max_len: 24, min: 1, max: 99 },
  input: { kind: "codec", codec },
  presets: {
    random: (next) => {
      const ops: [string, number][] = [];
      const added: number[] = [];
      const n = int(next, 6, 10);
      for (let k = 0; k < n; k++) {
        if (k < 3 || next() < 0.45) {
          const key = int(next, 1, 99);
          added.push(key);
          ops.push(["add", key]);
        } else ops.push(["check", next() < 0.5 ? added[int(next, 0, added.length - 1)]! : int(next, 1, 99)]);
      }
      return enc(ops);
    },
    // 7 and 23 hit the same three bits (7, 4, 2 vs 7, 4, 2): checking 23 says "maybe" wrongly.
    "false-positive": () => enc([["add", 7], ["add", 12], ["check", 7], ["check", 23], ["check", 5]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "bits_set" },
    { slot: "comparisons", label: "bits_checked" },
  ],
  legend: ["compare", "swap"],
  roleLabels: { compare: "checked_bit", swap: "clear_bit" },
  vars: ["key", "bitsSet"],
  trace,
};
