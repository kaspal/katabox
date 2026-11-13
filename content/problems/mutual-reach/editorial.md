## Approach: Kosaraju's two passes

1. **Finishing order:** run DFS over the graph from every unvisited user, appending each user to `order` when all its out-links are done.
2. **Reverse pass:** build the reversed graph. Take users from the end of `order` (latest finish first); each one not yet assigned starts a DFS on the reversed graph that collects exactly its group. Record the group's size.
3. Sort the sizes.

**Why it works:** the user that finishes last lies in a "source" group (no links come into it from other groups). In the reversed graph that group has no links **out** to unassigned groups, so the reverse DFS cannot leak beyond it. Removing it, the argument repeats.

Both DFS passes use explicit stacks, so long chains do not hit recursion limits.

**Complexity:** O(n + m) time and space. Testing every user's reachability separately (a BFS per user, then comparing) is O(n·(n + m)).
