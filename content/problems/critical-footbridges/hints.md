Closing each footbridge in turn and checking connectivity is O(m·(n + m)). A footbridge is critical exactly when it lies on no loop. How can one DFS find the loops?
---
Number the islands in DFS order (`tin`). For each island, compute `low`: the smallest `tin` reachable from its DFS subtree using at most one footbridge that goes back up the tree.
---
The tree footbridge from parent p to child c is critical exactly when `low[c] > tin[p]`: nothing in c's subtree can climb back to p or above. Skip only the footbridge you came down by (not every footbridge to the parent). Use an explicit stack for long chains.
