## The idea

An undirected graph can fall apart into separate pieces: groups of nodes joined to each other by paths but with no edge reaching another group. Each piece is a **connected component**, and labelling them answers "can `a` reach `b`?" with a single comparison.

The sample uses a **flood fill**. Scan the nodes in index order. When you find one that has no label yet, it starts a new component: give it the next label and spread that label to everything it can reach. The spreading uses an explicit **stack**: pop a node, and every unlabelled neighbour gets the same label and is pushed. When the stack is empty, the component is complete, and the scan moves on. Because the scan goes in index order, labels are numbered in order of the smallest node of each component: the component holding node 0 gets label 0, and so on.

A node is labelled when it is pushed, not when it is popped, so it can never be pushed twice. Any walk works for the spreading: a queue would make it BFS, recursion would make it DFS; the labels come out the same.

### The default input

Ten nodes, undirected: a path 0 – 1 – 2, a triangle 3 – 4 – 5, and a chain 6 – 7 – 8 – 9 whose middle edge 7 – 8 is written last. Node 0 starts label 0 and the fill reaches 1, then 2. Nodes 1 and 2 are already labelled, so the scan skips them. Node 3 starts label 1: popping 3 labels both 4 and 5. Node 6 starts label 2, and the fill walks 7, 8, 9 along the chain.

The output is `[0, 0, 0, 1, 1, 1, 2, 2, 2, 2]`: three components.

## Steps

1. Set every label to `-1` and the component count to `0`.
2. Scan the nodes `s` in index order; skip any that already has a label.
3. Otherwise give `s` the label `count` and push it on an empty stack.
4. While the stack is not empty, pop `u`; for each unlabelled neighbour `v`, give it the label `count` and push it.
5. When the stack is empty, add one to `count` and continue the scan.

## Cost

| Operation | Cost |
|---|---|
| Whole labelling | `O(n + m)`: each node is pushed once, each list is read once |
| Extra space | `O(n)` for the labels and the stack |

## Pitfalls

- **Forgetting isolated nodes.** A node with no edges is a component of its own; the scan over every node catches it.
- **Labelling on pop.** Then a node with several labelled neighbours is pushed several times; harmless for the answer, wasteful for memory.
- **Using it on directed graphs.** Following arrows one way finds what a node reaches, not a component; mutual reachability needs strongly connected components (coming soon).

## When to use it

Use it to count islands, networks or groups, to check whether a graph is connected in one piece, or to answer many "same group?" questions after one pass. If edges keep arriving over time, union-find does the same job incrementally. Grids are graphs too: each cell is a node joined to its neighbours, and filling regions of a picture or a map is the same algorithm.

## Related topics

- [Union-find](learn:trees/union-find): components that grow edge by edge.
- [Stack](learn:linear/stack): the stack that drives the fill.
- [Depth-first search](learn:graphs/dfs): the recursive way to spread a label.
- Grid flood fill (coming soon): the same idea on the cells of a grid.
