// The UI's only door to the backend (ADR 0012). Wraps the generated tauri-specta
// commands: unwraps `{status, data | error}` and throws the typed AppError. Components
// use the hooks in `queries.ts` (or call `api` for actions); tests mock this module.
import { Channel } from "@tauri-apps/api/core";
import { recordError } from "@/lib/diagnostics";
import {
  commands,
  type CustomCheck,
  type StressEvent,
  type StressResult,
  type ReadingAnswer,
  type ReadingAnswerRow,
  type ReadingLink,
  type ReadingResult,
  type ReadingSetSummary,
  type ReadingSetView,
  type AppError,
  type AppInfo,
  type Diagnostics,
  type ShrinkEvent,
  type ShrinkResult,
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
  type KataAttemptRow,
  type KataOptions,
  type KataOverview,
  type KataStart,
  type KataView,
  type SearchResults,
  type SnippetInput,
  type SnippetRow,
  type JournalOverview,
  type MistakeView,
  type ReflectionRow,
  type PlanIcsLabels,
  type PlanSetup,
  type PlanView,
  type RatingsView,
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
  type LearnPath,
  type PatternTopics,
  type TopicReview,
  LearnSubject,
  type LearnTopicSummary,
  type LearnTopicView,
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
  if (r.status === "error") {
    // The code only, for Copy diagnostics: messages can name files or users.
    recordError(r.error.code);
    throw r.error;
  }
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
  diagnostics: (): Promise<Diagnostics> => call(commands.diagnostics()),
  thirdPartyNotices: (): Promise<string | null> => call(commands.thirdPartyNotices()),
  /** Settings → Appearance: the default window size, centred (window memory, ui-polish). */
  resetWindow: (): Promise<null> => call(commands.resetWindow()),
  activity: (sinceMs: number): Promise<ActivityRow[]> => call(commands.activity(sinceMs)),
  solvedBreakdown: (lang: UiLang): Promise<SolvedBreakdown> => call(commands.solvedBreakdown(lang)),
  reviewSummary: (today: string): Promise<ReviewSummary> => call(commands.reviewSummary(today)),
  listProblems: (filter: ProblemFilter): Promise<ProblemSummary[]> =>
    call(commands.listProblems(filter)),
  getProblem: (id: string, lang: UiLang): Promise<ProblemView> =>
    call(commands.getProblem(id, lang)),
  // Learn (ADR 0029)
  learnCatalog: (lang: UiLang): Promise<LearnSubject[]> => call(commands.learnCatalog(lang)),
  learnPaths: (lang: UiLang): Promise<LearnPath[]> => call(commands.learnPaths(lang)),
  placementProblems: (lang: UiLang): Promise<ProblemSummary[]> => call(commands.placementProblems(lang)),
  finishOnboarding: (placement: string[] | null, lang: UiLang): Promise<ProblemSummary | null> =>
    call(commands.finishOnboarding(placement, lang)),
  learnByPattern: (lang: UiLang): Promise<PatternTopics[]> => call(commands.learnByPattern(lang)),
  topicToReview: (viewed: string[], lang: UiLang): Promise<TopicReview | null> =>
    call(commands.topicToReview(viewed, lang)),
  learnTopic: (id: string, lang: UiLang): Promise<LearnTopicView> =>
    call(commands.learnTopic(id, lang)),
  searchLearn: (query: string, lang: UiLang): Promise<LearnTopicSummary[]> =>
    call(commands.searchLearn(query, lang)),
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
  shrink: (
    problem: string,
    language: Language,
    code: string,
    input: CustomInput,
    onEvent: (e: ShrinkEvent) => void,
  ): Promise<ShrinkResult> => {
    const ch = new Channel<ShrinkEvent>();
    ch.onmessage = onEvent;
    return call(commands.shrink(problem, language, code, input, ch));
  },
  cancelShrink: (): Promise<null> => call(commands.cancelShrink()),
  checkCustom: (problem: string, inputs: CustomInput[]): Promise<CustomCheck> =>
    call(commands.checkCustom(problem, inputs)),
  // The stress lab (expansion M8, ADR 0036)
  stressTest: (
    problem: string,
    language: Language,
    code: string,
    brute: string,
    onEvent: (e: StressEvent) => void,
  ): Promise<StressResult> => {
    const ch = new Channel<StressEvent>();
    ch.onmessage = onEvent;
    return call(commands.stressTest(problem, language, code, brute, ch));
  },
  cancelStress: (): Promise<null> => call(commands.cancelStress()),
  stressBrute: (problem: string, language: Language): Promise<string | null> =>
    call(commands.stressBrute(problem, language)),
  saveStressBrute: (problem: string, language: Language, code: string): Promise<null> =>
    call(commands.saveStressBrute(problem, language, code)),
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

  // Snippets and search (expansion M4, ADR 0034)
  // Reading exercises (expansion M7, ADR 0035)
  readingSets: (lang: UiLang): Promise<ReadingSetSummary[]> => call(commands.readingSets(lang)),
  readingSet: (id: string, lang: UiLang): Promise<ReadingSetView> => call(commands.readingSet(id, lang)),
  readingAnswers: (): Promise<ReadingAnswerRow[]> => call(commands.readingAnswers()),
  readingLinks: (topic: string, lang: UiLang): Promise<ReadingLink[]> =>
    call(commands.readingLinks(topic, lang)),
  answerReading: (item: string, answer: ReadingAnswer, lang: UiLang): Promise<ReadingResult> =>
    call(commands.answerReading(item, answer, lang)),
  snippets: (): Promise<SnippetRow[]> => call(commands.snippets()),
  saveSnippet: (snippet: SnippetInput): Promise<SnippetRow> => call(commands.saveSnippet(snippet)),
  deleteSnippet: (uid: string): Promise<null> => call(commands.deleteSnippet(uid)),
  searchAll: (query: string, lang: UiLang): Promise<SearchResults> => call(commands.searchAll(query, lang)),

  // Mistake journal and reflections (expansion M3, ADR 0034)
  journalOverview: (lang: UiLang): Promise<JournalOverview> => call(commands.journalOverview(lang)),
  submissionMistake: (submission: number): Promise<MistakeView | null> =>
    call(commands.submissionMistake(submission)),
  setSubmissionMistake: (submission: number, tags: string[], note: string): Promise<null> =>
    call(commands.setSubmissionMistake(submission, tags, note)),
  tagShrunk: (submission: number): Promise<null> => call(commands.tagShrunk(submission)),
  reflection: (problem: string): Promise<ReflectionRow | null> => call(commands.reflection(problem)),
  setReflection: (problem: string, why: string, time: string | null, space: string | null): Promise<null> =>
    call(commands.setReflection(problem, why, time, space)),

  // Skill rating and study plans (expansion M2, ADR 0033)
  ratings: (): Promise<RatingsView> => call(commands.ratings()),
  planView: (lang: UiLang): Promise<PlanView | null> => call(commands.planView(lang)),
  createPlan: (setup: PlanSetup, lang: UiLang): Promise<PlanView> => call(commands.createPlan(setup, lang)),
  setPlanItem: (plan: number, position: number, status: "pending" | "done" | "skipped"): Promise<null> =>
    call(commands.setPlanItem(plan, position, status)),
  reschedulePlanItem: (plan: number, position: number): Promise<null> =>
    call(commands.reschedulePlanItem(plan, position)),
  archivePlan: (plan: number): Promise<null> => call(commands.archivePlan(plan)),
  /** A Save dialog (opened by Rust), then the plan as an .ics file; null when cancelled. */
  exportPlanIcs: (labels: PlanIcsLabels, lang: UiLang): Promise<string | null> =>
    call(commands.exportPlanIcs(labels, lang)),

  // Kata drills (expansion M1, ADR 0033)
  kataOptions: (today: string, lang: UiLang): Promise<KataOptions> => call(commands.kataOptions(today, lang)),
  startKata: (start: KataStart, lang: UiLang): Promise<KataView> => call(commands.startKata(start, lang)),
  kataState: (lang: UiLang): Promise<KataView | null> => call(commands.kataState(lang)),
  kataOpen: (problem: string): Promise<KataAttemptRow> => call(commands.kataOpen(problem)),
  endKata: (lang: UiLang): Promise<KataView | null> => call(commands.endKata(lang)),
  kataOverview: (today: string, lang: UiLang): Promise<KataOverview> => call(commands.kataOverview(today, lang)),
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
