// React Query hooks over `api` (ADR 0012). Query keys include the UI language where the
// result depends on it, so switching language refetches titles and texts.
import { useQuery } from "@tanstack/react-query";
import type { ProblemFilter } from "@/bindings";
import { api } from "@/lib/api";
import { usePrefs } from "@/lib/prefs";

export const EMPTY_FILTER: ProblemFilter = {
  difficulty: null,
  tag: null,
  pattern: null,
  status: null,
  query: null,
  lang: null,
};

export const queryKeys = {
  appInfo: ["app_info"] as const,
  languages: ["languages"] as const,
  problems: (filter: ProblemFilter) => ["problems", filter] as const,
  problem: (id: string, lang: string) => ["problem", id, lang] as const,
  continueItem: (lang: string) => ["continue", lang] as const,
};

export function useAppInfo() {
  return useQuery({ queryKey: queryKeys.appInfo, queryFn: api.appInfo });
}

/** Install state of each language, with sizes and free space (ADR 0015). */
export function useLanguages() {
  return useQuery({ queryKey: queryKeys.languages, queryFn: api.languages });
}

/** Problems matching `filter`, titles in the UI language. */
export function useProblems(filter: Partial<ProblemFilter> = {}) {
  const { lang } = usePrefs();
  const full: ProblemFilter = { ...EMPTY_FILTER, ...filter, lang };
  return useQuery({
    queryKey: queryKeys.problems(full),
    queryFn: () => api.listProblems(full),
    placeholderData: (previous) => previous,
  });
}

export function useProblem(id: string) {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: queryKeys.problem(id, lang),
    queryFn: () => api.getProblem(id, lang),
  });
}

export function useContinue() {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: queryKeys.continueItem(lang),
    queryFn: () => api.continueItem(lang),
  });
}
