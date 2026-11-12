## Approach: dynamic programming over prefixes

Let `d[i][j]` be the fewest fixes that turn `typed[:i]` into `meant[:j]`.

- `d[i][0] = i` (delete everything) and `d[0][j] = j` (insert everything).
- If the last letters match, `typed[i−1] = meant[j−1]`, they need no fix: `d[i][j] = d[i−1][j−1]`.
- Otherwise the last fix deletes `typed[i−1]`, inserts `meant[j−1]`, or replaces one with the other:
  `d[i][j] = 1 + min(d[i−1][j], d[i][j−1], d[i−1][j−1])`.

Each row only needs the previous one, so keep two rows of length `m + 1`.

### Walkthrough

`typed = "stake"`, `meant = "skate"` (rows: prefixes of `typed`; columns: prefixes of `meant`):

| | "" | s | k | a | t | e |
| --- | --- | --- | --- | --- | --- | --- |
| "" | 0 | 1 | 2 | 3 | 4 | 5 |
| s | 1 | 0 | 1 | 2 | 3 | 4 |
| t | 2 | 1 | 1 | 2 | 2 | 3 |
| a | 3 | 2 | 2 | 1 | 2 | 3 |
| k | 4 | 3 | 2 | 2 | 2 | 3 |
| e | 5 | 4 | 3 | 3 | 3 | **2** |

**Complexity:** O(n·m) time, O(m) space: 10⁶ cells for two 1000-letter words. The plain recursion branches three ways at every mismatch and is exponential.
