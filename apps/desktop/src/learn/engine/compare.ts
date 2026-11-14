// Compare mode (ADR 0029, Phase 1): two topics of the same subject run side by side on
// one input. Topics compare when they share a `compareGroup` (the input means the same
// thing: values to sort, a value to find) and the current input fits both; one shared
// player drives both, and the shorter run holds its last step. Compare never counts as
// viewing a topic and never saves the "Continue" position.
import { useMemo, useState } from "react";
import { checkInput, parseCustom, presetInput, type PresetId } from "./inputs";
import { runTrace, type TraceError } from "./useTrace";
import type { Step, TracerDef } from "./types";

/** Other topics that can run on `input` next to `def`, in registry order. */
export function comparePartners(def: TracerDef, all: readonly TracerDef[], input: readonly number[]): TracerDef[] {
  if (!def.compareGroup) return [];
  return all.filter((d) => d.id !== def.id && d.subject === def.subject && d.compareGroup === def.compareGroup && checkInput(d, input) === null);
}

/** Whether `def` has any partner at all (for the button), whatever the current input. */
export const canCompare = (def: TracerDef, all: readonly TracerDef[]) =>
  !!def.compareGroup && all.some((d) => d.id !== def.id && d.subject === def.subject && d.compareGroup === def.compareGroup);

/** One input for both topics: a step index for each pane from the shared player. */
export function paneStep(steps: readonly Step[], at: number): { step: Step; finished: boolean } {
  const last = steps.length - 1;
  return { step: steps[Math.min(at, last)]!, finished: at >= last };
}

/** The shared input and both traces; every new input must fit both topics. */
export function useCompareTrace(a: TracerDef, b: TracerDef, initial: number[]) {
  const [values, setValues] = useState<number[]>(initial);
  const [version, setVersion] = useState(0);
  const [error, setError] = useState<TraceError | null>(null);
  const traces = useMemo(() => [runTrace(a, values), runTrace(b, values)] as const, [a, b, values]);
  const steps = traces.map((r) => ("steps" in r ? r.steps : [])) as [Step[], Step[]];

  const apply = (next: number[]) => {
    for (const d of [a, b]) {
      const r = runTrace(d, next);
      if ("error" in r) {
        setError(r.error);
        return false;
      }
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
      // A preset made for one topic may not fit the other (a narrower range): try a few seeds.
      const base = Date.now() % 100_000;
      for (let k = 0; k < 20; k++) {
        const next = presetInput(a, id, base + k, initial.length);
        if (checkInput(b, next) === null) return apply(next);
      }
      return apply(presetInput(a, id, base, initial.length));
    },
    applyCustom: (text: string, target?: string) => {
      const r = parseCustom(a, text, target);
      if ("error" in r) {
        setError(r.error);
        return false;
      }
      return apply(r.values);
    },
    clearError: () => setError(null),
  };
}
