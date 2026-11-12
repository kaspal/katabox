A record shop keeps its catalogue numbers in a list sorted from smallest to largest; a number may appear more than once (several copies of the same record). Customers often ask "how many records are filed before number `x`?", that is, how many catalogue numbers are **strictly smaller** than `x`.

`codes` is the sorted list and `queries` holds the customers' numbers. Return the answer for each query, in order.

### Example 1

```
codes = [3, 8, 8, 15, 20], queries = [8, 1, 21, 16]
Output: [1, 0, 5, 4]
```

Only 3 is smaller than 8. Nothing is smaller than 1, everything is smaller than 21, and four numbers are smaller than 16.

### Example 2

```
codes = [5], queries = [5, 6]
Output: [0, 1]
```

### Example 3

```
codes = [2, 2, 2], queries = [2, 3, 0]
Output: [0, 3, 0]
```

### Constraints

- 1 ≤ codes.length, queries.length ≤ 10⁵
- 0 ≤ codes[i], queries[j] ≤ 10⁹
- `codes` is sorted in non-decreasing order
