## Approach: divide by the root, with an index map

The first value of a preorder range is that subtree's root. Find it in the inorder range: the `L` values to its left form the left subtree, the rest its right subtree. In preorder, the left subtree is then the next `L` values and the right subtree the remaining ones. Recurse on both halves.

A dictionary from value to inorder position, built once, makes each lookup O(1); scanning the inorder range for every root instead costs O(n·h).

### Walkthrough

`preorder = [5, 2, 4, 8, 6]`, `inorder = [2, 4, 5, 6, 8]`: root 5 is at inorder position 2, so the left subtree has preorder `[2, 4]` and inorder `[2, 4]` (2 with right child 4), and the right subtree has preorder `[8, 6]` and inorder `[6, 8]` (8 with left child 6).

**Complexity:** O(n) time, O(n) for the map and O(h) for the recursion.
