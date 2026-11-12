// Progress page data (ADR 0021): the activity heatmap, solved by difficulty and topic,
// and the review-queue summary. kb-app returns Submits as UTC instants; this module turns
// them into the user's local calendar days (kb-app never guesses the time zone).
import { useQuery } from "@tanstack/react-query";
import type { ActivityRow } from "@/bindings";
import { api } from "@/lib/api";
import { usePrefs } from "@/lib/prefs";
import { localToday, useToday } from "@/lib/today";

export const progressKeys = {
  // Under "practice": a Submit invalidates every practice query.
  activity: (today: string) => ["practice", "activity", today] as const,
  breakdown: (lang: string) => ["practice", "breakdown", lang] as const,
  reviewSummary: (today: string) => ["practice", "review-summary", today] as const,
};

/** Weeks shown by the heatmap: this week plus the 52 before it (12 months). */
export const HEATMAP_WEEKS = 53;

/** A "YYYY-MM-DD" day as a local Date at noon (safe from DST shifts when adding days). */
export function dayDate(day: string): Date {
  const [y, m, d] = day.split("-").map(Number);
  return new Date(y, m - 1, d, 12);
}

export function addDays(day: string, n: number): string {
  const d = dayDate(day);
  d.setDate(d.getDate() + n);
  return localToday(d);
}

/** Monday of the week holding `day` (weeks start on Monday). */
export function weekStart(day: string): string {
  const offset = (dayDate(day).getDay() + 6) % 7; // Mon = 0 … Sun = 6
  return addDays(day, -offset);
}

/** Every day the heatmap shows, oldest first: whole weeks from Monday 52 weeks ago
 * through today (the current week stops at today). */
export function heatmapDays(today: string): string[] {
  const first = addDays(weekStart(today), -7 * (HEATMAP_WEEKS - 1));
  const out: string[] = [];
  for (let d = first; d <= today; d = addDays(d, 1)) out.push(d);
  return out;
}

/** Local midnight of `day`, in UTC ms (the `since` of the activity query). */
export function dayStartMs(day: string): number {
  const [y, m, d] = day.split("-").map(Number);
  return new Date(y, m - 1, d).getTime();
}

export type DayActivity = { submissions: number; solved: number };

/** Submits per local day, and the distinct problems Accepted that day. */
export function bucketByDay(rows: ActivityRow[]): Map<string, DayActivity> {
  const subs = new Map<string, number>();
  const solved = new Map<string, Set<string>>();
  for (const r of rows) {
    const day = localToday(new Date(r.at));
    subs.set(day, (subs.get(day) ?? 0) + 1);
    if (r.accepted) {
      if (!solved.has(day)) solved.set(day, new Set());
      solved.get(day)!.add(r.problem_id);
    }
  }
  const out = new Map<string, DayActivity>();
  for (const [day, n] of subs) out.set(day, { submissions: n, solved: solved.get(day)?.size ?? 0 });
  return out;
}

/** Color level 0-4: 0 = no Submit; 1-4 = quarters of the busiest day. */
export function heatLevel(submissions: number, max: number): 0 | 1 | 2 | 3 | 4 {
  if (submissions <= 0 || max <= 0) return 0;
  return Math.min(4, Math.max(1, Math.ceil((4 * submissions) / max))) as 1 | 2 | 3 | 4;
}

export function useActivity() {
  const today = useToday();
  return useQuery({
    queryKey: progressKeys.activity(today),
    queryFn: () => api.activity(dayStartMs(heatmapDays(today)[0])),
  });
}

export function useSolvedBreakdown() {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: progressKeys.breakdown(lang),
    queryFn: () => api.solvedBreakdown(lang),
  });
}

export function useReviewSummary() {
  const today = useToday();
  return useQuery({
    queryKey: progressKeys.reviewSummary(today),
    queryFn: () => api.reviewSummary(today),
  });
}
