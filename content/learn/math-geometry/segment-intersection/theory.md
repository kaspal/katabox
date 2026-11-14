## The idea

Do two segments, `p1p2` and `p3p4`, share a point? Solving the two line equations works, but it needs division, fractions and a special case for parallel lines. The orientation test answers with whole-number multiplications only.

`orient(a, b, c)` is the sign of the **cross product** `(b - a) × (c - a)`: `+1` when `c` lies to the left of the direction `a → b`, `-1` when it lies to the right, and `0` when the three points are on one line. The sample makes four of these tests:

- `d1`, `d2`: the sides of `p1` and `p2` relative to the segment `p3p4`;
- `d3`, `d4`: the sides of `p3` and `p4` relative to the segment `p1p2`.

There are two ways to meet:

- **A proper crossing.** `p1` and `p2` lie on opposite sides of the line through `p3p4` (`d1 · d2 < 0`), **and** `p3` and `p4` lie on opposite sides of the line through `p1p2` (`d3 · d4 < 0`). One condition alone is not enough: a short segment can straddle a long line far beyond where the other segment ends.
- **A touch.** Some test gives 0, so that point lies on the other segment's line; it is really on the segment when it also sits inside the segment's **bounding box** (between its smallest and largest `x` and `y`).

The output is `[1 or 0, d1, d2, d3, d4]`: 1 when the segments meet. The two segments are typed as four `x y` points, coordinates 0 to 20.

### The default input

`p1 = (2, 3)`, `p2 = (15, 14)`, `p3 = (3, 16)`, `p4 = (17, 2)` (the "crossing" preset). Seen from `p3 → p4`, `p1` is on the right (`d1 = -1`) and `p2` on the left (`d2 = 1`). Seen from `p1 → p2`, `p3` is on the left (`d3 = 1`) and `p4` on the right (`d4 = -1`). Both pairs straddle, so the output is `[1, -1, 1, 1, -1]`.

"An end on the other line" (`(3, 3)`, `(15, 9)` against `(9, 6)`, `(13, 16)`): the first pair straddles, but `d3 = 0`, since `(9, 6)` lies exactly on the line through `(3, 3)` and `(15, 9)`. It is inside that segment's box, so they touch: `[1, 1, -1, 0, 1]`.

"Side by side" (`(2, 2)`, `(8, 14)` against `(10, 2)`, `(16, 14)`): both ends of each segment are on one side of the other, and no test is 0. The output is `[0, 1, 1, -1, -1]`.

## Steps

1. Compute `d1 = orient(p3, p4, p1)`, `d2 = orient(p3, p4, p2)`, `d3 = orient(p1, p2, p3)`, `d4 = orient(p1, p2, p4)`.
2. If `d1 · d2 < 0` and `d3 · d4 < 0`, they cross: return 1.
3. For each test that is 0, if its point is inside the other segment's bounding box, they touch: return 1.
4. Otherwise return 0.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(1)`: four cross products and a few comparisons |
| Extra space | `O(1)` |

## Pitfalls

- **Skipping the box check.** Two segments on one line, like `(1, 1)–(3, 3)` and `(5, 5)–(8, 8)`, give four zeros but never meet; only the box check says `[0, 0, 0, 0, 0]`.
- **Using `≤ 0` for the crossing test.** A zero there is a touch candidate, not a crossing; it still needs the box check.
- **Floating-point slopes.** Comparing slopes with division invites rounding errors and division by zero for vertical segments; integer cross products are exact.

## When to use it

Use it to check whether paths, walls or edges meet: collision tests, whether a polygon crosses itself, or whether a path cuts a fence. The linked problem, Crossing Paths, asks exactly this for many pairs, overlaps and shared endpoints included.

## Related topics

- [Convex hull (monotone chain)](learn:math-geometry/convex-hull): the same cross product decides each turn of the hull.
- [Merge intervals](learn:greedy/merge-intervals): the one-dimensional cousin, deciding when two ranges on a line overlap.
