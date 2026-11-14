// The buffered merge shared by merge sort and Tim sort (ADR 0029). It copies a[lo..hi]
// into a buffer and writes the smaller head back each time, exactly as the samples'
// `merge`, and hands the tracer one event per step with the scene's frame: the buffer row
// (`aux`, sharing identity with the main row), the ids in it (`hidden`), the two runs.
// `items` stays a permutation: writing a buffered value to position k swaps its hidden
// main-row twin into k, and the value that was hidden at k moves to the twin's old spot.
import type { Recorder } from "./recorder";
import type { AuxRow, Item, Pointer, Role, SceneData } from "./types";

export type MergeEvent =
  | { kind: "copy" }
  | { kind: "pick"; left: number | null; right: number | null; takeLeft: boolean }
  | { kind: "take"; side: "left" | "right"; value: number };

export type MergeFrame = SceneData & {
  roles: Record<number, Role>;
  pointers: Pointer[];
  range: [number, number];
  vars: { i: number; j: number; k: number | null };
};

export function bufferedMerge(
  r: Recorder,
  lo: number,
  mid: number,
  hi: number,
  emit: (e: MergeEvent, frame: MergeFrame) => void,
) {
  const buffer: (Item | null)[] = r.items.slice(lo, hi + 1).map((it) => it);
  const left = mid - lo; // last buffer index of the left run
  const last = hi - lo; // last buffer index
  let i = 0;
  let j = mid + 1 - lo;
  const frame = (k: number | null, wrote: number | null): MergeFrame => {
    const auxRoles: Record<number, Role> = {};
    if (i <= left) auxRoles[i] = "compare";
    if (j <= last) auxRoles[j] = "compare";
    const aux: AuxRow = { label: "buffer", cells: buffer.map((c) => (c ? { ...c } : null)), roles: auxRoles, offset: lo };
    return {
      aux,
      hidden: buffer.filter((c): c is Item => c !== null).map((c) => c.id),
      runs: [
        [lo, mid],
        [mid + 1, hi],
      ],
      roles: wrote === null ? {} : { [wrote]: "swap" },
      pointers: k === null ? [] : [{ label: "k", at: k, tone: "key" }],
      range: [lo, hi],
      vars: { i, j, k },
    };
  };

  emit({ kind: "copy" }, frame(null, null));
  for (let k = lo; k <= hi; k++) {
    const leftDone = i > left;
    const rightDone = j > last;
    const takeLeft = rightDone || (!leftDone && buffer[i]!.value <= buffer[j]!.value);
    if (!leftDone && !rightDone) r.comparisons++;
    emit(
      {
        kind: "pick",
        left: leftDone ? null : buffer[i]!.value,
        right: rightDone ? null : buffer[j]!.value,
        takeLeft,
      },
      frame(k, null),
    );
    const from = takeLeft ? i : j;
    const item = buffer[from]!;
    buffer[from] = null;
    // Write it at k: swap its hidden twin into place.
    const p = r.items.findIndex((it) => it.id === item.id);
    const t = r.items[k]!;
    r.items[k] = r.items[p]!;
    r.items[p] = t;
    r.writes++;
    r.moves++;
    // The step shows the write line, before i or j moves on.
    emit({ kind: "take", side: takeLeft ? "left" : "right", value: item.value }, frame(k, k));
    if (takeLeft) i++;
    else j++;
  }
}
