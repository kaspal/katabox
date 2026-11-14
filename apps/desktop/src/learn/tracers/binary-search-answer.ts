// Binary search on the answer: the smallest daily capacity that ships every parcel, in
// order, within D days. Mirrors content/learn/searching/binary-search-answer/sample.{py,ts}:
// the answer lies between the heaviest parcel and the total; "does capacity c fit in D
// days?" is no, no, …, yes, yes as c grows, so binary search finds the first yes. Each
// tried capacity shows its day groups, labelled by day number.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Pointer, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"binary-search-answer">;
const KEYS = [
  "binary-search-answer.ready",
  "binary-search-answer.try",
  "binary-search-answer.fits",
  "binary-search-answer.too_small",
  "binary-search-answer.answer",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["try", "check", "answer"] as const;

/** Day of each parcel when every day carries at most `capacity`, as the sample counts. */
export function dayGroups(weights: readonly number[], capacity: number): number[] {
  const day: number[] = [];
  let d = 1;
  let load = 0;
  for (const w of weights) {
    if (load + w > capacity) {
      d++;
      load = 0;
    }
    load += w;
    day.push(d);
  }
  return day;
}

function trace(input: readonly number[]): Step[] {
  const days = input[0]!;
  const weights = input.slice(1);
  const r = new Recorder(weights);
  let lo = Math.max(...weights);
  let hi = weights.reduce((s, w) => s + w, 0);
  const groups = (capacity: number) => {
    const day = dayGroups(weights, capacity);
    const roles: Record<number, Role> = {};
    const pointers: Pointer[] = [];
    day.forEach((d, p) => {
      if (d % 2 === 1) roles[p] = "compare";
      if (p === 0 || day[p - 1] !== d) pointers.push({ label: `d${d}`, at: p, tone: d % 2 === 1 ? "i" : "j" });
    });
    return { roles, pointers, used: day[day.length - 1]! };
  };
  r.snap({ narration: say("binary-search-answer.ready", { days, lo, hi, n: weights.length }), marker: null, vars: { days, lo, hi, mid: null, need: null } });
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const g = groups(mid);
    r.comparisons++;
    r.snap({ roles: g.roles, pointers: g.pointers, narration: say("binary-search-answer.try", { lo, hi, mid }), marker: "try", vars: { days, lo, hi, mid, need: null } });
    const fits = g.used <= days;
    r.snap({
      roles: g.roles,
      pointers: g.pointers,
      narration: say(fits ? "binary-search-answer.fits" : "binary-search-answer.too_small", { mid, need: g.used, days }),
      marker: "check",
      vars: { days, lo, hi, mid, need: g.used },
    });
    if (fits) hi = mid;
    else lo = mid + 1;
  }
  const g = groups(lo);
  r.snap({
    roles: g.roles,
    pointers: g.pointers,
    narration: say("binary-search-answer.answer", { capacity: lo, need: g.used, days, tries: r.comparisons }),
    marker: "answer",
    vars: { days, lo, hi, mid: null, need: g.used },
    result: [lo],
  });
  return r.steps;
}

export const binarySearchAnswer: TracerDef = {
  id: "binary-search-answer",
  subject: "searching",
  scene: "array",
  limits: { min_len: 2, max_len: 13, min: 1, max: 20 },
  input: {
    kind: "target",
    targetLabel: "days",
    pickTarget: (values, next) => 1 + Math.floor(next() * Math.max(1, Math.ceil(values.length / 2))),
    checkTarget: (d, values) => (d >= 1 && d <= values.length ? null : { min: 1, max: values.length }),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [{ slot: "comparisons", label: "capacities_tried" }],
  legend: ["compare", "idle"],
  roleLabels: { compare: "odd_day", idle: "even_day" },
  vars: ["days", "lo", "hi", "mid", "need"],
  trace,
};
