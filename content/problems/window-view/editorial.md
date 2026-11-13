## Approach: breadth-first search, last node of each level

Process the tree level by level with a queue. At the start of a round, the queue holds exactly one level; pop its nodes left to right, push their children, and record the value of the **last** node popped.

### Walkthrough

`[1, 2, 3, null, 5, null, 4]`: level 1 is `1`; level 2 is `2, 3` (see 3); level 3 is `5, 4` (see 4). The answer is `[1, 3, 4]`.

**Complexity:** O(n) time, O(w) space for the widest level. A depth-first walk that visits right children first and records the first node seen at each depth works too.
