// Topic id → tracer (ADR 0029). kb-pack checks every topic in content/learn has a file
// here; Vitest checks the registry matches the files.
import type { TracerDef } from "../engine/types";
import { dynamicArrayOps } from "./dynamic-array-ops";
import { stack } from "./stack";
import { queueRingBuffer } from "./queue-ring-buffer";
import { dequeMonotonicQueue } from "./deque-monotonic-queue";
import { singlyLinkedList } from "./singly-linked-list";
import { doublyLinkedList } from "./doubly-linked-list";
import { reverseList } from "./reverse-list";
import { floydCycle } from "./floyd-cycle";
import { lruCache } from "./lru-cache";
import { bigOGrowth } from "./big-o-growth";
import { bitManipulation } from "./bit-manipulation";
import { bubbleSort } from "./bubble-sort";
import { dynamicArray } from "./dynamic-array";
import { insertionSort } from "./insertion-sort";
import { prefixSums } from "./prefix-sums";
import { quickSort } from "./quick-sort";
import { recursionCallStack } from "./recursion-call-stack";
import { towersOfHanoi } from "./towers-of-hanoi";
import { linearSearch } from "./linear-search";
import { binarySearch } from "./binary-search";
import { lowerUpperBound } from "./lower-upper-bound";
import { binarySearchAnswer } from "./binary-search-answer";
import { exponentialSearch } from "./exponential-search";
import { twoPointers } from "./two-pointers";
import { slidingWindow } from "./sliding-window";
import { monotonicStack } from "./monotonic-stack";
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
  "linear-search": linearSearch,
  "binary-search": binarySearch,
  "lower-upper-bound": lowerUpperBound,
  "binary-search-answer": binarySearchAnswer,
  "exponential-search": exponentialSearch,
  "two-pointers": twoPointers,
  "sliding-window": slidingWindow,
  "monotonic-stack": monotonicStack,
  "dynamic-array-ops": dynamicArrayOps,
  "stack": stack,
  "queue-ring-buffer": queueRingBuffer,
  "deque-monotonic-queue": dequeMonotonicQueue,
  "singly-linked-list": singlyLinkedList,
  "doubly-linked-list": doublyLinkedList,
  "reverse-list": reverseList,
  "floyd-cycle": floydCycle,
  "lru-cache": lruCache,
};
