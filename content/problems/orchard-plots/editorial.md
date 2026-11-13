## Approach: 2-D prefix sums

Build a table `pre` with one extra row and column of zeros, where `pre[r][c]` is the total of all trees in rows `0..r−1` and columns `0..c−1`. Row by row, keep the running total `run` of the current row: `pre[r+1][c+1] = pre[r][c+1] + run`.

A plot `[top, left, bottom, right]` is the big corner rectangle down to `(bottom, right)`, minus the strip above it, minus the strip to its left, plus the corner that was subtracted twice:

`pre[bottom+1][right+1] − pre[top][right+1] − pre[bottom+1][left] + pre[top][left]`.

### Walkthrough

Example 1, plot `[1, 1, 2, 2]`: the whole grid is 24, the top row is 5, the left column is 10, and the top-left tree (3) was removed twice: 24 − 5 − 10 + 3 = 12.

**Complexity:** O(rows · columns) to build, O(1) per query. Summing each plot directly is O(rows · columns) per query, up to 9·10⁹ steps for 10⁵ large plots.
