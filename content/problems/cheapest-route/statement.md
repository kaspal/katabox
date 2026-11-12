A courier company covers `n` towns, numbered `0` to `n − 1`. Each entry `[a, b, toll]` of `roads` is a road between towns `a` and `b` that can be driven in both directions for a fee of `toll`. Several roads may join the same two towns.

Return the cheapest total toll to drive from town `start` to town `goal`, or `-1` if `goal` cannot be reached. Driving from a town to itself costs 0.

### Example 1

```
n = 4, roads = [[0, 1, 5], [1, 3, 5], [0, 2, 2], [2, 3, 6], [1, 2, 1]], start = 0, goal = 3
Output: 8
```

0 → 2 → 3 costs 2 + 6 = 8, cheaper than 0 → 1 → 3 (10). The route 0 → 2 → 1 → 3 also costs 2 + 1 + 5 = 8.

### Example 2

```
n = 3, roads = [[0, 1, 4]], start = 0, goal = 2
Output: -1
```

### Example 3

```
n = 2, roads = [[1, 0, 7], [0, 1, 3]], start = 1, goal = 1
Output: 0
```

### Constraints

- 1 ≤ n ≤ 2·10⁴
- 0 ≤ roads.length ≤ 5·10⁴
- roads[i] = [a, b, toll] with 0 ≤ a, b < n, a ≠ b and 1 ≤ toll ≤ 10⁴
- 0 ≤ start, goal < n
