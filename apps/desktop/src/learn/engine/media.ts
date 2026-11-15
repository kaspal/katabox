// A media query as React state (follows changes). Used for the narrow layout and for
// "reduce motion", which turns every scene transition off (stepping still works).
import { useSyncExternalStore } from "react";

export function useMedia(query: string): boolean {
  return useSyncExternalStore(
    (cb) => {
      const m = window.matchMedia?.(query);
      m?.addEventListener?.("change", cb);
      return () => m?.removeEventListener?.("change", cb);
    },
    () => window.matchMedia?.(query).matches ?? false,
  );
}

export const REDUCED_MOTION = "(prefers-reduced-motion: reduce)";

/** Whether <html> has a class, following changes (the app's own preferences). */
export function useRootClass(name: string): boolean {
  return useSyncExternalStore(
    (cb) => {
      if (typeof MutationObserver === "undefined") return () => {};
      const mo = new MutationObserver(cb);
      mo.observe(document.documentElement, { attributes: true, attributeFilter: ["class"] });
      return () => mo.disconnect();
    },
    () => document.documentElement.classList.contains(name),
  );
}

/** Reduce motion: the system's setting, or KataBox's own (Settings → Appearance, ui.motion). */
export const useReducedMotion = () => {
  const system = useMedia(REDUCED_MOTION);
  const app = useRootClass("reduce-motion");
  return system || app;
};
