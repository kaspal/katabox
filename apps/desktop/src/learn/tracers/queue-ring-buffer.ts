// Queue in a ring buffer. Mirrors content/learn/linear/queue-ring-buffer/sample.{py,ts}: a
// fixed array with a head index and a size; the tail is (head + size) mod capacity, so
// both ends move forward and wrap around instead of shifting values. A full buffer refuses
// an enqueue; an empty one returns −1.
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, Pointer, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"queue-ring-buffer">;
const KEYS = [
  "queue-ring-buffer.ready",
  "queue-ring-buffer.full",
  "queue-ring-buffer.enqueue",
  "queue-ring-buffer.empty",
  "queue-ring-buffer.dequeue",
  "queue-ring-buffer.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["full", "enqueue", "empty", "dequeue"] as const;
export const OPS: OpSpec[] = [
  { name: "enqueue", args: 1 },
  { name: "dequeue", args: 0 },
];
const codec = opsCodec(OPS, ["capacity"]);
const enc = (capacity: number, ops: [string, number?][]) => [capacity, ...ops.flatMap(([name, v]) => [OPS.findIndex((o) => o.name === name) + 1, v ?? 0])];

function trace(input: readonly number[]): Step[] {
  const capacity = input[0]!;
  const ops = decodeOps(input, OPS, codec.width, 1);
  const r = new Recorder([]);
  const data: (Item | null)[] = new Array<Item | null>(capacity).fill(null);
  let head = 0;
  let size = 0;
  let nextId = 0;
  const out: Item[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, outRole = false) => {
    const tail = (head + size) % capacity;
    const pointers: Pointer[] = [
      { label: "head", at: head, tone: "i" },
      { label: "tail", at: tail, tone: "j" },
    ];
    r.snap({
      narration,
      marker,
      slots: { layout: "ring", label: "ring", cells: data.map((c) => (c ? { ...c } : null)), roles, pointers },
      aux: { label: "output", cells: out.map((c) => ({ ...c })), roles: outRole ? { [out.length - 1]: "swap" } : {} },
      vars: { capacity, head, size, tail },
    });
  };

  snap(say("queue-ring-buffer.ready", { capacity }), null);
  for (const op of ops) {
    if (op.name === "enqueue") {
      const v = op.args[0]!;
      if (size === capacity) {
        snap(say("queue-ring-buffer.full", { value: v, capacity }), "full");
        continue;
      }
      const tail = (head + size) % capacity;
      data[tail] = { id: nextId++, value: v };
      size++;
      r.writes++;
      snap(say("queue-ring-buffer.enqueue", { value: v, at: tail }), "enqueue", { [tail]: "swap" });
    } else {
      if (size === 0) {
        out.push({ id: nextId++, value: -1 });
        snap(say("queue-ring-buffer.empty"), "empty", {}, true);
        continue;
      }
      const it = data[head]!;
      const at = head;
      data[head] = null;
      out.push({ id: nextId++, value: it.value });
      head = (head + 1) % capacity;
      size--;
      r.moves++;
      snap(say("queue-ring-buffer.dequeue", { value: it.value, at, head }), "dequeue", {}, true);
    }
  }
  snap(say("queue-ring-buffer.done", { count: out.length }), null);
  r.steps[r.steps.length - 1]!.result = out.map((c) => c.value);
  return r.steps;
}

export const queueRingBuffer: TracerDef = {
  id: "queue-ring-buffer",
  subject: "linear",
  scene: "slots",
  limits: { min_len: 3, max_len: 27, min: 0, max: 99 },
  input: {
    kind: "codec",
    codec: {
      ...codec,
      check: (input) => (input[0]! < 2 || input[0]! > 6 ? { code: "custom", key: "capacity", params: { min: 2, max: 6 } } : codec.check!(input)),
    },
  },
  presets: {
    random: (next) => {
      const ops: [string, number?][] = [];
      const n = int(next, 8, 12);
      for (let k = 0; k < n; k++) ops.push(next() < 0.6 ? ["enqueue", int(next, 1, 99)] : ["dequeue"]);
      return enc(int(next, 3, 5), ops);
    },
    "wrap-around": () => enc(4, [["enqueue", 11], ["enqueue", 22], ["enqueue", 33], ["dequeue"], ["dequeue"], ["enqueue", 44], ["enqueue", 55], ["enqueue", 66], ["dequeue"], ["dequeue"]]),
    full: () => enc(3, [["enqueue", 5], ["enqueue", 6], ["enqueue", 7], ["enqueue", 8], ["dequeue"], ["enqueue", 9], ["dequeue"], ["dequeue"], ["dequeue"], ["dequeue"]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "enqueued" },
    { slot: "moves", label: "dequeued" },
  ],
  legend: ["swap"],
  roleLabels: { swap: "written" },
  vars: ["capacity", "head", "size", "tail"],
  trace,
};
