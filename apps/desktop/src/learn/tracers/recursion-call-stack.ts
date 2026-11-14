// Recursion and the call stack: the sum of a range, split in halves. Mirrors
// content/learn/foundations/recursion-call-stack/sample.{py,ts}. Every call pushes a frame
// and a tree node over its range; a base case reads one value; a combine adds the two
// halves' results and pops the frame.
import { Recorder } from "../engine/recorder";
import { placeTrees } from "../engine/tree";
import type { NarrationKey, Role, StackEntry, Step, TracerDef, TreeNode, VarValue } from "../engine/types";

type Key = NarrationKey<"recursion-call-stack">;
const KEYS = [
  "recursion-call-stack.ready",
  "recursion-call-stack.call",
  "recursion-call-stack.check_leaf",
  "recursion-call-stack.leaf",
  "recursion-call-stack.split",
  "recursion-call-stack.combine",
  "recursion-call-stack.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params });

export const MARKERS = ["check", "leaf", "split", "combine"] as const;

type Call = { id: number; lo: number; hi: number; mid: number | null; left: number | null; right: number | null };

function trace(values: readonly number[]): Step[] {
  const r = new Recorder(values);
  const nodes = new Map<number, TreeNode>();
  const stack: Call[] = [];
  let next = 0;

  const label = (lo: number, hi: number) => (lo === hi ? `${lo}` : `${lo}–${hi}`);
  const snap = (
    narration: ReturnType<typeof say>,
    marker: string | null,
    extra: { roles?: Record<number, Role>; range?: [number, number] | null } = {},
  ) => {
    const top = stack[stack.length - 1];
    const tree = [...nodes.values()].map((n) => {
      const onStack = stack.some((c) => c.id === n.id);
      const role: Role = top && n.id === top.id ? "key" : onStack ? "pivot" : n.result !== undefined ? "sorted" : "idle";
      return { ...n, role };
    });
    const frames: StackEntry[] = stack.map((c) => ({
      id: c.id,
      text: `total(${c.lo}, ${c.hi})`,
      role: c === top ? "key" : "pivot",
    }));
    const vars: Record<string, VarValue> = top
      ? { lo: top.lo, hi: top.hi, mid: top.mid, left: top.left, right: top.right }
      : { lo: null, hi: null, mid: null, left: null, right: null };
    r.moves = stack.length;
    r.snap({
      roles: extra.roles,
      range: extra.range === undefined ? (top ? [top.lo, top.hi] : null) : extra.range,
      narration,
      marker,
      vars,
      tree,
      stack: frames,
    });
  };

  const total = (lo: number, hi: number, parent: number | null): number => {
    const call: Call = { id: next++, lo, hi, mid: null, left: null, right: null };
    nodes.set(call.id, { id: call.id, parent, text: label(lo, hi), role: "key" });
    stack.push(call);
    r.comparisons++; // calls
    snap(say(lo === hi ? "recursion-call-stack.check_leaf" : "recursion-call-stack.call", { lo, hi, depth: stack.length }), "check");
    let result: number;
    if (lo === hi) {
      result = r.value(lo);
      nodes.get(call.id)!.result = `${result}`;
      snap(say("recursion-call-stack.leaf", { at: lo, value: result }), "leaf", { roles: { [lo]: "key" } });
    } else {
      const mid = Math.floor((lo + hi) / 2);
      call.mid = mid;
      snap(say("recursion-call-stack.split", { lo, hi, mid, next: mid + 1 }), "split");
      call.left = total(lo, mid, call.id);
      call.right = total(mid + 1, hi, call.id);
      result = call.left + call.right;
      nodes.get(call.id)!.result = `${result}`;
      snap(
        say("recursion-call-stack.combine", { left: call.left, right: call.right, result, lo, hi }),
        "combine",
      );
    }
    stack.pop();
    return result;
  };

  snap(say("recursion-call-stack.ready", { n: values.length }), null, { range: null });
  const sum = total(0, values.length - 1, null);
  snap(say("recursion-call-stack.done", { sum, calls: r.comparisons }), null, { range: null });
  r.steps[r.steps.length - 1]!.result = [sum];
  return placeTrees(r.steps);
}

export const recursionCallStack: TracerDef = {
  id: "recursion-call-stack",
  subject: "foundations",
  scene: "recursion",
  limits: { min_len: 1, max_len: 8, min: 1, max: 99 },
  input: { kind: "values" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "calls" },
    { slot: "moves", label: "depth" },
  ],
  legend: ["key", "pivot", "sorted"],
  roleLabels: { key: "running", pivot: "waiting", sorted: "returned" },
  vars: ["lo", "hi", "mid", "left", "right"],
  trace,
};
