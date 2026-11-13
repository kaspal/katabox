## Approach: breadth-first search, last node of each level

Process the tree level by level with a queue. At the start of a round, the queue holds exactly one level; pop its nodes left to right, push their children, and record the value of the **last** node popped.

### Walkthrough

`[7, 3, 9, 1, 5, null, null, null, 2]`: level 1 is `7`; level 2 is `3, 9` (see 9); level 3 is `1, 5` (see 5); level 4 is `2`. The answer is `[7, 9, 5, 2]`.

**Complexity:** O(n) time, O(w) space for the widest level. A depth-first walk that visits right children first and records the first node seen at each depth works too.
