## The bug

`routes(r, c)` counts the routes from cell `(r, c)` to the end, and that number depends on the cell. The starter caches it under `r + c`:

```python
        if r + c in memo:
            return memo[r + c]
        memo[r + c] = (routes(r + 1, c) + routes(r, c + 1)) % MOD
        return memo[r + c]
```

All cells on one anti-diagonal (same `r + c`) share one entry, so the first one computed answers for all the others. Cells on the same diagonal usually have different route counts, though tiny grids like the examples can come out right by luck.

With `floor = ["..", "..", ".."]` (3 rows, 2 columns): the recursion first goes down to `(2, 0)`, which has 1 route and is stored under key 2. Then `(1, 1)` finds key 2 and also gets 1 (correct by luck), so `(1, 0)` gets 2 and is stored under key 1. Then `(0, 1)` finds key 1 and gets 2, but it really has only 1 route (straight down). The starter returns 2 + 2 = 4 instead of 3.

## The fix

Key the memo by the cell:

```python
        if (r, c) in memo:
            return memo[(r, c)]
        memo[(r, c)] = (routes(r + 1, c) + routes(r, c + 1)) % MOD
        return memo[(r, c)]
```

In TypeScript, use a key that is unique per cell, such as `r * cols + c`, in both `memo.get` and `memo.set`.

**Complexity:** O(rows · cols) time and extra space; each cell is computed once.
