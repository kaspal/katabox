A delivery robot moves on the floor plan of a warehouse, given as `grid`: `0` is free floor and `1` is a shelf. The robot starts in the top-left cell and must reach the bottom-right cell. Each move goes to the cell directly above, below, left or right, and must stay on free floor inside the grid.

Return the smallest number of moves to reach the bottom-right cell, or `-1` if it cannot be reached (including when the start or the goal is a shelf).

### Example 1

```
grid = [[0, 0, 0],
        [1, 1, 0],
        [0, 0, 0]]
Output: 4
```

Right, right, down, down.

### Example 2

```
grid = [[0, 1],
        [1, 0]]
Output: -1
```

### Example 3

```
grid = [[0]]
Output: 0
```

### Constraints

- 1 ≤ rows, cols ≤ 200 (rows = grid.length, cols = grid[r].length for every row)
- grid[r][c] is 0 or 1
