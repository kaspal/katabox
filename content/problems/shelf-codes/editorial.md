## Approach: binary search for the first number ≥ x

Because `codes` is sorted, the numbers smaller than `x` are exactly the first few. So the answer is the index of the first element that is **not** smaller than `x`: the "lower bound" of `x`.

```
lo, hi = 0, len(codes)          # the answer is in [lo, hi]
while lo < hi:
    mid = (lo + hi) // 2
    if codes[mid] < x: lo = mid + 1   # mid and everything before it are smaller
    else: hi = mid                    # the first element ≥ x is at mid or before
return lo
```

Each round halves the range, so a query takes about log₂(10⁵) ≈ 17 comparisons. Repeated numbers need no special care: the loop moves past every copy smaller than `x` and stops at the first copy equal to it.

### Walkthrough

`codes = [3, 8, 8, 15, 20]`, `x = 16`: the range [0, 5] narrows with mid 2 (8 < 16, lo = 3), mid 4 (20 ≥ 16, hi = 4), mid 3 (15 < 16, lo = 4). Answer 4.

**Complexity:** O(q log n) time and O(1) extra space per query. Counting by scanning the whole list for every query is O(n · q).
