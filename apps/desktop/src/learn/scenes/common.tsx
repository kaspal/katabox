// Shared scene parts (ADR 0029): container width, role colours, the index row, pointers,
// the accessible summary and the text fallback. Scenes are SVG; items are keyed by their
// stable id, so a value that moves slides there (equal values included).
import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { motion, type Transition } from "motion/react";
import { useReducedMotion } from "../engine/media";
import type { TFunction } from "i18next";
import type { Pointer, Role, Step } from "../engine/types";

export type SceneProps = {
  step: Step;
  /** Jump to the new state with no animation (timeline drag, reduced motion). */
  instant: boolean;
  /** "swaps" or "shifts": how the moving role is named. */
  movesLabel: "swaps" | "shifts";
  /** Upper bound for bar heights (the topic's max value). */
  maxValue: number;
};

const FALLBACK_WIDTH = 640;

/** The element's width, following resizes (a fixed fallback where layout is unknown). */
export function useWidth<T extends HTMLElement>() {
  const ref = useRef<T | null>(null);
  const [width, setWidth] = useState(FALLBACK_WIDTH);
  useLayoutEffect(() => {
    const w = ref.current?.getBoundingClientRect().width;
    if (w) setWidth(w);
  }, []);
  useEffect(() => {
    const el = ref.current;
    if (!el || typeof ResizeObserver === "undefined") return;
    const ro = new ResizeObserver((entries) => {
      const w = entries[0]?.contentRect.width;
      if (w) setWidth(w);
    });
    ro.observe(el);
    return () => ro.disconnect();
  }, []);
  return [ref, width] as const;
}

export const roleAt = (step: Step, pos: number): Role => step.roles[pos] ?? "idle";
export const roleColor = (role: Role) => `var(--viz-${role})`;

/** Spring for moves, a short ease for lift and colour; nothing when `instant` or reduced motion. */
export function useTransitions(instant: boolean) {
  const reduce = useReducedMotion();
  const none: Transition = { duration: 0 };
  if (instant || reduce) return { move: none, lift: none, colors: "none" };
  return {
    move: { type: "spring", stiffness: 320, damping: 32, mass: 0.9 } as Transition,
    lift: { duration: 0.22, ease: "easeOut" } as Transition,
    /** CSS transition for fills and strokes (motion cannot tween `var(--…)` colours). */
    colors: "fill 220ms ease, stroke 220ms ease",
  };
}

/** Outline that tells roles apart without colour: dashed while compared, solid while moving. */
export function outline(role: Role): { stroke?: string; strokeWidth?: number; strokeDasharray?: string } {
  if (role === "compare") return { stroke: "var(--foreground)", strokeWidth: 2, strokeDasharray: "5 3" };
  if (role === "swap") return { stroke: "var(--foreground)", strokeWidth: 2 };
  return {};
}

export function roleName(t: TFunction, role: Role, movesLabel: "swaps" | "shifts"): string {
  if (role === "swap" && movesLabel === "shifts") return t("learn.roles.shift");
  return t(`learn.roles.${role}`);
}

/** Position numbers under the scene. */
export function IndexRow({ n, slot, y }: { n: number; slot: number; y: number }) {
  return (
    <g aria-hidden className="fill-muted-foreground font-mono text-[11px]">
      {Array.from({ length: n }, (_, k) => (
        <text key={k} x={(k + 0.5) * slot} y={y} textAnchor="middle">
          {k}
        </text>
      ))}
    </g>
  );
}

/** Pointer labels (pivot, i, j…) that glide between positions; shared positions fan out. */
export function Pointers({
  pointers,
  slot,
  y,
  transition,
}: {
  pointers: Pointer[];
  slot: number;
  y: number;
  transition: Transition;
}) {
  const seen = new Map<number, number>();
  const counts = new Map<number, number>();
  for (const p of pointers) counts.set(p.at, (counts.get(p.at) ?? 0) + 1);
  return (
    <g aria-hidden>
      {pointers.map((p) => {
        const k = seen.get(p.at) ?? 0;
        seen.set(p.at, k + 1);
        const spread = ((counts.get(p.at) ?? 1) - 1) * 13;
        const x = (p.at + 0.5) * slot - spread + k * 26;
        return (
          <motion.g
            key={p.label}
            initial={false}
            animate={{ x }}
            transition={transition}
            style={{ color: `var(--viz-ptr-${p.tone})` }}
            data-pointer={p.label}
          >
            <path d={`M0 ${y} l5 8 h-10 z`} fill="currentColor" />
            <text y={y + 22} textAnchor="middle" fill="currentColor" className="font-mono text-xs font-semibold">
              {p.label}
            </text>
          </motion.g>
        );
      })}
    </g>
  );
}

/** One sentence for the scene's `aria-label`. */
export function sceneSummary(t: TFunction, step: Step): string {
  const values = step.items.map((it) => it.value).join(", ");
  const parts = [t("learn.stage.summary", { values })];
  const at = (role: Role) =>
    Object.entries(step.roles)
      .filter(([, r]) => r === role)
      .map(([p]) => Number(p));
  const compared = at("compare");
  if (compared.length) parts.push(t("learn.stage.summary_compare", { positions: compared.join(", ") }));
  const moving = at("swap");
  if (moving.length) parts.push(t("learn.stage.summary_move", { positions: moving.join(", ") }));
  const pivot = at("pivot")[0];
  if (pivot !== undefined) {
    parts.push(t("learn.stage.summary_pivot", { value: step.items[pivot]!.value, at: pivot }));
  }
  if (step.lifted !== null) {
    parts.push(t("learn.stage.summary_key", { value: step.items[step.lifted]!.value, at: step.lifted }));
  }
  if (step.range) parts.push(t("learn.stage.summary_range", { lo: step.range[0], hi: step.range[1] }));
  return parts.join(" ");
}

/** The scene as text, for screen readers (visually hidden). */
export function SceneFallback({
  t,
  step,
  movesLabel,
}: {
  t: TFunction;
  step: Step;
  movesLabel: "swaps" | "shifts";
}) {
  return (
    <div className="sr-only" data-testid="scene-fallback">
      <p>{t("learn.stage.fallback")}</p>
      <ol>
        {step.items.map((it, p) => (
          <li key={it.id}>
            {t("learn.stage.fallback_item", {
              at: p,
              value: it.value,
              role: roleName(t, roleAt(step, p), movesLabel),
            })}
          </li>
        ))}
      </ol>
    </div>
  );
}
