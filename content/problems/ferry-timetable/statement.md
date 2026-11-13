An archipelago has `n` ports, numbered 0 to n − 1. Each ferry route `[a, b, minutes]` sails between ports `a` and `b` in either direction and takes `minutes`. Several routes may join the same two ports.

For each query `[from, to]`, return the shortest total sailing time from `from` to `to` (changing ferries is instant), `0` when `from = to`, or `-1` if `to` cannot be reached.

### Example 1

```
n = 4, routes = [[0, 1, 30], [1, 2, 20], [0, 2, 70], [2, 3, 10]], queries = [[0, 2], [3, 0], [1, 1]]
Output: [50, 60, 0]
```

From 0 to 2, going through 1 (30 + 20) beats the direct route (70).

### Example 2

```
n = 3, routes = [[0, 1, 5]], queries = [[0, 2], [1, 0]]
Output: [-1, 5]
```

### Example 3

```
n = 2, routes = [[0, 1, 9], [1, 0, 4]], queries = [[0, 1]]
Output: [4]
```

### Constraints

- 1 ≤ n ≤ 100
- 0 ≤ routes.length ≤ 5000, with 0 ≤ a, b < n, a ≠ b and 1 ≤ minutes ≤ 10⁴
- 1 ≤ queries.length ≤ 10⁴, with 0 ≤ from, to < n
