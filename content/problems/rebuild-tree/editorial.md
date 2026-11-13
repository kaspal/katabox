## Approach: divide by the root, with an index map

The first value of a preorder range is that subtree's root. Find it in the inorder range: the `L` values to its left form the left subtree, the rest its right subtree. In preorder, the left subtree is then the next `L` values and the right subtree the remaining ones. Recurse on both halves.

A dictionary from value to inorder position, built once, makes each lookup O(1); scanning the inorder range for every root instead costs O(n·h).

### Walkthrough

`preorder = [3, 9, 20, 15, 7]`, `inorder = [9, 3, 15, 20, 7]`: root 3 is at inorder position 1, so the left subtree is `[9]` and the right subtree has preorder `[20, 15, 7]` and inorder `[15, 20, 7]`, whose root is 20 with children 15 and 7.

**Complexity:** O(n) time, O(n) for the map and O(h) for the recursion.
