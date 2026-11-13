A postcard artist draws a city's outline from far away. Building `i` is a rectangle standing on the ground from `x = left` to `x = right` with height `height`: `buildings[i] = [left, right, height]`. The outline at a position `x` is the height of the tallest building covering it (`left ≤ x < right`), or 0 if there is none.

Return the outline as its **key points** `[x, h]`: every position `x` where the outline's height changes, with `h` the new height from `x` on. List them by increasing `x`; two consecutive key points never have the same height.

### Example 1

```
buildings = [[1, 4, 3], [2, 6, 5], [8, 9, 2]]
Output: [[1, 3], [2, 5], [6, 0], [8, 2], [9, 0]]
```

### Example 2

```
buildings = [[0, 2, 4], [2, 5, 4]]
Output: [[0, 4], [5, 0]]
```

The two buildings meet at x = 2 with the same height, so the outline does not change there.

### Example 3

```
buildings = [[3, 7, 6], [4, 5, 2]]
Output: [[3, 6], [7, 0]]
```

### Constraints

- 1 ≤ buildings.length ≤ 10⁵
- 0 ≤ left < right ≤ 10⁶
- 1 ≤ height ≤ 10⁴
