// Coin change: the fewest coins that make an amount, with unlimited coins of each value.
// Mirrors content/learn/dynamic-programming/coin-change/sample.{py,ts}: fewest[t] for
// every total t from 1 up; the last coin used was some coin c, so fewest[t] is the
// smallest fewest[t − c] + 1. 999 stands for "cannot be made". Output: [the fewest coins
// for the amount] or [-1].
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"coin-change">;
const KEYS = ["coin-change.ready", "coin-change.total", "coin-change.improve", "coin-change.done", "coin-change.impossible"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["improve"] as const;
export const INF = 999;

function trace(input: readonly number[]): Step[] {
  const [amount = 1, ...coins] = input;
  const r = new Recorder([]);
  const fewest: (number | null)[] = [0, ...new Array<number | null>(amount).fill(null)];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: number | null = null, deps: number[] = [], coin: number | null = null) => {
    const roles: Record<number, Role> = {};
    if (coin !== null) roles[coin] = "key";
    r.snap({
      narration,
      marker,
      slots: { layout: "row", label: "coins", cells: coins.map((value, id) => ({ id, value })), roles },
      dp: { label: "fewest", colLabels: Array.from({ length: amount + 1 }, (_, k) => String(k)), cells: [fewest.map((v) => (v === INF ? Infinity : v))], hot: hot === null ? null : [0, hot], deps: deps.map((d) => [0, d]) },
      vars: { total: hot, fewest: hot === null ? null : (fewest[hot] ?? INF) },
    });
  };

  snap(say("coin-change.ready", { amount, kinds: coins.length }), null);
  for (let total = 1; total <= amount; total++) {
    fewest[total] = INF;
    snap(say("coin-change.total", { total }), null, total);
    coins.forEach((coin, j) => {
      r.comparisons++;
      if (coin <= total && fewest[total - coin]! + 1 < fewest[total]!) {
        fewest[total] = fewest[total - coin]! + 1;
        r.writes++;
        snap(say("coin-change.improve", { total, coin, from: total - coin, num: fewest[total]! }), "improve", total, [total - coin], j);
      }
    });
  }
  const answer = fewest[amount]! < INF ? fewest[amount]! : -1;
  snap(answer === -1 ? say("coin-change.impossible", { amount }) : say("coin-change.done", { amount, num: answer }), null);
  r.steps[r.steps.length - 1]!.result = [answer];
  return r.steps;
}

export const coinChange: TracerDef = {
  id: "coin-change",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 2, max_len: 6, min: 1, max: 20 },
  input: {
    kind: "target",
    targetLabel: "amount",
    pickTarget: (_values, next) => int(next, 6, 20),
    checkTarget: (amount) => (amount >= 1 && amount <= 20 ? null : { min: 1, max: 20 }),
  },
  presets: {
    "greedy-fails": () => [12, 1, 6, 9],
    random: (next) => [int(next, 6, 20), ...Array.from({ length: int(next, 1, 4) }, () => int(next, 2, 9))],
    "cannot-make": () => [7, 2, 4],
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "read", sorted: "filled" },
  vars: ["total", "fewest"],
  trace,
};
