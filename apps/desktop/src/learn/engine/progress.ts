// Learn progress in user.db settings (ADR 0029): `learn.viewed` (topic ids whose animation
// was watched to the end by playing or stepping), `learn.last` (for Continue) and
// `learn.speed`. viewed and last travel with an export; speed and layout stay home.
import { useCallback } from "react";
import type { JsonValue } from "@/bindings";
import { useSetting } from "@/lib/settings";
import { isSpeed, type Speed } from "./usePlayer";

export type LastTopic = { subject: string; topic: string; step: number; total: number };

const MAX_VIEWED = 500;

export const parseViewed = (v: JsonValue | null): string[] | undefined =>
  Array.isArray(v) && v.every((x) => typeof x === "string") ? (v as string[]).slice(0, MAX_VIEWED) : undefined;

export const parseLast = (v: JsonValue | null): LastTopic | undefined => {
  if (typeof v !== "object" || v === null || Array.isArray(v)) return undefined;
  const o = v as Record<string, JsonValue>;
  const nat = (x: JsonValue | undefined) => typeof x === "number" && Number.isInteger(x) && x >= 0;
  return typeof o.subject === "string" && typeof o.topic === "string" && nat(o.step) && nat(o.total)
    ? { subject: o.subject, topic: o.topic, step: o.step as number, total: o.total as number }
    : undefined;
};

export const parseSpeed = (v: JsonValue | null): Speed | undefined => (isSpeed(v) ? v : undefined);

export function useViewed() {
  const [viewed, setViewed] = useSetting<string[]>("learn.viewed", [], parseViewed);
  const markViewed = useCallback(
    (topic: string) => {
      if (!viewed.includes(topic)) setViewed([...viewed, topic]);
    },
    [viewed, setViewed],
  );
  return { viewed, markViewed };
}

export function useLastTopic() {
  return useSetting<LastTopic | null>("learn.last", null, (v) => parseLast(v) ?? null);
}

export function useLearnSpeed() {
  return useSetting<Speed>("learn.speed", 1, parseSpeed);
}
