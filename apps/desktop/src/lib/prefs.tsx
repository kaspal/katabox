// UI preferences: theme and UI language (ADR 0012). user.db (get/set_setting) is the
// source of truth; localStorage only mirrors them so the first paint is in the right
// theme and language before the backend answers.
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";
import { initI18n, isUiLang, detectUiLang, type UiLangCode } from "@/i18n";
import { api } from "@/lib/api";

export const THEMES = ["system", "light", "dark"] as const;
export type ThemePref = (typeof THEMES)[number];

export const SETTING_THEME = "theme";
export const SETTING_UI_LANG = "ui_lang";
const MIRROR_THEME = "katabox.theme";
const MIRROR_LANG = "katabox.ui_lang";

function isTheme(v: unknown): v is ThemePref {
  return typeof v === "string" && (THEMES as readonly string[]).includes(v);
}

function readMirror(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

function writeMirror(key: string, value: string) {
  try {
    localStorage.setItem(key, value);
  } catch {
    // Storage can be unavailable; user.db still has the value.
  }
}

const darkQuery = () =>
  typeof window.matchMedia === "function"
    ? window.matchMedia("(prefers-color-scheme: dark)")
    : null;

export function resolveTheme(pref: ThemePref): "light" | "dark" {
  if (pref !== "system") return pref;
  return darkQuery()?.matches ? "dark" : "light";
}

export function applyTheme(pref: ThemePref) {
  document.documentElement.classList.toggle("dark", resolveTheme(pref) === "dark");
}

/** First-paint values, before user.db answers. */
export function initialPrefs(): { theme: ThemePref; lang: UiLangCode } {
  const theme = readMirror(MIRROR_THEME);
  const lang = readMirror(MIRROR_LANG);
  return {
    theme: isTheme(theme) ? theme : "system",
    lang: isUiLang(lang) ? lang : detectUiLang(),
  };
}

type Prefs = {
  theme: ThemePref;
  lang: UiLangCode;
  setTheme: (t: ThemePref) => void;
  setLang: (l: UiLangCode) => void;
};

const PrefsContext = createContext<Prefs | null>(null);

export function PrefsProvider({
  children,
  initial = initialPrefs(),
}: {
  children: React.ReactNode;
  initial?: { theme: ThemePref; lang: UiLangCode };
}) {
  const [theme, setThemeState] = useState<ThemePref>(initial.theme);
  const [lang, setLangState] = useState<UiLangCode>(initial.lang);

  useEffect(() => {
    applyTheme(theme);
    if (theme !== "system") return;
    const q = darkQuery();
    const onChange = () => applyTheme("system");
    q?.addEventListener("change", onChange);
    return () => q?.removeEventListener("change", onChange);
  }, [theme]);

  useEffect(() => {
    initI18n(lang);
  }, [lang]);

  // user.db wins over the first-paint mirror.
  useEffect(() => {
    let live = true;
    void Promise.all([api.getSetting(SETTING_THEME), api.getSetting(SETTING_UI_LANG)])
      .then(([t, l]) => {
        if (!live) return;
        if (isTheme(t)) {
          setThemeState(t);
          writeMirror(MIRROR_THEME, t);
        }
        if (isUiLang(l)) {
          setLangState(l);
          writeMirror(MIRROR_LANG, l);
        }
      })
      .catch(() => {
        // Keep the first-paint values; the shell shows backend errors elsewhere.
      });
    return () => {
      live = false;
    };
  }, []);

  const setTheme = useCallback((t: ThemePref) => {
    setThemeState(t);
    writeMirror(MIRROR_THEME, t);
    void api.setSetting(SETTING_THEME, t).catch(() => {});
  }, []);

  const setLang = useCallback((l: UiLangCode) => {
    setLangState(l);
    writeMirror(MIRROR_LANG, l);
    void api.setSetting(SETTING_UI_LANG, l).catch(() => {});
  }, []);

  const value = useMemo(
    () => ({ theme, lang, setTheme, setLang }),
    [theme, lang, setTheme, setLang],
  );
  return <PrefsContext.Provider value={value}>{children}</PrefsContext.Provider>;
}

export function usePrefs(): Prefs {
  const ctx = useContext(PrefsContext);
  if (!ctx) throw new Error("usePrefs outside PrefsProvider");
  return ctx;
}
