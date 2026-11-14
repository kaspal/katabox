// Amortised cost: a dynamic array that doubles when full. Mirrors
// content/learn/foundations/dynamic-array/sample.{py,ts}. The main row is the backing
// storage (empty slots up to the capacity); a resize opens the new storage below it and
// copies every value across, then the new storage becomes the array.
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"dynamic-array">;
const KEYS = [
  "dynamic-array.ready",
  "dynamic-array.room",
  "dynamic-array.full",
  "dynamic-array.copy",
  "dynamic-array.write",
  "dynamic-array.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["check", "copy", "write"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder([]);
  let capacity = 1;
  let copies = 0;
  let appends = 0;
  /** Cost per append so far: every write plus every copy, over the appends. */
  const perAppend = () => (appends === 0 ? 0 : Math.round(((appends + copies) / appends) * 100) / 100);
  const counters = () => {
    r.comparisons = appends;
    r.moves = copies;
    r.writes = perAppend();
  };
  const vars = (v: number | null) => ({ v, size: r.items.length, capacity, copies });

  r.snap({ narration: say("dynamic-array.ready", { n: values.length }), marker: null, capacity, vars: vars(null) });
  values.forEach((v, k) => {
    const size = r.items.length;
    if (size === capacity) {
      counters();
      r.snap({
        narration: say("dynamic-array.full", { size, capacity, bigger: capacity * 2, v }),
        marker: "check",
        capacity,
        vars: vars(v),
      });
      const old = capacity;
      capacity *= 2;
      const bigger: (Item | null)[] = Array.from({ length: capacity }, () => null);
      for (let i = 0; i < size; i++) {
        bigger[i] = { ...r.items[i]! };
        copies++;
        counters();
        const auxRoles: Record<number, Role> = { [i]: "swap" };
        r.snap({
          roles: { [i]: "compare" },
          pointers: [{ label: "i", at: i, tone: "i" }],
          narration: say("dynamic-array.copy", { i, value: r.value(i), copies }),
          marker: "copy",
          capacity: old,
          aux: { label: "storage", cells: bigger.map((c) => (c ? { ...c } : null)), roles: auxRoles },
          vars: { ...vars(v), capacity },
        });
      }
    } else {
      counters();
      r.snap({
        narration: say("dynamic-array.room", { size, capacity, v }),
        marker: "check",
        capacity,
        vars: vars(v),
      });
    }
    r.items.push({ id: k, value: v });
    appends++;
    counters();
    r.snap({
      roles: { [size]: "swap" },
      narration: say("dynamic-array.write", { v, at: size, size: size + 1, capacity }),
      marker: "write",
      capacity,
      vars: vars(v),
    });
  });
  counters();
  r.snap({
    narration: say("dynamic-array.done", { n: appends, copies, capacity, cost: perAppend() }),
    marker: null,
    capacity,
    vars: vars(null),
    result: [capacity, copies, ...r.items.map((it) => it.value)],
  });
  return r.steps;
}

export const dynamicArray: TracerDef = {
  id: "dynamic-array",
  subject: "foundations",
  scene: "array",
  limits: { min_len: 1, max_len: 16, min: 1, max: 99 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "appends" },
    { slot: "moves", label: "copies" },
    { slot: "writes", label: "per_append" },
  ],
  legend: ["compare", "swap"],
  roleLabels: { compare: "reading", swap: "writing" },
  vars: ["v", "size", "capacity", "copies"],
  trace,
};
