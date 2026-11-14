## The idea

The **convex hull** of a set of points is the smallest convex polygon that contains them all: stretch a rubber band around the points and let go. Its corners are some of the points; the rest lie inside or on its edges.

The tool is the **cross product** of three points `o`, `a`, `b`:

`cross(o, a, b) = (a.x - o.x) · (b.y - o.y) - (a.y - o.y) · (b.x - o.x)`

Its sign tells which way you turn walking from `o` to `a` and then on to `b`: **positive** is a left (counter-clockwise) turn, **negative** a right (clockwise) turn, and **zero** means the three points are on one line.

**Andrew's monotone chain** sorts the points by `x`, then by `y`, and removes duplicates. It builds the **lower chain** from left to right: before adding a point, while the last two points of the chain and the new one do not make a strict left turn (`cross ≤ 0`), the last point is dropped, since it lies on or above the line that skips it. Then it builds the **upper chain** from right to left with the same rule. Each chain ends where the other begins, so the last point of each is removed before joining them. The output lists the corners counter-clockwise, starting from the first point in sorted order (the leftmost, lowest on a tie). Points on an edge are dropped, so only true corners remain. The points are typed as `x y` pairs, coordinates 0 to 20.

### The default input

Nine points (the "a few inside" preset). Sorted: (2, 3), (4, 13), (6, 10), (8, 7), (9, 1), (11, 16), (12, 9), (15, 4), (17, 11).

Lower chain: (4, 13), (6, 10) and (8, 7) are each dropped in turn, because each makes a right turn with the next point. (9, 1) stays. (11, 16) is dropped by (12, 9), which is dropped by (15, 4). The lower chain is (2, 3), (9, 1), (15, 4), (17, 11).

Upper chain, from (17, 11): (15, 4) and (12, 9) are dropped; (11, 16) stays; (9, 1), (8, 7) and (6, 10) are each dropped in turn, and (4, 13) and (2, 3) stay. The upper chain is (17, 11), (11, 16), (4, 13), (2, 3).

The output is `[2, 3, 9, 1, 15, 4, 17, 11, 11, 16, 4, 13]`: six corners. (8, 7), (12, 9) and (6, 10) are inside.

## Steps

1. Sort the points by `x`, then `y`, and remove duplicates. With two or fewer, return them.
2. For each point from left to right: while the lower chain has two points and `cross(second-last, last, p) ≤ 0`, drop the last; then add `p`.
3. Do the same from right to left for the upper chain.
4. Return the lower chain without its last point, then the upper chain without its last point.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n log n)`: the sort; the chains are `O(n)` |
| Extra space | `O(n)` for the chains |

Each point is added to a chain once and dropped at most once, so building both chains is linear.

## Pitfalls

- **`< 0` instead of `≤ 0`.** Points on an edge then stay as fake corners; the "a square" preset keeps (10, 4) and (4, 10) that way.
- **Duplicates.** Two copies of one point give a zero cross product in odd places; remove them first.
- **Overflow.** The cross product multiplies coordinate differences; large coordinates need 64-bit integers.

## When to use it

Use it to wrap a set of points: fences, collision outlines, the farthest pair of points, or as the first step of any question about a shape's outside. The linked problem, Tree Fence, asks for the area inside the hull of up to 10⁵ trees.

## Related topics

- [Segment intersection](learn:math-geometry/segment-intersection): the same cross product, used to test whether two segments meet.
- [Monotonic stack](learn:searching/monotonic-stack): each chain is a stack that pops while a condition fails.
- [Merge Sort](learn:sorting/merge-sort): the sort that sets the `O(n log n)` cost.
