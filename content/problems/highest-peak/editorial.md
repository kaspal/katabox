## Approach: binary search on the slope

Compare a marker `m` with the next one:

- `heights[m] < heights[m + 1]`: the trail is climbing at `m`, so the summit is after `m`;
- otherwise the trail is descending from `m` (or `m` is the summit), so the summit is at `m` or before it.

Keep the summit inside `[lo, hi]`, starting with the whole trail. While `lo < hi`, check `m = ⌊(lo + hi) / 2⌋` (always `< hi`, so `m + 1` exists) and keep the half that holds the summit. When `lo = hi`, that is the summit.

### Walkthrough

`[1, 4, 9, 7, 2]`: lo = 0, hi = 4, m = 2: 9 > 7, so hi = 2. m = 1: 4 < 9, so lo = 2. The summit is 2.

**Complexity:** O(log n) time, O(1) space.
