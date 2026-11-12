A woodworking shop has `lathes` identical lathes, all free at minute 0. The day's jobs wait in a queue: job `i` takes `jobs[i]` minutes. The foreman hands out the jobs **in queue order**, each one to the lathe that becomes free **first** (any of them when several free up at the same minute). A lathe works on one job at a time, without pauses.

Return the minute at which the last job is finished.

### Example 1

```
jobs = [5, 3, 8, 2, 4], lathes = 2
Output: 11
```

| job | minutes | lathe (free at) | ends |
| --- | --- | --- | --- |
| 0 | 5 | A (0) | 5 |
| 1 | 3 | B (0) | 3 |
| 2 | 8 | B (3) | 11 |
| 3 | 2 | A (5) | 7 |
| 4 | 4 | A (7) | 11 |

### Example 2

```
jobs = [7, 7, 7], lathes = 5
Output: 7
```

### Example 3

```
jobs = [4, 1, 1, 1, 1, 1], lathes = 2
Output: 5
```

One lathe turns the 4-minute job while the other does four short ones; both free up at minute 4, and the last job ends at 5.

### Constraints

- 1 ≤ jobs.length ≤ 10⁵
- 1 ≤ jobs[i] ≤ 10⁴
- 1 ≤ lathes ≤ 10⁵
