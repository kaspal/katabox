Before reversing a batch, make sure it is full: look k nodes ahead.
---
Reversing one batch in place needs three pointers. Afterwards, the batch's old first node is its new last node, and it must link to whatever comes next.
---
Keep `before`, the node just ahead of the current batch (a dummy node at first). Check that k nodes follow it; reverse them; link `before.next` to the batch's new first node and the batch's new last node to the rest; then move `before` to that last node.
