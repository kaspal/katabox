## Idea

Extend the base problem's prefix sums to a grid. Build `P` with one extra row and column of
zeros, where `P[r][c]` is the total of all cells in rows `0..r−1` and columns `0..c−1`.
Each entry is its cell plus the block above plus the block to the left, minus the corner
block that both of those counted. A rectangle `[v1, d1, v2, d2]` is then the big block
`P[v2+1][d2+1]`, minus the strip above it `P[v1][d2+1]`, minus the strip to its left
`P[v2+1][d1]`, plus `P[v1][d1]`, which was subtracted twice (inclusion-exclusion).

## Complexity

O(V · D) time to build the table for `V` vans and `D` days, then O(1) per question;
O(V · D) extra memory. A total is at most 10⁵ · 1000 = 10⁸, well within a 32-bit integer.
