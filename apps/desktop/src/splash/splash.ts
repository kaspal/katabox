// The startup splash (ADR 0041). Plain TS on a static page: picks the text language,
// writes the version, places the sparks and starts the animation. No IPC, no Tauri API.
import "@/splash/splash.css";
import { SPLASH_TEXT, pickLang, prefersReducedMotion } from "@/splash/text";

declare const __APP_VERSION__: string;

const root = document.querySelector<HTMLElement>(".sp");
if (root) {
  const lang = pickLang(readLocal("katabox.ui_lang"), navigator.language);
  document.documentElement.lang = lang;
  for (const el of document.querySelectorAll<HTMLElement>("[data-text]")) {
    const key = el.dataset.text as keyof (typeof SPLASH_TEXT)["en"];
    el.textContent = SPLASH_TEXT[lang][key];
  }
  const version = root.querySelector<HTMLElement>(".version");
  if (version) version.textContent = `v${__APP_VERSION__}`;

  // Eight sparks around the ring, in the four status colours.
  const sparks = root.querySelector<SVGGElement>(".sparks");
  for (let i = 0; i < 8 && sparks; i++) {
    const a = (i / 8) * Math.PI * 2 + 0.3;
    const c = document.createElementNS("http://www.w3.org/2000/svg", "circle");
    c.setAttribute("r", "2.4");
    c.setAttribute("class", `spark spark-${i % 4}`);
    c.style.setProperty("--dx", `${Math.round(Math.cos(a) * 120)}px`);
    c.style.setProperty("--dy", `${Math.round(Math.sin(a) * 90)}px`);
    sparks.appendChild(c);
  }

  // KataBox's own "reduce motion" (mirrored by the main window), or the system's.
  if (prefersReducedMotion(readLocal("katabox.ui.motion"), window.matchMedia?.("(prefers-reduced-motion: reduce)").matches)) {
    root.classList.add("rm");
  }
  requestAnimationFrame(() => root.classList.add("run"));
}

function readLocal(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}
