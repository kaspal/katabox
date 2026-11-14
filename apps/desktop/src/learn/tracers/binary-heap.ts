// Binary heap as a priority queue (a min-heap). Mirrors content/learn/trees/binary-heap/sample.{py,ts}:
// push appends at the end and sifts up while smaller than its parent; pop takes the root,
// moves the last value to the root and sifts it down, swapping with the smaller child.
// Both O(log n). The heap scene draws the array as a complete tree above it.
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Pointer, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"binary-heap">;
const KEYS = [
  "binary-heap.ready",
  "binary-heap.append",
  "binary-heap.compare_up_stop",
  "binary-heap.compare_up_swap",
  "binary-heap.swap_up",
  "binary-heap.empty",
  "binary-heap.take",
  "binary-heap.move_last",
  "binary-heap.compare_down_stop",
  "binary-heap.compare_down_swap",
  "binary-heap.swap_down",
  "binary-heap.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["append", "compare-up", "swap-up", "empty", "take", "move-last", "compare-down", "swap-down"] as const;
export const OPS: OpSpec[] = [
  { name: "push", args: 1 },
  { name: "pop", args: 0 },
];
const codec = opsCodec(OPS);
const enc = (ops: [string, number?][]) => ops.flatMap(([name, v]) => [OPS.findIndex((o) => o.name === name) + 1, v ?? 0]);

function trace(input: readonly number[]): Step[] {
  const ops = decodeOps(input, OPS, codec.width);
  const r = new Recorder([]);
  const out: number[] = [];
  let nextId = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, pointers: Pointer[] = []) =>
    r.snap({ narration, marker, roles, pointers, heap: r.items.length, vars: { size: r.items.length, top: r.items.length ? r.value(0) : null, popped: out.length ? out[out.length - 1]! : null } });

  snap(say("binary-heap.ready", { n: ops.length }), null);
  for (const op of ops) {
    if (op.name === "push") {
      const value = op.args[0]!;
      r.items.push({ id: nextId++, value });
      r.writes++;
      let i = r.items.length - 1;
      snap(say("binary-heap.append", { value, at: i }), "append", { [i]: "swap" });
      while (i > 0) {
        const parent = Math.floor((i - 1) / 2);
        r.comparisons++;
        const stop = r.value(parent) <= r.value(i);
        snap(say(stop ? "binary-heap.compare_up_stop" : "binary-heap.compare_up_swap", { value: r.value(i), parent: r.value(parent) }), "compare-up", { [parent]: "compare", [i]: "key" });
        if (stop) break;
        r.swap(parent, i);
        r.moves++;
        snap(say("binary-heap.swap_up", { value: r.value(parent), at: parent }), "swap-up", { [parent]: "swap", [i]: "swap" });
        i = parent;
      }
    } else {
      if (r.items.length === 0) {
        out.push(-1);
        snap(say("binary-heap.empty"), "empty");
        continue;
      }
      const top = r.value(0);
      out.push(top);
      snap(say("binary-heap.take", { value: top }), "take", { 0: "sorted" });
      const last = r.items.pop()!;
      if (r.items.length > 0) {
        r.items[0] = last;
        r.writes++;
        snap(say("binary-heap.move_last", { value: last.value }), "move-last", { 0: "key" });
        let i = 0;
        while (2 * i + 1 < r.items.length) {
          let child = 2 * i + 1;
          if (child + 1 < r.items.length) {
            r.comparisons++; // pick the smaller child
            if (r.value(child + 1) < r.value(child)) child++;
          }
          r.comparisons++;
          const stop = r.value(i) <= r.value(child);
          snap(say(stop ? "binary-heap.compare_down_stop" : "binary-heap.compare_down_swap", { value: r.value(i), child: r.value(child) }), "compare-down", { [i]: "key", [child]: "compare" });
          if (stop) break;
          r.swap(i, child);
          r.moves++;
          snap(say("binary-heap.swap_down", { value: r.value(child), at: child }), "swap-down", { [i]: "swap", [child]: "swap" });
          i = child;
        }
      }
    }
  }
  snap(say("binary-heap.done", { pops: out.length, size: r.items.length }), null);
  r.steps[r.steps.length - 1]!.result = out;
  return r.steps;
}

export const binaryHeap: TracerDef = {
  id: "binary-heap",
  subject: "trees",
  scene: "heap",
  limits: { min_len: 2, max_len: 32, min: 0, max: 99 },
  input: { kind: "codec", codec },
  presets: {
    random: (next) => {
      const ops: [string, number?][] = [];
      let size = 0;
      const n = int(next, 8, 14);
      for (let k = 0; k < n; k++) {
        if (size === 0 || next() < 0.62) {
          ops.push(["push", int(next, 1, 99)]);
          size++;
        } else {
          ops.push(["pop"]);
          size--;
        }
      }
      return enc(ops);
    },
    "heap-sort-order": () => enc([["push", 42], ["push", 17], ["push", 8], ["push", 23], ["push", 4], ["pop"], ["pop"], ["pop"], ["pop"], ["pop"]]),
    "rising-pushes": () => enc([["push", 50], ["push", 40], ["push", 30], ["push", 20], ["push", 10], ["pop"]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "moves", label: "swaps" },
  ],
  legend: ["key", "compare", "swap", "sorted"],
  roleLabels: { key: "sifting", compare: "comparing_with", swap: "swap", sorted: "popped" },
  vars: ["size", "top", "popped"],
  trace,
};
