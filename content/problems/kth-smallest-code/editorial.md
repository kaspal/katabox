## Approach: in-order traversal, stopping at k

The in-order traversal of a search tree visits the values in increasing order, so the answer is the k-th node it visits.

With an explicit stack: push the whole left spine; pop a node (the next smallest); count it, and if the count reaches k return its value; otherwise move to its right child and push that child's left spine.

### Walkthrough

`[8, 3, 10, 1, 6, null, 14]`, k = 3: push 8, 3, 1; pop 1 (1st), pop 3 (2nd), push 6, pop 6 (3rd): the answer is 6.

**Complexity:** O(h + k) time, O(h) space, where h ≤ 1000 is the height.
