Three cousins want to buy their grandmother **three different gifts** from a shop window, and they would like the total to be as close as possible to the `target` they collected. `prices[i]` is the price of gift `i`; two gifts may have the same price.

Return the total of the three gifts whose sum is **closest** to `target`. If two totals are equally close, one above and one below the target, return the **smaller** one.

### Example 1

```
prices = [12, 40, 7, 25, 33], target = 70
Output: 70
```

12 + 25 + 33 = 70 hits the target exactly.

### Example 2

```
prices = [5, 9, 14, 30], target = 20
Output: 28
```

The cheapest three gifts already cost 5 + 9 + 14 = 28.

### Example 3

```
prices = [10, 20, 30, 40], target = 75
Output: 70
```

70 (10 + 20 + 40) and 80 (10 + 30 + 40) are both 5 away; the smaller one wins.

### Constraints

- 3 ≤ prices.length ≤ 2000
- 1 ≤ prices[i] ≤ 10⁶
- 1 ≤ target ≤ 3·10⁶
