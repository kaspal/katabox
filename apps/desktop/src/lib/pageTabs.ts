// Page tabs (ui-polish): Progress tabs and Settings sections. The tab lives in the route's
// search params (`?tab=` / `?section=`), so every tab is linkable and Back works; the last
// one is remembered per machine in a setting (`progress.tab`, `settings.section`; never
// exported) and opens when the URL names none.
import { useCallback, useEffect } from "react";
import type { JsonValue } from "@/bindings";
import { useSetting } from "@/lib/settings";

export const PROGRESS_TABS = ["overview", "patterns", "topics", "plan", "katas", "history"] as const;
export type ProgressTab = (typeof PROGRESS_TABS)[number];

export const SETTINGS_SECTIONS = ["appearance", "editor", "practice", "languages", "learn", "data", "about"] as const;
export type SettingsSection = (typeof SETTINGS_SECTIONS)[number];

export const oneOfTabs = <T extends string>(v: unknown, values: readonly T[]): T | undefined =>
  typeof v === "string" && (values as readonly string[]).includes(v) ? (v as T) : undefined;

/**
 * The open tab: the URL's, else the remembered one, else `fallback`. `select` changes the
 * URL (replacing the history entry, so tab switches don't pile up behind Back) and
 * remembers the tab; a tab reached by a link is remembered too.
 */
export function useRememberedTab<T extends string>(
  key: string,
  values: readonly T[],
  fallback: T,
  fromUrl: T | undefined,
  toUrl: (tab: T) => void,
): [T, (tab: string) => void] {
  const parse = useCallback((v: JsonValue | null) => oneOfTabs(v, values), [values]);
  const [remembered, remember] = useSetting<T>(key, fallback, parse, { mirror: true });
  const tab = fromUrl ?? remembered;
  useEffect(() => {
    if (fromUrl !== undefined && fromUrl !== remembered) remember(fromUrl as T & JsonValue);
  }, [fromUrl, remembered, remember]);
  const select = useCallback(
    (next: string) => {
      const v = oneOfTabs(next, values);
      if (!v) return;
      remember(v as T & JsonValue);
      toUrl(v);
    },
    [values, remember, toUrl],
  );
  return [tab, select];
}
