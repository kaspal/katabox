// LRU cache. Mirrors content/learn/linear/lru-cache/sample.{py,ts}: a map for O(1) lookup
// plus a recency order (an OrderedDict / a Map keeps insertion order). A hit moves the key
// to the most-recent end; a put past the capacity evicts the least recent key. The scene
// shows the order as a doubly linked list (LRU on the left, MRU on the right), which is
// how such a cache is built by hand, and the map beside it.
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { listData, type LNode } from "../engine/list";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"lru-cache">;
const KEYS = [
  "lru-cache.ready",
  "lru-cache.miss",
  "lru-cache.hit",
  "lru-cache.refresh",
  "lru-cache.put",
  "lru-cache.evict",
  "lru-cache.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["miss", "hit", "refresh", "put", "evict"] as const;
export const OPS: OpSpec[] = [
  { name: "get", args: 1 },
  { name: "put", args: 2 },
];
const codec = opsCodec(OPS, ["capacity"]);
const enc = (capacity: number, ops: [string, number, number?][]) => [
  capacity,
  ...ops.flatMap(([name, k, v]) => [OPS.findIndex((o) => o.name === name) + 1, k, v ?? 0]),
];

function trace(input: readonly number[]): Step[] {
  const capacity = input[0]!;
  const ops = decodeOps(input, OPS, codec.width, 1);
  const r = new Recorder([]);
  /** Keys from least to most recently used, with their values. */
  const order: { key: number; value: number }[] = [];
  const out: number[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, evicted?: { key: number; value: number }) => {
    const shown = evicted ? [evicted, ...order] : order;
    const nodes = new Map<number, LNode>();
    shown.forEach((e, k) =>
      nodes.set(e.key, {
        id: e.key,
        value: e.value,
        text: `${e.key}:${e.value}`,
        next: k + 1 < shown.length ? shown[k + 1]!.key : null,
        prev: k > 0 ? shown[k - 1]!.key : null,
      }),
    );
    if (evicted) {
      // The evicted node is cut off: no links to or from it.
      nodes.get(evicted.key)!.next = null;
      if (order.length) nodes.get(order[0]!.key)!.prev = null;
    }
    r.snap({
      narration,
      marker,
      list: listData(
        nodes,
        shown.map((e) => e.key),
        {
          roles,
          doubly: true,
          labels: [
            ["LRU", order.length ? order[0]!.key : null, "j"],
            ["MRU", order.length ? order[order.length - 1]!.key : null, "key"],
          ],
        },
      ),
      table: { label: "map", rows: [...order].sort((a, b) => a.key - b.key).map((e) => ({ key: e.key, value: e.value, role: roles[e.key] })) },
      vars: { capacity, size: order.length, last: out.length ? out[out.length - 1]! : null },
    });
  };
  const touch = (key: number) => {
    const k = order.findIndex((e) => e.key === key);
    const [e] = order.splice(k, 1);
    order.push(e!);
  };

  snap(say("lru-cache.ready", { capacity }), null);
  for (const op of ops) {
    const key = op.args[0]!;
    const has = order.some((e) => e.key === key);
    if (op.name === "get") {
      r.comparisons++;
      if (!has) {
        out.push(-1);
        snap(say("lru-cache.miss", { key }), "miss");
      } else {
        touch(key);
        const value = order[order.length - 1]!.value;
        out.push(value);
        r.moves++;
        snap(say("lru-cache.hit", { key, value }), "hit", { [key]: "compare" });
      }
    } else {
      const value = op.args[1]!;
      if (has) {
        touch(key);
        r.moves++;
        snap(say("lru-cache.refresh", { key }), "refresh", { [key]: "compare" });
        order[order.length - 1]!.value = value;
      } else {
        order.push({ key, value });
      }
      r.writes++;
      snap(say("lru-cache.put", { key, value }), "put", { [key]: "swap" });
      if (order.length > capacity) {
        const evicted = order.shift()!;
        snap(say("lru-cache.evict", { key: evicted.key, capacity }), "evict", { [evicted.key]: "out" }, evicted);
      }
    }
  }
  snap(say("lru-cache.done", { gets: out.length }), null);
  r.steps[r.steps.length - 1]!.result = out;
  return r.steps;
}

export const lruCache: TracerDef = {
  id: "lru-cache",
  subject: "linear",
  scene: "list",
  limits: { min_len: 4, max_len: 37, min: 0, max: 99 },
  input: {
    kind: "codec",
    codec: {
      ...codec,
      check: (input) => (input[0]! < 1 || input[0]! > 5 ? { code: "custom", key: "capacity", params: { min: 1, max: 5 } } : codec.check!(input)),
    },
  },
  presets: {
    random: (next) => {
      const ops: [string, number, number?][] = [];
      const n = int(next, 7, 11);
      for (let k = 0; k < n; k++) {
        const key = int(next, 1, 5);
        ops.push(next() < 0.55 ? ["put", key, int(next, 10, 99)] : ["get", key]);
      }
      return enc(int(next, 2, 3), ops);
    },
    "lru-thrash": () => enc(2, [["put", 1, 10], ["put", 2, 20], ["get", 1], ["put", 3, 30], ["get", 2], ["get", 3], ["get", 1]]),
    "hits-keep-it": () => enc(3, [["put", 1, 11], ["put", 2, 22], ["put", 3, 33], ["get", 1], ["put", 4, 44], ["get", 2], ["get", 1]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "gets" },
    { slot: "writes", label: "puts" },
    { slot: "moves", label: "moved_to_front" },
  ],
  legend: ["compare", "swap", "out"],
  roleLabels: { compare: "used", swap: "written", out: "evicted" },
  vars: ["capacity", "size", "last"],
  trace,
};
