// Typed user settings (ADR 0013): user.db through get/set_setting is the source of truth;
// `mirror` keeps a localStorage copy so layout-like settings apply on first paint.
import { useCallback } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import type { JsonValue, Language } from "@/bindings";
import { api } from "@/lib/api";

const MIRROR_PREFIX = "katabox.setting.";

function readMirror(key: string): JsonValue | undefined {
  try {
    const raw = localStorage.getItem(MIRROR_PREFIX + key);
    return raw === null ? undefined : (JSON.parse(raw) as JsonValue);
  } catch {
    return undefined;
  }
}

function writeMirror(key: string, value: JsonValue) {
  try {
    localStorage.setItem(MIRROR_PREFIX + key, JSON.stringify(value));
  } catch {
    // Storage unavailable: user.db still has it.
  }
}

export const settingKey = (key: string) => ["setting", key] as const;

/**
 * A setting parsed by `parse` (which returns undefined for invalid values, so the
 * fallback applies). Returns the value and a setter that saves it.
 */
export function useSetting<T>(
  key: string,
  fallback: T,
  parse: (v: JsonValue | null) => T | undefined,
  opts: { mirror?: boolean } = {},
): [T, (value: T & JsonValue) => void] {
  const queryClient = useQueryClient();
  const query = useQuery({
    queryKey: settingKey(key),
    queryFn: () => api.getSetting(key),
    staleTime: Infinity,
    placeholderData: opts.mirror ? (readMirror(key) ?? undefined) : undefined,
  });
  const parsed = query.data === undefined ? undefined : parse(query.data);
  const value = parsed ?? fallback;
  const set = useCallback(
    (next: T & JsonValue) => {
      queryClient.setQueryData(settingKey(key), next);
      if (opts.mirror) writeMirror(key, next);
      void api.setSetting(key, next).catch(() => {});
    },
    [key, opts.mirror, queryClient],
  );
  return [value, set];
}

// --- Editor preferences ---------------------------------------------------------------

export const KEYMAPS = ["default", "vscode", "vim"] as const;
export type KeymapPref = (typeof KEYMAPS)[number];
export const FONT_SIZES = [12, 13, 14, 15, 16, 18, 20] as const;
export const JUDGE_LANGUAGES: Language[] = ["python", "typescript", "php", "rust"];

export const parseKeymap = (v: JsonValue | null): KeymapPref | undefined =>
  typeof v === "string" && (KEYMAPS as readonly string[]).includes(v) ? (v as KeymapPref) : undefined;
export const parseFontSize = (v: JsonValue | null): number | undefined =>
  typeof v === "number" && v >= 10 && v <= 28 ? v : undefined;
export const parseLanguage = (v: JsonValue | null): Language | undefined =>
  typeof v === "string" && (JUDGE_LANGUAGES as string[]).includes(v) ? (v as Language) : undefined;

export function useEditorPrefs() {
  const [keymap, setKeymap] = useSetting<KeymapPref>("editor.keymap", "default", parseKeymap, {
    mirror: true,
  });
  const [fontSize, setFontSize] = useSetting<number>("editor.font_size", 14, parseFontSize, {
    mirror: true,
  });
  return { keymap, setKeymap, fontSize, setFontSize };
}
