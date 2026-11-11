// i18n (ADR 0012): every visible string comes from these resources. Keys are typed
// (see i18next.d.ts), so a missing key is a compile error; `i18n.test.ts` checks that
// English and Spanish have the same keys.
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import en from "./locales/en.json";
import es from "./locales/es.json";

export const UI_LANGS = ["en", "es"] as const;
export type UiLangCode = (typeof UI_LANGS)[number];

export const resources = {
  en: { translation: en },
  es: { translation: es },
} as const;

export function isUiLang(value: unknown): value is UiLangCode {
  return typeof value === "string" && (UI_LANGS as readonly string[]).includes(value);
}

/** The OS/browser language when it is one we support, else English. */
export function detectUiLang(): UiLangCode {
  const nav = typeof navigator !== "undefined" ? navigator.language : "en";
  return nav.toLowerCase().startsWith("es") ? "es" : "en";
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
