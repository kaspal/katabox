At a food market, the entrance is at the origin (0, 0) and stall `i` stands at `stalls[i] = [x, y]`. A visitor with little time wants the `k` stalls closest to the entrance, by straight-line distance.

Return the indices of the `k` nearest stalls, nearest first. When two stalls are equally far, the one with the smaller index comes first (and wins a place among the `k`).

### Example 1

```
stalls = [[3, 4], [1, 1], [-2, 0], [0, -1], [5, 5]], k = 3
Output: [3, 1, 2]
```

The squared distances are 25, 2, 4, 1 and 50.

### Example 2

```
stalls = [[1, 0], [0, 1], [-1, 0]], k = 2
Output: [0, 1]
```

All three stalls are at distance 1; the smaller indices win.

### Example 3

```
stalls = [[7, -7]], k = 1
Output: [0]
```

### Constraints

- 1 ≤ k ≤ stalls.length ≤ 10⁵
- All coordinates are integers with absolute value at most 10⁴.
