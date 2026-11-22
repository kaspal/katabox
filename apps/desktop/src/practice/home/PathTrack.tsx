// Home → "Your path" (ADR 0041): a wavy track through the current window of steps. The
// track is a decorative SVG; the nodes are real links (focusable, labelled) laid over it.
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Check, ChevronDown } from "lucide-react";
import { ErrorNotice } from "@/components/ErrorNotice";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Skeleton } from "@/components/ui/skeleton";
import { useMedia, useReducedMotion } from "@/learn/engine/media";
import { cn } from "@/lib/utils";
import type { PathProgress, PathStep } from "@/practice/home/path";
import type { useHomePath } from "@/practice/home/path";
import { CARD } from "@/practice/home/TodayRow";

const W = 1240;
const MID = 32;
const AMP = 16;
/** Below this width only the current step and two on either side are shown. */
export const NARROW_QUERY = "(max-width: 899px)";

type Node =
  | { kind: "step"; step: PathStep; index: number }
  | { kind: "more"; count: number; names: string[] };

/** The x position of node i of n, in viewBox units. */
const nodeX = (i: number, n: number) => (n <= 1 ? W / 2 : 50 + (i * (W - 100)) / (n - 1));

/** An S-shaped wave from node to node. */
function wave(xs: number[]): string {
  if (xs.length === 0) return "";
  let d = `M${xs[0]} ${MID}`;
  for (let i = 1; i < xs.length; i++) {
    const a = xs[i - 1];
    const b = xs[i];
    const k = (b - a) / 3;
    d += ` C ${a + k} ${MID - AMP}, ${b - k} ${MID + AMP}, ${b} ${MID}`;
  }
  return d;
}

export function PathCard({ home }: { home: ReturnType<typeof useHomePath> }) {
  const { t } = useTranslation();
  const g = home.progress;
  return (
    <section aria-labelledby="home-path-title" className={cn(CARD, "px-6 pb-3 pt-5")} data-testid="path-card">
      <div className="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2">
        <div className="flex min-w-0 flex-wrap items-baseline gap-x-3 gap-y-1">
          <h2 id="home-path-title" className="text-lg font-semibold">
            {t("home.path.title")}
          </h2>
          {g && (
            <span className="text-[13px] text-muted-foreground" data-testid="path-summary">
              {t("home.path.summary", { path: g.path.title, done: g.done, total: g.steps.length })}
              {" · "}
              {g.current < g.steps.length ? (
                <>
                  {t("home.path.on")} <b className="text-primary">{g.steps[g.current].topic.title}</b>
                </>
              ) : (
                t("home.path.finished")
              )}
            </span>
          )}
        </div>
        <div className="flex items-center gap-3 text-[13px]">
          {home.paths.length > 1 && g && (
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="link" size="sm" className="h-auto p-0 text-[13px]" data-testid="change-path">
                  {t("home.path.change")}
                  <ChevronDown aria-hidden />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuRadioGroup value={g.path.id} onValueChange={(id) => home.setHomePath(id)}>
                  {home.paths.map((p) => (
                    <DropdownMenuRadioItem key={p.id} value={p.id}>
                      {p.title}
                    </DropdownMenuRadioItem>
                  ))}
                </DropdownMenuRadioGroup>
              </DropdownMenuContent>
            </DropdownMenu>
          )}
          <Link to="/learn" className="text-primary hover:underline">
            {t("home.path.map")} →
          </Link>
        </div>
      </div>
      {home.isPending ? (
        <Skeleton className="mt-3 h-28 w-full" />
      ) : home.error ? (
        <div className="mt-3">
          <ErrorNotice error={home.error} onRetry={home.refetch} />
        </div>
      ) : g && g.steps.length > 0 ? (
        <Track g={g} />
      ) : null}
    </section>
  );
}

function Track({ g }: { g: PathProgress }) {
  const { t } = useTranslation();
  const narrow = useMedia(NARROW_QUERY);
  const reduced = useReducedMotion();
  const nodes: Node[] = [];
  if (narrow) {
    const anchor = Math.min(g.current, g.steps.length - 1);
    const from = Math.max(0, anchor - 2);
    const to = Math.min(g.steps.length, anchor + 3);
    for (let i = from; i < to; i++) nodes.push({ kind: "step", step: g.steps[i], index: i });
  } else {
    g.window.steps.forEach((step, i) => nodes.push({ kind: "step", step, index: g.window.start + i }));
    if (g.window.moreAfter > 0) {
      const after = g.steps.slice(g.window.start + g.window.steps.length);
      nodes.push({ kind: "more", count: g.window.moreAfter, names: after.slice(0, 2).map((s) => s.topic.title) });
    }
  }
  const xs = nodes.map((_, i) => nodeX(i, nodes.length));
  // Filled up to the current node (or all the way when the path is done).
  const curPos = nodes.findIndex((n) => n.kind === "step" && n.index === g.current);
  const lastDone = nodes.reduce((last, n, i) => (n.kind === "step" && n.step.state === "done" ? i : last), -1);
  const fillTo = curPos >= 0 ? curPos : lastDone;
  return (
    <div
      className="relative mt-2 h-[120px]"
      role="group"
      aria-label={t("home.path.aria", { path: g.path.title, done: g.done, total: g.steps.length })}
      data-testid="path-track"
    >
      <svg
        viewBox={`0 0 ${W} 64`}
        preserveAspectRatio="none"
        className="absolute inset-x-0 top-0 h-16 w-full"
        aria-hidden
      >
        <path d={wave(xs)} fill="none" className="stroke-border" strokeWidth={8} strokeLinecap="round" vectorEffect="non-scaling-stroke" />
        {fillTo > 0 && (
          <path
            d={wave(xs.slice(0, fillTo + 1))}
            fill="none"
            className="stroke-primary"
            strokeWidth={8}
            strokeLinecap="round"
            vectorEffect="non-scaling-stroke"
          />
        )}
      </svg>
      <ol className="contents">
        {nodes.map((n, i) => (
          <li
            key={n.kind === "step" ? n.step.topic.id : "more"}
            className="absolute top-[15px] flex w-[140px] -translate-x-1/2 flex-col items-center"
            style={{ left: `${(xs[i] / W) * 100}%` }}
          >
            {n.kind === "step" ? <StepNode step={n.step} index={n.index} pulse={!reduced} /> : <MoreNode count={n.count} names={n.names} />}
          </li>
        ))}
      </ol>
    </div>
  );
}

const RING = 2 * Math.PI * 17;

function StepNode({ step, index, pulse }: { step: PathStep; index: number; pulse: boolean }) {
  const { t } = useTranslation();
  const { topic, state } = step;
  const label =
    state === "done"
      ? t("home.path.node.done", { topic: topic.title })
      : state === "current"
        ? t("home.path.node.current", { topic: topic.title, solved: step.solved, total: step.total })
        : t("home.path.node.todo", { topic: topic.title, n: index + 1 });
  const sub =
    step.total > 0
      ? t("home.path.solvedOf", { solved: step.solved, total: step.total })
      : t(step.viewed ? "home.path.watched" : "home.path.toWatch");
  const progress = (step.viewed ? 0.5 : 0) + (step.solved > 0 ? 0.5 : 0);
  return (
    <Link
      to="/learn/$subject/$topic"
      params={{ subject: topic.subject, topic: topic.id }}
      aria-label={label}
      aria-current={state === "current" ? "step" : undefined}
      className="group flex flex-col items-center rounded-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-card"
      data-testid={`path-node-${topic.id}`}
      data-state={state}
    >
      <span className="relative flex size-[34px] items-center justify-center">
        {state === "current" && pulse && <span aria-hidden className="kb-pulse absolute inset-0 rounded-full bg-primary" />}
        <span
          aria-hidden
          className={cn(
            "relative flex size-[34px] items-center justify-center rounded-full border-[3px] font-mono text-xs font-bold transition-transform group-hover:scale-110",
            state === "done" && "border-primary bg-primary text-primary-foreground",
            state === "current" && "border-border bg-card text-primary",
            state === "todo" && "border-border bg-card text-muted-foreground",
          )}
        >
          {state === "done" ? <Check className="size-4" strokeWidth={3} /> : index + 1}
        </span>
        {state === "current" && (
          <svg aria-hidden viewBox="0 0 40 40" className="pointer-events-none absolute -inset-[3px] size-10 -rotate-90">
            <circle
              cx="20"
              cy="20"
              r="17"
              fill="none"
              className="stroke-primary"
              strokeWidth={3}
              strokeLinecap="round"
              strokeDasharray={`${Math.max(progress, 0.08) * RING} ${RING}`}
            />
          </svg>
        )}
      </span>
      <span
        className={cn(
          "mt-[18px] line-clamp-1 max-w-full text-center text-[13px] font-semibold",
          state === "current" ? "text-foreground" : state === "done" ? "text-foreground/80" : "text-muted-foreground",
        )}
      >
        {topic.title}
      </span>
      <span className="text-[11px] text-muted-foreground">{sub}</span>
    </Link>
  );
}

function MoreNode({ count, names }: { count: number; names: string[] }) {
  const { t } = useTranslation();
  return (
    <Link
      to="/learn"
      aria-label={t("home.path.moreAria", { count })}
      className="flex flex-col items-center rounded-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      data-testid="path-more"
    >
      <span
        aria-hidden
        className="flex size-[34px] items-center justify-center rounded-full border-[3px] border-border/60 bg-card font-mono text-xs font-bold text-muted-foreground"
      >
        …
      </span>
      <span className="mt-[18px] text-[13px] font-semibold text-muted-foreground">{t("home.path.more", { count })}</span>
      <span className="line-clamp-1 max-w-full text-[11px] text-muted-foreground">{names.join(", ")}…</span>
    </Link>
  );
}
