Every walk has a highest fountain, where it turns from going up to going down. Which walks turn at a given node?
---
A walk turning at node v goes down into its left subtree as deep as possible and into its right subtree as deep as possible.
---
Compute each subtree's height (in paths) bottom-up. At node v, a walk turning there has `height(left) + height(right) + 2` paths when both children exist (count a missing child as height −1). Keep the maximum over all nodes.
