// React Query hooks for skill ratings and study plans (expansion M2, ADR 0033).
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { usePrefs } from "@/lib/prefs";

export const planKeys = {
  all: ["plan"] as const,
  plan: (lang: string) => ["plan", "active", lang] as const,
  ratings: ["ratings"] as const,
};

/** The active study plan (null when none). */
export function usePlan() {
  const { lang } = usePrefs();
  return useQuery({ queryKey: planKeys.plan(lang), queryFn: () => api.planView(lang) });
}

export function useInvalidatePlan() {
  const queryClient = useQueryClient();
  return () => void queryClient.invalidateQueries({ queryKey: planKeys.all });
}

/** Ratings per pattern and overall (derived from the history by kb-app). */
export function useRatings() {
  return useQuery({ queryKey: planKeys.ratings, queryFn: () => api.ratings() });
}
