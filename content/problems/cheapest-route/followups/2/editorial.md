## Idea

Layer the graph. Keep two distances per town: `dist[v][0]` with the pass unused and
`dist[v][1]` with it spent. Start at `dist[start][0] = 0` and run Dijkstra over these
2·n states. Leaving an unused state along a road `[u, v, t]` gives two moves: pay `t` and
stay unused, or spend the pass and reach `dist[v][1]` at no cost. From a spent state every
road costs its toll. All weights are non-negative, so Dijkstra is correct, and the answer
is `min(dist[goal][0], dist[goal][1])`, or `-1` if both are infinite.

## Complexity

O((n + m) log(n + m)) time for `m` roads with a binary heap, and O(n + m) extra memory for
the adjacency lists and the heap.
