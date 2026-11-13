## Approach: heights bottom-up, best turn at each node

Let `depth(v)` be the number of nodes on the longest downward path starting at `v` (0 for an empty subtree). Every walk has a highest node, where it turns; a walk turning at `v` has `depth(left) + depth(right)` paths at most, and that is achieved.

A post-order walk computes `depth(v) = 1 + max(depth(left), depth(right))` and, at the same time, updates `best = max(best, depth(left) + depth(right))`.

### Walkthrough

Example 2: at node 2, the left side (3 → 5) has depth 2 and the right side (4 → 6) has depth 2, so a walk turning at 2 has 4 paths. At the root, the best is depth(2's subtree) = 3 on one side and 0 on the other: 3. The answer is 4.

**Complexity:** O(n) time, O(h) for the recursion.
