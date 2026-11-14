// Bucket sort for values 1-99 in five buckets of 20. Mirrors
// content/learn/sorting/bucket-sort/sample.{py,ts}: scatter each value into the bucket for
// its range, sort each (small) bucket, and gather the buckets in order.
import { BucketBoard, fixBucketDepth } from "../engine/buckets";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"bucket-sort">;
const KEYS = [
  "bucket-sort.ready",
  "bucket-sort.scatter",
  "bucket-sort.sort",
  "bucket-sort.sort_empty",
  "bucket-sort.gather",
  "bucket-sort.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params });

export const MARKERS = ["scatter", "sort", "gather"] as const;
const COUNT = 5;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const labels = Array.from({ length: COUNT }, (_, b) => `${(b * 100) / COUNT}–${((b + 1) * 100) / COUNT - 1}`);
  const board = new BucketBoard(r, labels);
  r.snap({ ...board.frame(null), narration: say("bucket-sort.ready", { n, count: COUNT }), marker: null, vars: { v: null, bucket: null, k: null } });
  for (let p = 0; p < n; p++) {
    const v = r.value(p);
    const b = Math.floor((v * COUNT) / 100);
    board.drop(p, b);
    r.moves++;
    r.snap({
      ...board.frame(b),
      pointers: [{ label: "v", at: p, tone: "i" }],
      narration: say("bucket-sort.scatter", { v, range: labels[b]! }),
      marker: "scatter",
      vars: { v, bucket: b, k: null },
    });
  }
  let k = 0;
  for (let b = 0; b < COUNT; b++) {
    const size = board.buckets[b]!.length;
    // Comparisons a small sort needs at most: insertion sort's worst case.
    r.comparisons += (size * (size - 1)) / 2;
    board.sortBucket(b);
    r.snap({
      ...board.frame(b),
      narration: say(size === 0 ? "bucket-sort.sort_empty" : "bucket-sort.sort", { range: labels[b]!, size }),
      marker: "sort",
      vars: { v: null, bucket: b, k },
    });
    while (board.buckets[b]!.length > 0) {
      const it = board.take(b, k);
      r.writes++;
      r.settle(k);
      r.snap({
        ...board.frame(b),
        roles: { [k]: "swap" },
        pointers: [{ label: "k", at: k, tone: "key" }],
        narration: say("bucket-sort.gather", { v: it.value, k }),
        marker: "gather",
        vars: { v: it.value, bucket: b, k },
      });
      k++;
    }
  }
  r.snap({ ...board.frame(null), narration: say("bucket-sort.done", { n, count: COUNT }), marker: null, vars: { v: null, bucket: null, k: null } });
  return fixBucketDepth(r.steps);
}

export const bucketSort: TracerDef = {
  id: "bucket-sort",
  subject: "sorting",
  compareGroup: "sort",
  scene: "buckets",
  limits: { min_len: 2, max_len: 16, min: 1, max: 99 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "moves", label: "scattered" },
    { slot: "comparisons", label: "comparisons_max" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["compare", "swap", "sorted"],
  roleLabels: { compare: "bucket", swap: "writing" },
  vars: ["v", "bucket", "k"],
  trace,
};
