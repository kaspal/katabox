## The idea

A **minimum spanning tree** (MST) joins all the nodes of a weighted, undirected graph using a subset of its edges, with no cycles and the smallest possible total weight. Picture laying cable between villages: every village must be connected, and you pay for each stretch of cable.

**Prim's algorithm** grows the tree from one node, here node 0. At every step it adds the cheapest edge that joins a node already in the tree to a node outside it. Such an edge can never be a mistake: whatever the best tree looks like, it has to cross from "inside" to "outside" somewhere, and the cheapest crossing is at least as good as any other.

The tool is a **priority queue** of `(weight, node, from)` triples, smallest first, ties broken by the node and then by where it came from. The start enters as `(0, 0, 0)`. When a triple comes out and its node is already in the tree, it is **stale** and skipped; otherwise the node joins the tree, its weight is added to the total, and every edge to a node still outside is pushed. The output is the total weight, then the nodes in the order they joined.

Python uses `heapq`. TypeScript has no priority queue, so the TS sample keeps an array of triples sorted after each push, enough for a dozen nodes.

### The default input

Six nodes, undirected: `0 1 4, 1 2 2, 2 3 5, 3 4 1, 4 5 3, 5 0 2, 0 3 8, 1 4 6`: a ring of six with two chords. From 0 the queue gets `(4, 1, 0)`, `(2, 5, 0)` and `(8, 3, 0)`. The cheapest is node 5 (2). Node 5 offers node 4 at weight 3, so 4 joins next (total 5), then 3 through the edge `3 – 4` of weight 1 (total 6). Now the cheapest crossing is `0 – 1` with 4 (total 10), and finally `1 – 2` with 2 (total 12). The entries `(5, 2, 3)`, `(6, 1, 4)` and `(8, 3, 0)` are stale when they come out.

The output is `[12, 0, 5, 4, 3, 1, 2]`. Only the ring edge `2 – 3` (5) and both chords are left out.

## Steps

1. Push `(0, start, start)`.
2. Pop the smallest triple `(w, v, from)`. If `v` is already in the tree, skip it.
3. Add `v` to the tree, append it to the order and add `w` to the total.
4. For each edge `v – x` with weight `wx` where `x` is outside the tree, push `(wx, x, v)`.
5. Repeat until the queue is empty.

## Cost

| Operation | Cost |
|---|---|
| Whole run with a binary heap | `O(m log m)`: each edge is pushed at most once and popped at most once |
| Extra space | `O(n + m)`: the in-tree flags plus the queue |

## Pitfalls

- **A disconnected graph.** Prim only ever crosses edges from the tree it is growing, so it spans **node 0's component** and silently ignores the rest. For a spanning forest, restart from each unreached node or use Kruskal.
- **Directed graphs.** The spanning tree idea needs undirected edges; directed versions are a different, harder problem.
- **Forgetting to skip stale entries.** Without the in-tree check a node would join twice and its weight would be counted twice.

## When to use it

Use Prim for the cheapest network that connects everything: cables, pipes, roads, or clustering points by cutting the most expensive tree edges. It suits dense graphs given as adjacency lists or matrices.

## Related topics

- [Kruskal's minimum spanning tree](learn:graphs/kruskal): the same tree built edge by edge, cheapest first.
- [Dijkstra's shortest paths](learn:graphs/dijkstra): the same queue, but keyed by total distance instead of one edge.
- [Binary heap and priority queue](learn:trees/binary-heap): the queue behind the algorithm.
