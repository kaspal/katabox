## Approach: a min-heap of free times

The only question the foreman asks is "which lathe is free first?", and the answer changes after every job. A min-heap answers it in O(log m):

1. Push `min(lathes, n)` zeros: lathes beyond the number of jobs never matter.
2. For each job in order: pop the smallest free time `t`, the job ends at `t + job`, push it back.
3. The answer is the largest end time.

Because every lathe is identical, *which* lathe gets a job when several are free at once does not change any time.

### Walkthrough

`jobs = [5, 3, 8, 2, 4]`, `lathes = 2`, heap `{0, 0}`:

| job | pop | push | heap after | latest end |
| --- | --- | --- | --- | --- |
| 5 | 0 | 5 | {0, 5} | 5 |
| 3 | 0 | 3 | {3, 5} | 5 |
| 8 | 3 | 11 | {5, 11} | 11 |
| 2 | 5 | 7 | {7, 11} | 11 |
| 4 | 7 | 11 | {11, 11} | 11 |

**Complexity:** O(n log m) time and O(m) space, with m = min(lathes, n). Scanning all lathes for each job is O(n·m), up to 10¹⁰ steps.
