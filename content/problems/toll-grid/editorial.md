## Approach: Dijkstra on (cell, coupon used)

Without the coupon, the cheapest crossing is a shortest path where entering a cell costs its toll, and all costs are positive: Dijkstra's algorithm. The coupon is a one-time choice, so put it **into the state**: `(r, c, used)`, two layers of the grid.

From `(r, c, used)` to a neighbour `(r', c')`:

- pay `tolls[r'][c']` and stay in the same layer;
- if `used` is false, alternatively pay `tolls[r'][c'] // 2` and move to the `used` layer.

The start has two states as well: `(0, 0, false)` costs `tolls[0][0]` and `(0, 0, true)` costs `tolls[0][0] // 2`. Run Dijkstra from both with a min-heap of `(cost, state)`; the answer is `min(cost(end, false), cost(end, true))`.

Every route with at most one coupon is a path in this layered graph with the same cost, and every path in it is a valid route, so the shortest path is the answer. All edge costs are non-negative (half of a toll ≥ 1 is ≥ 0), so Dijkstra applies. Entering a cell twice never helps, because tolls are positive.

### Walkthrough

Example 3: in the unused layer the cheapest way to the bottom-right is 153 (one 100 and one 50). In the used layer, halving the 100 at `(0, 1)` gives 1 + 50 + 1 + 50 + 1 = 103, cheaper than halving a 50 (1 + 100 + 1 + 25 + 1 = 128). Answer 103.

**Complexity:** O(V log V) with V = 2 · rows · cols states and 4 moves each (up to 2 · 22 500 states). Re-relaxing every cell over and over until nothing changes (Bellman–Ford style) is O(V²) on a winding route.
