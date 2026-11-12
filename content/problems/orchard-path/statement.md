An orchard is planted as a grid of trees, and `grid[r][c]` is how many ripe apples the tree in row `r`, column `c` has. A picker starts at the top-left tree and must reach the bottom-right tree, moving only **right** or **down** one tree at a time, and picks every apple on each tree visited (including the first and the last).

Return the largest number of apples the picker can collect.

### Example 1

```
grid = [[3, 1, 4], [1, 5, 9], [2, 6, 5]]
Output: 23
```

The best path is 3 → 1 → 5 → 9 → 5. Going along the top row first, 3 → 1 → 4 → 9 → 5, collects only 22.

### Example 2

```
grid = [[7]]
Output: 7
```

### Example 3

```
grid = [[1, 2, 3, 4]]
Output: 10
```

A single row allows only one path.

### Constraints

- 1 ≤ grid.length ≤ 300
- 1 ≤ grid[i].length ≤ 300, all rows the same length
- 0 ≤ grid[i][j] ≤ 100
