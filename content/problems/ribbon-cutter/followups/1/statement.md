The gift-wrapping stall from the base problem now gets a whole day of orders at once.
It has rolls of ribbon with whole-number lengths `ribbons` (in centimetres), and
`orders[k]` is the number of pieces the `k`-th customer needs, all of the same
whole-number length. As before, each roll is cut into as many pieces of that length as
fit, leftovers are thrown away and pieces cannot be glued.

The orders are independent: each one is planned on the full set of rolls. Return an
array whose `k`-th entry is the greatest length (at least 1) that yields at least
`orders[k]` pieces, or `0` if even pieces of length 1 are not enough.

### Example 1

```
ribbons = [30, 47, 18], orders = [6, 1, 95, 200]
Output: [15, 47, 1, 0]
```

Length 15 gives 2 + 3 + 1 = 6 pieces. Length 1 gives 95 pieces, which is not enough for
200.

### Example 2

```
ribbons = [37, 5], orders = [32, 31, 14]
Output: [1, 1, 2]
```

Length 2 gives 18 + 2 = 20 pieces; length 3 gives only 12 + 1 = 13.

### Example 3

```
ribbons = [6, 6], orders = [3, 11, 5]
Output: [3, 1, 2]
```

### Constraints

- 1 ≤ ribbons.length, orders.length ≤ 10⁵
- 1 ≤ ribbons[i] ≤ 10⁵
- 1 ≤ orders[k] ≤ 10¹⁰ (more than a 32-bit integer holds)
