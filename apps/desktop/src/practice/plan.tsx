// Study plans (expansion M2, ADR 0033): Home's "Plan" card (today's items) and the
// Progress → Plan tab (make a plan, follow it, export it to a calendar).
import { useState } from "react";
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { CalendarCheck, CalendarDays, Check, ChevronsRight, Download, SkipForward, Undo2 } from "lucide-react";
import type { PlanItemView, PlanView } from "@/bindings";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel, usePatternLabel } from "@/components/Semantic";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Progress as Bar } from "@/components/ui/progress";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { api } from "@/lib/api";
import { useInvalidatePlan, usePlan } from "@/lib/plan";
import { usePatterns } from "@/lib/practice";
import { usePrefs } from "@/lib/prefs";
import { useToday } from "@/lib/today";
import { cn } from "@/lib/utils";
import { CompactCard, HomeCard } from "@/practice/HomeCards";
import { formatDay } from "@/practice/shared";

const WEEKS = [1, 2, 4, 6, 8, 12] as const;
const MINUTES = [20, 30, 45, 60, 90] as const;

/** What an item is, in words, with a link to it. */
function ItemLabel({ item }: { item: PlanItemView }) {
  const { t } = useTranslation();
  if (item.kind === "problem" && item.reference) {
    return (
      <span className="flex min-w-0 items-center gap-2">
        <Link to="/problems/$problemId" params={{ problemId: item.reference }} className="truncate font-medium hover:underline">
          {item.title ?? item.reference}
        </Link>
        {item.difficulty && <DifficultyLabel difficulty={item.difficulty} />}
      </span>
    );
  }
  if (item.kind === "topic" && item.reference && item.subject) {
    return (
      <span className="min-w-0 truncate">
        {t("plan.kind.topic")}:{" "}
        <Link to="/learn/$subject/$topic" params={{ subject: item.subject, topic: item.reference }} className="font-medium hover:underline">
          {item.title ?? item.reference}
        </Link>
      </span>
    );
  }
  if (item.kind === "kata") {
    return (
      <Link to="/kata" className="font-medium hover:underline">
        {t("plan.kind.kata")}
      </Link>
    );
  }
  if (item.kind === "review") {
    return (
      <Link to="/" className="font-medium hover:underline">
        {t("plan.kind.review")}
      </Link>
    );
  }
  return <span>{item.title ?? item.reference}</span>;
}

/** One item with its state and actions: done, skip, later; done and skipped can be undone. */
function ItemRow({ plan, item }: { plan: PlanView; item: PlanItemView }) {
  const { t } = useTranslation();
  const invalidate = useInvalidatePlan();
  const [error, setError] = useState<unknown>(null);
  const act = async (f: () => Promise<unknown>) => {
    setError(null);
    try {
      await f();
      invalidate();
    } catch (e) {
      setError(e);
    }
  };
  const pending = item.status === "pending";
  return (
    <li className="flex flex-wrap items-center gap-2 py-1 text-sm" data-testid="plan-item" data-status={item.status}>
      {item.status === "done" ? (
        <Check aria-hidden className="size-4 text-verdict-accepted" />
      ) : item.status === "skipped" ? (
        <SkipForward aria-hidden className="size-4 text-muted-foreground" />
      ) : (
        <span aria-hidden className="size-4" />
      )}
      <span className={cn("min-w-0 flex-1", !pending && "text-muted-foreground")}>
        <ItemLabel item={item} />
      </span>
      <span className="text-xs tabular-nums text-muted-foreground">{t("plan.minutes", { count: item.minutes })}</span>
      <span className="sr-only">{t(`plan.status.${item.status}` as "plan.status.pending")}</span>
      {pending ? (
        <>
          <Button size="sm" variant="ghost" className="h-7 px-2" onClick={() => void act(() => api.setPlanItem(plan.created_at, item.position, "done"))}>
            <Check aria-hidden />
            {t("plan.done")}
          </Button>
          <Button size="sm" variant="ghost" className="h-7 px-2" onClick={() => void act(() => api.setPlanItem(plan.created_at, item.position, "skipped"))}>
            <SkipForward aria-hidden />
            {t("plan.skip")}
          </Button>
          <Button size="sm" variant="ghost" className="h-7 px-2" onClick={() => void act(() => api.reschedulePlanItem(plan.created_at, item.position))}>
            <ChevronsRight aria-hidden />
            {t("plan.later")}
          </Button>
        </>
      ) : (
        <Button size="sm" variant="ghost" className="h-7 px-2" onClick={() => void act(() => api.setPlanItem(plan.created_at, item.position, "pending"))}>
          <Undo2 aria-hidden />
          {t("plan.undo")}
        </Button>
      )}
      {error !== null && <ErrorNotice error={error} />}
    </li>
  );
}

/** Home: today's part of the active plan (and anything overdue). */
export function PlanCard({ compact = false }: { compact?: boolean }) {
  const { t } = useTranslation();
  const today = useToday();
  const plan = usePlan();
  if (compact) {
    const items = plan.data ? todayItems(plan.data, today).filter((i) => i.status === "pending") : [];
    return (
      <CompactCard
        title={t("plan.card")}
        icon={<CalendarCheck aria-hidden className="size-5" />}
        testId="plan-card"
        busy={plan.isPending}
        error={plan.error}
        line={
          !plan.data
            ? t("home.fresh.planNone")
            : items.length === 0
              ? t("home.fresh.planNothing")
              : t("home.fresh.planToday", { count: items.length })
        }
        action={
          <Button asChild size="sm" variant="outline">
            <Link to="/progress" search={{ tab: "plan" }}>
              {t("home.fresh.planOpen")}
            </Link>
          </Button>
        }
      />
    );
  }
  return (
    <HomeCard title={t("plan.card")} icon={<CalendarCheck aria-hidden className="size-4" />} testId="plan-card">
      {plan.isPending ? (
        <Skeleton className="h-16 w-full" />
      ) : plan.isError ? (
        <ErrorNotice error={plan.error} />
      ) : !plan.data ? (
        <>
          <p className="text-sm text-muted-foreground">{t("plan.none")}</p>
          <Button asChild size="sm" variant="outline" className="self-start">
            <Link to="/progress" search={{ tab: "plan" }}>
              {t("plan.make")}
            </Link>
          </Button>
        </>
      ) : (
        <TodayList plan={plan.data} today={today} />
      )}
    </HomeCard>
  );
}

/** Today's items, plus earlier ones still pending. */
function todayItems(plan: PlanView, today: string) {
  return plan.items.filter((i) => i.day === today || (i.day < today && i.status === "pending"));
}

function TodayList({ plan, today }: { plan: PlanView; today: string }) {
  const { t } = useTranslation();
  const items = todayItems(plan, today);
  if (items.length === 0) return <p className="text-sm text-muted-foreground">{t("plan.nothingToday")}</p>;
  return (
    <ul className="divide-y" data-testid="plan-today">
      {items.map((i) => (
        <ItemRow key={i.position} plan={plan} item={i} />
      ))}
    </ul>
  );
}

/** Progress → Plan. */
export function PlanTab() {
  const { t, i18n } = useTranslation();
  const plan = usePlan();
  const invalidate = useInvalidatePlan();
  const [error, setError] = useState<unknown>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [making, setMaking] = useState(false);
  if (plan.isPending) return <Skeleton className="h-40 w-full" />;
  if (plan.isError) return <ErrorNotice error={plan.error} />;
  const p = plan.data;
  if (!p || making) return <PlanForm onDone={() => setMaking(false)} canCancel={!!p && making} />;
  const exportIcs = async () => {
    setError(null);
    setNotice(null);
    try {
      const saved = await api.exportPlanIcs(
        {
          title: t("plan.ics.title"),
          review: t("plan.kind.review"),
          kata: t("plan.kind.kata"),
          topic: `${t("plan.kind.topic")}: `,
          minutes: t("plan.ics.min"),
        },
        i18n.language === "es" ? "es" : "en",
      );
      if (saved) setNotice(t("plan.ics.saved"));
    } catch (e) {
      setError(e);
    }
  };
  const byDay = new Map<string, PlanItemView[]>();
  for (const it of p.items) byDay.set(it.day, [...(byDay.get(it.day) ?? []), it]);
  return (
    <div className="space-y-4" data-testid="plan-tab">
      <div className="flex flex-wrap items-center gap-3">
        <span className="text-sm">
          {t("plan.range", { from: formatDay(p.start_day, i18n.language), to: formatDay(p.end_day, i18n.language) })}
        </span>
        <span className="text-sm tabular-nums" data-testid="plan-progress">
          {t("plan.progress", { done: p.done, total: p.total })}
        </span>
        <Bar value={p.total === 0 ? 0 : (100 * p.done) / p.total} className="w-40" aria-label={t("plan.progressLabel")} />
        <div className="ml-auto flex gap-2">
          <Button size="sm" variant="outline" onClick={() => void exportIcs()}>
            <Download aria-hidden />
            {t("plan.ics.export")}
          </Button>
          <Button size="sm" variant="outline" onClick={() => setMaking(true)}>
            <CalendarDays aria-hidden />
            {t("plan.new")}
          </Button>
          <Button
            size="sm"
            variant="ghost"
            onClick={async () => {
              await api.archivePlan(p.created_at);
              invalidate();
            }}
          >
            {t("plan.archive")}
          </Button>
        </div>
      </div>
      {p.left_over > 0 && <p className="text-sm text-muted-foreground">{t("plan.leftOver", { count: p.left_over })}</p>}
      {error !== null && <ErrorNotice error={error} />}
      {notice && (
        <p role="status" className="text-sm">
          {notice}
        </p>
      )}
      <ol className="space-y-3">
        {[...byDay.entries()].map(([day, items]) => (
          <li key={day} className="rounded-lg border bg-card px-4 py-2">
            <h3 className="text-sm font-semibold">
              {formatDay(day, i18n.language)}
              <span className="ml-2 text-xs font-normal text-muted-foreground">
                {t("plan.minutes", { count: items.reduce((s, i) => s + i.minutes, 0) })}
              </span>
            </h3>
            <ul className="divide-y">
              {items.map((i) => (
                <ItemRow key={i.position} plan={p} item={i} />
              ))}
            </ul>
          </li>
        ))}
      </ol>
    </div>
  );
}

function PlanForm({ onDone, canCancel }: { onDone: () => void; canCancel: boolean }) {
  const { t } = useTranslation();
  const { lang } = usePrefs();
  const today = useToday();
  const patternLabel = usePatternLabel();
  const patterns = usePatterns();
  const invalidate = useInvalidatePlan();
  const [weeks, setWeeks] = useState("4");
  const [minutes, setMinutes] = useState("30");
  const [all, setAll] = useState(true);
  const [picked, setPicked] = useState<string[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>(null);
  const make = async () => {
    setBusy(true);
    setError(null);
    try {
      await api.createPlan(
        { patterns: all ? [] : picked, weeks: Number(weeks), minutes_per_day: Number(minutes), today },
        lang,
      );
      invalidate();
      onDone();
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  };
  return (
    <section className="max-w-2xl space-y-4 rounded-lg border bg-card p-5" data-testid="plan-form">
      <h2 className="font-semibold">{t("plan.formTitle")}</h2>
      <p className="text-sm text-muted-foreground">{t("plan.formIntro")}</p>
      <div className="grid grid-cols-[160px_1fr] items-center gap-3 text-sm">
        <span className="text-muted-foreground">{t("plan.weeks")}</span>
        <Select value={weeks} onValueChange={setWeeks}>
          <SelectTrigger size="sm" className="w-40" aria-label={t("plan.weeks")}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {WEEKS.map((w) => (
              <SelectItem key={w} value={String(w)}>
                {t("plan.weeksOption", { count: w })}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <span className="text-muted-foreground">{t("plan.perDay")}</span>
        <Select value={minutes} onValueChange={setMinutes}>
          <SelectTrigger size="sm" className="w-40" aria-label={t("plan.perDay")}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {MINUTES.map((m) => (
              <SelectItem key={m} value={String(m)}>
                {t("plan.minutes", { count: m })}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <span className="self-start text-muted-foreground">{t("plan.goal")}</span>
        <div className="space-y-2">
          <label className="flex items-center gap-2">
            <Checkbox checked={all} onCheckedChange={(on) => setAll(on === true)} aria-label={t("plan.allPatterns")} />
            {t("plan.allPatterns")}
          </label>
          {!all && (
            <div className="grid max-h-56 grid-cols-2 gap-1 overflow-auto rounded-md border p-2">
              {(patterns.data ?? []).map((p) => (
                <label key={p.pattern} className="flex items-center gap-2 text-xs">
                  <Checkbox
                    aria-label={patternLabel(p.pattern)}
                    checked={picked.includes(p.pattern)}
                    onCheckedChange={(on) =>
                      setPicked((cur) => (on ? [...cur, p.pattern] : cur.filter((x) => x !== p.pattern)))
                    }
                  />
                  {patternLabel(p.pattern)}
                </label>
              ))}
            </div>
          )}
        </div>
      </div>
      {error !== null && <ErrorNotice error={error} />}
      <div className="flex gap-2">
        <Button onClick={() => void make()} disabled={busy || (!all && picked.length === 0)}>
          {t("plan.create")}
        </Button>
        {canCancel && (
          <Button variant="ghost" onClick={onDone}>
            {t("plan.cancel")}
          </Button>
        )}
      </div>
    </section>
  );
}
