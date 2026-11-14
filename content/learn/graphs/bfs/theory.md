## The idea

**Breadth-first search** (BFS) explores a graph in rings: first the start node, then every node one edge away, then every node two edges away, and so on. Because it never reaches a node by a longer route before a shorter one, the first time it meets a node is along a path with the fewest edges. That makes BFS the shortest-path algorithm for **unweighted** graphs.

The engine is a **queue**, first in, first out. The sample starts at node 0 with distance 0. It takes the node at the front, looks at its neighbours in adjacency-list order, and every neighbour not seen yet gets distance "mine + 1" and joins the back of the queue. A node's distance is set the moment it is discovered, which also marks it as seen, so no node enters the queue twice. Nodes BFS never reaches keep the distance `-1`.

### The default input

Nine nodes form a 3 × 3 grid, numbered row by row: 0, 1, 2 on top, 3, 4, 5 in the middle, 6, 7, 8 at the bottom, with edges between side-by-side and stacked nodes. Visiting 0 discovers 1 and 3 (distance 1). Visiting 1 discovers 2 and 4 (distance 2); visiting 3 finds 0 and 4 already seen and discovers 6 (distance 2). Then 2 discovers 5 and 4 discovers 7 (distance 3), and finally 5 discovers 8 (distance 4).

The nodes leave the queue as 0, 1, 3, 2, 4, 6, 5, 7, 8, and the output is `[0, 1, 2, 1, 2, 3, 2, 3, 4]`: each node's distance from the top-left corner, counted in edges.

## Steps

1. Set every distance to `-1`, set the start's distance to `0` and put it in the queue.
2. While the queue is not empty, take the node `u` at its front.
3. For each neighbour `v` of `u`, in list order: if `v`'s distance is still `-1`, set it to `u`'s distance plus 1 and add `v` to the back of the queue.
4. When the queue is empty, every reachable node has its distance; the rest stay `-1`.

## Cost

| Operation | Cost |
|---|---|
| Whole search | `O(n + m)`: each node is queued once, each list is read once |
| Extra space | `O(n)` for the distances and the queue |

## Pitfalls

- **Marking a node when it leaves the queue instead of when it joins.** Then a node can be queued several times by different neighbours before it is processed, wasting work and, on dense graphs, memory.
- **A slow queue.** `list.pop(0)` in Python and `shift()` in JavaScript move every element; use `collections.deque` or a head index, as the samples do.
- **Using BFS on weighted edges.** Fewest edges is not lowest total weight; for weights you need Dijkstra.

## When to use it

Use BFS for the fewest moves, hops or steps between two states: shortest routes in unweighted networks, word ladders, puzzles where every move costs the same, and spreading processes such as infection or rumours, where the ring number is the time step.

## Related topics

- [Queue and circular buffer](learn:linear/queue-ring-buffer): the queue that drives the search.
- [Level order](learn:trees/level-order): the same walk on a tree, where every node has one route.
- [Dijkstra's shortest paths](learn:graphs/dijkstra): shortest paths when edges have weights.
- 0-1 BFS (coming soon): BFS with a deque when weights are only 0 or 1.
