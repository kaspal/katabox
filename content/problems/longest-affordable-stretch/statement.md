A long-distance hiking trail is split into stages, and `costs[i]` is what stage `i` costs (food, huts, transport). You want to walk **consecutive** stages without spending more than `budget` in total.

Return the largest number of consecutive stages whose total cost is at most `budget`. If even a single stage is too expensive everywhere, return `0`.

### Example 1

```
costs = [4, 2, 1, 6, 1, 1], budget = 8
Output: 3
```

Stages 0 to 2 cost 4 + 2 + 1 = 7, and stages 3 to 5 cost 6 + 1 + 1 = 8. No four consecutive stages fit.

### Example 2

```
costs = [9, 9, 9], budget = 5
Output: 0
```

### Example 3

```
costs = [0, 3, 0, 0], budget = 0
Output: 2
```

Stages that cost nothing always fit.

### Constraints

- 1 ≤ costs.length ≤ 5·10⁴
- 0 ≤ costs[i] ≤ 10⁴
- 0 ≤ budget ≤ 10⁹
