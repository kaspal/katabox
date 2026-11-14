// The Learn player (ADR 0029): play, pause, step, scrub and speed over precomputed steps.
// One timer at a time, scheduled against a target time so playback does not drift, and
// cleared on pause, unmount, a new trace and a speed change.
import { useCallback, useEffect, useRef, useState } from "react";

export const SPEEDS = [0.5, 1, 2, 4] as const;
export type Speed = (typeof SPEEDS)[number];
/** Milliseconds per step at each speed. */
export const DELAY_MS: Record<Speed, number> = { 0.5: 1500, 1: 850, 2: 420, 4: 200 };
export const isSpeed = (v: unknown): v is Speed => (SPEEDS as readonly unknown[]).includes(v);

/** How the current step was reached: only "play" and "step" count as watching it. */
export type Move = "start" | "play" | "step" | "seek";

export type Player = {
  step: number;
  playing: boolean;
  speed: Speed;
  /** The timeline is being dragged: scenes jump instead of animating. */
  dragging: boolean;
  lastMove: Move;
  atEnd: boolean;
  play: () => void;
  pause: () => void;
  toggle: () => void;
  forward: () => void;
  back: () => void;
  restart: () => void;
  /** Jump to a step (the timeline); `drag` while the thumb is held. */
  seek: (step: number, opts?: { drag?: boolean }) => void;
  endDrag: () => void;
  setSpeed: (s: Speed) => void;
};

type Opts = {
  initialStep?: number;
  speed?: Speed;
  /** The final step was reached by playing or stepping (the "viewed" rule). */
  onFinish?: () => void;
  onSpeed?: (s: Speed) => void;
  /** Playback paused (by the user or at the end), with the step it paused on. */
  onPause?: (step: number) => void;
};

/**
 * `total` is the number of steps; `traceKey` changes when a new trace replaces the old
 * one (new input or topic), which resets to `initialStep` and pauses.
 */
export function usePlayer(total: number, traceKey: unknown, opts: Opts = {}): Player {
  const last = Math.max(0, total - 1);
  const [step, setStep] = useState(() => Math.min(opts.initialStep ?? 0, last));
  const [playing, setPlaying] = useState(false);
  const [speed, setSpeedState] = useState<Speed>(opts.speed ?? 1);
  const [dragging, setDragging] = useState(false);
  const [lastMove, setLastMove] = useState<Move>("start");
  const optsRef = useRef(opts);
  optsRef.current = opts;
  const stepRef = useRef(step);
  stepRef.current = step;

  // A new trace: back to the start (or the remembered step), paused.
  const firstTrace = useRef(true);
  useEffect(() => {
    if (firstTrace.current) {
      firstTrace.current = false;
      return;
    }
    setPlaying(false);
    setStep(0);
    setLastMove("start");
  }, [traceKey]);

  // Keep the step in range if the trace shrinks.
  useEffect(() => {
    if (step > last) setStep(last);
  }, [step, last]);

  // The final step, reached by playing or stepping, counts as viewed.
  useEffect(() => {
    if (total > 0 && step === last && (lastMove === "play" || lastMove === "step")) {
      optsRef.current.onFinish?.();
    }
  }, [step, last, lastMove, total]);

  // Playback: one timeout chain against target times (no drift, no overlap).
  useEffect(() => {
    if (!playing) return;
    const delay = DELAY_MS[speed];
    let target = Date.now() + delay;
    let timer: ReturnType<typeof setTimeout>;
    const tick = () => {
      const next = stepRef.current + 1;
      if (next > last) {
        setPlaying(false);
        optsRef.current.onPause?.(stepRef.current);
        return;
      }
      stepRef.current = next;
      setStep(next);
      setLastMove("play");
      if (next >= last) {
        setPlaying(false);
        optsRef.current.onPause?.(next);
        return;
      }
      target += delay;
      timer = setTimeout(tick, Math.max(0, target - Date.now()));
    };
    timer = setTimeout(tick, delay);
    return () => clearTimeout(timer);
  }, [playing, speed, last, traceKey]);

  const pauseAt = useCallback((at: number) => {
    setPlaying((was) => {
      if (was) optsRef.current.onPause?.(at);
      return false;
    });
  }, []);

  const play = useCallback(() => {
    if (stepRef.current >= last) {
      // Play at the end starts over.
      stepRef.current = 0;
      setStep(0);
      setLastMove("start");
    }
    setPlaying(true);
  }, [last]);
  const pause = useCallback(() => pauseAt(stepRef.current), [pauseAt]);
  const toggle = useCallback(() => (playing ? pause() : play()), [playing, pause, play]);
  const moveTo = useCallback(
    (to: number, how: Move) => {
      const clamped = Math.min(Math.max(0, to), last);
      pauseAt(clamped);
      stepRef.current = clamped;
      setStep(clamped);
      setLastMove(how);
    },
    [last, pauseAt],
  );

  return {
    step: Math.min(step, last),
    playing,
    speed,
    dragging,
    lastMove,
    atEnd: step >= last,
    play,
    pause,
    toggle,
    forward: useCallback(() => moveTo(stepRef.current + 1, "step"), [moveTo]),
    back: useCallback(() => moveTo(stepRef.current - 1, "step"), [moveTo]),
    restart: useCallback(() => moveTo(0, "seek"), [moveTo]),
    seek: useCallback(
      (to: number, o?: { drag?: boolean }) => {
        if (o?.drag) setDragging(true);
        moveTo(to, "seek");
      },
      [moveTo],
    ),
    endDrag: useCallback(() => setDragging(false), []),
    setSpeed: useCallback((s: Speed) => {
      setSpeedState(s);
      optsRef.current.onSpeed?.(s);
    }, []),
  };
}
