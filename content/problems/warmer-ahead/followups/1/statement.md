This time the hut's records repeat every year: after the last day of `temps` comes the first day again. For every day, count how many days you walk forward, wrapping around from the end to the start, until the first **strictly warmer** noon.

Return an array `wait` of the same length, where `wait[i]` is that number of days (at most `temps.length − 1`), or `0` if no other day of the season is warmer than day `i`.

### Example 1

```
temps = [4, 2, 9, 1, 3]
Output: [2, 1, 0, 1, 1]
```

Day 4 (3°) wraps around to day 0 (4°), one day later; day 2 (9°) is never beaten.

### Example 2

```
temps = [-5, 9, -7, -1, -6, 6]
Output: [1, 0, 1, 2, 1, 2]
```

### Example 3

```
temps = [-8, -7, -7, 2, -4, 0]
Output: [1, 2, 1, 0, 1, 4]
```

Day 5 (0°) wraps past days 0, 1 and 2 to day 3 (2°): 4 days. Day 1 waits 2 days, because day 2 is equally cold, not warmer.

### Constraints

- 1 ≤ temps.length ≤ 10⁵
- −50 ≤ temps[i] ≤ 50
