Same towns and roads as before, but now the courier may drive **at most `k` roads** on the
way. Return the cheapest total toll to get from town `start` to town `goal` using at most
`k` roads, or `-1` if `goal` cannot be reached within that many roads.

Each entry `[a, b, toll]` of `roads` is a road between towns `a` and `b` that can be
driven in both directions for a fee of `toll`; several roads may join the same two towns.
A road driven twice counts (and is paid) twice. If `start == goal`, the answer is 0.

### Example 1

```
n = 4, roads = [[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 10]], start = 0, goal = 3, k = 2
Output: 10
```

0 → 1 → 2 → 3 costs only 3 but uses three roads; with at most two, the direct road (10)
is the only way.

### Example 2

```
n = 5, roads = [[1, 0, 5], [0, 3, 8], [3, 4, 4], [0, 3, 1], [3, 4, 1], [3, 2, 4]], start = 4, goal = 0, k = 2
Output: 2
```

### Example 3

```
n = 5, roads = [[0, 4, 2], [2, 1, 5], [2, 1, 1], [4, 1, 7], [3, 2, 9], [3, 2, 1]], start = 0, goal = 2, k = 2
Output: -1
```

The only route is 0 → 4 → 1 → 2, which needs three roads.

### Constraints

- 1 ≤ n ≤ 1000
- 0 ≤ roads.length ≤ 10⁴
- roads[i] = [a, b, toll] with 0 ≤ a, b < n, a ≠ b and 1 ≤ toll ≤ 10⁴
- 0 ≤ start, goal < n
- 1 ≤ k ≤ 50
