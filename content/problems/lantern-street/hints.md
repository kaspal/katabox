For each post, there are two situations: it is lit, or it is not. What does each situation allow for its children?
---
Compute two numbers per node, bottom-up: `lit(v)` = best total in v's subtree with v lit, and `dark(v)` = best total with v not lit.
---
`lit(v) = val + dark(left) + dark(right)` and `dark(v) = max(lit(left), dark(left)) + max(lit(right), dark(right))`. The answer is `max(lit(root), dark(root))`. Recomputing grandchildren from scratch instead is exponential.
