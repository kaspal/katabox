// Stack. Mirrors content/learn/linear/stack/sample.{py,ts}: push puts a value on top, pop
// takes the top value off (−1 when empty), peek reads it without removing it. The values
// returned by pop and peek collect in the output row.
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"stack">;
const KEYS = [
  "stack.ready",
  "stack.push",
  "stack.pop",
  "stack.pop_empty",
  "stack.peek",
  "stack.peek_empty",
  "stack.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["push", "pop", "peek"] as const;
export const OPS: OpSpec[] = [
  { name: "push", args: 1 },
  { name: "pop", args: 0 },
  { name: "peek", args: 0 },
];
const codec = opsCodec(OPS);

/** Ops as stored: [code, arg] pairs. */
const enc = (ops: [string, number?][]) => ops.flatMap(([name, v]) => [OPS.findIndex((o) => o.name === name) + 1, v ?? 0]);

function trace(input: readonly number[]): Step[] {
  const ops = decodeOps(input, OPS, codec.width);
  const r = new Recorder([]);
  const stack: Item[] = [];
  const out: Item[] = [];
  let next = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, top: Role | null = null, outRole = false) => {
    const roles: Record<number, Role> = {};
    if (top && stack.length > 0) roles[stack.length - 1] = top;
    r.snap({
      narration,
      marker,
      slots: {
        layout: "stack",
        label: "stack",
        cells: stack.map((it) => ({ ...it })),
        roles,
        pointers: stack.length > 0 ? [{ label: "top", at: stack.length - 1, tone: "key" }] : [],
      },
      aux: { label: "output", cells: out.map((it) => ({ ...it })), roles: outRole ? { [out.length - 1]: "swap" } : {} },
      vars: { size: stack.length, top: stack.length > 0 ? stack[stack.length - 1]!.value : null },
    });
  };
  snap(say("stack.ready", { n: ops.length }), null);
  for (const op of ops) {
    if (op.name === "push") {
      stack.push({ id: next++, value: op.args[0]! });
      r.writes++;
      snap(say("stack.push", { value: op.args[0]!, size: stack.length }), "push", "swap");
    } else if (op.name === "pop") {
      r.moves++;
      if (stack.length === 0) {
        out.push({ id: next++, value: -1 });
        snap(say("stack.pop_empty"), "pop", null, true);
      } else {
        const it = stack.pop()!;
        out.push({ id: next++, value: it.value });
        snap(say("stack.pop", { value: it.value, size: stack.length }), "pop", null, true);
      }
    } else {
      r.comparisons++;
      if (stack.length === 0) {
        out.push({ id: next++, value: -1 });
        snap(say("stack.peek_empty"), "peek", null, true);
      } else {
        const v = stack[stack.length - 1]!.value;
        out.push({ id: next++, value: v });
        snap(say("stack.peek", { value: v }), "peek", "compare", true);
      }
    }
  }
  r.snap({ ...r.steps[r.steps.length - 1]!, narration: say("stack.done", { count: out.length }), marker: null, result: out.map((it) => it.value) });
  return r.steps;
}

export const stack: TracerDef = {
  id: "stack",
  subject: "linear",
  scene: "slots",
  limits: { min_len: 2, max_len: 24, min: 0, max: 99 },
  input: { kind: "codec", codec },
  presets: {
    random: (next) => {
      const ops: [string, number?][] = [];
      const n = int(next, 6, 10);
      let size = 0;
      for (let k = 0; k < n; k++) {
        const roll = next();
        if (size === 0 || roll < 0.5) {
          ops.push(["push", int(next, 1, 99)]);
          size++;
        } else if (roll < 0.8) {
          ops.push(["pop"]);
          size--;
        } else ops.push(["peek"]);
      }
      return enc(ops);
    },
    "last-in-first-out": () => enc([["push", 10], ["push", 20], ["push", 30], ["pop"], ["pop"], ["push", 40], ["peek"], ["pop"], ["pop"]]),
    underflow: () => enc([["push", 7], ["pop"], ["pop"], ["peek"], ["push", 3], ["peek"]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "pushes" },
    { slot: "moves", label: "pops" },
    { slot: "comparisons", label: "peeks" },
  ],
  legend: ["swap", "compare"],
  roleLabels: { swap: "pushed", compare: "peeked" },
  vars: ["size", "top"],
  trace,
};
