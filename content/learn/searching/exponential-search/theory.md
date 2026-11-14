## The idea

Binary search needs both ends of the range before it starts. Exponential search first finds a small range that must hold the target, then runs binary search only inside it. It checks `a[0]`, then probes positions 1, 2, 4, 8, … (doubling a `bound` each time) while the value there is still smaller than the target. As soon as `a[bound]` is not smaller (or `bound` runs past the end), the target can only be between the previous bound, `bound // 2`, and `bound`.

Since the bound doubles, reaching a target at position `i` takes about `log₂ i` probes, and the range left over is at most `i` positions wide. The whole search costs `O(log i)`, which depends on where the target is, not on how long the list is. A target near the front is found fast even in a huge list.

## Steps

1. If `a[0] == target`, return `0`.
2. Set `bound = 1`.
3. While `bound < len(a)` and `a[bound] < target`, double it: `bound *= 2`.
4. The target can only be in `lo = bound // 2` to `hi = min(bound, len(a) - 1)`.
5. Run binary search on `lo..hi`: return the position if found, `-1` if the range becomes empty.

## Cost

| Case | Cost |
|---|---|
| Best | `O(1)` (the target is at position 0) |
| Average | `O(log i)`: `i` is the target's position |
| Worst | `O(log n)` (the target is near the end, or missing) |
| Extra space | `O(1)` |

In the worst case it does roughly twice the checks of a plain binary search: one round to find the range, another to search it.

## Pitfalls

- **Running past the end.** The doubling can overshoot `len(a) - 1`. Test `bound < len(a)` before reading `a[bound]`, and clamp `hi` with `min`.
- **Starting the doubling at 0.** `0 * 2` is still 0, so the loop would never move. That is why `a[0]` is checked on its own and `bound` starts at 1.
- **Searching too wide a range.** The previous bound already held a value smaller than the target, so the range starts at `bound // 2`, not at 0. Starting at 0 still works, but loses the `O(log i)` cost.

## When to use it

Use it when the end of the data is unknown or very far away: a sorted stream you can read by position, a huge sorted file, or a function that grows with its input and has no clear upper limit. It also pays off when targets tend to sit near the front. Tim sort's galloping mode uses the same doubling to skip long runs while merging.

## Related topics

- [Binary search](learn:searching/binary-search): the second phase, on a known range.
- [Linear search](learn:searching/linear-search): checks positions one by one instead of doubling.
- [Tim Sort (simplified)](learn:sorting/tim-sort): galloping doubles in the same way.
- [Amortised cost: the dynamic array](learn:foundations/dynamic-array): another place where doubling keeps a cost low.
