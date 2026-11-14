// Radix sort, least significant digit first, base 10. Mirrors
// content/learn/sorting/radix-sort/sample.{py,ts}: each pass drops every value into the
// bucket of one digit (ones, then tens, then hundreds) and collects the buckets in order.
// Buckets hand values back first in, first out, so each pass keeps the order of the one
// before: that stability is why the last pass leaves everything sorted.
import { BucketBoard, fixBucketDepth } from "../engine/buckets";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"radix-sort">;
const KEYS = [
  "radix-sort.ready",
  "radix-sort.pass",
  "radix-sort.distribute",
  "radix-sort.collect",
  "radix-sort.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["pass", "distribute", "collect"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const max = Math.max(...values);
  const labels = Array.from({ length: 10 }, (_, d) => `${d}`);
  let board = new BucketBoard(r, labels);
  r.snap({ ...board.frame(null), narration: say("radix-sort.ready", { n, max }), marker: null, vars: { place: null, v: null, digit: null, k: null } });
  let passes = 0;
  for (let place = 1; place <= max; place *= 10) {
    passes++;
    board = new BucketBoard(r, labels);
    r.snap({ ...board.frame(null), narration: say("radix-sort.pass", { place }), marker: "pass", vars: { place, v: null, digit: null, k: null } });
    for (let p = 0; p < n; p++) {
      const v = r.value(p);
      const digit = Math.floor(v / place) % 10;
      board.drop(p, digit);
      r.moves++;
      r.snap({
        ...board.frame(digit),
        pointers: [{ label: "v", at: p, tone: "i" }],
        narration: say("radix-sort.distribute", { v, digit, place }),
        marker: "distribute",
        vars: { place, v, digit, k: null },
      });
    }
    let k = 0;
    for (let d = 0; d < 10; d++) {
      while (board.buckets[d]!.length > 0) {
        const it = board.take(d, k);
        r.writes++;
        r.snap({
          ...board.frame(d),
          roles: { [k]: "swap" },
          pointers: [{ label: "k", at: k, tone: "key" }],
          narration: say("radix-sort.collect", { v: it.value, digit: d, k }),
          marker: "collect",
          vars: { place, v: it.value, digit: d, k },
        });
        k++;
      }
    }
  }
  r.settleAll();
  r.snap({ ...board.frame(null), narration: say("radix-sort.done", { n, passes }), marker: null, vars: { place: null, v: null, digit: null, k: null } });
  return fixBucketDepth(r.steps);
}

export const radixSort: TracerDef = {
  id: "radix-sort",
  subject: "sorting",
  compareGroup: "sort",
  scene: "buckets",
  limits: { min_len: 2, max_len: 12, min: 1, max: 999 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "moves", label: "distributed" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["compare", "swap", "sorted"],
  roleLabels: { compare: "bucket", swap: "writing" },
  vars: ["place", "v", "digit", "k"],
  trace,
};
