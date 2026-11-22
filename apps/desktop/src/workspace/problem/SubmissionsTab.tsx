import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { ArrowLeft, GitCompare } from "lucide-react";
import { Checkbox } from "@/components/ui/checkbox";
import { DiffView } from "@/workspace/problem/DiffView";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { ErrorNotice } from "@/components/ErrorNotice";
import { VerdictBadge } from "@/components/Verdict";
import { api } from "@/lib/api";
import { relativeTime } from "@/lib/time";
import { CodeEditor } from "@/workspace/editor/CodeEditor";
import { ResultView, mb, ms } from "@/workspace/results/ResultView";
import { MistakeEditor } from "@/practice/journal";

export function SubmissionsTab({ problemId }: { problemId: string }) {
  const { t, i18n } = useTranslation();
  const [open, setOpen] = useState<number | null>(null);
  // Compare any two submissions (expansion M4).
  const [comparing, setComparing] = useState(false);
  const [picked, setPicked] = useState<number[]>([]);
  const list = useQuery({
    queryKey: ["submissions", problemId],
    queryFn: () => api.listSubmissions(problemId),
  });

  if (open !== null) return <SubmissionDetail id={open} onBack={() => setOpen(null)} />;
  if (list.isPending) return <Skeleton className="h-24 w-full" />;
  if (list.error) return <ErrorNotice error={list.error} onRetry={() => void list.refetch()} />;
  if (list.data.length === 0) {
    return <p className="text-sm text-muted-foreground">{t("workspace.submissions.none")}</p>;
  }
  return (
    <div className="space-y-2">
      {list.data.length > 1 && (
        <div className="flex items-center gap-2">
          <Button
            size="sm"
            variant={comparing ? "secondary" : "outline"}
            aria-pressed={comparing}
            onClick={() => {
              setComparing(!comparing);
              setPicked([]);
            }}
          >
            <GitCompare aria-hidden />
            {t("workspace.submissions.compare")}
          </Button>
          {comparing && <span className="text-xs text-muted-foreground">{t("workspace.submissions.pickTwo")}</span>}
        </div>
      )}
      {comparing && picked.length === 2 && <SubmissionDiff a={picked[0]} b={picked[1]} />}
    <ul className="divide-y rounded-lg border" data-testid="submissions-list">
      {list.data.map((s) => (
        <li key={s.id} className="flex items-center">
          {comparing && (
            <Checkbox
              className="ml-3"
              aria-label={t("workspace.submissions.pick", { n: list.data.indexOf(s) + 1 })}
              checked={picked.includes(s.id)}
              onCheckedChange={(on) =>
                setPicked((cur) => (on ? [...cur.filter((x) => x !== s.id), s.id].slice(-2) : cur.filter((x) => x !== s.id)))
              }
            />
          )}
          <button
            type="button"
            onClick={() => setOpen(s.id)}
            className="grid min-w-0 flex-1 grid-cols-[1fr_auto] items-center gap-2 px-3 py-2 text-left hover:bg-accent"
          >
            <span className="flex flex-col">
              <VerdictBadge verdict={s.verdict} />
              <span className="text-xs text-muted-foreground">
                {t(`languages.${s.language}`)} · {relativeTime(s.created_at, i18n.language)}
              </span>
            </span>
            <span className="text-right text-xs text-muted-foreground tabular-nums">
              {ms(s.time_us)} ms · {mb(s.memory_kb)} MB
            </span>
          </button>
        </li>
      ))}
    </ul>
    </div>
  );
}

/** Two submissions side by side, the older on the left. */
function SubmissionDiff({ a, b }: { a: number; b: number }) {
  const { t } = useTranslation();
  const [older, newer] = a < b ? [a, b] : [b, a];
  const qa = useQuery({ queryKey: ["submission", older], queryFn: () => api.getSubmission(older) });
  const qb = useQuery({ queryKey: ["submission", newer], queryFn: () => api.getSubmission(newer) });
  if (qa.error || qb.error) return <ErrorNotice error={qa.error ?? qb.error} />;
  if (!qa.data || !qb.data) return <Skeleton className="h-32 w-full" />;
  return (
    <div className="space-y-1" data-testid="submission-diff">
      <p className="text-xs text-muted-foreground">
        {t("workspace.submissions.diffLabel", {
          a: `${t(`languages.${qa.data.language}`)} · ${t(`judge.verdict.${qa.data.verdict}`)}`,
          b: `${t(`languages.${qb.data.language}`)} · ${t(`judge.verdict.${qb.data.verdict}`)}`,
        })}
      </p>
      <DiffView a={qa.data.code} b={qb.data.code} language={qb.data.language} testId="submission-diff-view" />
    </div>
  );
}

function SubmissionDetail({ id, onBack }: { id: number; onBack: () => void }) {
  const { t } = useTranslation();
  const q = useQuery({ queryKey: ["submission", id], queryFn: () => api.getSubmission(id) });
  return (
    <div className="space-y-3">
      <Button variant="ghost" size="sm" onClick={onBack}>
        <ArrowLeft aria-hidden />
        {t("workspace.submissions.back")}
      </Button>
      {q.isPending ? (
        <Skeleton className="h-32 w-full" />
      ) : q.error ? (
        <ErrorNotice error={q.error} />
      ) : (
        <>
          <ResultView report={q.data.report} />
          {q.data.verdict !== "accepted" && <MistakeEditor submission={id} />}
          <h4 className="text-sm font-semibold">
            {t("workspace.submissions.code", { language: t(`languages.${q.data.language}`) })}
          </h4>
          <div className="h-64 overflow-hidden rounded-md border">
            <CodeEditor
              docKey={`submission:${id}`}
              value={q.data.code}
              lang={q.data.language}
              readOnly
              ariaLabel={t("workspace.submissions.codeLabel")}
            />
          </div>
        </>
      )}
    </div>
  );
}
