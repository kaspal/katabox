## The idea

The **diameter** of a tree is the length of its longest path, counted in edges. Here the tree is a general one (a node can have any number of neighbours), given as a list of parents: the parent of node `k` is the `k`-th number, always a smaller node, so node 0 is the root.

A neat trick needs just two **breadth-first searches** (BFS), each `O(n)`:

1. BFS from any node, here node 0, and take the **farthest** node it reaches. That node is always one end of some longest path.
2. BFS again from that end. The farthest distance it reaches is the diameter.

In a tree there is exactly one path between two nodes, so BFS distances are the true ones.

### Why the first end is right

If the farthest node `u` were not an end of any longest path, the route from 0 to `u` could be joined with part of a longest path to make an even longer one, which is impossible. Node 0 itself, though, may lie in the middle.

### The default input

The parents 0, 1, 2, 3, 1, 2, 3, 0 give 9 nodes: a spine 0 – 1 – 2 – 3 – 4 with a leg on 1, 2 and 3 (nodes 5, 6, 7) and node 8 hanging from 0, the shape of a caterpillar. The first BFS from 0 finds node 4 at distance 4 (node 7 is just as far but is reached later). The second BFS from 4 reaches node 8 at distance 5, so the diameter is 5: the path 4 – 3 – 2 – 1 – 0 – 8.

## Steps

1. Build neighbour lists: for each node `k ≥ 1`, link `k` and its parent both ways.
2. BFS from node 0 with a queue, recording each node's distance; keep the farthest node, `end`.
3. BFS again from `end`, recording distances from it.
4. The largest distance in the second BFS is the diameter, in edges.

## Cost

| Operation | Cost |
|---|---|
| Building the neighbour lists | `O(n)` |
| Each BFS | `O(n)`: a tree has `n − 1` edges |
| Extra space | `O(n)`: the lists, the distances and the queue |

## Pitfalls

- **Counting nodes instead of edges.** The path 4 – 3 – 2 – 1 – 0 – 8 has 6 nodes but 5 edges. Check which one the question asks for.
- **Using only child links.** The longest path often climbs up and goes down another branch; the BFS needs links in both directions.
- **Using it on graphs with cycles.** The two-BFS trick relies on a tree's single path between nodes; on general graphs it can give a wrong answer.

## When to use it

Use the diameter for "how far apart can two points be" questions on a tree: the longest trail in a park's path network, the worst-case delay between two machines in a tree network.

## Related topics

- [Level order](learn:trees/level-order): BFS with a queue on a binary tree.
- [Queue and circular buffer](learn:linear/queue-ring-buffer): the queue each BFS runs on.
- Breadth-first search on graphs (coming soon): the same search when nodes can be reached in several ways.
