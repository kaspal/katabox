A vending machine accepts coins of a few values, `coins` (all different), and you may use any number of coins of each value. List every way to pay exactly `target`.

A way to pay is a bag of coins: order does not matter, so write each one with its coin values in non-decreasing order. Return all the ways, sorted lexicographically (compare the first coins, then the second, and so on). If there is no way, return an empty list.

### Example 1

```
coins = [2, 3, 5], target = 8
Output: [[2, 2, 2, 2], [2, 3, 3], [3, 5]]
```

### Example 2

```
coins = [4], target = 6
Output: []
```

### Example 3

```
coins = [7, 1], target = 3
Output: [[1, 1, 1]]
```

### Constraints

- 1 ≤ coins.length ≤ 10, all values different
- 1 ≤ coins[i] ≤ 50
- 1 ≤ target ≤ 60
- There are at most 1000 ways to pay.
