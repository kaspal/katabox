// The user's local calendar date, "YYYY-MM-DD" (ADR 0019). The backend never guesses the
// time zone: every practice command takes this date, so reviews and the daily problem
// follow the user's local midnight and survive time-zone changes.
import { useEffect, useState } from "react";

export function localToday(now: Date = new Date()): string {
  const y = now.getFullYear();
  const m = String(now.getMonth() + 1).padStart(2, "0");
  const d = String(now.getDate()).padStart(2, "0");
  return `${y}-${m}-${d}`;
}

/** Milliseconds until the next local midnight. */
export function msUntilMidnight(now: Date = new Date()): number {
  const next = new Date(now.getFullYear(), now.getMonth(), now.getDate() + 1);
  return next.getTime() - now.getTime();
}

/** Today's local date, updated at local midnight while the app stays open. */
export function useToday(): string {
  const [today, setToday] = useState(() => localToday());
  useEffect(() => {
    const id = window.setTimeout(() => setToday(localToday()), msUntilMidnight() + 1000);
    return () => window.clearTimeout(id);
  }, [today]);
  return today;
}
