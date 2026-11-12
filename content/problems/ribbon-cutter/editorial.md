## Approach: binary search on the answer

Let `count(L) = Σ ⌊r / L⌋` over the rolls. Making pieces longer never gives more of them, so `count` is non-increasing in `L`, and the lengths with `count(L) ≥ pieces` are exactly `1 … answer`.

1. If `count(1) = Σ r < pieces`, return 0.
2. Otherwise binary search in `[1, max(ribbons)]` for the last `L` with `count(L) ≥ pieces`:
   - `lo = 1` (works), `hi = max(ribbons)`;
   - while `lo < hi`: `mid = (lo + hi + 1) / 2`; if `count(mid) ≥ pieces` then `lo = mid` else `hi = mid − 1`.

### Walkthrough

`ribbons = [30, 47, 18]`, `pieces = 6`:

| lo | hi | mid | count(mid) | move |
| --- | --- | --- | --- | --- |
| 1 | 47 | 24 | 1 + 1 + 0 = 2 | hi = 23 |
| 1 | 23 | 12 | 2 + 3 + 1 = 6 | lo = 12 |
| 12 | 23 | 18 | 1 + 2 + 1 = 4 | hi = 17 |
| 12 | 17 | 15 | 2 + 3 + 1 = 6 | lo = 15 |
| 15 | 17 | 16 | 1 + 2 + 1 = 4 | hi = 15 |

Answer 15.

**Complexity:** O(n log M) time with M the longest roll, O(1) extra space. Trying lengths one by one from the longest roll down is O(n·M).
