## The idea

Tim sort, the sort behind Python's built-in sort and Java's sort for objects, mixes two strengths: insertion sort is fast on short ranges, and merging is fast on long sorted ones. This topic shows a **simplified** version. Insertion sort turns each block of `RUN = 4` values into a sorted **run**; then runs are merged in pairs, with the width doubling (4, 8, 16, …) until one run covers the array. The merge is the same buffered `merge` as in merge sort.

### What real Tim sort adds

This topic leaves all of these out:

- It **finds natural runs**: stretches that are already ascending (strictly descending ones are reversed) instead of cutting fixed blocks.
- It **chooses the minimum run length from `n`** (usually between 32 and 64) and extends short natural runs to it with insertion sort.
- It keeps a **stack of runs** and merges them by rules on their lengths, so merges stay balanced.
- It **gallops**: when one run keeps winning during a merge, it jumps ahead with a binary search instead of comparing one value at a time.

## Steps

1. Cut the array into blocks of `RUN = 4` (the last may be shorter) and sort each with insertion sort.
2. Set `width = RUN`.
3. For each pair of neighbouring runs `lo..mid` and `mid + 1..hi`, merge them through the buffer. A run with no partner waits.
4. Double `width` and repeat while `width < n`.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n)` for real Tim sort on sorted input; this version still merges, `O(n log n)` |
| Average | `O(n log n)` |
| Worst | `O(n log n)` |
| Extra space | `O(n)` for the buffer |
| Stable | Yes: insertion sort and the merge both keep ties in order |
| In place | No |

## Pitfalls

- **The leftover run.** With 10 values the runs are 0–3, 4–7 and 8–9. In the width-4 pass, 8–9 has no partner and waits; the width-8 pass merges it with 0–7. The loop bound `lo < n - width` is what skips it.
- **Fixed blocks miss free order.** Here sorted input still goes through every merge. Finding natural runs is what makes real Tim sort `O(n)` on it.
- **Ties.** If insertion sort or the merge lets a value pass an equal one, the sort is no longer stable.

## When to use it

You rarely write Tim sort yourself: it is what your language's library sort already does. Knowing it explains why sorting nearly sorted data is fast in Python and why library sorts are stable.

## Related topics

- [Insertion Sort](learn:sorting/insertion-sort): builds the runs.
- [Merge Sort](learn:sorting/merge-sort): the same `merge`.
- [Binary search](learn:searching/binary-search): what galloping uses.
