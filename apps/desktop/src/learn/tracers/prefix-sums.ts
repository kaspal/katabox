// Prefix sums. Mirrors content/learn/foundations/prefix-sums/sample.{py,ts}: p[i + 1] =
// p[i] + a[i] fills the prefix row (drawn between the values: p[i] is the sum of everything
// left of position i), then one range query reads two prefix cells. The query covers
// positions n / 4 to 3n / 4 (the sample check makes the same call).
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"prefix-sums">;
const KEYS = [
  "prefix-sums.ready",
  "prefix-sums.add",
  "prefix-sums.query",
  "prefix-sums.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["add", "query"] as const;

/** The query the sample check asks for (also in topic.toml's `[sample]` calls). */
export const queryOf = (n: number): [number, number] => [Math.floor(n / 4), Math.floor((3 * n) / 4)];

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const n = values.length;
  const p: (Item | null)[] = Array.from({ length: n + 1 }, () => null);
  const sums: number[] = [0];
  p[0] = { id: 1000, value: 0 };
  const aux = (roles: Record<number, Role> = {}) => ({ label: "prefix" as const, cells: p.map((c) => (c ? { ...c } : null)), roles, offset: -0.5 });

  r.snap({ narration: say("prefix-sums.ready", { n }), marker: null, aux: aux(), vars: { i: null, "p[i]": 0, lo: null, hi: null } });
  for (let i = 0; i < n; i++) {
    const next = sums[i]! + r.value(i);
    sums.push(next);
    p[i + 1] = { id: 1001 + i, value: next };
    r.moves++;
    r.writes++;
    r.snap({
      roles: { [i]: "compare" },
      pointers: [{ label: "i", at: i, tone: "i" }],
      narration: say("prefix-sums.add", { i, before: sums[i]!, value: r.value(i), sum: next, next: i + 1 }),
      marker: "add",
      aux: aux({ [i]: "compare", [i + 1]: "swap" }),
      vars: { i, "p[i]": sums[i]!, lo: null, hi: null },
    });
  }
  const [lo, hi] = queryOf(n);
  const answer = sums[hi + 1]! - sums[lo]!;
  r.comparisons++;
  r.snap({
    range: [lo, hi],
    narration: say("prefix-sums.query", { lo, hi, upper: hi + 1, top: sums[hi + 1]!, bottom: sums[lo]!, answer }),
    marker: "query",
    aux: aux({ [lo]: "compare", [hi + 1]: "compare" }),
    vars: { i: null, "p[i]": null, lo, hi },
  });
  r.snap({
    range: [lo, hi],
    narration: say("prefix-sums.done", { n, answer, additions: r.moves }),
    marker: null,
    aux: aux(),
    vars: { i: null, "p[i]": null, lo, hi },
    result: [...sums, answer],
  });
  return r.steps;
}

export const prefixSums: TracerDef = {
  id: "prefix-sums",
  subject: "foundations",
  scene: "array",
  limits: { min_len: 1, max_len: 12, min: -9, max: 30 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "moves", label: "additions" },
    { slot: "comparisons", label: "queries" },
  ],
  legend: ["compare", "swap"],
  roleLabels: { compare: "reading", swap: "writing" },
  vars: ["i", "p[i]", "lo", "hi"],
  trace,
};
