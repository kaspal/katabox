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
import { fieldsOf, presetIds, valueLimits, type PresetId } from "./engine/inputs";
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
    case "custom": {
      // A `field` parameter is a field key: show its label.
      const params = { ...e.params };
      if (typeof params.field === "string") params.field = t(`learn.input.field.${params.field}` as "learn.title");
      return t(`learn.input.errors.custom.${e.key}` as "learn.title", params);
    }
  }
}

/** Legend and counters, the scene and the narration of one topic's step. */
export function StagePane({
  def,
  step,
  instant,
  maxValue,
  live = true,
}: {
  def: TracerDef;
  step: Step;
  instant: boolean;
  maxValue: number;
  /** Announce the narration (one pane per page does, so compare mode is not read twice). */
  live?: boolean;
}) {
  const { t } = useTranslation();
  const Scene = SCENES[def.scene];
  const narration = t(`learn.narration.${step.narration.key}` as "learn.title", step.narration.params ?? {});
  return (
    <>
      <div className="flex flex-wrap items-center justify-between gap-2.5 text-xs text-muted-foreground">
        <ul aria-label={t("learn.stage.legend")} className="flex flex-wrap gap-x-3.5 gap-y-1.5">
          {def.legend.map((role) => (
            <li key={role} className="flex items-center gap-1.5">
              <Swatch role={role} />
              {roleName(t, role, def)}
            </li>
          ))}
        </ul>
        <dl className="flex flex-wrap gap-x-4 gap-y-1 font-mono" data-testid="counters">
          {def.counters.map((c) => (
            <div key={c.label} className="flex gap-1.5">
              <dt>{t(`learn.stage.counter.${c.label}`)}</dt>
              <dd className="font-semibold text-foreground">{step.counters[c.slot]}</dd>
            </div>
          ))}
        </dl>
      </div>

      <div className="px-2 pt-1.5">
        <Scene step={step} instant={instant} def={def} maxValue={maxValue} />
      </div>

      <p
        aria-live={live ? "polite" : "off"}
        className="min-h-12 rounded-lg border bg-muted/40 px-3.5 py-3 text-sm leading-relaxed"
        data-testid="narration"
      >
        {narration}
      </p>
    </>
  );
}

/** Restart, back, play/pause, forward, the timeline and the speed. */
export function PlayerControls({ player, total }: { player: Player; total: number }) {
  const { t } = useTranslation();
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
  );
}

export type InputRowProps = {
  def: TracerDef;
  input: LearnInput;
  presets: string[];
  values: number[];
  error: TraceError | null;
  onPreset: (id: PresetId) => void;
  onCustom: (texts: readonly string[]) => boolean;
  onClearError: () => void;
};

/** Presets and "your own" input: one field per part of the input (values, a target, n,
 * or a structured topic's fields such as operations or text). */
export function InputRow({ def, input, presets, values, error, onPreset, onCustom, onClearError }: InputRowProps) {
  const { t } = useTranslation();
  const fields = fieldsOf(def, values);
  const [texts, setTexts] = useState<string[]>(() => fields.map((f) => f.text));
  const errorId = useId();
  const vl = valueLimits(def);
  const offered = presetIds(def).filter((p) => presets.includes(p));
  const label = (key: string) => t(`learn.input.field.${key}` as "learn.title");
  return (
    <div className="flex flex-wrap items-start gap-2 border-t pt-3 text-sm">
      <span className="mt-1.5 mr-1 text-muted-foreground">{t("learn.input.label")}</span>
      {offered.map((p) => (
        <Button
          key={p}
          type="button"
          variant="outline"
          size="sm"
          className="rounded-full"
          onClick={() => {
            onPreset(p);
            setTexts(fields.map(() => ""));
          }}
          data-preset={p}
        >
          {t(`learn.input.presets.${p}` as "learn.title")}
        </Button>
      ))}
      <form
        className="ml-auto flex flex-col gap-1"
        onSubmit={(e) => {
          e.preventDefault();
          onCustom(texts);
        }}
      >
        <div className="flex flex-wrap items-center gap-1.5">
          {fields.map((f, k) => {
            // Lists and text get a wide box; single numbers (n, a target, a capacity) a short one.
            const short = !["values", "ops", "text", "words", "pattern", "edges", "grid", "map", "word", "segments", "distances", "first", "second", "parents", "pairs", "points", "intervals", "items"].includes(f.key);
            return (
              <div key={f.key} className="flex items-center gap-1.5">
                <label htmlFor={`${errorId}-${f.key}`} className="text-muted-foreground">
                  {label(f.key)}
                </label>
                <input
                  id={`${errorId}-${f.key}`}
                  value={texts[k] ?? ""}
                  onChange={(e) => {
                    const next = [...texts];
                    next[k] = e.target.value;
                    setTexts(next);
                    if (error) onClearError();
                  }}
                  aria-label={
                    f.key === "values"
                      ? t("learn.input.custom_label", { min: vl.min_len, max: vl.max_len, lo: input.min, hi: input.max })
                      : f.key === "count"
                        ? t("learn.input.count_label", { lo: input.min, hi: input.max })
                        : undefined
                  }
                  aria-invalid={error ? true : undefined}
                  aria-describedby={error ? errorId : undefined}
                  placeholder={f.text}
                  className={cn(
                    "h-8 rounded-md border bg-card px-2.5 font-mono text-xs aria-invalid:border-destructive",
                    short ? "w-16" : "w-56",
                  )}
                  data-field={f.key}
                />
              </div>
            );
          })}
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
  );
}

export function Stage({
  def,
  step,
  total,
  player,
  maxValue,
  ...input
}: {
  def: TracerDef;
  step: Step;
  total: number;
  player: Player;
  maxValue: number;
} & Omit<InputRowProps, "def">) {
  const { t } = useTranslation();
  return (
    <section
      aria-label={t("learn.stage.label")}
      className="flex flex-col gap-3.5 rounded-2xl border bg-card p-5"
      data-testid="stage"
    >
      <StagePane def={def} step={step} instant={player.dragging} maxValue={maxValue} />
      <PlayerControls player={player} total={total} />
      <InputRow def={def} {...input} />
    </section>
  );
}
