A cinema opens a second door and the two waiting queues must merge into one, by ticket number. Each queue, `first` and `second`, is a linked list of ticket numbers in non-decreasing order.

Merge them into a single queue, also in non-decreasing order, and return it. You may reuse the existing nodes.

Lists are written as their values in order, so `[1, 4, 9]` is the list 1 → 4 → 9, and `[]` is the empty list.

### Example 1

```
first = [1, 4, 9], second = [2, 4, 5, 12]
Output: [1, 2, 4, 4, 5, 9, 12]
```

### Example 2

```
first = [], second = [3, 3]
Output: [3, 3]
```

### Example 3

```
first = [], second = []
Output: []
```

### Constraints

- 0 ≤ length of each queue ≤ 5·10⁴
- 1 ≤ ticket number ≤ 10⁶
- Each queue is in non-decreasing order.
