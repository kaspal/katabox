## The idea

A walker starts in the top-left cell of a grid and must reach the bottom-right cell, moving only **right** or **down**. Some cells are **blocked**. How many different paths are there? Listing them all is hopeless: an open 10 by 10 grid already has 48 620.

Every path into a cell arrives either **from above** or **from the left**, never both, so the counts simply add up. `ways[r][c]` is the number of paths from the top-left to cell `(r, c)`:

- A **blocked** cell gets 0: no path may stand on it.
- The **top-left** cell gets 1 (the path that has not moved yet), unless it is blocked.
- Any other cell gets `up + left`, where a neighbour outside the grid counts as 0.

The sample fills the table row by row, left to right, so both neighbours are ready when a cell needs them. The output is `[ways[R - 1][C - 1]]`, the count at the bottom-right. The map is typed one row at a time, rows separated by `/`, with `.` for an open cell and `#` for a blocked one.

### The default input

Five rows of six cells: `....../.#..#./....../..#.../......`. Row 0 is all 1s: one way along the top edge. Row 1 is `1, 0, 1, 2, 0, 1`: the blocked cells are 0, and the cell after the first block gets only its 1 from above. Row 2 is `1, 1, 2, 4, 4, 5`. Row 3 is `1, 2, 0, 4, 8, 13`. Row 4 is `1, 3, 3, 7, 15, 28`.

The output is `[28]`. Without the three blocks the same grid would have 126 paths.

## Steps

1. Make a table of `R` rows and `C` columns.
2. For each row `r` and column `c`, in order: if the cell is blocked, write 0; else if it is the top-left, write 1; else write `up + left`, using 0 for a missing neighbour.
3. Return the bottom-right cell.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(R · C)`: each cell adds two numbers |
| Extra space | `O(R · C)` for the table |

## Pitfalls

- **Forgetting that a block cuts off what lies behind it.** A cell whose up and left are both 0 gets 0 even when it is open; the "walled off" preset leaves a single path.
- **A blocked start or end.** Then the answer is 0; the sample writes 0 at the top-left before checking for the start.
- **Huge counts.** Paths grow very fast with the grid size; real problems ask for the count modulo some number, applied after every addition.

## When to use it

Use it to count routes with restricted moves, and with `max` in place of the sum to find the best route. Both linked problems use this table: Blocked Walks counts walks around closed crossings, and Orchard Path collects the most apples along the way.

## Related topics

- [Climbing stairs](learn:dynamic-programming/climbing-stairs): the same counting, along a line.
- [Edit distance](learn:dynamic-programming/edit-distance): a table that reads its up and left neighbours, and the diagonal.
- [Breadth-first search on a grid](learn:grid-pathfinding/grid-bfs): the shortest route when moves go in every direction.
- [Longest common subsequence](learn:dynamic-programming/lcs): another grid filled row by row.
