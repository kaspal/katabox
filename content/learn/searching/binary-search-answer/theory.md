## The idea

Sometimes there is no sorted array to search, yet binary search still works: you search the range of possible **answers**. Take this task: parcels with weights `weights` must ship in their given order, a day's load cannot go over a capacity, and everything must ship within `days` days. What is the smallest capacity that works?

Answering "is capacity `c` enough?" is easy: `days_needed` fills each day greedily and counts the days. Finding the smallest good `c` directly is not. The key is that the yes/no answer is **monotone**: if capacity `c` is enough, any bigger capacity is too. Along the possible capacities the answers read no, no, …, no, yes, yes, …, yes, and binary search finds the first yes.

The range is easy to bound. The capacity must be at least the heaviest parcel (it has to fit in some day), and the total weight always works (everything ships on day one).

## Steps

1. Set `lo = max(weights)` and `hi = sum(weights)`.
2. While `lo < hi`, try `mid = (lo + hi) // 2`.
3. Count `days_needed(weights, mid)`.
4. If it is `<= days`, `mid` works and the answer is `mid` or smaller: `hi = mid`.
5. Otherwise `mid` is too small: `lo = mid + 1`.
6. When `lo == hi`, return `lo`: the smallest capacity that fits.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n log S)` |
| Average | `O(n log S)`: `log S` tries, each checking all `n` parcels |
| Worst | `O(n log S)` |
| Extra space | `O(1)` |

Here `S` is the width of the answer range, `sum − max`. Trying every capacity one by one would cost `O(n · S)`.

## Pitfalls

- **A check that is not monotone.** The method relies on "yes stays yes". If a bigger value can turn a yes back into a no, the search lands on a wrong answer without complaint.
- **A range that leaves the answer out.** Starting below `max(weights)` makes `days_needed` meaningless (a parcel that never fits). Make sure `hi` is always a yes.
- **Off by one at the end.** For "smallest that works", keep `mid` on a yes (`hi = mid`) and skip it on a no (`lo = mid + 1`). For "largest that works", mirror it, and round `mid` up so the loop cannot stall.

## When to use it

Reach for it whenever a task says "find the smallest (or largest) value such that…" and checking one candidate is simple: the slowest speed that still finishes on time, the longest piece you can cut a given number of times, the minimum largest sum when splitting a list into `k` parts. Write the check, prove it is monotone, bound the range, then halve.

## Related topics

- [Binary search](learn:searching/binary-search): the same halving, on a sorted array.
- [Lower and upper bound](learn:searching/lower-upper-bound): the first yes is a lower bound on a yes/no row.
- [Big-O growth curves](learn:foundations/big-o-growth): why `log S` tries beat `S` of them.
