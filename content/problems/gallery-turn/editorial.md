## Approach: map every tile to its new place

Turning clockwise sends row `i`, column `j` of an `r × c` grid to row `j`, column `r − 1 − i` of a `c × r` grid. Equivalently, row `j` of the result is column `j` of the original read from bottom to top.

Allocate the `c × r` result and copy every tile once.

### Walkthrough

`grid = [[1, 2, 3], [4, 5, 6]]` (r = 2, c = 3):

| tile | from (i, j) | to (j, r − 1 − i) |
| --- | --- | --- |
| 1 | (0, 0) | (0, 1) |
| 4 | (1, 0) | (0, 0) |
| 3 | (0, 2) | (2, 1) |
| 6 | (1, 2) | (2, 0) |

Result: `[[4, 1], [5, 2], [6, 3]]`.

**Complexity:** O(r·c) time and space, the size of the output. (An in-place turn exists for square grids, but it cannot change the shape of a rectangular one.)
