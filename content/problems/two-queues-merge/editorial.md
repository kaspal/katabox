## Approach: splice the smaller front

Use a dummy node `head` and a `tail` pointer at it. While both queues have nodes, compare their fronts, link the smaller one after `tail`, move `tail` to it, and advance that queue. Ties can take either front (here the first queue's), since only the ticket numbers matter. When one queue is empty, attach the other's remaining nodes to `tail` at once. The answer is `head.next`.

No node is created or copied: the result reuses the input nodes.

**Complexity:** O(n + m) time, O(1) extra space.
