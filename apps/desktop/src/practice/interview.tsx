// Mock interview pieces (ADR 0020): the wall-clock timer and the scorecard.
import { useEffect, useState } from "react";
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { CheckCircle2, CircleDashed, Timer } from "lucide-react";
import type { InterviewResult } from "@/bindings";
import { formatClock, useInterviewActive, useInvalidateInterview } from "@/lib/modes";
import { cn } from "@/lib/utils";

/** Seconds left until `endsAt` (UTC ms), ticking every second from the wall clock. */
export function useRemaining(endsAt: number | null): number | null {
  const [, setTick] = useState(0);
  useEffect(() => {
    if (endsAt === null) return;
    const id = window.setInterval(() => setTick((n) => n + 1), 1000);
    return () => window.clearInterval(id);
  }, [endsAt]);
  // Read the wall clock at render: the timer is never ahead of the deadline.
  return endsAt === null ? null : Math.max(0, Math.ceil((endsAt - Date.now()) / 1000));
}

/** The running interview's clock, for the top bar and the workspace header. */
export function InterviewTimer({ compact = false }: { compact?: boolean }) {
  const { t } = useTranslation();
  const active = useInterviewActive();
  const invalidate = useInvalidateInterview();
  const remaining = useRemaining(active.data?.ends_at ?? null);
  // Time is up: ask kb-app, which ends the session and unlocks everything.
  useEffect(() => {
    if (remaining === 0) invalidate();
  }, [remaining === 0]);
  if (!active.data || remaining === null) return null;
  const time = formatClock(remaining);
  const low = remaining < 5 * 60;
  return (
    <Link
      to="/interview"
      className={cn(
        "inline-flex items-center gap-1.5 rounded-md border px-2 py-1 font-mono text-sm tabular-nums",
        low ? "border-verdict-wrong/50 text-verdict-wrong" : "text-foreground",
      )}
      aria-label={t("practice.interview.timerLabel", { time })}
      data-testid="interview-timer"
      data-low={low || undefined}
    >
      <Timer aria-hidden className="size-4" />
      {time}
      {!compact && <span className="font-sans text-xs text-muted-foreground">{t("practice.interview.running")}</span>}
    </Link>
  );
}

export function Scorecard({ result }: { result: InterviewResult }) {
  const { t } = useTranslation();
  const title = (id: string) =>
    result.titles.find(([pid]) => pid === id)?.[1] ?? t("practice.interview.unavailable");
  return (
    <div className="space-y-3" data-testid="scorecard">
      <div className="flex flex-wrap gap-x-6 gap-y-1 text-sm">
        <span className="font-semibold">
          {t("practice.interview.solved", { solved: result.solved, total: result.problems.length })}
        </span>
        <span>{t("practice.interview.firstTry", { count: result.first_try })}</span>
        <span className="tabular-nums">{t("practice.interview.used", { time: formatClock(result.used_s) })}</span>
      </div>
      <ul className="divide-y rounded-lg border">
        {result.problems.map((p) => (
          <li key={p.problem_id} className="flex flex-wrap items-center gap-3 px-3 py-2 text-sm">
            {p.solved ? (
              <CheckCircle2 aria-hidden className="size-4 text-verdict-accepted" />
            ) : (
              <CircleDashed aria-hidden className="size-4 text-muted-foreground" />
            )}
            <Link
              to="/problems/$problemId"
              params={{ problemId: p.problem_id }}
              className="min-w-0 flex-1 truncate font-medium hover:underline"
            >
              {title(p.problem_id)}
            </Link>
            <span className="tabular-nums text-muted-foreground">
              {p.solved && p.time_to_solve_s !== null
                ? t("practice.interview.problemSolved", { time: formatClock(p.time_to_solve_s) })
                : t("practice.interview.problemUnsolved")}
            </span>
            <span className="text-muted-foreground">{t("practice.interview.submits", { count: p.submits })}</span>
            {p.first_try && (
              <span className="rounded border border-verdict-accepted/40 px-1.5 text-xs text-verdict-accepted">
                {t("practice.interview.firstTryBadge")}
              </span>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
}
