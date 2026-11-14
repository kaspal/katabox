// The topic's current input and its trace (ADR 0029). Inputs are checked against the
// topic's limits before the tracer runs; a trace over MAX_STEPS is refused too.
import { useMemo, useState } from "react";
import type { LearnInput } from "@/bindings";
import { parseCustom, preset, type InputError, type PresetId } from "./inputs";
import { MAX_STEPS, type Step, type TracerDef } from "./types";

export type TraceError = InputError | { code: "steps"; max: number };

export type TraceResult = { steps: Step[] } | { error: TraceError };

export function runTrace(def: TracerDef, values: readonly number[], lim: LearnInput): TraceResult {
  if (values.length < lim.min_len || values.length > lim.max_len) {
    return { error: { code: "count", min: lim.min_len, max: lim.max_len } };
  }
  const bad = values.find((v) => v < lim.min || v > lim.max);
  if (bad !== undefined) return { error: { code: "range", value: bad, min: lim.min, max: lim.max } };
  const steps = def.trace(values);
  if (steps.length > MAX_STEPS) return { error: { code: "steps", max: MAX_STEPS } };
  return { steps };
}

export function useTrace(def: TracerDef, lim: LearnInput, initial: number[]) {
  const [values, setValues] = useState<number[]>(initial);
  /** Bumped on every applied input, so the player resets even for equal values. */
  const [version, setVersion] = useState(0);
  const [error, setError] = useState<TraceError | null>(null);
  const [seed, setSeed] = useState(1);
  const result = useMemo(() => runTrace(def, values, lim), [def, values, lim]);
  const steps = "steps" in result ? result.steps : [];

  const apply = (next: number[]) => {
    const r = runTrace(def, next, lim);
    if ("error" in r) {
      setError(r.error);
      return false;
    }
    setError(null);
    setValues(next);
    setVersion((v) => v + 1);
    return true;
  };

  return {
    values,
    steps,
    version,
    error,
    applyPreset: (id: PresetId) => {
      // A new seed each time, so "Random" really changes.
      const s = seed + 1;
      setSeed(s);
      return apply(preset(id, Date.now() % 100_000 + s, initial.length, lim));
    },
    applyCustom: (text: string) => {
      const r = parseCustom(text, lim);
      if ("error" in r) {
        setError(r.error);
        return false;
      }
      return apply(r.values);
    },
    clearError: () => setError(null),
  };
}
