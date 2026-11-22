// Workspace state (ADR 0013): the editor session (draft load/save per problem and
// language) and judge runs (Run/Submit with progress).
import { useCallback, useEffect, useRef, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import type { JudgeEvent, KataOutcome, Language, PracticeOutcome, Report } from "@/bindings";
import type { ComplexityState } from "@/workspace/results/ComplexityPanel";
import { api, type CustomInput } from "@/lib/api";
import { practiceKeys } from "@/lib/practice";
import { usePrefs } from "@/lib/prefs";
import { queryKeys, useCalibration } from "@/lib/queries";
import { localToday } from "@/lib/today";

export const AUTOSAVE_MS = 600;

export type SaveState = "idle" | "saving" | "saved" | "error";

/** The document in the editor: which language it belongs to, and its text. */
export type EditorDoc = { lang: Language; key: string; text: string; rev: number };

/**
 * The code being edited for `problem` in `language`: the saved draft, else the stub.
 * `doc` only changes once the new language's text is ready, so the editor never pairs
 * one language's key with another's text and stays mounted while loading.
 * Edits autosave after AUTOSAVE_MS; `flush()` saves now (before Run/Submit and before
 * switching language). `reload()` drops the cached text (after restoring a snapshot).
 *
 * Review mode (ADR 0019): every language starts from its stub and nothing is saved, so
 * the earlier code stays hidden and untouched until the review is passed.
 */
export function useEditorSession(problem: string, language: Language | null, opts: { review?: boolean } = {}) {
  const review = opts.review === true;
  const cache = useRef(new Map<string, string>());
  const pending = useRef<{ lang: Language; code: string } | null>(null);
  const timer = useRef<number | undefined>(undefined);
  const [doc, setDoc] = useState<EditorDoc | null>(null);
  const [revision, setRevision] = useState(0);
  const [save, setSave] = useState<SaveState>("idle");
  const [loadError, setLoadError] = useState<unknown>(null);

  const write = useCallback(
    async (lang: Language, text: string) => {
      if (review) return;
      setSave("saving");
      try {
        await api.saveDraft(problem, lang, text);
        setSave("saved");
      } catch {
        setSave("error");
      }
    },
    [problem, review],
  );

  const flush = useCallback(async () => {
    window.clearTimeout(timer.current);
    const p = pending.current;
    pending.current = null;
    if (p) await write(p.lang, p.code);
  }, [write]);

  // A new problem (or entering/leaving review mode) starts empty.
  useEffect(() => {
    cache.current.clear();
    setDoc(null);
    setSave("idle");
  }, [problem, review]);

  // Load when the language or revision changes.
  useEffect(() => {
    if (!language) return;
    let live = true;
    const key = `${problem}:${language}`;
    const show = (text: string) => setDoc({ lang: language, key, text, rev: revision });
    const cached = cache.current.get(key);
    if (cached !== undefined) {
      show(cached);
      return;
    }
    setLoadError(null);
    void (async () => {
      try {
        const draft = review ? null : await api.loadDraft(problem, language);
        const text = draft ? draft.code : await api.getStub(problem, language);
        if (!live) return;
        cache.current.set(key, text);
        show(text);
      } catch (e) {
        if (live) setLoadError(e);
      }
    })();
    return () => {
      live = false;
    };
  }, [problem, language, revision, review]);

  // Save what is pending when leaving the problem.
  useEffect(() => () => void flush(), [flush]);

  const onChange = useCallback(
    (text: string) => {
      setDoc((d) => {
        if (!d) return d;
        cache.current.set(d.key, text);
        if (!review) pending.current = { lang: d.lang, code: text };
        return { ...d, text };
      });
      if (review) return;
      setSave("saving");
      window.clearTimeout(timer.current);
      timer.current = window.setTimeout(() => void flush(), AUTOSAVE_MS);
    },
    [flush, review],
  );

  const reload = useCallback(
    (lang: Language) => {
      cache.current.delete(`${problem}:${lang}`);
      setRevision((r) => r + 1);
    },
    [problem],
  );

  const current = doc && doc.lang === language ? doc : null;
  return {
    /** The document shown in the editor (possibly the previous language while loading). */
    doc,
    /** The code of the selected language, once loaded; null while loading. */
    code: current ? current.text : null,
    docKey: doc ? `${doc.key}:${doc.rev}` : "",
    save,
    loadError,
    onChange,
    flush,
    reload,
  };
}

export type Phase = { kind: "idle" } | { kind: "starting" } | { kind: "event"; event: JudgeEvent };

/** Run and Submit with progress; one at a time. */
export function useJudge(problem: string) {
  const queryClient = useQueryClient();
  const { lang } = usePrefs();
  /** What the last Submit did for practice (review, daily problem, next problem). */
  const [practice, setPractice] = useState<PracticeOutcome | null>(null);
  /** What the last kata Submit did (expansion M1). */
  const [kata, setKata] = useState<KataOutcome | null>(null);
  /** The stored submission of the last Submit (none after a Run or a kata Submit). */
  const [submissionId, setSubmissionId] = useState<number | null>(null);
  // Unmeasured (null) until the first Run or Submit measures this machine.
  const measured = useCalibration().data != null;
  const [busy, setBusy] = useState<"run" | "submit" | null>(null);
  const [phase, setPhase] = useState<Phase>({ kind: "idle" });
  const [report, setReport] = useState<Report | null>(null);
  const [error, setError] = useState<unknown>(null);
  const [lastKind, setLastKind] = useState<"run" | "submit" | null>(null);
  /** The code and language the last report judged (the complexity profiler measures it). */
  const [judged, setJudged] = useState<{ language: Language; code: string } | null>(null);
  const running = useRef(false);

  // Another problem (e.g. opened from the Next card): forget the last result.
  useEffect(() => {
    setReport(null);
    setError(null);
    setPractice(null);
    setKata(null);
    setSubmissionId(null);
    setLastKind(null);
    setJudged(null);
    setPhase({ kind: "idle" });
  }, [problem]);

  const start = useCallback(
    async (
      kind: "run" | "submit",
      source: { language: Language; code: string },
      job: (onEvent: (e: JudgeEvent) => void) => Promise<Report>,
    ) => {
      if (running.current) return;
      running.current = true;
      setBusy(kind);
      setLastKind(kind);
      setJudged(source);
      setError(null);
      setReport(null);
      setPractice(null);
      setPhase({ kind: "starting" });
      try {
        const r = await job((event) => setPhase({ kind: "event", event }));
        setReport(r);
      } catch (e) {
        setError(e);
        setPhase({ kind: "idle" });
      } finally {
        running.current = false;
        setBusy(null);
        void queryClient.invalidateQueries({ queryKey: ["snapshots", problem] });
        // The first Run or Submit measures this machine (ADR 0017): refresh the limits.
        if (!measured) {
          void queryClient.invalidateQueries({ queryKey: queryKeys.calibration });
          void queryClient.invalidateQueries({ queryKey: ["problem", problem] });
        }
        if (kind === "submit") {
          for (const key of [
            ["submissions", problem],
            ["editorial", problem],
            ["problems"],
            ["continue"],
            practiceKeys.all,
          ]) {
            void queryClient.invalidateQueries({ queryKey: key });
          }
        }
      }
    },
    [problem, queryClient, measured],
  );

  const run = useCallback(
    (language: Language, code: string, custom: CustomInput[]) =>
      start("run", { language, code }, (onEvent) => api.run(problem, language, code, custom, onEvent)),
    [problem, start],
  );
  const submit = useCallback(
    (language: Language, code: string, opts: { review?: boolean; kata?: number | null } = {}) =>
      start("submit", { language, code }, async (onEvent) => {
        const result = await api.submit(
          problem,
          language,
          code,
          { today: localToday(), review: opts.review === true, lang, kata: opts.kata ?? null },
          onEvent,
        );
        setPractice(result.practice);
        setKata(result.kata);
        setSubmissionId(result.submission_id > 0 ? result.submission_id : null);
        if (result.kata) void queryClient.invalidateQueries({ queryKey: ["kata"] });
        return result.report;
      }),
    [problem, start, lang, queryClient],
  );

  return { busy, phase, report, error, lastKind, judged, run, submit, practice, kata, submissionId };
}

/**
 * The complexity profiler (ADR 0024) for the last judged code. It forgets its result
 * when another report arrives (new code) or the problem changes.
 */
export function useComplexity(problem: string, judged: { language: Language; code: string } | null) {
  const [state, setState] = useState<ComplexityState>({ status: "idle" });
  const running = useRef(false);
  useEffect(() => setState({ status: "idle" }), [problem, judged]);
  const measure = useCallback(async () => {
    if (running.current || !judged) return;
    running.current = true;
    setState({ status: "running" });
    try {
      const view = await api.profileSolution(problem, judged.language, judged.code);
      setState({ status: "done", view });
    } catch (error) {
      setState({ status: "error", error });
    } finally {
      running.current = false;
    }
  }, [problem, judged]);
  return { state, measure };
}
