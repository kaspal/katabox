// The topic's current input and its trace (ADR 0029). Inputs are checked against the
// topic's limits and shape before the tracer runs; a trace over MAX_STEPS is refused too.
import { useMemo, useState } from "react";
import { checkInput, parseCustom, presetInput, type InputError, type PresetId } from "./inputs";
import { MAX_STEPS, type Step, type TracerDef } from "./types";

export type TraceError = InputError | { code: "steps"; max: number };

export type TraceResult = { steps: Step[] } | { error: TraceError };

export function runTrace(def: TracerDef, input: readonly number[]): TraceResult {
  const e = checkInput(def, input);
  if (e) return { error: e };
  const steps = def.trace(input);
  if (steps.length > MAX_STEPS) return { error: { code: "steps", max: MAX_STEPS } };
  return { steps };
}

export function useTrace(def: TracerDef, initial: number[]) {
  const [values, setValues] = useState<number[]>(initial);
  /** Bumped on every applied input, so the player resets even for equal values. */
  const [version, setVersion] = useState(0);
  const [error, setError] = useState<TraceError | null>(null);
  const [seed, setSeed] = useState(1);
  const result = useMemo(() => runTrace(def, values), [def, values]);
  const steps = "steps" in result ? result.steps : [];

  const apply = (next: number[]) => {
    const r = runTrace(def, next);
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
    /** The whole flat input (target first for target kinds). */
    values,
    steps,
    version,
    error,
    applyPreset: (id: PresetId) => {
      // A new seed each time, so "Random" really changes.
      const s = seed + 1;
      setSeed(s);
      return apply(presetInput(def, id, (Date.now() % 100_000) + s, initial.length));
    },
    applyCustom: (texts: readonly string[]) => {
      const r = parseCustom(def, texts);
      if ("error" in r) {
        setError(r.error);
        return false;
      }
      return apply(r.values);
    },
    clearError: () => setError(null),
  };
}
