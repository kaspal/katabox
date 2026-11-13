An orchard is planted as a grid of trees; `grid[r][c]` is how many crates of apples the tree in row `r`, column `c` gave this year. Buyers ask about rectangular plots: query `[top, left, bottom, right]` covers rows `top` to `bottom` and columns `left` to `right`, all included.

Return the total number of crates of each plot, in the order of the queries.

### Example 1

```
grid = [[3, 0, 2],
        [1, 4, 5],
        [6, 2, 1]],
queries = [[0, 0, 1, 1], [1, 1, 2, 2], [0, 2, 2, 2]]
Output: [8, 12, 8]
```

The first plot is 3 + 0 + 1 + 4; the third is the right-hand column, 2 + 5 + 1.

### Example 2

```
grid = [[7]], queries = [[0, 0, 0, 0]]
Output: [7]
```

### Example 3

```
grid = [[1, 2, 3, 4]], queries = [[0, 1, 0, 3], [0, 0, 0, 0]]
Output: [9, 1]
```

### Constraints

- 1 ≤ rows, columns ≤ 300 (all rows have the same length)
- 0 ≤ grid[r][c] ≤ 1000
- 1 ≤ queries.length ≤ 10⁵
- 0 ≤ top ≤ bottom < rows, 0 ≤ left ≤ right < columns
