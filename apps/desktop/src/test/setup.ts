import "@testing-library/jest-dom/vitest";
import { afterEach, vi } from "vitest";
import { cleanup, configure } from "@testing-library/react";
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
// CodeMirror measures text ranges.
Range.prototype.getClientRects ??= () => ({ length: 0, item: () => null, [Symbol.iterator]: [][Symbol.iterator] }) as unknown as DOMRectList;
Range.prototype.getBoundingClientRect ??= () => new DOMRect(0, 0, 0, 0);
document.elementFromPoint ??= () => null;
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
  // Only the colour-scheme query ever changes in tests, so only its listeners are kept
  // (a browser notifies just the listeners of the query that changed; CodeMirror, for
  // one, listens to "print").
  addEventListener: (_: string, cb: () => void) => {
    if (query.includes("prefers-color-scheme")) media.listeners.add(cb);
  },
  removeEventListener: (_: string, cb: () => void) => media.listeners.delete(cb),
  addListener: () => {},
  removeListener: () => {},
  dispatchEvent: () => false,
})) as unknown as typeof window.matchMedia;

initI18n("en");

// One generous default for every findBy*/waitFor (Testing Library's own is 1 s). The
// lazy routes (workspace, CodeMirror, the language client) can take several seconds
// when the suite runs right after a heavy cargo build; a per-call timeout here and
// there is what made those tests flaky.
configure({ asyncUtilTimeout: 10_000 });

vi.mock("@/components/ui/resizable", () => import("@/test/resizableStub"));

afterEach(() => {
  cleanup();
  localStorage.clear();
  document.documentElement.classList.remove("dark");
  media.dark = false;
  media.listeners.clear();
  initI18n("en");
});
