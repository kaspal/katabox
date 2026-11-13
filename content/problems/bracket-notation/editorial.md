## Approach: preorder with two rules

Walk the tree in preorder and append pieces to a list:

1. the node's value;
2. if it has a left child **or** a right child: `(`, the left subtree (possibly empty), `)`;
3. if it has a right child: `(`, the right subtree, `)`.

Rule 2 writes `()` for a missing left child exactly when a right child follows, which keeps the text unambiguous; a missing right child is simply left out.

**Complexity:** O(n) time and O(n) for the text (built in a list and joined once, not by repeated string concatenation, which could be O(n²)).
