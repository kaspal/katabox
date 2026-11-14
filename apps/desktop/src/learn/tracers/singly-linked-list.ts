// Singly linked list. Mirrors content/learn/linear/singly-linked-list/sample.{py,ts}: add
// at the front (one link, O(1)), add at the back (walk to the last node, O(n)), delete the
// first node holding a value (walk with prev and cur, then route the link around it).
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { chain, listData, type LNode } from "../engine/list";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, PointerTone, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"singly-linked-list">;
const KEYS = [
  "singly-linked-list.ready",
  "singly-linked-list.link",
  "singly-linked-list.new_head",
  "singly-linked-list.first",
  "singly-linked-list.walk",
  "singly-linked-list.append",
  "singly-linked-list.search",
  "singly-linked-list.unlink_head",
  "singly-linked-list.unlink",
  "singly-linked-list.not_found",
  "singly-linked-list.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["link", "new-head", "first", "walk", "append", "search", "unlink-head", "unlink"] as const;
export const OPS: OpSpec[] = [
  { name: "front", args: 1 },
  { name: "back", args: 1 },
  { name: "delete", args: 1 },
];
const codec = opsCodec(OPS);
const enc = (ops: [string, number][]) => ops.flatMap(([name, v]) => [OPS.findIndex((o) => o.name === name) + 1, v]);

function trace(input: readonly number[]): Step[] {
  const ops = decodeOps(input, OPS, codec.width);
  const r = new Recorder([]);
  const nodes = new Map<number, LNode>();
  let head: number | null = null;
  let nextId = 0;
  /** Display order: the chain from head, plus a node not linked in yet. */
  let pending: { id: number; at: "start" | "end" } | null = null;
  const order = () => {
    const c = chain(nodes, head);
    if (pending && !c.includes(pending.id)) return pending.at === "start" ? [pending.id, ...c] : [...c, pending.id];
    return c;
  };
  const snap = (
    narration: ReturnType<typeof say>,
    marker: string | null,
    roles: Record<number, Role> = {},
    labels: [string, number | null, PointerTone][] = [],
    extraOrder?: number[],
  ) => {
    r.snap({
      narration,
      marker,
      list: listData(nodes, extraOrder ?? order(), { roles, labels: [["head", head, "key"], ...labels] }),
      vars: { size: chain(nodes, head).length, head: head === null ? null : nodes.get(head)!.value },
    });
  };

  snap(say("singly-linked-list.ready", { n: ops.length }), null);
  for (const op of ops) {
    const v = op.args[0]!;
    if (op.name === "front") {
      const id = nextId++;
      nodes.set(id, { id, value: v, next: head, prev: null });
      pending = { id, at: "start" };
      r.writes++;
      snap(say("singly-linked-list.link", { value: v }), "link", { [id]: "swap" });
      head = id;
      pending = null;
      snap(say("singly-linked-list.new_head", { value: v }), "new-head", { [id]: "swap" });
    } else if (op.name === "back") {
      const id = nextId++;
      nodes.set(id, { id, value: v, next: null, prev: null });
      r.writes++;
      if (head === null) {
        head = id;
        snap(say("singly-linked-list.first", { value: v }), "first", { [id]: "swap" });
      } else {
        pending = { id, at: "end" };
        let cur = head;
        while (nodes.get(cur)!.next !== null) {
          cur = nodes.get(cur)!.next!;
          r.comparisons++;
          snap(say("singly-linked-list.walk", { value: nodes.get(cur)!.value }), "walk", { [cur]: "compare", [id]: "key" }, [["cur", cur, "i"]]);
        }
        nodes.get(cur)!.next = id;
        pending = null;
        snap(say("singly-linked-list.append", { value: v, after: nodes.get(cur)!.value }), "append", { [id]: "swap" }, [["cur", cur, "i"]]);
      }
    } else {
      let prev: number | null = null;
      let cur: number | null = head;
      while (cur !== null && nodes.get(cur)!.value !== v) {
        prev = cur;
        cur = nodes.get(cur)!.next;
        r.comparisons++;
        snap(
          say("singly-linked-list.search", { value: v, seen: nodes.get(prev)!.value }),
          "search",
          cur === null ? { [prev]: "out" } : { [prev]: "out", [cur]: "compare" },
          [
            ["prev", prev, "j"],
            ["cur", cur, "i"],
          ],
        );
      }
      if (cur === null) {
        snap(say("singly-linked-list.not_found", { value: v }), null);
        continue;
      }
      const shown = order();
      const target = cur;
      r.moves++;
      if (prev === null) {
        head = nodes.get(cur)!.next;
        snap(say("singly-linked-list.unlink_head", { value: v }), "unlink-head", { [target]: "out" }, [["cur", target, "i"]], shown);
      } else {
        nodes.get(prev)!.next = nodes.get(cur)!.next;
        snap(
          say("singly-linked-list.unlink", { value: v, prev: nodes.get(prev)!.value }),
          "unlink",
          { [target]: "out" },
          [
            ["prev", prev, "j"],
            ["cur", target, "i"],
          ],
          shown,
        );
      }
      nodes.delete(target);
    }
  }
  const values = chain(nodes, head).map((id) => nodes.get(id)!.value);
  r.snap({ ...r.steps[r.steps.length - 1]!, list: listData(nodes, order(), { labels: [["head", head, "key"]] }), narration: say("singly-linked-list.done", { size: values.length }), marker: null, result: values });
  return r.steps;
}

export const singlyLinkedList: TracerDef = {
  id: "singly-linked-list",
  subject: "linear",
  scene: "list",
  limits: { min_len: 2, max_len: 24, min: 1, max: 99 },
  input: { kind: "codec", codec },
  presets: {
    random: (next) => {
      const ops: [string, number][] = [];
      const values: number[] = [];
      const n = int(next, 5, 9);
      for (let k = 0; k < n; k++) {
        const roll = next();
        if (values.length > 1 && roll < 0.25) {
          const v = values[int(next, 0, values.length - 1)]!;
          ops.push(["delete", v]);
          values.splice(values.indexOf(v), 1);
        } else {
          const v = int(next, 1, 99);
          ops.push([roll < 0.6 ? "front" : "back", v]);
          values.push(v);
        }
      }
      return enc(ops);
    },
    "build-and-delete": () => enc([["back", 12], ["back", 25], ["front", 7], ["back", 31], ["delete", 25], ["front", 3], ["delete", 3]]),
    "delete-missing": () => enc([["front", 5], ["front", 9], ["delete", 42], ["back", 1], ["delete", 5]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "inserts" },
    { slot: "moves", label: "deletes" },
    { slot: "comparisons", label: "steps_walked" },
  ],
  legend: ["swap", "compare", "key", "out"],
  roleLabels: { swap: "linking", compare: "visiting", key: "waiting_link", out: "passed" },
  vars: ["size", "head"],
  trace,
};
