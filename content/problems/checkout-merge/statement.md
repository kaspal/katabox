At closing time a supermarket merges all its checkout lines into one. Each line in `lines` is a linked list of the customers' ticket numbers, from the front of the line to the back, already in non-decreasing order.

Merge every line into a single line, also in non-decreasing order of ticket number, and return it. Lines may be empty; if there are no customers at all, return an empty line.

Lists are written as their values in order, so `[2, 5, 9]` is the list 2 → 5 → 9, and `[]` is the empty list.

### Example 1

```
lines = [[2, 5, 9], [1, 5], [3]]
Output: [1, 2, 3, 5, 5, 9]
```

### Example 2

```
lines = [[], [4]]
Output: [4]
```

### Example 3

```
lines = []
Output: []
```

### Constraints

- 0 ≤ lines.length ≤ 10⁴
- 0 ≤ total number of customers ≤ 10⁵
- 1 ≤ ticket number ≤ 10⁶
- Each line is in non-decreasing order.
