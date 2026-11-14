// The welcome flow's machine-local settings (features-content M2). They are under
// `onboarding.`, which the export leaves out: a new computer gets its own welcome.
// Where practice starts (`practice.start`) is decided in kb-app (finish_onboarding).
import { useQuery } from "@tanstack/react-query";
import type { JsonValue } from "@/bindings";
import { api } from "@/lib/api";
import { settingKey, useSetting } from "@/lib/settings";

export const DONE_KEY = "onboarding.done";
export const PLACEMENT_KEY = "onboarding.placement";
export const TOUR_KEY = "onboarding.tour";

export type Placement = { problems: string[]; index: number };

/** `onboarding.done`: true (finished or skipped), false ("Show the welcome again"),
 * null (never decided); `loaded` once user.db answered. */
export function useOnboardingDone() {
  const query = useQuery({ queryKey: settingKey(DONE_KEY), queryFn: () => api.getSetting(DONE_KEY), staleTime: Infinity });
  const [done, setDone] = useSetting<boolean | null>(DONE_KEY, null, (v) => (typeof v === "boolean" ? v : undefined));
  return { loaded: query.isSuccess, done, setDone };
}

export const parsePlacement = (v: JsonValue | null): Placement | null | undefined => {
  if (v === null) return null;
  if (typeof v !== "object" || Array.isArray(v)) return undefined;
  const { problems, index } = v as Record<string, JsonValue>;
  return Array.isArray(problems) &&
    problems.every((p) => typeof p === "string") &&
    typeof index === "number" &&
    Number.isInteger(index) &&
    index >= 0 &&
    index < problems.length
    ? { problems: problems as string[], index }
    : undefined;
};

export function usePlacement() {
  return useSetting<Placement | null>(PLACEMENT_KEY, null, parsePlacement);
}

export function useTourPending() {
  return useSetting<boolean>(TOUR_KEY, false, (v) => (typeof v === "boolean" ? v : undefined));
}
