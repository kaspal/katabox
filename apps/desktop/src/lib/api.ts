// The UI's only door to the backend (ADR 0012). Wraps the generated tauri-specta
// commands: unwraps `{status, data | error}` and throws the typed AppError. Components
// use the hooks in `queries.ts` (or call `api` for actions); tests mock this module.
import { Channel } from "@tauri-apps/api/core";
import {
  commands,
  type AppError,
  type AppInfo,
  type AcceptedSolution,
  type DebugAction,
  type DebugInspect,
  type DebugStart,
  type DebugView,
  type ProfileView,
  type EvalResult,
  type ActivityRow,
  type ReviewSummary,
  type SolvedBreakdown,
  type CalibrationInfo,
  type ImportPreview,
  type ImportResult,
  type InterviewResult,
  type InterviewSetup,
  type InterviewView,
  type LanguageCount,
  type TransferCounts,
  type DailyProblem,
  type ListView,
  type PatternProgress,
  type ReviewItem,
  type ReviewState,
  type SubmitOptions,
  type ContinueItem,
  type Draft,
  type EditorialState,
  type InstallEvent,
  type JsonValue,
  type JudgeEvent,
  type Language,
  type LanguagesInfo,
  type LspConnection,
  type LspEvent,
  type ProblemFilter,
  type ProblemSummary,
  type ProblemView,
  type Report,
  type Result,
  type Snapshot,
  type SubmissionSummary,
  type SubmissionView,
  type SubmitResult,
  type UiLang,
} from "@/bindings";

/** A user-written test input: argument values keyed by parameter name. */
export type CustomInput = Partial<{ [key in string]: JsonValue }>;

export async function call<T>(p: Promise<Result<T, AppError>>): Promise<T> {
  const r = await p;
  if (r.status === "error") throw r.error;
  return r.data;
}

/** A Tauri channel delivering judge progress to `onEvent`. */
function progress(onEvent: (e: JudgeEvent) => void): Channel<JudgeEvent> {
  const ch = new Channel<JudgeEvent>();
  ch.onmessage = onEvent;
  return ch;
}

export const api = {
  appInfo: (): Promise<AppInfo> => call(commands.appInfo()),
  thirdPartyNotices: (): Promise<string | null> => call(commands.thirdPartyNotices()),
  activity: (sinceMs: number): Promise<ActivityRow[]> => call(commands.activity(sinceMs)),
  solvedBreakdown: (lang: UiLang): Promise<SolvedBreakdown> => call(commands.solvedBreakdown(lang)),
  reviewSummary: (today: string): Promise<ReviewSummary> => call(commands.reviewSummary(today)),
  listProblems: (filter: ProblemFilter): Promise<ProblemSummary[]> =>
    call(commands.listProblems(filter)),
  getProblem: (id: string, lang: UiLang): Promise<ProblemView> =>
    call(commands.getProblem(id, lang)),
  continueItem: (lang: UiLang): Promise<ContinueItem | null> =>
    call(commands.continueItem(lang)),
  getSetting: (key: string): Promise<JsonValue | null> => call(commands.getSetting(key)),
  setSetting: (key: string, value: JsonValue): Promise<null> =>
    call(commands.setSetting(key, value)),
  // This machine's speed factor for time limits (ADR 0017)
  calibration: (): Promise<CalibrationInfo | null> => call(commands.calibration()),
  recalibrate: (): Promise<CalibrationInfo> => call(commands.recalibrate()),

  // Workspace
  editorial: (problem: string, lang: UiLang): Promise<EditorialState> =>
    call(commands.editorial(problem, lang)),
  lastLanguage: (problem: string): Promise<Language | null> =>
    call(commands.lastLanguage(problem)),
  getStub: (problem: string, language: Language): Promise<string> =>
    call(commands.getStub(problem, language)),
  loadDraft: (problem: string, language: Language): Promise<Draft | null> =>
    call(commands.loadDraft(problem, language)),
  saveDraft: (problem: string, language: Language, code: string): Promise<null> =>
    call(commands.saveDraft(problem, language, code)),
  run: (
    problem: string,
    language: Language,
    code: string,
    custom: CustomInput[],
    onEvent: (e: JudgeEvent) => void,
  ): Promise<Report> => call(commands.run(problem, language, code, custom, progress(onEvent))),
  submit: (
    problem: string,
    language: Language,
    code: string,
    opts: SubmitOptions,
    onEvent: (e: JudgeEvent) => void,
  ): Promise<SubmitResult> => call(commands.submit(problem, language, code, opts, progress(onEvent))),
  /** Measure the solution at growing sizes and estimate its complexity (ADR 0024). */
  profileSolution: (problem: string, language: Language, code: string): Promise<ProfileView> =>
    call(commands.profileSolution(problem, language, code)),
  listSubmissions: (problem: string): Promise<SubmissionSummary[]> =>
    call(commands.listSubmissions(problem)),
  getSubmission: (id: number): Promise<SubmissionView> => call(commands.getSubmission(id)),
  listSnapshots: (problem: string): Promise<Snapshot[]> => call(commands.listSnapshots(problem)),
  restoreSnapshot: (id: number): Promise<Snapshot> => call(commands.restoreSnapshot(id)),
  getNote: (problem: string): Promise<string | null> => call(commands.getNote(problem)),
  setNote: (problem: string, body: string): Promise<null> => call(commands.setNote(problem, body)),
  unlockHint: (problem: string, tier: number): Promise<number[]> =>
    call(commands.unlockHint(problem, tier)),
  unlockedHints: (problem: string): Promise<number[]> => call(commands.unlockedHints(problem)),

  // Practice (ADR 0019): `today` is the local date, YYYY-MM-DD (see lib/today.ts)
  patterns: (lang: UiLang): Promise<PatternProgress[]> => call(commands.patterns(lang)),
  nextProblem: (problem: string, lang: UiLang): Promise<ProblemSummary | null> =>
    call(commands.nextProblem(problem, lang)),
  dailyProblem: (today: string, lang: UiLang): Promise<DailyProblem | null> =>
    call(commands.dailyProblem(today, lang)),
  streak: (today: string): Promise<number> => call(commands.streak(today)),
  reviewsDue: (today: string, lang: UiLang): Promise<ReviewItem[]> => call(commands.reviewsDue(today, lang)),
  reviewState: (problem: string, today: string): Promise<ReviewState | null> =>
    call(commands.reviewState(problem, today)),
  reviewSnooze: (problem: string, today: string): Promise<null> => call(commands.reviewSnooze(problem, today)),
  reviewRemove: (problem: string): Promise<null> => call(commands.reviewRemove(problem)),
  reviewAdd: (problem: string, today: string): Promise<null> => call(commands.reviewAdd(problem, today)),
  lists: (lang: UiLang): Promise<ListView[]> => call(commands.lists(lang)),
  createList: (name: string): Promise<number> => call(commands.createList(name)),
  renameList: (id: number, name: string): Promise<null> => call(commands.renameList(id, name)),
  deleteList: (id: number): Promise<null> => call(commands.deleteList(id)),
  addToList: (id: number, problem: string): Promise<null> => call(commands.addToList(id, problem)),
  removeFromList: (id: number, problem: string): Promise<null> => call(commands.removeFromList(id, problem)),
  reorderList: (id: number, problems: string[]): Promise<null> => call(commands.reorderList(id, problems)),
  listsWith: (problem: string): Promise<number[]> => call(commands.listsWith(problem)),

  // Practice modes (ADR 0020)
  markOpened: (problem: string): Promise<null> => call(commands.markOpened(problem)),
  startInterview: (setup: InterviewSetup, lang: UiLang): Promise<InterviewView> =>
    call(commands.startInterview(setup, lang)),
  interviewActive: (lang: UiLang): Promise<InterviewView | null> => call(commands.interviewActive(lang)),
  endInterview: (lang: UiLang): Promise<InterviewResult> => call(commands.endInterview(lang)),
  interviews: (lang: UiLang): Promise<InterviewResult[]> => call(commands.interviews(lang)),
  acceptedSolution: (problem: string, language: Language): Promise<AcceptedSolution | null> =>
    call(commands.acceptedSolution(problem, language)),
  solvedLanguages: (problem: string): Promise<Language[]> => call(commands.solvedLanguages(problem)),
  solvedByLanguage: (): Promise<LanguageCount[]> => call(commands.solvedByLanguage()),
  /** Opens a native Save dialog (in Rust); null when cancelled. */
  exportProgress: (): Promise<TransferCounts | null> => call(commands.exportProgress()),
  /** Opens a native Open dialog (in Rust), reads and checks the file; null when cancelled. */
  importPreview: (): Promise<ImportPreview | null> => call(commands.importPreview()),
  importApply: (id: number): Promise<ImportResult> => call(commands.importApply(id)),
  importCancel: (id: number): Promise<null> => call(commands.importCancel(id)),

  // Language servers (ADR 0014)
  lspConnect: (
    problem: string,
    language: Language,
    text: string,
    onEvent: (e: LspEvent) => void,
  ): Promise<LspConnection> => {
    const ch = new Channel<LspEvent>();
    ch.onmessage = onEvent;
    return call(commands.lspConnect(problem, language, text, ch));
  },
  lspSend: (language: Language, session: number, message: string): Promise<null> =>
    call(commands.lspSend(language, session, message)),
  lspDisconnect: (language: Language, session: number): Promise<null> =>
    call(commands.lspDisconnect(language, session)),

  // Languages: runtimes and language servers (ADR 0015)
  languages: (): Promise<LanguagesInfo> => call(commands.languages()),
  installLanguages: (languages: Language[], onEvent: (e: InstallEvent) => void): Promise<null> => {
    const ch = new Channel<InstallEvent>();
    ch.onmessage = onEvent;
    return call(commands.installLanguages(languages, ch));
  },
  cancelInstall: (): Promise<null> => call(commands.cancelInstall()),
  uninstallLanguage: (language: Language): Promise<null> => call(commands.uninstallLanguage(language)),

  // Debugger (ADR 0022): replay runs in the sandbox, one session at a time
  debugLanguages: (): Promise<Language[]> => commands.debugLanguages(),
  debugStart: (start: DebugStart): Promise<DebugView> => call(commands.debugStart(start)),
  debugAction: (session: number, action: DebugAction): Promise<DebugView> =>
    call(commands.debugAction(session, action)),
  debugSetBreakpoints: (session: number, lines: number[]): Promise<null> =>
    call(commands.debugSetBreakpoints(session, lines)),
  debugInspect: (session: number, inspect: DebugInspect): Promise<DebugView> =>
    call(commands.debugInspect(session, inspect)),
  debugEvaluate: (session: number, frame: number, expr: string): Promise<EvalResult> =>
    call(commands.debugEvaluate(session, frame, expr)),
  debugEnd: (session: number): Promise<null> => call(commands.debugEnd(session)),
};

export type Api = typeof api;
