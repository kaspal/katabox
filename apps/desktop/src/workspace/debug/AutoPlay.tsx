// Auto-play for the debugger and its Visualize tab (expansion M15): steps on a timer at
// 0.5×, 1× or 2× (`debug.autoplay_speed`). Each step is an ordinary replay (ADR 0022), so
// the next tick waits for the previous step to come back; play stops when the run ends,
// fails, or the user pauses. Reduced motion only shortens the scenes' transitions (they
// already honour it); the stepping itself continues.
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { FastForward, Pause } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useSetting } from "@/lib/settings";
import type { Debugger } from "./useDebugger";

export const AUTOPLAY_SPEEDS = [0.5, 1, 2] as const;
export type AutoplaySpeed = (typeof AUTOPLAY_SPEEDS)[number];
export const SETTING_AUTOPLAY_SPEED = "debug.autoplay_speed";
/** Milliseconds between steps at 1×. */
export const AUTOPLAY_BASE_MS = 1000;

const isSpeed = (v: unknown): v is AutoplaySpeed =>
  typeof v === "number" && (AUTOPLAY_SPEEDS as readonly number[]).includes(v);

/** The timer: one `step_into` per tick while playing and paused at a stop. */
export function useAutoPlay(dbg: Pick<Debugger, "paused" | "busy" | "error" | "active" | "act">, speed: number) {
  const [playing, setPlaying] = useState(false);
  const { paused, busy, error, active, act } = dbg;
  // The run ended, failed or was stopped: stop playing.
  useEffect(() => {
    if (playing && (!active || error !== null || (!paused && !busy))) setPlaying(false);
  }, [playing, active, error, paused, busy]);
  useEffect(() => {
    if (!playing || !paused || busy) return;
    const h = window.setTimeout(() => void act("step_into"), AUTOPLAY_BASE_MS / speed);
    return () => window.clearTimeout(h);
  }, [playing, paused, busy, act, speed]);
  return { playing, setPlaying };
}

export function AutoPlay({ dbg }: { dbg: Debugger }) {
  const { t, i18n } = useTranslation();
  const [speed, setSpeed] = useSetting<AutoplaySpeed>(SETTING_AUTOPLAY_SPEED, 1, (v) => (isSpeed(v) ? v : undefined));
  const { playing, setPlaying } = useAutoPlay(dbg, speed);
  const can = dbg.active && dbg.paused;
  return (
    <div className="ml-1 flex items-center gap-1 border-l pl-1.5" data-testid="autoplay">
      <Button
        size="icon"
        variant={playing ? "secondary" : "ghost"}
        className="size-7"
        aria-pressed={playing}
        aria-label={t(playing ? "debug.autoplay.pause" : "debug.autoplay.play")}
        title={t(playing ? "debug.autoplay.pause" : "debug.autoplay.play")}
        onClick={() => setPlaying(!playing)}
        disabled={!playing && !can}
        data-testid="autoplay-toggle"
      >
        {playing ? <Pause aria-hidden /> : <FastForward aria-hidden />}
      </Button>
      <select
        className="h-7 rounded-md border bg-card px-1 text-xs text-foreground"
        aria-label={t("debug.autoplay.speed")}
        value={speed}
        onChange={(e) => {
          const v = Number(e.target.value);
          if (isSpeed(v)) setSpeed(v);
        }}
        data-testid="autoplay-speed"
      >
        {AUTOPLAY_SPEEDS.map((s) => (
          <option key={s} value={s}>
            {t("debug.autoplay.times", { speed: s.toLocaleString(i18n.language) })}
          </option>
        ))}
      </select>
    </div>
  );
}
