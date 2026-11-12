Lucía is choosing an outfit for a school play from the costume shop. The shop groups its items by category (hats, jackets, shoes, …): `prices[c]` lists the prices of the items in category `c`.

An outfit takes **exactly one** item from **every** category. How many different outfits cost **at most** `budget` in total? Two outfits are different if they differ in at least one item, even if the items have the same price.

### Example 1

```
prices = [[30, 50], [20, 40, 60], [10]], budget = 100
Output: 5
```

Of the 2 · 3 · 1 = 6 outfits, only 50 + 60 + 10 = 120 is over the budget.

### Example 2

```
prices = [[500], [600]], budget = 1000
Output: 0
```

### Example 3

```
prices = [[1, 2, 3]], budget = 2
Output: 2
```

### Constraints

- 1 ≤ prices.length ≤ 6
- 1 ≤ prices[c].length ≤ 6
- 1 ≤ prices[c][i] ≤ 1000
- 1 ≤ budget ≤ 6000
