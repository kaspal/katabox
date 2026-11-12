A bike courier leaves the depot (stop `0`), delivers to every other stop exactly once, in any order, and comes back to the depot. Riding from stop `i` straight to stop `j` takes `dist[i][j]` minutes. The city has one-way streets, so `dist[i][j]` and `dist[j][i]` may differ, and a detour through another stop is never part of a ride: each ride goes straight to the next stop.

Return the fewest minutes the whole round can take. With only the depot, the round takes 0 minutes.

### Example 1

```
dist = [[0, 2, 9], [1, 0, 6], [15, 7, 0]]
Output: 17
```

The round 0 → 1 → 2 → 0 takes 2 + 6 + 15 = 23 minutes, but 0 → 2 → 1 → 0 takes 9 + 7 + 1 = 17.

### Example 2

```
dist = [[0]]
Output: 0
```

### Example 3

```
dist = [[0, 5], [3, 0]]
Output: 8
```

### Constraints

- 1 ≤ n ≤ 12, where `n = dist.length` and every `dist[i].length = n`
- dist[i][i] = 0, and 1 ≤ dist[i][j] ≤ 10⁴ for i ≠ j
