## The idea

A **topological order** lists the nodes of a directed graph so that every edge `u → v` points forwards: `u` comes before `v`. Think of tasks and their prerequisites: a valid order is a schedule in which nothing starts before the things it depends on.

**Kahn's algorithm** builds the order from the front. A node's **in-degree** is the number of edges pointing into it. A node with in-degree 0 waits for nothing, so it can go first. Put all such nodes in a FIFO queue, in index order. Take one out, write it, and remove its outgoing edges by lowering each target's in-degree by one. Any target that drops to 0 has just lost its last prerequisite and joins the back of the queue.

If the graph has a **cycle**, the nodes on it never reach in-degree 0: each one waits on the node before it in the loop, and none of them can go first. They never enter the queue, so the order comes out **shorter than `n`**. Comparing its length with `n` is a free cycle check.

### The default input

Seven nodes, directed, with edges `0→2, 1→2, 1→3, 2→4, 3→4, 4→5, 3→6`. The in-degrees are 0, 0, 2, 1, 2, 1, 1, so the queue starts as 0, 1. Taking 0 lowers 2 to 1. Taking 1 lowers 2 to 0 and 3 to 0, so both join the queue. Taking 2 lowers 4 to 1; taking 3 lowers 4 to 0 and 6 to 0. Then 4 frees 5, and the queue hands out 6 before 5 because 6 joined first.

The output is `0, 1, 2, 3, 4, 6, 5`: all seven nodes, so the graph has no cycle.

## Steps

1. Count each node's in-degree by walking every edge once.
2. Put every node with in-degree 0 in the queue, in index order.
3. While the queue is not empty, take its front node `u` and write it.
4. For each edge `u → v`, lower `v`'s in-degree by one; if it reaches 0, add `v` to the back of the queue.
5. If fewer than `n` nodes were written, the graph has a cycle.

## Cost

| Operation | Cost |
|---|---|
| Whole sort | `O(n + m)`: in-degrees take one pass over the edges, then each node and edge is handled once |
| Extra space | `O(n)` for the in-degrees and the queue |

## Pitfalls

- **Expecting one answer.** A graph usually has many topological orders; this one depends on the queue's rules. Use a priority queue instead if you need the smallest order by index.
- **Ignoring a short result.** Returning the partial order as if it were complete hides the cycle; check the length.
- **Forgetting nodes with no edges.** They have in-degree 0 and must be in the starting queue too.

## When to use it

Use it to schedule anything with dependencies: build steps, course plans, spreadsheet recalculation, or dynamic programming on a directed acyclic graph, where each node needs its predecessors' answers first. The alternative is DFS: write each node when it finishes, then reverse that list. It gives a valid order too, but Kahn's version detects cycles by counting and needs no recursion.

## Related topics

- [Cycle detection](learn:graphs/cycle-detection): the DFS way to prove an order exists.
- [Depth-first search](learn:graphs/dfs): its reversed finish order is also topological.
- [Queue and circular buffer](learn:linear/queue-ring-buffer): the queue of ready nodes.
- [Binary heap and priority queue](learn:trees/binary-heap): for the smallest order by index.
