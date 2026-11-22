The arcade's change machine still holds an unlimited supply of each token value in
`coins`. This time, instead of the fewest tokens, find **how many different
combinations** of tokens add up to exactly `amount`.

Order does not matter: a combination is just how many tokens of each value you take, so
`1 + 2` and `2 + 1` are the same combination. Paying `0` has exactly one combination (no
tokens). The number grows fast: return it modulo 1 000 000 007.

### Example 1

```
coins = [3, 4, 10], amount = 20
Output: 4
```

10 + 10, 10 + 4 + 3 + 3, 4 + 4 + 3 + 3 + 3 + 3, and 4 + 4 + 4 + 4 + 4.

### Example 2

```
coins = [2, 5, 1], amount = 9
Output: 8
```

### Example 3

```
coins = [7, 1, 6], amount = 6
Output: 2
```

One token of 6, or six tokens of 1.

### Constraints

- 1 ≤ coins.length ≤ 30
- 1 ≤ coins[i] ≤ 10⁴, all different
- 0 ≤ amount ≤ 2·10⁴
