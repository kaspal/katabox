// Fibonacci two ways. Mirrors content/learn/dynamic-programming/fibonacci-memo-table/sample.{py,ts}:
// first top-down, a recursion that stores each answer in a memo and returns a stored one
// at once (a "hit"); then bottom-up, a table filled from 0 to n where each cell reads the
// two before it. The two rows of the table fill with the same numbers, in different
// orders. Output: fib(n), then the bottom-up table.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"fibonacci-memo-table">;
const KEYS = [
  "fibonacci-memo-table.ready",
  "fibonacci-memo-table.call",
  "fibonacci-memo-table.hit",
  "fibonacci-memo-table.store",
  "fibonacci-memo-table.table",
  "fibonacci-memo-table.fill",
  "fibonacci-memo-table.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["hit", "store", "fill"] as const;

function trace(input: readonly number[]): Step[] {
  const n = input[0]!;
  const r = new Recorder([]);
  const memo: (number | null)[] = new Array(n + 1).fill(null);
  const table: (number | null)[] = new Array(n + 1).fill(null);
  const stack: number[] = [];
  let frameId = 0;
  const frames: number[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: [number, number] | null = null, deps: [number, number][] = []) =>
    r.snap({
      narration,
      marker,
      dp: { label: "memo_table", colLabels: Array.from({ length: n + 1 }, (_, k) => String(k)), rowLabels: ["memo", "table"], cells: [[...memo], [...table]], hot, deps },
      slots: { layout: "row", label: "stack", cells: stack.map((k, j) => ({ id: frames[j]!, value: k, text: `f(${k})` })), roles: stack.length ? { [stack.length - 1]: "key" } : {} },
      vars: { calls: r.comparisons },
    });
  const fib = (k: number): number => {
    stack.push(k);
    frames.push(frameId++);
    r.comparisons++;
    snap(say("fibonacci-memo-table.call", { k }), null);
    let result: number;
    if (memo[k] !== null) {
      snap(say("fibonacci-memo-table.hit", { k, value: memo[k]! }), "hit", [0, k]);
      result = memo[k]!;
    } else {
      result = k < 2 ? k : fib(k - 1) + fib(k - 2);
      memo[k] = result;
      r.writes++;
      snap(say("fibonacci-memo-table.store", { k, value: result }), "store", [0, k], k >= 2 ? [[0, k - 1], [0, k - 2]] : []);
    }
    stack.pop();
    frames.pop();
    return result;
  };

  snap(say("fibonacci-memo-table.ready", { n }), null);
  const top = fib(n);
  table[0] = 0;
  if (n > 0) table[1] = 1;
  snap(say("fibonacci-memo-table.table", { calls: r.comparisons }), null);
  for (let i = 2; i <= n; i++) {
    table[i] = table[i - 1]! + table[i - 2]!;
    r.moves++;
    snap(say("fibonacci-memo-table.fill", { i, value: table[i]! }), "fill", [1, i], [
      [1, i - 1],
      [1, i - 2],
    ]);
  }
  snap(say("fibonacci-memo-table.done", { n, value: top, calls: r.comparisons, fills: r.moves }), null);
  r.steps[r.steps.length - 1]!.result = [top, ...table.map((v) => v!)];
  return r.steps;
}

export const fibonacciMemoTable: TracerDef = {
  id: "fibonacci-memo-table",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 1, max_len: 1, min: 1, max: 12 },
  input: { kind: "count" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "calls" },
    { slot: "writes", label: "stored" },
    { slot: "moves", label: "filled" },
  ],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "read", sorted: "filled" },
  vars: ["calls"],
  trace,
};
