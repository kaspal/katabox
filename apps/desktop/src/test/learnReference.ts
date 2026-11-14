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
