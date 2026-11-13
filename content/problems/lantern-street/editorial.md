## Approach: DP on the tree with two states

For every node `v`, compute in post-order:

- `lit(v)`: the best total in `v`'s subtree when `v` is lit. Its children must be dark: `val(v) + dark(left) + dark(right)`.
- `dark(v)`: the best total when `v` is not lit. Each child is free: `max(lit(left), dark(left)) + max(lit(right), dark(right))`.

An empty subtree has `lit = dark = 0`. The answer is `max(lit(root), dark(root))`.

### Walkthrough

Example 2, `[2, 9, 1, null, null, 7, 8]`: for 9, lit = 9 and dark = 0; for 1, lit = 1 and dark = 7 + 8 = 15. At the root, lit = 2 + 0 + 15 = 17 and dark = max(9, 0) + max(1, 15) = 24. The answer is 24.

**Complexity:** O(n) time, O(h) for the recursion. The recursion "best(v) = max(val + best of grandchildren, best of children)" without remembering results recomputes subtrees again and again and is exponential in the height.
