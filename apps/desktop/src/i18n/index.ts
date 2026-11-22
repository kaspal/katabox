// i18n (ADR 0012): every visible string comes from these resources. Keys are typed
// (see i18next.d.ts), so a missing key is a compile error; `i18n.test.ts` checks that
// English, Spanish and Portuguese have the same keys.
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import en from "./locales/en.json";
import es from "./locales/es.json";
import ptBR from "./locales/pt-BR.json";

export const UI_LANGS = ["en", "es", "pt-BR"] as const;
export type UiLangCode = (typeof UI_LANGS)[number];

/** The languages problems and Learn are written in (expansion M14): the backend's
 * `UiLang`. With a UI language content isn't written in (pt-BR), the user picks one. */
export const CONTENT_LANGS = ["en", "es"] as const;
export type ContentLangCode = (typeof CONTENT_LANGS)[number];

export function isContentLang(value: unknown): value is ContentLangCode {
  return typeof value === "string" && (CONTENT_LANGS as readonly string[]).includes(value);
}

/** The content language for a UI language: the same one when content exists in it,
 * else the user's choice (English by default). */
export function contentLangFor(ui: UiLangCode, chosen: ContentLangCode | null): ContentLangCode {
  return isContentLang(ui) ? ui : (chosen ?? "en");
}

export const resources = {
  en: { translation: en },
  es: { translation: es },
  "pt-BR": { translation: ptBR },
} as const;

export function isUiLang(value: unknown): value is UiLangCode {
  return typeof value === "string" && (UI_LANGS as readonly string[]).includes(value);
}

/** The OS/browser language when it is one we support, else English. */
export function detectUiLang(): UiLangCode {
  const nav = (typeof navigator !== "undefined" ? navigator.language : "en").toLowerCase();
  if (nav.startsWith("es")) return "es";
  if (nav.startsWith("pt")) return "pt-BR";
  return "en";
}

export function initI18n(lang: UiLangCode) {
  if (!i18n.isInitialized) {
    void i18n.use(initReactI18next).init({
      resources,
      lng: lang,
      fallbackLng: "en",
      interpolation: { escapeValue: false },
      returnNull: false,
      // Resources are bundled: initialize synchronously so the first render is translated.
      initAsync: false,
    });
  } else if (i18n.language !== lang) {
    void i18n.changeLanguage(lang);
  }
  document.documentElement.lang = lang;
  return i18n;
}

export default i18n;
