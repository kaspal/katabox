A collector follows the price of a vintage camera on a second-hand market, one price per day, in `prices`. A **steep drop** is a pair of days, an earlier day `i` and a later day `j` (`i < j`, not necessarily next to each other), where the later price is less than half the earlier one: `prices[i] > 2 · prices[j]`.

Return the number of steep drops.

### Example 1

```
prices = [9, 4, 3, 10, 2]
Output: 4
```

9 → 4, 9 → 3, 9 → 2 and 10 → 2. The pair 4 → 2 is exactly half, which is not less than half.

### Example 2

```
prices = [1, 2, 3]
Output: 0
```

### Example 3

```
prices = [5, 2, 2]
Output: 2
```

### Constraints

- 1 ≤ prices.length ≤ 2·10⁵
- 1 ≤ prices[i] ≤ 10⁹
