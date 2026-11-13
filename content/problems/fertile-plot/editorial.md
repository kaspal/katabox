## Approach: fix two rows, run Kadane over the columns

Trying every rectangle, even with running sums, costs O(R²·C²): about 2.5·10⁹ rectangles on a 10 × 10 000 field. The trick is to fix only the **rows** and let a one-dimensional algorithm choose the columns.

Fix a top row `top` and a bottom row `bottom`. Every rectangle with exactly those rows is a run of consecutive columns, and its total is the sum of that run in the array

```
col[j] = grid[top][j] + grid[top + 1][j] + ... + grid[bottom][j]
```

So the best rectangle for this pair of rows is the **maximum-sum non-empty subarray** of `col`, which **Kadane's algorithm** finds in one pass:

```
cur = 0
for x in col:
    cur = cur + x if cur > 0 else x   # best run ending here
    best = max(best, cur)
```

A run ending at `x` either extends the best run ending just before it (worth it only when that run is positive) or starts fresh at `x`. Starting `cur` at 0 makes the first step pick `x` itself, so the run is never empty, and an all-negative field correctly answers its largest single cell.

For each `top`, start `col` at zeros and move `bottom` down from `top`, adding row `bottom` into `col` before each Kadane pass. Every rectangle has one top and one bottom row, so it is considered exactly once.

**Which side goes outside:** the pairs of rows cost R²/2 passes of length C. If R > C, transpose the grid first (swap the roles of rows and columns; the answer does not change), so the cost is min(R, C)²·max(R, C)/2. With R·C ≤ 10⁵ that is at most about 1.6·10⁷ steps (a 316 × 316 field), and only 5·10⁵ for 10 × 10 000.

**Complexity:** O(min(R, C)²·max(R, C)) time, O(R·C) memory. Totals stay within ±10⁸, so 32-bit integers are enough.

**Input:** read all tokens at once; after `R` and `C` come the `R·C` scores row by row. In Python, update the column sums with `list(map(operator.add, col, row))` (the loop runs in C) and keep the Kadane loop tight.
