A shared calendar stores a person's busy time as `slots`, each `[start, end]` with both moments included. The slots are sorted by start and no two of them share a moment. A new busy slot `fresh` arrives.

Insert it and return the updated list in the same form: sorted, and with every pair of slots that share at least one moment (even just an end point) combined into one.

### Example 1

```
slots = [[1, 3], [6, 8], [10, 12]], fresh = [7, 10]
Output: [[1, 3], [6, 12]]
```

[7, 10] overlaps [6, 8] and touches [10, 12] at moment 10.

### Example 2

```
slots = [], fresh = [4, 5]
Output: [[4, 5]]
```

### Example 3

```
slots = [[2, 4]], fresh = [5, 6]
Output: [[2, 4], [5, 6]]
```

### Example 4

```
slots = [[1, 2], [5, 9]], fresh = [2, 5]
Output: [[1, 9]]
```

### Constraints

- 0 ≤ slots.length ≤ 10⁴
- 0 ≤ start ≤ end ≤ 10⁶ for every slot and for `fresh`
- `slots` is sorted by start, and each slot starts after the previous one ends.
