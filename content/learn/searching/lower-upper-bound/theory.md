## The idea

Plain binary search stops at any copy of the target. Often you need more: where do the copies start, and where do they end? Two closely related searches answer that on sorted values.

- The **lower bound** is the first position whose value is `≥ target`.
- The **upper bound** is the first position whose value is `> target`.

The copies of the target sit exactly between them, so `upper − lower` is how many there are. If the target is missing, both bounds land on the same spot: the place where it would be inserted to keep the order. Either bound can be `n`, one past the end, when every value is smaller.

Both searches work on a **half-open** range `[lo, hi)`: `lo` is included, `hi` is not, and `hi` starts at `len(a)` so that `n` is a possible answer. Instead of stopping on a match, they keep shrinking until `lo == hi`, and that single spot is the answer.

## Steps

1. Set `lo = 0` and `hi = len(a)`.
2. While `lo < hi`, take `mid = (lo + hi) // 2`.
3. Lower bound: if `a[mid] < target`, the answer is to the right, so `lo = mid + 1`; otherwise `mid` might be the answer, so `hi = mid` (keep it).
4. Upper bound: the same, but test `a[mid] <= target`.
5. When `lo == hi`, return `lo`.
6. The number of copies is `upper − lower`.

## Cost

| Case | Cost |
|---|---|
| Best | `O(log n)`: no early exit, every search narrows to one spot |
| Average | `O(log n)` |
| Worst | `O(log n)`, twice: one search for each bound |
| Extra space | `O(1)` |

## Pitfalls

- **`hi = mid - 1`.** In this form `mid` may be the answer, so it has to stay in the range: `hi = mid`. Dropping it skips the very position you were looking for.
- **`hi = len(a) - 1`.** The answer can be `n` (all values smaller than the target). Starting `hi` at the last index makes that answer impossible.
- **Reading `a[lower]` without checking.** A lower bound of `n` is not a valid index, and a valid index may still hold a bigger value. The target is present only if `lower < n` and `a[lower] == target`.
- **Mixing up `<` and `<=`.** That one character is the only difference between the two bounds.

## When to use it

Use the bounds to count copies of a value, to find the first or last occurrence, to count values inside a range `[x, y]` (`lower(y + 1) − lower(x)` for integers) or to find where to insert a value and keep a list sorted. Python's `bisect_left` and `bisect_right` and C++'s `lower_bound` and `upper_bound` are exactly these two searches.

## Related topics

- [Binary search](learn:searching/binary-search): stops at any match.
- [Binary search on the answer](learn:searching/binary-search-answer): the same "keep `mid`" shape, on a range of answers.
- [Insertion Sort](learn:sorting/insertion-sort): a lower or upper bound tells where each key goes.
