## The idea

Merge sort splits the range into two halves, sorts each half the same way, and then **merges** the two sorted halves into one. The splitting stops at ranges of one value, which are already sorted. All the real work happens in `merge`: it copies `a[lo..hi]` into a `buffer`, walks the two halves with one index each, and every time writes the smaller of the two heads back into `a`.

## Steps

1. If the range `lo..hi` holds one value (`lo >= hi`), return.
2. Split at `mid = (lo + hi) // 2`; sort `lo..mid`, then `mid + 1..hi`.
3. Copy `a[lo..hi]` into the `buffer`. `i` starts at the left half's head, `j` at the right half's head.
4. For each position `k` from `lo` to `hi`, write the smaller head to `a[k]` and move that index on. On a tie, the left head wins.
5. When one half is used up, the rest of the other half follows in order.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n log n)` |
| Average | `O(n log n)` |
| Worst | `O(n log n)` (the split never depends on the values) |
| Extra space | `O(n)` for the buffer, plus `O(log n)` of stack |
| Stable | Yes: on a tie the left head goes first |
| In place | No |

## Pitfalls

- **Taking the right head on ties.** Comparing with `<` instead of `<=` lets an equal value from the right half jump ahead of one from the left, and the sort is no longer stable.
- **Forgetting the leftovers.** When one half runs out, the other still has values. The checks `j > hi - lo` (right half used up) and `i <= mid - lo` (left half not used up) handle both cases in one condition.
- **Mixing indices.** The buffer starts at 0, not at `lo`, so the right half starts at `mid + 1 - lo`. Confusing buffer and array positions is the classic merge bug.
- Every input costs about the same: each of the roughly `log₂ n` levels writes all `n` values; only the comparison count changes a little.

## When to use it

Use merge sort when you need a guaranteed `O(n log n)` or a stable sort, for example to sort records by one key and then by another. It is also the natural choice for linked lists, which can be merged without a buffer, and for data too big for memory, where sorted chunks on disk are merged.

## Related topics

- [Recursion and the call stack](learn:foundations/recursion-call-stack): the same halving split.
- [Tim Sort (simplified)](learn:sorting/tim-sort): insertion-sorted runs merged with this same `merge`.
- [Quick Sort](learn:sorting/quick-sort): does its work before the recursive calls instead of after, and needs no buffer.
