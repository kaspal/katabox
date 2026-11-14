## The idea

A **directed** graph has a cycle when you can follow its arrows from some node and come back to it. A depth-first search finds one by giving every node one of three colours:

- **white**: not visited yet;
- **grey**: entered but not finished; the grey nodes are exactly the current path on the call stack;
- **black**: finished; everything reachable from it has been explored.

While exploring `u`, an edge to a **grey** node points back to a node on the current path. That is a **back edge**: the path from that node down to `u`, plus the edge, closes a loop, so the graph has a cycle. An edge to a **black** node is harmless: that node was fully explored and none of its paths led back to the current path, so going there again cannot close a loop. Edges to white nodes are followed as usual.

The sample starts a search from every node that is still white, in index order, and outputs `[1]` as soon as it meets a back edge, or `[0]` when all nodes turn black.

### The default input

Seven nodes, directed, with edges `0→1, 1→2, 0→3, 3→4, 4→5, 5→3, 2→6`. From 0, DFS goes grey through 1, 2 and 6. Node 6 has no edges and turns black; so do 2 and 1 on the way back. Then 0 follows its second edge to 3, and the path grows to 0, 3, 4, 5, all grey. Node 5's edge leads to 3, which is grey: a back edge, and the loop 3 → 4 → 5 → 3. The output is `[1]`.

## Steps

1. Colour every node white.
2. For each node `s` in index order, if it is white, call `visit(s)`.
3. `visit(u)`: colour `u` grey. For each edge `u → v`: if `v` is grey, report a cycle; if `v` is white, visit it, and report a cycle if that visit found one.
4. When `u`'s edges run out, colour it black and return "no cycle".
5. If every search returns without finding a cycle, the graph has none.

## Cost

| Operation | Cost |
|---|---|
| Whole check | `O(n + m)`: each node turns grey and black once, each edge is looked at once |
| Extra space | `O(n)` for the colours and the call stack |

## Pitfalls

- **Only two colours.** With just "seen" and "unseen", an edge into an already finished branch looks like a cycle. In a graph like `0→1, 0→2, 1→2`, the edge `1→2` reaches a seen node but there is no loop.
- **Starting only from node 0.** A cycle may sit in a part of the graph that 0 cannot reach; start a search from every white node.
- **Undirected graphs work differently.** There every edge leads straight back to the node you came from, and that is not a cycle. You must remember each node's parent and ignore that one edge, or use union-find and report a cycle when an edge joins two nodes already in the same group.

## When to use it

Use it before trusting a dependency graph: build systems, course prerequisites, spreadsheet formulas and task schedules all break when a cycle sneaks in. It is also the check that tells you a topological order exists.

## Related topics

- [Depth-first search](learn:graphs/dfs): the walk that does the colouring.
- [Topological sort](learn:graphs/topological-sort): ordering a graph that has no cycle.
- [Floyd cycle detection](learn:linear/floyd-cycle): finding a loop in a linked list with two pointers.
- [Union-find](learn:trees/union-find): spotting cycles in undirected graphs edge by edge.
