A town has `n` chess players, numbered `0` to `n − 1`, and at first each one is a club of their own. Then clubs start to merge: `merges[i] = [a, b]` means the club of player `a` and the club of player `b` become one club. If the two players are already in the same club, nothing changes.

After each merge, how many clubs are there? Return the counts in order.

### Example 1

```
n = 5, merges = [[0, 1], [3, 4], [1, 0], [1, 4]]
Output: [4, 3, 3, 2]
```

After the third merge, 1 and 0 are already in one club, so nothing changes. The last merge joins {0, 1} with {3, 4}, leaving that club and player 2's.

### Example 2

```
n = 3, merges = [[2, 2]]
Output: [3]
```

### Example 3

```
n = 4, merges = [[0, 1], [2, 3], [0, 2]]
Output: [3, 2, 1]
```

### Constraints

- 1 ≤ n ≤ 10⁵
- 1 ≤ merges.length ≤ 10⁵
- `merges[i] = [a, b]` with 0 ≤ a, b < n (a may equal b)
