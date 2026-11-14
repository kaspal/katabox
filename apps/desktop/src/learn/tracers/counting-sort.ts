// Counting sort for small whole numbers. Mirrors
// content/learn/sorting/counting-sort/sample.{py,ts}: one bucket per possible value; each
// value drops into its bucket (the bucket's height is its count), then the buckets are
// emptied in value order. No value is ever compared with another.
import { BucketBoard, fixBucketDepth } from "../engine/buckets";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"counting-sort">;
const KEYS = ["counting-sort.ready", "counting-sort.count", "counting-sort.write", "counting-sort.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["count", "write"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const max = Math.max(...values);
  const board = new BucketBoard(
    r,
    Array.from({ length: max + 1 }, (_, v) => `${v}`),
  );
  r.snap({ ...board.frame(null), narration: say("counting-sort.ready", { n, max }), marker: null, vars: { v: null, k: null, count: null } });
  for (let p = 0; p < n; p++) {
    const v = r.value(p);
    board.drop(p, v);
    r.moves++;
    r.snap({
      ...board.frame(v),
      pointers: [{ label: "v", at: p, tone: "i" }],
      narration: say("counting-sort.count", { v, count: board.buckets[v]!.length }),
      marker: "count",
      vars: { v, k: null, count: board.buckets[v]!.length },
    });
  }
  let k = 0;
  for (let v = 0; v <= max; v++) {
    while (board.buckets[v]!.length > 0) {
      board.take(v, k);
      r.writes++;
      r.settle(k);
      r.snap({
        ...board.frame(v),
        roles: { [k]: "swap" },
        pointers: [{ label: "k", at: k, tone: "key" }],
        narration: say("counting-sort.write", { v, k, left: board.buckets[v]!.length }),
        marker: "write",
        vars: { v, k, count: board.buckets[v]!.length },
      });
      k++;
    }
  }
  r.snap({ ...board.frame(null), narration: say("counting-sort.done", { n, buckets: max + 1 }), marker: null, vars: { v: null, k: null, count: null } });
  return fixBucketDepth(r.steps);
}

export const countingSort: TracerDef = {
  id: "counting-sort",
  subject: "sorting",
  compareGroup: "sort",
  scene: "buckets",
  limits: { min_len: 2, max_len: 16, min: 0, max: 9 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "moves", label: "counted" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["compare", "swap", "sorted"],
  roleLabels: { compare: "bucket", swap: "writing" },
  vars: ["v", "count", "k"],
  trace,
};
