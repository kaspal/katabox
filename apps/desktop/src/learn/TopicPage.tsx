// A Learn topic (ADR 0029, mockup "Main"): the picker, the stage and the Theory / Code /
// Practice tabs, in resizable panels (stacked in narrow windows). Lazy-loaded with the
// engine, the scenes, the tracers and motion.
import { useCallback, useEffect, useRef, useState } from "react";
import { Link, getRouteApi } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { GitCompare, PanelLeftClose, PanelLeftOpen } from "lucide-react";
import type { JsonValue, LearnTopicView } from "@/bindings";
import { Button } from "@/components/ui/button";
import { ErrorNotice } from "@/components/ErrorNotice";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { isAppError } from "@/lib/errors";
import { useLearnCatalog, useLearnTopic } from "@/lib/queries";
import { useSetting } from "@/lib/settings";
import { canCompare, comparePartners } from "./engine/compare";
import { playerKeyAction } from "./engine/keys";
import { useMedia } from "./engine/media";
import { useLastTopic, useLearnSpeed, useViewed } from "./engine/progress";
import type { TracerDef } from "./engine/types";
import { usePlayer } from "./engine/usePlayer";
import { useTrace } from "./engine/useTrace";
import { CodeTab, type SampleLang } from "./panels/CodeTab";
import { PracticeTab } from "./panels/PracticeTab";
import { TheoryTab } from "./panels/TheoryTab";
import { CompareView } from "./CompareView";
import { Stage } from "./Stage";
import { TopicPicker } from "./TopicPicker";
import { TRACERS } from "./tracers";

const route = getRouteApi("/learn/$subject/$topic");
const NARROW = "(max-width: 999px)";

type Layouts = { main?: Record<string, number> };
const parseLayouts = (v: JsonValue | null): Layouts | undefined =>
  typeof v === "object" && v !== null && !Array.isArray(v) ? (v as Layouts) : undefined;

export function TopicPage() {
  const { t } = useTranslation();
  const { subject, topic } = route.useParams();
  const catalog = useLearnCatalog();
  const view = useLearnTopic(topic);
  const def = TRACERS[topic];
  const { viewed, markViewed } = useViewed();

  const missing =
    (view.isError && isAppError(view.error) && view.error.code === "not_found") ||
    (view.data && (view.data.topic.subject !== subject || !def));
  if (missing) {
    return (
      <section className="flex flex-col items-start gap-3 p-8" data-testid="topic-not-found">
        <h1 className="text-xl font-semibold">{t("learn.topic.not_found")}</h1>
        <p className="text-muted-foreground">{t("learn.topic.not_found_body")}</p>
        <Button asChild variant="outline">
          <Link to="/learn">{t("learn.topic.back")}</Link>
        </Button>
      </section>
    );
  }
  if (view.isError) return <ErrorNotice error={view.error} onRetry={() => void view.refetch()} />;
  if (!view.data || !def) return null;

  return (
    <TopicView
      key={topic}
      data={view.data}
      def={def}
      titles={Object.fromEntries((catalog.data ?? []).flatMap((s) => s.topics.map((x) => [x.id, x.title])))}
      picker={
        <TopicPicker subjects={catalog.data ?? []} current={{ subject, topic }} viewed={viewed} />
      }
      onViewed={() => markViewed(topic)}
    />
  );
}

function TopicView({
  data,
  def,
  titles,
  picker,
  onViewed,
}: {
  data: LearnTopicView;
  def: TracerDef;
  /** Topic titles by id (compare mode names both topics). */
  titles: Record<string, string>;
  picker: React.ReactNode;
  onViewed: () => void;
}) {
  const { t } = useTranslation();
  const { topic, problems } = data;
  const narrow = useMedia(NARROW);
  const [pickerOpen, setPickerOpen] = useState(false);
  const [tab, setTab] = useState("code");
  const [lang, setLang] = useState<SampleLang>("python");
  const [layouts, setLayouts] = useSetting<Layouts>("learn.layout", {}, parseLayouts, { mirror: true });
  const layoutTimer = useRef<number | undefined>(undefined);
  const [speed, setSpeed] = useLearnSpeed();
  const [last, setLast] = useLastTopic();
  const trace = useTrace(def, topic.default_input);
  const [compare, setCompare] = useState<TracerDef | null>(null);
  const compareRef = useRef(compare);
  compareRef.current = compare;
  const partners = comparePartners(def, Object.values(TRACERS), trace.values);
  const maxValue = topic.input.max;

  // Resume where Continue left off, only for the default input.
  const initialStep = last?.topic === topic.id ? last.step : 0;
  const save = useCallback(
    (step: number) =>
      setLast({ subject: topic.subject, topic: topic.id, step, total: trace.steps.length }),
    [setLast, topic.subject, topic.id, trace.steps.length],
  );
  const player = usePlayer(trace.steps.length, `${topic.id}:${trace.version}`, {
    initialStep,
    speed,
    onFinish: onViewed,
    onSpeed: (s) => setSpeed(s),
    onPause: save,
  });
  const step = trace.steps[player.step];

  // Remember the step when leaving the page.
  const saveRef = useRef(save);
  saveRef.current = save;
  const stepRef = useRef(player.step);
  stepRef.current = player.step;
  useEffect(() => () => saveRef.current(stepRef.current), []);

  // Shortcuts while the page is open; focused controls keep their own keys.
  const playerRef = useRef(player);
  playerRef.current = player;
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (compareRef.current) return; // compare mode drives its own player
      const action = playerKeyAction(e);
      if (!action) return;
      e.preventDefault();
      playerRef.current[action]();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  const values = trace.values;
  if (!step) return null;

  const header = (
    <div className="flex flex-wrap items-start justify-between gap-3">
      <div className="flex flex-col gap-2">
        <nav aria-label="breadcrumb" className="text-sm text-muted-foreground">
          <Link to="/learn" className="hover:text-foreground">
            {t("learn.title")}
          </Link>
          <span aria-hidden> / </span>
          <span>{topic.subject_name}</span>
        </nav>
        <h1 className="text-2xl font-bold tracking-tight">{topic.title}</h1>
        <ul className="flex flex-wrap gap-1.5 text-xs" aria-label={t("learn.tabs.label")}>
          {topic.tags.map((tag) => (
            <li key={tag} className="rounded-full bg-accent px-2 py-1 font-medium">
              {t(`learn.tags.${tag}` as "learn.title")}
            </li>
          ))}
          <li className="rounded-full bg-muted px-2 py-1 font-mono">
            {topic.subject === "sorting"
              ? t("learn.topic.average", { cost: topic.complexity.average })
              : t("learn.topic.cost", { cost: topic.complexity.average })}
          </li>
          {/* Stability and in-place are sorting traits; other subjects leave them out. */}
          {topic.subject === "sorting" && (
            <>
              <li className="rounded-full bg-muted px-2 py-1">
                {topic.complexity.in_place ? t("learn.topic.in_place") : t("learn.topic.not_in_place")}
              </li>
              <li className="rounded-full bg-muted px-2 py-1">
                {topic.complexity.stable ? t("learn.topic.stable") : t("learn.topic.not_stable")}
              </li>
            </>
          )}
        </ul>
      </div>
      <div className="flex gap-2">
        {canCompare(def, Object.values(TRACERS)) ? (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="outline" data-testid="compare-button">
                <GitCompare aria-hidden className="size-4" />
                {t("learn.topic.compare")}
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuLabel>{t("learn.compare.pick")}</DropdownMenuLabel>
              {partners.length === 0 && (
                <DropdownMenuItem disabled>{t("learn.compare.none_fit")}</DropdownMenuItem>
              )}
              {partners.map((p) => (
                <DropdownMenuItem key={p.id} onSelect={() => setCompare(p)} data-partner={p.id}>
                  {titles[p.id] ?? p.id}
                </DropdownMenuItem>
              ))}
            </DropdownMenuContent>
          </DropdownMenu>
        ) : (
          <Tooltip>
            <TooltipTrigger asChild>
              {/* A disabled button cannot show a tooltip; the wrapper can. */}
              <span tabIndex={0} className="rounded-md">
                <Button variant="outline" disabled aria-describedby="compare-none" className="pointer-events-none">
                  <GitCompare aria-hidden className="size-4" />
                  {t("learn.topic.compare")}
                </Button>
              </span>
            </TooltipTrigger>
            <TooltipContent id="compare-none">{t("learn.compare.none")}</TooltipContent>
          </Tooltip>
        )}
        <Button onClick={() => setTab("practice")} disabled={problems.length === 0}>
          {t("learn.topic.practice", { count: problems.length })}
        </Button>
      </div>
    </div>
  );

  const stage = compare ? (
    <CompareView
      key={compare.id}
      a={def}
      b={compare}
      titles={[topic.title, titles[compare.id] ?? compare.id]}
      initial={trace.values}
      input={topic.input}
      presets={topic.presets}
      speed={speed}
      onSpeed={(s) => setSpeed(s)}
      onExit={() => setCompare(null)}
    />
  ) : (
    <Stage
      def={def}
      step={step}
      total={trace.steps.length}
      player={player}
      maxValue={maxValue}
      input={topic.input}
      presets={topic.presets}
      values={values}
      error={trace.error}
      onPreset={trace.applyPreset}
      onCustom={trace.applyCustom}
      onClearError={trace.clearError}
    />
  );

  const tabs = (
    <Tabs value={tab} onValueChange={setTab} className="flex h-full min-h-0 flex-col gap-0">
      <TabsList variant="line" aria-label={t("learn.tabs.label")} className="w-full justify-start border-b px-3">
        <TabsTrigger value="theory">{t("learn.tabs.theory")}</TabsTrigger>
        <TabsTrigger value="code">{t("learn.tabs.code")}</TabsTrigger>
        <TabsTrigger value="practice">{t("learn.tabs.practice")}</TabsTrigger>
      </TabsList>
      <div className="min-h-0 flex-1 overflow-auto">
        <TabsContent value="theory">
          <TheoryTab theory={topic.theory} />
        </TabsContent>
        <TabsContent value="code">
          <CodeTab samples={topic.samples} step={step} def={def} lang={lang} onLang={setLang} />
        </TabsContent>
        <TabsContent value="practice">
          <PracticeTab quiz={topic.quiz} problems={problems} />
        </TabsContent>
      </div>
    </Tabs>
  );

  if (narrow) {
    return (
      <div className="flex flex-col gap-4 p-4" data-testid="learn-narrow">
        <div className="rounded-xl border bg-card">
          <Button
            variant="ghost"
            className="w-full justify-start"
            aria-expanded={pickerOpen}
            onClick={() => setPickerOpen((o) => !o)}
          >
            {pickerOpen ? <PanelLeftClose aria-hidden className="size-4" /> : <PanelLeftOpen aria-hidden className="size-4" />}
            {pickerOpen ? t("learn.picker.hide") : t("learn.picker.show")}
          </Button>
          {pickerOpen && <div className="max-h-80 border-t">{picker}</div>}
        </div>
        {header}
        {stage}
        <div className="rounded-xl border bg-card">{tabs}</div>
      </div>
    );
  }

  return (
    <ResizablePanelGroup
      orientation="horizontal"
      className="h-full min-h-0"
      defaultLayout={layouts.main}
      onLayoutChanged={(l) => {
        window.clearTimeout(layoutTimer.current);
        layoutTimer.current = window.setTimeout(() => setLayouts({ ...layouts, main: l }), 400);
      }}
      data-testid="learn-panels"
    >
      <ResizablePanel id="topics" defaultSize="18" minSize="12" collapsible collapsedSize="0">
        <aside className="h-full border-r bg-card">{picker}</aside>
      </ResizablePanel>
      <ResizableHandle withHandle aria-label={t("learn.topic.resize")} />
      <ResizablePanel id="stage" defaultSize="52" minSize="35">
        <div className="flex h-full min-h-0 flex-col gap-4 overflow-auto p-5">
          {header}
          {stage}
        </div>
      </ResizablePanel>
      <ResizableHandle withHandle aria-label={t("learn.topic.resize")} />
      <ResizablePanel id="details" defaultSize="30" minSize="20">
        <aside className="h-full border-l bg-card">{tabs}</aside>
      </ResizablePanel>
    </ResizablePanelGroup>
  );
}
