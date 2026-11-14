// Doubly linked list as a deque. Mirrors content/learn/linear/doubly-linked-list/sample.{py,ts}:
// every node links both ways, so both ends are O(1): push and pop at the front or the back
// only touch the end node and its neighbour. Output: the popped values, 0, the list read
// forwards, 0, the list read backwards (which checks every back link).
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { chain, listData, type LNode } from "../engine/list";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"doubly-linked-list">;
const KEYS = [
  "doubly-linked-list.ready",
  "doubly-linked-list.link_front",
  "doubly-linked-list.link_back",
  "doubly-linked-list.empty",
  "doubly-linked-list.unlink_front",
  "doubly-linked-list.unlink_back",
  "doubly-linked-list.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["link-front", "link-back", "empty-front", "empty-back", "unlink-front", "unlink-back"] as const;
export const OPS: OpSpec[] = [
  { name: "front", args: 1 },
  { name: "back", args: 1 },
  { name: "pop-front", args: 0 },
  { name: "pop-back", args: 0 },
];
const codec = opsCodec(OPS);
const enc = (ops: [string, number?][]) => ops.flatMap(([name, v]) => [OPS.findIndex((o) => o.name === name) + 1, v ?? 0]);

function trace(input: readonly number[]): Step[] {
  const ops = decodeOps(input, OPS, codec.width);
  const r = new Recorder([]);
  const nodes = new Map<number, LNode>();
  let head: number | null = null;
  let tail: number | null = null;
  let nextId = 0;
  const popped: number[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, order?: number[]) =>
    r.snap({
      narration,
      marker,
      list: listData(nodes, order ?? chain(nodes, head), {
        roles,
        doubly: true,
        labels: [
          ["head", head, "key"],
          ["tail", tail, "j"],
        ],
      }),
      vars: { size: chain(nodes, head).length, popped: popped.length ? popped[popped.length - 1]! : null },
    });

  snap(say("doubly-linked-list.ready", { n: ops.length }), null);
  for (const op of ops) {
    if (op.name === "front" || op.name === "back") {
      const v = op.args[0]!;
      const id = nextId++;
      r.writes++;
      if (op.name === "front") {
        nodes.set(id, { id, value: v, next: head, prev: null });
        if (head !== null) nodes.get(head)!.prev = id;
        else tail = id;
        head = id;
        snap(say("doubly-linked-list.link_front", { value: v }), "link-front", { [id]: "swap" });
      } else {
        nodes.set(id, { id, value: v, next: null, prev: tail });
        if (tail !== null) nodes.get(tail)!.next = id;
        else head = id;
        tail = id;
        snap(say("doubly-linked-list.link_back", { value: v }), "link-back", { [id]: "swap" });
      }
    } else {
      const front = op.name === "pop-front";
      const end: number | null = front ? head : tail;
      if (end === null) {
        popped.push(-1);
        snap(say("doubly-linked-list.empty"), front ? "empty-front" : "empty-back");
        continue;
      }
      const shown = chain(nodes, head);
      const node: LNode = nodes.get(end)!;
      r.moves++;
      popped.push(node.value);
      if (front) {
        head = node.next;
        if (head !== null) nodes.get(head)!.prev = null;
        else tail = null;
      } else {
        tail = node.prev;
        if (tail !== null) nodes.get(tail)!.next = null;
        else head = null;
      }
      node.next = null;
      node.prev = null;
      snap(say(front ? "doubly-linked-list.unlink_front" : "doubly-linked-list.unlink_back", { value: node.value }), front ? "unlink-front" : "unlink-back", { [end]: "out" }, shown);
      nodes.delete(end);
    }
  }
  const forward = chain(nodes, head).map((id) => nodes.get(id)!.value);
  const backward: number[] = [];
  for (let id = tail; id !== null; id = nodes.get(id)!.prev) backward.push(nodes.get(id)!.value);
  r.snap({
    ...r.steps[r.steps.length - 1]!,
    list: listData(nodes, chain(nodes, head), { doubly: true, labels: [["head", head, "key"], ["tail", tail, "j"]] }),
    narration: say("doubly-linked-list.done", { size: forward.length, pops: popped.length }),
    marker: null,
    result: [...popped, 0, ...forward, 0, ...backward],
  });
  return r.steps;
}

export const doublyLinkedList: TracerDef = {
  id: "doubly-linked-list",
  subject: "linear",
  scene: "list",
  limits: { min_len: 2, max_len: 24, min: 0, max: 99 },
  input: { kind: "codec", codec },
  presets: {
    random: (next) => {
      const ops: [string, number?][] = [];
      const n = int(next, 6, 10);
      for (let k = 0; k < n; k++) {
        const roll = next();
        if (roll < 0.3) ops.push(["front", int(next, 1, 99)]);
        else if (roll < 0.65) ops.push(["back", int(next, 1, 99)]);
        else ops.push([roll < 0.82 ? "pop-front" : "pop-back"]);
      }
      return enc(ops);
    },
    "both-ends": () => enc([["back", 10], ["back", 20], ["front", 5], ["back", 30], ["pop-front"], ["pop-back"], ["front", 1]]),
    "drain": () => enc([["front", 4], ["back", 8], ["pop-back"], ["pop-back"], ["pop-front"]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "inserts" },
    { slot: "moves", label: "pops" },
  ],
  legend: ["swap", "out"],
  roleLabels: { swap: "linking", out: "unlinked" },
  vars: ["size", "popped"],
  trace,
};
