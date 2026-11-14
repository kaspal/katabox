// Hash table with open addressing (linear probing). Mirrors
// content/learn/hashing/open-addressing/sample.{py,ts}: one key per slot; from the home
// slot (key mod 8), step right (wrapping) past occupied slots until the key or an empty
// slot. Deleting leaves a tombstone, so later searches still probe past it.
import { decodeOps, opsCodec, type OpSpec } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { Item, NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"open-addressing">;
const KEYS = [
  "open-addressing.ready",
  "open-addressing.hash",
  "open-addressing.probe",
  "open-addressing.place",
  "open-addressing.found",
  "open-addressing.not_found",
  "open-addressing.tombstone",
  "open-addressing.delete_missing",
  "open-addressing.done",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams<number>) => ({ key, params });

export const MARKERS = ["hash", "probe", "place", "found", "tombstone"] as const;
export const OPS: OpSpec[] = [
  { name: "insert", args: 1 },
  { name: "find", args: 1 },
  { name: "delete", args: 1 },
];
const codec = opsCodec(OPS);
const M = 8;
const EMPTY = -1;
const DELETED = -2;
const enc = (ops: [string, number][]) => ops.flatMap(([name, v]) => [OPS.findIndex((o) => o.name === name) + 1, v]);

function trace(input: readonly number[]): Step[] {
  const ops = decodeOps(input, OPS, codec.width);
  const r = new Recorder([]);
  const slots: number[] = new Array<number>(M).fill(EMPTY);
  const ids: number[] = new Array<number>(M).fill(-1);
  let nextId = 0;
  const out: Item[] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, key: number | null, probe: number[], outRole = false) =>
    r.snap({
      narration,
      marker,
      hash: {
        mode: "open",
        buckets: slots.map((v, k) => (v === EMPTY ? [] : [{ id: v === DELETED ? 10_000 + k : ids[k]!, value: v }])),
        probe: [...probe],
        key,
        home: key === null ? null : key % M,
      },
      aux: { label: "found", cells: out.map((c) => ({ ...c })), roles: outRole ? { [out.length - 1]: "swap" } : {} },
      vars: { key, slot: probe.length ? probe[probe.length - 1]! : null, probes: probe.length },
    });
  const findSlot = (key: number, probe: number[]) => {
    let i = key % M;
    probe.push(i);
    snap(say("open-addressing.hash", { key, home: i }), "hash", key, probe);
    while (slots[i] !== EMPTY && slots[i] !== key) {
      const was = slots[i]!;
      i = (i + 1) % M;
      probe.push(i);
      r.comparisons++;
      snap(say("open-addressing.probe", { key, was, slot: i }), "probe", key, probe);
    }
    return i;
  };

  snap(say("open-addressing.ready", { m: M }), null, null, []);
  for (const op of ops) {
    const key = op.args[0]!;
    const probe: number[] = [];
    const i = findSlot(key, probe);
    if (op.name === "insert") {
      if (slots[i] !== key) ids[i] = nextId++;
      slots[i] = key;
      r.writes++;
      snap(say("open-addressing.place", { key, slot: i, probes: probe.length }), "place", key, probe);
    } else if (op.name === "find") {
      const hit = slots[i] === key ? 1 : 0;
      out.push({ id: nextId++, value: hit });
      snap(say(hit ? "open-addressing.found" : "open-addressing.not_found", { key, slot: i }), "found", key, probe, true);
    } else if (slots[i] === key) {
      slots[i] = DELETED;
      r.moves++;
      snap(say("open-addressing.tombstone", { key, slot: i }), "tombstone", key, probe);
    } else {
      snap(say("open-addressing.delete_missing", { key }), null, key, probe);
    }
  }
  snap(say("open-addressing.done", { finds: out.length, probes: r.comparisons }), null, null, []);
  r.steps[r.steps.length - 1]!.result = out.map((c) => c.value);
  return r.steps;
}

export const openAddressing: TracerDef = {
  id: "open-addressing",
  subject: "hashing",
  scene: "hash",
  limits: { min_len: 2, max_len: 24, min: 1, max: 99 },
  input: {
    kind: "codec",
    codec: {
      ...codec,
      // At most 7 inserts keep an empty slot, so every probe stops (tombstones are not reused).
      check: (input) => {
        const e = codec.check!(input);
        if (e) return e;
        const inserts = decodeOps(input, OPS, codec.width).filter((o) => o.name === "insert").length;
        return inserts > M - 1 ? { code: "custom", key: "too_many_inserts", params: { max: M - 1 } } : null;
      },
    },
  },
  presets: {
    random: (next) => {
      const ops: [string, number][] = [];
      const keys: number[] = [];
      const n = int(next, 6, 10);
      let inserts = 0;
      for (let k = 0; k < n; k++) {
        const roll = next();
        if ((roll < 0.5 || keys.length === 0) && inserts < 6) {
          const key = int(next, 1, 99);
          keys.push(key);
          ops.push(["insert", key]);
          inserts++;
        } else if (roll < 0.8) ops.push(["find", next() < 0.7 && keys.length ? keys[int(next, 0, keys.length - 1)]! : int(next, 1, 99)]);
        else if (keys.length) ops.push(["delete", keys[int(next, 0, keys.length - 1)]!]);
      }
      return enc(ops.length ? ops : [["insert", 5]]);
    },
    cluster: () => enc([["insert", 3], ["insert", 11], ["insert", 19], ["insert", 4], ["find", 19], ["find", 27]]),
    tombstone: () => enc([["insert", 2], ["insert", 10], ["insert", 18], ["delete", 10], ["find", 18], ["find", 10]]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "inserts" },
    { slot: "comparisons", label: "probes" },
    { slot: "moves", label: "deletes" },
  ],
  legend: ["compare", "swap"],
  roleLabels: { compare: "probing", swap: "written" },
  vars: ["key", "slot", "probes"],
  trace,
};
