// Home → "This week" (ADR 0041): Monday to Sunday of the current week, from the same
// activity rows as the Progress heatmap (no new storage).
import { useMemo } from "react";
import type { ActivityRow } from "@/bindings";
import { addDays, bucketByDay, useActivity, weekStart } from "@/lib/progress";
import { useToday } from "@/lib/today";

export type WeekDay = { day: string; submissions: number; solved: number; today: boolean; future: boolean };

export function weekDays(rows: ActivityRow[], today: string): WeekDay[] {
  const byDay = bucketByDay(rows);
  const monday = weekStart(today);
  return Array.from({ length: 7 }, (_, i) => {
    const day = addDays(monday, i);
    const a = byDay.get(day);
    return { day, submissions: a?.submissions ?? 0, solved: a?.solved ?? 0, today: day === today, future: day > today };
  });
}

export function useWeekActivity() {
  const today = useToday();
  const activity = useActivity();
  const days = useMemo(() => (activity.data ? weekDays(activity.data, today) : undefined), [activity.data, today]);
  return { days, isPending: activity.isPending, error: activity.error, refetch: () => void activity.refetch() };
}
