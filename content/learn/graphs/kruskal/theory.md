## The idea

**Kruskal's algorithm** builds a minimum spanning tree without growing it from a start node. It sorts all the edges from cheapest to most expensive and walks down the list: an edge is **taken** if it joins two pieces that are not yet connected, and **skipped** if both ends are already in the same piece, because then it would close a cycle.

Why is the cheapest useful edge always safe? Split the nodes into any two groups. Every spanning tree must use at least one edge crossing between them, and swapping a pricier crossing for the cheapest one never makes a tree worse. When Kruskal reaches an edge joining two separate pieces, nothing cheaper crosses between those pieces (it would have joined them already), so the edge belongs to some best tree. This is the **cut property**, stated informally.

To ask "same piece?" quickly, the sample keeps a **union-find** structure. `find(x)` follows parent links up to the piece's root, and **path halving** shortens the trail on the way: each visited node is pointed at its grandparent. Taking an edge `u – v` sets the root of `u` to point at the root of `v`. The sort key is `(weight, u, v)`, so ties go to the smaller first endpoint, then the smaller second one. The output is the total weight, then the chosen edges as `u, v` pairs, in the order they were taken and as they were written in the input.

### The default input

The same ring with two chords as in Prim: `0 1 4, 1 2 2, 2 3 5, 3 4 1, 4 5 3, 5 0 2, 0 3 8, 1 4 6`. Sorted, the list starts `3 4 1`, `1 2 2`, `5 0 2`, `4 5 3`, `0 1 4`. Those five each join two separate pieces and are taken: the totals run 1, 3, 5, 8, 12, and after `0 1 4` every node is in one piece. The rest, `2 3 5`, `1 4 6` and `0 3 8`, all find both ends with the same root and are skipped.

The output is `[12, 3, 4, 1, 2, 5, 0, 4, 5, 0, 1]`: total 12, the same tree Prim finds, but its edges come out in weight order.

## Steps

1. Give every node its own piece: `parent[x] = x`.
2. Sort the edges by `(weight, u, v)`.
3. For each edge `u – v`: find both roots. If they are equal, skip it.
4. Otherwise point one root at the other, add the weight and record the edge.

## Cost

| Operation | Cost |
|---|---|
| Sorting the edges | `O(m log m)`, the dominant part |
| All the finds and unions | close to `O(m)` with path halving |
| Extra space | `O(n + m)`: the parent array and the sorted edge list |

## Pitfalls

- **Testing for cycles by searching the graph each time.** A fresh DFS per edge turns the run quadratic; union-find answers in near-constant time.
- **Expecting one tree from a disconnected graph.** Kruskal never needs a start node, so it returns a **spanning forest**: one tree per component, with fewer than `n - 1` edges in total.
- **Uniting nodes instead of roots.** Writing `parent[u] = v` instead of linking the two roots splits pieces apart.

## When to use it

Use Kruskal when edges arrive as a list, the graph is sparse, or it may be disconnected. Stopping early, before the last joins, gives clusters: the pieces that remain are groups whose links are all cheap.

## Related topics

- [Union-find](learn:trees/union-find): the structure that answers "same piece?".
- [Prim's minimum spanning tree](learn:graphs/prim): the same tree grown from a start node with a priority queue.
- [Connected components](learn:graphs/connected-components): the pieces Kruskal's forest spans.
