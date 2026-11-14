// The inputs every topic's samples are checked on (ADR 0029): each preset, then 20 seeded
// random inputs of varying length (count topics: every n in range). `pnpm learn:cases`
// writes them, with what the samples must print, to
// content/learn/<subject>/<topic>/cases.json; kb-pack runs both samples on them in the
// sandbox.
import type { LearnInput } from "@/bindings";
import { PRESETS, presetInput } from "./inputs";
import { outputOf, type TracerDef } from "./types";

/** The limits sorting topics use (topic.toml `[input]`; kb-pack checks cases fit). */
export const SORT_INPUT: LearnInput = { min_len: 2, max_len: 16, min: 1, max: 99 };
export const RANDOM_CASES = 20;

export type Case = { name: string; input: number[]; output: number[] };

export function caseInputs(def: TracerDef): { name: string; input: number[] }[] {
  const lim = def.limits;
  if (def.input.kind === "count") {
    return Array.from({ length: lim.max - lim.min + 1 }, (_, k) => ({
      name: `n-${lim.min + k}`,
      input: [lim.min + k],
    }));
  }
  const out: { name: string; input: number[] }[] = PRESETS.map((p) => ({
    name: p,
    input: presetInput(def, p, 1, Math.min(10, lim.max_len)),
  }));
  for (let k = 1; k <= RANDOM_CASES; k++) {
    const len = lim.min_len + ((k * 7) % (lim.max_len - lim.min_len + 1));
    out.push({ name: `random-${k}`, input: presetInput(def, "random", 1000 + k, len) });
  }
  return out;
}

export function buildCases(def: TracerDef): Case[] {
  return caseInputs(def).map(({ name, input }) => ({ name, input, output: outputOf(def.trace(input)) }));
}

/** cases.json text: one case per line, so diffs stay readable. */
export function casesJson(cases: Case[]): string {
  const lines = cases.map((c) => `    ${JSON.stringify(c)}`.replace(/,"/g, ', "').replace(/":/g, '": '));
  return `{\n  "cases": [\n${lines.join(",\n")}\n  ]\n}\n`;
}
