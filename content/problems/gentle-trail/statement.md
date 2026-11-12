A hiking club plans a trail across a hillside mapped as a grid: `heights[r][c]` is the height of the spot in row `r`, column `c`, in centimetres. The trail starts at the top-left spot, ends at the bottom-right spot, and moves one spot at a time **up, down, left or right**. It may wind and turn back as much as it likes.

A step between two neighbouring spots is as hard as the difference of their heights. The club cares only about the **hardest single step** of the trail, not its length. Return the smallest possible value of the hardest step over all trails.

### Example 1

```
heights = [[10, 30, 31], [12, 50, 33], [15, 17, 40]]
Output: 20
```

Down the left side, 10 → 12 → 15 → 17 → 40, the last step is 23. Along the top, 10 → 30 → 31 → 33 → 40, the hardest step is the first one, 20. No trail does better.

### Example 2

```
heights = [[5]]
Output: 0
```

The trail starts where it ends.

### Example 3

```
heights = [[1, 2, 3]]
Output: 1
```

### Constraints

- 1 ≤ heights.length ≤ 200
- 1 ≤ heights[i].length ≤ 200, all rows the same length
- 0 ≤ heights[i][j] ≤ 10⁶
