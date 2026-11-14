// React Query hooks for the practice features (ADR 0019): daily problem, reviews, the
// pattern map and personal lists. Logic lives in kb-app; these only fetch and refresh.
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { JsonValue } from "@/bindings";
import { api } from "@/lib/api";
import { usePrefs } from "@/lib/prefs";
import { useSetting } from "@/lib/settings";
import { useToday } from "@/lib/today";

export const practiceKeys = {
  all: ["practice"] as const,
  daily: (today: string, lang: string) => ["practice", "daily", today, lang] as const,
  reviews: (today: string, lang: string) => ["practice", "reviews", today, lang] as const,
  review: (problem: string, today: string) => ["practice", "review", problem, today] as const,
  streak: (today: string) => ["practice", "streak", today] as const,
  patterns: (lang: string) => ["practice", "patterns", lang] as const,
  lists: (lang: string) => ["practice", "lists", lang] as const,
  listsWith: (problem: string) => ["practice", "lists-with", problem] as const,
};

const parseBool = (v: JsonValue | null): boolean | undefined => (typeof v === "boolean" ? v : undefined);

/** Settings → Practice switches (both default on). */
export function usePracticePrefs() {
  const [review, setReview] = useSetting<boolean>("practice.review", true, parseBool, { mirror: true });
  const [daily, setDaily] = useSetting<boolean>("practice.daily", true, parseBool, { mirror: true });
  const [learnReview, setLearnReview] = useSetting<boolean>("practice.learn_review", true, parseBool, {
    mirror: true,
  });
  return { review, setReview, daily, setDaily, learnReview, setLearnReview };
}

export function useDailyProblem(enabled = true) {
  const { lang } = usePrefs();
  const today = useToday();
  return useQuery({
    queryKey: practiceKeys.daily(today, lang),
    queryFn: () => api.dailyProblem(today, lang),
    enabled,
  });
}

export function useStreak() {
  const today = useToday();
  return useQuery({ queryKey: practiceKeys.streak(today), queryFn: () => api.streak(today) });
}

export function useReviewsDue(enabled = true) {
  const { lang } = usePrefs();
  const today = useToday();
  return useQuery({
    queryKey: practiceKeys.reviews(today, lang),
    queryFn: () => api.reviewsDue(today, lang),
    enabled,
  });
}

export function useReviewState(problem: string) {
  const today = useToday();
  return useQuery({
    queryKey: practiceKeys.review(problem, today),
    queryFn: () => api.reviewState(problem, today),
  });
}

export function usePatterns() {
  const { lang } = usePrefs();
  return useQuery({ queryKey: practiceKeys.patterns(lang), queryFn: () => api.patterns(lang) });
}

export function useLists() {
  const { lang } = usePrefs();
  return useQuery({ queryKey: practiceKeys.lists(lang), queryFn: () => api.lists(lang) });
}

export function useListsWith(problem: string) {
  return useQuery({ queryKey: practiceKeys.listsWith(problem), queryFn: () => api.listsWith(problem) });
}

/** A practice change: run it, then refresh every practice query (and the library). */
export function usePracticeMutation<A>(fn: (arg: A) => Promise<unknown>) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: fn,
    onSettled: () => {
      void queryClient.invalidateQueries({ queryKey: practiceKeys.all });
    },
  });
}

/** Review actions for Home and the workspace. */
export function useReviewActions() {
  const today = useToday();
  return {
    snooze: usePracticeMutation((problem: string) => api.reviewSnooze(problem, today)),
    remove: usePracticeMutation((problem: string) => api.reviewRemove(problem)),
    add: usePracticeMutation((problem: string) => api.reviewAdd(problem, today)),
  };
}

/** List actions for the Lists view and the "Add to list" menus. */
export function useListActions() {
  return {
    create: usePracticeMutation((name: string) => api.createList(name)),
    rename: usePracticeMutation(({ id, name }: { id: number; name: string }) => api.renameList(id, name)),
    remove: usePracticeMutation((id: number) => api.deleteList(id)),
    add: usePracticeMutation(({ id, problem }: { id: number; problem: string }) => api.addToList(id, problem)),
    drop: usePracticeMutation(({ id, problem }: { id: number; problem: string }) =>
      api.removeFromList(id, problem),
    ),
    reorder: usePracticeMutation(({ id, problems }: { id: number; problems: string[] }) =>
      api.reorderList(id, problems),
    ),
  };
}
