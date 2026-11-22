Felix packs gift baskets for a farm shop: each basket holds a fixed number of jars and must weigh exactly the amount on the label. He wrote a function to list every way to fill one, but it usually insists that no basket exists, even when Felix can put one together by hand in seconds.

The jars have the distinct weights in `weights`. A basket is a choice of exactly `k` different jars whose weights add up to exactly `target`. Return every possible basket, each written as the list of its weights in ascending order, with the baskets sorted in lexicographic order (compared element by element). Return an empty list if no basket works.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
weights = [4, 1, 3, 2, 5], k = 2, target = 6
Output: [[1, 5], [2, 4]]
```

3 + 3 would also make 6, but there is only one jar of weight 3.

### Example 2

```
weights = [19,28,26,25], k = 1, target = 25
Output: [[25]]
```

### Example 3

```
weights = [3,12,27,6], k = 3, target = 42
Output: [[3,12,27]]
```

### Constraints

- 1 ≤ weights.length ≤ 12
- 1 ≤ weights[i] ≤ 30, and all weights are distinct
- 1 ≤ k ≤ weights.length
- 1 ≤ target ≤ 360
