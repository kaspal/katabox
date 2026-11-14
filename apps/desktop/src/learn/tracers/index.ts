// Topic id → tracer (ADR 0029). kb-pack checks every topic in content/learn has a file
// here; Vitest checks the registry matches the files.
import type { TracerDef } from "../engine/types";
import { bigOGrowth } from "./big-o-growth";
import { bitManipulation } from "./bit-manipulation";
import { bubbleSort } from "./bubble-sort";
import { dynamicArray } from "./dynamic-array";
import { insertionSort } from "./insertion-sort";
import { prefixSums } from "./prefix-sums";
import { quickSort } from "./quick-sort";
import { recursionCallStack } from "./recursion-call-stack";
import { towersOfHanoi } from "./towers-of-hanoi";

export const TRACERS: Record<string, TracerDef> = {
  "big-o-growth": bigOGrowth,
  "recursion-call-stack": recursionCallStack,
  "towers-of-hanoi": towersOfHanoi,
  "bit-manipulation": bitManipulation,
  "prefix-sums": prefixSums,
  "dynamic-array": dynamicArray,
  "bubble-sort": bubbleSort,
  "insertion-sort": insertionSort,
  "quick-sort": quickSort,
};
