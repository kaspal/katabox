This time the alley wants a **live** leaderboard. `scores` lists the games in the order
they were played. After each game, report the `k`-th highest score among all games so
far, or `-1` while fewer than `k` games have been played.

Equal scores each count, as before: after games `[200, 200, 90]` the 2nd highest score is
200. Return one value per game, in order.

### Example 1

```
scores = [120, 200, 90, 250, 180], k = 2
Output: [-1, 120, 120, 200, 200]
```

After the last game the scores so far, highest first, are 250, 200, 180, 120, 90, so the
2nd highest is 200.

### Example 2

```
scores = [68, 291, 32, 130, 60, 253], k = 3
Output: [-1, -1, 32, 68, 68, 130]
```

### Example 3

```
scores = [28, 46, 43, 184, 86, 157], k = 3
Output: [-1, -1, 28, 43, 46, 86]
```

### Constraints

- 1 ≤ k ≤ scores.length ≤ 10⁵
- 0 ≤ scores[i] ≤ 300
