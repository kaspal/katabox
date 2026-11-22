import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, renderHook } from "@testing-library/react";
import { AUTOPLAY_BASE_MS, useAutoPlay } from "./AutoPlay";

type Dbg = Parameters<typeof useAutoPlay>[0];

describe("debugger auto-play (expansion M15)", () => {
  beforeEach(() => vi.useFakeTimers());
  afterEach(() => vi.useRealTimers());

  const make = (over: Partial<Dbg> = {}): Dbg => ({
    paused: true,
    busy: false,
    error: null,
    active: true,
    act: vi.fn(async () => undefined),
    ...over,
  });

  it("steps into the next line on every tick, faster at 2×", () => {
    const dbg = make();
    const { result, rerender } = renderHook(({ d, speed }) => useAutoPlay(d, speed), {
      initialProps: { d: dbg, speed: 1 },
    });
    act(() => result.current.setPlaying(true));
    act(() => vi.advanceTimersByTime(AUTOPLAY_BASE_MS - 1));
    expect(dbg.act).not.toHaveBeenCalled();
    act(() => vi.advanceTimersByTime(1));
    expect(dbg.act).toHaveBeenCalledWith("step_into");
    // A new stop (a new act identity, as after each step) at 2×: half the wait.
    const faster = make();
    rerender({ d: faster, speed: 2 });
    act(() => vi.advanceTimersByTime(AUTOPLAY_BASE_MS / 2));
    expect(faster.act).toHaveBeenCalledTimes(1);
  });

  it("waits while a step replays, and stops when the run ends or fails", () => {
    const busy = make({ busy: true, paused: true });
    const { result, rerender } = renderHook(({ d }) => useAutoPlay(d, 1), { initialProps: { d: busy } });
    act(() => result.current.setPlaying(true));
    act(() => vi.advanceTimersByTime(5 * AUTOPLAY_BASE_MS));
    expect(busy.act).not.toHaveBeenCalled();
    expect(result.current.playing).toBe(true);
    // Finished: no longer paused, not replaying.
    rerender({ d: make({ paused: false }) });
    expect(result.current.playing).toBe(false);
    act(() => result.current.setPlaying(true));
    rerender({ d: make({ error: new Error("boom") }) });
    expect(result.current.playing).toBe(false);
  });

  it("does nothing until played", () => {
    const dbg = make();
    renderHook(() => useAutoPlay(dbg, 1));
    act(() => vi.advanceTimersByTime(10 * AUTOPLAY_BASE_MS));
    expect(dbg.act).not.toHaveBeenCalled();
  });
});
