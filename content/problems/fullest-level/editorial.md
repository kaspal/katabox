## Approach: breadth-first search, one level per round

A queue visits the tree level by level. At the start of each round the queue holds exactly the nodes of one level:

1. Start with the root in the queue (an empty tree returns 0).
2. Read the queue's length `m`; pop `m` nodes, add up their values, and push their non-null children.
3. If this level's total is **greater** than the best so far, remember the total and the level.
4. Repeat until the queue is empty.

Comparing with `>` and not `≥` keeps the first level of a tie, the one closest to the root.

### Walkthrough

`root = [4, 7, 2, null, 1, 8, 3]`: round 1 pops 4 (best 4, level 1); round 2 pops 7 and 2 (9, level 2); round 3 pops 1, 8 and 3 (12, level 3). The answer is 3.

**Complexity:** O(n) time, O(w) extra space for the widest level. A depth-first walk that adds each value to a per-depth total works too, in O(n) time and O(h) space for the recursion.
