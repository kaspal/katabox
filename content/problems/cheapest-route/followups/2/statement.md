Same towns and roads as before, but now the courier holds **one pass** that makes a single
road of their choice free: on that road the toll is 0. The pass may also go unused.
Return the cheapest total toll to drive from town `start` to town `goal`, or `-1` if
`goal` cannot be reached.

Each entry `[a, b, toll]` of `roads` is a road between towns `a` and `b` that can be
driven in both directions for a fee of `toll`; several roads may join the same two towns.
Driving from a town to itself costs 0.

### Example 1

```
n = 4, roads = [[0, 1, 5], [1, 2, 7], [2, 3, 2], [0, 2, 20]], start = 0, goal = 3
Output: 2
```

Use the pass on the expensive road 0 → 2, then pay 2 for 2 → 3. The route 0 → 1 → 2 → 3
costs 14, and still 7 with the pass on 1 → 2.

### Example 2

```
n = 5, roads = [[1, 0, 5], [0, 3, 8], [3, 4, 4], [0, 3, 1], [3, 4, 1], [3, 2, 4]], start = 4, goal = 0
Output: 1
```

### Example 3

```
n = 5, roads = [[0, 4, 2], [2, 1, 5], [2, 1, 1], [4, 1, 7], [3, 2, 9], [3, 2, 1]], start = 0, goal = 2
Output: 3
```

0 → 4 → 1 → 2 costs 2 + 7 + 1 = 10; the pass on 4 → 1 leaves 2 + 1 = 3.

### Constraints

- 1 ≤ n ≤ 2·10⁴
- 0 ≤ roads.length ≤ 5·10⁴
- roads[i] = [a, b, toll] with 0 ≤ a, b < n, a ≠ b and 1 ≤ toll ≤ 10⁴
- 0 ≤ start, goal < n
