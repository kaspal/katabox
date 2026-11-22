// Kata pieces (expansion M1, ADR 0033): the soft clock, the workspace bar, the Home
// "Rusty" card and the Progress → Katas tab.
import { useEffect, useState } from "react";
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowRight, Dumbbell, Flame, Hourglass, Timer, Trophy } from "lucide-react";
import type { KataOutcome, KataView } from "@/bindings";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel, usePatternLabel } from "@/components/Semantic";
import { ExerciseShell } from "@/components/exercise/ExerciseShell";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { formatDuration, useKataOverview } from "@/lib/kata";
import { cn } from "@/lib/utils";
import { HomeCard } from "@/practice/HomeCards";
import { SavedReflection } from "@/practice/journal";

/** Milliseconds since `since`, ticking every second from the wall clock. */
function useElapsed(since: number | null): number {
  const [, setTick] = useState(0);
  useEffect(() => {
    if (since === null) return;
    const id = window.setInterval(() => setTick((n) => n + 1), 1000);
    return () => window.clearInterval(id);
  }, [since]);
  return since === null ? 0 : Math.max(0, Date.now() - since);
}

/** The session's soft clock: time used against its budget; overtime is shown, not enforced. */
export function KataClock({ view }: { view: KataView }) {
  const { t } = useTranslation();
  const elapsed = useElapsed(view.ended_at === null ? view.started_at : null);
  const used = view.ended_at === null ? elapsed : view.ended_at - view.started_at;
  const budget = view.duration_s * 1000;
  const over = used > budget;
  const time = formatDuration(used);
  const of = formatDuration(budget);
  return (
    <span
      className={cn(
        "inline-flex items-center gap-1.5 rounded-md border px-2 py-1 font-mono text-sm tabular-nums",
        over && "border-verdict-wrong/50 text-verdict-wrong",
      )}
      aria-label={over ? t("kata.clock.overLabel", { time, of }) : t("kata.clock.label", { time, of })}
      data-testid="kata-clock"
      data-over={over || undefined}
    >
      {over ? <Hourglass aria-hidden className="size-4" /> : <Timer aria-hidden className="size-4" />}
      {time}
      <span className="font-sans text-xs text-muted-foreground">/ {of}</span>
      {over && <span className="font-sans text-xs">{t("kata.clock.over")}</span>}
    </span>
  );
}

/**
 * The workspace bar of a kata problem: "k of n", the clock, and the way to the next
 * problem or back to the kata page.
 */
export function KataBar({
  view,
  problemId,
  outcome,
}: {
  view: KataView;
  problemId: string;
  outcome: KataOutcome | null;
}) {
  const { t } = useTranslation();
  const index = view.problems.findIndex((p) => p.id === problemId);
  const next = view.problems.slice(index + 1).find((p) => p.attempt?.solved_at == null) ?? null;
  const solvedNow = outcome?.attempt.solved_at != null;
  return (
    <ExerciseShell
      kind="kata"
      icon={<Dumbbell />}
      title={view.problems[index]?.title ?? problemId}
      position={index >= 0 ? { index: index + 1, total: view.problems.length } : undefined}
      timer={<KataClock view={view} />}
      testId="kata-bar"
      footer={
        <>
          {next && (
            <Button asChild size="sm" variant={solvedNow ? "default" : "outline"}>
              <Link to="/problems/$problemId" params={{ problemId: next.id }} search={{ kata: view.started_at }}>
                {solvedNow ? t("kata.next") : t("kata.skip")}
                <ArrowRight aria-hidden />
              </Link>
            </Button>
          )}
          <Button asChild size="sm" variant="ghost">
            <Link to="/kata">{t("kata.back")}</Link>
          </Button>
        </>
      }
    >
      <p className="text-sm text-muted-foreground">
        {solvedNow
          ? outcome!.new_best
            ? t("kata.newBest", { time: formatDuration(outcome!.attempt.time_ms ?? 0) })
            : t("kata.solvedIn", { time: formatDuration(outcome!.attempt.time_ms ?? 0), count: outcome!.attempt.submits })
          : t("kata.fromMemory")}
      </p>
      {solvedNow && <SavedReflection problemId={problemId} />}
    </ExerciseShell>
  );
}

/** Home: the rustiest solved problems, each a kata away. */
export function RustyCard() {
  const { t } = useTranslation();
  const overview = useKataOverview();
  const patternLabel = usePatternLabel();
  const rusty = overview.data?.rusty.slice(0, 5) ?? [];
  return (
    <HomeCard title={t("kata.rusty.title")} icon={<Hourglass aria-hidden className="size-4" />} testId="rusty-card">
      {overview.isPending ? (
        <Skeleton className="h-16 w-full" />
      ) : overview.isError ? (
        <ErrorNotice error={overview.error} />
      ) : rusty.length === 0 ? (
        <p className="text-sm text-muted-foreground">{t("kata.rusty.none")}</p>
      ) : (
        <>
          <ul className="flex flex-col gap-1.5 text-sm">
            {rusty.map((r) => (
              <li key={r.problem_id} className="flex items-center gap-2">
                <span className="min-w-0 flex-1 truncate font-medium">{r.title}</span>
                <span className="text-xs text-muted-foreground">{patternLabel(r.pattern)}</span>
                <span className="text-xs tabular-nums text-muted-foreground" title={t("kata.rusty.scoreHelp")}>
                  {t("kata.rusty.score", { score: r.rusty })}
                </span>
              </li>
            ))}
          </ul>
          <Button asChild size="sm" variant="outline" className="self-start">
            <Link to="/kata">
              {t("kata.rusty.practise")}
              <ArrowRight aria-hidden />
            </Link>
          </Button>
        </>
      )}
    </HomeCard>
  );
}

/** Progress → Katas: streak, personal bests, the rusty list and the latest sessions. */
export function KatasTab() {
  const { t, i18n } = useTranslation();
  const overview = useKataOverview();
  if (overview.isPending) return <Skeleton className="h-40 w-full" />;
  if (overview.isError) return <ErrorNotice error={overview.error} />;
  const o = overview.data;
  const dateFmt = new Intl.DateTimeFormat(i18n.language, { dateStyle: "medium" });
  return (
    <div className="space-y-6" data-testid="katas-tab">
      <div className="flex flex-wrap items-center gap-4">
        <span className="inline-flex items-center gap-2 text-sm" data-testid="kata-streak">
          <Flame aria-hidden className="size-4" />
          {t("kata.streak", { count: o.streak })}
        </span>
        <Button asChild size="sm">
          <Link to="/kata">
            <Dumbbell aria-hidden />
            {t("kata.start")}
          </Link>
        </Button>
      </div>
      <section className="space-y-2">
        <h2 className="flex items-center gap-2 font-semibold">
          <Trophy aria-hidden className="size-4" />
          {t("kata.bestsTitle")}
        </h2>
        {o.bests.length === 0 ? (
          <p className="text-sm text-muted-foreground">{t("kata.noBests")}</p>
        ) : (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>{t("kata.col.problem")}</TableHead>
                <TableHead>{t("kata.col.best")}</TableHead>
                <TableHead>{t("kata.col.submits")}</TableHead>
                <TableHead>{t("kata.col.katas")}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {o.bests.map((b) => (
                <TableRow key={b.problem_id}>
                  <TableCell>
                    <span className="flex items-center gap-2">
                      <Link to="/problems/$problemId" params={{ problemId: b.problem_id }} className="font-medium hover:underline">
                        {b.title}
                      </Link>
                      <DifficultyLabel difficulty={b.difficulty} />
                    </span>
                  </TableCell>
                  <TableCell className="font-mono tabular-nums">
                    {b.best_time_ms === null ? "—" : formatDuration(b.best_time_ms)}
                  </TableCell>
                  <TableCell className="tabular-nums">{b.fewest_submits ?? "—"}</TableCell>
                  <TableCell className="tabular-nums">{t("kata.solvedOf", { solved: b.solved, total: b.attempts })}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        )}
      </section>
      <section className="space-y-2">
        <h2 className="flex items-center gap-2 font-semibold">
          <Hourglass aria-hidden className="size-4" />
          {t("kata.rusty.title")}
        </h2>
        {o.rusty.length === 0 ? (
          <p className="text-sm text-muted-foreground">{t("kata.rusty.none")}</p>
        ) : (
          <ul className="flex flex-col gap-1 text-sm">
            {o.rusty.map((r) => (
              <li key={r.problem_id} className="flex items-center gap-2">
                <span className="font-medium">{r.title}</span>
                <span className="text-xs tabular-nums text-muted-foreground">{t("kata.rusty.score", { score: r.rusty })}</span>
                <span className="text-xs text-muted-foreground">
                  {r.days === null ? t("kata.rusty.never") : t("kata.rusty.days", { count: r.days })}
                </span>
              </li>
            ))}
          </ul>
        )}
      </section>
      <section className="space-y-2">
        <h2 className="font-semibold">{t("kata.sessionsTitle")}</h2>
        {o.sessions.length === 0 ? (
          <p className="text-sm text-muted-foreground">{t("kata.noSessions")}</p>
        ) : (
          <ul className="flex flex-col gap-1 text-sm" data-testid="kata-sessions">
            {o.sessions.map((s) => (
              <li key={s.started_at} className="flex flex-wrap items-center gap-2">
                <span className="tabular-nums">{dateFmt.format(new Date(s.started_at))}</span>
                <span className="text-muted-foreground">{t(`kata.kind.${s.kind}`)}</span>
                {s.source && <span className="text-muted-foreground">· {s.source}</span>}
                <span className="tabular-nums">{t("kata.solvedOf", { solved: s.solved, total: s.total })}</span>
                {!s.ended && <span className="text-xs text-muted-foreground">{t("kata.inProgress")}</span>}
              </li>
            ))}
          </ul>
        )}
      </section>
    </div>
  );
}
