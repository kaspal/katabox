// The Learn engine's step format (ADR 0029). A tracer turns an input into every step up
// front, so playing, stepping and scrubbing only index into an array.
import type en from "@/i18n/locales/en.json";

/** A value with a stable identity: scenes key on `id`, so equal values still slide. */
export type Item = { id: number; value: number };

/** How a position is drawn. Every role has a legend label; colour is never the only cue. */
export type Role = "idle" | "out" | "pivot" | "compare" | "swap" | "sorted" | "key";

export type PointerTone = "pivot" | "i" | "j" | "key";
export type Pointer = { label: string; at: number; tone: PointerTone };

/** `moves` are swaps or shifts, depending on the topic (`TracerDef.movesLabel`). */
export type Counters = { comparisons: number; moves: number; writes: number };

type Narrations = (typeof en)["learn"]["narration"];
export type TopicId = keyof Narrations;
/** A narration key of one topic, typed from the English locale. */
export type NarrationKey<T extends TopicId> = `${T}.${keyof Narrations[T] & string}`;
export type Narration = { key: string; params?: Record<string, string | number> };

/** A variable shown in the variables grid. */
export type VarValue = number | boolean | null;

/** One frame. Snapshots, not diffs: scrubbing is O(1). */
export type Step = {
  items: Item[];
  /** Roles by position; positions not listed are `idle`. */
  roles: Record<number, Role>;
  /** The position whose item is lifted above the row (insertion's key), or null. */
  lifted: number | null;
  pointers: Pointer[];
  /** The active range (inclusive positions), or null. */
  range: [number, number] | null;
  counters: Counters;
  narration: Narration;
  /** The `@step` id of the sample line behind this step, or null (start, end). */
  marker: string | null;
  vars: Record<string, VarValue>;
};

export type TracerDef = {
  id: TopicId;
  /** The subject folder in content/learn. */
  subject: string;
  scene: "bars" | "array";
  /** Every `@step` id the tracer emits; both samples must carry each once. */
  markers: readonly string[];
  /** Every narration key the tracer can emit (full keys under `learn.narration`). */
  narrationKeys: readonly string[];
  movesLabel: "swaps" | "shifts";
  legend: readonly Role[];
  vars: readonly string[];
  trace: (values: readonly number[]) => Step[];
};

/** The most steps a trace may hold; inputs that need more are refused up front. */
export const MAX_STEPS = 2000;
