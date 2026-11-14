// Scene registry (ADR 0029): a tracer names its scene kind; the Stage draws that scene.
import type { ComponentType } from "react";
import type { SceneKind } from "../engine/types";
import { ArrayScene } from "./ArrayScene";
import { BarsScene } from "./BarsScene";
import { BitsScene } from "./BitsScene";
import { BucketsScene } from "./BucketsScene";
import type { SceneProps } from "./common";
import { GrowthScene } from "./GrowthScene";
import { HanoiScene } from "./HanoiScene";
import { HeapScene } from "./HeapScene";
import { MonoStackScene } from "./MonoStackScene";
import { RecursionScene } from "./RecursionScene";

export const SCENES: Record<SceneKind, ComponentType<SceneProps>> = {
  bars: BarsScene,
  array: ArrayScene,
  growth: GrowthScene,
  recursion: RecursionScene,
  hanoi: HanoiScene,
  bits: BitsScene,
  buckets: BucketsScene,
  merge: ArrayScene,
  heap: HeapScene,
  monostack: MonoStackScene,
};
