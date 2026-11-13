The front of the merged queue is the smaller of the two fronts.
---
Keep a pointer into each queue. Repeatedly take the node with the smaller ticket, link it after the last node of the result, and advance in that queue.
---
A dummy node in front of the result avoids a special case for the first node. When one queue runs out, link the rest of the other one in a single step.
