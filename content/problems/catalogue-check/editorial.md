## Approach: carry an open interval down the tree

The definition is about whole subtrees, so each node inherits bounds from all its ancestors. Walk the tree with an interval `(low, high)`, starting at `(−∞, +∞)`:

- the node's value must satisfy `low < val < high`;
- its left child gets `(low, val)` and its right child gets `(val, high)`.

If every node passes, the tree is a search tree.

**Equivalent check:** an in-order traversal (left, node, right) of a search tree lists the values in strictly increasing order; compare each visited value with the previous one.

### Walkthrough

`[8, 3, 10, 1, 9]`: the node 9 is the right child of 3, which is the left child of 8, so its interval is `(3, 8)`. 9 is not below 8: invalid.

**Complexity:** O(n) time, O(h) for the recursion (h ≤ 1000).
