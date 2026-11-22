// Focus sessions (expansion M10, ADR 0037): work and break periods on the wall clock, a
// chip in the top bar, an in-app notice when the phase changes (no OS notifications),
// and the finished sessions with what happened in them on Progress → Overview.
import { useEffect, useRef, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { Coffee, Square, Timer } from "lucide-react";
import type { FocusCurrent } from "@/bindings";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { api } from "@/lib/api";
import { formatClock } from "@/lib/modes";
import { useSetting } from "@/lib/settings";
import { useRemaining } from "@/practice/interview";

const keys = { current: ["focus_current"] as const, sessions: ["focus_sessions"] as const };

export function useFocusCurrent() {
  return useQuery({ queryKey: keys.current, queryFn: () => api.focusCurrent() });
}

export function useFocusSessions() {
  return useQuery({ queryKey: keys.sessions, queryFn: () => api.focusSessions() });
}

export function useFocusActions() {
  const qc = useQueryClient();
  const done = () => {
    void qc.invalidateQueries({ queryKey: keys.current });
    void qc.invalidateQueries({ queryKey: keys.sessions });
  };
  const start = useMutation({
    mutationFn: ({ work, brk }: { work: number; brk: number }) => api.focusStart(work, brk),
    onSuccess: done,
  });
  const stop = useMutation({ mutationFn: () => api.focusStop(), onSuccess: done });
  return { start, stop };
}

const num = (v: unknown, d: number, lo: number, hi: number) =>
  typeof v === "number" && Number.isInteger(v) && v >= lo && v <= hi ? v : d;
export const useFocusWork = () => useSetting<number>("focus.work", 25, (v) => num(v, 25, 5, 120));
export const useFocusBreak = () => useSetting<number>("focus.break", 5, (v) => num(v, 5, 1, 60));

/** The phase and when it ends (UTC ms), from the start and the two lengths. */
export function phaseOf(c: FocusCurrent, now: number): { phase: "work" | "break"; endsAt: number; cycle: number } {
  const work = c.work_min * 60_000;
  const cycle = work + c.break_min * 60_000;
  const elapsed = Math.max(0, now - c.started_at);
  const k = Math.floor(elapsed / cycle);
  const into = elapsed % cycle;
  const base = c.started_at + k * cycle;
  return into < work
    ? { phase: "work", endsAt: base + work, cycle: k + 1 }
    : { phase: "break", endsAt: base + cycle, cycle: k + 1 };
}

/** The top bar chip: phase and time left; Stop; a notice when the phase changes. */
export function FocusTimer() {
  const { t } = useTranslation();
  const current = useFocusCurrent();
  const { stop } = useFocusActions();
  const c = current.data ?? null;
  const p = c ? phaseOf(c, Date.now()) : null;
  const remaining = useRemaining(p?.endsAt ?? null);
  const [notice, setNotice] = useState<string | null>(null);
  const last = useRef<string | null>(null);
  const key = p ? `${p.phase}:${p.cycle}` : null;
  useEffect(() => {
    if (key && last.current && last.current !== key && p) {
      setNotice(t(p.phase === "work" ? "focus.toWork" : "focus.toBreak"));
      const id = window.setTimeout(() => setNotice(null), 8000);
      last.current = key;
      return () => window.clearTimeout(id);
    }
    last.current = key;
  }, [key]);
  if (!c || !p || remaining === null) return null;
  const Icon = p.phase === "work" ? Timer : Coffee;
  const time = formatClock(remaining);
  return (
    <>
      <span
        className="inline-flex items-center gap-1.5 rounded-md border px-2 py-1 text-sm"
        data-testid="focus-timer"
        data-phase={p.phase}
      >
        <Icon aria-hidden className="size-4" />
        <span className="font-mono tabular-nums" aria-label={t(`focus.chip.${p.phase}`, { time })}>
          {time}
        </span>
        <span className="text-xs text-muted-foreground">{t(`focus.phase.${p.phase}`)}</span>
        <Button
          size="icon"
          variant="ghost"
          className="size-6"
          onClick={() => stop.mutate()}
          aria-label={t("focus.stop")}
          data-testid="focus-stop"
        >
          <Square aria-hidden className="size-3.5" />
        </Button>
      </span>
      {notice && (
        <div
          role="status"
          data-testid="focus-notice"
          className="fixed right-4 bottom-4 z-50 flex items-center gap-2 rounded-lg border bg-card px-4 py-3 text-sm shadow-lg"
        >
          <Icon aria-hidden className="size-4" />
          {notice}
        </div>
      )}
    </>
  );
}

/** Progress → Overview: start a session, and the last sessions with their summary. */
export function FocusCard() {
  const { t, i18n } = useTranslation();
  const current = useFocusCurrent();
  const sessions = useFocusSessions();
  const { start, stop } = useFocusActions();
  const [work, setWork] = useFocusWork();
  const [brk, setBreak] = useFocusBreak();
  const running = current.data ?? null;
  const fmt = new Intl.DateTimeFormat(i18n.language, { dateStyle: "medium", timeStyle: "short" });
  return (
    <div className="space-y-3" data-testid="focus-card">
      <div className="flex flex-wrap items-end gap-3">
        <label className="space-y-1 text-xs">
          <span className="text-muted-foreground">{t("focus.work")}</span>
          <Input
            type="number"
            min={5}
            max={120}
            value={work}
            onChange={(e) => setWork(Number.parseInt(e.target.value, 10) || 25)}
            className="h-8 w-20"
            disabled={running !== null}
          />
        </label>
        <label className="space-y-1 text-xs">
          <span className="text-muted-foreground">{t("focus.break")}</span>
          <Input
            type="number"
            min={1}
            max={60}
            value={brk}
            onChange={(e) => setBreak(Number.parseInt(e.target.value, 10) || 5)}
            className="h-8 w-20"
            disabled={running !== null}
          />
        </label>
        {running ? (
          <Button size="sm" variant="outline" onClick={() => stop.mutate()} data-testid="focus-card-stop">
            <Square aria-hidden />
            {t("focus.stop")}
          </Button>
        ) : (
          <Button size="sm" onClick={() => start.mutate({ work, brk })} data-testid="focus-start">
            <Timer aria-hidden />
            {t("focus.start")}
          </Button>
        )}
      </div>
      {(sessions.data?.length ?? 0) === 0 ? (
        <p className="text-sm text-muted-foreground">{t("focus.none")}</p>
      ) : (
        <ul className="divide-y rounded-lg border text-sm" data-testid="focus-sessions">
          {sessions.data!.slice(0, 5).map((s) => (
            <li key={s.started_at} className="flex flex-wrap items-center gap-x-4 gap-y-1 px-3 py-2">
              <span className="font-medium">{fmt.format(new Date(s.started_at))}</span>
              <span className="tabular-nums text-muted-foreground">
                {t("focus.length", { minutes: Math.round((s.ended_at - s.started_at) / 60_000), cycles: s.cycles })}
              </span>
              <span>{t("focus.summary", s.summary)}</span>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
