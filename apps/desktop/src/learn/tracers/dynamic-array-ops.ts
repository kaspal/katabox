// Dynamic array: insert and delete in the middle. Mirrors
// content/learn/linear/dynamic-array-ops/sample.{py,ts}: inserting at i shifts every later
// value one place right (from the end), then writes; deleting at i shifts every later value
// one place left over it. Appending is an insert at the end (no shifts). The storage
// doubles when full (Foundations covers the cost of that). Indices past the end are clamped,
// as the sample does.
//
// The moving gap is an item with `hidden` set: shifting swaps it with its neighbour, so the
// neighbour slides over while the empty slot moves the other way.
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"dynamic-array-ops">;
const KEYS = [
  "dynamic-array-ops.ready",
  "dynamic-array-ops.grow",
  "dynamic-array-ops.shift_right",
  "dynamic-array-ops.write",
  "dynamic-array-ops.shift_left",
  "dynamic-array-ops.shrink",
  "dynamic-array-ops.empty",
  "dynamic-array-ops.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["grow", "shift-right", "write", "shift-left", "shrink"] as const;
export const OPS: OpSpec[] = [
  { name: "append", args: 1 },
  { name: "insert", args: 2 },
  { name: "remove", args: 1 },
];
const codec = opsCodec(OPS);
const enc = (ops: [string, number, number?][]) => ops.flatMap(([name, a, b]) => [OPS.findIndex((o) => o.name === name) + 1, a, b ?? 0]);

function trace(input: readonly number[]): Step[] {
  const ops = decodeOps(input, OPS, codec.width);
  const r = new Recorder([]);
  let capacity = 2;
  let nextId = 0;
  let gap: number | null = null; // the hidden item's id while it moves
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, extra: { i?: number } = {}) =>
    r.snap({
      narration,
      marker,
      roles,
      capacity,
      hidden: gap === null ? [] : [gap],
      pointers: extra.i === undefined ? [] : [{ label: "i", at: extra.i, tone: "i" }],
      vars: { size: r.items.length - (gap === null ? 0 : 1), capacity, i: extra.i ?? null },
    });

  snap(say("dynamic-array-ops.ready", { n: ops.length }), null);
  for (const op of ops) {
    const size = r.items.length;
    if (op.name === "append" || op.name === "insert") {
      const value = op.name === "append" ? op.args[0]! : op.args[1]!;
      const i = op.name === "append" ? size : Math.min(op.args[0]!, size);
      if (size === capacity) {
        capacity *= 2;
        snap(say("dynamic-array-ops.grow", { capacity, size }), "grow");
      }
      const id = nextId++;
      r.items.push({ id, value });
      gap = id;
      for (let k = size; k > i; k--) {
        r.swap(k, k - 1);
        r.moves++;
        r.writes++;
        snap(say("dynamic-array-ops.shift_right", { value: r.value(k), from: k - 1, to: k }), "shift-right", { [k]: "swap" }, { i });
      }
      gap = null;
      r.writes++;
      snap(say("dynamic-array-ops.write", { value, at: i }), "write", { [i]: "key" }, { i });
    } else {
      if (size === 0) {
        snap(say("dynamic-array-ops.empty"), null);
        continue;
      }
      const i = Math.min(op.args[0]!, size - 1);
      const removed = r.items[i]!;
      gap = removed.id;
      for (let k = i; k < size - 1; k++) {
        r.swap(k, k + 1);
        r.moves++;
        r.writes++;
        snap(say("dynamic-array-ops.shift_left", { value: r.value(k), from: k + 1, to: k, removed: removed.value }), "shift-left", { [k]: "swap" }, { i });
      }
      r.items.pop();
      gap = null;
      snap(say("dynamic-array-ops.shrink", { removed: removed.value, size: size - 1 }), "shrink", {}, { i });
    }
  }
  snap(say("dynamic-array-ops.done", { size: r.items.length, shifts: r.moves }), null);
  r.steps[r.steps.length - 1]!.result = r.items.map((it) => it.value);
  return r.steps;
}

export const dynamicArrayOps: TracerDef = {
  id: "dynamic-array-ops",
  subject: "linear",
  scene: "array",
  limits: { min_len: 3, max_len: 36, min: 0, max: 99 },
  input: { kind: "codec", codec },
  presets: {
    random: (next) => {
      const ops: [string, number, number?][] = [];
      let size = 0;
      const n = int(next, 6, 10);
      for (let k = 0; k < n; k++) {
        const roll = next();
        if (size === 0 || roll < 0.35) {
          ops.push(["append", int(next, 1, 99)]);
          size++;
        } else if (roll < 0.75) {
          ops.push(["insert", int(next, 0, size), int(next, 1, 99)]);
          size++;
        } else {
          ops.push(["remove", int(next, 0, size - 1)]);
          size--;
        }
      }
      return enc(ops);
    },
    "middle-inserts": () => enc([["append", 10], ["append", 20], ["append", 30], ["insert", 1, 15], ["insert", 0, 5], ["insert", 2, 12]]),
    "front-deletes": () => enc([["append", 4], ["append", 8], ["append", 15], ["append", 16], ["remove", 0], ["remove", 0], ["remove", 1]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "moves", label: "shifts" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["swap", "key"],
  roleLabels: { swap: "shift", key: "written" },
  vars: ["size", "capacity", "i"],
  trace,
};
