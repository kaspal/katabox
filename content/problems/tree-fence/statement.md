A ranger wants to enclose every tree of a grove with the shortest possible fence: a rubber band stretched around all the trees. The enclosed region is the **convex hull** of the trees.

Given the trees' positions `trees[i] = [x, y]`, return **twice** the area enclosed by that fence (twice the area is always a whole number). If all the trees lie on one line, or there is only one, the fence encloses nothing: return `0`.

Several trees may stand at the same position.

### Example 1

```
trees = [[0, 0], [4, 0], [4, 3], [0, 3], [2, 1], [1, 2]]
Output: 24
```

The fence is the 4 × 3 rectangle: area 12, twice the area 24.

### Example 2

```
trees = [[1, 1], [2, 2], [3, 3]]
Output: 0
```

### Example 3

```
trees = [[0, 0], [2, 0], [1, 1], [1, 3], [0, 0]]
Output: 6
```

The fence is the triangle (0, 0), (2, 0), (1, 3) with area 3.

### Constraints

- 1 ≤ trees.length ≤ 10⁵
- All coordinates are integers with absolute value at most 10⁷.
