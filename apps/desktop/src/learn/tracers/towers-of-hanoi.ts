// Towers of Hanoi. Mirrors content/learn/foundations/towers-of-hanoi/sample.{py,ts}: every
// call is a frame on the stack (including the n = 0 calls that just return); every move
// carries disk n from its source peg to its target peg.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, StackEntry, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"towers-of-hanoi">;
const KEYS = [
  "towers-of-hanoi.ready",
  "towers-of-hanoi.call",
  "towers-of-hanoi.call_one",
  "towers-of-hanoi.empty",
  "towers-of-hanoi.move",
  "towers-of-hanoi.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["check", "move"] as const;

type Frame = { id: number; n: number; source: number; target: number; spare: number };

function trace(input: readonly number[]): Step[] {
  const disks = input[0]!;
  const r = new Recorder([]);
  const pegs: number[][] = [Array.from({ length: disks }, (_, k) => disks - k), [], []];
  const stack: Frame[] = [];
  const moves: number[] = [];
  let next = 0;

  const snap = (narration: ReturnType<typeof say>, marker: string | null, active: number | null = null) => {
    const top = stack[stack.length - 1];
    const frames: StackEntry[] = stack.map((f) => ({
      id: f.id,
      text: `hanoi(${f.n}, ${f.source}→${f.target})`,
      role: f === top ? "key" : "pivot",
    }));
    r.snap({
      narration,
      marker,
      pegs: pegs.map((p) => [...p]),
      stack: frames,
      active,
      vars: top
        ? { n: top.n, source: top.source, target: top.target, spare: top.spare, moves: moves.length / 2 }
        : { n: null, source: null, target: null, spare: null, moves: moves.length / 2 },
    });
  };

  const hanoi = (n: number, source: number, target: number, spare: number) => {
    const f: Frame = { id: next++, n, source, target, spare };
    stack.push(f);
    r.comparisons++; // calls
    if (n === 0) {
      snap(say("towers-of-hanoi.empty"), "check");
      stack.pop();
      return;
    }
    snap(
      n === 1
        ? say("towers-of-hanoi.call_one", { source, target })
        : say("towers-of-hanoi.call", { n, source, target, spare, smaller: n - 1 }),
      "check",
    );
    hanoi(n - 1, source, spare, target);
    const disk = pegs[source - 1]!.pop()!;
    pegs[target - 1]!.push(disk);
    moves.push(source, target);
    r.moves++;
    snap(say("towers-of-hanoi.move", { disk, source, target, count: r.moves }), "move", disk);
    hanoi(n - 1, spare, target, source);
    stack.pop();
  };

  snap(say("towers-of-hanoi.ready", { n: disks, minimum: 2 ** disks - 1 }), null);
  hanoi(disks, 1, 3, 2);
  snap(say("towers-of-hanoi.done", { n: disks, moves: r.moves, calls: r.comparisons }), null);
  r.steps[r.steps.length - 1]!.result = moves;
  return r.steps;
}

export const towersOfHanoi: TracerDef = {
  id: "towers-of-hanoi",
  subject: "foundations",
  scene: "hanoi",
  limits: { min_len: 1, max_len: 1, min: 1, max: 5 },
  input: { kind: "count" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "moves", label: "moves" },
    { slot: "comparisons", label: "calls" },
  ],
  legend: ["swap"],
  roleLabels: { swap: "moving" },
  vars: ["n", "source", "target", "spare", "moves"],
  trace,
};
