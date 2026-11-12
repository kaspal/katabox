## Approach: dynamic programming over the grid

Every path to tree `(r, c)` arrives from the tree above, `(r − 1, c)`, or from the tree on the left, `(r, c − 1)`. The rest of the path does not matter, only how many apples it collected. So the most apples on a path ending at `(r, c)` is

```
best[r][c] = grid[r][c] + max(best[r - 1][c], best[r][c - 1])
```

On the top row only the left neighbour exists, and in the left column only the one above. Filling the table row by row means both values are ready when needed. The answer is `best[rows − 1][cols − 1]`.

A single array of length `cols` is enough: before the update, `best[c]` still holds the value from the row above, and `best[c − 1]` already holds the new value on the left.

```
best = [0] * cols
for r in rows:
    for c in cols:
        if r == 0 and c == 0: from = 0
        elif r == 0: from = best[c - 1]
        elif c == 0: from = best[c]
        else: from = max(best[c], best[c - 1])
        best[c] = grid[r][c] + from
return best[cols - 1]
```

### Walkthrough

For `[[3, 1, 4], [1, 5, 9], [2, 6, 5]]` the table is:

| | col 0 | col 1 | col 2 |
| --- | --- | --- | --- |
| row 0 | 3 | 4 | 8 |
| row 1 | 4 | 9 | 18 |
| row 2 | 6 | 15 | 23 |

The answer is 23.

**Complexity:** O(rows × cols) time and O(cols) extra space. Exploring every path recursively is exponential: a 20 × 20 orchard already has about 3.5·10¹⁰ paths.
