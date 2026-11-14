// Compare mode (ADR 0029, Phase 1): the current topic and a partner from the same subject
// side by side on one input, each with its own legend, counters, scene and narration, under
// one shared player and input row. The shorter run waits on its last step, marked
// "Finished". Nothing here marks a topic viewed or saves the Continue position.
import { useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import { CheckCircle2, X } from "lucide-react";
import type { LearnInput } from "@/bindings";
import { Button } from "@/components/ui/button";
import { paneStep, useCompareTrace } from "./engine/compare";
import { playerKeyAction } from "./engine/keys";
import type { TracerDef } from "./engine/types";
import { usePlayer, type Speed } from "./engine/usePlayer";
import { InputRow, PlayerControls, StagePane } from "./Stage";

export function CompareView({
  a,
  b,
  titles,
  initial,
  input,
  presets,
  speed,
  onSpeed,
  onExit,
}: {
  a: TracerDef;
  b: TracerDef;
  titles: [string, string];
  initial: number[];
  input: LearnInput;
  presets: string[];
  speed: Speed;
  onSpeed: (s: Speed) => void;
  onExit: () => void;
}) {
  const { t } = useTranslation();
  const trace = useCompareTrace(a, b, initial);
  const total = Math.max(trace.steps[0].length, trace.steps[1].length);
  const player = usePlayer(total, `compare:${a.id}:${b.id}:${trace.version}`, { speed, onSpeed });

  // The page's shortcuts drive this player while compare mode is open.
  const playerRef = useRef(player);
  playerRef.current = player;
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      const action = playerKeyAction(e);
      if (!action) return;
      e.preventDefault();
      playerRef.current[action]();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  if (total === 0) return null;
  const panes = [a, b].map((def, k) => ({ def, title: titles[k]!, ...paneStep(trace.steps[k as 0 | 1], player.step) }));

  return (
    <section
      aria-label={t("learn.compare.label", { a: titles[0], b: titles[1] })}
      className="flex flex-col gap-3.5 rounded-2xl border bg-card p-5"
      data-testid="compare"
    >
      <div className="flex items-center justify-between gap-2">
        <h2 className="text-sm font-semibold">{t("learn.compare.heading")}</h2>
        <Button variant="ghost" size="sm" onClick={onExit} data-testid="compare-exit">
          <X aria-hidden className="size-4" />
          {t("learn.compare.exit")}
        </Button>
      </div>
      <div className="grid gap-4 [grid-template-columns:repeat(auto-fit,minmax(min(100%,260px),1fr))]">
        {panes.map((p, k) => (
          <div key={p.def.id} className="flex min-w-0 flex-col gap-3 rounded-xl border p-3" data-testid={`compare-pane-${k}`}>
            <div className="flex items-center justify-between gap-2">
              <h3 className="font-semibold">{p.title}</h3>
              {p.finished && (
                <span className="flex items-center gap-1 rounded-full bg-accent px-2 py-0.5 text-xs font-medium" data-testid="compare-finished">
                  <CheckCircle2 aria-hidden className="size-3.5" />
                  {t("learn.compare.finished")}
                </span>
              )}
            </div>
            <StagePane def={p.def} step={p.step} instant={player.dragging} maxValue={input.max} live={k === 0} />
          </div>
        ))}
      </div>
      <PlayerControls player={player} total={total} />
      <InputRow
        def={a}
        input={input}
        presets={presets}
        values={trace.values}
        error={trace.error}
        onPreset={trace.applyPreset}
        onCustom={trace.applyCustom}
        onClearError={trace.clearError}
      />
    </section>
  );
}
