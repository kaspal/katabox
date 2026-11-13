## Approach: Andrew's monotone chain, then the shoelace formula

1. Sort the trees by x, then by y (duplicates end up next to each other).
2. **Lower hull:** go left to right. Before adding a tree, pop the last corner while the last two corners and the new tree do not make a strict left turn: `cross(h[−2], h[−1], p) ≤ 0`. Then push the tree.
3. **Upper hull:** the same, right to left.
4. Join them (dropping each half's last point, which starts the other half). The corners are now in counterclockwise order, with no collinear points.
5. **Twice the area** is `Σ (x_i · y_{i+1} − x_{i+1} · y_i)` around the hull (shoelace). With fewer than 3 corners, the answer is 0.

Using `≤ 0` when popping removes collinear and duplicate points, so they never become corners.

### Why it works

On the lower hull, consecutive edges always turn left. If adding a new tree would make the chain turn right or go straight, the last corner lies on or above the segment joining its neighbours, so it is not an extreme point.

**Complexity:** O(n log n) for the sort, then O(n): each tree is pushed and popped at most once per half. Gift wrapping (from each corner, scan every tree for the next one) is O(n·h), which is O(n²) when most trees are corners. Coordinates up to 10⁷ keep every cross product and the area below 2⁵³.
