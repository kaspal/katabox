// Heap sort. Mirrors content/learn/sorting/heap-sort/sample.{py,ts}: build a max-heap by
// sifting down every parent from the last one up, then swap the root (the biggest value)
// to the end, shrink the heap by one and sift the new root down, until one value is left.
import { SORT_INPUT } from "../engine/cases";
import { Recorder } from "../engine/recorder";
import { type NarrationKey, type NarrationParams, type Pointer, type Role, sortCounters, type Step, type TracerDef } from "../engine/types";

type Key = NarrationKey<"heap-sort">;
const KEYS = [
  "heap-sort.ready",
  "heap-sort.build",
  "heap-sort.pick",
  "heap-sort.pick_only",
  "heap-sort.compare_stop",
  "heap-sort.compare_swap",
  "heap-sort.swap",
  "heap-sort.heap_ready",
  "heap-sort.extract",
  "heap-sort.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["build", "pick", "compare", "swap", "extract"] as const;

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const ptrs = (root: number, child: number | null): Pointer[] => [
    { label: "root", at: root, tone: "pivot" },
    ...(child === null ? [] : [{ label: "child", at: child, tone: "j" } as Pointer]),
  ];
  r.snap({ heap: n, narration: say("heap-sort.ready", { n }), marker: null, vars: { root: null, child: null, size: n } });

  const siftDown = (start: number, size: number) => {
    let root = start;
    let child = 2 * root + 1;
    while (child < size) {
      const vars = () => ({ root, child, size });
      if (child + 1 < size) {
        r.comparisons++;
        const right = r.value(child + 1) > r.value(child);
        const roles: Record<number, Role> = { [root]: "key", [child]: "compare", [child + 1]: "compare" };
        if (right) child++;
        r.snap({
          heap: size,
          roles,
          pointers: ptrs(root, child),
          narration: say("heap-sort.pick", { child: r.value(child), at: child }),
          marker: "pick",
          vars: vars(),
        });
      } else {
        r.snap({
          heap: size,
          roles: { [root]: "key", [child]: "compare" },
          pointers: ptrs(root, child),
          narration: say("heap-sort.pick_only", { child: r.value(child), at: child }),
          marker: "pick",
          vars: vars(),
        });
      }
      r.comparisons++;
      const stop = r.value(root) >= r.value(child);
      r.snap({
        heap: size,
        roles: { [root]: "key", [child]: "compare" },
        pointers: ptrs(root, child),
        narration: say(stop ? "heap-sort.compare_stop" : "heap-sort.compare_swap", {
          parent: r.value(root),
          child: r.value(child),
        }),
        marker: "compare",
        vars: vars(),
      });
      if (stop) return;
      r.swap(root, child);
      r.moves++;
      r.writes += 2;
      r.snap({
        heap: size,
        roles: { [root]: "swap", [child]: "swap" },
        pointers: ptrs(root, child),
        narration: say("heap-sort.swap", { up: r.value(root), down: r.value(child) }),
        marker: "swap",
        vars: vars(),
      });
      root = child;
      child = 2 * root + 1;
    }
  };

  for (let start = Math.floor(n / 2) - 1; start >= 0; start--) {
    r.snap({
      heap: n,
      roles: { [start]: "key" },
      pointers: ptrs(start, null),
      narration: say("heap-sort.build", { start, value: r.value(start) }),
      marker: "build",
      vars: { root: start, child: null, size: n },
    });
    siftDown(start, n);
  }
  r.snap({ heap: n, roles: { 0: "pivot" }, narration: say("heap-sort.heap_ready", { max: r.value(0) }), marker: null, vars: { root: null, child: null, size: n } });
  for (let end = n - 1; end > 0; end--) {
    const max = r.value(0);
    r.swap(0, end);
    r.moves++;
    r.writes += 2;
    r.settle(end);
    r.snap({
      heap: end,
      roles: { 0: "swap", [end]: "swap" },
      pointers: ptrs(0, null),
      narration: say("heap-sort.extract", { max, end }),
      marker: "extract",
      vars: { root: 0, child: null, size: end },
    });
    siftDown(0, end);
  }
  r.settleAll();
  r.snap({ heap: 0, narration: say("heap-sort.done", { comparisons: r.comparisons, moves: r.moves, n }), marker: null });
  return r.steps;
}

export const heapSort: TracerDef = {
  id: "heap-sort",
  subject: "sorting",
  compareGroup: "sort",
  scene: "heap",
  limits: SORT_INPUT,
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: sortCounters("swaps"),
  legend: ["key", "compare", "swap", "sorted"],
  roleLabels: { key: "parent" },
  vars: ["root", "child", "size"],
  trace,
};
