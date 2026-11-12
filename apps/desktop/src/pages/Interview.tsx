// Mock interview (ADR 0020): setup, the running session, and the scorecard when it ends.
import { useState } from "react";
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowRight, Square } from "lucide-react";
import type { InterviewResult, Language } from "@/bindings";
import { Page } from "@/components/Page";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel, usePatternLabel } from "@/components/Semantic";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { api } from "@/lib/api";
import { formatClock, useInterviewActive, useInvalidateInterview } from "@/lib/modes";
import { usePrefs } from "@/lib/prefs";
import { JUDGE_LANGUAGES } from "@/lib/settings";
import { Scorecard, useRemaining } from "@/practice/interview";

const DURATIONS = [30, 45, 60] as const;
const MIXES = ["easy+easy", "easy+medium", "medium+medium", "medium+hard", "hard+hard"] as const;

export function Interview() {
  const { t } = useTranslation();
  const { lang } = usePrefs();
  const active = useInterviewActive();
  const invalidate = useInvalidateInterview();
  const patternLabel = usePatternLabel();
  const remaining = useRemaining(active.data?.ends_at ?? null);
  const [duration, setDuration] = useState<number>(45);
  const [mix, setMix] = useState<string>("easy+medium");
  const [languages, setLanguages] = useState<Language[]>(["python"]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>(null);
  const [result, setResult] = useState<InterviewResult | null>(null);

  const start = async () => {
    setBusy(true);
    setError(null);
    setResult(null);
    try {
      await api.startInterview({ duration_min: duration, mix, languages }, lang);
      invalidate();
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  };
  const end = async () => {
    setBusy(true);
    try {
      setResult(await api.endInterview(lang));
      invalidate();
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  };

  return (
    <Page title={t("practice.interview.title")}>
      <div className="max-w-3xl space-y-4">
        <p className="text-sm text-muted-foreground">{t("practice.interview.intro")}</p>
        {error !== null && <ErrorNotice error={error} />}
        {active.isPending ? (
          <Skeleton className="h-40 w-full" />
        ) : active.data ? (
          <section className="space-y-3 rounded-lg border bg-card p-5" data-testid="interview-running">
            <div className="flex items-center justify-between gap-3">
              <h2 className="font-semibold">{t("practice.interview.running")}</h2>
              <span className="font-mono text-2xl tabular-nums" data-testid="interview-remaining">
                {t("practice.interview.remaining", { time: formatClock(remaining ?? active.data.remaining_s) })}
              </span>
            </div>
            <ul className="grid gap-3 sm:grid-cols-2">
              {active.data.problems.map((p) => (
                <li key={p.id} className="flex flex-col gap-2 rounded-lg border p-4">
                  <span className="font-medium">{p.title}</span>
                  <span className="flex items-center gap-2 text-xs text-muted-foreground">
                    <DifficultyLabel difficulty={p.difficulty} />
                    <span>·</span>
                    <span>{patternLabel(p.pattern)}</span>
                  </span>
                  <Button asChild size="sm" className="self-start">
                    <Link to="/problems/$problemId" params={{ problemId: p.id }}>
                      {t("practice.interview.open")}
                      <ArrowRight aria-hidden />
                    </Link>
                  </Button>
                </li>
              ))}
            </ul>
            <p className="text-xs text-muted-foreground">
              {t("practice.interview.languages")}: {active.data.languages.map((l) => t(`languages.${l}`)).join(", ")}
            </p>
            <Button variant="outline" onClick={() => void end()} disabled={busy}>
              <Square aria-hidden />
              {t("practice.interview.end")}
            </Button>
          </section>
        ) : (
          <>
            {result && (
              <section className="space-y-3 rounded-lg border bg-card p-5">
                <h2 className="font-semibold">{t("practice.interview.scorecard")}</h2>
                <Scorecard result={result} />
              </section>
            )}
            <section className="space-y-4 rounded-lg border bg-card p-5" data-testid="interview-setup">
              <h2 className="font-semibold">
                {result ? t("practice.interview.again") : t("practice.interview.setup")}
              </h2>
              <div className="grid grid-cols-[140px_1fr] items-center gap-4 text-sm">
                <span className="text-muted-foreground">{t("practice.interview.duration")}</span>
                <ToggleGroup
                  type="single"
                  variant="outline"
                  size="sm"
                  aria-label={t("practice.interview.duration")}
                  value={String(duration)}
                  onValueChange={(v) => v && setDuration(Number(v))}
                >
                  {DURATIONS.map((d) => (
                    <ToggleGroupItem key={d} value={String(d)} className="px-3">
                      {t("practice.interview.minutes", { count: d })}
                    </ToggleGroupItem>
                  ))}
                </ToggleGroup>
                <span className="text-muted-foreground">{t("practice.interview.mix")}</span>
                <Select value={mix} onValueChange={setMix}>
                  <SelectTrigger size="sm" className="w-48" aria-label={t("practice.interview.mix")}>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {MIXES.map((m) => (
                      <SelectItem key={m} value={m}>
                        {t(`practice.interview.mixes.${m}`)}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <span className="text-muted-foreground">{t("practice.interview.languages")}</span>
                <div className="flex flex-wrap gap-4">
                  {JUDGE_LANGUAGES.map((l) => (
                    <label key={l} className="inline-flex items-center gap-2">
                      <Checkbox
                        checked={languages.includes(l)}
                        onCheckedChange={(on) =>
                          setLanguages((cur) => (on ? [...cur, l] : cur.filter((x) => x !== l)))
                        }
                      />
                      {t(`languages.${l}`)}
                    </label>
                  ))}
                </div>
              </div>
              <Button onClick={() => void start()} disabled={busy || languages.length === 0}>
                {t("practice.interview.start")}
              </Button>
            </section>
          </>
        )}
      </div>
    </Page>
  );
}
