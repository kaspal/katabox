A mountain hut records the noon temperature of each day of the season in `temps`. Climbers want to know, for every day, how long they would have to wait for a **strictly warmer** noon.

Return an array `wait` of the same length, where `wait[i]` is the number of days from day `i` until the first later day with a higher temperature, or `0` if no later day is warmer.

### Example 1

```
temps = [3, 1, 4, 1, 5, 2]
Output: [2, 1, 2, 1, 0, 0]
```

Day 0 (3°) waits until day 2 (4°); day 4 (5°) is never beaten.

### Example 2

```
temps = [-5, -5, -2]
Output: [2, 1, 0]
```

An equal temperature is not warmer, so day 0 waits two days.

### Example 3

```
temps = [9, 8, 7]
Output: [0, 0, 0]
```

### Constraints

- 1 ≤ temps.length ≤ 10⁵
- −50 ≤ temps[i] ≤ 50
