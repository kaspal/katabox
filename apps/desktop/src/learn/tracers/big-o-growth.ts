// Big-O growth curves. Mirrors content/learn/foundations/big-o-growth/sample.{py,ts}: six
// loops count their steps for one n. The doubling loop is shown iteration by iteration
// (the doubling is the point); every other loop is one step with its total, and its curve
// joins the chart.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"big-o-growth">;
const KEYS = [
  "big-o-growth.ready",
  "big-o-growth.constant",
  "big-o-growth.log",
  "big-o-growth.log_done",
  "big-o-growth.linear",
  "big-o-growth.nlogn",
  "big-o-growth.quadratic",
  "big-o-growth.exponential",
  "big-o-growth.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["constant", "log", "linear", "nlogn", "quadratic", "exponential"] as const;

function trace(input: readonly number[]): Step[] {
  const n = input[0]!;
  const r = new Recorder([]);
  // Run the sample's loops (the chart draws the closed forms; Vitest checks they agree).
  const loops = [
    () => 1,
    () => {
      let s = 0;
      for (let i = 1; i <= n; i *= 2) s++;
      return s;
    },
    () => {
      let s = 0;
      for (let i = 0; i < n; i++) s++;
      return s;
    },
    () => {
      let s = 0;
      for (let i = 0; i < n; i++) for (let j = 1; j <= n; j *= 2) s++;
      return s;
    },
    () => {
      let s = 0;
      for (let i = 0; i < n; i++) for (let j = 0; j < n; j++) s++;
      return s;
    },
    () => {
      let s = 0;
      for (let k = 0; k < 2 ** n; k++) s++;
      return s;
    },
  ];
  const counts = loops.map((f) => f());
  const chart = (shown: number) => ({ shown, n });
  r.snap({ narration: say("big-o-growth.ready", { n }), marker: null, chart: chart(0), vars: { n, steps: null, i: null } });
  r.snap({
    narration: say("big-o-growth.constant", { n }),
    marker: "constant",
    chart: chart(1),
    vars: { n, steps: 1, i: null },
  });
  let steps = 0;
  for (let i = 1; i <= n; i *= 2) {
    steps++;
    r.snap({
      narration: say("big-o-growth.log", { i, steps, next: i * 2, n }),
      marker: "log",
      chart: chart(1),
      vars: { n, steps, i },
    });
  }
  r.snap({
    narration: say("big-o-growth.log_done", { steps, n }),
    marker: "log",
    chart: chart(2),
    vars: { n, steps, i: null },
  });
  const rest = [
    ["linear", "big-o-growth.linear"],
    ["nlogn", "big-o-growth.nlogn"],
    ["quadratic", "big-o-growth.quadratic"],
    ["exponential", "big-o-growth.exponential"],
  ] as const;
  rest.forEach(([marker, key], k) => {
    const shown = k + 3;
    r.snap({
      narration: say(key, { steps: counts[shown - 1]!, n, log: counts[1]! }),
      marker,
      chart: chart(shown),
      vars: { n, steps: counts[shown - 1]!, i: null },
    });
  });
  r.snap({
    narration: say("big-o-growth.done", { n, fastest: counts[0]!, slowest: counts[5]! }),
    marker: null,
    chart: chart(6),
    vars: { n, steps: null, i: null },
    result: counts,
  });
  return r.steps;
}

export const bigOGrowth: TracerDef = {
  id: "big-o-growth",
  subject: "foundations",
  scene: "growth",
  limits: { min_len: 1, max_len: 1, min: 1, max: 16 },
  input: { kind: "count" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [],
  legend: [],
  vars: ["n", "i", "steps"],
  trace,
};
