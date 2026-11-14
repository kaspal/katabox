## The idea

In a directed graph, two nodes are **strongly connected** when each can reach the other by following the arrows. This splits the nodes into **strongly connected components** (SCCs): groups where everyone reaches everyone. Shrink each group to a single dot and what remains is always acyclic, a map of one-way traffic between groups.

This topic uses **Kosaraju's algorithm**: two depth-first passes. (Tarjan's algorithm is the well-known alternative that finds the same groups in a single pass with low-link values.)

**Pass 1** runs DFS over all nodes, starting a new search from every node not yet seen, in number order, and records each node when it **finishes**, after all its descendants. **Pass 2** builds the **reversed graph**, where every arrow points the other way, and takes the nodes in **reverse finish order**. Each node without a label starts a DFS in the reversed graph; everything it reaches gets the current label, and the label counter goes up. Labels are numbered in the order pass 2 finds the components.

Why does that order work? If any arrow leads from group A to group B, some node of A finishes after every node of B. So the last node to finish lies in a group that no other group points into. In the reversed graph, that group's arrows to the others now point inward, so the search started there cannot leave its own group. Once it is labelled, the same argument applies to what is left.

### The default input

Eight nodes, directed: three loops joined one way, `0 → 1 → 2 → 0`, `3 → 4 → 5 → 3`, `6 → 7 → 6`, plus the links `2 → 3` and `5 → 6`. Pass 1 starts at 0 and dives `0, 1, 2, 3, 4, 5, 6, 7`; nodes finish in the order 7, 6, 5, 4, 3, 2, 1, 0. Pass 2 takes 0 first: in the reversed graph 0 reaches 2 and then 1, which become component 0. Next unlabelled is 3: it reaches 5 and 4, but its reversed link to 2 leads to a labelled node and stops there, so `{3, 4, 5}` is component 1. Finally 6 reaches 7: component 2.

The output is `[0, 0, 0, 1, 1, 1, 2, 2]`, each node's component label.

## Steps

1. Pass 1: for each node `u` in order, if unseen, DFS from it; append each node to `finished` when its DFS call ends.
2. Build the reversed graph: for every edge `u → v`, add `v → u`.
3. Pass 2: for each node in reverse `finished` order, if unlabelled, DFS in the reversed graph, giving every unlabelled node reached the current label; then add 1 to the label.
4. Return the labels.

## Cost

| Operation | Cost |
|---|---|
| Two DFS passes and the reversal | `O(n + m)` |
| Extra space | `O(n + m)`: the reversed graph, flags, labels and the finish list |

## Pitfalls

- **Using the original graph in pass 2.** Following the arrows forwards from the first node would sweep downstream groups into its component.
- **Using discovery order instead of finish order.** Only the finish order guarantees that upstream groups come first.
- **Deep recursion.** A long chain makes both passes recurse once per node; on big graphs, use an explicit stack.

## When to use it

Use SCCs to find groups of mutually reachable states, circular dependencies between modules, or to simplify a directed graph into its acyclic map of groups before running topological sort. They are also the core of the 2-SAT logic puzzle method.

## Related topics

- [Depth-first search](learn:graphs/dfs): both passes are plain DFS.
- [Topological sort](learn:graphs/topological-sort): the map of groups is always ready for it.
- [Connected components](learn:graphs/connected-components): the undirected version, with a single pass.
- [Cycle detection](learn:graphs/cycle-detection): a group with more than one node always contains a cycle.
