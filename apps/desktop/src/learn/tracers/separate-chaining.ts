// Hash table with separate chaining. Mirrors content/learn/hashing/separate-chaining/sample.{py,ts}:
// a key's bucket is key mod 7; the bucket is a list, scanned for a duplicate, and the key
// is appended. Colliding keys share a bucket's list. Output: each bucket's length, then
// its keys.
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"separate-chaining">;
const KEYS = [
  "separate-chaining.ready",
  "separate-chaining.hash",
  "separate-chaining.scan_empty",
  "separate-chaining.scan",
  "separate-chaining.duplicate",
  "separate-chaining.insert",
  "separate-chaining.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["hash", "scan", "insert"] as const;
export const M = 7;

function trace(keys: readonly number[]): Step[] {
  const r = new Recorder([]);
  const buckets: Item[][] = Array.from({ length: M }, () => []);
  let id = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, key: number | null, b: number | null) =>
    r.snap({
      narration,
      marker,
      hash: { mode: "chain", buckets: buckets.map((x) => x.map((it) => ({ ...it }))), probe: b === null ? [] : [b], key, home: b },
      vars: { key, bucket: b, chain: b === null ? null : buckets[b]!.length },
    });
  snap(say("separate-chaining.ready", { n: keys.length, m: M }), null, null, null);
  for (const key of keys) {
    const b = key % M;
    snap(say("separate-chaining.hash", { key, b }), "hash", key, b);
    r.comparisons += buckets[b]!.length;
    const dup = buckets[b]!.some((it) => it.value === key);
    snap(
      say(buckets[b]!.length === 0 ? "separate-chaining.scan_empty" : dup ? "separate-chaining.duplicate" : "separate-chaining.scan", { key, b, len: buckets[b]!.length }),
      "scan",
      key,
      b,
    );
    if (dup) continue;
    buckets[b]!.push({ id: id++, value: key });
    r.writes++;
    snap(say("separate-chaining.insert", { key, b, len: buckets[b]!.length }), "insert", key, b);
  }
  snap(say("separate-chaining.done", { n: r.writes, longest: Math.max(...buckets.map((x) => x.length)) }), null, null, null);
  r.steps[r.steps.length - 1]!.result = buckets.flatMap((x) => [x.length, ...x.map((it) => it.value)]);
  return r.steps;
}

export const separateChaining: TracerDef = {
  id: "separate-chaining",
  subject: "hashing",
  scene: "hash",
  limits: { min_len: 1, max_len: 12, min: 1, max: 99 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "inserts" },
    { slot: "comparisons", label: "comparisons" },
  ],
  legend: ["swap", "compare"],
  roleLabels: { swap: "written", compare: "visiting" },
  vars: ["key", "bucket", "chain"],
  trace,
};
