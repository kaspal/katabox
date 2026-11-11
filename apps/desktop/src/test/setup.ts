import "@testing-library/jest-dom/vitest";
import { afterEach, vi } from "vitest";
import { cleanup } from "@testing-library/react";
import { initI18n } from "@/i18n";

// jsdom lacks a few browser APIs that Radix, cmdk and the virtualizer use.
class ResizeObserverStub {
  observe() {}
  unobserve() {}
  disconnect() {}
}
globalThis.ResizeObserver ??= ResizeObserverStub as unknown as typeof ResizeObserver;
Element.prototype.scrollIntoView ??= function () {};
Element.prototype.hasPointerCapture ??= () => false;
Element.prototype.releasePointerCapture ??= function () {};
Element.prototype.scrollTo ??= function () {} as typeof Element.prototype.scrollTo;
// jsdom has no layout: give elements a size so the virtualized library renders rows.
Object.defineProperty(HTMLElement.prototype, "clientHeight", { configurable: true, get: () => 600 });
Object.defineProperty(HTMLElement.prototype, "clientWidth", { configurable: true, get: () => 800 });

/** `prefers-color-scheme: dark` answer for the tests (default light). */
export const media = { dark: false, listeners: new Set<() => void>() };
window.matchMedia = vi.fn((query: string) => ({
  get matches() {
    return query.includes("dark") ? media.dark : false;
  },
  media: query,
  onchange: null,
  addEventListener: (_: string, cb: () => void) => media.listeners.add(cb),
  removeEventListener: (_: string, cb: () => void) => media.listeners.delete(cb),
  addListener: () => {},
  removeListener: () => {},
  dispatchEvent: () => false,
})) as unknown as typeof window.matchMedia;

initI18n("en");

afterEach(() => {
  cleanup();
  localStorage.clear();
  document.documentElement.classList.remove("dark");
  media.dark = false;
  media.listeners.clear();
  initI18n("en");
});
