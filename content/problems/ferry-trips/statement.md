Hikers wait at a lake for a small rowing boat. The boat carries **at most two** hikers per trip, and their total weight must not exceed `limit`. Each hiker weighs no more than `limit`, so everyone can cross alone if needed.

Given the hikers' `weights`, return the fewest trips needed to take everyone across. (Count one-way trips with passengers; the boat's return is free.)

### Example 1

```
weights = [60, 45, 80, 35], limit = 100
Output: 3
```

One plan: 60 + 35, then 45 alone, then 80 alone. 80 cannot share with anyone, and only one pair fits among the rest.

### Example 2

```
weights = [50, 50, 50, 50], limit = 100
Output: 2
```

### Example 3

```
weights = [70, 90, 99], limit = 100
Output: 3
```

### Constraints

- 1 ≤ weights.length ≤ 10⁵
- 1 ≤ weights[i] ≤ limit ≤ 3·10⁴
