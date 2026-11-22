This time clubs can change their minds. There are `n` players, numbered `0` to `n − 1`, each starting as a club of their own. Apply `ops` in order:

- `[1, a, b]`: a merge, as before: the clubs of `a` and `b` become one (nothing changes if they already are).
- `[0]`: undo the latest merge that is still in effect, putting the clubs back exactly as they were before it. A merge that changed nothing still counts as a merge: undoing it changes nothing either. If no merge is left to undo, `[0]` does nothing.

An undone merge is gone for good, so the next `[0]` undoes the merge before it. After every operation, return the number of clubs.

### Example 1

```
n = 4, ops = [[1, 0, 1], [1, 1, 0], [1, 2, 3], [0], [0], [0], [0]]
Output: [3, 3, 2, 3, 3, 4, 4]
```

The undos take back [2, 3], then [1, 0] (which had changed nothing), then [0, 1]; the last undo finds nothing left.

### Example 2

```
n = 5, ops = [[0], [1, 0, 2], [0], [1, 3, 3], [1, 0, 3], [0]]
Output: [5, 4, 5, 5, 4, 5]
```

### Example 3

```
n = 5, ops = [[1, 0, 0], [0], [1, 2, 2], [1, 4, 0], [1, 1, 3], [1, 4, 2]]
Output: [5, 5, 5, 4, 3, 2]
```

### Constraints

- 1 ≤ n ≤ 10⁵
- 1 ≤ ops.length ≤ 10⁵
- Each op is `[0]` (undo) or `[1, a, b]` (merge) with 0 ≤ a, b < n (a may equal b)
