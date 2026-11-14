// Tracers (ADR 0029): golden step sequences checked by hand, markers in sync with both
// samples, outputs equal to an independent reference, narration keys complete in English
// and Spanish, limits equal to topic.toml, stable item identity.
import { describe, expect, it } from "vitest";
import en from "@/i18n/locales/en.json";
import es from "@/i18n/locales/es.json";
import { sampleSource, topicLimits } from "@/test/fakeLearn";
import { GOLDEN, REFERENCE, WORST } from "@/test/learnReference";
import { parseSample } from "../engine/markers";
import { caseInputs } from "../engine/cases";
import { checkInput } from "../engine/inputs";
import { MAX_STEPS, outputOf, type Step, type TracerDef } from "../engine/types";
import { TRACERS } from ".";

const lineOf = (def: TracerDef, ext: "py" | "ts") => parseSample(sampleSource(def.subject, def.id, ext)).lineOf;
const lines = (steps: Step[], map: Record<string, number>) =>
  steps.map((s) => (s.marker === null ? null : (map[s.marker] ?? -1)));

function lookup(obj: unknown, key: string): unknown {
  return key.split(".").reduce<unknown>((o, k) => (o as Record<string, unknown> | undefined)?.[k], obj);
}
const params = (text: string) => [...text.matchAll(/\{\{(\w+)\}\}/g)].map((m) => m[1]).sort();

/** Every case input, plus the worst case and a few edge inputs that fit the topic. */
function inputs(def: TracerDef): number[][] {
  const extra: number[][] = [];
  if (def.input.kind === "values") extra.push([5, 5, 5, 5], [2, 1]);
  const all = [...caseInputs(def).map((c) => c.input), ...extra, ...(WORST[def.id] ? [WORST[def.id]!] : [])];
  return all.filter((xs) => checkInput(def, xs) === null);
}

describe.each(Object.values(TRACERS))("$id tracer", (def) => {
  it("matches the hand-checked golden sequence and lines in both samples", () => {
    const g = GOLDEN[def.id];
    expect(g, "add a golden run to learnReference.ts").toBeDefined();
    const steps = def.trace(g!.input);
    expect(steps.map((s) => s.marker)).toEqual(g!.markers);
    expect(lines(steps, lineOf(def, "py"))).toEqual(g!.py);
    expect(lines(steps, lineOf(def, "ts"))).toEqual(g!.ts);
  });

  it("marks exactly its markers once in each sample", () => {
    for (const ext of ["py", "ts"] as const) {
      expect(Object.keys(lineOf(def, ext)).sort(), ext).toEqual([...def.markers].sort());
    }
  });

  it("has the limits of its topic.toml", () => {
    expect(def.limits).toEqual(topicLimits(def.subject, def.id));
  });

  it("outputs the reference, emits only known markers and keys, and stays under the step cap", () => {
    const reference = REFERENCE[def.id];
    expect(reference, "add a reference to learnReference.ts").toBeDefined();
    for (const input of inputs(def)) {
      const steps = def.trace(input);
      expect(steps.length, `${input}`).toBeLessThanOrEqual(MAX_STEPS);
      expect(outputOf(steps), `${input}`).toEqual(reference!(input));
      for (const s of steps) {
        if (s.marker !== null) expect(def.markers).toContain(s.marker);
        expect(def.narrationKeys).toContain(s.narration.key);
      }
    }
  });

  it("the hardest inputs stay well under the step cap", () => {
    const worst =
      WORST[def.id] ??
      (def.input.kind === "values"
        ? // Descending from the top of the range, wrapping inside it for short ranges.
          Array.from({ length: def.limits.max_len }, (_, k) => def.limits.max - (k % (def.limits.max - def.limits.min + 1)))
        : null);
    if (worst) expect(def.trace(worst).length).toBeLessThan(MAX_STEPS / 2);
  });

  it("has every narration key in English and Spanish with the same parameters", () => {
    for (const key of def.narrationKeys) {
      const e = lookup(en.learn.narration, key);
      const s = lookup(es.learn.narration, key);
      expect(typeof e, `en ${key}`).toBe("string");
      expect(typeof s, `es ${key}`).toBe("string");
      expect(params(s as string), key).toEqual(params(e as string));
    }
  });

  it.runIf(def.subject === "sorting" && def.input.kind === "values")("keeps each value's identity, equal values included", () => {
    const input = [7, 3, 7, 3, 7];
    const steps = def.trace(input);
    for (const s of steps) {
      // Every step holds each original item exactly once.
      expect(s.items.map((it) => it.id).sort()).toEqual([0, 1, 2, 3, 4]);
      for (const it of s.items) expect(it.value).toBe(input[it.id]);
    }
  });
});

describe("equal values swapping", () => {
  it("swaps two equal-valued items by id, so each slides to the other's place", () => {
    // Quick sort on [5, 5, 1]: pivot 1, nothing smaller, the pivot swaps with the first 5.
    const steps = TRACERS["quick-sort"]!.trace([5, 5, 1]);
    const place = steps.find((s) => s.marker === "place")!;
    expect(place.items.map((it) => it.id)).toEqual([2, 1, 0]);
    // Bubble sort on [6, 6, 2]: the second 6 (id 1) passes 2 while the first stays put.
    const bubble = TRACERS["bubble-sort"]!.trace([6, 6, 2]);
    const swap = bubble.find((s) => s.marker === "swap")!;
    expect(swap.items.map((it) => it.id)).toEqual([0, 2, 1]);
  });
});

describe("Lomuto pitfall", () => {
  it("reversed input costs about n²/2 comparisons, random far fewer", () => {
    const qs = TRACERS["quick-sort"]!;
    const last = (xs: number[]) => qs.trace(xs).at(-1)!.counters.comparisons;
    const reversed = Array.from({ length: 16 }, (_, k) => 90 - k * 5);
    expect(last(reversed)).toBe((16 * 15) / 2);
    expect(last([38, 27, 43, 3, 9, 82, 10, 55, 21, 64, 47, 12, 70, 5, 33, 60])).toBeLessThan(80);
  });
});
