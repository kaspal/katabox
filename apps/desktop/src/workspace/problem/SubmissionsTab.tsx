import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { ArrowLeft } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { ErrorNotice } from "@/components/ErrorNotice";
import { VerdictBadge } from "@/components/Verdict";
import { api } from "@/lib/api";
import { relativeTime } from "@/lib/time";
import { CodeEditor } from "@/workspace/editor/CodeEditor";
import { ResultView, mb, ms } from "@/workspace/results/ResultView";

export function SubmissionsTab({ problemId }: { problemId: string }) {
  const { t, i18n } = useTranslation();
  const [open, setOpen] = useState<number | null>(null);
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
    <ul className="divide-y rounded-lg border" data-testid="submissions-list">
      {list.data.map((s) => (
        <li key={s.id}>
          <button
            type="button"
            onClick={() => setOpen(s.id)}
            className="grid w-full grid-cols-[1fr_auto] items-center gap-2 px-3 py-2 text-left hover:bg-accent"
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
