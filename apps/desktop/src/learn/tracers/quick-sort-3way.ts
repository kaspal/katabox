// Quick sort with a three-way partition (Dijkstra's "Dutch flag"). Mirrors
// content/learn/sorting/quick-sort-3way/sample.{py,ts}: the first value is the pivot;
// lt..i-1 hold values equal to it, lo..lt-1 the smaller ones, gt+1..hi the bigger ones.
// The whole equal block is in its final place after one pass, so many equal values cost
// nothing more (Hoare's scheme is in the theory).
import { SORT_INPUT } from "../engine/cases";
import { Recorder } from "../engine/recorder";
import { sortCounters, type NarrationKey, type Pointer, type Role, type Step, type TracerDef } from "../engine/types";

type Key = NarrationKey<"quick-sort-3way">;
const KEYS = [
  "quick-sort-3way.ready",
  "quick-sort-3way.base",
  "quick-sort-3way.pivot",
  "quick-sort-3way.compare_less",
  "quick-sort-3way.compare_greater",
  "quick-sort-3way.compare_equal",
  "quick-sort-3way.less",
  "quick-sort-3way.greater",
  "quick-sort-3way.equal",
  "quick-sort-3way.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["base", "pivot", "compare", "less", "greater", "equal"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const ptrs = (lt: number, i: number, gt: number): Pointer[] => [
    { label: "lt", at: lt, tone: "pivot" },
    ...(i <= gt ? [{ label: "i", at: i, tone: "i" } as Pointer] : []),
    { label: "gt", at: gt, tone: "j" },
  ];
  const equal = (lt: number, i: number): Record<number, Role> => {
    const roles: Record<number, Role> = {};
    for (let p = lt; p < i; p++) roles[p] = "pivot";
    return roles;
  };
  const none = { lo: null, hi: null, pivot: null, lt: null, i: null, gt: null };
  r.snap({ range: [0, n - 1], narration: say("quick-sort-3way.ready"), marker: null, vars: none });

  const sort = (lo: number, hi: number) => {
    if (lo >= hi) {
      if (lo === hi) {
        r.settle(lo);
        r.snap({ range: [lo, hi], narration: say("quick-sort-3way.base", { lo }), marker: "base", vars: { ...none, lo, hi } });
      }
      return;
    }
    const pivot = r.value(lo);
    const range: [number, number] = [lo, hi];
    let lt = lo;
    let i = lo + 1;
    let gt = hi;
    const vars = () => ({ lo, hi, pivot, lt, i, gt });
    r.snap({
      roles: { [lo]: "pivot" },
      pointers: ptrs(lt, i, gt),
      range,
      narration: say("quick-sort-3way.pivot", { lo, hi, pivot }),
      marker: "pivot",
      vars: vars(),
    });
    while (i <= gt) {
      const value = r.value(i);
      r.comparisons++;
      if (value >= pivot) r.comparisons++; // not smaller: the elif compares again
      const kind = value < pivot ? "less" : value > pivot ? "greater" : "equal";
      r.snap({
        roles: { ...equal(lt, i), [i]: "compare" },
        pointers: ptrs(lt, i, gt),
        range,
        narration: say(`quick-sort-3way.compare_${kind}`, { value, pivot }),
        marker: "compare",
        vars: vars(),
      });
      if (kind === "less") {
        if (lt !== i) {
          r.swap(lt, i);
          r.moves++;
          r.writes += 2;
        }
        r.snap({
          roles: { ...equal(lt + 1, i + 1), [lt]: "swap" },
          pointers: ptrs(lt, i, gt),
          range,
          narration: say("quick-sort-3way.less", { value, lt }),
          marker: "less",
          vars: vars(),
        });
        lt++;
        i++;
      } else if (kind === "greater") {
        if (i !== gt) {
          r.swap(i, gt);
          r.moves++;
          r.writes += 2;
        }
        r.snap({
          roles: { ...equal(lt, i), [gt]: "swap", [i]: "compare" },
          pointers: ptrs(lt, i, gt),
          range,
          narration: say("quick-sort-3way.greater", { value, gt }),
          marker: "greater",
          vars: vars(),
        });
        gt--;
      } else {
        r.snap({
          roles: equal(lt, i + 1),
          pointers: ptrs(lt, i, gt),
          range,
          narration: say("quick-sort-3way.equal", { value }),
          marker: "equal",
          vars: vars(),
        });
        i++;
      }
    }
    for (let p = lt; p <= gt; p++) r.settle(p);
    sort(lo, lt - 1);
    sort(gt + 1, hi);
  };
  sort(0, n - 1);
  r.settleAll();
  r.snap({ narration: say("quick-sort-3way.done", { comparisons: r.comparisons, moves: r.moves, n }), marker: null });
  return r.steps;
}

export const quickSort3Way: TracerDef = {
  id: "quick-sort-3way",
  subject: "sorting",
  compareGroup: "sort",
  scene: "array",
  limits: SORT_INPUT,
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: sortCounters("swaps"),
  legend: ["pivot", "compare", "swap", "sorted", "out"],
  roleLabels: { pivot: "equal_pivot" },
  vars: ["lo", "hi", "pivot", "lt", "i", "gt"],
  trace,
};
