A park planner draws footpaths as straight segments and wants to know which pairs of paths meet, so that a crossing can be paved. Each entry of `pairs` is `[x1, y1, x2, y2, x3, y3, x4, y4]`: the first path goes from (x1, y1) to (x2, y2), the second from (x3, y3) to (x4, y4).

Two paths **meet** when they share at least one point: they cross, one ends on the other, they share an endpoint, or they overlap along a line. Return `true` or `false` for each pair.

### Example 1

```
pairs = [[0, 0, 4, 4, 0, 4, 4, 0], [0, 0, 2, 2, 3, 3, 5, 5], [0, 0, 2, 0, 2, 0, 2, 5]]
Output: [true, false, true]
```

The diagonals of a square cross. The second pair lies on one line but leaves a gap. The third pair shares the point (2, 0).

### Example 2

```
pairs = [[0, 0, 4, 0, 2, 0, 6, 0], [0, 0, 1, 1, 1, 0, 2, -1]]
Output: [true, false]
```

The first pair overlaps from x = 2 to x = 4.

### Example 3

```
pairs = [[-2, 3, 2, 3, 0, 5, 0, 4]]
Output: [false]
```

### Constraints

- 1 ≤ pairs.length ≤ 10⁴
- All coordinates are integers with absolute value at most 10⁴.
- Each path has two different endpoints.
