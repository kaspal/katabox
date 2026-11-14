## The idea

Place `n` queens on an `n × n` board so that no two attack each other: no two share a row, a column or a diagonal. Since each row holds exactly one queen, the search goes **row by row**, top to bottom, and the state is just `cols`, where `cols[r]` is the column of the queen in row `r`.

For the next row `r`, a column `c` is **safe** when every earlier queen `(rr, cc)` has `cc ≠ c` (another column) and `|c − cc| ≠ r − rr` (not on a diagonal: on a diagonal, the column distance equals the row distance). The sample tries the columns from left to right; for each safe one it places the queen, recurses into the next row, then removes it. When all `n` rows are filled, the columns are recorded. The search does not stop at the first solution: it **counts all of them**. The output is the count, then the first solution's columns, row by row. The board size is typed as one number from 4 to 6; the board shades the squares the placed queens attack.

### The default input

`n = 5`. Row 0 takes column 0. In row 1, column 0 shares the column and column 1 a diagonal, so the queen goes to column 2. In row 2, columns 0 and 2 are taken and columns 1 and 3 touch the queen in row 1 diagonally; column 4 is safe. Row 3 takes column 1, and row 4 finds columns 0 to 2 used, so it takes column 3. The first solution, `[0, 2, 4, 1, 3]`, appears without a single dead end. The search goes on through every other branch and finds 10 solutions in all.

The output is `[10, 0, 2, 4, 1, 3]`. Larger is not always more: `n = 4` has 2 solutions and `n = 6` only 4.

## Steps

1. Start with an empty `cols` and call `place()`.
2. If `cols` holds `n` columns, record a copy and return.
3. Otherwise, for each column `c` from 0 to `n − 1`: if it is safe against every queen in `cols`, append it, call `place()` and pop it.
4. Return the number of solutions, then the first one's columns.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n!)` bound: the column rule alone leaves `n` choices in row 0, `n − 1` in row 1, and so on |
| Average | Far less than `n!` with pruning: diagonals cut most branches after a few rows |
| Worst | `O(n!)`, and each safety check scans up to `n` queens |
| Extra space | `O(n)` for `cols` and the call depth |

## Pitfalls

- **The diagonal test.** The column distance needs the absolute value; the row distance is already positive because earlier rows are above.
- **Stopping at the first solution.** That is fine for finding one, but the count would be wrong.
- **Mirror images.** The 10 solutions for `n = 5` include mirrored and rotated copies of each other; they are all counted.

## When to use it

N-Queens is the model for **constraint search**: place one piece at a time and reject a partial placement the moment it breaks a rule. Two speed-ups are worth knowing. Keeping sets (or bitmasks) of used columns and of both diagonal directions (`r + c` and `r − c` are constant along each) makes the safety check constant time. And symmetry halves the work: every solution has a mirror image, so it is enough to try the left half of row 0 and double the count; when `n` is odd, the middle column is searched on its own.

## Related topics

- [Permutations](learn:backtracking/permutations): one queen per row and column is a permutation of the columns.
- [Sudoku](learn:backtracking/sudoku): the same place, check and remove on a number grid.
- [Bit manipulation basics](learn:foundations/bit-manipulation): the tool behind the fastest queen solvers.
- [Depth-first search](learn:graphs/dfs): backtracking is a depth-first walk over the choices.
