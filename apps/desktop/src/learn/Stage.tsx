// The stage (ADR 0029, mockup "Main"): legend and counters, the scene, the narration,
// the playback controls and the input row.
import { useId, useState } from "react";
import type { TFunction } from "i18next";
import { useTranslation } from "react-i18next";
import { Pause, Play, RotateCcw, SkipBack, SkipForward } from "lucide-react";
import type { LearnInput } from "@/bindings";
import { Button } from "@/components/ui/button";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";
import { PRESETS, hasTarget, valueLimits, type PresetId } from "./engine/inputs";
import type { TraceError } from "./engine/useTrace";
import { SPEEDS, isSpeed, type Player } from "./engine/usePlayer";
import type { Role, Step, TracerDef } from "./engine/types";
import { outline, roleColor, roleName } from "./scenes/common";
import { SCENES } from "./scenes";

function Swatch({ role }: { role: Role }) {
  const o = outline(role);
  return (
    <svg aria-hidden width="12" height="12" viewBox="0 0 12 12" className="flex-none">
      <rect
        x="1"
        y="1"
        width="10"
        height="10"
        rx="2.5"
        style={{ fill: roleColor(role) }}
        stroke={o.stroke}
        strokeWidth={o.stroke ? 1.5 : undefined}
        strokeDasharray={o.strokeDasharray ? "3 2" : undefined}
      />
    </svg>
  );
}

function errorText(t: TFunction, e: TraceError): string {
  switch (e.code) {
    case "count":
      return t("learn.input.errors.count", { min: e.min, max: e.max });
    case "nan":
      return t("learn.input.errors.nan", { token: e.token });
    case "range":
      return t("learn.input.errors.range", { value: e.value, min: e.min, max: e.max });
    case "steps":
      return t("learn.input.errors.steps", { max: e.max });
    case "unsorted":
      return t("learn.input.errors.unsorted");
    case "target":
      return t("learn.input.errors.target", { min: e.min, max: e.max });
  }
}

export function Stage({
  def,
  step,
  total,
  player,
  maxValue,
  input,
  presets,
  values,
  error,
  onPreset,
  onCustom,
  onClearError,
}: {
  def: TracerDef;
  step: Step;
  total: number;
  player: Player;
  maxValue: number;
  input: LearnInput;
  presets: string[];
  values: number[];
  error: TraceError | null;
  onPreset: (id: PresetId) => void;
  onCustom: (text: string, target?: string) => boolean;
  onClearError: () => void;
}) {
  const { t } = useTranslation();
  const withTarget = hasTarget(def);
  const count = def.input.kind === "count";
  const shown = withTarget ? values.slice(1) : values;
  const [custom, setCustom] = useState(() => shown.join(", "));
  const [target, setTarget] = useState(() => (withTarget ? String(values[0]) : ""));
  const errorId = useId();
  const Scene = SCENES[def.scene];
  const vl = valueLimits(def);
  const narration = t(`learn.narration.${step.narration.key}` as "learn.title", step.narration.params ?? {});
  const shortcuts = t("learn.controls.shortcuts");

  const control = (label: string, icon: React.ReactNode, onClick: () => void, keys: string, primary = false) => (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button
          type="button"
          variant={primary ? "default" : "outline"}
          size="icon"
          aria-label={label}
          aria-keyshortcuts={keys}
          onClick={onClick}
          className={cn("size-10 rounded-lg", primary && "w-13")}
        >
          {icon}
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <span className="font-medium">{label}</span>
        <span className="block text-xs opacity-80">{shortcuts}</span>
      </TooltipContent>
    </Tooltip>
  );

  return (
    <section
      aria-label={t("learn.stage.label")}
      className="flex flex-col gap-3.5 rounded-2xl border bg-card p-5"
      data-testid="stage"
    >
      <div className="flex flex-wrap items-center justify-between gap-2.5 text-xs text-muted-foreground">
        <ul aria-label={t("learn.stage.legend")} className="flex flex-wrap gap-x-3.5 gap-y-1.5">
          {def.legend.map((role) => (
            <li key={role} className="flex items-center gap-1.5">
              <Swatch role={role} />
              {roleName(t, role, def)}
            </li>
          ))}
        </ul>
        <dl className="flex gap-4 font-mono" data-testid="counters">
          {def.counters.map((c) => (
            <div key={c.label} className="flex gap-1.5">
              <dt>{t(`learn.stage.counter.${c.label}`)}</dt>
              <dd className="font-semibold text-foreground">{step.counters[c.slot]}</dd>
            </div>
          ))}
        </dl>
      </div>

      <div className="px-2 pt-1.5">
        <Scene step={step} instant={player.dragging} def={def} maxValue={maxValue} />
      </div>

      <p
        aria-live="polite"
        className="min-h-12 rounded-lg border bg-muted/40 px-3.5 py-3 text-sm leading-relaxed"
        data-testid="narration"
      >
        {narration}
      </p>

      <div role="group" aria-label={t("learn.controls.label")} className="flex flex-wrap items-center gap-2.5">
        <div className="flex gap-1.5">
          {control(t("learn.controls.restart"), <RotateCcw aria-hidden className="size-4" />, player.restart, "Home")}
          {control(t("learn.controls.back"), <SkipBack aria-hidden className="size-4" />, player.back, "ArrowLeft")}
          {control(
            player.playing ? t("learn.controls.pause") : t("learn.controls.play"),
            player.playing ? <Pause aria-hidden className="size-4" /> : <Play aria-hidden className="size-4" />,
            player.toggle,
            "Space",
            true,
          )}
          {control(t("learn.controls.forward"), <SkipForward aria-hidden className="size-4" />, player.forward, "ArrowRight")}
        </div>
        <label className="flex min-w-56 flex-1 items-center gap-2.5 font-mono text-xs text-muted-foreground">
          <input
            type="range"
            min={0}
            max={Math.max(0, total - 1)}
            value={player.step}
            aria-label={t("learn.controls.timeline")}
            aria-valuetext={t("learn.controls.step", { step: player.step + 1, total })}
            onChange={(e) => player.seek(Number(e.target.value), { drag: true })}
            onPointerUp={player.endDrag}
            onKeyUp={player.endDrag}
            onBlur={player.endDrag}
            className="w-full min-w-0 flex-1 accent-primary"
          />
          <span className="flex-none whitespace-nowrap tabular-nums" data-testid="step-label">
            {t("learn.controls.step", { step: player.step + 1, total })}
          </span>
        </label>
        <label className="flex items-center gap-1.5 text-sm text-muted-foreground">
          {t("learn.controls.speed")}
          <select
            value={player.speed}
            onChange={(e) => {
              const s = Number(e.target.value);
              if (isSpeed(s)) player.setSpeed(s);
            }}
            className="h-9 rounded-md border bg-card px-1.5 text-foreground"
          >
            {SPEEDS.map((s) => (
              <option key={s} value={s}>
                {s}×
              </option>
            ))}
          </select>
        </label>
      </div>

      <div className="flex flex-wrap items-start gap-2 border-t pt-3 text-sm">
        <span className="mt-1.5 mr-1 text-muted-foreground">{t("learn.input.label")}</span>
        {PRESETS.filter((p) => presets.includes(p) && !(count && p !== "random")).map((p) => (
          <Button
            key={p}
            type="button"
            variant="outline"
            size="sm"
            className="rounded-full"
            onClick={() => {
              onPreset(p);
              setCustom("");
              setTarget("");
            }}
            data-preset={p}
          >
            {t(`learn.input.presets.${p}`)}
          </Button>
        ))}
        <form
          className="ml-auto flex flex-col gap-1"
          onSubmit={(e) => {
            e.preventDefault();
            onCustom(custom, withTarget ? target : undefined);
          }}
        >
          <div className="flex items-center gap-1.5">
            <label htmlFor={`${errorId}-input`} className="text-muted-foreground">
              {count ? t("learn.input.count") : t("learn.input.custom")}
            </label>
            <input
              id={`${errorId}-input`}
              value={custom}
              onChange={(e) => {
                setCustom(e.target.value);
                if (error) onClearError();
              }}
              aria-label={
                count
                  ? t("learn.input.count_label", { lo: input.min, hi: input.max })
                  : t("learn.input.custom_label", { min: vl.min_len, max: vl.max_len, lo: input.min, hi: input.max })
              }
              aria-invalid={error ? true : undefined}
              aria-describedby={error ? errorId : undefined}
              placeholder={shown.join(", ")}
              className={cn(
                "h-8 rounded-md border bg-card px-2.5 font-mono text-xs aria-invalid:border-destructive",
                count ? "w-16" : "w-56",
              )}
            />
            {withTarget && (
              <>
                <label htmlFor={`${errorId}-target`} className="text-muted-foreground">
                  {t(`learn.input.target.${def.input.targetLabel ?? "target"}`)}
                </label>
                <input
                  id={`${errorId}-target`}
                  value={target}
                  onChange={(e) => {
                    setTarget(e.target.value);
                    if (error) onClearError();
                  }}
                  aria-invalid={error ? true : undefined}
                  aria-describedby={error ? errorId : undefined}
                  placeholder={String(values[0])}
                  className="h-8 w-14 rounded-md border bg-card px-2 font-mono text-xs aria-invalid:border-destructive"
                  data-testid="target-input"
                />
              </>
            )}
            <Button type="submit" size="sm" variant="secondary">
              {t("learn.input.apply")}
            </Button>
          </div>
          {error && (
            <p id={errorId} role="alert" className="text-xs text-destructive">
              {errorText(t, error)}
            </p>
          )}
        </form>
      </div>
    </section>
  );
}

