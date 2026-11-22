Same merges as before, but now the town wants to know the size of its **biggest club** instead of the number of clubs. There are `n` players, numbered `0` to `n − 1`, each starting as a club of their own; `merges[i] = [a, b]` joins the clubs of players `a` and `b` (nothing changes if they are already together).

After each merge, return the number of players in the largest club, in order.

### Example 1

```
n = 5, merges = [[0, 1], [3, 4], [1, 0], [1, 4]]
Output: [2, 2, 2, 4]
```

The last merge joins {0, 1} with {3, 4}.

### Example 2

```
n = 5, merges = [[1, 4], [0, 2], [0, 3], [3, 3]]
Output: [2, 2, 3, 3]
```

### Example 3

```
n = 5, merges = [[0, 0], [0, 2], [1, 2], [2, 4]]
Output: [1, 2, 3, 4]
```

Merging a player with themselves changes nothing, so the biggest club still has 1 player.

### Constraints

- 1 ≤ n ≤ 10⁵
- 1 ≤ merges.length ≤ 10⁵
- `merges[i] = [a, b]` with 0 ≤ a, b < n (a may equal b)
