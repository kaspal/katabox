## The idea

**Bellman-Ford** finds the cheapest route from node 0 to every node of a directed graph, and unlike Dijkstra it accepts **negative** weights. It does not choose a clever order. It simply walks the whole edge list again and again, relaxing every edge `u → v` with weight `w`: if `u` is reached and `dist[u] + w` beats `dist[v]`, lower `dist[v]`.

Why do `n − 1` rounds suffice? A cheapest path that does not loop visits each node at most once, so it has at most `n − 1` edges. After round 1, every node whose cheapest path has one edge has its final distance; after round `k`, every node whose path has at most `k` edges. Each round makes at least one more edge of every path final, so `n − 1` rounds settle them all. The sample stops early after a round in which nothing changed, since later rounds would change nothing either.

The **order** of the edge list decides how much one round achieves: if the edges of a path appear in path order, a single round carries the distance all the way along; in reverse order, each round moves it one edge. The rounds differ, but the final distances do not.

If a **negative cycle** is reachable from 0, going round it again and again lowers the cost forever, and no answer exists. One extra pass over the edges detects it: if any edge can still improve a distance after `n − 1` rounds, the sample outputs `[1]`. Otherwise it outputs `[0]` followed by the distances, with `999` meaning unreachable.

### The default input

Five nodes, edges `3→4 2, 1→3 −2, 0→1 4, 0→2 1, 2→1 2, 2→3 5`. Round 1 skips the first two edges (3 and 1 are not reached yet), then sets 1 to 4, 2 to 1, improves 1 to 3 through 2, and sets 3 to 6. Round 2 sets 4 to 8, then lowers 3 to 1 through the negative edge. Round 3 lowers 4 to 3. Round 4 changes nothing, so the loop stops, and the check pass finds no improvement.

The output is `[0, 0, 3, 1, 1, 3]`: no negative cycle, and distances 0, 3, 1, 1, 3. The path to 4 is 0 → 2 → 1 → 3 → 4, four edges, which is why it needed several rounds.

## Steps

1. Set `dist[0] = 0` and every other distance to `999`.
2. Repeat up to `n − 1` times: relax every edge in list order; stop early if no distance changed.
3. Run one more pass: if any edge still improves a distance, output `[1]`.
4. Otherwise output `[0]` and the distances.

## Cost

| Operation | Cost |
|---|---|
| Best case | `O(m)` when one round settles everything |
| Average and worst | `O(n · m)`: up to `n − 1` rounds over `m` edges |
| Extra space | `O(n)` for the distances |

## Pitfalls

- **Relaxing from an unreached node.** `999 + (−2)` would look like a real distance; skip edges whose start is still unreachable.
- **Reading distances after a negative cycle.** They are meaningless; check first.

## When to use it

Use it when weights can be negative: refunds, currency exchange, or differences between constraints. With only non-negative weights, Dijkstra is much faster.

## Related topics

- [Dijkstra's shortest paths](learn:graphs/dijkstra): faster, but non-negative weights only.
- [Cycle detection](learn:graphs/cycle-detection): finding any cycle in a directed graph.
- [Floyd-Warshall](learn:graphs/floyd-warshall): shortest paths between every pair of nodes.
