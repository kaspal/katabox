// The Debug panel's Visualize tab (ADR 0027): the chosen variables drawn at each stop, with
// what changed since the previous step. While this tab is open every stop carries the
// debugger's structured snapshot; closing it stops asking.
import { useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { Maximize2, ZoomIn, ZoomOut } from "lucide-react";
import type { StopInfo, VarInfo } from "@/bindings";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { useSetting } from "@/lib/settings";
import type { Debugger } from "@/workspace/debug/useDebugger";
import { DebugToolbar } from "@/workspace/debug/DebugPanel";
import { diffModels, NO_CHANGES, nodeIndex } from "@/workspace/debug/visualize/diff";
import {
  DEFAULT_SETTINGS,
  chosenNames,
  intLocals,
  isPointer,
  modelsOf,
  parseSettings,
  settingKey,
  type Model,
  type ViewAs,
  type VizSettings,
} from "@/workspace/debug/visualize/model";
import { ChangeLegend, ModelCard } from "@/workspace/debug/visualize/Views";

const HIDDEN_TYPES = new Set(["function", "module", "type", "builtin_function_or_method", "method"]);
const CONTAINER_TYPES = new Set(["list", "tuple", "dict", "defaultdict", "Counter", "OrderedDict", "deque", "Array", "Map", "Object"]);
const ZOOMS = [0.5, 0.75, 1, 1.25, 1.5, 2, 3];

export function useVizSettings(problem: string) {
  return useSetting<VizSettings>(settingKey(problem), DEFAULT_SETTINGS, parseSettings);
}

/** The drawings for one stop, compared with the previous step's. */
function Canvas({
  stop,
  previous,
  names,
  settings,
  zoom,
  large,
}: {
  stop: StopInfo;
  previous: StopInfo | null;
  names: string[];
  settings: VizSettings;
  zoom: number;
  large?: boolean;
}) {
  const { t } = useTranslation();
  const snap = stop.snapshot;
  const models = useMemo(() => (snap ? modelsOf(snap, names, stop.locals, settings) : []), [snap, names, stop.locals, settings]);
  const [before, known] = useMemo(() => {
    const prev = previous?.snapshot;
    if (!prev) return [new Map<string, Model>(), undefined];
    // Every root of the previous step (not only the chosen ones): a node may move between
    // drawings, and its old place is wherever it was reachable from.
    const all = modelsOf(prev, (prev.roots ?? []).map((r) => r.name), previous.locals, settings);
    return [new Map(all.map((m) => [m.name, m])), nodeIndex(all)];
  }, [previous, settings]);
  if (!snap) return null;
  if (snap.error) {
    const key = snap.error === "too_large" ? "debug.visualize.error.too_large" : "debug.visualize.error.other";
    return <p className="text-sm text-muted-foreground">{t(key)}</p>;
  }
  const drawn = new Set(models.map((m) => m.name));
  const missing = names.filter((n) => !drawn.has(n));
  return (
    <div className="space-y-2" data-testid="viz-canvas">
      {models.length === 0 && names.length === 0 && <p className="text-sm text-muted-foreground">{t("debug.visualize.empty")}</p>}
      {models.map((m) => (
        <ModelCard key={m.name} m={m} changes={previous ? diffModels(before.get(m.name), m, known) : NO_CHANGES} zoom={zoom} large={large} />
      ))}
      {missing.map((n) => (
        <p key={n} className="text-xs text-muted-foreground">
          {t("debug.visualize.missing", { name: n })}
        </p>
      ))}
      {snap.truncated && <p className="text-xs text-muted-foreground">{t("debug.visualize.truncated")}</p>}
    </div>
  );
}

/** Which variables to draw, which ints are pointers, and how to draw containers. */
function Picker({
  vars,
  names,
  settings,
  onChange,
}: {
  vars: VarInfo[];
  names: string[];
  settings: VizSettings;
  onChange: (s: VizSettings) => void;
}) {
  const { t } = useTranslation();
  const ints = intLocals(vars);
  return (
    <fieldset className="space-y-1 text-xs" data-testid="viz-picker">
      <legend className="mb-1 font-semibold text-muted-foreground uppercase">{t("debug.visualize.choose")}</legend>
      <ul className="space-y-1">
        {vars.map((v) => {
          const shown = names.includes(v.name);
          const pointer = ints.has(v.name);
          return (
            <li key={v.name} className="flex flex-wrap items-center gap-x-3 gap-y-0.5">
              <label className="flex items-center gap-1 font-mono">
                <input
                  type="checkbox"
                  checked={shown}
                  aria-label={t("debug.visualize.show", { name: v.name })}
                  onChange={() =>
                    onChange({
                      ...settings,
                      show: shown ? names.filter((n) => n !== v.name) : [...names, v.name],
                    })
                  }
                />
                {v.name}
              </label>
              {pointer && (
                <label className="flex items-center gap-1 text-muted-foreground">
                  <input
                    type="checkbox"
                    checked={isPointer(v.name, settings)}
                    aria-label={t("debug.visualize.pointer", { name: v.name })}
                    onChange={() =>
                      onChange({ ...settings, pointers: { ...settings.pointers, [v.name]: !isPointer(v.name, settings) } })
                    }
                  />
                  {t("debug.visualize.pointers")}
                </label>
              )}
              {shown && CONTAINER_TYPES.has(v.type) && (
                <select
                  className="rounded border bg-background px-1 py-0.5"
                  aria-label={t("debug.visualize.viewAs", { name: v.name })}
                  value={settings.as[v.name] ?? "auto"}
                  onChange={(e) => onChange({ ...settings, as: { ...settings.as, [v.name]: e.target.value as ViewAs } })}
                >
                  {(["auto", "graph", "heap"] as const).map((a) => (
                    <option key={a} value={a}>
                      {t(`debug.visualize.as.${a}`)}
                    </option>
                  ))}
                </select>
              )}
            </li>
          );
        })}
      </ul>
      <Button size="sm" variant="ghost" className="h-6 px-2 text-xs" onClick={() => onChange(DEFAULT_SETTINGS)}>
        {t("debug.visualize.defaults")}
      </Button>
    </fieldset>
  );
}

export function VisualizeTab({ dbg, problem, params }: { dbg: Debugger; problem: string; params: string[] }) {
  const { t } = useTranslation();
  const [settings, setSettings] = useVizSettings(problem);
  // Remembered per problem: stepping, re-running or a new session reopen it.
  const large = settings.large === true;
  const setLarge = (open: boolean) => setSettings({ ...settings, large: open || undefined });
  const [zoom, setZoom] = useState(1.5);
  const names = useMemo(() => chosenNames(settings, params), [settings, params]);
  const stop = dbg.view?.state.state === "paused" ? dbg.view.state.stop : null;

  // Ask for snapshots while this tab is open, with the current names; stop when it closes.
  const dbgRef = useRef(dbg);
  dbgRef.current = dbg;
  const namesKey = JSON.stringify(names);
  useEffect(() => {
    void dbgRef.current.setVisualize(JSON.parse(namesKey) as string[]);
  }, [namesKey]);
  useEffect(() => () => void dbgRef.current.setVisualize(null), []);
  // A stop that came back without a snapshot (asked before the session knew): once per step.
  const retried = useRef<number | null>(null);
  useEffect(() => {
    if (stop && !stop.snapshot && !dbg.busy && dbg.visualize !== null && retried.current !== stop.step) {
      retried.current = stop.step;
      void dbg.setVisualize(dbg.visualize, true);
    }
  }, [stop, dbg]);

  if (!stop) return <p className="text-sm text-muted-foreground">{t("debug.visualize.paused")}</p>;
  const vars = [...stop.locals, ...stop.globals].filter((v) => !HIDDEN_TYPES.has(v.type));
  const total = dbg.view?.steps ?? 0;
  return (
    <div className="space-y-2" data-testid="debug-visualize">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <p className="text-xs text-muted-foreground">{t("debug.visualize.intro")}</p>
        <Button size="sm" variant="outline" className="h-7 gap-1 text-xs" onClick={() => setLarge(true)}>
          <Maximize2 aria-hidden className="size-3.5" />
          {t("debug.visualize.larger")}
        </Button>
      </div>
      <ChangeLegend />
      <div className="grid gap-3 md:grid-cols-[minmax(10rem,14rem)_1fr]">
        <Picker vars={vars} names={names} settings={settings} onChange={setSettings} />
        <Canvas stop={stop} previous={dbg.previous} names={names} settings={settings} zoom={1} />
      </div>
      <Dialog open={large} onOpenChange={setLarge}>
        <DialogContent className="max-h-[92vh] overflow-auto sm:max-w-6xl" data-testid="viz-large">
          <DialogHeader>
            <DialogTitle>{t("debug.visualize.largerTitle", { step: stop.step + 1, total })}</DialogTitle>
            <DialogDescription>{t("debug.visualize.intro")}</DialogDescription>
          </DialogHeader>
          <div className="flex flex-wrap items-center gap-2">
            <DebugToolbar dbg={dbg} />
            <div className="ml-auto flex items-center gap-1">
              <Button
                size="icon"
                variant="ghost"
                className="size-7"
                aria-label={t("debug.visualize.zoomOut")}
                disabled={zoom <= ZOOMS[0]}
                onClick={() => setZoom(ZOOMS[Math.max(ZOOMS.indexOf(zoom) - 1, 0)])}
              >
                <ZoomOut aria-hidden />
              </Button>
              <span className="w-12 text-center text-xs tabular-nums">{Math.round(zoom * 100)}%</span>
              <Button
                size="icon"
                variant="ghost"
                className="size-7"
                aria-label={t("debug.visualize.zoomIn")}
                disabled={zoom >= ZOOMS[ZOOMS.length - 1]}
                onClick={() => setZoom(ZOOMS[Math.min(ZOOMS.indexOf(zoom) + 1, ZOOMS.length - 1)])}
              >
                <ZoomIn aria-hidden />
              </Button>
            </div>
          </div>
          <ChangeLegend />
          <Canvas stop={stop} previous={dbg.previous} names={names} settings={settings} zoom={zoom} large />
        </DialogContent>
      </Dialog>
    </div>
  );
}
