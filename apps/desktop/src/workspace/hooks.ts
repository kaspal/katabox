// Workspace state (ADR 0013): the editor session (draft load/save per problem and
// language) and judge runs (Run/Submit with progress).
import { useCallback, useEffect, useRef, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import type { JudgeEvent, Language, Report } from "@/bindings";
import { api, type CustomInput } from "@/lib/api";

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
 */
export function useEditorSession(problem: string, language: Language | null) {
  const cache = useRef(new Map<string, string>());
  const pending = useRef<{ lang: Language; code: string } | null>(null);
  const timer = useRef<number | undefined>(undefined);
  const [doc, setDoc] = useState<EditorDoc | null>(null);
  const [revision, setRevision] = useState(0);
  const [save, setSave] = useState<SaveState>("idle");
  const [loadError, setLoadError] = useState<unknown>(null);

  const write = useCallback(
    async (lang: Language, text: string) => {
      setSave("saving");
      try {
        await api.saveDraft(problem, lang, text);
        setSave("saved");
      } catch {
        setSave("error");
      }
    },
    [problem],
  );

  const flush = useCallback(async () => {
    window.clearTimeout(timer.current);
    const p = pending.current;
    pending.current = null;
    if (p) await write(p.lang, p.code);
  }, [write]);

  // A new problem starts empty (no cross-problem leftovers).
  useEffect(() => {
    cache.current.clear();
    setDoc(null);
    setSave("idle");
  }, [problem]);

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
        const draft = await api.loadDraft(problem, language);
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
  }, [problem, language, revision]);

  // Save what is pending when leaving the problem.
  useEffect(() => () => void flush(), [flush]);

  const onChange = useCallback(
    (text: string) => {
      setDoc((d) => {
        if (!d) return d;
        cache.current.set(d.key, text);
        pending.current = { lang: d.lang, code: text };
        return { ...d, text };
      });
      setSave("saving");
      window.clearTimeout(timer.current);
      timer.current = window.setTimeout(() => void flush(), AUTOSAVE_MS);
    },
    [flush],
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
  const [busy, setBusy] = useState<"run" | "submit" | null>(null);
  const [phase, setPhase] = useState<Phase>({ kind: "idle" });
  const [report, setReport] = useState<Report | null>(null);
  const [error, setError] = useState<unknown>(null);
  const [lastKind, setLastKind] = useState<"run" | "submit" | null>(null);
  const running = useRef(false);

  const start = useCallback(
    async (kind: "run" | "submit", job: (onEvent: (e: JudgeEvent) => void) => Promise<Report>) => {
      if (running.current) return;
      running.current = true;
      setBusy(kind);
      setLastKind(kind);
      setError(null);
      setReport(null);
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
        if (kind === "submit") {
          for (const key of [["submissions", problem], ["editorial", problem], ["problems"], ["continue"]]) {
            void queryClient.invalidateQueries({ queryKey: key });
          }
        }
      }
    },
    [problem, queryClient],
  );

  const run = useCallback(
    (language: Language, code: string, custom: CustomInput[]) =>
      start("run", (onEvent) => api.run(problem, language, code, custom, onEvent)),
    [problem, start],
  );
  const submit = useCallback(
    (language: Language, code: string) =>
      start("submit", async (onEvent) => (await api.submit(problem, language, code, onEvent)).report),
    [problem, start],
  );

  return { busy, phase, report, error, lastKind, run, submit };
}
