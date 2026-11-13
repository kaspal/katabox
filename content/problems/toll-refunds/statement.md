A region has `n` towns, numbered 0 to n − 1, joined by one-way toll roads. Road `[a, b, cost]` goes from `a` to `b`; its `cost` is what you pay, and it is **negative** when the region refunds drivers for using it.

From the town `start`, find the cheapest total cost to reach every town:

- the cost to reach `start` itself is 0;
- a town you cannot reach gets `1000000000`;
- if you can reach a loop of roads whose total cost is negative (so driving around it forever would make any cost arbitrarily low), return an **empty list** instead.

Return the list of the n costs.

### Example 1

```
n = 4, roads = [[0, 1, 4], [0, 2, 5], [2, 1, -3], [1, 3, 2]], start = 0
Output: [0, 2, 5, 4]
```

Going 0 → 2 → 1 costs 5 − 3 = 2, less than the direct road to 1.

### Example 2

```
n = 3, roads = [[0, 1, 1], [1, 2, -2], [2, 1, 1]], start = 0
Output: []
```

The loop 1 → 2 → 1 costs −1 and can be reached from 0.

### Example 3

```
n = 3, roads = [[1, 2, -5]], start = 0
Output: [0, 1000000000, 1000000000]
```

### Constraints

- 1 ≤ n ≤ 500
- 0 ≤ roads.length ≤ 3000, with 0 ≤ a, b < n and a ≠ b
- -1000 ≤ cost ≤ 1000
- 0 ≤ start < n
