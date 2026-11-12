A scout leader has `budget` cents to buy snacks for a hike. The shop has one of each snack, and `prices[i]` is the price of snack `i` in cents.

Return the **largest number of snacks** the leader can buy without spending more than `budget`.

### Example 1

```
prices = [120, 35, 80, 60, 200], budget = 180
Output: 3
```

35 + 60 + 80 = 175 fits. No four snacks cost 180 or less.

### Example 2

```
prices = [50, 50, 50], budget = 49
Output: 0
```

### Example 3

```
prices = [10, 10, 10, 10], budget = 40
Output: 4
```

### Constraints

- 1 ≤ prices.length ≤ 10⁵
- 1 ≤ prices[i] ≤ 10⁴
- 0 ≤ budget ≤ 10⁹
