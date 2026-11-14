## The idea

**Floyd-Warshall** answers every shortest-path question at once: the cheapest route from each node to each other node. It works on a **distance matrix**: cell `dist[i][j]` starts as the edge weight from `i` to `j`, `0` on the diagonal, and `999` (meaning "no path known") everywhere else.

The algorithm is **dynamic programming** over which nodes a route may pass through. Before round `k`, each cell holds the best route whose intermediate stops all come from nodes `0 .. k-1`. Round `k` allows node `k` as a stop too, and only one question is new: is going `i → k → j` cheaper than the best route without `k`? So for every pair it tries `dist[i][k] + dist[k][j] < dist[i][j]` and keeps the smaller value. After the last round every node is allowed, and the matrix holds the true shortest distances.

That is why `k` must be the **outer** loop. The two halves `i → k` and `k → j` must already be optimal for stops below `k` when they are combined. With `k` inside the `i` and `j` loops, a cell would be finalised before the routes it depends on were ready.

### The default input

Five nodes, directed: `0 1 4, 1 2 3, 0 2 9, 2 3 1, 3 0 2, 1 4 7, 3 4 2`. In round 0, node 3 learns routes through 0: `3 → 1` costs 2 + 4 = 6 and `3 → 2` costs 11. Round 1 lowers `0 → 2` from 9 to 4 + 3 = 7, gives `0 → 4` the value 11 and improves `3 → 2` to 9. Round 2 gives `0 → 3` 8 and `1 → 3` 4. Round 3 is the busiest, with six improvements; for example `0 → 4` drops to 10 and node 2 can now reach 0 (3), 1 (7) and 4 (3). Round 4 changes nothing: node 4 has no outgoing edges, so no route can pass through it.

The output is the whole matrix, row by row: `[0, 4, 7, 8, 10, 6, 0, 3, 4, 6, 3, 7, 0, 1, 3, 2, 6, 9, 0, 2, 999, 999, 999, 999, 0]`. The last row stays `999` except its diagonal: nothing leaves node 4.

## Steps

1. Fill the matrix: `0` on the diagonal, the edge weight for each edge, `999` elsewhere.
2. For each `k` from `0` to `n - 1`:
3. For every pair `i, j`: if `dist[i][k] + dist[k][j]` is smaller than `dist[i][j]`, replace it.
4. Return the matrix; a cell still at `999` has no path.

## Cost

| Operation | Cost |
|---|---|
| Whole run | `O(n³)`: three nested loops over the nodes |
| Extra space | `O(n²)` for the matrix, updated in place |

## Pitfalls

- **Putting `k` in the wrong loop.** The code still runs and returns numbers, but some of them are too large.
- **An "infinity" that overflows or is too small.** Adding two infinities must not wrap around, and the marker must exceed every real path. The sample's `999` works because weights and node counts are small.
- **Negative cycles.** Negative edges are fine, but if a cycle has negative total weight, some diagonal cell turns negative and the distances mean nothing. Checking the diagonal detects it.

## When to use it

Use Floyd-Warshall when the graph is small (a few hundred nodes) and you need distances between many pairs: route tables, "closest town to everyone" questions, or reachability between all pairs. For one start node, Dijkstra or BFS is much cheaper.

## Related topics

- [Graph representations](learn:graphs/graph-representations): the adjacency matrix the algorithm runs on.
- [Dijkstra's shortest paths](learn:graphs/dijkstra): shortest paths from a single start, much faster on sparse graphs.
- [Bellman-Ford](learn:graphs/bellman-ford): another relaxation algorithm, for one start and negative weights.
