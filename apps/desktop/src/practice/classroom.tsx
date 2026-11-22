// Classroom mode (expansion M12, ADR 0039). Student: the assignments that came with packs,
// started and finished here; kb-app enforces their rules (languages, hints) while they
// run. The results file is shown in full before it is saved. Teacher: results files are
// imported and summarised per student and problem, then exported as CSV.
import { useState } from "react";
import { Link } from "@tanstack/react-router";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import {
  CheckCircle2,
  Circle,
  Download,
  FileDown,
  PlayCircle,
  ShieldAlert,
  ShieldCheck,
  Square,
  Timer,
  Upload,
} from "lucide-react";
import type { AssignmentState, AssignmentView, ClassAssignment, ClassImport } from "@/bindings";
import { Page } from "@/components/Page";
import { ErrorNotice } from "@/components/ErrorNotice";
import { VerdictBadge } from "@/components/Verdict";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Skeleton } from "@/components/ui/skeleton";
import { api } from "@/lib/api";
import { formatClock } from "@/lib/modes";
import { usePrefs } from "@/lib/prefs";
import { useSetting } from "@/lib/settings";
import { useRemaining } from "@/practice/interview";

const STATE_ICON: Record<AssignmentState, typeof Circle> = {
  not_started: Circle,
  running: PlayCircle,
  finished: CheckCircle2,
};

export function Classroom() {
  const { t } = useTranslation();
  const { lang } = usePrefs();
  const assignments = useQuery({ queryKey: ["assignments", lang], queryFn: () => api.assignments(lang) });
  return (
    <Page title={t("classroom.title")}>
      <div className="max-w-4xl space-y-8 overflow-y-auto pb-6">
        <p className="text-sm text-muted-foreground">{t("classroom.intro")}</p>
        <section aria-labelledby="classroom-mine" className="space-y-3">
          <h2 id="classroom-mine" className="font-semibold">
            {t("classroom.mine")}
          </h2>
          {assignments.isPending ? (
            <Skeleton className="h-32 w-full" />
          ) : assignments.error ? (
            <ErrorNotice error={assignments.error} />
          ) : (assignments.data?.length ?? 0) === 0 ? (
            <p className="text-sm text-muted-foreground" data-testid="assignments-none">
              {t("classroom.none")}
            </p>
          ) : (
            assignments.data!.map((a) => <AssignmentCard key={a.id} a={a} />)
          )}
        </section>
        <section aria-labelledby="classroom-teacher" className="space-y-3">
          <h2 id="classroom-teacher" className="font-semibold">
            {t("classroom.teacher.title")}
          </h2>
          <TeacherPanel />
        </section>
      </div>
    </Page>
  );
}

function AssignmentCard({ a }: { a: AssignmentView }) {
  const { t, i18n } = useTranslation();
  const { lang } = usePrefs();
  const qc = useQueryClient();
  const remaining = useRemaining(a.state === "running" ? a.ends_at : null);
  const [exporting, setExporting] = useState(false);
  const refresh = () => void qc.invalidateQueries({ queryKey: ["assignments"] });
  const start = useMutation({ mutationFn: () => api.startAssignment(a.id, lang), onSuccess: refresh });
  const finish = useMutation({ mutationFn: () => api.finishAssignment(a.id, lang), onSuccess: refresh });
  const Icon = STATE_ICON[a.state];
  const due = new Intl.DateTimeFormat(i18n.language, { dateStyle: "medium", timeZone: "UTC" }).format(
    new Date(`${a.meta.due}T00:00:00Z`),
  );
  const languages = a.meta.languages.map((l) => t(`languages.${l}` as "languages.python")).join(", ");
  return (
    <article className="space-y-3 rounded-lg border bg-card p-4" data-testid={`assignment-${a.id}`}>
      <header className="flex flex-wrap items-center gap-x-3 gap-y-1">
        <h3 className="font-semibold">{a.meta.title}</h3>
        <span className="flex items-center gap-1 text-sm text-muted-foreground" data-testid="assignment-state">
          <Icon aria-hidden className="size-4" />
          {t(`classroom.state.${a.state}`)}
        </span>
        {remaining !== null && (
          <span className="ml-auto flex items-center gap-1 font-mono tabular-nums" data-testid="assignment-remaining">
            <Timer aria-hidden className="size-4" />
            {t("classroom.remaining", { time: formatClock(remaining) })}
          </span>
        )}
      </header>
      <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-sm">
        <dt className="text-muted-foreground">{t("classroom.due")}</dt>
        <dd>{due}</dd>
        <dt className="text-muted-foreground">{t("classroom.languages")}</dt>
        <dd>{languages}</dd>
        <dt className="text-muted-foreground">{t("classroom.timeLimit")}</dt>
        <dd>
          {a.meta.time_limit_min === null
            ? t("classroom.noTimeLimit")
            : t("classroom.minutes", { count: a.meta.time_limit_min })}
        </dd>
        <dt className="text-muted-foreground">{t("classroom.hints")}</dt>
        <dd>{a.meta.hints ? t("classroom.hintsOn") : t("classroom.hintsOff")}</dd>
      </dl>
      <ul className="divide-y rounded-md border text-sm">
        {a.problems.map((p) => (
          <li key={p.id} className="flex flex-wrap items-center gap-x-3 gap-y-1 px-3 py-2">
            {p.summary ? (
              <Link to="/problems/$problemId" params={{ problemId: p.id }} className="font-medium hover:underline">
                {p.summary.title}
              </Link>
            ) : (
              <span className="font-mono">
                {p.id} <span className="font-sans text-muted-foreground">({t("classroom.afterRestart")})</span>
              </span>
            )}
            <span className="ml-auto flex items-center gap-3">
              {p.best_verdict ? (
                <VerdictBadge verdict={p.best_verdict} />
              ) : (
                <span className="text-muted-foreground">{t("classroom.notTried")}</span>
              )}
              <span className="text-muted-foreground">{t("classroom.submits", { count: p.submits })}</span>
            </span>
          </li>
        ))}
      </ul>
      {a.state === "not_started" && (
        <p className="text-sm text-muted-foreground">{t("classroom.startNote")}</p>
      )}
      {a.state === "running" && (
        <p className="text-sm" data-testid="assignment-rules">
          {t(a.meta.hints ? "classroom.rules" : "classroom.rulesNoHints", { languages })}
        </p>
      )}
      {(start.error || finish.error) && <ErrorNotice error={start.error ?? finish.error} />}
      <div className="flex flex-wrap gap-2">
        {a.state === "not_started" && (
          <Button size="sm" onClick={() => start.mutate()} disabled={start.isPending} data-testid="assignment-start">
            <PlayCircle aria-hidden />
            {t("classroom.start")}
          </Button>
        )}
        {a.state === "running" && (
          <Button size="sm" variant="outline" onClick={() => finish.mutate()} disabled={finish.isPending}>
            <Square aria-hidden />
            {t("classroom.finish")}
          </Button>
        )}
        {a.state !== "not_started" && (
          <Button size="sm" variant="outline" onClick={() => setExporting(true)} data-testid="assignment-export">
            <FileDown aria-hidden />
            {t("classroom.export")}
          </Button>
        )}
      </div>
      {exporting && <ResultsDialog a={a} onClose={() => setExporting(false)} />}
    </article>
  );
}

/** The results file, shown exactly as it will be saved (what is included, nothing more). */
function ResultsDialog({ a, onClose }: { a: AssignmentView; onClose: () => void }) {
  const { t, i18n } = useTranslation();
  const [student, setStudent] = useSetting<string>("classroom.student", "", (v) =>
    typeof v === "string" ? v : undefined,
  );
  const [includeCode, setIncludeCode] = useState(false);
  const name = student.trim();
  const preview = useQuery({
    queryKey: ["assignment-results", a.id, name, includeCode],
    queryFn: () => api.assignmentResults(a.id, name, includeCode),
    enabled: name.length > 0,
  });
  const save = useMutation({ mutationFn: () => api.saveAssignmentResults(a.id, name, includeCode) });
  const fmt = new Intl.DateTimeFormat(i18n.language, { dateStyle: "medium", timeStyle: "short" });
  const f = preview.data;
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent data-testid="results-dialog" className="max-h-[85vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>{t("classroom.results.title", { title: a.meta.title })}</DialogTitle>
          <DialogDescription>{t("classroom.results.intro")}</DialogDescription>
        </DialogHeader>
        <label className="space-y-1 text-sm">
          <span className="font-medium">{t("classroom.results.name")}</span>
          <Input
            value={student}
            maxLength={80}
            onChange={(e) => setStudent(e.target.value)}
            aria-label={t("classroom.results.name")}
            data-testid="results-name"
          />
        </label>
        <label className="flex items-center gap-2 text-sm">
          <Checkbox
            checked={includeCode}
            onCheckedChange={(v) => setIncludeCode(v === true)}
            aria-label={t("classroom.results.includeCode")}
          />
          {t("classroom.results.includeCode")}
        </label>
        {preview.error && <ErrorNotice error={preview.error} />}
        {f && (
          <div className="space-y-2 text-sm" data-testid="results-preview">
            <p className="font-medium">{t("classroom.results.included")}</p>
            <ul className="list-disc space-y-1 pl-5">
              <li>{t("classroom.results.who", { name: f.student })}</li>
              <li>{t("classroom.results.which", { title: f.assignment.title, hash: f.assignment.pack_sha256.slice(0, 12) })}</li>
              <li>
                {t("classroom.results.when", { start: fmt.format(new Date(f.started_at)), end: fmt.format(new Date(f.ended_at)) })}
              </li>
              {f.entries.map((e) => (
                <li key={e.problem_id}>
                  <span className="font-mono">{e.problem_id}</span>:{" "}
                  {e.best_verdict ? t(`judge.verdict.${e.best_verdict}`) : t("classroom.notTried")}
                  {", "}
                  {t("classroom.submits", { count: e.submits })}
                  {e.language && `, ${t(`languages.${e.language}` as "languages.python")}`}
                  {e.first_accepted_s !== null &&
                    `, ${t("classroom.results.firstAccepted", { time: formatClock(e.first_accepted_s) })}`}
                  {e.code !== null && `, ${t("classroom.results.withCode")}`}
                </li>
              ))}
              <li>{t("classroom.results.chain")}</li>
            </ul>
          </div>
        )}
        {save.error && <ErrorNotice error={save.error} />}
        {save.data && (
          <p role="status" className="text-sm" data-testid="results-saved">
            {t("classroom.results.saved")}
          </p>
        )}
        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            {t("classroom.close")}
          </Button>
          <Button onClick={() => save.mutate()} disabled={!f || save.isPending} data-testid="results-save">
            <Download aria-hidden />
            {t("classroom.results.save")}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function TeacherPanel() {
  const { t } = useTranslation();
  const qc = useQueryClient();
  const results = useQuery({ queryKey: ["class-results"], queryFn: () => api.classResults() });
  const [last, setLast] = useState<ClassImport | null>(null);
  const imp = useMutation({
    mutationFn: () => api.importClassResults(),
    onSuccess: (r) => {
      if (r) setLast(r);
      void qc.invalidateQueries({ queryKey: ["class-results"] });
    },
  });
  return (
    <div className="space-y-3">
      <p className="text-sm text-muted-foreground">{t("classroom.teacher.intro")}</p>
      <Button size="sm" variant="outline" onClick={() => imp.mutate()} disabled={imp.isPending} data-testid="class-import">
        <Upload aria-hidden />
        {t("classroom.teacher.import")}
      </Button>
      {imp.error && <ErrorNotice error={imp.error} />}
      {last && (
        <div role="status" className="space-y-1 rounded-md border bg-muted/40 p-2 text-sm" data-testid="class-import-result">
          <p>{t("classroom.teacher.imported", { added: last.added, duplicates: last.duplicates, edited: last.edited })}</p>
          {last.refused.length > 0 && (
            <ul className="list-disc pl-5">
              {last.refused.map(([file, why], i) => (
                <li key={i}>{t("classroom.teacher.refused", { file: file || "…", why })}</li>
              ))}
            </ul>
          )}
        </div>
      )}
      {results.error && <ErrorNotice error={results.error} />}
      {(results.data?.length ?? 0) === 0 ? (
        <p className="text-sm text-muted-foreground">{t("classroom.teacher.none")}</p>
      ) : (
        results.data!.map((a) => <ClassTable key={a.id} a={a} />)
      )}
    </div>
  );
}

function ClassTable({ a }: { a: ClassAssignment }) {
  const { t } = useTranslation();
  const csv = useMutation({ mutationFn: () => api.saveClassCsv(a.id) });
  return (
    <div className="space-y-2" data-testid={`class-${a.id}`}>
      <div className="flex flex-wrap items-center gap-2">
        <h3 className="font-medium">
          {a.title} <span className="font-mono text-xs text-muted-foreground">({a.id})</span>
        </h3>
        <Button size="sm" variant="ghost" className="ml-auto" onClick={() => csv.mutate()} disabled={csv.isPending}>
          <FileDown aria-hidden />
          {t("classroom.teacher.csv")}
        </Button>
      </div>
      {csv.error && <ErrorNotice error={csv.error} />}
      <div className="overflow-x-auto rounded-md border">
        <table className="w-full text-sm">
          <caption className="sr-only">{t("classroom.teacher.caption", { title: a.title })}</caption>
          <thead className="bg-muted/40 text-left">
            <tr>
              <th scope="col" className="px-3 py-2 font-medium">
                {t("classroom.teacher.student")}
              </th>
              <th scope="col" className="px-3 py-2 font-medium">
                {t("classroom.teacher.file")}
              </th>
              {a.problems.map((p) => (
                <th key={p} scope="col" className="px-3 py-2 font-mono text-xs font-medium">
                  {p}
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="divide-y">
            {a.students.map((s) => (
              <tr key={s.student}>
                <th scope="row" className="px-3 py-2 text-left font-medium">
                  {s.student}
                </th>
                <td className="px-3 py-2">
                  {s.chain_ok ? (
                    <span className="flex items-center gap-1">
                      <ShieldCheck aria-hidden className="size-4" />
                      {t("classroom.teacher.unchanged")}
                    </span>
                  ) : (
                    <span className="flex items-center gap-1 font-medium" data-testid="class-edited">
                      <ShieldAlert aria-hidden className="size-4" />
                      {t("classroom.teacher.edited")}
                    </span>
                  )}
                </td>
                {s.cells.map((c, i) => (
                  <td key={a.problems[i]} className="px-3 py-2">
                    {c?.best_verdict ? (
                      <span className="flex flex-col gap-0.5">
                        <VerdictBadge verdict={c.best_verdict} />
                        <span className="text-xs text-muted-foreground">{t("classroom.submits", { count: c.submits })}</span>
                      </span>
                    ) : (
                      <span className="text-muted-foreground">{t("classroom.notTried")}</span>
                    )}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
