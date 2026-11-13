A messaging app has `n` users, numbered 0 to n − 1. A link `[a, b]` means `a` can forward messages to `b` (not the other way round). Messages can be forwarded along several links.

Two users **reach each other** when a message from either one can arrive at the other. This splits the users into groups where everyone reaches everyone else in the same group (a user alone is a group of one).

Return the sizes of all the groups, in increasing order.

### Example 1

```
n = 5, links = [[0, 1], [1, 2], [2, 0], [2, 3], [3, 4]]
Output: [1, 1, 3]
```

Users 0, 1 and 2 reach each other; 3 and 4 are alone, since nothing comes back from them.

### Example 2

```
n = 4, links = [[0, 1], [1, 0], [2, 3], [3, 2]]
Output: [2, 2]
```

### Example 3

```
n = 3, links = []
Output: [1, 1, 1]
```

### Constraints

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 2·10⁵
- 0 ≤ a, b < n and a ≠ b
