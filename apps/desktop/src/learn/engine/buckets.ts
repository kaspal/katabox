// Buckets shared by counting, radix and bucket sort (ADR 0029). A value keeps its identity:
// `drop` moves the item at a main-row position into a bucket (its slot shows empty, the id
// is `hidden`), `take` hands a bucket's first item back to position k. `items` stays a
// permutation: taking into k swaps the item's hidden twin into k.
import type { Recorder } from "./recorder";
import type { Bucket, Item, SceneData, Step } from "./types";

export class BucketBoard {
  readonly buckets: Item[][];

  constructor(
    private readonly r: Recorder,
    readonly labels: string[],
  ) {
    this.buckets = labels.map(() => []);
  }

  /** Drop the item at main position `pos` into bucket `b`. */
  drop(pos: number, b: number) {
    this.buckets[b]!.push(this.r.items[pos]!);
  }

  /** Move bucket `b`'s first item to main position `k`; returns it. */
  take(b: number, k: number): Item {
    const item = this.buckets[b]!.shift()!;
    const items = this.r.items;
    const p = items.findIndex((it) => it.id === item.id);
    const t = items[k]!;
    items[k] = items[p]!;
    items[p] = t;
    return item;
  }

  /** Reorder bucket `b` (bucket sort sorts each bucket); stable for equal values. */
  sortBucket(b: number) {
    this.buckets[b]!.sort((x, y) => x.value - y.value);
  }

  frame(active: number | null): SceneData {
    const buckets: Bucket[] = this.buckets.map((items, k) => ({ label: this.labels[k]!, items: items.map((it) => ({ ...it })) }));
    return { buckets, bucket: active, hidden: this.buckets.flat().map((it) => it.id) };
  }
}

/** Give every step the trace's fullest bucket, so the scene's height never jumps. */
export function fixBucketDepth(steps: Step[]): Step[] {
  const depth = Math.max(1, ...steps.flatMap((s) => (s.buckets ?? []).map((b) => b.items.length)));
  for (const s of steps) if (s.buckets) s.bucketDepth = depth;
  return steps;
}
