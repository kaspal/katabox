// Fractional knapsack. Mirrors content/learn/greedy/fractional-knapsack/sample.{py,ts}:
// items can be split, so take them by value per unit of weight, best first (ties: input
// order); the first one that does not fit is taken in part, to fill the bag exactly.
// The answer can be a fraction, so the output is exact: [whole part, numerator,
// denominator] (the fraction in lowest terms; 0/1 when everything fits).
import { int } from "../engine/inputs";
import { itemsCodec } from "../engine/shapes";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"fractional-knapsack">;
const KEYS = ["fractional-knapsack.ready", "fractional-knapsack.sorted", "fractional-knapsack.take", "fractional-knapsack.part", "fractional-knapsack.done", "fractional-knapsack.all"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["take", "part"] as const;
const gcd = (a: number, b: number): number => (b === 0 ? a : gcd(b, a % b));

function trace(input: readonly number[]): Step[] {
  const capacity = input[0]!;
  const items = Array.from({ length: (input.length - 1) / 2 }, (_, k) => ({ id: k, w: input[1 + 2 * k]!, v: input[2 + 2 * k]! }));
  const order = [...items].sort((a, b) => b.v / b.w - a.v / a.w || a.id - b.id);
  const r = new Recorder(order.map((it) => it.v));
  const roles: Record<number, Role> = {};
  let whole = 0;
  let room = capacity;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, at: number | null = null) => {
    const shown: Record<number, Role> = { ...roles };
    if (at !== null) shown[at] = roles[at] ?? "key";
    r.snap({
      roles: shown,
      narration,
      marker,
      aux: { label: "weights", cells: order.map((it) => ({ id: it.id, value: it.w, text: `w${it.w}` })), roles: shown },
      vars: { room, value: whole },
    });
  };
  // Show each item as value / weight, best ratio first.
  r.items.forEach((it, k) => (it.text = `${order[k]!.v}/${order[k]!.w}`));

  snap(say("fractional-knapsack.ready", { n: items.length, capacity }), null);
  snap(say("fractional-knapsack.sorted"), null);
  for (const [k, it] of order.entries()) {
    r.comparisons++;
    if (it.w <= room) {
      whole += it.v;
      room -= it.w;
      roles[k] = "sorted";
      r.writes++;
      snap(say("fractional-knapsack.take", { k: it.id, w: it.w, v: it.v, room, total: whole }), "take", k);
    } else {
      const num = it.v * room;
      const den = it.w;
      const g = gcd(num, den);
      roles[k] = "swap";
      snap(say("fractional-knapsack.part", { k: it.id, room, w: it.w, v: it.v, num, den }), "part", k);
      const out = [whole + Math.floor(num / den), (num % den) / g, den / g];
      snap(say("fractional-knapsack.done", { whole: out[0]!, num: out[1]!, den: out[2]! }), null);
      r.steps[r.steps.length - 1]!.result = out;
      return r.steps;
    }
  }
  snap(say("fractional-knapsack.all", { total: whole }), null);
  r.steps[r.steps.length - 1]!.result = [whole, 0, 1];
  return r.steps;
}

const enc = (capacity: number, items: [number, number][]) => [capacity, ...items.flat()];

export const fractionalKnapsack: TracerDef = {
  id: "fractional-knapsack",
  subject: "greedy",
  scene: "array",
  limits: { min_len: 3, max_len: 13, min: 1, max: 20 },
  input: { kind: "codec", codec: itemsCodec({ capacity: 20, count: 6, weight: 9 }) },
  presets: {
    "spice-shelf": () => enc(10, [[4, 12], [6, 9], [3, 12], [5, 5]]),
    random: (next) => enc(int(next, 5, 20), Array.from({ length: int(next, 2, 6) }, () => [int(next, 1, 9), int(next, 1, 20)] as [number, number])),
    "all-fit": () => enc(20, [[3, 4], [5, 10], [2, 2]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "taken" },
  ],
  legend: ["key", "sorted", "swap"],
  roleLabels: { key: "current", sorted: "taken", swap: "part_taken" },
  vars: ["room", "value"],
  trace,
};
