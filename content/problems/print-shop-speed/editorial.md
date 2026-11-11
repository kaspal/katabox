## Approach: binary search on the answer

Let `need(s)` be the hours at speed `s`: the sum of ⌈p / s⌉ over all jobs. It never increases as `s` grows, and `need(max(pages)) = pages.length ≤ hours`. So the speeds that work are exactly `answer, answer + 1, …`, and we can binary search the boundary:

- `lo = 1`, `hi = max(pages)`.
- While `lo < hi`: let `mid = (lo + hi) / 2`. If `need(mid) ≤ hours`, the answer is at most `mid` (`hi = mid`); otherwise it is above (`lo = mid + 1`).

Compute ⌈p / s⌉ with integers as `(p + s - 1) / s`. The total can reach 5·10¹³, so use 64-bit integers.

### Walkthrough

`pages = [7, 3, 12]`, `hours = 6`:

| lo | hi | mid | need(mid) | move |
| --- | --- | --- | --- | --- |
| 1 | 12 | 6 | 2 + 1 + 2 = 5 | fits: hi = 6 |
| 1 | 6 | 3 | 3 + 1 + 4 = 8 | too slow: lo = 4 |
| 4 | 6 | 5 | 2 + 1 + 3 = 6 | fits: hi = 5 |
| 4 | 5 | 4 | 2 + 1 + 3 = 6 | fits: hi = 4 |

`lo = hi = 4`.

**Complexity:** O(n log M) time for n jobs and M = max(pages), O(1) extra space. Trying speeds 1, 2, 3, … is O(n·M) when the answer is large.
