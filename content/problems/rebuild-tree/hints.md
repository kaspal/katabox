The first member in preorder is the root. Where does the root sit in inorder, and what is on each side of it?
---
In inorder, everything left of the root is its left subtree and everything right of it is its right subtree. Their sizes tell you how to split the rest of preorder.
---
Recurse on (preorder range, inorder range). Look up the root's position in inorder with a dictionary built once, instead of searching every time.
