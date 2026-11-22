// The mistake journal and reflections (expansion M3, ADR 0034): tags and a note on a
// failed submission, Progress → Mistakes, and the "why it works" reflection after
// Accepted, shown again in review mode, katas and the Notes tab.
import { useEffect, useState } from "react";
import { Link } from "@tanstack/react-router";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { Lightbulb, NotebookPen, Tag } from "lucide-react";
import type { ReflectionRow } from "@/bindings";
import { ErrorNotice } from "@/components/ErrorNotice";
import { Button } from "@/components/ui/button";
import { Progress as Bar } from "@/components/ui/progress";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { api } from "@/lib/api";
import { usePrefs } from "@/lib/prefs";
import { relativeTime } from "@/lib/time";
import { cn } from "@/lib/utils";

/** Every tag, in display order (the same set as kb-app's `journal::TAGS`). */
export const TAGS = [
  "wa",
  "tle",
  "mle",
  "ole",
  "compile",
  "runtime:index",
  "runtime:key",
  "runtime:null",
  "runtime:zero_division",
  "runtime:recursion",
  "runtime:overflow",
  "runtime:type",
  "runtime:other",
  "edge:empty",
  "edge:single",
  "edge:max",
  "edge:negative",
  "edge:zero",
  "edge:duplicates",
  "shrunk",
  "logic",
] as const;

export const CLASSES = ["O(1)", "O(log n)", "O(n)", "O(n log n)", "O(n^2)", "O(n^3)", "O(2^n)", "O(n!)", "other"] as const;

/** A tag's name: `:` is i18next's namespace separator, so keys use `_`. */
export function useTagName() {
  const { t } = useTranslation();
  return (tag: string) => t(`journal.tag.${tag.replace(":", "_")}` as "journal.tag.wa", { defaultValue: tag });
}

const journalKeys = {
  all: ["journal"] as const,
  overview: (lang: string) => ["journal", "overview", lang] as const,
  mistake: (id: number) => ["journal", "mistake", id] as const,
  reflection: (problem: string) => ["journal", "reflection", problem] as const,
};

export function useReflection(problem: string, enabled = true) {
  return useQuery({
    queryKey: journalKeys.reflection(problem),
    queryFn: () => api.reflection(problem),
    enabled,
    retry: false,
  });
}

/** Tags (toggle chips) and a note on a failed submission. */
export function MistakeEditor({ submission }: { submission: number }) {
  const { t } = useTranslation();
  const tagName = useTagName();
  const queryClient = useQueryClient();
  const q = useQuery({ queryKey: journalKeys.mistake(submission), queryFn: () => api.submissionMistake(submission) });
  const [tags, setTags] = useState<string[] | null>(null);
  const [note, setNote] = useState("");
  const [status, setStatus] = useState<"idle" | "saved" | "error">("idle");
  useEffect(() => {
    if (q.data) {
      setTags(q.data.tags);
      setNote(q.data.note);
    }
  }, [q.data]);
  if (q.isPending) return <Skeleton className="h-16 w-full" />;
  if (q.error) return <ErrorNotice error={q.error} />;
  if (!q.data || tags === null) return null;
  const save = async () => {
    try {
      await api.setSubmissionMistake(submission, tags, note);
      setStatus("saved");
      void queryClient.invalidateQueries({ queryKey: journalKeys.all });
    } catch {
      setStatus("error");
    }
  };
  return (
    <section className="space-y-2 rounded-lg border p-3" data-testid="mistake-editor">
      <h4 className="flex items-center gap-2 text-sm font-semibold">
        <Tag aria-hidden className="size-4" />
        {t("journal.title")}
      </h4>
      <p className="text-xs text-muted-foreground">{t("journal.intro")}</p>
      <div className="flex flex-wrap gap-1.5" role="group" aria-label={t("journal.tags")}>
        {TAGS.map((tag) => {
          const on = tags.includes(tag);
          const auto = q.data!.auto_tags.includes(tag);
          return (
            <button
              key={tag}
              type="button"
              aria-pressed={on}
              onClick={() => setTags((cur) => (on ? cur!.filter((x) => x !== tag) : [...cur!, tag]))}
              className={cn(
                "rounded-full border px-2 py-0.5 text-xs transition-colors motion-reduce:transition-none",
                on ? "border-primary bg-primary/10 font-medium" : "text-muted-foreground hover:bg-accent",
              )}
              title={auto ? t("journal.auto") : undefined}
            >
              {tagName(tag)}
              {auto && <span className="sr-only"> ({t("journal.auto")})</span>}
            </button>
          );
        })}
      </div>
      <label className="flex flex-col gap-1 text-xs text-muted-foreground">
        {t("journal.note")}
        <textarea
          value={note}
          maxLength={2000}
          rows={2}
          onChange={(e) => {
            setNote(e.target.value);
            setStatus("idle");
          }}
          className="rounded-md border bg-background px-2 py-1 text-sm text-foreground"
        />
      </label>
      <div className="flex items-center gap-2">
        <Button size="sm" variant="outline" onClick={() => void save()}>
          {t("journal.save")}
        </Button>
        <span role="status" className="text-xs text-muted-foreground">
          {status === "saved" ? t("journal.saved") : status === "error" ? t("journal.saveError") : ""}
        </span>
      </div>
    </section>
  );
}

/** Progress → Mistakes: counts per tag, the latest examples, a filter. */
export function MistakesTab() {
  const { t, i18n } = useTranslation();
  const { lang } = usePrefs();
  const tagName = useTagName();
  const q = useQuery({ queryKey: journalKeys.overview(lang), queryFn: () => api.journalOverview(lang) });
  const [filter, setFilter] = useState("all");
  if (q.isPending) return <Skeleton className="h-40 w-full" />;
  if (q.error) return <ErrorNotice error={q.error} />;
  const o = q.data;
  if (o.failures === 0) return <p className="text-sm text-muted-foreground">{t("journal.none")}</p>;
  const max = Math.max(...o.tags.map((x) => x.count), 1);
  const shown = filter === "all" ? o.tags : o.tags.filter((x) => x.tag === filter);
  return (
    <div className="space-y-4" data-testid="mistakes-tab">
      <div className="flex flex-wrap items-center gap-3">
        <span className="text-sm">{t("journal.failures", { count: o.failures })}</span>
        <Select value={filter} onValueChange={setFilter}>
          <SelectTrigger size="sm" className="w-56" aria-label={t("journal.filter")}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{t("journal.allTags")}</SelectItem>
            {o.tags.map((x) => (
              <SelectItem key={x.tag} value={x.tag}>
                {tagName(x.tag)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
      <ul className="space-y-3">
        {shown.map((x) => (
          <li key={x.tag} className="rounded-lg border bg-card p-3" data-testid={`mistake-tag-${x.tag}`}>
            <div className="flex items-center gap-3">
              <span className="w-48 shrink-0 text-sm font-medium">{tagName(x.tag)}</span>
              <Bar value={(100 * x.count) / max} className="flex-1" aria-label={`${tagName(x.tag)}: ${x.count}`} />
              <span className="w-10 text-right text-sm tabular-nums">{x.count}</span>
            </div>
            <ul className="mt-2 space-y-1 text-xs text-muted-foreground">
              {x.examples.map((e) => (
                <li key={`${e.problem_id}-${e.language}-${e.submitted_at}`} className="flex flex-wrap gap-2">
                  <Link to="/problems/$problemId" params={{ problemId: e.problem_id }} className="font-medium text-foreground hover:underline">
                    {e.title}
                  </Link>
                  <span>{t(`languages.${e.language}` as "languages.python")}</span>
                  <span>{relativeTime(e.submitted_at, i18n.language)}</span>
                  {e.note && <span className="italic">“{e.note}”</span>}
                </li>
              ))}
            </ul>
          </li>
        ))}
      </ul>
    </div>
  );
}

/** After Accepted: an optional "why it works" and the complexity, saved per problem. */
export function ReflectionCard({ problemId }: { problemId: string }) {
  const { t } = useTranslation();
  const queryClient = useQueryClient();
  const q = useReflection(problemId);
  const [why, setWhy] = useState("");
  const [time, setTime] = useState<string>("");
  const [space, setSpace] = useState<string>("");
  const [saved, setSaved] = useState(false);
  const [error, setError] = useState<unknown>(null);
  useEffect(() => {
    if (q.data) {
      setWhy(q.data.why);
      setTime(q.data.time_class ?? "");
      setSpace(q.data.space_class ?? "");
    }
  }, [q.data]);
  if (q.isPending) return null;
  const save = async () => {
    setError(null);
    try {
      await api.setReflection(problemId, why, time || null, space || null);
      setSaved(true);
      void queryClient.invalidateQueries({ queryKey: journalKeys.reflection(problemId) });
    } catch (e) {
      setError(e);
    }
  };
  return (
    <section className="space-y-2 rounded-lg border p-3" data-testid="reflection-card">
      <h4 className="flex items-center gap-2 text-sm font-semibold">
        <Lightbulb aria-hidden className="size-4" />
        {t("reflection.title")}
      </h4>
      <label className="flex flex-col gap-1 text-xs text-muted-foreground">
        {t("reflection.why")}
        <input
          value={why}
          maxLength={500}
          onChange={(e) => {
            setWhy(e.target.value);
            setSaved(false);
          }}
          className="rounded-md border bg-background px-2 py-1 text-sm text-foreground"
        />
      </label>
      <div className="flex flex-wrap items-center gap-3 text-xs text-muted-foreground">
        {(
          [
            ["time", time, setTime],
            ["space", space, setSpace],
          ] as const
        ).map(([k, v, set]) => (
          <span key={k} className="inline-flex items-center gap-2">
            {t(`reflection.${k}`)}
            <Select
              value={v || "none"}
              onValueChange={(x) => {
                set(x === "none" ? "" : x);
                setSaved(false);
              }}
            >
              <SelectTrigger size="sm" className="w-32" aria-label={t(`reflection.${k}`)}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="none">{t("reflection.unset")}</SelectItem>
                {CLASSES.map((c) => (
                  <SelectItem key={c} value={c}>
                    {c === "other" ? t("reflection.other") : c}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </span>
        ))}
        <Button size="sm" variant="outline" onClick={() => void save()} disabled={!why.trim() && !time && !space}>
          {t("reflection.save")}
        </Button>
        <span role="status">{saved ? t("reflection.saved") : ""}</span>
      </div>
      {error !== null && <ErrorNotice error={error} />}
    </section>
  );
}

/** The saved reflection, read-only (review mode, katas, the Notes tab). */
export function ReflectionNote({ reflection, className }: { reflection: ReflectionRow; className?: string }) {
  const { t } = useTranslation();
  const classes = [
    reflection.time_class && t("reflection.timeIs", { c: reflection.time_class }),
    reflection.space_class && t("reflection.spaceIs", { c: reflection.space_class }),
  ].filter(Boolean);
  return (
    <div className={cn("flex items-start gap-2 rounded-md border bg-muted/40 p-2 text-sm", className)} data-testid="reflection-note">
      <NotebookPen aria-hidden className="mt-0.5 size-4 shrink-0" />
      <span>
        <span className="font-medium">{t("reflection.yours")}</span> {reflection.why}
        {classes.length > 0 && <span className="text-muted-foreground"> · {classes.join(" · ")}</span>}
      </span>
    </div>
  );
}

/** The reflection, if any, fetched and shown read-only (nothing while locked or empty). */
export function SavedReflection({ problemId, enabled = true }: { problemId: string; enabled?: boolean }) {
  const q = useReflection(problemId, enabled);
  if (!q.data) return null;
  return <ReflectionNote reflection={q.data} />;
}
