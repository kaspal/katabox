You need the total of every level. Can you visit the tree one level at a time?
---
A queue visits nodes in level order. If you know how many nodes the current level has before you start it, you know where it ends.
---
Process the queue in rounds: read its length, pop that many nodes (adding their values to this level's total and pushing their children), then compare the total with the best so far. Only a strictly larger total moves the answer, so ties keep the shallower level.
