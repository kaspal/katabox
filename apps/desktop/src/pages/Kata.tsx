// Kata drills (expansion M1, ADR 0033): pick a set of solved problems, re-solve them
// from memory against the clock, with the editor starting from the starter code.
import { useState } from "react";
import { Link, useNavigate } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowRight, CheckCircle2, CircleDashed, Dumbbell, Square } from "lucide-react";
import type { KataKind, KataView } from "@/bindings";
import { Page } from "@/components/Page";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel, usePatternLabel } from "@/components/Semantic";
import { Button } from "@/components/ui/button";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { api } from "@/lib/api";
import { formatDuration, useInvalidateKata, useKataOptions } from "@/lib/kata";
import { usePrefs } from "@/lib/prefs";
import { useToday } from "@/lib/today";
import { KataClock } from "@/practice/kata";

export function Kata() {
  const { t } = useTranslation();
  const { lang } = usePrefs();
  const today = useToday();
  const navigate = useNavigate();
  const options = useKataOptions();
  const invalidate = useInvalidateKata();
  const patternLabel = usePatternLabel();
  const [pattern, setPattern] = useState<string>("");
  const [list, setList] = useState<string>("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>(null);
  const [ended, setEnded] = useState<KataView | null>(null);

  const start = async (kind: KataKind, source: string | null) => {
    setBusy(true);
    setError(null);
    setEnded(null);
    try {
      const view = await api.startKata({ kind, source, problems: [], today }, lang);
      invalidate();
      const first = view.problems[0];
      if (first) {
        void navigate({
          to: "/problems/$problemId",
          params: { problemId: first.id },
          search: { kata: view.started_at },
        });
      }
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  };
  const end = async () => {
    setBusy(true);
    try {
      setEnded(await api.endKata(lang));
      invalidate();
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  };

  const o = options.data;
  const running = o?.running ?? null;
  return (
    <Page title={t("kata.title")}>
      <div className="max-w-3xl space-y-4">
        <p className="text-sm text-muted-foreground">{t("kata.intro")}</p>
        {error !== null && <ErrorNotice error={error} />}
        {options.isPending ? (
          <Skeleton className="h-40 w-full" />
        ) : options.isError ? (
          <ErrorNotice error={options.error} />
        ) : running ? (
          <section className="space-y-3 rounded-lg border bg-card p-5" data-testid="kata-running">
            <div className="flex flex-wrap items-center justify-between gap-3">
              <h2 className="font-semibold">{t("kata.running")}</h2>
              <KataClock view={running} />
            </div>
            <KataProblemList view={running} />
            <Button variant="outline" onClick={() => void end()} disabled={busy}>
              <Square aria-hidden />
              {t("kata.end")}
            </Button>
          </section>
        ) : (
          <>
            {ended && (
              <section className="space-y-3 rounded-lg border bg-card p-5" data-testid="kata-summary">
                <h2 className="font-semibold">{t("kata.summary", { solved: ended.solved, total: ended.problems.length })}</h2>
                <KataProblemList view={ended} />
              </section>
            )}
            <section className="space-y-3 rounded-lg border bg-card p-5" data-testid="kata-daily">
              <h2 className="flex items-center gap-2 font-semibold">
                <Dumbbell aria-hidden className="size-4" />
                {t("kata.daily.title")}
              </h2>
              {o!.daily.length === 0 ? (
                <p className="text-sm text-muted-foreground">{t("kata.daily.none")}</p>
              ) : o!.daily_session ? (
                <p className="text-sm text-muted-foreground" data-testid="kata-daily-done">
                  {t("kata.daily.done", { solved: o!.daily_session.solved, total: o!.daily_session.problems.length })}
                </p>
              ) : (
                <>
                  <p className="text-sm text-muted-foreground">{t("kata.daily.intro")}</p>
                  <ul className="flex flex-col gap-1 text-sm">
                    {o!.daily.map((p) => (
                      <li key={p.id} className="flex items-center gap-2">
                        <span className="font-medium">{p.title}</span>
                        <DifficultyLabel difficulty={p.difficulty} />
                      </li>
                    ))}
                  </ul>
                  <Button onClick={() => void start("daily", null)} disabled={busy}>
                    {t("kata.daily.start")}
                    <ArrowRight aria-hidden />
                  </Button>
                </>
              )}
            </section>
            <section className="space-y-3 rounded-lg border bg-card p-5" data-testid="kata-sets">
              <h2 className="font-semibold">{t("kata.sets.title")}</h2>
              <p className="text-sm text-muted-foreground">{t("kata.sets.intro")}</p>
              <div className="grid grid-cols-[140px_1fr_auto] items-center gap-3 text-sm">
                <span className="text-muted-foreground">{t("kata.sets.pattern")}</span>
                <Select value={pattern} onValueChange={setPattern} disabled={o!.patterns.length === 0}>
                  <SelectTrigger size="sm" className="w-64" aria-label={t("kata.sets.pattern")}>
                    <SelectValue placeholder={t("kata.sets.pick")} />
                  </SelectTrigger>
                  <SelectContent>
                    {o!.patterns.map((p) => (
                      <SelectItem key={p.id} value={p.id}>
                        {t("kata.sets.option", { name: patternLabel(p.id), count: p.solved })}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Button size="sm" variant="outline" disabled={busy || !pattern} onClick={() => void start("pattern", pattern)}>
                  {t("kata.sets.start")}
                </Button>
                <span className="text-muted-foreground">{t("kata.sets.list")}</span>
                <Select value={list} onValueChange={setList} disabled={o!.lists.length === 0}>
                  <SelectTrigger size="sm" className="w-64" aria-label={t("kata.sets.list")}>
                    <SelectValue placeholder={o!.lists.length === 0 ? t("kata.sets.noLists") : t("kata.sets.pick")} />
                  </SelectTrigger>
                  <SelectContent>
                    {o!.lists.map((l) => (
                      <SelectItem key={l.id} value={l.id}>
                        {t("kata.sets.option", { name: l.name ?? "", count: l.solved })}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Button size="sm" variant="outline" disabled={busy || !list} onClick={() => void start("list", list)}>
                  {t("kata.sets.start")}
                </Button>
              </div>
            </section>
            <p className="text-sm">
              <Link to="/progress" search={{ tab: "katas" }} className="underline underline-offset-2">
                {t("kata.bests")}
              </Link>
            </p>
          </>
        )}
      </div>
    </Page>
  );
}

/** The problems of a session with their state: solved (with the time), tried, or not yet. */
export function KataProblemList({ view }: { view: KataView }) {
  const { t } = useTranslation();
  const open = view.ended_at === null;
  return (
    <ol className="flex flex-col gap-2" data-testid="kata-problems">
      {view.problems.map((p, i) => {
        const a = p.attempt;
        const solved = a?.solved_at != null;
        return (
          <li key={p.id} className="flex flex-wrap items-center gap-3 rounded-md border px-3 py-2 text-sm">
            <span className="text-xs text-muted-foreground tabular-nums">{i + 1}.</span>
            {solved ? (
              <CheckCircle2 aria-hidden className="size-4 text-verdict-accepted" />
            ) : (
              <CircleDashed aria-hidden className="size-4 text-muted-foreground" />
            )}
            <span className="font-medium">{p.title}</span>
            <DifficultyLabel difficulty={p.difficulty} />
            <span className="text-xs text-muted-foreground">
              {solved
                ? t("kata.solvedIn", { time: formatDuration(a!.time_ms ?? 0), count: a!.submits })
                : a
                  ? t("kata.tried", { count: a.submits })
                  : t("kata.notYet")}
            </span>
            {p.best_time_ms !== null && (
              <span className="text-xs text-muted-foreground">{t("kata.best", { time: formatDuration(p.best_time_ms) })}</span>
            )}
            {open && (
              <Button asChild size="sm" variant="outline" className="ml-auto">
                <Link to="/problems/$problemId" params={{ problemId: p.id }} search={{ kata: view.started_at }}>
                  {solved ? t("kata.again") : t("kata.open")}
                </Link>
              </Button>
            )}
          </li>
        );
      })}
    </ol>
  );
}
