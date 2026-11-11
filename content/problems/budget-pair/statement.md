A gift shop lists its `prices` in non-decreasing order. You want to buy exactly **two different items** (two different positions in the list) and spend as much of your `budget` as possible without going over it.

Return the largest total of two different items that is at most `budget`, or `-1` if no two items fit.

### Example 1

```
prices = [3, 5, 8, 12, 20], budget = 19
Output: 17
```

5 + 12 = 17 is the best total that does not exceed 19.

### Example 2

```
prices = [4, 4, 9], budget = 8
Output: 8
```

The two items priced 4 are different items.

### Example 3

```
prices = [10, 15], budget = 24
Output: -1
```

### Constraints

- 2 ≤ prices.length ≤ 5·10⁴
- 1 ≤ prices[i] ≤ 10⁶
- prices is sorted in non-decreasing order.
- 1 ≤ budget ≤ 2·10⁶
