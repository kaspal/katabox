To fill a plane from both ends at once, a gate agent reorders the boarding queue: the first passenger, then the last, then the second, then the second to last, and so on until everyone is placed.

The queue is a linked list of passenger numbers, `queue`, from front to back. Return it in zigzag order: L0, Ln−1, L1, Ln−2, L2, …

Lists are written as their values in order, so `[1, 4, 9]` is the list 1 → 4 → 9, and `[]` is the empty list.

### Example 1

```
queue = [1, 2, 3, 4, 5]
Output: [1, 5, 2, 4, 3]
```

### Example 2

```
queue = [10, 20, 30, 40]
Output: [10, 40, 20, 30]
```

### Example 3

```
queue = [7]
Output: [7]
```

### Constraints

- 0 ≤ number of passengers ≤ 10⁵
- 1 ≤ passenger number ≤ 10⁶
