## The idea

**Depth-first search** (DFS) explores a graph by always going deeper: from the current node, step to the first neighbour not seen yet, and from there to its first unseen neighbour, and so on. Only when a node has no unseen neighbours left does DFS back up one step and try the next neighbour of the node before it.

The sample writes this as a recursive function `visit(u)`. The **call stack** is the current path: the frames waiting on it are exactly the nodes from the start down to the node being explored. Each node gets two moments. It **enters** when `visit` is called on it (it is marked seen and added to the entry order) and it **finishes** when `visit` returns, after all of its descendants have finished. The output is the entry order followed by the finish order.

### The default input

Eight nodes, undirected: a chain 0 – 1 – 2 – 3 – 4, a second chain 0 – 5 – 6 – 7, and one extra edge 2 – 6, written last. From 0, DFS enters 1, 2, 3 and 4; the stack now holds the path 0, 1, 2, 3, 4. Node 4 has nothing new, so it finishes, then 3 finishes. Back at 2, the next neighbour is 6, so DFS enters 6, then 5 (6's first neighbour), then 7. Node 5 is reached through 2 and 6, not straight from 0: by the time 0 looks at 5, it is already seen.

Entry order: 0, 1, 2, 3, 4, 6, 5, 7. Finish order: 4, 3, 5, 7, 6, 2, 1, 0. The start always finishes last.

## Steps

1. Mark every node unseen, then call `visit(0)`.
2. `visit(u)`: mark `u` seen and record it in the entry order.
3. For each neighbour `v` of `u`, in list order, if `v` is unseen, call `visit(v)` and wait for it to return.
4. When the neighbours run out, record `u` in the finish order and return.
5. Output the entry order, then the finish order.

## Cost

| Operation | Cost |
|---|---|
| Whole search | `O(n + m)`: each node is entered once, each list is read once |
| Extra space | `O(n)`: the seen marks and up to `n` frames on the stack |

## Pitfalls

- **Recursion depth.** A long path means one frame per node: a chain of 10 000 nodes overflows Python's default limit of about 1 000. Raise the limit or switch to an explicit stack.
- **An explicit stack is not automatically the same order.** Pushing all neighbours at once and popping them reverses their order and loses the finish times; to match the recursion, keep each node's position in its list on the stack too.
- **Marking too late.** Mark a node seen when you enter it, or a cycle sends DFS around forever.

## When to use it

Use DFS when the structure of paths matters more than their length: reachability, cycle detection, topological order from finish times, connected components, maze generation, and backtracking searches that try one choice fully before the next.

## Related topics

- [Recursion and the call stack](learn:foundations/recursion-call-stack): the stack that holds the current path.
- [Pre-, in- and post-order](learn:trees/tree-traversals): entry and finish order on a tree are pre- and post-order.
- [Cycle detection](learn:graphs/cycle-detection): DFS with three colours.
- [Breadth-first search](learn:graphs/bfs): the ring-by-ring alternative.
