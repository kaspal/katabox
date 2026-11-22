The shelf is the same as in the base problem: folders with different catalogue numbers,
sorted from smallest to largest, after a helper moved a block from the start to the end.
Catalogue numbers are positive.

New folders need numbers, so the librarian asks about **free numbers**: a positive number
is free when no folder on the shelf has it. For each `k` in `queries`, return the `k`-th
smallest free number (`k = 1` is the smallest), in the order asked.

### Example 1

```
shelf = [9, 12, 2, 4, 5], queries = [1, 3, 4, 8]
Output: [1, 6, 7, 13]
```

The free numbers are 1, 3, 6, 7, 8, 10, 11, 13, 14, …

### Example 2

```
shelf = [1, 2, 3], queries = [1, 2]
Output: [4, 5]
```

### Example 3

```
shelf = [7], queries = [6, 7]
Output: [6, 8]
```

### Constraints

- 1 ≤ shelf.length ≤ 10⁵
- 1 ≤ shelf[i] ≤ 10⁹; the numbers are all different, and `shelf` is a sorted list with a
  block moved from the start to the end.
- 1 ≤ queries.length ≤ 10⁵
- 1 ≤ queries[j] ≤ 10⁹ (every answer stays below 1.1·10⁹)
