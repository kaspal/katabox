## The idea

When values are sorted, one comparison tells you far more than "yes" or "no". Look at the middle value: if it is smaller than the target, the target cannot be anywhere to its left, because everything there is smaller still. So half the range is ruled out at once. Binary search keeps a range `lo..hi` that must contain the target if it is there at all, checks its middle position `mid`, and throws away the half that cannot hold it. Each check halves the range, so a million values need about 20 checks.

The sample returns the position where it meets the target, or `-1` when the range becomes empty. With duplicates, that is **some** matching position, not necessarily the first one; [Lower and upper bound](learn:searching/lower-upper-bound) finds the first.

## Steps

1. Set `lo = 0` and `hi = len(a) - 1`: the whole array.
2. While `lo <= hi`, take `mid = (lo + hi) // 2`.
3. If `a[mid] == target`, return `mid`.
4. If `a[mid] < target`, the target is to the right: `lo = mid + 1`.
5. Otherwise it is to the left: `hi = mid - 1`.
6. When `lo > hi`, the range is empty: return `-1`.

## Cost

| Case | Cost |
|---|---|
| Best | `O(1)` (the first middle is the target) |
| Average | `O(log n)` |
| Worst | `O(log n)`: about `log₂ n + 1` checks, then the range is empty |
| Extra space | `O(1)`: three indices |

## Pitfalls

- **Unsorted input.** On values out of order the answer is simply wrong, with no error. Sort first, or check the order if the data comes from outside.
- **Moving a bound to `mid` instead of past it.** With `lo <= hi`, writing `lo = mid` can repeat the same `mid` forever. `mid` was already checked, so step past it.
- **Overflow in the middle.** In languages with fixed-size integers, `lo + hi` can overflow on huge arrays; `lo + (hi - lo) / 2` avoids it. Python and JavaScript are safe here.
- **Expecting the first copy.** On `[2, 5, 5, 5, 9]` with target 5, the first middle already hits a 5 and returns 2, not 1.

## When to use it

Use it for lookups in sorted data that does not change often: a sorted list of IDs, timestamps in a log, a table of thresholds. The real power is wider: whenever a yes/no question flips only once along an ordered range, the same halving finds the flip point, which is what [Binary search on the answer](learn:searching/binary-search-answer) does.

## Related topics

- [Linear search](learn:searching/linear-search): no order needed, `O(n)` checks.
- [Lower and upper bound](learn:searching/lower-upper-bound): the first and last place of a value.
- [Binary search on the answer](learn:searching/binary-search-answer): halving a range of possible answers.
- [Exponential search](learn:searching/exponential-search): binary search when the end is far or unknown.
- [Recursion and the call stack](learn:foundations/recursion-call-stack): binary search can also be written recursively.
