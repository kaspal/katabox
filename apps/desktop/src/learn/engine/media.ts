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
export const useReducedMotion = () => useMedia(REDUCED_MOTION);
