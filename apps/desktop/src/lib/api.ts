// The UI's only door to the backend (ADR 0012). Wraps the generated tauri-specta
// commands: unwraps `{status, data | error}` and throws the typed AppError. Components
// use the hooks in `queries.ts`; tests mock this module.
import {
  commands,
  type AppError,
  type AppInfo,
  type ContinueItem,
  type JsonValue,
  type ProblemFilter,
  type ProblemSummary,
  type ProblemView,
  type Result,
  type UiLang,
} from "@/bindings";

export async function call<T>(p: Promise<Result<T, AppError>>): Promise<T> {
  const r = await p;
  if (r.status === "error") throw r.error;
  return r.data;
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
};

export type Api = typeof api;
