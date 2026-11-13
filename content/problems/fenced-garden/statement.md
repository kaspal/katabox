A community garden is surrounded by a fence. The fence is a simple polygon (its sides never cross or touch except at shared corners), given by its corners `fence[i] = [x, y]` in order around the garden, either clockwise or counterclockwise.

For each point `points[j] = [x, y]` where a gardener wants to plant, return exactly one of these strings:

- `"inside"` if the point is strictly inside the garden,
- `"boundary"` if it lies on the fence (a side or a corner),
- `"outside"` otherwise.

### Example 1

```
fence = [[0, 0], [6, 0], [6, 4], [3, 2], [0, 4]], points = [[1, 1], [3, 3], [6, 2], [3, 2], [7, 1]]
Output: ["inside", "outside", "boundary", "boundary", "outside"]
```

The fence has a notch at (3, 2), so (3, 3) is outside.

### Example 2

```
fence = [[0, 0], [4, 0], [0, 4]], points = [[1, 1], [2, 2], [3, 2]]
Output: ["inside", "boundary", "outside"]
```

### Example 3

```
fence = [[0, 0], [2, 0], [2, 2], [0, 2]], points = [[0, 0], [1, 1], [-1, 1]]
Output: ["boundary", "inside", "outside"]
```

### Constraints

- 3 ≤ fence.length ≤ 300, and the fence is a simple polygon with distinct corners.
- 1 ≤ points.length ≤ 1000
- All coordinates are integers with absolute value at most 10⁴.
