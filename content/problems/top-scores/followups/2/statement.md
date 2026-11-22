This time the alley cares about **recent form**. `scores` lists the games in the order
they were played. For every window of `w` consecutive games, report the `k`-th highest
score inside that window. Equal scores each count, as before.

Return one value per window, from the window that starts at game 0 to the one that ends
at the last game: `scores.length − w + 1` values in all.

### Example 1

```
scores = [120, 200, 90, 250, 180], w = 3, k = 2
Output: [120, 200, 180]
```

The windows are [120, 200, 90], [200, 90, 250] and [90, 250, 180]; their 2nd highest
scores are 120, 200 and 180.

### Example 2

```
scores = [291, 32, 130, 60, 253, 230], w = 3, k = 1
Output: [291, 130, 253, 253]
```

### Example 3

```
scores = [46, 43, 184, 86, 157, 128], w = 3, k = 1
Output: [184, 184, 184, 157]
```

### Constraints

- 1 ≤ k ≤ w ≤ scores.length ≤ 10⁵
- 0 ≤ scores[i] ≤ 300
