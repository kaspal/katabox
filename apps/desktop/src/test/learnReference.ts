// What each Learn topic's samples must print for an input (ADR 0029), written
// independently of the tracers: Vitest checks every tracer's output (and so cases.json)
// against these, and kb-pack checks the samples against cases.json in the sandbox.
const sorted = (xs: readonly number[]) => [...xs].sort((a, b) => a - b);

const log2 = (n: number) => Math.floor(Math.log2(n)) + 1;
const sum = (xs: readonly number[]) => xs.reduce((a, b) => a + b, 0);

/** Hanoi by the iterative rule (not the recursion): move m goes from peg (m & (m - 1)) % 3
 * to ((m | (m - 1)) + 1) % 3, which ends on the third peg for odd n and the second for even
 * n; relabel so the tower always ends on peg 3. */
function hanoiMoves(n: number): number[] {
  const label = n % 2 === 1 ? [1, 2, 3] : [1, 3, 2];
  const out: number[] = [];
  for (let m = 1; m < 2 ** n; m++) out.push(label[(m & (m - 1)) % 3]!, label[((m | (m - 1)) + 1) % 3]!);
  return out;
}

export const REFERENCE: Record<string, (input: readonly number[]) => number[]> = {
  "big-o-growth": ([n]) => [1, log2(n!), n!, n! * log2(n!), n! * n!, 2 ** n!],
  "recursion-call-stack": (xs) => [sum(xs)],
  "towers-of-hanoi": ([n]) => hanoiMoves(n!),
  "bit-manipulation": ([x, y]) => [x! & y!, x! | y!, x! ^ y!, x! * 2, Math.floor(x! / 2), [...x!.toString(2)].filter((c) => c === "1").length],
  "prefix-sums": (xs) => {
    const p = xs.map((_, k) => sum(xs.slice(0, k)));
    p.push(sum(xs));
    const lo = Math.floor(xs.length / 4);
    const hi = Math.floor((3 * xs.length) / 4);
    return [...p, sum(xs.slice(lo, hi + 1))];
  },
  "dynamic-array": (xs) => {
    const cap = 2 ** Math.ceil(Math.log2(xs.length));
    return [cap, cap - 1, ...xs];
  },
  "bubble-sort": sorted,
  "insertion-sort": sorted,
  "quick-sort": sorted,
  "selection-sort": sorted,
  "shell-sort": sorted,
  "merge-sort": sorted,
  "quick-sort-3way": sorted,
  "heap-sort": sorted,
  "counting-sort": sorted,
  "radix-sort": sorted,
  "bucket-sort": sorted,
  "tim-sort": sorted,
  quickselect: ([k, ...values]) => [sorted(values)[k! - 1]!],
  "linear-search": ([t, ...a]) => [a.indexOf(t!)],
  "binary-search": ([t, ...a]) => {
    let lo = 0;
    let hi = a.length - 1;
    while (lo <= hi) {
      const m = (lo + hi) >> 1;
      if (a[m] === t) return [m];
      if (a[m]! < t!) lo = m + 1;
      else hi = m - 1;
    }
    return [-1];
  },
  "lower-upper-bound": ([t, ...a]) => [a.filter((v) => v < t!).length, a.filter((v) => v <= t!).length],
  "binary-search-answer": ([days, ...w]) => {
    // The first capacity that fits, by trying every one in turn.
    const need = (c: number) => {
      let d = 1;
      let load = 0;
      for (const x of w) {
        if (load + x > c) {
          d++;
          load = 0;
        }
        load += x;
      }
      return d;
    };
    for (let c = Math.max(...w); ; c++) if (need(c) <= days!) return [c];
  },
  "exponential-search": ([t, ...a]) => {
    if (a[0] === t) return [0];
    let bound = 1;
    while (bound < a.length && a[bound]! < t!) bound *= 2;
    for (let lo = bound >> 1, hi = Math.min(bound, a.length - 1); lo <= hi; ) {
      const m = (lo + hi) >> 1;
      if (a[m] === t) return [m];
      if (a[m]! < t!) lo = m + 1;
      else hi = m - 1;
    }
    return [-1];
  },
  "two-pointers": ([t, ...a]) => {
    for (let i = 0, j = a.length - 1; i < j; ) {
      const s = a[i]! + a[j]!;
      if (s === t) return [i, j];
      if (s < t!) i++;
      else j--;
    }
    return [-1, -1];
  },
  "sliding-window": ([limit, ...a]) => {
    // Every window, by brute force: for each end, the longest window that fits; ties keep
    // the earlier window, as the sample does (it only takes strictly longer ones).
    let best = 0;
    let start = 0;
    for (let hi = 0; hi < a.length; hi++) {
      for (let lo = 0; lo <= hi; lo++) {
        const sum = a.slice(lo, hi + 1).reduce((x, y) => x + y, 0);
        if (sum <= limit!) {
          if (hi - lo + 1 > best) {
            best = hi - lo + 1;
            start = lo;
          }
          break;
        }
      }
    }
    return [best, start];
  },
  "monotonic-stack": (a) => a.map((v, i) => a.slice(i + 1).find((x) => x > v) ?? -1),
};

/** Hand-checked golden runs: the input, then the marker behind each step and its line in
 * sample.py / sample.ts, worked out by hand from the samples. */
export type Golden = {
  input: number[];
  markers: (string | null)[];
  py: (number | null)[];
  ts: (number | null)[];
};

export const GOLDEN: Record<string, Golden> = {
  "linear-search": {
    input: [5, 3, 5, 1],
    markers: [null, "check", "check", "found"],
    py: [null, 3, 3, 4],
    ts: [null, 3, 3, 4],
  },
  "binary-search": {
    input: [5, 1, 3, 5, 7],
    markers: [null, "mid", "compare", "mid", "found"],
    py: [null, 4, 7, 4, 6],
    ts: [null, 5, 9, 5, 7],
  },
  "lower-upper-bound": {
    input: [3, 1, 3, 3, 5],
    markers: [null, "lower", "lower", "lower", "lower-done", "upper", "upper", "upper-done", null],
    py: [null, 5, 5, 5, 9, 16, 16, 20, null],
    ts: [null, 6, 6, 6, 12, 20, 20, 26, null],
  },
  "binary-search-answer": {
    input: [2, 3, 1, 2],
    markers: [null, "try", "check", "try", "check", "answer"],
    py: [null, 14, 15, 14, 15, 19],
    ts: [null, 18, 19, 18, 19, 25],
  },
  "exponential-search": {
    input: [5, 1, 3, 5, 7, 9],
    markers: [null, "first", "double", "double", "range", "mid", "compare", "mid", "found"],
    py: [null, 2, 5, 5, 7, 9, 12, 9, 11],
    ts: [null, 2, 4, 4, 7, 10, 14, 10, 12],
  },
  "two-pointers": {
    input: [10, 1, 3, 5, 7],
    markers: [null, "sum", "move", "sum", "found"],
    py: [null, 4, 7, 4, 6],
    ts: [null, 5, 9, 5, 7],
  },
  "sliding-window": {
    input: [6, 4, 3, 2],
    markers: [null, "grow", "check", "grow", "shrink", "check", "grow", "check", null],
    py: [null, 5, 9, 5, 7, 9, 5, 9, null],
    ts: [null, 7, 12, 7, 9, 12, 7, 12, null],
  },
  "monotonic-stack": {
    input: [2, 1, 3],
    markers: [null, "push", "compare", "push", "compare", "pop", "compare", "pop", "push", null],
    py: [null, 7, 5, 7, 5, 6, 5, 6, 7, null],
    ts: [null, 8, 5, 8, 5, 6, 5, 6, 8, null],
  },
  "selection-sort": {
    input: [3, 1, 2],
    markers: [null, "start", "compare", "new-min", "compare", "swap", "start", "compare", "new-min", "swap", null],
    py: [null, 4, 6, 7, 6, 8, 4, 6, 7, 8, null],
    ts: [null, 4, 6, 7, 6, 10, 4, 6, 7, 10, null],
  },
  "shell-sort": {
    input: [3, 1, 2],
    markers: [null, "gap", "take", "compare", "shift", "insert", "take", "compare", "shift", "compare", "insert", null],
    py: [null, 3, 5, 7, 8, 10, 5, 7, 8, 7, 10, null],
    ts: [null, 3, 5, 7, 8, 11, 5, 7, 8, 7, 11, null],
  },
  "merge-sort": {
    input: [3, 1, 2],
    markers: [null, "split", "split", "base", "base", "copy", "pick", "take-right", "pick", "take-left", "base", "copy", "pick", "take-left", "pick", "take-right", "pick", "take-left", null],
    py: [null, 4, 4, 2, 2, 11, 14, 18, 14, 15, 2, 11, 14, 15, 14, 18, 14, 15, null],
    ts: [null, 3, 3, 2, 2, 10, 14, 18, 14, 15, 2, 10, 14, 15, 14, 18, 14, 15, null],
  },
  "quick-sort-3way": {
    input: [3, 1, 2],
    markers: [null, "pivot", "compare", "less", "compare", "less", "pivot", "compare", "greater", "base", null],
    py: [null, 4, 7, 8, 7, 8, 4, 7, 12, 2, null],
    ts: [null, 3, 8, 9, 8, 9, 3, 8, 13, 2, null],
  },
  "heap-sort": {
    input: [3, 1, 2],
    markers: [null, "build", "pick", "compare", null, "extract", "pick", "compare", "extract", null],
    py: [null, 4, 13, 15, null, 6, 13, 15, 6, null],
    ts: [null, 4, 15, 18, null, 7, 15, 18, 7, null],
  },
  "counting-sort": {
    input: [3, 1, 2],
    markers: [null, "count", "count", "count", "write", "write", "write", null],
    py: [null, 4, 4, 4, 8, 8, 8, null],
    ts: [null, 4, 4, 4, 9, 9, 9, null],
  },
  "radix-sort": {
    input: [3, 1, 2],
    markers: [null, "pass", "distribute", "distribute", "distribute", "collect", "collect", "collect", null],
    py: [null, 3, 6, 6, 6, 10, 10, 10, null],
    ts: [null, 2, 5, 5, 5, 10, 10, 10, null],
  },
  "bucket-sort": {
    input: [3, 1, 2],
    markers: [null, "scatter", "scatter", "scatter", "sort", "gather", "gather", "gather", "sort", "sort", "sort", "sort", null],
    py: [null, 4, 4, 4, 7, 9, 9, 9, 7, 7, 7, 7, null],
    ts: [null, 4, 4, 4, 8, 10, 10, 10, 8, 8, 8, 8, null],
  },
  "tim-sort": {
    input: [3, 1, 2],
    markers: [null, "run", "take", "compare", "shift", "insert", "take", "compare", "shift", "compare", "insert", null],
    py: [null, 7, 19, 21, 22, 24, 19, 21, 22, 21, 24, null],
    ts: [null, 6, 19, 21, 22, 25, 19, 21, 22, 21, 25, null],
  },
  quickselect: {
    input: [1, 3, 1, 2],
    markers: [null, "pivot", "compare", "compare", "swap", "place", "side", "pivot", "place", "found", null],
    py: [null, 14, 17, 17, 18, 20, 7, 14, 20, 5, null],
    ts: [null, 16, 19, 19, 20, 24, 7, 16, 24, 6, null],
  },
  "big-o-growth": {
    input: [2],
    markers: [null, "constant", "log", "log", "log", "linear", "nlogn", "quadratic", "exponential", null],
    py: [null, 2, 10, 10, 10, 18, 27, 36, 43, null],
    ts: [null, 2, 9, 9, 9, 17, 26, 36, 45, null],
  },
  "recursion-call-stack": {
    input: [5, 7, 2],
    markers: [null, "check", "split", "check", "split", "check", "leaf", "check", "leaf", "combine", "check", "leaf", "combine", null],
    py: [null, 2, 4, 2, 4, 2, 3, 2, 3, 7, 2, 3, 7, null],
    ts: [null, 2, 5, 2, 5, 2, 3, 2, 3, 8, 2, 3, 8, null],
  },
  "towers-of-hanoi": {
    input: [1],
    markers: [null, "check", "check", "move", "check", null],
    py: [null, 2, 2, 5, 2, null],
    ts: [null, 2, 2, 4, 2, null],
  },
  "bit-manipulation": {
    input: [6, 3],
    markers: [null, "and", "or", "xor", "left", "right", "clear", "clear", "count", null],
    py: [null, 2, 3, 4, 5, 6, 9, 9, 11, null],
    ts: [null, 2, 3, 4, 5, 6, 9, 9, 12, null],
  },
  "prefix-sums": {
    input: [4, 1, 3],
    markers: [null, "add", "add", "add", "query", null],
    py: [null, 4, 4, 4, 9, null],
    ts: [null, 4, 4, 4, 10, null],
  },
  "dynamic-array": {
    input: [8, 5, 9],
    markers: [null, "check", "write", "check", "copy", "write", "check", "copy", "copy", "write", null],
    py: [null, 7, 14, 7, 11, 14, 7, 11, 11, 14, null],
    ts: [null, 7, 16, 7, 11, 16, 7, 11, 11, 16, null],
  },
  "quick-sort": {
    input: [3, 1, 2],
    markers: [null, "pivot", "compare", "compare", "swap", "place", "base", "base", null],
    py: [null, 10, 13, 13, 14, 16, 2, 2, null],
    ts: [null, 9, 12, 12, 13, 17, 2, 2, null],
  },
  "bubble-sort": {
    input: [3, 1, 2],
    markers: [null, "pass", "compare", "swap", "compare", "swap", "check", "pass", "compare", "check", null],
    py: [null, 3, 6, 7, 6, 7, 9, 3, 6, 9, null],
    ts: [null, 3, 6, 7, 6, 7, 11, 3, 6, 11, null],
  },
  "insertion-sort": {
    input: [3, 1, 2],
    markers: [null, "take", "compare", "shift", "insert", "take", "compare", "shift", "compare", "insert", null],
    py: [null, 3, 5, 6, 8, 3, 5, 6, 5, 8, null],
    ts: [null, 3, 5, 6, 9, 3, 5, 6, 5, 9, null],
  },
};

/** Inputs that make each topic work hardest (the step cap must hold with room). */
export const WORST: Record<string, number[]> = {
  "big-o-growth": [16],
  "towers-of-hanoi": [5],
};
