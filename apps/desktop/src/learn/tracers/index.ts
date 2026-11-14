// Topic id → tracer (ADR 0029). kb-pack checks every topic in content/learn has a file
// here; Vitest checks the registry matches the files.
import type { TracerDef } from "../engine/types";
import { bubbleSort } from "./bubble-sort";
import { insertionSort } from "./insertion-sort";
import { quickSort } from "./quick-sort";

export const TRACERS: Record<string, TracerDef> = {
  "bubble-sort": bubbleSort,
  "insertion-sort": insertionSort,
  "quick-sort": quickSort,
};
