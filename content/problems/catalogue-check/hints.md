Comparing each node only with its two children is not enough (see Example 2): the rule involves all ancestors.
---
Every node must lie strictly between bounds set by its ancestors: going left lowers the upper bound to the parent's value, going right raises the lower bound.
---
Recurse with `(node, low, high)`, starting with no bounds. Fail when `node.val ≤ low` or `node.val ≥ high`. Alternatively, an in-order walk of a search tree visits strictly increasing values.
