// Scenes (ADR 0029): render and summaries, identity across moves (equal values too), and
// transitions that switch off for timeline drags and "reduce motion".
import { describe, expect, it } from "vitest";
import { render, renderHook, screen, within } from "@testing-library/react";
import type { Step } from "../engine/types";
import { REDUCED_MOTION } from "../engine/media";
import { ArrayScene } from "./ArrayScene";
import { BarsScene } from "./BarsScene";
import { useTransitions } from "./common";
import { TRACERS } from "../tracers";
import { SCENES } from ".";
import { caseInputs } from "../engine/cases";

const step = (values: [number, number][], extra: Partial<Step> = {}): Step => ({
  items: values.map(([id, value]) => ({ id, value })),
  roles: {},
  lifted: null,
  pointers: [],
  range: null,
  counters: { comparisons: 0, moves: 0, writes: 0 },
  narration: { key: "quick-sort.ready" },
  marker: null,
  vars: {},
  ...extra,
});

describe("BarsScene", () => {
  it("describes itself and lists every value by position", () => {
    render(
      <BarsScene
        step={step(
          [
            [0, 5],
            [1, 9],
            [2, 2],
          ],
          { roles: { 0: "compare", 2: "pivot" }, range: [0, 2], pointers: [{ label: "j", at: 0, tone: "j" }] },
        )}
        instant={false}
        def={TRACERS["bubble-sort"]!}
        maxValue={99}
      />,
    );
    expect(screen.getByRole("img")).toHaveAccessibleName(
      "Values in order: 5, 9, 2. Comparing positions 0. Pivot 2 at position 2. Active range: positions 0 to 2.",
    );
    const items = within(screen.getByTestId("scene-fallback")).getAllByRole("listitem");
    expect(items.map((li) => li.textContent)).toEqual([
      "Position 0: 5, Comparing",
      "Position 1: 9, Not yet placed",
      "Position 2: 2, Pivot",
    ]);
    // Compared bars carry a dashed outline, not only a colour.
    const compared = screen.getByTestId("bars-scene").querySelector('[data-role="compare"] rect');
    expect(compared?.getAttribute("stroke-dasharray")).toBe("5 3");
    expect(screen.getByTestId("bars-scene").querySelector('[data-pointer="j"]')).not.toBeNull();
  });

  it("keeps each bar's element when equal values trade places", () => {
    const a = step([
      [0, 5],
      [1, 5],
      [2, 1],
    ]);
    const b = step([
      [1, 5],
      [0, 5],
      [2, 1],
    ]);
    const { rerender, container } = render(<BarsScene step={a} instant={false} def={TRACERS["bubble-sort"]!} maxValue={9} />);
    const first = container.querySelector('[data-id="0"]');
    const second = container.querySelector('[data-id="1"]');
    expect(first?.getAttribute("data-pos")).toBe("0");
    rerender(<BarsScene step={b} instant={false} def={TRACERS["bubble-sort"]!} maxValue={9} />);
    // The same nodes, at new positions: they slide rather than being redrawn.
    expect(container.querySelector('[data-id="0"]')).toBe(first);
    expect(container.querySelector('[data-id="1"]')).toBe(second);
    expect(first?.getAttribute("data-pos")).toBe("1");
    expect(second?.getAttribute("data-pos")).toBe("0");
  });
});

describe("ArrayScene", () => {
  it("names the shifting role and shows the lifted key over a gap", () => {
    render(
      <ArrayScene
        step={step(
          [
            [0, 3],
            [1, 8],
          ],
          { roles: { 0: "swap", 1: "key" }, lifted: 1 },
        )}
        instant
        def={TRACERS["insertion-sort"]!}
        maxValue={99}
      />,
    );
    expect(screen.getByRole("img").getAttribute("aria-label")).toContain("Key 8 lifted above position 1.");
    const items = within(screen.getByTestId("scene-fallback")).getAllByRole("listitem");
    expect(items[0]).toHaveTextContent("Shifting");
    expect(screen.getByTestId("array-scene").querySelector("[data-gap]")).not.toBeNull();
  });
});

describe("scene transitions", () => {
  it("spring and ease normally", () => {
    const { result } = renderHook(() => useTransitions(false));
    expect(result.current.move).toMatchObject({ type: "spring" });
    expect(result.current.colors).not.toBe("none");
  });

  it("are instant while the timeline is dragged", () => {
    const { result } = renderHook(() => useTransitions(true));
    expect(result.current.move).toEqual({ duration: 0 });
    expect(result.current.lift).toEqual({ duration: 0 });
    expect(result.current.colors).toBe("none");
  });

  it("are instant with reduce motion", () => {
    const original = window.matchMedia;
    window.matchMedia = ((q: string) => ({ ...original(q), matches: q === REDUCED_MOTION })) as typeof window.matchMedia;
    try {
      const { result } = renderHook(() => useTransitions(false));
      expect(result.current.move).toEqual({ duration: 0 });
      expect(result.current.colors).toBe("none");
    } finally {
      window.matchMedia = original;
    }
  });
});

describe("every topic's scene", () => {
  it.each(Object.values(TRACERS).map((d) => [d.id, d] as const))("%s renders its first, middle and last steps", (_, def) => {
    const steps = def.trace(caseInputs(def)[0]!.input);
    const Scene = SCENES[def.scene];
    for (const k of [0, Math.floor(steps.length / 2), steps.length - 1]) {
      const { container, unmount } = render(<Scene step={steps[k]!} instant def={def} maxValue={def.limits.max} />);
      const svg = container.querySelector("svg[role=img]");
      expect(svg, `${def.id} step ${k}`).not.toBeNull();
      // A screen reader gets the summary (aria-label) and the text fallback; together they
      // must describe the step (an empty first step may have nothing to list yet).
      const said = `${svg!.getAttribute("aria-label")} ${screen.getByTestId("scene-fallback").textContent}`;
      expect(said.trim().length, `${def.id} step ${k}`).toBeGreaterThan(10);
      unmount();
    }
  });
});
