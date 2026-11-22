// React Query hooks for kata drills (expansion M1, ADR 0033). The rules (picks, rusty
// scores, bests, locks) live in kb-app; these only fetch and refresh.
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { usePrefs } from "@/lib/prefs";
import { useToday } from "@/lib/today";

export const kataKeys = {
  all: ["kata"] as const,
  options: (today: string, lang: string) => ["kata", "options", today, lang] as const,
  state: (lang: string) => ["kata", "state", lang] as const,
  overview: (today: string, lang: string) => ["kata", "overview", today, lang] as const,
};

export function useKataOptions() {
  const { lang } = usePrefs();
  const today = useToday();
  return useQuery({ queryKey: kataKeys.options(today, lang), queryFn: () => api.kataOptions(today, lang) });
}

/** The running kata (null when none). Rechecked every 30 s: a session expires after 2 h. */
export function useKataState() {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: kataKeys.state(lang),
    queryFn: () => api.kataState(lang),
    refetchInterval: 30_000,
  });
}

export function useKataOverview(enabled = true) {
  const { lang } = usePrefs();
  const today = useToday();
  return useQuery({
    queryKey: kataKeys.overview(today, lang),
    queryFn: () => api.kataOverview(today, lang),
    enabled,
  });
}

export function useInvalidateKata() {
  const queryClient = useQueryClient();
  return () => void queryClient.invalidateQueries({ queryKey: kataKeys.all });
}

/** `m:ss` (or `h:mm:ss`) of a duration in milliseconds. */
export function formatDuration(ms: number): string {
  const s = Math.max(0, Math.round(ms / 1000));
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const sec = String(s % 60).padStart(2, "0");
  return h > 0 ? `${h}:${String(m).padStart(2, "0")}:${sec}` : `${m}:${sec}`;
}
