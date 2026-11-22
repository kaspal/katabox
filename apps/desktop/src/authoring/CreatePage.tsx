// The in-app problem editor (expansion M13, ADR 0040). A form over a draft that kb-app
// keeps in the app's own folder (the UI never touches files): saved as you type, checked
// with a pack's rules (issues shown by field), judged in the sandbox on demand, and
// exported as a pack the import accepts.
import { useEffect, useMemo, useRef, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { AlertCircle, CheckCircle2, FilePlus2, FlaskConical, PackagePlus, Plus, Trash2, X } from "lucide-react";
import type { DraftIssue, DraftTest, JsonValue, Language, ProblemDraft, Report } from "@/bindings";
import { Page } from "@/components/Page";
import { ErrorNotice } from "@/components/ErrorNotice";
import { usePatternLabel } from "@/components/Semantic";
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
import { api } from "@/lib/api";
import { JUDGE_LANGUAGES } from "@/lib/settings";
import { Markdown } from "@/workspace/problem/Markdown";
import { ParamBuilder } from "@/workspace/results/builders/Builders";

const SELECT = "h-9 w-full rounded-md border bg-card px-2 text-sm text-foreground";
const AREA = "w-full rounded-md border bg-card p-2 text-sm text-foreground";
const ID_RE = /^[a-z0-9]+(-[a-z0-9]+)*$/;
const CHECKERS = ["exact", "unordered", "float:1e-6"];
const SAVE_DELAY_MS = 700;

export function CreatePage() {
  const { t } = useTranslation();
  const qc = useQueryClient();
  const drafts = useQuery({ queryKey: ["authoring"], queryFn: () => api.authoringDrafts() });
  const [selected, setSelected] = useState<string | null>(null);
  const [newId, setNewId] = useState("");
  const create = useMutation({
    mutationFn: (id: string) => api.authoringNew(id),
    onSuccess: (d) => {
      setNewId("");
      setSelected(d.id);
      void qc.invalidateQueries({ queryKey: ["authoring"] });
    },
  });
  const idOk = ID_RE.test(newId) && newId.length <= 40;
  return (
    <Page title={t("authoring.title")}>
      <div className="flex min-h-0 flex-1 gap-6">
        <aside className="w-60 flex-none space-y-3" aria-label={t("authoring.drafts")}>
          <form
            className="space-y-2"
            onSubmit={(e) => {
              e.preventDefault();
              if (idOk) create.mutate(newId);
            }}
          >
            <label className="block space-y-1 text-sm">
              <span className="font-medium">{t("authoring.newId")}</span>
              <Input
                value={newId}
                onChange={(e) => setNewId(e.target.value.trim())}
                placeholder="tallest-tower"
                data-testid="authoring-new-id"
              />
            </label>
            <Button type="submit" size="sm" disabled={!idOk || create.isPending} data-testid="authoring-create">
              <FilePlus2 aria-hidden />
              {t("authoring.create")}
            </Button>
            {create.error && <ErrorNotice error={create.error} />}
          </form>
          <ul className="space-y-1" data-testid="authoring-list">
            {(drafts.data ?? []).map((d) => (
              <li key={d.id}>
                <Button
                  variant={selected === d.id ? "secondary" : "ghost"}
                  size="sm"
                  className="h-auto w-full flex-col items-start py-1.5 text-left"
                  aria-current={selected === d.id ? "true" : undefined}
                  onClick={() => setSelected(d.id)}
                >
                  <span className="font-medium">{d.title || t("authoring.untitled")}</span>
                  <span className="font-mono text-xs text-muted-foreground">{d.id}</span>
                </Button>
              </li>
            ))}
          </ul>
          <p className="text-xs text-muted-foreground">{t("authoring.quality")}</p>
        </aside>
        <div className="min-w-0 flex-1 overflow-y-auto pb-6">
          {selected ? (
            <DraftEditor
              key={selected}
              id={selected}
              others={(drafts.data ?? []).map((d) => d.id)}
              onDeleted={() => {
                setSelected(null);
                void qc.invalidateQueries({ queryKey: ["authoring"] });
              }}
            />
          ) : (
            <p className="text-sm text-muted-foreground">{t("authoring.pick")}</p>
          )}
        </div>
      </div>
    </Page>
  );
}

function DraftEditor({ id, others, onDeleted }: { id: string; others: string[]; onDeleted: () => void }) {
  const { t } = useTranslation();
  const qc = useQueryClient();
  const loaded = useQuery({ queryKey: ["authoring", id], queryFn: () => api.authoringLoad(id), staleTime: Infinity });
  const firstIssues = useQuery({ queryKey: ["authoring-issues", id], queryFn: () => api.authoringIssues(id) });
  const [draft, setDraft] = useState<ProblemDraft | null>(null);
  const [issues, setIssues] = useState<DraftIssue[] | null>(null);
  const [saveState, setSaveState] = useState<"saved" | "saving" | "error">("saved");
  const [saveError, setSaveError] = useState<unknown>(null);
  const [report, setReport] = useState<Report | null>(null);
  const [exporting, setExporting] = useState(false);
  const [confirmDelete, setConfirmDelete] = useState(false);
  const pending = useRef<ProblemDraft | null>(null);

  useEffect(() => {
    if (loaded.data && draft === null) setDraft(loaded.data);
  }, [loaded.data, draft]);

  // Save shortly after the last change, and on leaving.
  const flush = useRef<() => void>(() => {});
  flush.current = () => {
    const d = pending.current;
    if (!d) return;
    pending.current = null;
    setSaveState("saving");
    api
      .authoringSave(d)
      .then((found) => {
        setIssues(found);
        setSaveState(pending.current ? "saving" : "saved");
        void qc.invalidateQueries({ queryKey: ["authoring"], exact: true });
      })
      .catch((e: unknown) => {
        setSaveError(e);
        setSaveState("error");
      });
  };
  useEffect(() => {
    if (!pending.current) return;
    const h = window.setTimeout(() => flush.current(), SAVE_DELAY_MS);
    return () => window.clearTimeout(h);
  }, [draft]);
  useEffect(() => () => flush.current(), []);

  const check = useMutation({
    mutationFn: () => api.authoringCheck(id),
    onMutate: () => {
      flush.current();
      setReport(null);
    },
    onSuccess: setReport,
  });
  const remove = useMutation({ mutationFn: () => api.authoringDelete(id), onSuccess: onDeleted });

  if (loaded.error) return <ErrorNotice error={loaded.error} />;
  if (!draft) return null;
  const update = (patch: Partial<ProblemDraft>) => {
    const next = { ...draft, ...patch };
    pending.current = next;
    setSaveState("saving");
    setDraft(next);
  };
  const shown = issues ?? firstIssues.data ?? [];
  const byField = (field: string) => shown.filter((i) => i.field === field);
  const passed = report ? report.tests.filter((x) => x.verdict === "accepted").length : 0;

  return (
    <div className="max-w-3xl space-y-6" data-testid="authoring-editor">
      <div className="flex flex-wrap items-center gap-2">
        <h2 className="font-mono text-lg font-semibold">{id}</h2>
        <span className="text-sm text-muted-foreground" role="status" data-testid="authoring-save-state">
          {t(`authoring.save.${saveState}`)}
        </span>
        <span className="ml-auto flex flex-wrap gap-2">
          <Button
            size="sm"
            variant="outline"
            onClick={() => check.mutate()}
            disabled={check.isPending || shown.length > 0}
            data-testid="authoring-check"
          >
            <FlaskConical aria-hidden />
            {t("authoring.check")}
          </Button>
          <Button size="sm" variant="outline" onClick={() => setExporting(true)} disabled={shown.length > 0}>
            <PackagePlus aria-hidden />
            {t("authoring.export.open")}
          </Button>
          {confirmDelete ? (
            <Button size="sm" variant="destructive" onClick={() => remove.mutate()}>
              <Trash2 aria-hidden />
              {t("authoring.deleteConfirm")}
            </Button>
          ) : (
            <Button size="sm" variant="ghost" onClick={() => setConfirmDelete(true)}>
              <Trash2 aria-hidden />
              {t("authoring.delete")}
            </Button>
          )}
        </span>
      </div>
      {saveState === "error" && <ErrorNotice error={saveError} />}
      <div
        className="flex items-start gap-2 rounded-md border bg-muted/40 p-2 text-sm"
        data-testid="authoring-issues"
      >
        {shown.length === 0 ? (
          <>
            <CheckCircle2 aria-hidden className="mt-0.5 size-4 flex-none text-verdict-accepted" />
            {t("authoring.ready")}
          </>
        ) : (
          <>
            <AlertCircle aria-hidden className="mt-0.5 size-4 flex-none" />
            {t("authoring.issues", { count: shown.length })}
          </>
        )}
      </div>
      {byField("general").length > 0 && <Issues list={byField("general")} />}
      {check.error && <ErrorNotice error={check.error} />}
      {report && (
        <p className="flex flex-wrap items-center gap-2 text-sm" role="status" data-testid="authoring-report">
          <VerdictBadge verdict={report.verdict} />
          {t("authoring.passed", { passed, total: report.tests.length })}
        </p>
      )}

      <Section title={t("authoring.basics")}>
        <Field label={t("authoring.field.title")} issues={byField("title")}>
          <Input value={draft.title} onChange={(e) => update({ title: e.target.value })} />
        </Field>
        <Field label={t("authoring.field.title_es")} issues={byField("title_es")}>
          <Input value={draft.title_es} onChange={(e) => update({ title_es: e.target.value })} />
        </Field>
        <div className="grid gap-3 sm:grid-cols-3">
          <Field label={t("authoring.field.difficulty")} issues={[]}>
            <select
              className={SELECT}
              value={draft.difficulty}
              onChange={(e) => update({ difficulty: e.target.value as ProblemDraft["difficulty"] })}
            >
              {(["easy", "medium", "hard"] as const).map((d) => (
                <option key={d} value={d}>
                  {t(`difficulty.${d}`)}
                </option>
              ))}
            </select>
          </Field>
          <Field label={t("authoring.field.pattern")} issues={byField("pattern")}>
            <PatternSelect value={draft.pattern} onChange={(pattern) => update({ pattern })} />
          </Field>
          <Field label={t("authoring.field.rank")} issues={byField("rank")}>
            <Input
              type="number"
              min={1}
              max={10}
              value={draft.rank}
              onChange={(e) => update({ rank: clampInt(e.target.value, 1, 10) })}
            />
          </Field>
        </div>
        <Field label={t("authoring.field.tags")} hint={t("authoring.tagsHint")} issues={byField("tags")}>
          <Input
            defaultValue={draft.tags.join(", ")}
            onChange={(e) =>
              update({
                tags: e.target.value
                  .split(",")
                  .map((s) => s.trim())
                  .filter(Boolean),
              })
            }
          />
        </Field>
      </Section>

      <Section title={t("authoring.signature")}>
        <SignatureEditor draft={draft} update={update} />
        <Issues list={byField("signature")} />
      </Section>

      <Section title={t("authoring.texts")}>
        <TextWithPreview
          label={t("authoring.field.statement")}
          value={draft.statement}
          onChange={(statement) => update({ statement })}
          issues={byField("statement")}
        />
        <TextWithPreview
          label={t("authoring.field.statement_es")}
          value={draft.statement_es}
          onChange={(statement_es) => update({ statement_es })}
          issues={byField("statement_es")}
        />
        <HintsEditor
          label={t("authoring.field.hints")}
          hints={draft.hints}
          onChange={(hints) => update({ hints })}
          issues={byField("hints")}
        />
        <HintsEditor
          label={t("authoring.field.hints_es")}
          hints={draft.hints_es}
          onChange={(hints_es) => update({ hints_es })}
          issues={[]}
        />
        <TextWithPreview
          label={t("authoring.field.editorial")}
          value={draft.editorial}
          onChange={(editorial) => update({ editorial })}
          issues={byField("editorial")}
        />
        <TextWithPreview
          label={t("authoring.field.editorial_es")}
          value={draft.editorial_es}
          onChange={(editorial_es) => update({ editorial_es })}
          issues={[]}
        />
      </Section>

      <Section title={t("authoring.tests")}>
        <TestsEditor
          kind="public"
          draft={draft}
          tests={draft.public_tests}
          onChange={(public_tests) => update({ public_tests })}
          issues={byField("public_tests")}
        />
        <TestsEditor
          kind="hidden"
          draft={draft}
          tests={draft.hidden_tests}
          onChange={(hidden_tests) => update({ hidden_tests })}
          issues={byField("hidden_tests")}
        />
      </Section>

      <Section title={t("authoring.reference")}>
        <Field label={t("authoring.field.language")} issues={[]}>
          <select
            className={SELECT}
            value={draft.reference_language}
            onChange={(e) => update({ reference_language: e.target.value as Language })}
          >
            {JUDGE_LANGUAGES.map((l) => (
              <option key={l} value={l}>
                {t(`languages.${l}` as "languages.python")}
              </option>
            ))}
          </select>
        </Field>
        <StarterButton draft={draft} onCode={(reference) => update({ reference })} />
        <Field label={t("authoring.field.reference")} issues={byField("reference")}>
          <textarea
            className={`${AREA} font-mono`}
            rows={12}
            spellCheck={false}
            value={draft.reference}
            onChange={(e) => update({ reference: e.target.value })}
            data-testid="authoring-reference"
          />
        </Field>
      </Section>

      <Section title={t("authoring.limits")}>
        <div className="grid gap-3 sm:grid-cols-3">
          <Field label={t("authoring.field.checker")} issues={byField("checker")}>
            <select className={SELECT} value={draft.checker} onChange={(e) => update({ checker: e.target.value })}>
              {(CHECKERS.includes(draft.checker) ? CHECKERS : [...CHECKERS, draft.checker]).map((c) => (
                <option key={c} value={c}>
                  {c}
                </option>
              ))}
            </select>
          </Field>
          <Field label={t("authoring.field.time")} issues={byField("limits")}>
            <Input
              type="number"
              min={100}
              max={10000}
              value={draft.time_limit_ms}
              onChange={(e) => update({ time_limit_ms: clampInt(e.target.value, 100, 10000) })}
            />
          </Field>
          <Field label={t("authoring.field.memory")} issues={[]}>
            <Input
              type="number"
              min={16}
              max={1024}
              value={draft.memory_mb}
              onChange={(e) => update({ memory_mb: clampInt(e.target.value, 16, 1024) })}
            />
          </Field>
        </div>
      </Section>
      {exporting && <ExportDialog current={id} drafts={others} onClose={() => setExporting(false)} />}
    </div>
  );
}

/** Fill the reference with the starter code: the exact declaration the judge expects. */
function StarterButton({ draft, onCode }: { draft: ProblemDraft; onCode: (code: string) => void }) {
  const { t } = useTranslation();
  const starter = useMutation({ mutationFn: () => api.authoringStarter(draft), onSuccess: onCode });
  return (
    <div className="space-y-1">
      <Button size="sm" variant="outline" onClick={() => starter.mutate()} disabled={starter.isPending}>
        <FilePlus2 aria-hidden />
        {t(draft.reference.trim() ? "authoring.replaceStarter" : "authoring.insertStarter")}
      </Button>
      {starter.error && <ErrorNotice error={starter.error} />}
    </div>
  );
}

function clampInt(text: string, lo: number, hi: number): number {
  const n = Math.round(Number(text));
  return Number.isFinite(n) ? Math.min(hi, Math.max(lo, n)) : lo;
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <fieldset className="space-y-3 rounded-lg border p-4">
      <legend className="px-1 font-semibold">{title}</legend>
      {children}
    </fieldset>
  );
}

function Issues({ list }: { list: DraftIssue[] }) {
  if (list.length === 0) return null;
  return (
    <ul className="space-y-0.5 text-sm" data-testid="field-issues">
      {list.map((i, n) => (
        <li key={n} className="flex items-start gap-1.5">
          <AlertCircle aria-hidden className="mt-0.5 size-3.5 flex-none" />
          <span className="break-words">{i.message}</span>
        </li>
      ))}
    </ul>
  );
}

function Field({
  label,
  hint,
  issues,
  children,
}: {
  label: string;
  hint?: string;
  issues: DraftIssue[];
  children: React.ReactElement<{ "aria-label"?: string }>;
}) {
  return (
    <div className="space-y-1">
      <label className="block space-y-1 text-sm">
        <span className="font-medium">{label}</span>
        {children}
      </label>
      {hint && <p className="text-xs text-muted-foreground">{hint}</p>}
      <Issues list={issues} />
    </div>
  );
}

function PatternSelect({ value, onChange }: { value: string; onChange: (v: string) => void }) {
  const patterns = useQuery({ queryKey: ["authoring-patterns"], queryFn: () => api.authoringPatterns() });
  const label = usePatternLabel();
  const list = patterns.data ?? [value];
  return (
    <select className={SELECT} value={value} onChange={(e) => onChange(e.target.value)}>
      {(list.includes(value) ? list : [value, ...list]).map((p) => (
        <option key={p} value={p}>
          {label(p)}
        </option>
      ))}
    </select>
  );
}

function SignatureEditor({ draft, update }: { draft: ProblemDraft; update: (p: Partial<ProblemDraft>) => void }) {
  const { t } = useTranslation();
  const setParam = (i: number, patch: Partial<ProblemDraft["params"][number]>) =>
    update({ params: draft.params.map((p, n) => (n === i ? { ...p, ...patch } : p)) });
  return (
    <div className="space-y-3">
      <div className="grid gap-3 sm:grid-cols-2">
        <Field label={t("authoring.field.function")} issues={[]}>
          <Input className="font-mono" value={draft.function} onChange={(e) => update({ function: e.target.value })} />
        </Field>
        <Field label={t("authoring.field.returns")} issues={[]}>
          <Input className="font-mono" value={draft.returns} onChange={(e) => update({ returns: e.target.value })} />
        </Field>
      </div>
      <p className="text-xs text-muted-foreground">{t("authoring.typesHint")}</p>
      <ul className="space-y-2">
        {draft.params.map((p, i) => (
          <li key={i} className="flex flex-wrap items-end gap-2">
            <label className="space-y-1 text-sm">
              <span className="font-medium">{t("authoring.field.paramName", { n: i + 1 })}</span>
              <Input className="w-40 font-mono" value={p.name} onChange={(e) => setParam(i, { name: e.target.value })} />
            </label>
            <label className="space-y-1 text-sm">
              <span className="font-medium">{t("authoring.field.paramType", { n: i + 1 })}</span>
              <Input className="w-40 font-mono" value={p.ty} onChange={(e) => setParam(i, { ty: e.target.value })} />
            </label>
            <Button
              size="sm"
              variant="ghost"
              aria-label={t("authoring.removeParam", { name: p.name || i + 1 })}
              onClick={() => update({ params: draft.params.filter((_, n) => n !== i) })}
            >
              <X aria-hidden />
            </Button>
          </li>
        ))}
      </ul>
      <Button
        size="sm"
        variant="outline"
        onClick={() => update({ params: [...draft.params, { name: `p${draft.params.length + 1}`, ty: "int" }] })}
      >
        <Plus aria-hidden />
        {t("authoring.addParam")}
      </Button>
    </div>
  );
}

function TextWithPreview({
  label,
  value,
  onChange,
  issues,
}: {
  label: string;
  value: string;
  onChange: (v: string) => void;
  issues: DraftIssue[];
}) {
  const { t } = useTranslation();
  const [preview, setPreview] = useState(false);
  return (
    <div className="space-y-1">
      <div className="flex items-center gap-2">
        <label className="flex-1 space-y-1 text-sm">
          <span className="font-medium">{label}</span>
          <textarea className={AREA} rows={6} value={value} onChange={(e) => onChange(e.target.value)} />
        </label>
      </div>
      <Button size="sm" variant="ghost" className="h-7" aria-expanded={preview} onClick={() => setPreview((p) => !p)}>
        {t(preview ? "authoring.hidePreview" : "authoring.showPreview")}
      </Button>
      {preview && (
        <div className="rounded-md border p-3" data-testid="authoring-preview">
          <Markdown text={value} trust="pack" />
        </div>
      )}
      <Issues list={issues} />
    </div>
  );
}

function HintsEditor({
  label,
  hints,
  onChange,
  issues,
}: {
  label: string;
  hints: string[];
  onChange: (v: string[]) => void;
  issues: DraftIssue[];
}) {
  const { t } = useTranslation();
  const three = [0, 1, 2].map((i) => hints[i] ?? "");
  return (
    <div className="space-y-1">
      <p className="text-sm font-medium">{label}</p>
      {three.map((h, i) => (
        <label key={i} className="relative block text-sm">
          <span className="sr-only">{t("authoring.hintTier", { label, n: i + 1 })}</span>
          <textarea
            className={AREA}
            rows={2}
            placeholder={t("authoring.hintTier", { label, n: i + 1 })}
            value={h}
            onChange={(e) => onChange(three.map((x, n) => (n === i ? e.target.value : x)))}
          />
        </label>
      ))}
      <Issues list={issues} />
    </div>
  );
}

function TestsEditor({
  kind,
  draft,
  tests,
  onChange,
  issues,
}: {
  kind: "public" | "hidden";
  draft: ProblemDraft;
  tests: DraftTest[];
  onChange: (v: DraftTest[]) => void;
  issues: DraftIssue[];
}) {
  const { t } = useTranslation();
  const blank = (): DraftTest => ({
    input: Object.fromEntries(draft.params.map((p) => [p.name, null])),
    output: null,
  });
  return (
    <div className="space-y-2" data-testid={`tests-${kind}`}>
      <p className="text-sm font-medium">{t(`authoring.${kind}Tests`, { count: tests.length })}</p>
      <ol className="space-y-3">
        {tests.map((test, i) => (
          <li key={i} className="space-y-2 rounded-md border p-3">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">{t("authoring.testN", { n: i + 1 })}</span>
              <Button
                size="sm"
                variant="ghost"
                aria-label={t("authoring.removeTest", { n: i + 1 })}
                onClick={() => onChange(tests.filter((_, n) => n !== i))}
              >
                <X aria-hidden />
              </Button>
            </div>
            {draft.params.map((p) => (
              <JsonField
                key={p.name}
                label={`${p.name}: ${p.ty}`}
                value={test.input[p.name] ?? null}
                builder={{ name: p.name, type: p.ty }}
                onValue={(v) =>
                  onChange(tests.map((x, n) => (n === i ? { ...x, input: { ...x.input, [p.name]: v } } : x)))
                }
              />
            ))}
            <JsonField
              label={t("authoring.output", { type: draft.returns })}
              value={test.output}
              onValue={(v) => onChange(tests.map((x, n) => (n === i ? { ...x, output: v } : x)))}
            />
          </li>
        ))}
      </ol>
      <Button size="sm" variant="outline" onClick={() => onChange([...tests, blank()])}>
        <Plus aria-hidden />
        {t(`authoring.add${kind === "public" ? "Public" : "Hidden"}`)}
      </Button>
      <Issues list={issues} />
    </div>
  );
}

/** A JSON value edited as text (kept while it doesn't parse), with a visual builder when
 * the type has one (expansion M9). */
function JsonField({
  label,
  value,
  onValue,
  builder,
}: {
  label: string;
  value: JsonValue | undefined;
  onValue: (v: JsonValue) => void;
  builder?: { name: string; type: string };
}) {
  const { t } = useTranslation();
  const canonical = useMemo(() => (value === null || value === undefined ? "" : JSON.stringify(value)), [value]);
  const [text, setText] = useState(canonical);
  const [bad, setBad] = useState(false);
  useEffect(() => {
    // Follow outside changes (a builder) unless the text already means the same value.
    setText((old) => {
      try {
        return JSON.stringify(JSON.parse(old)) === canonical ? old : canonical;
      } catch {
        return canonical;
      }
    });
  }, [canonical]);
  const edit = (next: string) => {
    setText(next);
    try {
      onValue(next.trim() === "" ? null : (JSON.parse(next) as JsonValue));
      setBad(false);
    } catch {
      setBad(true);
    }
  };
  return (
    <div className="space-y-1">
      <label className="block space-y-1 text-sm">
        <span className="font-mono text-xs">{label}</span>
        <Input className="font-mono" value={text} onChange={(e) => edit(e.target.value)} aria-invalid={bad || undefined} />
      </label>
      {bad && <p className="text-xs">{t("authoring.badJson")}</p>}
      {builder && <ParamBuilder name={builder.name} type={builder.type} text={text} onText={edit} />}
    </div>
  );
}

function ExportDialog({ current, drafts, onClose }: { current: string; drafts: string[]; onClose: () => void }) {
  const { t } = useTranslation();
  const [head, setHead] = useState({ id: "", name: "", author: "", version: "1.0.0", licence: "CC-BY-SA-4.0" });
  const [ids, setIds] = useState<string[]>([current]);
  const run = useMutation({ mutationFn: () => api.authoringExport(ids, head) });
  const set = (k: keyof typeof head) => (e: React.ChangeEvent<HTMLInputElement>) => setHead({ ...head, [k]: e.target.value });
  const ready = /^[a-z][a-z0-9]{2,19}$/.test(head.id) && head.name.trim() && head.author.trim() && ids.length > 0;
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent data-testid="authoring-export" className="max-h-[85vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>{t("authoring.export.title")}</DialogTitle>
          <DialogDescription>{t("authoring.export.intro")}</DialogDescription>
        </DialogHeader>
        {(["id", "name", "author", "version", "licence"] as const).map((k) => (
          <label key={k} className="block space-y-1 text-sm">
            <span className="font-medium">{t(`authoring.export.${k}`)}</span>
            <Input value={head[k]} onChange={set(k)} data-testid={`export-${k}`} />
          </label>
        ))}
        <fieldset className="space-y-1 text-sm">
          <legend className="font-medium">{t("authoring.export.problems")}</legend>
          {drafts.map((d) => (
            <label key={d} className="flex items-center gap-2">
              <Checkbox
                checked={ids.includes(d)}
                onCheckedChange={(v) => setIds(v === true ? [...ids, d] : ids.filter((x) => x !== d))}
                aria-label={d}
              />
              <span className="font-mono">{d}</span>
            </label>
          ))}
        </fieldset>
        {run.error && <ErrorNotice error={run.error} />}
        {run.data && (
          <p role="status" className="break-all text-sm" data-testid="authoring-exported">
            {t("authoring.export.done", { sha: run.data })}
          </p>
        )}
        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            {t("authoring.export.close")}
          </Button>
          <Button onClick={() => run.mutate()} disabled={!ready || run.isPending} data-testid="export-save">
            <PackagePlus aria-hidden />
            {t("authoring.export.save")}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
