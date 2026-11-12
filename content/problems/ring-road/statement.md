A valley has `n` villages, numbered `0` to `n − 1`, joined by exactly `n` two-way roads; `roads[i] = [a, b]` joins villages `a` and `b`. Every village can reach every other one. No road starts and ends at the same village, and no two roads join the same pair.

A connected road network with as many roads as villages contains **exactly one loop**: a round trip that passes through some villages once each and returns to its start without reusing a road. The council wants to turn that loop into a ring road. How many villages lie **on the loop**?

### Example 1

```
n = 5, roads = [[0, 1], [1, 2], [2, 0], [2, 3], [3, 4]]
Output: 3
```

The loop is 0 → 1 → 2 → 0. Villages 3 and 4 hang off village 2.

### Example 2

```
n = 4, roads = [[0, 1], [1, 2], [2, 3], [3, 0]]
Output: 4
```

### Example 3

```
n = 7, roads = [[0, 1], [1, 2], [2, 3], [3, 0], [2, 4], [4, 5], [0, 6]]
Output: 4
```

### Constraints

- 3 ≤ n ≤ 10⁵, roads.length = n
- `roads[i] = [a, b]` with 0 ≤ a, b < n and a ≠ b; no pair of villages is joined twice
- Every village can reach every other one
