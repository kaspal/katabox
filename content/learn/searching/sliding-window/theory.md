## The idea

Find the longest run of neighbouring values whose sum is at most `limit`, in a list of **positive** values. Checking every run costs `O(n²)` or more. A sliding window keeps one run, `a[lo..hi]`, and its sum in `total`, and only ever moves its edges forward.

Each step, the right edge `hi` takes in one more value. If that pushes `total` over the limit, the left edge `lo` drops values until the window fits again. Then the window is a candidate: if it is longer than the best so far, remember its length and start. The sample returns `[best, best_start]`.

Why can the left edge never need to move back? Because all values are positive: if `a[lo..hi]` is over the limit, any run that starts at `lo` and ends further right is over it too. So `lo` will never again be a useful start once it has been dropped.

## Steps

1. Set `best = 0`, `best_start = 0`, `total = 0` and `lo = 0`.
2. For each `hi` from left to right, add `a[hi]` to `total`.
3. While `total > limit`, subtract `a[lo]` and move `lo` one place right.
4. If `hi - lo + 1 > best`, record `best = hi - lo + 1` and `best_start = lo`.
5. After the last value, return `[best, best_start]`.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n)` |
| Average | `O(n)` |
| Worst | `O(n)`: `hi` moves `n` times and `lo` at most `n` times, `2n` moves in all |
| Extra space | `O(1)`: two edges and a running sum |

The inner `while` loop looks like it could make this `O(n²)`, but it cannot: `lo` only moves forward, so across the whole run it moves at most `n` times.

## Pitfalls

- **Negative values (or zeros, for some tasks).** With a negative value, adding more on the right can bring an over-limit sum back down, so dropping `lo` early may throw away the best run. The method needs positive values; when values can be negative, use methods built on prefix sums instead.
- **Recomputing the sum.** Summing `a[lo..hi]` from scratch at each step brings back `O(n²)`. Keep `total` up to date: add on the right, subtract on the left.

## When to use it

Use it for "longest or shortest run such that…" questions where growing the run only makes things worse (or better) in one direction: the longest stretch within a budget, the shortest run reaching a total, the longest substring without repeated letters (with a count of letters in the window instead of a sum). Fixed-size windows, like the best total over any 7 days, are the simplest case: add one on the right, drop one on the left.

## Related topics

- [Two pointers](learn:searching/two-pointers): the same two indices, starting at opposite ends.
- [Prefix sums](learn:foundations/prefix-sums): range sums for any run, also with negative values.
- Monotonic queue (coming soon): the maximum inside a sliding window.
