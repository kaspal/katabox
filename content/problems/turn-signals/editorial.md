## Approach: the sign of a cross product

For the vectors `u = B − A` and `v = C − B`, the cross product

```
cross = u.x · v.y − u.y · v.x
```

is the signed area of the parallelogram they span. It is positive when turning from `u` to `v` goes counterclockwise (a left turn, with y pointing north), negative for clockwise, and zero when `u` and `v` are parallel, so that A, B and C are on one line.

Coordinates are integers up to 10⁴, so every product fits easily and the test is exact: no angles, no floating point.

### Walkthrough

`[0, 0, 2, 0, 2, 3]`: u = (2, 0), v = (0, 3), cross = 2·3 − 0·0 = 6 > 0, so "left".

**Complexity:** O(1) per route, O(n) in all.
