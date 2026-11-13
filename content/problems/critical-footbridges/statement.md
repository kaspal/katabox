A city of canals has `n` islands, numbered 0 to n − 1, joined by footbridges. Footbridge `[a, b]` connects islands `a` and `b` both ways; no two footbridges join the same pair of islands.

A footbridge is **critical** when closing it would leave some pair of islands that could reach each other before unable to reach each other now. The council wants to inspect those first.

Return the critical footbridges, each written as `[a, b]` with `a < b`, sorted by `a` and then by `b`.

### Example 1

```
n = 6, bridges = [[0, 1], [1, 2], [2, 3], [3, 1], [3, 4], [4, 5]]
Output: [[0, 1], [3, 4], [4, 5]]
```

The loop 1–2–3 survives closing any one of its footbridges.

### Example 2

```
n = 4, bridges = [[0, 1], [1, 2], [2, 3], [3, 0]]
Output: []
```

### Example 3

```
n = 3, bridges = [[2, 1]]
Output: [[1, 2]]
```

Island 0 has no footbridges at all.

### Constraints

- 1 ≤ n ≤ 10⁵
- 0 ≤ bridges.length ≤ 2·10⁵
- 0 ≤ a, b < n, a ≠ b, and no pair is joined twice.
