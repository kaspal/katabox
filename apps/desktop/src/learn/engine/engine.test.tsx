// The Learn engine (ADR 0029): the player's timing and rules, the keyboard rule, input
// parsing and presets, and marker stripping.
import { act, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { sampleSource } from "@/test/fakeLearn";
import { TRACERS } from "../tracers";
import { comparePartners, paneStep } from "./compare";
import { checkInput, parseCustom, preset, presetInput, PRESETS, rng } from "./inputs";
import type { TracerDef } from "./types";
import { playerKeyAction } from "./keys";
import { parseSample } from "./markers";
import { parseLast, parseViewed } from "./progress";
import { DELAY_MS, usePlayer } from "./usePlayer";

const LIM = { min_len: 2, max_len: 16, min: 1, max: 99 };

describe("usePlayer", () => {
  beforeEach(() => vi.useFakeTimers());
  afterEach(() => vi.useRealTimers());

  const setup = (total = 5, opts: Parameters<typeof usePlayer>[2] = {}) =>
    renderHook(({ total, key }) => usePlayer(total, key, opts), { initialProps: { total, key: "a" } });

  it("plays one step per delay and stops at the end", () => {
    const onPause = vi.fn();
    const { result } = setup(4, { onPause });
    act(() => result.current.play());
    expect(result.current.playing).toBe(true);
    act(() => vi.advanceTimersByTime(DELAY_MS[1] - 1));
    expect(result.current.step).toBe(0);
    act(() => vi.advanceTimersByTime(1));
    expect(result.current.step).toBe(1);
    act(() => vi.advanceTimersByTime(DELAY_MS[1] * 2));
    expect(result.current.step).toBe(3);
    expect(result.current.playing).toBe(false);
    expect(result.current.atEnd).toBe(true);
    expect(onPause).toHaveBeenLastCalledWith(3);
    act(() => vi.advanceTimersByTime(DELAY_MS[1] * 5));
    expect(result.current.step).toBe(3);
  });

  it("pauses, and Play at the end starts over", () => {
    const { result } = setup(3);
    act(() => result.current.play());
    act(() => vi.advanceTimersByTime(DELAY_MS[1]));
    act(() => result.current.pause());
    act(() => vi.advanceTimersByTime(DELAY_MS[1] * 3));
    expect(result.current.step).toBe(1);
    act(() => result.current.seek(2));
    act(() => result.current.play());
    expect(result.current.step).toBe(0);
    expect(result.current.playing).toBe(true);
  });

  it("speed changes the delay without a burst of steps", () => {
    const { result } = setup(20);
    act(() => result.current.setSpeed(4));
    act(() => result.current.play());
    act(() => vi.advanceTimersByTime(DELAY_MS[4] * 3));
    expect(result.current.step).toBe(3);
    act(() => result.current.setSpeed(0.5));
    act(() => vi.advanceTimersByTime(DELAY_MS[0.5] - 1));
    expect(result.current.step).toBe(3);
    act(() => vi.advanceTimersByTime(1));
    expect(result.current.step).toBe(4);
  });

  it("does not drift: 10 steps take 10 delays even when timers fire late", () => {
    const { result } = setup(50);
    act(() => result.current.play());
    for (let k = 0; k < 10; k++) {
      // A late timer: the clock moves on before the callback runs.
      act(() => vi.advanceTimersByTime(DELAY_MS[1] + 30));
    }
    // 10 × 880 ms = 8800 ms ≈ 10.35 delays: lateness is absorbed, not added up.
    expect(result.current.step).toBe(10);
    act(() => vi.advanceTimersByTime(DELAY_MS[1] - 300 + 1));
    expect(result.current.step).toBe(11);
  });

  it("cleans up its timer on unmount and on a new trace", () => {
    const { result, unmount, rerender } = setup(10);
    act(() => result.current.play());
    expect(vi.getTimerCount()).toBe(1);
    rerender({ total: 8, key: "b" });
    expect(result.current.playing).toBe(false);
    expect(result.current.step).toBe(0);
    expect(vi.getTimerCount()).toBe(0);
    act(() => result.current.play());
    expect(vi.getTimerCount()).toBe(1);
    unmount();
    expect(vi.getTimerCount()).toBe(0);
  });

  it("step and scrub pause playback and stay in range", () => {
    const { result } = setup(5);
    act(() => result.current.play());
    act(() => result.current.forward());
    expect(result.current.playing).toBe(false);
    expect(result.current.step).toBe(1);
    act(() => result.current.back());
    act(() => result.current.back());
    expect(result.current.step).toBe(0);
    act(() => result.current.seek(99));
    expect(result.current.step).toBe(4);
    act(() => result.current.restart());
    expect(result.current.step).toBe(0);
  });

  it("marks a drag on the timeline until it ends", () => {
    const { result } = setup(5);
    act(() => result.current.seek(3, { drag: true }));
    expect(result.current.dragging).toBe(true);
    act(() => result.current.endDrag());
    expect(result.current.dragging).toBe(false);
  });

  describe("the viewed rule", () => {
    it("counts reaching the end by playing", () => {
      const onFinish = vi.fn();
      const { result } = setup(3, { onFinish });
      act(() => result.current.play());
      act(() => vi.advanceTimersByTime(DELAY_MS[1] * 2));
      expect(onFinish).toHaveBeenCalledTimes(1);
    });

    it("counts reaching the end by stepping", () => {
      const onFinish = vi.fn();
      const { result } = setup(3, { onFinish });
      act(() => result.current.forward());
      expect(onFinish).not.toHaveBeenCalled();
      act(() => result.current.forward());
      expect(onFinish).toHaveBeenCalledTimes(1);
    });

    it("does not count scrubbing to the end or opening the page", () => {
      const onFinish = vi.fn();
      const { result } = setup(3, { onFinish, initialStep: 2 });
      expect(onFinish).not.toHaveBeenCalled();
      act(() => result.current.restart());
      act(() => result.current.seek(2, { drag: true }));
      act(() => result.current.endDrag());
      expect(onFinish).not.toHaveBeenCalled();
    });
  });
});

describe("playerKeyAction", () => {
  const key = (k: string, target: Element = document.body, extra: Partial<KeyboardEvent> = {}) =>
    playerKeyAction({
      key: k,
      target,
      ctrlKey: false,
      metaKey: false,
      altKey: false,
      shiftKey: false,
      defaultPrevented: false,
      ...extra,
    } as KeyboardEvent);

  it("maps Space, arrows and Home on the page", () => {
    expect(key(" ")).toBe("toggle");
    expect(key("ArrowLeft")).toBe("back");
    expect(key("ArrowRight")).toBe("forward");
    expect(key("Home")).toBe("restart");
    expect(key("a")).toBeNull();
  });

  it("never takes keys from text entry, sliders, selects or tabs", () => {
    for (const html of [
      "<input />",
      "<textarea></textarea>",
      "<select></select>",
      '<div contenteditable="true"></div>',
      '<input type="range" />',
      '<div role="tab"></div>',
    ]) {
      const box = document.createElement("div");
      box.innerHTML = html;
      const el = box.firstElementChild!;
      for (const k of [" ", "ArrowLeft", "ArrowRight", "Home"]) expect(key(k, el), `${html} ${k}`).toBeNull();
    }
  });

  it("leaves Space to a focused button (it activates natively) but still steps with arrows", () => {
    const button = document.createElement("button");
    expect(key(" ", button)).toBeNull();
    expect(key("ArrowRight", button)).toBe("forward");
  });

  it("ignores modified and already handled keys", () => {
    expect(key(" ", document.body, { ctrlKey: true })).toBeNull();
    expect(key("ArrowLeft", document.body, { altKey: true })).toBeNull();
    expect(key(" ", document.body, { defaultPrevented: true })).toBeNull();
  });
});

const SORT = TRACERS["bubble-sort"]!;
/** A made-up target topic: [target, ...sorted values], target 1..99 and in range k ≤ n. */
const TARGET: TracerDef = {
  ...SORT,
  limits: { min_len: 2, max_len: 6, min: 1, max: 99 },
  input: {
    kind: "sorted-target",
    pickTarget: (v, next) => 1 + Math.floor(next() * v.length),
    checkTarget: (t, v) => (t > v.length ? { min: 1, max: v.length } : null) },
};
const COUNT: TracerDef = { ...SORT, limits: { min_len: 1, max_len: 1, min: 1, max: 5 }, input: { kind: "count" } };

describe("input kinds", () => {
  it("puts the target first, and refuses unsorted values or a bad target", () => {
    expect(parseCustom(TARGET, "1, 2, 3", "2")).toEqual({ values: [2, 1, 2, 3] });
    expect(parseCustom(TARGET, "3, 1", "1")).toEqual({ error: { code: "unsorted" } });
    expect(parseCustom(TARGET, "1, 2", "5")).toEqual({ error: { code: "target", min: 1, max: 2 } });
    expect(parseCustom(TARGET, "1, 2", "x")).toEqual({ error: { code: "nan", token: "x" } });
    expect(parseCustom(TARGET, "1, 2", "")).toEqual({ error: { code: "nan", token: "—" } });
    // The count is of the values: the target is one of max_len.
    expect(parseCustom(TARGET, "1 2 3 4 5 6", "1")).toEqual({ error: { code: "count", min: 1, max: 5 } });
  });

  it("presets fit the shape", () => {
    for (const p of PRESETS) {
      const xs = presetInput(TARGET, p, 4, 6);
      expect(xs).toHaveLength(6);
      const values = xs.slice(1);
      expect(values).toEqual([...values].sort((a, b) => a - b));
      expect(checkInput(TARGET, xs)).toBeNull();
    }
    const n = presetInput(COUNT, "random", 9, 1);
    expect(n).toHaveLength(1);
    expect(checkInput(COUNT, n)).toBeNull();
    expect(checkInput(COUNT, [6])).toEqual({ code: "range", value: 6, min: 1, max: 5 });
  });
});

describe("inputs", () => {
  it("parses commas, spaces and semicolons", () => {
    expect(parseCustom(SORT, "3, 1;2  9")).toEqual({ values: [3, 1, 2, 9] });
  });

  it("names the problem instead of clamping", () => {
    expect(parseCustom(SORT, "5")).toEqual({ error: { code: "count", min: 2, max: 16 } });
    expect(parseCustom(SORT, Array(17).fill(1).join(","))).toEqual({ error: { code: "count", min: 2, max: 16 } });
    expect(parseCustom(SORT, "3, x, 4")).toEqual({ error: { code: "nan", token: "x" } });
    expect(parseCustom(SORT, "3, 2.5")).toEqual({ error: { code: "nan", token: "2.5" } });
    expect(parseCustom(SORT, "3, 120")).toEqual({ error: { code: "range", value: 120, min: 1, max: 99 } });
    expect(parseCustom(SORT, "0, 4")).toEqual({ error: { code: "range", value: 0, min: 1, max: 99 } });
  });

  it("presets are seeded, sized and shaped", () => {
    expect(preset("random", 7, 10, LIM)).toEqual(preset("random", 7, 10, LIM));
    for (const p of PRESETS) {
      const xs = preset(p, 3, 10, LIM);
      expect(xs).toHaveLength(10);
      expect(xs.every((v) => v >= 1 && v <= 99)).toBe(true);
    }
    const rev = preset("reversed", 3, 10, LIM);
    expect(rev).toEqual([...rev].sort((a, b) => b - a));
    expect(new Set(preset("few-unique", 3, 12, LIM)).size).toBeLessThanOrEqual(3);
    const nearly = preset("nearly-sorted", 3, 12, LIM);
    const outOfPlace = nearly.filter((v, k) => v !== [...nearly].sort((a, b) => a - b)[k]).length;
    expect(outOfPlace).toBeLessThanOrEqual(4);
  });

  it("the generator is stable across machines", () => {
    const next = rng(42);
    expect([next(), next()].map((x) => x.toFixed(6))).toEqual(["0.601104", "0.448291"]);
  });
});

describe("markers", () => {
  it("strips trailing marker comments and maps ids to lines", () => {
    const src = "def f(a):  # @step one\n    return a  # keep this\n    pass // @step two\n";
    const v = parseSample(src);
    expect(v.lines).toEqual(["def f(a):", "    return a  # keep this", "    pass"]);
    expect(v.lineOf).toEqual({ one: 1, two: 3 });
  });

  it("never shows a marker", () => {
    for (const def of Object.values(TRACERS)) {
      for (const ext of ["py", "ts"] as const) {
        const shown = parseSample(sampleSource(def.subject, def.id, ext)).lines.join(" | ");
        expect(shown, `${def.id}.${ext}`).not.toContain("@step");
      }
    }
  });
});

describe("progress settings", () => {
  it("accepts only well-formed values", () => {
    expect(parseViewed(["a", "b"])).toEqual(["a", "b"]);
    expect(parseViewed(["a", 1])).toBeUndefined();
    expect(parseLast({ subject: "sorting", topic: "quick-sort", step: 3, total: 40 })).toEqual({
      subject: "sorting",
      topic: "quick-sort",
      step: 3,
      total: 40,
    });
    expect(parseLast({ subject: "sorting", topic: "quick-sort", step: -1, total: 4 })).toBeUndefined();
    expect(parseLast("quick-sort")).toBeUndefined();
  });
});

describe("compare partners", () => {
  const all = Object.values(TRACERS);
  it("pairs topics of one subject and group whose limits take the current input", () => {
    const ids = (id: string, input: number[]) => comparePartners(TRACERS[id]!, all, input).map((d) => d.id);
    const forQuick = ids("quick-sort", [38, 27, 43, 3]);
    expect(forQuick).toContain("bubble-sort");
    expect(forQuick).toContain("merge-sort");
    expect(forQuick).not.toContain("quick-sort");
    expect(forQuick).not.toContain("quickselect"); // its input means k first
    expect(forQuick).not.toContain("counting-sort"); // 38 is outside 0..9
    // Counting sort with a 0: the 1..99 sorts cannot take it.
    expect(ids("counting-sort", [3, 0, 2])).toEqual([]);
    expect(ids("counting-sort", [3, 1, 2])).toContain("bubble-sort");
    expect(comparePartners(TRACERS["prefix-sums"]!, all, [1, 2])).toEqual([]);
  });

  it("the shorter run holds its last step, marked finished", () => {
    const steps = TRACERS["bubble-sort"]!.trace([2, 1]);
    expect(paneStep(steps, 0)).toEqual({ step: steps[0], finished: false });
    expect(paneStep(steps, steps.length + 5)).toEqual({ step: steps.at(-1), finished: true });
  });
});
