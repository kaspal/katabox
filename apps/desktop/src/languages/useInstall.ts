// Installing languages (ADR 0015): one install at a time, progress per component over a
// Tauri channel, and Retry after a failure or cancel (the backend keeps partial
// downloads, so a retry resumes them).
import { useCallback, useRef, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import type { ComponentPhase, InstallEvent, Language } from "@/bindings";
import { api } from "@/lib/api";
import { isAppError } from "@/lib/errors";
import { queryKeys } from "@/lib/queries";

export type ItemProgress = {
  component: string;
  phase: ComponentPhase;
  /** Bytes downloaded so far, while downloading. */
  done: number;
  total: number;
  downloadBytes: number;
};

export type InstallState = "idle" | "running" | "done" | "cancelled" | "failed";

/** JavaScript comes with the TypeScript unit. */
export function installUnit(language: Language): Language {
  return language === "javascript" ? "typescript" : language;
}

/** Decimal megabytes/gigabytes, as download sites show them. */
export function formatBytes(bytes: number, locale: string): string {
  const gb = bytes >= 1e9;
  return new Intl.NumberFormat(locale, {
    style: "unit",
    unit: gb ? "gigabyte" : "megabyte",
    maximumFractionDigits: gb ? 1 : 0,
  }).format(gb ? bytes / 1e9 : Math.max(bytes / 1e6, bytes > 0 ? 1 : 0));
}

export function useInstall() {
  const qc = useQueryClient();
  const [state, setState] = useState<InstallState>("idle");
  const [items, setItems] = useState<ItemProgress[]>([]);
  const [error, setError] = useState<unknown>(null);
  const requested = useRef<Language[]>([]);
  const running = useRef(false);

  const onEvent = useCallback((e: InstallEvent) => {
    if (e.kind === "plan") {
      setItems(
        e.items.map((i) => ({
          component: i.component,
          phase: "queued",
          done: 0,
          total: 0,
          downloadBytes: i.download_bytes,
        })),
      );
      return;
    }
    setItems((prev) =>
      prev.map((i) => (i.component === e.component ? { ...i, phase: e.phase, done: e.done, total: e.total } : i)),
    );
  }, []);

  const start = useCallback(
    async (languages: Language[]) => {
      if (running.current || languages.length === 0) return;
      running.current = true;
      requested.current = languages;
      setState("running");
      setError(null);
      setItems([]);
      try {
        await api.installLanguages(languages, onEvent);
        setState("done");
      } catch (e) {
        setError(e);
        setState(isAppError(e) && e.code === "install_cancelled" ? "cancelled" : "failed");
      } finally {
        running.current = false;
        void qc.invalidateQueries({ queryKey: queryKeys.languages });
        void qc.invalidateQueries({ queryKey: queryKeys.appInfo });
      }
    },
    [onEvent, qc],
  );

  const cancel = useCallback(() => void api.cancelInstall().catch(() => {}), []);
  const retry = useCallback(() => void start(requested.current), [start]);

  return { state, items, error, start, cancel, retry };
}
