## Approach: two-colouring with breadth-first search

The players and rivalries form a graph. A valid split is a colouring with two colours where every edge joins different colours: the graph must be **bipartite**.

Colour the graph component by component. Start a BFS from any uncoloured player with colour 0; every rival of a player with colour c gets colour 1 − c. If a rival already has colour c, there is an edge between two players that must be on the same team and on different teams at once (an odd cycle), so the answer is false. If every component is coloured without conflict, the answer is true.

### Walkthrough

Example 2: 0 gets 0, then 1 and 2 get 1. The rivalry 1–2 joins two players with colour 1: false.

**Complexity:** O(n + m) time and space. BFS (or an explicit stack) avoids deep recursion on long chains.
