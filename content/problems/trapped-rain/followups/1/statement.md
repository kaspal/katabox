This time the garden is not a single wall but a whole field of stone columns. `heights` is a grid: `heights[r][c]` is the height of the 1 × 1 column at row `r`, column `c`. After a storm, water collects on top of the columns. It flows between columns that share a side (up, down, left or right) and runs off the garden past any column on the outer edge.

So the water over a column rises to the lowest level at which it could escape: among all paths of side-adjacent columns from it to a column on the edge, take the one whose tallest column (counting both ends) is lowest; that height is the water level, and the column holds the level minus its own height. Columns on the edge never hold water. Return the total amount of trapped water, in unit cubes.

### Example 1

```
heights = [[3, 3, 3, 3], [3, 1, 2, 3], [3, 3, 3, 3]]
Output: 3
```

The two inner columns are surrounded by height 3: 2 + 1.

### Example 2

```
heights = [[1, 4, 6, 6, 6], [0, 2, 0, 3, 6], [3, 3, 5, 3, 6], [1, 0, 3, 0, 6]]
Output: 2
```

The 0 in the middle of row 1 fills up to 2; any higher and the water spills over its left neighbour (height 2) to the 0 on the edge.

### Example 3

```
heights = [[6, 6, 0, 0, 0], [2, 6, 1, 5, 6], [5, 6, 2, 2, 4], [1, 4, 0, 4, 5]]
Output: 0
```

Every inner column has a path to the edge that never climbs above its own height.

### Constraints

- 1 ≤ rows ≤ 200
- 1 ≤ columns ≤ 200, every row the same length
- 0 ≤ heights[r][c] ≤ 10⁴
