## The bug

The starter uses the classic `tails` method: `tails[k]` is the smallest height that can end a rising pick of length `k + 1`, and `tails` stays sorted. For each height it finds the first entry that `h` should replace, or appends if there is none. The search is the wrong one:

```python
i = bisect_right(tails, h)
```

`bisect_right` returns the position just after every entry equal to `h`. So when `h` is already in `tails`, it lands one place further and either replaces a bigger value or extends `tails`, as if `h` could follow an equal height. That computes the longest **non-decreasing** pick, but the climb must be strictly rising.

With `heights = [2, 2]`: the first 2 gives `tails = [2]`; for the second, `bisect_right([2], 2)` is 1, which equals the length, so it is appended and `tails = [2, 2]`. The code returns 2, but two equal heights cannot both be picked, so the answer is 1.

## The fix

Search for the first entry that is greater than **or equal to** `h`, so an equal height replaces its twin and never extends:

```python
from bisect import bisect_left

i = bisect_left(tails, h)
```

The TypeScript version writes its own binary search; change the comparison `tails[mid] <= h` to `tails[mid] < h` so it stops at the first entry not smaller than `h`.

**Complexity:** O(n log n) time, O(n) extra space for `tails`.
