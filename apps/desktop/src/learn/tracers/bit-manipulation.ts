// Bit manipulation basics. Mirrors content/learn/foundations/bit-manipulation/sample.{py,ts}:
// AND, OR, XOR and both shifts on two bytes, then counting x's 1 bits by clearing the
// lowest one (x & (x - 1)) until none are left.
import { Recorder } from "../engine/recorder";
import type { BitRow, NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"bit-manipulation">;
const KEYS = [
  "bit-manipulation.ready",
  "bit-manipulation.and",
  "bit-manipulation.or",
  "bit-manipulation.xor",
  "bit-manipulation.left",
  "bit-manipulation.right",
  "bit-manipulation.clear",
  "bit-manipulation.count",
  "bit-manipulation.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["and", "or", "xor", "left", "right", "clear", "count"] as const;

function trace(input: readonly number[]): Step[] {
  const [x0, y] = [input[0]!, input[1]!];
  const r = new Recorder([]);
  const results: number[] = [];
  const row = (label: string, value: number, extra: Partial<BitRow> = {}): BitRow => ({ label, value, ...extra });
  const snap = (
    narration: ReturnType<typeof say>,
    marker: string | null,
    bits: BitRow[],
    vars: { x: number; count: number | null },
  ) => r.snap({ narration, marker, bits, vars: { x: vars.x, y, count: vars.count } });

  snap(say("bit-manipulation.ready", { x: x0, y }), null, [row("x", x0), row("y", y)], { x: x0, count: null });
  const binary: [string, string, (a: number, b: number) => number][] = [
    ["and", "x & y", (a, b) => a & b],
    ["or", "x | y", (a, b) => a | b],
    ["xor", "x ^ y", (a, b) => a ^ b],
  ];
  for (const [marker, label, op] of binary) {
    const v = op(x0, y);
    results.push(v);
    r.comparisons++;
    snap(
      say(`bit-manipulation.${marker}` as (typeof KEYS)[number], { x: x0, y, result: v }),
      marker,
      [row("x", x0), row("y", y), row(label, v, { role: "sorted" })],
      { x: x0, count: null },
    );
  }
  const left = x0 << 1;
  results.push(left);
  r.comparisons++;
  snap(say("bit-manipulation.left", { x: x0, result: left }), "left", [row("x", x0), row("x << 1", left, { role: "sorted", shift: -1 })], {
    x: x0,
    count: null,
  });
  const right = x0 >> 1;
  results.push(right);
  r.comparisons++;
  snap(say("bit-manipulation.right", { x: x0, result: right }), "right", [row("x", x0), row("x >> 1", right, { role: "sorted", shift: 1 })], {
    x: x0,
    count: null,
  });
  let x = x0;
  let count = 0;
  while (x !== 0) {
    const before = x;
    x &= x - 1;
    r.comparisons++;
    count++;
    snap(
      say("bit-manipulation.clear", { before, minus: before - 1, after: x, num: count }),
      "clear",
      [row("x", before), row("x − 1", before - 1), row("x & (x − 1)", x, { role: "sorted" })],
      { x: before, count: count - 1 },
    );
  }
  results.push(count);
  snap(say("bit-manipulation.count", { x: x0, num: count }), "count", [row("x₀", x0), row("x", 0, { role: "sorted" })], { x, count });
  r.snap({
    narration: say("bit-manipulation.done", { ops: r.comparisons }),
    marker: null,
    bits: [row("x", x0), row("y", y)],
    vars: { x, y, count },
    result: results,
  });
  return r.steps;
}

export const bitManipulation: TracerDef = {
  id: "bit-manipulation",
  subject: "foundations",
  scene: "bits",
  limits: { min_len: 2, max_len: 2, min: 0, max: 255 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "comparisons", label: "operations" }],
  legend: ["sorted"],
  roleLabels: { sorted: "result" },
  vars: ["x", "y", "count"],
  trace,
};
