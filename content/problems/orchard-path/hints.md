There are far too many paths to try them all: about C(598, 299) for a 300 × 300 orchard. But the best way to reach a tree only depends on the best ways to reach the trees just above and just to its left.
---
Let `best[r][c]` be the most apples on any path ending at `(r, c)`. You can only arrive from `(r − 1, c)` or `(r, c − 1)`.
---
`best[r][c] = grid[r][c] + max(best[r − 1][c], best[r][c − 1])`, using only the neighbour that exists on the top row and the left column. Fill the table row by row; the answer is the bottom-right entry. One row of memory is enough.
