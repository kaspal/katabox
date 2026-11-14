// Shared bits of the searching topics (ADR 0029): their input limits and target picking.

import type { LearnInput } from "@/bindings";

/** Target first, then 1 to 16 values from 1 to 99 (topic.toml `[input]`). */
export const SEARCH_INPUT: LearnInput = { min_len: 2, max_len: 17, min: 1, max: 99 };

/** A target that is in the values about two times in three, otherwise one that is not
 * (a value between two others when possible), so presets show both outcomes. */
export function pickSearchTarget(values: number[], next: () => number): number {
  if (next() < 0.67) return values[Math.floor(next() * values.length)]!;
  const taken = new Set(values);
  for (let tries = 0; tries < 50; tries++) {
    const v = 1 + Math.floor(next() * 99);
    if (!taken.has(v)) return v;
  }
  return values[0]!;
}
