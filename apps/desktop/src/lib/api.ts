// The UI's only door to the backend (ADR 0012). Wraps the generated tauri-specta
// commands: unwraps `{status, data | error}` and throws the typed AppError. Components
// use the hooks in `queries.ts` (or call `api` for actions); tests mock this module.
import { Channel } from "@tauri-apps/api/core";
import {
  commands,
  type AppError,
  type AppInfo,
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
  listProblems: (filter: ProblemFilter): Promise<ProblemSummary[]> =>
    call(commands.listProblems(filter)),
  getProblem: (id: string, lang: UiLang): Promise<ProblemView> =>
    call(commands.getProblem(id, lang)),
  continueItem: (lang: UiLang): Promise<ContinueItem | null> =>
    call(commands.continueItem(lang)),
  getSetting: (key: string): Promise<JsonValue | null> => call(commands.getSetting(key)),
  setSetting: (key: string, value: JsonValue): Promise<null> =>
    call(commands.setSetting(key, value)),

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
    onEvent: (e: JudgeEvent) => void,
  ): Promise<SubmitResult> => call(commands.submit(problem, language, code, progress(onEvent))),
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
};

export type Api = typeof api;
