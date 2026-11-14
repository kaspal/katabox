## The idea

Some graphs have only two kinds of edge: free ones (weight 0) and ones that cost a single step (weight 1). Think of a grid where walking along a corridor is free but breaking a wall costs 1. Plain BFS counts edges, not weight; Dijkstra works, but its priority queue is more than the problem needs. **0-1 BFS** gets the same answer with a **deque** (a double-ended queue).

The sample starts at node 0 with distance 0. It takes the node `u` at the front of the deque and looks at each edge `u – v` with weight `w`, in adjacency-list order. If `v` has no distance yet, or `dist[u] + w` beats it, `v` gets the new distance. Then the trick: a **0-edge** puts `v` at the **front**, because it is exactly as far as `u` and should be handled next; a **1-edge** puts `v` at the **back**, one step further away. This keeps the deque in order: nodes at distance `d` at the front, nodes at `d + 1` behind them, never anything else. So, like in Dijkstra, the first time a node leaves the deque its distance is final.

Unlike plain BFS, a node may be queued **more than once**: it can first be reached by a 1-edge and later improved by a 0-edge. The later copy, when it comes out, finds nothing left to improve.

### The default input

Six nodes, undirected: `0 1 1, 1 2 1, 2 5 1, 0 3 0, 3 4 0, 4 5 1, 3 1 1`. From 0, node 1 costs 1 and goes to the back; node 3 is free and goes to the front, ahead of 1. Node 3 comes out next and puts 4 at the front for free (distance 0); its edge to 1 would give 0 + 1 = 1, no better. Node 4 sends 5 to the back with distance 1. Then 1 comes out and gives 2 the distance 2, and nodes 5 and 2 improve nothing.

The nodes leave the deque as 0, 3, 4, 1, 5, 2, and the output is `[0, 1, 2, 0, 0, 1]`. Node 5 costs 1 along 0 → 3 → 4 → 5, even though that route has three edges.

## Steps

1. Set every distance to `-1`, the start's to `0`, and put the start in the deque.
2. Take the node `u` at the front.
3. For each edge `u – v` with weight `w`: if `v` is unknown or `dist[u] + w < dist[v]`, set `dist[v] = dist[u] + w` and add `v` at the front when `w` is 0, at the back when it is 1.
4. Repeat until the deque is empty; unreached nodes stay `-1`.

## Cost

| Operation | Cost |
|---|---|
| Whole search | `O(n + m)`: deque operations are `O(1)` and each node is settled once |
| Extra space | `O(n)` for the distances and the deque |

## Pitfalls

- **Marking nodes as seen on discovery, as plain BFS does.** A node found through a 1-edge must still accept a cheaper 0-edge later.
- **Pushing 0-edges to the back.** The deque loses its order and the first pop of a node is no longer its final distance.
- **A slow deque.** The TS sample uses `shift` and `unshift` on an array, which move every element; that is fine for a dozen nodes, but real code needs a true deque.

## When to use it

Use 0-1 BFS when moves are either free or cost one unit: the fewest walls to break, the fewest direction changes, or the fewest reversed one-way streets. With more than two weight values, use Dijkstra.

## Related topics

- [Breadth-first search](learn:graphs/bfs): the same idea when every edge costs 1.
- [Dijkstra's shortest paths](learn:graphs/dijkstra): the general tool for any non-negative weights.
- [Deque and monotonic queue](learn:linear/deque-monotonic-queue): the double-ended queue that drives the search.
- Grid BFS (coming soon): shortest routes over the cells of a grid.
