Rosa is stocking a school science kit on a tight budget and needs the total cost of the `k` cheapest items on the supplier's list. Her script uses a heap to keep only `k` prices as it reads the list, but on most lists its totals come out too high.

Given `prices` and `k`, return the sum of the `k` smallest prices. Equal prices are separate items.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
prices = [7, 2, 9, 4, 4], k = 3
Output: 10
```

The three cheapest items cost 2, 4 and 4.

### Example 2

```
prices = [19,28,26,25], k = 1
Output: 19
```

### Example 3

```
prices = [3,3,12,27], k = 1
Output: 3
```

### Constraints

- 1 ≤ prices.length ≤ 10⁵
- 1 ≤ prices[i] ≤ 10⁴
- 1 ≤ k ≤ prices.length
