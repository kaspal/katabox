An arcade's change machine pays out tokens. It holds an unlimited supply of each token value in `coins`, and it always pays an exact `amount` using as **few tokens** as possible.

Return the fewest tokens that add up to exactly `amount`, or `-1` if no combination of the token values makes that amount. Paying `0` takes no tokens.

### Example 1

```
coins = [1, 6, 10], amount = 12
Output: 2
```

Two tokens of 6. Taking the largest token first (10 + 1 + 1) would use three.

### Example 2

```
coins = [4, 6], amount = 9
Output: -1
```

Every combination of 4s and 6s is even.

### Example 3

```
coins = [5], amount = 0
Output: 0
```

### Constraints

- 1 ≤ coins.length ≤ 30
- 1 ≤ coins[i] ≤ 10⁴, all different
- 0 ≤ amount ≤ 2·10⁴
