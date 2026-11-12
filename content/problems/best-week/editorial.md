## Approach: slide a fixed window

Adjacent windows share `k − 1` days, so there is no need to add them up again.

1. `window` = the sum of the first `k` days; `best = window`.
2. For `i = k … n − 1`: `window += sales[i] − sales[i − k]` and `best = max(best, window)`.

### Walkthrough

`sales = [12, 30, 7, 25, 18, 4]`, `k = 3`:

| window (days) | change | total | best |
| --- | --- | --- | --- |
| 0-2 | first sum | 49 | 49 |
| 1-3 | +25 −12 | 62 | 62 |
| 2-4 | +18 −30 | 50 | 62 |
| 3-5 | +4 −7 | 47 | 62 |

**Complexity:** O(n) time, O(1) extra space. Re-summing every window is O(n·k), up to 10¹⁰ additions for 2·10⁵ days.
