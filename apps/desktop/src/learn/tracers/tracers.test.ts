// Tracers (ADR 0029): golden step sequences checked by hand, markers in sync with both
// samples, narration keys complete in English and Spanish, stable item identity.
import { describe, expect, it } from "vitest";
import en from "@/i18n/locales/en.json";
import es from "@/i18n/locales/es.json";
import { sampleSource } from "@/test/fakeLearn";
import { parseSample } from "../engine/markers";
import { caseInputs } from "../engine/cases";
import { MAX_STEPS, type Step } from "../engine/types";
import { TRACERS } from ".";

const lineOf = (id: string, ext: "py" | "ts") => parseSample(sampleSource(id, ext)).lineOf;
const lines = (steps: Step[], map: Record<string, number>) =>
  steps.map((s) => (s.marker === null ? null : (map[s.marker] ?? -1)));

/** For [3, 1, 2]: the marker behind each step and its line in sample.py / sample.ts,
 * worked out by hand from the samples. */
const GOLDEN: Record<string, { markers: (string | null)[]; py: (number | null)[]; ts: (number | null)[] }> = {
  "quick-sort": {
    markers: [null, "pivot", "compare", "compare", "swap", "place", "base", "base", null],
    py: [null, 10, 13, 13, 14, 16, 2, 2, null],
    ts: [null, 9, 12, 12, 13, 17, 2, 2, null],
  },
  "bubble-sort": {
    markers: [null, "pass", "compare", "swap", "compare", "swap", "check", "pass", "compare", "check", null],
    py: [null, 3, 6, 7, 6, 7, 9, 3, 6, 9, null],
    ts: [null, 3, 6, 7, 6, 7, 11, 3, 6, 11, null],
  },
  "insertion-sort": {
    markers: [null, "take", "compare", "shift", "insert", "take", "compare", "shift", "compare", "insert", null],
    py: [null, 3, 5, 6, 8, 3, 5, 6, 5, 8, null],
    ts: [null, 3, 5, 6, 9, 3, 5, 6, 5, 9, null],
  },
};

function lookup(obj: unknown, key: string): unknown {
  return key.split(".").reduce<unknown>((o, k) => (o as Record<string, unknown> | undefined)?.[k], obj);
}
const params = (text: string) => [...text.matchAll(/\{\{(\w+)\}\}/g)].map((m) => m[1]).sort();

describe.each(Object.values(TRACERS))("$id tracer", (def) => {
  it("matches the hand-checked golden sequence and lines in both samples", () => {
    const steps = def.trace([3, 1, 2]);
    const g = GOLDEN[def.id]!;
    expect(steps.map((s) => s.marker)).toEqual(g.markers);
    expect(lines(steps, lineOf(def.id, "py"))).toEqual(g.py);
    expect(lines(steps, lineOf(def.id, "ts"))).toEqual(g.ts);
  });

  it("marks exactly its markers once in each sample", () => {
    for (const ext of ["py", "ts"] as const) {
      expect(Object.keys(lineOf(def.id, ext)).sort(), ext).toEqual([...def.markers].sort());
    }
  });

  it("ends sorted, emits only known markers and keys, and stays under the step cap", () => {
    for (const { input } of [...caseInputs(), { input: [5, 5, 5, 5] }, { input: [2, 1] }]) {
      const steps = def.trace(input);
      expect(steps.length).toBeLessThanOrEqual(MAX_STEPS);
      expect(steps[steps.length - 1]!.items.map((it) => it.value)).toEqual([...input].sort((a, b) => a - b));
      for (const s of steps) {
        if (s.marker !== null) expect(def.markers).toContain(s.marker);
        expect(def.narrationKeys).toContain(s.narration.key);
      }
    }
  });

  it("the largest input stays well under the step cap", () => {
    const worst = Array.from({ length: 16 }, (_, k) => 99 - k); // reversed, 16 values
    expect(def.trace(worst).length).toBeLessThan(MAX_STEPS / 2);
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

  it("keeps each value's identity, equal values included", () => {
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
