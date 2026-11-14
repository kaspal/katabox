## The idea

A **flow network** is a directed graph where each edge has a **capacity**: how much can pass through it, like water through a pipe. The **maximum flow** is the most that can travel from a **source** to a **sink** when no edge carries more than its capacity and every other node passes on exactly what it receives. In this topic the source is node 0 and the sink is node `n - 1`.

The sample keeps a **capacity matrix**: `cap[u][v]` is how much more can still go from `u` to `v`, the **residual capacity**. Each round runs a BFS from the source over edges with residual capacity left, scanning a node's neighbours in **node-number order** (one matrix row). If the sink is reached, the BFS parents give a path, an **augmenting path**. Its **bottleneck**, the smallest residual capacity along it, is how much flow it can add. Every edge on the path loses that amount, and the opposite direction **gains** it: `cap[v][u] += push`. When the BFS can no longer reach the sink, the total is the maximum. Always taking a shortest path by BFS is what makes this **Edmonds-Karp**.

The reverse capacity is the clever part. A greedy early path may route flow badly, using an edge that a later path needs. The reverse capacity lets a later path **take that flow back**, sending it a different way, so no early choice is ever final. Because the reverse capacity of `u → v` lives in the cell `cap[v][u]`, this topic's input does not allow both `u → v` and `v → u` in the same graph.

When the search stops, the nodes the BFS still reaches and the rest form a **cut** whose crossing edges are all full. The **max-flow min-cut theorem** says the largest flow equals the smallest total capacity you would have to remove to separate source from sink.

### The default input

Six nodes: `0 1 5, 0 2 4, 1 3 3, 2 4 6, 1 4 2, 3 5 4, 4 5 7`, with source 0 and sink 5. Round 1 finds `0 → 1 → 3 → 5` with bottleneck 3 (the edge `1 → 3`). Round 2 finds `0 → 1 → 4 → 5` with bottleneck 2, which fills `0 → 1`. Round 3 finds `0 → 2 → 4 → 5` with bottleneck 4. Now both edges out of the source are full, the BFS cannot leave node 0, and the output is `[9]`: 3 + 2 + 4. The cut is the two source edges, 5 + 4 = 9.

## Steps

1. Fill `cap[u][v]` with each edge's capacity; the total starts at 0.
2. BFS from the source over cells with `cap[u][v] > 0`, recording parents.
3. If the sink was not reached, return the total.
4. Walk back from the sink to find the bottleneck; subtract it forwards, add it backwards, and add it to the total. Repeat from step 2.

## Cost

| Operation | Cost |
|---|---|
| Whole run | `O(n · m²)`: at most `O(n · m)` augmenting rounds, each a BFS of `O(m)` (the sample's matrix BFS costs `O(n²)` per round) |
| Extra space | `O(n²)` for the matrix |

## Pitfalls

- **Forgetting the reverse capacity.** Without it the algorithm can get stuck below the maximum after an unlucky first path.
- **Choosing paths by DFS.** The number of rounds can then grow with the flow value itself; BFS bounds it by the graph size.
- **Overwriting parallel edges.** A matrix holds one number per pair; two edges `u → v` must have their capacities added.

## When to use it

Use max flow to route as much as possible through limited links (traffic, pipes, data) and for problems that reduce to it, such as matching workers to jobs.

## Related topics

- [Breadth-first search](learn:graphs/bfs): the search that finds each augmenting path.
- [Graph representations](learn:graphs/graph-representations): the matrix the capacities live in.
- [Bipartite check](learn:graphs/bipartite-check): bipartite matching is max flow on a two-sided graph.
