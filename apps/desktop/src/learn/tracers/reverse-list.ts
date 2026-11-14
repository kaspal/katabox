// Reversing a singly linked list in place. Mirrors
// content/learn/linear/reverse-list/sample.{py,ts}: walk once with prev, cur and nxt; save
// cur's next, point cur back at prev, move both on. The nodes stay where they are and the
// arrows flip one by one; at the end prev is the new head.
import { listData, type LNode } from "../engine/list";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, PointerTone, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"reverse-list">;
const KEYS = [
  "reverse-list.ready",
  "reverse-list.save",
  "reverse-list.flip",
  "reverse-list.advance",
  "reverse-list.new_head",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params });

export const MARKERS = ["save", "flip", "advance", "new-head"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder([]);
  const nodes = new Map<number, LNode>();
  values.forEach((value, id) => nodes.set(id, { id, value, next: id + 1 < values.length ? id + 1 : null, prev: null }));
  const order = values.map((_, id) => id);
  let prev: number | null = null;
  let cur: number | null = 0;
  let nxt: number | null = null;
  const val = (id: number | null) => (id === null ? null : nodes.get(id)!.value);
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, extra: [string, number | null, PointerTone][] = []) =>
    r.snap({
      narration,
      marker,
      list: listData(nodes, order, {
        roles,
        labels: [
          ["prev", prev, "j"],
          ["cur", cur, "i"],
          ["nxt", nxt, "pivot"],
          ...extra,
        ],
      }),
      vars: { prev: val(prev), cur: val(cur), nxt: val(nxt) },
    });

  snap(say("reverse-list.ready", { n: values.length }), null, {}, [["head", 0, "key"]]);
  while (cur !== null) {
    nxt = nodes.get(cur)!.next;
    snap(say("reverse-list.save", { cur: val(cur)!, next: val(nxt) ?? "∅" }), "save", { [cur]: "compare" });
    nodes.get(cur)!.next = prev;
    r.writes++;
    snap(say("reverse-list.flip", { cur: val(cur)!, prev: val(prev) ?? "∅" }), "flip", { [cur]: "swap" });
    prev = cur;
    cur = nxt;
    snap(say("reverse-list.advance", { prev: val(prev)! }), "advance", { [prev]: "sorted" });
  }
  const done: Record<number, Role> = {};
  for (const id of order) done[id] = "sorted";
  nxt = null;
  snap(say("reverse-list.new_head", { head: val(prev)!, n: values.length }), "new-head", done, [["head", prev, "key"]]);
  const out: number[] = [];
  for (let id = prev; id !== null; id = nodes.get(id)!.next) out.push(nodes.get(id)!.value);
  r.steps[r.steps.length - 1]!.result = out;
  return r.steps;
}

export const reverseList: TracerDef = {
  id: "reverse-list",
  subject: "linear",
  scene: "list",
  limits: { min_len: 1, max_len: 10, min: 1, max: 99 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "writes", label: "links_flipped" }],
  legend: ["compare", "swap", "sorted"],
  roleLabels: { compare: "visiting", swap: "linking", sorted: "reversed_part" },
  vars: ["prev", "cur", "nxt"],
  trace,
};
