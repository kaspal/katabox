## The idea

**Dijkstra's algorithm** finds the cheapest route from one start node to every other node when each edge has a **non-negative** weight. It grows a set of settled nodes outward in order of distance: at every step it settles the unsettled node with the smallest known distance, then tries to improve its neighbours through it.

The tool is a **priority queue** of `(distance, node)` pairs, smallest distance first, ties broken by the smaller node. When a pair is taken out, its distance is final. Then each edge `u → v` with weight `w` is **relaxed**: if `d + w` beats `v`'s best distance so far, record it and push `(d + w, v)`. A node can sit in the queue several times with different distances; the old, larger entries are **stale**, and when one comes out it is simply skipped (lazy deletion). Nodes never reached report `-1`.

Python has `heapq`. TypeScript has no built-in priority queue, so the TS sample keeps a plain array sorted after each push, which is fine for a dozen nodes; a real program would use a binary heap.

### The default input

Six nodes, undirected: `0 1 7, 0 2 2, 2 1 3, 1 3 1, 2 4 8, 3 4 2, 4 5 3, 3 5 9`. From 0, node 1 gets 7 and node 2 gets 2. Node 2 comes out first and finds a detour: 2 + 3 = 5 beats 7 for node 1, and 4 gets 10. Node 1 (5) gives 3 the distance 6. Then the old entry `(7, 1)` comes out and is skipped. Node 3 (6) improves 4 to 8 and gives 5 the distance 15; node 4 (8) improves 5 to 11. The entries `(10, 4)` and `(15, 5)` are stale too.

The output is `[0, 5, 2, 6, 8, 11]`. The cheapest way to 5 is 0 → 2 → 1 → 3 → 4 → 5: five edges, yet cheaper than any shorter route.

## Steps

1. Set the start's distance to 0, every other to "unknown", and push `(0, start)`.
2. Pop the smallest pair `(d, u)`. If `d` is larger than `u`'s best distance, skip it.
3. For each edge `u → v` with weight `w`: if `v` is unknown or `d + w` is smaller than its distance, set it to `d + w` and push `(d + w, v)`.
4. Repeat until the queue is empty; unknown distances become `-1`.

## Cost

| Operation | Cost |
|---|---|
| Whole run with a binary heap | `O((n + m) log n)`: up to `m` pushes and pops, each `O(log m) = O(log n)` |
| Extra space | `O(n + m)`: distances plus up to one queue entry per relaxation |

## Pitfalls

- **Negative weights.** Settling a node assumes nothing found later can be cheaper, because every remaining route only adds weight. A negative edge breaks that promise: a node can be settled, and then a longer route through a negative edge reaches it more cheaply. Use Bellman-Ford instead.
- **Stopping at the first time a node is seen.** Only popping settles a node; a distance set while relaxing may still improve.

## When to use it

Use Dijkstra for road maps, network routing, game maps with terrain costs, and any search where moves have different non-negative costs. For equal costs, plain BFS is simpler and faster.

## Related topics

- [Binary heap and priority queue](learn:trees/binary-heap): the queue behind the algorithm.
- [Breadth-first search](learn:graphs/bfs): the special case where every edge costs 1.
- [Bellman-Ford](learn:graphs/bellman-ford): shortest paths when weights can be negative.
- A* (coming soon): Dijkstra steered towards a goal by an estimate.
