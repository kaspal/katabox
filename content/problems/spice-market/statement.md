A trader at a spice market can carry at most `capacity` grams home. Stall `i` sells a sack of spice weighing `weights[i]` grams for `prices[i]` coins in total, and lets buyers take **any part** of the sack: `x` grams cost `prices[i] · x / weights[i]` coins and are worth exactly that much back home.

Which spices should the trader take, and how much of each, to carry home the **greatest total value**? Return that value. Answers within 10⁻⁶ of the exact value are accepted.

### Example 1

```
weights = [10, 20, 30], prices = [60, 100, 120], capacity = 50
Output: 240
```

Per gram the spices are worth 6, 5 and 4 coins. The trader takes the first two sacks whole (30 grams, 160 coins) and 20 of the 30 grams of the third (80 coins).

### Example 2

```
weights = [5], prices = [10], capacity = 100
Output: 10
```

### Example 3

```
weights = [4, 8], prices = [10, 4], capacity = 1
Output: 2.5
```

### Constraints

- 1 ≤ weights.length ≤ 10⁵, prices.length = weights.length
- 1 ≤ weights[i], prices[i] ≤ 10⁴
- 1 ≤ capacity ≤ 10⁹
