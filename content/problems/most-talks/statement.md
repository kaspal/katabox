A conference runs many talks; talk `i` starts at `talks[i][0]` and ends at `talks[i][1]`. A visitor attends whole talks only, one at a time. A talk that ends at a moment leaves time to start another talk at that same moment.

Return the largest number of talks the visitor can attend.

### Example 1

```
talks = [[9, 11], [10, 12], [11, 13], [12, 15], [8, 9]]
Output: 3
```

For example 8–9, 9–11 and 11–13.

### Example 2

```
talks = [[1, 100], [2, 3], [3, 4], [4, 5]]
Output: 3
```

The long talk is not worth it.

### Example 3

```
talks = [[5, 6]]
Output: 1
```

### Constraints

- 1 ≤ talks.length ≤ 10⁵
- 0 ≤ talks[i][0] < talks[i][1] ≤ 10⁶
