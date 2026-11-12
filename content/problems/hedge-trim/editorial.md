## Approach: post-order, returning what remains and its total

The rule judges a branch only after its sub-branches, with **what remains** below it. So a post-order walk fits exactly: trim the children, then decide about the node.

Let `trim(node)` return the trimmed subtree and its total vigor:

1. `trim(null)` is `(null, 0)`.
2. Trim both children: `(left, a)` and `(right, b)`.
3. The total is `node.val + a + b`. If it is negative, return `(null, 0)`: the branch is cut, and a cut branch adds nothing to its parent.
4. Otherwise set `node.left = left`, `node.right = right` and return `(node, total)`.

The answer is the first part of `trim(root)`.

### Walkthrough

`root = [3, -5, 2, 1, -4]`: `trim(1)` = (1, 1); `trim(-4)` = cut; `trim(-5)`: -5 + 1 + 0 = -4, cut; `trim(2)` = (2, 2); the root: 3 + 0 + 2 = 5, kept with only its right child. The output is `[3, null, 2]`.

**Complexity:** O(n) time, O(h) extra space for the recursion (h ≤ 1000 levels). Recomputing each branch's total from scratch, top-down, repeats work: O(n·h).
