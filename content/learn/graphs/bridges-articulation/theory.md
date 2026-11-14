## The idea

In an undirected graph, a **bridge** is an edge whose removal splits a connected piece in two, and an **articulation point** (or cut vertex) is a node whose removal does the same. They are the single points of failure of a network. One DFS finds them all.

The DFS numbers nodes as it enters them: `tin[u]` is the **entry time**. It also computes the **low-link** `low[u]`: the smallest entry time that `u`'s DFS subtree can reach using its tree edges downwards plus **one back edge** to a node already visited. The sample sets `low[u] = tin[u]` on entry, lowers it to `tin[v]` for each back edge `u – v` (skipping the edge to the parent), and after each child `v` returns, lowers it to `low[v]`.

The tests are simple once `low` is known. For a tree edge `u – v`:

- If `low[v] > tin[u]`, nothing below `v` can climb back to `u` or above, so `u – v` is a **bridge**.
- If `low[v] >= tin[u]` and `u` is not the root, the subtree of `v` hangs from `u` alone, so `u` is a **cut vertex**.
- The **root** of a DFS is a cut vertex only if it has **two or more DFS children**: they could not reach each other without it.

The output lists the bridges as sorted `(smaller, larger)` pairs, then a `-1` separator, then the cut vertices in ascending order.

### The default input

Seven nodes, undirected: a triangle `0 – 1 – 2`, the link `2 – 3`, a triangle `3 – 4 – 5`, and a tail `5 – 6`. The DFS enters the nodes in number order, so `tin` is 0 to 6. In node 2 the back edge to 0 gives `low[2] = 0`; in node 5 the back edge to 3 gives `low[5] = 3`, and that spreads to `low[4] = 3` and `low[3] = 3`. Node 6 is a leaf with `low[6] = 6`.

Back in 5: `low[6] = 6 > tin[5] = 5`, so `5 – 6` is a bridge and 5 a cut vertex. In 3: `low[4] = 3 >= tin[3]`, so 3 is a cut vertex, though `3 – 4` is not a bridge. In 2: `low[3] = 3 > tin[2] = 2`, so `2 – 3` is a bridge and 2 a cut vertex. Root 0 has one DFS child, so it is not a cut vertex. The output is `[2, 3, 5, 6, -1, 2, 3, 5]`.

## Steps

1. For every node not yet visited, start a DFS with no parent.
2. On entering `u`: `tin[u] = low[u] = timer`, then add 1 to the timer.
3. For each neighbour `v` other than the parent: if visited, `low[u] = min(low[u], tin[v])`; otherwise visit it, count a child, set `low[u] = min(low[u], low[v])` and apply the bridge and cut tests.
4. After the loop, a root with two or more children is a cut vertex.

## Cost

| Operation | Cost |
|---|---|
| One DFS | `O(n + m)` |
| Extra space | `O(n)`: `tin`, `low` and the recursion |

## Pitfalls

- **Treating the edge to the parent as a back edge.** Then every `low` drops to the parent and no bridge is ever found.
- **Parallel edges.** The sample skips the parent by node, so a doubled edge `u – v` would be skipped twice and wrongly reported as a bridge; skip by edge id to support them.
- **Applying the `>=` cut test to the root.** The root has no entry time below its own, so it needs the child count.

## When to use it

Use it to find weak spots: roads or cables whose failure cuts a network, or routers that must not go down.

## Related topics

- [Depth-first search](learn:graphs/dfs): the walk that sets entry times.
- [Strongly connected components](learn:graphs/scc): the directed cousin, also built on DFS.
- [Connected components](learn:graphs/connected-components): what a bridge or cut vertex splits.
