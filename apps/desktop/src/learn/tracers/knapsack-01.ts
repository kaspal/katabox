// 0/1 knapsack. Mirrors content/learn/dynamic-programming/knapsack-01/sample.{py,ts}:
// best[i][c] is the most value from the first i items within capacity c. Item i is
// either left out (the cell above) or packed (the cell above and `weight` to the left,
// plus its value). Tracing back from the corner: wherever a cell differs from the one
// above, its item was packed. Output: the best value, then the items packed (0-based).
import { fail } from "../engine/codecs";
import { int, tokens, type InputError } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"knapsack-01">;
const KEYS = ["knapsack-01.ready", "knapsack-01.skip", "knapsack-01.take", "knapsack-01.trace", "knapsack-01.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["skip", "take", "trace"] as const;
export const MAX_CAPACITY = 12;
export const MAX_ITEMS = 6;

const itemsOf = (input: readonly number[]) => Array.from({ length: Math.floor((input.length - 1) / 2) }, (_, k) => [input[1 + 2 * k]!, input[2 + 2 * k]!] as [number, number]);
export const enc = (capacity: number, items: [number, number][]) => [capacity, ...items.flat()];

function trace(input: readonly number[]): Step[] {
  const capacity = input[0]!;
  const items = itemsOf(input);
  const n = items.length;
  const r = new Recorder([]);
  const best: (number | null)[][] = Array.from({ length: n + 1 }, (_, i) => new Array(capacity + 1).fill(i === 0 ? 0 : null));
  const chosen: number[] = [];
  const path: [number, number][] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: [number, number] | null = null, deps: [number, number][] = [], item: number | null = null) => {
    const roles: Record<number, Role> = {};
    for (const k of chosen) roles[k] = "swap";
    if (item !== null) roles[item] = "key";
    r.snap({
      narration,
      marker,
      slots: { layout: "row", label: "items", cells: items.map(([w, v], id) => ({ id, value: v, text: `${w}/${v}` })), roles },
      dp: {
        label: "best_value",
        colLabels: Array.from({ length: capacity + 1 }, (_, k) => String(k)),
        rowLabels: ["–", ...items.map(([w, v], k) => `${k}: ${w}/${v}`)],
        cells: best.map((row) => [...row]),
        hot,
        deps,
        path: [...path],
      },
      vars: { item: item, capacity: hot ? hot[1] : null },
    });
  };

  snap(say("knapsack-01.ready", { n, capacity }), null);
  for (let i = 1; i <= n; i++) {
    const [weight, value] = items[i - 1]!;
    for (let c = 0; c <= capacity; c++) {
      // Leave it out first (the cell above), then pack it if that is better.
      best[i]![c] = best[i - 1]![c]!;
      r.writes++;
      snap(say("knapsack-01.skip", { item: i - 1, c, value: best[i]![c]!, weight }), "skip", [i, c], [[i - 1, c]], i - 1);
      const fits = weight <= c;
      const takeValue = fits ? best[i - 1]![c - weight]! + value : -1;
      if (fits) r.comparisons++;
      if (fits && takeValue > best[i]![c]!) {
        best[i]![c] = takeValue;
        r.writes++;
        snap(say("knapsack-01.take", { item: i - 1, c, value: takeValue, without: best[i - 1]![c]! }), "take", [i, c], [[i - 1, c], [i - 1, c - weight]], i - 1);
      }
    }
  }
  let c = capacity;
  path.push([n, c]);
  for (let i = n; i > 0; i--) {
    if (best[i]![c] !== best[i - 1]![c]) {
      chosen.push(i - 1);
      c -= items[i - 1]![0];
      snap(say("knapsack-01.trace", { item: i - 1, weight: items[i - 1]![0], left: c }), "trace", null, [], i - 1);
    }
    path.push([i - 1, c]);
  }
  chosen.reverse();
  snap(say("knapsack-01.done", { best: best[n]![capacity]!, num: chosen.length }), null);
  r.steps[r.steps.length - 1]!.result = [best[n]![capacity]!, ...chosen];
  return r.steps;
}

export const knapsack01: TracerDef = {
  id: "knapsack-01",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 3, max_len: 13, min: 1, max: 20 },
  input: {
    kind: "codec",
    codec: {
      fields: ["capacity", "items"],
      format: (input) => [String(input[0] ?? ""), itemsOf(input).map(([w, v]) => `${w} ${v}`).join(", ")],
      parse: (texts) => {
        const cap = tokens(texts[0] ?? "");
        if ("error" in cap) return cap;
        if (cap.values.length !== 1) return fail("one_number", { field: "capacity" });
        const out = [cap.values[0]!];
        for (const chunk of (texts[1] ?? "").split(/[,;\n]+/).map((s) => s.trim()).filter((s) => s !== "")) {
          const t = tokens(chunk);
          if ("error" in t) return t;
          if (t.values.length !== 2) return fail("item_pair", { item: chunk });
          out.push(...t.values);
        }
        return { values: out };
      },
      check: (input): InputError | null => {
        if (input[0]! < 1 || input[0]! > MAX_CAPACITY) return { code: "custom", key: "capacity_range", params: { max: MAX_CAPACITY } };
        if ((input.length - 1) % 2 !== 0) return { code: "custom", key: "item_pair", params: { item: "" } };
        const items = itemsOf(input);
        if (items.length < 1 || items.length > MAX_ITEMS) return { code: "custom", key: "item_count", params: { min: 1, max: MAX_ITEMS } };
        if (items.some(([w]) => w > 9)) return { code: "custom", key: "weight_range", params: { min: 1, max: 9 } };
        return null;
      },
    },
  },
  presets: {
    "greedy-fails": () => enc(10, [[5, 10], [4, 7], [6, 12], [3, 6]]),
    random: (next) => enc(int(next, 5, MAX_CAPACITY), Array.from({ length: int(next, 2, MAX_ITEMS) }, () => [int(next, 1, 9), int(next, 1, 20)] as [number, number])),
    "one-heavy": () => enc(8, [[8, 15], [3, 6], [4, 7], [2, 3]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "compare", "sorted", "swap"],
  roleLabels: { key: "current", compare: "read", sorted: "filled", swap: "taken" },
  vars: ["item", "capacity"],
  trace,
};
