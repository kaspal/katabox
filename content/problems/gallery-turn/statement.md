A museum shows a mosaic as a grid of tile brightness values, `grid`, with `r` rows and `c` columns. Tonight the curators hang it on the wall turned a quarter turn **clockwise**: the left column becomes the top row (read from bottom to top), and the top row becomes the right column.

Return the turned grid. It has `c` rows and `r` columns.

### Example 1

```
grid = [[1, 2, 3],
        [4, 5, 6]]
Output: [[4, 1],
         [5, 2],
         [6, 3]]
```

### Example 2

```
grid = [[7], [8], [9]]
Output: [[9, 8, 7]]
```

A single column turns into a single row.

### Example 3

```
grid = [[42]]
Output: [[42]]
```

### Constraints

- 1 ≤ r, c ≤ 300, where r = grid.length and c = grid[i].length (every row has the same length)
- 0 ≤ grid[i][j] ≤ 255
