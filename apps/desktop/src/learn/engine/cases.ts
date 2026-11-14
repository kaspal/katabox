// The inputs every topic's samples are checked on (ADR 0029): each preset, then 20 seeded
// random inputs of varying length. `pnpm learn:cases` writes them, with the tracer's
// final state, to content/learn/<subject>/<topic>/cases.json; kb-pack runs both samples
// on them in the sandbox.
import type { LearnInput } from "@/bindings";
import { PRESETS, preset } from "./inputs";
import type { TracerDef } from "./types";

/** The limits every Phase 0 topic uses (topic.toml `[input]`; kb-pack checks cases fit). */
export const SORT_INPUT: LearnInput = { min_len: 2, max_len: 16, min: 1, max: 99 };
export const RANDOM_CASES = 20;

export type Case = { name: string; input: number[]; output: number[] };

export function caseInputs(lim: LearnInput = SORT_INPUT): { name: string; input: number[] }[] {
  const out: { name: string; input: number[] }[] = PRESETS.map((p) => ({ name: p, input: preset(p, 1, 10, lim) }));
  for (let k = 1; k <= RANDOM_CASES; k++) {
    const len = lim.min_len + ((k * 7) % (lim.max_len - lim.min_len + 1));
    out.push({ name: `random-${k}`, input: preset("random", 1000 + k, len, lim) });
  }
  return out;
}

export function buildCases(def: TracerDef, lim: LearnInput = SORT_INPUT): Case[] {
  return caseInputs(lim).map(({ name, input }) => {
    const steps = def.trace(input);
    const last = steps[steps.length - 1]!;
    return { name, input, output: last.items.map((it) => it.value) };
  });
}

/** cases.json text: one case per line, so diffs stay readable. */
export function casesJson(cases: Case[]): string {
  const lines = cases.map((c) => `    ${JSON.stringify(c)}`.replace(/,"/g, ', "').replace(/":/g, '": '));
  return `{\n  "cases": [\n${lines.join(",\n")}\n  ]\n}\n`;
}
