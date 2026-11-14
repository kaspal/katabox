## The idea

A graph is **bipartite** when its nodes can be split into two sides, A and B, so that every edge joins one side to the other and no edge stays inside a side.

To test it, try to colour the nodes. The sample runs a BFS from each node that has no side yet: the start goes to side A (`0`), and every newly discovered neighbour goes to the opposite side of the node that found it (`1 − side`). Along every path BFS follows, the sides simply alternate, so the side of a node records whether it is an even or odd number of steps from the start. An edge whose two ends are already on the **same side** means there are two routes between those ends, one of even length and one of odd length, and together they close an **odd cycle**. An odd cycle can never alternate all the way round, so the answer is no and the sample outputs `[-1]`. Otherwise it outputs each node's side.

The outer loop over every node matters: each component is coloured on its own, and a graph is bipartite only if all of its components are.

### The default input

Eight nodes, undirected: a square 0 – 1 – 2 – 3 – 0, an edge 2 – 4, and a second square 4 – 5 – 6 – 7 – 4. Node 0 goes to A, and visiting it puts 1 and 3 on B. Visiting 1 puts 2 on A; visiting 3 finds 2 on A and 0 on A, both opposite to B, so no clash. Then 2 puts 4 on B, 4 puts 5 and 7 on A, and 5 puts 6 on B. Every remaining check finds opposite sides.

The output is `[0, 1, 0, 1, 1, 0, 1, 0]`: side A is {0, 2, 5, 7} and side B is {1, 3, 4, 6}. Both squares are cycles of length 4, which is even, so they alternate without trouble.

## Steps

1. Give every node the side `-1` (none yet).
2. For each node `s` in index order without a side: put it on side `0` and start a BFS queue with it.
3. Take the front node `u`. For each neighbour `v`: if `v` has no side, give it `1 − side[u]` and queue it.
4. If `v` already has the same side as `u`, stop and output `[-1]`.
5. When every component is coloured, output the sides.

## Cost

| Operation | Cost |
|---|---|
| Whole check | `O(n + m)`: each node is queued once, each list is read once |
| Extra space | `O(n)` for the sides and the queue |

## Pitfalls

- **Colouring only from node 0.** A clash in a component node 0 cannot reach would go unnoticed.
- **Treating "same side" as harmless.** An edge back to an already coloured node is only fine when its side is opposite; a matching side is the whole proof of failure.
- **Expecting a unique split.** Each component can be flipped, A for B, independently; the sample's answer is the one that puts each component's smallest node on side A.

## When to use it

Use it to split people into two teams with every rivalry across them, to check whether a scheduling conflict graph can be served by two time slots, or as the first step before bipartite matching, which pairs one side with the other.

## Related topics

- [Breadth-first search](learn:graphs/bfs): the walk that spreads the colours.
- [Connected components](learn:graphs/connected-components): the same outer loop over every piece.
- [Cycle detection](learn:graphs/cycle-detection): finding any cycle, not just odd ones.
- Max flow (coming soon): matching the two sides of a bipartite graph.
