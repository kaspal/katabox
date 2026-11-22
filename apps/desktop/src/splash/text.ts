// The splash's few words (ADR 0041), in the three UI languages. The splash has no
// i18next: this tiny map is all it needs. The language is the UI language the main window
// mirrored in localStorage, else the system's.
export const SPLASH_LANGS = ["en", "es", "pt-BR"] as const;
export type SplashLang = (typeof SPLASH_LANGS)[number];

export const SPLASH_TEXT: Record<SplashLang, { tag: string; s1: string; s2: string; s3: string }> = {
  en: {
    tag: "Practice, offline. One kata at a time.",
    s1: "Opening your library…",
    s2: "Waking the sandbox…",
    s3: "Ready",
  },
  es: {
    tag: "Practica sin conexión. Una kata a la vez.",
    s1: "Abriendo tu biblioteca…",
    s2: "Despertando el sandbox…",
    s3: "Listo",
  },
  "pt-BR": {
    tag: "Pratique offline. Uma kata de cada vez.",
    s1: "Abrindo sua biblioteca…",
    s2: "Acordando o sandbox…",
    s3: "Pronto",
  },
};

/** The stored UI language when valid, else the system language's family, else English. */
export function pickLang(stored: string | null, system: string | undefined): SplashLang {
  if (stored && (SPLASH_LANGS as readonly string[]).includes(stored)) return stored as SplashLang;
  const s = (system ?? "").toLowerCase();
  if (s.startsWith("es")) return "es";
  if (s.startsWith("pt")) return "pt-BR";
  return "en";
}

/** KataBox's own setting wins when it says "reduced"; otherwise the system's. */
export function prefersReducedMotion(stored: string | null, system: boolean | undefined): boolean {
  return stored === "reduced" || system === true;
}
