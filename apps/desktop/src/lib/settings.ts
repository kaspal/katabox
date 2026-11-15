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

export const KEYMAPS = ["default", "vscode", "vim", "emacs"] as const;
export type KeymapPref = (typeof KEYMAPS)[number];
export const FONT_SIZES = [12, 13, 14, 15, 16, 18, 20] as const;
/** Spaces per indent; 0 keeps each language's own (4 for Python, PHP and Rust; 2 for TS). */
export const TAB_WIDTHS = [0, 2, 4, 8] as const;
/** Editor colour themes; `katabox` follows the app's tokens (styles/tokens.css). */
export const EDITOR_THEMES = ["katabox", "high-ink", "paper", "dusk"] as const;
export type EditorTheme = (typeof EDITOR_THEMES)[number];
export const JUDGE_LANGUAGES: Language[] = ["python", "typescript", "php", "rust"];

export const parseKeymap = (v: JsonValue | null): KeymapPref | undefined =>
  typeof v === "string" && (KEYMAPS as readonly string[]).includes(v) ? (v as KeymapPref) : undefined;
export const parseFontSize = (v: JsonValue | null): number | undefined =>
  typeof v === "number" && v >= 10 && v <= 28 ? v : undefined;
export const parseTabWidth = (v: JsonValue | null): number | undefined =>
  typeof v === "number" && (TAB_WIDTHS as readonly number[]).includes(v) ? v : undefined;
export const parseEditorTheme = (v: JsonValue | null): EditorTheme | undefined =>
  typeof v === "string" && (EDITOR_THEMES as readonly string[]).includes(v) ? (v as EditorTheme) : undefined;
export const parseLanguage = (v: JsonValue | null): Language | undefined =>
  typeof v === "string" && (JUDGE_LANGUAGES as string[]).includes(v) ? (v as Language) : undefined;

export function useEditorPrefs() {
  const [keymap, setKeymap] = useSetting<KeymapPref>("editor.keymap", "default", parseKeymap, {
    mirror: true,
  });
  const [fontSize, setFontSize] = useSetting<number>("editor.font_size", 14, parseFontSize, {
    mirror: true,
  });
  const [ligatures, setLigatures] = useSetting<boolean>(
    "editor.ligatures",
    false,
    (v) => (typeof v === "boolean" ? v : undefined),
    { mirror: true },
  );
  const [tabWidth, setTabWidth] = useSetting<number>("editor.tab_width", 0, parseTabWidth, {
    mirror: true,
  });
  const [theme, setTheme] = useSetting<EditorTheme>("editor.theme", "katabox", parseEditorTheme, {
    mirror: true,
  });
  return {
    keymap,
    setKeymap,
    fontSize,
    setFontSize,
    ligatures,
    setLigatures,
    tabWidth,
    setTabWidth,
    theme,
    setTheme,
  };
}
