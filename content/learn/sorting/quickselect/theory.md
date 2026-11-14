## The idea

To find the k-th smallest value you don't need to sort everything. Quickselect uses the same Lomuto partition as quick sort: after one partition, the pivot sits at its final position `p`. Here `k` counts from 1 but positions count from 0, so the answer is the value that would land at position `k − 1`. If `p` is exactly `k − 1`, the pivot is the answer. Otherwise the answer is on one side only: right of `p` when `p < k − 1`, left of it when `p > k − 1`. Quickselect drops the other side and partitions again. Since only one side is ever kept, the sample uses a loop that narrows `lo..hi` instead of a recursive call.

## Steps

1. Set `lo = 0` and `hi = len(a) - 1`.
2. Partition `lo..hi` with the last value as pivot: smaller values are swapped to the front, then the pivot goes to position `p`.
3. If `p == k - 1`, return `a[p]`.
4. If `p < k - 1`, keep the right side: `lo = p + 1`. Otherwise keep the left side: `hi = p - 1`.
5. Repeat from step 2.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n)` (the first pivot lands at `k − 1`) |
| Average | `O(n)`: if each pivot halves the range, `n + n/2 + n/4 + … < 2n` |
| Worst | `O(n²)` (every pivot is the smallest or biggest value left) |
| Extra space | `O(1)`: a loop, no recursion |
| Stable | No |
| In place | Yes: it reorders the array |

## Pitfalls

- **Off by one on k.** k is 1-based (the 1st smallest is the minimum) but positions are 0-based, so the target is `k − 1`. Comparing `p` with `k` returns the next value up.
- **It rearranges the input.** The array comes back partly sorted. Work on a copy if the original order matters.
- **Bad pivots.** With the last value as pivot, sorted or reversed input lets each partition remove only one value, and the work grows towards `n²`. A random pivot makes this very unlikely.

## When to use it

Use it for a median, a percentile, or the k-th largest score. For a single query it beats sorting: `O(n)` on average against `O(n log n)`. For many queries on the same data, sort once and read positions. Many libraries ship it, such as C++'s `nth_element` and NumPy's `partition`.

## Related topics

- [Quick Sort](learn:sorting/quick-sort): the same partition, recursing into both sides.
- [Quick Sort: Hoare and 3-way](learn:sorting/quick-sort-3way): other partitions that work here too.
- Binary search (coming soon): it also throws away one side at every step.
