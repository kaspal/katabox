// Scene registry (ADR 0029): a tracer names its scene kind; the Stage draws that scene.
import type { ComponentType } from "react";
import type { SceneKind } from "../engine/types";
import { ArrayScene } from "./ArrayScene";
import { BarsScene } from "./BarsScene";
import { BoardScene } from "./BoardScene";
import { BitsScene } from "./BitsScene";
import { BucketsScene } from "./BucketsScene";
import type { SceneProps } from "./common";
import { DecisionScene } from "./DecisionScene";
import { DPTableScene } from "./DPTableScene";
import { GraphScene } from "./GraphScene";
import { GridScene } from "./GridScene";
import { GrowthScene } from "./GrowthScene";
import { HanoiScene } from "./HanoiScene";
import { BitArrayScene, HashScene } from "./HashScene";
import { HeapScene } from "./HeapScene";
import { ListScene } from "./ListScene";
import { MonoStackScene } from "./MonoStackScene";
import { RecursionScene } from "./RecursionScene";
import { SlotsScene } from "./SlotsScene";
import { StringScene } from "./StringScene";
import { TimelineScene } from "./TimelineScene";
import { TreeScene } from "./TreeScene";

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
  list: ListScene,
  slots: SlotsScene,
  tree: TreeScene,
  hash: HashScene,
  "bits-array": BitArrayScene,
  graph: GraphScene,
  grid: GridScene,
  dp: DPTableScene,
  timeline: TimelineScene,
  decision: DecisionScene,
  board: BoardScene,
  string: StringScene,
};
