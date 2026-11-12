A bowling alley prints a "hall of fame" with the `k` best scores of the season. `scores` lists every game played, in the order they were played; the same score may appear many times.

Return the `k` highest scores, **highest first**. Equal scores each count, so a score can appear more than once in the answer.

### Example 1

```
scores = [132, 201, 87, 201, 156, 99], k = 3
Output: [201, 201, 156]
```

### Example 2

```
scores = [40, 75, 63], k = 3
Output: [75, 63, 40]
```

### Example 3

```
scores = [300, 12, 300, 12], k = 1
Output: [300]
```

### Constraints

- 1 ≤ k ≤ scores.length ≤ 10⁵
- 0 ≤ scores[i] ≤ 300
