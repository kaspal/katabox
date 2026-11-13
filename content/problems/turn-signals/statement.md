A driving-school app shows each learner a short route on a map: from point A drive straight to point B, then straight to point C. At B the learner must signal **left**, **right**, or nothing.

Each route is `[ax, ay, bx, by, cx, cy]`, with the x axis pointing east and the y axis pointing north. Return, for each route, one of these exact strings:

- `"left"` if the turn at B is to the left (counterclockwise),
- `"right"` if it is to the right (clockwise),
- `"straight"` if A, B and C lie on one line (this includes turning back on the same line).

### Example 1

```
routes = [[0, 0, 2, 0, 2, 3], [0, 0, 0, 2, 3, 2], [1, 1, 2, 2, 5, 5]]
Output: ["left", "right", "straight"]
```

The first route drives east, then north: a left turn.

### Example 2

```
routes = [[0, 0, 4, 0, 1, 0]]
Output: ["straight"]
```

C is behind B on the same line.

### Example 3

```
routes = [[-3, 5, 1, 1, 0, -4]]
Output: ["right"]
```

### Constraints

- 1 ≤ routes.length ≤ 10⁴
- Each route has 6 integers with absolute value at most 10⁴.
- A ≠ B and B ≠ C.
