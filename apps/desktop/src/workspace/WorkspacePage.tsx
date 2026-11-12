// The workspace (ADR 0013): problem pane | editor over tests/results. Loaded lazily, so
// CodeMirror, KaTeX and Markdown stay out of the library route.
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Link, getRouteApi, useNavigate } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import {
  ArrowLeft,
  Download,
  Loader2,
  Maximize2,
  Minimize2,
  PanelBottom,
  PanelLeft,
  Play,
  RotateCcw,
  Send,
} from "lucide-react";
import type { PanelImperativeHandle } from "react-resizable-panels";
import type { EditorView } from "@codemirror/view";
import { formatDocument } from "@codemirror/lsp-client";
import type { JsonValue, Language } from "@/bindings";
import { Button } from "@/components/ui/button";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { ErrorNotice } from "@/components/ErrorNotice";
import { InstallDialog } from "@/languages/InstallDialog";
import { DifficultyLabel } from "@/components/Semantic";
import { api } from "@/lib/api";
import { isAppError } from "@/lib/errors";
import { useFocusMode } from "@/lib/focus";
import { useReviewActions, useReviewState } from "@/lib/practice";
import { useInterviewActive, useSolvedLanguages } from "@/lib/modes";
import { InterviewTimer } from "@/practice/interview";
import { CompareView, SolveInMenu } from "@/practice/crossLanguage";
import { useAppInfo, useProblem } from "@/lib/queries";
import { AddToListMenu, formatDay } from "@/practice/shared";
import { JUDGE_LANGUAGES, parseLanguage, useEditorPrefs, useSetting } from "@/lib/settings";
import { CodeEditor, type EditorStatus } from "@/workspace/editor/CodeEditor";
import { EditorStatusBar } from "@/workspace/editor/lsp/EditorStatusBar";
import { phpBasicCompletion } from "@/workspace/editor/phpCompletions";
import { useLanguageServer } from "@/workspace/editor/lsp/useLanguageServer";
import { useEditorSession, useJudge } from "@/workspace/hooks";
import { ProblemPane } from "@/workspace/problem/ProblemPane";
import { ResultsPane, type ResultsTab } from "@/workspace/results/ResultsPane";
import { parseSignature, validateCustom, type CustomTestDraft } from "@/workspace/results/validate";

const route = getRouteApi("/problems/$problemId");

type Layouts = { main?: Record<string, number>; right?: Record<string, number> };
const parseLayouts = (v: JsonValue | null): Layouts | undefined =>
  typeof v === "object" && v !== null && !Array.isArray(v) ? (v as Layouts) : undefined;

export const CELEBRATE_MS = 1200;

/** Ctrl/Cmd + key, as shown in buttons and tooltips. */
const KEYS = { run: "Ctrl+'", submit: "Ctrl+Enter", focus: "Ctrl+Shift+F" };

export function WorkspacePage() {
  const { t, i18n } = useTranslation();
  const { problemId } = route.useParams();
  const navigate = useNavigate({ from: "/problems/$problemId" });
  const search = route.useSearch();
  const reviewMode = search.review === true;
  const compare = search.compare ?? null;
  const interview = useInterviewActive();
  const inInterview = interview.data?.problems.some((x) => x.id === problemId) === true;
  const allowed = inInterview ? interview.data!.languages : JUDGE_LANGUAGES;
  const solvedIn = useSolvedLanguages(problemId, !inInterview);
  const reviewState = useReviewState(problemId);
  const reviewActions = useReviewActions();
  const problem = useProblem(problemId);
  const info = useAppInfo();
  const { keymap, fontSize, ligatures } = useEditorPrefs();
  const [lastLanguage, setLastLanguage] = useSetting<Language>("editor.language", "python", parseLanguage, {
    mirror: true,
  });
  const [layouts, setLayouts] = useSetting<Layouts>("workspace.layout", {}, parseLayouts, { mirror: true });
  const [language, setLanguage] = useState<Language | null>(null);
  /** A language to offer installing (picked in the selector, or the current one). */
  const [installFor, setInstallFor] = useState<Language | null>(null);
  const session = useEditorSession(problemId, language, { review: reviewMode });
  const docLang = session.doc?.lang ?? null;
  const serverInfo = info.data?.language_servers.find((l) => l.language === docLang);
  const docText = useRef("");
  docText.current = session.doc?.text ?? "";
  const server = useLanguageServer(
    problemId,
    docLang,
    serverInfo && !serverInfo.available ? (serverInfo.reason ?? "") : null,
    () => docText.current,
  );
  const [editorStatus, setEditorStatus] = useState<EditorStatus | null>(null);
  // PHP has no language server on Windows: offer keywords and common functions instead.
  const basicCompletion = docLang === "php" && server.status.state === "unavailable";
  const editorView = useRef<EditorView | null>(null);
  const judge = useJudge(problemId);
  // Review mode stays on until a review-mode Submit passes; then the old code may be seen.
  const reviewOutcome = judge.practice?.review?.outcome;
  const reviewPassed = reviewOutcome === "scheduled" || reviewOutcome === "graduated";
  const locked = reviewMode && !reviewPassed;
  const { focus, setFocus } = useFocusMode();
  const [tab, setTab] = useState<ResultsTab>("testcases");
  const [customs, setCustoms] = useState<CustomTestDraft[]>([]);
  const [celebrate, setCelebrate] = useState(false);
  const problemPanel = useRef<PanelImperativeHandle | null>(null);
  const resultsPanel = useRef<PanelImperativeHandle | null>(null);
  const layoutTimer = useRef<number | undefined>(undefined);

  // In a mock interview only its languages may be used on its problems.
  useEffect(() => {
    if (inInterview && language && !allowed.includes(language)) void changeLanguage(allowed[0]);
  }, [inInterview, language]);

  // Mock interviews pick problems never opened (ADR 0020).
  useEffect(() => {
    void api.markOpened(problemId).catch(() => {});
  }, [problemId]);

  // Starting language: this problem's latest draft, else the last language used.
  useEffect(() => {
    let live = true;
    setLanguage(null);
    setCustoms([]);
    api
      .lastLanguage(problemId)
      .then((l) => live && setLanguage(l ?? lastLanguage))
      .catch(() => live && setLanguage(lastLanguage));
    return () => {
      live = false;
    };
    // Only when the problem changes; lastLanguage is the fallback at that moment.
  }, [problemId]);

  const signature = useMemo(() => (problem.data ? parseSignature(problem.data.signature) : null), [problem.data]);
  const customInputs = signature ? customs.map((c) => validateCustom(signature, c.values).input) : [];
  const customsValid = customInputs.every((i) => i !== null);
  const busy = judge.busy !== null;
  const ready = !busy && language !== null && session.code !== null;

  const run = useCallback(async () => {
    if (!ready || !language || session.code === null) return;
    if (!customsValid) {
      setTab("testcases");
      return;
    }
    setTab("result");
    await session.flush();
    await judge.run(language, session.code, customInputs.filter((i) => i !== null));
  }, [ready, language, session, customsValid, customInputs, judge]);

  // A custom input was refused (its constraints, keys, or the reference): show why where
  // the input is edited.
  useEffect(() => {
    if (isAppError(judge.error) && judge.error.code === "invalid_custom_input") setTab("testcases");
  }, [judge.error]);

  const submit = useCallback(async () => {
    if (!ready || !language || session.code === null) return;
    setTab("result");
    await session.flush();
    await judge.submit(language, session.code, { review: locked });
  }, [ready, language, session, judge, locked]);

  // A brief celebration when a submission is Accepted.
  useEffect(() => {
    if (judge.lastKind === "submit" && judge.report?.verdict === "accepted") {
      setCelebrate(true);
      const id = window.setTimeout(() => setCelebrate(false), CELEBRATE_MS);
      return () => window.clearTimeout(id);
    }
  }, [judge.report, judge.lastKind]);

  const toggleFocus = useCallback(() => setFocus(!focus), [focus, setFocus]);

  // Focus mode hides the problem pane too; leaving the workspace leaves focus mode.
  useEffect(() => {
    if (focus) problemPanel.current?.collapse();
    else problemPanel.current?.expand();
  }, [focus]);
  useEffect(() => () => setFocus(false), [setFocus]);

  // Shortcuts (the editor binds Run/Submit itself so they win over its own keymap).
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.defaultPrevented) return;
      const mod = e.ctrlKey || e.metaKey;
      if (mod && e.key === "Enter") {
        e.preventDefault();
        void submit();
      } else if (mod && e.key === "'") {
        e.preventDefault();
        void run();
      } else if (mod && e.shiftKey && e.key.toLowerCase() === "f") {
        e.preventDefault();
        toggleFocus();
      } else if (e.key === "Escape" && focus) {
        // Vim uses Escape inside the editor; only leave focus mode from elsewhere.
        const inEditor = (e.target as HTMLElement | null)?.closest(".cm-editor");
        if (!inEditor) setFocus(false);
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [run, submit, toggleFocus, focus, setFocus]);

  const changeLanguage = async (next: Language) => {
    if (next === language) return;
    await session.flush();
    setLanguage(next);
    setLastLanguage(next);
  };

  const saveLayout = (which: keyof Layouts, layout: Record<string, number>) => {
    window.clearTimeout(layoutTimer.current);
    layoutTimer.current = window.setTimeout(() => setLayouts({ ...layouts, [which]: layout }), 400);
  };

  const togglePanel = (ref: React.RefObject<PanelImperativeHandle | null>) => {
    const p = ref.current;
    if (!p) return;
    if (p.isCollapsed()) p.expand();
    else p.collapse();
  };

  if (problem.isPending) {
    return (
      <div className="space-y-3 p-6">
        <Skeleton className="h-8 w-64" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }
  if (problem.error) {
    return (
      <div className="p-6">
        <ErrorNotice error={problem.error} onRetry={() => void problem.refetch()} />
      </div>
    );
  }
  const p = problem.data;
  const toolchains = new Map((info.data?.toolchains ?? []).map((tc) => [tc.language, tc]));

  return (
    <div className="flex h-full min-h-0 flex-col" data-testid="workspace">
      <InstallDialog
        language={installFor}
        onOpenChange={(open) => !open && setInstallFor(null)}
        onInstalled={(l) => {
          if (l !== language) void changeLanguage(l);
        }}
      />
      <div className="flex flex-wrap items-center gap-2 border-b px-3 py-2">
        {!focus && (
          <Button asChild variant="ghost" size="icon" className="size-8">
            <Link to="/library" aria-label={t("workspace.back")}>
              <ArrowLeft aria-hidden />
            </Link>
          </Button>
        )}
        <h1 className="truncate text-base font-semibold">{p.title}</h1>
        <DifficultyLabel difficulty={p.difficulty} className="text-xs" />
        <div className="ml-auto flex items-center gap-2">
          {inInterview && <InterviewTimer compact />}
          {!inInterview && (solvedIn.data?.length ?? 0) > 0 && (
            <SolveInMenu
              solved={solvedIn.data ?? []}
              current={language}
              onPick={(from, to) => {
                void navigate({ search: (prev) => ({ ...prev, compare: from }), replace: true });
                if (to !== language) void changeLanguage(to);
              }}
            />
          )}
          <AddToListMenu problem={p.id} size="sm" />
          {reviewState.data ? (
            <span
              className="hidden items-center gap-1 text-xs text-muted-foreground lg:inline-flex"
              data-testid="review-queued"
            >
              <RotateCcw aria-hidden className="size-3.5" />
              {t("practice.review.inQueue", { date: formatDay(reviewState.data.due_on, i18n.language) })}
            </span>
          ) : (
            reviewState.data === null && (
              <Button
                variant="ghost"
                size="sm"
                className="gap-1.5"
                onClick={() => reviewActions.add.mutate(p.id)}
                title={t("practice.review.add")}
              >
                <RotateCcw aria-hidden />
                <span className="hidden xl:inline">{t("practice.review.add")}</span>
              </Button>
            )
          )}
          <span className="text-xs text-muted-foreground" aria-live="polite" data-testid="save-state">
            {session.save === "saving"
              ? t("workspace.save.saving")
              : session.save === "saved"
                ? t("workspace.save.saved")
                : session.save === "error"
                  ? t("workspace.save.error")
                  : ""}
          </span>
          <Select
            value={language ?? undefined}
            onValueChange={(v) => {
              const l = v as Language;
              const tc = toolchains.get(l);
              // Not installed: offer to install instead of switching to a dead end.
              if (tc !== undefined && !tc.present) setInstallFor(l);
              else void changeLanguage(l);
            }}
            disabled={busy}
          >
            <SelectTrigger size="sm" className="w-36" aria-label={t("workspace.language")}>
              <SelectValue placeholder={t("common.loading")} />
            </SelectTrigger>
            <SelectContent>
              {allowed.map((l) => {
                const tc = toolchains.get(l);
                const missing = tc !== undefined && !tc.present;
                return (
                  <SelectItem key={l} value={l}>
                    {t(`languages.${l}`)}
                    {missing && (
                      <span className="text-xs text-muted-foreground"> · {t("install.notInstalled")}</span>
                    )}
                  </SelectItem>
                );
              })}
            </SelectContent>
          </Select>
          <ShortcutButton
            label={t("workspace.run")}
            keys={KEYS.run}
            onClick={() => void run()}
            disabled={!ready}
            variant="outline"
            icon={judge.busy === "run" ? <Loader2 className="animate-spin" aria-hidden /> : <Play aria-hidden />}
          />
          <ShortcutButton
            label={t("workspace.submit")}
            keys={KEYS.submit}
            onClick={() => void submit()}
            disabled={!ready}
            icon={judge.busy === "submit" ? <Loader2 className="animate-spin" aria-hidden /> : <Send aria-hidden />}
          />
          <IconToggle
            label={t("workspace.toggleProblem")}
            onClick={() => togglePanel(problemPanel)}
            icon={<PanelLeft aria-hidden />}
          />
          <IconToggle
            label={t("workspace.toggleResults")}
            onClick={() => togglePanel(resultsPanel)}
            icon={<PanelBottom aria-hidden />}
          />
          <IconToggle
            label={`${focus ? t("workspace.focusExit") : t("workspace.focusEnter")} (${KEYS.focus})`}
            onClick={toggleFocus}
            pressed={focus}
            icon={focus ? <Minimize2 aria-hidden /> : <Maximize2 aria-hidden />}
          />
        </div>
      </div>

      {reviewMode && (
        <div
          role="status"
          data-testid="review-banner"
          className="flex items-center gap-2 border-b bg-muted/40 px-3 py-1.5 text-sm"
        >
          <RotateCcw aria-hidden className="size-4 shrink-0" />
          {reviewPassed
            ? reviewOutcome === "graduated"
              ? t("practice.review.graduated")
              : t("practice.review.passed", {
                  date: formatDay(
                    judge.practice?.review?.outcome === "scheduled" ? judge.practice.review.due_on : "",
                    i18n.language,
                  ),
                })
            : t("practice.review.banner")}
        </div>
      )}
      <ResizablePanelGroup
        orientation="horizontal"
        className="min-h-0 flex-1"
        defaultLayout={layouts.main}
        onLayoutChanged={(l) => saveLayout("main", l)}
      >
        <ResizablePanel id="problem" panelRef={problemPanel} defaultSize="40" minSize="20" collapsible collapsedSize="0">
          <section aria-label={t("workspace.problemPane")} className="h-full min-h-0">
            <ProblemPane
              key={p.id}
              problem={p}
              currentCode={session.code}
              currentLanguage={language}
              locks={
                inInterview
                  ? { tabs: ["hints", "editorial", "submissions", "history"], reason: "interview" }
                  : locked
                    ? { tabs: ["submissions", "history"], reason: "review" }
                    : null
              }
              onRestored={(l) => {
                session.reload(l);
                if (l !== language) setLanguage(l);
              }}
            />
          </section>
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel id="work" minSize="30">
          <ResizablePanelGroup
            orientation="vertical"
            defaultLayout={layouts.right}
            onLayoutChanged={(l) => saveLayout("right", l)}
          >
            <ResizablePanel id="editor" defaultSize="62" minSize="20">
              {session.loadError ? (
                <div className="p-4">
                  <ErrorNotice error={session.loadError} />
                </div>
              ) : !session.doc ? (
                <Skeleton className="m-4 h-40" />
              ) : (
                <div className="flex h-full min-h-0 flex-col">
                  {toolchains.get(session.doc.lang)?.present === false && (
                    <div
                      role="status"
                      data-testid="language-missing"
                      className="flex items-center gap-3 border-b bg-muted/40 px-3 py-2 text-sm"
                    >
                      <span className="min-w-0 flex-1">
                        {t("install.missingTitle", { language: t(`languages.${session.doc.lang}`) })}
                      </span>
                      <Button size="sm" className="h-7 gap-1.5" onClick={() => setInstallFor(session.doc!.lang)}>
                        <Download aria-hidden className="size-3.5" />
                        {t("install.install")}
                      </Button>
                    </div>
                  )}
                  <div className="flex min-h-0 flex-1">
                    {compare && !inInterview && (
                      <CompareView
                        problem={p.id}
                        language={compare}
                        onClose={() =>
                          void navigate({ search: (prev) => ({ ...prev, compare: undefined }), replace: true })
                        }
                      />
                    )}
                    <div className="min-h-0 min-w-0 flex-1">
                    <CodeEditor
                      docKey={session.docKey}
                      value={session.doc.text}
                      lang={session.doc.lang}
                      keymap={keymap}
                      fontSize={fontSize}
                      ligatures={ligatures}
                      lspExtension={basicCompletion ? phpBasicCompletion : server.extension}
                      ariaLabel={t("workspace.editorLabel", { language: t(`languages.${session.doc.lang}`) })}
                      onChange={(text) => {
                        session.onChange(text);
                        server.wake();
                      }}
                      onRun={() => void run()}
                      onSubmit={() => void submit()}
                      onStatus={setEditorStatus}
                      onView={(v) => (editorView.current = v)}
                    />
                    </div>
                  </div>
                  <EditorStatusBar
                    language={session.doc.lang}
                    server={server.status}
                    basicCompletion={basicCompletion}
                    onInstall={() => setInstallFor(session.doc!.lang)}
                    status={editorStatus}
                    onFormat={() => {
                      if (editorView.current) formatDocument(editorView.current);
                    }}
                  />
                </div>
              )}
            </ResizablePanel>
            <ResizableHandle withHandle />
            <ResizablePanel id="results" panelRef={resultsPanel} defaultSize="38" minSize="12" collapsible collapsedSize="0">
              <section aria-label={t("workspace.resultsPane")} className="h-full min-h-0 border-t">
                <ResultsPane
                  problem={p}
                  signature={signature}
                  tab={tab}
                  onTab={setTab}
                  customs={customs}
                  validatorNeedsPython={p.has_validator && toolchains.get("python")?.present === false}
                  onInstallPython={() => setInstallFor("python")}
                  onCustoms={setCustoms}
                  busy={busy}
                  phase={judge.phase}
                  report={judge.report}
                  error={judge.error}
                  celebrate={celebrate}
                  practice={judge.lastKind === "submit" ? judge.practice : null}
                />
              </section>
            </ResizablePanel>
          </ResizablePanelGroup>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}

function ShortcutButton({
  label,
  keys,
  icon,
  onClick,
  disabled,
  variant = "default",
}: {
  label: string;
  keys: string;
  icon: React.ReactNode;
  onClick: () => void;
  disabled: boolean;
  variant?: "default" | "outline";
}) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button size="sm" variant={variant} onClick={onClick} disabled={disabled} aria-keyshortcuts={keys}>
          {icon}
          {label}
        </Button>
      </TooltipTrigger>
      <TooltipContent>{keys}</TooltipContent>
    </Tooltip>
  );
}

function IconToggle({
  label,
  icon,
  onClick,
  pressed,
}: {
  label: string;
  icon: React.ReactNode;
  onClick: () => void;
  pressed?: boolean;
}) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button size="icon" variant="ghost" className="size-8" onClick={onClick} aria-label={label} aria-pressed={pressed}>
          {icon}
        </Button>
      </TooltipTrigger>
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  );
}
