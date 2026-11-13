A sports club often asks: "in how many ways can we pick a team of `k` players from a squad of `n`?" (The order of picking does not matter, so this is the binomial coefficient C(n, k); picking more players than the squad has is impossible: 0 ways.)

Answer every query `[n, k]` in `queries`. The answer can be huge, so return it **modulo 1 000 003** (a prime): the remainder after dividing by 1 000 003.

### Example 1

```
queries = [[5, 2], [6, 3], [4, 0]]
Output: [10, 20, 1]
```

### Example 2

```
queries = [[3, 5]]
Output: [0]
```

### Example 3

```
queries = [[40, 20]]
Output: [115282]
```

C(40, 20) = 137 846 528 820, whose remainder modulo 1 000 003 is 115 282.

### Constraints

- 1 ≤ queries.length ≤ 10⁵
- 0 ≤ n, k ≤ 10⁵
