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
import { selectionSort } from "./selection-sort";
import { shellSort } from "./shell-sort";
import { mergeSort } from "./merge-sort";
import { quickSort3Way } from "./quick-sort-3way";
import { heapSort } from "./heap-sort";
import { countingSort } from "./counting-sort";
import { radixSort } from "./radix-sort";
import { bucketSort } from "./bucket-sort";
import { timSort } from "./tim-sort";
import { quickselect } from "./quickselect";

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
  "selection-sort": selectionSort,
  "shell-sort": shellSort,
  "merge-sort": mergeSort,
  "quick-sort-3way": quickSort3Way,
  "heap-sort": heapSort,
  "counting-sort": countingSort,
  "radix-sort": radixSort,
  "bucket-sort": bucketSort,
  "tim-sort": timSort,
  "quickselect": quickselect,
};
