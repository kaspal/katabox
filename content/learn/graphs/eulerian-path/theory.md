## The idea

An **Eulerian path** walks along every edge of a graph exactly once; nodes may be visited as often as needed. It is the puzzle of drawing a figure without lifting the pen or retracing a line. If the walk ends where it started, it is an **Eulerian circuit**.

Whether one exists depends only on **degrees** (the number of edges at each node) and connectivity. Every time the walk passes through a node it uses one edge in and one edge out, so in-between nodes need an even degree. Only the two ends may be odd. So an undirected graph has an Eulerian path only if it has **0 or 2 odd-degree nodes** and all its edges are in one connected piece: with 0 the path is a circuit and can start anywhere, with 2 it must start at one odd node and end at the other.

The sample checks the degrees first and returns `[-1]` if the count of odd nodes is not 0 or 2. It starts at the first odd node, or, when there is none, at the `u` of the first edge. Then it runs **Hierholzer's algorithm** with a **stack**. Look at the node on top: if it still has an unused edge, mark the edge used and push the other end. If it has none, pop it onto the path. A per-node **next pointer** into the edge list means used edges are passed over only once. Getting stuck early is not a mistake: the popped nodes form the tail of the route, and loops found later are spliced in before them. That is why the path comes out **reversed** and the sample flips it at the end. If the path has fewer than `m + 1` nodes, some edges were never reached (they sit in another piece of the graph) and the answer is `[-1]`.

### The default input

Five nodes, undirected: `0 1, 1 2, 2 3, 3 0, 0 2, 1 4, 4 2`, a square with a diagonal and a roof. The degrees are 3, 3, 4, 2, 2: nodes 0 and 1 are odd, so the walk starts at 0. Following each node's first unused edge it pushes 0, 1, 2, 3, 0, 2, 4, 1 and uses all seven edges without getting stuck. Node 1 has nothing left, so it is popped first, then 4, 2, 0, 3, 2, 1, 0. Reversed, the output is `[0, 1, 2, 3, 0, 2, 4, 1]`: eight nodes, seven edges, ending at the other odd node.

## Steps

1. Build adjacency lists that store each edge's id, and count the odd-degree nodes; if there are not 0 or 2, return `[-1]`.
2. Push the start node.
3. While the stack is not empty, look at its top `u`. Skip its used edges; if one is left, mark it used and push its other end. Otherwise pop `u` onto the path.
4. If the path has `m + 1` nodes, return it reversed; otherwise return `[-1]`.

## Cost

| Operation | Cost |
|---|---|
| Whole run | `O(n + m)`: each edge is used once and each pointer only moves forward |
| Extra space | `O(n + m)`: the lists, the used flags, the stack and the path |

## Pitfalls

- **Walking greedily and printing as you go.** A walk can return to its start before covering everything; the stack is what splices the missing loops in.
- **Marking nodes instead of edges.** Nodes are revisited all the time; it is the edges that may be used only once, which is why each list entry carries the edge id.

## When to use it

Use it for routes that must cover every street or link once: snow ploughs, mail delivery, drawing machines.

## Related topics

- [Stack](learn:linear/stack): the stack that holds the current walk.
- [Depth-first search](learn:graphs/dfs): Hierholzer is a DFS over edges instead of nodes.
- [Connected components](learn:graphs/connected-components): every edge must lie in one component.
