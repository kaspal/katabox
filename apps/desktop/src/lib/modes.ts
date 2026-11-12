// React Query hooks for the practice modes (ADR 0020): mock interviews, cross-language
// practice. Rules and locks live in kb-app.
import { useQuery, useQueryClient } from "@tanstack/react-query";
import type { Language } from "@/bindings";
import { api } from "@/lib/api";
import { usePrefs } from "@/lib/prefs";

export const modeKeys = {
  interview: (lang: string) => ["interview", "active", lang] as const,
  interviews: (lang: string) => ["interview", "history", lang] as const,
  solvedLanguages: (problem: string) => ["cross-lang", "solved", problem] as const,
  accepted: (problem: string, language: string) => ["cross-lang", "accepted", problem, language] as const,
  byLanguage: ["cross-lang", "by-language"] as const,
};

/** The running mock interview (null when none). Rechecked every 30 s, so a session that
 * timed out elsewhere disappears. */
export function useInterviewActive() {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: modeKeys.interview(lang),
    queryFn: () => api.interviewActive(lang),
    refetchInterval: 30_000,
  });
}

export function useInterviews() {
  const { lang } = usePrefs();
  return useQuery({ queryKey: modeKeys.interviews(lang), queryFn: () => api.interviews(lang) });
}

export function useSolvedLanguages(problem: string, enabled = true) {
  return useQuery({
    queryKey: modeKeys.solvedLanguages(problem),
    queryFn: () => api.solvedLanguages(problem),
    enabled,
  });
}

export function useAcceptedSolution(problem: string, language: Language | null) {
  return useQuery({
    queryKey: modeKeys.accepted(problem, language ?? ""),
    queryFn: () => api.acceptedSolution(problem, language!),
    enabled: language !== null,
  });
}

export function useSolvedByLanguage() {
  return useQuery({ queryKey: modeKeys.byLanguage, queryFn: api.solvedByLanguage });
}

/** Refresh everything an interview changes (sessions, locks on problems, progress). */
export function useInvalidateInterview() {
  const queryClient = useQueryClient();
  return () => {
    for (const key of [["interview"], ["problem"], ["practice"], ["problems"], ["cross-lang"]]) {
      void queryClient.invalidateQueries({ queryKey: key });
    }
  };
}

/** "mm:ss" (or "h:mm:ss") for a number of seconds. */
export function formatClock(seconds: number): string {
  const s = Math.max(0, Math.floor(seconds));
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const sec = String(s % 60).padStart(2, "0");
  return h > 0 ? `${h}:${String(m).padStart(2, "0")}:${sec}` : `${m}:${sec}`;
}
