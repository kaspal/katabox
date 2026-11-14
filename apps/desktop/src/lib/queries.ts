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
  mode: null,
  lang: null,
};

export const queryKeys = {
  appInfo: ["app_info"] as const,
  languages: ["languages"] as const,
  problems: (filter: ProblemFilter) => ["problems", filter] as const,
  problem: (id: string, lang: string) => ["problem", id, lang] as const,
  continueItem: (lang: string) => ["continue", lang] as const,
  calibration: ["calibration"] as const,
  learnCatalog: (lang: string) => ["learn_catalog", lang] as const,
  learnTopic: (id: string, lang: string) => ["learn_topic", id, lang] as const,
  learnPaths: (lang: string) => ["learn_paths", lang] as const,
  learnByPattern: (lang: string) => ["learn_by_pattern", lang] as const,
  topicToReview: (viewed: string[], lang: string) => ["topic_to_review", viewed, lang] as const,
  learnSearch: (q: string, lang: string) => ["learn_search", q, lang] as const,
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

/** This machine's speed factor (ADR 0017); `null` until it is measured. */
export function useCalibration() {
  return useQuery({ queryKey: queryKeys.calibration, queryFn: api.calibration });
}

export function useContinue() {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: queryKeys.continueItem(lang),
    queryFn: () => api.continueItem(lang),
  });
}

/** Every Learn subject with its topics (ADR 0029); content only, so it never goes stale. */
export function useLearnCatalog() {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: queryKeys.learnCatalog(lang),
    queryFn: () => api.learnCatalog(lang),
    staleTime: Infinity,
  });
}

/** Learn topics per practice pattern (content only). */
export function useLearnByPattern() {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: queryKeys.learnByPattern(lang),
    queryFn: () => api.learnByPattern(lang),
    staleTime: Infinity,
  });
}

/** Home's "Review a topic": depends on solves, so it refetches like other progress. */
export function useTopicToReview(viewed: string[], enabled = true) {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: queryKeys.topicToReview(viewed, lang),
    queryFn: () => api.topicToReview(viewed, lang),
    enabled,
  });
}

/** The learning paths on Learn home (Phase 6); content only. */
export function useLearnPaths() {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: queryKeys.learnPaths(lang),
    queryFn: () => api.learnPaths(lang),
    staleTime: Infinity,
  });
}

export function useLearnTopic(id: string) {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: queryKeys.learnTopic(id, lang),
    queryFn: () => api.learnTopic(id, lang),
    staleTime: Infinity,
  });
}

/** Topics matching `q` (empty text: no query). */
export function useLearnSearch(q: string) {
  const { lang } = usePrefs();
  const text = q.trim();
  return useQuery({
    queryKey: queryKeys.learnSearch(text, lang),
    queryFn: () => api.searchLearn(text, lang),
    enabled: text !== "",
    staleTime: Infinity,
    placeholderData: (previous) => previous,
  });
}
