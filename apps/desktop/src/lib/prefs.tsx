// UI preferences: theme, UI language, contrast and motion (ADR 0012). user.db (get/set_setting) is the
// source of truth; localStorage only mirrors them so the first paint is in the right
// theme and language before the backend answers.
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";
import {
  contentLangFor,
  detectUiLang,
  initI18n,
  isContentLang,
  isUiLang,
  type ContentLangCode,
  type UiLangCode,
} from "@/i18n";
import { api } from "@/lib/api";

export const THEMES = ["system", "light", "dark"] as const;
export type ThemePref = (typeof THEMES)[number];

export const SETTING_THEME = "theme";
export const SETTING_UI_LANG = "ui_lang";
/** The content language chosen while the UI is in a language content isn't written in
 * (expansion M14). */
export const SETTING_CONTENT_LANG = "content_lang";
const MIRROR_THEME = "katabox.theme";
const MIRROR_LANG = "katabox.ui_lang";
const MIRROR_CONTENT = "katabox.content_lang";

/** High contrast: stronger text, borders and focus rings (`.hc` tokens, ≥ 7:1 text). */
export const CONTRASTS = ["normal", "high"] as const;
export type ContrastPref = (typeof CONTRASTS)[number];
/** Motion: follow the system's "reduce motion", or always reduce it in KataBox. */
export const MOTIONS = ["system", "reduced"] as const;
export type MotionPref = (typeof MOTIONS)[number];
export const SETTING_CONTRAST = "ui.contrast";
export const SETTING_MOTION = "ui.motion";
const MIRROR_CONTRAST = "katabox.ui.contrast";
const MIRROR_MOTION = "katabox.ui.motion";

const isContrast = (v: unknown): v is ContrastPref => typeof v === "string" && (CONTRASTS as readonly string[]).includes(v);
const isMotion = (v: unknown): v is MotionPref => typeof v === "string" && (MOTIONS as readonly string[]).includes(v);

/** `.hc` on <html> switches the high-contrast tokens on. */
export function applyContrast(pref: ContrastPref) {
  document.documentElement.classList.toggle("hc", pref === "high");
}

/** `.reduce-motion` on <html> stops CSS transitions and animations, and Learn's scenes
 * read it as well as the system setting (`useReducedMotion`). */
export function applyMotion(pref: MotionPref) {
  document.documentElement.classList.toggle("reduce-motion", pref === "reduced");
}

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

/** `lang` is the UI language here. */
type Initial = {
  theme: ThemePref;
  lang: UiLangCode;
  contrast?: ContrastPref;
  motion?: MotionPref;
  contentLang?: ContentLangCode | null;
};

/** First-paint values, before user.db answers. */
export function initialPrefs(): Required<Initial> {
  const theme = readMirror(MIRROR_THEME);
  const lang = readMirror(MIRROR_LANG);
  const contrast = readMirror(MIRROR_CONTRAST);
  const motion = readMirror(MIRROR_MOTION);
  const content = readMirror(MIRROR_CONTENT);
  return {
    theme: isTheme(theme) ? theme : "system",
    lang: isUiLang(lang) ? lang : detectUiLang(),
    contrast: isContrast(contrast) ? contrast : "normal",
    motion: isMotion(motion) ? motion : "system",
    contentLang: isContentLang(content) ? content : null,
  };
}

type Prefs = {
  theme: ThemePref;
  /** The content language (problems, Learn): what the backend's commands take. */
  lang: ContentLangCode;
  /** The UI language (i18n). */
  uiLang: UiLangCode;
  /** The content language chosen for a UI language content isn't written in. */
  contentChoice: ContentLangCode | null;
  contrast: ContrastPref;
  motion: MotionPref;
  setTheme: (t: ThemePref) => void;
  setUiLang: (l: UiLangCode) => void;
  setContentLang: (l: ContentLangCode) => void;
  setContrast: (c: ContrastPref) => void;
  setMotion: (m: MotionPref) => void;
};

const PrefsContext = createContext<Prefs | null>(null);

export function PrefsProvider({
  children,
  initial = initialPrefs(),
}: {
  children: React.ReactNode;
  initial?: Initial;
}) {
  const [theme, setThemeState] = useState<ThemePref>(initial.theme);
  const [uiLang, setUiLangState] = useState<UiLangCode>(initial.lang);
  const [contentChoice, setContentChoice] = useState<ContentLangCode | null>(initial.contentLang ?? null);
  const [contrast, setContrastState] = useState<ContrastPref>(initial.contrast ?? "normal");
  const [motion, setMotionState] = useState<MotionPref>(initial.motion ?? "system");

  useEffect(() => applyContrast(contrast), [contrast]);
  useEffect(() => applyMotion(motion), [motion]);

  useEffect(() => {
    applyTheme(theme);
    if (theme !== "system") return;
    const q = darkQuery();
    const onChange = () => applyTheme("system");
    q?.addEventListener("change", onChange);
    return () => q?.removeEventListener("change", onChange);
  }, [theme]);

  useEffect(() => {
    initI18n(uiLang);
  }, [uiLang]);

  // user.db wins over the first-paint mirror.
  useEffect(() => {
    let live = true;
    void Promise.all([
      api.getSetting(SETTING_THEME),
      api.getSetting(SETTING_UI_LANG),
      api.getSetting(SETTING_CONTRAST),
      api.getSetting(SETTING_MOTION),
      api.getSetting(SETTING_CONTENT_LANG),
    ])
      .then(([t, l, c, m, cl]) => {
        if (!live) return;
        if (isContrast(c)) {
          setContrastState(c);
          writeMirror(MIRROR_CONTRAST, c);
        }
        if (isMotion(m)) {
          setMotionState(m);
          writeMirror(MIRROR_MOTION, m);
        }
        if (isTheme(t)) {
          setThemeState(t);
          writeMirror(MIRROR_THEME, t);
        }
        if (isUiLang(l)) {
          setUiLangState(l);
          writeMirror(MIRROR_LANG, l);
        }
        if (isContentLang(cl)) {
          setContentChoice(cl);
          writeMirror(MIRROR_CONTENT, cl);
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

  const setUiLang = useCallback((l: UiLangCode) => {
    setUiLangState(l);
    writeMirror(MIRROR_LANG, l);
    void api.setSetting(SETTING_UI_LANG, l).catch(() => {});
  }, []);

  const setContentLang = useCallback((l: ContentLangCode) => {
    setContentChoice(l);
    writeMirror(MIRROR_CONTENT, l);
    void api.setSetting(SETTING_CONTENT_LANG, l).catch(() => {});
  }, []);

  const setContrast = useCallback((c: ContrastPref) => {
    setContrastState(c);
    writeMirror(MIRROR_CONTRAST, c);
    void api.setSetting(SETTING_CONTRAST, c).catch(() => {});
  }, []);

  const setMotion = useCallback((m: MotionPref) => {
    setMotionState(m);
    writeMirror(MIRROR_MOTION, m);
    void api.setSetting(SETTING_MOTION, m).catch(() => {});
  }, []);

  const lang = contentLangFor(uiLang, contentChoice);
  const value = useMemo(
    () => ({
      theme,
      lang,
      uiLang,
      contentChoice,
      contrast,
      motion,
      setTheme,
      setUiLang,
      setContentLang,
      setContrast,
      setMotion,
    }),
    [theme, lang, uiLang, contentChoice, contrast, motion, setTheme, setUiLang, setContentLang, setContrast, setMotion],
  );
  return <PrefsContext.Provider value={value}>{children}</PrefsContext.Provider>;
}

export function usePrefs(): Prefs {
  const ctx = useContext(PrefsContext);
  if (!ctx) throw new Error("usePrefs outside PrefsProvider");
  return ctx;
}
